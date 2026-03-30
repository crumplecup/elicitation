//! Context menu, popup, tooltip, modal, and notification tools.
//!
//! Each tool returns a JSON description of a menu/popup/tooltip/modal action.

use elicitation::elicit_tool;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::Vec2Json;

// ---------------------------------------------------------------------------
// JSON interchange types
// ---------------------------------------------------------------------------

/// Serializable menu/popup/tooltip/modal/notification action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum MenuActionJson {
    /// A right-click context menu on a region.
    ContextMenu {
        /// Identifier for the region.
        region_id: String,
    },
    /// An item inside a context menu.
    ContextMenuItem {
        /// Display label for the item.
        label: String,
        /// Optional keyboard shortcut hint text.
        shortcut: Option<String>,
    },
    /// A separator line in a context menu.
    ContextMenuSeparator,
    /// A popup at a specific position.
    Popup {
        /// Popup identifier.
        id: String,
        /// Position to show the popup at.
        position: Vec2Json,
        /// Description of the popup content.
        content: String,
    },
    /// A popup anchored below a widget.
    PopupBelowWidget {
        /// Identifier of the anchor widget.
        anchor_id: String,
        /// Description of the popup content.
        content: String,
    },
    /// Close the current popup.
    ClosePopup,
    /// A hover tooltip for a widget.
    Tooltip {
        /// Identifier of the widget to attach the tooltip to.
        widget_id: String,
        /// Tooltip text.
        text: String,
    },
    /// A rich tooltip with custom UI content.
    TooltipRich {
        /// Identifier of the widget to attach the tooltip to.
        widget_id: String,
        /// Description of rich tooltip UI content.
        content: String,
    },
    /// A tooltip shown at the mouse pointer position.
    TooltipAtPointer {
        /// Tooltip text.
        text: String,
    },
    /// A modal dialog.
    Modal {
        /// Dialog title.
        title: String,
        /// Description of the dialog body content.
        content: String,
        /// Button labels (e.g. \["OK", "Cancel"\]).
        buttons: Vec<String>,
    },
    /// A yes/no confirmation dialog.
    ConfirmDialog {
        /// Dialog title.
        title: String,
        /// Message to display.
        message: String,
    },
    /// An alert/info dialog with a single OK button.
    AlertDialog {
        /// Dialog title.
        title: String,
        /// Message to display.
        message: String,
    },
    /// A temporary notification/toast message.
    Notification {
        /// Notification text.
        text: String,
        /// Duration to show in seconds.
        duration_secs: f32,
        /// Screen position for the notification.
        position: Option<Vec2Json>,
    },
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn menu_result(action: &MenuActionJson) -> CallToolResult {
    match serde_json::to_string(action) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ---------------------------------------------------------------------------
// Context menus
// ---------------------------------------------------------------------------

/// Parameters for [`egui_context_menu`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ContextMenuParams {
    /// Identifier for the region to attach the context menu to.
    pub region_id: String,
}

/// Add a right-click context menu to a region.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_context_menu",
    description = "Add a right-click context menu to a region. Returns MenuActionJson::ContextMenu.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_context_menu(p: ContextMenuParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::ContextMenu {
        region_id: p.region_id,
    };
    Ok(menu_result(&a))
}

/// Parameters for [`egui_context_menu_item`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ContextMenuItemParams {
    /// Display label for the menu item.
    pub label: String,
    /// Optional keyboard shortcut hint text (e.g. "Ctrl+C").
    pub shortcut: Option<String>,
}

/// Add an item to a context menu.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_context_menu_item",
    description = "Add an item to a context menu. Returns MenuActionJson::ContextMenuItem.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_context_menu_item(p: ContextMenuItemParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::ContextMenuItem {
        label: p.label,
        shortcut: p.shortcut,
    };
    Ok(menu_result(&a))
}

/// Empty params for tools that take no arguments.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct EmptyMenuParams {}

/// Add a separator line in a context menu.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_context_menu_separator",
    description = "Add a separator line in a context menu. Returns MenuActionJson::ContextMenuSeparator.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_context_menu_separator(p: EmptyMenuParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(menu_result(&MenuActionJson::ContextMenuSeparator))
}

// ---------------------------------------------------------------------------
// Popups
// ---------------------------------------------------------------------------

/// Parameters for [`egui_popup`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct MenuPopupParams {
    /// Popup identifier.
    pub id: String,
    /// Position to show the popup at.
    pub position: Vec2Json,
    /// Description of the popup content.
    pub content: String,
}

/// Show a popup at a position.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_popup",
    description = "Show a popup at a position. Returns MenuActionJson::Popup.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_popup(p: MenuPopupParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::Popup {
        id: p.id,
        position: p.position,
        content: p.content,
    };
    Ok(menu_result(&a))
}

/// Parameters for [`egui_popup_below_widget`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct PopupBelowWidgetParams {
    /// Identifier of the anchor widget.
    pub anchor_id: String,
    /// Description of the popup content.
    pub content: String,
}

/// Show a popup below a specific widget.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_popup_below_widget",
    description = "Show popup below a specific widget. Returns MenuActionJson::PopupBelowWidget.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_popup_below_widget(p: PopupBelowWidgetParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::PopupBelowWidget {
        anchor_id: p.anchor_id,
        content: p.content,
    };
    Ok(menu_result(&a))
}

/// Close the current popup.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_close_popup",
    description = "Close the current popup. Returns MenuActionJson::ClosePopup.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_close_popup(p: EmptyMenuParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(menu_result(&MenuActionJson::ClosePopup))
}

// ---------------------------------------------------------------------------
// Tooltips
// ---------------------------------------------------------------------------

/// Parameters for [`egui_tooltip`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct MenuTooltipParams {
    /// Identifier of the widget to attach the tooltip to.
    pub widget_id: String,
    /// Tooltip text.
    pub text: String,
}

/// Show a tooltip on hover for a widget.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_tooltip",
    description = "Show a tooltip on hover for a widget. Returns MenuActionJson::Tooltip.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_tooltip(p: MenuTooltipParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::Tooltip {
        widget_id: p.widget_id,
        text: p.text,
    };
    Ok(menu_result(&a))
}

/// Parameters for [`egui_tooltip_rich`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TooltipRichParams {
    /// Identifier of the widget to attach the tooltip to.
    pub widget_id: String,
    /// Description of the rich tooltip UI content.
    pub content: String,
}

/// Show a rich tooltip with custom UI content.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_tooltip_rich",
    description = "Show a rich tooltip with custom UI content. Returns MenuActionJson::TooltipRich.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_tooltip_rich(p: TooltipRichParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::TooltipRich {
        widget_id: p.widget_id,
        content: p.content,
    };
    Ok(menu_result(&a))
}

/// Parameters for [`egui_tooltip_at_pointer`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TooltipAtPointerParams {
    /// Tooltip text.
    pub text: String,
}

/// Show a tooltip at the mouse pointer position.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_tooltip_at_pointer",
    description = "Show tooltip at the mouse pointer position. Returns MenuActionJson::TooltipAtPointer.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_tooltip_at_pointer(p: TooltipAtPointerParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::TooltipAtPointer { text: p.text };
    Ok(menu_result(&a))
}

// ---------------------------------------------------------------------------
// Modals / Dialogs
// ---------------------------------------------------------------------------

/// Parameters for [`egui_modal`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ModalParams {
    /// Dialog title.
    pub title: String,
    /// Description of the dialog body content.
    pub content: String,
    /// Button labels (e.g. \["OK", "Cancel"\]).
    pub buttons: Vec<String>,
}

/// Show a modal dialog.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_modal",
    description = "Show a modal dialog with title, content, and buttons. Returns MenuActionJson::Modal.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_modal(p: ModalParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::Modal {
        title: p.title,
        content: p.content,
        buttons: p.buttons,
    };
    Ok(menu_result(&a))
}

/// Parameters for [`egui_confirm_dialog`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ConfirmDialogParams {
    /// Dialog title.
    pub title: String,
    /// Message to display.
    pub message: String,
}

/// Show a yes/no confirmation dialog.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_confirm_dialog",
    description = "Show a yes/no confirmation dialog. Returns MenuActionJson::ConfirmDialog.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_confirm_dialog(p: ConfirmDialogParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::ConfirmDialog {
        title: p.title,
        message: p.message,
    };
    Ok(menu_result(&a))
}

/// Parameters for [`egui_alert_dialog`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AlertDialogParams {
    /// Dialog title.
    pub title: String,
    /// Message to display.
    pub message: String,
}

/// Show an alert/info dialog with a single OK button.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_alert_dialog",
    description = "Show an alert/info dialog with single OK button. Returns MenuActionJson::AlertDialog.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_alert_dialog(p: AlertDialogParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::AlertDialog {
        title: p.title,
        message: p.message,
    };
    Ok(menu_result(&a))
}

// ---------------------------------------------------------------------------
// Notification / Toast
// ---------------------------------------------------------------------------

/// Parameters for [`egui_notification`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct NotificationParams {
    /// Notification text.
    pub text: String,
    /// Duration to show the notification in seconds.
    pub duration_secs: f32,
    /// Optional screen position for the notification.
    pub position: Option<Vec2Json>,
}

/// Show a temporary notification/toast message.
#[elicit_tool(
    plugin = "egui_menus",
    name = "egui_notification",
    description = "Show a temporary notification/toast message. Returns MenuActionJson::Notification.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_notification(p: NotificationParams) -> Result<CallToolResult, ErrorData> {
    let a = MenuActionJson::Notification {
        text: p.text,
        duration_secs: p.duration_secs,
        position: p.position,
    };
    Ok(menu_result(&a))
}
