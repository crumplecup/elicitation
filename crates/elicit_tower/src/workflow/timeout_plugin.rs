//! `TowerTimeoutPlugin` — TimeoutLayer configuration MCP tools.

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

/// Configuration for a TimeoutLayer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct TimeoutConfig {
    /// Timeout duration in milliseconds.
    pub timeout_ms: u64,
    /// Human-readable description.
    pub description: String,
}

/// Parameters wrapping a TimeoutConfig.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimeoutConfigInput {
    /// The current timeout configuration.
    pub config: TimeoutConfig,
}

/// Parameters for creating a timeout by milliseconds.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimeoutNewParams {
    /// Timeout duration in milliseconds.
    pub timeout_ms: u64,
}

/// Parameters for creating a timeout by seconds.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimeoutSecsParams {
    /// Timeout duration in seconds (may be fractional).
    pub secs: f64,
}

/// Parameters for creating a timeout by milliseconds.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TimeoutMillisParams {
    /// Timeout duration in milliseconds.
    pub millis: u64,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_timeout",
    name = "timeout_new",
    description = "Create a TimeoutLayer configuration from a millisecond value."
)]
#[instrument]
async fn timeout_new(p: TimeoutNewParams) -> Result<CallToolResult, ErrorData> {
    let result = TimeoutConfig {
        timeout_ms: p.timeout_ms,
        description: format!("TimeoutLayer({}ms)", p.timeout_ms),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_timeout",
    name = "timeout_with_timeout_secs",
    description = "Create a TimeoutLayer configuration from a duration in seconds."
)]
#[instrument]
async fn timeout_with_timeout_secs(p: TimeoutSecsParams) -> Result<CallToolResult, ErrorData> {
    let ms = (p.secs * 1000.0) as u64;
    let result = TimeoutConfig {
        timeout_ms: ms,
        description: format!("TimeoutLayer({:.3}s / {}ms)", p.secs, ms),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_timeout",
    name = "timeout_with_timeout_millis",
    description = "Create a TimeoutLayer configuration from a duration in milliseconds."
)]
#[instrument]
async fn timeout_with_timeout_millis(p: TimeoutMillisParams) -> Result<CallToolResult, ErrorData> {
    let result = TimeoutConfig {
        timeout_ms: p.millis,
        description: format!("TimeoutLayer({}ms)", p.millis),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_timeout",
    name = "timeout_value_secs",
    description = "Get the timeout value in seconds from a TimeoutLayer configuration."
)]
#[instrument]
async fn timeout_value_secs(p: TimeoutConfigInput) -> Result<CallToolResult, ErrorData> {
    let secs = p.config.timeout_ms as f64 / 1000.0;
    let result = serde_json::json!({
        "timeout_ms": p.config.timeout_ms,
        "timeout_secs": secs,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing TimeoutLayer configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_timeout")]
pub struct TowerTimeoutPlugin;
