# ELICIT_WKB_PLAN.md

## Goal

Implement `elicit_wkb` as a **faithful shadow crate** for the upstream `wkb` 0.9.x API, following the same rule established for `elicit_wkt`: **shadow, do not reshape**.

This plan is intentionally centered on the **actual public API** of `wkb` 0.9.1 rather than on invented wrapper types or made-up plugin groupings.

## Upstream API to Shadow

The relevant stable public surface is:

- `wkb::Endianness`
- `wkb::error::WkbError`
- `wkb::reader::{Wkb<'a>, Dimension, GeometryType, read_wkb}`
- `wkb::writer::{WriteOptions, write_geometry, write_point, write_line_string, write_polygon, write_multi_point, write_multi_line_string, write_multi_polygon, write_geometry_collection, write_rect, write_triangle, write_line}`
- `wkb::writer::{geometry_wkb_size, point_wkb_size, line_string_wkb_size, polygon_wkb_size, multi_point_wkb_size, multi_line_string_wkb_size, multi_polygon_wkb_size, geometry_collection_wkb_size, rect_wkb_size, triangle_wkb_size, line_wkb_size}`

Important upstream details:

- `wkb::reader::Wkb<'a>` is an **opaque parsed reader view**, not a public enum/AST.
- `wkb::reader::read_wkb(buf)` is an alias for `Wkb::try_new(buf)`.
- `Wkb<'a>` publicly exposes at least:
  - `try_new(&[u8]) -> WkbResult<Wkb<'_>>`
  - `dimension(&self) -> Dimension`
  - `geometry_type(&self) -> GeometryType`
- `wkb` uses `geo_traits` for reading/writing, not a bespoke owned geometry model.
- `geozero` is relevant as a downstream integration story, but **the crate being shadowed here is `wkb`**.

## Design Constraints

1. **Mirror upstream names first.**
   - Keep `Endianness`, `WkbError`, `reader::Wkb`, `reader::GeometryType`, `writer::WriteOptions`, and the writer function names.
   - Do not invent a top-level `WkbGeometry` abstraction.

2. **Adapt only where MCP/serde boundaries force it.**
   - `reader::Wkb<'a>` cannot cross MCP as a borrowed opaque parser view.
   - The shadow crate should therefore keep the **public name and method semantics** while backing the wrapper with an owned validated byte buffer.
   - This is an implementation adaptation, not an API reshape.

3. **Do not flatten reader/writer into one synthetic API.**
   - Preserve the upstream module split:
     - `elicit_wkb::reader::*`
     - `elicit_wkb::writer::*`
     - `elicit_wkb::error::*`

4. **Prefer exact function names over “friendly” aliases.**
   - If upstream says `read_wkb`, `try_new`, `write_polygon`, `polygon_wkb_size`, use those names.

## Implementation Outline

## Phase 1: Workspace Wiring

### Files

- `Cargo.toml`
- `Cargo.lock`
- `crates/elicitation/Cargo.toml`

### Changes

1. Add `wkb = "0.9"` to workspace dependencies.
2. Add `crates/elicit_wkb` to workspace members.
3. Add `elicit_wkb` as a workspace dependency.
4. Add an `elicitation` feature for WKB support.

Recommended feature naming:

- `wkb-types` in `elicitation`

This should match the existing `wkt-types` pattern rather than using a generic `wkb` feature name.

## Phase 2: Phase-2 `elicitation` Primitives

Unlike `wkt`, the upstream `wkb` crate does **not** expose a public owned geometry AST. The Phase-2 layer should therefore only model the stable public pieces we actually need.

### New primitives

- `WkbBytes`
  - validated owned byte buffer for WKB payloads
  - builder-type role analogous to `WktString`
- `WkbEndianness`
  - mirrors `wkb::Endianness`
- `WkbDimension`
  - mirrors `wkb::reader::Dimension`
- `WkbGeometryType`
  - mirrors `wkb::reader::GeometryType`
- `WkbWriteOptions`
  - mirrors `wkb::writer::WriteOptions`

### Notes

- Do **not** invent `WkbGeometry`.
- Do **not** attempt to mirror the reader internals (`reader::Point<'a>`, etc.) unless they are publicly exported and stable enough to justify it.
- `WkbBytes` should validate via `wkb::reader::Wkb::try_new` / `wkb::reader::read_wkb`.

## Phase 3: `elicit_wkb` Crate Structure

Create `crates/elicit_wkb/` with upstream-shaped modules:

```text
crates/elicit_wkb/src/
├── lib.rs
├── error.rs
├── reader.rs
└── writer.rs
```

### `lib.rs`

Only:

- `mod error;`
- `mod reader;`
- `mod writer;`
- `pub use ...`

### `error.rs`

Shadow:

- `WkbError`

Because upstream `WkbError` is non-exhaustive and not directly MCP-friendly, the wrapper should preserve the upstream variant names where possible while remaining serializable and proof-friendly.

### `reader.rs`

Shadow:

- `Dimension`
- `GeometryType`
- `Wkb`
- `read_wkb`

`Wkb` wrapper design:

- public name remains `Wkb`
- constructor remains `try_new`
- `read_wkb` remains a top-level free function
- methods should include:
  - `dimension()`
  - `geometry_type()`

Implementation note:

- Back `Wkb` with owned validated bytes plus cached metadata needed to answer the public reader methods.
- Do not expose extra public methods unless they are strictly required for MCP transport and clearly marked as shadow-support internals.

### `writer.rs`

Shadow:

- `WriteOptions`
- all public `write_*` functions
- all public `*_wkb_size` functions

These functions should accept existing workspace geometry wrappers where practical:

- `elicitation::GeoPoint`
- `elicitation::GeoLineString`
- `elicitation::GeoPolygon`
- `elicitation::GeoMultiPoint`
- `elicitation::GeoMultiLineString`
- `elicitation::GeoMultiPolygon`
- `elicitation::GeoGeometryCollection`
- `elicitation::GeoRect`
- `elicitation::GeoLine`
- `elicitation::GeoTriangle`
- `elicitation::GeoGeometry`

The shadow function names must still remain the upstream names (`write_point`, `polygon_wkb_size`, etc.).

## Phase 4: Explicit Workflow Plugins

Add explicit plugins for the real upstream surfaces, not synthetic “WKB operations”.

### Proposed plugins

1. `WkbReaderPlugin`
   - namespace: `wkb_reader__*`
   - tools:
     - `read_wkb`
     - `wkb_try_new`
     - `wkb_dimension`
     - `wkb_geometry_type`

2. `WkbWriterPlugin`
   - namespace: `wkb_writer__*`
   - tools named after upstream write/size functions:
     - `write_geometry`
     - `write_point`
     - `write_line_string`
     - `write_polygon`
     - `write_multi_point`
     - `write_multi_line_string`
     - `write_multi_polygon`
     - `write_geometry_collection`
     - `write_rect`
     - `write_triangle`
     - `write_line`
     - `geometry_wkb_size`
     - `point_wkb_size`
     - `line_string_wkb_size`
     - `polygon_wkb_size`
     - `multi_point_wkb_size`
     - `multi_line_string_wkb_size`
     - `multi_polygon_wkb_size`
     - `geometry_collection_wkb_size`
     - `rect_wkb_size`
     - `triangle_wkb_size`
     - `line_wkb_size`

### Propositions

- `WkbParsed`
- `WkbWritten`

These should be the only new workflow propositions unless implementation reveals a real need for another one.

## Phase 5: Proofs

Add the same three proof layers used for WKT.

### `elicitation_kani`

Add `src/wkb_types.rs` for:

- `WkbEndianness` roundtrip
- `WkbWriteOptions` field preservation
- `WkbBytes` trusted parse success/failure boundaries
- `WkbGeometryType` preservation

### `elicitation_creusot`

Add `src/wkb_types.rs` with trusted wrapper-preservation style proofs for:

- endianness mapping
- validated bytes construction
- reader metadata preservation

### `elicitation_verus`

Add `src/wkb_types.rs` shadow proofs for:

- `WkbEndianness`
- `WkbDimension`
- `WkbGeometryType`
- `WkbWriteOptions`

### `elicitation` proof wiring

Update:

- `crates/elicitation/src/verification/runner.rs`
- `crates/elicitation/src/verification/creusot_runner.rs`
- `crates/elicitation/src/verification/verus_runner.rs`
- `crates/elicitation/tests/proof_non_empty_test.rs`

## Phase 6: Tests

### `crates/elicit_wkb/tests/`

Add:

- `smoke_test.rs`
  - parse valid bytes with `read_wkb`
  - inspect `dimension()` and `geometry_type()`
  - write a point/line/polygon to bytes
- `workflow_test.rs`
  - plugin creation
  - expected tool names
  - proposition proofs non-empty

### Validation target

At minimum:

```bash
just check elicit_wkb
just test-package elicit_wkb
just check-all elicit_wkb
```

And once the `elicitation` feature work lands:

```bash
cargo check -p elicitation --features wkb-types
cargo test -p elicitation --features wkb-types --test proof_non_empty_test
```

## Deferred Until After WKB Lands

- geozero/PostGIS/SQLx integration details
- `elicit_db` geo value storage work
- combined WKT/WKB storage strategy in `geo-db-consumer`

Those are consumer stories. The immediate task is a faithful `wkb` shadow.

## Success Criteria

`elicit_wkb` is considered complete when all of the following are true:

1. The public crate shape mirrors upstream `wkb` modules and names.
2. No synthetic `WkbGeometry` abstraction exists.
3. Reader and writer surfaces are available as explicit MCP tools.
4. `elicitation` has the minimal WKB Phase-2 primitives needed to support the shadow crate.
5. Kani, Creusot, Verus, and `proof_non_empty_test` wiring all pass.
6. `just check-all elicit_wkb` passes cleanly.
