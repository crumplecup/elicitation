//! Widget property setter tools for ratatui.
//!
//! Each tool takes an existing [`WidgetJson`] and sets a single property,
//! returning the modified widget. This enables incremental builder-style
//! construction over MCP.

use crate::serde_types::{
    AxisJson, BlockJson, BorderTypeJson, BordersJson, DirectionJson, LegendPositionJson,
    ListStateJson, PaddingJson, RowJson, ScrollbarStateJson, StyleJson, TableStateJson, WidgetJson,
};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use elicitation::elicit_tool;

/// Serialise a widget to a JSON `CallToolResult`.
fn widget_result(widget: &WidgetJson) -> CallToolResult {
    match serde_json::to_string(widget) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Return an error indicating the wrong widget variant was supplied.
fn wrong_variant(expected: &str, widget: &WidgetJson) -> CallToolResult {
    CallToolResult::error(vec![Content::text(format!(
        "Expected {expected} widget, got {:?}",
        std::mem::discriminant(widget),
    ))])
}

// ---------------------------------------------------------------------------
// Block properties
// ---------------------------------------------------------------------------

/// Parameters for `block_set_title`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockSetTitleParams {
    /// The widget to modify (must be a Block variant).
    pub widget: WidgetJson,
    /// New title text.
    pub title: String,
}

/// Set the title on a Block widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "block_set_title",
    description = "Set the title on a Block widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn block_set_title(p: BlockSetTitleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Block { block } => {
            block.title = Some(p.title);
        }
        other => return Ok(wrong_variant("Block", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `block_set_borders`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockSetBordersParams {
    /// The widget to modify (must be a Block variant).
    pub widget: WidgetJson,
    /// New borders value.
    pub borders: BordersJson,
}

/// Set the borders on a Block widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "block_set_borders",
    description = "Set the borders on a Block widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn block_set_borders(p: BlockSetBordersParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Block { block } => {
            block.borders = p.borders;
        }
        other => return Ok(wrong_variant("Block", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `block_set_border_type`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockSetBorderTypeParams {
    /// The widget to modify (must be a Block variant).
    pub widget: WidgetJson,
    /// New border type.
    pub border_type: BorderTypeJson,
}

/// Set the border type on a Block widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "block_set_border_type",
    description = "Set the border type on a Block widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn block_set_border_type(p: BlockSetBorderTypeParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Block { block } => {
            block.border_type = Some(p.border_type);
        }
        other => return Ok(wrong_variant("Block", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `block_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockSetStyleParams {
    /// The widget to modify (must be a Block variant).
    pub widget: WidgetJson,
    /// New block style.
    pub style: StyleJson,
}

/// Set the style on a Block widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "block_set_style",
    description = "Set the style on a Block widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn block_set_style(p: BlockSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Block { block } => {
            block.style = Some(p.style);
        }
        other => return Ok(wrong_variant("Block", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `block_set_border_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockSetBorderStyleParams {
    /// The widget to modify (must be a Block variant).
    pub widget: WidgetJson,
    /// New border style.
    pub border_style: StyleJson,
}

/// Set the border style on a Block widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "block_set_border_style",
    description = "Set the border style on a Block widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn block_set_border_style(p: BlockSetBorderStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Block { block } => {
            block.border_style = Some(p.border_style);
        }
        other => return Ok(wrong_variant("Block", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `block_set_padding`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockSetPaddingParams {
    /// The widget to modify (must be a Block variant).
    pub widget: WidgetJson,
    /// New inner padding.
    pub padding: PaddingJson,
}

/// Set the inner padding on a Block widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "block_set_padding",
    description = "Set the inner padding on a Block widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn block_set_padding(p: BlockSetPaddingParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Block { block } => {
            block.padding = Some(p.padding);
        }
        other => return Ok(wrong_variant("Block", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Paragraph properties
// ---------------------------------------------------------------------------

/// Parameters for `paragraph_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParagraphSetStyleParams {
    /// The widget to modify (must be a Paragraph variant).
    pub widget: WidgetJson,
    /// New text style.
    pub style: StyleJson,
}

/// Set the text style on a Paragraph widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "paragraph_set_style",
    description = "Set the text style on a Paragraph widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn paragraph_set_style(p: ParagraphSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Paragraph { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("Paragraph", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `paragraph_set_wrap`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParagraphSetWrapParams {
    /// The widget to modify (must be a Paragraph variant).
    pub widget: WidgetJson,
    /// Enable or disable text wrapping.
    pub wrap: bool,
}

/// Enable or disable text wrapping on a Paragraph widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "paragraph_set_wrap",
    description = "Enable or disable text wrapping on a Paragraph widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn paragraph_set_wrap(p: ParagraphSetWrapParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Paragraph { wrap, .. } => {
            *wrap = p.wrap;
        }
        other => return Ok(wrong_variant("Paragraph", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `paragraph_set_scroll`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParagraphSetScrollParams {
    /// The widget to modify (must be a Paragraph variant).
    pub widget: WidgetJson,
    /// Vertical scroll offset.
    pub row: u16,
    /// Horizontal scroll offset.
    pub col: u16,
}

/// Set the scroll offset on a Paragraph widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "paragraph_set_scroll",
    description = "Set the scroll offset on a Paragraph widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn paragraph_set_scroll(p: ParagraphSetScrollParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Paragraph { scroll, .. } => {
            *scroll = Some((p.row, p.col));
        }
        other => return Ok(wrong_variant("Paragraph", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `paragraph_set_alignment`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParagraphSetAlignmentParams {
    /// The widget to modify (must be a Paragraph variant).
    pub widget: WidgetJson,
    /// Text alignment: "Left", "Center", or "Right".
    pub alignment: String,
}

/// Set the text alignment on a Paragraph widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "paragraph_set_alignment",
    description = "Set the text alignment on a Paragraph widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn paragraph_set_alignment(
    p: ParagraphSetAlignmentParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Paragraph { alignment, .. } => {
            *alignment = Some(p.alignment);
        }
        other => return Ok(wrong_variant("Paragraph", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `paragraph_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParagraphSetBlockParams {
    /// The widget to modify (must be a Paragraph variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a Paragraph widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "paragraph_set_block",
    description = "Set the surrounding block on a Paragraph widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn paragraph_set_block(p: ParagraphSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Paragraph { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("Paragraph", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// List properties
// ---------------------------------------------------------------------------

/// Parameters for `list_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSetBlockParams {
    /// The widget to modify (must be a List variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a List widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "list_set_block",
    description = "Set the surrounding block on a List widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn list_set_block(p: ListSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::List { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("List", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `list_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSetStyleParams {
    /// The widget to modify (must be a List variant).
    pub widget: WidgetJson,
    /// New item style.
    pub style: StyleJson,
}

/// Set the item style on a List widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "list_set_style",
    description = "Set the item style on a List widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn list_set_style(p: ListSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::List { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("List", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `list_set_highlight_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSetHighlightStyleParams {
    /// The widget to modify (must be a List variant).
    pub widget: WidgetJson,
    /// New selection highlight style.
    pub highlight_style: StyleJson,
}

/// Set the selection highlight style on a List widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "list_set_highlight_style",
    description = "Set the selection highlight style on a List widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn list_set_highlight_style(
    p: ListSetHighlightStyleParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::List {
            highlight_style, ..
        } => {
            *highlight_style = Some(p.highlight_style);
        }
        other => return Ok(wrong_variant("List", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `list_set_highlight_symbol`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSetHighlightSymbolParams {
    /// The widget to modify (must be a List variant).
    pub widget: WidgetJson,
    /// New selection marker string.
    pub highlight_symbol: String,
}

/// Set the selection marker on a List widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "list_set_highlight_symbol",
    description = "Set the selection marker on a List widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn list_set_highlight_symbol(
    p: ListSetHighlightSymbolParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::List {
            highlight_symbol, ..
        } => {
            *highlight_symbol = Some(p.highlight_symbol);
        }
        other => return Ok(wrong_variant("List", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `list_set_state`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListSetStateParams {
    /// The widget to modify (must be a List variant).
    pub widget: WidgetJson,
    /// New list state (selected index and offset).
    pub state: ListStateJson,
}

/// Set the state on a List widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "list_set_state",
    description = "Set the state (selected index and offset) on a List widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn list_set_state(p: ListSetStateParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::List { state, .. } => {
            *state = Some(p.state);
        }
        other => return Ok(wrong_variant("List", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Table properties
// ---------------------------------------------------------------------------

/// Parameters for `table_set_header`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableSetHeaderParams {
    /// The widget to modify (must be a Table variant).
    pub widget: WidgetJson,
    /// New header row.
    pub header: RowJson,
}

/// Set the header row on a Table widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "table_set_header",
    description = "Set the header row on a Table widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn table_set_header(p: TableSetHeaderParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Table { header, .. } => {
            *header = Some(p.header);
        }
        other => return Ok(wrong_variant("Table", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `table_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableSetBlockParams {
    /// The widget to modify (must be a Table variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a Table widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "table_set_block",
    description = "Set the surrounding block on a Table widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn table_set_block(p: TableSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Table { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("Table", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `table_set_highlight_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableSetHighlightStyleParams {
    /// The widget to modify (must be a Table variant).
    pub widget: WidgetJson,
    /// New selection highlight style.
    pub highlight_style: StyleJson,
}

/// Set the selection highlight style on a Table widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "table_set_highlight_style",
    description = "Set the selection highlight style on a Table widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn table_set_highlight_style(
    p: TableSetHighlightStyleParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Table {
            highlight_style, ..
        } => {
            *highlight_style = Some(p.highlight_style);
        }
        other => return Ok(wrong_variant("Table", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `table_set_highlight_symbol`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableSetHighlightSymbolParams {
    /// The widget to modify (must be a Table variant).
    pub widget: WidgetJson,
    /// New selection marker string.
    pub highlight_symbol: String,
}

/// Set the selection marker on a Table widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "table_set_highlight_symbol",
    description = "Set the selection marker on a Table widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn table_set_highlight_symbol(
    p: TableSetHighlightSymbolParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Table {
            highlight_symbol, ..
        } => {
            *highlight_symbol = Some(p.highlight_symbol);
        }
        other => return Ok(wrong_variant("Table", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `table_set_column_spacing`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableSetColumnSpacingParams {
    /// The widget to modify (must be a Table variant).
    pub widget: WidgetJson,
    /// Gap between columns.
    pub column_spacing: u16,
}

/// Set the gap between columns on a Table widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "table_set_column_spacing",
    description = "Set the gap between columns on a Table widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn table_set_column_spacing(
    p: TableSetColumnSpacingParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Table { column_spacing, .. } => {
            *column_spacing = Some(p.column_spacing);
        }
        other => return Ok(wrong_variant("Table", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `table_set_state`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableSetStateParams {
    /// The widget to modify (must be a Table variant).
    pub widget: WidgetJson,
    /// New table state.
    pub state: TableStateJson,
}

/// Set the state on a Table widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "table_set_state",
    description = "Set the state (selected row and offset) on a Table widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn table_set_state(p: TableSetStateParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Table { state, .. } => {
            *state = Some(p.state);
        }
        other => return Ok(wrong_variant("Table", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Gauge properties
// ---------------------------------------------------------------------------

/// Parameters for `gauge_set_label`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GaugeSetLabelParams {
    /// The widget to modify (must be a Gauge variant).
    pub widget: WidgetJson,
    /// New label text.
    pub label: String,
}

/// Set the label on a Gauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "gauge_set_label",
    description = "Set the label on a Gauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn gauge_set_label(p: GaugeSetLabelParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Gauge { label, .. } => {
            *label = Some(p.label);
        }
        other => return Ok(wrong_variant("Gauge", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `gauge_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GaugeSetBlockParams {
    /// The widget to modify (must be a Gauge variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a Gauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "gauge_set_block",
    description = "Set the surrounding block on a Gauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn gauge_set_block(p: GaugeSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Gauge { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("Gauge", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `gauge_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GaugeSetStyleParams {
    /// The widget to modify (must be a Gauge variant).
    pub widget: WidgetJson,
    /// New gauge style.
    pub style: StyleJson,
}

/// Set the style on a Gauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "gauge_set_style",
    description = "Set the style on a Gauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn gauge_set_style(p: GaugeSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Gauge { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("Gauge", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `gauge_set_gauge_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GaugeSetGaugeStyleParams {
    /// The widget to modify (must be a Gauge variant).
    pub widget: WidgetJson,
    /// New filled-portion style.
    pub gauge_style: StyleJson,
}

/// Set the filled-portion style on a Gauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "gauge_set_gauge_style",
    description = "Set the filled-portion style on a Gauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn gauge_set_gauge_style(p: GaugeSetGaugeStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Gauge { gauge_style, .. } => {
            *gauge_style = Some(p.gauge_style);
        }
        other => return Ok(wrong_variant("Gauge", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Sparkline properties
// ---------------------------------------------------------------------------

/// Parameters for `sparkline_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SparklineSetBlockParams {
    /// The widget to modify (must be a Sparkline variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a Sparkline widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "sparkline_set_block",
    description = "Set the surrounding block on a Sparkline widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn sparkline_set_block(p: SparklineSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Sparkline { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("Sparkline", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `sparkline_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SparklineSetStyleParams {
    /// The widget to modify (must be a Sparkline variant).
    pub widget: WidgetJson,
    /// New sparkline style.
    pub style: StyleJson,
}

/// Set the style on a Sparkline widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "sparkline_set_style",
    description = "Set the style on a Sparkline widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn sparkline_set_style(p: SparklineSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Sparkline { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("Sparkline", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `sparkline_set_max`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SparklineSetMaxParams {
    /// The widget to modify (must be a Sparkline variant).
    pub widget: WidgetJson,
    /// New maximum value.
    pub max: u64,
}

/// Set the maximum value on a Sparkline widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "sparkline_set_max",
    description = "Set the maximum value on a Sparkline widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn sparkline_set_max(p: SparklineSetMaxParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Sparkline { max, .. } => {
            *max = Some(p.max);
        }
        other => return Ok(wrong_variant("Sparkline", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `sparkline_set_direction`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SparklineSetDirectionParams {
    /// The widget to modify (must be a Sparkline variant).
    pub widget: WidgetJson,
    /// New render direction.
    pub direction: DirectionJson,
}

/// Set the render direction on a Sparkline widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "sparkline_set_direction",
    description = "Set the render direction on a Sparkline widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn sparkline_set_direction(
    p: SparklineSetDirectionParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Sparkline { direction, .. } => {
            *direction = Some(p.direction);
        }
        other => return Ok(wrong_variant("Sparkline", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Tabs properties
// ---------------------------------------------------------------------------

/// Parameters for `tabs_set_selected`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabsSetSelectedParams {
    /// The widget to modify (must be a Tabs variant).
    pub widget: WidgetJson,
    /// Index of the selected tab.
    pub selected: usize,
}

/// Set the selected tab on a Tabs widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "tabs_set_selected",
    description = "Set the selected tab index on a Tabs widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn tabs_set_selected(p: TabsSetSelectedParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Tabs { selected, .. } => {
            *selected = Some(p.selected);
        }
        other => return Ok(wrong_variant("Tabs", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `tabs_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabsSetBlockParams {
    /// The widget to modify (must be a Tabs variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a Tabs widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "tabs_set_block",
    description = "Set the surrounding block on a Tabs widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn tabs_set_block(p: TabsSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Tabs { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("Tabs", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `tabs_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabsSetStyleParams {
    /// The widget to modify (must be a Tabs variant).
    pub widget: WidgetJson,
    /// New tab style.
    pub style: StyleJson,
}

/// Set the tab style on a Tabs widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "tabs_set_style",
    description = "Set the tab style on a Tabs widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn tabs_set_style(p: TabsSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Tabs { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("Tabs", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `tabs_set_highlight_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabsSetHighlightStyleParams {
    /// The widget to modify (must be a Tabs variant).
    pub widget: WidgetJson,
    /// New selected-tab highlight style.
    pub highlight_style: StyleJson,
}

/// Set the selected-tab highlight style on a Tabs widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "tabs_set_highlight_style",
    description = "Set the selected-tab highlight style on a Tabs widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn tabs_set_highlight_style(
    p: TabsSetHighlightStyleParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Tabs {
            highlight_style, ..
        } => {
            *highlight_style = Some(p.highlight_style);
        }
        other => return Ok(wrong_variant("Tabs", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `tabs_set_divider`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabsSetDividerParams {
    /// The widget to modify (must be a Tabs variant).
    pub widget: WidgetJson,
    /// New divider character.
    pub divider: String,
}

/// Set the divider character on a Tabs widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "tabs_set_divider",
    description = "Set the divider character on a Tabs widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn tabs_set_divider(p: TabsSetDividerParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Tabs { divider, .. } => {
            *divider = Some(p.divider);
        }
        other => return Ok(wrong_variant("Tabs", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// BarChart properties
// ---------------------------------------------------------------------------

/// Parameters for `bar_chart_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetBlockParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_block",
    description = "Set the surrounding block on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_block(p: BarChartSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `bar_chart_set_max_value`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetMaxValueParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// New maximum bar value.
    pub max_value: u64,
}

/// Set the maximum bar value on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_max_value",
    description = "Set the maximum bar value on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_max_value(
    p: BarChartSetMaxValueParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { max_value, .. } => {
            *max_value = Some(p.max_value);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `bar_chart_set_bar_width`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetBarWidthParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// New bar width.
    pub bar_width: u16,
}

/// Set the bar width on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_bar_width",
    description = "Set the bar width on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_bar_width(
    p: BarChartSetBarWidthParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { bar_width, .. } => {
            *bar_width = Some(p.bar_width);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `bar_chart_set_bar_gap`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetBarGapParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// New gap between bars.
    pub bar_gap: u16,
}

/// Set the gap between bars on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_bar_gap",
    description = "Set the gap between bars on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_bar_gap(p: BarChartSetBarGapParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { bar_gap, .. } => {
            *bar_gap = Some(p.bar_gap);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `bar_chart_set_bar_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetBarStyleParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// New bar style.
    pub bar_style: StyleJson,
}

/// Set the bar style on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_bar_style",
    description = "Set the bar style on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_bar_style(
    p: BarChartSetBarStyleParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { bar_style, .. } => {
            *bar_style = Some(p.bar_style);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `bar_chart_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetStyleParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// New overall chart style.
    pub style: StyleJson,
}

/// Set the overall style on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_style",
    description = "Set the overall style on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_style(p: BarChartSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `bar_chart_set_direction`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartSetDirectionParams {
    /// The widget to modify (must be a BarChart variant).
    pub widget: WidgetJson,
    /// New layout direction.
    pub direction: DirectionJson,
}

/// Set the layout direction on a BarChart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "bar_chart_set_direction",
    description = "Set the layout direction on a BarChart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn bar_chart_set_direction(
    p: BarChartSetDirectionParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::BarChart { direction, .. } => {
            *direction = Some(p.direction);
        }
        other => return Ok(wrong_variant("BarChart", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Chart properties
// ---------------------------------------------------------------------------

/// Parameters for `chart_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChartSetBlockParams {
    /// The widget to modify (must be a Chart variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a Chart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "chart_set_block",
    description = "Set the surrounding block on a Chart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn chart_set_block(p: ChartSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Chart { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("Chart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `chart_set_x_axis`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChartSetXAxisParams {
    /// The widget to modify (must be a Chart variant).
    pub widget: WidgetJson,
    /// New X axis configuration.
    pub x_axis: AxisJson,
}

/// Set the X axis on a Chart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "chart_set_x_axis",
    description = "Set the X axis on a Chart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn chart_set_x_axis(p: ChartSetXAxisParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Chart { x_axis, .. } => {
            *x_axis = Some(p.x_axis);
        }
        other => return Ok(wrong_variant("Chart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `chart_set_y_axis`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChartSetYAxisParams {
    /// The widget to modify (must be a Chart variant).
    pub widget: WidgetJson,
    /// New Y axis configuration.
    pub y_axis: AxisJson,
}

/// Set the Y axis on a Chart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "chart_set_y_axis",
    description = "Set the Y axis on a Chart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn chart_set_y_axis(p: ChartSetYAxisParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Chart { y_axis, .. } => {
            *y_axis = Some(p.y_axis);
        }
        other => return Ok(wrong_variant("Chart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `chart_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChartSetStyleParams {
    /// The widget to modify (must be a Chart variant).
    pub widget: WidgetJson,
    /// New chart style.
    pub style: StyleJson,
}

/// Set the style on a Chart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "chart_set_style",
    description = "Set the style on a Chart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn chart_set_style(p: ChartSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Chart { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("Chart", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `chart_set_legend_position`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChartSetLegendPositionParams {
    /// The widget to modify (must be a Chart variant).
    pub widget: WidgetJson,
    /// New legend position.
    pub legend_position: LegendPositionJson,
}

/// Set the legend position on a Chart widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "chart_set_legend_position",
    description = "Set the legend position on a Chart widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn chart_set_legend_position(
    p: ChartSetLegendPositionParams,
) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Chart {
            legend_position, ..
        } => {
            *legend_position = Some(p.legend_position);
        }
        other => return Ok(wrong_variant("Chart", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// LineGauge properties
// ---------------------------------------------------------------------------

/// Parameters for `line_gauge_set_label`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LineGaugeSetLabelParams {
    /// The widget to modify (must be a LineGauge variant).
    pub widget: WidgetJson,
    /// New label text.
    pub label: String,
}

/// Set the label on a LineGauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "line_gauge_set_label",
    description = "Set the label on a LineGauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn line_gauge_set_label(p: LineGaugeSetLabelParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::LineGauge { label, .. } => {
            *label = Some(p.label);
        }
        other => return Ok(wrong_variant("LineGauge", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `line_gauge_set_block`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LineGaugeSetBlockParams {
    /// The widget to modify (must be a LineGauge variant).
    pub widget: WidgetJson,
    /// Surrounding block.
    pub block: BlockJson,
}

/// Set the surrounding block on a LineGauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "line_gauge_set_block",
    description = "Set the surrounding block on a LineGauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn line_gauge_set_block(p: LineGaugeSetBlockParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::LineGauge { block, .. } => {
            *block = Some(p.block);
        }
        other => return Ok(wrong_variant("LineGauge", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `line_gauge_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LineGaugeSetStyleParams {
    /// The widget to modify (must be a LineGauge variant).
    pub widget: WidgetJson,
    /// New style.
    pub style: StyleJson,
}

/// Set the style on a LineGauge widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "line_gauge_set_style",
    description = "Set the style on a LineGauge widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn line_gauge_set_style(p: LineGaugeSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::LineGauge { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("LineGauge", other)),
    }
    Ok(widget_result(&widget))
}

// ---------------------------------------------------------------------------
// Scrollbar properties
// ---------------------------------------------------------------------------

/// Parameters for `scrollbar_set_style`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollbarSetStyleParams {
    /// The widget to modify (must be a Scrollbar variant).
    pub widget: WidgetJson,
    /// New scrollbar style.
    pub style: StyleJson,
}

/// Set the style on a Scrollbar widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "scrollbar_set_style",
    description = "Set the style on a Scrollbar widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn scrollbar_set_style(p: ScrollbarSetStyleParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Scrollbar { style, .. } => {
            *style = Some(p.style);
        }
        other => return Ok(wrong_variant("Scrollbar", other)),
    }
    Ok(widget_result(&widget))
}

/// Parameters for `scrollbar_set_state`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollbarSetStateParams {
    /// The widget to modify (must be a Scrollbar variant).
    pub widget: WidgetJson,
    /// New scrollbar state.
    pub state: ScrollbarStateJson,
}

/// Set the state on a Scrollbar widget.
#[elicit_tool(
    plugin = "ratatui_properties",
    name = "scrollbar_set_state",
    description = "Set the state on a Scrollbar widget. Returns the modified WidgetJson."
)]
#[instrument(skip_all)]
async fn scrollbar_set_state(p: ScrollbarSetStateParams) -> Result<CallToolResult, ErrorData> {
    let mut widget = p.widget;
    match &mut widget {
        WidgetJson::Scrollbar { state, .. } => {
            *state = Some(p.state);
        }
        other => return Ok(wrong_variant("Scrollbar", other)),
    }
    Ok(widget_result(&widget))
}
