# elicit_jiff

Elicitation-enabled wrappers around [`jiff`](https://docs.rs/jiff) datetime types.

## Types

| Type | Inner | MCP tools |
|---|---|---|
| `Zoned` | `jiff::Zoned` | year, month, day, hour, minute, second, nanosecond, subsec_nanosecond, day_of_year, days_in_month, weekday, timezone_name, timestamp_seconds, in_tz |
| `Timestamp` | `jiff::Timestamp` | as_second, as_millisecond, as_microsecond, subsec_nanosecond, is_zero, signum, in_tz |

Both implement `JsonSchema` (delegated to jiff via `schemars/jiff02`) and
`Serialize`/`Deserialize` transparently.

## Usage

```rust
use elicit_jiff::{Timestamp, Zoned};

let z = Zoned::parse("2024-01-15T12:30:00+00:00[UTC]").unwrap();
println!("{} {}", z.year(), z.weekday());

// Convert to another timezone
if let Some(ny_time) = z.in_tz("America/New_York".to_string()) {
    println!("New York: {ny_time}");
}

let ts = Timestamp::from_second(1_700_000_000).unwrap();
println!("ms: {}", ts.as_millisecond());
```
