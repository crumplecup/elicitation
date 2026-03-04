# EmitCode — Code Recovery Plan

## The Core Idea

MCP tool composition **is** Rust. Every time an agent chains
`parse_and_focus → validate_object → pointer_update`, they have authored a
verified Rust program without knowing it. `EmitCode` recovers that program —
losslessly, ceremony intact.

The emitted binary:
- Calls our library's verified API directly (not reimplementing logic)
- Preserves full typestate: `RawJson`, `ParsedJson`, `Established<P>`, all of it
- Compiles against our workspace crates as dependencies
- Inherits formal verification by virtue of using verified building blocks

This is **not** code generation. It is **code recovery**.

---

## What We Are NOT Doing

- Not re-implementing fetch logic raw (no `reqwest::get()` in emitted code)
- Not stripping contract types or proof tokens
- Not synthesizing new logic
- Not auto-deriving `EmitCode` via proc-macro (too much coupling between
  signature and implementation — hand-write impls for workflow tools)

---

## Architecture

```
elicitation
  └── EmitCode trait        ← new: the recovery contract
  └── CrateDep type         ← new: dependency descriptor

elicit_reqwest
  └── workflow.rs           ← new: EmitCode impls per typestate transition

elicit_serde_json
  └── workflow.rs           ← new: EmitCode impls per typestate transition

elicit_bin (new crate)
  ├── emit_code.rs          ← re-exports EmitCode, CrateDep
  ├── scaffold.rs           ← wraps emitted steps in tokio main + tracing init
  ├── artifact.rs           ← writes main.rs + Cargo.toml, invokes cargo build
  └── plugin.rs             ← MCP plugin: emit_binary tool
```

---

## Phase 1: `EmitCode` trait in `elicitation`

Add to `crates/elicitation/src/`:

```rust
/// A type that knows how to recover itself as Rust source code.
pub trait EmitCode {
    /// Emit the Rust token stream for this step's logic.
    /// The emitted code runs in an async context with `?` available.
    fn emit_code(&self) -> proc_macro2::TokenStream;

    /// Crate dependencies required by the emitted code.
    fn crate_deps(&self) -> Vec<CrateDep> { vec![] }
}

/// A Cargo dependency descriptor with pinned version from our workspace.
pub struct CrateDep {
    pub name: &'static str,
    pub version: &'static str,
    pub features: Vec<&'static str>,
}
```

Add `proc-macro2` to `elicitation` non-dev dependencies (already in workspace).

Export `EmitCode`, `CrateDep` from `elicitation::lib.rs`.

---

## Phase 2: `EmitCode` impls for workflow typestate

### In `elicit_serde_json/src/workflow.rs`

Each params struct gets a hand-written `EmitCode` impl that emits the
typestate sequence it wraps. Example — `ParseAndFocusParams`:

```rust
impl EmitCode for ParseAndFocusParams {
    fn emit_code(&self) -> TokenStream {
        let json = &self.json;
        let pointer = &self.pointer;
        quote! {
            let raw = elicit_serde_json::RawJson::new(#json.to_string());
            let (parsed, json_proof) = raw.parse()
                .map_err(|e| format!("JSON parse failed: {}", e))?;
            let (focused, _focus_proof) = parsed.focus(#pointer, json_proof)
                .map_err(|e| format!("Pointer resolution failed: {}", e))?;
            let value = focused.extract();
            println!("{}", value);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![CrateDep { name: "elicit_serde_json", version: "0.8", features: vec![] }]
    }
}
```

One impl per workflow tool params struct. Five in `elicit_serde_json`, several
in `elicit_reqwest`.

### In `elicit_reqwest/src/plugins/workflow.rs`

Same pattern — `FetchParams`, `FetchAuthParams`, `PostJsonParams`, etc. each
emit the reqwest typestate sequence they wrap.

---

## Phase 3: `elicit_bin` crate

New crate: `crates/elicit_bin/`

### `scaffold.rs`

```rust
pub struct BinaryScaffold {
    pub steps: Vec<Box<dyn EmitCode>>,
    pub with_tracing: bool,
}

impl BinaryScaffold {
    pub fn render(&self) -> proc_macro2::TokenStream {
        let steps: Vec<_> = self.steps.iter().map(|s| s.emit_code()).collect();
        let tracing_init = self.with_tracing.then(|| quote! {
            tracing_subscriber::fmt::init();
        });
        quote! {
            #[tokio::main]
            async fn main() -> Result<(), Box<dyn std::error::Error>> {
                #tracing_init
                #( #steps )*
                Ok(())
            }
        }
    }

    pub fn all_deps(&self) -> Vec<CrateDep> {
        // collect + deduplicate deps from all steps
    }
}
```

### `artifact.rs`

```rust
/// Write main.rs + Cargo.toml to disk, run cargo build --release.
pub fn emit_to_disk(scaffold: &BinaryScaffold, output_dir: &Path) -> Result<(), EmitError>;
pub fn compile(dir: &Path) -> Result<PathBuf, CompileError>;
```

`Cargo.toml` generation pins exact versions (no `"*"`). Version strings live
as constants in each crate's `CrateDep` impls — we know the versions because
they are our workspace.

Use `prettyplease` (already a common Rust ecosystem dep) to format emitted
source before writing — makes the recovered code readable.

### `plugin.rs` — the MCP tool

```rust
#[derive(Deserialize, JsonSchema)]
struct EmitBinaryParams {
    /// Ordered list of tool calls to recover as a binary.
    steps: Vec<WorkflowStep>,
    /// Include tracing_subscriber init in main().
    with_tracing: bool,
    /// Output directory path for the generated project.
    output_dir: String,
}

#[derive(Deserialize, JsonSchema)]
struct WorkflowStep {
    tool: String,          // e.g. "parse_and_focus"
    params: serde_json::Value,
}
```

The plugin dispatches `tool` names to their params-struct deserializers, calls
`emit_code()`, assembles the scaffold, writes to disk, compiles, returns binary
path.

---

## Phase 4: Session recording tool (optional but powerful)

Add a `RecordPlugin` to `elicit_server` with two tools:

- `record_step(tool, params)` — accumulates steps in a session buffer
- `emit_recorded()` — recovers whatever was recorded this session

This gives agents a "record mode": they call tools normally, then say "emit
what I just did as a binary." Zero extra params required from the agent.

---

## Crate placement

- No new `elicit_bin` crate.
- `EmitCode` trait + `CrateDep` + `BinaryScaffold` + artifact pipeline → `elicitation`
  behind `feature = "emit"`
- `EmitCode` impls for workflow tools → each existing crate owns its own
- `EmitBinaryPlugin` + `RecordPlugin` MCP tools → `elicit_server` (planned crate,
  needs cross-crate knowledge)

---

## Checklist

### Phase 1 — `EmitCode` trait in `elicitation`

- [ ] Add `proc-macro2` + `quote` + `prettyplease` to `elicitation` Cargo.toml
      non-dev deps (behind `feature = "emit"`)
- [ ] Create `crates/elicitation/src/emit_code.rs`
  - [ ] `pub trait EmitCode` with `emit_code(&self) -> TokenStream` +
        `crate_deps(&self) -> Vec<CrateDep>`
  - [ ] `pub struct CrateDep { name, version, features }`
- [ ] Blanket impl: `impl<T: quote::ToTokens> EmitCode for T` — covers all
      primitives that already impl `ToTokens` (bool, numerics, char, &str, String)
- [ ] Export `EmitCode`, `CrateDep` from `lib.rs` under `#[cfg(feature = "emit")]`

### Phase 2 — primitive + std type impls (in `elicitation`)

These live in `crates/elicitation/src/emit_code.rs` or alongside each primitive module.
Blanket covers most; write specific impls for types that don't impl `ToTokens`.

- [ ] `bool`, `char` — covered by blanket
- [ ] `i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`,
      `usize`, `isize` — covered by blanket
- [ ] `f32`, `f64` — covered by blanket
- [ ] `String`, `&str` — covered by blanket
- [ ] `Vec<T: EmitCode>` — specific impl, emits `vec![...]`
- [ ] `Option<T: EmitCode>` — specific impl, emits `Some(...)` or `None`
- [ ] `PathBuf` — specific impl, emits `std::path::PathBuf::from("...")`
- [ ] `Duration` — specific impl, emits `std::time::Duration::from_secs(n)`
- [ ] Tuples `(A, B)`, `(A, B, C)`, `(A, B, C, D)` — macro stamp

### Phase 3 — feature-gated type impls (in `elicitation`)

Each behind matching `#[cfg(feature = "...")]`:

- [ ] `uuid::Uuid` (`feature = "uuid"`) — emits `uuid::Uuid::parse_str("...").unwrap()`
- [ ] `url::Url` (`feature = "url"`) — emits `url::Url::parse("...").unwrap()`
- [ ] `chrono::DateTime<Utc>` (`feature = "chrono"`) — emits `chrono::DateTime::parse_from_rfc3339(...)`
- [ ] `time::OffsetDateTime` (`feature = "time"`) — emits `time::OffsetDateTime::parse(...)`
- [ ] `jiff::Timestamp` (`feature = "jiff"`) — emits `jiff::Timestamp::from_str(...)`
- [ ] `serde_json::Value` (`feature = "serde_json"`) — emits `serde_json::json!(...)`
- [ ] `std::net::IpAddr`, `Ipv4Addr`, `Ipv6Addr` — emits `"...".parse().unwrap()`
- [ ] `reqwest::StatusCode` (`feature = "reqwest"`) — emits `reqwest::StatusCode::from_u16(n).unwrap()`

### Phase 4 — `BinaryScaffold` + artifact pipeline (in `elicitation`, `feature = "emit"`)

- [ ] `scaffold.rs`: `BinaryScaffold { steps: Vec<Box<dyn EmitCode>>, with_tracing: bool }`
  - [ ] `render() -> TokenStream` — wraps steps in `#[tokio::main] async fn main()`
  - [ ] `all_deps() -> Vec<CrateDep>` — collects + deduplicates deps across all steps
  - [ ] `to_source() -> String` — calls `render()` then `prettyplease::unparse()`
- [ ] `artifact.rs`: disk write + `cargo build --release` invocation
  - [ ] `emit_to_disk(scaffold, output_dir) -> Result<(), EmitError>`
  - [ ] `compile(dir) -> Result<PathBuf, CompileError>` — runs `cargo build --release`,
        captures stderr on failure, returns binary path on success
  - [ ] `Cargo.toml` generation with pinned versions from `CrateDep`

### Phase 5 — `EmitCode` impls for workflow params (in each crate)

**`elicit_serde_json/src/workflow.rs`** (5 impls):
- [ ] `ParseFocusParams` → emits `RawJson::new → .parse() → .focus() → .extract()`
- [ ] `ValidateObjectParams` → emits `RawJson → .parse() → .assert_object() → .validate_required()`
- [ ] `MergeParams` → emits two `RawJson` parse chains + `ObjectJson::merge(both(...))`
- [ ] `PointerUpdateParams` → emits `RawJson → .parse() → set_pointer()`
- [ ] `FieldChainParams` → emits the iterative focus loop

**`elicit_reqwest/src/plugins/workflow.rs`** (8 impls):
- [ ] `FetchParams` → emits `WorkflowPlugin::client → .fetch() typestate chain`
- [ ] `AuthFetchParams` → emits bearer/basic/api-key auth chain
- [ ] `PostParams` → emits POST body chain
- [ ] `ApiCallParams` → emits generic API call chain
- [ ] `HealthCheckParams` → emits health check chain
- [ ] `UrlBuildParams` → emits URL construction
- [ ] `StatusSummaryParams` → emits status inspection chain
- [ ] `BuildRequestParams` → emits request builder chain
- [ ] `PaginatedGetParams` → emits pagination loop

### Phase 6 — `EmitBinaryPlugin` MCP tool (in `elicit_server`, future crate)

- [ ] `EmitBinaryPlugin` — `emit_binary(steps, with_tracing, output_dir)` tool
  - [ ] Dispatches `WorkflowStep { tool, params }` to each crate's params deserializer
  - [ ] Calls `emit_code()` on each, assembles `BinaryScaffold`, writes + compiles
  - [ ] Returns binary path or compile error with stderr
- [ ] `RecordPlugin` — session capture tools
  - [ ] `record_step(tool, params)` — accumulates into session buffer
  - [ ] `emit_recorded(output_dir)` — recovers entire session as binary

---

## What the recovered code looks like

Agent composed: `parse_and_focus("/name") → validate_object(["name", "age"])`

Recovered `main.rs` (after `prettyplease` formatting):

```rust
use elicitation::contracts::*;
use elicit_serde_json::{RawJson, JsonParsed, IsObject, RequiredKeysPresent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Step 1: parse_and_focus
    let raw = elicit_serde_json::RawJson::new(r#"{"name":"Alice","age":30}"#.to_string());
    let (parsed, json_proof) = raw.parse()
        .map_err(|e| format!("JSON parse failed: {}", e))?;
    let (focused, _focus_proof) = parsed.focus("/name", json_proof)
        .map_err(|e| format!("Pointer resolution failed: {}", e))?;
    let value = focused.extract();

    // Step 2: validate_object
    let raw2 = elicit_serde_json::RawJson::new(value.to_string());
    let (parsed2, proof2) = raw2.parse()
        .map_err(|e| format!("JSON parse failed: {}", e))?;
    let (obj, obj_proof) = parsed2.assert_object(proof2)
        .map_err(|e| format!("Not an object: {}", e))?;
    let (_validated, _val_proof) = obj.validate_required(&["name", "age"], obj_proof)
        .map_err(|e| format!("Missing required keys: {}", e))?;

    println!("Validated successfully");
    Ok(())
}
```

Proof tokens preserved. Typestate preserved. Formally verified by virtue of
calling the same action trait impls. A Rust developer can read, audit, extend,
and ship this directly.

---

## Key design decisions

- `EmitCode` in `elicitation` — downstream crates impl it without circular deps
- Blanket `impl<T: ToTokens> EmitCode for T` — gets primitives for free
- Specific impls for workflow params are hand-written — macro can't infer the
  typestate sequence from a struct signature alone
- Version pins live next to `CrateDep` impls in each crate — no central registry
- `prettyplease` for formatting — recovered source is human-readable
- MCP plugin deferred to `elicit_server` — it's the only crate with visibility
  across all workflow crates simultaneously
