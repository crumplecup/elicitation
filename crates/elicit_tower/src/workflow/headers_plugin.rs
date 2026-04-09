//! `TowerHeadersPlugin` — Header layer configuration MCP tools.

use elicitation::{ElicitPlugin, ToCodeLiteral, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Configuration for a header manipulation layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct HeaderLayerConfig {
    /// The header manipulation action ("insert", "append", "override", "if_not_present").
    pub action: String,
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
    /// Whether this header contains sensitive data.
    pub sensitive: bool,
    /// Whether this applies to "request" or "response".
    pub direction: String,
}

// ── Unique per-tool params ────────────────────────────────────────────────────

/// Parameters for headers_set_request_insert.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetRequestInsertParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_request_append.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetRequestAppendParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_request_override.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetRequestOverrideParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_request_if_not_present.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetRequestIfNotPresentParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_response_insert.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetResponseInsertParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_response_append.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetResponseAppendParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_response_override.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetResponseOverrideParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_set_response_if_not_present.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HeadersSetResponseIfNotPresentParams {
    /// The header name.
    pub header_name: String,
    /// The header value.
    pub header_value: String,
}

/// Parameters for headers_sensitive_new.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SensitiveHeadersParams {
    /// The list of header names to mark as sensitive.
    pub header_names: Vec<String>,
}

/// Parameters for headers_sensitive_add.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SensitiveAddParams {
    /// The current header layer configuration.
    pub config: HeaderLayerConfig,
    /// The header name to add.
    pub header_name: String,
}

/// Parameters for headers_sensitive_remove.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SensitiveRemoveParams {
    /// The current header layer configuration.
    pub config: HeaderLayerConfig,
    /// The header name to remove.
    pub header_name: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_request_insert",
    description = "Create a layer that inserts a header into requests."
)]
#[instrument]
async fn headers_set_request_insert(
    p: HeadersSetRequestInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "insert".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "request".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_request_append",
    description = "Create a layer that appends a header to requests."
)]
#[instrument]
async fn headers_set_request_append(
    p: HeadersSetRequestAppendParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "append".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "request".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_request_override",
    description = "Create a layer that overrides a header in requests."
)]
#[instrument]
async fn headers_set_request_override(
    p: HeadersSetRequestOverrideParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "override".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "request".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_request_if_not_present",
    description = "Create a layer that sets a request header only if it is not already present."
)]
#[instrument]
async fn headers_set_request_if_not_present(
    p: HeadersSetRequestIfNotPresentParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "if_not_present".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "request".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_response_insert",
    description = "Create a layer that inserts a header into responses."
)]
#[instrument]
async fn headers_set_response_insert(
    p: HeadersSetResponseInsertParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "insert".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "response".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_response_append",
    description = "Create a layer that appends a header to responses."
)]
#[instrument]
async fn headers_set_response_append(
    p: HeadersSetResponseAppendParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "append".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "response".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_response_override",
    description = "Create a layer that overrides a header in responses."
)]
#[instrument]
async fn headers_set_response_override(
    p: HeadersSetResponseOverrideParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "override".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "response".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_set_response_if_not_present",
    description = "Create a layer that sets a response header only if it is not already present."
)]
#[instrument]
async fn headers_set_response_if_not_present(
    p: HeadersSetResponseIfNotPresentParams,
) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "if_not_present".to_string(),
        header_name: p.header_name,
        header_value: p.header_value,
        sensitive: false,
        direction: "response".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_sensitive_new",
    description = "Create a sensitive-headers layer configuration for the given header names."
)]
#[instrument]
async fn headers_sensitive_new(p: SensitiveHeadersParams) -> Result<CallToolResult, ErrorData> {
    let result = HeaderLayerConfig {
        action: "sensitive".to_string(),
        header_name: p.header_names.join(", "),
        header_value: String::new(),
        sensitive: true,
        direction: "request".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_sensitive_add",
    description = "Add a header name to the sensitive headers list."
)]
#[instrument]
async fn headers_sensitive_add(p: SensitiveAddParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    let mut names: Vec<String> = config
        .header_name
        .split(", ")
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    if !names.contains(&p.header_name) {
        names.push(p.header_name);
    }
    config.header_name = names.join(", ");
    config.sensitive = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_headers",
    name = "headers_sensitive_remove",
    description = "Remove a header name from the sensitive headers list."
)]
#[instrument]
async fn headers_sensitive_remove(p: SensitiveRemoveParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    let names: Vec<String> = config
        .header_name
        .split(", ")
        .filter(|s| !s.is_empty() && *s != p.header_name)
        .map(String::from)
        .collect();
    config.header_name = names.join(", ");
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

/// Plugin exposing header manipulation layer tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_headers")]
pub struct TowerHeadersPlugin;
