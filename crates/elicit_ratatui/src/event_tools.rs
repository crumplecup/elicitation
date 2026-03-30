//! Runtime event handling tools for ratatui.
//!
//! These tools read terminal events (key presses, mouse, resize).
//! Requires the `runtime` feature.

use std::time::Duration;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::serde_types::{EventJson, KeyEventJson, MouseEventJson};
use elicitation::elicit_tool;

/// Serialise a JSON value to a `CallToolResult`.
fn json_result(value: &impl serde::Serialize) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Convert crossterm `KeyModifiers` to a list of modifier name strings.
fn modifier_strings(mods: KeyModifiers) -> Vec<String> {
    let mut v = Vec::new();
    if mods.contains(KeyModifiers::CONTROL) {
        v.push("CONTROL".to_string());
    }
    if mods.contains(KeyModifiers::SHIFT) {
        v.push("SHIFT".to_string());
    }
    if mods.contains(KeyModifiers::ALT) {
        v.push("ALT".to_string());
    }
    v
}

/// Convert a crossterm `KeyCode` to a human-readable string.
fn key_code_string(code: &KeyCode) -> String {
    match code {
        KeyCode::Char(c) => format!("Char({c})"),
        KeyCode::F(n) => format!("F({n})"),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Null => "Null".to_string(),
        KeyCode::CapsLock => "CapsLock".to_string(),
        KeyCode::ScrollLock => "ScrollLock".to_string(),
        KeyCode::NumLock => "NumLock".to_string(),
        KeyCode::PrintScreen => "PrintScreen".to_string(),
        KeyCode::Pause => "Pause".to_string(),
        KeyCode::Menu => "Menu".to_string(),
        KeyCode::KeypadBegin => "KeypadBegin".to_string(),
        _ => format!("{code:?}"),
    }
}

/// Convert a crossterm `KeyEvent` to a `KeyEventJson`.
fn key_event_to_json(ke: &KeyEvent) -> KeyEventJson {
    KeyEventJson {
        code: key_code_string(&ke.code),
        modifiers: modifier_strings(ke.modifiers),
    }
}

/// Convert a crossterm `MouseEvent` to a `MouseEventJson`.
fn mouse_event_to_json(me: &MouseEvent) -> MouseEventJson {
    let kind = match me.kind {
        MouseEventKind::Down(btn) => format!("Down({btn:?})"),
        MouseEventKind::Up(btn) => format!("Up({btn:?})"),
        MouseEventKind::Drag(btn) => format!("Drag({btn:?})"),
        MouseEventKind::Moved => "Moved".to_string(),
        MouseEventKind::ScrollDown => "ScrollDown".to_string(),
        MouseEventKind::ScrollUp => "ScrollUp".to_string(),
        MouseEventKind::ScrollLeft => "ScrollLeft".to_string(),
        MouseEventKind::ScrollRight => "ScrollRight".to_string(),
    };
    MouseEventJson {
        kind,
        column: me.column,
        row: me.row,
        modifiers: modifier_strings(me.modifiers),
    }
}

/// Convert a crossterm `Event` to an `EventJson`.
fn event_to_json(ev: &Event) -> EventJson {
    match ev {
        Event::Key(ke) => EventJson::Key {
            event: key_event_to_json(ke),
        },
        Event::Mouse(me) => EventJson::Mouse {
            event: mouse_event_to_json(me),
        },
        Event::Resize(w, h) => EventJson::Resize {
            width: *w,
            height: *h,
        },
        Event::FocusGained => EventJson::FocusGained,
        Event::FocusLost => EventJson::FocusLost,
        Event::Paste(text) => EventJson::Paste {
            text: text.clone(),
        },
    }
}

// ---------------------------------------------------------------------------
// event_poll
// ---------------------------------------------------------------------------

/// Parameters for [`event_poll`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventPollParams {
    /// Timeout in milliseconds to wait for an event.
    pub timeout_ms: u64,
}

/// Poll for terminal events with a timeout.
///
/// Returns `{ available: true }` if an event is ready to be read.
#[elicit_tool(
    plugin = "ratatui_events",
    name = "event_poll",
    description = "Poll for terminal events with a timeout. Returns { available: bool }."
)]
#[instrument(skip_all)]
async fn event_poll(p: EventPollParams) -> Result<CallToolResult, ErrorData> {
    let timeout = Duration::from_millis(p.timeout_ms);
    match event::poll(timeout) {
        Ok(available) => Ok(json_result(&serde_json::json!({ "available": available }))),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "poll error: {e}"
        ))])),
    }
}

// ---------------------------------------------------------------------------
// event_read
// ---------------------------------------------------------------------------

/// Parameters for [`event_read`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventReadParams {
    /// Optional timeout in milliseconds. If absent, blocks indefinitely.
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

/// Read the next terminal event.
///
/// Blocks until an event is available, or until the optional timeout expires.
/// Returns an `EventJson` describing the event.
#[elicit_tool(
    plugin = "ratatui_events",
    name = "event_read",
    description = "Read the next terminal event. Blocks until available or timeout. Returns EventJson."
)]
#[instrument(skip_all)]
async fn event_read(p: EventReadParams) -> Result<CallToolResult, ErrorData> {
    if let Some(ms) = p.timeout_ms {
        let timeout = Duration::from_millis(ms);
        match event::poll(timeout) {
            Ok(true) => {}
            Ok(false) => {
                return Ok(CallToolResult::error(vec![Content::text(
                    "timeout: no event available",
                )]));
            }
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "poll error: {e}"
                ))]));
            }
        }
    }
    match event::read() {
        Ok(ev) => Ok(json_result(&event_to_json(&ev))),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "read error: {e}"
        ))])),
    }
}

// ---------------------------------------------------------------------------
// event_read_key
// ---------------------------------------------------------------------------

/// Parameters for [`event_read_key`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EventReadKeyParams {}

/// Read the next key event, skipping non-key events.
///
/// Blocks until a key press is received. Returns a `KeyEventJson`.
#[elicit_tool(
    plugin = "ratatui_events",
    name = "event_read_key",
    description = "Read the next key event (blocks until a key is pressed). Returns KeyEventJson."
)]
#[instrument(skip_all)]
async fn event_read_key(_p: EventReadKeyParams) -> Result<CallToolResult, ErrorData> {
    loop {
        match event::read() {
            Ok(Event::Key(ke)) => return Ok(json_result(&key_event_to_json(&ke))),
            Ok(_) => continue,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "read error: {e}"
                ))]));
            }
        }
    }
}
