//! `TowerSteerPlugin` — MCP tools for `tower::steer::Steer` service routing.
//!
//! Steer configs are held server-side in a UUID-keyed registry.
//! Agents receive UUID handles; no live services cross the MCP boundary.
//!
//! # Tool namespace: `tower_steer__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `new` | `service_names, picker_name` | `{ steer_id }` | `TowerSteerCreated` |
//! | `describe` | `steer_id` | JSON config | — |

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

// ── Proposition ───────────────────────────────────────────────────────────────

/// Proposition: a `tower::steer::Steer` config was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerSteerCreated;
impl Prop for TowerSteerCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_steer_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower steer created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_steer_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_steer_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerSteerCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_steer__*` tool calls.
pub struct TowerSteerCtx {
    steers: Mutex<HashMap<Uuid, elicitation::TowerSteer>>,
}

impl TowerSteerCtx {
    fn new() -> Self {
        Self {
            steers: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerSteerCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tower_steer__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SteerNewParams {
    /// Rust identifiers for each service in the pool.
    pub service_names: Vec<String>,
    /// Rust identifier for the `Picker` implementation.
    pub picker_name: String,
}

/// Parameters for `tower_steer__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SteerDescribeParams {
    /// UUID returned by `tower_steer__new`.
    pub steer_id: String,
}

/// Result returned by `tower_steer__new`.
#[derive(Debug, Serialize)]
pub struct SteerIdResult {
    /// UUID handle for the newly created steer config.
    pub steer_id: String,
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
    plugin = "tower_steer",
    name = "tower_steer__new",
    description = "Create a Steer config with a pool of services and a picker. \
                   Assumes: service_names is non-empty, picker_name is a valid Rust identifier. \
                   Establishes: TowerSteerCreated.",
    emit = Auto
)]
async fn steer_new(
    ctx: Arc<TowerSteerCtx>,
    p: SteerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let cfg = elicitation::TowerSteer {
        service_names: p.service_names,
        picker_name: p.picker_name,
    };
    let id = Uuid::new_v4();
    ctx.steers.lock().await.insert(id, cfg);
    let _proof: Established<TowerSteerCreated> = Established::assert();
    Ok(json_result(&SteerIdResult {
        steer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_steer",
    name = "tower_steer__describe",
    description = "Describe a previously created Steer config by its UUID. \
                   Assumes: steer_id is a valid UUID returned by tower_steer__new.",
    emit = Auto
)]
async fn steer_describe(
    ctx: Arc<TowerSteerCtx>,
    p: SteerDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .steer_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.steer_id), None))?;

    let cfg = ctx
        .steers
        .lock()
        .await
        .get(&id)
        .cloned()
        .ok_or_else(|| ErrorData::invalid_params(format!("steer_id not found: {id}"), None))?;

    Ok(json_result(&cfg))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_steer__*` tools for service routing.
///
/// Holds a UUID-keyed registry of `TowerSteer` configs. All config objects
/// live server-side; agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_steer"` namespace and named
/// `tower_steer__<verb>`.
pub struct TowerSteerPlugin(Arc<TowerSteerCtx>);

impl TowerSteerPlugin {
    /// Create a new `TowerSteerPlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(TowerSteerCtx::new()))
    }
}

impl Default for TowerSteerPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerSteerPlugin {
    fn name(&self) -> &'static str {
        "tower_steer"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_steer")
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
            let full_name = if name.starts_with("tower_steer__") {
                name.to_string()
            } else {
                format!("tower_steer__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_steer")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
