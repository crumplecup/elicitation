# EmitCode Auto-Derive via `#[elicit_tool]` Body Capture

## Problem

Every `impl EmitCode for XParams` is a hand-written parallel of the tool handler body.
They share the same structure but diverge the moment someone edits the handler.
The goal: **one source of truth** — the handler body — compiled for two targets
(MCP runtime and standalone binary) by `#[elicit_tool]`.

## Core Idea

The `#[elicit_tool]` macro captures the function body at compile time.
The handler body is treated as a **template** that gets rendered twice:

1. **MCP target** (existing): the body as-is, executed against real types
2. **Binary target** (new): the body rewritten — params substituted as literals,
   context replaced, MCP return types unwrapped — emitted as a `TokenStream`
   at runtime by `emit_code()`

The key insight: `#__field` inside a `quote!` block is a runtime interpolation.
If the macro generates the `emit_code()` body so that `#__field` interpolates
a `ToCodeLiteral`-produced `TokenStream`, the exact construction expression
(e.g. `UrlHttps::new("https://example.com").unwrap()`) is substituted wherever
`p.field` appeared in the original handler.

## New Trait: `ToCodeLiteral`

```rust
/// How a runtime value serializes itself as a Rust source-code expression.
///
/// Used by the auto-generated `EmitCode` impl to substitute concrete field
/// values into the captured handler body template.
#[cfg(feature = "emit")]
pub trait ToCodeLiteral {
    fn to_code_literal(&self) -> proc_macro2::TokenStream;
}
```

Impls cover:
- All Rust primitives (`f64`, `String`, `bool`, integers, `char`) — via `quote!(#self)`
- `Option<T: ToCodeLiteral>` — `None` or `Some(#inner_literal)`
- `Vec<T: ToCodeLiteral>` — `vec![#(#items),*]`
- `HashMap<String, String>` — `HashMap::from([#(#pairs),*])`
- Every constrained type in `verification/types/`:
  - `UrlHttps` → `elicitation::verification::types::UrlHttps::new(#s).expect("valid")`
  - `F64Positive` → `elicitation::verification::types::F64Positive::new(#v).expect("positive")`
  - `StringNonEmpty<N>` → `elicitation::verification::types::StringNonEmpty::<N>::new(#s.to_string()).expect("non-empty")`
  - etc. — each type knows how to reconstruct itself

## New `#[elicit_tool]` Attributes

```rust
#[elicit_tool(
    plugin   = "secure_fetch",
    name     = "secure_fetch",
    description = "...",

    // Declare context field → replacement expression
    emit_ctx("ctx.http" => "reqwest::Client::new()"),
)]
async fn secure_fetch(ctx: Arc<PluginContext>, p: SecureFetchParams) -> ... { ... }
```

`emit = false` opts out: the macro generates nothing for EmitCode, allowing a
manual `impl EmitCode` to coexist for complex handlers.

## What the Macro Generates

Given the handler above, the macro generates (in addition to the existing
`PluginToolRegistration`):

```rust
#[cfg(feature = "emit")]
impl elicitation::emit_code::EmitCode for SecureFetchParams {
    fn emit_code(&self) -> proc_macro2::TokenStream {
        // Bind each field's literal representation:
        let __url          = elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.url);
        let __timeout_secs = elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.timeout_secs);

        // Rewritten body — produced at macro-expansion time from the handler body:
        //   p.url          → #__url   (quote! interpolation of the literal TS)
        //   p.timeout_secs → #__timeout_secs
        //   ctx.http       → reqwest::Client::new()   (declared emit_ctx)
        //   ErrorData::*() → format!(...)             (standard rewrite)
        //   Ok(CallToolResult::success(...)) → (output expression)
        ::quote::quote! {
            let url_str = #__url .get().as_str().to_owned();
            let __http = reqwest::Client::new();
            let response = __http
                .get(&url_str)
                .timeout(::std::time::Duration::from_secs_f64(#__timeout_secs))
                .send()
                .await
                .map_err(|e| format!("HTTP request failed: {e}"))?;
            let status = response.status();
            let body = response.text().await.map_err(|e| format!("Body error: {e}"))?;
            println!("status={status}");
            println!("body_len={}", body.len());
            println!("{}", &body[..512.min(body.len())]);
        }
    }

    fn crate_deps(&self) -> Vec<elicitation::emit_code::CrateDep> {
        vec![
            elicitation::emit_code::CrateDep::new("reqwest", "0.13"),
            elicitation::emit_code::CrateDep::new("elicitation", "0.8"),
        ]
    }
}

// Also auto-generated (replacing manual register_emit!):
#[cfg(feature = "emit")]
elicitation::register_emit!("secure_fetch", SecureFetchParams);
```

The key: `#__url` inside `quote!` is a *runtime* interpolation of the
`TokenStream` produced by `self.url.to_code_literal()`. The body tokens
(with `p.field` replaced by `Punct('#') + Ident("__field")`) were baked in
at macro-expansion time.

## Token Rewriter

New module: `crates/elicitation_derive/src/emit_rewriter.rs`

```rust
pub struct EmitRewriter {
    /// Field names found by scanning body for `p . <ident>` patterns.
    pub param_fields: Vec<String>,
    /// Declared context substitutions: token sequence → replacement.
    pub ctx_subs: Vec<(Vec<TokenTree>, TokenStream)>,
}

impl EmitRewriter {
    /// Walk `body`, return rewritten TokenStream suitable for embedding
    /// inside a `quote!` call in the generated `emit_code()` body.
    pub fn rewrite(&self, body: TokenStream) -> TokenStream { ... }
}
```

Rewrite passes (applied in order, recursive over Groups):

| Pattern | Replacement |
|---|---|
| `p . <ident>` | `Punct('#') + Ident("__<ident>")` |
| `ctx . http` (declared) | `reqwest::Client::new()` |
| `ErrorData :: <v> ( <msg> , <_> )` | `<msg>` (strip wrapper, keep message) |
| `Ok ( CallToolResult :: success ( vec! [ Content :: text ( <x> ) ] ) )` | emit `<x>` as output |
| `return Ok ( CallToolResult :: error (...) )` | `return Err(format!(...))` |

Any unrecognised pattern passes through unchanged (fail-safe).

The rewriter is a best-effort pass. Complex handlers that defeat it use
`emit = false` and a manual `impl EmitCode` fallback — this is not an error,
it's the designed escape hatch.

## Phases

### Phase 1 — `ToCodeLiteral` trait

**File**: `crates/elicitation/src/emit_code.rs`

1. Declare `ToCodeLiteral` trait (feature-gated `emit`)
2. Blanket impls for `f64`, `f32`, `i8..i64`, `u8..u64`, `usize`, `bool`, `char`,
   `String`, `&str`
3. Impls for `Option<T>`, `Vec<T>`, `HashMap<String, String>`
4. Impls for every type in `verification/types/floats.rs`, `integers.rs`,
   `strings.rs`, `urls.rs` — one per constrained type
5. Tests in `tests/emit_literal_test.rs`: serialize to `TokenStream`, parse back
   via `syn::parse_str`, assert value round-trips

### Phase 2 — Extend `#[elicit_tool]` attribute parsing

**File**: `crates/elicitation_derive/src/elicit_tool.rs`

Add to `ElicitToolArgs`:
```rust
emit: bool,                             // default true; false = opt-out
emit_ctx_subs: Vec<(String, String)>,   // ("ctx.http", "reqwest::Client::new()")
```

Parse `emit_ctx("ctx.http" => "reqwest::Client::new()")` using a custom
`Parse` impl for the new list-style attribute form.

**No `emit_crate` attribute.** Crate dependencies are inferred automatically:
1. After the token rewriter produces the final body `TokenStream`, scan it for
   top-level path prefixes — any `<ident> ::` where the leading ident is not
   `std`, `core`, `alloc`, `self`, `super`, `crate`.
2. Also scan `emit_ctx` replacement expressions for the same prefixes.
3. For each unique prefix, walk up from `CARGO_MANIFEST_DIR` to the workspace
   root and look up the version in `[workspace.dependencies]`.
4. Emit a `CrateDep` for each found; emit a compile warning (not error) for
   any prefix not found in workspace deps (may be a local path dep or std-re-export).

### Phase 3 — Token rewriter

**New file**: `crates/elicitation_derive/src/emit_rewriter.rs`

- `scan_param_fields(body: &TokenStream) -> Vec<String>` — find all `p . <ident>` patterns
- `rewrite(body: TokenStream, fields: &[String], ctx_subs: &[(TokenStream, TokenStream)]) -> TokenStream`
- Return value rewriter: recognise the `Ok(CallToolResult::success(...))` shape and extract inner text
- Test with the `secure_fetch` body: assert the rewritten TokenStream compiles and produces expected output

### Phase 4 — Generated `impl EmitCode` + `register_emit!`

**File**: `crates/elicitation_derive/src/elicit_tool.rs`

In `expand()`, after generating `PluginToolRegistration`, also emit:

```rust
if args.emit {
    let rewriter = EmitRewriter::from(&args, &func);
    let rewritten_body = rewriter.rewrite(func.block.to_token_stream());
    let field_bindings = rewriter.param_fields.iter().map(|f| {
        let ident = format_ident!("__{}", f);
        let field = format_ident!("{}", f);
        quote! { let #ident = elicitation::emit_code::ToCodeLiteral::to_code_literal(&p.#field); }
        // p here is `self` in emit_code — use self
    });
    let crate_deps = /* inferred by scanning rewritten body + emit_ctx exprs for path prefixes,
                        versions resolved from workspace Cargo.toml */;

    quote! {
        #[cfg(feature = "emit")]
        impl elicitation::emit_code::EmitCode for #params_ty {
            fn emit_code(&self) -> proc_macro2::TokenStream {
                #(#field_bindings)*
                ::quote::quote! { #rewritten_body }
            }
            fn crate_deps(&self) -> Vec<elicitation::emit_code::CrateDep> {
                vec![ #crate_deps ]
            }
        }

        #[cfg(feature = "emit")]
        elicitation::register_emit!(#tool_name, #params_ty);
    }
}
```

### Phase 5 — Canary migration

**File**: `crates/elicit_server/src/secure_fetch.rs`

1. Add `emit_ctx` + `emit_crate` attrs to `#[elicit_tool]` on `secure_fetch` and `validated_api_call`
2. Delete the `mod emit { impl EmitCode for SecureFetchParams ... }` block
3. Delete the `dispatch_secure_fetch_emit` fn and manual `register_emit!` calls
4. `cargo check` + run emit smoke test: assert generated code matches (or improves on) the deleted manual impl
5. Run `just check-all elicit_server`

### Phase 6 — Ecosystem rollout

Apply to remaining workflow crates that have manual EmitCode:
- `elicit_url/src/workflow.rs` — 5 tools, stateless
- `elicit_chrono/src/workflow.rs` — 5 tools, stateless
- `elicit_jiff/src/workflow.rs` — 5 tools, stateless
- `elicit_time/src/workflow.rs` — 5 tools, stateless
- `elicit_reqwest/src/plugins/workflow.rs` — 10 tools, ctx-aware
- `elicit_serde_json/src/workflow.rs` — stateless

Each crate: add `emit_ctx` (context-aware handlers only) to `#[elicit_tool]`, delete manual impls,
delete per-crate `dispatch_*_emit` free functions, run tests.

After all crates migrated, the global `dispatch_emit()` in `emit_code.rs` is the
sole dispatch path — the per-crate variants become dead code and are removed.

## Limitations & Escape Hatch

Not every handler can be auto-rewritten. Opt out with `emit = false`:

```rust
#[elicit_tool(plugin = "...", name = "...", description = "...", emit = false)]
async fn complex_tool(ctx: Arc<PluginContext>, p: ComplexParams) -> ... { ... }

// Manual impl coexists safely:
#[cfg(feature = "emit")]
impl EmitCode for ComplexParams { ... }
```

Cases that require `emit = false`:
- `let client = ctx.http.clone()` — aliased context field
- Multiple `return Ok(CallToolResult::error(...))` mid-body
- Handlers that build params dynamically from other params
- Any handler that uses `ctx` fields not declared in `emit_ctx`

## Expected Outcome

After Phase 5 (canary):
- `secure_fetch.rs` loses ~80 lines of manual EmitCode
- Handler body and emitted binary are provably in sync (same source)
- Any future edit to `secure_fetch` automatically propagates to the emitted binary

After Phase 6 (full rollout):
- ~600 lines of manual EmitCode deleted across all workflow crates
- `dispatch_*_emit` free functions all deleted
- Zero drift possible: you cannot update a handler without updating the emit output

## Key Technical Notes

- `##ident` in `quote!` produces `#ident` in the output — this is how the macro
  embeds `quote!` interpolation points into the generated `emit_code()` body
- The rewriter is fail-safe: unrecognised patterns pass through unchanged
- `ToCodeLiteral` is the only per-type work; each impl is ~3 lines and
  co-located with the type definition
- `emit_ctx` declarations are the only per-handler configuration; most stateless
  handlers need no `emit_ctx` at all
