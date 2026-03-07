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

use elicitation::ElicitPlugin;
use elicitation::contracts::{And, Established, Prop};
use futures::future::BoxFuture;
use jiff::Timestamp;
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
    inner: Timestamp,
}

/// A parsed timestamp proven to be strictly in the future.
pub struct FutureTimestampState {
    inner: Timestamp,
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
    inner: jiff::Zoned,
}

/// A zoned datetime successfully converted to a new timezone.
pub struct ConvertedZonedState {
    inner: jiff::Zoned,
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

fn typed_tool<T: JsonSchema + 'static>(name: &'static str, description: &'static str) -> Tool {
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}

fn parse_args<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

fn parse_ts(s: &str) -> Result<Timestamp, String> {
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
#[derive(Debug)]
pub struct JiffWorkflowPlugin;

impl ElicitPlugin for JiffWorkflowPlugin {
    fn name(&self) -> &'static str {
        "jiff_workflow"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<ParseTimestampParams>(
                "parse_timestamp",
                "Parse an ISO 8601 timestamp string using jiff. \
                 Establishes: TimestampParsed. \
                 Returns seconds, milliseconds, nanoseconds, and human-readable form.",
            ),
            typed_tool::<ParseZonedParams>(
                "parse_zoned",
                "Parse a jiff zoned datetime string (e.g. '2025-03-05T12:00:00[America/New_York]'). \
                 Establishes: ZonedParsed. \
                 Returns year/month/day/hour/minute/second and timezone name.",
            ),
            typed_tool::<AssertFutureParams>(
                "assert_future",
                "Parse an ISO 8601 timestamp and assert it is strictly after now. \
                 Establishes: TimestampParsed ∧ TimestampFuture. \
                 Returns the timestamp and seconds-from-now.",
            ),
            typed_tool::<ConvertTzParams>(
                "convert_tz",
                "Parse a jiff zoned datetime and convert it to the named IANA timezone. \
                 Establishes: ZonedParsed ∧ TimezoneConverted. \
                 The resulting datetime preserves the same instant in time.",
            ),
            typed_tool::<ComputeSpanParams>(
                "compute_span",
                "Compute the signed duration between two ISO 8601 timestamps. \
                 Establishes: TimestampParsed(from) ∧ TimestampParsed(to). \
                 Returns span in seconds, minutes, hours, and days.",
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
            let bare = params.name.trim_start_matches("jiff_workflow__");
            match bare {
                "parse_timestamp" => {
                    let p: ParseTimestampParams = parse_args(&params)?;
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

                "parse_zoned" => {
                    let p: ParseZonedParams = parse_args(&params)?;
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

                "assert_future" => {
                    let p: AssertFutureParams = parse_args(&params)?;
                    let (parsed, parsed_proof) =
                        match UnvalidatedTimestampStr::new(p.timestamp).parse() {
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

                "convert_tz" => {
                    let p: ConvertTzParams = parse_args(&params)?;
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

                "compute_span" => {
                    let p: ComputeSpanParams = parse_args(&params)?;
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

                other => Ok(CallToolResult::error(vec![Content::text(format!(
                    "Unknown tool: {other}"
                ))])),
            }
        })
    }
}

// ── EmitCode ──────────────────────────────────────────────────────────────────

#[cfg(feature = "emit")]
use elicitation::emit_code::{CrateDep, EmitCode};
#[cfg(feature = "emit")]
use elicitation::proc_macro2::TokenStream;

#[cfg(feature = "emit")]
const ELICIT_JIFF_DEP: CrateDep = CrateDep::new("elicit_jiff", "0.8");
#[cfg(feature = "emit")]
const ELICITATION_DEP_J: CrateDep = CrateDep::new("elicitation", "0.8");
#[cfg(feature = "emit")]
const JIFF_DEP: CrateDep = CrateDep::new("jiff", "0.2");

/// `parse_timestamp` → `UnvalidatedTimestampStr::new → .parse()`
#[cfg(feature = "emit")]
impl EmitCode for ParseTimestampParams {
    fn emit_code(&self) -> TokenStream {
        let ts = &self.timestamp;
        quote::quote! {
            let (_ts, _ts_proof) = elicit_jiff::UnvalidatedTimestampStr::new(#ts.to_string())
                .parse()
                .map_err(|e| format!("Timestamp parse failed: {}", e))?;
            let _inner = _ts.into_inner();
            println!("TimestampParsed: {} ({}s)", _inner, _inner.as_second());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_J, ELICIT_JIFF_DEP]
    }
}

/// `parse_zoned` → `UnvalidatedZonedStr::new → .parse()`
#[cfg(feature = "emit")]
impl EmitCode for ParseZonedParams {
    fn emit_code(&self) -> TokenStream {
        let z = &self.zoned;
        quote::quote! {
            let (_zoned, _zoned_proof) = elicit_jiff::UnvalidatedZonedStr::new(#z.to_string())
                .parse()
                .map_err(|e| format!("Zoned parse failed: {}", e))?;
            let _inner = _zoned.into_inner();
            println!("ZonedParsed: {}", _inner);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_J, ELICIT_JIFF_DEP]
    }
}

/// `assert_future` → `UnvalidatedTimestampStr → ParsedTimestamp → FutureTimestampState`
#[cfg(feature = "emit")]
impl EmitCode for AssertFutureParams {
    fn emit_code(&self) -> TokenStream {
        let ts = &self.timestamp;
        quote::quote! {
            let (_ts, _ts_proof) = elicit_jiff::UnvalidatedTimestampStr::new(#ts.to_string())
                .parse()
                .map_err(|e| format!("Timestamp parse failed: {}", e))?;
            let (_future, _future_proof) = _ts.assert_future(_ts_proof)
                .map_err(|e| format!("TimestampFuture not established: {}", e))?;
            println!("TimestampParsed \u{2227} TimestampFuture: {}", _future.into_inner());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_J, ELICIT_JIFF_DEP]
    }
}

/// `convert_tz` → `UnvalidatedZonedStr → ParsedZoned → ConvertedZonedState`
#[cfg(feature = "emit")]
impl EmitCode for ConvertTzParams {
    fn emit_code(&self) -> TokenStream {
        let z = &self.zoned;
        let tz = &self.timezone;
        quote::quote! {
            let (_zoned, _zoned_proof) = elicit_jiff::UnvalidatedZonedStr::new(#z.to_string())
                .parse()
                .map_err(|e| format!("Zoned parse failed: {}", e))?;
            let (_converted, _tz_proof) = _zoned.convert_tz(#tz, _zoned_proof)
                .map_err(|e| format!("TimezoneConverted not established: {}", e))?;
            println!("ZonedParsed \u{2227} TimezoneConverted: {}", _converted.into_inner());
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_J, ELICIT_JIFF_DEP]
    }
}

/// `compute_span` → parse two timestamps and compute span
#[cfg(feature = "emit")]
impl EmitCode for ComputeSpanParams {
    fn emit_code(&self) -> TokenStream {
        let from = &self.from;
        let to = &self.to;
        quote::quote! {
            let _from: jiff::Timestamp = #from.parse()
                .map_err(|e| format!("From parse failed: {}", e))?;
            let _to: jiff::Timestamp = #to.parse()
                .map_err(|e| format!("To parse failed: {}", e))?;
            let _secs = _to.as_second() - _from.as_second();
            println!("Span: {}s / {}m / {}h / {}d",
                _secs, _secs / 60, _secs / 3600, _secs / 86400);
        }
    }
    fn crate_deps(&self) -> Vec<CrateDep> {
        vec![ELICITATION_DEP_J, ELICIT_JIFF_DEP, JIFF_DEP]
    }
}

// ── dispatch_emit ─────────────────────────────────────────────────────────────

/// Deserialize a jiff_workflow tool's params from JSON and return its [`EmitCode`] impl.
#[cfg(feature = "emit")]
pub fn dispatch_emit(
    tool_name: &str,
    params: serde_json::Value,
) -> Result<Box<dyn EmitCode>, String> {
    match tool_name {
        "parse_timestamp" => serde_json::from_value::<ParseTimestampParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "parse_zoned" => serde_json::from_value::<ParseZonedParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "assert_future" => serde_json::from_value::<AssertFutureParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "convert_tz" => serde_json::from_value::<ConvertTzParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        "compute_span" => serde_json::from_value::<ComputeSpanParams>(params)
            .map(|p| Box::new(p) as Box<dyn EmitCode>)
            .map_err(|e| format!("{e}")),
        other => Err(format!("Unknown jiff_workflow tool: '{other}'")),
    }
}
