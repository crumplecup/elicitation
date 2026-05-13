//! `WinitInputPlugin` — code-generation tools for winit keyboard, mouse, and touch input.
//!
//! All tools return Rust code strings; no real input is processed at runtime.
//!
//! # Tool namespace: `winit_input__*`
//!
//! | Tool | Params | Emits |
//! |------|--------|-------|
//! | `keyboard_handler` | `WinitKeyCodeSelect`, `state` | `KeyboardInput` match arm |
//! | `named_key_handler` | `key_name` | `Key::Named(NamedKey::...)` match arm |
//! | `mouse_button_handler` | `WinitMouseButtonSelect`, `state` | `MouseInput` match arm |
//! | `cursor_moved` | none | `CursorMoved` match arm |
//! | `scroll_handler` | none | `MouseWheel` match arm |
//! | `touch_handler` | `WinitTouchPhaseSelect` | `Touch` match arm |
//! | `modifiers_handler` | none | `ModifiersChanged` match arm |

use elicitation::{
    Prop, VerifiedWorkflow, WinitElementStateSelect, WinitKeyCodeSelect, WinitMouseButtonSelect,
    WinitTouchPhaseSelect, elicit_tool,
};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: input handler code was successfully generated.
#[derive(Prop)]
pub struct WinitInputHandled;

impl VerifiedWorkflow for WinitInputHandled {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `winit_input__keyboard_handler`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct KeyboardHandlerParams {
    /// Physical key code to match.
    pub key_code: WinitKeyCodeSelect,
    /// Element state to match (`Pressed` or `Released`).
    pub state: WinitElementStateSelect,
}

/// Parameters for `winit_input__named_key_handler`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NamedKeyHandlerParams {
    /// Named-key variant (e.g. `"Enter"`, `"Escape"`, `"Space"`, `"Tab"`).
    pub key_name: String,
    /// Element state to match (`Pressed` or `Released`).
    pub state: WinitElementStateSelect,
}

/// Parameters for `winit_input__mouse_button_handler`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MouseButtonHandlerParams {
    /// Mouse button to match.
    pub button: WinitMouseButtonSelect,
    /// Element state to match (`Pressed` or `Released`).
    pub state: WinitElementStateSelect,
}

/// Parameters for `winit_input__touch_handler`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TouchHandlerParams {
    /// Touch phase to match.
    pub phase: WinitTouchPhaseSelect,
}

/// Parameters for `winit_input__cursor_moved` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CursorMovedParams {}

/// Parameters for `winit_input__scroll_handler` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ScrollHandlerParams {}

/// Parameters for `winit_input__modifiers_handler` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ModifiersHandlerParams {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn text(s: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__keyboard_handler",
    description = "Generate a `WindowEvent::KeyboardInput` match arm for a specific physical key code \
                   and element state (Pressed / Released)."
)]
#[instrument]
async fn input_keyboard_handler(p: KeyboardHandlerParams) -> Result<CallToolResult, ErrorData> {
    let key_str = format!("{:?}", *p.key_code);
    let state_str = format!("{:?}", *p.state);
    let code = format!(
        r#"WindowEvent::KeyboardInput {{
    event: winit::event::KeyEvent {{
        physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::{key_str}),
        state: winit::event::ElementState::{state_str},
        repeat: false,
        ..
    }},
    ..
}} => {{
    // TODO: handle {key_str} {state_str}
}}"#
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__named_key_handler",
    description = "Generate a `WindowEvent::KeyboardInput` match arm for a `NamedKey` variant \
                   such as Enter, Escape, Space, or Tab."
)]
#[instrument]
async fn input_named_key_handler(p: NamedKeyHandlerParams) -> Result<CallToolResult, ErrorData> {
    let state_str = format!("{:?}", *p.state);
    let code = format!(
        r#"WindowEvent::KeyboardInput {{
    event: winit::event::KeyEvent {{
        logical_key: winit::keyboard::Key::Named(winit::keyboard::NamedKey::{name}),
        state: winit::event::ElementState::{state_str},
        repeat: false,
        ..
    }},
    ..
}} => {{
    // TODO: handle {name} {state_str}
}}"#,
        name = p.key_name,
        state_str = state_str,
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__mouse_button_handler",
    description = "Generate a `WindowEvent::MouseInput` match arm for a specific mouse button \
                   and element state."
)]
#[instrument]
async fn input_mouse_button_handler(
    p: MouseButtonHandlerParams,
) -> Result<CallToolResult, ErrorData> {
    let btn_str = format!("{:?}", *p.button);
    let state_str = format!("{:?}", *p.state);
    let code = format!(
        r#"WindowEvent::MouseInput {{
    button: winit::event::MouseButton::{btn_str},
    state: winit::event::ElementState::{state_str},
    ..
}} => {{
    // TODO: handle {btn_str} {state_str}
}}"#
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__cursor_moved",
    description = "Generate a `WindowEvent::CursorMoved` match arm that captures the cursor position."
)]
#[instrument]
async fn input_cursor_moved(_p: CursorMovedParams) -> Result<CallToolResult, ErrorData> {
    text(
        r#"WindowEvent::CursorMoved { position, .. } => {
    // `position` is PhysicalPosition<f64>
    let _ = position;
    // TODO: update cursor tracking / hover state
}"#
        .to_string(),
    )
}

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__scroll_handler",
    description = "Generate a `WindowEvent::MouseWheel` match arm with delta handling for \
                   both line-based and pixel-based scroll deltas."
)]
#[instrument]
async fn input_scroll_handler(_p: ScrollHandlerParams) -> Result<CallToolResult, ErrorData> {
    text(
        r#"WindowEvent::MouseWheel { delta, .. } => {
    match delta {
        winit::event::MouseScrollDelta::LineDelta(x, y) => {
            // TODO: handle line scroll (x, y)
        }
        winit::event::MouseScrollDelta::PixelDelta(pos) => {
            // TODO: handle pixel scroll (pos.x, pos.y)
        }
    }
}"#
        .to_string(),
    )
}

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__touch_handler",
    description = "Generate a `WindowEvent::Touch` match arm for a specific `TouchPhase`."
)]
#[instrument]
async fn input_touch_handler(p: TouchHandlerParams) -> Result<CallToolResult, ErrorData> {
    let phase_str = format!("{:?}", *p.phase);
    let code = format!(
        r#"WindowEvent::Touch(winit::event::Touch {{
    phase: winit::event::TouchPhase::{phase_str},
    location,
    id,
    ..
}}) => {{
    // TODO: handle touch {phase_str} at location (location.x, location.y) with id
}}"#
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_input",
    name = "winit_input__modifiers_handler",
    description = "Generate a `WindowEvent::ModifiersChanged` match arm that captures the current \
                   modifier key state (Shift, Ctrl, Alt, Super)."
)]
#[instrument]
async fn input_modifiers_handler(_p: ModifiersHandlerParams) -> Result<CallToolResult, ErrorData> {
    text(
        r#"WindowEvent::ModifiersChanged(modifiers) => {
    let shift = modifiers.state().shift_key();
    let ctrl  = modifiers.state().control_key();
    let alt   = modifiers.state().alt_key();
    let supe  = modifiers.state().super_key();
    // TODO: update stored modifier state
    let _ = (shift, ctrl, alt, supe);
}"#
        .to_string(),
    )
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: winit keyboard, mouse, and touch input code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "winit_input")]
pub struct WinitInputPlugin;

impl WinitInputPlugin {
    /// Create a new input plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }

    /// Convenience dispatcher for tests and direct integration.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        use elicitation::{NoContext, PluginToolRegistration, inventory};
        let found = inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "winit_input")
            .find(|r| r.name == name)
            .map(|r| (r.constructor)());
        let params = if let Some(m) = args.as_object().cloned() {
            rmcp::model::CallToolRequestParams::new(name.to_string()).with_arguments(m)
        } else {
            rmcp::model::CallToolRequestParams::new(name.to_string())
        };
        match found {
            Some(descriptor) => {
                descriptor
                    .dispatch(std::sync::Arc::new(NoContext), params)
                    .await
            }
            None => Err(rmcp::ErrorData::invalid_params(
                format!("unknown tool: {name}"),
                None,
            )),
        }
    }
}

impl Default for WinitInputPlugin {
    fn default() -> Self {
        Self::new()
    }
}
