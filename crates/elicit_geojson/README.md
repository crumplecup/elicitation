# elicit_geojson

`elicit_geojson` is the [elicitation] shadow crate for the [geojson] crate. It mirrors the upstream
document/value surface — `GeoJson`, `Geometry`, `Value`, `Feature`, `FeatureCollection`, and
`feature::Id` — and exposes them as MCP-compatible types with four workflow plugins.

## Plugins

| Plugin | Namespace | Description |
|---|---|---|
| `GeoJsonDocumentPlugin` | `geojson_document__*` | Parse and inspect top-level GeoJSON documents |
| `GeoJsonGeometryPlugin` | `geojson_geometry__*` | Construct and inspect `Geometry` / `Value` |
| `GeoJsonFeaturePlugin` | `geojson_feature__*` | Construct and inspect `Feature` / `FeatureCollection` / `Id` |
| `GeoJsonConversionPlugin` | `geojson_conversion__*` | Bridge GeoJSON wrappers to and from `elicit_geo_types` |

## Shadow crate concept

`elicit_geojson` shares the familiar type names of the `geojson` crate but exposes them as
serializable, MCP-boundary-safe descriptors. An agent constructs and inspects GeoJSON values
through these tools; no raw file I/O or streaming parser state crosses the boundary.

## Usage

```toml
[dependencies]
elicit_geojson = "0.11"
```

```rust
use elicit_geojson::{
    GeoJsonDocumentPlugin, GeoJsonFeaturePlugin,
    GeoJsonGeometryPlugin, GeoJsonConversionPlugin,
};

let server = server
    .register_plugin(GeoJsonDocumentPlugin::new())
    .register_plugin(GeoJsonGeometryPlugin::new())
    .register_plugin(GeoJsonFeaturePlugin::new())
    .register_plugin(GeoJsonConversionPlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[geojson]: https://crates.io/crates/geojson
