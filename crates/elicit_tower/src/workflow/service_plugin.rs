//! `TowerServicePlugin` — Tower service factory MCP tools.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> ErrorData {
    ErrorData::internal_error(e.to_string(), None)
}

// ── Types ─────────────────────────────────────────────────────────────────────

/// Description of a created Tower service.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceCreated {
    /// Unique identifier for this service.
    pub service_id: String,
    /// Service kind/type.
    pub kind: String,
    /// Human-readable description.
    pub description: String,
    /// Whether the service is immediately ready.
    pub ready: bool,
}

/// Parameters for create_service.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceSpec {
    /// Service kind (e.g. "echo", "proxy", "static").
    pub kind: String,
    /// Description of what this service does.
    pub description: String,
}

/// Parameters for call_service.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CallServiceParams {
    /// The service identifier.
    pub service_id: String,
    /// The request encoded as a JSON string.
    pub request_json: String,
}

/// Parameters for service_map.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceTransformParams {
    /// The service identifier.
    pub service_id: String,
    /// Description of the map function to apply.
    pub map_fn_description: String,
}

/// Parameters for service_layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceLayerParams {
    /// The service identifier.
    pub service_id: String,
    /// The kind of layer to apply.
    pub layer_kind: String,
}

/// Parameters for poll_ready.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PollReadyParams {
    /// The service identifier.
    pub service_id: String,
}

/// Parameters for service_map_err.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceMapErrParams {
    /// The service identifier.
    pub service_id: String,
    /// Description of the error-mapping function.
    pub map_fn_description: String,
}

/// Parameters for service_boxed.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceBoxedParams {
    /// The service identifier.
    pub service_id: String,
}

/// Parameters for service_into_make_service.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceIntoMakeServiceParams {
    /// The service identifier.
    pub service_id: String,
}

/// Parameters for service_then.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceThenParams {
    /// The service identifier.
    pub service_id: String,
    /// Description of the next service.
    pub then_description: String,
}

/// Parameters for service_and_then.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServiceAndThenParams {
    /// The service identifier.
    pub service_id: String,
    /// Description of the next service.
    pub then_description: String,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_service",
    name = "create_service",
    description = "Create a new Tower service descriptor."
)]
#[instrument]
async fn create_service(p: ServiceSpec) -> Result<CallToolResult, ErrorData> {
    let result = ServiceCreated {
        service_id: uuid::Uuid::new_v4().to_string(),
        kind: p.kind,
        description: p.description,
        ready: true,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "call_service",
    description = "Describe calling a Tower service with a given request."
)]
#[instrument]
async fn call_service(p: CallServiceParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": p.service_id,
        "request_json": p.request_json,
        "description": format!("Called service '{}' with request: {}", p.service_id, p.request_json),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "poll_ready",
    description = "Poll a Tower service for readiness."
)]
#[instrument]
async fn poll_ready(p: PollReadyParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": p.service_id,
        "ready": true,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_map",
    description = "Apply a map transformation to a Tower service's responses."
)]
#[instrument]
async fn service_map(p: ServiceTransformParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "transform": "map",
        "description": p.map_fn_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_then",
    description = "Chain a Tower service with a subsequent service."
)]
#[instrument]
async fn service_then(p: ServiceThenParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "transform": "then",
        "description": p.then_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_and_then",
    description = "Chain a Tower service with an async subsequent service."
)]
#[instrument]
async fn service_and_then(p: ServiceAndThenParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "transform": "and_then",
        "description": p.then_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_map_err",
    description = "Apply an error-mapping function to a Tower service."
)]
#[instrument]
async fn service_map_err(p: ServiceMapErrParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "transform": "map_err",
        "description": p.map_fn_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_boxed",
    description = "Box a Tower service to erase its type."
)]
#[instrument]
async fn service_boxed(p: ServiceBoxedParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "transform": "boxed",
        "description": format!("Service '{}' wrapped in BoxService", p.service_id),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_layer",
    description = "Apply a Tower layer to a service."
)]
#[instrument]
async fn service_layer(p: ServiceLayerParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "layer_kind": p.layer_kind,
        "description": format!("Service '{}' wrapped with '{}' layer", p.service_id, p.layer_kind),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_service",
    name = "service_into_make_service",
    description = "Convert a Tower service into a MakeService."
)]
#[instrument]
async fn service_into_make_service(
    p: ServiceIntoMakeServiceParams,
) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.service_id,
        "transform": "into_make_service",
        "description": format!("Service '{}' converted to MakeService", p.service_id),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing Tower service factory tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_service")]
pub struct TowerServicePlugin;
