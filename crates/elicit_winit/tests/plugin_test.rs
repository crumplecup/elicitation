//! Integration tests for `elicit_winit`.

use elicit_winit::{Window, WinitError};
use serde_json;
use uuid::Uuid;

// ── Window UUID serde ─────────────────────────────────────────────────────────

#[test]
fn window_serializes_as_uuid_string() {
    let id = Uuid::new_v4();
    let w = Window(id);
    let json = serde_json::to_string(&w).expect("serialize");
    assert_eq!(json, format!("\"{}\"", id));
}

#[test]
fn window_deserializes_from_uuid_string() {
    let id = Uuid::new_v4();
    let json = format!("\"{}\"", id);
    let w: Window = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(w, Window(id));
}

#[test]
fn window_round_trips_through_json() {
    let id = Uuid::new_v4();
    let original = Window(id);
    let json = serde_json::to_string(&original).expect("serialize");
    let restored: Window = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(original, restored);
}

#[test]
fn window_rejects_invalid_uuid() {
    let result = serde_json::from_str::<Window>("\"not-a-uuid\"");
    assert!(result.is_err());
}

// ── WinitError Display ────────────────────────────────────────────────────────

#[test]
fn winit_error_display_window_not_found() {
    let e = WinitError::WindowNotFound("abc-123".to_string());
    assert_eq!(e.to_string(), "window not found: abc-123");
}

#[test]
fn winit_error_display_event_loop_closed() {
    let e = WinitError::EventLoopClosed;
    assert_eq!(e.to_string(), "winit event loop closed");
}

#[test]
fn winit_error_display_reply_channel_closed() {
    let e = WinitError::ReplyChannelClosed;
    assert_eq!(e.to_string(), "reply channel closed unexpectedly");
}

#[test]
fn winit_error_display_os_error() {
    let e = WinitError::OsError("permission denied".to_string());
    assert_eq!(e.to_string(), "OS error: permission denied");
}
