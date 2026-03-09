//! `JiffWorkflowPlugin` — contract-verified datetime composition tools using jiff.
//!
//! While the atomic types (`Zoned`, `Timestamp`) wrap jiff for MCP reflection,
//! this plugin provides **phrase-level** tools: parsing, temporal assertions,
//! timezone conversion, and span computation.
//!
//! # Typestate Design
//!
//! ```text
//! UnvalidatedTimestampStr ──parse()──→ ParsedTimestamp + Established<TimestampParsed>
//!                                            │
//!                                  assert_future()
//!                                            │
//!                                            ↓
//!                                FutureTimestamp + Established<TimestampFuture>
//!
//! UnvalidatedZonedStr ──parse()──→ ParsedZoned + Established<ZonedParsed>
//!                                       │
//!                             convert_tz(name)
//!                                       │
//!                                       ↓
//!                             ConvertedZoned + Established<TimezoneConverted>
//! ```
//!
//! # Propositions and Contracts
//!
//! ```text
//! parse_timestamp:  TimestampParsed
//! parse_zoned:      ZonedParsed
//! assert_future:    TimestampParsed ∧ TimestampFuture
//! convert_tz:       ZonedParsed ∧ TimezoneConverted
//! compute_span:     TimestampParsed(from) ∧ TimestampParsed(to)
//! ```
//!
//! Registered under the `"jiff_workflow"` namespace.

use elicitation::contracts::{And, Established, Prop};
use elicitation::{ElicitPlugin, elicit_tool};
use jiff::Timestamp;
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the input string is a valid jiff `Timestamp`.
pub struct TimestampParsed;
impl Prop for TimestampParsed {}

/// Proposition: the timestamp is strictly in the future (after `Timestamp::now()`).
pub struct TimestampFuture;
impl Prop for TimestampFuture {}

/// Proposition: the input string is a valid jiff `Zoned` datetime.
pub struct ZonedParsed;
impl Prop for ZonedParsed {}

/// Proposition: a timezone conversion was successfully applied to a `Zoned` value.
pub struct TimezoneConverted;
impl Prop for TimezoneConverted {}

/// Composite: parsed AND in the future.
pub type FutureTimestampProof = And<TimestampParsed, TimestampFuture>;

/// Composite: zoned parsed AND converted to the target timezone.
pub type ConvertedZonedProof = And<ZonedParsed, TimezoneConverted>;

// ── Typestate structs ─────────────────────────────────────────────────────────

/// An unvalidated timestamp string — the initial state.
pub struct UnvalidatedTimestampStr {
    src: String,
}

/// A successfully parsed jiff `Timestamp`.
pub struct ParsedTimestamp {
    /// The inner value carried by this typestate wrapper.
    pub inner: Timestamp,
}

/// A parsed timestamp proven to be strictly in the future.
pub struct FutureTimestampState {
    /// The inner value carried by this typestate wrapper.
    pub inner: Timestamp,
}

impl FutureTimestampState {
    /// Return the inner jiff `Timestamp`.
    pub fn into_inner(self) -> Timestamp {
        self.inner
    }
}

/// An unvalidated zoned datetime string — the initial state.
pub struct UnvalidatedZonedStr {
    src: String,
}

/// A successfully parsed jiff `Zoned` datetime.
pub struct ParsedZoned {
    /// The inner value carried by this typestate wrapper.
    pub inner: jiff::Zoned,
}

/// A zoned datetime successfully converted to a new timezone.
pub struct ConvertedZonedState {
    /// The inner value carried by this typestate wrapper.
    pub inner: jiff::Zoned,
}

impl ConvertedZonedState {
    /// Return the inner jiff `Zoned` datetime.
    pub fn into_inner(self) -> jiff::Zoned {
        self.inner
    }
}

// ── Typestate transitions ─────────────────────────────────────────────────────

impl UnvalidatedTimestampStr {
    /// Wrap a raw string as an unvalidated timestamp input.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input, establishing `TimestampParsed` proof on success.
    pub fn parse(self) -> Result<(ParsedTimestamp, Established<TimestampParsed>), String> {
        self.src
            .parse::<Timestamp>()
            .map(|inner| (ParsedTimestamp { inner }, Established::assert()))
            .map_err(|e| format!("TimestampParsed not established: {e}"))
    }
}

impl ParsedTimestamp {
    /// Return the inner jiff `Timestamp`.
    pub fn into_inner(self) -> Timestamp {
        self.inner
    }

    /// Assert that this timestamp is strictly after `Timestamp::now()`.
    pub fn assert_future(
        self,
        parsed: Established<TimestampParsed>,
    ) -> Result<(FutureTimestampState, Established<FutureTimestampProof>), String> {
        let now = Timestamp::now();
        if self.inner > now {
            let proof =
                elicitation::contracts::both(parsed, Established::<TimestampFuture>::assert());
            Ok((FutureTimestampState { inner: self.inner }, proof))
        } else {
            Err(format!(
                "TimestampFuture not established: {} is not after now ({})",
                self.inner, now
            ))
        }
    }
}

impl UnvalidatedZonedStr {
    /// Wrap a raw string as an unvalidated zoned datetime input.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input, establishing `ZonedParsed` proof on success.
    pub fn parse(self) -> Result<(ParsedZoned, Established<ZonedParsed>), String> {
        self.src
            .parse::<jiff::Zoned>()
            .map(|inner| (ParsedZoned { inner }, Established::assert()))
            .map_err(|e| format!("ZonedParsed not established: {e}"))
    }
}

impl ParsedZoned {
    /// Return the inner jiff `Zoned` datetime.
    pub fn into_inner(self) -> jiff::Zoned {
        self.inner
    }

    /// Convert to the named IANA timezone, establishing `TimezoneConverted`.
    pub fn convert_tz(
        self,
        tz_name: &str,
        parsed: Established<ZonedParsed>,
    ) -> Result<(ConvertedZonedState, Established<ConvertedZonedProof>), String> {
        match self.inner.in_tz(tz_name) {
            Ok(converted) => {
                let proof = elicitation::contracts::both(
                    parsed,
                    Established::<TimezoneConverted>::assert(),
                );
                Ok((ConvertedZonedState { inner: converted }, proof))
            }
            Err(e) => Err(format!("TimezoneConverted not established: {e}")),
        }
    }
}

// ── Params structs ────────────────────────────────────────────────────────────

/// Parameters for [`JiffWorkflowPlugin::parse_timestamp`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ParseTimestampParams {
    /// ISO 8601 timestamp string (e.g. `"2025-03-05T12:00:00Z"`).
    pub timestamp: String,
}

/// Parameters for [`JiffWorkflowPlugin::parse_zoned`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ParseZonedParams {
    /// Zoned datetime string (e.g. `"2025-03-05T12:00:00[America/New_York]"`).
    pub zoned: String,
}

/// Parameters for [`JiffWorkflowPlugin::assert_future`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AssertFutureParams {
    /// ISO 8601 timestamp string to check.
    pub timestamp: String,
}

/// Parameters for [`JiffWorkflowPlugin::convert_tz`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ConvertTzParams {
    /// Zoned datetime string. Assumes: valid jiff Zoned string.
    pub zoned: String,
    /// Target IANA timezone name (e.g. `"America/New_York"`, `"Europe/London"`).
    pub timezone: String,
}

/// Parameters for [`JiffWorkflowPlugin::compute_span`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ComputeSpanParams {
    /// ISO 8601 start timestamp. Assumes: valid, before `to`.
    pub from: String,
    /// ISO 8601 end timestamp. Assumes: valid, after `from`.
    pub to: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a datetime string. Returns an error string on failure.
pub fn parse_ts(s: &str) -> Result<Timestamp, String> {
    s.parse::<Timestamp>()
        .map_err(|e| format!("TimestampParsed not established: {e}"))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing contract-verified jiff datetime composition tools.
///
/// Register under the `"jiff_workflow"` namespace:
///
/// ```ignore
/// use elicitation::PluginRegistry;
/// use elicit_jiff::JiffWorkflowPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("jiff_workflow", JiffWorkflowPlugin);
/// ```
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "jiff_workflow")]
pub struct JiffWorkflowPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "jiff_workflow",
    name = "parse_timestamp",
    description = "Parse an ISO 8601 timestamp string using jiff. \
                   Establishes: TimestampParsed. \
                   Returns seconds, milliseconds, nanoseconds, and human-readable form."
)]
#[instrument(skip_all)]
async fn parse_timestamp(p: ParseTimestampParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, _proof) = match UnvalidatedTimestampStr::new(p.timestamp).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let ts = parsed.inner;
    let summary = format!(
        "TimestampParsed established.\n\
         timestamp:    {}\n\
         as_second:    {}\n\
         as_millis:    {}\n\
         subsec_nanos: {}",
        ts,
        ts.as_second(),
        ts.as_millisecond(),
        ts.subsec_nanosecond(),
    );
    Ok(CallToolResult::success(vec![Content::text(summary)]))
}

#[elicit_tool(
    plugin = "jiff_workflow",
    name = "parse_zoned",
    description = "Parse a jiff zoned datetime string (e.g. '2025-03-05T12:00:00[America/New_York]'). \
                   Establishes: ZonedParsed. \
                   Returns year/month/day/hour/minute/second and timezone name."
)]
#[instrument(skip_all)]
async fn parse_zoned(p: ParseZonedParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, _proof) = match UnvalidatedZonedStr::new(p.zoned).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let z = parsed.inner;
    let summary = format!(
        "ZonedParsed established.\n\
         zoned:    {}\n\
         year:     {}\n\
         month:    {}\n\
         day:      {}\n\
         hour:     {}\n\
         minute:   {}\n\
         second:   {}\n\
         timezone: {}",
        z,
        z.year(),
        z.month(),
        z.day(),
        z.hour(),
        z.minute(),
        z.second(),
        z.time_zone().iana_name().unwrap_or("(fixed offset)"),
    );
    Ok(CallToolResult::success(vec![Content::text(summary)]))
}

#[elicit_tool(
    plugin = "jiff_workflow",
    name = "assert_future",
    description = "Parse an ISO 8601 timestamp and assert it is strictly after now. \
                   Establishes: TimestampParsed ∧ TimestampFuture. \
                   Returns the timestamp and seconds-from-now."
)]
#[instrument(skip_all)]
async fn assert_future(p: AssertFutureParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, parsed_proof) = match UnvalidatedTimestampStr::new(p.timestamp).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (future, _proof) = match parsed.assert_future(parsed_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let now = Timestamp::now();
    let diff_secs = (future.inner.as_second()) - (now.as_second());
    Ok(CallToolResult::success(vec![Content::text(format!(
        "TimestampParsed ∧ TimestampFuture established.\n\
         timestamp:       {}\n\
         seconds_from_now: {}",
        future.inner, diff_secs,
    ))]))
}

#[elicit_tool(
    plugin = "jiff_workflow",
    name = "convert_tz",
    description = "Parse a jiff zoned datetime and convert it to the named IANA timezone. \
                   Establishes: ZonedParsed ∧ TimezoneConverted. \
                   The resulting datetime preserves the same instant in time."
)]
#[instrument(skip_all)]
async fn convert_tz(p: ConvertTzParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, parsed_proof) = match UnvalidatedZonedStr::new(p.zoned).parse() {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let (converted, _proof) = match parsed.convert_tz(&p.timezone, parsed_proof) {
        Ok(r) => r,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    Ok(CallToolResult::success(vec![Content::text(format!(
        "ZonedParsed ∧ TimezoneConverted established.\n\
         result:   {}\n\
         timezone: {}",
        converted.inner,
        converted
            .inner
            .time_zone()
            .iana_name()
            .unwrap_or("(fixed offset)"),
    ))]))
}

#[elicit_tool(
    plugin = "jiff_workflow",
    name = "compute_span",
    description = "Compute the signed duration between two ISO 8601 timestamps. \
                   Establishes: TimestampParsed(from) ∧ TimestampParsed(to). \
                   Returns span in seconds, minutes, hours, and days."
)]
#[instrument(skip_all)]
async fn compute_span(p: ComputeSpanParams) -> Result<CallToolResult, ErrorData> {
    let from = match parse_ts(&p.from) {
        Ok(t) => t,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let to = match parse_ts(&p.to) {
        Ok(t) => t,
        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
    };
    let secs = to.as_second() - from.as_second();
    let summary = format!(
        "TimestampParsed(from) ∧ TimestampParsed(to) established.\n\
         from:    {}\n\
         to:      {}\n\
         seconds: {}\n\
         minutes: {}\n\
         hours:   {}\n\
         days:    {}",
        from,
        to,
        secs,
        secs / 60,
        secs / 3600,
        secs / 86400,
    );
    Ok(CallToolResult::success(vec![Content::text(summary)]))
}
