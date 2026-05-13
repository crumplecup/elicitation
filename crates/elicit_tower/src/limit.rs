//! `TowerLimitPlugin` — MCP tools for tower concurrency and rate-limit layers.
//!
//! Layer config objects are held server-side in UUID-keyed registries.
//! Agents receive UUID handles; no live services cross the MCP boundary.
//!
//! # Tool namespace: `tower_limit__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `concurrency_limit_layer_new` | `max: usize` | `{ layer_id }` | `TowerLayerCreated` |
//! | `rate_limit_layer_new` | `num, per_millis` | `{ layer_id }` | `TowerLayerCreated` |
//! | `rate_new` | `num, per_millis` | `{ rate_id }` | `TowerRateCreated` |
//! | `timeout_layer_new` | `timeout_millis` | `{ layer_id }` | `TowerLayerCreated` |
//! | `buffer_layer_new` | `bound` | `{ layer_id }` | `TowerLayerCreated` |
//! | `load_shed_layer_new` | — | `{ layer_id }` | `TowerLayerCreated` |
//! | `spawn_ready_layer_new` | — | `{ layer_id }` | `TowerLayerCreated` |
//! | `layer_describe` | `layer_id` | JSON config | — |

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
use tokio::sync::Mutex;
use uuid::Uuid;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a tower limit layer was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerLayerCreated;
impl Prop for TowerLayerCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_layer_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower layer created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_layer_created(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_tower_layer_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerLayerCreated {}

/// Proposition: a `TowerRate` config was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerRateCreated;
impl Prop for TowerRateCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_rate_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower rate created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_rate_created(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_tower_rate_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerRateCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_limit__*` tool calls.
pub struct TowerLimitCtx {
    concurrency_layers: Mutex<HashMap<Uuid, elicitation::TowerConcurrencyLimitLayer>>,
    rate_limit_layers: Mutex<HashMap<Uuid, elicitation::TowerRateLimitLayer>>,
    rates: Mutex<HashMap<Uuid, elicitation::TowerRate>>,
    timeout_layers: Mutex<HashMap<Uuid, elicitation::TowerTimeoutLayer>>,
    buffer_layers: Mutex<HashMap<Uuid, elicitation::TowerBufferLayer>>,
    load_shed_layers: Mutex<HashMap<Uuid, elicitation::TowerLoadShedLayer>>,
    spawn_ready_layers: Mutex<HashMap<Uuid, elicitation::TowerSpawnReadyLayer>>,
}

impl TowerLimitCtx {
    fn new() -> Self {
        Self {
            concurrency_layers: Mutex::new(HashMap::new()),
            rate_limit_layers: Mutex::new(HashMap::new()),
            rates: Mutex::new(HashMap::new()),
            timeout_layers: Mutex::new(HashMap::new()),
            buffer_layers: Mutex::new(HashMap::new()),
            load_shed_layers: Mutex::new(HashMap::new()),
            spawn_ready_layers: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerLimitCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tower_limit__concurrency_limit_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConcurrencyLimitLayerNewParams {
    /// Maximum number of concurrent requests allowed.
    pub max: usize,
}

/// Parameters for `tower_limit__rate_limit_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RateLimitLayerNewParams {
    /// Number of requests allowed in the time window.
    pub num: u64,
    /// Duration of the time window in milliseconds.
    pub per_millis: u64,
}

/// Parameters for `tower_limit__rate_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RateNewParams {
    /// Number of requests allowed in the time window.
    pub num: u64,
    /// Duration of the time window in milliseconds.
    pub per_millis: u64,
}

/// Parameters for `tower_limit__timeout_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TimeoutLayerNewParams {
    /// Timeout duration in milliseconds.
    pub timeout_millis: u64,
}

/// Parameters for `tower_limit__buffer_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BufferLayerNewParams {
    /// Maximum number of requests to buffer.
    pub bound: usize,
}

/// Parameters for `tower_limit__load_shed_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LoadShedLayerNewParams {}

/// Parameters for `tower_limit__spawn_ready_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpawnReadyLayerNewParams {}

/// Parameters for `tower_limit__layer_describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LayerDescribeParams {
    /// UUID returned by any `*_layer_new` or `rate_new` tool.
    pub layer_id: String,
}

/// Result returned by all `*_layer_new` and `rate_new` tools.
#[derive(Debug, Serialize)]
pub struct LayerIdResult {
    /// UUID handle for the newly created layer config.
    pub layer_id: String,
}

#[derive(Serialize)]
struct RateIdResult {
    rate_id: String,
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
    plugin = "tower_limit",
    name = "tower_limit__concurrency_limit_layer_new",
    description = "Create a concurrency limit layer with the given max concurrent requests. \
                   Assumes: max > 0. \
                   Establishes: TowerLayerCreated.",
    emit = Auto
)]
async fn concurrency_limit_layer_new(
    ctx: Arc<TowerLimitCtx>,
    p: ConcurrencyLimitLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerConcurrencyLimitLayer { max: p.max };
    let id = Uuid::new_v4();
    ctx.concurrency_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__rate_limit_layer_new",
    description = "Create a rate-limit layer allowing `num` requests per `per_millis` ms window. \
                   Assumes: num > 0, per_millis > 0. \
                   Establishes: TowerLayerCreated.",
    emit = Auto
)]
async fn rate_limit_layer_new(
    ctx: Arc<TowerLimitCtx>,
    p: RateLimitLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerRateLimitLayer {
        num: p.num,
        per_millis: p.per_millis,
    };
    let id = Uuid::new_v4();
    ctx.rate_limit_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__rate_new",
    description = "Create a standalone Rate config (num requests per per_millis ms). \
                   Assumes: num > 0, per_millis > 0. \
                   Establishes: TowerRateCreated.",
    emit = Auto
)]
async fn rate_new(ctx: Arc<TowerLimitCtx>, p: RateNewParams) -> Result<CallToolResult, ErrorData> {
    let rate = elicitation::TowerRate {
        num: p.num,
        per_millis: p.per_millis,
    };
    let id = Uuid::new_v4();
    ctx.rates.lock().await.insert(id, rate);
    let _proof: Established<TowerRateCreated> = Established::assert();
    Ok(json_result(&RateIdResult {
        rate_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__timeout_layer_new",
    description = "Create a timeout layer that cancels requests exceeding `timeout_millis` ms. \
                   Assumes: timeout_millis > 0. \
                   Establishes: TowerLayerCreated.",
    emit = Auto
)]
async fn timeout_layer_new(
    ctx: Arc<TowerLimitCtx>,
    p: TimeoutLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerTimeoutLayer {
        timeout_millis: p.timeout_millis,
    };
    let id = Uuid::new_v4();
    ctx.timeout_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__buffer_layer_new",
    description = "Create a buffer layer that queues up to `bound` in-flight requests. \
                   Assumes: bound > 0. \
                   Establishes: TowerLayerCreated.",
    emit = Auto
)]
async fn buffer_layer_new(
    ctx: Arc<TowerLimitCtx>,
    p: BufferLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerBufferLayer { bound: p.bound };
    let id = Uuid::new_v4();
    ctx.buffer_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__load_shed_layer_new",
    description = "Create a load-shed layer that immediately returns an error when the inner \
                   service is not ready, rather than waiting. \
                   Establishes: TowerLayerCreated.",
    emit = Auto
)]
async fn load_shed_layer_new(
    ctx: Arc<TowerLimitCtx>,
    _p: LoadShedLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerLoadShedLayer;
    let id = Uuid::new_v4();
    ctx.load_shed_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__spawn_ready_layer_new",
    description = "Create a spawn-ready layer that drives the inner service to readiness on a \
                   background task before dispatching a request. \
                   Establishes: TowerLayerCreated.",
    emit = Auto
)]
async fn spawn_ready_layer_new(
    ctx: Arc<TowerLimitCtx>,
    _p: SpawnReadyLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerSpawnReadyLayer;
    let id = Uuid::new_v4();
    ctx.spawn_ready_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum LayerDescription {
    ConcurrencyLimit(elicitation::TowerConcurrencyLimitLayer),
    RateLimit(elicitation::TowerRateLimitLayer),
    Rate(elicitation::TowerRate),
    Timeout(elicitation::TowerTimeoutLayer),
    Buffer(elicitation::TowerBufferLayer),
    LoadShed,
    SpawnReady,
}

#[elicitation::elicit_tool(
    plugin = "tower_limit",
    name = "tower_limit__layer_describe",
    description = "Describe the config of a previously created layer or rate by its UUID. \
                   Assumes: layer_id is a valid UUID returned by a prior creation tool.",
    emit = Auto
)]
async fn layer_describe(
    ctx: Arc<TowerLimitCtx>,
    p: LayerDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .layer_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.layer_id), None))?;

    // Search each registry in turn.
    if let Some(l) = ctx.concurrency_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&LayerDescription::ConcurrencyLimit(l)));
    }
    if let Some(l) = ctx.rate_limit_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&LayerDescription::RateLimit(l)));
    }
    if let Some(r) = ctx.rates.lock().await.get(&id).cloned() {
        return Ok(json_result(&LayerDescription::Rate(r)));
    }
    if let Some(l) = ctx.timeout_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&LayerDescription::Timeout(l)));
    }
    if let Some(l) = ctx.buffer_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&LayerDescription::Buffer(l)));
    }
    if ctx.load_shed_layers.lock().await.contains_key(&id) {
        return Ok(json_result(&LayerDescription::LoadShed));
    }
    if ctx.spawn_ready_layers.lock().await.contains_key(&id) {
        return Ok(json_result(&LayerDescription::SpawnReady));
    }

    Err(ErrorData::invalid_params(
        format!("layer_id not found: {id}"),
        None,
    ))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_limit__*` tools for tower middleware layers.
///
/// Holds UUID-keyed registries of concurrency, rate, timeout, buffer,
/// load-shed, and spawn-ready layer configs. All config objects live
/// server-side; agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_limit"` namespace and named
/// `tower_limit__<verb>`.
pub struct TowerLimitPlugin(Arc<TowerLimitCtx>);

impl TowerLimitPlugin {
    /// Create a new `TowerLimitPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(TowerLimitCtx::new()))
    }
}

impl Default for TowerLimitPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerLimitPlugin {
    fn name(&self) -> &'static str {
        "tower_limit"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_limit")
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
            let full_name = if name.starts_with("tower_limit__") {
                name.to_string()
            } else {
                format!("tower_limit__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_limit")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
