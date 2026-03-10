//! `MethodPlugin` — MCP tools for every `reqwest::Method` method.
//!
//! Registered under the `"method"` namespace, producing tools:
//! `method__from_str`, `method__as_str`, `method__is_safe`, `method__is_idempotent`.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

/// Parameters for tools that operate on an HTTP method string.
#[derive(Debug, Deserialize, JsonSchema)]
struct MethodParams {
    /// HTTP method string (e.g. `"GET"`, `"POST"`). Case-insensitive.
    method: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MethodAsStrParams {
    method: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MethodIsSafeParams {
    method: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MethodIsIdempotentParams {
    method: String,
}

/// MCP plugin exposing all `reqwest::Method` methods as tools.
///
/// Register under the `"method"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::MethodPlugin;
///
/// let registry = PluginRegistry::new()
///     .register("method", MethodPlugin);
/// ```
#[derive(ElicitPlugin)]
#[plugin(name = "method")]
pub struct MethodPlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "method",
    name = "from_str",
    description = "Parse and validate an HTTP method string. Returns the normalized uppercase method and its properties (is_safe, is_idempotent), or an error for invalid input."
)]
#[instrument(skip_all, fields(method = %p.method))]
async fn method_from_str(p: MethodParams) -> Result<CallToolResult, ErrorData> {
    match reqwest::Method::from_bytes(p.method.as_bytes()) {
        Ok(m) => {
            let json = serde_json::json!({
                "method": m.as_str(),
                "is_safe": m.is_safe(),
                "is_idempotent": m.is_idempotent(),
            });
            Ok(CallToolResult::success(vec![Content::text(
                json.to_string(),
            )]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "method",
    name = "as_str",
    description = "Return the canonical uppercase string representation of the method (e.g. \"GET\")."
)]
#[instrument(skip_all, fields(method = %p.method))]
async fn method_as_str(p: MethodAsStrParams) -> Result<CallToolResult, ErrorData> {
    match reqwest::Method::from_bytes(p.method.as_bytes()) {
        Ok(m) => Ok(CallToolResult::success(vec![Content::text(m.as_str())])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "method",
    name = "is_safe",
    description = "Return true if the method is safe (has no intended side effects): GET, HEAD, OPTIONS, TRACE."
)]
#[instrument(skip_all, fields(method = %p.method))]
async fn method_is_safe(p: MethodIsSafeParams) -> Result<CallToolResult, ErrorData> {
    match reqwest::Method::from_bytes(p.method.as_bytes()) {
        Ok(m) => Ok(CallToolResult::success(vec![Content::text(
            m.is_safe().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "method",
    name = "is_idempotent",
    description = "Return true if the method is idempotent (repeated requests have the same effect): GET, HEAD, PUT, DELETE, OPTIONS, TRACE."
)]
#[instrument(skip_all, fields(method = %p.method))]
async fn method_is_idempotent(p: MethodIsIdempotentParams) -> Result<CallToolResult, ErrorData> {
    match reqwest::Method::from_bytes(p.method.as_bytes()) {
        Ok(m) => Ok(CallToolResult::success(vec![Content::text(
            m.is_idempotent().to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
