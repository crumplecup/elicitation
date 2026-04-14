# ELICIT_GEO_PLAN.md

## Goal

Add complete geo support to elicitation as geometric algorithms alphabet:

1. **Core type integration** — geo algorithms in `elicitation` with feature gating
2. **Shadow crate** — `elicit_geo` with MCP tools for geometric operations

## Architecture Overview

Following established patterns from elicit_tokio, elicit_reqwest:

- **Core**: Feature-gated geo with trait-based algorithm support
- **Shadow crate**: ~8-10 workflow plugins covering geometric algorithms
- **Algorithms alphabet**: Operations on geo-types primitives (predicates, measurements, transformations)

## API Coverage

geo provides geometric algorithms operating on geo-types:

- **Predicates**: `Contains`, `Intersects`, `Within`, `Crosses`
- **Measurements**: `Area`, `Length`, `EuclideanDistance`, `HaversineDistance`
- **Calculations**: `Centroid`, `BoundingRect`, `ConvexHull`, `Bearing`
- **Transformations**: `Rotate`, `Scale`, `Translate`, `Simplify`
- **Boolean Ops**: `Union`, `Intersection`, `Difference` (via geo-booleanop)
- **Validation**: `IsValid`, `IsSimple`, `IsClosed`

**Total API surface**: ~30 trait-based algorithms → ~60-80 MCP tools across 8-10 plugins

## Phase 1: Workspace Configuration

### Files to modify

- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes

**1.1 Add geo to workspace dependencies**:

```toml
# Geometric algorithms
geo = "0.28"
```

**1.2 Add elicit_geo member**:

```toml
  "crates/elicit_geo",
```

**1.3 Add elicit_geo workspace dependency**:

```toml
elicit_geo = { path = "crates/elicit_geo", version = "0.9.1" }
```

**1.4 Add geo feature to elicitation**:

- Add optional dependency: `geo = { workspace = true, optional = true }`
- Add feature: `geo = ["dep:geo", "geo_types"]`
- Update `full` feature to include `"geo"`

## Phase 2: Core Algorithm Integration

### Files to create/modify

- `crates/elicitation/src/geo_algorithms.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Algorithm Support Strategy

**2.1 Trait-Based Algorithms** (re-export, no custom impl needed):

geo uses extension traits on geo-types:

```rust
use geo::{Area, Contains, EuclideanDistance};
use geo_types::{Point, Rect, Polygon};

// These traits are already implemented for geo-types
rect.contains(&point);           // bool
polygon.unsigned_area();         // f64
point.euclidean_distance(&other); // f64
```

**2.2 Algorithm Categories**:

- **Predicates** (return bool):
  - `Contains<Rhs>` - geometry contains another
  - `Intersects<Rhs>` - geometries intersect
  - `Within<Rhs>` - geometry within another
  - `Crosses<Rhs>` - geometries cross

- **Measurements** (return f64):
  - `Area` - unsigned_area(), signed_area()
  - `EuclideanLength` - euclidean_length()
  - `EuclideanDistance<Rhs>` - euclidean_distance()
  - `HaversineDistance<Rhs>` - haversine_distance()
  - `VincentyDistance<Rhs>` - vincenty_distance()

- **Calculations** (return geometry):
  - `Centroid` - centroid()
  - `BoundingRect` - bounding_rect()
  - `ConvexHull` - convex_hull()
  - `ExtremePoints` - extreme_points()

- **Transformations** (return geometry):
  - `Rotate` - rotate_around_point()
  - `Scale` - scale()
  - `Translate` - translate()
  - `Simplify` - simplify()
  - `SimplifyVw` - simplify_vw()

- **Validation** (return bool):
  - `IsConvex` - is_convex()
  - `HasDimensions` - has_dimensions()

### Implementation Pattern

```rust
// crates/elicitation/src/geo_algorithms.rs
#![cfg(feature = "geo")]

// Re-export all algorithm traits
pub use geo::{
    // Predicates
    Contains, Intersects, Within, Crosses,
    // Measurements
    Area, EuclideanDistance, EuclideanLength,
    HaversineDistance, VincentyDistance,
    // Calculations
    BoundingRect, Centroid, ConvexHull, ExtremePoints,
    // Transformations
    Rotate, Scale, Translate, Simplify, SimplifyVw,
    // Validation
    IsConvex, HasDimensions,
};

// No custom Elicitation impl needed - traits work on geo-types directly
// MCP tools will provide parameters and call trait methods
```

**2.3 Export from lib.rs**:

```rust
#[cfg(feature = "geo")]
pub mod geo_algorithms;

#[cfg(feature = "geo")]
pub use geo_algorithms::*;
```

## Phase 3: Create elicit_geo Shadow Crate

### Directory Structure

```
crates/elicit_geo/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── workflow/
│       ├── mod.rs
│       ├── predicates_plugin.rs      (~12 tools: Contains, Intersects, Within, etc.)
│       ├── measurements_plugin.rs    (~10 tools: Area, Distance, Length, etc.)
│       ├── calculations_plugin.rs    (~8 tools: Centroid, BoundingRect, ConvexHull)
│       ├── transformations_plugin.rs (~10 tools: Rotate, Scale, Translate, Simplify)
│       ├── validation_plugin.rs      (~6 tools: IsConvex, HasDimensions, etc.)
│       ├── boolean_plugin.rs         (~8 tools: Union, Intersection, Difference)
│       ├── geodesic_plugin.rs        (~6 tools: Haversine, Vincenty distances)
│       └── workflow_plugin.rs        (~10 tools: Common patterns)
└── tests/
    └── geo_test.rs
```

### Cargo.toml

```toml
[package]
name = "elicit_geo"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled geo wrappers with MCP tools for geometric algorithms"
keywords = ["mcp", "geo", "geometry", "algorithms", "elicitation"]
categories = ["science::geo", "mathematics", "algorithms", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["geo", "geo_types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
geo = { workspace = true }
geo-types = { workspace = true }
elicit_geo_types = { workspace = true }
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true

# Code emission
proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

### lib.rs structure

```rust
//! `elicit_geo` — comprehensive geo algorithms API exposure via MCP tools.
//!
//! Provides geometric algorithms operating on geo-types primitives:
//! - Predicates (Contains, Intersects, Within)
//! - Measurements (Area, Distance, Length)
//! - Calculations (Centroid, BoundingRect, ConvexHull)
//! - Transformations (Rotate, Scale, Translate, Simplify)
//! - Boolean operations (Union, Intersection, Difference)
//! - Validation (IsConvex, HasDimensions)
//!
//! # Plugin Organization (8 plugins, ~70 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `GeoPredicatesPlugin` | 12 | Contains, Intersects, Within, Crosses |
//! | `GeoMeasurementsPlugin` | 10 | Area, Distance, Length |
//! | `GeoCalculationsPlugin` | 8 | Centroid, BoundingRect, ConvexHull |
//! | `GeoTransformationsPlugin` | 10 | Rotate, Scale, Translate, Simplify |
//! | `GeoValidationPlugin` | 6 | IsConvex, HasDimensions |
//! | `GeoBooleanPlugin` | 8 | Union, Intersection, Difference |
//! | `GeoGeodesicPlugin` | 6 | Haversine, Vincenty distances |
//! | `GeoWorkflowPlugin` | 10 | Common patterns |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{
    GeoBooleanPlugin, GeoCalculationsPlugin, GeoGeodesicPlugin,
    GeoMeasurementsPlugin, GeoPredicatesPlugin, GeoTransformationsPlugin,
    GeoValidationPlugin, GeoWorkflowPlugin,
};
```

## Phase 4: Implement MCP Tools

### 4.1 Predicates Plugin (workflow/predicates_plugin.rs)

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContainsParams {
    pub container_type: String,  // "Rect", "Polygon", etc.
    pub container: serde_json::Value,
    pub geometry_type: String,
    pub geometry: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "geo_predicates__contains",
    description = "Check if one geometry contains another. \
                   Returns true if the second geometry is completely inside the first.",
    emit = Auto
)]
async fn predicates_contains(p: ContainsParams) -> Result<CallToolResult, ErrorData> {
    // Parse geometries, call Contains trait
    Ok(CallToolResult::success(vec![
        Content::text(format!("{} contains {}: computed", p.container_type, p.geometry_type))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct IntersectsParams {
    pub geometry1_type: String,
    pub geometry1: serde_json::Value,
    pub geometry2_type: String,
    pub geometry2: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "geo_predicates__intersects",
    description = "Check if two geometries intersect (share any points).",
    emit = Auto
)]
async fn predicates_intersects(p: IntersectsParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("{} intersects {}: computed", p.geometry1_type, p.geometry2_type))
    ]))
}

// ... 10 more tools: within, crosses, disjoint, touches, overlaps, etc.
```

### 4.2 Measurements Plugin (workflow/measurements_plugin.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AreaParams {
    pub geometry_type: String,  // "Polygon", "Rect", etc.
    pub geometry: serde_json::Value,
    pub signed: Option<bool>,
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "geo_measurements__area",
    description = "Calculate the area of a polygon or rectangle. \
                   Use signed=true for signed area (accounts for winding order).",
    emit = Auto
)]
async fn measurements_area(p: AreaParams) -> Result<CallToolResult, ErrorData> {
    let method = if p.signed.unwrap_or(false) { "signed_area" } else { "unsigned_area" };
    Ok(CallToolResult::success(vec![
        Content::text(format!("Area of {}: {} computed", p.geometry_type, method))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EuclideanDistanceParams {
    pub from_type: String,
    pub from: serde_json::Value,
    pub to_type: String,
    pub to: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "geo_measurements__euclidean_distance",
    description = "Calculate Euclidean (straight-line) distance between two geometries.",
    emit = Auto
)]
async fn measurements_euclidean_distance(p: EuclideanDistanceParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Euclidean distance: {} -> {}", p.from_type, p.to_type))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LengthParams {
    pub geometry_type: String,  // "LineString", "Line", etc.
    pub geometry: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "geo_measurements__length",
    description = "Calculate the Euclidean length of a line or line string.",
    emit = Auto
)]
async fn measurements_length(p: LengthParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Length of {}: computed", p.geometry_type))
    ]))
}

// ... 7 more tools: haversine_distance, vincenty_distance, perimeter, etc.
```

### 4.3 Calculations Plugin (workflow/calculations_plugin.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CentroidParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "geo_calculations__centroid",
    description = "Calculate the centroid (geometric center) of a geometry.",
    emit = Auto
)]
async fn calculations_centroid(p: CentroidParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Centroid of {}: computed", p.geometry_type))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BoundingRectParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "geo_calculations__bounding_rect",
    description = "Calculate axis-aligned bounding rectangle for a geometry.",
    emit = Auto
)]
async fn calculations_bounding_rect(p: BoundingRectParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Bounding rect of {}: computed", p.geometry_type))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConvexHullParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "geo_calculations__convex_hull",
    description = "Calculate the convex hull of a geometry (smallest convex polygon containing all points).",
    emit = Auto
)]
async fn calculations_convex_hull(p: ConvexHullParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Convex hull of {}: computed", p.geometry_type))
    ]))
}

// ... 5 more tools: extreme_points, envelope, closest_point, etc.
```

### 4.4 Transformations Plugin (workflow/transformations_plugin.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RotateParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
    pub angle_degrees: f64,
    pub origin_x: f64,
    pub origin_y: f64,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "geo_transformations__rotate",
    description = "Rotate a geometry by angle (degrees) around an origin point.",
    emit = Auto
)]
async fn transformations_rotate(p: RotateParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Rotated {} by {}° around ({}, {})",
            p.geometry_type, p.angle_degrees, p.origin_x, p.origin_y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScaleParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
    pub scale_x: f64,
    pub scale_y: f64,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "geo_transformations__scale",
    description = "Scale a geometry by x and y factors.",
    emit = Auto
)]
async fn transformations_scale(p: ScaleParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Scaled {} by x:{}, y:{}",
            p.geometry_type, p.scale_x, p.scale_y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TranslateParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
    pub x_offset: f64,
    pub y_offset: f64,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "geo_transformations__translate",
    description = "Translate (move) a geometry by x and y offsets.",
    emit = Auto
)]
async fn transformations_translate(p: TranslateParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Translated {} by ({}, {})",
            p.geometry_type, p.x_offset, p.y_offset))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SimplifyParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
    pub epsilon: f64,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "geo_transformations__simplify",
    description = "Simplify a geometry using Douglas-Peucker algorithm with epsilon tolerance.",
    emit = Auto
)]
async fn transformations_simplify(p: SimplifyParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Simplified {} with epsilon {}",
            p.geometry_type, p.epsilon))
    ]))
}

// ... 6 more tools: simplify_vw, affine_transform, etc.
```

### 4.5 Workflow Plugin (workflow/workflow_plugin.rs)

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BufferParams {
    pub geometry_type: String,
    pub geometry: serde_json::Value,
    pub distance: f64,
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "geo_workflow__buffer",
    description = "Create a buffer (expanded region) around a geometry at specified distance.",
    emit = Auto
)]
async fn workflow_buffer(p: BufferParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Buffered {} by distance {}",
            p.geometry_type, p.distance))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PointInPolygonParams {
    pub point: (f64, f64),
    pub polygon: serde_json::Value,
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "geo_workflow__point_in_polygon",
    description = "Fast point-in-polygon test (common workflow pattern).",
    emit = Auto
)]
async fn workflow_point_in_polygon(p: PointInPolygonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Point ({}, {}) in polygon: computed",
            p.point.0, p.point.1))
    ]))
}

// ... 8 more tools: nearest_point, clip_to_rect, merge_geometries, etc.
```

## Phase 5: Testing

### File to create

- `crates/elicit_geo/tests/geo_test.rs`

### Test Coverage

```rust
use geo::{Area, Contains, EuclideanDistance};
use geo_types::{Point, Rect, Polygon};

#[test]
fn test_rect_contains_point() {
    let rect = Rect::new(
        geo_types::Coord { x: 0.0, y: 0.0 },
        geo_types::Coord { x: 100.0, y: 50.0 }
    );
    let point = Point::new(50.0, 25.0);

    assert!(rect.contains(&point));
}

#[test]
fn test_polygon_area() {
    let exterior = vec![
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 10.0),
        (0.0, 10.0),
        (0.0, 0.0),
    ];
    let polygon = Polygon::new(
        geo_types::LineString::from(exterior),
        vec![]
    );

    assert_eq!(polygon.unsigned_area(), 100.0);
}

#[test]
fn test_point_distance() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);

    assert_eq!(p1.euclidean_distance(&p2), 5.0);
}
```

## Phase 6: Documentation

### File to create

- `crates/elicit_geo/README.md`

### Content

```markdown
# elicit_geo

Elicitation-enabled wrappers around [`geo`](https://docs.rs/geo) for geometric algorithms.

## Purpose

Provides the **geometric algorithms alphabet** — operations on geo-types primitives:
- Spatial predicates (Contains, Intersects)
- Measurements (Area, Distance, Length)
- Calculations (Centroid, BoundingRect)
- Transformations (Rotate, Scale, Simplify)
- Boolean operations (Union, Intersection)

## API Coverage

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `geo_predicates` | 12 | Contains, Intersects, Within, Crosses |
| `geo_measurements` | 10 | Area, Distance, Length |
| `geo_calculations` | 8 | Centroid, BoundingRect, ConvexHull |
| `geo_transformations` | 10 | Rotate, Scale, Translate, Simplify |
| `geo_validation` | 6 | IsConvex, HasDimensions |
| `geo_boolean` | 8 | Union, Intersection, Difference |
| `geo_geodesic` | 6 | Haversine, Vincenty distances |
| `geo_workflow` | 10 | Common patterns |

**Total: ~70 MCP tools**

## Usage

```rust
use geo::{Area, Contains, EuclideanDistance};
use elicit_geo_types::{Point, Rect, Polygon};

// MCP tools use these algorithms:
let rect = Rect::new(/* ... */);
let point = Point::new(50.0, 25.0);

// Predicate
if rect.contains(&point) { /* ... */ }

// Measurement
let area = polygon.unsigned_area();

// Distance
let distance = point1.euclidean_distance(&point2);
```

## Integration with elicit_ui

elicit_ui uses geo algorithms for all spatial validation:

```rust
use geo::Contains;
use elicit_ui::validators;

// Validator uses geo::Contains internally
validators::validate_no_overflow(&nodes, node_id, viewport)?;

// Equivalent to:
if !viewport.contains(&element_bounds) {
    return Err(/* overflow error */);
}
```

```

## Verification Steps

**After implementation**:
1. `cargo check -p elicit_geo`
2. `cargo test -p elicit_geo`
3. `cargo check -p elicitation --no-default-features --features geo`
4. `cargo check --all-features`

## Critical Files

### To create:
- `crates/elicit_geo/Cargo.toml`
- `crates/elicit_geo/README.md`
- `crates/elicit_geo/src/lib.rs`
- `crates/elicit_geo/src/workflow/mod.rs`
- `crates/elicit_geo/src/workflow/predicates_plugin.rs`
- `crates/elicit_geo/src/workflow/measurements_plugin.rs`
- `crates/elicit_geo/src/workflow/calculations_plugin.rs`
- `crates/elicit_geo/src/workflow/transformations_plugin.rs`
- `crates/elicit_geo/src/workflow/validation_plugin.rs`
- `crates/elicit_geo/src/workflow/boolean_plugin.rs`
- `crates/elicit_geo/src/workflow/geodesic_plugin.rs`
- `crates/elicit_geo/src/workflow/workflow_plugin.rs`
- `crates/elicit_geo/tests/geo_test.rs`
- `crates/elicitation/src/geo_algorithms.rs`

### To modify:
- `Cargo.toml`
- `crates/elicitation/Cargo.toml`
- `crates/elicitation/src/lib.rs`

## Implementation Order

1. **Phase 1**: Workspace configuration (15 min)
2. **Phase 2**: Core algorithm integration (30 min)
3. **Phase 3**: Create elicit_geo structure (30 min)
4. **Phase 4**: Implement MCP tools (~70 tools) (8-12 hours)
5. **Phase 5**: Testing (1 hour)
6. **Phase 6**: Documentation (30 min)

**Total estimated time**: 11-15 hours

## Notes

### Why geo?

Proven geometric algorithms for spatial operations:
- Extensively tested and benchmarked
- Trait-based design works naturally with geo-types
- Powers Rust GIS ecosystem (PostGIS bindings, mapping libraries, etc.)

### Trait-Based Design

geo uses extension traits:
```rust
use geo::{Contains, Area};

// Traits extend geo-types
impl Contains<Point> for Rect { /* ... */ }
impl Area for Polygon { /* ... */ }

// Natural usage
rect.contains(&point)
polygon.unsigned_area()
```

### Use Cases

- **UI layout**: elicit_ui spatial validation
- **Mapping**: GIS applications, cartography
- **Computational geometry**: CAD, graphics, robotics
- **Spatial queries**: Database operations, indexing
