# ELICIT_GEORASTER_PLAN.md

## Goal

Add `georaster` support using the same pattern that succeeded for `geo_types`, `wkt`,
`wkb`, and `geojson`:

1. **Shadow the real upstream API** instead of inventing a new raster model.
2. **Land practical core support in `elicitation`** for upstream value types that fit
   the Phase 2 vocabulary.
3. **Create `elicit_georaster`** as the runtime shadow crate with MCP workflow tools.
4. **Wire Kani / Creusot / Verus proofs** for the new surface.

The architectural constraint is the same as the rest of this GeoRust track:
**shadow crates must shadow, not reshape**.

---

## Upstream Orientation

The current `georaster` crate is much smaller than the old plan assumed.

### Public API surface observed in docs.rs

At crate root:

- `georaster::Coordinate` (re-export)

Modules:

- `georaster::geo`
- `georaster::geotiff`

`georaster::geotiff` public types:

- `GeoTiffReader<R: Read + Seek>`
- `ImageInfo`
- `Pixels<'a, R: Read + Seek>`
- `RasterValue`

### Key behavioral observations

- `georaster` is currently a **GeoTIFF / COG reader crate**, not a general raster
  algebra framework.
- `GeoTiffReader` exposes reading and coordinate-conversion methods:
  - `open`
  - `images`
  - `image_info`
  - `seek_to_image`
  - `origin`
  - `pixel_size`
  - `select_raster_band`
  - `read_pixel`
  - `read_pixel_at_location`
  - `pixels`
  - `coord_to_pixel`
  - `pixel_to_coord`
- `RasterValue` is a `#[non_exhaustive]` enum covering scalar and RGB/RGBA pixel values.
- `ImageInfo` is metadata for the current TIFF image/IFD.
- `Pixels` is an iterator over `(x, y, RasterValue)` triples.

### Immediate implication

The older plan was wrong in scope. It proposed:

- custom `Raster<T>` and `MultiBandRaster<T>` types
- custom `GeoTransform`
- `ndarray`-backed raster algebra
- terrain analysis, reprojection, and resampling tools

Those are **not** the upstream `georaster` surface. They should not be the starting
point for `elicit_georaster`.

---

## Scope Decision

`elicit_georaster` should target the **actual upstream reader vocabulary** first:

1. `RasterValue`
2. `ImageInfo`
3. `GeoTiffReader`
4. `Pixels`
5. `Coordinate` interop where appropriate

Anything like raster algebra, statistics, hillshade, or resampling belongs in:

- a later phase only if upstream `georaster` grows those capabilities, or
- a different crate entirely

This keeps the shadow crate faithful and prevents repeating the GeoJSON mistake of
designing an aspirational abstraction that diverges from the source crate.

---

## Phase 1: Workspace Wiring

### Files

- `Cargo.toml`
- `crates/elicitation/Cargo.toml`

### Changes

1. Add `georaster` as a workspace dependency.
2. Add `crates/elicit_georaster` as a workspace member.
3. Add `elicit_georaster` as a workspace dependency.
4. Add a feature-gated `georaster-types` path in `crates/elicitation/Cargo.toml`.
5. Decide whether `georaster-types` should roll into an existing geo meta-feature or
   remain independent initially. The safer default is **independent first**.

### Notes

- Do **not** add `ndarray` just to satisfy the old plan.
- Do **not** add `gdal` or writing support unless required by the real upstream crate
  or by a later explicitly scoped extension.

---

## Phase 2: Core Support in `elicitation`

### Target

Add Phase 2 support only for the upstream types that make sense as stable value
descriptors.

### Likely in-scope

- `georaster::geotiff::RasterValue`
- `georaster::geotiff::ImageInfo`

### Likely out-of-scope for Phase 2

- `GeoTiffReader<R>` because it is generic over an I/O source and represents runtime
  state rather than a small serializable value object
- `Pixels<'a, R>` because it is an iterator borrowing a live reader

### Expected file layout

- `crates/elicitation/src/primitives/georaster_types/`
  - `mod.rs`
  - `raster_value.rs`
  - `image_info.rs`
  - helpers as needed
- `crates/elicitation/src/type_spec/georaster_specs.rs`
- feature/export wiring in `crates/elicitation/src/lib.rs`

### Design notes

- Preserve upstream names and variant structure.
- Treat `RasterValue` as the core pixel-value alphabet.
- Reuse existing coordinate vocabulary where possible instead of inventing a second
  geometry foundation.
- Be careful with `#[non_exhaustive]` behavior when mirroring `RasterValue`.

---

## Phase 3: `elicit_georaster` Shadow Crate

### Goal

Create `crates/elicit_georaster/` as a faithful wrapper crate around the
reader-oriented `georaster` API.

### Core wrapper surface

- `GeoTiffReader`
- `ImageInfo`
- `RasterValue`
- possibly a wrapper for pixel iterator output if useful for MCP transport

### Reader strategy

The main technical wrinkle is that upstream `GeoTiffReader` is generic:

```rust
GeoTiffReader<R: Read + Seek>
```

For MCP/runtime use, the wrapper should likely standardize on an owned backing store,
for example:

- `std::io::Cursor<Vec<u8>>` for byte-backed readers

This is a runtime-storage decision, not a public API redesign. The wrapper should still
mirror the upstream method vocabulary and semantics.

### Plugin shape

Prefer a small set of upstream-shaped workflow plugins such as:

1. **Reader plugin**
   - open from bytes/path
   - inspect images
   - seek image
   - select band
2. **Sampling plugin**
   - `read_pixel`
   - `read_pixel_at_location`
   - `coord_to_pixel`
   - `pixel_to_coord`
3. **Metadata/iteration plugin**
   - `origin`
   - `pixel_size`
   - image metadata access
   - collect pixel windows from `pixels(...)`

### Explicit non-goals for first delivery

- invented `Raster<T>` construction API
- generalized raster algebra
- terrain-analysis helpers
- reprojection/resampling framework
- database trait support

---

## Phase 4: Verification

### Kani

Cover lightweight structural properties such as:

- `RasterValue` variant preservation
- `ImageInfo` metadata field roundtrips / invariants
- non-empty proof inventory wiring

### Creusot

Use trusted, structural proofs only. Focus on:

- constructor/accessor consistency for value types
- simple metadata properties

### Verus

Follow the same lightweight-shadow approach used for the other third-party crates:

- shadow structs/enums for proof-only structural claims
- avoid proving live I/O behavior

### Runner/test wiring

Update:

- `crates/elicitation/src/verification/runner.rs`
- `crates/elicitation/src/verification/creusot_runner.rs`
- `crates/elicitation/src/verification/verus_runner.rs`
- `crates/elicitation/tests/proof_non_empty_test.rs`

---

## Validation Targets

As phases land, validate with the same pattern used elsewhere:

### Core

```bash
cargo check -p elicitation --features georaster-types
cargo test -p elicitation --features georaster-types --test georaster_types_test
```

### Shadow crate

```bash
just check elicit_georaster
just test-package elicit_georaster
just check-all elicit_georaster
```

### Proof crates

```bash
cargo check -p elicitation_kani --features 'kani,georaster-types'
cargo check -p elicitation_creusot --features 'georaster-types'
cargo check --manifest-path crates/elicitation_verus/Cargo.toml
```

---

## Implementation Order

1. Rewrite planning/docs around the real `georaster` surface.
2. Add workspace + `elicitation` feature wiring.
3. Land Phase 2 support for `RasterValue` and `ImageInfo`.
4. Create `crates/elicit_georaster`.
5. Add upstream-shaped reader/sampling plugins.
6. Add proof modules and runner wiring.
7. Run package- and proof-level validation.

---

## Open Questions

1. Should `Coordinate` support be treated as pure reuse of existing geo coordinate
   support, or should `elicit_georaster` expose an explicit conversion layer for
   clarity?
2. Should `pixels(...)` be surfaced as:
   - a collected window result type for MCP ergonomics, or
   - a stricter iterator-oriented wrapper with explicit materialization tools?
3. Do we want to support opening only from bytes initially, or both bytes and
   filesystem paths in the first shadow-crate cut?

The default implementation assumption should be:

- reuse existing geo coordinate vocabulary,
- expose a collected pixel-window result for MCP tools,
- support both bytes and path entrypoints if that can be done without reshaping the
  underlying reader semantics.
