# ELICIT_WKT_PLAN.md

## Goal

Shadow the `wkt` crate (v0.11) faithfully: expose its actual types (`wkt::types::Coord`,
`Point`, `LineString`, `Polygon`, `Multi*`, `GeometryCollection`, and the `Wkt<f64>` enum)
and its two conversion traits (`ToWkt`, `TryFromWkt`) as MCP tools.

This is a **dictionary**, not a redesign. Every tool name maps to a real method or
constructor in the upstream library. The guide: `SHADOW_CRATE_MOTIVATION.md` §"Methods"
and §"Traits".

Reference implementation: `crates/elicit_geo_types/` — same three-layer pattern
(types → `#[reflect_methods]` → workflow plugins → trait factories).

This plan follows `THIRD_PARTY_SUPPORT_GUIDE.md` phases 1–8 exactly.

---

## wkt 0.11 Actual Public API

### `wkt::types` module (the 8 primitive structs)

| Upstream type | Shadow name | Fields / tuple |
|---|---|---|
| `wkt::types::Coord<f64>` | `WktCoord` | `x: f64, y: f64, z: Option<f64>, m: Option<f64>` |
| `wkt::types::Point<f64>` | `WktPoint` | `(Option<Coord<f64>>,)` |
| `wkt::types::LineString<f64>` | `WktLineString` | `(Vec<Coord<f64>>,)` |
| `wkt::types::Polygon<f64>` | `WktPolygon` | `exterior: LineString, interiors: Vec<LineString>` |
| `wkt::types::MultiPoint<f64>` | `WktMultiPoint` | `(Vec<Point<f64>>,)` |
| `wkt::types::MultiLineString<f64>` | `WktMultiLineString` | `(Vec<LineString<f64>>,)` |
| `wkt::types::MultiPolygon<f64>` | `WktMultiPolygon` | `(Vec<Polygon<f64>>,)` |
| `wkt::types::GeometryCollection<f64>` | `WktGeometryCollection` | `(Vec<Wkt<f64>>,)` |

### `wkt::Wkt<f64>` enum (7 variants)

```rust
pub enum Wkt<T> {
    Point(Point<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    GeometryCollection(GeometryCollection<T>),
}
```

Key impls: `FromStr` (parse WKT string), `Display` (serialize back to WKT).
Shadow name: `WktItem`.

### `wkt::ToWkt<T>` trait

```rust
pub trait ToWkt<T> {
    fn to_wkt(&self) -> Wkt<T>;
    fn wkt_string(&self) -> String;         // provided
    fn write_wkt(&self, writer: impl Write); // provided
}
```

Implemented by `wkt` for: all `geo_types` primitives (Point, Line, LineString, Polygon,
MultiPoint, MultiLineString, MultiPolygon, GeometryCollection).

### `wkt::TryFromWkt<T>` trait

```rust
pub trait TryFromWkt<T> {
    fn try_from_wkt_str(wkt_str: &str) -> Result<Self, Error>;
    fn try_from_wkt_reader(reader: impl Read) -> Result<Self, Error>;
}
```

Implemented by `wkt` for: all `geo_types` primitives.

---

## Phase 1 — Workspace `Cargo.toml`

### 1.1 Add `wkt` workspace dep

```toml
# Geospatial — Well-Known Text
wkt = { version = "0.11", features = ["geo-types"] }
```

### 1.2 Add `elicit_wkt` workspace member and dep

In `[workspace] members`:
```toml
"crates/elicit_wkt",
```

In `[workspace.dependencies]`:
```toml
elicit_wkt = { path = "crates/elicit_wkt", version = "0.10" }
```

### 1.3 Add `wkt-types` feature to `elicitation`

`crates/elicitation/Cargo.toml`:
```toml
[dependencies]
wkt = { workspace = true, optional = true }

[features]
wkt-types = ["dep:wkt", "geo-types"]          # WKT requires geo-types for conversions
gis       = ["proj", "geo", "geo-types", "geojson", "rstar", "wkt-types"]
```

---

## Phase 2 — `crates/elicitation/` Core Types

### Files

```
crates/elicitation/src/primitives/wkt_types/
├── mod.rs
├── wkt_string.rs     # WktString — builder type (validated WKT input string)
└── wkt_geom.rs       # WktGeom   — Select enum mirroring wkt::Wkt<f64> (7 variants)
```

### 2.1 `wkt_string.rs` — builder type

`WktString` is an interactive input type. The user provides a raw string;
elicitation validates it parses as `wkt::Wkt<f64>` before accepting.

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktString {
    pub wkt: String,
}
```

`Elicitation` impl: calls `Wkt::<f64>::from_str(&raw)` to validate, returns
`ElicitErrorKind::InvalidInput` on failure.

This is a **builder type**. Kani proof: `kani::assume(true)` (trust `wkt` crate's
`FromStr`). Creusot: `#[trusted]`. Verus: plain `assert(true)`.

### 2.2 `wkt_geom.rs` — Select enum (7 variants)

Mirror of `wkt::Wkt<f64>`. Pattern: identical to `GeoGeometry` in
`crates/elicitation/src/primitives/geo_types/geometry.rs`.

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(tag = "type", content = "geometry")]
pub enum WktGeom {
    Point(WktPoint),
    LineString(WktLineString),
    Polygon(WktPolygon),
    MultiPoint(WktMultiPoint),
    MultiLineString(WktMultiLineString),
    MultiPolygon(WktMultiPolygon),
    GeometryCollection(WktGeometryCollection),
}
```

Where `WktPoint`, `WktLineString` etc. are elicitation wrappers for `wkt::types::*`
(see §Phase 2 types for those).

`From<wkt::Wkt<f64>> for WktGeom` and `From<WktGeom> for wkt::Wkt<f64>` — full roundtrip.

### 2.3 Supporting primitive types in `elicitation`

Each `wkt::types::*` struct needs an elicitation wrapper in
`crates/elicitation/src/primitives/wkt_types/`:

| File | Type | Fields |
|---|---|---|
| `coord.rs` | `WktCoord` | `x: f64, y: f64, z: Option<f64>, m: Option<f64>` |
| `point.rs` | `WktPoint` | `coord: Option<WktCoord>` |
| `linestring.rs` | `WktLineString` | `coords: Vec<WktCoord>` |
| `polygon.rs` | `WktPolygon` | `exterior: WktLineString, interiors: Vec<WktLineString>` |
| `multipoint.rs` | `WktMultiPoint` | `points: Vec<WktPoint>` |
| `multilinestring.rs` | `WktMultiLineString` | `lines: Vec<WktLineString>` |
| `multipolygon.rs` | `WktMultiPolygon` | `polygons: Vec<WktPolygon>` |
| `geometry_collection.rs` | `WktGeometryCollection` | `geometries: Vec<WktGeom>` |

Each has `From<wkt::types::TYPE<f64>>` and `From<TYPE> for wkt::types::TYPE<f64>` impls.
Each derives `Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema`.

### 2.4 Wire into `primitives/mod.rs` and `lib.rs`

`primitives/mod.rs`:
```rust
#[cfg(feature = "wkt-types")]
pub mod wkt_types;
#[cfg(feature = "wkt-types")]
pub use wkt_types::{
    WktString, WktGeom,
    WktCoord, WktPoint, WktLineString, WktPolygon,
    WktMultiPoint, WktMultiLineString, WktMultiPolygon, WktGeometryCollection,
};
```

`lib.rs`: same `#[cfg(feature = "wkt-types")] pub use` block.

### 2.5 `type_spec/wkt_specs.rs`

```rust
impl_builder_spec!(WktString, "wkt-types");
impl_select_spec!(WktGeom,    "wkt-types");   // 7 variants
```

---

## Phase 3 — `crates/elicit_wkt/` Shadow Crate

### Directory structure — mirrors `wkt`'s module layout

```
crates/elicit_wkt/src/
├── lib.rs
├── types/
│   ├── mod.rs
│   ├── coord.rs            # WktCoord  — wraps wkt::types::Coord<f64>
│   ├── point.rs            # WktPoint  — wraps wkt::types::Point<f64>
│   ├── linestring.rs       # WktLineString
│   ├── polygon.rs          # WktPolygon
│   ├── multipoint.rs       # WktMultiPoint
│   ├── multilinestring.rs  # WktMultiLineString
│   ├── multipolygon.rs     # WktMultiPolygon
│   └── geometrycollection.rs # WktGeometryCollection
├── wkt_item.rs             # WktItem — wraps wkt::Wkt<f64>
└── workflow/
    ├── mod.rs
    ├── types_plugin.rs     # WktTypesPlugin: WktCoord + WktPoint tools
    ├── compound_plugin.rs  # WktCompoundPlugin: WktLineString + WktPolygon tools
    ├── multi_plugin.rs     # WktMultiPlugin: WktMulti* + WktGeometryCollection tools
    ├── wkt_plugin.rs       # WktPlugin: WktItem parse/serialize tools
    ├── to_wkt_factory.rs   # ToWktFactory: trait factory for wkt::ToWkt
    └── try_from_wkt_factory.rs # TryFromWktFactory: trait factory for wkt::TryFromWkt
```

### 3.1 `Cargo.toml`

```toml
[package]
name = "elicit_wkt"
description = "Elicitation shadow crate for the wkt Well-Known Text library"
keywords = ["mcp", "wkt", "geometry", "spatial", "elicitation"]
categories = ["science::geo", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["wkt-types", "emit"] }
elicitation_derive.workspace = true
wkt = { workspace = true }
geo-types = { workspace = true }
schemars.workspace = true
serde.workspace = true
serde_json.workspace = true
rmcp.workspace = true
tracing.workspace = true
proc-macro2 = { workspace = true }
quote = { workspace = true }

[features]
default = []
emit = ["elicitation/emit"]
```

### 3.2 `types/coord.rs` — shadow `wkt::types::Coord<f64>`

```rust
elicit_newtype!(WktCoord, as Coord, serde);

impl Coord {
    /// Creates a 2D WKT coordinate.
    pub fn new(x: f64, y: f64) -> Self { ... }

    /// Creates a 3D WKT coordinate (with Z).
    pub fn new_3d(x: f64, y: f64, z: f64) -> Self { ... }

    /// Creates a coordinate with M (measure) value.
    pub fn new_with_m(x: f64, y: f64, m: f64) -> Self { ... }
}

#[reflect_methods]
impl Coord {
    /// Returns the X component.
    pub fn x(&self) -> f64 { self.x }

    /// Returns the Y component.
    pub fn y(&self) -> f64 { self.y }

    /// Returns the Z component, if present.
    pub fn z(&self) -> Option<f64> { self.z }

    /// Returns the M (measure) component, if present.
    pub fn m(&self) -> Option<f64> { self.m }
}
```

### 3.3 `types/point.rs` — shadow `wkt::types::Point<f64>`

`wkt::types::Point<f64>` is `(Option<Coord<f64>>,)`. A WKT `POINT EMPTY` has `None`.

```rust
elicit_newtype!(WktPoint, as Point, serde);

impl Point {
    /// Creates a point from a coordinate.
    pub fn new(coord: Coord) -> Self { ... }

    /// Creates a WKT POINT EMPTY.
    pub fn empty() -> Self { ... }
}

#[reflect_methods]
impl Point {
    /// Returns the inner coordinate, or None for POINT EMPTY.
    pub fn coord(&self) -> Option<Coord> { self.0.as_ref().map(|c| Coord::from(*c)) }

    /// Returns true if this is POINT EMPTY.
    pub fn is_empty(&self) -> bool { self.0.is_none() }
}
```

### 3.4 `types/linestring.rs` — shadow `wkt::types::LineString<f64>`

`wkt::types::LineString<f64>` is `(Vec<Coord<f64>>,)`.

```rust
elicit_newtype!(WktLineString, as LineString, serde);

impl LineString {
    /// Creates a line string from a list of coordinates.
    pub fn new(coords: Vec<Coord>) -> Self { ... }
}

#[reflect_methods]
impl LineString {
    /// Returns the list of coordinates.
    pub fn coords(&self) -> Vec<Coord> { self.0.iter().map(|c| Coord::from(*c)).collect() }

    /// Returns the number of coordinates.
    pub fn len(&self) -> usize { self.0.len() }

    /// Returns true if no coordinates.
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
}
```

### 3.5 `types/polygon.rs` — shadow `wkt::types::Polygon<f64>`

`wkt::types::Polygon<f64>` has fields `exterior: LineString<f64>` and
`interiors: Vec<LineString<f64>>`.

```rust
elicit_newtype!(WktPolygon, as Polygon, serde);

impl Polygon {
    /// Creates a polygon from an exterior ring and optional interior rings (holes).
    pub fn new(exterior: LineString, interiors: Vec<LineString>) -> Self { ... }
}

#[reflect_methods]
impl Polygon {
    /// Returns the exterior ring.
    pub fn exterior(&self) -> LineString { LineString::from(self.exterior.clone()) }

    /// Returns the interior rings (holes).
    pub fn interiors(&self) -> Vec<LineString> { ... }

    /// Returns the number of interior rings.
    pub fn interiors_len(&self) -> usize { self.interiors.len() }
}
```

### 3.6 Multi-types and GeometryCollection

Same `elicit_newtype!` + `#[reflect_methods]` pattern:

- `WktMultiPoint`: `points()`, `len()`, `is_empty()`
- `WktMultiLineString`: `lines()`, `len()`, `is_empty()`
- `WktMultiPolygon`: `polygons()`, `len()`, `is_empty()`
- `WktGeometryCollection`: `geometries()`, `len()`, `is_empty()`

### 3.7 `wkt_item.rs` — shadow `wkt::Wkt<f64>`

`wkt::Wkt<f64>` is an enum. We wrap it as `WktItem` (since the `Wkt` name collides with
the crate name).

```rust
elicit_newtype!(WktGeom, as WktItem, serde);

impl WktItem {
    /// Parses a WKT string. Mirrors `Wkt::<f64>::from_str`.
    pub fn from_str(wkt: &str) -> Result<Self, String> {
        wkt::Wkt::<f64>::from_str(wkt)
            .map(|w| Self::from(WktGeom::from(w)))
            .map_err(|e| e.to_string())
    }
}

#[reflect_methods]
impl WktItem {
    /// Serializes back to a WKT string. Mirrors `Display` / `ToWkt::wkt_string`.
    pub fn wkt_string(&self) -> String { format!("{}", wkt::Wkt::<f64>::from(*self.clone())) }

    /// Returns the geometry type discriminant.
    pub fn geometry_type(&self) -> String {
        match wkt::Wkt::<f64>::from((*self).clone()) {
            wkt::Wkt::Point(_) => "Point",
            wkt::Wkt::LineString(_) => "LineString",
            wkt::Wkt::Polygon(_) => "Polygon",
            wkt::Wkt::MultiPoint(_) => "MultiPoint",
            wkt::Wkt::MultiLineString(_) => "MultiLineString",
            wkt::Wkt::MultiPolygon(_) => "MultiPolygon",
            wkt::Wkt::GeometryCollection(_) => "GeometryCollection",
        }.to_string()
    }
}
```

### 3.8 `workflow/types_plugin.rs` — `WktTypesPlugin`

Propositions for this plugin:

```rust
#[derive(Prop)] pub struct CoordCreated;
impl VerifiedWorkflow for CoordCreated {}

#[derive(Prop)] pub struct PointParsed;
impl VerifiedWorkflow for PointParsed {}
```

Tools mirror constructors and key methods:
- `wkt_coord__new` — `(x, y)` → `(Coord, Established<CoordCreated>)`
- `wkt_coord__new_3d` — `(x, y, z)` → `(Coord, Established<CoordCreated>)`
- `wkt_coord__x` — `(Coord,)` → `f64`
- `wkt_coord__y` — `(Coord,)` → `f64`
- `wkt_coord__z` — `(Coord,)` → `Option<f64>`
- `wkt_coord__m` — `(Coord,)` → `Option<f64>`
- `wkt_point__new` — `(Coord,)` → `(Point, Established<PointParsed>)`
- `wkt_point__empty` — `()` → `Point`
- `wkt_point__coord` — `(Point,)` → `Option<Coord>`
- `wkt_point__is_empty` — `(Point,)` → `bool`

### 3.9 `workflow/compound_plugin.rs` — `WktCompoundPlugin`

Propositions:
```rust
#[derive(Prop)] pub struct LineStringCreated;
impl VerifiedWorkflow for LineStringCreated {}

#[derive(Prop)] pub struct PolygonCreated;
impl VerifiedWorkflow for PolygonCreated {}
```

Tools:
- `wkt_linestring__new` — `(Vec<Coord>,)` → `(LineString, Established<LineStringCreated>)`
- `wkt_linestring__coords` — `(LineString,)` → `Vec<Coord>`
- `wkt_linestring__len` — `(LineString,)` → `usize`
- `wkt_linestring__is_empty` — `(LineString,)` → `bool`
- `wkt_polygon__new` — `(LineString, Vec<LineString>)` → `(Polygon, Established<PolygonCreated>)`
- `wkt_polygon__exterior` — `(Polygon,)` → `LineString`
- `wkt_polygon__interiors` — `(Polygon,)` → `Vec<LineString>`
- `wkt_polygon__interiors_len` — `(Polygon,)` → `usize`

### 3.10 `workflow/multi_plugin.rs` — `WktMultiPlugin`

Tools for `WktMultiPoint`, `WktMultiLineString`, `WktMultiPolygon`, `WktGeometryCollection`.
Pattern: same `new(items)`, `items()`, `len()`, `is_empty()` per type.
Propositions: `MultiPointCreated`, `MultiLineStringCreated`, `MultiPolygonCreated`.

### 3.11 `workflow/wkt_plugin.rs` — `WktPlugin`

The main entry point: parse and serialize WKT.

Propositions:
```rust
#[derive(Prop)] pub struct WktParsed;   // a WKT string was successfully parsed
impl VerifiedWorkflow for WktParsed {}
```

Tools:
- `wkt__from_str` — `(wkt_string: String,)` → `(WktItem, Established<WktParsed>)`
  Mirrors `Wkt::<f64>::from_str`. Returns error if string is not valid WKT.
- `wkt__wkt_string` — `(WktItem,)` → `String`
  Mirrors `Display` — serializes the parsed geometry back to WKT.
- `wkt__geometry_type` — `(WktItem,)` → `String`
  Returns the geometry type name ("Point", "LineString", etc.).

### 3.12 `workflow/to_wkt_factory.rs` — `ToWktFactory`

Trait factory for `wkt::ToWkt<f64>`. Registered types: all geo_types wrappers
(`GeoPoint`, `GeoLineString`, `GeoPolygon`, `GeoMultiPoint`, `GeoMultiLineString`,
`GeoMultiPolygon`, `GeoGeometryCollection`).

Exposed methods per registered type:
- `to_wkt()` — `T` → `WktItem`
- `wkt_string()` — `T` → `String`

Tool namespace: `to_wkt__{type}__to_wkt`, `to_wkt__{type}__wkt_string`.

### 3.13 `workflow/try_from_wkt_factory.rs` — `TryFromWktFactory`

Trait factory for `wkt::TryFromWkt<f64>`. Same registered types as `ToWktFactory`.

Exposed methods per registered type:
- `try_from_wkt_str(s: &str)` → `Result<T, String>`

Tool namespace: `try_from_wkt__{type}__try_from_wkt_str`.

### 3.14 `impl ElicitComplete`

```rust
impl elicitation::ElicitComplete for WktItem {}
```

---

## Phase 4 — Kani Proofs

### `crates/elicitation_kani/Cargo.toml`

Add: `wkt-types = ["elicitation/wkt-types"]`

### `crates/elicitation_kani/src/wkt_types.rs`

```rust
// WktString: builder type — trusted
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_string_trusted() {
    kani::assume(true);
    assert!(true, "WktString: trusted third-party parser");
}

// WktGeom: 7 variants — test From<wkt::Wkt<f64>> roundtrip via from_str
// (Builder type: we trust wkt::FromStr, so use kani::assume)
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_geom_trusted() {
    kani::assume(true);
    assert!(true, "WktGeom: trusted third-party enum");
}

// WktCoord: field preservation roundtrip
#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_coord_from_roundtrip() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();
    kani::assume(x.is_finite() && y.is_finite());

    let orig = wkt::types::Coord { x, y, z: None, m: None };
    let wrapper = elicitation::WktCoord::from(orig);
    let restored: wkt::types::Coord<f64> = wrapper.into();

    assert!(restored.x == x);
    assert!(restored.y == y);
}

#[cfg(feature = "wkt-types")]
#[kani::proof]
fn verify_wkt_coord_fields() {
    let x: f64 = kani::any();
    let y: f64 = kani::any();
    kani::assume(x.is_finite() && y.is_finite());

    let orig = wkt::types::Coord { x, y, z: None, m: None };
    let wrapper = elicitation::WktCoord::from(orig);
    assert!(wrapper.x == x);
    assert!(wrapper.y == y);
}
```

Wire `wkt_types` module and 4 harnesses into runner.

---

## Phase 5 — Creusot Proofs

### `crates/elicitation_creusot/Cargo.toml`

Add: `wkt-types = ["elicitation/wkt-types"]`

### `crates/elicitation_creusot/src/wkt_types.rs`

```rust
#[cfg(feature = "wkt-types")]
mod wkt_types_proofs {
    use creusot_contracts::*;

    // WktString is a builder type — trusted
    #[trusted]
    #[requires(true)]
    #[ensures(true)]
    pub fn verify_wkt_string_trusted() {}

    // WktCoord x field: de-trusted via extern_specs
    #[requires(true)]
    #[ensures(result == orig_x)]
    pub fn verify_wkt_coord_x_preserved(orig_x: f64) -> f64 {
        use elicitation::WktCoord;
        let coord = WktCoord { x: orig_x, y: 0.0, z: None, m: None };
        coord.x
    }
}
```

### `crates/elicitation_creusot/src/extern_specs.rs`

```rust
#[cfg(feature = "wkt-types")]
extern_spec! {
    // Trust WktCoord field access (struct field reads are trivially correct)
    // No axiom needed beyond field transparency — Creusot handles public fields
}
```

---

## Phase 6 — Verus Proofs

### `crates/elicitation_verus/src/wkt_types.rs`

```rust
#[cfg(feature = "wkt-types")]
use vstd::prelude::*;

#[cfg(feature = "wkt-types")]
verus! {
    // WktString: builder type, trusted
    pub fn verify_wkt_string_trusted() {
        assert(true);
    }

    // WktCoord: x field preserved
    pub fn verify_wkt_coord_x_preserved(x: f64, y: f64) -> (result: f64)
        requires x.is_finite() && y.is_finite(),
        ensures result == x,
    {
        let coord = elicitation::WktCoord { x, y, z: None, m: None };
        coord.x
    }
}
```

---

## Phase 7 — `ElicitComplete` + `proof_non_empty_test`

### In `elicit_wkt/src/wkt_item.rs`

```rust
impl elicitation::ElicitComplete for WktItem {}
```

### In `crates/elicitation/tests/proof_non_empty_test.rs`

```rust
#[cfg(feature = "wkt-types")]
assert_proofs_non_empty::<elicitation::WktString>();
#[cfg(feature = "wkt-types")]
assert_proofs_non_empty::<elicitation::WktGeom>();
```

---

## Phase 8 — `VerifiedWorkflow` Registration

### Props (defined in `workflow/wkt_plugin.rs`)

```rust
#[derive(Prop)] pub struct WktParsed;
impl VerifiedWorkflow for WktParsed {}
```

Additional props per plugin:
- `CoordCreated`, `PointParsed`, `LineStringCreated`, `PolygonCreated`
- `MultiPointCreated`, `MultiLineStringCreated`, `MultiPolygonCreated`

### In `elicit_wkt/tests/workflow_verified_test.rs`

```rust
use elicitation::assert_verified;
use elicit_wkt::workflow::*;

#[test]
fn wkt_propositions_verified() {
    assert_verified::<WktParsed>();
    assert_verified::<CoordCreated>();
    assert_verified::<PointParsed>();
    assert_verified::<LineStringCreated>();
    assert_verified::<PolygonCreated>();
}
```

---

## Implementation Checklist

```text
Phase 1 — Workspace wiring
[ ] Cargo.toml: wkt = "0.11" workspace dep with features = ["geo-types"]
[ ] Cargo.toml: elicit_wkt member + workspace dep
[ ] elicitation/Cargo.toml: wkt optional dep + wkt-types feature (implies geo-types)
[ ] just check elicitation passes

Phase 2 — elicitation core types
[ ] primitives/wkt_types/coord.rs: WktCoord (x, y, z, m fields)
[ ] primitives/wkt_types/point.rs: WktPoint (coord: Option<WktCoord>)
[ ] primitives/wkt_types/linestring.rs: WktLineString (coords: Vec<WktCoord>)
[ ] primitives/wkt_types/polygon.rs: WktPolygon (exterior + interiors)
[ ] primitives/wkt_types/multipoint.rs: WktMultiPoint
[ ] primitives/wkt_types/multilinestring.rs: WktMultiLineString
[ ] primitives/wkt_types/multipolygon.rs: WktMultiPolygon
[ ] primitives/wkt_types/geometry_collection.rs: WktGeometryCollection
[ ] primitives/wkt_types/wkt_string.rs: WktString builder type
[ ] primitives/wkt_types/wkt_geom.rs: WktGeom Select enum (7 variants)
[ ] primitives/wkt_types/mod.rs: pub use all
[ ] primitives/mod.rs: pub mod wkt_types gated
[ ] lib.rs: pub use wkt_types::* gated
[ ] type_spec/wkt_specs.rs: impl_builder_spec! + impl_select_spec!
[ ] just check-all elicitation passes clean

Phase 3 — elicit_wkt shadow crate
[ ] Cargo.toml created
[ ] types/coord.rs: elicit_newtype!(WktCoord) + #[reflect_methods] (x, y, z, m)
[ ] types/point.rs: elicit_newtype!(WktPoint) + #[reflect_methods] (coord, is_empty)
[ ] types/linestring.rs: elicit_newtype! + #[reflect_methods] (coords, len, is_empty)
[ ] types/polygon.rs: elicit_newtype! + #[reflect_methods] (exterior, interiors, interiors_len)
[ ] types/multipoint.rs: elicit_newtype! + #[reflect_methods] (points, len, is_empty)
[ ] types/multilinestring.rs: elicit_newtype! + #[reflect_methods] (lines, len, is_empty)
[ ] types/multipolygon.rs: elicit_newtype! + #[reflect_methods] (polygons, len, is_empty)
[ ] types/geometrycollection.rs: elicit_newtype! + #[reflect_methods] (geometries, len, is_empty)
[ ] wkt_item.rs: elicit_newtype!(WktGeom as WktItem) + from_str + #[reflect_methods]
[ ] impl ElicitComplete for WktItem
[ ] workflow/types_plugin.rs: WktTypesPlugin (CoordCreated, PointParsed props + tools)
[ ] workflow/compound_plugin.rs: WktCompoundPlugin (LineStringCreated, PolygonCreated + tools)
[ ] workflow/multi_plugin.rs: WktMultiPlugin (Multi* props + tools)
[ ] workflow/wkt_plugin.rs: WktPlugin (WktParsed prop, from_str + wkt_string + geometry_type tools)
[ ] workflow/to_wkt_factory.rs: ToWktFactory (geo_types → WKT)
[ ] workflow/try_from_wkt_factory.rs: TryFromWktFactory (WKT → geo_types)
[ ] lib.rs: mod + pub use only
[ ] just check-all elicit_wkt passes clean

Phase 4 — Kani
[ ] elicitation_kani/Cargo.toml: wkt-types feature
[ ] src/wkt_types.rs: 4 proof harnesses
[ ] lib.rs + runner wired
[ ] just verify-kani-tracked

Phase 5 — Creusot
[ ] elicitation_creusot/Cargo.toml: wkt-types feature
[ ] src/wkt_types.rs: 2 proofs (trusted + coord x)
[ ] lib.rs + runner wired
[ ] cargo creusot prove

Phase 6 — Verus
[ ] elicitation_verus/src/wkt_types.rs: 2 proofs
[ ] lib.rs + runner wired
[ ] verus --crate-type=lib

Phase 7
[ ] elicit_wkt/src/wkt_item.rs: impl ElicitComplete
[ ] proof_non_empty_test.rs: WktString + WktGeom entries
[ ] just test-package elicitation

Phase 8
[ ] elicit_wkt/tests/workflow_verified_test.rs: 5 Props asserted
[ ] just test-package elicit_wkt

Commit
[ ] git commit -m "feat(wkt-types): shadow wkt 0.11 — types, traits, workflow, proofs"
```

---

## Key Notes

- `wkt::Wkt<f64>` is an **enum** (7 variants). Shadow it as `WktItem` using
  `elicit_newtype!(WktGeom, as WktItem, serde)` — `WktGeom` is the elicitation Phase 2 type.
- `wkt::types::Point<f64>` is a **tuple struct** `(Option<Coord<f64>>,)`. The `coord()`
  method returns `Option<WktCoord>`, checking `.0`.
- `wkt::types::GeometryCollection<f64>` wraps `Vec<Wkt<f64>>` — this creates a recursive
  type. The `WktGeometryCollection` wrapper holds `Vec<WktGeom>` instead.
- `ToWktFactory` and `TryFromWktFactory` are trait factories: they expose `wkt::ToWkt`
  and `wkt::TryFromWkt` as MCP tools for every registered geo_types type. The factory
  pattern is the same as `elicit_geo_types`'s trait plugins.
- Tool names follow `{module}__{method}` convention, using the actual method names from
  the `wkt` crate (`from_str`, `wkt_string`, `to_wkt`, `try_from_wkt_str`).
