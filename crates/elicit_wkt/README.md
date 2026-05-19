# elicit_wkt

`elicit_wkt` is the [elicitation] shadow crate for the [wkt] Well-Known Text library. It provides
shadow-crate wrappers over the `elicitation::Wkt*` types plus a parsed `WktItem` wrapper for
structured `wkt::Wkt<f64>` values, exposed through two workflow plugins.

## Plugins

| Plugin | Namespace | Description |
|---|---|---|
| `WktTypesPlugin` | `wkt_types__*` | Explicit constructor tools for WKT coordinate and geometry wrappers |
| `WktParsePlugin` | `wkt_parse__*` | Parse and inspect structured WKT values via `WktItem` |

## Types

`elicit_wkt` re-exports the standard WKT geometry types with familiar names:

`Coord`, `Point`, `LineString`, `Polygon`, `MultiPoint`, `MultiLineString`, `MultiPolygon`,
`GeometryCollection`

The `WktItem` type wraps a parsed `wkt::Wkt<f64>` value and provides structured inspection tools
for MCP agents.

## Usage

```toml
[dependencies]
elicit_wkt = "0.11"
```

```rust
use elicit_wkt::{WktTypesPlugin, WktParsePlugin};

let server = server
    .register_plugin(WktTypesPlugin::new())
    .register_plugin(WktParsePlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[wkt]: https://crates.io/crates/wkt
