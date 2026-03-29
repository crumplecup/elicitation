//! Dual-mode layout tools.
//!
//! Each tool returns a [`LayoutJson`] variant describing how widgets
//! are arranged within a container.

use elicitation::elicit_tool;
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::serde_types::{LayoutAlign, LayoutJson, Vec2Json};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn layout_result(layout: &LayoutJson) -> CallToolResult {
    match serde_json::to_string(layout) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Empty params for layout tools with no arguments.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyLayoutParams {}

// ---------------------------------------------------------------------------
// Basic layouts
// ---------------------------------------------------------------------------

/// Parameters for [`layout_horizontal`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HorizontalParams {
    /// Cross-axis (vertical) alignment.
    pub align: Option<LayoutAlign>,
}

/// Arrange widgets horizontally (left to right).
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_horizontal",
    description = "Arrange widgets horizontally. Returns LayoutJson::Horizontal."
)]
#[instrument(skip_all)]
async fn layout_horizontal(p: HorizontalParams) -> Result<CallToolResult, ErrorData> {
    let l = LayoutJson::Horizontal { align: p.align };
    Ok(layout_result(&l))
}

/// Parameters for [`layout_vertical`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct VerticalParams {
    /// Cross-axis (horizontal) alignment.
    pub align: Option<LayoutAlign>,
}

/// Arrange widgets vertically (top to bottom).
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_vertical",
    description = "Arrange widgets vertically. Returns LayoutJson::Vertical."
)]
#[instrument(skip_all)]
async fn layout_vertical(p: VerticalParams) -> Result<CallToolResult, ErrorData> {
    let l = LayoutJson::Vertical { align: p.align };
    Ok(layout_result(&l))
}

/// Arrange widgets horizontally with centred cross-axis.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_horizontal_centered",
    description = "Horizontal layout with centred cross-axis. Returns LayoutJson::HorizontalCentered."
)]
#[instrument(skip_all)]
async fn layout_horizontal_centered(
    p: EmptyLayoutParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(layout_result(&LayoutJson::HorizontalCentered))
}

/// Arrange widgets vertically with centred cross-axis.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_vertical_centered",
    description = "Vertical layout with centred cross-axis. Returns LayoutJson::VerticalCentered."
)]
#[instrument(skip_all)]
async fn layout_vertical_centered(
    p: EmptyLayoutParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(layout_result(&LayoutJson::VerticalCentered))
}

/// Arrange widgets horizontally, justified (items stretch to fill).
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_horizontal_justified",
    description = "Horizontal layout with justified items. Returns LayoutJson::HorizontalJustified."
)]
#[instrument(skip_all)]
async fn layout_horizontal_justified(
    p: EmptyLayoutParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(layout_result(&LayoutJson::HorizontalJustified))
}

/// Arrange widgets vertically, justified.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_vertical_justified",
    description = "Vertical layout with justified items. Returns LayoutJson::VerticalJustified."
)]
#[instrument(skip_all)]
async fn layout_vertical_justified(
    p: EmptyLayoutParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(layout_result(&LayoutJson::VerticalJustified))
}

/// Arrange widgets horizontally, wrapping to next line.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_horizontal_wrapped",
    description = "Horizontal layout that wraps to next line. Returns LayoutJson::HorizontalWrapped."
)]
#[instrument(skip_all)]
async fn layout_horizontal_wrapped(
    p: EmptyLayoutParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(layout_result(&LayoutJson::HorizontalWrapped))
}

// ---------------------------------------------------------------------------
// Columns and grid
// ---------------------------------------------------------------------------

/// Parameters for [`layout_columns`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ColumnsParams {
    /// Number of columns.
    pub count: usize,
}

/// Create a column-based layout.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_columns",
    description = "Create a column-based layout. Returns LayoutJson::Columns."
)]
#[instrument(skip_all)]
async fn layout_columns(p: ColumnsParams) -> Result<CallToolResult, ErrorData> {
    let l = LayoutJson::Columns { count: p.count };
    Ok(layout_result(&l))
}

/// Parameters for [`layout_grid`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GridParams {
    /// Grid identifier.
    pub id: String,
    /// Number of columns.
    pub num_columns: Option<usize>,
    /// Whether to stripe alternating rows.
    #[serde(default)]
    pub striped: bool,
    /// Minimum column width.
    pub min_col_width: Option<f32>,
    /// Maximum column width.
    pub max_col_width: Option<f32>,
    /// Cell spacing.
    pub spacing: Option<Vec2Json>,
}

/// Create a grid layout.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_grid",
    description = "Create a grid layout. Returns LayoutJson::Grid."
)]
#[instrument(skip_all)]
async fn layout_grid(p: GridParams) -> Result<CallToolResult, ErrorData> {
    let l = LayoutJson::Grid {
        id: p.id,
        num_columns: p.num_columns,
        striped: p.striped,
        min_col_width: p.min_col_width,
        max_col_width: p.max_col_width,
        spacing: p.spacing,
    };
    Ok(layout_result(&l))
}

// ---------------------------------------------------------------------------
// Spacing and indentation
// ---------------------------------------------------------------------------

/// Parameters for [`layout_indent`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct IndentParams {
    /// Indentation amount in logical pixels.
    pub indent: Option<f32>,
}

/// Add indentation to following widgets.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_indent",
    description = "Add indentation. Returns LayoutJson::Indent."
)]
#[instrument(skip_all)]
async fn layout_indent(p: IndentParams) -> Result<CallToolResult, ErrorData> {
    let l = LayoutJson::Indent { indent: p.indent };
    Ok(layout_result(&l))
}

/// Parameters for [`layout_add_space`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddSpaceParams {
    /// Space amount in logical pixels.
    pub amount: f32,
}

/// Add explicit spacing between widgets.
#[elicit_tool(
    plugin = "egui_layout",
    name = "layout_add_space",
    description = "Add explicit spacing between widgets. Returns LayoutJson::AddSpace."
)]
#[instrument(skip_all)]
async fn layout_add_space(p: AddSpaceParams) -> Result<CallToolResult, ErrorData> {
    let l = LayoutJson::AddSpace { amount: p.amount };
    Ok(layout_result(&l))
}
