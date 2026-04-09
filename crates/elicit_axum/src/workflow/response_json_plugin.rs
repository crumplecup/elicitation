//! `AxumResponseJsonPlugin` — MCP tools for axum JSON responses.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes an axum JSON response.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JsonResponseDescriptor {
    /// HTTP status code.
    pub status: u16,
    /// JSON-encoded body string.
    pub body_json: String,
    /// Content-Type header value.
    pub content_type: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for response_json_create.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonResponseParams {
    /// JSON-encoded body string.
    pub body_json: String,
}

/// Parameters for response_json_with_status.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonResponseWithStatusParams {
    /// HTTP status code.
    pub status: u16,
    /// JSON-encoded body string.
    pub body_json: String,
}

/// Parameters for response_json_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonDescribeResponseParams {
    /// The Rust type name being serialized to JSON.
    pub type_name: String,
}

/// Parameters for json_response_headers.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonResponseDescriptorInput {
    /// The JSON response descriptor.
    pub descriptor: JsonResponseDescriptor,
}

/// Parameters for json_response_body_bytes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonBodyBytesParams {
    /// JSON-encoded body string.
    pub body_json: String,
}

/// Parameters for json_response_content_type (no inputs needed).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NoParamsJsonCt {}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_response_json",
    name = "response_json_create",
    emit = None,
    description = "Create a JSON response descriptor with HTTP 200 status."
)]
#[instrument]
async fn response_json_create(p: JsonResponseParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = JsonResponseDescriptor {
        status: 200,
        body_json: p.body_json,
        content_type: "application/json".to_string(),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_json",
    name = "response_json_with_status",
    emit = None,
    description = "Create a JSON response descriptor with a specified HTTP status code."
)]
#[instrument]
async fn response_json_with_status(
    p: JsonResponseWithStatusParams,
) -> Result<CallToolResult, ErrorData> {
    let descriptor = JsonResponseDescriptor {
        status: p.status,
        body_json: p.body_json,
        content_type: "application/json".to_string(),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_json",
    name = "response_json_describe",
    emit = None,
    description = "Describe axum's Json<T> response type for a given Rust type name."
)]
#[instrument]
async fn response_json_describe(
    p: JsonDescribeResponseParams,
) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "Json<{}> serializes the value to JSON and sets Content-Type: application/json. \
         Implements IntoResponse. Will return 500 if serialization fails.",
        p.type_name
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_response_json",
    name = "json_response_headers",
    emit = None,
    description = "Return the HTTP headers that a JSON response will include."
)]
#[instrument]
async fn json_response_headers(
    p: JsonResponseDescriptorInput,
) -> Result<CallToolResult, ErrorData> {
    let headers = serde_json::json!([
        "Content-Type: application/json",
        format!("Content-Length: {}", p.descriptor.body_json.len()),
    ]);
    let val = serde_json::to_string(&headers).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_json",
    name = "json_response_body_bytes",
    emit = None,
    description = "Estimate the byte size of a JSON response body."
)]
#[instrument]
async fn json_response_body_bytes(p: JsonBodyBytesParams) -> Result<CallToolResult, ErrorData> {
    let text = format!("Estimated {} bytes (UTF-8 encoded JSON)", p.body_json.len());
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_response_json",
    name = "json_response_content_type",
    emit = None,
    description = "Return the Content-Type value used by axum JSON responses."
)]
#[instrument]
async fn json_response_content_type(_p: NoParamsJsonCt) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        "application/json",
    )]))
}

/// Plugin exposing axum JSON response tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_response_json")]
pub struct AxumResponseJsonPlugin;
