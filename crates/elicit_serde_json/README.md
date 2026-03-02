# elicit\_serde\_json

MCP tool transport for [`serde_json`](https://docs.rs/serde_json) types — expose
`Value`, `Number`, and JSON navigation as MCP tools for agent workflows.

## What it does

`elicit_serde_json` wraps `serde_json`'s concrete types as elicitation-enabled newtypes
with MCP tools generated from their methods via `#[reflect_methods]`. Every method on
the wrapped type becomes a tool an agent can call by name.

```
serde_json::Value  →  JsonValue  (15 tools)
serde_json::Number →  JsonNumber  (7 tools)
```

## Usage

```rust,ignore
use elicit_serde_json::{JsonValue, JsonNumber};
use serde_json::json;

// Wrap a Value
let v: JsonValue = json!({"name": "Alice", "age": 30}).into();

// Methods become MCP tools automatically via #[reflect_methods]
assert_eq!(v.type_name(), "object");
assert!(!v.is_null());
assert_eq!(v.len(), Some(2));

// JSON Pointer navigation
let name = v.pointer("/name".to_string());

// Serialize back
let json_str = v.to_json_string();
```

## Design

This crate follows the same pattern as [`elicit_reqwest`](../elicit_reqwest):

1. `elicit_newtype!(serde_json::Value, as JsonValue, serde)` — generates an
   `Arc<T>` wrapper with transparent `Serialize + Deserialize` (via the `serde` flag)
2. `#[reflect_methods]` on the `impl` block — generates MCP param/output structs
   and tool routing for every method

Because `serde_json::Value` implements `Serialize + Deserialize`, the `serde` arm
of `elicit_newtype!` delegates serialization transparently through `Arc<Value>`.

## Generated tools

### `JsonValue`

| Method | Description |
|---|---|
| `is_null` | Returns true if the value is `null` |
| `is_boolean` | Returns true if the value is a boolean |
| `is_number` | Returns true if the value is a number |
| `is_string` | Returns true if the value is a string |
| `is_array` | Returns true if the value is an array |
| `is_object` | Returns true if the value is an object |
| `as_bool` | Returns the boolean, or `None` |
| `as_str` | Returns the string, or `None` |
| `as_i64` | Returns as `i64`, or `None` |
| `as_u64` | Returns as `u64`, or `None` |
| `as_f64` | Returns as `f64`, or `None` |
| `len` | Number of elements (array/object), or `None` |
| `is_empty` | True if empty array or object |
| `type_name` | JSON type name: `"null"`, `"bool"`, `"number"`, `"string"`, `"array"`, `"object"` |
| `pointer` | RFC 6901 JSON Pointer lookup |
| `as_number` | Returns as `JsonNumber`, or `None` |
| `to_json_string` | Compact JSON serialization |
| `to_json_string_pretty` | Pretty-printed JSON serialization |

### `JsonNumber`

| Method | Description |
|---|---|
| `is_i64` | True if representable as `i64` |
| `is_u64` | True if representable as `u64` |
| `is_f64` | True if a float |
| `as_i64` | Returns as `i64`, or `None` |
| `as_u64` | Returns as `u64`, or `None` |
| `as_f64` | Returns as `f64`, or `None` |
| `to_json_string` | JSON string representation |

## Relationship to other crates

| Crate | Role |
|---|---|
| `elicit_reqwest` | Fetches HTTP responses, returns JSON payloads |
| `elicit_serde_json` | Navigates and inspects those JSON payloads |
| `elicit_serde` | Validates typed round-trips for registered types |

## License

Apache-2.0 OR MIT
