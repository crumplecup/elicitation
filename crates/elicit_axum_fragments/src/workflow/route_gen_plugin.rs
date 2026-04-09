//! AxumRouteGenPlugin — emit axum route and router fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// A route specification.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RouteSpec {
    /// HTTP path (e.g. `/users`).
    pub path: String,
    /// HTTP method lowercase (e.g. `get`, `post`).
    pub method: String,
    /// Handler function name.
    pub handler: String,
}

/// A method+handler pair for a chained route.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MethodHandler {
    /// HTTP method lowercase.
    pub method: String,
    /// Handler function name.
    pub handler: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_route_def.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitRouteParams {
    /// HTTP path (e.g. `/users`).
    pub path: String,
    /// HTTP method lowercase (e.g. `get`, `post`).
    pub method: String,
    /// Handler function name.
    pub handler_name: String,
}

/// Parameters for emit_routes_module.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitRoutesModuleParams {
    /// Route specifications to include in the router.
    pub routes: Vec<RouteSpec>,
}

/// Parameters for emit_nested_router.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitNestedRouterParams {
    /// URL prefix to nest under (e.g. `/api`).
    pub prefix: String,
    /// Module name providing the nested `routes()` function.
    pub module_name: String,
}

/// Parameters for emit_method_chain.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitMethodChainParams {
    /// HTTP path for the route.
    pub path: String,
    /// Method+handler pairs to chain.
    pub methods: Vec<MethodHandler>,
}

/// Parameters for emit_route_with_middleware.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitRouteMiddlewareParams {
    /// HTTP path for the route.
    pub path: String,
    /// HTTP method lowercase.
    pub method: String,
    /// Handler function name.
    pub handler: String,
    /// Middleware layer expressions to apply.
    pub middleware: Vec<String>,
}

/// Parameters for emit_protected_route.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitProtectedRouteParams {
    /// HTTP path for the route.
    pub path: String,
    /// HTTP method lowercase.
    pub method: String,
    /// Handler function name.
    pub handler: String,
    /// Auth middleware expression (e.g. `from_fn(auth_middleware)`).
    pub auth_middleware: String,
}

/// Parameters for emit_versioned_routes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitVersionedRoutesParams {
    /// Version strings (e.g. `["v1", "v2"]`).
    pub versions: Vec<String>,
    /// Routes to include under each version.
    pub routes: Vec<RouteSpec>,
}

/// Parameters for emit_crud_routes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCrudRoutesParams {
    /// Resource name in plural lowercase (e.g. `users`).
    pub resource: String,
    /// Application state type.
    pub state_type: String,
}

/// Parameters for emit_websocket_route.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitWsRouteParams {
    /// HTTP path for the WebSocket endpoint.
    pub path: String,
    /// Handler function name for the WebSocket upgrade.
    pub handler_name: String,
}

/// Parameters for emit_fallback_handler.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitFallbackParams {
    /// HTTP status code for the fallback response.
    pub status: u16,
    /// Response body text.
    pub body: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_route_def",
    description = "Emit a single `.route()` call for a path, method, and handler."
)]
#[instrument]
async fn emit_route_def(p: EmitRouteParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(".route(\"{}\", {}({}))", p.path, p.method, p.handler_name);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_routes_module",
    description = "Emit a complete `pub fn routes() -> Router` function with all provided routes."
)]
#[instrument]
async fn emit_routes_module(p: EmitRoutesModuleParams) -> Result<CallToolResult, ErrorData> {
    let route_lines = p
        .routes
        .iter()
        .map(|r| {
            format!(
                "        .route(\"{}\", {}({}))",
                r.path, r.method, r.handler
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "pub fn routes() -> Router {{\n    Router::new()\n{}\n}}",
        route_lines
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_nested_router",
    description = "Emit a `.nest()` call to mount a sub-router under a prefix."
)]
#[instrument]
async fn emit_nested_router(p: EmitNestedRouterParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(".nest(\"{}\", {}::routes())", p.prefix, p.module_name);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_method_chain",
    description = "Emit a `.route()` call with chained method handlers for a single path."
)]
#[instrument]
async fn emit_method_chain(p: EmitMethodChainParams) -> Result<CallToolResult, ErrorData> {
    if p.methods.is_empty() {
        return Ok(CallToolResult::success(vec![Content::text(format!(
            "// No methods provided for path {}",
            p.path
        ))]));
    }
    let chain = p
        .methods
        .iter()
        .map(|m| format!("{}({})", m.method, m.handler))
        .collect::<Vec<_>>()
        .join(".");
    let code = format!(".route(\"{}\", {})", p.path, chain);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_route_with_middleware",
    description = "Emit a route wrapped with one or more `.layer()` calls."
)]
#[instrument]
async fn emit_route_with_middleware(
    p: EmitRouteMiddlewareParams,
) -> Result<CallToolResult, ErrorData> {
    let layers = p
        .middleware
        .iter()
        .map(|m| format!("    .layer({})", m))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "Router::new()\n    .route(\"{}\", {}({}))\n{}",
        p.path, p.method, p.handler, layers
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_protected_route",
    description = "Emit a route protected by an authentication middleware layer."
)]
#[instrument]
async fn emit_protected_route(p: EmitProtectedRouteParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "Router::new()\n    .route(\"{}\", {}({}))\n    .route_layer(middleware::{})",
        p.path, p.method, p.handler, p.auth_middleware
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_versioned_routes",
    description = "Emit a router with the same routes nested under each API version prefix."
)]
#[instrument]
async fn emit_versioned_routes(p: EmitVersionedRoutesParams) -> Result<CallToolResult, ErrorData> {
    let mut code = String::new();
    code.push_str("pub fn versioned_routes() -> Router {\n    Router::new()\n");
    for version in &p.versions {
        let route_lines = p
            .routes
            .iter()
            .map(|r| {
                format!(
                    "            .route(\"{}\", {}({}))",
                    r.path, r.method, r.handler
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        code.push_str(&format!(
            "        .nest(\"/{}\", Router::new()\n{}\n        )\n",
            version, route_lines
        ));
    }
    code.push('}');
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_crud_routes",
    description = "Emit a full CRUD router function with list, create, get, update, and delete routes."
)]
#[instrument]
async fn emit_crud_routes(p: EmitCrudRoutesParams) -> Result<CallToolResult, ErrorData> {
    let resource = &p.resource;
    let state = &p.state_type;
    let singular = resource.trim_end_matches('s');
    let code = format!(
        r#"pub fn {resource}_routes() -> Router {{
    Router::new()
        .route("/{resource}", get(list_{resource}).post(create_{singular}))
        .route("/{resource}/:id", get(get_{singular}).put(update_{singular}).delete(delete_{singular}))
        .with_state({state}::new())
}}"#,
        resource = resource,
        singular = singular,
        state = state
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_websocket_route",
    description = "Emit a WebSocket route definition."
)]
#[instrument]
async fn emit_websocket_route(p: EmitWsRouteParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(".route(\"{}\", get({}))", p.path, p.handler_name);
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_route_gen",
    emit = None,
    name = "emit_fallback_handler",
    description = "Emit a fallback handler function returning a fixed status code and body."
)]
#[instrument]
async fn emit_fallback_handler(p: EmitFallbackParams) -> Result<CallToolResult, ErrorData> {
    let status_expr = match p.status {
        200 => "StatusCode::OK",
        201 => "StatusCode::CREATED",
        204 => "StatusCode::NO_CONTENT",
        400 => "StatusCode::BAD_REQUEST",
        401 => "StatusCode::UNAUTHORIZED",
        403 => "StatusCode::FORBIDDEN",
        404 => "StatusCode::NOT_FOUND",
        405 => "StatusCode::METHOD_NOT_ALLOWED",
        409 => "StatusCode::CONFLICT",
        422 => "StatusCode::UNPROCESSABLE_ENTITY",
        429 => "StatusCode::TOO_MANY_REQUESTS",
        500 => "StatusCode::INTERNAL_SERVER_ERROR",
        503 => "StatusCode::SERVICE_UNAVAILABLE",
        _ => "StatusCode::INTERNAL_SERVER_ERROR",
    };
    let code = format!(
        "pub async fn fallback_handler() -> (StatusCode, &'static str) {{\n    ({}, \"{}\")\n}}",
        status_expr, p.body
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

/// Plugin exposing axum route and router generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_route_gen")]
pub struct AxumRouteGenPlugin;
