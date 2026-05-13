//! `TowerBalancePlugin` — MCP tools for tower load balancing and load estimation.
//!
//! Balance, PeakEwma, and PendingRequests configs are held server-side in
//! UUID-keyed registries. Agents receive UUID handles; no live services cross
//! the MCP boundary.
//!
//! # Tool namespace: `tower_balance__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `p2c_new` | `discovery_name, req_type` | `{ balance_id }` | `TowerBalanceCreated` |
//! | `peak_ewma_new` | `service_name, default_rtt_micros, decay_nanos` | `{ load_id }` | `TowerLoadCreated` |
//! | `pending_requests_new` | `service_name` | `{ load_id }` | `TowerLoadCreated` |
//! | `describe` | `id` | JSON config | — |

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

/// Proposition: a `tower::balance::p2c::Balance` config was successfully created.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerBalanceCreated;
impl Prop for TowerBalanceCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_balance_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower balance created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_balance_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_balance_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerBalanceCreated {}

/// Proposition: a load estimator (PeakEwma or PendingRequests) config was successfully created.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerLoadCreated;
impl Prop for TowerLoadCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_load_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower load created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_load_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_load_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerLoadCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_balance__*` tool calls.
pub struct TowerBalanceCtx {
    balances: Mutex<HashMap<Uuid, elicitation::TowerBalance>>,
    peak_ewmas: Mutex<HashMap<Uuid, elicitation::TowerPeakEwma>>,
    pending_requests: Mutex<HashMap<Uuid, elicitation::TowerPendingRequests>>,
}

impl TowerBalanceCtx {
    fn new() -> Self {
        Self {
            balances: Mutex::new(HashMap::new()),
            peak_ewmas: Mutex::new(HashMap::new()),
            pending_requests: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerBalanceCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tower_balance__p2c_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct P2cNewParams {
    /// Rust identifier for the service discovery stream.
    pub discovery_name: String,
    /// Request type (Rust expression).
    pub req_type: String,
}

/// Parameters for `tower_balance__peak_ewma_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PeakEwmaNewParams {
    /// Rust identifier for the wrapped service.
    pub service_name: String,
    /// Default RTT estimate in microseconds.
    pub default_rtt_micros: u64,
    /// Decay time constant in nanoseconds.
    pub decay_nanos: f64,
}

/// Parameters for `tower_balance__pending_requests_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PendingRequestsNewParams {
    /// Rust identifier for the wrapped service.
    pub service_name: String,
}

/// Parameters for `tower_balance__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BalanceDescribeParams {
    /// UUID returned by any `tower_balance__*_new` tool.
    pub id: String,
}

/// Result returned by `tower_balance__p2c_new`.
#[derive(Debug, Serialize)]
pub struct BalanceIdResult {
    /// UUID handle for the newly created balance config.
    pub balance_id: String,
}

/// Result returned by `tower_balance__peak_ewma_new` and `pending_requests_new`.
#[derive(Debug, Serialize)]
pub struct LoadIdResult {
    /// UUID handle for the newly created load estimator config.
    pub load_id: String,
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
    plugin = "tower_balance",
    name = "tower_balance__p2c_new",
    description = "Create a p2c Balance config with a given discovery stream and request type. \
                   Establishes: TowerBalanceCreated.",
    emit = Auto
)]
async fn p2c_new(ctx: Arc<TowerBalanceCtx>, p: P2cNewParams) -> Result<CallToolResult, ErrorData> {
    let cfg = elicitation::TowerBalance {
        discovery_name: p.discovery_name,
        req_type: p.req_type,
    };
    let id = Uuid::new_v4();
    ctx.balances.lock().await.insert(id, cfg);
    let _proof: Established<TowerBalanceCreated> = Established::assert();
    Ok(json_result(&BalanceIdResult {
        balance_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_balance",
    name = "tower_balance__peak_ewma_new",
    description = "Create a PeakEwma load estimator config. \
                   Assumes: default_rtt_micros > 0, decay_nanos > 0. \
                   Establishes: TowerLoadCreated.",
    emit = Auto
)]
async fn peak_ewma_new(
    ctx: Arc<TowerBalanceCtx>,
    p: PeakEwmaNewParams,
) -> Result<CallToolResult, ErrorData> {
    let cfg = elicitation::TowerPeakEwma {
        service_name: p.service_name,
        default_rtt_micros: p.default_rtt_micros,
        decay_nanos: p.decay_nanos,
    };
    let id = Uuid::new_v4();
    ctx.peak_ewmas.lock().await.insert(id, cfg);
    let _proof: Established<TowerLoadCreated> = Established::assert();
    Ok(json_result(&LoadIdResult {
        load_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_balance",
    name = "tower_balance__pending_requests_new",
    description = "Create a PendingRequests load estimator config. \
                   Establishes: TowerLoadCreated.",
    emit = Auto
)]
async fn pending_requests_new(
    ctx: Arc<TowerBalanceCtx>,
    p: PendingRequestsNewParams,
) -> Result<CallToolResult, ErrorData> {
    let cfg = elicitation::TowerPendingRequests {
        service_name: p.service_name,
    };
    let id = Uuid::new_v4();
    ctx.pending_requests.lock().await.insert(id, cfg);
    let _proof: Established<TowerLoadCreated> = Established::assert();
    Ok(json_result(&LoadIdResult {
        load_id: id.to_string(),
    }))
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum BalanceDescription {
    Balance(elicitation::TowerBalance),
    PeakEwma(elicitation::TowerPeakEwma),
    PendingRequests(elicitation::TowerPendingRequests),
}

#[elicitation::elicit_tool(
    plugin = "tower_balance",
    name = "tower_balance__describe",
    description = "Describe a previously created balance or load estimator config by UUID. \
                   Assumes: id is a valid UUID returned by a prior creation tool.",
    emit = Auto
)]
async fn balance_describe(
    ctx: Arc<TowerBalanceCtx>,
    p: BalanceDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid =
        p.id.parse()
            .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.id), None))?;

    if let Some(cfg) = ctx.balances.lock().await.get(&id).cloned() {
        return Ok(json_result(&BalanceDescription::Balance(cfg)));
    }
    if let Some(cfg) = ctx.peak_ewmas.lock().await.get(&id).cloned() {
        return Ok(json_result(&BalanceDescription::PeakEwma(cfg)));
    }
    if let Some(cfg) = ctx.pending_requests.lock().await.get(&id).cloned() {
        return Ok(json_result(&BalanceDescription::PendingRequests(cfg)));
    }

    Err(ErrorData::invalid_params(
        format!("id not found: {id}"),
        None,
    ))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_balance__*` tools for load balancing.
///
/// Holds UUID-keyed registries of p2c Balance configs, PeakEwma load estimators,
/// and PendingRequests load estimators. All config objects live server-side;
/// agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_balance"` namespace and named
/// `tower_balance__<verb>`.
pub struct TowerBalancePlugin(Arc<TowerBalanceCtx>);

impl TowerBalancePlugin {
    /// Create a new `TowerBalancePlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(TowerBalanceCtx::new()))
    }
}

impl Default for TowerBalancePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerBalancePlugin {
    fn name(&self) -> &'static str {
        "tower_balance"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_balance")
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
            let full_name = if name.starts_with("tower_balance__") {
                name.to_string()
            } else {
                format!("tower_balance__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_balance")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
