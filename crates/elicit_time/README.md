# elicit_time

Elicitation-enabled wrappers around [`time`](https://docs.rs/time) datetime types.

## Types

| Type | Inner | Serialization | JsonSchema format |
|---|---|---|---|
| `OffsetDateTime` | `time::OffsetDateTime` | RFC 3339 string | `{ "type": "string", "format": "date-time" }` |
| `PrimitiveDateTime` | `time::PrimitiveDateTime` | ISO 8601 local string | `{ "type": "string" }` |

## Why this crate

`time::OffsetDateTime` and `time::PrimitiveDateTime` have serde support (with the `serde`
feature) but have no `JsonSchema` implementation in schemars. This means they cannot be
used as fields in `#[derive(Elicit)]` structs registered as MCP tools.

`elicit_time` provides newtypes with manual `JsonSchema` implementations that describe
the types as string formats, allowing them to participate fully in the MCP tool ecosystem.

## Usage

```rust
use elicit_time::OffsetDateTime;

// Parse from RFC 3339
let dt = OffsetDateTime::parse("2024-01-15T12:30:00+00:00").unwrap();

// Access components
println!("{}-{}-{}", dt.year(), dt.month(), dt.day());

// Serde round-trip
let json = serde_json::to_string(&dt).unwrap();
let dt2: OffsetDateTime = serde_json::from_str(&json).unwrap();
```

## MCP reflect methods

`OffsetDateTime` exposes: `year`, `month`, `day`, `hour`, `minute`, `second`,
`nanosecond`, `unix_timestamp`, `utc_offset`, `to_rfc3339`.

`PrimitiveDateTime` exposes: `year`, `month`, `day`, `hour`, `minute`, `second`,
`nanosecond`, `to_iso8601`.
