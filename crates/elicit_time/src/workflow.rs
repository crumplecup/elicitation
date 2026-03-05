//! `TimeWorkflowPlugin` — contract-verified datetime composition tools using the `time` crate.
//!
//! While the atomic types (`OffsetDateTime`, `PrimitiveDateTime`) wrap `time` for MCP
//! reflection, this plugin provides **phrase-level** tools: parsing, temporal assertions,
//! duration computation, and formatting.
//!
//! # Typestate Design
//!
//! ```text
//! UnvalidatedOffsetStr ──parse()──→ ParsedOffsetDateTime + Established<OffsetDateTimeParsed>
//!                                          │
//!                               assert_future()
//!                                          │
//!                                          ↓
//!                                FutureOffsetDateTime + Established<OffsetDateTimeFuture>
//!
//! UnvalidatedPrimitiveStr ──parse()──→ ParsedPrimitiveDateTime + Established<PrimitiveDateTimeParsed>
//! ```
//!
//! # Propositions and Contracts
//!
//! ```text
//! parse_offset_datetime:    OffsetDateTimeParsed
//! parse_primitive_datetime: PrimitiveDateTimeParsed
//! assert_future:            OffsetDateTimeParsed ∧ OffsetDateTimeFuture
//! compute_duration:         OffsetDateTimeParsed(from) ∧ OffsetDateTimeParsed(to)
//! add_seconds:              OffsetDateTimeParsed ⟹ OffsetDateTimeParsed(result)
//! ```
//!
//! Registered under the `"time_workflow"` namespace.

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
use time::{Duration, OffsetDateTime, PrimitiveDateTime, format_description::well_known::Rfc3339};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: the input string is a valid RFC 3339 `OffsetDateTime`.
pub struct OffsetDateTimeParsed;
impl Prop for OffsetDateTimeParsed {}

/// Proposition: the `OffsetDateTime` is strictly in the future.
pub struct OffsetDateTimeFuture;
impl Prop for OffsetDateTimeFuture {}

/// Proposition: the input string is a valid ISO 8601 local `PrimitiveDateTime`.
pub struct PrimitiveDateTimeParsed;
impl Prop for PrimitiveDateTimeParsed {}

/// Composite: parsed AND in the future.
pub type FutureOffsetProof = And<OffsetDateTimeParsed, OffsetDateTimeFuture>;

// ── Typestate structs ─────────────────────────────────────────────────────────

/// An unvalidated RFC 3339 string — the initial state for `OffsetDateTime`.
pub struct UnvalidatedOffsetStr {
    src: String,
}

/// A successfully parsed `OffsetDateTime`.
pub struct ParsedOffsetDateTime {
    inner: OffsetDateTime,
}

/// A parsed `OffsetDateTime` proven to be strictly in the future.
pub struct FutureOffsetDateTimeState {
    inner: OffsetDateTime,
}

/// An unvalidated ISO 8601 local datetime string.
pub struct UnvalidatedPrimitiveStr {
    src: String,
}

/// A successfully parsed `PrimitiveDateTime`.
pub struct ParsedPrimitiveDateTime {
    inner: PrimitiveDateTime,
}

// ── Typestate transitions ─────────────────────────────────────────────────────

impl UnvalidatedOffsetStr {
    /// Wrap a raw string as an unvalidated RFC 3339 input.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input as RFC 3339, establishing `OffsetDateTimeParsed` proof on success.
    pub fn parse(
        self,
    ) -> Result<(ParsedOffsetDateTime, Established<OffsetDateTimeParsed>), String> {
        OffsetDateTime::parse(&self.src, &Rfc3339)
            .map(|inner| (ParsedOffsetDateTime { inner }, Established::assert()))
            .map_err(|e| format!("OffsetDateTimeParsed not established: {e}"))
    }
}

impl ParsedOffsetDateTime {
    /// Return the inner `OffsetDateTime`.
    pub fn into_inner(self) -> OffsetDateTime {
        self.inner
    }

    /// Assert that this datetime is strictly after `OffsetDateTime::now_utc()`.
    pub fn assert_future(
        self,
        parsed: Established<OffsetDateTimeParsed>,
    ) -> Result<(FutureOffsetDateTimeState, Established<FutureOffsetProof>), String> {
        let now = OffsetDateTime::now_utc();
        if self.inner > now {
            let proof =
                elicitation::contracts::both(parsed, Established::<OffsetDateTimeFuture>::assert());
            Ok((FutureOffsetDateTimeState { inner: self.inner }, proof))
        } else {
            Err(format!(
                "OffsetDateTimeFuture not established: {} is not after now",
                self.inner
                    .format(&Rfc3339)
                    .unwrap_or_else(|_| "(format error)".to_string())
            ))
        }
    }
}

impl UnvalidatedPrimitiveStr {
    /// Wrap a raw string as an unvalidated ISO 8601 local datetime input.
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }

    /// Parse the input as ISO 8601 local datetime, establishing `PrimitiveDateTimeParsed`.
    pub fn parse(
        self,
    ) -> Result<
        (
            ParsedPrimitiveDateTime,
            Established<PrimitiveDateTimeParsed>,
        ),
        String,
    > {
        use time::macros::format_description;
        let fmt = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        PrimitiveDateTime::parse(&self.src, fmt)
            .map(|inner| (ParsedPrimitiveDateTime { inner }, Established::assert()))
            .map_err(|e| format!("PrimitiveDateTimeParsed not established: {e}"))
    }
}

// ── Params structs ────────────────────────────────────────────────────────────

/// Parameters for [`TimeWorkflowPlugin::parse_offset_datetime`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ParseOffsetParams {
    /// RFC 3339 datetime string (e.g. `"2025-03-05T12:00:00Z"`).
    pub datetime: String,
}

/// Parameters for [`TimeWorkflowPlugin::parse_primitive_datetime`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ParsePrimitiveParams {
    /// ISO 8601 local datetime string (e.g. `"2025-03-05T12:00:00"`).
    pub datetime: String,
}

/// Parameters for [`TimeWorkflowPlugin::assert_future`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AssertFutureParams {
    /// RFC 3339 datetime string to check.
    pub datetime: String,
}

/// Parameters for [`TimeWorkflowPlugin::compute_duration`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ComputeDurationParams {
    /// RFC 3339 start datetime. Assumes: valid, before `to`.
    pub from: String,
    /// RFC 3339 end datetime. Assumes: valid, after `from`.
    pub to: String,
}

/// Parameters for [`TimeWorkflowPlugin::add_seconds`].
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

fn parse_rfc3339(s: &str) -> Result<OffsetDateTime, String> {
    OffsetDateTime::parse(s, &Rfc3339)
        .map_err(|e| format!("OffsetDateTimeParsed not established: {e}"))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin exposing contract-verified `time` crate datetime composition tools.
///
/// Register under the `"time_workflow"` namespace:
///
/// ```ignore
/// use elicitation::PluginRegistry;
/// use elicit_time::TimeWorkflowPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("time_workflow", TimeWorkflowPlugin);
/// ```
#[derive(Debug)]
pub struct TimeWorkflowPlugin;

impl ElicitPlugin for TimeWorkflowPlugin {
    fn name(&self) -> &'static str {
        "time_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<ParseOffsetParams>(
                "parse_offset_datetime",
                "Parse an RFC 3339 string as a `time::OffsetDateTime`. \
                 Establishes: OffsetDateTimeParsed. \
                 Returns year, month, day, hour, minute, second, UTC offset, and Unix timestamp.",
            ),
            typed_tool::<ParsePrimitiveParams>(
                "parse_primitive_datetime",
                "Parse an ISO 8601 local datetime string (no timezone) as `time::PrimitiveDateTime`. \
                 Establishes: PrimitiveDateTimeParsed. \
                 Returns year, month, day, hour, minute, second.",
            ),
            typed_tool::<AssertFutureParams>(
                "assert_future",
                "Parse an RFC 3339 datetime and assert it is strictly after the current UTC time. \
                 Establishes: OffsetDateTimeParsed ∧ OffsetDateTimeFuture. \
                 Returns the datetime and seconds-from-now.",
            ),
            typed_tool::<ComputeDurationParams>(
                "compute_duration",
                "Compute the signed duration between two RFC 3339 datetimes. \
                 Establishes: OffsetDateTimeParsed(from) ∧ OffsetDateTimeParsed(to). \
                 Returns duration in seconds, minutes, hours, and days.",
            ),
            typed_tool::<AddSecondsParams>(
                "add_seconds",
                "Add (or subtract) a number of seconds to an RFC 3339 datetime. \
                 Establishes: OffsetDateTimeParsed ⟹ OffsetDateTimeParsed(result). \
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
            let bare = params.name.trim_start_matches("time_workflow__");
            match bare {
                "parse_offset_datetime" => {
                    let p: ParseOffsetParams = parse_args(&params)?;
                    let (parsed, _proof) = match UnvalidatedOffsetStr::new(p.datetime).parse() {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let dt = parsed.inner;
                    let rfc = dt
                        .format(&Rfc3339)
                        .unwrap_or_else(|_| "(format error)".to_string());
                    let offset = dt.offset();
                    let summary = format!(
                        "OffsetDateTimeParsed established.\n\
                         rfc3339:   {rfc}\n\
                         year:      {}\n\
                         month:     {}\n\
                         day:       {}\n\
                         hour:      {}\n\
                         minute:    {}\n\
                         second:    {}\n\
                         utc_offset: {}h {}m\n\
                         unix_timestamp: {}",
                        dt.year(),
                        dt.month() as u8,
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second(),
                        offset.whole_hours(),
                        offset.minutes_past_hour(),
                        dt.unix_timestamp(),
                    );
                    Ok(CallToolResult::success(vec![Content::text(summary)]))
                }

                "parse_primitive_datetime" => {
                    let p: ParsePrimitiveParams = parse_args(&params)?;
                    let (parsed, _proof) = match UnvalidatedPrimitiveStr::new(p.datetime).parse() {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let dt = parsed.inner;
                    let summary = format!(
                        "PrimitiveDateTimeParsed established.\n\
                         year:   {}\n\
                         month:  {}\n\
                         day:    {}\n\
                         hour:   {}\n\
                         minute: {}\n\
                         second: {}",
                        dt.year(),
                        dt.month() as u8,
                        dt.day(),
                        dt.hour(),
                        dt.minute(),
                        dt.second(),
                    );
                    Ok(CallToolResult::success(vec![Content::text(summary)]))
                }

                "assert_future" => {
                    let p: AssertFutureParams = parse_args(&params)?;
                    let (parsed, parsed_proof) = match UnvalidatedOffsetStr::new(p.datetime).parse()
                    {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let (future, _proof) = match parsed.assert_future(parsed_proof) {
                        Ok(r) => r,
                        Err(e) => return Ok(CallToolResult::error(vec![Content::text(e)])),
                    };
                    let now = OffsetDateTime::now_utc();
                    let diff = (future.inner - now).whole_seconds();
                    let rfc = future
                        .inner
                        .format(&Rfc3339)
                        .unwrap_or_else(|_| "(format error)".to_string());
                    Ok(CallToolResult::success(vec![Content::text(format!(
                        "OffsetDateTimeParsed ∧ OffsetDateTimeFuture established.\n\
                         datetime:         {rfc}\n\
                         seconds_from_now: {diff}"
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
                    let dur = to - from;
                    let secs = dur.whole_seconds();
                    let summary = format!(
                        "OffsetDateTimeParsed(from) ∧ OffsetDateTimeParsed(to) established.\n\
                         from:    {}\n\
                         to:      {}\n\
                         seconds: {secs}\n\
                         minutes: {}\n\
                         hours:   {}\n\
                         days:    {}",
                        from.format(&Rfc3339)
                            .unwrap_or_else(|_| "(format error)".to_string()),
                        to.format(&Rfc3339)
                            .unwrap_or_else(|_| "(format error)".to_string()),
                        dur.whole_minutes(),
                        dur.whole_hours(),
                        dur.whole_days(),
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
                        "OffsetDateTimeParsed ⟹ OffsetDateTimeParsed(result) established.\n\
                         original: {}\n\
                         delta_s:  {}\n\
                         result:   {}",
                        dt.format(&Rfc3339)
                            .unwrap_or_else(|_| "(format error)".to_string()),
                        p.seconds,
                        result
                            .format(&Rfc3339)
                            .unwrap_or_else(|_| "(format error)".to_string()),
                    ))]))
                }

                other => Ok(CallToolResult::error(vec![Content::text(format!(
                    "Unknown tool: {other}"
                ))])),
            }
        })
    }
}
