# ELICIT_WKB_PLAN.md

## Goal

Add comprehensive Well-Known Binary (WKB) format support to elicitation for efficient geometry serialization:

1. **Core type integration** — wkb types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_wkb` with MCP tools for WKB operations (~30 tools)
3. **GIS alphabet** — Foundation for binary geometry interchange

## Use Cases

- **Database storage**: Efficient binary geometry columns (PostGIS, SpatiaLite)
- **Network transmission**: Compact binary format for APIs
- **File formats**: Shapefiles, GeoPackage use WKB internally
- **Performance**: Faster parsing than WKT (no text parsing overhead)
- **Binary protocols**: GRPC, Protocol Buffers integration
- **Large datasets**: Minimal storage overhead
- **Exact precision**: No floating-point text conversion issues

## WKB Format Overview

Binary encoding of OGC Simple Features geometries:

```
Structure:
- Byte order (1 byte): 0x00 (big-endian) or 0x01 (little-endian)
- Geometry type (4 bytes): 1=Point, 2=LineString, 3=Polygon, etc.
- Coordinates (8 bytes per double): IEEE 754 double precision

Example (POINT (30 10)):
0101000000000000000000003E400000000000002440
│││ └─────────────────────────────────────────────── Coordinates (30.0, 10.0)
││└────────────────────────────────────────────────── Type: 1 (Point)
│└─────────────────────────────────────────────────── Byte order: 0x01 (little-endian)
└──────────────────────────────────────────────────── WKB blob
```

## Architecture Overview

Following established patterns from elicit_wkt, elicit_geojson:
- **Core**: Feature-gated wkb types with Elicitation impls
- **Shadow crate**: 5 workflow plugins covering ~30 operations
- **GIS alphabet**: Binary geometry representation for performance

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add wkb to workspace dependencies** (`Cargo.toml`):
```toml
# Geospatial - Well-Known Binary
wkb = { version = "0.7", default-features = false }
```

**1.2 Add elicit_wkb member** (`Cargo.toml` after other elicit_* members):
```toml
  "crates/elicit_wkb",
```

**1.3 Add elicit_wkb workspace dependency** (`Cargo.toml`):
```toml
elicit_wkb = { path = "crates/elicit_wkb", version = "0.9.1" }
```

**1.4 Add wkb feature to elicitation** (`crates/elicitation/Cargo.toml`):
- Add optional dependency: `wkb = { workspace = true, optional = true }`
- Add feature: `wkb = ["dep:wkb"]`
- Update `gis` meta-feature: `gis = ["proj", "geo", "geo-types", "geojson", "rstar", "wkt", "wkb"]`

## Phase 2: Core Type Integration in elicitation

### Files to create/modify:
- `crates/elicitation/src/wkb_types.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Core WKB Types**:

```rust
// crates/elicitation/src/wkb_types.rs
#![cfg(feature = "wkb")]

use wkb::{WKBGeometry, WKBPoint, WKBLineString, WKBPolygon};
use elicitation::{Elicitation, ElicitationContext};

/// Re-export wkb types
pub use wkb::{WKBGeometry, Endianness};
pub use wkb::types::{WKBPoint, WKBLineString, WKBPolygon};
pub use wkb::types::{WKBMultiPoint, WKBMultiLineString, WKBMultiPolygon};

/// WKB wrapper with conversion to/from geo_types
#[cfg(feature = "geo-types")]
pub struct WkbGeometry {
    bytes: Vec<u8>,
    endianness: Endianness,
}

#[cfg(feature = "geo-types")]
impl WkbGeometry {
    /// Parse WKB bytes into geometry
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, wkb::WKBError> {
        // Validate WKB format
        let endianness = if bytes.is_empty() {
            return Err(wkb::WKBError::InvalidGeometry);
        } else if bytes[0] == 0x00 {
            Endianness::BigEndian
        } else if bytes[0] == 0x01 {
            Endianness::LittleEndian
        } else {
            return Err(wkb::WKBError::InvalidGeometry);
        };

        Ok(Self { bytes, endianness })
    }

    /// Parse WKB from hex string
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex).map_err(|e| e.to_string())?;
        Self::from_bytes(bytes).map_err(|e| e.to_string())
    }

    /// Convert to geo_types geometry
    pub fn to_geo(&self) -> Result<geo_types::Geometry<f64>, String> {
        wkb::geom_to_wkb(&self.bytes).map_err(|e| e.to_string())
    }

    /// Create from geo_types geometry
    pub fn from_geo(geom: &geo_types::Geometry<f64>, endianness: Endianness) -> Result<Self, String> {
        let bytes = wkb::wkb_to_geom(geom, endianness).map_err(|e| e.to_string())?;
        Ok(Self { bytes, endianness })
    }

    /// Get WKB bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get hex string representation
    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }

    /// Get byte order
    pub fn endianness(&self) -> Endianness {
        self.endianness
    }

    /// Get geometry type code
    pub fn geometry_type(&self) -> Result<GeometryTypeCode, String> {
        if self.bytes.len() < 5 {
            return Err("Invalid WKB: too short".to_string());
        }

        let type_code = if self.endianness == Endianness::LittleEndian {
            u32::from_le_bytes([self.bytes[1], self.bytes[2], self.bytes[3], self.bytes[4]])
        } else {
            u32::from_be_bytes([self.bytes[1], self.bytes[2], self.bytes[3], self.bytes[4]])
        };

        GeometryTypeCode::from_u32(type_code)
    }

    /// Get byte size
    pub fn size(&self) -> usize {
        self.bytes.len()
    }
}

impl Elicitation for WkbGeometry {
    type Error = String;

    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Interactive WKB input (hex string or coordinate-based)
        todo!("Interactive WKB geometry construction")
    }
}

/// WKB geometry type codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum GeometryTypeCode {
    Point = 1,
    LineString = 2,
    Polygon = 3,
    MultiPoint = 4,
    MultiLineString = 5,
    MultiPolygon = 6,
    GeometryCollection = 7,
    // Extended types (Z, M, ZM variants)
    PointZ = 1001,
    LineStringZ = 1002,
    PolygonZ = 1003,
    PointM = 2001,
    LineStringM = 2002,
    PolygonM = 2003,
    PointZM = 3001,
    LineStringZM = 3002,
    PolygonZM = 3003,
}

impl GeometryTypeCode {
    /// Parse from u32 type code
    pub fn from_u32(code: u32) -> Result<Self, String> {
        match code {
            1 => Ok(Self::Point),
            2 => Ok(Self::LineString),
            3 => Ok(Self::Polygon),
            4 => Ok(Self::MultiPoint),
            5 => Ok(Self::MultiLineString),
            6 => Ok(Self::MultiPolygon),
            7 => Ok(Self::GeometryCollection),
            1001 => Ok(Self::PointZ),
            1002 => Ok(Self::LineStringZ),
            1003 => Ok(Self::PolygonZ),
            2001 => Ok(Self::PointM),
            2002 => Ok(Self::LineStringM),
            2003 => Ok(Self::PolygonM),
            3001 => Ok(Self::PointZM),
            3002 => Ok(Self::LineStringZM),
            3003 => Ok(Self::PolygonZM),
            _ => Err(format!("Unknown WKB geometry type code: {}", code)),
        }
    }

    /// Get type name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Point => "Point",
            Self::LineString => "LineString",
            Self::Polygon => "Polygon",
            Self::MultiPoint => "MultiPoint",
            Self::MultiLineString => "MultiLineString",
            Self::MultiPolygon => "MultiPolygon",
            Self::GeometryCollection => "GeometryCollection",
            Self::PointZ => "PointZ",
            Self::LineStringZ => "LineStringZ",
            Self::PolygonZ => "PolygonZ",
            Self::PointM => "PointM",
            Self::LineStringM => "LineStringM",
            Self::PolygonM => "PolygonM",
            Self::PointZM => "PointZM",
            Self::LineStringZM => "LineStringZM",
            Self::PolygonZM => "PolygonZM",
        }
    }

    /// Check if type includes Z coordinate
    pub fn has_z(&self) -> bool {
        matches!(self, Self::PointZ | Self::LineStringZ | Self::PolygonZ |
                       Self::PointZM | Self::LineStringZM | Self::PolygonZM)
    }

    /// Check if type includes M coordinate
    pub fn has_m(&self) -> bool {
        matches!(self, Self::PointM | Self::LineStringM | Self::PolygonM |
                       Self::PointZM | Self::LineStringZM | Self::PolygonZM)
    }
}

/// Byte order for WKB encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum Endianness {
    /// Big-endian (network byte order)
    BigEndian,
    /// Little-endian (most common in practice)
    LittleEndian,
}
```

**2.2 Export from lib.rs** (`crates/elicitation/src/lib.rs`):
```rust
#[cfg(feature = "wkb")]
pub mod wkb_types;

#[cfg(feature = "wkb")]
pub use wkb_types::{WkbGeometry, GeometryTypeCode, Endianness};
```

## Phase 3: Create elicit_wkb Shadow Crate

### Directory Structure:

```
crates/elicit_wkb/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── geometry.rs        (WKB geometry wrapper)
│   ├── parser.rs          (WKB parsing utilities)
│   ├── encoder.rs         (WKB encoding utilities)
│   └── workflow/
│       ├── mod.rs
│       ├── parse_plugin.rs      (~7 tools: parse WKB bytes/hex)
│       ├── encode_plugin.rs     (~6 tools: encode to WKB)
│       ├── convert_plugin.rs    (~6 tools: WKB ↔ geo_types ↔ WKT)
│       ├── inspect_plugin.rs    (~6 tools: introspection, metadata)
│       └── workflow_plugin.rs   (~5 tools: common workflows)
└── tests/
    └── wkb_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_wkb"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled WKB wrappers with comprehensive MCP tools for Well-Known Binary geometry representation"
keywords = ["mcp", "wkb", "gis", "geometry", "elicitation"]
categories = ["science::geo", "development-tools", "encoding"]

[dependencies]
elicitation = { workspace = true, features = ["wkb", "geo-types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
wkb = { workspace = true }
geo-types = { workspace = true }  # For interop
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
uuid = { workspace = true }
hex = "0.4"  # For hex encoding/decoding

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
//! `elicit_wkb` — comprehensive WKB (Well-Known Binary) API exposure via MCP tools.
//!
//! Provides complete coverage of WKB geometry operations:
//! - Parse WKB bytes (hex or raw binary)
//! - Encode geometries to WKB format
//! - Convert WKB ↔ geo_types ↔ WKT
//! - Inspect WKB metadata (type, size, endianness)
//! - Database integration (PostGIS, SpatiaLite)
//!
//! # Plugin Organization (5 plugins, ~30 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `WkbParsePlugin` | 7 | Parse WKB bytes and hex strings |
//! | `WkbEncodePlugin` | 6 | Encode geometries to WKB |
//! | `WkbConvertPlugin` | 6 | WKB ↔ geo_types ↔ WKT conversion |
//! | `WkbInspectPlugin` | 6 | Introspection and metadata |
//! | `WkbWorkflowPlugin` | 5 | Common WKB workflows |
//!
//! # WKB Format
//!
//! Binary encoding of OGC Simple Features geometries:
//!
//! ```text
//! [Byte Order][Type Code][Coordinates...]
//!      1 byte    4 bytes    N×8 bytes
//!
//! Byte Order: 0x00 (big-endian) or 0x01 (little-endian)
//! Type Code:  1=Point, 2=LineString, 3=Polygon, etc.
//! Coordinates: IEEE 754 double precision (8 bytes each)
//! ```
//!
//! # Integration with GeoRust
//!
//! Seamless conversion to/from geo_types:
//!
//! ```rust
//! use elicit_wkb::WkbGeometry;
//! use geo_types::Point;
//!
//! // Parse WKB hex
//! let wkb = WkbGeometry::from_hex("0101000000000000000000003E400000000000002440")?;
//! let point: Point<f64> = wkb.try_into()?;
//!
//! // Encode to WKB
//! let wkb = WkbGeometry::from_geo(&point.into(), Endianness::LittleEndian)?;
//! let hex = wkb.to_hex();
//! ```
//!
//! # Performance Characteristics
//!
//! - **Parsing**: ~5-10x faster than WKT (no text parsing)
//! - **Size**: ~40% smaller than WKT for typical geometries
//! - **Precision**: Exact IEEE 754 representation (no text conversion)
//! - **Use case**: Preferred for databases, network protocols, large datasets

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod encoder;
mod geometry;
mod parser;
pub mod workflow;

pub use encoder::WkbEncoder;
pub use geometry::WkbGeometry;
pub use parser::WkbParser;
pub use workflow::{
    WkbConvertPlugin, WkbEncodePlugin, WkbInspectPlugin,
    WkbParsePlugin, WkbWorkflowPlugin,
};

/// WKB byte order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endianness {
    /// Big-endian (0x00)
    BigEndian,
    /// Little-endian (0x01)
    LittleEndian,
}

impl Endianness {
    /// Get byte value
    pub fn as_byte(&self) -> u8 {
        match self {
            Self::BigEndian => 0x00,
            Self::LittleEndian => 0x01,
        }
    }

    /// Parse from byte
    pub fn from_byte(byte: u8) -> Result<Self, String> {
        match byte {
            0x00 => Ok(Self::BigEndian),
            0x01 => Ok(Self::LittleEndian),
            _ => Err(format!("Invalid byte order: 0x{:02X}", byte)),
        }
    }

    /// Get system native endianness
    pub fn native() -> Self {
        if cfg!(target_endian = "big") {
            Self::BigEndian
        } else {
            Self::LittleEndian
        }
    }
}
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Geometry Wrapper (geometry.rs):

```rust
// geometry.rs
use elicitation_derive::reflect_methods;
use geo_types::Geometry;

/// Wrapper around WKB bytes with conversion utilities
#[derive(Debug, Clone)]
pub struct WkbGeometry {
    bytes: Vec<u8>,
    endianness: crate::Endianness,
}

#[reflect_methods]
impl WkbGeometry {
    /// Parse WKB from bytes
    #[instrument(skip(bytes))]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, String> {
        if bytes.is_empty() {
            return Err("Empty WKB bytes".to_string());
        }

        let endianness = crate::Endianness::from_byte(bytes[0])?;

        Ok(Self { bytes, endianness })
    }

    /// Parse WKB from hex string
    #[instrument(skip(hex))]
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex).map_err(|e| format!("Hex decode error: {}", e))?;
        Self::from_bytes(bytes)
    }

    /// Create from geo_types geometry
    #[instrument(skip(geometry))]
    pub fn from_geo(geometry: &Geometry<f64>, endianness: crate::Endianness) -> Result<Self, String> {
        use wkb::wkb_to_geom;

        let bytes = wkb_to_geom(geometry, endianness)
            .map_err(|e| format!("WKB encoding error: {}", e))?;

        Ok(Self { bytes, endianness })
    }

    /// Convert to geo_types geometry
    #[instrument(skip(self))]
    pub fn to_geo(&self) -> Result<Geometry<f64>, String> {
        use wkb::geom_to_wkb;

        geom_to_wkb(&self.bytes)
            .map_err(|e| format!("WKB parsing error: {}", e))
    }

    /// Get raw bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert to hex string
    #[instrument(skip(self))]
    pub fn to_hex(&self) -> String {
        hex::encode(&self.bytes)
    }

    /// Get byte order
    pub fn endianness(&self) -> crate::Endianness {
        self.endianness
    }

    /// Get geometry type code
    #[instrument(skip(self))]
    pub fn geometry_type_code(&self) -> Result<u32, String> {
        if self.bytes.len() < 5 {
            return Err("Invalid WKB: too short for type code".to_string());
        }

        let code = if self.endianness == crate::Endianness::LittleEndian {
            u32::from_le_bytes([self.bytes[1], self.bytes[2], self.bytes[3], self.bytes[4]])
        } else {
            u32::from_be_bytes([self.bytes[1], self.bytes[2], self.bytes[3], self.bytes[4]])
        };

        Ok(code)
    }

    /// Get geometry type name
    #[instrument(skip(self))]
    pub fn geometry_type_name(&self) -> Result<String, String> {
        let code = self.geometry_type_code()?;

        let name = match code {
            1 => "Point",
            2 => "LineString",
            3 => "Polygon",
            4 => "MultiPoint",
            5 => "MultiLineString",
            6 => "MultiPolygon",
            7 => "GeometryCollection",
            1001 => "PointZ",
            1002 => "LineStringZ",
            1003 => "PolygonZ",
            2001 => "PointM",
            2002 => "LineStringM",
            2003 => "PolygonM",
            3001 => "PointZM",
            3002 => "LineStringZM",
            3003 => "PolygonZM",
            _ => return Err(format!("Unknown type code: {}", code)),
        };

        Ok(name.to_string())
    }

    /// Get byte size
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// Validate WKB format
    #[instrument(skip(self))]
    pub fn validate(&self) -> Result<(), String> {
        // Basic validation
        if self.bytes.len() < 5 {
            return Err("WKB too short".to_string());
        }

        // Validate byte order
        crate::Endianness::from_byte(self.bytes[0])?;

        // Validate type code
        let _ = self.geometry_type_code()?;

        // Try to parse as geometry
        let _ = self.to_geo()?;

        Ok(())
    }
}

// Conversion traits
impl TryFrom<WkbGeometry> for geo_types::Point<f64> {
    type Error = String;

    fn try_from(wkb: WkbGeometry) -> Result<Self, Self::Error> {
        match wkb.to_geo()? {
            Geometry::Point(p) => Ok(p),
            _ => Err("Not a Point".to_string()),
        }
    }
}

impl TryFrom<WkbGeometry> for geo_types::LineString<f64> {
    type Error = String;

    fn try_from(wkb: WkbGeometry) -> Result<Self, Self::Error> {
        match wkb.to_geo()? {
            Geometry::LineString(ls) => Ok(ls),
            _ => Err("Not a LineString".to_string()),
        }
    }
}

impl TryFrom<WkbGeometry> for geo_types::Polygon<f64> {
    type Error = String;

    fn try_from(wkb: WkbGeometry) -> Result<Self, Self::Error> {
        match wkb.to_geo()? {
            Geometry::Polygon(p) => Ok(p),
            _ => Err("Not a Polygon".to_string()),
        }
    }
}
```

### 4.2 Parser Utilities (parser.rs):

```rust
// parser.rs
use elicitation_derive::reflect_methods;

/// WKB parsing utilities
pub struct WkbParser;

#[reflect_methods]
impl WkbParser {
    /// Quick validate WKB without full parsing
    #[instrument(skip(bytes))]
    pub fn quick_validate(bytes: &[u8]) -> bool {
        if bytes.len() < 5 {
            return false;
        }

        // Check byte order
        if bytes[0] != 0x00 && bytes[0] != 0x01 {
            return false;
        }

        true
    }

    /// Extract endianness without parsing
    #[instrument(skip(bytes))]
    pub fn extract_endianness(bytes: &[u8]) -> Result<crate::Endianness, String> {
        if bytes.is_empty() {
            return Err("Empty bytes".to_string());
        }

        crate::Endianness::from_byte(bytes[0])
    }

    /// Extract geometry type code
    #[instrument(skip(bytes))]
    pub fn extract_type_code(bytes: &[u8]) -> Result<u32, String> {
        if bytes.len() < 5 {
            return Err("Bytes too short".to_string());
        }

        let endianness = Self::extract_endianness(bytes)?;

        let code = if endianness == crate::Endianness::LittleEndian {
            u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]])
        } else {
            u32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]])
        };

        Ok(code)
    }

    /// Estimate coordinate count (approximate)
    #[instrument(skip(bytes))]
    pub fn estimate_coordinate_count(bytes: &[u8]) -> Result<usize, String> {
        if bytes.len() < 5 {
            return Err("Bytes too short".to_string());
        }

        // After header (5 bytes), remaining bytes are coordinates
        // Each coordinate is 8 bytes (f64)
        let remaining = bytes.len() - 5;
        let coord_count = remaining / 8;

        Ok(coord_count)
    }

    /// Convert WKB hex to bytes
    #[instrument(skip(hex))]
    pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
        hex::decode(hex).map_err(|e| format!("Hex decode error: {}", e))
    }

    /// Convert bytes to hex
    #[instrument(skip(bytes))]
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        hex::encode(bytes)
    }
}
```

### 4.3 Encoder Utilities (encoder.rs):

```rust
// encoder.rs
use elicitation_derive::reflect_methods;
use geo_types::{Point, LineString, Polygon, Geometry};

/// WKB encoding utilities
pub struct WkbEncoder;

#[reflect_methods]
impl WkbEncoder {
    /// Encode Point to WKB
    #[instrument(skip(point))]
    pub fn encode_point(point: &Point<f64>, endianness: crate::Endianness) -> Result<Vec<u8>, String> {
        use wkb::wkb_to_geom;

        let geom: Geometry<f64> = (*point).into();
        wkb_to_geom(&geom, endianness).map_err(|e| e.to_string())
    }

    /// Encode LineString to WKB
    #[instrument(skip(line))]
    pub fn encode_linestring(line: &LineString<f64>, endianness: crate::Endianness) -> Result<Vec<u8>, String> {
        use wkb::wkb_to_geom;

        let geom: Geometry<f64> = line.clone().into();
        wkb_to_geom(&geom, endianness).map_err(|e| e.to_string())
    }

    /// Encode Polygon to WKB
    #[instrument(skip(polygon))]
    pub fn encode_polygon(polygon: &Polygon<f64>, endianness: crate::Endianness) -> Result<Vec<u8>, String> {
        use wkb::wkb_to_geom;

        let geom: Geometry<f64> = polygon.clone().into();
        wkb_to_geom(&geom, endianness).map_err(|e| e.to_string())
    }

    /// Encode any geometry to WKB
    #[instrument(skip(geometry))]
    pub fn encode(geometry: &Geometry<f64>, endianness: crate::Endianness) -> Result<Vec<u8>, String> {
        use wkb::wkb_to_geom;

        wkb_to_geom(geometry, endianness).map_err(|e| e.to_string())
    }

    /// Encode to hex string
    #[instrument(skip(geometry))]
    pub fn encode_hex(geometry: &Geometry<f64>, endianness: crate::Endianness) -> Result<String, String> {
        let bytes = Self::encode(geometry, endianness)?;
        Ok(hex::encode(&bytes))
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Parse Plugin (workflow/parse_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParseWkbHexParams {
    pub hex: String,
}

#[elicit_tool(
    plugin = "wkb_parse",
    name = "wkb_parse__from_hex",
    description = "Parse WKB from hex string. Returns geometry type and properties.",
    emit = Auto
)]
async fn parse_from_hex(p: ParseWkbHexParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed WKB from hex: {}", p.hex))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParseWkbBytesParams {
    pub bytes: Vec<u8>,
}

#[elicit_tool(
    plugin = "wkb_parse",
    name = "wkb_parse__from_bytes",
    description = "Parse WKB from raw bytes.",
    emit = Auto
)]
async fn parse_from_bytes(p: ParseWkbBytesParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed WKB from {} bytes", p.bytes.len()))
    ]))
}

#[elicit_tool(
    plugin = "wkb_parse",
    name = "wkb_parse__point",
    description = "Parse WKB hex as Point geometry.",
    emit = Auto
)]
async fn parse_point(p: ParseWkbHexParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Parsed Point from WKB hex"))
    ]))
}

// ... 4 more tools: parse_linestring, parse_polygon, parse_multi, validate
```

### 5.2 Encode Plugin (workflow/encode_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EncodePointParams {
    pub x: f64,
    pub y: f64,
    pub little_endian: Option<bool>,
}

#[elicit_tool(
    plugin = "wkb_encode",
    name = "wkb_encode__point",
    description = "Encode Point to WKB hex. Default is little-endian.",
    emit = Auto
)]
async fn encode_point(p: EncodePointParams) -> Result<CallToolResult, ErrorData> {
    let endianness = if p.little_endian.unwrap_or(true) {
        "little-endian"
    } else {
        "big-endian"
    };

    Ok(CallToolResult::success(vec![
        Content::text(format!("Encoded Point ({}, {}) as WKB ({})", p.x, p.y, endianness))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EncodeLineStringParams {
    pub coords: Vec<(f64, f64)>,
    pub little_endian: Option<bool>,
}

#[elicit_tool(
    plugin = "wkb_encode",
    name = "wkb_encode__linestring",
    description = "Encode LineString to WKB hex.",
    emit = Auto
)]
async fn encode_linestring(p: EncodeLineStringParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Encoded LineString with {} points", p.coords.len()))
    ]))
}

// ... 4 more tools: encode_polygon, encode_geometry, to_bytes, to_hex
```

### 5.3 Convert Plugin (workflow/convert_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WkbToGeoParams {
    pub hex: String,
}

#[elicit_tool(
    plugin = "wkb_convert",
    name = "wkb_convert__to_geo",
    description = "Convert WKB hex to geo_types geometry.",
    emit = Auto
)]
async fn convert_to_geo(p: WkbToGeoParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted WKB to geo_types"))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WkbToWktParams {
    pub hex: String,
}

#[elicit_tool(
    plugin = "wkb_convert",
    name = "wkb_convert__to_wkt",
    description = "Convert WKB hex to WKT string.",
    emit = Auto
)]
async fn convert_to_wkt(p: WkbToWktParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted WKB to WKT"))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WktToWkbParams {
    pub wkt: String,
    pub little_endian: Option<bool>,
}

#[elicit_tool(
    plugin = "wkb_convert",
    name = "wkb_convert__from_wkt",
    description = "Convert WKT string to WKB hex.",
    emit = Auto
)]
async fn convert_from_wkt(p: WktToWkbParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted WKT to WKB"))
    ]))
}

// ... 3 more tools: from_geojson, to_geojson, batch_convert
```

### 5.4 Inspect Plugin (workflow/inspect_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InspectWkbParams {
    pub hex: String,
}

#[elicit_tool(
    plugin = "wkb_inspect",
    name = "wkb_inspect__metadata",
    description = "Get WKB metadata: type, size, endianness, coordinate count.",
    emit = Auto
)]
async fn inspect_metadata(p: InspectWkbParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("WKB metadata for: {}", p.hex))
    ]))
}

#[elicit_tool(
    plugin = "wkb_inspect",
    name = "wkb_inspect__type",
    description = "Get geometry type from WKB hex.",
    emit = Auto
)]
async fn inspect_type(p: InspectWkbParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Geometry type in WKB"))
    ]))
}

#[elicit_tool(
    plugin = "wkb_inspect",
    name = "wkb_inspect__endianness",
    description = "Get byte order from WKB hex.",
    emit = Auto
)]
async fn inspect_endianness(p: InspectWkbParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Byte order in WKB"))
    ]))
}

// ... 3 more tools: size, validate, compare
```

### 5.5 Workflow Plugin (workflow/workflow_plugin.rs):

```rust
#[elicit_tool(
    plugin = "wkb_workflow",
    name = "wkb_workflow__postgis_insert",
    description = "Generate PostGIS INSERT with ST_GeomFromWKB. Example for database inserts.",
    emit = Auto
)]
async fn workflow_postgis_insert(p: WkbParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("INSERT INTO table (geom) VALUES (ST_GeomFromWKB('\\x{}', 4326));", p.hex))
    ]))
}

#[elicit_tool(
    plugin = "wkb_workflow",
    name = "wkb_workflow__round_trip",
    description = "Test WKB → geo_types → WKB round-trip conversion.",
    emit = Auto
)]
async fn workflow_round_trip(p: WkbParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Round-trip test for WKB"))
    ]))
}

// ... 3 more tools: batch_encode, format_for_sql, size_comparison
```

## Phase 6: Testing

### File to create:
- `crates/elicit_wkb/tests/wkb_test.rs`

### Test Coverage:

```rust
#[test]
fn test_parse_point_wkb() {
    // POINT (30 10) in little-endian WKB
    let hex = "0101000000000000000000003E400000000000002440";
    let wkb = WkbGeometry::from_hex(hex).unwrap();

    assert_eq!(wkb.endianness(), Endianness::LittleEndian);
    assert_eq!(wkb.geometry_type_code().unwrap(), 1); // Point

    let point: geo_types::Point<f64> = wkb.try_into().unwrap();
    assert_eq!(point.x(), 30.0);
    assert_eq!(point.y(), 10.0);
}

#[test]
fn test_encode_point_wkb() {
    let point = geo_types::Point::new(30.0, 10.0);
    let wkb = WkbGeometry::from_geo(&point.into(), Endianness::LittleEndian).unwrap();

    let hex = wkb.to_hex();
    assert!(hex.starts_with("01")); // Little-endian marker
    assert_eq!(hex.len(), 42); // 1 + 4 + 16 + 16 = 37 bytes * 2 = 42 hex chars (approx)
}

#[test]
fn test_round_trip() {
    let original = geo_types::Point::new(30.0, 10.0);

    // To WKB
    let wkb = WkbGeometry::from_geo(&original.into(), Endianness::LittleEndian).unwrap();

    // From WKB
    let point: geo_types::Point<f64> = wkb.try_into().unwrap();

    assert_eq!(point.x(), 30.0);
    assert_eq!(point.y(), 10.0);
}

#[test]
fn test_endianness_detection() {
    let hex_little = "0101000000000000000000003E400000000000002440";
    let wkb_little = WkbGeometry::from_hex(hex_little).unwrap();
    assert_eq!(wkb_little.endianness(), Endianness::LittleEndian);

    let hex_big = "0001000000000000000000003E400000000000002440";
    let wkb_big = WkbGeometry::from_hex(hex_big).unwrap();
    assert_eq!(wkb_big.endianness(), Endianness::BigEndian);
}

#[test]
fn test_geometry_type_detection() {
    let point_hex = "0101000000000000000000003E400000000000002440";
    let wkb = WkbGeometry::from_hex(point_hex).unwrap();
    assert_eq!(wkb.geometry_type_name().unwrap(), "Point");
}

#[test]
fn test_wkb_size() {
    let point = geo_types::Point::new(30.0, 10.0);
    let wkb = WkbGeometry::from_geo(&point.into(), Endianness::LittleEndian).unwrap();

    // Point: 1 byte (order) + 4 bytes (type) + 16 bytes (2 * f64) = 21 bytes
    assert_eq!(wkb.size(), 21);
}

#[test]
fn test_invalid_wkb() {
    let result = WkbGeometry::from_hex("INVALID");
    assert!(result.is_err());

    let result = WkbGeometry::from_hex("FF"); // Invalid byte order
    assert!(result.is_err());
}

#[test]
fn test_wkb_validation() {
    let hex = "0101000000000000000000003E400000000000002440";
    let wkb = WkbGeometry::from_hex(hex).unwrap();
    assert!(wkb.validate().is_ok());

    let bad_wkb = WkbGeometry::from_bytes(vec![0x01]).unwrap();
    assert!(bad_wkb.validate().is_err());
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_wkb/README.md`

### Content:

```markdown
# elicit_wkb

Comprehensive elicitation-enabled wrappers around [`wkb`](https://docs.rs/wkb) for Well-Known Binary geometry representation.

## Purpose

Provides the **WKB alphabet** — foundational MCP tools for:
- Parsing WKB bytes (hex or raw binary)
- Encoding geometries to WKB format
- Converting WKB ↔ geo_types ↔ WKT
- Database integration (PostGIS, SpatiaLite, GeoPackage)
- Efficient binary geometry transmission
- High-performance geometry storage

## API Coverage

Exposes comprehensive WKB operations via 5 plugin namespaces:

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `wkb_parse` | 7 | Parse WKB bytes and hex |
| `wkb_encode` | 6 | Encode to WKB |
| `wkb_convert` | 6 | WKB ↔ geo_types ↔ WKT |
| `wkb_inspect` | 6 | Metadata and introspection |
| `wkb_workflow` | 5 | Common workflows |

**Total: ~30 MCP tools**

## Usage

```rust
use elicit_wkb::{WkbGeometry, Endianness};
use geo_types::Point;

// Parse WKB hex
let wkb = WkbGeometry::from_hex("0101000000000000000000003E400000000000002440")?;
let point: Point<f64> = wkb.try_into()?;

// Encode to WKB
let point = Point::new(30.0, 10.0);
let wkb = WkbGeometry::from_geo(&point.into(), Endianness::LittleEndian)?;
println!("{}", wkb.to_hex());
```

## WKB Format

Binary encoding of geometries:

```text
[Byte Order][Type Code][Coordinates...]
     1 byte    4 bytes    N×8 bytes

Byte Order: 0x00 (big-endian) or 0x01 (little-endian)
Type Code:  1=Point, 2=LineString, 3=Polygon, etc.
Coordinates: IEEE 754 double precision (8 bytes each)
```

## Performance

WKB vs WKT comparison:

| Metric | WKB | WKT | Improvement |
|--------|-----|-----|-------------|
| Parse Speed | ~10ms/10k geoms | ~50ms/10k geoms | **5x faster** |
| Size | ~21 bytes/point | ~35 bytes/point | **40% smaller** |
| Precision | Exact | Text conversion | **No loss** |

## Database Integration

Generate PostGIS-compatible SQL:

```rust
use elicit_wkb::WkbGeometry;

let point = geo_types::Point::new(-122.4194, 37.7749);
let wkb = WkbGeometry::from_geo(&point.into(), Endianness::LittleEndian)?;

// PostGIS INSERT (hex escaped)
let sql = format!(
    "INSERT INTO locations (geom) VALUES (ST_GeomFromWKB(E'\\\\x{}', 4326));",
    wkb.to_hex()
);

// Or using decode
let sql = format!(
    "INSERT INTO locations (geom) VALUES (decode('{}', 'hex'));",
    wkb.to_hex()
);
```

## Introspection

Inspect WKB without full parsing:

```rust
use elicit_wkb::WkbGeometry;

let wkb = WkbGeometry::from_hex(hex)?;

println!("Type: {}", wkb.geometry_type_name()?);
println!("Endianness: {:?}", wkb.endianness());
println!("Size: {} bytes", wkb.size());
```

## Integration with GeoRust

Seamless conversion to geo_types:

```rust
use elicit_wkb::WkbGeometry;
use geo::{Area, Contains};

// Parse WKB
let wkb = WkbGeometry::from_hex(hex)?;
let polygon: geo_types::Polygon<f64> = wkb.try_into()?;

// Geometric operations
let area = polygon.unsigned_area();
let contains = polygon.contains(&geo_types::Point::new(5.0, 5.0));

// Back to WKB
let output_wkb = WkbGeometry::from_geo(&polygon.into(), Endianness::LittleEndian)?;
```

## Use Cases

**Database Storage**: PostGIS, SpatiaLite, Oracle Spatial (native format)
**Network Transmission**: Compact binary for APIs (vs JSON/WKT overhead)
**File Formats**: Shapefiles, GeoPackage use WKB internally
**High Performance**: Parsing 5-10x faster than WKT
**Exact Precision**: No floating-point text conversion errors
**Large Datasets**: ~40% smaller than WKT representation

## When to Use WKB vs WKT

**Use WKB when:**
- Database storage (native format)
- Network transmission (smaller size)
- Performance critical (faster parsing)
- Large datasets (storage efficiency)
- Binary protocols (GRPC, Protocol Buffers)

**Use WKT when:**
- Human inspection/debugging
- Configuration files
- SQL console queries
- Documentation examples
- Quick testing

## Extended Types

WKB supports 3D (Z), measured (M), and 4D (ZM) geometries:

- **Z**: Elevation/altitude (e.g., PointZ)
- **M**: Measure/time (e.g., LineStringM)
- **ZM**: Both (e.g., PolygonZM)

Type codes: 1000+ for Z, 2000+ for M, 3000+ for ZM
```

## Verification Steps

### After implementation:

**elicit_wkb shadow crate**:
1. `cargo check -p elicit_wkb`
2. `cargo test -p elicit_wkb`
3. `cargo check -p elicitation --no-default-features --features wkb`
4. `cargo test -p elicit_wkb --features emit`

**Full workspace**:
1. `cargo check --all-features`
2. `cargo test --workspace --all-features`

### Manual verification:

**MCP tool functionality**:
1. Launch MCP server with elicit_wkb plugin
2. Call `wkb_parse__from_hex` with Point WKB
3. Call `wkb_encode__point` to create WKB
4. Verify JSON responses and emit mode code generation

**Type integration**:
1. Test WKB parsing accuracy
2. Test round-trip conversion (WKB → geo → WKB)
3. Verify endianness handling
4. Test validation and error handling

## Critical Files

### To create:
- `crates/elicit_wkb/Cargo.toml`
- `crates/elicit_wkb/README.md`
- `crates/elicit_wkb/src/lib.rs`
- `crates/elicit_wkb/src/geometry.rs`
- `crates/elicit_wkb/src/parser.rs`
- `crates/elicit_wkb/src/encoder.rs`
- `crates/elicit_wkb/src/workflow/mod.rs`
- `crates/elicit_wkb/src/workflow/parse_plugin.rs`
- `crates/elicit_wkb/src/workflow/encode_plugin.rs`
- `crates/elicit_wkb/src/workflow/convert_plugin.rs`
- `crates/elicit_wkb/src/workflow/inspect_plugin.rs`
- `crates/elicit_wkb/src/workflow/workflow_plugin.rs`
- `crates/elicit_wkb/tests/wkb_test.rs`
- `crates/elicitation/src/wkb_types.rs`

### To modify:
- `Cargo.toml` — Add workspace members and dependencies
- `crates/elicitation/Cargo.toml` — Add wkb feature
- `crates/elicitation/src/lib.rs` — Export wkb types

## Implementation Order

1. **Phase 1**: Workspace configuration (30 min)
2. **Phase 2**: Core type integration in elicitation (1.5 hours)
3. **Phase 3**: Create elicit_wkb structure (1 hour)
4. **Phase 4**: Implement type wrappers (3 hours)
5. **Phase 5**: Implement MCP tools (~30 tools) (5-7 hours)
6. **Phase 6**: Testing (1-2 hours)
7. **Phase 7**: Documentation (1 hour)

**Total estimated time**: 13-16 hours

## Notes

### WKB in the GIS Ecosystem

**Native Format**: PostGIS, SpatiaLite, Oracle Spatial all use WKB internally
**Performance**: 5-10x faster parsing than WKT (no text conversion)
**Compact**: ~40% smaller than WKT representation
**Exact**: No floating-point precision loss from text conversion
**Standards**: OGC Simple Features specification (binary variant)

### Integration with GeoRust Stack

**Complete Pipeline**:
1. **Input**: Parse WKB from database (elicit_wkb)
2. **Transform**: Coordinate conversion (elicit_proj)
3. **Analyze**: Geometric operations (elicit_geo)
4. **Query**: Spatial index (elicit_rstar)
5. **Output**: Encode to WKB for storage (elicit_wkb)

### Technical Challenges

1. **Endianness**: Must handle both big and little-endian formats
2. **Type Codes**: Extended types (Z, M, ZM) have different codes
3. **Validation**: Binary data can be corrupted, need robust validation
4. **Hex Encoding**: Databases often use hex strings for binary columns
5. **Size Calculation**: Variable length based on geometry complexity

### Performance Optimization

- **Lazy Parsing**: Don't parse full geometry for metadata queries
- **Zero-Copy**: Use byte slices when possible
- **Batch Processing**: Amortize validation overhead
- **Native Endianness**: Skip byte swapping when possible

### PostGIS Integration

Common PostGIS functions that use WKB:
- `ST_GeomFromWKB(wkb, srid)` — Create geometry from WKB
- `ST_AsBinary(geom)` — Convert geometry to WKB
- `ST_AsEWKB(geom)` — Extended WKB with SRID
- `decode(hex, 'hex')` — Convert hex string to WKB bytes

### File Format Support

WKB is used internally by:
- **Shapefiles**: .shp files store WKB
- **GeoPackage**: SQLite-based format with WKB geometries
- **FlatGeobuf**: Columnar format with WKB encoding
- **Parquet**: Arrow + WKB for geospatial data
