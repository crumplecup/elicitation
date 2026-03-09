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

    /// Scan a token stream for leading path-segment identifiers (`ident ::`).
    ///
    /// Returns unique crate names inferred from top-level path prefixes,
    /// excluding `std`, `core`, `alloc`, `self`, `super`, `crate`,
    /// `elicitation`, and CamelCase names (which are types, not crates).
    pub(crate) fn infer_crate_names(ts: &TokenStream) -> Vec<String> {
        const EXCLUDED: &[&str] = &[
            "std",
            "core",
            "alloc",
            "self",
            "super",
            "crate",
            "elicitation",
        ];
        let mut names = std::collections::HashSet::new();
        collect_path_prefixes(ts, &mut names);
        names
            .into_iter()
            .filter(|n| {
                !EXCLUDED.contains(&n.as_str())
                    && n.len() > 1
                    && !n.starts_with(|c: char| c.is_uppercase())
            })
            .collect()
    }

    /// Attempt to resolve a crate name to its version from the workspace
    /// `Cargo.toml`.
    ///
    /// Walks up from `CARGO_MANIFEST_DIR` looking for a `Cargo.toml` with a
    /// `[workspace]` section, then line-scans `[workspace.dependencies]`.
    pub(crate) fn resolve_workspace_version(crate_name: &str) -> Option<String> {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").ok()?;
        let mut dir = std::path::PathBuf::from(&manifest_dir);

        loop {
            let candidate = dir.join("Cargo.toml");
            if candidate.exists()
                && let Ok(content) = std::fs::read_to_string(&candidate)
                && content.contains("[workspace]")
            {
                return parse_workspace_dep_version(&content, crate_name);
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => return None,
            }
        }
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

fn collect_path_prefixes(ts: &TokenStream, out: &mut std::collections::HashSet<String>) {
    let tokens: Vec<TokenTree> = ts.clone().into_iter().collect();
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            TokenTree::Ident(id) => {
                // Skip method calls: `.collect::<Vec<_>>()` has a preceding dot
                let preceded_by_dot =
                    i > 0 && matches!(&tokens[i - 1], TokenTree::Punct(p) if p.as_char() == '.');
                if !preceded_by_dot
                    && i + 2 < tokens.len()
                    && let (TokenTree::Punct(c1), TokenTree::Punct(c2)) =
                        (&tokens[i + 1], &tokens[i + 2])
                    && c1.as_char() == ':'
                    && c1.spacing() == Spacing::Joint
                    && c2.as_char() == ':'
                {
                    out.insert(id.to_string());
                }
            }
            TokenTree::Group(g) => {
                collect_path_prefixes(&g.stream(), out);
            }
            _ => {}
        }
        i += 1;
    }
}

fn parse_workspace_dep_version(content: &str, crate_name: &str) -> Option<String> {
    let mut in_ws_deps = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[workspace.dependencies]" {
            in_ws_deps = true;
            continue;
        }
        if in_ws_deps {
            if trimmed.starts_with('[') {
                in_ws_deps = false;
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix(crate_name) {
                let rest = rest.trim();
                // Ensure exact name match (not a prefix of a longer name)
                if !rest.starts_with('=') && !rest.starts_with(' ') {
                    continue;
                }
                let rest = rest.trim_start_matches('=').trim();
                if rest.starts_with('"') {
                    // crate_name = "version"
                    let inner = rest.trim_matches('"');
                    if !inner.is_empty() {
                        return Some(inner.to_string());
                    }
                } else if rest.starts_with('{') {
                    // crate_name = { version = "v", features = [...] }
                    if let Some(ver_start) = rest.find("version = \"") {
                        let after = &rest[ver_start + 11..];
                        if let Some(ver_end) = after.find('"') {
                            return Some(after[..ver_end].to_string());
                        }
                    }
                }
            }
        }
    }
    None
}
