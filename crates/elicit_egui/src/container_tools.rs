//! Dual-mode container creation tools.
//!
//! Each tool returns a [`ContainerJson`] variant describing a layout container.

use elicitation::elicit_tool;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::serde_types::{
    ColorJson, ContainerJson, CornerRadiusJson, MarginJson, StrokeJson, Vec2Json,
};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn container_result(container: &ContainerJson) -> CallToolResult {
    match serde_json::to_string(container) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Empty params for tools that take no arguments.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyContainerParams {}

// ---------------------------------------------------------------------------
// Window
// ---------------------------------------------------------------------------

/// Parameters for [`container_window`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowParams {
    /// Window title.
    pub title: String,
    /// Initial position (x, y).
    pub default_pos: Option<Vec2Json>,
    /// Initial size (width, height).
    pub default_size: Option<Vec2Json>,
    /// Whether the window is resizable.
    #[serde(default = "default_true")]
    pub resizable: bool,
    /// Whether the window is collapsible.
    #[serde(default = "default_true")]
    pub collapsible: bool,
    /// Whether to enable scrolling.
    #[serde(default)]
    pub scroll: bool,
    /// Whether to show the title bar.
    #[serde(default = "default_true")]
    pub title_bar: bool,
}

fn default_true() -> bool {
    true
}

/// Create a floating window container.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_window",
    description = "Create a floating window. Returns ContainerJson::Window."
)]
#[instrument(skip_all)]
async fn container_window(p: WindowParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::Window {
        title: p.title,
        default_pos: p.default_pos,
        default_size: p.default_size,
        resizable: p.resizable,
        collapsible: p.collapsible,
        scroll: p.scroll,
        title_bar: p.title_bar,
    };
    Ok(container_result(&c))
}

// ---------------------------------------------------------------------------
// Panels
// ---------------------------------------------------------------------------

/// Parameters for [`container_left_panel`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LeftPanelParams {
    /// Panel identifier.
    pub id: String,
    /// Default panel width.
    pub default_width: Option<f32>,
    /// Whether the panel is resizable.
    #[serde(default = "default_true")]
    pub resizable: bool,
    /// Minimum panel width.
    pub min_width: Option<f32>,
    /// Maximum panel width.
    pub max_width: Option<f32>,
}

/// Create a left side panel.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_left_panel",
    description = "Create a left side panel. Returns ContainerJson::LeftPanel."
)]
#[instrument(skip_all)]
async fn container_left_panel(p: LeftPanelParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::LeftPanel {
        id: p.id,
        default_width: p.default_width,
        resizable: p.resizable,
        min_width: p.min_width,
        max_width: p.max_width,
    };
    Ok(container_result(&c))
}

/// Parameters for [`container_right_panel`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RightPanelParams {
    /// Panel identifier.
    pub id: String,
    /// Default panel width.
    pub default_width: Option<f32>,
    /// Whether the panel is resizable.
    #[serde(default = "default_true")]
    pub resizable: bool,
}

/// Create a right side panel.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_right_panel",
    description = "Create a right side panel. Returns ContainerJson::RightPanel."
)]
#[instrument(skip_all)]
async fn container_right_panel(p: RightPanelParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::RightPanel {
        id: p.id,
        default_width: p.default_width,
        resizable: p.resizable,
    };
    Ok(container_result(&c))
}

/// Parameters for [`container_top_panel`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TopPanelParams {
    /// Panel identifier.
    pub id: String,
    /// Default panel height.
    pub default_height: Option<f32>,
    /// Whether the panel is resizable.
    #[serde(default)]
    pub resizable: bool,
}

/// Create a top panel.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_top_panel",
    description = "Create a top panel. Returns ContainerJson::TopPanel."
)]
#[instrument(skip_all)]
async fn container_top_panel(p: TopPanelParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::TopPanel {
        id: p.id,
        default_height: p.default_height,
        resizable: p.resizable,
    };
    Ok(container_result(&c))
}

/// Parameters for [`container_bottom_panel`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct BottomPanelParams {
    /// Panel identifier.
    pub id: String,
    /// Default panel height.
    pub default_height: Option<f32>,
    /// Whether the panel is resizable.
    #[serde(default)]
    pub resizable: bool,
}

/// Create a bottom panel.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_bottom_panel",
    description = "Create a bottom panel. Returns ContainerJson::BottomPanel."
)]
#[instrument(skip_all)]
async fn container_bottom_panel(p: BottomPanelParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::BottomPanel {
        id: p.id,
        default_height: p.default_height,
        resizable: p.resizable,
    };
    Ok(container_result(&c))
}

/// Create a central panel (fills remaining space).
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_central_panel",
    description = "Create a central panel that fills remaining space. Returns ContainerJson::CentralPanel."
)]
#[instrument(skip_all)]
async fn container_central_panel(p: EmptyContainerParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(container_result(&ContainerJson::CentralPanel))
}

// ---------------------------------------------------------------------------
// Scroll area
// ---------------------------------------------------------------------------

/// Parameters for [`container_scroll_area`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ScrollAreaParams {
    /// Enable vertical scrolling.
    #[serde(default = "default_true")]
    pub vertical: bool,
    /// Enable horizontal scrolling.
    #[serde(default)]
    pub horizontal: bool,
    /// Maximum height before scrolling.
    pub max_height: Option<f32>,
    /// Maximum width before scrolling.
    pub max_width: Option<f32>,
    /// Whether to auto-shrink to content.
    #[serde(default)]
    pub auto_shrink: bool,
    /// Whether to always show scroll bars.
    #[serde(default)]
    pub always_show_scroll: bool,
}

/// Create a scrollable region.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_scroll_area",
    description = "Create a scrollable region. Returns ContainerJson::ScrollArea."
)]
#[instrument(skip_all)]
async fn container_scroll_area(p: ScrollAreaParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::ScrollArea {
        vertical: p.vertical,
        horizontal: p.horizontal,
        max_height: p.max_height,
        max_width: p.max_width,
        auto_shrink: p.auto_shrink,
        always_show_scroll: p.always_show_scroll,
    };
    Ok(container_result(&c))
}

// ---------------------------------------------------------------------------
// Collapsing, group, frame
// ---------------------------------------------------------------------------

/// Parameters for [`container_collapsing`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CollapsingParams {
    /// Header text.
    pub text: String,
    /// Whether the section starts open.
    #[serde(default)]
    pub default_open: bool,
}

/// Create a collapsible section.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_collapsing",
    description = "Create a collapsible section with header. Returns ContainerJson::CollapsingHeader."
)]
#[instrument(skip_all)]
async fn container_collapsing(p: CollapsingParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::CollapsingHeader {
        text: p.text,
        default_open: p.default_open,
    };
    Ok(container_result(&c))
}

/// Create a visual grouping (box around content).
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_group",
    description = "Create a visual grouping box around content. Returns ContainerJson::Group."
)]
#[instrument(skip_all)]
async fn container_group(p: EmptyContainerParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(container_result(&ContainerJson::Group))
}

/// Parameters for [`container_frame`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct FrameParams {
    /// Optional fill colour.
    pub fill: Option<ColorJson>,
    /// Optional border stroke.
    pub stroke: Option<StrokeJson>,
    /// Optional corner rounding.
    pub corner_radius: Option<CornerRadiusJson>,
    /// Optional inner margin.
    pub inner_margin: Option<MarginJson>,
    /// Optional outer margin.
    pub outer_margin: Option<MarginJson>,
}

/// Create a framed region with custom styling.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_frame",
    description = "Create a framed region with fill, stroke, and margins. Returns ContainerJson::Frame."
)]
#[instrument(skip_all)]
async fn container_frame(p: FrameParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::Frame {
        fill: p.fill,
        stroke: p.stroke,
        corner_radius: p.corner_radius,
        inner_margin: p.inner_margin,
        outer_margin: p.outer_margin,
    };
    Ok(container_result(&c))
}

// ---------------------------------------------------------------------------
// Menu & tooltip
// ---------------------------------------------------------------------------

/// Create a menu bar container.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_menu_bar",
    description = "Create a menu bar. Returns ContainerJson::MenuBar."
)]
#[instrument(skip_all)]
async fn container_menu_bar(p: EmptyContainerParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(container_result(&ContainerJson::MenuBar))
}

/// Parameters for [`container_menu`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MenuParams {
    /// Menu title.
    pub title: String,
}

/// Create a menu within a menu bar.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_menu",
    description = "Create a menu within a menu bar. Returns ContainerJson::Menu."
)]
#[instrument(skip_all)]
async fn container_menu(p: MenuParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::Menu { title: p.title };
    Ok(container_result(&c))
}

/// Parameters for [`container_tooltip`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TooltipParams {
    /// Tooltip text.
    pub text: String,
}

/// Create a tooltip container.
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_tooltip",
    description = "Create a tooltip. Returns ContainerJson::Tooltip."
)]
#[instrument(skip_all)]
async fn container_tooltip(p: TooltipParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::Tooltip { text: p.text };
    Ok(container_result(&c))
}

/// Parameters for [`container_popup`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct PopupParams {
    /// Popup identifier.
    pub id: String,
}

/// Create a popup area (context menu, dropdown).
#[elicit_tool(
    plugin = "egui_containers",
    name = "container_popup",
    description = "Create a popup area. Returns ContainerJson::Popup."
)]
#[instrument(skip_all)]
async fn container_popup(p: PopupParams) -> Result<CallToolResult, ErrorData> {
    let c = ContainerJson::Popup { id: p.id };
    Ok(container_result(&c))
}
