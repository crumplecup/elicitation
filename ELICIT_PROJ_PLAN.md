# ELICIT_PROJ_PLAN.md

## Goal

Add faithful `proj` support in the same style as the recent GeoRust work:

1. **Core type integration** in `elicitation` for the concrete upstream value types that fit Phase 2 well.
2. **Shadow crate** `elicit_proj` for practical transformation, builder, and geometry workflows over the real upstream API.
3. **Proof wiring** for the new core `proj` support.

## Upstream orientation

`proj` is a Rust binding to the PROJ coordinate transformation library, centered on:

- `proj::Proj`
- `proj::ProjBuilder`
- `proj::Area`
- `proj::ProjError`
- `proj::Coord`
- `proj::Transform`

Important constraints from the real API:

- `Proj` and `ProjBuilder` are **stateful FFI-backed runtime objects**
- `Coord` is a **trait**, not a concrete data type
- `new_known_crs()` normalizes coordinate order to **Longitude, Latitude / Easting, Northing**
- the crate defaults to using an already-installed system `libproj`; `bundled_proj` is optional
- grid downloads / cache / remote endpoint controls are optional and mainly live behind builder/network APIs
- with the default `geo-types` feature, `Transform` applies directly to many `geo_types` geometries

Because PROJ is already installed locally in this environment, the initial plan should prefer the default **system-library** linkage path and avoid `bundled_proj` unless a later integration problem forces it.

## Scope correction

The old plan was too broad and too metadata-heavy too early.

For the initial landing:

- **do not start** with a speculative `~45 tool` surface
- **do not treat** EPSG search, geodesics, remote grids, and metadata browsing as the primary first pass
- **do not try** to make raw `proj::Proj` a Phase 2 elicitation type in `elicitation`
- **do** center the design on transformation objects and geometry conversion workflows
- **do** reuse existing `geo-types` support instead of inventing a parallel geometry substrate
- **do** model stateful PROJ objects in the shadow crate with serializable snapshots, as with `GeoTiffReader` and `RstarTree`

## Recommended implementation shape

## Phase 1: Workspace wiring

### Files

- `Cargo.toml`
- `crates/elicitation/Cargo.toml`

### Changes

1. Add workspace dependency for `proj`, using the default system-lib linking behavior first.
2. Add workspace member:
   - `crates/elicit_proj`
3. Add workspace dependency entry:
   - `elicit_proj = { path = "crates/elicit_proj", version = "0.10.0" }`
4. Add `elicitation` optional dependency:
   - `proj = { workspace = true, optional = true }`
5. Add `elicitation` feature:
   - `proj-types = ["dep:proj", "geo-types"]`
6. Add `proj-types` to `full`

Use the `-types` suffix for consistency with the rest of this workspace.

## Phase 2: Core support in `elicitation`

### Files

- `crates/elicitation/src/primitives/proj_types/` (new module directory)
- `crates/elicitation/src/primitives/mod.rs`
- `crates/elicitation/src/type_spec/proj_specs.rs`
- `crates/elicitation/src/type_spec/mod.rs`
- `crates/elicitation/src/lib.rs`
- `crates/elicitation/tests/proj_types_test.rs`

### Core strategy

Because the upstream crate is built around stateful transformation handles, Phase 2 should target the concrete value types that are:

- structurally meaningful,
- serializable enough for elicitation,
- useful as building blocks for the shadow crate.

Recommended initial surface:

1. `proj::Area`

Coordinates themselves can continue to use existing elicitation support such as:

- `[f64; 2]`
- `geo_types::{Coord, Point, LineString, Polygon, ...}` when the `geo-types` feature is active

### Important constraint

Raw upstream `proj::Proj` and `proj::ProjBuilder` are not good Phase 2 elicitation types:

- they are runtime FFI objects,
- they are not stable serializable descriptors,
- and they are better represented as shadow-crate snapshot wrappers.

The intended solution follows the same **trenchcoat / snapshot** approach we have used elsewhere:

- wrap the stateful upstream object in a local serializable snapshot type,
- reconstruct the runtime `Proj` / `ProjBuilder` object on demand,
- expose the wrapper in the shadow crate, not as a bare core value type.

### Explicit non-goals for Phase 2

- direct elicitation of raw `proj::Proj`
- direct elicitation of raw `proj::ProjBuilder`
- exhaustive CRS database browsing
- remote grid/network configuration as a required initial capability
- geodesic / ellipsoid analysis beyond what is directly needed for transformation workflows

### Landed in the first core pass

- workspace `proj` dependency using the default system-library-first linkage path
- `elicitation` `proj-types` feature wiring and `full` bundle integration
- `elicitation::ProjArea` trenchcoat wrapper over `proj::Area`
- conversions between `ProjArea` and upstream `proj::Area`
- `crates/elicitation/src/primitives/proj_types/`
- `crates/elicitation/src/type_spec/proj_specs.rs`
- `crates/elicitation/tests/proj_types_test.rs`
- `proof_non_empty_test` coverage for `ProjArea`
- `proof_composition_test` coverage asserting `ProjArea` delegates to `f64`

This keeps Phase 2 faithful while respecting the upstream trait surface: `proj::Area`
does not directly satisfy `ElicitComplete`, so the trenchcoat wrapper is the right
extension seam.

## Phase 3: `elicit_proj` shadow crate

### Files

- `crates/elicit_proj/Cargo.toml`
- `crates/elicit_proj/src/lib.rs`
- `crates/elicit_proj/src/`
- `crates/elicit_proj/tests/`

### Shadow-crate strategy

The shadow crate should expose the real PROJ transformation workflow through **serializable snapshot wrappers**.

Recommended shape:

1. A local `ProjTransform` snapshot wrapper over the inputs needed to recreate a `proj::Proj`
2. A local builder/config wrapper for the subset of `ProjBuilder` we choose to expose
3. Focus the first workflow surface on actual transformation work:
   - create from known CRS
   - create from PROJ strings / pipelines
   - optionally bind an area of use
   - convert one coordinate
   - convert arrays of coordinates
   - inspect definition / area of use / info fields that are already exposed upstream
4. Reuse `geo-types` interop for geometry transforms where the upstream `Transform` trait already supports them

### Proposed first-pass workflow split

Keep the plugin design simple and faithful:

1. **Transform plugin**
   - create from known CRS
   - create from PROJ string / pipeline
   - convert one coordinate
   - convert arrays
2. **Builder plugin**
   - configure search paths
   - configure area of use
   - optional network/cache controls if we decide to enable the upstream `network` feature
3. **Geo-types plugin**
   - transform point/coord/line/polygon values using the upstream trait support
   - prefer a local adapter/factory seam over a giant hand-written per-geometry matrix
4. **Info plugin** if the exposed upstream inspection methods justify it

Do not over-split before the concrete tool surface is clear.

### Initial shadow-crate boundaries

The first pass should prefer:

- **known-CRS transforms**
- **PROJ-string / pipeline transforms**
- **coordinate array conversion**
- **basic `geo-types` geometry transformation**

Defer unless the core surface proves insufficient:

- exhaustive EPSG search / listing
- remote grid downloading and cache management
- advanced CRS metadata browsing
- geodesic computations not directly exposed by the current upstream `proj` crate surface

## Phase 4: Proof wiring

### Files

- `crates/elicitation_kani/src/proj_types.rs`
- `crates/elicitation_creusot/src/proj_types.rs`
- `crates/elicitation_verus/src/proj_types.rs`
- verification runner files in `crates/elicitation/src/verification/`
- `crates/elicitation/tests/proof_non_empty_test.rs`

### Proof strategy

Follow the lightweight structural pattern used for the other GeoRust integrations:

- **Kani**: concrete field-preservation / round-trip checks for `proj::Area`
- **Creusot**: trusted structural wrappers
- **Verus**: lightweight shadow structs proving the same shape-level invariants

The goal is coverage of the new core support, not verification of PROJ’s internal algorithms or FFI behavior.

## Validation plan

When implementation starts, validate in this order:

1. `cargo check -p elicitation --features proj-types`
2. `cargo test -p elicitation --features proj-types --test proj_types_test`
3. `cargo test -p elicitation --features proj-types --test proof_non_empty_test`
4. `just check elicit_proj`
5. `just test-package elicit_proj`
6. `just check-all elicit_proj`
7. `cargo check -p elicitation_kani --features 'kani,proj-types'`
8. `cargo check -p elicitation_creusot --features 'proj-types'`
9. `cargo check --manifest-path crates/elicitation_verus/Cargo.toml`

## Suggested execution order

1. Refresh workspace and `elicitation` feature wiring.
2. Land minimal core `proj` support in `elicitation`, starting with `proj::Area`.
3. Add targeted tests and proof non-empty coverage.
4. Create `elicit_proj` around snapshot-based transformation workflows.
5. Add Kani / Creusot / Verus proof modules and runner wiring.
6. Run the full validation set.

## Open questions

1. Whether the first shadow-crate pass should include `ProjBuilder` network/cache controls, or defer them and stay entirely local/system-library first.
2. Whether `geo-types` transformation support belongs in the first pass or should follow immediately after the transform wrapper lands.
3. Whether any CRS metadata/info wrappers beyond `Area` are worth promoting into Phase 2 core support, or should remain shadow-crate concerns.
