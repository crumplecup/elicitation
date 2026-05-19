# elicit_wkb

`elicit_wkb` is the [elicitation] shadow crate for the [wkb] Well-Known Binary library. It mirrors
the upstream reader/writer surface — `Endianness`, reader types, and writer types — and exposes them
as MCP-compatible types with two workflow plugins.

## Plugins

| Plugin | Namespace | Description |
|---|---|---|
| `WkbReaderPlugin` | `wkb_reader__*` | Parse and inspect WKB-encoded geometry values |
| `WkbWriterPlugin` | `wkb_writer__*` | Write geo-types wrapper values to WKB bytes |

## Shadow crate concept

`elicit_wkb` shares the familiar type names of the `wkb` crate but exposes them as serializable,
MCP-boundary-safe descriptors. An agent can describe WKB serialization and deserialization
operations without holding raw byte buffers across the boundary.

## Usage

```toml
[dependencies]
elicit_wkb = "0.11"
```

```rust
use elicit_wkb::{WkbReaderPlugin, WkbWriterPlugin};

let server = server
    .register_plugin(WkbReaderPlugin::new())
    .register_plugin(WkbWriterPlugin::new());
```

[elicitation]: https://crates.io/crates/elicitation
[wkb]: https://crates.io/crates/wkb
