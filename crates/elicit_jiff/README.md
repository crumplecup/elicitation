# elicit_jiff

[Elicitation] shadow crate for [`jiff`](https://docs.rs/jiff) datetime types —
makes `Zoned` and `Timestamp` usable in `#[derive(Elicit)]` structs and exposes
datetime inspection as MCP tools.

## Why this crate

`jiff` types implement `Serialize`/`Deserialize` but not `JsonSchema`,
blocking their use in MCP tool registrations. This crate provides `JsonSchema`-enabled
newtypes via `schemars`' `jiff02` feature, with transparent serialization and
`Deref` access to all upstream methods.

## Types

| Type | Inner | Description |
|---|---|---|
| `Zoned` | `jiff::Zoned` | Wall-clock datetime with IANA timezone |
| `Timestamp` | `jiff::Timestamp` | Absolute instant in time (TAI-like) |

## MCP tools

| Type | Tools |
|---|---|
| `Zoned` | `year`, `month`, `day`, `hour`, `minute`, `second`, `nanosecond`, `subsec_nanosecond`, `day_of_year`, `days_in_month`, `weekday`, `timezone_name`, `timestamp_seconds`, `in_tz` |
| `Timestamp` | `as_second`, `as_millisecond`, `as_microsecond`, `subsec_nanosecond`, `is_zero`, `signum`, `in_tz` |

Both serialize transparently (jiff's own format).

## Usage

```toml
[dependencies]
elicit_jiff = "0.11"
```

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

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

[Elicitation]: https://crates.io/crates/elicitation
