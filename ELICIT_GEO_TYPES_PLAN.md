# ELICIT_GEO_TYPES_PLAN.md

## Goal
Add complete geo-types support to elicitation as geometric primitives alphabet:
1. **Core type integration** — geo-types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_geo_types` with MCP tools for geometric primitives

## Architecture Overview

Following established patterns from elicit_chrono, elicit_url:
- **Core**: Feature-gated geo-types with Select enums and Elicitation impls
- **Shadow crate**: ~4-6 workflow plugins covering geometric primitives
- **Geometry alphabet**: Foundation for spatial calculations, mapping, UI layout

## API Coverage

geo-types provides fundamental 2D/3D geometric primitives:
- **Points**: `Point`, `Coord` (2D coordinates)
- **Lines**: `Line`, `LineString` (connected points)
- **Polygons**: `Polygon`, `MultiPolygon` (closed shapes with holes)
- **Rectangles**: `Rect` (axis-aligned bounding boxes)
- **Triangles**: `Triangle` (3 points)
- **Collections**: `MultiPoint`, `MultiLineString`, `GeometryCollection`

**Total API surface**: ~15 core types, ~30 constructors/accessors → ~25-35 MCP tools across 4-6 plugins

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add geo-types to workspace dependencies**:
```toml
# Geometric primitives
geo-types = "0.7"
```

**1.2 Add elicit_geo_types member**:
```toml
  "crates/elicit_geo_types",
```

**1.3 Add elicit_geo_types workspace dependency**:
```toml
elicit_geo_types = { path = "crates/elicit_geo_types", version = "0.9.1" }
```

**1.4 Add geo_types feature to elicitation**:
- Add optional dependency: `geo-types = { workspace = true, optional = true }`
- Add feature: `geo_types = ["dep:geo-types"]`
- Update `full` feature to include `"geo_types"`

## Phase 2: Core Type Integration

### Files to create/modify:
- `crates/elicitation/src/geo_types_support.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Primitive Types** (manual `Elicitation` impl):
- `Coord<T>` - (x, y) coordinate pair
- `Point<T>` - Single point
- `Line<T>` - Two points forming a line segment
- `Rect<T>` - Axis-aligned rectangle (min, max corners)
- `Triangle<T>` - Three points

**2.2 Collection Types** (manual `Elicitation` impl):
- `LineString<T>` - Sequence of connected points
- `Polygon<T>` - Exterior ring + optional holes
- `MultiPoint<T>` - Collection of points
- `MultiLineString<T>` - Collection of line strings
- `MultiPolygon<T>` - Collection of polygons

**2.3 Geometry Enum** (use `select_trenchcoat!` macro):
```rust
// Wrapper for all geometry types
pub enum Geometry<T> {
    Point(Point<T>),
    Line(Line<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    Rect(Rect<T>),
    Triangle(Triangle<T>),
    GeometryCollection(GeometryCollection<T>),
}
```

### Implementation Pattern:

```rust
// crates/elicitation/src/geo_types_support.rs
#![cfg(feature = "geo_types")]

use geo_types::{Coord, Point, Line, Rect, Polygon};

/// Elicitation impl for Coord (x, y pair)
impl Elicitation for Coord<f64> {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for x, y coordinates
        let x = ctx.prompt_f64("x coordinate")?;
        let y = ctx.prompt_f64("y coordinate")?;
        Ok(Coord { x, y })
    }
}

/// Elicitation impl for Point
impl Elicitation for Point<f64> {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        let coord = Coord::<f64>::elicit(ctx).await?;
        Ok(Point::from(coord))
    }
}

/// Elicitation impl for Rect (bounding box)
impl Elicitation for Rect<f64> {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for min corner (x0, y0) and max corner (x1, y1)
        let min = Coord::<f64>::elicit(ctx).await?;
        let max = Coord::<f64>::elicit(ctx).await?;
        Ok(Rect::new(min, max))
    }
}

/// Elicitation impl for Polygon
impl Elicitation for Polygon<f64> {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for exterior ring points
        // Optional: prompt for holes
        // Build LineString for exterior, convert to Polygon
    }
}
```

**2.4 Export from lib.rs**:
```rust
#[cfg(feature = "geo_types")]
pub mod geo_types_support;

#[cfg(feature = "geo_types")]
pub use geo_types_support::*;
```

## Phase 3: Create elicit_geo_types Shadow Crate

### Directory Structure:

```
crates/elicit_geo_types/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── primitives.rs       (Point, Coord, Line wrappers)
│   ├── shapes.rs           (Rect, Triangle, Polygon wrappers)
│   ├── collections.rs      (MultiPoint, MultiLineString wrappers)
│   └── workflow/
│       ├── mod.rs
│       ├── primitives_plugin.rs   (~8 tools: Point, Coord, Line creation)
│       ├── shapes_plugin.rs       (~10 tools: Rect, Triangle, Polygon)
│       ├── collections_plugin.rs  (~8 tools: Multi* types)
│       └── workflow_plugin.rs     (~6 tools: Common patterns)
└── tests/
    └── geo_types_test.rs
```

### Cargo.toml:

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
elicitation = { workspace = true, features = ["geo_types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
geo-types = { workspace = true }
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

### lib.rs structure:

```rust
//! `elicit_geo_types` — comprehensive geo-types API exposure via MCP tools.
//!
//! Provides geometric primitives alphabet:
//! - Points and coordinates
//! - Lines and line strings
//! - Rectangles and polygons
//! - Multi-geometry collections
//!
//! # Plugin Organization (4 plugins, ~32 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `GeoTypesPrimitivesPlugin` | 8 | Point, Coord, Line creation |
//! | `GeoTypesShapesPlugin` | 10 | Rect, Triangle, Polygon |
//! | `GeoTypesCollectionsPlugin` | 8 | MultiPoint, MultiLineString, etc. |
//! | `GeoTypesWorkflowPlugin` | 6 | Common patterns and conversions |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod collections;
mod primitives;
mod shapes;
pub mod workflow;

pub use collections::{MultiLineString, MultiPoint, MultiPolygon};
pub use primitives::{Coord, Line, Point};
pub use shapes::{Polygon, Rect, Triangle};
pub use workflow::{
    GeoTypesCollectionsPlugin, GeoTypesPrimitivesPlugin,
    GeoTypesShapesPlugin, GeoTypesWorkflowPlugin,
};
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Primitives (primitives.rs):

```rust
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use geo_types as gt;

// Coord doesn't need newtype - it's Copy + simple
// Just re-export with custom Elicitation impl
pub type Coord = gt::Coord<f64>;

elicit_newtype!(gt::Point<f64>, as Point, serde);

#[reflect_methods]
impl Point {
    #[instrument(skip(self))]
    pub fn x(&self) -> f64 {
        self.0.x()
    }

    #[instrument(skip(self))]
    pub fn y(&self) -> f64 {
        self.0.y()
    }

    #[instrument]
    pub fn new(x: f64, y: f64) -> Self {
        Self(gt::Point::new(x, y))
    }
}

elicit_newtype!(gt::Line<f64>, as Line, serde);

#[reflect_methods]
impl Line {
    #[instrument]
    pub fn new(start: Point, end: Point) -> Self {
        Self(gt::Line::new(start.0, end.0))
    }

    #[instrument(skip(self))]
    pub fn start(&self) -> Point {
        Point(self.0.start)
    }

    #[instrument(skip(self))]
    pub fn end(&self) -> Point {
        Point(self.0.end)
    }
}
```

### 4.2 Shapes (shapes.rs):

```rust
elicit_newtype!(gt::Rect<f64>, as Rect, serde);

#[reflect_methods]
impl Rect {
    #[instrument]
    pub fn new(min: Coord, max: Coord) -> Self {
        Self(gt::Rect::new(min, max))
    }

    #[instrument(skip(self))]
    pub fn min(&self) -> Coord {
        self.0.min()
    }

    #[instrument(skip(self))]
    pub fn max(&self) -> Coord {
        self.0.max()
    }

    #[instrument(skip(self))]
    pub fn width(&self) -> f64 {
        self.0.width()
    }

    #[instrument(skip(self))]
    pub fn height(&self) -> f64 {
        self.0.height()
    }
}

elicit_newtype!(gt::Polygon<f64>, as Polygon, serde);

#[reflect_methods]
impl Polygon {
    #[instrument]
    pub fn new(exterior: LineString, interiors: Vec<LineString>) -> Self {
        Self(gt::Polygon::new(
            exterior.0,
            interiors.into_iter().map(|ls| ls.0).collect()
        ))
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Primitives Plugin (workflow/primitives_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePointParams {
    pub x: f64,
    pub y: f64,
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "geo_types_primitives__create_point",
    description = "Create a Point from x, y coordinates.",
    emit = Auto
)]
async fn primitives_create_point(p: CreatePointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created Point({}, {})", p.x, p.y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateLineParams {
    pub start_x: f64,
    pub start_y: f64,
    pub end_x: f64,
    pub end_y: f64,
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "geo_types_primitives__create_line",
    description = "Create a Line from start and end coordinates.",
    emit = Auto
)]
async fn primitives_create_line(p: CreateLineParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created Line: ({}, {}) -> ({}, {})",
            p.start_x, p.start_y, p.end_x, p.end_y))
    ]))
}

// ... 6 more tools: create_coord, line_from_points, etc.
```

### 5.2 Shapes Plugin (workflow/shapes_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRectParams {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "geo_types_shapes__create_rect",
    description = "Create a Rect (axis-aligned bounding box) from min and max coordinates.",
    emit = Auto
)]
async fn shapes_create_rect(p: CreateRectParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created Rect: ({}, {}) to ({}, {})",
            p.x0, p.y0, p.x1, p.y1))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreatePolygonParams {
    pub exterior_points: Vec<(f64, f64)>,
    pub holes: Option<Vec<Vec<(f64, f64)>>>,
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "geo_types_shapes__create_polygon",
    description = "Create a Polygon from exterior ring and optional holes.",
    emit = Auto
)]
async fn shapes_create_polygon(p: CreatePolygonParams) -> Result<CallToolResult, ErrorData> {
    let hole_count = p.holes.as_ref().map(|h| h.len()).unwrap_or(0);
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created Polygon: {} exterior points, {} holes",
            p.exterior_points.len(), hole_count))
    ]))
}

// ... 8 more tools: create_triangle, rect_from_corners, polygon_from_points, etc.
```

### 5.3 Collections Plugin (workflow/collections_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateMultiPointParams {
    pub points: Vec<(f64, f64)>,
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "geo_types_collections__create_multi_point",
    description = "Create a MultiPoint collection from coordinate pairs.",
    emit = Auto
)]
async fn collections_create_multi_point(p: CreateMultiPointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created MultiPoint: {} points", p.points.len()))
    ]))
}

// ... 7 more tools: create_multi_line_string, create_multi_polygon, etc.
```

### 5.4 Workflow Plugin (workflow/workflow_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BoundingBoxParams {
    pub points: Vec<(f64, f64)>,
}

#[elicit_tool(
    plugin = "geo_types_workflow",
    name = "geo_types_workflow__bounding_box",
    description = "Calculate bounding box (Rect) for a set of points.",
    emit = Auto
)]
async fn workflow_bounding_box(p: BoundingBoxParams) -> Result<CallToolResult, ErrorData> {
    // Calculate min/max from points
    Ok(CallToolResult::success(vec![
        Content::text(format!("Bounding box calculated from {} points", p.points.len()))
    ]))
}

// ... 5 more tools: centroid_of_points, convex_hull_points, etc.
```

## Phase 6: Testing

### File to create:
- `crates/elicit_geo_types/tests/geo_types_test.rs`

### Test Coverage:

```rust
#[test]
fn test_point_creation() {
    let point = geo_types::Point::new(10.0, 20.0);
    let wrapped = Point::from(point);

    assert_eq!(wrapped.x(), 10.0);
    assert_eq!(wrapped.y(), 20.0);
}

#[test]
fn test_rect_creation() {
    let min = geo_types::Coord { x: 0.0, y: 0.0 };
    let max = geo_types::Coord { x: 100.0, y: 50.0 };
    let rect = geo_types::Rect::new(min, max);
    let wrapped = Rect::from(rect);

    assert_eq!(wrapped.width(), 100.0);
    assert_eq!(wrapped.height(), 50.0);
}

#[test]
fn test_polygon_serialization() {
    let exterior = vec![
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 10.0),
        (0.0, 10.0),
        (0.0, 0.0),
    ];

    let json = serde_json::to_value(&PolygonParams {
        exterior_points: exterior,
        holes: None,
    }).unwrap();

    assert_eq!(json["exterior_points"].as_array().unwrap().len(), 5);
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_geo_types/README.md`

### Content:

```markdown
# elicit_geo_types

Elicitation-enabled wrappers around [`geo-types`](https://docs.rs/geo-types) for geometric primitives.

## Purpose

Provides the **geometric primitives alphabet** — foundational types for:
- Spatial calculations and mapping
- UI layout (with elicit_ui)
- Geographic information systems
- Computational geometry

## API Coverage

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `geo_types_primitives` | 8 | Point, Coord, Line |
| `geo_types_shapes` | 10 | Rect, Triangle, Polygon |
| `geo_types_collections` | 8 | MultiPoint, MultiLineString, MultiPolygon |
| `geo_types_workflow` | 6 | Bounding boxes, conversions |

**Total: ~32 MCP tools**

## Usage

```rust
use elicit_geo_types::{Point, Rect, Polygon};

// MCP tools generate this code:
let point = Point::new(10.0, 20.0);
let rect = Rect::new(
    Coord { x: 0.0, y: 0.0 },
    Coord { x: 100.0, y: 50.0 }
);
```

## Integration with elicit_geo

Combine with `elicit_geo` for geometric algorithms:

```rust
use geo::Contains;
use elicit_geo_types::{Point, Rect};

let rect = Rect::new(/* ... */);
let point = Point::new(50.0, 25.0);

// Use geo algorithms on geo-types primitives
if rect.contains(&point) {
    println!("Point is inside rectangle");
}
```

## Integration with elicit_ui

elicit_ui uses geo-types for all spatial calculations:

```rust
use elicit_ui::{Layout, Viewport};
use elicit_geo_types::Rect;

// Viewport is geo_types::Rect
let viewport = Viewport::from_dimensions(1920, 1080);

// Element bounds are geo_types::Rect
let element_bounds = Rect::new(/* ... */);
```
```

## Verification Steps

**After implementation**:
1. `cargo check -p elicit_geo_types`
2. `cargo test -p elicit_geo_types`
3. `cargo check -p elicitation --no-default-features --features geo_types`
4. `cargo check --all-features`

**Manual verification**:
1. Launch MCP server with elicit_geo_types
2. Call `geo_types_primitives__create_point`
3. Call `geo_types_shapes__create_rect`
4. Verify JSON responses and code emission

## Critical Files

### To create:
- `crates/elicit_geo_types/Cargo.toml`
- `crates/elicit_geo_types/README.md`
- `crates/elicit_geo_types/src/lib.rs`
- `crates/elicit_geo_types/src/primitives.rs`
- `crates/elicit_geo_types/src/shapes.rs`
- `crates/elicit_geo_types/src/collections.rs`
- `crates/elicit_geo_types/src/workflow/mod.rs`
- `crates/elicit_geo_types/src/workflow/primitives_plugin.rs`
- `crates/elicit_geo_types/src/workflow/shapes_plugin.rs`
- `crates/elicit_geo_types/src/workflow/collections_plugin.rs`
- `crates/elicit_geo_types/src/workflow/workflow_plugin.rs`
- `crates/elicit_geo_types/tests/geo_types_test.rs`
- `crates/elicitation/src/geo_types_support.rs`

### To modify:
- `Cargo.toml`
- `crates/elicitation/Cargo.toml`
- `crates/elicitation/src/lib.rs`

## Implementation Order

1. **Phase 1**: Workspace configuration (15 min)
2. **Phase 2**: Core type integration (1 hour)
3. **Phase 3**: Create elicit_geo_types structure (30 min)
4. **Phase 4**: Implement type wrappers (1-2 hours)
5. **Phase 5**: Implement MCP tools (~32 tools) (4-6 hours)
6. **Phase 6**: Testing (1 hour)
7. **Phase 7**: Documentation (30 min)

**Total estimated time**: 8-11 hours

## Notes

### Why geo-types?

Fundamental building blocks for all spatial work:
- Every spatial algorithm operates on these primitives
- GeoJSON, WKT, and other formats serialize to/from these types
- Proven, well-tested, widely adopted in Rust geo ecosystem

### Type Hierarchy

```
Coord<T>        - (x, y) pair
  ↓
Point<T>        - Single point (wraps Coord)
  ↓
Line<T>         - Two points
  ↓
LineString<T>   - Sequence of points (open path)
  ↓
Polygon<T>      - Closed exterior + holes (LineStrings)
  ↓
MultiPolygon<T> - Collection of polygons
```

### Coordinate System Agnostic

geo-types doesn't assume any coordinate system:
- UI: pixel coordinates (f64)
- Maps: lat/lon (f64)
- CAD: engineering units (f64)
- Generic: any numeric type T

This makes it perfect for UI spatial calculations.
