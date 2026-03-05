# elicit_serde

MCP tool transport for [`serde`](https://docs.rs/serde) — expose JSON
serialization and deserialization as concrete MCP tool calls.

## Why this crate

`serde`'s `Serialize`/`Deserialize` traits are generic over their
serializer/deserializer parameters, which makes them impossible to expose
directly as MCP tools. `elicit_serde` uses
[`erased-serde`](https://docs.rs/erased-serde) to erase those generic
parameters and wire four concrete JSON operations into the MCP tool protocol.

## MCP tools

Registered under the `"serde"` namespace:

| Tool | Description |
|---|---|
| `serde__serialize` | Parse `value_json` and re-emit as canonical compact JSON |
| `serde__deserialize` | Parse `json` and return the parsed JSON value |
| `serde__round_trip_check` | Verify no data is lost across a JSON parse → serialize → parse cycle |
| `serde__list_formats` | List supported serialization formats (currently `["json"]`) |

## Usage

```rust,no_run
use elicit_serde::SerdePlugin;
use elicitation::PluginRegistry;

#[tokio::main]
async fn main() {
    let registry = PluginRegistry::new()
        .register("serde", SerdePlugin);
    // registry.serve(rmcp::transport::stdio()).await.unwrap();
}
```

### Round-trip check

```rust
// An agent can verify a value survives serialization unchanged:
// tool: serde__round_trip_check
// args: { "type_name": "MyType", "json": "{\"id\":1,\"name\":\"alice\"}" }
// → "true"
```
