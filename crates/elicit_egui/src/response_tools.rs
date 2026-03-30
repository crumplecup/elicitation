//! Dual-mode response-checking and interaction tools.
//!
//! These tools query the result of a widget interaction, providing
//! a declarative way to inspect [`egui::Response`] properties.

use elicitation::elicit_tool;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::serde_types::{RectJson, Vec2Json};

// ---------------------------------------------------------------------------
// Response query JSON types
// ---------------------------------------------------------------------------

/// Serializable response query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ResponseQueryJson {
    /// Check if widget was clicked.
    Clicked,
    /// Check if widget was double-clicked.
    DoubleClicked,
    /// Check if widget was secondary-clicked (right-click).
    SecondaryClicked,
    /// Check if widget is being hovered.
    Hovered,
    /// Check if widget has keyboard focus.
    HasFocus,
    /// Check if widget gained focus.
    GainedFocus,
    /// Check if widget lost focus.
    LostFocus,
    /// Check if widget is being dragged.
    Dragged,
    /// Check if drag was released.
    DragReleased,
    /// Check if the value changed.
    Changed,
    /// Get the widget bounding rect.
    Rect,
    /// Get the drag delta (how much the pointer moved while dragging).
    DragDelta,
    /// Get the hover position.
    HoverPos,
    /// Request focus for the widget.
    RequestFocus,
    /// Surrender focus from the widget.
    SurrenderFocus,
    /// Show a tooltip on hover with given text.
    ShowTooltip {
        /// Tooltip text.
        text: String,
    },
    /// Set the enabled state.
    SetEnabled {
        /// Whether the widget is enabled.
        enabled: bool,
    },
    /// Highlight the widget (e.g. during a tutorial).
    Highlight,
    /// Scroll to make the widget visible.
    ScrollToMe,
    /// Check if widget was clicked N times.
    ClickedN {
        /// Click count.
        count: usize,
    },
    /// Context menu response.
    ContextMenu,
}

/// Serializable response info (what the runtime returns).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum ResponseInfoJson {
    /// Boolean result.
    Bool {
        /// The result.
        value: bool,
    },
    /// Rectangle result.
    Rect {
        /// The bounding rectangle.
        rect: RectJson,
    },
    /// Vector/delta result.
    Vec2 {
        /// The vector value.
        value: Vec2Json,
    },
    /// Optional position result.
    OptionalPos {
        /// The position, if any.
        pos: Option<Vec2Json>,
    },
    /// Action performed (no data returned).
    Action,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn query_result(query: &ResponseQueryJson) -> CallToolResult {
    match serde_json::to_string(query) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Empty params for no-argument response tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyResponseParams {}

// ---------------------------------------------------------------------------
// Click / hover checks
// ---------------------------------------------------------------------------

/// Check if a widget was clicked.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_clicked",
    description = "Check if the widget was clicked. Returns ResponseQueryJson::Clicked."
)]
#[instrument(skip_all)]
async fn response_clicked(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::Clicked))
}

/// Check if a widget was double-clicked.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_double_clicked",
    description = "Check if the widget was double-clicked. Returns ResponseQueryJson::DoubleClicked.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_double_clicked(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::DoubleClicked))
}

/// Check if a widget was right-clicked.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_secondary_clicked",
    description = "Check if the widget was right-clicked. Returns ResponseQueryJson::SecondaryClicked.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_secondary_clicked(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::SecondaryClicked))
}

/// Parameters for [`response_clicked_n`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ClickedNParams {
    /// Click count to check.
    pub count: usize,
}

/// Check if a widget was clicked N times.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_clicked_n",
    description = "Check if the widget was clicked N times. Returns ResponseQueryJson::ClickedN."
)]
#[instrument(skip_all)]
async fn response_clicked_n(p: ClickedNParams) -> Result<CallToolResult, ErrorData> {
    let q = ResponseQueryJson::ClickedN { count: p.count };
    Ok(query_result(&q))
}

/// Check if a widget is hovered.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_hovered",
    description = "Check if the widget is being hovered. Returns ResponseQueryJson::Hovered.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_hovered(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::Hovered))
}

// ---------------------------------------------------------------------------
// Focus
// ---------------------------------------------------------------------------

/// Check if a widget has keyboard focus.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_has_focus",
    description = "Check if the widget has keyboard focus. Returns ResponseQueryJson::HasFocus.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_has_focus(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::HasFocus))
}

/// Check if a widget gained focus.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_gained_focus",
    description = "Check if the widget just gained focus. Returns ResponseQueryJson::GainedFocus.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_gained_focus(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::GainedFocus))
}

/// Check if a widget lost focus.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_lost_focus",
    description = "Check if the widget just lost focus. Returns ResponseQueryJson::LostFocus.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_lost_focus(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::LostFocus))
}

/// Request keyboard focus for a widget.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_request_focus",
    description = "Request keyboard focus for the widget. Returns ResponseQueryJson::RequestFocus.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_request_focus(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::RequestFocus))
}

/// Surrender keyboard focus from a widget.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_surrender_focus",
    description = "Surrender keyboard focus from the widget. Returns ResponseQueryJson::SurrenderFocus.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_surrender_focus(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::SurrenderFocus))
}

// ---------------------------------------------------------------------------
// Drag
// ---------------------------------------------------------------------------

/// Check if a widget is being dragged.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_dragged",
    description = "Check if the widget is being dragged. Returns ResponseQueryJson::Dragged.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_dragged(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::Dragged))
}

/// Check if drag was released.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_drag_released",
    description = "Check if drag was released. Returns ResponseQueryJson::DragReleased.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_drag_released(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::DragReleased))
}

/// Get the drag delta vector.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_drag_delta",
    description = "Get the drag delta (pointer movement while dragging). Returns ResponseQueryJson::DragDelta.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_drag_delta(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::DragDelta))
}

// ---------------------------------------------------------------------------
// Value / state
// ---------------------------------------------------------------------------

/// Check if the widget value changed.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_changed",
    description = "Check if the widget value changed. Returns ResponseQueryJson::Changed.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_changed(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::Changed))
}

/// Get the widget bounding rectangle.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_rect",
    description = "Get the widget bounding rectangle. Returns ResponseQueryJson::Rect.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_rect(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::Rect))
}

/// Get the hover position.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_hover_pos",
    description = "Get the pointer position while hovering. Returns ResponseQueryJson::HoverPos.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_hover_pos(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::HoverPos))
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

/// Parameters for [`response_show_tooltip`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShowTooltipParams {
    /// Tooltip text.
    pub text: String,
}

/// Show a tooltip on hover.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_show_tooltip",
    description = "Show a tooltip when the widget is hovered. Returns ResponseQueryJson::ShowTooltip."
)]
#[instrument(skip_all)]
async fn response_show_tooltip(p: ShowTooltipParams) -> Result<CallToolResult, ErrorData> {
    let q = ResponseQueryJson::ShowTooltip { text: p.text };
    Ok(query_result(&q))
}

/// Parameters for [`response_set_enabled`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetEnabledParams {
    /// Whether the widget is enabled.
    pub enabled: bool,
}

/// Set the enabled state of a widget.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_set_enabled",
    description = "Set the widget enabled/disabled state. Returns ResponseQueryJson::SetEnabled."
)]
#[instrument(skip_all)]
async fn response_set_enabled(p: SetEnabledParams) -> Result<CallToolResult, ErrorData> {
    let q = ResponseQueryJson::SetEnabled { enabled: p.enabled };
    Ok(query_result(&q))
}

/// Highlight a widget (e.g. for tutorials).
#[elicit_tool(
    plugin = "egui_response",
    name = "response_highlight",
    description = "Highlight the widget for emphasis. Returns ResponseQueryJson::Highlight.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_highlight(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::Highlight))
}

/// Scroll to make the widget visible.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_scroll_to_me",
    description = "Scroll to make the widget visible. Returns ResponseQueryJson::ScrollToMe.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_scroll_to_me(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::ScrollToMe))
}

/// Show a context menu for the widget.
#[elicit_tool(
    plugin = "egui_response",
    name = "response_context_menu",
    description = "Show a context menu for the widget. Returns ResponseQueryJson::ContextMenu.",
    emit = None
)]
#[instrument(skip_all)]
async fn response_context_menu(p: EmptyResponseParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(query_result(&ResponseQueryJson::ContextMenu))
}
