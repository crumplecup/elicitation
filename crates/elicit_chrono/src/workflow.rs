//! `ChronoWorkflowPlugin` — contract-verified datetime composition tools.
//!
//! While the atomic types (`DateTime`, `DateTimeFixed`, `NaiveDateTime`) wrap
//! chrono for MCP reflection, this plugin provides **phrase-level** tools:
//! parsing, temporal assertions, duration computation, and formatting.
//!
//! # Typestate Design
//!
//! ```text
//! UnvalidatedDateStr ──parse()──→ ParsedDateTime + Established<DateTimeParsed>
//!                                       │
//!                             assert_future()
//!                                       │
//!                                       ↓
//!                             FutureDateTime + Established<DateTimeFuture>
//!
//!                                       │ assert_in_range(start, end)
//!                                       ↓
//!                             RangedDateTime + Established<DateTimeInRange>
//! ```
//!
//! # Propositions and Contracts
//!
//! ```text
//! parse_datetime:   DateTimeParsed
//! assert_future:    DateTimeParsed ∧ DateTimeFuture
//! assert_in_range:  DateTimeParsed ∧ DateTimeInRange
//! compute_duration: DateTimeParsed(from) ∧ DateTimeParsed(to)
//! add_seconds:      DateTimeParsed ⟹ DateTimeParsed(result)
//! ```
//!
//! Registered under the `"chrono_workflow"` namespace.

use chrono::{DateTime, Duration, Utc};
use elicitation::contracts::{And, Established};
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow};
use elicitation_derive::reflect_methods;
use rmcp::ErrorData;
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the input string is a valid RFC 3339 datetime.
#[derive(Prop)]
pub struct DateTimeParsed;
impl VerifiedWorkflow for DateTimeParsed {}

/// Proposition: the datetime is strictly in the future (after `Utc::now()`).
#[derive(Prop)]
pub struct DateTimeFuture;
impl VerifiedWorkflow for DateTimeFuture {}

/// Proposition: the datetime falls within the asserted [start, end] range.
#[derive(Prop)]
pub struct DateTimeInRange;
impl VerifiedWorkflow for DateTimeInRange {}

/// Composite: parsed AND in the future.
pub type FutureDateTimeProof = And<DateTimeParsed, DateTimeFuture>;

/// Composite: parsed AND within the declared range.
pub type RangedDateTimeProof = And<DateTimeParsed, DateTimeInRange>;

// ── Typestate structs ─────────────────────────────────────────────────────────

/// An unvalidated datetime string — the initial state.
pub struct UnvalidatedDateStr {
    src: String,
}

/// A successfully parsed UTC datetime.
///
/// Carries the parsed `DateTime<Utc>` internally. Can transition to
/// `FutureDateTimeState` or `RangedDateTimeState`.
pub struct ParsedDateTime {
    /// The inner value carried by this typestate wrapper.
    pub inner: DateTime<Utc>,
}

/// A parsed datetime proven to be strictly in the future.
pub struct FutureDateTimeState {
    /// The inner value carried by this typestate wrapper.
    pub inner: DateTime<Utc>,
}

/// A parsed datetime proven to fall within an asserted range.
pub struct RangedDateTimeState {
    /// The inner value carried by this typestate wrapper.
    pub inner: DateTime<Utc>,
}

// ── Typestate transitions ─────────────────────────────────────────────────────

impl UnvalidatedDateStr {
    /// Wrap a raw string as an unvalidated datetime input.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input as RFC 3339, establishing `DateTimeParsed` proof on success.
    pub fn parse(self) -> Result<(ParsedDateTime, Established<DateTimeParsed>), String> {
        self.src
            .parse::<DateTime<Utc>>()
            .map(|inner| (ParsedDateTime { inner }, Established::assert()))
            .map_err(|e| format!("DateTimeParsed not established: {e}"))
    }
}

impl ParsedDateTime {
    /// Return the inner UTC datetime.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.inner
    }

    /// Assert that this datetime is strictly after `Utc::now()`.
    pub fn assert_future(
        self,
        parsed: Established<DateTimeParsed>,
    ) -> Result<(FutureDateTimeState, Established<FutureDateTimeProof>), String> {
        let now = Utc::now();
        if self.inner > now {
            let proof =
                elicitation::contracts::both(parsed, Established::<DateTimeFuture>::assert());
            Ok((FutureDateTimeState { inner: self.inner }, proof))
        } else {
            Err(format!(
                "DateTimeFuture not established: {} is not after now ({})",
                self.inner.to_rfc3339(),
                now.to_rfc3339()
            ))
        }
    }

    /// Assert that this datetime falls within `[start, end]`.
    pub fn assert_in_range(
        self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        parsed: Established<DateTimeParsed>,
    ) -> Result<(RangedDateTimeState, Established<RangedDateTimeProof>), String> {
        if self.inner >= start && self.inner <= end {
            let proof =
                elicitation::contracts::both(parsed, Established::<DateTimeInRange>::assert());
            Ok((RangedDateTimeState { inner: self.inner }, proof))
        } else {
            Err(format!(
                "DateTimeInRange not established: {} is not within [{}, {}]",
                self.inner.to_rfc3339(),
                start.to_rfc3339(),
                end.to_rfc3339()
            ))
        }
    }
}

impl FutureDateTimeState {
    /// Return the inner UTC datetime.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.inner
    }
}

impl RangedDateTimeState {
    /// Return the inner UTC datetime.
    pub fn into_inner(self) -> DateTime<Utc> {
        self.inner
    }
}

// ── Params structs ────────────────────────────────────────────────────────────

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_rfc3339(s: &str) -> Result<DateTime<Utc>, ErrorData> {
    s.parse::<DateTime<Utc>>().map_err(|e| {
        ErrorData::internal_error(format!("DateTimeParsed not established: {e}"), None)
    })
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing contract-verified chrono datetime composition tools.
///
/// Register under the `"chrono_workflow"` namespace:
///
/// ```ignore
/// use elicitation::PluginRegistry;
/// use elicit_chrono::ChronoWorkflowPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("chrono_workflow", ChronoWorkflowPlugin);
/// ```
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "chrono_workflow")]
pub struct ChronoWorkflowPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[reflect_methods]
impl ChronoWorkflowPlugin {
    /// Parse an RFC 3339 datetime string and normalize it to UTC.
    /// Establishes: DateTimeParsed.
    /// Returns year, month, day, hour, minute, second, weekday, and Unix timestamp.
    #[instrument(skip_all)]
    pub async fn parse_datetime(&self, datetime: String) -> Result<String, ErrorData> {
        let (parsed, _proof) = UnvalidatedDateStr::new(datetime)
            .parse()
            .map_err(|e| ErrorData::internal_error(e, None))?;
        let dt = parsed.inner;
        use chrono::{Datelike, Timelike};
        Ok(format!(
            "DateTimeParsed established.\n\
             rfc3339:   {}\n\
             year:      {}\n\
             month:     {}\n\
             day:       {}\n\
             hour:      {}\n\
             minute:    {}\n\
             second:    {}\n\
             weekday:   {}\n\
             timestamp: {}",
            dt.to_rfc3339(),
            dt.year(),
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            dt.weekday(),
            dt.timestamp(),
        ))
    }

    /// Parse an RFC 3339 datetime and assert it is strictly after the current UTC time.
    /// Establishes: DateTimeParsed ∧ DateTimeFuture.
    /// Useful for validating scheduling inputs before committing a workflow.
    #[instrument(skip_all)]
    pub async fn assert_future(&self, datetime: String) -> Result<String, ErrorData> {
        let (parsed, parsed_proof) = UnvalidatedDateStr::new(datetime)
            .parse()
            .map_err(|e| ErrorData::internal_error(e, None))?;
        let (future, _proof) = parsed
            .assert_future(parsed_proof)
            .map_err(|e| ErrorData::internal_error(e, None))?;
        Ok(format!(
            "DateTimeParsed ∧ DateTimeFuture established.\n\
             datetime: {}\n\
             seconds_from_now: {}",
            future.inner.to_rfc3339(),
            (future.inner - Utc::now()).num_seconds(),
        ))
    }

    /// Parse an RFC 3339 datetime and assert it falls within [start, end] (inclusive).
    /// Establishes: DateTimeParsed ∧ DateTimeInRange.
    /// All three inputs must be valid RFC 3339 strings.
    #[instrument(skip_all)]
    pub async fn assert_in_range(
        &self,
        datetime: String,
        start: String,
        end: String,
    ) -> Result<String, ErrorData> {
        let start = parse_rfc3339(&start)?;
        let end = parse_rfc3339(&end)?;
        let (parsed, parsed_proof) = UnvalidatedDateStr::new(datetime)
            .parse()
            .map_err(|e| ErrorData::internal_error(e, None))?;
        let (ranged, _proof) = parsed
            .assert_in_range(start, end, parsed_proof)
            .map_err(|e| ErrorData::internal_error(e, None))?;
        Ok(format!(
            "DateTimeParsed ∧ DateTimeInRange established.\n\
             datetime: {}\n\
             range:    [{}, {}]",
            ranged.inner.to_rfc3339(),
            start.to_rfc3339(),
            end.to_rfc3339(),
        ))
    }

    /// Compute the signed duration between two RFC 3339 datetimes.
    /// Establishes: DateTimeParsed(from) ∧ DateTimeParsed(to).
    /// Returns duration in seconds, minutes, hours, and days.
    #[instrument(skip_all)]
    pub async fn compute_duration(&self, from: String, to: String) -> Result<String, ErrorData> {
        let from = parse_rfc3339(&from)?;
        let to = parse_rfc3339(&to)?;
        let dur = to.signed_duration_since(from);
        Ok(format!(
            "DateTimeParsed(from) ∧ DateTimeParsed(to) established.\n\
             from:    {}\n\
             to:      {}\n\
             seconds: {}\n\
             minutes: {}\n\
             hours:   {}\n\
             days:    {}",
            from.to_rfc3339(),
            to.to_rfc3339(),
            dur.num_seconds(),
            dur.num_minutes(),
            dur.num_hours(),
            dur.num_days(),
        ))
    }

    /// Add (or subtract) a number of seconds to an RFC 3339 datetime.
    /// Establishes: DateTimeParsed ⟹ DateTimeParsed(result).
    /// Returns the resulting datetime as RFC 3339.
    #[instrument(skip_all)]
    pub async fn add_seconds(&self, datetime: String, seconds: i64) -> Result<String, ErrorData> {
        let dt = parse_rfc3339(&datetime)?;
        let result = dt + Duration::seconds(seconds);
        Ok(format!(
            "DateTimeParsed ⟹ DateTimeParsed(result) established.\n\
             original: {}\n\
             delta_s:  {}\n\
             result:   {}",
            dt.to_rfc3339(),
            seconds,
            result.to_rfc3339(),
        ))
    }
}
