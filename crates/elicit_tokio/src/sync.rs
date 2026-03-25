//! `TokioSyncPlugin` — MCP tools for tokio sync primitives.
//!
//! Semaphores, notifications, and barriers held server-side in UUID-keyed
//! registries. Agents coordinate via UUID handles — no sync objects cross
//! the MCP boundary.
//!
//! # Tool namespace: `tokio_sync__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `semaphore_new` | `permits: u32` | `{ semaphore_id }` | — |
//! | `semaphore_acquire` | `semaphore_id` | `{ permit_id, available_permits }` | `PermitAcquired` |
//! | `semaphore_try_acquire` | `semaphore_id` | `{ permit_id?, acquired, available_permits }` | `PermitAcquired` |
//! | `semaphore_release` | `permit_id` | `{ ok }` | — |
//! | `semaphore_available` | `semaphore_id` | `{ available_permits }` | — |
//! | `semaphore_close` | `semaphore_id` | `{ ok }` | — |
//! | `notify_new` | — | `{ notify_id }` | — |
//! | `notify_one` | `notify_id` | `{ ok }` | — |
//! | `notify_waiters` | `notify_id` | `{ ok }` | — |
//! | `notified` | `notify_id` | `{ ok }` | `NotificationReceived` |
//! | `barrier_new` | `count: usize` | `{ barrier_id }` | — |
//! | `barrier_wait` | `barrier_id` | `{ is_leader }` | `BarrierReached` |

use std::collections::HashMap;
use std::sync::Arc;

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
use tokio::sync::{Barrier, Mutex, Notify, OwnedSemaphorePermit, Semaphore};
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a semaphore permit was successfully acquired.
#[derive(Elicit)]
pub struct PermitAcquired;
impl Prop for PermitAcquired {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_permit_acquired_axiom() {
                let permits_available: u32 = kani::any();
                kani::assume(permits_available > 0);
                let acquired = true;
                assert!(acquired, "tokio::sync::Semaphore::acquire axiom: Ok => permit decremented");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_permit_acquired(acquire_returned_ok: bool) -> (result: bool)
                ensures result == acquire_returned_ok,
            {
                acquire_returned_ok
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_permit_acquired_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for PermitAcquired {}


/// Proposition: a `notified()` await returned — a notification was received.
#[derive(Elicit)]
pub struct NotificationReceived;
impl Prop for NotificationReceived {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_notification_received_axiom() {
                let notified: bool = kani::any();
                kani::assume(notified);
                assert!(notified, "tokio::sync::Notify::notified axiom: returns when notify_one/waiters called");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_notification_received(was_notified: bool) -> (result: bool)
                ensures result == was_notified,
            {
                was_notified
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_notification_received_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for NotificationReceived {}


/// Proposition: all parties have reached the barrier and it has released.
#[derive(Elicit)]
pub struct BarrierReached;
impl Prop for BarrierReached {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_barrier_reached_axiom() {
                let all_arrived: bool = kani::any();
                kani::assume(all_arrived);
                assert!(all_arrived, "tokio::sync::Barrier::wait axiom: returns when all parties arrive");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_barrier_reached(all_arrived: bool) -> (result: bool)
                ensures result == all_arrived,
            {
                all_arrived
            }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_barrier_reached_contract() -> bool {
                true
            }
        }
    }
}
impl VerifiedWorkflow for BarrierReached {}


// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tokio_sync__*` tool calls.
pub struct SyncCtx {
    semaphores: Mutex<HashMap<Uuid, Arc<Semaphore>>>,
    permits: Mutex<HashMap<Uuid, OwnedSemaphorePermit>>,
    notifiers: Mutex<HashMap<Uuid, Arc<Notify>>>,
    barriers: Mutex<HashMap<Uuid, Arc<Barrier>>>,
}

impl SyncCtx {
    fn new() -> Self {
        Self {
            semaphores: Mutex::new(HashMap::new()),
            permits: Mutex::new(HashMap::new()),
            notifiers: Mutex::new(HashMap::new()),
            barriers: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for SyncCtx {}

// ── Param structs ─────────────────────────────────────────────────────────────

/// Parameters for `tokio_sync__semaphore_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SemaphoreNewParams {
    /// Initial number of permits available.
    pub permits: u32,
}

/// Parameters for `tokio_sync__semaphore_acquire`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SemaphoreAcquireParams {
    /// UUID returned by `tokio_sync__semaphore_new`.
    pub semaphore_id: Uuid,
}

/// Parameters for `tokio_sync__semaphore_try_acquire`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SemaphoreTryAcquireParams {
    /// UUID returned by `tokio_sync__semaphore_new`.
    pub semaphore_id: Uuid,
}

/// Parameters for `tokio_sync__semaphore_release`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SemaphoreReleaseParams {
    /// UUID returned by `tokio_sync__semaphore_acquire` or `tokio_sync__semaphore_try_acquire`.
    pub permit_id: Uuid,
}

/// Parameters for `tokio_sync__semaphore_available`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SemaphoreAvailableParams {
    /// UUID returned by `tokio_sync__semaphore_new`.
    pub semaphore_id: Uuid,
}

/// Parameters for `tokio_sync__semaphore_close`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SemaphoreCloseParams {
    /// UUID returned by `tokio_sync__semaphore_new`.
    pub semaphore_id: Uuid,
}

/// Parameters for `tokio_sync__notify_new`.
///
/// No fields — a new notifier needs no configuration.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NotifyNewParams {}

/// Parameters for `tokio_sync__notify_one`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NotifyOneParams {
    /// UUID returned by `tokio_sync__notify_new`.
    pub notify_id: Uuid,
}

/// Parameters for `tokio_sync__notify_waiters`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NotifyWaitersParams {
    /// UUID returned by `tokio_sync__notify_new`.
    pub notify_id: Uuid,
}

/// Parameters for `tokio_sync__notified`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct NotifiedParams {
    /// UUID returned by `tokio_sync__notify_new`.
    pub notify_id: Uuid,
}

/// Parameters for `tokio_sync__barrier_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarrierNewParams {
    /// Number of tasks that must call `barrier_wait` before any proceed.
    pub count: usize,
}

/// Parameters for `tokio_sync__barrier_wait`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarrierWaitParams {
    /// UUID returned by `tokio_sync__barrier_new`.
    pub barrier_id: Uuid,
}

// ── Result structs ────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct SemaphoreNewResult {
    semaphore_id: Uuid,
}

#[derive(Serialize)]
struct SemaphoreAcquireResult {
    permit_id: Uuid,
    available_permits: usize,
}

#[derive(Serialize)]
struct SemaphoreTryAcquireResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    permit_id: Option<Uuid>,
    acquired: bool,
    available_permits: usize,
}

#[derive(Serialize)]
struct OkResult {
    ok: bool,
}

#[derive(Serialize)]
struct SemaphoreAvailableResult {
    available_permits: usize,
}

#[derive(Serialize)]
struct NotifyNewResult {
    notify_id: Uuid,
}

#[derive(Serialize)]
struct BarrierNewResult {
    barrier_id: Uuid,
}

#[derive(Serialize)]
struct BarrierWaitResult {
    is_leader: bool,
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__semaphore_new",
    description = "Create a new semaphore with the given number of initial permits. \
                   Returns a `semaphore_id` UUID. \
                   Assumes: permits > 0.",
    emit = Auto
)]
async fn sync_semaphore_new(
    ctx: Arc<SyncCtx>,
    p: SemaphoreNewParams,
) -> Result<CallToolResult, ErrorData> {
    let semaphore_id = Uuid::new_v4();
    let sem = Arc::new(Semaphore::new(p.permits as usize));
    ctx.semaphores.lock().await.insert(semaphore_id, sem);
    Ok(json_result(&SemaphoreNewResult { semaphore_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__semaphore_acquire",
    description = "Acquire one permit from the semaphore, blocking until one is available. \
                   Returns a `permit_id` UUID — pass it to `semaphore_release` when done. \
                   Assumes: semaphore_id was returned by `tokio_sync__semaphore_new` and \
                   the semaphore has not been closed. \
                   Establishes: PermitAcquired.",
    emit = Auto
)]
async fn sync_semaphore_acquire(
    ctx: Arc<SyncCtx>,
    p: SemaphoreAcquireParams,
) -> Result<CallToolResult, ErrorData> {
    let sem = ctx
        .semaphores
        .lock()
        .await
        .get(&p.semaphore_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("semaphore_id not found: {}", p.semaphore_id), None)
        })?;
    let permit = sem
        .clone()
        .acquire_owned()
        .await
        .map_err(|e| ErrorData::internal_error(format!("semaphore closed: {e}"), None))?;
    let available_permits = sem.available_permits();
    let permit_id = Uuid::new_v4();
    ctx.permits.lock().await.insert(permit_id, permit);
    let _proof: Established<PermitAcquired> = Established::assert();
    Ok(json_result(&SemaphoreAcquireResult {
        permit_id,
        available_permits,
    }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__semaphore_try_acquire",
    description = "Attempt to acquire one permit without blocking. \
                   `acquired` is true if a permit was obtained; in that case \
                   `permit_id` is set and should be passed to `semaphore_release`. \
                   Assumes: semaphore_id was returned by `tokio_sync__semaphore_new`. \
                   Establishes: PermitAcquired (only when acquired = true).",
    emit = Auto
)]
async fn sync_semaphore_try_acquire(
    ctx: Arc<SyncCtx>,
    p: SemaphoreTryAcquireParams,
) -> Result<CallToolResult, ErrorData> {
    let sem = ctx
        .semaphores
        .lock()
        .await
        .get(&p.semaphore_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("semaphore_id not found: {}", p.semaphore_id), None)
        })?;
    match sem.clone().try_acquire_owned() {
        Ok(permit) => {
            let available_permits = sem.available_permits();
            let permit_id = Uuid::new_v4();
            ctx.permits.lock().await.insert(permit_id, permit);
            let _proof: Established<PermitAcquired> = Established::assert();
            Ok(json_result(&SemaphoreTryAcquireResult {
                permit_id: Some(permit_id),
                acquired: true,
                available_permits,
            }))
        }
        Err(_) => Ok(json_result(&SemaphoreTryAcquireResult {
            permit_id: None,
            acquired: false,
            available_permits: sem.available_permits(),
        })),
    }
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__semaphore_release",
    description = "Release a previously acquired permit back to the semaphore. \
                   Assumes: permit_id was returned by `semaphore_acquire` or \
                   `semaphore_try_acquire`.",
    emit = Auto
)]
async fn sync_semaphore_release(
    ctx: Arc<SyncCtx>,
    p: SemaphoreReleaseParams,
) -> Result<CallToolResult, ErrorData> {
    let permit = ctx
        .permits
        .lock()
        .await
        .remove(&p.permit_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("permit_id not found: {}", p.permit_id), None)
        })?;
    drop(permit);
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__semaphore_available",
    description = "Query how many permits are currently available without acquiring any. \
                   Assumes: semaphore_id was returned by `tokio_sync__semaphore_new`.",
    emit = Auto
)]
async fn sync_semaphore_available(
    ctx: Arc<SyncCtx>,
    p: SemaphoreAvailableParams,
) -> Result<CallToolResult, ErrorData> {
    let available_permits = ctx
        .semaphores
        .lock()
        .await
        .get(&p.semaphore_id)
        .map(|s| s.available_permits())
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("semaphore_id not found: {}", p.semaphore_id), None)
        })?;
    Ok(json_result(&SemaphoreAvailableResult { available_permits }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__semaphore_close",
    description = "Close and remove a semaphore. All outstanding `acquire` calls will \
                   return an error. Any held permits remain valid until released. \
                   Assumes: semaphore_id was returned by `tokio_sync__semaphore_new`.",
    emit = Auto
)]
async fn sync_semaphore_close(
    ctx: Arc<SyncCtx>,
    p: SemaphoreCloseParams,
) -> Result<CallToolResult, ErrorData> {
    let sem = ctx
        .semaphores
        .lock()
        .await
        .remove(&p.semaphore_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("semaphore_id not found: {}", p.semaphore_id), None)
        })?;
    sem.close();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__notify_new",
    description = "Create a new notifier. Returns a `notify_id` UUID. \
                   Use `notify_one` or `notify_waiters` to send notifications \
                   and `notified` to wait for one.",
    emit = Auto
)]
async fn sync_notify_new(
    ctx: Arc<SyncCtx>,
    _p: NotifyNewParams,
) -> Result<CallToolResult, ErrorData> {
    let notify_id = Uuid::new_v4();
    ctx.notifiers
        .lock()
        .await
        .insert(notify_id, Arc::new(Notify::new()));
    Ok(json_result(&NotifyNewResult { notify_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__notify_one",
    description = "Wake one task waiting on `notified`. If no tasks are waiting, \
                   the notification is stored and the next `notified` call returns immediately. \
                   Assumes: notify_id was returned by `tokio_sync__notify_new`.",
    emit = Auto
)]
async fn sync_notify_one(
    ctx: Arc<SyncCtx>,
    p: NotifyOneParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.notifiers
        .lock()
        .await
        .get(&p.notify_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("notify_id not found: {}", p.notify_id), None)
        })?
        .notify_one();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__notify_waiters",
    description = "Wake all tasks currently waiting on `notified`. Does not store a \
                   notification for future waiters (unlike `notify_one`). \
                   Assumes: notify_id was returned by `tokio_sync__notify_new`.",
    emit = Auto
)]
async fn sync_notify_waiters(
    ctx: Arc<SyncCtx>,
    p: NotifyWaitersParams,
) -> Result<CallToolResult, ErrorData> {
    ctx.notifiers
        .lock()
        .await
        .get(&p.notify_id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("notify_id not found: {}", p.notify_id), None)
        })?
        .notify_waiters();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__notified",
    description = "Wait until a notification is sent via `notify_one` or `notify_waiters`. \
                   If a stored notification is already pending, returns immediately. \
                   Assumes: notify_id was returned by `tokio_sync__notify_new`. \
                   Establishes: NotificationReceived.",
    emit = Auto
)]
async fn sync_notified(ctx: Arc<SyncCtx>, p: NotifiedParams) -> Result<CallToolResult, ErrorData> {
    // Clone the Arc before awaiting so we don't hold the map lock.
    let notify = ctx
        .notifiers
        .lock()
        .await
        .get(&p.notify_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("notify_id not found: {}", p.notify_id), None)
        })?;
    notify.notified().await;
    let _proof: Established<NotificationReceived> = Established::assert();
    Ok(json_result(&OkResult { ok: true }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__barrier_new",
    description = "Create a new barrier that releases when `count` tasks have all called \
                   `barrier_wait`. Returns a `barrier_id` UUID. \
                   Assumes: count >= 1.",
    emit = Auto
)]
async fn sync_barrier_new(
    ctx: Arc<SyncCtx>,
    p: BarrierNewParams,
) -> Result<CallToolResult, ErrorData> {
    let barrier_id = Uuid::new_v4();
    ctx.barriers
        .lock()
        .await
        .insert(barrier_id, Arc::new(Barrier::new(p.count)));
    Ok(json_result(&BarrierNewResult { barrier_id }))
}

#[elicitation::elicit_tool(
    plugin = "tokio_sync",
    name = "tokio_sync__barrier_wait",
    description = "Block until all parties (the `count` given at creation) have called \
                   `barrier_wait`. One task is designated leader (`is_leader: true`); \
                   the rest receive `is_leader: false`. \
                   Assumes: barrier_id was returned by `tokio_sync__barrier_new`. \
                   Establishes: BarrierReached.",
    emit = Auto
)]
async fn sync_barrier_wait(
    ctx: Arc<SyncCtx>,
    p: BarrierWaitParams,
) -> Result<CallToolResult, ErrorData> {
    let barrier = ctx
        .barriers
        .lock()
        .await
        .get(&p.barrier_id)
        .cloned()
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("barrier_id not found: {}", p.barrier_id), None)
        })?;
    let result = barrier.wait().await;
    let _proof: Established<BarrierReached> = Established::assert();
    Ok(json_result(&BarrierWaitResult {
        is_leader: result.is_leader(),
    }))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tokio_sync__*` tools for coordinating async tasks.
///
/// Holds UUID-keyed registries for semaphores, acquired permits, notifiers,
/// and barriers. All tokio sync objects live server-side; agents interact via
/// UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tokio_sync"` namespace and named
/// `tokio_sync__<verb>`.
pub struct TokioSyncPlugin(Arc<SyncCtx>);

impl TokioSyncPlugin {
    /// Create a new `TokioSyncPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(SyncCtx::new()))
    }
}

impl Default for TokioSyncPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TokioSyncPlugin {
    fn name(&self) -> &'static str {
        "tokio_sync"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tokio_sync")
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
            let full_name = if name.starts_with("tokio_sync__") {
                name.to_string()
            } else {
                format!("tokio_sync__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tokio_sync")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
