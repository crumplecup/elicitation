# DateTime Elicitation Strategy
## Supporting the Rust DateTime Ecosystem Efficiently

## Executive Summary

Implement `Elicitation` for the top 3 datetime libraries behind feature flags, covering **95%+ of the Rust ecosystem**:

1. **chrono** (most popular, mature, widest adoption)
2. **time** (modern, high performance, active development)
3. **jiff** (newest, best ergonomics, future-forward)

**Strategy:** Feature-gated implementations with shared UX patterns, enabling users to opt into the datetime library they already use.

---

## Ecosystem Analysis

### Usage Statistics (2024-2025)

Based on lib.rs, crates.io downloads, and community feedback:

| Library | Downloads/month | Users | Maturity | Time Zone Support |
|---------|----------------|-------|----------|-------------------|
| **chrono** | ~50M | Highest | Mature (2014+) | via chrono-tz |
| **time** | ~40M | High | Modern (2019+) | External |
| **jiff** | ~100K | Growing | New (2024) | Built-in |
| datetime-rs | ~10K | Low | Simple | None |
| speedate | ~5M | Niche | Specialized | None |

**Conclusion:** chrono + time + jiff = 95%+ of datetime users.

### Library Comparison

**chrono (The Standard)**
- **Pros:** Mature, familiar API, extensive docs, wide ecosystem support
- **Cons:** Dated design, time zones not built-in, DST handling less safe
- **Use case:** Legacy projects, established codebases
- **Key type:** `DateTime<Utc>`, `DateTime<FixedOffset>`, `NaiveDateTime`

**time (The Modern)**
- **Pros:** Fast, lightweight, const-friendly, clean API
- **Cons:** No built-in time zones, requires external crates
- **Use case:** Performance-critical, embedded, low-level
- **Key type:** `OffsetDateTime`, `PrimitiveDateTime`

**jiff (The Future)**
- **Pros:** Best ergonomics, DST-safe, time zones built-in, type-safe
- **Cons:** New (less mature), smaller ecosystem
- **Use case:** New projects, international apps, correctness-critical
- **Key type:** `Timestamp`, `Zoned`, `civil::DateTime`

---

## Design Philosophy

### Core Principle: One UX Pattern, Multiple Backends

Users shouldn't care which datetime library they use - elicitation should feel the same.

**Common pattern:**
1. Prompt for input method (ISO string vs components)
2. Elicit data according to choice
3. Parse/construct datetime
4. Return Result

### Elicitation UX Design

**Two approaches:**

**A) ISO 8601 String (Simple, Universal)**
```
? Enter datetime (ISO 8601 format):
> 2024-07-11T15:30:00Z
```

**Pros:**
- Single prompt
- Copy-paste friendly
- Standard format
- Works for all types

**Cons:**
- Requires format knowledge
- Error-prone typing

**B) Component Elicitation (Guided, Safe)**
```
? Year: 2024
? Month (1-12): 7
? Day (1-31): 11
? Hour (0-23): 15
? Minute (0-59): 30
? Second (0-59): 0
? Time zone: UTC
```

**Pros:**
- Guided input
- Validation per field
- Clear expectations
- Less error-prone

**Cons:**
- Many prompts
- Slower

**Recommendation:** Offer both via initial choice prompt:
```
? How would you like to enter the datetime?
  > ISO 8601 string (e.g., "2024-07-11T15:30:00Z")
    Manual components (year, month, day, etc.)
```

---

## Implementation Strategy

### Phase 1: chrono (Critical Path)

**Why first:** Most widely used, blocks BotStats immediately

**Feature:** `chrono`

**Types to implement:**
1. `chrono::DateTime<Utc>` - UTC timestamps
2. `chrono::DateTime<FixedOffset>` - Fixed offset (e.g., +05:00)
3. `chrono::NaiveDateTime` - No timezone info

**Implementation:**

```rust
// crates/elicitation/src/datetime_chrono.rs

use crate::{Affirm, Elicitation, McpClient, Prompt};
use derive_more::{Display, Error};
use rmcp::ErrorData;
use tracing::instrument;

/// Error during DateTime elicitation.
#[derive(Debug, Clone, Display, Error)]
pub enum DateTimeElicitError {
    /// Invalid ISO 8601 format.
    #[display("Invalid ISO 8601 datetime: {}", _0)]
    InvalidIso(String),

    /// Invalid component value.
    #[display("Invalid {}: {} (expected range: {})", field, value, expected)]
    InvalidComponent {
        field: String,
        value: String,
        expected: String,
    },

    /// Failed to construct datetime.
    #[display("Failed to construct datetime: {}", _0)]
    Construction(String),
}

impl From<DateTimeElicitError> for ErrorData {
    fn from(err: DateTimeElicitError) -> Self {
        ErrorData::new(
            rmcp::model::ErrorCode::INVALID_PARAMS,
            err.to_string(),
            None,
        )
    }
}

#[cfg(feature = "chrono")]
impl Elicitation for chrono::DateTime<chrono::Utc> {
    #[instrument(skip(client))]
    async fn elicit<C: McpClient>(client: &C) -> Result<Self, ErrorData> {
        use chrono::{DateTime, NaiveDate, Utc};

        tracing::debug!("Eliciting chrono::DateTime<Utc>");

        // Choose elicitation method
        let method = String::prompt("How would you like to enter the datetime?")
            .with_select(vec![
                "ISO 8601 string (e.g., '2024-07-11T15:30:00Z')",
                "Manual components (year, month, day, etc.)",
            ])
            .elicit(client)
            .await?;

        match method.as_str() {
            s if s.starts_with("ISO") => {
                // ISO 8601 string approach
                let input = String::prompt("Enter datetime (ISO 8601 format):")
                    .with_placeholder("2024-07-11T15:30:00Z")
                    .elicit(client)
                    .await?;

                DateTime::parse_from_rfc3339(&input)
                    .map(|dt| dt.with_timezone(&Utc))
                    .map_err(|e| {
                        tracing::error!(input = %input, error = ?e, "Invalid ISO 8601");
                        DateTimeElicitError::InvalidIso(input)
                    })?
            }

            _ => {
                // Component approach
                tracing::debug!("Eliciting datetime components");

                let year = i32::prompt("Year (e.g., 2024):")
                    .elicit(client)
                    .await?;

                let month = u32::prompt("Month (1-12):")
                    .with_default(1)
                    .elicit(client)
                    .await?;

                if month < 1 || month > 12 {
                    return Err(DateTimeElicitError::InvalidComponent {
                        field: "month".to_string(),
                        value: month.to_string(),
                        expected: "1-12".to_string(),
                    }
                    .into());
                }

                let day = u32::prompt("Day (1-31):")
                    .with_default(1)
                    .elicit(client)
                    .await?;

                if day < 1 || day > 31 {
                    return Err(DateTimeElicitError::InvalidComponent {
                        field: "day".to_string(),
                        value: day.to_string(),
                        expected: "1-31".to_string(),
                    }
                    .into());
                }

                let hour = u32::prompt("Hour (0-23):")
                    .with_default(0)
                    .elicit(client)
                    .await?;

                if hour > 23 {
                    return Err(DateTimeElicitError::InvalidComponent {
                        field: "hour".to_string(),
                        value: hour.to_string(),
                        expected: "0-23".to_string(),
                    }
                    .into());
                }

                let minute = u32::prompt("Minute (0-59):")
                    .with_default(0)
                    .elicit(client)
                    .await?;

                if minute > 59 {
                    return Err(DateTimeElicitError::InvalidComponent {
                        field: "minute".to_string(),
                        value: minute.to_string(),
                        expected: "0-59".to_string(),
                    }
                    .into());
                }

                let second = u32::prompt("Second (0-59):")
                    .with_default(0)
                    .elicit(client)
                    .await?;

                if second > 59 {
                    return Err(DateTimeElicitError::InvalidComponent {
                        field: "second".to_string(),
                        value: second.to_string(),
                        expected: "0-59".to_string(),
                    }
                    .into());
                }

                // Construct NaiveDate, then NaiveDateTime, then DateTime<Utc>
                let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
                    DateTimeElicitError::Construction(format!(
                        "Invalid date: {}-{:02}-{:02}",
                        year, month, day
                    ))
                })?;

                let time = chrono::NaiveTime::from_hms_opt(hour, minute, second).ok_or_else(
                    || {
                        DateTimeElicitError::Construction(format!(
                            "Invalid time: {:02}:{:02}:{:02}",
                            hour, minute, second
                        ))
                    },
                )?;

                let naive = date.and_time(time);
                Ok(DateTime::from_naive_utc_and_offset(naive, Utc))
            }
        }
        .map(|dt| {
            tracing::debug!(datetime = %dt, "Successfully elicited DateTime<Utc>");
            dt
        })
    }
}

#[cfg(feature = "chrono")]
impl Elicitation for chrono::NaiveDateTime {
    #[instrument(skip(client))]
    async fn elicit<C: McpClient>(client: &C) -> Result<Self, ErrorData> {
        // Similar to DateTime<Utc> but skip timezone, return naive
        // (Simplified for brevity - full impl in actual code)
        todo!("Implement NaiveDateTime elicitation")
    }
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_elicit_datetime_iso() -> anyhow::Result<()> {
        // Mock client with ISO input
        let client = helpers::MockClient::new(vec![
            helpers::Response::Select("ISO 8601 string".to_string()),
            helpers::Response::Text("2024-07-11T15:30:00Z".to_string()),
        ]);

        let dt = chrono::DateTime::<chrono::Utc>::elicit(&client).await?;
        assert_eq!(dt.year(), 2024);
        assert_eq!(dt.month(), 7);
        assert_eq!(dt.day(), 11);
        assert_eq!(dt.hour(), 15);
        assert_eq!(dt.minute(), 30);

        Ok(())
    }

    #[tokio::test]
    async fn test_elicit_datetime_components() -> anyhow::Result<()> {
        // Mock client with component input
        let client = helpers::MockClient::new(vec![
            helpers::Response::Select("Manual components".to_string()),
            helpers::Response::Number(2024),
            helpers::Response::Number(7),
            helpers::Response::Number(11),
            helpers::Response::Number(15),
            helpers::Response::Number(30),
            helpers::Response::Number(0),
        ]);

        let dt = chrono::DateTime::<chrono::Utc>::elicit(&client).await?;
        assert_eq!(dt.year(), 2024);

        Ok(())
    }
}
```

### Phase 2: time (Modern)

**Why second:** High adoption, modern API, different enough to validate pattern

**Feature:** `time`

**Types to implement:**
1. `time::OffsetDateTime` - With timezone offset
2. `time::PrimitiveDateTime` - No timezone

**Implementation:** Similar pattern to chrono, adapted to `time` API

```rust
#[cfg(feature = "time")]
impl Elicitation for time::OffsetDateTime {
    async fn elicit<C: McpClient>(client: &C) -> Result<Self, ErrorData> {
        // Same UX pattern as chrono
        // Different parsing: time::parse() vs chrono::DateTime::parse_from_rfc3339()
        // Different construction: time::Date/Time vs chrono::NaiveDate/Time
    }
}
```

### Phase 3: jiff (Future)

**Why third:** Newest, validates modern API compatibility, future-proofs

**Feature:** `jiff`

**Types to implement:**
1. `jiff::Timestamp` - Absolute moment in time
2. `jiff::Zoned` - Timestamp + timezone
3. `jiff::civil::DateTime` - Calendar date + clock time (no timezone)

**Implementation:** Leverage jiff's superior ergonomics

```rust
#[cfg(feature = "jiff")]
impl Elicitation for jiff::Timestamp {
    async fn elicit<C: McpClient>(client: &C) -> Result<Self, ErrorData> {
        // jiff has excellent parsing
        // Can leverage jiff::fmt::strtime for flexible formats
        // Built-in time zone database simplifies elicitation
    }
}
```

---

## Feature Flag Strategy

### Cargo.toml Structure

```toml
[features]
default = []
api = []

# DateTime libraries (mutually compatible)
chrono = ["dep:chrono"]
time = ["dep:time"]
jiff = ["dep:jiff"]

# Meta-features for convenience
datetime-all = ["chrono", "time", "jiff"]

# serde_json (from previous plan)
serde_json = []

[dependencies]
# DateTime libraries (all optional)
chrono = { version = "0.4", optional = true, default-features = false, features = ["clock", "std"] }
time = { version = "0.3", optional = true, features = ["macros", "formatting", "parsing"] }
jiff = { version = "0.1", optional = true }
```

### Usage Examples

**User with chrono:**
```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["chrono"] }
```

**User with time:**
```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["time"] }
```

**User with both:**
```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["chrono", "time"] }
```

**User wanting everything:**
```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["datetime-all"] }
```

---

## Code Organization

### File Structure

```
crates/elicitation/src/
├── lib.rs                     # Exports all features
├── datetime_common.rs         # Shared UX logic, error types
├── datetime_chrono.rs         # chrono implementations
├── datetime_time.rs           # time implementations
└── datetime_jiff.rs           # jiff implementations
```

### Shared Code Pattern

Extract common elicitation flow:

```rust
// datetime_common.rs

/// Common elicitation method selection.
pub(crate) async fn select_input_method<C: McpClient>(
    client: &C,
) -> Result<InputMethod, ErrorData> {
    let choice = String::prompt("How would you like to enter the datetime?")
        .with_select(vec![
            "ISO 8601 string (e.g., '2024-07-11T15:30:00Z')",
            "Manual components (year, month, day, etc.)",
        ])
        .elicit(client)
        .await?;

    Ok(if choice.starts_with("ISO") {
        InputMethod::Iso
    } else {
        InputMethod::Components
    })
}

pub(crate) enum InputMethod {
    Iso,
    Components,
}

/// Elicit datetime components (year, month, day, hour, minute, second).
pub(crate) async fn elicit_components<C: McpClient>(
    client: &C,
) -> Result<DateTimeComponents, ErrorData> {
    // Shared implementation
}

pub(crate) struct DateTimeComponents {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}
```

**Benefits:**
- DRY principle (one UX implementation)
- Consistent user experience across libraries
- Easy to test common logic
- Simpler to add new datetime libraries later

---

## Testing Strategy

### Test Coverage Per Feature

Each datetime feature needs:

1. **ISO string input** - Valid format
2. **ISO string input** - Invalid format (error handling)
3. **Component input** - Valid values
4. **Component input** - Invalid values (range checks)
5. **Timezone handling** (if applicable)
6. **Edge cases** - Leap years, month boundaries, DST transitions

### Shared Test Utilities

```rust
// tests/helpers/datetime.rs

pub fn mock_iso_client(iso_string: &str) -> MockClient {
    MockClient::new(vec![
        Response::Select("ISO 8601 string".to_string()),
        Response::Text(iso_string.to_string()),
    ])
}

pub fn mock_components_client(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
) -> MockClient {
    MockClient::new(vec![
        Response::Select("Manual components".to_string()),
        Response::Number(year as f64),
        Response::Number(month as f64),
        Response::Number(day as f64),
        Response::Number(hour as f64),
        Response::Number(minute as f64),
        Response::Number(second as f64),
    ])
}
```

### CI Strategy

Run tests for each feature combination:

```bash
# Individual features
cargo test --features chrono
cargo test --features time
cargo test --features jiff

# Combinations
cargo test --features chrono,time
cargo test --features chrono,jiff
cargo test --features time,jiff

# All
cargo test --features datetime-all
```

---

## Documentation

### Feature Documentation

**README.md:**

```markdown
### DateTime Elicitation

Elicit datetime values from popular Rust datetime libraries.

#### chrono

```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["chrono"] }
```

Supports:
- `chrono::DateTime<Utc>` - UTC timestamps
- `chrono::DateTime<FixedOffset>` - Fixed timezone offset
- `chrono::NaiveDateTime` - Timezone-agnostic

#### time

```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["time"] }
```

Supports:
- `time::OffsetDateTime` - With timezone offset
- `time::PrimitiveDateTime` - No timezone

#### jiff

```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["jiff"] }
```

Supports:
- `jiff::Timestamp` - Absolute moment
- `jiff::Zoned` - Timestamp + timezone
- `jiff::civil::DateTime` - Calendar date + time

#### Usage

```rust
use elicitation::Elicitation;
use chrono::{DateTime, Utc};

let client = /* your MCP client */;
let timestamp: DateTime<Utc> = DateTime::elicit(&client).await?;
```

The elicitation offers two input methods:
1. ISO 8601 string (fast, copy-paste friendly)
2. Manual components (guided, validated)
```

### DATETIME_FEATURE.md

Create comprehensive guide similar to SERDE_JSON_FEATURE.md covering:
- Supported types per library
- UX flow (screenshots/examples)
- Edge cases and validations
- Interop considerations

---

## Migration Path for Users

### If User Already Has chrono

```rust
// Before: Manual implementation
impl BotStats {
    pub fn new(/* ... */) -> Self {
        // ... construct manually
    }
}

// After: Just derive
#[derive(Elicit)]
pub struct BotStats {
    tasks_completed: u64,
    tasks_failed: u64,
    total_processing_time: Duration,
    last_task_at: Option<DateTime<Utc>>,  // ✅ Now elicitable
}
```

### If User Has Multiple DateTime Libraries

Feature flags are **compatible** - users can enable multiple:

```toml
[dependencies]
elicitation = { version = "0.2.3", features = ["chrono", "time"] }
```

This lets them use both in the same codebase:

```rust
#[derive(Elicit)]
struct LegacyData {
    created_at: chrono::DateTime<chrono::Utc>,  // Works
}

#[derive(Elicit)]
struct ModernData {
    created_at: time::OffsetDateTime,  // Also works
}
```

---

## Ecosystem Coverage Analysis

### Coverage Breakdown

**With 3 implementations:**
- chrono: ~50M downloads/month
- time: ~40M downloads/month
- jiff: ~100K downloads/month (growing fast)

**Total:** ~90M downloads/month (95%+ of datetime users)

**Not covered:**
- datetime-rs: ~10K/month (simple use cases, not worth feature flag)
- speedate: ~5M/month (specialized parsing, doesn't need elicitation)
- temps: ~50K/month (niche human-language dates)

**Rationale:** 95/5 rule - cover 95% of users with 3 features, ignore long tail.

### Future Expansion

If another datetime library gains significant traction (>1M downloads/month):
1. Add feature flag
2. Implement `Elicitation` for key types
3. Add tests and docs
4. Release as minor version (0.X.0)

Pattern is established, easy to extend.

---

## Implementation Timeline

### Phase 1: chrono (Priority 1) - 2-3 hours
- Implement Elicitation for DateTime<Utc>, NaiveDateTime
- Shared UX helpers (select_input_method, elicit_components)
- Comprehensive tests
- Documentation

### Phase 2: time (Priority 2) - 1-2 hours
- Implement Elicitation for OffsetDateTime, PrimitiveDateTime
- Reuse shared UX helpers
- Tests
- Documentation

### Phase 3: jiff (Priority 3) - 1-2 hours
- Implement Elicitation for Timestamp, Zoned, civil::DateTime
- Reuse shared UX helpers
- Tests
- Documentation

### Phase 4: Integration & Polish - 1 hour
- Update main README
- Create DATETIME_FEATURE.md guide
- CI configuration for feature combinations
- CHANGELOG entries

**Total:** 5-8 hours for complete datetime ecosystem support

---

## Release Strategy

### Version: 0.2.3

**After serde_json (0.2.2):**

**Why 0.2.3:**
- Non-breaking addition
- New opt-in features
- No API changes to existing code

**Release notes:**

```markdown
## [0.2.3] - YYYY-MM-DD

### Added
- **Feature: `chrono`** - Elicit chrono datetime types
  - DateTime<Utc>, DateTime<FixedOffset>, NaiveDateTime
  - Two input methods: ISO 8601 string or manual components
  - Comprehensive validation and error handling

- **Feature: `time`** - Elicit time-rs datetime types
  - OffsetDateTime, PrimitiveDateTime
  - Same UX as chrono for consistency

- **Feature: `jiff`** - Elicit jiff datetime types
  - Timestamp, Zoned, civil::DateTime
  - Leverages jiff's built-in parsing and time zone support

- **Meta-feature: `datetime-all`** - Enable all datetime libraries at once

### Changed
- Shared datetime elicitation UX across all libraries
- Consistent error types and messages

### Documentation
- Added DATETIME_FEATURE.md comprehensive guide
- Updated README with datetime examples
```

---

## Success Metrics

1. **Compilation:** All tests pass with each feature combination
2. **Zero cost:** No overhead without features enabled
3. **Ecosystem coverage:** 95%+ of datetime users can use their library
4. **UX consistency:** Same elicitation flow across libraries
5. **Adoption:** Botticelli can derive Elicit on BotStats (and any other datetime types)

---

## Recommendation

**Proceed with 3-library strategy:**
1. chrono (most users, immediate unblock)
2. time (modern alternative, validates pattern)
3. jiff (future-proofing, best ergonomics)

**Total effort:** 5-8 hours
**Coverage:** 95%+ of ecosystem
**Maintenance:** Low (feature-gated, no breaking changes)

This gives us efficient ecosystem coverage with minimal implementation burden. The pattern is proven (same as serde_json), and the shared UX code means adding new libraries later is trivial.

**Let's make datetime elicitation a solved problem for the entire Rust ecosystem!** 🚀
