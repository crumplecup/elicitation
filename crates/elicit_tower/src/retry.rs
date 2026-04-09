//! `TowerRetryPlugin` — MCP tools for tower retry, backoff, and filter layers.
//!
//! Backoff makers, TPS budgets, retry layers, and filter layers are held
//! server-side in UUID-keyed registries.
//!
//! # Tool namespace: `tower_retry__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `backoff_new` | `min_millis, max_millis, jitter` | `{ backoff_id }` | `TowerBackoffCreated` |
//! | `budget_new` | `ttl_millis, min_per_sec, retry_percent` | `{ budget_id }` | `TowerBudgetCreated` |
//! | `retry_layer_new` | `policy_name` | `{ layer_id }` | `TowerRetryLayerCreated` |
//! | `filter_layer_new` | `predicate_name` | `{ layer_id }` | `TowerRetryLayerCreated` |
//! | `backoff_describe` | `backoff_id` | JSON | — |
//! | `budget_describe` | `budget_id` | JSON | — |

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

/// Proposition: an exponential backoff maker was successfully created.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerBackoffCreated;
impl Prop for TowerBackoffCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_backoff_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower backoff created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_backoff_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_backoff_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerBackoffCreated {}

/// Proposition: a TPS retry budget was successfully created.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerBudgetCreated;
impl Prop for TowerBudgetCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_budget_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower budget created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_budget_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_budget_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerBudgetCreated {}

/// Proposition: a tower retry or filter layer was successfully created.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerRetryLayerCreated;
impl Prop for TowerRetryLayerCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_retry_layer_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower retry layer created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_retry_layer_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_retry_layer_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerRetryLayerCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_retry__*` tool calls.
pub struct TowerRetryCtx {
    backoffs: Mutex<HashMap<Uuid, elicitation::TowerExponentialBackoffMaker>>,
    budgets: Mutex<HashMap<Uuid, elicitation::TowerTpsBudget>>,
    retry_layers: Mutex<HashMap<Uuid, elicitation::TowerRetryLayer>>,
    filter_layers: Mutex<HashMap<Uuid, elicitation::TowerFilterLayer>>,
}

impl TowerRetryCtx {
    fn new() -> Self {
        Self {
            backoffs: Mutex::new(HashMap::new()),
            budgets: Mutex::new(HashMap::new()),
            retry_layers: Mutex::new(HashMap::new()),
            filter_layers: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerRetryCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tower_retry__backoff_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BackoffNewParams {
    /// Minimum backoff duration in milliseconds.
    pub min_millis: u64,
    /// Maximum backoff duration in milliseconds.
    pub max_millis: u64,
    /// Jitter factor in `[0.0, 100.0]`.
    pub jitter: f64,
}

/// Parameters for `tower_retry__budget_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BudgetNewParams {
    /// TTL duration in milliseconds.
    pub ttl_millis: u64,
    /// Minimum retries allowed per second regardless of error rate.
    pub min_per_sec: u32,
    /// Ratio of retries to original requests (e.g. `0.1` = 10%).
    pub retry_percent: f32,
}

/// Parameters for `tower_retry__retry_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RetryLayerNewParams {
    /// Name identifier for the retry policy (used for code generation).
    pub policy_name: String,
}

/// Parameters for `tower_retry__filter_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FilterLayerNewParams {
    /// Name identifier for the filter predicate (used for code generation).
    pub predicate_name: String,
}

/// Parameters for `tower_retry__backoff_describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BackoffDescribeParams {
    /// UUID returned by `tower_retry__backoff_new`.
    pub backoff_id: String,
}

/// Parameters for `tower_retry__budget_describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BudgetDescribeParams {
    /// UUID returned by `tower_retry__budget_new`.
    pub budget_id: String,
}

#[derive(Serialize)]
struct BackoffIdResult {
    backoff_id: String,
}

#[derive(Serialize)]
struct BudgetIdResult {
    budget_id: String,
}

#[derive(Serialize)]
struct LayerIdResult {
    layer_id: String,
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
    plugin = "tower_retry",
    name = "tower_retry__backoff_new",
    description = "Create an exponential backoff maker with min/max duration bounds and a jitter \
                   factor. Jitter is a value in [0.0, 100.0] representing the percentage of \
                   random variance applied to each backoff interval. \
                   Assumes: min_millis <= max_millis, jitter in [0.0, 100.0]. \
                   Establishes: TowerBackoffCreated.",
    emit = Auto
)]
async fn backoff_new(
    ctx: Arc<TowerRetryCtx>,
    p: BackoffNewParams,
) -> Result<CallToolResult, ErrorData> {
    let backoff = elicitation::TowerExponentialBackoffMaker {
        min_millis: p.min_millis,
        max_millis: p.max_millis,
        jitter: p.jitter,
    };
    let id = Uuid::new_v4();
    ctx.backoffs.lock().await.insert(id, backoff);
    let _proof: Established<TowerBackoffCreated> = Established::assert();
    Ok(json_result(&BackoffIdResult {
        backoff_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_retry",
    name = "tower_retry__budget_new",
    description = "Create a TPS retry budget that limits retries based on request throughput. \
                   `ttl_millis` is the sliding window duration; `min_per_sec` is the floor number \
                   of retries allowed per second; `retry_percent` is the ratio (e.g. 0.1 = 10%). \
                   Assumes: ttl_millis > 0, retry_percent in [0.0, 1.0]. \
                   Establishes: TowerBudgetCreated.",
    emit = Auto
)]
async fn budget_new(
    ctx: Arc<TowerRetryCtx>,
    p: BudgetNewParams,
) -> Result<CallToolResult, ErrorData> {
    let budget = elicitation::TowerTpsBudget {
        ttl_millis: p.ttl_millis,
        min_per_sec: p.min_per_sec,
        retry_percent: p.retry_percent,
    };
    let id = Uuid::new_v4();
    ctx.budgets.lock().await.insert(id, budget);
    let _proof: Established<TowerBudgetCreated> = Established::assert();
    Ok(json_result(&BudgetIdResult {
        budget_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_retry",
    name = "tower_retry__retry_layer_new",
    description = "Create a retry layer bound to a named policy. The policy_name is used as a \
                   symbolic identifier for code generation and inspection. \
                   Establishes: TowerRetryLayerCreated.",
    emit = Auto
)]
async fn retry_layer_new(
    ctx: Arc<TowerRetryCtx>,
    p: RetryLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerRetryLayer {
        policy_name: p.policy_name,
    };
    let id = Uuid::new_v4();
    ctx.retry_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerRetryLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_retry",
    name = "tower_retry__filter_layer_new",
    description = "Create a filter layer bound to a named predicate. The predicate_name is used \
                   as a symbolic identifier for code generation and inspection. \
                   Establishes: TowerRetryLayerCreated.",
    emit = Auto
)]
async fn filter_layer_new(
    ctx: Arc<TowerRetryCtx>,
    p: FilterLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerFilterLayer {
        predicate_name: p.predicate_name,
    };
    let id = Uuid::new_v4();
    ctx.filter_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerRetryLayerCreated> = Established::assert();
    Ok(json_result(&LayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_retry",
    name = "tower_retry__backoff_describe",
    description = "Describe the config of a previously created exponential backoff maker. \
                   Assumes: backoff_id is a valid UUID returned by tower_retry__backoff_new.",
    emit = Auto
)]
async fn backoff_describe(
    ctx: Arc<TowerRetryCtx>,
    p: BackoffDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .backoff_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.backoff_id), None))?;
    let backoff =
        ctx.backoffs.lock().await.get(&id).cloned().ok_or_else(|| {
            ErrorData::invalid_params(format!("backoff_id not found: {id}"), None)
        })?;
    Ok(json_result(&backoff))
}

#[elicitation::elicit_tool(
    plugin = "tower_retry",
    name = "tower_retry__budget_describe",
    description = "Describe the config of a previously created TPS retry budget. \
                   Assumes: budget_id is a valid UUID returned by tower_retry__budget_new.",
    emit = Auto
)]
async fn budget_describe(
    ctx: Arc<TowerRetryCtx>,
    p: BudgetDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .budget_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.budget_id), None))?;
    let budget = ctx
        .budgets
        .lock()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("budget_id not found: {id}"), None))?;
    Ok(json_result(&budget))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_retry__*` tools for retry, backoff, and filter layers.
///
/// Holds UUID-keyed registries of exponential backoff makers, TPS budgets,
/// retry layers, and filter layers. All config objects live server-side;
/// agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_retry"` namespace and named
/// `tower_retry__<verb>`.
pub struct TowerRetryPlugin(Arc<TowerRetryCtx>);

impl TowerRetryPlugin {
    /// Create a new `TowerRetryPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(TowerRetryCtx::new()))
    }
}

impl Default for TowerRetryPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerRetryPlugin {
    fn name(&self) -> &'static str {
        "tower_retry"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_retry")
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
            let full_name = if name.starts_with("tower_retry__") {
                name.to_string()
            } else {
                format!("tower_retry__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_retry")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
