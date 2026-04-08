# ELICIT_GEO_TYPES_PLAN.md

## Goal

Complete the geo-types elicitation vocabulary and ship `elicit_geo_types`:

1. **Expand core primitives** — 9 remaining geo-types types in `elicitation`
   (Coord/Line/Rect are already done)
2. **Shadow crate** — `elicit_geo_types` with newtypes, `reflect_methods`, and
   MCP tools for all geometric primitives

## Current State

The `geo-types` feature in `elicitation` already exists and covers three types:

| Wrapper | geo-types type | Status |
|---------|---------------|--------|
| `GeoCoord` | `Coord<f64>` | ✅ Done — Elicitation + ElicitIntrospect + ElicitSpec |
| `GeoLine` | `Line<f64>` | ✅ Done |
| `GeoRect` | `Rect<f64>` | ✅ Done |

Nine types remain:

| Wrapper (to add) | geo-types type | Composition |
|-----------------|---------------|-------------|
| `GeoPoint` | `Point<f64>` | Survey: one `GeoCoord` field (Point wraps Coord) |
| `GeoTriangle` | `Triangle<f64>` | Survey: three `GeoCoord` fields |
| `GeoLineString` | `LineString<f64>` | Delegates to `Vec<GeoCoord>::elicit()` |
| `GeoPolygon` | `Polygon<f64>` | Survey: exterior `GeoLineString` + `Vec<GeoLineString>` |
| `GeoMultiPoint` | `MultiPoint<f64>` | Delegates to `Vec<GeoPoint>::elicit()` |
| `GeoMultiLineString` | `MultiLineString<f64>` | Delegates to `Vec<GeoLineString>::elicit()` |
| `GeoMultiPolygon` | `MultiPolygon<f64>` | Delegates to `Vec<GeoPolygon>::elicit()` |
| `GeoGeometryCollection` | `GeometryCollection<f64>` | Delegates to `Vec<GeoGeometry>::elicit()` |
| `GeoGeometry` | `Geometry<f64>` | Select over all the above variants |

### Why Vec<T> makes this a slam dunk

`Vec<T: Elicitation>` already has a full `Elicitation` impl (bool-gated loop). This
means variable-length types like `LineString` (a `Vec<Coord>` under the hood),
`MultiPoint`, `MultiLineString`, `MultiPolygon`, and `GeometryCollection` all
**compose for free** once the leaf types are elicitable. `GeoPolygon` is a
two-field Survey (`exterior: GeoLineString`, `interiors: Vec<GeoLineString>`) —
nothing new needed. `GeoGeometry` is a Select over the concrete variants.

## Architecture

Follows the six-location checklist from `THIRD_PARTY_SUPPORT_GUIDE.md`.

```
Cargo.toml (workspace)          — add member + dep
crates/elicitation/             — Phase 2 first: 9 remaining primitives
crates/elicit_geo_types/        — Phase 3+: newtypes, reflect_methods, MCP tools
crates/elicitation_kani/        — Kani harnesses
crates/elicitation_creusot/     — Creusot proofs
crates/elicitation_verus/       — Verus proofs
```

## Phase 1: Workspace Wiring

### 1.1 `Cargo.toml` — new workspace member and dep

geo-types `0.7` is already in `[workspace.dependencies]`. Add:

```toml
# [workspace.members] — after "crates/elicit_jiff"
"crates/elicit_geo_types",

# [workspace.dependencies]
elicit_geo_types = { path = "crates/elicit_geo_types", version = "0.10.0" }
```

### 1.2 `crates/elicitation/Cargo.toml` — no change needed

`geo-types` is already an optional dep under the `geo-types` feature. The feature
is already excluded from `full` — add it:

```toml
full = ["chrono", "time", "jiff", "uuid", "url", "regex", "rand", "reqwest",
        "graph", "clap-types", "sqlx-types", "ratatui", "geo-types", "emit"]
```

## Phase 2: Core Elicitation Primitives (elicitation crate)

**Do this before the shadow crate.** The shadow crate delegates to these impls.

All nine types live in `crates/elicitation/src/primitives/geo_types/`, each in
its own module, following the exact pattern of `coord.rs` / `line.rs` / `rect.rs`.

### The ElicitComplete mechanism

`ElicitComplete` is the **compiler-enforced completion gate**: writing
`impl ElicitComplete for GeoFoo {}` only compiles when every supertrait bound is
satisfied — `Elicitation`, `ElicitIntrospect`, `ElicitSpec`, `ElicitPromptTree`,
`Serialize`, `Deserialize`, `JsonSchema`, and `ToCodeLiteral`. If anything is
missing, the compiler tells you exactly what.

**Implementation location:** `ElicitComplete` is NOT impl'd in the primitive
module. It lives in `crates/elicitation/src/type_spec/geo_specs.rs`, emitted by
the spec macro on the last line of each macro invocation:

```rust
// Inside impl_geo_composite_spec! — existing pattern:
inventory::submit!(TypeSpecInventoryKey::new(...));
impl ElicitComplete for $wrapper {}  // ← compiler validates ALL bounds here
```

**For each new type, the workflow is:**
1. Write the primitive module (`primitives/geo_types/foo.rs`) — all traits except
   `ElicitSpec` and `ElicitComplete`
2. Add the type to `geo_specs.rs` — the macro adds `ElicitSpec` + `ElicitComplete`
3. Attempt `cargo check -p elicitation --features geo-types` — fix every compile
   error the `ElicitComplete` impl surfaces
4. The compiler cannot lie: if it compiles, the type is genuinely complete

**Per-module checklist (for the primitive module itself):**
- Wrapper struct with `pub` fields (or pub accessors)
- `From<geo_types::T>` and `From<Wrapper> for geo_types::T`
- `default_style!` macro
- `impl Prompt`
- `impl Elicitation` (with `kani_proof`, `verus_proof`, `creusot_proof`)
- `impl ElicitIntrospect`
- `impl ElicitPromptTree`
- `impl ToCodeLiteral`
- (`ElicitSpec` + `ElicitComplete` go in `geo_specs.rs` — not here)

**For `GeoGeometry` (Select pattern):** needs a separate `impl_geo_select_spec!`
macro in `geo_specs.rs` (modelled on `egui_specs.rs` line ~24), which also ends
with `impl ElicitComplete for GeoGeometry {}`.

**Delegation types** (`GeoLineString`, `GeoMultiPoint`, etc.) still need
`impl_geo_composite_spec!` entries in `geo_specs.rs` even though their
elicitation delegates to `Vec`. The spec entries describe the delegation in the
`pattern` field (`"Vec delegation — repeated GeoCoord elicitation"`).

**Proof delegation pattern for newtypes over Vec:**

```rust
fn kani_proof() -> proc_macro2::TokenStream {
    Vec::<GeoCoord>::kani_proof()  // delegate to Vec<Inner>
}
```

This is the `kani_proof_contains::<GeoCoord>()` test assertion that should be
added to `tests/proof_composition_test.rs` for every delegation type.

### 2.1 `point.rs` — `GeoPoint`

Simple Survey: wraps one `GeoCoord`.

```rust
pub struct GeoPoint {
    pub coord: GeoCoord,
}

// elicit: GeoCoord::elicit(communicator).await? → wrap
```

### 2.2 `triangle.rs` — `GeoTriangle`

Survey over three named `GeoCoord` fields (`v1`, `v2`, `v3`).

### 2.3 `line_string.rs` — `GeoLineString`

Newtype over `Vec<GeoCoord>`. Elicitation delegates entirely:

```rust
pub struct GeoLineString(pub Vec<GeoCoord>);

async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
    let coords = Vec::<GeoCoord>::elicit(communicator).await?;
    Ok(Self(coords))
}
```

Proof methods delegate to `Vec::<GeoCoord>::kani_proof()` etc.

### 2.4 `polygon.rs` — `GeoPolygon`

Survey with two fields:

```rust
pub struct GeoPolygon {
    pub exterior: GeoLineString,
    pub interiors: Vec<GeoLineString>,
}
```

Both fields already elicitable — pure composition.

### 2.5 `multi_point.rs`, `multi_line_string.rs`, `multi_polygon.rs`

All three are newtypes over `Vec<Inner>`, following the same pattern as
`GeoLineString`. Elicitation and proofs delegate to `Vec`.

### 2.6 `geometry_collection.rs` — `GeoGeometryCollection`

Newtype over `Vec<GeoGeometry>` — depends on `GeoGeometry` (2.7), so implement last.

### 2.7 `geometry.rs` — `GeoGeometry`

Select over all concrete variants:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum GeoGeometry {
    Point(GeoPoint),
    Line(GeoLine),
    LineString(GeoLineString),
    Polygon(GeoPolygon),
    MultiPoint(GeoMultiPoint),
    MultiLineString(GeoMultiLineString),
    MultiPolygon(GeoMultiPolygon),
    Rect(GeoRect),
    Triangle(GeoTriangle),
    GeometryCollection(GeoGeometryCollection),
}
```

`impl Elicitation` uses `Select`: present the variant names, then recurse
into the chosen variant's elicitation.

### 2.8 `mod.rs` exports

```rust
mod geometry;
mod geometry_collection;
mod line_string;
mod multi_line_string;
mod multi_point;
mod multi_polygon;
mod point;
mod polygon;
mod triangle;

// existing
pub use coord::{GeoCoord, GeoCoordStyle};
pub use line::{GeoLine, GeoLineStyle};
pub use rect::{GeoRect, GeoRectStyle};

// new
pub use geometry::{GeoGeometry, GeoGeometryStyle};
pub use geometry_collection::{GeoGeometryCollection, GeoGeometryCollectionStyle};
pub use line_string::{GeoLineString, GeoLineStringStyle};
pub use multi_line_string::{GeoMultiLineString, GeoMultiLineStringStyle};
pub use multi_point::{GeoMultiPoint, GeoMultiPointStyle};
pub use multi_polygon::{GeoMultiPolygon, GeoMultiPolygonStyle};
pub use point::{GeoPoint, GeoPointStyle};
pub use polygon::{GeoPolygon, GeoPolygonStyle};
pub use triangle::{GeoTriangle, GeoTriangleStyle};
```

### 2.9 `geo_specs.rs` — extend existing specs

Three macro paths depending on type category:

1. **Survey structs** (`GeoPoint`, `GeoTriangle`, `GeoPolygon`, `GeoLine`, `GeoCoord`, `GeoRect`):
   use existing `impl_geo_composite_spec!` — one entry per type, lists fields.

2. **Vec-delegation newtypes** (`GeoLineString`, `GeoMultiPoint`, `GeoMultiLineString`,
   `GeoMultiPolygon`, `GeoGeometryCollection`): also use `impl_geo_composite_spec!` but
   with a single field entry describing the inner Vec element type, and `pattern` set to
   `"Vec delegation — repeated <Inner> elicitation"`.

3. **Select enum** (`GeoGeometry`): add a new `impl_geo_select_spec!` macro (model it on
   `egui_specs.rs` `impl_egui_select_spec!`), listing all 10 variants. Ends with
   `impl ElicitComplete for GeoGeometry {}`.

All three macro paths end with `impl ElicitComplete for $wrapper {}` — this is the
compiler gate that validates the full implementation.

### 2.10 `lib.rs` — add pub uses

Export all new wrapper types at the crate root under `#[cfg(feature = "geo-types")]`.

## Phase 3: Create `elicit_geo_types` Shadow Crate

### Directory structure

```
crates/elicit_geo_types/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── primitives.rs          — Point, Coord, Line, Triangle newtypes + reflect_methods
│   ├── shapes.rs              — Rect, Polygon newtypes + reflect_methods
│   ├── collections.rs         — LineString, Multi*, GeometryCollection + reflect_methods
│   ├── geometry.rs            — Geometry enum wrapper + reflect_methods
│   └── workflow/
│       ├── mod.rs
│       ├── primitives_plugin.rs  — ~8 tools: create_point, create_coord, create_line, create_triangle
│       ├── shapes_plugin.rs      — ~8 tools: create_rect, create_polygon, rect_*, polygon_*
│       ├── collections_plugin.rs — ~10 tools: create_line_string, create_multi_*, collection_*
│       └── geometry_plugin.rs    — ~6 tools: create_geometry, geometry_type, geometry_*
└── tests/
    └── geo_types_test.rs
```

### `Cargo.toml`

```toml
[package]
name = "elicit_geo_types"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled geo-types wrappers with MCP tools for geometric primitives"
keywords = ["mcp", "geo", "geometry", "spatial", "elicitation"]
categories = ["science::geo", "mathematics", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["geo-types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
geo-types.workspace = true
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true

proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

## Phase 4: Type Wrappers (`elicit_newtype!` + `reflect_methods`)

All wrappers follow the canonical pattern from `elicit_clap`. The shadow crate
newtypes wrap the `elicitation` wrapper types (not the raw geo-types), so
`From` impls compose through the chain: `geo_types::T → GeoT → ElicitGeoT`.

### `primitives.rs` — `Point`, `Coord`, `Line`, `Triangle`

```rust
use elicitation::{GeoCoord, GeoLine, GeoPoint, GeoTriangle};

elicit_newtype!(GeoPoint, as Point, serde);

#[reflect_methods]
impl Point {
    #[instrument]
    pub fn new(x: f64, y: f64) -> Self { /* ... */ }

    #[instrument(skip(self))]
    pub fn x(&self) -> f64 { self.0.coord.x }

    #[instrument(skip(self))]
    pub fn y(&self) -> f64 { self.0.coord.y }

    #[instrument(skip(self))]
    pub fn lng(&self) -> f64 { self.0.coord.x }

    #[instrument(skip(self))]
    pub fn lat(&self) -> f64 { self.0.coord.y }
}

// Similar for Coord, Line, Triangle
```

### `shapes.rs` — `Rect`, `Polygon`

```rust
elicit_newtype!(GeoRect, as Rect, serde);

#[reflect_methods]
impl Rect {
    #[instrument]
    pub fn new(min: Coord, max: Coord) -> Self { /* ... */ }

    #[instrument(skip(self))]
    pub fn min(&self) -> Coord { /* ... */ }

    #[instrument(skip(self))]
    pub fn max(&self) -> Coord { /* ... */ }

    #[instrument(skip(self))]
    pub fn width(&self) -> f64 { /* ... */ }

    #[instrument(skip(self))]
    pub fn height(&self) -> f64 { /* ... */ }

    #[instrument(skip(self))]
    pub fn center(&self) -> Coord { /* ... */ }
}

elicit_newtype!(GeoPolygon, as Polygon, serde);

#[reflect_methods]
impl Polygon {
    #[instrument]
    pub fn new(exterior: LineString, interiors: Vec<LineString>) -> Self { /* ... */ }

    #[instrument(skip(self))]
    pub fn exterior(&self) -> &LineString { /* ... */ }

    #[instrument(skip(self))]
    pub fn interiors(&self) -> &[LineString] { /* ... */ }
}
```

### `collections.rs` — `LineString`, `Multi*`, `GeometryCollection`

```rust
elicit_newtype!(GeoLineString, as LineString, serde);

#[reflect_methods]
impl LineString {
    #[instrument]
    pub fn new(coords: Vec<Coord>) -> Self { /* ... */ }

    #[instrument(skip(self))]
    pub fn coords_count(&self) -> usize { /* ... */ }

    #[instrument(skip(self))]
    pub fn is_closed(&self) -> bool { /* ... */ }

    #[instrument(skip(self))]
    pub fn points(&self) -> Vec<Point> { /* ... */ }
}

// MultiPoint, MultiLineString, MultiPolygon, GeometryCollection follow same pattern
```

## Phase 5: MCP Tools (~32 tools across 4 plugins)

### Plugin summary

| Plugin | Namespace | Tool count | Coverage |
|--------|-----------|------------|----------|
| `GeoTypesPrimitivesPlugin` | `geo_types_primitives__*` | 8 | Point, Coord, Line, Triangle create + accessors |
| `GeoTypesShapesPlugin` | `geo_types_shapes__*` | 8 | Rect, Polygon create + accessors |
| `GeoTypesCollectionsPlugin` | `geo_types_collections__*` | 10 | LineString, Multi*, Collection create + inspect |
| `GeoTypesGeometryPlugin` | `geo_types_geometry__*` | 6 | Geometry enum create, type-check, unwrap |

### Propositions (contracts)

```rust
pub struct PointCreated;
pub struct RectCreated;
pub struct PolygonCreated;
pub struct LineStringCreated;
pub struct GeometryCreated;
impl Prop for PointCreated {}
impl Prop for RectCreated {}
impl Prop for PolygonCreated {}
impl Prop for LineStringCreated {}
impl Prop for GeometryCreated {}
```

### Sample tool — `geo_types_primitives__create_point`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePointParams {
    /// X coordinate (longitude for geographic data).
    pub x: f64,
    /// Y coordinate (latitude for geographic data).
    pub y: f64,
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "create_point",
    description = "Create a Point<f64> from x, y coordinates. \
                   Establishes: PointCreated.",
    emit = Auto
)]
#[instrument]
async fn primitives_create_point(
    p: CreatePointParams,
) -> Result<CallToolResult, ErrorData> {
    let point = Point::new(p.x, p.y);
    Ok(CallToolResult::success(vec![
        Content::text(serde_json::to_string(&point).map_err(|e| {
            ErrorData::internal_error(e.to_string(), None)
        })?),
    ]))
}
```

## Phase 6: Formal Verification

### `elicitation_kani` — `geo_types.rs`

Harnesses verify:
- `GeoPoint` roundtrip: `geo_types::Point → GeoPoint → geo_types::Point`
- `GeoRect` constructor normalisation (min ≤ max guaranteed)
- `GeoLineString` delegation: proof composes from `Vec<GeoCoord>` harness
- `GeoGeometry` Select roundtrip: every variant label accepted by enum constructor

Pattern (trust the source, verify the wrapper):

```rust
#[cfg(feature = "ui-types")]
mod geo_types_proofs {
    use elicitation::{GeoPoint, GeoRect, GeoLineString};

    #[kani::proof]
    fn verify_geo_point_roundtrip() {
        let x: f64 = kani::any();
        let y: f64 = kani::any();
        kani::assume(x.is_finite() && y.is_finite());
        let raw = geo_types::Point::new(x, y);
        let wrapped = GeoPoint::from(raw);
        let back: geo_types::Point<f64> = wrapped.into();
        assert!((back.x() - x).abs() < f64::EPSILON);
        assert!((back.y() - y).abs() < f64::EPSILON);
    }
}
```

### `elicitation_creusot` — `geo_types.rs`

Trusted constructors + postconditions linking to logic accessors, following the
pattern established in `egui_types.rs`. Key: Creusot cannot construct foreign
types directly — use trusted wrapper functions with postconditions.

### `elicitation_verus` — `geo_types.rs`

Structural/boolean proofs (avoid f64 arithmetic in `ensures` — known limitation).
Verify: every `GeoGeometry` variant discriminant maps to a unique string label.

## Phase 7: Documentation

`crates/elicit_geo_types/README.md` covers:
- Purpose (geometry primitives alphabet, foundation for elicit_geo + elicit_geojson)
- Plugin/tool table
- Type hierarchy diagram
- Usage examples (construct, inspect, use with geo algorithms)
- Integration with `elicit_ui` (Rect ↔ layout bounds)

## File Checklist

### Create
- `crates/elicit_geo_types/Cargo.toml`
- `crates/elicit_geo_types/README.md`
- `crates/elicit_geo_types/src/lib.rs`
- `crates/elicit_geo_types/src/primitives.rs`
- `crates/elicit_geo_types/src/shapes.rs`
- `crates/elicit_geo_types/src/collections.rs`
- `crates/elicit_geo_types/src/geometry.rs`
- `crates/elicit_geo_types/src/workflow/mod.rs`
- `crates/elicit_geo_types/src/workflow/primitives_plugin.rs`
- `crates/elicit_geo_types/src/workflow/shapes_plugin.rs`
- `crates/elicit_geo_types/src/workflow/collections_plugin.rs`
- `crates/elicit_geo_types/src/workflow/geometry_plugin.rs`
- `crates/elicit_geo_types/tests/geo_types_test.rs`
- `crates/elicitation/src/primitives/geo_types/point.rs`
- `crates/elicitation/src/primitives/geo_types/triangle.rs`
- `crates/elicitation/src/primitives/geo_types/line_string.rs`
- `crates/elicitation/src/primitives/geo_types/polygon.rs`
- `crates/elicitation/src/primitives/geo_types/multi_point.rs`
- `crates/elicitation/src/primitives/geo_types/multi_line_string.rs`
- `crates/elicitation/src/primitives/geo_types/multi_polygon.rs`
- `crates/elicitation/src/primitives/geo_types/geometry_collection.rs`
- `crates/elicitation/src/primitives/geo_types/geometry.rs`

### Modify
- `Cargo.toml` — add workspace member + dep
- `crates/elicitation/Cargo.toml` — add `geo-types` to `full`
- `crates/elicitation/src/primitives/geo_types/mod.rs` — export 9 new types
- `crates/elicitation/src/type_spec/geo_specs.rs` — 9 new spec entries
- `crates/elicitation/src/lib.rs` — 9 new `pub use` under `geo-types` feature
- `crates/elicitation_kani/src/` — add `geo_types.rs` harnesses
- `crates/elicitation_creusot/src/` — add `geo_types.rs` proofs
- `crates/elicitation_verus/src/` — add `geo_types.rs` proofs
- `PLANNING_INDEX.md` — add this plan entry

## Implementation Order

1. Phase 1 — workspace wiring (~10 min)
2. Phase 2 — 9 elicitation primitives, leaf types first, enum last (~2 h)
3. Phase 3 — scaffold shadow crate structure (~20 min)
4. Phase 4 — newtypes + `reflect_methods` for all wrappers (~1 h)
5. Phase 5 — ~32 MCP tools across 4 plugins (~2–3 h)
6. Phase 6 — Kani/Creusot/Verus harnesses (~1 h)
7. Phase 7 — README + docs (~30 min)

`just check -p elicitation --features geo-types` after Phase 2.
`just check-all elicit_geo_types` after Phase 5.
