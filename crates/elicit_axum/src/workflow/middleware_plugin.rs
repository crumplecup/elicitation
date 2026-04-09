//! `AxumMiddlewarePlugin` — MCP tools for axum middleware configuration.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes an axum middleware configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareDescriptor {
    /// Middleware name or function name.
    pub name: String,
    /// Middleware kind (e.g. "from_fn", "from_extractor").
    pub kind: String,
    /// Human-readable description.
    pub description: String,
    /// Optional state type for stateful middleware.
    pub state_type: Option<String>,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for middleware_from_fn_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareFnParams {
    /// Rust function signature string for the middleware function.
    pub fn_signature: String,
}

/// Parameters for middleware_from_fn_with_state_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareFnWithStateParams {
    /// Rust function signature string for the middleware function.
    pub fn_signature: String,
    /// State type injected into the middleware.
    pub state_type: String,
}

/// Parameters for middleware_from_extractor_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareExtractorParams {
    /// The extractor type used as middleware.
    pub extractor_type: String,
}

/// Parameters for middleware_map_request and middleware_map_response.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareMapParams {
    /// Description of the mapping transformation.
    pub mapper_description: String,
}

/// Parameters for middleware_add_extension.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareAddExtParams {
    /// Type inserted into request extensions.
    pub extension_type: String,
    /// Description of the value being inserted.
    pub value_description: String,
}

/// Parameters for middleware_set_header.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MiddlewareSetHeaderParams {
    /// Header name to set.
    pub header_name: String,
    /// Header value to set.
    pub header_value: String,
}

/// Parameters for next_describe (no inputs needed).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsNext {}

/// Parameters for next_run_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NextRunParams {
    /// Description of the request being passed to the next middleware.
    pub request_description: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn fn_name_from_sig(sig: &str) -> String {
    sig.find('(')
        .map(|i| sig[..i].trim().to_string())
        .unwrap_or_else(|| sig.trim().to_string())
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_from_fn_describe",
    emit = None,
    description = "Describe a middleware::from_fn() created from an async function signature."
)]
#[instrument]
async fn middleware_from_fn_describe(p: MiddlewareFnParams) -> Result<CallToolResult, ErrorData> {
    let name = fn_name_from_sig(&p.fn_signature);
    let descriptor = MiddlewareDescriptor {
        name,
        kind: "from_fn".to_string(),
        description: format!(
            "middleware::from_fn({}) — wraps an async function as tower middleware. The function \
             receives (Request, Next) and returns Response.",
            p.fn_signature
        ),
        state_type: None,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_from_fn_with_state_describe",
    emit = None,
    description = "Describe a middleware::from_fn_with_state() created from a function and state type."
)]
#[instrument]
async fn middleware_from_fn_with_state_describe(
    p: MiddlewareFnWithStateParams,
) -> Result<CallToolResult, ErrorData> {
    let name = fn_name_from_sig(&p.fn_signature);
    let descriptor = MiddlewareDescriptor {
        name,
        kind: "from_fn_with_state".to_string(),
        description: format!(
            "middleware::from_fn_with_state({}, {}) — wraps an async function as tower middleware \
             with access to shared state.",
            p.fn_signature, p.state_type
        ),
        state_type: Some(p.state_type),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_from_extractor_describe",
    emit = None,
    description = "Describe a middleware::from_extractor() created from an extractor type."
)]
#[instrument]
async fn middleware_from_extractor_describe(
    p: MiddlewareExtractorParams,
) -> Result<CallToolResult, ErrorData> {
    let descriptor = MiddlewareDescriptor {
        name: p.extractor_type.clone(),
        kind: "from_extractor".to_string(),
        description: format!(
            "middleware::from_extractor::<{}>() — uses a type that implements FromRequestParts as \
             middleware. If extraction fails, the request is rejected.",
            p.extractor_type
        ),
        state_type: None,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_map_request",
    emit = None,
    description = "Describe a map_request middleware that transforms incoming requests."
)]
#[instrument]
async fn middleware_map_request(p: MiddlewareMapParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "map_request(|req| async {{ {} }}) — transforms the request before passing to the inner \
         service",
        p.mapper_description
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_map_response",
    emit = None,
    description = "Describe a map_response middleware that transforms outgoing responses."
)]
#[instrument]
async fn middleware_map_response(p: MiddlewareMapParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "map_response(|res| async {{ {} }}) — transforms the response from the inner service",
        p.mapper_description
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_add_extension",
    emit = None,
    description = "Describe AddExtension middleware that inserts a value into request extensions."
)]
#[instrument]
async fn middleware_add_extension(p: MiddlewareAddExtParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "AddExtension<{}>({}) — inserts a value into request extensions, accessible via \
         Extension<{}> extractor",
        p.extension_type, p.value_description, p.extension_type
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "middleware_set_header",
    emit = None,
    description = "Describe SetRequestHeader middleware that sets a request header if not present."
)]
#[instrument]
async fn middleware_set_header(p: MiddlewareSetHeaderParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "SetRequestHeader(header::{}): '{}' — sets a request header if not present",
        p.header_name, p.header_value
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "next_describe",
    emit = None,
    description = "Describe the Next<B> type that represents the remaining middleware chain."
)]
#[instrument]
async fn next_describe(_p: NoParamsNext) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "Next<B> represents the remaining middleware chain. Call next.run(request) to pass the \
         request to the next service in the chain.",
    )]))
}

#[elicit_tool(
    plugin = "axum_middleware",
    name = "next_run_describe",
    emit = None,
    description = "Describe calling next.run(request) to continue the middleware chain."
)]
#[instrument]
async fn next_run_describe(p: NextRunParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "next.run(request) with '{}' — passes the (possibly modified) request to the next \
         middleware or final handler, returning the Response",
        p.request_description
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

/// Plugin exposing axum middleware configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_middleware")]
pub struct AxumMiddlewarePlugin;
