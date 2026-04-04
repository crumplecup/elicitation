//! `TokioTimePlugin` — MCP tools for async time management.
//!
//! All tokio time objects are held server-side in UUID-keyed registries.
//! Agents interact via serializable handles; no futures or runtime objects
//! cross the MCP boundary.
//!
//! # Tool namespace: `tokio_time__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `sleep` | `duration_ms: u64` | `{ elapsed_ms }` | `SleepCompleted` |
//! | `sleep_until` | `deadline_unix_ms: u64` | `{ elapsed_ms }` | `SleepCompleted` |
//! | `timeout_create` | `duration_ms: u64` | `{ timeout_id }` | — |
//! | `timeout_check` | `timeout_id: Uuid` | `{ elapsed_ms, remaining_ms, expired }` | — |
//! | `timeout_await` | `timeout_id: Uuid` | `{ elapsed_ms, expired }` | `TimeoutResolved` |
//! | `interval_create` | `period_ms: u64` | `{ interval_id }` | — |
//! | `interval_tick` | `interval_id: Uuid` | `{ tick_number, elapsed_ms }` | — |

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use elicitation::contracts::{Established, Prop};
use elicitation::{Elicit, PluginContext, VerifiedWorkflow};
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tokio::time::{Instant, Interval, interval, sleep_until};
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a `sleep` or `sleep_until` completed without interruption.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct SleepCompleted;
impl Prop for SleepCompleted {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_sleep_completed_axiom() {
                let duration_ms: u64 = kani::any();
                let sleep_returned_ok: bool = kani::any();
                kani::assume(sleep_returned_ok);
                assert!(sleep_returned_ok, "tokio::time::sleep axiom: returns when duration elapsed");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_sleep_completed(sleep_returned_ok: bool) -> (result: bool)
                ensures result == sleep_returned_ok,
            {
                sleep_returned_ok
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_sleep_completed_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for SleepCompleted {}

/// Proposition: a `timeout_await` call returned (either expired or checked).
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TimeoutResolved;
impl Prop for TimeoutResolved {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_timeout_resolved_axiom() {
                let timed_out: bool = kani::any();
                let resolved = true;
                assert!(resolved, "tokio::time::timeout axiom: always resolves to Ok or Elapsed");
                let _ = timed_out;
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_timeout_resolved(timeout_resolved: bool) -> (result: bool)
                ensures result == timeout_resolved,
            {
                timeout_resolved
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_timeout_resolved_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for TimeoutResolved {}

// ── Internal context entries ──────────────────────────────────────────────────

struct DeadlineEntry {
    created: Instant,
    deadline: Instant,
}

struct IntervalEntry {
    started: Instant,
    tick_count: u64,
    inner: Arc<Mutex<Interval>>,
}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_time__*` tool calls.
pub struct TokioTimeCtx {
    deadlines: Mutex<HashMap<Uuid, DeadlineEntry>>,
    intervals: Mutex<HashMap<Uuid, IntervalEntry>>,
}

impl TokioTimeCtx {
    fn new() -> Self {
        Self {
            deadlines: Mutex::new(HashMap::new()),
            intervals: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TokioTimeCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tokio_time__sleep`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SleepParams {
    /// Number of milliseconds to sleep.
    pub duration_ms: u64,
}

/// Parameters for `tokio_time__sleep_until`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SleepUntilParams {
    /// Absolute deadline as milliseconds since the Unix epoch (UTC).
    /// If the deadline is already in the past, returns immediately.
    pub deadline_unix_ms: u64,
}

/// Parameters for `tokio_time__timeout_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeoutCreateParams {
    /// Timeout duration in milliseconds.
    pub duration_ms: u64,
}

/// Parameters for `tokio_time__timeout_check`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeoutCheckParams {
    /// UUID returned by `tokio_time__timeout_create`.
    pub timeout_id: Uuid,
}

/// Parameters for `tokio_time__timeout_await`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeoutAwaitParams {
    /// UUID returned by `tokio_time__timeout_create`.
    pub timeout_id: Uuid,
}

/// Parameters for `tokio_time__interval_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IntervalCreateParams {
    /// Interval period in milliseconds.
    pub period_ms: u64,
}

/// Parameters for `tokio_time__interval_tick`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IntervalTickParams {
    /// UUID returned by `tokio_time__interval_create`.
    pub interval_id: Uuid,
}

#[derive(Serialize)]
struct SleepResult {
    elapsed_ms: u64,
}

#[derive(Serialize)]
struct TimeoutCreateResult {
    timeout_id: Uuid,
}

#[derive(Serialize)]
struct TimeoutCheckResult {
    elapsed_ms: u64,
    remaining_ms: u64,
    expired: bool,
}

#[derive(Serialize)]
struct TimeoutAwaitResult {
    elapsed_ms: u64,
    expired: bool,
}

#[derive(Serialize)]
struct IntervalCreateResult {
    interval_id: Uuid,
}

#[derive(Serialize)]
struct IntervalTickResult {
    tick_number: u64,
    elapsed_ms: u64,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__sleep",
    description = "Sleep for a given number of milliseconds. \
                   Assumes: duration_ms is non-zero. \
                   Establishes: SleepCompleted.",
    emit = Auto
)]
async fn time_sleep(_ctx: Arc<TokioTimeCtx>, p: SleepParams) -> Result<CallToolResult, ErrorData> {
    let start = Instant::now();
    tokio::time::sleep(Duration::from_millis(p.duration_ms)).await;
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let _proof: Established<SleepCompleted> = Established::assert();
    Ok(json_result(&SleepResult { elapsed_ms }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__sleep_until",
    description = "Sleep until an absolute deadline given as milliseconds since the Unix epoch. \
                   If the deadline is already past, returns immediately. \
                   Assumes: deadline_unix_ms is a valid Unix timestamp in milliseconds. \
                   Establishes: SleepCompleted.",
    emit = Auto
)]
async fn time_sleep_until(
    _ctx: Arc<TokioTimeCtx>,
    p: SleepUntilParams,
) -> Result<CallToolResult, ErrorData> {
    let start = Instant::now();
    let now_unix_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    if p.deadline_unix_ms > now_unix_ms {
        let remaining = Duration::from_millis(p.deadline_unix_ms - now_unix_ms);
        tokio::time::sleep(remaining).await;
    }
    let elapsed_ms = start.elapsed().as_millis() as u64;
    let _proof: Established<SleepCompleted> = Established::assert();
    Ok(json_result(&SleepResult { elapsed_ms }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__timeout_create",
    description = "Create a timeout handle for a given duration. Returns a `timeout_id` UUID. \
                   Use `timeout_check` to inspect remaining time without blocking, or \
                   `timeout_await` to block until the deadline fires. \
                   The timeout is not cancelled when the handle is dropped — call \
                   `timeout_await` to consume it.",
    emit = Auto
)]
async fn time_timeout_create(
    ctx: Arc<TokioTimeCtx>,
    p: TimeoutCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let created = Instant::now();
    let deadline = created + Duration::from_millis(p.duration_ms);
    let timeout_id = Uuid::new_v4();
    ctx.deadlines
        .lock()
        .await
        .insert(timeout_id, DeadlineEntry { created, deadline });
    Ok(json_result(&TimeoutCreateResult { timeout_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__timeout_check",
    description = "Non-blocking probe of a timeout handle. Returns elapsed_ms, remaining_ms, \
                   and expired flag. Does not consume the handle. \
                   Assumes: timeout_id was returned by tokio_time__timeout_create.",
    emit = Auto
)]
async fn time_timeout_check(
    ctx: Arc<TokioTimeCtx>,
    p: TimeoutCheckParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = {
        let guard = ctx.deadlines.lock().await;
        guard
            .get(&p.timeout_id)
            .map(|e| (e.created, e.deadline))
            .ok_or_else(|| {
                ErrorData::invalid_params(format!("timeout_id not found: {}", p.timeout_id), None)
            })?
    };
    let (created, deadline) = entry;
    let now = Instant::now();
    let elapsed_ms = now.duration_since(created).as_millis() as u64;
    let expired = now >= deadline;
    let remaining_ms = if expired {
        0
    } else {
        deadline.duration_since(now).as_millis() as u64
    };
    Ok(json_result(&TimeoutCheckResult {
        elapsed_ms,
        remaining_ms,
        expired,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__timeout_await",
    description = "Block until the timeout fires, then consume the handle. \
                   If the deadline has already passed, returns immediately. \
                   Returns elapsed_ms and whether the timeout expired naturally. \
                   Assumes: timeout_id was returned by tokio_time__timeout_create. \
                   Establishes: TimeoutResolved.",
    emit = Auto
)]
async fn time_timeout_await(
    ctx: Arc<TokioTimeCtx>,
    p: TimeoutAwaitParams,
) -> Result<CallToolResult, ErrorData> {
    let entry = ctx
        .deadlines
        .lock()
        .await
        .remove(&p.timeout_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("timeout_id not found: {}", p.timeout_id), None)
        })?;
    let now = Instant::now();
    let expired = now >= entry.deadline;
    if !expired {
        sleep_until(entry.deadline).await;
    }
    let elapsed_ms = entry.created.elapsed().as_millis() as u64;
    let _proof: Established<TimeoutResolved> = Established::assert();
    Ok(json_result(&TimeoutAwaitResult {
        elapsed_ms,
        expired,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__interval_create",
    description = "Create a periodic interval. Returns an `interval_id` UUID. \
                   The first tick fires immediately (at creation time). \
                   Call `interval_tick` to await each subsequent period. \
                   Assumes: period_ms > 0.",
    emit = Auto
)]
async fn time_interval_create(
    ctx: Arc<TokioTimeCtx>,
    p: IntervalCreateParams,
) -> Result<CallToolResult, ErrorData> {
    if p.period_ms == 0 {
        return Err(ErrorData::invalid_params("period_ms must be > 0", None));
    }
    let started = Instant::now();
    let inner = interval(Duration::from_millis(p.period_ms));
    let interval_id = Uuid::new_v4();
    ctx.intervals.lock().await.insert(
        interval_id,
        IntervalEntry {
            started,
            tick_count: 0,
            inner: Arc::new(Mutex::new(inner)),
        },
    );
    Ok(json_result(&IntervalCreateResult { interval_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_time",
    name = "tokio_time__interval_tick",
    description = "Await the next tick of a periodic interval. Returns the tick number \
                   (0-indexed) and elapsed_ms since interval creation. \
                   Assumes: interval_id was returned by tokio_time__interval_create.",
    emit = Auto
)]
async fn time_interval_tick(
    ctx: Arc<TokioTimeCtx>,
    p: IntervalTickParams,
) -> Result<CallToolResult, ErrorData> {
    // Extract the Arc<Mutex<Interval>> without holding the outer map lock across the await.
    let (inner_arc, started) = {
        let mut guard = ctx.intervals.lock().await;
        let entry = guard.get_mut(&p.interval_id).ok_or_else(|| {
            ErrorData::invalid_params(format!("interval_id not found: {}", p.interval_id), None)
        })?;
        entry.tick_count += 1;
        let tick_number = entry.tick_count - 1;
        let started = entry.started;
        (entry.inner.clone(), (tick_number, started))
    };
    let (tick_number, started) = started;
    inner_arc.lock().await.tick().await;
    let elapsed_ms = started.elapsed().as_millis() as u64;
    Ok(json_result(&IntervalTickResult {
        tick_number,
        elapsed_ms,
    }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_time__*` tools for async time management.
///
/// Holds a UUID-keyed registry of active timeout deadlines and periodic
/// intervals. All tokio time objects live server-side; agents interact via
/// UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tokio_time"` namespace and named
/// `tokio_time__<verb>`.
pub struct TokioTimePlugin(Arc<TokioTimeCtx>);

impl TokioTimePlugin {
    /// Create a new `TokioTimePlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(TokioTimeCtx::new()))
    }
}

impl Default for TokioTimePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioTimePlugin {
    fn name(&self) -> &'static str {
        "tokio_time"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_time")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            let full_name = if name.starts_with("tokio_time__") {
                name.to_string()
            } else {
                format!("tokio_time__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_time")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
