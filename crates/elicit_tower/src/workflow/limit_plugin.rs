//! `TowerLimitPlugin` — Request body limit, panic catch, and path normalization MCP tools.

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

/// Configuration for a limit or utility layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct LimitConfig {
    /// The kind of limit layer ("body_limit", "catch_panic", "normalize_path").
    pub kind: String,
    /// Optional numeric value associated with the limit.
    pub value: Option<u64>,
    /// Human-readable description.
    pub description: String,
}

/// Parameters wrapping a LimitConfig.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LimitConfigInput {
    /// The current limit configuration.
    pub config: LimitConfig,
}

/// Parameters for creating a body limit.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BodyLimitParams {
    /// Maximum allowed request body size in bytes.
    pub limit_bytes: u64,
}

/// Parameters for a catch-panic layer with a custom handler.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CatchPanicHandlerParams {
    /// Description of the panic handler function.
    pub handler_description: String,
}

/// Parameters for catch_panic_new (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CatchPanicNewParams {}

/// Parameters for normalize_path_new (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NormalizePathNewParams {}

/// Parameters for normalizing request paths.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NormalizePathParams {
    /// Whether to trim trailing slashes.
    pub trim: bool,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_limit",
    name = "request_body_limit_new",
    description = "Create a RequestBodyLimitLayer configuration with a maximum body size."
)]
#[instrument]
async fn request_body_limit_new(p: BodyLimitParams) -> Result<CallToolResult, ErrorData> {
    let result = LimitConfig {
        kind: "body_limit".to_string(),
        value: Some(p.limit_bytes),
        description: format!("RequestBodyLimitLayer({}B)", p.limit_bytes),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_limit",
    name = "request_body_limit_bytes",
    description = "Get the limit bytes value from a RequestBodyLimitLayer configuration."
)]
#[instrument]
async fn request_body_limit_bytes(p: LimitConfigInput) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "limit_bytes": p.config.value,
        "description": p.config.description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_limit",
    name = "catch_panic_new",
    description = "Create a CatchPanicLayer configuration with the default handler."
)]
#[instrument]
async fn catch_panic_new(_p: CatchPanicNewParams) -> Result<CallToolResult, ErrorData> {
    let result = LimitConfig {
        kind: "catch_panic".to_string(),
        value: None,
        description: "CatchPanicLayer with default handler (500 Internal Server Error)".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_limit",
    name = "catch_panic_with_handler",
    description = "Create a CatchPanicLayer configuration with a custom panic handler."
)]
#[instrument]
async fn catch_panic_with_handler(p: CatchPanicHandlerParams) -> Result<CallToolResult, ErrorData> {
    let result = LimitConfig {
        kind: "catch_panic".to_string(),
        value: None,
        description: format!("CatchPanicLayer with handler: {}", p.handler_description),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_limit",
    name = "normalize_path_new",
    description = "Create a NormalizePathLayer configuration with default settings."
)]
#[instrument]
async fn normalize_path_new(_p: NormalizePathNewParams) -> Result<CallToolResult, ErrorData> {
    let result = LimitConfig {
        kind: "normalize_path".to_string(),
        value: None,
        description: "NormalizePathLayer (trims trailing slashes)".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_limit",
    name = "normalize_path_trim_trailing",
    description = "Create a NormalizePathLayer configuration with explicit trailing-slash trimming setting."
)]
#[instrument]
async fn normalize_path_trim_trailing(p: NormalizePathParams) -> Result<CallToolResult, ErrorData> {
    let result = LimitConfig {
        kind: "normalize_path".to_string(),
        value: None,
        description: format!("NormalizePathLayer (trim_trailing_slash={})", p.trim),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing request body limit, catch-panic, and normalize-path layer tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_limit")]
pub struct TowerLimitPlugin;
