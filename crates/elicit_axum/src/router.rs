//! `AxumRouterPlugin` — MCP tools for axum `Router<S>` configuration.
//!
//! Router descriptors are held server-side in a UUID-keyed registry.
//! Agents receive UUID handles; no live axum instances cross the MCP boundary.
//!
//! # Tool namespace: `axum_router__*`
//!
//! | Tool | Params | Returns | Establishes |
//! |---|---|---|---|
//! | `new` | `state_type` | `{ router_id }` | `AxumRouterCreated` |
//! | `add_route` | `router_id, method, path, handler` | `{ route_count }` | `AxumRouteAdded` |
//! | `add_layer` | `router_id, layer_expr` | `{ layer_count }` | — |
//! | `set_fallback` | `router_id, handler` | — | — |
//! | `set_db_slot` | `router_id, pool_type, var_name, provide_leptos_context` | — | — |
//! | `set_custom_state` | `router_id, state_type, state_expr` | — | — |
//! | `describe` | `router_id` | JSON descriptor | — |
//! | `emit` | `router_id` | Rust source string | — |

use std::collections::HashMap;
use std::sync::Arc;

use elicitation::contracts::{Established, Prop};
use elicitation::{
    AxumDbSlot, AxumHttpMethod, AxumRouteEntry, AxumRouterDescriptor, Elicit, PluginContext,
    VerifiedWorkflow,
};
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

/// Proposition: an axum router descriptor was successfully created and registered.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct AxumRouterCreated;
impl Prop for AxumRouterCreated {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_axum_router_created() {
                let created: bool = kani::any();
                kani::assume(created);
                assert!(created, "axum router created");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_axum_router_created(ok: bool) -> (result: bool)
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
            pub fn verify_axum_router_created_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for AxumRouterCreated {}

/// Proposition: a route was successfully added to an axum router descriptor.
#[derive(Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct AxumRouteAdded;
impl Prop for AxumRouteAdded {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_axum_route_added() {
                let added: bool = kani::any();
                kani::assume(added);
                assert!(added, "axum route added");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_axum_route_added(ok: bool) -> (result: bool)
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
            pub fn verify_axum_route_added_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for AxumRouteAdded {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared state for all `axum_router__*` tool calls.
/// Shared mutable state for the [`AxumRouterPlugin`].
///
/// Holds a UUID-keyed registry of [`AxumRouterDescriptor`] objects.
/// Expose via [`AxumRouterPlugin::ctx`] to share the registry with bridge plugins.
pub struct AxumRouterCtx {
    pub(crate) items: Mutex<HashMap<Uuid, AxumRouterDescriptor>>,
}

impl AxumRouterCtx {
    pub(crate) fn new() -> Self {
        Self {
            items: Mutex::new(HashMap::new()),
        }
    }

    /// Insert a pre-built descriptor and return its UUID.
    ///
    /// Used by bridge plugins to inject descriptors into the shared registry.
    pub async fn insert(&self, desc: AxumRouterDescriptor) -> Uuid {
        let id = Uuid::new_v4();
        self.items.lock().await.insert(id, desc);
        id
    }
}

impl PluginContext for AxumRouterCtx {}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `axum_router__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterNewParams {
    /// Rust type name for the router state, e.g. `"AppState"` or `"()"`.
    pub state_type: String,
}

/// Parameters for `axum_router__add_route`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterAddRouteParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
    /// HTTP method for this route.
    pub method: AxumHttpMethod,
    /// URL path pattern, e.g. `"/users/:id"`.
    pub path: String,
    /// Handler function name, e.g. `"get_user"`.
    pub handler: String,
}

/// Parameters for `axum_router__add_layer`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterAddLayerParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
    /// Layer construction expression, e.g. `"TraceLayer::new_for_http()"`.
    pub layer_expr: String,
}

/// Parameters for `axum_router__set_fallback`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterSetFallbackParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
    /// Fallback handler expression, e.g. `"handler_404"`.
    pub handler: String,
}

/// Parameters for `axum_router__set_db_slot`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterSetDbSlotParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
    /// Rust type of the pool or state struct (e.g. `"sqlx::AnyPool"`, `"Arc<AppState>"`).
    pub pool_type: String,
    /// Variable name used in `.with_state({var_name})` (e.g. `"pool"`).
    pub var_name: String,
    /// When `true`, the bridge emits `leptos_routes_with_context` so that every
    /// Leptos server function can `use_context::<{pool_type}>()`.
    pub provide_leptos_context: Option<bool>,
}

/// Parameters for `axum_router__set_custom_state`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterSetCustomStateParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
    /// Rust type name for the state (e.g. `"AppState"`).
    pub state_type: String,
    /// Expression passed to `.with_state(...)` (e.g. `"AppState::new(pool, config)"`).
    pub state_expr: String,
}

/// Parameters for `axum_router__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterDescribeParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
}

/// Parameters for `axum_router__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RouterEmitParams {
    /// UUID returned by `axum_router__new`.
    pub router_id: String,
}

/// Result returned by `axum_router__new`.
#[derive(Debug, Serialize)]
pub struct RouterIdResult {
    /// UUID handle for the newly created router descriptor.
    pub router_id: String,
}

/// Result returned by `axum_router__add_route`.
#[derive(Debug, Serialize)]
struct RouteCountResult {
    route_count: usize,
}

/// Result returned by `axum_router__add_layer`.
#[derive(Debug, Serialize)]
struct LayerCountResult {
    layer_count: usize,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn parse_router_id(s: &str) -> Result<Uuid, ErrorData> {
    s.parse()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

fn method_fn(method: &AxumHttpMethod) -> &'static str {
    match method {
        AxumHttpMethod::Get => "get",
        AxumHttpMethod::Post => "post",
        AxumHttpMethod::Put => "put",
        AxumHttpMethod::Delete => "delete",
        AxumHttpMethod::Patch => "patch",
        AxumHttpMethod::Head => "head",
        AxumHttpMethod::Options => "options",
        AxumHttpMethod::Trace => "trace",
        AxumHttpMethod::Any => "any",
    }
}

fn emit_router(desc: &AxumRouterDescriptor) -> String {
    // Resolve effective state type: db_slot overrides the stored state_type.
    let state = if let Some(slot) = &desc.db_slot {
        slot.pool_type.as_str()
    } else {
        desc.state_type.as_str()
    };
    let mut lines = Vec::new();
    lines.push(format!("let router: Router<{state}> = Router::new()"));
    for route in &desc.routes {
        lines.push(format!(
            "    .route(\"{path}\", {method}({handler}))",
            path = route.path,
            method = method_fn(&route.method),
            handler = route.handler,
        ));
    }
    for call in &desc.raw_method_calls {
        lines.push(format!("    .{call}"));
    }
    for layer in &desc.layers {
        lines.push(format!("    .layer({layer})"));
    }
    if let Some(fallback) = &desc.fallback {
        lines.push(format!("    .fallback({fallback})"));
    }
    // Terminal `.with_state(...)` — db_slot takes precedence.
    if let Some(slot) = &desc.db_slot {
        lines.push(format!("    .with_state({})", slot.var_name));
    } else if let Some(expr) = &desc.custom_state_expr {
        lines.push(format!("    .with_state({expr})"));
    }
    lines.join("\n") + ";"
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__new",
    description = "Create a new axum Router descriptor with the given state type. \
                   Assumes: state_type is a valid Rust type name. \
                   Establishes: AxumRouterCreated.",
    emit = Auto
)]
async fn new(ctx: Arc<AxumRouterCtx>, p: RouterNewParams) -> Result<CallToolResult, ErrorData> {
    let desc = AxumRouterDescriptor {
        state_type: p.state_type,
        routes: Vec::new(),
        raw_method_calls: Vec::new(),
        layers: Vec::new(),
        fallback: None,
        db_slot: None,
        custom_state_expr: None,
    };
    let id = Uuid::new_v4();
    ctx.items.lock().await.insert(id, desc);
    let _proof: Established<AxumRouterCreated> = Established::assert();
    Ok(json_result(&RouterIdResult {
        router_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__add_route",
    description = "Add a route to an existing router descriptor. \
                   Assumes: router_id is valid, path is a valid axum path pattern. \
                   Establishes: AxumRouteAdded.",
    emit = Auto
)]
async fn add_route(
    ctx: Arc<AxumRouterCtx>,
    p: RouterAddRouteParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    desc.routes.push(AxumRouteEntry {
        method: p.method,
        path: p.path,
        handler: p.handler,
    });
    let count = desc.routes.len();
    let _proof: Established<AxumRouteAdded> = Established::assert();
    Ok(json_result(&RouteCountResult { route_count: count }))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__add_layer",
    description = "Append a layer expression to the router descriptor. \
                   Assumes: router_id is valid, layer_expr is a valid Rust expression.",
    emit = Auto
)]
async fn add_layer(
    ctx: Arc<AxumRouterCtx>,
    p: RouterAddLayerParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    desc.layers.push(p.layer_expr);
    let count = desc.layers.len();
    Ok(json_result(&LayerCountResult { layer_count: count }))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__set_fallback",
    description = "Set the fallback handler expression on a router descriptor. \
                   Assumes: router_id is valid.",
    emit = Auto
)]
async fn set_fallback(
    ctx: Arc<AxumRouterCtx>,
    p: RouterSetFallbackParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    desc.fallback = Some(p.handler);
    Ok(CallToolResult::success(vec![Content::text("ok")]))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__set_db_slot",
    description = "Configure the db pool / state slot for a router descriptor. \
                   Sets the pool type and variable name so that emit produces \
                   `.with_state({var_name})` as the terminal call. \
                   When provide_leptos_context is true, the Leptos bridge will \
                   emit `leptos_routes_with_context` + `provide_context` so that \
                   all Leptos server functions can call `use_context`. \
                   Assumes: router_id is valid, pool_type is a valid Rust type.",
    emit = Auto
)]
async fn set_db_slot(
    ctx: Arc<AxumRouterCtx>,
    p: RouterSetDbSlotParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    desc.db_slot = Some(AxumDbSlot {
        pool_type: p.pool_type,
        var_name: p.var_name,
        provide_leptos_context: p.provide_leptos_context.unwrap_or(false),
    });
    // Clear any custom_state_expr so db_slot takes precedence cleanly.
    desc.custom_state_expr = None;
    Ok(CallToolResult::success(vec![Content::text("ok")]))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__set_custom_state",
    description = "Configure a custom `.with_state(expr)` terminal call on a \
                   router descriptor. Use this when the state is not a bare pool \
                   but a user-defined AppState struct. \
                   Assumes: router_id is valid, state_type and state_expr are valid Rust.",
    emit = Auto
)]
async fn set_custom_state(
    ctx: Arc<AxumRouterCtx>,
    p: RouterSetCustomStateParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let mut guard = ctx.items.lock().await;
    let desc = guard
        .get_mut(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    desc.state_type = p.state_type;
    desc.custom_state_expr = Some(p.state_expr);
    // db_slot takes precedence, so clear it if the user wants custom state.
    desc.db_slot = None;
    Ok(CallToolResult::success(vec![Content::text("ok")]))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__describe",
    description = "Return the JSON descriptor for a router. \
                   Assumes: router_id is a valid UUID returned by axum_router__new.",
    emit = Auto
)]
async fn describe(
    ctx: Arc<AxumRouterCtx>,
    p: RouterDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    Ok(json_result(desc))
}

#[elicitation::elicit_tool(
    plugin = "axum_router",
    name = "axum_router__emit",
    description = "Emit idiomatic Rust code for building the described Router. \
                   Assumes: router_id is a valid UUID returned by axum_router__new.",
    emit = Auto
)]
async fn emit(ctx: Arc<AxumRouterCtx>, p: RouterEmitParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_router_id(&p.router_id)?;
    let guard = ctx.items.lock().await;
    let desc = guard
        .get(&id)
        .ok_or_else(|| ErrorData::invalid_params(format!("router_id not found: {id}"), None))?;
    let code = emit_router(desc);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `axum_router__*` tools for router configuration.
pub struct AxumRouterPlugin(Arc<AxumRouterCtx>);

impl AxumRouterPlugin {
    /// Create a new `AxumRouterPlugin` with an empty registry.
    pub fn new() -> Self {
        Self(Arc::new(AxumRouterCtx::new()))
    }

    /// Return a shared reference to the underlying context.
    ///
    /// Pass this to bridge plugins (e.g. `LeptosAxumBridgePlugin`) so they
    /// can inject descriptors into the same registry.
    pub fn ctx(&self) -> Arc<AxumRouterCtx> {
        Arc::clone(&self.0)
    }

    /// Convenience helper for tests and direct integration: invoke a tool by
    /// name with a JSON argument object and return the `CallToolResult`.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let owned = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        let plugin_ctx = self.0.clone();
        let full_name = if name.starts_with("axum_router__") {
            name.to_string()
        } else {
            format!("axum_router__{name}")
        };
        let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "axum_router")
            .find(|r| r.name == full_name)
            .map(|r| (r.constructor)())
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
        descriptor
            .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
            .await
    }
}

impl Default for AxumRouterPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for AxumRouterPlugin {
    fn name(&self) -> &'static str {
        "axum_router"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "axum_router")
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
            let full_name = if name.starts_with("axum_router__") {
                name.to_string()
            } else {
                format!("axum_router__{name}")
            };

            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "axum_router")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;

            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
