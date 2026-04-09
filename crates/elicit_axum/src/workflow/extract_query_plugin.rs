//! `AxumExtractQueryPlugin` — MCP tools for axum Query extractor.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for extract_query_struct.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryStructParams {
    /// The Rust struct type name used for deserialization.
    pub struct_type: String,
    /// The raw URL query string, e.g. `page=1&limit=20`.
    pub query_string: String,
}

/// Parameters for extract_query_raw.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RawQueryParams {
    /// The raw URL query string.
    pub query_string: String,
}

/// Parameters for query_parse.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryParseParams {
    /// The URL-encoded query string to parse.
    pub query_string: String,
    /// The expected Rust type for deserialization.
    pub expected_type: String,
}

/// Parameters for query_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryDescribeParams {
    /// The Rust struct type that the Query extractor will deserialize into.
    pub struct_type: String,
}

/// Parameters for raw_query_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RawQueryDescribeParams {
    /// The current raw query string value.
    pub query_string: String,
}

/// Parameters for query_missing_field_error.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryMissingParams {
    /// The name of the missing required query field.
    pub field_name: String,
}

/// Parameters for query_decode.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct QueryDecodeParams {
    /// The URL-encoded string to decode.
    pub encoded: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn url_decode(s: &str) -> String {
    let s = s.replace('+', " ");
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            let hex = format!("{}{}", h1, h2);
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            } else {
                result.push('%');
                result.push(h1);
                result.push(h2);
            }
        } else {
            result.push(c);
        }
    }
    result
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "extract_query_struct",
    description = "Describe a Query<T> extraction from a URL query string into a typed struct."
)]
#[instrument]
async fn extract_query_struct(p: QueryStructParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Query<{}> parsed from '{}'. Fields: {}",
        p.struct_type, p.query_string, p.query_string
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "extract_query_raw",
    description = "Describe a raw query string extraction from the request URI."
)]
#[instrument]
async fn extract_query_raw(p: RawQueryParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Raw query string extracted: '{}'. Use RawQuery to obtain the undecoded query string as Option<String>.",
        p.query_string
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "query_parse",
    description = "Parse a URL-encoded query string into key-value pairs and return as JSON."
)]
#[instrument]
async fn query_parse(p: QueryParseParams) -> Result<CallToolResult, ErrorData> {
    let fields: Vec<serde_json::Value> = p
        .query_string
        .split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = url_decode(parts.next().unwrap_or(""));
            let value = url_decode(parts.next().unwrap_or(""));
            serde_json::json!({ "key": key, "value": value })
        })
        .collect();
    let val = serde_json::json!({
        "query_string": p.query_string,
        "expected_type": p.expected_type,
        "fields": fields,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&val).unwrap_or_default(),
    )]))
}

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "query_describe",
    description = "Describe how Query<T> deserializes the URL query string into a typed struct."
)]
#[instrument]
async fn query_describe(p: QueryDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "Query<{}> extracts from the URL query string using serde deserialization. \
        Missing fields with no default will cause a 422 rejection.",
        p.struct_type
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "raw_query_describe",
    description = "Describe the RawQuery extractor and its current value."
)]
#[instrument]
async fn raw_query_describe(p: RawQueryDescribeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "RawQuery extracts the raw query string as Option<String>. Current value: '{}'",
        p.query_string
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "query_missing_field_error",
    description = "Format the error message produced when a required query field is missing."
)]
#[instrument]
async fn query_missing_field_error(p: QueryMissingParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(format!(
        "400 Bad Request: Failed to deserialize query string: missing field `{}`",
        p.field_name
    ))]))
}

#[elicit_tool(
    plugin = "axum_extract_query",
    name = "query_decode",
    description = "URL-decode a percent-encoded query string, replacing + with space and %XX sequences."
)]
#[instrument]
async fn query_decode(p: QueryDecodeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(url_decode(
        &p.encoded,
    ))]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum Query extractor tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_extract_query")]
pub struct AxumExtractQueryPlugin;
