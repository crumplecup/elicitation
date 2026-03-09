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

use elicitation::contracts::{And, Established, Prop};
use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
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

impl FutureOffsetDateTimeState {
    /// Return the inner `OffsetDateTime`.
    pub fn into_inner(self) -> OffsetDateTime {
        self.inner
    }
}

/// An unvalidated ISO 8601 local datetime string.
pub struct UnvalidatedPrimitiveStr {
    src: String,
}

/// A successfully parsed `PrimitiveDateTime`.
pub struct ParsedPrimitiveDateTime {
    inner: PrimitiveDateTime,
}

impl ParsedPrimitiveDateTime {
    /// Return the inner `PrimitiveDateTime`.
    pub fn into_inner(self) -> PrimitiveDateTime {
        self.inner
    }
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
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "time_workflow")]
pub struct TimeWorkflowPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "time_workflow",
    name = "parse_offset_datetime",
    description = "Parse an RFC 3339 string as a `time::OffsetDateTime`. \
                   Establishes: OffsetDateTimeParsed. \
                   Returns year, month, day, hour, minute, second, UTC offset, and Unix timestamp."
)]
#[instrument(skip_all)]
async fn parse_offset(p: ParseOffsetParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "time_workflow",
    name = "parse_primitive_datetime",
    description = "Parse an ISO 8601 local datetime string (no timezone) as `time::PrimitiveDateTime`. \
                   Establishes: PrimitiveDateTimeParsed. \
                   Returns year, month, day, hour, minute, second."
)]
#[instrument(skip_all)]
async fn parse_primitive(p: ParsePrimitiveParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "time_workflow",
    name = "assert_future",
    description = "Parse an RFC 3339 datetime and assert it is strictly after the current UTC time. \
                   Establishes: OffsetDateTimeParsed ∧ OffsetDateTimeFuture. \
                   Returns the datetime and seconds-from-now."
)]
#[instrument(skip_all)]
async fn assert_future(p: AssertFutureParams) -> Result<CallToolResult, ErrorData> {
    let (parsed, parsed_proof) = match UnvalidatedOffsetStr::new(p.datetime).parse() {
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

#[elicit_tool(
    plugin = "time_workflow",
    name = "compute_duration",
    description = "Compute the signed duration between two RFC 3339 datetimes. \
                   Establishes: OffsetDateTimeParsed(from) ∧ OffsetDateTimeParsed(to). \
                   Returns duration in seconds, minutes, hours, and days."
)]
#[instrument(skip_all)]
async fn compute_duration(p: ComputeDurationParams) -> Result<CallToolResult, ErrorData> {
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

#[elicit_tool(
    plugin = "time_workflow",
    name = "add_seconds",
    description = "Add (or subtract) a number of seconds to an RFC 3339 datetime. \
                   Establishes: OffsetDateTimeParsed ⟹ OffsetDateTimeParsed(result). \
                   Returns the resulting datetime as RFC 3339."
)]
#[instrument(skip_all)]
async fn add_seconds(p: AddSecondsParams) -> Result<CallToolResult, ErrorData> {
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

// ── EmitCode ──────────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
use elicitation::emit_code::{CrateDep, EmitCode};
#[cfg(feature = "emit")]
use proc_macro2::TokenStream;

#[cfg(feature = "emit")]
const ELICIT_TIME_DEP: CrateDep = CrateDep::new("elicit_time", "0.8");
#[cfg(feature = "emit")]
const ELICITATION_DEP_T: CrateDep = CrateDep::new("elicitation", "0.8");
#[cfg(feature = "emit")]
const TIME_DEP: CrateDep = CrateDep::new("time", "0.3");

/// `parse_offset_datetime` → `UnvalidatedOffsetStr::new → .parse()`
#[cfg(feature = "emit")]
impl EmitCode for ParseOffsetParams {
    fn emit_code(&self) -> TokenStream {
        let dt = &self.datetime;
        quote::quote! {
            let (_dt, _dt_proof) = elicit_time::UnvalidatedOffsetStr::new(#dt.to_string())
                .parse()
                .map_err(|e| format!("OffsetDateTime parse failed: {}", e))?;
            let _inner = _dt.into_inner();
            println!("OffsetDateTimeParsed: {}", _inner.unix_timestamp());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_T, ELICIT_TIME_DEP]
    }
}

/// `parse_primitive_datetime` → `UnvalidatedPrimitiveStr::new → .parse()`
#[cfg(feature = "emit")]
impl EmitCode for ParsePrimitiveParams {
    fn emit_code(&self) -> TokenStream {
        let dt = &self.datetime;
        quote::quote! {
            let (_dt, _dt_proof) = elicit_time::UnvalidatedPrimitiveStr::new(#dt.to_string())
                .parse()
                .map_err(|e| format!("PrimitiveDateTime parse failed: {}", e))?;
            let _inner = _dt.into_inner();
            println!("PrimitiveDateTimeParsed: {}-{}-{}", _inner.year(), _inner.month() as u8, _inner.day());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_T, ELICIT_TIME_DEP]
    }
}

/// `assert_future` → `UnvalidatedOffsetStr → ParsedOffsetDateTime → FutureOffsetDateTimeState`
#[cfg(feature = "emit")]
impl EmitCode for AssertFutureParams {
    fn emit_code(&self) -> TokenStream {
        let dt = &self.datetime;
        quote::quote! {
            let (_dt, _dt_proof) = elicit_time::UnvalidatedOffsetStr::new(#dt.to_string())
                .parse()
                .map_err(|e| format!("OffsetDateTime parse failed: {}", e))?;
            let (_future, _future_proof) = _dt.assert_future(_dt_proof)
                .map_err(|e| format!("OffsetDateTimeFuture not established: {}", e))?;
            println!("OffsetDateTimeParsed \u{2227} OffsetDateTimeFuture: {}",
                _future.into_inner().unix_timestamp());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_T, ELICIT_TIME_DEP]
    }
}

/// `compute_duration` → parse two OffsetDateTimes and compute signed duration
#[cfg(feature = "emit")]
impl EmitCode for ComputeDurationParams {
    fn emit_code(&self) -> TokenStream {
        let from = &self.from;
        let to = &self.to;
        quote::quote! {
            use time::format_description::well_known::Rfc3339;
            let _from = time::OffsetDateTime::parse(#from, &Rfc3339)
                .map_err(|e| format!("From parse failed: {}", e))?;
            let _to = time::OffsetDateTime::parse(#to, &Rfc3339)
                .map_err(|e| format!("To parse failed: {}", e))?;
            let _dur = _to - _from;
            println!("Duration: {}s / {}m / {}h / {}d",
                _dur.whole_seconds(), _dur.whole_minutes(), _dur.whole_hours(), _dur.whole_days());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_T, ELICIT_TIME_DEP, TIME_DEP]
    }
}

/// `add_seconds` → parse OffsetDateTime and add/subtract seconds
#[cfg(feature = "emit")]
impl EmitCode for AddSecondsParams {
    fn emit_code(&self) -> TokenStream {
        let dt = &self.datetime;
        let secs = self.seconds;
        quote::quote! {
            use time::format_description::well_known::Rfc3339;
            let _dt = time::OffsetDateTime::parse(#dt, &Rfc3339)
                .map_err(|e| format!("DateTime parse failed: {}", e))?;
            let _result = _dt + time::Duration::seconds(#secs);
            println!("Result: {}", _result.format(&Rfc3339).unwrap_or_else(|_| "(err)".to_string()));
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_T, ELICIT_TIME_DEP, TIME_DEP]
    }
}

// ── dispatch_emit ─────────────────────────────────────────────────────────────

/// Deserialize a time_workflow tool's params from JSON and return its [`EmitCode`] impl.
#[cfg(feature = "emit")]
pub fn dispatch_emit(
    tool_name: &str,
    params: serde_json::Value,
) -> Result<Box<dyn EmitCode>, String> {
    match tool_name {
        "parse_offset_datetime" => serde_json::from_value::<ParseOffsetParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "parse_primitive_datetime" => serde_json::from_value::<ParsePrimitiveParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "assert_future" => serde_json::from_value::<AssertFutureParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "compute_duration" => serde_json::from_value::<ComputeDurationParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "add_seconds" => serde_json::from_value::<AddSecondsParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        other => Err(format!("Unknown time_workflow tool: '{other}'")),
    }
}

// ── Global emit registry ──────────────────────────────────────────────────────

#[cfg(feature = "emit")]
elicitation::register_emit!("parse_offset_datetime", ParseOffsetParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("parse_primitive_datetime", ParsePrimitiveParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("assert_future", AssertFutureParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("compute_duration", ComputeDurationParams);
#[cfg(feature = "emit")]
elicitation::register_emit!("add_seconds", AddSecondsParams);
