//! `AxumCoreFromRefPlugin` — FromRef factory MCP tools.

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

/// Description of a `FromRef` extraction from state.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FromRefDescriptor {
    /// The source state type (e.g. `"AppState"`).
    pub source_type: String,
    /// The target sub-state type extracted via `FromRef` (e.g. `"DatabasePool"`).
    pub target_type: String,
    /// Human-readable description of the extraction.
    pub description: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for `from_ref_describe`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRefDescribeParams {
    /// The source state type.
    pub source_type: String,
    /// The target type extracted from the source.
    pub target_type: String,
}

/// Parameters for `from_ref_simulate`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRefSimulateParams {
    /// The source state type.
    pub source_type: String,
    /// The target type extracted from the source.
    pub target_type: String,
    /// A JSON representation of the source state value.
    pub source_json: String,
}

/// Parameters for `state_clone_from_ref`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StateCloneParams {
    /// The overall application state type.
    pub state_type: String,
    /// The name of the field being extracted.
    pub field_name: String,
    /// The type of the field being extracted.
    pub field_type: String,
}

/// Parameters for `state_nested_from_ref`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NestedStateParams {
    /// The outer state type containing the inner state.
    pub outer_type: String,
    /// The inner state type being extracted.
    pub inner_type: String,
    /// A description of the nesting relationship.
    pub description: String,
}

/// Parameters for `arc_from_ref`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ArcFromRefParams {
    /// The inner type wrapped in `Arc<T>`.
    pub inner_type: String,
}

/// Parameters for `from_ref_chain`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FromRefChainParams {
    /// The sequence of types in the `FromRef` extraction chain (outermost first).
    pub types: Vec<String>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_from_ref",
    name = "from_ref_describe",
    description = "Describe how FromRef extracts a sub-state value from a larger application state type."
)]
#[instrument]
async fn from_ref_describe(p: FromRefDescribeParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = FromRefDescriptor {
        source_type: p.source_type.clone(),
        target_type: p.target_type.clone(),
        description: format!(
            "`{}` implements `FromRef<{}>`. Axum calls `{}::from_ref(&state)` to extract \
             a `{}` from the shared application state. Used by extractors that declare \
             `S: FromRef<{}>` in their `FromRequestParts` impl.",
            p.target_type, p.source_type, p.target_type, p.target_type, p.source_type
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_ref",
    name = "from_ref_simulate",
    description = "Simulate extracting a sub-state value using FromRef, given a JSON representation of the source state."
)]
#[instrument]
async fn from_ref_simulate(p: FromRefSimulateParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = FromRefDescriptor {
        source_type: p.source_type.clone(),
        target_type: p.target_type.clone(),
        description: format!(
            "Simulate `{}::from_ref(&state)` where `state: {}` = `{}`. \
             This clones or borrows the relevant sub-field to produce a `{}`.",
            p.target_type, p.source_type, p.source_json, p.target_type
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_ref",
    name = "state_clone_from_ref",
    description = "Describe the FromRef implementation that clones a specific field from application state."
)]
#[instrument]
async fn state_clone_from_ref(p: StateCloneParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = FromRefDescriptor {
        source_type: p.state_type.clone(),
        target_type: p.field_type.clone(),
        description: format!(
            "Extracts field `{}` of type `{}` from `{}` state by cloning. \
             Implement `FromRef<{}>` for `{}` by returning `state.{}.clone()`. \
             This is the standard pattern for sharing sub-state with axum extractors.",
            p.field_name, p.field_type, p.state_type, p.state_type, p.field_type, p.field_name
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_ref",
    name = "state_nested_from_ref",
    description = "Describe how nested state types compose via FromRef, allowing inner state to be extracted from outer state."
)]
#[instrument]
async fn state_nested_from_ref(p: NestedStateParams) -> Result<CallToolResult, ErrorData> {
    let descriptor = FromRefDescriptor {
        source_type: p.outer_type.clone(),
        target_type: p.inner_type.clone(),
        description: format!(
            "Nested `FromRef`: `{}` contains `{}`. {}. \
             Axum traverses the `FromRef` chain to satisfy extractor state bounds. \
             `{}` can be extracted from handlers that hold `{}` as their state.",
            p.outer_type, p.inner_type, p.description, p.inner_type, p.outer_type
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_ref",
    name = "arc_from_ref",
    description = "Describe how Arc<T> implements FromRef<Arc<T>>, allowing cheap cloning of shared state."
)]
#[instrument]
async fn arc_from_ref(p: ArcFromRefParams) -> Result<CallToolResult, ErrorData> {
    let arc_type = format!("Arc<{}>", p.inner_type);
    let descriptor = FromRefDescriptor {
        source_type: arc_type.clone(),
        target_type: arc_type.clone(),
        description: format!(
            "`Arc<{}>` implements `FromRef<Arc<{}>>` via the blanket impl in axum-core. \
             `from_ref` simply clones the `Arc`, incrementing the reference count. \
             This is the recommended way to share expensive-to-clone state \
             (e.g. database pools, configuration) across handlers.",
            p.inner_type, p.inner_type
        ),
    };
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&descriptor).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "axum_from_ref",
    name = "from_ref_chain",
    description = "Describe a chain of FromRef extractions, where each step extracts a sub-state from the previous type."
)]
#[instrument]
async fn from_ref_chain(p: FromRefChainParams) -> Result<CallToolResult, ErrorData> {
    if p.types.is_empty() {
        return Err(ErrorData::invalid_params(
            "from_ref_chain requires at least one type",
            None,
        ));
    }
    let chain_desc: Vec<String> = p
        .types
        .windows(2)
        .map(|w| format!("`{}` → `{}`", w[0], w[1]))
        .collect();
    let result = serde_json::json!({
        "chain": p.types,
        "steps": chain_desc,
        "description": format!(
            "FromRef extraction chain of {} type(s): {}. \
             Axum resolves each step by calling `from_ref` on the accumulated state, \
             allowing deeply nested sub-states to be accessed by extractors.",
            p.types.len(),
            if chain_desc.is_empty() {
                p.types.first().cloned().unwrap_or_default()
            } else {
                chain_desc.join(", ")
            }
        ),
    });
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Plugin exposing axum-core `FromRef` factory tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_from_ref")]
pub struct AxumCoreFromRefPlugin;
