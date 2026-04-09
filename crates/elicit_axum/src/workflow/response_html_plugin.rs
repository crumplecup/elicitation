//! `AxumResponseHtmlPlugin` — MCP tools for axum HTML responses, redirects, and status codes.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for response_html_create.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HtmlResponseParams {
    /// Raw HTML body content.
    pub html_content: String,
}

/// Parameters for redirect tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RedirectParams {
    /// The target URL for the redirect.
    pub location: String,
}

/// Parameters for response_status_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StatusDescribeParams {
    /// The HTTP status code to describe.
    pub status: u16,
}

/// Parameters for response_ok (no inputs needed).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsOk {}

/// Parameters for response_created (no inputs needed).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsCreated {}

/// Parameters for response_no_content (no inputs needed).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsNoContent {}

/// Parameters for error response tools.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ErrorMessageParam {
    /// Human-readable error message.
    pub message: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_html_create",
    emit = None,
    description = "Create an HTML response descriptor with HTTP 200 status."
)]
#[instrument]
async fn response_html_create(p: HtmlResponseParams) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::json!({
        "status": 200,
        "content_type": "text/html; charset=utf-8",
        "body_length": p.html_content.len(),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_redirect_to",
    emit = None,
    description = "Create a temporary (307) redirect response descriptor."
)]
#[instrument]
async fn response_redirect_to(p: RedirectParams) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::json!({
        "status": 307,
        "location": p.location,
        "description": "Temporary redirect (307 Temporary Redirect)",
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_redirect_permanent",
    emit = None,
    description = "Create a permanent (308) redirect response descriptor."
)]
#[instrument]
async fn response_redirect_permanent(p: RedirectParams) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::json!({
        "status": 308,
        "location": p.location,
        "description": "Permanent redirect (308 Permanent Redirect)",
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_redirect_temporary",
    emit = None,
    description = "Create a temporary (307) redirect response descriptor."
)]
#[instrument]
async fn response_redirect_temporary(p: RedirectParams) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::json!({
        "status": 307,
        "location": p.location,
        "description": "Temporary redirect (307 Temporary Redirect)",
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_status_describe",
    emit = None,
    description = "Return the canonical reason phrase for an HTTP status code."
)]
#[instrument]
async fn response_status_describe(p: StatusDescribeParams) -> Result<CallToolResult, ErrorData> {
    let reason = match p.status {
        100 => "Continue",
        101 => "Switching Protocols",
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        410 => "Gone",
        422 => "Unprocessable Entity",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Unknown status code",
    };
    Ok(CallToolResult::success(vec![Content::text(reason)]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_ok",
    emit = None,
    description = "Describe the HTTP 200 OK status."
)]
#[instrument]
async fn response_ok(_p: NoParamsOk) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "200 OK — request succeeded",
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_created",
    emit = None,
    description = "Describe the HTTP 201 Created status."
)]
#[instrument]
async fn response_created(_p: NoParamsCreated) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "201 Created — resource created successfully",
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_no_content",
    emit = None,
    description = "Describe the HTTP 204 No Content status."
)]
#[instrument]
async fn response_no_content(_p: NoParamsNoContent) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "204 No Content — request succeeded with no response body",
    )]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_bad_request",
    emit = None,
    description = "Format a 400 Bad Request error message."
)]
#[instrument]
async fn response_bad_request(p: ErrorMessageParam) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "400 Bad Request: {}",
        p.message
    ))]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_not_found",
    emit = None,
    description = "Format a 404 Not Found error message."
)]
#[instrument]
async fn response_not_found(p: ErrorMessageParam) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "404 Not Found: {}",
        p.message
    ))]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_internal_error",
    emit = None,
    description = "Format a 500 Internal Server Error message."
)]
#[instrument]
async fn response_internal_error(p: ErrorMessageParam) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "500 Internal Server Error: {}",
        p.message
    ))]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_unauthorized",
    emit = None,
    description = "Format a 401 Unauthorized error message."
)]
#[instrument]
async fn response_unauthorized(p: ErrorMessageParam) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "401 Unauthorized: {}",
        p.message
    ))]))
}

#[elicit_tool(
    plugin = "axum_response_html",
    name = "response_forbidden",
    emit = None,
    description = "Format a 403 Forbidden error message."
)]
#[instrument]
async fn response_forbidden(p: ErrorMessageParam) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "403 Forbidden: {}",
        p.message
    ))]))
}

/// Plugin exposing axum HTML response, redirect, and status code tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_response_html")]
pub struct AxumResponseHtmlPlugin;
