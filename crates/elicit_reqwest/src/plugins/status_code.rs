//! `StatusCodePlugin` — MCP tools for every `reqwest::StatusCode` method.
//!
//! Registered under the `"status_code"` namespace, producing tools:
//! `status_code__from_u16`, `status_code__as_str`, `status_code__canonical_reason`,
//! `status_code__is_informational`, `status_code__is_success`,
//! `status_code__is_redirection`, `status_code__is_client_error`,
//! `status_code__is_server_error`.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

/// Parameters for tools that operate on a status code.
#[derive(Debug, Deserialize, JsonSchema)]
struct StatusParams {
    /// HTTP status code (e.g. `200`, `404`).
    status: u16,
}

/// Parameters for tools that construct a status code.
#[derive(Debug, Deserialize, JsonSchema)]
struct FromU16Params {
    /// Integer status code to parse.
    code: u16,
}

/// MCP plugin exposing all `reqwest::StatusCode` methods as tools.
///
/// Register under the `"status_code"` namespace:
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_reqwest::plugins::StatusCodePlugin;
///
/// let registry = PluginRegistry::new()
///     .register("status_code", StatusCodePlugin);
/// ```
#[derive(ElicitPlugin)]
#[plugin(name = "status_code")]
pub struct StatusCodePlugin;

// ── Tool handlers ─────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "status_code",
    name = "from_u16",
    description = "Parse an integer into a status code; returns its string form, canonical reason, and class booleans.",
    emit = false
)]
#[instrument(skip_all, fields(code = p.code))]
async fn sc_from_u16(p: FromU16Params) -> Result<CallToolResult, ErrorData> {
    match reqwest::StatusCode::from_u16(p.code) {
        Ok(sc) => {
            let json = serde_json::json!({
                "code": sc.as_u16(),
                "str": sc.as_str(),
                "canonical_reason": sc.canonical_reason(),
                "is_informational": sc.is_informational(),
                "is_success": sc.is_success(),
                "is_redirection": sc.is_redirection(),
                "is_client_error": sc.is_client_error(),
                "is_server_error": sc.is_server_error(),
            });
            Ok(CallToolResult::success(vec![Content::text(
                json.to_string(),
            )]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "status_code",
    name = "as_str",
    description = "Return the three-digit ASCII representation of the status code (e.g. \"404\").",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_as_str(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    match reqwest::StatusCode::from_u16(p.status) {
        Ok(sc) => Ok(CallToolResult::success(vec![Content::text(sc.as_str())])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "status_code",
    name = "canonical_reason",
    description = "Return the canonical reason phrase for the status code (e.g. \"Not Found\"), or null if unknown.",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_canonical_reason(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    match reqwest::StatusCode::from_u16(p.status) {
        Ok(sc) => {
            let reason = sc.canonical_reason().unwrap_or("(unknown)");
            Ok(CallToolResult::success(vec![Content::text(reason)]))
        }
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}

#[elicit_tool(
    plugin = "status_code",
    name = "is_informational",
    description = "Return true if the status code is 1xx Informational.",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_is_informational(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    sc_bool(p, |sc| sc.is_informational())
}

#[elicit_tool(
    plugin = "status_code",
    name = "is_success",
    description = "Return true if the status code is 2xx Success.",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_is_success(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    sc_bool(p, |sc| sc.is_success())
}

#[elicit_tool(
    plugin = "status_code",
    name = "is_redirection",
    description = "Return true if the status code is 3xx Redirection.",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_is_redirection(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    sc_bool(p, |sc| sc.is_redirection())
}

#[elicit_tool(
    plugin = "status_code",
    name = "is_client_error",
    description = "Return true if the status code is 4xx Client Error.",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_is_client_error(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    sc_bool(p, |sc| sc.is_client_error())
}

#[elicit_tool(
    plugin = "status_code",
    name = "is_server_error",
    description = "Return true if the status code is 5xx Server Error.",
    emit = false
)]
#[instrument(skip_all, fields(status = p.status))]
async fn sc_is_server_error(p: StatusParams) -> Result<CallToolResult, ErrorData> {
    sc_bool(p, |sc| sc.is_server_error())
}

fn sc_bool(
    p: StatusParams,
    f: impl Fn(reqwest::StatusCode) -> bool,
) -> Result<CallToolResult, ErrorData> {
    match reqwest::StatusCode::from_u16(p.status) {
        Ok(sc) => Ok(CallToolResult::success(vec![Content::text(
            f(sc).to_string(),
        )])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
