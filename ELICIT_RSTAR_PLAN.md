# ELICIT_RSTAR_PLAN.md

## Goal

Add faithful `rstar` support in the same style as the recent GeoRust work:

1. **Core type integration** in `elicitation` for the concrete upstream value types that fit Phase 2 well.
2. **Shadow crate** `elicit_rstar` for practical tree construction and query operations over a concrete, serializable subset of the upstream API.
   Prefer a factory abstraction that exposes generic `RTree<T, Params>` behavior at runtime by letting the user select a supported `T`, then cutting custom tools for that instantiation.
3. **Proof wiring** for the new core `rstar` support.

## Upstream orientation

`rstar` is not a domain-specific UI crate. It is a generic spatial-index crate centered on:

- `rstar::RTree<T, Params = DefaultParams>`
- `rstar::AABB<P>`
- traits:
  - `rstar::RTreeObject`
  - `rstar::PointDistance`
  - `rstar::RTreeParams`
  - `rstar::SelectionFunction`
- primitives:
  - `rstar::primitives::Rectangle<P>`
  - `rstar::primitives::Line<P>`
  - `rstar::primitives::GeomWithData<R, T>`
  - `rstar::primitives::PointWithData<P, T>`
  - `rstar::primitives::CachedEnvelope<T>`

Important constraints from the real API:

- the crate is **highly generic**
- many useful operations require trait bounds like `RTreeObject` and `PointDistance`
- `RTree<T, Params>` itself is a **runtime data structure**, not a simple descriptor/value type
- `rstar` already interoperates with `geo-types`; we should reuse that instead of inventing a parallel geometry layer

## Scope correction

The old plan was too reshape-oriented.

For the initial landing:

- **do not invent** a UI-specific indexing abstraction as the primary API
- **do not start** with a bespoke `IndexedElement` model in core support
- **do not try** to make fully generic `RTree<T, Params>` a Phase 2 elicitation type
- **do** prefer concrete upstream types and concrete tree specializations that remain faithful to the crate

This follows the same correction we already made for `geojson` and `georaster`: shadow the upstream crate first, then add consumer-specific integrations later.

## Recommended implementation shape

## Phase 1: Workspace wiring

### Files

- `Cargo.toml`
- `crates/elicitation/Cargo.toml`

### Changes

1. Add workspace dependency:
   - `rstar = "0.12"`
2. Add workspace member:
   - `crates/elicit_rstar`
3. Add workspace dependency entry:
   - `elicit_rstar = { path = "crates/elicit_rstar", version = "0.10.0" }`
4. Add `elicitation` optional dependency:
   - `rstar = { workspace = true, optional = true }`
5. Add `elicitation` feature:
   - `rstar-types = ["dep:rstar", "geo-types"]`
6. Add `rstar-types` to `full`

Use the `-types` suffix for consistency with the rest of this workspace.

## Phase 2: Core support in `elicitation`

### Files

- `crates/elicitation/src/primitives/rstar_types/` (new module directory)
- `crates/elicitation/src/primitives/mod.rs`
- `crates/elicitation/src/type_spec/rstar_specs.rs`
- `crates/elicitation/src/type_spec/mod.rs`
- `crates/elicitation/src/lib.rs`
- `crates/elicitation/tests/rstar_types_test.rs`

### Core strategy

Because `rstar` is generic, Phase 2 should target the concrete value types that are:

- structurally meaningful,
- serializable enough for elicitation,
- useful as building blocks for the shadow crate.

Recommended initial surface:

1. `rstar::AABB<[f64; 2]>`
2. `rstar::primitives::Rectangle<[f64; 2]>`
3. `rstar::primitives::Line<[f64; 2]>`

These are the best first candidates because they are:

- faithful upstream types,
- spatially meaningful on their own,
- concrete enough to support `Elicitation`, `PromptTree`, `TypeSpec`, and proof methods.

### Important constraint

Direct upstream `rstar` types cannot themselves satisfy `ElicitComplete` here,
because `ElicitComplete` requires `schemars::JsonSchema` and Rust orphan rules
forbid implementing that external trait for external `rstar` types in this crate.

The intended solution is the **trenchcoat pattern**:

- wrap the upstream type in a local newtype,
- implement the elicitation/schema/codegen/proof traits on the wrapper,
- implement the needed `rstar` traits on the wrapper where appropriate,
- convert back to the upstream type at the boundary.

That keeps the design faithful to the upstream crate while still enabling fully
`ElicitComplete` factory inputs.

### Explicit non-goals for Phase 2

- generic `RTree<T, Params>` support
- trait object support for `RTreeObject` / `PointDistance`
- consumer-specific wrappers like `IndexedElement`
- aggressive support for `GeomWithData<R, T>` until we decide on a concrete data type strategy

### Notes

- arrays like `[f64; 2]` should be reused directly if already supported by `elicitation`
- if `AABB<[f64; 2]>` proves awkward to introspect directly, keep the support concrete and explicit rather than broadening into generic envelopes

## Phase 3: `elicit_rstar` shadow crate

### Files

- `crates/elicit_rstar/Cargo.toml`
- `crates/elicit_rstar/src/lib.rs`
- `crates/elicit_rstar/src/`
- `crates/elicit_rstar/tests/`

### Shadow-crate strategy

The shadow crate should expose the real generic tree API through a **factory-driven runtime abstraction**.

Recommended shape:

1. The user selects a supported reflected `T`.
2. A factory cuts a custom tool surface for `RTree<T, Params = DefaultParams>`.
3. Available tools depend on trait capabilities:
   - **`T: RTreeObject` factory**
      - construction
      - bulk load
      - insert
      - iteration snapshots
      - envelope queries
   - **`T: RTreeObject + PointDistance` factory**
      - all of the above
      - nearest-neighbor queries
      - within-distance queries

This keeps the runtime surface faithful to upstream generics without collapsing the design to a few narrow hardcoded specializations.

Concrete tree instantiations such as `RTree<[f64; 2]>` or `RTree<Rectangle<[f64; 2]>>` can still be used internally for validation and early test coverage, but they should not be the primary public design.

The intended trait seam remains:

- base factory: `T: ElicitComplete + RTreeObject`
- richer factory: `T: ElicitComplete + RTreeObject + PointDistance`

For the built-in `rstar` primitives, this means the factory runtime should use
the trenchcoat wrappers as the concrete `T` choices.

### Landed in the first shadow-crate pass

- `crates/elicit_rstar` now exists as a workspace crate.
- The public runtime wrapper is `RstarTree<T>`, a serializable snapshot over the
  logical contents of `RTree<T>`.
- Two manual dynamic factories are implemented:
  - `RTreeObjectFactory`
  - `PointDistanceFactory`
- The current factory seam is generic over:
  - `T: ElicitComplete + RTreeObject<Envelope = rstar::AABB<[f64; 2]>>`
  - `T: ElicitComplete + RTreeObject<Envelope = rstar::AABB<[f64; 2]>> + PointDistance`
- The richer `PointDistanceFactory` emits the full base tool surface plus its
  distance-aware tools, because `DynamicToolRegistry::instantiate` replaces all
  previously-instantiated tools for the same prefix.
- Current built-in concrete `T` choices are the trenchcoat wrappers:
  - `elicitation::RstarRectangle`
  - `elicitation::RstarLine`
- Validation is green for:
  - `just check elicit_rstar`
  - `just test-package elicit_rstar`
  - `just check-all elicit_rstar`

### Plugin split

Keep the plugin design simple and faithful:

1. **Factory plugin**
   - enumerate supported `T`
   - create `RTreeObject`-capable factories
   - create `PointDistance`-capable factories
2. **Construction plugin**
   - new / bulk_load
   - insert / remove
   - size / iter snapshots
3. **Query plugin**
   - locate_in_envelope
   - locate_in_envelope_intersecting
   - point and nearest-neighbor queries when `T` supports `PointDistance`
4. **Drain / mutation plugin** if needed

Do not over-split before the concrete tool surface is clear.

## Phase 4: Proof wiring

### Files

- `crates/elicitation_kani/src/rstar_types.rs`
- `crates/elicitation_creusot/src/rstar_types.rs`
- `crates/elicitation_verus/src/rstar_types.rs`
- verification runner files in `crates/elicitation/src/verification/`
- `crates/elicitation/tests/proof_non_empty_test.rs`

### Proof strategy

Follow the lightweight structural pattern used for the other GeoRust integrations:

- **Kani**: concrete round-trip / field-preservation checks for `AABB`, `Rectangle`, and `Line`
- **Creusot**: trusted structural wrappers
- **Verus**: lightweight shadow structs proving the same shape-level invariants

The goal is composition-friendly coverage of the new core support, not verification of rstar’s internal tree algorithms.

### Landed

- `crates/elicitation_kani/src/rstar_types.rs`
- `crates/elicitation_creusot/src/rstar_types.rs`
- `crates/elicitation_verus/src/rstar_types.rs`
- runner registration in:
  - `crates/elicitation/src/verification/runner.rs`
  - `crates/elicitation/src/verification/creusot_runner.rs`
  - `crates/elicitation/src/verification/verus_runner.rs`
- `rstar-types` feature wiring in:
  - `crates/elicitation_kani/Cargo.toml`
  - `crates/elicitation_creusot/Cargo.toml`

Validated with:

1. `cargo check -p elicitation --features rstar-types`
2. `cargo test -p elicitation --features rstar-types --test proof_non_empty_test`
3. `cargo check -p elicitation_kani --features 'kani,rstar-types'`
4. `cargo check -p elicitation_creusot --features 'rstar-types'`
5. `cargo check --manifest-path crates/elicitation_verus/Cargo.toml`

## Validation plan

When implementation starts, validate in this order:

1. `cargo check -p elicitation --features rstar-types`
2. `cargo test -p elicitation --features rstar-types --test rstar_types_test`
3. `cargo test -p elicitation --features rstar-types --test proof_non_empty_test`
4. `just check elicit_rstar`
5. `just test-package elicit_rstar`
6. `just check-all elicit_rstar`
7. `cargo check -p elicitation_kani --features 'kani,rstar-types'`
8. `cargo check -p elicitation_creusot --features 'rstar-types'`
9. `cargo check --manifest-path crates/elicitation_verus/Cargo.toml`

## Suggested execution order

1. Refresh workspace and `elicitation` feature wiring.
2. Land `rstar` core support for `AABB<[f64; 2]>`, `Rectangle<[f64; 2]>`, and `Line<[f64; 2]>`.
3. Add targeted tests and proof non-empty coverage.
4. Create `elicit_rstar` around factory-driven runtime exposure of `RTree<T, Params>` for supported reflected `T`.
5. Add Kani / Creusot / Verus proof modules and runner wiring.
6. Run the full validation set.

## Open questions

1. Whether `GeomWithData<R, T>` should be in scope for the first shadow-crate pass or deferred until the basic tree/query surface lands.
2. Whether `Line<[f64; 2]>` belongs in the initial core surface or should wait until the tree/query runtime is in place.
3. Whether any immediate `elicit_ui` consumer integration is warranted, or whether that should remain a follow-on task after faithful `rstar` support lands.
