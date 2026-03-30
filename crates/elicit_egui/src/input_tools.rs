//! Input handling and event tools for keyboard, mouse, clipboard, and focus.
//!
//! Each tool returns a JSON description of an input query or action.

use elicitation::elicit_tool;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ---------------------------------------------------------------------------
// JSON interchange types
// ---------------------------------------------------------------------------

/// Serializable input query or action.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum InputActionJson {
    /// Check if a key is pressed this frame.
    KeyPressed {
        /// Key name (e.g. "A", "Enter", "Space", "Escape").
        key: String,
    },
    /// Check if a key was released this frame.
    KeyReleased {
        /// Key name.
        key: String,
    },
    /// Check if a key is currently held down.
    KeyDown {
        /// Key name.
        key: String,
    },
    /// Get current modifier key state.
    Modifiers,
    /// Get current pointer/mouse position.
    PointerPos,
    /// Check if a mouse button was pressed.
    PointerButtonPressed {
        /// Mouse button name ("primary", "secondary", "middle").
        button: String,
    },
    /// Check if a mouse button was released.
    PointerButtonReleased {
        /// Mouse button name ("primary", "secondary", "middle").
        button: String,
    },
    /// Get pointer movement delta this frame.
    PointerDelta,
    /// Get scroll wheel delta.
    ScrollDelta,
    /// Get text from clipboard.
    ClipboardGet,
    /// Set text to clipboard.
    ClipboardSet {
        /// Text to copy to clipboard.
        text: String,
    },
    /// Request keyboard focus for a widget.
    RequestFocus {
        /// Widget identifier.
        widget_id: String,
    },
    /// Release keyboard focus from a widget.
    SurrenderFocus {
        /// Widget identifier.
        widget_id: String,
    },
    /// Check if a widget has keyboard focus.
    HasFocus {
        /// Widget identifier.
        widget_id: String,
    },
}

/// Result of a modifier key state query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ModifiersJson {
    /// Whether Ctrl is held.
    pub ctrl: bool,
    /// Whether Shift is held.
    pub shift: bool,
    /// Whether Alt is held.
    pub alt: bool,
    /// Whether Command (macOS) / Windows key is held.
    pub command: bool,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn input_result(action: &InputActionJson) -> CallToolResult {
    match serde_json::to_string(action) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Empty params for tools that take no arguments.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct EmptyInputParams {}

// ---------------------------------------------------------------------------
// Keyboard
// ---------------------------------------------------------------------------

/// Parameters for [`egui_key_pressed`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct KeyParams {
    /// Key name (e.g. "A", "Enter", "Space", "Escape").
    pub key: String,
}

/// Check if a key is pressed this frame.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_key_pressed",
    description = "Check if a key is pressed this frame. Returns InputActionJson::KeyPressed.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_key_pressed(p: KeyParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::KeyPressed { key: p.key };
    Ok(input_result(&a))
}

/// Check if a key was released this frame.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_key_released",
    description = "Check if a key was released this frame. Returns InputActionJson::KeyReleased.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_key_released(p: KeyParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::KeyReleased { key: p.key };
    Ok(input_result(&a))
}

/// Check if a key is currently held down.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_key_down",
    description = "Check if a key is currently held down. Returns InputActionJson::KeyDown.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_key_down(p: KeyParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::KeyDown { key: p.key };
    Ok(input_result(&a))
}

/// Get current modifier key state (ctrl, shift, alt, command).
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_modifiers",
    description = "Get current modifier key state (ctrl, shift, alt, command). Returns InputActionJson::Modifiers.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_modifiers(p: EmptyInputParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(input_result(&InputActionJson::Modifiers))
}

// ---------------------------------------------------------------------------
// Mouse / Pointer
// ---------------------------------------------------------------------------

/// Get current pointer/mouse position.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_pointer_pos",
    description = "Get current pointer/mouse position. Returns InputActionJson::PointerPos.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_pointer_pos(p: EmptyInputParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(input_result(&InputActionJson::PointerPos))
}

/// Parameters for [`egui_pointer_button_pressed`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct PointerButtonParams {
    /// Mouse button name ("primary", "secondary", "middle").
    pub button: String,
}

/// Check if a mouse button was pressed.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_pointer_button_pressed",
    description = "Check if a mouse button was pressed. Returns InputActionJson::PointerButtonPressed.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_pointer_button_pressed(p: PointerButtonParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::PointerButtonPressed { button: p.button };
    Ok(input_result(&a))
}

/// Check if a mouse button was released.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_pointer_button_released",
    description = "Check if a mouse button was released. Returns InputActionJson::PointerButtonReleased.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_pointer_button_released(p: PointerButtonParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::PointerButtonReleased { button: p.button };
    Ok(input_result(&a))
}

/// Get pointer movement delta this frame.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_pointer_delta",
    description = "Get pointer movement delta this frame. Returns InputActionJson::PointerDelta.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_pointer_delta(p: EmptyInputParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(input_result(&InputActionJson::PointerDelta))
}

/// Get scroll wheel delta.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_scroll_delta",
    description = "Get scroll wheel delta. Returns InputActionJson::ScrollDelta.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_scroll_delta(p: EmptyInputParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(input_result(&InputActionJson::ScrollDelta))
}

// ---------------------------------------------------------------------------
// Clipboard
// ---------------------------------------------------------------------------

/// Get text from clipboard.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_clipboard_get",
    description = "Get text from clipboard. Returns InputActionJson::ClipboardGet.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_clipboard_get(p: EmptyInputParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(input_result(&InputActionJson::ClipboardGet))
}

/// Parameters for [`egui_clipboard_set`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ClipboardSetParams {
    /// Text to copy to clipboard.
    pub text: String,
}

/// Set text to clipboard.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_clipboard_set",
    description = "Set text to clipboard. Returns InputActionJson::ClipboardSet.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_clipboard_set(p: ClipboardSetParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::ClipboardSet { text: p.text };
    Ok(input_result(&a))
}

// ---------------------------------------------------------------------------
// Focus
// ---------------------------------------------------------------------------

/// Parameters for focus tools.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FocusParams {
    /// Widget identifier.
    pub widget_id: String,
}

/// Request keyboard focus for a widget by ID.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_request_focus",
    description = "Request keyboard focus for a widget by ID. Returns InputActionJson::RequestFocus.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_request_focus(p: FocusParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::RequestFocus {
        widget_id: p.widget_id,
    };
    Ok(input_result(&a))
}

/// Release keyboard focus from a widget.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_surrender_focus",
    description = "Release keyboard focus from a widget. Returns InputActionJson::SurrenderFocus.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_surrender_focus(p: FocusParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::SurrenderFocus {
        widget_id: p.widget_id,
    };
    Ok(input_result(&a))
}

/// Check if a widget has keyboard focus.
#[elicit_tool(
    plugin = "egui_input",
    name = "egui_has_focus",
    description = "Check if a widget has keyboard focus. Returns InputActionJson::HasFocus.",
    emit = Auto,
    emit = None
)]
#[instrument(skip_all)]
async fn egui_has_focus(p: FocusParams) -> Result<CallToolResult, ErrorData> {
    let a = InputActionJson::HasFocus {
        widget_id: p.widget_id,
    };
    Ok(input_result(&a))
}
