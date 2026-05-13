# elicit_doc — Implementation Plan

**Name:** `elicit_doc` (play on "doc" as documentation and doctor)

**What it is:** A standalone companion repo/crate for static analysis of the
elicitation workspace. Uses `cargo rustdoc --output-format json` as its data
source to inventory crate API surfaces and run coverage/drift analyses.

---

## Problem Statement

The elicitation workspace contains many shadow crates (`elicit_bevy`,
`elicit_wgpu`, etc.) whose job is to mirror every public type, method, trait,
and macro from their upstream targets. We have no automated way to:

1. Measure what percentage of a target's public API is actually covered by its
   shadow crate
2. Detect name drift when upstream cuts a new release (renames, removals)
3. Know which types in `elicitation` still lack `ElicitComplete` coverage

`elicit_doc` solves all three by treating compiled rustdoc JSON as ground truth.

---

## Architecture

Standalone crate — separate repo, not in the elicitation workspace. Added to
the workspace as a git dev-dependency / tool once stable.

```
elicit_doc/
  src/
    lib.rs          — mod + pub use only
    error.rs        — ElicitDocError + ErrorKind (derive_more)
    inventory.rs    — Inventory, Item, ItemKind (data model)
    collect.rs      — invoke cargo rustdoc, parse JSON → Inventory
    compare.rs      — CoverageReport, DriftReport (analysis)
    report.rs       — text/JSON/CSV output formatting
    cli.rs          — CLI subcommands (feature = "cli")
  Cargo.toml
  justfile
```

### Key Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `rustdoc-types` | 0.57 | Official rustdoc JSON schema types |
| `serde_json` | 1 | Deserialize rustdoc JSON |
| `cargo_metadata` | 0.23 | Locate crate source and target dirs |
| `derive_more` | 2 | Error types |
| `clap` | 4 | CLI (optional, `cli` feature) |

No semver dependency needed — the identifier/comparison algebra from semver
is not applicable to API surface diffing. `String` is the right representation
for symbol names.

---

## Data Model (`inventory.rs`)

```rust
pub struct Inventory {
    pub crate_name: String,
    pub crate_version: String,
    pub items: Vec<Item>,
}

pub struct Item {
    pub path: Vec<String>,       // ["bevy", "render", "Color"]
    pub kind: ItemKind,
    pub name: String,
    pub docs: Option<String>,
    pub feature_gate: Option<String>,
}

pub enum ItemKind {
    Struct,
    Enum,
    Trait,
    TypeAlias,
    Method { self_ty: String },
    Function,
    Macro,
    Constant,
    Module,
}
```

---

## Phase 1 — Foundation

- Create repo, Cargo.toml, justfile matching workspace conventions
- `error.rs`: `ElicitDocError` + `ElicitDocErrorKind` with `derive_more`
- `inventory.rs`: data model above; all types `ElicitComplete`-ready
- `lib.rs`: mod + pub use only
- `justfile`: `check`, `test-package`, `check-all` recipes

**Done when:** `just check-all` passes on an empty-but-compiling skeleton.

---

## Phase 2 — Collection Layer (`collect.rs`)

```rust
pub fn collect(crate_path: &Path, features: &[&str]) -> Result<Inventory>
```

Steps:
1. Run `cargo rustdoc --output-format json -p <crate>` via `std::process::Command`
2. Locate the generated `target/doc/<crate>.json` via `cargo_metadata`
3. Parse with `serde_json::from_str::<rustdoc_types::Crate>(...)`
4. Walk `index` (the item map): map each `rustdoc_types::ItemEnum` variant to
   our `ItemKind`
5. Resolve full paths via the `paths` map in the JSON

The rustdoc JSON gives us: full public surface, trait impls, associated types,
feature gates, and doc comments — everything needed for all three analyses.

**Toolchain note:** `rustdoc-types` version must match the rustdoc that
generates the JSON. Pinning to stable toolchain is recommended; document the
required toolchain version in the README.

**Done when:** `collect("crates/elicitation")` returns a non-empty `Inventory`
with a reasonable item count.

---

## Phase 3 — Analysis Layer (`compare.rs`)

### Coverage Report

```rust
pub struct CoverageReport {
    pub target_crate: String,
    pub shadow_crate: String,
    pub covered: Vec<Item>,       // present in both
    pub missing: Vec<Item>,       // in target, absent from shadow
    pub extra: Vec<Item>,         // in shadow but not in target
    pub drifted: Vec<DriftPair>,  // probable renames
    pub coverage_pct: f32,
}

pub struct DriftPair {
    pub target_item: Item,
    pub shadow_item: Item,
    pub confidence: f32,
}
```

### Name Drift Detection

Normalize both sides to `snake_case`, strip common prefixes (`Bevy`, `Wgpu`,
`Elicit`, etc.), then fuzzy-match by edit distance or token overlap. Pairs
above a confidence threshold surface as `drifted` rather than `missing`/`extra`.

**Done when:** `compare(&bevy_inventory, &elicit_bevy_inventory)` produces a
plausible coverage percentage and correctly identifies a few known-missing
items.

---

## Phase 4 — ElicitComplete Coverage

Special analysis path (no new external dependencies):

1. Collect the `elicitation` crate inventory
2. Walk all `Struct` and `Enum` items
3. For each, check if any `impl` in their `impls` list resolves to the
   `ElicitComplete` trait
4. Output a three-state table:
   - `✅ complete` — implements `ElicitComplete`
   - `❌ missing` — public type with no `ElicitComplete` impl
   - `⚠️ partial` — implements some but not all required sub-traits

This answers "what types in elicitation still need work" directly from the
compiled artifact — no source scanning needed, no heuristics.

**Done when:** running against the current `elicitation` crate returns a
table that matches manual inspection of a sample.

---

## Phase 5 — CLI (`cli.rs`, feature = `"cli"`)

```
elicit_doc check-shadow --target bevy --shadow elicit_bevy [--features "..."]
elicit_doc check-shadow --target wgpu  --shadow elicit_wgpu
elicit_doc check-complete --crate elicitation
elicit_doc drift --crate bevy --from 0.14 --to 0.15
elicit_doc report --format [text|json|csv] --input report.json
```

Output formats:
- **text** — human-readable table (default)
- **json** — machine-readable for CI consumption
- **csv** — spreadsheet-friendly for tracking over time

**Done when:** `elicit_doc check-shadow --target bevy --shadow elicit_bevy`
prints a coverage table for the full bevy surface.

---

## Phase 6 — Workspace Integration

Add recipes to the elicitation workspace `justfile`:

```makefile
doc-coverage crate="elicit_bevy" target="bevy":
    cargo run --manifest-path ../elicit_doc/Cargo.toml --features cli \
        -- check-shadow --target {{target}} --shadow {{crate}}

doc-complete:
    cargo run --manifest-path ../elicit_doc/Cargo.toml --features cli \
        -- check-complete --crate elicitation

doc-drift crate target from to:
    cargo run --manifest-path ../elicit_doc/Cargo.toml --features cli \
        -- drift --crate {{crate}} --target {{target}} --from {{from}} --to {{to}}
```

Wire `doc-coverage` into the pre-release checklist once it's stable enough to
be a gate.

---

## Open Questions for Review

1. **Repo home:** Separate GitHub repo under `crumplecup/elicit_doc`, or a
   subdirectory of the elicitation monorepo? Separate repo keeps toolchain
   conflicts isolated; monorepo is simpler to iterate.

2. **Rustdoc toolchain pinning:** Stable is safer but may miss nightly-only
   items. Worth a `rust-toolchain.toml` that pins to a specific stable release?

3. **ElicitComplete partial detection:** The "partial" category requires knowing
   which sub-traits make up `ElicitComplete`. Worth encoding that as a config
   file so it stays in sync as `ElicitComplete` evolves?

4. **Name drift thresholds:** What confidence level should promote a pair from
   "missing" to "drifted"? Needs tuning against real data.

5. **Publishing:** Publish to crates.io eventually for other shadow crate
   authors, or keep it a private tool?
