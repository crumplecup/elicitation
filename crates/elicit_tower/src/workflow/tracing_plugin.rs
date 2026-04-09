//! `TowerTracingPlugin` — TraceLayer configuration MCP tools.

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

/// Configuration for a TraceLayer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct TraceConfig {
    /// The tracing target (e.g. "tower_http::trace").
    pub target: String,
    /// The log level for trace events (e.g. "DEBUG", "INFO").
    pub level: String,
    /// Whether to include request/response headers in spans.
    pub include_headers: bool,
    /// Description of the on_request handler.
    pub on_request: String,
    /// Description of the on_response handler.
    pub on_response: String,
    /// Description of the on_failure handler.
    pub on_failure: String,
}

// ── Unique per-tool params ────────────────────────────────────────────────────

/// Parameters for trace_new_for_http (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceNewForHttpParams {}

/// Parameters for trace_new_for_grpc (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceNewForGrpcParams {}

/// Parameters for trace_make_span_with_default.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceMakeSpanWithDefaultParams {
    /// The current trace configuration.
    pub config: TraceConfig,
}

/// Parameters for trace_on_request.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceOnRequestParams {
    /// The current trace configuration.
    pub config: TraceConfig,
    /// Description of the new handler.
    pub handler_description: String,
}

/// Parameters for trace_on_response.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceOnResponseParams {
    /// The current trace configuration.
    pub config: TraceConfig,
    /// Description of the new handler.
    pub handler_description: String,
}

/// Parameters for trace_on_failure.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceOnFailureParams {
    /// The current trace configuration.
    pub config: TraceConfig,
    /// Description of the new handler.
    pub handler_description: String,
}

/// Parameters for trace_on_body_chunk.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceOnBodyChunkParams {
    /// The current trace configuration.
    pub config: TraceConfig,
    /// Description of the new handler.
    pub handler_description: String,
}

/// Parameters for trace_on_eos.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceOnEosParams {
    /// The current trace configuration.
    pub config: TraceConfig,
    /// Description of the new handler.
    pub handler_description: String,
}

/// Parameters for trace_classify_response.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TraceClassifyResponseParams {
    /// The current trace configuration.
    pub config: TraceConfig,
    /// Description of the new handler.
    pub handler_description: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_new_for_http",
    description = "Create a default TraceLayer configuration for HTTP services."
)]
#[instrument]
async fn trace_new_for_http(_p: TraceNewForHttpParams) -> Result<CallToolResult, ErrorData> {
    let result = TraceConfig {
        target: "tower_http::trace".to_string(),
        level: "DEBUG".to_string(),
        include_headers: false,
        on_request: "DefaultOnRequest".to_string(),
        on_response: "DefaultOnResponse".to_string(),
        on_failure: "DefaultOnFailure".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_new_for_grpc",
    description = "Create a default TraceLayer configuration for gRPC services."
)]
#[instrument]
async fn trace_new_for_grpc(_p: TraceNewForGrpcParams) -> Result<CallToolResult, ErrorData> {
    let result = TraceConfig {
        target: "tower_http::trace".to_string(),
        level: "DEBUG".to_string(),
        include_headers: true,
        on_request: "DefaultOnRequest (gRPC)".to_string(),
        on_response: "DefaultOnResponse (gRPC)".to_string(),
        on_failure: "DefaultOnFailure (gRPC)".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_on_request",
    description = "Set the on_request handler description for a TraceLayer."
)]
#[instrument]
async fn trace_on_request(p: TraceOnRequestParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.on_request = p.handler_description;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_on_response",
    description = "Set the on_response handler description for a TraceLayer."
)]
#[instrument]
async fn trace_on_response(p: TraceOnResponseParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.on_response = p.handler_description;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_on_failure",
    description = "Set the on_failure handler description for a TraceLayer."
)]
#[instrument]
async fn trace_on_failure(p: TraceOnFailureParams) -> Result<CallToolResult, ErrorData> {
    let mut config = p.config;
    config.on_failure = p.handler_description;
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&config).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_on_body_chunk",
    description = "Describe a body chunk handler for a TraceLayer."
)]
#[instrument]
async fn trace_on_body_chunk(p: TraceOnBodyChunkParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "config": p.config,
        "on_body_chunk": p.handler_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_on_eos",
    description = "Describe an end-of-stream handler for a TraceLayer."
)]
#[instrument]
async fn trace_on_eos(p: TraceOnEosParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "config": p.config,
        "on_eos": p.handler_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_make_span_with_default",
    description = "Set the span maker to the default implementation for a TraceLayer."
)]
#[instrument]
async fn trace_make_span_with_default(
    p: TraceMakeSpanWithDefaultParams,
) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "config": p.config,
        "make_span_with": "DefaultMakeSpan",
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_tracing",
    name = "trace_classify_response",
    description = "Set the response classifier description for a TraceLayer."
)]
#[instrument]
async fn trace_classify_response(
    p: TraceClassifyResponseParams,
) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "config": p.config,
        "classify_response": p.handler_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing TraceLayer configuration tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_tracing")]
pub struct TowerTracingPlugin;
