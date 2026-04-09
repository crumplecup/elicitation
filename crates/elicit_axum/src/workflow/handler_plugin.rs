//! `AxumHandlerPlugin` — MCP tools for axum Handler trait and service utilities.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// Describes an axum handler function.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HandlerDescriptor {
    /// Handler function name.
    pub name: String,
    /// List of extractor type names.
    pub extractors: Vec<String>,
    /// Return type of the handler.
    pub return_type: String,
    /// Human-readable description.
    pub description: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for handler_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HandlerDescribeParams {
    /// Rust function signature string to parse.
    pub fn_signature: String,
}

/// Parameters for handler_with_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HandlerWithStateParams {
    /// The handler descriptor.
    pub handler: HandlerDescriptor,
    /// The state type to inject.
    pub state_type: String,
}

/// Parameters for tools that take only a handler descriptor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HandlerDescriptorInput {
    /// The handler descriptor.
    pub handler: HandlerDescriptor,
}

/// Parameters for handler_layer.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HandlerLayerParams {
    /// The handler descriptor.
    pub handler: HandlerDescriptor,
    /// Human-readable description of the layer.
    pub layer_description: String,
}

/// Parameters for into_service_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntoServiceParams {
    /// The service type name.
    pub service_type: String,
}

/// Parameters for on_upgrade_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OnUpgradeParams {
    /// The upgrade protocol (e.g. "WebSocket").
    pub protocol: String,
}

/// Parameters for app_error_describe.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppErrorParams {
    /// The error type name.
    pub error_type: String,
    /// The Display impl output string.
    pub display_impl: String,
}

/// Parameters for error_response_status.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ErrorResponseStatusParams {
    /// The error type name.
    pub error_type: String,
    /// The HTTP status code.
    pub status: u16,
}

/// Parameters for error_response_json.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ErrorResponseJsonParams {
    /// The error type name.
    pub error_type: String,
    /// A JSON template string for the error body.
    pub json_template: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_handler",
    name = "handler_describe",
    emit = None,
    description = "Parse a Rust function signature string and return a HandlerDescriptor."
)]
#[instrument]
async fn handler_describe(p: HandlerDescribeParams) -> Result<CallToolResult, ErrorData> {
    let sig = p.fn_signature.trim();
    let name = sig
        .find('(')
        .map(|i| sig[..i].trim().to_string())
        .unwrap_or_else(|| sig.to_string());
    let extractors = sig
        .find('(')
        .and_then(|start| sig.find(')').map(|end| (start, end)))
        .map(|(start, end)| {
            let inner = &sig[start + 1..end];
            inner
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let return_type = sig
        .find("->")
        .map(|i| sig[i + 2..].trim().to_string())
        .unwrap_or_default();
    let description = format!("Handler function '{}'", name);
    let descriptor = HandlerDescriptor {
        name,
        extractors,
        return_type,
        description,
    };
    let val = serde_json::to_string(&descriptor).unwrap_or_default();
    Ok(CallToolResult::success(vec![Content::text(val)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "handler_with_state",
    emit = None,
    description = "Describe calling .with_state() on a handler to inject state."
)]
#[instrument]
async fn handler_with_state(p: HandlerWithStateParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{}.with_state::<{}>() — creates a Service from the handler with state injected",
        p.handler.name, p.state_type
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "handler_boxed",
    emit = None,
    description = "Describe calling .boxed() on a handler to erase its concrete type."
)]
#[instrument]
async fn handler_boxed(p: HandlerDescriptorInput) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{}.boxed() — boxes the handler to erase the concrete type, returning BoxedHandler<S>",
        p.handler.name
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "handler_layer",
    emit = None,
    description = "Describe wrapping a handler with a tower Layer."
)]
#[instrument]
async fn handler_layer(p: HandlerLayerParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{}.layer({}) — wraps the handler with a tower Layer",
        p.handler.name, p.layer_description
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "handler_into_service",
    emit = None,
    description = "Describe converting a handler into a tower Service."
)]
#[instrument]
async fn handler_into_service(p: HandlerDescriptorInput) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{}.into_service() — converts the handler into a tower Service",
        p.handler.name
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "handler_make_service",
    emit = None,
    description = "Describe converting a handler into an IntoMakeService."
)]
#[instrument]
async fn handler_make_service(p: HandlerDescriptorInput) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{}.into_make_service() — converts the handler into an IntoMakeService",
        p.handler.name
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "into_service_describe",
    emit = None,
    description = "Describe IntoMakeService for a given service type."
)]
#[instrument]
async fn into_service_describe(p: IntoServiceParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "IntoMakeService<{}> — wraps a service to implement MakeService, enabling it to be used \
         with axum::serve()",
        p.service_type
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "on_upgrade_describe",
    emit = None,
    description = "Describe handling a WebSocket upgrade for a given protocol."
)]
#[instrument]
async fn on_upgrade_describe(p: OnUpgradeParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "WebSocket upgrade handler for {} protocol. Use axum::extract::WebSocketUpgrade to \
         handle the upgrade request.",
        p.protocol
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "app_error_describe",
    emit = None,
    description = "Describe an application error type that implements IntoResponse via Display."
)]
#[instrument]
async fn app_error_describe(p: AppErrorParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{} implements IntoResponse via Display: '{}'. Implement IntoResponse to return custom \
         error responses.",
        p.error_type, p.display_impl
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "error_response_status",
    emit = None,
    description = "Describe an error type's mapping to an HTTP status code in IntoResponse."
)]
#[instrument]
async fn error_response_status(p: ErrorResponseStatusParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{} maps to HTTP {} status in IntoResponse impl",
        p.error_type, p.status
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "axum_handler",
    name = "error_response_json",
    emit = None,
    description = "Describe an error type that returns a JSON error response body."
)]
#[instrument]
async fn error_response_json(p: ErrorResponseJsonParams) -> Result<CallToolResult, ErrorData> {
    let text = format!(
        "{} returns JSON error response with template: {}",
        p.error_type, p.json_template
    );
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

/// Plugin exposing axum handler and service utility tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_handler")]
pub struct AxumHandlerPlugin;
