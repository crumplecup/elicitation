# elicit_uuid

Elicitation-enabled `Uuid` newtype — `JsonSchema`-compatible wrapper around `uuid::Uuid` for MCP tool integration.

## Why this crate?

`uuid::Uuid` does not implement `JsonSchema`, which prevents it from being used directly in MCP tool registrations via `#[derive(Elicit)]`. This crate provides a transparent `Uuid` newtype that satisfies the schema requirement while staying fully interoperable with the upstream `uuid` crate.

## What it provides

| Feature | Details |
|---|---|
| `JsonSchema` | Emits `{ "type": "string", "format": "uuid" }` via `schemars` `uuid1` feature |
| `Serialize`/`Deserialize` | Transparent — encodes/decodes identically to `uuid::Uuid` |
| `Deref`/`DerefMut` | All `uuid::Uuid` methods accessible without unwrapping |
| `From`/`Into` | Zero-cost conversion to/from `uuid::Uuid` |
| `#[reflect_methods]` | Inspect and transform UUIDs as MCP tool calls |

## Usage

```rust
use elicit_uuid::Uuid;
use elicitation_derive::Elicit;

#[derive(Debug, Clone, Elicit)]
pub struct Record {
    id: Uuid,
    name: String,
}
```

### Converting from `uuid::Uuid`

```rust
use elicit_uuid::Uuid;

// From uuid::Uuid
let raw = uuid::Uuid::new_v4();
let id: Uuid = raw.into();

// Back to uuid::Uuid
let back: uuid::Uuid = *id;

// Parse from string
let id = Uuid::parse("550e8400-e29b-41d4-a716-446655440000").unwrap();
assert_eq!(id.to_hyphenated(), "550e8400-e29b-41d4-a716-446655440000");
```

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
