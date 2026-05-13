# `elicitation generate` CLI Feature

> **Status:** Planning — supersedes the Verus Phase 4 / build.rs migration work.

---

## The Problem

`elicit_proofs/build.rs` calls `Machine::vsm_kani_proof()` / `vsm_verus_proof()` at
build time. This requires all machine deps (`elicit_server` → tokio/sqlx/axum) to
compile under the target toolchain. For Verus, those deps **cannot** compile under
vargo → stuck. The segregation breaks down.

The fix is to move proof generation OUT of `build.rs` and into a CLI subcommand that
uses **syn source scanning** rather than runtime trait dispatch. The CLI is normal
Rust; it can read source files without compiling them. The generated files become
toolchain-independent artifacts.

---

## Vision

```
elicitation generate kani    --crate crates/elicit_server
elicitation generate verus   --crate crates/elicit_server
elicitation generate creusot --crate crates/elicit_server
elicitation generate all     --crate crates/elicit_server
```

Users (and us) never write `build.rs` for proof generation again. They annotate their
VSMs normally; the CLI handles the rest.

---

## Discovery: how does the CLI find VSMs?

The CLI walks `.rs` files under the given crate path using `syn`. It looks for:

1. `#[derive(VerifiedStateMachine)] #[vsm(transitions = [t1, t2, ...])]` →
   `VsmDescriptor { machine, state, invariant, transitions }`
2. `#[derive(Prop)] #[prop(kani_invariant_fn = "...", verus_invariant_fn = "...")]` →
   invariant metadata (fn names, and — see below — fn bodies)
3. The state enum definition → variant shapes for inline type stubs

All of this is already present in the source as syn-parseable tokens.

---

## The Invariant Body Problem

The current approach has the invariant body in **two** separate places:
- `#[cfg(kani)] pub fn archive_connection_consistent(...)` in the VSM source
- `VERUS_INV_*` string constants in `elicit_proofs/build.rs`

For the CLI to generate proof files **without** calling trait methods, the invariant
body must be co-located with the annotation the CLI scans.

### Option A: `verus_inv_body` attribute on `#[prop]` ← **recommended**

```rust
#[derive(Prop)]
#[prop(
    credential = ConnectionEstablished,
    kani_invariant_fn  = "archive_connection_consistent",
    verus_invariant_fn = "archive_connection_consistent",
    verus_inv_body     = "true",
)]
pub struct ArchiveConnectionConsistent;
```

The body lives as a string literal, co-located with the Prop declaration.
CLI reads it with syn, emits it verbatim into the generated `spec fn`.

**Pros:** Single source of truth, trivially parsed by syn, easy to see invariant
alongside credential.

**Cons:** Verus syntax in a Rust string literal — no IDE support for the body.
Complex invariants (multi-arm matches) are awkward as one-liners.

### Option B: Infer body from existing `#[cfg(kani)] fn`

The Kani invariant fn already has the body as real Rust:
```rust
#[cfg(kani)]
pub fn archive_connection_consistent(_state: &ArchiveConnectionState) -> bool {
    true
}
```

CLI finds this fn (same name as `kani_invariant_fn`), extracts the body as a
TokenStream, substitutes for Verus. Works for simple cases; breaks for patterns
using Kani-specific APIs.

**Pros:** No new annotations — DRY.

**Cons:** Requires body-level translation (Rust bool → Verus spec syntax). Not
reliable for complex invariants.

### Option C: Separate `*.verus_spec.rs` sidecar file per VSM

A file `connection.verus_spec.rs` contains only `spec fn` bodies in real Verus
syntax. CLI includes it verbatim in generated output.

**Pros:** Full Verus syntax support (IDE, editor, vargo-check).

**Cons:** Another file per VSM to maintain; less ergonomic.

**Recommendation:** Start with Option A for proof-of-concept, migrate to Option C
once the generated pipeline is established (spec files can be vargo-checked directly).

---

## The Inline Type Stub Problem (Verus only)

Verus can't compile `elicit_server`, so the generated `.rs` file can't
`use elicit_server::...`. It needs the state enum and referenced domain types
defined inline.

The CLI scans:
1. The state enum → inline its variants
2. Referenced types in variant fields (e.g. `BackendKind`, `DatabaseDescriptor`) →
   generate minimal ZST stubs or opaque enums
3. Only inline what the **invariant body actually touches** (minimize stubs)

For `verus_inv_body = "true"`, zero field-level stubs are needed — only the
enum variant names.

For `archive_panel_consistent`: `SqlEditor { running, result, .. }` →
need `running: bool` and `result: Option<_>` fields inlined.

**Sub-options:**
- **A4 (recommended):** Parse the invariant body with syn, collect referenced field
  identifiers, inline only those fields (with their types). Generate opaque stubs for
  any types that can't be directly inlined.
- **A1:** Walk the full type dependency graph and inline everything. Authoritative
  but generates large files with many stubs.

---

## Architecture: New modules

```
crates/elicitation/src/cli/
├── generate/
│   ├── mod.rs           — GenerateCommand, dispatch, --crate / --out args
│   ├── scanner.rs       — syn walker, VsmDescriptor, PropDescriptor
│   ├── kani_gen.rs      — Kani harness emitter (replaces build.rs logic)
│   ├── verus_gen.rs     — Verus inline-stub + spec fn + assume_spec emitter
│   └── creusot_gen.rs   — Creusot companion emitter
```

`VsmDescriptor` (scanner output):
```rust
pub struct VsmDescriptor {
    pub machine:          Ident,           // ArchiveConnectionMachine
    pub state:            Ident,           // ArchiveConnectionState
    pub invariant:        Ident,           // ArchiveConnectionConsistent
    pub transitions:      Vec<Ident>,      // [begin_connect_sql, ...]
    pub inv_kani_fn:      String,          // "archive_connection_consistent"
    pub inv_verus_fn:     String,          // "archive_connection_consistent"
    pub inv_verus_body:   Option<String>,  // from verus_inv_body attr
    pub state_variants:   Vec<VariantDesc>, // from enum scan
}
```

---

## CLI command additions

Add to `Commands` enum in `cli.rs`:
```rust
/// Generate formal verification proof files from VSM source annotations
Generate {
    #[command(subcommand)]
    target: GenerateTarget,
    /// Root of the crate to scan
    #[arg(long, default_value = ".")]
    crate_path: PathBuf,
    /// Output directory (default: verifier-specific crate's src/vsm/)
    #[arg(long)]
    out: Option<PathBuf>,
},
```

```rust
pub enum GenerateTarget { Kani, Verus, Creusot, All }
```

---

## What happens to `elicit_proofs/build.rs`?

**Transition path:**
1. Phase 1–2: CLI generates files; build.rs still runs as fallback.
2. Phase 3: Remove Verus generation from build.rs (it can't work anyway).
3. Phase 4: Remove Kani generation from build.rs once CLI output is validated.
4. Long-term: `elicit_proofs/build.rs` is empty or deleted.

For now, Verus VSM files go to `elicitation_verus/src/vsm/` (already planned).
For Kani, files continue to `elicit_proofs/src/kani/generated/` until transition.

---

## Justfile recipes

```just
# Generate all proof files for elicit_server
generate-vsm-proofs:
    elicitation generate all --crate-path crates/elicit_server

# Generate only Verus (fast, no CBMC run needed)
generate-verus-vsm:
    elicitation generate verus --crate-path crates/elicit_server \
        --out crates/elicitation_verus/src/vsm

verify-verus-vsm:
    cd crates/elicitation_verus && vargo build
```

---

## Implementation Phases

**Phase 1: Scanner (VsmDescriptor)**
- Add `walkdir` dep to elicitation (cli feature)
- Implement `scanner.rs`: find `#[derive(VerifiedStateMachine)]`, parse `#[vsm(...)]`
- Find companion `#[derive(Prop)] #[prop(...)]` for each machine's invariant type
- Unit-test: scan elicit_server VSMs, assert 4 machines found with correct transitions

**Phase 2: Kani generator**
- Implement `kani_gen.rs`: emit same TokenStreams as `vsm_kani_proof()` but from
  VsmDescriptor rather than runtime trait call
- Validate: diff output against current `elicit_proofs/src/kani/generated/`
- Add `elicitation generate kani` command
- Add `generate-kani-vsm` justfile recipe

**Phase 3: Verus generator + `verus_inv_body` attribute**
- Add `verus_inv_body` key to `#[prop]` attribute parser in `derive_prop.rs`
- Annotate all 4 archive `*Consistent` structs with their bodies
- Implement `verus_gen.rs`: inline state enum, emit spec fn, emit assume_specification
- Add `elicitation generate verus` command
- Remove `elicit_server` dep from `elicitation_verus` (delete the half-written build.rs)
- `just verify-verus-vsm` end-to-end green

**Phase 4: Creusot generator**
- Implement `creusot_gen.rs`
- Add `creusot_inv_body` to `#[prop]`

**Phase 5: Remove `elicit_proofs/build.rs`**
- Once Kani output is validated, remove build.rs generation
- `elicit_proofs` becomes a thin re-export crate or is merged into generated crates

---

## Open Questions

1. **Option A vs B vs C** for invariant body — attribute string vs infer from kani fn
   vs sidecar file?
2. **Stub depth for Verus** — only inline invariant-touched fields (A4) or full enum
   (A1)?
3. **Kani transition first or Verus first?** Kani lets us validate the scanner without
   Verus toolchain risk; Verus is the actual blocker.
4. **Where is `verus_inv_body` parsed?** `derive_prop.rs` proc-macro (ignored at
   expansion time, read by CLI) or just a doc-attribute the CLI reads from source?

---

## Archive VSM Invariant Bodies (for Phase 3)

| Machine | Invariant type | Body |
|---------|---------------|------|
| `ArchiveConnectionMachine` | `ArchiveConnectionConsistent` | `true` |
| `ArchiveNavMachine`        | `ArchiveNavConsistent`        | `NavFiltered { filter, .. } => filter@.len() > 0` |
| `ArchiveOverlayMachine`    | `ArchiveOverlayConsistent`    | `ExportPickerOpen { idx, formats } => idx <= formats@.len()` |
| `ArchivePanelMachine`      | `ArchivePanelConsistent`      | `SqlEditor { running, result, .. } => *running ==> result.is_None()` |
