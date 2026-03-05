//! `ChronoWorkflowPlugin` — contract-verified datetime composition tools.
//!
//! While the atomic types (`DateTimeUtc`, `DateTimeFixed`, `NaiveDateTime`) wrap
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
use elicitation::ElicitPlugin;
use elicitation::contracts::{And, Established, Prop};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the input string is a valid RFC 3339 datetime.
pub struct DateTimeParsed;
impl Prop for DateTimeParsed {}

/// Proposition: the datetime is strictly in the future (after `Utc::now()`).
pub struct DateTimeFuture;
impl Prop for DateTimeFuture {}

/// Proposition: the datetime falls within the asserted [start, end] range.
pub struct DateTimeInRange;
impl Prop for DateTimeInRange {}

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
    inner: DateTime<Utc>,
}

/// A parsed datetime proven to be strictly in the future.
pub struct FutureDateTimeState {
    inner: DateTime<Utc>,
}

/// A parsed datetime proven to fall within an asserted range.
pub struct RangedDateTimeState {
    inner: DateTime<Utc>,
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

/// Parameters for [`ChronoWorkflowPlugin::parse_datetime`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ParseDateTimeParams {
    /// RFC 3339 datetime string (e.g. `"2025-03-05T12:00:00Z"`).
    pub datetime: String,
}

/// Parameters for [`ChronoWorkflowPlugin::assert_future`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AssertFutureParams {
    /// RFC 3339 datetime string to check.
    pub datetime: String,
}

/// Parameters for [`ChronoWorkflowPlugin::assert_in_range`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AssertInRangeParams {
    /// RFC 3339 datetime to check.
    pub datetime: String,
    /// RFC 3339 start of the inclusive range.
    pub start: String,
    /// RFC 3339 end of the inclusive range.
    pub end: String,
}

/// Parameters for [`ChronoWorkflowPlugin::compute_duration`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ComputeDurationParams {
    /// RFC 3339 start datetime. Assumes: valid, before `to`.
    pub from: String,
    /// RFC 3339 end datetime. Assumes: valid, after `from`.
    pub to: String,
}

/// Parameters for [`ChronoWorkflowPlugin::add_seconds`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AddSecondsParams {
    /// RFC 3339 base datetime.
    pub datetime: String,
    /// Seconds to add (negative to subtract).
    pub seconds: i64,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn typed_tool<T: JsonSchema + 'static>(name: &'static str, description: &'static str) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}

fn parse_args<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

fn parse_rfc3339(s: &str) -> Result<DateTime<Utc>, String> {
    s.parse::<DateTime<Utc>>()
        .map_err(|e| format!("DateTimeParsed not established: {e}"))
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
#[derive(Debug)]
pub struct ChronoWorkflowPlugin;

impl ElicitPlugin for ChronoWorkflowPlugin {
    fn name(&self) -> &'static str {
        "chrono_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<ParseDateTimeParams>(
                "parse_datetime",
                "Parse an RFC 3339 datetime string and normalize it to UTC. \
                 Establishes: DateTimeParsed. \
                 Returns year, month, day, hour, minute, second, weekday, and Unix timestamp.",
            ),
            typed_tool::<AssertFutureParams>(
                "assert_future",
                "Parse an RFC 3339 datetime and assert it is strictly after the current UTC time. \
                 Establishes: DateTimeParsed ∧ DateTimeFuture. \
                 Useful for validating scheduling inputs before committing a workflow.",
            ),
            typed_tool::<AssertInRangeParams>(
                "assert_in_range",
                "Parse an RFC 3339 datetime and assert it falls within [start, end] (inclusive). \
                 Establishes: DateTimeParsed ∧ DateTimeInRange. \
                 All three inputs must be valid RFC 3339 strings.",
            ),
            typed_tool::<ComputeDurationParams>(
                "compute_duration",
                "Compute the signed duration between two RFC 3339 datetimes. \
                 Establishes: DateTimeParsed(from) ∧ DateTimeParsed(to). \
                 Returns duration in seconds, minutes, hours, and days.",
            ),
            typed_tool::<AddSecondsParams>(
                "add_seconds",
                "Add (or subtract) a number of seconds to an RFC 3339 datetime. \
                 Establishes: DateTimeParsed ⟹ DateTimeParsed(result). \
                 Returns the resulting datetime as RFC 3339.",
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            let bare = params.name.trim_start_matches("chrono_workflow__");
            match bare {
                "parse_datetime" => {
                    let p: ParseDateTimeParams = parse_args(&params)?;
                    let (parsed, _proof) = match UnvalidatedDateStr::new(p.datetime).parse() {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let dt = parsed.inner;
                    use chrono::{Datelike, Timelike};
                    let summary = format!(
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
                    );
                    Ok(CallToolResult::success(vec![Content::text(summary)]))
                }

                "assert_future" => {
                    let p: AssertFutureParams = parse_args(&params)?;
                    let (parsed, parsed_proof) = match UnvalidatedDateStr::new(p.datetime).parse() {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let (future, _proof) = match parsed.assert_future(parsed_proof) {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    Ok(CallToolResult::success(vec![Content::text(format!(
                        "DateTimeParsed ∧ DateTimeFuture established.\n\
                         datetime: {}\n\
                         seconds_from_now: {}",
                        future.inner.to_rfc3339(),
                        (future.inner - Utc::now()).num_seconds(),
                    ))]))
                }

                "assert_in_range" => {
                    let p: AssertInRangeParams = parse_args(&params)?;
                    let start = match parse_rfc3339(&p.start) {
                        Ok(d) => d,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let end = match parse_rfc3339(&p.end) {
                        Ok(d) => d,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let (parsed, parsed_proof) = match UnvalidatedDateStr::new(p.datetime).parse() {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let (ranged, _proof) = match parsed.assert_in_range(start, end, parsed_proof) {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    Ok(CallToolResult::success(vec![Content::text(format!(
                        "DateTimeParsed ∧ DateTimeInRange established.\n\
                         datetime: {}\n\
                         range:    [{}, {}]",
                        ranged.inner.to_rfc3339(),
                        start.to_rfc3339(),
                        end.to_rfc3339(),
                    ))]))
                }

                "compute_duration" => {
                    let p: ComputeDurationParams = parse_args(&params)?;
                    let from = match parse_rfc3339(&p.from) {
                        Ok(d) => d,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let to = match parse_rfc3339(&p.to) {
                        Ok(d) => d,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let dur = to.signed_duration_since(from);
                    let secs = dur.num_seconds();
                    let summary = format!(
                        "DateTimeParsed(from) ∧ DateTimeParsed(to) established.\n\
                         from:    {}\n\
                         to:      {}\n\
                         seconds: {}\n\
                         minutes: {}\n\
                         hours:   {}\n\
                         days:    {}",
                        from.to_rfc3339(),
                        to.to_rfc3339(),
                        secs,
                        dur.num_minutes(),
                        dur.num_hours(),
                        dur.num_days(),
                    );
                    Ok(CallToolResult::success(vec![Content::text(summary)]))
                }

                "add_seconds" => {
                    let p: AddSecondsParams = parse_args(&params)?;
                    let dt = match parse_rfc3339(&p.datetime) {
                        Ok(d) => d,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let result = dt + Duration::seconds(p.seconds);
                    Ok(CallToolResult::success(vec![Content::text(format!(
                        "DateTimeParsed ⟹ DateTimeParsed(result) established.\n\
                         original: {}\n\
                         delta_s:  {}\n\
                         result:   {}",
                        dt.to_rfc3339(),
                        p.seconds,
                        result.to_rfc3339(),
                    ))]))
                }

                other => Ok(CallToolResult::error(vec![Content::text(format!(
                    "Unknown tool: {other}"
                ))])),
            }
        })
    }
}
