//! `TowerRequestIdPlugin` — RequestIdLayer configuration MCP tools.

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

/// Configuration for a RequestIdLayer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct RequestIdConfig {
    /// The header name used to carry the request ID.
    pub header_name: String,
    /// Whether to propagate an existing ID from the request.
    pub propagate: bool,
    /// Human-readable description of the ID generator.
    pub description: String,
}

/// Parameters for request_id_make_uuid (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestIdMakeUuidParams {}

/// Parameters wrapping a RequestIdConfig.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestIdConfigInput {
    /// The current request ID configuration.
    pub config: RequestIdConfig,
}

/// Parameters for creating a request ID layer from a header name.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestIdHeaderParams {
    /// The header name used to carry the request ID.
    pub header_name: String,
}

/// Parameters for updating the header name.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestIdSetHeaderParams {
    /// The current request ID configuration.
    pub config: RequestIdConfig,
    /// The new header name.
    pub header_name: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_request_id",
    name = "request_id_make_uuid",
    description = "Create a RequestIdLayer configuration using a UUID v4 generator."
)]
#[instrument]
async fn request_id_make_uuid(_p: RequestIdMakeUuidParams) -> Result<CallToolResult, ErrorData> {
    let result = RequestIdConfig {
        header_name: "x-request-id".to_string(),
        propagate: false,
        description: "UUID v4 generator".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_request_id",
    name = "request_id_make_header",
    description = "Create a RequestIdLayer configuration using a specific header name."
)]
#[instrument]
async fn request_id_make_header(p: RequestIdHeaderParams) -> Result<CallToolResult, ErrorData> {
    let result = RequestIdConfig {
        header_name: p.header_name.clone(),
        propagate: false,
        description: format!("Header-based ID from '{}'", p.header_name),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_request_id",
    name = "request_id_propagate",
    description = "Enable propagation of existing request IDs from incoming requests."
)]
#[instrument]
async fn request_id_propagate(p: RequestIdConfigInput) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.propagate = true;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_request_id",
    name = "request_id_header",
    description = "Update the header name used by a RequestIdLayer configuration."
)]
#[instrument]
async fn request_id_header(p: RequestIdSetHeaderParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.header_name = p.header_name;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

/// Plugin exposing RequestIdLayer configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_request_id")]
pub struct TowerRequestIdPlugin;
