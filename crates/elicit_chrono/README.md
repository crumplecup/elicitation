# elicit_chrono

A **shadow crate** that makes the entire [`chrono`] datetime library available to
AI agents over MCP, without forking chrono or patching its types.

Every public `chrono` type — `NaiveDate`, `NaiveDateTime`, `NaiveTime`,
`DateTime<Utc>`, `DateTime<FixedOffset>`, `TimeDelta`, `FixedOffset`, `Weekday`,
`WeekdaySet`, `Parsed`, `StrftimeItems`, and more — has a drop-in shadow that is
API-compatible with the original and additionally carries `JsonSchema`, full
`Serialize`/`Deserialize`, and MCP tool registration through the
[Elicitation][elicitation-crate] framework.

---

## The Problem

`chrono` types implement `Serialize`/`Deserialize` (when the `serde` feature is
on) but not `JsonSchema`.  MCP tool schemas require `JsonSchema`, so any
`#[derive(Elicit)]` struct that holds a `chrono` type fails to compile.

The obvious workaround — wrapping chrono types in newtype structs, one by one —
scales poorly.  A date-manipulation agent needs dozens of chrono types, each
requiring its own wrapper, manual `Deref` impl, and forwarded method set.

## The Shadow-Crate Solution

`elicit_chrono` provides a one-for-one shadow of every stable public type in
`chrono`.  Shadow types are produced by the `elicit_newtype!` macro, which
generates for each wrapped type:

- `Arc<InnerType>` storage (cheap clone, shared ownership)
- `Deref<Target = InnerType>` (all upstream methods work without forwarding)
- `AsRef<InnerType>`, `From<InnerType>`, `From<Arc<InnerType>>`
- `Serialize`/`Deserialize` delegated to the inner type
- `JsonSchema` delegated to the inner type (or hand-written where chrono does not provide one)

The `reflect_methods!` proc-macro additionally generates MCP tool entries for
every `pub fn` in each `impl` block, so AI agents can call `naive_date.year()`,
`time_delta.num_seconds()`, `weekday_set.intersection(other)`, and 650+ other
methods directly as MCP tools with typed, schema-validated parameters.

## Drop-In Compatibility

Shadow types are substitutes, not adapters.  Because every shadow `Deref`s to
its upstream type, code written for `chrono` works on shadow types without
modification:

```rust
use chrono::{Datelike, Timelike};

// Construction: same as chrono
let d = elicit_chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();

// Access: chrono trait methods work via Deref
assert_eq!(d.year(), 2024);
assert_eq!(d.weekday().to_string(), "Sat");

// Round-trip: Deref back to the inner chrono type with clone
let inner: chrono::NaiveDate = (*d).clone();
assert_eq!(inner, chrono::NaiveDate::from_ymd_opt(2024, 6, 15).unwrap());
```

Method-call parity is verified for all 40 shadowed types (596/651 methods, with
the 55 gap items documented as exceptions in the crate's audit patch set — all
are deprecated upstream items or format-backend traits that shadow types satisfy
automatically through delegation).

## Coverage

| Category | Covered |
|---|---|
| Naive date/time types | `NaiveDate`, `NaiveDateTime`, `NaiveTime` |
| Timezone-aware types | `DateTime<Utc>`, `DateTime<FixedOffset>` |
| Offsets and zones | `FixedOffset`, `Utc`, `Local`, `LocalResult` |
| Duration types | `TimeDelta`, `Duration`, `Days`, `Months` |
| Calendar types | `Weekday`, `WeekdaySet`, `Month`, `IsoWeek`, `NaiveWeek` |
| Formatting | `StrftimeItems`, `Item`, `Parsed`, `Colons`, `Fixed`, `Numeric`, `Pad`, `OffsetFormat`, `OffsetPrecision`, `InternalFixed`, `InternalNumeric`, `SecondsFormat` |
| Iterators | `NaiveDateDaysIterator`, `NaiveDateWeeksIterator` |
| Errors | `ParseError`, `ParseErrorKind`, `ParseMonthError`, `ParseWeekdayError`, `RoundingError`, `OutOfRange`, `OutOfRangeError` |
| Free functions | `parse`, `parse_and_remainder` |

## Architecture

### Type structure

Most shadow types are produced by `elicit_newtype!`:

```rust
elicit_newtype!(chrono::NaiveDate, as NaiveDate, serde);
```

This expands to a tuple struct `NaiveDate(pub Arc<chrono::NaiveDate>)` with the
full complement of delegation impls.  The `reflect_methods!` attribute on each
`impl` block then generates MCP tool wrappers alongside the original methods,
leaving the Rust API unchanged.

Types that are already `Copy` in chrono (e.g. `FixedOffset`, `Weekday`,
`SecondsFormat`) use a lighter wrapper backed by the elicitation framework's own
serialization support rather than `Arc`.

### Serialization strategy

Shadow types delegate `Serialize`/`Deserialize` to the inner chrono type.
Serde's `serde(transparent)` attribute (or equivalent manual impls) ensures that
JSON payloads are byte-for-byte identical to what plain chrono produces — RFC
3339 for datetime types, ISO 8601 for date-only types, and so on.

The `serde_core::Serializer` / `Deserializer` format-side traits are **not**
implemented by shadow types (they are the backend, not the data side).  Shadow
types satisfy `Serialize` and `Deserialize` (the data side), which is sufficient:
any serializer backend that works with the original chrono type works with the
shadow.

### MCP tool registration

Each shadow type ships with:

1. **Instance tools** (via `reflect_methods!`) — one MCP tool per public method,
   with a parameter struct derived automatically from the method signature.
2. **Constructor tools** (in `constructors.rs`) — static methods like
   `NaiveDate::from_ymd_opt` that have no `self` receiver and are registered
   separately with `#[elicit_tool]`.
3. **Registry plugins** — e.g. `NaiveDateRegistryPlugin`, which lets an agent
   hold live `NaiveDate` values across tool calls by storing them in a session
   registry keyed by UUID.

### Typestate workflow

`ChronoWorkflowPlugin` composes the atomic tools into contract-verified
multi-step operations:

```text
UnvalidatedDateStr ──parse()──→ ParsedDateTime  (establishes DateTimeParsed)
                                      │
                            assert_future()   →  FutureDateTimeState
                            assert_in_range() →  RangedDateTimeState
                            compute_duration()→  seconds as i64
                            add_seconds()     →  new ParsedDateTime
```

Each transition is typed: `assert_future` is only callable when
`DateTimeParsed` has been established, and its return type proves
`DateTimeParsed ∧ DateTimeFuture`.  Agents that follow the workflow cannot
produce logically inconsistent datetime states.

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
elicit_chrono = "0.11"
```

### Basic date arithmetic

```rust
use elicit_chrono::{NaiveDate, Duration};

// Same construction API as chrono
let start = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
let delta = Duration::try_days(100).expect("valid delta");
let end = start.checked_add_signed(delta).expect("no overflow");

// Format via the shadow's format() method (takes String, returns String)
println!("{}", end.format("%Y-%m-%d".to_string()));
// → "2024-04-10"
```

### Parsing with format items

```rust
use elicit_chrono::{StrftimeItems, Parsed, parse};

let mut parsed = Parsed::new();
let items = StrftimeItems::new("%Y-%m-%d".to_string());
parse(&mut parsed, "2024-06-15", items).expect("valid input");

println!("{:?} {:?} {:?}", parsed.year(), parsed.month(), parsed.day());
// → Some(2024) Some(6) Some(15)
```

### Weekday sets

```rust
use std::str::FromStr;
use elicit_chrono::{Weekday, WeekdaySet};

let weekdays: WeekdaySet = ["Mon", "Tue", "Wed", "Thu", "Fri"]
    .iter()
    .map(|s| Weekday::from_str(s).unwrap())
    .collect();

let weekend = WeekdaySet::from_array(vec![
    Weekday::from_str("Sat").unwrap(),
    Weekday::from_str("Sun").unwrap(),
]);

let full_week = weekdays.union(weekend);
assert_eq!(full_week.len(), 7);
```

### Using shadow types in an Elicit struct

```rust
use elicitation::Elicit;
use elicit_chrono::{DateTime, NaiveDate};

#[derive(Elicit)]
pub struct Appointment {
    title: String,
    date: NaiveDate,
    confirmed_at: DateTime,
}
```

Because both `NaiveDate` and `DateTime` implement `JsonSchema`, this compiles
and produces a valid MCP tool schema with no additional configuration.

## Relationship to chrono

`elicit_chrono` is an **additive wrapper**, not a fork.  Shadow types delegate
everything to the underlying chrono implementation:

- No chrono logic is duplicated.
- Bugs and improvements in chrono flow through automatically on version bumps.
- Code can mix `elicit_chrono` and `chrono` types freely: convert with
  `From`/`Into`, access the inner value via `Deref`, or clone it with
  `(*shadow).clone()`.

The crate's test suite verifies parity explicitly: for each shadow type, the same
operation is run on both the shadow and its upstream chrono equivalent, and the
results are asserted equal.

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT)
at your option.

[`chrono`]: https://docs.rs/chrono
[elicitation-crate]: https://crates.io/crates/elicitation
