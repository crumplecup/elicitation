//! Integration tests for `elicit_winit` workflow plugins.

use elicit_winit::{
    WinitEventLoopScaffolded, WinitEventPlugin, WinitInputHandled, WinitInputPlugin,
    WinitWindowConfigured, WinitWindowPlugin,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

// ── WinitWindowPlugin ─────────────────────────────────────────────────────────

#[test]
fn window_plugin_creates_successfully() {
    assert_eq!(WinitWindowPlugin::new().name(), "winit_window");
}

#[test]
fn window_plugin_lists_expected_tools() {
    let names: Vec<String> = WinitWindowPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "winit_window__attributes",
        "winit_window__set_title",
        "winit_window__set_visible",
        "winit_window__set_resizable",
        "winit_window__set_decorations",
        "winit_window__set_window_level",
        "winit_window__set_fullscreen",
        "winit_window__request_redraw",
        "winit_window__set_cursor_icon",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn window_proposition_proofs_non_empty() {
    assert_verified::<WinitWindowConfigured>("WinitWindowConfigured");
}

// ── WinitEventPlugin ──────────────────────────────────────────────────────────

#[test]
fn event_plugin_creates_successfully() {
    assert_eq!(WinitEventPlugin::new().name(), "winit_event");
}

#[test]
fn event_plugin_lists_expected_tools() {
    let names: Vec<String> = WinitEventPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "winit_event__app_skeleton",
        "winit_event__event_loop",
        "winit_event__resumed_handler",
        "winit_event__window_event_dispatch",
        "winit_event__about_to_wait",
        "winit_event__close_requested",
        "winit_event__resized",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn event_proposition_proofs_non_empty() {
    assert_verified::<WinitEventLoopScaffolded>("WinitEventLoopScaffolded");
}

// ── WinitInputPlugin ──────────────────────────────────────────────────────────

#[test]
fn input_plugin_creates_successfully() {
    assert_eq!(WinitInputPlugin::new().name(), "winit_input");
}

#[test]
fn input_plugin_lists_expected_tools() {
    let names: Vec<String> = WinitInputPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();

    for name in &[
        "winit_input__keyboard_handler",
        "winit_input__named_key_handler",
        "winit_input__mouse_button_handler",
        "winit_input__cursor_moved",
        "winit_input__scroll_handler",
        "winit_input__touch_handler",
        "winit_input__modifiers_handler",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn input_proposition_proofs_non_empty() {
    assert_verified::<WinitInputHandled>("WinitInputHandled");
}

// ── Tool invocation smoke tests ───────────────────────────────────────────────

#[tokio::test]
async fn window_attributes_tool_emits_code() {
    use elicit_winit::workflow::WinitWindowPlugin as Plugin;

    let plugin = Plugin::new();
    let result = plugin
        .invoke_tool(
            "winit_window__attributes",
            serde_json::json!({ "title": "My App" }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("result should have text content");

    assert!(
        text.contains("WindowAttributes::default()"),
        "should emit WindowAttributes builder; got: {text}"
    );
    assert!(
        text.contains("with_title"),
        "should emit with_title call; got: {text}"
    );
}

#[tokio::test]
async fn event_app_skeleton_tool_emits_code() {
    use elicit_winit::workflow::WinitEventPlugin as Plugin;

    let plugin = Plugin::new();
    let result = plugin
        .invoke_tool(
            "winit_event__app_skeleton",
            serde_json::json!({ "app_name": "MyApp", "title": "Test Window" }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("result should have text content");

    assert!(
        text.contains("ApplicationHandler"),
        "should emit ApplicationHandler; got: {text}"
    );
    assert!(
        text.contains("MyApp"),
        "should use provided app name; got: {text}"
    );
}

#[tokio::test]
async fn input_keyboard_handler_tool_emits_code() {
    use elicit_winit::workflow::WinitInputPlugin as Plugin;

    let plugin = Plugin::new();
    let result = plugin
        .invoke_tool(
            "winit_input__keyboard_handler",
            serde_json::json!({ "key_code": "Space", "state": "Pressed" }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("result should have text content");

    assert!(
        text.contains("KeyboardInput"),
        "should emit KeyboardInput match arm; got: {text}"
    );
    assert!(
        text.contains("Space"),
        "should include Space key; got: {text}"
    );
}
