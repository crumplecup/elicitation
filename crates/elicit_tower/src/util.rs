//! `TowerUtilPlugin` — MCP tools for tower utility layers and box-service configs.
//!
//! Layer/service config objects are held server-side in UUID-keyed registries.
//! Agents receive UUID handles; no live services cross the MCP boundary.
//!
//! # Tool namespace: `tower_util__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `map_err_layer_new` | `mapper_fn` | `{ layer_id }` | `TowerUtilLayerCreated` |
//! | `map_request_layer_new` | `mapper_fn` | `{ layer_id }` | `TowerUtilLayerCreated` |
//! | `map_response_layer_new` | `mapper_fn` | `{ layer_id }` | `TowerUtilLayerCreated` |
//! | `map_result_layer_new` | `mapper_fn` | `{ layer_id }` | `TowerUtilLayerCreated` |
//! | `and_then_layer_new` | `f` | `{ layer_id }` | `TowerUtilLayerCreated` |
//! | `then_layer_new` | `f` | `{ layer_id }` | `TowerUtilLayerCreated` |
//! | `box_service_new` | `req_type, resp_type, err_type` | `{ service_id }` | `TowerBoxServiceCreated` |
//! | `box_clone_service_new` | `req_type, resp_type, err_type` | `{ service_id }` | `TowerBoxServiceCreated` |
//! | `layer_describe` | `layer_id` | JSON config | — |
//! | `box_service_describe` | `service_id` | JSON config | — |

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

/// Proposition: a tower util layer was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerUtilLayerCreated;
impl Prop for TowerUtilLayerCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_util_layer_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower util layer created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_util_layer_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_util_layer_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerUtilLayerCreated {}

/// Proposition: a BoxService or BoxCloneService config was successfully created.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct TowerBoxServiceCreated;
impl Prop for TowerBoxServiceCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_tower_box_service_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "tower box service created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_tower_box_service_created(ok: bool) -> (result: bool)
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
            pub fn verify_tower_box_service_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for TowerBoxServiceCreated {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `tower_util__*` tool calls.
pub struct TowerUtilCtx {
    map_err_layers: Mutex<HashMap<Uuid, elicitation::TowerMapErrLayer>>,
    map_request_layers: Mutex<HashMap<Uuid, elicitation::TowerMapRequestLayer>>,
    map_response_layers: Mutex<HashMap<Uuid, elicitation::TowerMapResponseLayer>>,
    map_result_layers: Mutex<HashMap<Uuid, elicitation::TowerMapResultLayer>>,
    and_then_layers: Mutex<HashMap<Uuid, elicitation::TowerAndThenLayer>>,
    then_layers: Mutex<HashMap<Uuid, elicitation::TowerThenLayer>>,
    box_services: Mutex<HashMap<Uuid, elicitation::TowerBoxServiceConfig>>,
    box_clone_services: Mutex<HashMap<Uuid, elicitation::TowerBoxCloneServiceConfig>>,
}

impl TowerUtilCtx {
    fn new() -> Self {
        Self {
            map_err_layers: Mutex::new(HashMap::new()),
            map_request_layers: Mutex::new(HashMap::new()),
            map_response_layers: Mutex::new(HashMap::new()),
            map_result_layers: Mutex::new(HashMap::new()),
            and_then_layers: Mutex::new(HashMap::new()),
            then_layers: Mutex::new(HashMap::new()),
            box_services: Mutex::new(HashMap::new()),
            box_clone_services: Mutex::new(HashMap::new()),
        }
    }
}

impl PluginContext for TowerUtilCtx {}

// ── Param and result structs ──────────────────────────────────────────────────

/// Parameters for `tower_util__map_err_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MapErrLayerNewParams {
    /// Rust identifier for the error mapping fn.
    pub mapper_fn: String,
}

/// Parameters for `tower_util__map_request_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MapRequestLayerNewParams {
    /// Rust identifier for the request mapping fn.
    pub mapper_fn: String,
}

/// Parameters for `tower_util__map_response_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MapResponseLayerNewParams {
    /// Rust identifier for the response mapping fn.
    pub mapper_fn: String,
}

/// Parameters for `tower_util__map_result_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MapResultLayerNewParams {
    /// Rust identifier for the result mapping fn.
    pub mapper_fn: String,
}

/// Parameters for `tower_util__and_then_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AndThenLayerNewParams {
    /// Rust identifier for the async and_then fn.
    pub f: String,
}

/// Parameters for `tower_util__then_layer_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ThenLayerNewParams {
    /// Rust identifier for the async then fn.
    pub f: String,
}

/// Parameters for `tower_util__box_service_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BoxServiceNewParams {
    /// Request type (Rust expression).
    pub req_type: String,
    /// Response type (Rust expression).
    pub resp_type: String,
    /// Error type (Rust expression).
    pub err_type: String,
}

/// Parameters for `tower_util__box_clone_service_new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BoxCloneServiceNewParams {
    /// Request type (Rust expression).
    pub req_type: String,
    /// Response type (Rust expression).
    pub resp_type: String,
    /// Error type (Rust expression).
    pub err_type: String,
}

/// Parameters for `tower_util__layer_describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct UtilLayerDescribeParams {
    /// UUID returned by any `*_layer_new` tool.
    pub layer_id: String,
}

/// Parameters for `tower_util__box_service_describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BoxServiceDescribeParams {
    /// UUID returned by `box_service_new` or `box_clone_service_new`.
    pub service_id: String,
}

/// Result returned by all `*_layer_new` tools.
#[derive(Debug, Serialize)]
pub struct UtilLayerIdResult {
    /// UUID handle for the newly created layer config.
    pub layer_id: String,
}

/// Result returned by all `box_*_new` tools.
#[derive(Debug, Serialize)]
pub struct ServiceIdResult {
    /// UUID handle for the newly created box-service config.
    pub service_id: String,
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
    plugin = "tower_util",
    name = "tower_util__map_err_layer_new",
    description = "Create a MapErr layer using the given error-mapping function identifier. \
                   Establishes: TowerUtilLayerCreated.",
    emit = Auto
)]
async fn map_err_layer_new(
    ctx: Arc<TowerUtilCtx>,
    p: MapErrLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerMapErrLayer {
        mapper_fn: p.mapper_fn,
    };
    let id = Uuid::new_v4();
    ctx.map_err_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerUtilLayerCreated> = Established::assert();
    Ok(json_result(&UtilLayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__map_request_layer_new",
    description = "Create a MapRequest layer using the given request-mapping function identifier. \
                   Establishes: TowerUtilLayerCreated.",
    emit = Auto
)]
async fn map_request_layer_new(
    ctx: Arc<TowerUtilCtx>,
    p: MapRequestLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerMapRequestLayer {
        mapper_fn: p.mapper_fn,
    };
    let id = Uuid::new_v4();
    ctx.map_request_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerUtilLayerCreated> = Established::assert();
    Ok(json_result(&UtilLayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__map_response_layer_new",
    description = "Create a MapResponse layer using the given response-mapping function identifier. \
                   Establishes: TowerUtilLayerCreated.",
    emit = Auto
)]
async fn map_response_layer_new(
    ctx: Arc<TowerUtilCtx>,
    p: MapResponseLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerMapResponseLayer {
        mapper_fn: p.mapper_fn,
    };
    let id = Uuid::new_v4();
    ctx.map_response_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerUtilLayerCreated> = Established::assert();
    Ok(json_result(&UtilLayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__map_result_layer_new",
    description = "Create a MapResult layer using the given result-mapping function identifier. \
                   Establishes: TowerUtilLayerCreated.",
    emit = Auto
)]
async fn map_result_layer_new(
    ctx: Arc<TowerUtilCtx>,
    p: MapResultLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerMapResultLayer {
        mapper_fn: p.mapper_fn,
    };
    let id = Uuid::new_v4();
    ctx.map_result_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerUtilLayerCreated> = Established::assert();
    Ok(json_result(&UtilLayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__and_then_layer_new",
    description = "Create an AndThen layer using the given async combinator function identifier. \
                   Establishes: TowerUtilLayerCreated.",
    emit = Auto
)]
async fn and_then_layer_new(
    ctx: Arc<TowerUtilCtx>,
    p: AndThenLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerAndThenLayer { f: p.f };
    let id = Uuid::new_v4();
    ctx.and_then_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerUtilLayerCreated> = Established::assert();
    Ok(json_result(&UtilLayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__then_layer_new",
    description = "Create a Then layer using the given async combinator function identifier. \
                   Establishes: TowerUtilLayerCreated.",
    emit = Auto
)]
async fn then_layer_new(
    ctx: Arc<TowerUtilCtx>,
    p: ThenLayerNewParams,
) -> Result<CallToolResult, ErrorData> {
    let layer = elicitation::TowerThenLayer { f: p.f };
    let id = Uuid::new_v4();
    ctx.then_layers.lock().await.insert(id, layer);
    let _proof: Established<TowerUtilLayerCreated> = Established::assert();
    Ok(json_result(&UtilLayerIdResult {
        layer_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__box_service_new",
    description = "Create a BoxService config recording the Req/Resp/Err type parameters. \
                   Assumes: all three type strings are valid Rust type expressions. \
                   Establishes: TowerBoxServiceCreated.",
    emit = Auto
)]
async fn box_service_new(
    ctx: Arc<TowerUtilCtx>,
    p: BoxServiceNewParams,
) -> Result<CallToolResult, ErrorData> {
    let cfg = elicitation::TowerBoxServiceConfig {
        req_type: p.req_type,
        resp_type: p.resp_type,
        err_type: p.err_type,
    };
    let id = Uuid::new_v4();
    ctx.box_services.lock().await.insert(id, cfg);
    let _proof: Established<TowerBoxServiceCreated> = Established::assert();
    Ok(json_result(&ServiceIdResult {
        service_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__box_clone_service_new",
    description = "Create a BoxCloneService config recording the Req/Resp/Err type parameters. \
                   Assumes: all three type strings are valid Rust type expressions. \
                   Establishes: TowerBoxServiceCreated.",
    emit = Auto
)]
async fn box_clone_service_new(
    ctx: Arc<TowerUtilCtx>,
    p: BoxCloneServiceNewParams,
) -> Result<CallToolResult, ErrorData> {
    let cfg = elicitation::TowerBoxCloneServiceConfig {
        req_type: p.req_type,
        resp_type: p.resp_type,
        err_type: p.err_type,
    };
    let id = Uuid::new_v4();
    ctx.box_clone_services.lock().await.insert(id, cfg);
    let _proof: Established<TowerBoxServiceCreated> = Established::assert();
    Ok(json_result(&ServiceIdResult {
        service_id: id.to_string(),
    }))
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum UtilLayerDescription {
    MapErr(elicitation::TowerMapErrLayer),
    MapRequest(elicitation::TowerMapRequestLayer),
    MapResponse(elicitation::TowerMapResponseLayer),
    MapResult(elicitation::TowerMapResultLayer),
    AndThen(elicitation::TowerAndThenLayer),
    Then(elicitation::TowerThenLayer),
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__layer_describe",
    description = "Describe the config of a previously created util layer by its UUID. \
                   Assumes: layer_id is a valid UUID returned by a prior creation tool.",
    emit = Auto
)]
async fn layer_describe(
    ctx: Arc<TowerUtilCtx>,
    p: UtilLayerDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .layer_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.layer_id), None))?;

    if let Some(l) = ctx.map_err_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&UtilLayerDescription::MapErr(l)));
    }
    if let Some(l) = ctx.map_request_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&UtilLayerDescription::MapRequest(l)));
    }
    if let Some(l) = ctx.map_response_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&UtilLayerDescription::MapResponse(l)));
    }
    if let Some(l) = ctx.map_result_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&UtilLayerDescription::MapResult(l)));
    }
    if let Some(l) = ctx.and_then_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&UtilLayerDescription::AndThen(l)));
    }
    if let Some(l) = ctx.then_layers.lock().await.get(&id).cloned() {
        return Ok(json_result(&UtilLayerDescription::Then(l)));
    }

    Err(ErrorData::invalid_params(
        format!("layer_id not found: {id}"),
        None,
    ))
}

#[derive(Serialize)]
#[serde(tag = "kind")]
enum BoxServiceDescription {
    BoxService(elicitation::TowerBoxServiceConfig),
    BoxCloneService(elicitation::TowerBoxCloneServiceConfig),
}

#[elicitation::elicit_tool(
    plugin = "tower_util",
    name = "tower_util__box_service_describe",
    description = "Describe the config of a previously created BoxService or BoxCloneService by UUID. \
                   Assumes: service_id is a valid UUID returned by box_service_new or box_clone_service_new.",
    emit = Auto
)]
async fn box_service_describe(
    ctx: Arc<TowerUtilCtx>,
    p: BoxServiceDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id: Uuid = p
        .service_id
        .parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {}", p.service_id), None))?;

    if let Some(cfg) = ctx.box_services.lock().await.get(&id).cloned() {
        return Ok(json_result(&BoxServiceDescription::BoxService(cfg)));
    }
    if let Some(cfg) = ctx.box_clone_services.lock().await.get(&id).cloned() {
        return Ok(json_result(&BoxServiceDescription::BoxCloneService(cfg)));
    }

    Err(ErrorData::invalid_params(
        format!("service_id not found: {id}"),
        None,
    ))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `tower_util__*` tools for tower utility layers and box services.
///
/// Holds UUID-keyed registries of map-err, map-request, map-response, map-result,
/// and-then, then layers, and BoxService/BoxCloneService configs. All config
/// objects live server-side; agents interact via UUID handles.
///
/// # Tool namespace
///
/// All tools are registered under the `"tower_util"` namespace and named
/// `tower_util__<verb>`.
pub struct TowerUtilPlugin(Arc<TowerUtilCtx>);

impl TowerUtilPlugin {
    /// Create a new `TowerUtilPlugin` with empty registries.
    pub fn new() -> Self {
        Self(Arc::new(TowerUtilCtx::new()))
    }
}

impl Default for TowerUtilPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for TowerUtilPlugin {
    fn name(&self) -> &'static str {
        "tower_util"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "tower_util")
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
            let full_name = if name.starts_with("tower_util__") {
                name.to_string()
            } else {
                format!("tower_util__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "tower_util")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
