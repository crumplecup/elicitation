//! Token rewriter for `#[elicit_tool]` `EmitCode` auto-generation.
//!
//! Transforms a handler body [`TokenStream`] for use as an `EmitCode` body:
//! - `p . <field>` → `# __<field>` (quote! interpolation point)
//! - `ctx . <field>` → declared replacement expression
//! - `ErrorData :: <method> ( msg_expr , .. )` → `msg_expr` (strip MCP error wrapper)
//! - `Ok ( <group_containing_CallToolResult> )` → `println!("{}", <content>)` (strip MCP result)
//!
//! Unrecognised patterns pass through unchanged (fail-safe).

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

/// Rewriter that transforms a handler body into an `EmitCode`-suitable body.
pub(crate) struct EmitRewriter {
    /// Field names found by scanning for `p . <ident>` patterns.
    pub param_fields: Vec<String>,
    /// Declared substitutions: (ctx_field_name, replacement_tokens).
    pub(crate) ctx_subs: Vec<(String, TokenStream)>,
}

impl EmitRewriter {
    /// Create a new rewriter from the `emit_ctx` substitution list.
    ///
    /// Each entry is `("ctx.field", "replacement_expr_source")`.  The lhs is
    /// split on `.` to extract the field name; the rhs is parsed as a
    /// [`TokenStream`].
    pub(crate) fn new(ctx_subs: Vec<(String, String)>) -> Self {
        let ctx_subs = ctx_subs
            .into_iter()
            .map(|(k, v)| {
                let field = k.split_once('.').map_or(k.as_str(), |(_, r)| r).to_string();
                let replacement: TokenStream = v.parse().unwrap_or_default();
                (field, replacement)
            })
            .collect();
        EmitRewriter {
            param_fields: Vec::new(),
            ctx_subs,
        }
    }

    /// Walk `body`, applying substitutions, and return the rewritten stream.
    ///
    /// Applied rewrites (in priority order):
    /// 1. `p . <ident>` → `# __<ident>` — params field interpolation point
    /// 2. `ctx . <ident>` → declared replacement — context substitution
    /// 3. `ErrorData :: <method> ( msg, .. )` → `msg` — strip MCP error wrapper
    /// 4. `Ok ( <CallToolResult_group> )` → `println!("{}", content)` — strip MCP success wrapper
    /// 5. Everything else passes through unchanged (groups are recursed into).
    pub(crate) fn rewrite(&mut self, body: TokenStream) -> TokenStream {
        let tokens: Vec<TokenTree> = body.into_iter().collect();
        let mut output: Vec<TokenTree> = Vec::with_capacity(tokens.len());
        let mut i = 0;

        while i < tokens.len() {
            let tt = &tokens[i];

            match tt {
                // ── p . <ident> → # __<ident> ──────────────────────────────────
                TokenTree::Ident(id) if id == "p" => {
                    if i + 2 < tokens.len()
                        && let (TokenTree::Punct(dot), TokenTree::Ident(field)) =
                            (&tokens[i + 1], &tokens[i + 2])
                        && dot.as_char() == '.'
                    {
                        let field_name = field.to_string();
                        let mangled = format!("__{field_name}");
                        output.push(TokenTree::Punct(Punct::new('#', Spacing::Alone)));
                        output.push(TokenTree::Ident(Ident::new(&mangled, Span::call_site())));
                        if !self.param_fields.contains(&field_name) {
                            self.param_fields.push(field_name);
                        }
                        i += 3;
                        continue;
                    }
                    output.push(tt.clone());
                    i += 1;
                }

                // ── ctx . <ident> → declared replacement ───────────────────────
                TokenTree::Ident(id) if id == "ctx" => {
                    if i + 2 < tokens.len()
                        && let (TokenTree::Punct(dot), TokenTree::Ident(field)) =
                            (&tokens[i + 1], &tokens[i + 2])
                        && dot.as_char() == '.'
                    {
                        let field_name = field.to_string();
                        let replacement = self
                            .ctx_subs
                            .iter()
                            .find(|(k, _)| k == &field_name)
                            .map(|(_, v)| v.clone());
                        if let Some(rep) = replacement {
                            output.extend(rep);
                            i += 3;
                            continue;
                        }
                    }
                    // Bare `ctx` (not `ctx.field`) has no meaning in a standalone
                    // binary — substitute with `()` so surrounding expressions like
                    // `let _ = &ctx;` compile cleanly.
                    output.extend(quote::quote! { () });
                    i += 1;
                }

                // ── ErrorData :: <method> ( msg, .. ) → msg ────────────────────
                //
                // Strips the MCP error-wrapper so the emitted standalone binary
                // can use plain `format!(...)` strings with `?`.
                TokenTree::Ident(id) if id == "ErrorData" => {
                    if let Some((replacement, advance)) =
                        try_extract_error_data_first_arg(&tokens, i)
                    {
                        // Recurse into the extracted arg in case it contains `p.field`
                        let rewritten = self.rewrite(replacement);
                        output.extend(rewritten);
                        i = advance;
                        continue;
                    }
                    output.push(tt.clone());
                    i += 1;
                }

                // ── return Ok ( <CallToolResult_group> ) → print+return ────────
                //
                // `return Ok(CallToolResult::error/success(...))` in an early-return
                // position must become `{ println!("..."); return Ok(()); }` so the
                // emitted main still compiles (return type is Result, not ()).
                TokenTree::Ident(id) if id == "return" => {
                    if i + 1 < tokens.len()
                        && let TokenTree::Ident(next_id) = &tokens[i + 1]
                        && next_id == "Ok"
                        && let Some((println_ts, advance)) =
                            try_extract_ok_call_tool_result(&tokens, i + 1)
                    {
                        let rewritten = self.rewrite(println_ts);
                        // Emit: { println!(...); return Ok(()); }
                        let inner: TokenStream = rewritten
                            .into_iter()
                            .chain(quote::quote! { ; return Ok(()) }.into_iter())
                            .collect();
                        let group = Group::new(Delimiter::Brace, inner);
                        output.push(TokenTree::Group(group));
                        i = advance + 1; // skip past the `return` we consumed
                        continue;
                    }
                    // Fallback: `return Ok(single_ident)` — bare variable return.
                    if i + 1 < tokens.len()
                        && let TokenTree::Ident(next_id) = &tokens[i + 1]
                        && next_id == "Ok"
                        && try_extract_ok_variable(&tokens, i + 1).is_some()
                    {
                        // Emit `return Ok(())` — discard the variable value.
                        output.extend(quote::quote! { return Ok(()) });
                        i += 3; // skip `return`, `Ok`, `(ident)`
                        continue;
                    }
                    output.push(tt.clone());
                    i += 1;
                }

                //
                // The final return of a handler is
                //   `Ok(CallToolResult::success(vec![Content::text(X)]))`
                // which must not appear in a standalone binary. We replace it
                // with a `println!("{}", X)` so the output is still visible.
                TokenTree::Ident(id) if id == "Ok" => {
                    if let Some((println_ts, advance)) = try_extract_ok_call_tool_result(&tokens, i)
                    {
                        let rewritten = self.rewrite(println_ts);
                        output.extend(rewritten);
                        i = advance;
                        continue;
                    }
                    // Not a CallToolResult Ok — recurse into group if any
                    output.push(tt.clone());
                    i += 1;
                }

                // ── Recurse into groups ─────────────────────────────────────────
                TokenTree::Group(g) => {
                    let delim = g.delimiter();
                    let inner = self.rewrite(g.stream());
                    let mut new_group = Group::new(delim, inner);
                    new_group.set_span(g.span());
                    output.push(TokenTree::Group(new_group));
                    i += 1;
                }

                _ => {
                    output.push(tt.clone());
                    i += 1;
                }
            }
        }

        output.into_iter().collect()
    }

    /// Return all direct dependencies of the calling crate with resolved versions.
    ///
    /// Reads `$CARGO_MANIFEST_DIR/Cargo.toml` via the `toml` crate.  For each
    /// `[dependencies]` entry:
    /// - inline version string → used directly
    /// - `{ version = "x" }` → used directly  
    /// - `{ workspace = true }` → resolved from `[workspace.dependencies]`
    /// - `{ path = "..." }` → workspace member; uses `[workspace.package].version`
    ///
    /// Called at macro expansion time; returns a `Vec<(name, version)>` that the
    /// macro embeds as a literal `crate_deps()` body.
    pub(crate) fn all_crate_deps() -> Vec<(String, String)> {
        let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") else {
            return vec![];
        };
        let manifest_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");
        let Ok(manifest_str) = std::fs::read_to_string(&manifest_path) else {
            return vec![];
        };
        let Ok(manifest) = toml::from_str::<toml::Value>(&manifest_str) else {
            return vec![];
        };

        // Locate workspace root and parse it.
        let workspace_val = find_workspace_root(&manifest_dir)
            .and_then(|root| std::fs::read_to_string(root.join("Cargo.toml")).ok())
            .and_then(|s| toml::from_str::<toml::Value>(&s).ok());

        let workspace_pkg_version = workspace_val
            .as_ref()
            .and_then(|w| w.get("workspace"))
            .and_then(|ws| ws.get("package"))
            .and_then(|pkg| pkg.get("version"))
            .and_then(|v| v.as_str())
            .unwrap_or("0")
            .to_string();

        let Some(dep_table) = manifest.get("dependencies").and_then(|d| d.as_table()) else {
            return vec![];
        };

        let mut deps = Vec::new();

        // Always include the crate defining these handlers — it contains the
        // types referenced bare (without a `crate_name::` prefix) in the emitted
        // code, and doesn't appear in its own `[dependencies]`.
        let own_name = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
        let own_version =
            std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| workspace_pkg_version.clone());
        if !own_name.is_empty() {
            deps.push((own_name, own_version));
        }

        for (name, value) in dep_table {
            let version = match value {
                toml::Value::String(v) => v.clone(),
                toml::Value::Table(t) => {
                    if t.get("workspace")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false)
                    {
                        // workspace = true: resolve from [workspace.dependencies]
                        workspace_val
                            .as_ref()
                            .and_then(|w| w.get("workspace"))
                            .and_then(|ws| ws.get("dependencies"))
                            .and_then(|d| d.get(name.as_str()))
                            .and_then(|entry| match entry {
                                toml::Value::String(v) => Some(v.clone()),
                                toml::Value::Table(t) => t
                                    .get("version")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string()),
                                _ => None,
                            })
                            .unwrap_or_else(|| "0".to_string())
                    } else if t.contains_key("path") {
                        // path = workspace member — version is the workspace package version
                        workspace_pkg_version.clone()
                    } else {
                        t.get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("0")
                            .to_string()
                    }
                }
                _ => continue,
            };
            deps.push((name.clone(), version));
        }
        deps
    }
}

// ── Pattern helpers ───────────────────────────────────────────────────────────

/// Match `ErrorData :: <method> ( args... )` starting at `i`.
///
/// Returns `(first_arg_stream, new_i)` on success, where `first_arg_stream`
/// is the tokens before the first top-level `,` inside the call's parentheses,
/// and `new_i` is the index after the closing `)`.
fn try_extract_error_data_first_arg(
    tokens: &[TokenTree],
    i: usize,
) -> Option<(TokenStream, usize)> {
    // tokens[i]   = Ident("ErrorData")
    // tokens[i+1] = Punct(':', Joint)
    // tokens[i+2] = Punct(':', Alone)
    // tokens[i+3] = Ident(method)
    // tokens[i+4] = Group(Paren, args)
    if i + 4 >= tokens.len() {
        return None;
    }
    let c1 = match &tokens[i + 1] {
        TokenTree::Punct(p) if p.as_char() == ':' && p.spacing() == Spacing::Joint => p,
        _ => return None,
    };
    let _ = c1;
    if !matches!(&tokens[i + 2], TokenTree::Punct(p) if p.as_char() == ':') {
        return None;
    }
    if !matches!(&tokens[i + 3], TokenTree::Ident(_)) {
        return None;
    }
    let args_group = match &tokens[i + 4] {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => g,
        _ => return None,
    };

    let inner: Vec<TokenTree> = args_group.stream().into_iter().collect();
    let first_arg = extract_first_comma_arg(&inner);
    Some((first_arg, i + 5))
}

/// Match `Ok ( <group_containing_CallToolResult> )` and extract the string
/// content from the deepest `Content::text(X)` call.
///
/// Returns `(println_stream, new_i)` on success.
fn try_extract_ok_call_tool_result(tokens: &[TokenTree], i: usize) -> Option<(TokenStream, usize)> {
    // tokens[i]   = Ident("Ok")
    // tokens[i+1] = Group(Paren, inner)
    if i + 1 >= tokens.len() {
        return None;
    }
    let inner_group = match &tokens[i + 1] {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => g,
        _ => return None,
    };

    // Only trigger when inner contains "CallToolResult" — avoids false matches
    // on other Ok(x) expressions.
    let inner_str = inner_group.stream().to_string();
    if !inner_str.contains("CallToolResult") {
        return None;
    }

    // Recursively find `text ( X )` — the innermost Content::text call.
    let content_ts = extract_content_text(&inner_group.stream())?;

    // Emit: println!("{}", <content_ts>)
    let println_stream: TokenStream = quote::quote! { println!("{}", #content_ts) };
    Some((println_stream, i + 2))
}

/// Match `Ok ( <single_ident> )` where the ident is a bare variable name
/// (not a CallToolResult constructor). Emits `()` so the match arm is
/// compatible with arms that were rewritten to `println!(...)`.
///
/// This handles error-propagation patterns like `Err(e) => Ok(e)` where
/// `e: CallToolResult` — in the emitted binary there is no CallToolResult,
/// so we simply discard the arm result.
fn try_extract_ok_variable(tokens: &[TokenTree], i: usize) -> Option<(TokenStream, usize)> {
    // tokens[i]   = Ident("Ok")
    // tokens[i+1] = Group(Paren, single_ident)
    if i + 1 >= tokens.len() {
        return None;
    }
    let inner_group = match &tokens[i + 1] {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => g,
        _ => return None,
    };
    let inner: Vec<TokenTree> = inner_group.stream().into_iter().collect();
    // Only handle `Ok(single_ident)` — a single identifier and nothing else.
    if inner.len() == 1 && matches!(&inner[0], TokenTree::Ident(_)) {
        Some((quote::quote! { () }, i + 2))
    } else {
        None
    }
}

/// Collect tokens before the first top-level `,` (the first call argument).
fn extract_first_comma_arg(tokens: &[TokenTree]) -> TokenStream {
    let mut result: Vec<TokenTree> = Vec::new();
    for tt in tokens {
        if let TokenTree::Punct(p) = tt
            && p.as_char() == ','
        {
            break;
        }
        result.push(tt.clone());
    }
    result.into_iter().collect()
}

/// Recursively search for `Ident("text") Group(Paren, X)` and return `X`.
///
/// This matches `Content::text(X)` regardless of the preceding path segments.
fn extract_content_text(ts: &TokenStream) -> Option<TokenStream> {
    let tokens: Vec<TokenTree> = ts.clone().into_iter().collect();
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Ident(id) if id == "text" => {
                if let Some(TokenTree::Group(g)) = tokens.get(i + 1)
                    && g.delimiter() == Delimiter::Parenthesis
                {
                    return Some(g.stream());
                }
            }
            TokenTree::Group(g) => {
                if let Some(result) = extract_content_text(&g.stream()) {
                    return Some(result);
                }
            }
            _ => {}
        }
        i += 1;
    }
    None
}

// ── Workspace dep resolution ──────────────────────────────────────────────────

/// Walk up from `manifest_dir` to find the workspace root `Cargo.toml`.
///
/// Returns the directory containing it, or `None` if not found.
fn find_workspace_root(manifest_dir: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::PathBuf::from(manifest_dir);
    loop {
        let candidate = dir.join("Cargo.toml");
        if candidate.exists()
            && let Ok(content) = std::fs::read_to_string(&candidate)
            && content.contains("[workspace]")
        {
            return Some(dir);
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => return None,
        }
    }
}
