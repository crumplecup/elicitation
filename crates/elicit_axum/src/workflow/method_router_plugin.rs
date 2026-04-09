//! `AxumMethodRouterPlugin` — MCP tools for axum MethodRouter configuration.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Shared types ──────────────────────────────────────────────────────────────

/// Descriptor representing an axum MethodRouter configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterDescriptor {
    /// HTTP methods handled.
    pub methods: Vec<String>,
    /// Handler or service description.
    pub handler_description: String,
    /// Applied middleware layer descriptions.
    pub layers: Vec<String>,
    /// Optional state type name.
    pub state_type: Option<String>,
    /// Optional fallback description.
    pub fallback: Option<String>,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for single-handler method router constructors (get, post, put, etc.).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HandlerDescParam {
    /// Description of the handler.
    pub handler_description: String,
}

/// Parameters for method_router_on.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterOnParams {
    /// HTTP methods to handle.
    pub methods: Vec<String>,
    /// Description of the handler.
    pub handler_description: String,
}

/// Parameters for method_router_on_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterOnServiceParams {
    /// HTTP methods to handle.
    pub methods: Vec<String>,
    /// Description of the service.
    pub service_description: String,
}

/// Parameters for method_router_layer.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterLayerParams {
    /// The method router to add the layer to.
    pub router: MethodRouterDescriptor,
    /// Description of the middleware layer.
    pub layer_description: String,
}

/// Parameters for method_router_route_layer.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterRouteLayerParams {
    /// The method router to add the route-only layer to.
    pub router: MethodRouterDescriptor,
    /// Description of the middleware layer.
    pub layer_description: String,
}

/// Parameters for method_router_fallback.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterFallbackParams {
    /// The method router to set the fallback on.
    pub router: MethodRouterDescriptor,
    /// Description of the fallback handler.
    pub handler_description: String,
}

/// Parameters for method_router_fallback_service.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterFallbackServiceParams {
    /// The method router to set the fallback service on.
    pub router: MethodRouterDescriptor,
    /// Description of the fallback service.
    pub service_description: String,
}

/// Parameters for method_router_with_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterWithStateParams {
    /// The method router to attach state to.
    pub router: MethodRouterDescriptor,
    /// The Rust type name of the state.
    pub state_type: String,
}

/// Parameters for method_router_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodRouterDescribeInput {
    /// The method router descriptor to describe.
    pub router: MethodRouterDescriptor,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_get",
    emit = None,
    description = "Create a MethodRouter descriptor for GET requests."
)]
#[instrument]
async fn method_router_get(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["GET".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_post",
    emit = None,
    description = "Create a MethodRouter descriptor for POST requests."
)]
#[instrument]
async fn method_router_post(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["POST".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_put",
    emit = None,
    description = "Create a MethodRouter descriptor for PUT requests."
)]
#[instrument]
async fn method_router_put(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["PUT".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_delete",
    emit = None,
    description = "Create a MethodRouter descriptor for DELETE requests."
)]
#[instrument]
async fn method_router_delete(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["DELETE".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_patch",
    emit = None,
    description = "Create a MethodRouter descriptor for PATCH requests."
)]
#[instrument]
async fn method_router_patch(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["PATCH".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_head",
    emit = None,
    description = "Create a MethodRouter descriptor for HEAD requests."
)]
#[instrument]
async fn method_router_head(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["HEAD".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_options",
    emit = None,
    description = "Create a MethodRouter descriptor for OPTIONS requests."
)]
#[instrument]
async fn method_router_options(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["OPTIONS".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_trace",
    emit = None,
    description = "Create a MethodRouter descriptor for TRACE requests."
)]
#[instrument]
async fn method_router_trace(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["TRACE".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_any",
    emit = None,
    description = "Create a MethodRouter descriptor that handles all HTTP methods."
)]
#[instrument]
async fn method_router_any(p: HandlerDescParam) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: vec!["*".to_string()],
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_on",
    emit = None,
    description = "Create a MethodRouter descriptor for a specific set of HTTP methods."
)]
#[instrument]
async fn method_router_on(p: MethodRouterOnParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: p.methods,
        handler_description: p.handler_description,
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_on_service",
    emit = None,
    description = "Create a MethodRouter descriptor for a service on specific HTTP methods."
)]
#[instrument]
async fn method_router_on_service(
    p: MethodRouterOnServiceParams,
) -> Result<CallToolResult, ErrorData> {
    let descriptor = MethodRouterDescriptor {
        methods: p.methods,
        handler_description: format!("Service: {}", p.service_description),
        layers: vec![],
        state_type: None,
        fallback: None,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_layer",
    emit = None,
    description = "Apply a middleware layer to all routes on a MethodRouter descriptor. Returns the updated descriptor."
)]
#[instrument]
async fn method_router_layer(p: MethodRouterLayerParams) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.layers.push(p.layer_description);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_route_layer",
    emit = None,
    description = "Apply a route-only middleware layer to a MethodRouter descriptor. Returns the updated descriptor."
)]
#[instrument]
async fn method_router_route_layer(
    p: MethodRouterRouteLayerParams,
) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router
        .layers
        .push(format!("route-only: {}", p.layer_description));
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_fallback",
    emit = None,
    description = "Set a fallback handler on a MethodRouter descriptor. Returns the updated descriptor."
)]
#[instrument]
async fn method_router_fallback(
    p: MethodRouterFallbackParams,
) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.fallback = Some(p.handler_description);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_fallback_service",
    emit = None,
    description = "Set a fallback service on a MethodRouter descriptor. Returns the updated descriptor."
)]
#[instrument]
async fn method_router_fallback_service(
    p: MethodRouterFallbackServiceParams,
) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.fallback = Some(format!("Service: {}", p.service_description));
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_with_state",
    emit = None,
    description = "Attach application state to a MethodRouter descriptor. Returns the updated descriptor."
)]
#[instrument]
async fn method_router_with_state(
    p: MethodRouterWithStateParams,
) -> Result<CallToolResult, ErrorData> {
    let mut router = p.router;
    router.state_type = Some(p.state_type);
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&router).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_method_router",
    name = "method_router_describe",
    emit = None,
    description = "Return a human-readable description of a MethodRouter descriptor."
)]
#[instrument]
async fn method_router_describe(p: MethodRouterDescribeInput) -> Result<CallToolResult, ErrorData> {
    let r = &p.router;
    let methods_str = r.methods.join(", ");
    let layers_str = if r.layers.is_empty() {
        "none".to_string()
    } else {
        r.layers.join(", ")
    };
    let description = format!(
        "Methods: {}\nHandler: {}\nLayers ({}): {}\nState: {}\nFallback: {}",
        methods_str,
        r.handler_description,
        r.layers.len(),
        layers_str,
        r.state_type.as_deref().unwrap_or("none"),
        r.fallback.as_deref().unwrap_or("none"),
    );
    Ok(CallToolResult::success(vec![Content::text(description)]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum MethodRouter configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_method_router")]
pub struct AxumMethodRouterPlugin;
