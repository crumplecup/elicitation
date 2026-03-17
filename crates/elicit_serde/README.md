# elicit_serde

MCP tool transport for [`serde`](https://docs.rs/serde) — expose per-type
JSON serialization, deserialization, and structural type conversion as
agent-callable MCP tools.

## Why this crate

`serde`'s `Serialize` / `Deserialize` traits carry **method-level** generic
parameters (`S: Serializer`, `D: Deserializer<'de>`) that cannot be wrapped
directly by `#[reflect_trait]`.  `elicit_serde` solves this by introducing two
concrete wrapper traits that fix the format to `serde_json`, erasing those
generics.  The result is a pair of per-type tool factories and a direct
type-to-type conversion primitive.

## Three tools in one crate

| Feature | Mechanism | Registration |
|---|---|---|
| `{prefix}__to_json` | `SerializeJson` factory | `prime_serialize_json::<T>()` + `register_type` + `instantiate` |
| `{prefix}__from_json` | `DeserializeJson` factory | `prime_deserialize_json::<T>()` + `register_type` + `instantiate` |
| `convert__{t}__to__{u}` | `register_convert::<T, U>()` | direct, no factory slot needed |

---

## Trait factories (`to_json` / `from_json`)

The factory approach registers a type once and then exposes serde operations
as named MCP tools.  Registration follows the standard three-step lifecycle:

1. **Prime** — store the vtable for `T` in the global registry
2. **register_type** — claim a prefix for `T` in a `DynamicToolRegistry`
3. **instantiate** — materialise the per-type tools and notify the agent

```rust,no_run
use elicit_serde::{prime_serialize_json, prime_deserialize_json};
use elicitation::DynamicToolRegistry;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use elicitation_derive::Elicit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
struct Point { x: f64, y: f64 }

#[tokio::main]
async fn main() {
    // Step 1 — prime both factories for Point
    prime_serialize_json::<Point>();
    prime_deserialize_json::<Point>();

    // Steps 2 & 3 — register and instantiate
    let registry = DynamicToolRegistry::new()
        .register_type::<Point>("geo");
    registry.instantiate("crate::SerializeJson", "geo").await.unwrap();
    registry.instantiate("crate::DeserializeJson", "geo").await.unwrap();
    // Exposes: geo__to_json, geo__from_json
}
```

### Tool parameters

**`{prefix}__to_json`** — serializes a `T` value to a compact JSON string.
The `target` parameter must be the JSON representation of a `T`.

**`{prefix}__from_json`** — parses a JSON string into a `T` value.
The `json` parameter must be a valid JSON string encoding a `T`.

### Under the hood

`serde::Serialize<S: Serializer>` and `Deserialize<D: Deserializer>` use
generics at the method level.  `elicit_serde` wraps them in two fixed-format
traits and applies `#[reflect_trait]` to those instead:

| Wrapper | Wraps | Method |
|---|---|---|
| `SerializeJson` | `serde::Serialize` | `to_json(&self) -> Result<String, String>` |
| `DeserializeJson` | `serde::de::DeserializeOwned` | `from_json(&str) -> Result<Self, String>` |

Both have blanket impls — any type that derives serde automatically implements
them.

---

## Type-to-type conversion (`register_convert`)

`DynamicToolRegistry::register_convert::<T, U>()` creates a single concrete
MCP tool that converts any `T` into any `U` via serde's structural data model.
No factory slot or `instantiate` call is needed — the tool appears in
`list_tools()` immediately.

```rust,no_run
use elicit_serde::DynamicToolRegistry;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
struct ConfigV1 { name: String, value: i32 }

#[derive(Serialize, Deserialize, JsonSchema)]
struct ConfigV2 { name: String, value: i64, #[serde(default)] tag: Option<String> }

let registry = DynamicToolRegistry::new()
    .register_convert::<ConfigV1, ConfigV2>();
// Exposes: convert__config_v1__to__config_v2
```

### What it does

```
params (JSON object matching T's schema)
  └─► deserialize as T
        └─► serde_json::to_value(&t)
              └─► deserialize as U
                    └─► serde_json::to_string(&u)  →  tool output
```

The input schema is `T`'s JSON Schema directly — no wrapper object.  The
agent passes the source value as the entire arguments object.

### Auto-naming

Tool names are derived from the last path segment of `std::any::type_name`,
converted to snake_case, and joined with `__to__`:

```
ConfigV1 → config_v1
ConfigV2 → config_v2
tool name: convert__config_v1__to__config_v2
```

These names are back-of-house primitives, intended as building blocks for
agent workflows rather than user-facing operations.

### When it works

Conversion succeeds whenever `T` and `U` are compatible in serde's data model:

- **Schema migration** — `i32` → `i64`, optional new fields with `#[serde(default)]`
- **Newtype unwrapping** — `struct Wrapper(Inner)` ↔ `Inner`
- **Field renaming** — via `#[serde(rename = "...")]` on either side
- **Subset projection** — `U` with fewer fields than `T` (extra fields silently dropped)

### Duplicate registration

Registering the same `(T, U)` pair twice panics immediately:

```rust,should_panic
# use elicit_serde::DynamicToolRegistry;
# use serde::{Serialize, Deserialize};
# use schemars::JsonSchema;
# #[derive(Serialize, Deserialize, JsonSchema)] struct A { x: i32 }
# #[derive(Serialize, Deserialize, JsonSchema)] struct B { x: i32 }
DynamicToolRegistry::new()
    .register_convert::<A, B>()
    .register_convert::<A, B>(); // panics: "already registered"
```

---

## Dependency

```toml
[dependencies]
elicit_serde = { path = "../elicit_serde" }
```

