# elicit_chrono

Elicitation-enabled wrappers around [`chrono`](https://docs.rs/chrono) datetime types.

## Types

| Type | Inner | MCP tools |
|---|---|---|
| `DateTimeUtc` | `chrono::DateTime<Utc>` | year, month, day, hour, minute, second, timestamp, weekday, ordinal, to_rfc3339, to_rfc2822 |
| `DateTimeFixed` | `chrono::DateTime<FixedOffset>` | year, month, day, hour, minute, second, timestamp, offset_seconds, weekday, ordinal, to_rfc3339 |
| `NaiveDateTime` | `chrono::NaiveDateTime` | year, month, day, hour, minute, second, timestamp, weekday, ordinal, format_str |

All three implement `JsonSchema` (delegated to the inner chrono type via `schemars/chrono04`)
and `Serialize`/`Deserialize` in RFC 3339 format.

## Usage

```rust
use elicit_chrono::{DateTimeUtc, NaiveDateTime};

let dt = DateTimeUtc::parse("2024-01-15T12:30:00Z").unwrap();
println!("{}-{}-{} {:?}", dt.year(), dt.month(), dt.day(), dt.weekday());

let naive = NaiveDateTime::parse("2024-01-15T12:30:00").unwrap();
println!("{}", naive.format_str("%Y/%m/%d".to_string()));
```
