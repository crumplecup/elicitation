# elicit_proj

`elicit_proj` is the [elicitation] shadow crate for the [proj] coordinate transformation library.
It provides a serializable [`ProjTransform`] snapshot wrapper around `proj::Proj` and one MCP
workflow plugin for creating transforms and converting coordinates and geometries between CRS.

## Plugin

| Plugin | Namespace | Description |
|---|---|---|
| `ProjTransformPlugin` | `proj__*` | Create transforms, convert coordinates, project geometries, transform bounds |

## Tool reference

| Tool | Description | Establishes |
|---|---|---|
| `create_from_proj_string` | Create a transform from a PROJ string definition | `ProjCreated` |
| `create_from_known_crs` | Create a transform between two known CRS identifiers (e.g. `EPSG:4326`) | `ProjCreated` |
| `convert_coord` | Convert a single coordinate from source CRS to target CRS | — |
| `project_coord` | Project a coordinate to/from the projection plane | — |
| `convert_geometry` | Convert all coordinates in a geometry from source CRS to target CRS | — |
| `transform_bounds` | Transform a bounding box between CRS, densifying edges for accuracy | — |
| `definition` | Return the PROJ definition string for a transform snapshot | — |

## Usage

```toml
[dependencies]
elicit_proj = "0.11"
```

```rust
use elicit_proj::ProjTransformPlugin;

let server = server.register_plugin(ProjTransformPlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[proj]: https://crates.io/crates/proj
[`ProjTransform`]: https://docs.rs/elicit_proj/latest/elicit_proj/struct.ProjTransform.html
