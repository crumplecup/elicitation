//! `AxumResponseSsePlugin` — MCP tools for axum SSE streaming responses.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes a Server-Sent Events event.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SseEventDescriptor {
    /// Optional event ID.
    pub id: Option<String>,
    /// Optional event name (the `event:` field).
    pub event_name: Option<String>,
    /// Event data payload.
    pub data: String,
    /// Optional retry interval in milliseconds.
    pub retry_ms: Option<u64>,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for sse_event_create.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseEventParams {
    /// Event data payload.
    pub data: String,
}

/// Parameters for sse_event_with_id.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseEventWithIdParams {
    /// Event data payload.
    pub data: String,
    /// Event ID.
    pub id: String,
}

/// Parameters for sse_event_with_retry.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseEventWithRetryParams {
    /// Event data payload.
    pub data: String,
    /// Retry interval in milliseconds.
    pub retry_ms: u64,
}

/// Parameters for sse_event_data.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseEventDescriptorInput {
    /// The SSE event descriptor.
    pub event: SseEventDescriptor,
}

/// Parameters for sse_event_event_name.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseEventNameParams {
    /// The SSE event descriptor to update.
    pub event: SseEventDescriptor,
    /// The event name to set.
    pub name: String,
}

/// Parameters for sse_stream_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseStreamDescribeParams {
    /// The Rust type name of events in the stream.
    pub event_type: String,
}

/// Parameters for sse_keepalive_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SseKeepaliveParams {
    /// Keepalive interval in seconds.
    pub interval_secs: u64,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_event_create",
    emit = None,
    description = "Create a basic SSE event descriptor with only a data payload."
)]
#[instrument]
async fn sse_event_create(p: SseEventParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = SseEventDescriptor {
        id: None,
        event_name: None,
        data: p.data,
        retry_ms: None,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_event_with_id",
    emit = None,
    description = "Create an SSE event descriptor with a data payload and an event ID."
)]
#[instrument]
async fn sse_event_with_id(p: SseEventWithIdParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = SseEventDescriptor {
        id: Some(p.id),
        event_name: None,
        data: p.data,
        retry_ms: None,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_event_with_retry",
    emit = None,
    description = "Create an SSE event descriptor with a data payload and a retry interval."
)]
#[instrument]
async fn sse_event_with_retry(p: SseEventWithRetryParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = SseEventDescriptor {
        id: None,
        event_name: None,
        data: p.data,
        retry_ms: Some(p.retry_ms),
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_event_data",
    emit = None,
    description = "Extract the data payload from an SSE event descriptor."
)]
#[instrument]
async fn sse_event_data(p: SseEventDescriptorInput) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(p.event.data)]))
}

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_event_event_name",
    emit = None,
    description = "Set the event name on an SSE event descriptor and return the updated descriptor."
)]
#[instrument]
async fn sse_event_event_name(p: SseEventNameParams) -> Result<CallToolResult, ErrorData> {
    let mut event = p.event.clone();
    event.event_name = Some(p.name);
    let val = serde_json::to_string(&event).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_stream_describe",
    emit = None,
    description = "Describe how to create an axum SSE stream for a given event type."
)]
#[instrument]
async fn sse_stream_describe(p: SseStreamDescribeParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "Sse stream of {} events. Use axum::response::sse::Sse::new(stream) to create the SSE \
         response. The stream must yield Event items.",
        p.event_type
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_response_sse",
    name = "sse_keepalive_describe",
    emit = None,
    description = "Describe SSE keepalive configuration for a given interval."
)]
#[instrument]
async fn sse_keepalive_describe(p: SseKeepaliveParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "SSE keepalive configured with {} second interval. Sends ':keepalive\\n\\n' comments to \
         prevent connection timeout.",
        p.interval_secs
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

/// Plugin exposing axum SSE streaming response tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_response_sse")]
pub struct AxumResponseSsePlugin;
