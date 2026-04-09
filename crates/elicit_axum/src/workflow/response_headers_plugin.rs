//! `AxumResponseHeadersPlugin` — MCP tools for axum response header manipulation.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes the parts of an HTTP response (status + headers).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ResponsePartsDescriptor {
    /// HTTP status code.
    pub status: u16,
    /// Primary response headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
    /// Additional headers appended after the primary headers.
    pub extra_headers: Vec<String>,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for append_headers_create.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppendHeadersParams {
    /// Headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
}

/// Parameters for append_headers_single.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppendHeaderSingleParams {
    /// Header name.
    pub name: String,
    /// Header value.
    pub value: String,
}

/// Parameters for append_headers_multiple.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppendHeadersMultipleParams {
    /// List of (name, value) header pairs.
    pub headers: Vec<(String, String)>,
}

/// Parameters for response_parts_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResponsePartsDescribeParams {
    /// HTTP status code.
    pub status: u16,
    /// Headers as `"Name: Value"` strings.
    pub headers: Vec<String>,
}

/// Parameters for set_header.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetHeaderParams {
    /// The response parts to modify.
    pub parts: ResponsePartsDescriptor,
    /// Header name to set.
    pub name: String,
    /// Header value to set.
    pub value: String,
}

/// Parameters for override_header.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OverrideHeaderParams {
    /// The response parts to modify.
    pub parts: ResponsePartsDescriptor,
    /// Header name to override.
    pub name: String,
    /// New header value.
    pub value: String,
}

/// Parameters for remove_header.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RemoveHeaderParams {
    /// The response parts to modify.
    pub parts: ResponsePartsDescriptor,
    /// Header name to remove.
    pub name: String,
}

/// Parameters for header_value_str.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeaderValueStrParams {
    /// The header value string to validate.
    pub value: String,
}

/// Parameters for header_value_bytes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HeaderValueBytesParams {
    /// Raw bytes of the header value.
    pub bytes: Vec<u8>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "append_headers_create",
    emit = None,
    description = "Create an AppendHeaders descriptor from a list of header strings."
)]
#[instrument]
async fn append_headers_create(p: AppendHeadersParams) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::json!({
        "description": "AppendHeaders",
        "headers": p.headers,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "append_headers_single",
    emit = None,
    description = "Create an AppendHeaders descriptor for a single header name/value pair."
)]
#[instrument]
async fn append_headers_single(p: AppendHeaderSingleParams) -> Result<CallToolResult, ErrorData> {
    let val = serde_json::json!({
        "name": p.name,
        "value": p.value,
        "description": format!("AppendHeaders single header: {}: {}", p.name, p.value),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "append_headers_multiple",
    emit = None,
    description = "Create an AppendHeaders descriptor from multiple name/value pairs."
)]
#[instrument]
async fn append_headers_multiple(
    p: AppendHeadersMultipleParams,
) -> Result<CallToolResult, ErrorData> {
    let entries: Vec<serde_json::Value> = p
        .headers
        .into_iter()
        .map(|(k, v)| serde_json::json!({"name": k, "value": v}))
        .collect();
    let val = serde_json::to_string(&entries).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "response_parts_describe",
    emit = None,
    description = "Create a ResponsePartsDescriptor from a status code and header list."
)]
#[instrument]
async fn response_parts_describe(
    p: ResponsePartsDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let descriptor = ResponsePartsDescriptor {
        status: p.status,
        headers: p.headers,
        extra_headers: vec![],
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "set_header",
    emit = None,
    description = "Append a header to a ResponsePartsDescriptor."
)]
#[instrument]
async fn set_header(p: SetHeaderParams) -> Result<CallToolResult, ErrorData> {
    let mut parts = p.parts.clone();
    parts.headers.push(format!("{}: {}", p.name, p.value));
    let val = serde_json::to_string(&parts).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "override_header",
    emit = None,
    description = "Replace an existing header (case-insensitive) in a ResponsePartsDescriptor."
)]
#[instrument]
async fn override_header(p: OverrideHeaderParams) -> Result<CallToolResult, ErrorData> {
    let mut parts = p.parts.clone();
    let prefix = format!("{}:", p.name.to_lowercase());
    parts
        .headers
        .retain(|h| !h.to_lowercase().starts_with(&prefix));
    parts.headers.push(format!("{}: {}", p.name, p.value));
    let val = serde_json::to_string(&parts).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "remove_header",
    emit = None,
    description = "Remove all headers matching a name (case-insensitive) from a ResponsePartsDescriptor."
)]
#[instrument]
async fn remove_header(p: RemoveHeaderParams) -> Result<CallToolResult, ErrorData> {
    let mut parts = p.parts.clone();
    let prefix = format!("{}:", p.name.to_lowercase());
    parts
        .headers
        .retain(|h| !h.to_lowercase().starts_with(&prefix));
    let val = serde_json::to_string(&parts).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "header_value_str",
    emit = None,
    description = "Validate a header value string for illegal characters (CR, LF, non-visible)."
)]
#[instrument]
async fn header_value_str(p: HeaderValueStrParams) -> Result<CallToolResult, ErrorData> {
    let error: Option<String> = if p.value.contains('\r') {
        Some("Header value must not contain CR (\\r)".to_string())
    } else if p.value.contains('\n') {
        Some("Header value must not contain LF (\\n)".to_string())
    } else if p.value.chars().any(|c| {
        let code = c as u32;
        code < 0x20 && c != '\t'
    }) {
        Some("Header value contains non-visible characters other than tab".to_string())
    } else {
        None
    };
    let valid = error.is_none();
    let val = serde_json::json!({
        "value": p.value,
        "valid": valid,
        "error": error,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_response_headers",
    name = "header_value_bytes",
    emit = None,
    description = "Convert raw bytes to a header value string, reporting UTF-8 validity."
)]
#[instrument]
async fn header_value_bytes(p: HeaderValueBytesParams) -> Result<CallToolResult, ErrorData> {
    let byte_length = p.bytes.len();
    let (string_value, valid_utf8) = match String::from_utf8(p.bytes) {
        Ok(s) => (serde_json::Value::String(s), true),
        Err(_) => (serde_json::Value::Null, false),
    };
    let val = serde_json::json!({
        "string_value": string_value,
        "byte_length": byte_length,
        "valid_utf8": valid_utf8,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

/// Plugin exposing axum response header manipulation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_response_headers")]
pub struct AxumResponseHeadersPlugin;
