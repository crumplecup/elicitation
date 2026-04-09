//! `AxumExtractBodyPlugin` — MCP tools for axum body extractors.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for extract_json_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonDescribeParams {
    /// The Rust type name for JSON deserialization.
    pub type_name: String,
    /// An example JSON body for illustration.
    pub example_json: String,
}

/// Parameters for extract_form_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FormDescribeParams {
    /// The Rust type name for form deserialization.
    pub type_name: String,
    /// Example URL-encoded form data.
    pub form_data: String,
}

/// Parameters for extract_bytes_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BytesDescribeParams {
    /// Optional Content-Type header value.
    pub content_type: Option<String>,
    /// Optional maximum body size in bytes.
    pub limit_bytes: Option<u64>,
}

/// Parameters for extract_string_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringDescribeParams {
    /// The expected body encoding, e.g. `utf-8`.
    pub encoding: String,
}

/// Parameters for json_parse.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonParseParams {
    /// The raw JSON string to validate.
    pub json_string: String,
    /// The Rust target type name.
    pub target_type: String,
}

/// Parameters for form_parse.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FormParseParams {
    /// The URL-encoded form body.
    pub form_data: String,
    /// The Rust target type name.
    pub target_type: String,
}

/// Parameters for body_bytes_collect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BodyBytesParams {
    /// Optional size hint in bytes.
    pub size_hint: Option<u64>,
}

/// Parameters for body_string_collect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BodyStringParams {
    /// The expected string encoding, e.g. `utf-8`.
    pub encoding: String,
}

/// Parameters for json_rejection_display.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct JsonRejectionParams {
    /// The rejection variant name, e.g. `MissingJsonContentType`.
    pub rejection_kind: String,
    /// Detailed rejection message.
    pub detail: String,
}

/// Parameters for form_rejection_display.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FormRejectionParams {
    /// The rejection variant name, e.g. `FailedToDeserializeForm`.
    pub rejection_kind: String,
    /// Detailed rejection message.
    pub detail: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "extract_json_describe",
    description = "Describe the Json<T> extractor: requirements, rejections, and example usage."
)]
#[instrument]
async fn extract_json_describe(p: JsonDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Json<{}> deserializes the request body as JSON. \
        Requires Content-Type: application/json. \
        Returns 400 on invalid JSON or missing content-type. \
        Example body: {}",
        p.type_name, p.example_json
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "extract_form_describe",
    description = "Describe the Form<T> extractor: requirements, rejections, and example usage."
)]
#[instrument]
async fn extract_form_describe(p: FormDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Form<{}> deserializes the request body as URL-encoded form data. \
        Requires Content-Type: application/x-www-form-urlencoded. \
        Returns 400 on deserialization failure. \
        Example form data: {}",
        p.type_name, p.form_data
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "extract_bytes_describe",
    description = "Describe the Bytes extractor: collects the entire request body as raw bytes."
)]
#[instrument]
async fn extract_bytes_describe(p: BytesDescribeParams) -> Result<CallToolResult, ErrorData> {
    let ct = p.content_type.as_deref().unwrap_or("(any)");
    let limit = p
        .limit_bytes
        .map(|b| format!("{} bytes", b))
        .unwrap_or_else(|| "unlimited".to_string());
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Bytes extractor collects the complete request body into a Bytes buffer. \
        Content-Type: {}. Size limit: {}. \
        Use with a body limit layer to prevent unbounded memory usage.",
        ct, limit
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "extract_string_describe",
    description = "Describe the String extractor: reads the entire request body as a UTF-8 string."
)]
#[instrument]
async fn extract_string_describe(p: StringDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "String extractor reads the entire request body as a UTF-8 string. \
        Encoding: {}. Returns 400 if the body is not valid UTF-8. \
        Combine with a body limit layer to cap memory usage.",
        p.encoding
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "json_parse",
    description = "Validate a JSON string and report whether it is well-formed."
)]
#[instrument]
async fn json_parse(p: JsonParseParams) -> Result<CallToolResult, ErrorData> {
    let (valid, error) = match serde_json::from_str::<serde_json::Value>(&p.json_string) {
        Ok(_) => (true, serde_json::Value::Null),
        Err(e) => (false, serde_json::Value::String(e.to_string())),
    };
    let val = serde_json::json!({
        "valid": valid,
        "target_type": p.target_type,
        "json_string": p.json_string,
        "error": error,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "form_parse",
    description = "Parse URL-encoded form data into key-value pairs and return as JSON."
)]
#[instrument]
async fn form_parse(p: FormParseParams) -> Result<CallToolResult, ErrorData> {
    let fields: Vec<serde_json::Value> = p
        .form_data
        .split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            serde_json::json!({ "key": key, "value": value })
        })
        .collect();
    let val = serde_json::json!({
        "target_type": p.target_type,
        "fields": fields,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "body_bytes_collect",
    description = "Describe collecting the request body as raw Bytes with an optional size hint."
)]
#[instrument]
async fn body_bytes_collect(p: BodyBytesParams) -> Result<CallToolResult, ErrorData> {
    let hint = p
        .size_hint
        .map(|b| format!("Size hint: {} bytes.", b))
        .unwrap_or_else(|| "No size hint provided.".to_string());
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Collecting request body as Bytes. {}  \
        The body is buffered entirely in memory before the handler runs.",
        hint
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "body_string_collect",
    description = "Describe collecting the request body as a UTF-8 String."
)]
#[instrument]
async fn body_string_collect(p: BodyStringParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Collecting request body as String with encoding '{}'. \
        The extractor reads all bytes and validates UTF-8. \
        A 400 rejection is returned if the body contains invalid UTF-8 sequences.",
        p.encoding
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "json_rejection_display",
    description = "Format a JsonRejection error message for a given rejection kind and detail."
)]
#[instrument]
async fn json_rejection_display(p: JsonRejectionParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "JsonRejection::{}: {}",
        p.rejection_kind, p.detail
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_body",
    name = "form_rejection_display",
    description = "Format a FormRejection error message for a given rejection kind and detail."
)]
#[instrument]
async fn form_rejection_display(p: FormRejectionParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "FormRejection::{}: {}",
        p.rejection_kind, p.detail
    ))]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum body extractor tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_extract_body")]
pub struct AxumExtractBodyPlugin;
