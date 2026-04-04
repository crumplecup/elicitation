# ELICIT_PROJ_PLAN.md

## Goal

Add comprehensive PROJ library support to elicitation for coordinate reference system (CRS) transformations:

1. **Core type integration** — proj types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_proj` with MCP tools for CRS operations (~45 tools)
3. **GIS alphabet** — Foundation for professional geospatial workflows

## Use Cases

- **Coordinate transformations**: WGS84 ↔ UTM ↔ State Plane ↔ Web Mercator
- **Datum conversions**: NAD83 → WGS84, local datums
- **EPSG lookups**: Query CRS definitions by EPSG code
- **Custom projections**: Define and use custom CRS from PROJ strings
- **Distance calculations**: Geodesic and projected distances
- **Area calculations**: Accurate area on ellipsoid
- **Geospatial pipelines**: Chain transformations for complex workflows

## Architecture Overview

Following established patterns from elicit_geo, elicit_geojson:
- **Core**: Feature-gated proj types with Select enums and Elicitation impls
- **Shadow crate**: 7 workflow plugins covering ~45 operations
- **GIS alphabet**: Foundation for coordinate transformations in any projection

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add proj to workspace dependencies** (`Cargo.toml`):
```toml
# Geospatial - Coordinate Reference Systems
proj = { version = "0.27", default-features = false }
```

**1.2 Add elicit_proj member** (`Cargo.toml` after other elicit_* members):
```toml
  "crates/elicit_proj",
```

**1.3 Add elicit_proj workspace dependency** (`Cargo.toml`):
```toml
elicit_proj = { path = "crates/elicit_proj", version = "0.9.1" }
```

**1.4 Add proj feature to elicitation** (`crates/elicitation/Cargo.toml`):
- Add optional dependency: `proj = { workspace = true, optional = true }`
- Add feature: `proj = ["dep:proj"]`
- Update `full` feature to include `"proj"`
- Consider adding `gis = ["proj", "geo", "geo-types", "geojson", "rstar"]` meta-feature

## Phase 2: Core Type Integration in elicitation

### Files to create/modify:
- `crates/elicitation/src/proj_types.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Core Types**:

```rust
// crates/elicitation/src/proj_types.rs
#![cfg(feature = "proj")]

use proj::{Proj, Coord};
use elicitation::{Elicitation, ElicitationContext};

/// Coordinate in any CRS (x, y, z, t)
pub use proj::Coord;

/// Transformation between coordinate systems
pub struct ProjTransform {
    inner: Proj,
    from_crs: String,
    to_crs: String,
}

impl ProjTransform {
    pub fn new(from_crs: &str, to_crs: &str) -> Result<Self, proj::ProjError> {
        Ok(Self {
            inner: Proj::new_known_crs(from_crs, to_crs, None)?,
            from_crs: from_crs.to_string(),
            to_crs: to_crs.to_string(),
        })
    }

    pub fn from_epsg(from_epsg: u16, to_epsg: u16) -> Result<Self, proj::ProjError> {
        let from = format!("EPSG:{}", from_epsg);
        let to = format!("EPSG:{}", to_epsg);
        Self::new(&from, &to)
    }

    pub fn from_proj_string(definition: &str) -> Result<Self, proj::ProjError> {
        Ok(Self {
            inner: Proj::new(definition)?,
            from_crs: "custom".to_string(),
            to_crs: "custom".to_string(),
        })
    }

    pub fn convert(&self, coord: Coord<f64>) -> Result<Coord<f64>, proj::ProjError> {
        self.inner.convert(coord)
    }

    pub fn convert_array(&self, coords: &[(f64, f64)]) -> Result<Vec<(f64, f64)>, proj::ProjError> {
        coords.iter()
            .map(|&(x, y)| {
                let input = Coord { x, y, z: 0.0, t: f64::NAN };
                self.inner.convert(input).map(|c| (c.x, c.y))
            })
            .collect()
    }
}

impl Elicitation for ProjTransform {
    type Error = String;

    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Interactive CRS selection
        // Option 1: EPSG codes
        // Option 2: Well-known names (WGS84, UTM, etc.)
        // Option 3: Custom PROJ string
        todo!("Interactive projection selection")
    }
}
```

**2.2 Common Projections** (convenience constructors):

```rust
/// Common EPSG codes
pub mod epsg {
    pub const WGS84: u16 = 4326;           // World Geodetic System 1984
    pub const WEB_MERCATOR: u16 = 3857;    // Web Mercator (Google Maps)
    pub const NAD83: u16 = 4269;           // North American Datum 1983
    pub const UTM_ZONE_10N: u16 = 32610;   // UTM Zone 10N
    pub const STATE_PLANE_CA_III: u16 = 2227; // California State Plane Zone III
}

impl ProjTransform {
    /// WGS84 → Web Mercator (common for web mapping)
    pub fn wgs84_to_web_mercator() -> Result<Self, proj::ProjError> {
        Self::from_epsg(epsg::WGS84, epsg::WEB_MERCATOR)
    }

    /// Web Mercator → WGS84 (reverse)
    pub fn web_mercator_to_wgs84() -> Result<Self, proj::ProjError> {
        Self::from_epsg(epsg::WEB_MERCATOR, epsg::WGS84)
    }

    /// WGS84 → UTM Zone N (auto-select zone from longitude)
    pub fn wgs84_to_utm(lon: f64, north: bool) -> Result<Self, proj::ProjError> {
        let zone = ((lon + 180.0) / 6.0).floor() as u16 + 1;
        let epsg = if north {
            32600 + zone  // UTM North
        } else {
            32700 + zone  // UTM South
        };
        Self::from_epsg(epsg::WGS84, epsg)
    }
}
```

**2.3 Export from lib.rs** (`crates/elicitation/src/lib.rs`):
```rust
#[cfg(feature = "proj")]
pub mod proj_types;

#[cfg(feature = "proj")]
pub use proj_types::{ProjTransform, epsg};
```

## Phase 3: Create elicit_proj Shadow Crate

### Directory Structure:

```
crates/elicit_proj/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── transform.rs       (ProjTransform wrapper)
│   ├── info.rs            (ProjInfo wrapper)
│   ├── area.rs            (Area calculations)
│   └── workflow/
│       ├── mod.rs
│       ├── transform_plugin.rs   (~10 tools: create, convert, batch)
│       ├── epsg_plugin.rs        (~8 tools: lookup, search, info)
│       ├── geodesic_plugin.rs    (~6 tools: distance, area, bearing)
│       ├── custom_plugin.rs      (~5 tools: PROJ strings, WKT)
│       ├── batch_plugin.rs       (~6 tools: array transforms, pipelines)
│       ├── info_plugin.rs        (~5 tools: CRS metadata)
│       └── workflow_plugin.rs    (~5 tools: common workflows)
└── tests/
    └── proj_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_proj"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled PROJ wrappers with comprehensive MCP tools for coordinate reference system transformations"
keywords = ["mcp", "proj", "gis", "coordinate", "elicitation"]
categories = ["science::geo", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["proj", "geo-types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
proj = { workspace = true }
geo-types = { workspace = true }  # For interop
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
uuid = { workspace = true }

# Code emission
proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[dev-dependencies]
tokio = { workspace = true }

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

### lib.rs structure:

```rust
//! `elicit_proj` — comprehensive PROJ API exposure via MCP tools.
//!
//! Provides complete coverage of PROJ coordinate reference system operations:
//! - CRS transformations (WGS84, UTM, State Plane, Web Mercator, etc.)
//! - EPSG code lookup and metadata
//! - Geodesic calculations (distance, area, bearing on ellipsoid)
//! - Custom projections from PROJ strings and WKT
//! - Batch transformations for performance
//! - Coordinate system introspection
//!
//! # Plugin Organization (7 plugins, ~45 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `ProjTransformPlugin` | 10 | Create transforms, convert coordinates |
//! | `ProjEpsgPlugin` | 8 | EPSG lookup, search, metadata |
//! | `ProjGeodesicPlugin` | 6 | Distance, area, bearing calculations |
//! | `ProjCustomPlugin` | 5 | PROJ strings, WKT, custom CRS |
//! | `ProjBatchPlugin` | 6 | Array transforms, pipelines |
//! | `ProjInfoPlugin` | 5 | CRS metadata, axis info |
//! | `ProjWorkflowPlugin` | 5 | Common transformation workflows |
//!
//! # Integration with GeoRust
//!
//! Works seamlessly with geo-types for geometric transformations:
//!
//! ```rust
//! use elicit_proj::ProjTransform;
//! use geo_types::{Point, Polygon};
//!
//! let transform = ProjTransform::wgs84_to_utm(lon, true)?;
//!
//! // Transform Point
//! let point = Point::new(-122.4194, 37.7749);  // San Francisco
//! let utm_point = transform.transform_point(&point)?;
//!
//! // Transform Polygon
//! let polygon = Polygon::new(/* ... */);
//! let utm_polygon = transform.transform_polygon(&polygon)?;
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod area;
mod info;
mod transform;
pub mod workflow;

pub use area::Area;
pub use info::ProjInfo;
pub use transform::ProjTransform;
pub use workflow::{
    ProjBatchPlugin, ProjCustomPlugin, ProjEpsgPlugin,
    ProjGeodesicPlugin, ProjInfoPlugin, ProjTransformPlugin,
    ProjWorkflowPlugin,
};

/// Common EPSG codes for quick reference
pub mod epsg {
    /// WGS84 (lat/lon) - EPSG:4326
    pub const WGS84: u16 = 4326;
    /// Web Mercator (Google Maps, OpenStreetMap) - EPSG:3857
    pub const WEB_MERCATOR: u16 = 3857;
    /// NAD83 (North America) - EPSG:4269
    pub const NAD83: u16 = 4269;
    /// NAD27 (legacy North America) - EPSG:4267
    pub const NAD27: u16 = 4267;
    /// WGS84 / Pseudo-Mercator - EPSG:3395
    pub const WORLD_MERCATOR: u16 = 3395;
}
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Transform Wrapper (transform.rs):

```rust
// transform.rs
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use proj::{Coord, Proj};
use geo_types::{Point, LineString, Polygon, Rect};

/// Wrapper around proj::Proj for coordinate transformations
pub struct ProjTransform {
    inner: Proj,
    from_crs: String,
    to_crs: String,
}

impl ProjTransform {
    /// Create transform from CRS definitions
    #[instrument(skip(from_crs, to_crs))]
    pub fn new(from_crs: &str, to_crs: &str) -> Result<Self, proj::ProjError> {
        Ok(Self {
            inner: Proj::new_known_crs(from_crs, to_crs, None)?,
            from_crs: from_crs.to_string(),
            to_crs: to_crs.to_string(),
        })
    }

    /// Create transform from EPSG codes
    #[instrument]
    pub fn from_epsg(from_epsg: u16, to_epsg: u16) -> Result<Self, proj::ProjError> {
        let from = format!("EPSG:{}", from_epsg);
        let to = format!("EPSG:{}", to_epsg);
        Self::new(&from, &to)
    }

    /// Convert single coordinate
    #[instrument(skip(self))]
    pub fn convert(&self, coord: Coord<f64>) -> Result<Coord<f64>, proj::ProjError> {
        self.inner.convert(coord)
    }

    /// Convert geo_types::Point
    #[instrument(skip(self))]
    pub fn transform_point(&self, point: &Point) -> Result<Point, proj::ProjError> {
        let coord = Coord { x: point.x(), y: point.y(), z: 0.0, t: f64::NAN };
        let result = self.inner.convert(coord)?;
        Ok(Point::new(result.x, result.y))
    }

    /// Convert geo_types::LineString
    #[instrument(skip(self))]
    pub fn transform_line_string(&self, line: &LineString) -> Result<LineString, proj::ProjError> {
        let coords: Result<Vec<_>, _> = line.coords()
            .map(|c| {
                let coord = Coord { x: c.x, y: c.y, z: 0.0, t: f64::NAN };
                self.inner.convert(coord).map(|r| (r.x, r.y).into())
            })
            .collect();
        Ok(LineString::new(coords?))
    }

    /// Convert geo_types::Polygon
    #[instrument(skip(self))]
    pub fn transform_polygon(&self, polygon: &Polygon) -> Result<Polygon, proj::ProjError> {
        let exterior = self.transform_line_string(polygon.exterior())?;
        let interiors: Result<Vec<_>, _> = polygon.interiors()
            .iter()
            .map(|i| self.transform_line_string(i))
            .collect();
        Ok(Polygon::new(exterior, interiors?))
    }

    /// Batch convert array of coordinates
    #[instrument(skip(self, coords))]
    pub fn convert_array(&self, coords: &[(f64, f64)]) -> Result<Vec<(f64, f64)>, proj::ProjError> {
        coords.iter()
            .map(|&(x, y)| {
                let input = Coord { x, y, z: 0.0, t: f64::NAN };
                self.inner.convert(input).map(|c| (c.x, c.y))
            })
            .collect()
    }

    /// Get source CRS identifier
    pub fn from_crs(&self) -> &str {
        &self.from_crs
    }

    /// Get target CRS identifier
    pub fn to_crs(&self) -> &str {
        &self.to_crs
    }
}
```

### 4.2 Info Wrapper (info.rs):

```rust
// info.rs
use proj::ProjInfo;

/// CRS metadata wrapper
#[derive(Debug, Clone)]
pub struct CrsInfo {
    pub id: String,
    pub description: String,
    pub definition: String,
    pub accuracy: Option<f64>,
}

impl CrsInfo {
    #[instrument]
    pub fn from_epsg(code: u16) -> Result<Self, proj::ProjError> {
        let proj_string = format!("EPSG:{}", code);
        let proj = proj::Proj::new(&proj_string)?;

        Ok(Self {
            id: proj_string,
            description: format!("EPSG:{} coordinate system", code),
            definition: proj.def()?.to_string(),
            accuracy: None,
        })
    }

    #[instrument]
    pub fn from_proj_string(definition: &str) -> Result<Self, proj::ProjError> {
        let proj = proj::Proj::new(definition)?;

        Ok(Self {
            id: "custom".to_string(),
            description: "Custom projection".to_string(),
            definition: proj.def()?.to_string(),
            accuracy: None,
        })
    }
}
```

### 4.3 Area Calculations (area.rs):

```rust
// area.rs
use geo_types::{Point, Polygon};

/// Geodesic area calculator (ellipsoidal Earth)
pub struct GeodesicArea;

impl GeodesicArea {
    /// Calculate area of polygon on WGS84 ellipsoid (square meters)
    #[instrument(skip(polygon))]
    pub fn area_wgs84(polygon: &Polygon) -> f64 {
        // Use proj's area calculation on ellipsoid
        // This is more accurate than planar area for large polygons
        todo!("Implement geodesic area")
    }

    /// Calculate distance between two points on WGS84 ellipsoid (meters)
    #[instrument]
    pub fn distance_wgs84(from: &Point, to: &Point) -> f64 {
        // Use proj's geodesic distance
        todo!("Implement geodesic distance")
    }

    /// Calculate bearing from point A to point B (degrees)
    #[instrument]
    pub fn bearing_wgs84(from: &Point, to: &Point) -> f64 {
        // Calculate initial bearing on great circle
        todo!("Implement geodesic bearing")
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Transform Plugin (workflow/transform_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTransformParams {
    pub from_crs: String,
    pub to_crs: String,
}

#[elicit_tool(
    plugin = "proj_transform",
    name = "proj_transform__create",
    description = "Create a coordinate transformation. CRS can be EPSG:#### or proj string.",
    emit = Auto
)]
async fn transform_create(p: CreateTransformParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created transform: {} → {}", p.from_crs, p.to_crs))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTransformEpsgParams {
    pub from_epsg: u16,
    pub to_epsg: u16,
}

#[elicit_tool(
    plugin = "proj_transform",
    name = "proj_transform__create_epsg",
    description = "Create transformation from EPSG codes. Common: 4326 (WGS84), 3857 (Web Mercator).",
    emit = Auto
)]
async fn transform_create_epsg(p: CreateTransformEpsgParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created transform: EPSG:{} → EPSG:{}", p.from_epsg, p.to_epsg))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConvertCoordParams {
    pub x: f64,
    pub y: f64,
    pub transform_id: String,
}

#[elicit_tool(
    plugin = "proj_transform",
    name = "proj_transform__convert_coord",
    description = "Convert a single coordinate using existing transform.",
    emit = Auto
)]
async fn transform_convert_coord(p: ConvertCoordParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted ({}, {}) using transform {}", p.x, p.y, p.transform_id))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConvertPointParams {
    pub lon: f64,
    pub lat: f64,
    pub from_epsg: Option<u16>,
    pub to_epsg: u16,
}

#[elicit_tool(
    plugin = "proj_transform",
    name = "proj_transform__convert_point",
    description = "Convert a point. Default from_epsg is 4326 (WGS84).",
    emit = Auto
)]
async fn transform_convert_point(p: ConvertPointParams) -> Result<CallToolResult, ErrorData> {
    let from = p.from_epsg.unwrap_or(4326);
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted ({}, {}) from EPSG:{} to EPSG:{}", p.lon, p.lat, from, p.to_epsg))
    ]))
}

// ... 6 more tools: convert_array, convert_polygon, inverse, etc.
```

### 5.2 EPSG Plugin (workflow/epsg_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EpsgInfoParams {
    pub code: u16,
}

#[elicit_tool(
    plugin = "proj_epsg",
    name = "proj_epsg__info",
    description = "Get information about an EPSG code.",
    emit = Auto
)]
async fn epsg_info(p: EpsgInfoParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("EPSG:{} metadata", p.code))
    ]))
}

#[elicit_tool(
    plugin = "proj_epsg",
    name = "proj_epsg__wgs84",
    description = "Get EPSG code for WGS84 (4326).",
    emit = Auto
)]
async fn epsg_wgs84(_: ()) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("EPSG:4326 - WGS84 World Geodetic System 1984")
    ]))
}

#[elicit_tool(
    plugin = "proj_epsg",
    name = "proj_epsg__web_mercator",
    description = "Get EPSG code for Web Mercator (3857).",
    emit = Auto
)]
async fn epsg_web_mercator(_: ()) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("EPSG:3857 - Web Mercator (Google Maps, OSM)")
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UtmZoneParams {
    pub lon: f64,
    pub north: bool,
}

#[elicit_tool(
    plugin = "proj_epsg",
    name = "proj_epsg__utm_zone",
    description = "Calculate UTM zone EPSG code from longitude.",
    emit = Auto
)]
async fn epsg_utm_zone(p: UtmZoneParams) -> Result<CallToolResult, ErrorData> {
    let zone = ((p.lon + 180.0) / 6.0).floor() as u16 + 1;
    let epsg = if p.north { 32600 + zone } else { 32700 + zone };
    Ok(CallToolResult::success(vec![
        Content::text(format!("UTM Zone {} {}: EPSG:{}", zone, if p.north { "N" } else { "S" }, epsg))
    ]))
}

// ... 4 more tools: search, common_crs, datum_info, etc.
```

### 5.3 Geodesic Plugin (workflow/geodesic_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DistanceParams {
    pub from_lon: f64,
    pub from_lat: f64,
    pub to_lon: f64,
    pub to_lat: f64,
}

#[elicit_tool(
    plugin = "proj_geodesic",
    name = "proj_geodesic__distance",
    description = "Calculate geodesic distance on WGS84 ellipsoid (meters).",
    emit = Auto
)]
async fn geodesic_distance(p: DistanceParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Distance from ({}, {}) to ({}, {})",
            p.from_lon, p.from_lat, p.to_lon, p.to_lat))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AreaParams {
    pub polygon_coords: Vec<(f64, f64)>,
}

#[elicit_tool(
    plugin = "proj_geodesic",
    name = "proj_geodesic__area",
    description = "Calculate geodesic area of polygon on WGS84 ellipsoid (square meters).",
    emit = Auto
)]
async fn geodesic_area(p: AreaParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Area of polygon with {} vertices", p.polygon_coords.len()))
    ]))
}

// ... 4 more tools: bearing, destination, intermediate, buffer
```

### 5.4 Workflow Plugin (workflow/workflow_plugin.rs):

```rust
#[elicit_tool(
    plugin = "proj_workflow",
    name = "proj_workflow__wgs84_to_web_mercator",
    description = "Common workflow: WGS84 (4326) → Web Mercator (3857).",
    emit = Auto
)]
async fn workflow_wgs84_to_web_mercator(_: ()) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("Created WGS84 → Web Mercator transform")
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AutoUtmParams {
    pub lon: f64,
    pub lat: f64,
}

#[elicit_tool(
    plugin = "proj_workflow",
    name = "proj_workflow__auto_utm",
    description = "Automatically select UTM zone from coordinates and create transform.",
    emit = Auto
)]
async fn workflow_auto_utm(p: AutoUtmParams) -> Result<CallToolResult, ErrorData> {
    let zone = ((p.lon + 180.0) / 6.0).floor() as u16 + 1;
    let north = p.lat >= 0.0;
    Ok(CallToolResult::success(vec![
        Content::text(format!("Auto-selected UTM Zone {} {}", zone, if north { "N" } else { "S" }))
    ]))
}

// ... 3 more tools: geojson_transform, batch_convert, pipeline
```

## Phase 6: Testing

### File to create:
- `crates/elicit_proj/tests/proj_test.rs`

### Test Coverage:

```rust
#[test]
fn test_wgs84_to_web_mercator() {
    let transform = ProjTransform::from_epsg(4326, 3857).unwrap();

    // San Francisco: -122.4194, 37.7749
    let coord = Coord { x: -122.4194, y: 37.7749, z: 0.0, t: f64::NAN };
    let result = transform.convert(coord).unwrap();

    // Web Mercator coordinates
    assert!((result.x - -13628993.0).abs() < 1.0);
    assert!((result.y - 4547846.0).abs() < 1.0);
}

#[test]
fn test_utm_auto_select() {
    // San Francisco is in UTM Zone 10N
    let lon = -122.4194;
    let zone = ((lon + 180.0) / 6.0).floor() as u16 + 1;
    assert_eq!(zone, 10);

    let epsg = 32600 + zone;  // 32610
    let transform = ProjTransform::from_epsg(4326, epsg).unwrap();

    let coord = Coord { x: lon, y: 37.7749, z: 0.0, t: f64::NAN };
    let result = transform.convert(coord).unwrap();

    // UTM coordinates
    assert!(result.x > 500000.0 && result.x < 600000.0);  // Easting
    assert!(result.y > 4000000.0 && result.y < 5000000.0); // Northing
}

#[test]
fn test_geo_types_integration() {
    let transform = ProjTransform::from_epsg(4326, 3857).unwrap();

    let point = Point::new(-122.4194, 37.7749);
    let transformed = transform.transform_point(&point).unwrap();

    assert!((transformed.x() - -13628993.0).abs() < 1.0);
    assert!((transformed.y() - 4547846.0).abs() < 1.0);
}

#[test]
fn test_batch_conversion() {
    let transform = ProjTransform::from_epsg(4326, 3857).unwrap();

    let coords = vec![
        (-122.4194, 37.7749),  // SF
        (-118.2437, 34.0522),  // LA
        (-73.9352, 40.7306),   // NYC
    ];

    let results = transform.convert_array(&coords).unwrap();
    assert_eq!(results.len(), 3);
}

#[test]
fn test_epsg_metadata() {
    let info = CrsInfo::from_epsg(4326).unwrap();
    assert!(info.definition.contains("WGS"));
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_proj/README.md`

### Content:

```markdown
# elicit_proj

Comprehensive elicitation-enabled wrappers around [`proj`](https://docs.rs/proj) for coordinate reference system transformations.

## Purpose

Provides the **coordinate transformation alphabet** — foundational MCP tools for:
- Converting between coordinate systems (WGS84, UTM, State Plane, Web Mercator)
- EPSG code lookup and metadata
- Geodesic calculations (distance, area, bearing on ellipsoid)
- Custom projections from PROJ strings
- Batch transformations for performance

## API Coverage

Exposes comprehensive PROJ operations via 7 plugin namespaces:

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `proj_transform` | 10 | Create transforms, convert coordinates |
| `proj_epsg` | 8 | EPSG lookup, metadata, common CRS |
| `proj_geodesic` | 6 | Distance, area, bearing on ellipsoid |
| `proj_custom` | 5 | PROJ strings, WKT, custom CRS |
| `proj_batch` | 6 | Array transforms, pipelines |
| `proj_info` | 5 | CRS metadata, axis info |
| `proj_workflow` | 5 | Common transformation workflows |

**Total: ~45 MCP tools**

## Usage

```rust
use elicit_proj::ProjTransform;
use geo_types::Point;

// WGS84 → Web Mercator (common for web maps)
let transform = ProjTransform::wgs84_to_web_mercator()?;

let sf = Point::new(-122.4194, 37.7749);  // San Francisco
let web_mercator = transform.transform_point(&sf)?;

// Auto-select UTM zone from coordinates
let utm_transform = ProjTransform::wgs84_to_utm(-122.4194, true)?;
let utm_point = utm_transform.transform_point(&sf)?;
```

## Common EPSG Codes

```rust
use elicit_proj::epsg;

epsg::WGS84           // 4326 - World Geodetic System
epsg::WEB_MERCATOR    // 3857 - Google Maps, OSM
epsg::NAD83           // 4269 - North America
```

## Integration with GeoRust

Works seamlessly with geo-types geometries:

```rust
use geo_types::{Point, LineString, Polygon};

let transform = ProjTransform::from_epsg(4326, 32610)?;

// Transform primitives
transform.transform_point(&point)?;
transform.transform_line_string(&line)?;
transform.transform_polygon(&polygon)?;

// Batch processing
transform.convert_array(&coords)?;
```

## Geodesic Calculations

Accurate calculations on WGS84 ellipsoid:

```rust
use elicit_proj::GeodesicArea;

// Distance between two points (meters)
let distance = GeodesicArea::distance_wgs84(&sf, &la);

// Area of polygon (square meters)
let area = GeodesicArea::area_wgs84(&polygon);

// Bearing from A to B (degrees)
let bearing = GeodesicArea::bearing_wgs84(&from, &to);
```

## Use Cases

**Web Mapping**: WGS84 (GPS) → Web Mercator (tiles)
**GIS Analysis**: Transform data to appropriate projected CRS
**GPS Processing**: Convert GPS coordinates to local coordinate systems
**Surveying**: State Plane, UTM transformations
**Geospatial Pipelines**: Chain multiple transformations
```

## Verification Steps

### After implementation:

**elicit_proj shadow crate**:
1. `cargo check -p elicit_proj`
2. `cargo test -p elicit_proj`
3. `cargo check -p elicitation --no-default-features --features proj`
4. `cargo test -p elicit_proj --features emit`

**Full workspace**:
1. `cargo check --all-features`
2. `cargo test --workspace --all-features`

### Manual verification:

**MCP tool functionality**:
1. Launch MCP server with elicit_proj plugin
2. Call `proj_transform__create_epsg` with 4326 → 3857
3. Call `proj_transform__convert_point` with SF coordinates
4. Verify JSON responses and emit mode code generation

**Type integration**:
1. Test `ProjTransform::from_epsg()` creation
2. Test coordinate conversion accuracy
3. Verify geo-types integration
4. Test batch transformations

## Critical Files

### To create:
- `crates/elicit_proj/Cargo.toml`
- `crates/elicit_proj/README.md`
- `crates/elicit_proj/src/lib.rs`
- `crates/elicit_proj/src/transform.rs`
- `crates/elicit_proj/src/info.rs`
- `crates/elicit_proj/src/area.rs`
- `crates/elicit_proj/src/workflow/mod.rs`
- `crates/elicit_proj/src/workflow/transform_plugin.rs`
- `crates/elicit_proj/src/workflow/epsg_plugin.rs`
- `crates/elicit_proj/src/workflow/geodesic_plugin.rs`
- `crates/elicit_proj/src/workflow/custom_plugin.rs`
- `crates/elicit_proj/src/workflow/batch_plugin.rs`
- `crates/elicit_proj/src/workflow/info_plugin.rs`
- `crates/elicit_proj/src/workflow/workflow_plugin.rs`
- `crates/elicit_proj/tests/proj_test.rs`
- `crates/elicitation/src/proj_types.rs`

### To modify:
- `Cargo.toml` — Add workspace members and dependencies
- `crates/elicitation/Cargo.toml` — Add proj feature
- `crates/elicitation/src/lib.rs` — Export proj types

## Implementation Order

1. **Phase 1**: Workspace configuration (30 min)
2. **Phase 2**: Core type integration in elicitation (2 hours)
3. **Phase 3**: Create elicit_proj structure (1 hour)
4. **Phase 4**: Implement type wrappers (3 hours)
5. **Phase 5**: Implement MCP tools (~45 tools) (8-10 hours)
6. **Phase 6**: Testing (1-2 hours)
7. **Phase 7**: Documentation (1 hour)

**Total estimated time**: 16-19 hours

## Notes

### Use Cases

**Web Mapping Workflows**:
- User uploads GPS tracks (WGS84) → convert to Web Mercator for display
- Tile generation requires Web Mercator coordinates
- Back to WGS84 for geocoding/routing APIs

**GIS Analysis**:
- Area calculations need equal-area projections (not WGS84!)
- Distance measurements on WGS84 ellipsoid (geodesic, not Euclidean)
- UTM for local surveying/engineering (preserves distances/angles)

**Geospatial Pipelines**:
- Ingest: Parse GeoJSON (WGS84) → transform to analysis CRS
- Process: Perform geometric operations in projected space
- Export: Transform back to WGS84 for interoperability

### Integration with GeoRust Stack

**elicit_geo_types** → Primitive shapes (Point, Polygon)
**elicit_geo** → Geometric algorithms (Contains, Intersects, Area)
**elicit_proj** → Coordinate transformations (WGS84 ↔ UTM ↔ Web Mercator)
**elicit_geojson** → Serialization (with CRS support)
**elicit_rstar** → Spatial indexing (in projected space)

Complete GIS pipeline:
1. Parse GeoJSON (WGS84) with elicit_geojson
2. Transform to UTM with elicit_proj
3. Index with elicit_rstar for queries
4. Geometric operations with elicit_geo
5. Transform back and export with elicit_geojson

### Technical Challenges

1. **Accuracy vs Performance**: Geodesic calculations are slower but accurate for large areas
2. **CRS Selection**: Choosing appropriate projection for analysis (UTM for local, equal-area for measurements)
3. **Datum Transformations**: NAD83 vs WGS84 differences matter for high-precision work
4. **Axis Order**: Some CRS use (lat, lon), others (lon, lat) - PROJ handles this
5. **Batch Optimization**: Converting arrays is more efficient than individual points

### Future Extensions

- **PROJ pipelines**: Chain multiple transformations
- **Grid shift files**: High-accuracy datum transformations
- **Vertical datums**: Height above ellipsoid vs mean sea level
- **Time-dependent transformations**: Tectonic plate motion corrections
