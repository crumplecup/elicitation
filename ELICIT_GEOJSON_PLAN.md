# ELICIT_GEOJSON_PLAN.md

## Goal
Add complete geojson support to elicitation as geometry serialization alphabet:
1. **Core type integration** — geojson in `elicitation` with feature gating
2. **Shadow crate** — `elicit_geojson` with MCP tools for GeoJSON serialization

## Architecture Overview

Following established patterns from elicit_serde_json, elicit_url:
- **Core**: Feature-gated geojson with Elicitation impls for GeoJSON types
- **Shadow crate**: ~4-6 workflow plugins covering GeoJSON format operations
- **Serialization alphabet**: Convert geo-types ↔ GeoJSON, export/import layouts

## Why GeoJSON for UI?

GeoJSON provides universal interchange format:
- **Export layouts**: elicit_ui layouts → GeoJSON → visualization tools
- **Import layouts**: Design tools → GeoJSON → elicit_ui validation
- **Debugging**: Human-readable geometry dumps
- **Interop**: Web mapping libraries, design tools, spatial databases

## API Coverage

geojson provides GeoJSON format support:
- **Core Types**: `Geometry`, `Feature`, `FeatureCollection`
- **Geometry Variants**: `Point`, `LineString`, `Polygon`, `MultiPoint`, etc.
- **Conversions**: geo-types ↔ GeoJSON
- **Serialization**: JSON ↔ GeoJSON structs (via serde)
- **Validation**: GeoJSON specification compliance

**Total API surface**: ~8 core types, bidirectional conversions → ~25-35 MCP tools across 4-6 plugins

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add geojson to workspace dependencies**:
```toml
# GeoJSON format support
geojson = "0.24"
```

**1.2 Add elicit_geojson member**:
```toml
  "crates/elicit_geojson",
```

**1.3 Add elicit_geojson workspace dependency**:
```toml
elicit_geojson = { path = "crates/elicit_geojson", version = "0.9.1" }
```

**1.4 Add geojson feature to elicitation**:
- Add optional dependency: `geojson = { workspace = true, optional = true }`
- Add feature: `geojson = ["dep:geojson", "geo_types", "serde_json"]`
- Update `full` feature to include `"geojson"`

## Phase 2: Core Type Integration

### Files to create/modify:
- `crates/elicitation/src/geojson_support.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 GeoJSON Types** (manual `Elicitation` impl):

Core GeoJSON structure:
```rust
// Top-level GeoJSON object
pub enum GeoJson {
    Geometry(Geometry),
    Feature(Feature),
    FeatureCollection(FeatureCollection),
}

// Geometry types
pub struct Geometry {
    pub value: Value,  // Point, LineString, Polygon, etc.
    pub bbox: Option<Bbox>,
    pub foreign_members: Option<JsonObject>,
}

// Feature with properties
pub struct Feature {
    pub geometry: Option<Geometry>,
    pub properties: Option<JsonObject>,
    pub id: Option<feature::Id>,
    pub bbox: Option<Bbox>,
    pub foreign_members: Option<JsonObject>,
}

// Collection of features
pub struct FeatureCollection {
    pub features: Vec<Feature>,
    pub bbox: Option<Bbox>,
    pub foreign_members: Option<JsonObject>,
}
```

**2.2 Conversion Strategy**:

geojson provides bidirectional conversions with geo-types:
```rust
use geo_types::{Point, Polygon};
use geojson::{Geometry, Value};

// geo-types → GeoJSON
let point = Point::new(10.0, 20.0);
let geojson: Geometry = point.into();

// GeoJSON → geo-types
let geojson_point = Geometry {
    value: Value::Point(vec![10.0, 20.0]),
    bbox: None,
    foreign_members: None,
};
let geo_point: Point<f64> = geojson_point.try_into().unwrap();
```

### Implementation Pattern:

```rust
// crates/elicitation/src/geojson_support.rs
#![cfg(feature = "geojson")]

use geojson::{GeoJson, Geometry, Feature, FeatureCollection};

/// Elicitation impl for GeoJSON Geometry
impl Elicitation for Geometry {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for geometry type, coordinates
        // Build Geometry from user input
    }
}

/// Elicitation impl for Feature
impl Elicitation for Feature {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for geometry
        // Optional: prompt for properties (key-value pairs)
        // Optional: prompt for id
    }
}

/// Elicitation impl for FeatureCollection
impl Elicitation for FeatureCollection {
    type Error = String;
    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Prompt for number of features
        // Elicit each feature
    }
}
```

**2.3 Export from lib.rs**:
```rust
#[cfg(feature = "geojson")]
pub mod geojson_support;

#[cfg(feature = "geojson")]
pub use geojson_support::*;
```

## Phase 3: Create elicit_geojson Shadow Crate

### Directory Structure:

```
crates/elicit_geojson/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── geometry.rs         (Geometry wrapper)
│   ├── feature.rs          (Feature wrapper)
│   ├── collection.rs       (FeatureCollection wrapper)
│   └── workflow/
│       ├── mod.rs
│       ├── geometry_plugin.rs      (~8 tools: Create/parse geometries)
│       ├── feature_plugin.rs       (~6 tools: Create/parse features)
│       ├── collection_plugin.rs    (~6 tools: Create/parse collections)
│       ├── conversion_plugin.rs    (~8 tools: geo-types ↔ GeoJSON)
│       ├── io_plugin.rs            (~4 tools: Read/write GeoJSON files)
│       └── workflow_plugin.rs      (~6 tools: Common patterns)
└── tests/
    └── geojson_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_geojson"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled geojson wrappers with MCP tools for GeoJSON serialization"
keywords = ["mcp", "geojson", "geo", "serialization", "elicitation"]
categories = ["science::geo", "encoding", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["geojson", "geo_types", "serde_json"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
geojson = { workspace = true }
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

### lib.rs structure:

```rust
//! `elicit_geojson` — comprehensive GeoJSON API exposure via MCP tools.
//!
//! Provides GeoJSON serialization for geometric data:
//! - Geometry objects (Point, LineString, Polygon, etc.)
//! - Features (geometry + properties)
//! - FeatureCollections (multiple features)
//! - Bidirectional conversion with geo-types
//! - JSON serialization/deserialization
//!
//! # Plugin Organization (6 plugins, ~38 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `GeoJsonGeometryPlugin` | 8 | Create/parse GeoJSON geometries |
//! | `GeoJsonFeaturePlugin` | 6 | Create/parse features with properties |
//! | `GeoJsonCollectionPlugin` | 6 | Create/parse feature collections |
//! | `GeoJsonConversionPlugin` | 8 | Convert geo-types ↔ GeoJSON |
//! | `GeoJsonIoPlugin` | 4 | Read/write GeoJSON files |
//! | `GeoJsonWorkflowPlugin` | 6 | Common patterns |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod collection;
mod feature;
mod geometry;
pub mod workflow;

pub use collection::FeatureCollection;
pub use feature::Feature;
pub use geometry::Geometry;
pub use workflow::{
    GeoJsonCollectionPlugin, GeoJsonConversionPlugin, GeoJsonFeaturePlugin,
    GeoJsonGeometryPlugin, GeoJsonIoPlugin, GeoJsonWorkflowPlugin,
};
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Geometry wrapper (geometry.rs):

```rust
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use geojson as gj;

elicit_newtype!(gj::Geometry, as Geometry, serde);

#[reflect_methods]
impl Geometry {
    #[instrument]
    pub fn from_geo_types_point(point: &geo_types::Point<f64>) -> Self {
        Self(gj::Geometry::from(point))
    }

    #[instrument]
    pub fn from_geo_types_polygon(polygon: &geo_types::Polygon<f64>) -> Self {
        Self(gj::Geometry::from(polygon))
    }

    #[instrument(skip(self))]
    pub fn to_json_string(&self) -> Result<String, String> {
        serde_json::to_string(&self.0)
            .map_err(|e| format!("Serialization failed: {}", e))
    }

    #[instrument]
    pub fn from_json_string(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map(Self)
            .map_err(|e| format!("Deserialization failed: {}", e))
    }
}
```

### 4.2 Feature wrapper (feature.rs):

```rust
elicit_newtype!(gj::Feature, as Feature, serde);

#[reflect_methods]
impl Feature {
    #[instrument]
    pub fn new(geometry: Geometry, properties: Option<serde_json::Map<String, serde_json::Value>>) -> Self {
        Self(gj::Feature {
            geometry: Some(geometry.0),
            properties,
            id: None,
            bbox: None,
            foreign_members: None,
        })
    }

    #[instrument(skip(self))]
    pub fn geometry(&self) -> Option<Geometry> {
        self.0.geometry.clone().map(Geometry)
    }

    #[instrument(skip(self))]
    pub fn properties(&self) -> Option<&serde_json::Map<String, serde_json::Value>> {
        self.0.properties.as_ref()
    }
}
```

### 4.3 FeatureCollection wrapper (collection.rs):

```rust
elicit_newtype!(gj::FeatureCollection, as FeatureCollection, serde);

#[reflect_methods]
impl FeatureCollection {
    #[instrument]
    pub fn new(features: Vec<Feature>) -> Self {
        Self(gj::FeatureCollection {
            features: features.into_iter().map(|f| f.0).collect(),
            bbox: None,
            foreign_members: None,
        })
    }

    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.0.features.len()
    }

    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.0.features.is_empty()
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Geometry Plugin (workflow/geometry_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateGeometryParams {
    pub geometry_type: String,  // "Point", "LineString", "Polygon", etc.
    pub coordinates: serde_json::Value,
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "geojson_geometry__create",
    description = "Create a GeoJSON Geometry from type and coordinates. \
                   Types: Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon.",
    emit = Auto
)]
async fn geometry_create(p: CreateGeometryParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created GeoJSON {}", p.geometry_type))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParseGeometryParams {
    pub geojson_string: String,
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "geojson_geometry__parse",
    description = "Parse a GeoJSON Geometry from JSON string.",
    emit = Auto
)]
async fn geometry_parse(p: ParseGeometryParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("Parsed GeoJSON Geometry")
    ]))
}

// ... 6 more tools: geometry_to_json, geometry_type, geometry_coords, etc.
```

### 5.2 Feature Plugin (workflow/feature_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateFeatureParams {
    pub geometry: serde_json::Value,
    pub properties: Option<serde_json::Map<String, serde_json::Value>>,
    pub id: Option<String>,
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "geojson_feature__create",
    description = "Create a GeoJSON Feature with geometry and optional properties.",
    emit = Auto
)]
async fn feature_create(p: CreateFeatureParams) -> Result<CallToolResult, ErrorData> {
    let prop_count = p.properties.as_ref().map(|m| m.len()).unwrap_or(0);
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created Feature with {} properties", prop_count))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddPropertyParams {
    pub feature: serde_json::Value,
    pub key: String,
    pub value: serde_json::Value,
}

#[elicit_tool(
    plugin = "geojson_feature",
    name = "geojson_feature__add_property",
    description = "Add a property (key-value pair) to a Feature.",
    emit = Auto
)]
async fn feature_add_property(p: AddPropertyParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Added property '{}' to Feature", p.key))
    ]))
}

// ... 4 more tools: get_property, remove_property, list_properties, etc.
```

### 5.3 Conversion Plugin (workflow/conversion_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GeoTypesToGeoJsonParams {
    pub geometry_type: String,  // "Point", "Polygon", etc.
    pub geometry: serde_json::Value,
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geojson_conversion__from_geo_types",
    description = "Convert geo-types geometry to GeoJSON.",
    emit = Auto
)]
async fn conversion_from_geo_types(p: GeoTypesToGeoJsonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted {} to GeoJSON", p.geometry_type))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonToGeoTypesParams {
    pub geojson: serde_json::Value,
}

#[elicit_tool(
    plugin = "geojson_conversion",
    name = "geojson_conversion__to_geo_types",
    description = "Convert GeoJSON geometry to geo-types.",
    emit = Auto
)]
async fn conversion_to_geo_types(p: GeoJsonToGeoTypesParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("Converted GeoJSON to geo-types")
    ]))
}

// ... 6 more tools: batch conversion, validate conversion, etc.
```

### 5.4 I/O Plugin (workflow/io_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadGeoJsonParams {
    pub file_path: String,
}

#[elicit_tool(
    plugin = "geojson_io",
    name = "geojson_io__read_file",
    description = "Read GeoJSON from a file.",
    emit = Auto
)]
async fn io_read_file(p: ReadGeoJsonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Read GeoJSON from {}", p.file_path))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WriteGeoJsonParams {
    pub file_path: String,
    pub geojson: serde_json::Value,
    pub pretty: Option<bool>,
}

#[elicit_tool(
    plugin = "geojson_io",
    name = "geojson_io__write_file",
    description = "Write GeoJSON to a file. Use pretty=true for formatted output.",
    emit = Auto
)]
async fn io_write_file(p: WriteGeoJsonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Wrote GeoJSON to {}", p.file_path))
    ]))
}

// ... 2 more tools: validate_file, format_json, etc.
```

### 5.5 Workflow Plugin (workflow/workflow_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayoutToGeoJsonParams {
    pub layout_nodes: Vec<serde_json::Value>,  // AccessKit nodes
}

#[elicit_tool(
    plugin = "geojson_workflow",
    name = "geojson_workflow__layout_to_geojson",
    description = "Convert elicit_ui layout to GeoJSON FeatureCollection. \
                   Each UI element becomes a Feature with bounds as Polygon.",
    emit = Auto
)]
async fn workflow_layout_to_geojson(p: LayoutToGeoJsonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Converted {} layout nodes to GeoJSON", p.layout_nodes.len()))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BoundsToPolygonParams {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[elicit_tool(
    plugin = "geojson_workflow",
    name = "geojson_workflow__bounds_to_polygon",
    description = "Convert bounding box to GeoJSON Polygon.",
    emit = Auto
)]
async fn workflow_bounds_to_polygon(p: BoundsToPolygonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created Polygon from bounds ({}, {}) to ({}, {})",
            p.x0, p.y0, p.x1, p.y1))
    ]))
}

// ... 4 more tools: merge_collections, filter_features, etc.
```

## Phase 6: Testing

### File to create:
- `crates/elicit_geojson/tests/geojson_test.rs`

### Test Coverage:

```rust
use geojson::{Geometry, Value};
use geo_types::Point;

#[test]
fn test_point_to_geojson() {
    let point = Point::new(10.0, 20.0);
    let geometry: Geometry = (&point).into();

    let json = serde_json::to_string(&geometry).unwrap();
    assert!(json.contains("\"type\":\"Point\""));
    assert!(json.contains("[10.0,20.0]"));
}

#[test]
fn test_geojson_to_point() {
    let json = r#"{"type":"Point","coordinates":[10.0,20.0]}"#;
    let geometry: Geometry = serde_json::from_str(json).unwrap();

    if let Value::Point(coords) = geometry.value {
        assert_eq!(coords, vec![10.0, 20.0]);
    } else {
        panic!("Expected Point geometry");
    }
}

#[test]
fn test_feature_with_properties() {
    let point = Point::new(10.0, 20.0);
    let geometry: Geometry = (&point).into();

    let mut properties = serde_json::Map::new();
    properties.insert("name".to_string(), "Test Point".into());
    properties.insert("value".to_string(), 42.into());

    let feature = geojson::Feature {
        geometry: Some(geometry),
        properties: Some(properties),
        id: None,
        bbox: None,
        foreign_members: None,
    };

    let json = serde_json::to_string(&feature).unwrap();
    assert!(json.contains("\"name\":\"Test Point\""));
    assert!(json.contains("\"value\":42"));
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_geojson/README.md`

### Content:

```markdown
# elicit_geojson

Elicitation-enabled wrappers around [`geojson`](https://docs.rs/geojson) for GeoJSON serialization.

## Purpose

Provides **GeoJSON serialization alphabet** for:
- Exporting UI layouts to visualization tools
- Importing layouts from design tools
- Debugging spatial relationships
- Interop with web mapping libraries

## API Coverage

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `geojson_geometry` | 8 | Create/parse GeoJSON geometries |
| `geojson_feature` | 6 | Features with properties |
| `geojson_collection` | 6 | Feature collections |
| `geojson_conversion` | 8 | geo-types ↔ GeoJSON |
| `geojson_io` | 4 | Read/write GeoJSON files |
| `geojson_workflow` | 6 | Common patterns |

**Total: ~38 MCP tools**

## Usage

```rust
use elicit_geojson::{Geometry, Feature, FeatureCollection};
use geo_types::Point;

// Convert geo-types to GeoJSON
let point = Point::new(10.0, 20.0);
let geometry: Geometry = Geometry::from_geo_types_point(&point);

// Create Feature with properties
let mut properties = serde_json::Map::new();
properties.insert("name".to_string(), "Button".into());
let feature = Feature::new(geometry, Some(properties));

// Serialize to JSON
let json = feature.to_json_string()?;
```

## Integration with elicit_ui

Export UI layouts to GeoJSON for visualization:

```rust
use elicit_ui::Layout;
use elicit_geojson::FeatureCollection;

// Verified layout → GeoJSON
let layout = Layout::from_update(update);
let verified = layout.verify_aa(viewport)?;

// Each element becomes a Feature
let features: Vec<Feature> = verified.nodes()
    .map(|(id, node)| {
        let bounds = node.bounds().unwrap();
        let polygon = bounds_to_polygon(bounds);
        let mut props = serde_json::Map::new();
        props.insert("id".into(), id.to_string().into());
        props.insert("role".into(), format!("{:?}", node.role()).into());
        Feature::new(polygon, Some(props))
    })
    .collect();

let collection = FeatureCollection::new(features);
collection.write_to_file("layout.geojson", true)?;

// Now visualize in:
// - QGIS
// - Mapbox Studio
// - Leaflet/OpenLayers
// - Any GeoJSON viewer
```
```

## Verification Steps

**After implementation**:
1. `cargo check -p elicit_geojson`
2. `cargo test -p elicit_geojson`
3. `cargo check -p elicitation --no-default-features --features geojson`
4. `cargo check --all-features`

**Manual verification**:
1. Create geo-types geometry
2. Convert to GeoJSON via tool
3. Verify JSON output matches GeoJSON spec
4. Round-trip: GeoJSON → geo-types → GeoJSON

## Critical Files

### To create:
- `crates/elicit_geojson/Cargo.toml`
- `crates/elicit_geojson/README.md`
- `crates/elicit_geojson/src/lib.rs`
- `crates/elicit_geojson/src/geometry.rs`
- `crates/elicit_geojson/src/feature.rs`
- `crates/elicit_geojson/src/collection.rs`
- `crates/elicit_geojson/src/workflow/mod.rs`
- `crates/elicit_geojson/src/workflow/geometry_plugin.rs`
- `crates/elicit_geojson/src/workflow/feature_plugin.rs`
- `crates/elicit_geojson/src/workflow/collection_plugin.rs`
- `crates/elicit_geojson/src/workflow/conversion_plugin.rs`
- `crates/elicit_geojson/src/workflow/io_plugin.rs`
- `crates/elicit_geojson/src/workflow/workflow_plugin.rs`
- `crates/elicit_geojson/tests/geojson_test.rs`
- `crates/elicitation/src/geojson_support.rs`

### To modify:
- `Cargo.toml`
- `crates/elicitation/Cargo.toml`
- `crates/elicitation/src/lib.rs`

## Implementation Order

1. **Phase 1**: Workspace configuration (15 min)
2. **Phase 2**: Core type integration (1 hour)
3. **Phase 3**: Create elicit_geojson structure (30 min)
4. **Phase 4**: Implement type wrappers (1 hour)
5. **Phase 5**: Implement MCP tools (~38 tools) (5-7 hours)
6. **Phase 6**: Testing (1 hour)
7. **Phase 7**: Documentation (30 min)

**Total estimated time**: 9-12 hours

## Notes

### Why GeoJSON?

Universal interchange format for geometric data:
- **Standard**: RFC 7946 specification
- **Widely supported**: Every mapping library reads GeoJSON
- **Human-readable**: JSON format, easy to debug
- **Extensible**: Properties allow arbitrary metadata

### Use Cases

**UI Layout Export**:
```json
{
  "type": "FeatureCollection",
  "features": [
    {
      "type": "Feature",
      "geometry": {
        "type": "Polygon",
        "coordinates": [[[0,0], [100,0], [100,50], [0,50], [0,0]]]
      },
      "properties": {
        "element_id": "button_1",
        "role": "Button",
        "label": "Submit",
        "wcag_level": "AA"
      }
    }
  ]
}
```

**Visualization Tools**:
- QGIS - Desktop GIS
- Mapbox Studio - Web mapping
- geojson.io - Online viewer
- Leaflet/OpenLayers - JavaScript libraries

**Debugging**:
Export failed layouts to GeoJSON, visualize spatial issues in mapping tools.
