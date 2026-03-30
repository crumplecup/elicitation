//! Dual-mode layout and constraint tools for ratatui.
//!
//! Each tool returns a [`TuiNode`] or [`ConstraintJson`] description
//! that can be rendered by a ratatui backend or emitted as Rust source code.

use crate::serde_types::{ConstraintJson, DirectionJson, MarginJson, TuiNode};
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use elicitation::elicit_tool;

/// Serialise a layout node to a JSON `CallToolResult`.
fn layout_result(node: &TuiNode) -> CallToolResult {
    match serde_json::to_string(node) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Serialise a constraint to a JSON `CallToolResult`.
fn constraint_result(constraint: &ConstraintJson) -> CallToolResult {
    match serde_json::to_string(constraint) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ---------------------------------------------------------------------------
// Layout — vertical
// ---------------------------------------------------------------------------

/// Parameters for [`layout_vertical`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LayoutVerticalParams {
    /// Size constraints for each child slot.
    pub constraints: Vec<ConstraintJson>,
    /// Outer margin.
    #[serde(default)]
    pub margin: Option<MarginJson>,
}

/// Create a vertical layout split with constraints.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "layout_vertical",
    description = "Create a vertical layout split with constraints. Returns TuiNode::Layout with direction Vertical."
)]
#[instrument(skip_all)]
async fn layout_vertical(p: LayoutVerticalParams) -> Result<CallToolResult, ErrorData> {
    let node = TuiNode::Layout {
        direction: DirectionJson::Vertical,
        constraints: p.constraints,
        children: Vec::new(),
        margin: p.margin,
    };
    Ok(layout_result(&node))
}

// ---------------------------------------------------------------------------
// Layout — horizontal
// ---------------------------------------------------------------------------

/// Parameters for [`layout_horizontal`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LayoutHorizontalParams {
    /// Size constraints for each child slot.
    pub constraints: Vec<ConstraintJson>,
    /// Outer margin.
    #[serde(default)]
    pub margin: Option<MarginJson>,
}

/// Create a horizontal layout split with constraints.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "layout_horizontal",
    description = "Create a horizontal layout split with constraints. Returns TuiNode::Layout with direction Horizontal."
)]
#[instrument(skip_all)]
async fn layout_horizontal(p: LayoutHorizontalParams) -> Result<CallToolResult, ErrorData> {
    let node = TuiNode::Layout {
        direction: DirectionJson::Horizontal,
        constraints: p.constraints,
        children: Vec::new(),
        margin: p.margin,
    };
    Ok(layout_result(&node))
}

// ---------------------------------------------------------------------------
// Constraint — length
// ---------------------------------------------------------------------------

/// Parameters for [`constraint_length`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConstraintLengthParams {
    /// Exact length in rows or columns.
    pub value: u16,
}

/// Create a fixed-length constraint.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "constraint_length",
    description = "Create a fixed-length constraint (exact rows/columns). Returns ConstraintJson::Length."
)]
#[instrument(skip_all)]
async fn constraint_length(p: ConstraintLengthParams) -> Result<CallToolResult, ErrorData> {
    let c = ConstraintJson::Length { value: p.value };
    Ok(constraint_result(&c))
}

// ---------------------------------------------------------------------------
// Constraint — percentage
// ---------------------------------------------------------------------------

/// Parameters for [`constraint_percentage`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConstraintPercentageParams {
    /// Percentage of available space (0–100).
    pub value: u16,
}

/// Create a percentage constraint.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "constraint_percentage",
    description = "Create a percentage constraint (0–100% of available space). Returns ConstraintJson::Percentage."
)]
#[instrument(skip_all)]
async fn constraint_percentage(p: ConstraintPercentageParams) -> Result<CallToolResult, ErrorData> {
    let c = ConstraintJson::Percentage { value: p.value };
    Ok(constraint_result(&c))
}

// ---------------------------------------------------------------------------
// Constraint — min
// ---------------------------------------------------------------------------

/// Parameters for [`constraint_min`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConstraintMinParams {
    /// Minimum length in rows or columns.
    pub value: u16,
}

/// Create a minimum length constraint.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "constraint_min",
    description = "Create a minimum length constraint. Returns ConstraintJson::Min."
)]
#[instrument(skip_all)]
async fn constraint_min(p: ConstraintMinParams) -> Result<CallToolResult, ErrorData> {
    let c = ConstraintJson::Min { value: p.value };
    Ok(constraint_result(&c))
}

// ---------------------------------------------------------------------------
// Constraint — max
// ---------------------------------------------------------------------------

/// Parameters for [`constraint_max`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConstraintMaxParams {
    /// Maximum length in rows or columns.
    pub value: u16,
}

/// Create a maximum length constraint.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "constraint_max",
    description = "Create a maximum length constraint. Returns ConstraintJson::Max."
)]
#[instrument(skip_all)]
async fn constraint_max(p: ConstraintMaxParams) -> Result<CallToolResult, ErrorData> {
    let c = ConstraintJson::Max { value: p.value };
    Ok(constraint_result(&c))
}

// ---------------------------------------------------------------------------
// Constraint — fill
// ---------------------------------------------------------------------------

/// Parameters for [`constraint_fill`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConstraintFillParams {
    /// Fill weight (higher = more remaining space).
    pub value: u16,
}

/// Create a fill remaining space constraint.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "constraint_fill",
    description = "Create a fill remaining space constraint with proportional weight. Returns ConstraintJson::Fill."
)]
#[instrument(skip_all)]
async fn constraint_fill(p: ConstraintFillParams) -> Result<CallToolResult, ErrorData> {
    let c = ConstraintJson::Fill { value: p.value };
    Ok(constraint_result(&c))
}

// ---------------------------------------------------------------------------
// Constraint — ratio
// ---------------------------------------------------------------------------

/// Parameters for [`constraint_ratio`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ConstraintRatioParams {
    /// Numerator.
    pub num: u32,
    /// Denominator.
    pub den: u32,
}

/// Create a ratio constraint.
#[elicit_tool(
    plugin = "ratatui_layout",
    name = "constraint_ratio",
    description = "Create a ratio constraint (num/den of available space). Returns ConstraintJson::Ratio."
)]
#[instrument(skip_all)]
async fn constraint_ratio(p: ConstraintRatioParams) -> Result<CallToolResult, ErrorData> {
    let c = ConstraintJson::Ratio {
        num: p.num,
        den: p.den,
    };
    Ok(constraint_result(&c))
}
