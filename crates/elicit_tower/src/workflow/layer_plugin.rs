//! `TowerLayerPlugin` — Tower layer factory MCP tools.

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

/// Description of a created Tower layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerCreated {
    /// Unique identifier for this layer.
    pub layer_id: String,
    /// Layer kind/type.
    pub kind: String,
}

/// Parameters for create_layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerSpec {
    /// Layer kind (e.g. "timeout", "cors", "compression").
    pub kind: String,
    /// Description of what this layer does.
    pub description: String,
}

/// Parameters for apply_layer.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ApplyLayerParams {
    /// The layer identifier.
    pub layer_id: String,
    /// Description of the service to wrap.
    pub service_description: String,
}

/// Parameters for layer_map.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerTransformParams {
    /// The layer identifier.
    pub layer_id: String,
    /// Description of the map transformation.
    pub map_description: String,
}

/// Parameters for layer_then.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerChainParams {
    /// The layer identifier.
    pub layer_id: String,
    /// Description of the next layer.
    pub then_description: String,
}

/// Parameters for layer_identity (no inputs).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerIdentityParams {}

/// Parameters for layer_into_inner.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerIntoInnerParams {
    /// The layer identifier.
    pub layer_id: String,
}

/// Parameters for layer_boxed.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerBoxedParams {
    /// The layer identifier.
    pub layer_id: String,
}

/// Parameters for layer_stack.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerStackParams {
    /// The outer layer identifier.
    pub outer_layer_id: String,
    /// The inner layer identifier.
    pub inner_layer_id: String,
}

/// Parameters for layer_option.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerOptionParams {
    /// The layer identifier.
    pub layer_id: String,
    /// Whether the layer is enabled.
    pub enabled: bool,
}

/// Parameters for layer_chain with many layers.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LayerChainManyParams {
    /// The ordered list of layer identifiers to chain.
    pub layer_ids: Vec<String>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "tower_layer",
    name = "create_layer",
    description = "Create a new Tower layer descriptor."
)]
#[instrument]
async fn create_layer(p: LayerSpec) -> Result<CallToolResult, ErrorData> {
    let result = LayerCreated {
        layer_id: uuid::Uuid::new_v4().to_string(),
        kind: p.kind,
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "apply_layer",
    description = "Apply a Tower layer to a service, producing a new wrapped service."
)]
#[instrument]
async fn apply_layer(p: ApplyLayerParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "layer_id": p.layer_id,
        "description": format!("Layer '{}' applied to service: {}", p.layer_id, p.service_description),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_map",
    description = "Apply a map transformation to a Tower layer's wrapped service."
)]
#[instrument]
async fn layer_map(p: LayerTransformParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.layer_id,
        "transform": "map",
        "description": p.map_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_then",
    description = "Chain a Tower layer with a subsequent layer."
)]
#[instrument]
async fn layer_then(p: LayerChainParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.layer_id,
        "transform": "then",
        "description": p.then_description,
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_identity",
    description = "Create a Tower identity layer that passes requests through unchanged."
)]
#[instrument]
async fn layer_identity(_p: LayerIdentityParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "kind": "identity",
        "description": "Identity layer — passes requests through unchanged",
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_stack",
    description = "Stack two Tower layers, applying the outer layer around the inner."
)]
#[instrument]
async fn layer_stack(p: LayerStackParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "outer_layer_id": p.outer_layer_id,
        "inner_layer_id": p.inner_layer_id,
        "description": format!("Stack: outer='{}' wrapping inner='{}'", p.outer_layer_id, p.inner_layer_id),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_option",
    description = "Wrap a Tower layer in an Option to conditionally apply it."
)]
#[instrument]
async fn layer_option(p: LayerOptionParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.layer_id,
        "enabled": p.enabled,
        "description": format!("Optional layer '{}' (enabled={})", p.layer_id, p.enabled),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_into_inner",
    description = "Unwrap the inner service from a Tower layer."
)]
#[instrument]
async fn layer_into_inner(p: LayerIntoInnerParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "service_id": uuid::Uuid::new_v4().to_string(),
        "from_layer": p.layer_id,
        "description": format!("Inner service unwrapped from layer '{}'", p.layer_id),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_boxed",
    description = "Box a Tower layer to erase its type."
)]
#[instrument]
async fn layer_boxed(p: LayerBoxedParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "based_on": p.layer_id,
        "transform": "boxed",
        "description": format!("Layer '{}' wrapped in BoxLayer", p.layer_id),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "tower_layer",
    name = "layer_chain",
    description = "Chain multiple Tower layers together in order."
)]
#[instrument]
async fn layer_chain(p: LayerChainManyParams) -> Result<CallToolResult, ErrorData> {
    let result = serde_json::json!({
        "layer_id": uuid::Uuid::new_v4().to_string(),
        "chained_layers": p.layer_ids,
        "description": format!("Chained {} layers in order", p.layer_ids.len()),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing Tower layer factory tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "tower_layer")]
pub struct TowerLayerPlugin;
