//! `AxumRouterPlugin` — MCP tools for axum Router construction and configuration.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Shared types ──────────────────────────────────────────────────────────────

/// Describes a single route entry on a router.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RouteEntry {
    /// URL path pattern.
    pub path: String,
    /// HTTP methods handled.
    pub methods: Vec<String>,
    /// Description of the handler.
    pub handler_description: String,
}

/// Descriptor representing an axum Router configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RouterDescriptor {
    /// Unique ID for this router.
    pub router_id: String,
    /// Registered routes.
    pub routes: Vec<RouteEntry>,
    /// Applied middleware layer descriptions.
    pub middleware_layers: Vec<String>,
    /// Optional state type name.
    pub state_type: Option<String>,
    /// Optional fallback handler description.
    pub fallback: Option<String>,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for router_new.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterNewParams {}

/// Parameters for router_route.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterRouteParams {
    /// The router to add the route to.
    pub router: RouterDescriptor,
    /// URL path pattern for the route.
    pub path: String,
    /// HTTP methods handled by this route.
    pub methods: Vec<String>,
    /// Description of the handler.
    pub handler_description: String,
}

/// Parameters for router_route_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterRouteServiceParams {
    /// The router to add the service route to.
    pub router: RouterDescriptor,
    /// URL path pattern for the route.
    pub path: String,
    /// Description of the service.
    pub service_description: String,
}

/// Parameters for router_nest.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterNestParams {
    /// The outer router.
    pub router: RouterDescriptor,
    /// Path prefix to nest under.
    pub path: String,
    /// The router to nest.
    pub nested_router: RouterDescriptor,
}

/// Parameters for router_nest_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterNestServiceParams {
    /// The router to add the nested service to.
    pub router: RouterDescriptor,
    /// Path prefix for the nested service.
    pub path: String,
    /// Description of the service.
    pub service_description: String,
}

/// Parameters for router_merge.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterMergeParams {
    /// The primary router.
    pub router: RouterDescriptor,
    /// The router whose routes are merged in.
    pub other: RouterDescriptor,
}

/// Parameters for router_layer.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterLayerParams {
    /// The router to add the layer to.
    pub router: RouterDescriptor,
    /// Description of the middleware layer.
    pub layer_description: String,
}

/// Parameters for router_route_layer.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterRouteLayerParams {
    /// The router to add the route-only layer to.
    pub router: RouterDescriptor,
    /// Description of the middleware layer.
    pub layer_description: String,
}

/// Parameters for router_with_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterWithStateParams {
    /// The router to attach state to.
    pub router: RouterDescriptor,
    /// The Rust type name of the state.
    pub state_type: String,
    /// JSON-encoded state value.
    pub state_json: String,
}

/// Parameters for router_fallback.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterFallbackParams {
    /// The router to set the fallback on.
    pub router: RouterDescriptor,
    /// Description of the fallback handler.
    pub handler_description: String,
}

/// Parameters for router_fallback_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterFallbackServiceParams {
    /// The router to set the fallback service on.
    pub router: RouterDescriptor,
    /// Description of the fallback service.
    pub service_description: String,
}

/// Parameters for router_into_make_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterDescriptorInput {
    /// The router descriptor to convert.
    pub router: RouterDescriptor,
}

/// Parameters for router_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouterDescribeInput {
    /// The router descriptor to describe.
    pub router: RouterDescriptor,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_router",
    name = "router_new",
    emit = None,
    description = "Create a new empty axum Router descriptor with a generated UUID router_id."
)]
#[instrument]
async fn router_new(_p: RouterNewParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = RouterDescriptor {
        router_id: uuid::Uuid::new_v4().to_string(),
        routes: vec![],
        middleware_layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_route",
    emit = None,
    description = "Add a route to an axum Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_route(p: RouterRouteParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.routes.push(RouteEntry {
        path: p.path,
        methods: p.methods,
        handler_description: p.handler_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_route_service",
    emit = None,
    description = "Add a service route to an axum Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_route_service(p: RouterRouteServiceParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.routes.push(RouteEntry {
        path: p.path,
        methods: vec!["SERVICE".to_string()],
        handler_description: p.service_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_nest",
    emit = None,
    description = "Nest another Router under a path prefix. All nested routes are prefixed. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_nest(p: RouterNestParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    for nested_route in p.nested_router.routes {
        router.routes.push(RouteEntry {
            path: format!("{}{}", p.path, nested_route.path),
            methods: nested_route.methods,
            handler_description: nested_route.handler_description,
        });
    }
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_nest_service",
    emit = None,
    description = "Nest a service under a path prefix on a Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_nest_service(p: RouterNestServiceParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.routes.push(RouteEntry {
        path: p.path,
        methods: vec!["SERVICE".to_string()],
        handler_description: format!("Nested service: {}", p.service_description),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_merge",
    emit = None,
    description = "Merge two Router descriptors into one. Routes from both are combined under the primary router_id."
)]
#[instrument]
async fn router_merge(p: RouterMergeParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.routes.extend(p.other.routes);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_layer",
    emit = None,
    description = "Apply a middleware layer to all routes on a Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_layer(p: RouterLayerParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.middleware_layers.push(p.layer_description);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_route_layer",
    emit = None,
    description = "Apply a route-only middleware layer to a Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_route_layer(p: RouterRouteLayerParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router
        .middleware_layers
        .push(format!("route-only: {}", p.layer_description));
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_with_state",
    emit = None,
    description = "Attach application state to a Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_with_state(p: RouterWithStateParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.state_type = Some(format!("{} (value: {})", p.state_type, p.state_json));
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_fallback",
    emit = None,
    description = "Set a fallback handler on a Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_fallback(p: RouterFallbackParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.fallback = Some(p.handler_description);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_fallback_service",
    emit = None,
    description = "Set a fallback service on a Router descriptor. Returns the updated RouterDescriptor."
)]
#[instrument]
async fn router_fallback_service(
    p: RouterFallbackServiceParams,
) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.fallback = Some(format!("Service: {}", p.service_description));
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_into_make_service",
    emit = None,
    description = "Describe converting a Router descriptor into an IntoMakeService for serving."
)]
#[instrument]
async fn router_into_make_service(p: RouterDescriptorInput) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "IntoMakeService wrapping router '{}' with {} routes",
        p.router.router_id,
        p.router.routes.len()
    ))]))
}

#[elicit_tool(
    plugin = "axum_router",
    name = "router_describe",
    emit = None,
    description = "Return a human-readable description of a Router descriptor."
)]
#[instrument]
async fn router_describe(p: RouterDescribeInput) -> Result<CallToolResult, ErrorData> {
    let r = &p.router;
    let mut lines = vec![
        format!("Router ID: {}", r.router_id),
        format!("Routes ({}):", r.routes.len()),
    ];
    for route in &r.routes {
        lines.push(format!(
            "  {} {} → {}",
            route.methods.join("|"),
            route.path,
            route.handler_description
        ));
    }
    let layers_str = if r.middleware_layers.is_empty() {
        "none".to_string()
    } else {
        r.middleware_layers.join(", ")
    };
    lines.push(format!(
        "Middleware layers ({}): {}",
        r.middleware_layers.len(),
        layers_str
    ));
    lines.push(format!(
        "State: {}",
        r.state_type.as_deref().unwrap_or("none")
    ));
    lines.push(format!(
        "Fallback: {}",
        r.fallback.as_deref().unwrap_or("none")
    ));
    Ok(CallToolResult::success(vec![Content::text(
        lines.join("\n"),
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum Router construction and configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_router")]
pub struct AxumRouterPlugin;
