# Release Notes — elicitation 0.9.1

**Release date:** 2026-03-09

This is a patch release focused on architecture improvements, code quality, and
developer tooling. No public API removals. Several ergonomic additions make
`elicit_*` newtypes drop-in replacements for the types they wrap.

---

## Highlights

### Single source of truth for workflows and emitted binaries

The biggest architectural win of this cycle. Previously, a workflow plugin
defined its tool parameters, its runtime dispatch logic, and its code-emission
logic in three separate places — keeping them in sync was manual and error-prone.

`#[elicit_tool]` now drives all three from one place:

```rust
#[elicit_tool]
#[derive(ElicitPlugin, serde::Deserialize)]
pub struct FetchAndExtractParams {
    pub url: UrlValid,
    pub selector: StringNonEmpty,
}
```

The macro reads the struct once and generates:
- MCP tool descriptor (name, schema)
- Runtime dispatch entry in the global `inventory` registry
- `EmitCode` impl that reproduces the binary calling this tool

The global emit registry (`register_emit!` + `dispatch_emit`) means any crate
that links a plugin automatically makes it available to code recovery — no manual
routing table needed.

### `CustomEmit<P>` replaces `emit = false`

The old `emit = false` attribute silently dropped code emission. The new
`CustomEmit<P>` trait gives users an explicit escape hatch: implement
`emit_code()` yourself when the macro can't do it for you.

```rust
impl CustomEmit<MyParams> for MyParams {
    fn emit_code(&self) -> proc_macro2::TokenStream { ... }
}
```

If you can't auto-derive, you provide a function — the capability disappears
only if you choose not to implement it.

### Cargo metadata replaces heuristic dep inference

`EmitCode` scaffolds need to declare which crates they depend on. The old
approach inferred deps by walking type name prefixes — fragile and often wrong.

The new `all_crate_deps()` reads `Cargo.toml` directly via the `toml` crate at
proc-macro expansion time (`$CARGO_MANIFEST_DIR`), resolves `workspace = true`
entries from the workspace root, and prepends the crate's own name/version
(which a crate never lists in its own `[dependencies]`). Result: the dep list
is always authoritative.

### `elicit_newtype_traits!` — drop-in newtype compatibility

`elicit_uuid::Uuid`, `elicit_url::Url`, and friends wrap `Arc<T>` for
thread-safe shared ownership. But `Arc<T>` only derives `Debug + Clone` by
default, which blocked users from deriving `PartialEq`, `Hash`, `Ord`, etc. on
their own structs that contain these types.

The new `elicit_newtype_traits!` macro forwards standard traits from the inner
type to the wrapper:

```rust
// In elicit_uuid:
elicit_newtype_traits!(Uuid, uuid::Uuid, [cmp, display, from_str]);
```

Available flags: `eq`, `eq_hash`, `ord`, `cmp` (all 5 ordering traits),
`display`, `from_str`. This unblocks:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MyRecord {
    pub id: elicit_uuid::Uuid,   // ✅ now works
    pub endpoint: elicit_url::Url,
}
```

Applied to: `elicit_uuid::Uuid` → `[cmp, display, from_str]`,
`elicit_url::Url` → `[cmp, display, from_str]`,
`elicit_serde_json::JsonValue` → `[eq, display]`,
`elicit_serde_json::JsonNumber` → `[eq, display]`.

### Unified plugin migration

All workflow plugins across `elicit_chrono`, `elicit_jiff`, `elicit_time`,
`elicit_url`, and `elicit_reqwest` now use `#[elicit_tool]` +
`#[derive(ElicitPlugin)]`. This eliminates ~200 lines of hand-written
descriptor and dispatch boilerplate across the workspace and ensures every
plugin participates in the emit registry automatically.

---

## Developer tooling

### `just check-features` — cargo-hack powerset (depth 2)

A new justfile recipe runs `cargo hack --feature-powerset --depth 2` across
the workspace (122 feature combinations), tees output to a timestamped log,
and fails with a clear message if any warnings appear. This surfaced and fixed
six latent dead-code bugs caused by mismatched feature gates:

- `use anodized::spec` compiled when URL/regex types were absent
- `kani_url_*` / `kani_uuid_*` proof helpers visible when their type feature
  was off (even though `proofs` was on via the `emit` dep chain)
- Unused import clusters in `datetime_specs.rs` and `http_specs.rs` when
  datetime or reqwest features were absent
- `dispatch_fetch_and_parse_emit` was a public function in a private module
  with no export — now correctly re-exported under `#[cfg(feature = "emit")]`

The recipe lives in `just pre-merge` (not `just ci` or daily commits — the
122-combo run is a pre-PR gate, not a per-commit tax).

---

## What's next

The contract-carrying param types plan (`CONTRACT_PARAMS_PLAN.md`) is queued:
constrained types (`UrlValid`, `F64Positive`, etc.) propagating their contracts
into the MCP tool descriptor so agents see the validation rules without reading
source code.
