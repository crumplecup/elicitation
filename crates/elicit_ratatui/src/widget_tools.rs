//! Dual-mode widget creation tools for ratatui.
//!
//! Each tool returns a [`WidgetJson`] description that can be rendered
//! by a ratatui backend or emitted as Rust source code.

use crate::serde_types::{
    AxisJson, BarGroupJson, BlockJson, ConstraintJson, DatasetJson, LegendPositionJson,
    ListStateJson, ParagraphText, RowJson, ScrollbarOrientationJson, ScrollbarStateJson, StyleJson,
    TableStateJson, WidgetJson,
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

// ---------------------------------------------------------------------------
// Block
// ---------------------------------------------------------------------------

/// Parameters for [`widget_block`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BlockParams {
    /// Block title text.
    #[serde(default)]
    pub title: Option<String>,
    /// Which borders to draw (default: All).
    #[serde(default)]
    pub borders: Option<crate::serde_types::BordersJson>,
    /// Border line style.
    #[serde(default)]
    pub border_type: Option<crate::serde_types::BorderTypeJson>,
    /// Block background/foreground style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Border style.
    #[serde(default)]
    pub border_style: Option<StyleJson>,
    /// Inner padding.
    #[serde(default)]
    pub padding: Option<crate::serde_types::PaddingJson>,
}

/// Create a bordered container block.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_block",
    description = "Create a bordered container block with optional title. Returns WidgetJson::Block."
)]
#[instrument(skip_all)]
async fn widget_block(p: BlockParams) -> Result<CallToolResult, ErrorData> {
    let block = BlockJson {
        borders: p.borders.unwrap_or(crate::serde_types::BordersJson::All),
        border_type: p.border_type,
        title: p.title,
        style: p.style,
        border_style: p.border_style,
        padding: p.padding,
    };
    let w = WidgetJson::Block { block };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Paragraph
// ---------------------------------------------------------------------------

/// Parameters for [`widget_paragraph`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ParagraphParams {
    /// Display text — either a plain string or a richly-styled `TextJson` object.
    ///
    /// Plain: `"Hello world"`
    /// Rich: `{"lines":[{"spans":[{"content":"host","style":{"fg":"Cyan"}},{"content":": msg"}]}]}`
    pub text: ParagraphText,
    /// Text style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Enable text wrapping.
    #[serde(default)]
    pub wrap: bool,
    /// Scroll offset (row, col).
    #[serde(default)]
    pub scroll: Option<(u16, u16)>,
    /// Text alignment: "Left", "Center", "Right".
    #[serde(default)]
    pub alignment: Option<String>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
}

/// Create a text paragraph with optional wrapping and scrolling.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_paragraph",
    description = "Create a text paragraph with optional wrapping and scrolling. Returns WidgetJson::Paragraph."
)]
#[instrument(skip_all)]
async fn widget_paragraph(p: ParagraphParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Paragraph {
        text: p.text,
        style: p.style,
        wrap: p.wrap,
        scroll: p.scroll,
        alignment: p.alignment,
        block: p.block,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// List
// ---------------------------------------------------------------------------

/// Parameters for [`widget_list`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListParams {
    /// List item texts.
    pub items: Vec<String>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Item style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Selected-item highlight style.
    #[serde(default)]
    pub highlight_style: Option<StyleJson>,
    /// Selection indicator string (e.g. ">> ").
    #[serde(default)]
    pub highlight_symbol: Option<String>,
    /// Initial selection state.
    #[serde(default)]
    pub state: Option<ListStateJson>,
}

/// Create a selectable item list.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_list",
    description = "Create a selectable item list. Returns WidgetJson::List."
)]
#[instrument(skip_all)]
async fn widget_list(p: ListParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::List {
        items: p.items,
        block: p.block,
        style: p.style,
        highlight_style: p.highlight_style,
        highlight_symbol: p.highlight_symbol,
        state: p.state,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Table
// ---------------------------------------------------------------------------

/// Parameters for [`widget_table`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TableParams {
    /// Data rows.
    pub rows: Vec<RowJson>,
    /// Column width constraints.
    pub widths: Vec<ConstraintJson>,
    /// Header row.
    #[serde(default)]
    pub header: Option<RowJson>,
    /// Gap between columns.
    #[serde(default)]
    pub column_spacing: Option<u16>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Selected-row highlight style.
    #[serde(default)]
    pub highlight_style: Option<StyleJson>,
    /// Selection indicator string.
    #[serde(default)]
    pub highlight_symbol: Option<String>,
    /// Initial selection state.
    #[serde(default)]
    pub state: Option<TableStateJson>,
}

/// Create a multi-column data table.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_table",
    description = "Create a multi-column data table. Returns WidgetJson::Table."
)]
#[instrument(skip_all)]
async fn widget_table(p: TableParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Table {
        header: p.header,
        rows: p.rows,
        widths: p.widths,
        column_spacing: p.column_spacing,
        block: p.block,
        highlight_style: p.highlight_style,
        highlight_symbol: p.highlight_symbol,
        state: p.state,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Gauge
// ---------------------------------------------------------------------------

/// Parameters for [`widget_gauge`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GaugeParams {
    /// Progress ratio (0.0–1.0).
    pub ratio: f64,
    /// Optional label text.
    #[serde(default)]
    pub label: Option<String>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Gauge style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Filled-portion style.
    #[serde(default)]
    pub gauge_style: Option<StyleJson>,
}

/// Create a progress gauge.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_gauge",
    description = "Create a progress gauge (0.0–1.0). Returns WidgetJson::Gauge."
)]
#[instrument(skip_all)]
async fn widget_gauge(p: GaugeParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Gauge {
        ratio: p.ratio,
        label: p.label,
        block: p.block,
        style: p.style,
        gauge_style: p.gauge_style,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Sparkline
// ---------------------------------------------------------------------------

/// Parameters for [`widget_sparkline`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SparklineParams {
    /// Data points.
    pub data: Vec<u64>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Sparkline style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Maximum value (auto-scaled if absent).
    #[serde(default)]
    pub max: Option<u64>,
    /// Render direction.
    #[serde(default)]
    pub direction: Option<crate::serde_types::DirectionJson>,
}

/// Create a compact sparkline chart.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_sparkline",
    description = "Create a compact sparkline chart from data points. Returns WidgetJson::Sparkline."
)]
#[instrument(skip_all)]
async fn widget_sparkline(p: SparklineParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Sparkline {
        data: p.data,
        block: p.block,
        style: p.style,
        max: p.max,
        direction: p.direction,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

/// Parameters for [`widget_tabs`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TabsParams {
    /// Tab title texts.
    pub titles: Vec<String>,
    /// Currently selected tab index.
    #[serde(default)]
    pub selected: Option<usize>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Tab style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Selected-tab highlight style.
    #[serde(default)]
    pub highlight_style: Option<StyleJson>,
    /// Divider character between tabs.
    #[serde(default)]
    pub divider: Option<String>,
}

/// Create a horizontal tab selector.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_tabs",
    description = "Create a horizontal tab selector. Returns WidgetJson::Tabs."
)]
#[instrument(skip_all)]
async fn widget_tabs(p: TabsParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Tabs {
        titles: p.titles,
        selected: p.selected,
        block: p.block,
        style: p.style,
        highlight_style: p.highlight_style,
        divider: p.divider,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Clear
// ---------------------------------------------------------------------------

/// Parameters for [`widget_clear`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClearParams;

/// Clear a rectangular area.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_clear",
    description = "Clear a rectangular area. Returns WidgetJson::Clear."
)]
#[instrument(skip_all)]
async fn widget_clear(_p: ClearParams) -> Result<CallToolResult, ErrorData> {
    Ok(widget_result(&WidgetJson::Clear))
}

// ---------------------------------------------------------------------------
// BarChart
// ---------------------------------------------------------------------------

/// Parameters for [`widget_bar_chart`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BarChartParams {
    /// Bar groups.
    pub data: Vec<BarGroupJson>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Maximum bar value (auto-calculated if absent).
    #[serde(default)]
    pub max_value: Option<u64>,
    /// Bar width.
    #[serde(default)]
    pub bar_width: Option<u16>,
    /// Gap between bars in a group.
    #[serde(default)]
    pub bar_gap: Option<u16>,
    /// Gap between groups.
    #[serde(default)]
    pub group_gap: Option<u16>,
    /// Bar style.
    #[serde(default)]
    pub bar_style: Option<StyleJson>,
    /// Value label style.
    #[serde(default)]
    pub value_style: Option<StyleJson>,
    /// Label style.
    #[serde(default)]
    pub label_style: Option<StyleJson>,
    /// Layout direction.
    #[serde(default)]
    pub direction: Option<crate::serde_types::DirectionJson>,
    /// Bar chart style.
    #[serde(default)]
    pub style: Option<StyleJson>,
}

/// Create a bar chart with grouped bars.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_bar_chart",
    description = "Create a bar chart with grouped bars. Returns WidgetJson::BarChart."
)]
#[instrument(skip_all)]
async fn widget_bar_chart(p: BarChartParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::BarChart {
        data: p.data,
        block: p.block,
        max_value: p.max_value,
        bar_width: p.bar_width,
        bar_gap: p.bar_gap,
        group_gap: p.group_gap,
        bar_style: p.bar_style,
        value_style: p.value_style,
        label_style: p.label_style,
        direction: p.direction,
        style: p.style,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Chart
// ---------------------------------------------------------------------------

/// Parameters for [`widget_chart`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ChartParams {
    /// Datasets to plot.
    pub datasets: Vec<DatasetJson>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// X axis configuration.
    #[serde(default)]
    pub x_axis: Option<AxisJson>,
    /// Y axis configuration.
    #[serde(default)]
    pub y_axis: Option<AxisJson>,
    /// Chart style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Legend position.
    #[serde(default)]
    pub legend_position: Option<LegendPositionJson>,
}

/// Create a line/scatter chart with axes.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_chart",
    description = "Create a line/scatter chart with axes and datasets. Returns WidgetJson::Chart."
)]
#[instrument(skip_all)]
async fn widget_chart(p: ChartParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Chart {
        datasets: p.datasets,
        block: p.block,
        x_axis: p.x_axis,
        y_axis: p.y_axis,
        style: p.style,
        legend_position: p.legend_position,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// LineGauge
// ---------------------------------------------------------------------------

/// Parameters for [`widget_line_gauge`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LineGaugeParams {
    /// Progress ratio (0.0–1.0).
    pub ratio: f64,
    /// Optional label text.
    #[serde(default)]
    pub label: Option<String>,
    /// Optional surrounding block.
    #[serde(default)]
    pub block: Option<BlockJson>,
    /// Line gauge style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Filled portion style.
    #[serde(default)]
    pub filled_style: Option<StyleJson>,
    /// Unfilled portion style.
    #[serde(default)]
    pub unfilled_style: Option<StyleJson>,
}

/// Create a linear progress gauge.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_line_gauge",
    description = "Create a linear progress gauge (0.0–1.0). Returns WidgetJson::LineGauge."
)]
#[instrument(skip_all)]
async fn widget_line_gauge(p: LineGaugeParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::LineGauge {
        ratio: p.ratio,
        label: p.label,
        block: p.block,
        style: p.style,
        filled_style: p.filled_style,
        unfilled_style: p.unfilled_style,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Scrollbar
// ---------------------------------------------------------------------------

/// Parameters for [`widget_scrollbar`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollbarParams {
    /// Scrollbar orientation.
    pub orientation: ScrollbarOrientationJson,
    /// Thumb symbol.
    #[serde(default)]
    pub thumb_symbol: Option<String>,
    /// Track symbol.
    #[serde(default)]
    pub track_symbol: Option<String>,
    /// Begin symbol (arrow at start).
    #[serde(default)]
    pub begin_symbol: Option<String>,
    /// End symbol (arrow at end).
    #[serde(default)]
    pub end_symbol: Option<String>,
    /// Scrollbar style.
    #[serde(default)]
    pub style: Option<StyleJson>,
    /// Thumb style.
    #[serde(default)]
    pub thumb_style: Option<StyleJson>,
    /// Track style.
    #[serde(default)]
    pub track_style: Option<StyleJson>,
    /// State.
    #[serde(default)]
    pub state: Option<ScrollbarStateJson>,
}

/// Create a scrollbar indicator.
#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_scrollbar",
    description = "Create a scrollbar indicator with configurable orientation and symbols. Returns WidgetJson::Scrollbar."
)]
#[instrument(skip_all)]
async fn widget_scrollbar(p: ScrollbarParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Scrollbar {
        orientation: p.orientation,
        thumb_symbol: p.thumb_symbol,
        track_symbol: p.track_symbol,
        begin_symbol: p.begin_symbol,
        end_symbol: p.end_symbol,
        style: p.style,
        thumb_style: p.thumb_style,
        track_style: p.track_style,
        state: p.state,
    };
    Ok(widget_result(&w))
}
