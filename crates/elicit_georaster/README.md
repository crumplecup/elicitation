# elicit_georaster

`elicit_georaster` is the [elicitation] shadow crate for the [georaster] GeoTIFF reader. It mirrors
the current upstream surface — `Coordinate`, `GeoTiffReader`, `ImageInfo`, `Pixels`, and
`RasterValue` — and exposes them as MCP-compatible types with two workflow plugins.

## Plugins

| Plugin | Namespace | Description |
|---|---|---|
| `GeoTiffReaderPlugin` | `geotiff_reader__*` | Open and configure readers; inspect image metadata |
| `GeoTiffSamplingPlugin` | `geotiff_sampling__*` | Read pixels, convert coordinates, collect windows |

## Shadow crate concept

`elicit_georaster` shares the familiar type names of the `georaster` crate but exposes them as
serializable, MCP-boundary-safe descriptors. An agent can describe raster data sources and sampling
operations without holding live file handles or GPU-resident buffers across the MCP boundary.

## Usage

```toml
[dependencies]
elicit_georaster = "0.11"
```

```rust
use elicit_georaster::{GeoTiffReaderPlugin, GeoTiffSamplingPlugin};

let server = server
    .register_plugin(GeoTiffReaderPlugin::new())
    .register_plugin(GeoTiffSamplingPlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[georaster]: https://crates.io/crates/georaster
