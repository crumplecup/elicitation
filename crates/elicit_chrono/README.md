# elicit_chrono

[Elicitation] shadow crate for [`chrono`](https://docs.rs/chrono) datetime types —
makes `DateTime<Utc>`, `DateTime<FixedOffset>`, and `NaiveDateTime` usable in
`#[derive(Elicit)]` structs and exposes datetime inspection as MCP tools.

## Why this crate

`chrono` datetime types implement `Serialize`/`Deserialize` but not `JsonSchema`,
blocking their use in MCP tool registrations. This crate provides `JsonSchema`-enabled
newtypes via `schemars`' `chrono04` feature, with transparent serialization and
`Deref` access to all upstream methods.

## Types

| Type | Inner | Description |
|---|---|---|
| `DateTimeUtc` | `chrono::DateTime<Utc>` | UTC timestamp |
| `DateTimeFixed` | `chrono::DateTime<FixedOffset>` | Timezone-offset timestamp |
| `NaiveDateTime` | `chrono::NaiveDateTime` | Calendar datetime without timezone |

## MCP tools

| Type | Tools |
|---|---|
| `DateTimeUtc` | `year`, `month`, `day`, `hour`, `minute`, `second`, `timestamp`, `weekday`, `ordinal`, `to_rfc3339`, `to_rfc2822` |
| `DateTimeFixed` | `year`, `month`, `day`, `hour`, `minute`, `second`, `timestamp`, `offset_seconds`, `weekday`, `ordinal`, `to_rfc3339` |
| `NaiveDateTime` | `year`, `month`, `day`, `hour`, `minute`, `second`, `timestamp`, `weekday`, `ordinal`, `format_str` |

All three serialize in RFC 3339 format.

## Usage

```toml
[dependencies]
elicit_chrono = "0.11"
```

```rust
use elicit_chrono::{DateTimeUtc, NaiveDateTime};

let dt = DateTimeUtc::parse("2024-01-15T12:30:00Z").unwrap();
println!("{}-{}-{} {:?}", dt.year(), dt.month(), dt.day(), dt.weekday());

let naive = NaiveDateTime::parse("2024-01-15T12:30:00").unwrap();
println!("{}", naive.format_str("%Y/%m/%d".to_string()));
```

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

[Elicitation]: https://crates.io/crates/elicitation
