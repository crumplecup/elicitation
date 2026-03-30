# ELICIT_WKT_PLAN.md

## Goal

Add comprehensive Well-Known Text (WKT) format support to elicitation for geometry and CRS representation:

1. **Core type integration** — wkt types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_wkt` with MCP tools for WKT operations (~35 tools)
3. **GIS alphabet** — Foundation for text-based geometry interchange

## Use Cases

- **Geometry interchange**: Parse/serialize POINT, LINESTRING, POLYGON, etc.
- **CRS definitions**: WKT representation of coordinate reference systems
- **Database integration**: PostGIS, Oracle Spatial, SQL Server use WKT
- **Standards compliance**: OGC Simple Features specification
- **Human-readable format**: Debug and inspect geometries as text
- **Configuration files**: Define geometries in config without binary formats
- **SQL queries**: `ST_GeomFromText('POINT(-122 37)')` patterns

## WKT Format Examples

```wkt
POINT (30 10)
LINESTRING (30 10, 10 30, 40 40)
POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))
POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30))
MULTIPOINT ((10 40), (40 30), (20 20), (30 10))
MULTILINESTRING ((10 10, 20 20, 10 40), (40 40, 30 30, 40 20, 30 10))
MULTIPOLYGON (((30 20, 45 40, 10 40, 30 20)), ((15 5, 40 10, 10 20, 5 10, 15 5)))
GEOMETRYCOLLECTION (POINT (40 10), LINESTRING (10 10, 20 20, 10 40))
```

## Architecture Overview

Following established patterns from elicit_geojson, elicit_geo:
- **Core**: Feature-gated wkt types with Elicitation impls
- **Shadow crate**: 6 workflow plugins covering ~35 operations
- **GIS alphabet**: Text-based geometry representation for interoperability

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add wkt to workspace dependencies** (`Cargo.toml`):
```toml
# Geospatial - Well-Known Text
wkt = { version = "0.11", default-features = false }
```

**1.2 Add elicit_wkt member** (`Cargo.toml` after other elicit_* members):
```toml
  "crates/elicit_wkt",
```

**1.3 Add elicit_wkt workspace dependency** (`Cargo.toml`):
```toml
elicit_wkt = { path = "crates/elicit_wkt", version = "0.9.1" }
```

**1.4 Add wkt feature to elicitation** (`crates/elicitation/Cargo.toml`):
- Add optional dependency: `wkt = { workspace = true, optional = true }`
- Add feature: `wkt = ["dep:wkt"]`
- Update `gis` meta-feature: `gis = ["proj", "geo", "geo-types", "geojson", "rstar", "wkt"]`

## Phase 2: Core Type Integration in elicitation

### Files to create/modify:
- `crates/elicitation/src/wkt_types.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Core WKT Types**:

```rust
// crates/elicitation/src/wkt_types.rs
#![cfg(feature = "wkt")]

use wkt::{Wkt, Geometry};
use elicitation::{Elicitation, ElicitationContext};

/// Re-export wkt types
pub use wkt::{Wkt, Geometry, GeometryCollection};
pub use wkt::types::{Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon};
pub use wkt::types::{Coord};

/// WKT wrapper with conversion to/from geo_types
#[cfg(feature = "geo-types")]
pub struct WktGeometry {
    wkt: Wkt<f64>,
}

#[cfg(feature = "geo-types")]
impl WktGeometry {
    /// Parse WKT string into geometry
    pub fn from_str(wkt: &str) -> Result<Self, wkt::WktError> {
        Ok(Self {
            wkt: wkt.parse()?,
        })
    }

    /// Convert to geo_types geometry
    pub fn to_geo(&self) -> Result<geo_types::Geometry<f64>, String> {
        use wkt::conversion::try_into_geometry;

        self.wkt.items.first()
            .ok_or_else(|| "Empty WKT".to_string())
            .and_then(|item| try_into_geometry(item).map_err(|e| e.to_string()))
    }

    /// Create from geo_types geometry
    pub fn from_geo(geom: &geo_types::Geometry<f64>) -> Result<Self, String> {
        use wkt::conversion::geometry_to_wkt;

        Ok(Self {
            wkt: geometry_to_wkt(geom),
        })
    }

    /// Convert to WKT string
    pub fn to_wkt_string(&self) -> String {
        self.wkt.to_string()
    }
}

impl Elicitation for WktGeometry {
    type Error = String;

    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Interactive WKT input with validation
        // Could show geometry type selection first, then coordinates
        todo!("Interactive WKT geometry construction")
    }
}

/// CRS definition in WKT format
pub struct WktCrs {
    definition: String,
}

impl WktCrs {
    /// Parse WKT CRS definition
    pub fn from_str(wkt: &str) -> Result<Self, String> {
        // Validate WKT CRS format
        if !wkt.starts_with("GEOGCS[") && !wkt.starts_with("PROJCS[") {
            return Err("Invalid WKT CRS format".to_string());
        }
        Ok(Self {
            definition: wkt.to_string(),
        })
    }

    /// Get WKT string
    pub fn as_str(&self) -> &str {
        &self.definition
    }

    /// Extract CRS name
    pub fn name(&self) -> Option<String> {
        // Parse name from WKT (first quoted string)
        self.definition
            .split('"')
            .nth(1)
            .map(|s| s.to_string())
    }
}
```

**2.2 Geometry Type Helpers**:

```rust
/// WKT geometry type discriminator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum WktGeometryType {
    Point,
    LineString,
    Polygon,
    MultiPoint,
    MultiLineString,
    MultiPolygon,
    GeometryCollection,
}

impl WktGeometryType {
    /// Get WKT type name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Point => "POINT",
            Self::LineString => "LINESTRING",
            Self::Polygon => "POLYGON",
            Self::MultiPoint => "MULTIPOINT",
            Self::MultiLineString => "MULTILINESTRING",
            Self::MultiPolygon => "MULTIPOLYGON",
            Self::GeometryCollection => "GEOMETRYCOLLECTION",
        }
    }

    /// Parse from WKT string (detect type)
    pub fn from_wkt(wkt: &str) -> Result<Self, String> {
        let upper = wkt.trim().to_uppercase();
        if upper.starts_with("POINT") {
            Ok(Self::Point)
        } else if upper.starts_with("LINESTRING") {
            Ok(Self::LineString)
        } else if upper.starts_with("POLYGON") {
            Ok(Self::Polygon)
        } else if upper.starts_with("MULTIPOINT") {
            Ok(Self::MultiPoint)
        } else if upper.starts_with("MULTILINESTRING") {
            Ok(Self::MultiLineString)
        } else if upper.starts_with("MULTIPOLYGON") {
            Ok(Self::MultiPolygon)
        } else if upper.starts_with("GEOMETRYCOLLECTION") {
            Ok(Self::GeometryCollection)
        } else {
            Err(format!("Unknown WKT geometry type: {}", wkt))
        }
    }
}
```

**2.3 Export from lib.rs** (`crates/elicitation/src/lib.rs`):
```rust
#[cfg(feature = "wkt")]
pub mod wkt_types;

#[cfg(feature = "wkt")]
pub use wkt_types::{WktGeometry, WktCrs, WktGeometryType};
```

## Phase 3: Create elicit_wkt Shadow Crate

### Directory Structure:

```
crates/elicit_wkt/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── geometry.rs        (WKT geometry wrapper)
│   ├── crs.rs             (WKT CRS definitions)
│   ├── parser.rs          (WKT parsing utilities)
│   └── workflow/
│       ├── mod.rs
│       ├── parse_plugin.rs      (~8 tools: parse WKT geometries)
│       ├── serialize_plugin.rs  (~7 tools: to WKT string)
│       ├── convert_plugin.rs    (~6 tools: WKT ↔ geo_types)
│       ├── crs_plugin.rs        (~5 tools: CRS definitions)
│       ├── validate_plugin.rs   (~4 tools: WKT validation)
│       └── workflow_plugin.rs   (~5 tools: common workflows)
└── tests/
    └── wkt_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_wkt"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled WKT wrappers with comprehensive MCP tools for Well-Known Text geometry representation"
keywords = ["mcp", "wkt", "gis", "geometry", "elicitation"]
categories = ["science::geo", "development-tools", "parsing"]

[dependencies]
elicitation = { workspace = true, features = ["wkt", "geo-types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
wkt = { workspace = true }
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
//! `elicit_wkt` — comprehensive WKT (Well-Known Text) API exposure via MCP tools.
//!
//! Provides complete coverage of WKT geometry and CRS operations:
//! - Parse WKT strings (POINT, LINESTRING, POLYGON, etc.)
//! - Serialize geometries to WKT format
//! - Convert WKT ↔ geo_types for geometric operations
//! - CRS definitions in WKT format
//! - WKT validation and introspection
//!
//! # Plugin Organization (6 plugins, ~35 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `WktParsePlugin` | 8 | Parse WKT strings to geometries |
//! | `WktSerializePlugin` | 7 | Serialize geometries to WKT |
//! | `WktConvertPlugin` | 6 | WKT ↔ geo_types conversion |
//! | `WktCrsPlugin` | 5 | CRS definitions in WKT |
//! | `WktValidatePlugin` | 4 | WKT validation and inspection |
//! | `WktWorkflowPlugin` | 5 | Common WKT workflows |
//!
//! # WKT Format Support
//!
//! Supports all OGC Simple Features geometries:
//! - POINT (x y)
//! - LINESTRING (x1 y1, x2 y2, ...)
//! - POLYGON ((x1 y1, x2 y2, ...), (hole1), ...)
//! - MULTIPOINT ((x1 y1), (x2 y2), ...)
//! - MULTILINESTRING ((...), (...))
//! - MULTIPOLYGON (((...)), ((...)))
//! - GEOMETRYCOLLECTION (geom1, geom2, ...)
//!
//! # Integration with GeoRust
//!
//! Seamless conversion to/from geo_types:
//!
//! ```rust
//! use elicit_wkt::WktGeometry;
//! use geo_types::Point;
//!
//! // Parse WKT
//! let wkt = WktGeometry::from_str("POINT (30 10)")?;
//! let point: Point<f64> = wkt.try_into()?;
//!
//! // To WKT
//! let wkt = WktGeometry::from_geo(&point.into())?;
//! assert_eq!(wkt.to_string(), "POINT(30 10)");
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod crs;
mod geometry;
mod parser;
pub mod workflow;

pub use crs::WktCrs;
pub use geometry::WktGeometry;
pub use parser::WktParser;
pub use workflow::{
    WktConvertPlugin, WktCrsPlugin, WktParsePlugin,
    WktSerializePlugin, WktValidatePlugin, WktWorkflowPlugin,
};

/// WKT geometry type discriminator
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeometryType {
    /// POINT (x y)
    Point,
    /// LINESTRING (x1 y1, x2 y2, ...)
    LineString,
    /// POLYGON ((exterior), (hole1), ...)
    Polygon,
    /// MULTIPOINT ((x1 y1), (x2 y2), ...)
    MultiPoint,
    /// MULTILINESTRING ((line1), (line2), ...)
    MultiLineString,
    /// MULTIPOLYGON ((poly1), (poly2), ...)
    MultiPolygon,
    /// GEOMETRYCOLLECTION (geom1, geom2, ...)
    GeometryCollection,
}
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Geometry Wrapper (geometry.rs):

```rust
// geometry.rs
use elicitation_derive::reflect_methods;
use geo_types::Geometry;
use wkt::Wkt;

/// Wrapper around WKT geometry with geo_types conversion
#[derive(Debug, Clone)]
pub struct WktGeometry {
    wkt: Wkt<f64>,
}

#[reflect_methods]
impl WktGeometry {
    /// Parse WKT string
    #[instrument(skip(wkt_string))]
    pub fn from_str(wkt_string: &str) -> Result<Self, wkt::WktError> {
        Ok(Self {
            wkt: wkt_string.parse()?,
        })
    }

    /// Create from geo_types geometry
    #[instrument(skip(geometry))]
    pub fn from_geo(geometry: &Geometry<f64>) -> Self {
        use wkt::conversion::geometry_to_wkt;
        Self {
            wkt: geometry_to_wkt(geometry),
        }
    }

    /// Convert to geo_types geometry
    #[instrument(skip(self))]
    pub fn to_geo(&self) -> Result<Geometry<f64>, String> {
        use wkt::conversion::try_into_geometry;

        self.wkt.items.first()
            .ok_or_else(|| "Empty WKT".to_string())
            .and_then(|item| try_into_geometry(item).map_err(|e| e.to_string()))
    }

    /// Get WKT string representation
    #[instrument(skip(self))]
    pub fn to_wkt_string(&self) -> String {
        self.wkt.to_string()
    }

    /// Get geometry type
    #[instrument(skip(self))]
    pub fn geometry_type(&self) -> Option<crate::GeometryType> {
        use wkt::Geometry as WktGeom;

        self.wkt.items.first().map(|item| {
            match item {
                WktGeom::Point(_) => crate::GeometryType::Point,
                WktGeom::LineString(_) => crate::GeometryType::LineString,
                WktGeom::Polygon(_) => crate::GeometryType::Polygon,
                WktGeom::MultiPoint(_) => crate::GeometryType::MultiPoint,
                WktGeom::MultiLineString(_) => crate::GeometryType::MultiLineString,
                WktGeom::MultiPolygon(_) => crate::GeometryType::MultiPolygon,
                WktGeom::GeometryCollection(_) => crate::GeometryType::GeometryCollection,
            }
        })
    }

    /// Check if WKT is valid
    #[instrument(skip(wkt_string))]
    pub fn validate(wkt_string: &str) -> Result<(), String> {
        wkt_string.parse::<Wkt<f64>>()
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    /// Get number of coordinates
    #[instrument(skip(self))]
    pub fn coordinate_count(&self) -> usize {
        // Count all coordinates in geometry
        todo!("Implement coordinate counting")
    }
}

// Conversion traits
impl TryFrom<WktGeometry> for geo_types::Point<f64> {
    type Error = String;

    fn try_from(wkt: WktGeometry) -> Result<Self, Self::Error> {
        match wkt.to_geo()? {
            Geometry::Point(p) => Ok(p),
            _ => Err("Not a Point".to_string()),
        }
    }
}

impl TryFrom<WktGeometry> for geo_types::LineString<f64> {
    type Error = String;

    fn try_from(wkt: WktGeometry) -> Result<Self, Self::Error> {
        match wkt.to_geo()? {
            Geometry::LineString(ls) => Ok(ls),
            _ => Err("Not a LineString".to_string()),
        }
    }
}

impl TryFrom<WktGeometry> for geo_types::Polygon<f64> {
    type Error = String;

    fn try_from(wkt: WktGeometry) -> Result<Self, Self::Error> {
        match wkt.to_geo()? {
            Geometry::Polygon(p) => Ok(p),
            _ => Err("Not a Polygon".to_string()),
        }
    }
}
```

### 4.2 CRS Wrapper (crs.rs):

```rust
// crs.rs
use elicitation_derive::reflect_methods;

/// WKT CRS definition wrapper
#[derive(Debug, Clone)]
pub struct WktCrs {
    definition: String,
    name: Option<String>,
}

#[reflect_methods]
impl WktCrs {
    /// Parse WKT CRS definition
    #[instrument(skip(wkt))]
    pub fn from_str(wkt: &str) -> Result<Self, String> {
        // Validate basic WKT CRS format
        let trimmed = wkt.trim();
        if !trimmed.starts_with("GEOGCS[") && !trimmed.starts_with("PROJCS[") {
            return Err(format!("Invalid WKT CRS format: must start with GEOGCS[ or PROJCS["));
        }

        let name = Self::extract_name(trimmed);

        Ok(Self {
            definition: trimmed.to_string(),
            name,
        })
    }

    /// Extract CRS name from WKT
    fn extract_name(wkt: &str) -> Option<String> {
        // Name is first quoted string after GEOGCS[ or PROJCS[
        wkt.split('"')
            .nth(1)
            .map(|s| s.to_string())
    }

    /// Get WKT definition string
    pub fn definition(&self) -> &str {
        &self.definition
    }

    /// Get CRS name if available
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Check if this is a geographic CRS (GEOGCS)
    #[instrument(skip(self))]
    pub fn is_geographic(&self) -> bool {
        self.definition.starts_with("GEOGCS[")
    }

    /// Check if this is a projected CRS (PROJCS)
    #[instrument(skip(self))]
    pub fn is_projected(&self) -> bool {
        self.definition.starts_with("PROJCS[")
    }
}

/// Common WKT CRS definitions
pub mod common {
    /// WGS84 geographic CRS
    pub const WGS84: &str = r#"GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4326"]]"#;

    /// Web Mercator projected CRS
    pub const WEB_MERCATOR: &str = r#"PROJCS["WGS 84 / Pseudo-Mercator",GEOGCS["WGS 84",DATUM["WGS_1984",SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],AUTHORITY["EPSG","6326"]],PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],AUTHORITY["EPSG","4326"]],PROJECTION["Mercator_1SP"],PARAMETER["central_meridian",0],PARAMETER["scale_factor",1],PARAMETER["false_easting",0],PARAMETER["false_nortihng",0],UNIT["metre",1,AUTHORITY["EPSG","9001"]],AXIS["X",EAST],AXIS["Y",NORTH],AUTHORITY["EPSG","3857"]]"#;
}
```

### 4.3 Parser Utilities (parser.rs):

```rust
// parser.rs
use elicitation_derive::reflect_methods;

/// WKT parsing utilities
pub struct WktParser;

#[reflect_methods]
impl WktParser {
    /// Extract geometry type from WKT string
    #[instrument(skip(wkt))]
    pub fn detect_geometry_type(wkt: &str) -> Result<crate::GeometryType, String> {
        let upper = wkt.trim().to_uppercase();

        if upper.starts_with("POINT") {
            Ok(crate::GeometryType::Point)
        } else if upper.starts_with("LINESTRING") {
            Ok(crate::GeometryType::LineString)
        } else if upper.starts_with("POLYGON") {
            Ok(crate::GeometryType::Polygon)
        } else if upper.starts_with("MULTIPOINT") {
            Ok(crate::GeometryType::MultiPoint)
        } else if upper.starts_with("MULTILINESTRING") {
            Ok(crate::GeometryType::MultiLineString)
        } else if upper.starts_with("MULTIPOLYGON") {
            Ok(crate::GeometryType::MultiPolygon)
        } else if upper.starts_with("GEOMETRYCOLLECTION") {
            Ok(crate::GeometryType::GeometryCollection)
        } else {
            Err(format!("Unknown WKT geometry type in: {}", wkt))
        }
    }

    /// Validate WKT format without parsing
    #[instrument(skip(wkt))]
    pub fn quick_validate(wkt: &str) -> bool {
        let upper = wkt.trim().to_uppercase();

        // Check basic structure
        let has_type = upper.starts_with("POINT") ||
                      upper.starts_with("LINESTRING") ||
                      upper.starts_with("POLYGON") ||
                      upper.starts_with("MULTI") ||
                      upper.starts_with("GEOMETRYCOLLECTION");

        let has_parens = wkt.contains('(') && wkt.contains(')');

        has_type && has_parens
    }

    /// Pretty-print WKT with indentation
    #[instrument(skip(wkt))]
    pub fn pretty_print(wkt: &str) -> String {
        // Add indentation for nested structures
        todo!("Implement WKT pretty printing")
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Parse Plugin (workflow/parse_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParseWktParams {
    pub wkt: String,
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_parse__geometry",
    description = "Parse WKT string into geometry object. Returns geometry type and properties.",
    emit = Auto
)]
async fn parse_geometry(p: ParseWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed WKT: {}", p.wkt))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParsePointParams {
    pub wkt: String,
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_parse__point",
    description = "Parse WKT POINT. Example: 'POINT (30 10)' or 'POINT (30 10 5)' for 3D.",
    emit = Auto
)]
async fn parse_point(p: ParsePointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed POINT from: {}", p.wkt))
    ]))
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_parse__linestring",
    description = "Parse WKT LINESTRING. Example: 'LINESTRING (30 10, 10 30, 40 40)'.",
    emit = Auto
)]
async fn parse_linestring(p: ParseWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed LINESTRING from: {}", p.wkt))
    ]))
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_parse__polygon",
    description = "Parse WKT POLYGON with optional holes. Example: 'POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))'.",
    emit = Auto
)]
async fn parse_polygon(p: ParseWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed POLYGON from: {}", p.wkt))
    ]))
}

// ... 4 more tools: parse_multipoint, parse_multilinestring, parse_multipolygon, parse_collection
```

### 5.2 Serialize Plugin (workflow/serialize_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PointToWktParams {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[elicit_tool(
    plugin = "wkt_serialize",
    name = "wkt_serialize__point",
    description = "Create WKT POINT from coordinates. Optional z for 3D.",
    emit = Auto
)]
async fn serialize_point(p: PointToWktParams) -> Result<CallToolResult, ErrorData> {
    let wkt = if let Some(z) = p.z {
        format!("POINT ({} {} {})", p.x, p.y, z)
    } else {
        format!("POINT ({} {})", p.x, p.y)
    };

    Ok(CallToolResult::success(vec![
        Content::text(format!("WKT: {}", wkt))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LineStringToWktParams {
    pub coords: Vec<(f64, f64)>,
}

#[elicit_tool(
    plugin = "wkt_serialize",
    name = "wkt_serialize__linestring",
    description = "Create WKT LINESTRING from coordinate array.",
    emit = Auto
)]
async fn serialize_linestring(p: LineStringToWktParams) -> Result<CallToolResult, ErrorData> {
    let coords_str = p.coords.iter()
        .map(|(x, y)| format!("{} {}", x, y))
        .collect::<Vec<_>>()
        .join(", ");

    let wkt = format!("LINESTRING ({})", coords_str);

    Ok(CallToolResult::success(vec![
        Content::text(format!("WKT: {}", wkt))
    ]))
}

// ... 5 more tools: serialize_polygon, serialize_multi*, to_wkt_string, pretty_print
```

### 5.3 Convert Plugin (workflow/convert_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WktToGeoParams {
    pub wkt: String,
}

#[elicit_tool(
    plugin = "wkt_convert",
    name = "wkt_convert__to_geo",
    description = "Convert WKT string to geo_types geometry for geometric operations.",
    emit = Auto
)]
async fn convert_to_geo(p: WktToGeoParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted WKT to geo_types: {}", p.wkt))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GeoToWktParams {
    pub geometry_json: String,  // geo_types serialized as JSON
}

#[elicit_tool(
    plugin = "wkt_convert",
    name = "wkt_convert__from_geo",
    description = "Convert geo_types geometry to WKT string.",
    emit = Auto
)]
async fn convert_from_geo(p: GeoToWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted geo_types to WKT"))
    ]))
}

// ... 4 more tools: to_geojson, from_geojson, batch_convert, validate_conversion
```

### 5.4 CRS Plugin (workflow/crs_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParseCrsParams {
    pub wkt_crs: String,
}

#[elicit_tool(
    plugin = "wkt_crs",
    name = "wkt_crs__parse",
    description = "Parse WKT CRS definition (GEOGCS or PROJCS).",
    emit = Auto
)]
async fn parse_crs(p: ParseCrsParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed CRS: {}", p.wkt_crs))
    ]))
}

#[elicit_tool(
    plugin = "wkt_crs",
    name = "wkt_crs__wgs84",
    description = "Get WGS84 CRS definition in WKT format.",
    emit = Auto
)]
async fn crs_wgs84(_: ()) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("WKT CRS: {}", crate::crs::common::WGS84))
    ]))
}

// ... 3 more tools: crs_web_mercator, extract_name, is_geographic
```

### 5.5 Validate Plugin (workflow/validate_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidateWktParams {
    pub wkt: String,
}

#[elicit_tool(
    plugin = "wkt_validate",
    name = "wkt_validate__check",
    description = "Validate WKT string format without full parsing.",
    emit = Auto
)]
async fn validate_check(p: ValidateWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Validated: {}", p.wkt))
    ]))
}

#[elicit_tool(
    plugin = "wkt_validate",
    name = "wkt_validate__detect_type",
    description = "Detect geometry type from WKT string.",
    emit = Auto
)]
async fn validate_detect_type(p: ValidateWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Detected type in: {}", p.wkt))
    ]))
}

// ... 2 more tools: count_coords, inspect
```

### 5.6 Workflow Plugin (workflow/workflow_plugin.rs):

```rust
#[elicit_tool(
    plugin = "wkt_workflow",
    name = "wkt_workflow__postgis_insert",
    description = "Generate PostGIS INSERT with ST_GeomFromText. Example for database inserts.",
    emit = Auto
)]
async fn workflow_postgis_insert(p: WktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("INSERT INTO table (geom) VALUES (ST_GeomFromText('{}', 4326));", p.wkt))
    ]))
}

#[elicit_tool(
    plugin = "wkt_workflow",
    name = "wkt_workflow__round_trip",
    description = "Test WKT → geo_types → WKT round-trip conversion.",
    emit = Auto
)]
async fn workflow_round_trip(p: WktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Round-trip test for: {}", p.wkt))
    ]))
}

// ... 3 more tools: batch_postgis, format_for_sql, validate_and_convert
```

## Phase 6: Testing

### File to create:
- `crates/elicit_wkt/tests/wkt_test.rs`

### Test Coverage:

```rust
#[test]
fn test_parse_point() {
    let wkt = WktGeometry::from_str("POINT (30 10)").unwrap();
    assert_eq!(wkt.geometry_type(), Some(GeometryType::Point));

    let point: geo_types::Point<f64> = wkt.try_into().unwrap();
    assert_eq!(point.x(), 30.0);
    assert_eq!(point.y(), 10.0);
}

#[test]
fn test_parse_linestring() {
    let wkt = WktGeometry::from_str("LINESTRING (30 10, 10 30, 40 40)").unwrap();
    assert_eq!(wkt.geometry_type(), Some(GeometryType::LineString));

    let line: geo_types::LineString<f64> = wkt.try_into().unwrap();
    assert_eq!(line.coords().count(), 3);
}

#[test]
fn test_parse_polygon() {
    let wkt = WktGeometry::from_str("POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))").unwrap();
    assert_eq!(wkt.geometry_type(), Some(GeometryType::Polygon));

    let poly: geo_types::Polygon<f64> = wkt.try_into().unwrap();
    assert_eq!(poly.exterior().coords().count(), 5);
}

#[test]
fn test_polygon_with_hole() {
    let wkt = WktGeometry::from_str(
        "POLYGON ((35 10, 45 45, 15 40, 10 20, 35 10), (20 30, 35 35, 30 20, 20 30))"
    ).unwrap();

    let poly: geo_types::Polygon<f64> = wkt.try_into().unwrap();
    assert_eq!(poly.interiors().len(), 1);
}

#[test]
fn test_round_trip() {
    let original = geo_types::Point::new(30.0, 10.0);
    let wkt = WktGeometry::from_geo(&original.into());
    let point: geo_types::Point<f64> = wkt.try_into().unwrap();

    assert_eq!(point.x(), 30.0);
    assert_eq!(point.y(), 10.0);
}

#[test]
fn test_wkt_string_output() {
    let wkt = WktGeometry::from_str("POINT (30 10)").unwrap();
    let output = wkt.to_wkt_string();

    // Note: output format may vary slightly
    assert!(output.contains("POINT"));
    assert!(output.contains("30"));
    assert!(output.contains("10"));
}

#[test]
fn test_crs_wgs84() {
    let crs = WktCrs::from_str(crate::crs::common::WGS84).unwrap();
    assert!(crs.is_geographic());
    assert!(!crs.is_projected());
    assert_eq!(crs.name(), Some("WGS 84"));
}

#[test]
fn test_crs_web_mercator() {
    let crs = WktCrs::from_str(crate::crs::common::WEB_MERCATOR).unwrap();
    assert!(!crs.is_geographic());
    assert!(crs.is_projected());
}

#[test]
fn test_validate() {
    assert!(WktGeometry::validate("POINT (30 10)").is_ok());
    assert!(WktGeometry::validate("INVALID").is_err());
}

#[test]
fn test_detect_geometry_type() {
    assert_eq!(
        WktParser::detect_geometry_type("POINT (30 10)").unwrap(),
        GeometryType::Point
    );
    assert_eq!(
        WktParser::detect_geometry_type("LINESTRING (30 10, 10 30)").unwrap(),
        GeometryType::LineString
    );
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_wkt/README.md`

### Content:

```markdown
# elicit_wkt

Comprehensive elicitation-enabled wrappers around [`wkt`](https://docs.rs/wkt) for Well-Known Text geometry representation.

## Purpose

Provides the **WKT alphabet** — foundational MCP tools for:
- Parsing WKT geometry strings (POINT, LINESTRING, POLYGON, etc.)
- Serializing geometries to WKT format
- Converting WKT ↔ geo_types for geometric operations
- WKT CRS definitions (GEOGCS, PROJCS)
- Database integration (PostGIS, Oracle Spatial, SQL Server)
- Human-readable geometry debugging

## API Coverage

Exposes comprehensive WKT operations via 6 plugin namespaces:

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `wkt_parse` | 8 | Parse WKT strings to geometries |
| `wkt_serialize` | 7 | Serialize geometries to WKT |
| `wkt_convert` | 6 | WKT ↔ geo_types conversion |
| `wkt_crs` | 5 | CRS definitions in WKT |
| `wkt_validate` | 4 | WKT validation and inspection |
| `wkt_workflow` | 5 | Common WKT workflows |

**Total: ~35 MCP tools**

## Usage

```rust
use elicit_wkt::WktGeometry;
use geo_types::Point;

// Parse WKT
let wkt = WktGeometry::from_str("POINT (30 10)")?;
let point: Point<f64> = wkt.try_into()?;

// Create WKT
let point = Point::new(-122.4194, 37.7749);
let wkt = WktGeometry::from_geo(&point.into());
println!("{}", wkt.to_wkt_string());
// Output: POINT(-122.4194 37.7749)
```

## Supported Geometry Types

- **POINT** — Single coordinate: `POINT (30 10)`
- **LINESTRING** — Sequence: `LINESTRING (30 10, 10 30, 40 40)`
- **POLYGON** — Exterior + holes: `POLYGON ((30 10, 40 40, 20 40, 10 20, 30 10))`
- **MULTIPOINT** — Point collection: `MULTIPOINT ((10 40), (40 30), (20 20))`
- **MULTILINESTRING** — Line collection
- **MULTIPOLYGON** — Polygon collection
- **GEOMETRYCOLLECTION** — Mixed types

## Database Integration

Generate PostGIS-compatible SQL:

```rust
use elicit_wkt::WktGeometry;

let wkt = WktGeometry::from_str("POINT (-122.4194 37.7749)")?;

// PostGIS INSERT
let sql = format!(
    "INSERT INTO locations (geom) VALUES (ST_GeomFromText('{}', 4326));",
    wkt.to_wkt_string()
);
```

## CRS Support

Common WKT CRS definitions:

```rust
use elicit_wkt::crs::common;

let wgs84_crs = WktCrs::from_str(common::WGS84)?;
let web_mercator_crs = WktCrs::from_str(common::WEB_MERCATOR)?;

assert!(wgs84_crs.is_geographic());
assert!(web_mercator_crs.is_projected());
```

## Integration with GeoRust

Seamless conversion to geo_types for geometric operations:

```rust
use elicit_wkt::WktGeometry;
use geo::{Area, Contains};

// Parse WKT polygon
let wkt = WktGeometry::from_str("POLYGON ((0 0, 10 0, 10 10, 0 10, 0 0))")?;
let polygon: geo_types::Polygon<f64> = wkt.try_into()?;

// Geometric operations
let area = polygon.unsigned_area();
let contains = polygon.contains(&geo_types::Point::new(5.0, 5.0));

// Back to WKT
let output_wkt = WktGeometry::from_geo(&polygon.into());
```

## Use Cases

**Database I/O**: PostGIS, Oracle Spatial, SQL Server geometry columns
**Configuration**: Define geometries in config files (human-readable)
**Debugging**: Inspect geometries as text during development
**Interoperability**: Exchange geometries with other GIS systems
**Standards Compliance**: OGC Simple Features WKT format
**Web APIs**: Text-based geometry in JSON responses
```

## Verification Steps

### After implementation:

**elicit_wkt shadow crate**:
1. `cargo check -p elicit_wkt`
2. `cargo test -p elicit_wkt`
3. `cargo check -p elicitation --no-default-features --features wkt`
4. `cargo test -p elicit_wkt --features emit`

**Full workspace**:
1. `cargo check --all-features`
2. `cargo test --workspace --all-features`

### Manual verification:

**MCP tool functionality**:
1. Launch MCP server with elicit_wkt plugin
2. Call `wkt_parse__point` with "POINT (30 10)"
3. Call `wkt_serialize__linestring` with coordinate array
4. Verify JSON responses and emit mode code generation

**Type integration**:
1. Test WKT parsing accuracy
2. Test round-trip conversion (WKT → geo → WKT)
3. Verify CRS parsing
4. Test validation and error handling

## Critical Files

### To create:
- `crates/elicit_wkt/Cargo.toml`
- `crates/elicit_wkt/README.md`
- `crates/elicit_wkt/src/lib.rs`
- `crates/elicit_wkt/src/geometry.rs`
- `crates/elicit_wkt/src/crs.rs`
- `crates/elicit_wkt/src/parser.rs`
- `crates/elicit_wkt/src/workflow/mod.rs`
- `crates/elicit_wkt/src/workflow/parse_plugin.rs`
- `crates/elicit_wkt/src/workflow/serialize_plugin.rs`
- `crates/elicit_wkt/src/workflow/convert_plugin.rs`
- `crates/elicit_wkt/src/workflow/crs_plugin.rs`
- `crates/elicit_wkt/src/workflow/validate_plugin.rs`
- `crates/elicit_wkt/src/workflow/workflow_plugin.rs`
- `crates/elicit_wkt/tests/wkt_test.rs`
- `crates/elicitation/src/wkt_types.rs`

### To modify:
- `Cargo.toml` — Add workspace members and dependencies
- `crates/elicitation/Cargo.toml` — Add wkt feature
- `crates/elicitation/src/lib.rs` — Export wkt types

## Implementation Order

1. **Phase 1**: Workspace configuration (30 min)
2. **Phase 2**: Core type integration in elicitation (1.5 hours)
3. **Phase 3**: Create elicit_wkt structure (1 hour)
4. **Phase 4**: Implement type wrappers (3 hours)
5. **Phase 5**: Implement MCP tools (~35 tools) (6-8 hours)
6. **Phase 6**: Testing (1-2 hours)
7. **Phase 7**: Documentation (1 hour)

**Total estimated time**: 14-17 hours

## Notes

### WKT in the GIS Ecosystem

**Standard Format**: OGC Simple Features specification
**Database Storage**: PostGIS, Oracle Spatial, SQL Server all use WKT
**Interchange**: Human-readable, unlike WKB (binary)
**Debugging**: Easy to inspect and verify geometries
**Configuration**: Define fixed geometries in config files

### Integration with GeoRust Stack

**Complete Pipeline**:
1. **Parse**: WKT → geo_types (elicit_wkt)
2. **Transform**: WGS84 → UTM (elicit_proj)
3. **Analyze**: Contains, Area, Distance (elicit_geo)
4. **Query**: Spatial index (elicit_rstar)
5. **Export**: geo_types → GeoJSON or WKT (elicit_geojson, elicit_wkt)

### Technical Challenges

1. **Case Sensitivity**: WKT is case-insensitive ("POINT" = "point")
2. **Whitespace**: Flexible whitespace handling
3. **3D/4D Coordinates**: Optional Z and M dimensions
4. **Precision**: Floating-point representation in text
5. **CRS WKT**: Much more complex than geometry WKT (long definitions)

### Performance Considerations

- **Parsing overhead**: Text parsing slower than binary (WKB)
- **Use case**: Prefer WKB for large datasets, WKT for human interaction
- **Caching**: Parse once, use geo_types for operations
- **Batch operations**: Parse multiple geometries efficiently

### PostGIS Integration

Common PostGIS functions that use WKT:
- `ST_GeomFromText(wkt, srid)` — Create geometry from WKT
- `ST_AsText(geom)` — Convert geometry to WKT
- `ST_GeomFromEWKT(ewkt)` — Extended WKT with SRID
- `ST_AsEWKT(geom)` — Extended WKT output
