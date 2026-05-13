//! `WinitWindowPlugin` — code-generation tools for winit window configuration.
//!
//! All tools return Rust code strings; no OS windows are created at runtime.
//!
//! # Tool namespace: `winit_window__*`
//!
//! | Tool | Params | Emits |
//! |------|--------|-------|
//! | `attributes` | `WinitWindowAttributes` | `WindowAttributes::default()` builder chain |
//! | `set_title` | `title` | `window.set_title(...)` |
//! | `set_visible` | `visible` | `window.set_visible(...)` |
//! | `set_resizable` | `resizable` | `window.set_resizable(...)` |
//! | `set_decorations` | `decorations` | `window.set_decorations(...)` |
//! | `set_window_level` | `WinitWindowLevelSelect` | `window.set_window_level(...)` |
//! | `set_fullscreen` | `enabled` | fullscreen toggle code |
//! | `request_redraw` | — | `window.request_redraw()` |
//! | `set_cursor_icon` | `WinitCursorIconSelect` | `window.set_cursor(...)` |

use elicitation::{
    Prop, VerifiedWorkflow, WinitCursorIconSelect, WinitWindowAttributes, WinitWindowLevelSelect,
    elicit_tool,
};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: window attributes code was successfully generated.
#[derive(Prop)]
pub struct WinitWindowConfigured;

impl VerifiedWorkflow for WinitWindowConfigured {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `winit_window__attributes`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WindowAttributesParams {
    #[serde(flatten)]
    /// Window configuration.
    pub attrs: WinitWindowAttributes,
}

/// Parameters for `winit_window__set_title`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetTitleParams {
    /// New window title.
    pub title: String,
}

/// Parameters for `winit_window__set_visible`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetVisibleParams {
    /// Whether the window should be visible.
    pub visible: bool,
}

/// Parameters for `winit_window__set_resizable`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetResizableParams {
    /// Whether the window should be user-resizable.
    pub resizable: bool,
}

/// Parameters for `winit_window__set_decorations`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetDecorationsParams {
    /// Whether OS decorations (title bar, borders) should be shown.
    pub decorations: bool,
}

/// Parameters for `winit_window__set_window_level`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetWindowLevelParams {
    /// Window stacking level.
    pub level: WinitWindowLevelSelect,
}

/// Parameters for `winit_window__set_fullscreen`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetFullscreenParams {
    /// `true` for borderless fullscreen; `false` to restore windowed mode.
    pub enabled: bool,
}

/// Parameters for `winit_window__set_cursor_icon`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SetCursorIconParams {
    /// Cursor icon to display.
    pub icon: WinitCursorIconSelect,
}

/// Parameters for `winit_window__request_redraw` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RequestRedrawParams {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn text(s: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

/// Format a `WindowAttributes` builder chain from the given config.
///
/// Uses `Debug` output of winit inner types (via the `Deref` impl on each
/// trenchcoat wrapper) to produce variant names suitable for code generation.
fn format_window_attributes(attrs: &WinitWindowAttributes) -> String {
    let mut chain = "WindowAttributes::default()".to_string();

    chain.push_str(&format!("\n    .with_title({:?})", attrs.title));

    if let Some(size) = attrs.inner_size {
        chain.push_str(&format!(
            "\n    .with_inner_size(LogicalSize::new({:.1}, {:.1}))",
            size.width, size.height
        ));
    }
    if let Some(size) = attrs.min_inner_size {
        chain.push_str(&format!(
            "\n    .with_min_inner_size(LogicalSize::new({:.1}, {:.1}))",
            size.width, size.height
        ));
    }
    if let Some(size) = attrs.max_inner_size {
        chain.push_str(&format!(
            "\n    .with_max_inner_size(LogicalSize::new({:.1}, {:.1}))",
            size.width, size.height
        ));
    }
    if let Some(pos) = attrs.position {
        chain.push_str(&format!(
            "\n    .with_position(LogicalPosition::new({:.1}, {:.1}))",
            pos.x, pos.y
        ));
    }
    if let Some(r) = attrs.resizable {
        chain.push_str(&format!("\n    .with_resizable({r})"));
    }
    if let Some(m) = attrs.maximized {
        chain.push_str(&format!("\n    .with_maximized({m})"));
    }
    if let Some(v) = attrs.visible {
        chain.push_str(&format!("\n    .with_visible({v})"));
    }
    if let Some(t) = attrs.transparent {
        chain.push_str(&format!("\n    .with_transparent({t})"));
    }
    if let Some(d) = attrs.decorations {
        chain.push_str(&format!("\n    .with_decorations({d})"));
    }
    if attrs.fullscreen == Some(true) {
        chain.push_str("\n    .with_fullscreen(Some(Fullscreen::Borderless(None)))");
    }
    if let Some(level) = attrs.window_level {
        // `*level` derefs to `winit::window::WindowLevel`; Debug gives us the variant name.
        chain.push_str(&format!(
            "\n    .with_window_level(WindowLevel::{:?})",
            *level
        ));
    }
    if let Some(theme) = attrs.theme {
        chain.push_str(&format!("\n    .with_theme(Some(Theme::{:?}))", *theme));
    }
    if let Some(a) = attrs.active {
        chain.push_str(&format!("\n    .with_active({a})"));
    }

    chain
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__attributes",
    description = "Generate a `WindowAttributes` builder chain from the given window configuration. \
                   Returns code suitable for passing to `ActiveEventLoop::create_window`."
)]
#[instrument(skip(p))]
async fn window_attributes(p: WindowAttributesParams) -> Result<CallToolResult, ErrorData> {
    text(format_window_attributes(&p.attrs))
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_title",
    description = "Generate code to update a window's title bar text at runtime."
)]
#[instrument]
async fn window_set_title(p: SetTitleParams) -> Result<CallToolResult, ErrorData> {
    text(format!("window.set_title({:?});", p.title))
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_visible",
    description = "Generate code to show or hide a window."
)]
#[instrument]
async fn window_set_visible(p: SetVisibleParams) -> Result<CallToolResult, ErrorData> {
    text(format!("window.set_visible({});", p.visible))
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_resizable",
    description = "Generate code to enable or disable user resizing of a window."
)]
#[instrument]
async fn window_set_resizable(p: SetResizableParams) -> Result<CallToolResult, ErrorData> {
    text(format!("window.set_resizable({});", p.resizable))
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_decorations",
    description = "Generate code to show or hide OS window decorations (title bar, borders)."
)]
#[instrument]
async fn window_set_decorations(p: SetDecorationsParams) -> Result<CallToolResult, ErrorData> {
    text(format!("window.set_decorations({});", p.decorations))
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_window_level",
    description = "Generate code to set a window's stacking level (AlwaysOnBottom / Normal / AlwaysOnTop)."
)]
#[instrument]
async fn window_set_window_level(p: SetWindowLevelParams) -> Result<CallToolResult, ErrorData> {
    text(format!(
        "window.set_window_level(WindowLevel::{:?});",
        *p.level
    ))
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_fullscreen",
    description = "Generate code to enter borderless fullscreen or restore windowed mode."
)]
#[instrument]
async fn window_set_fullscreen(p: SetFullscreenParams) -> Result<CallToolResult, ErrorData> {
    let code = if p.enabled {
        "window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));".to_string()
    } else {
        "window.set_fullscreen(None);".to_string()
    };
    text(code)
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__request_redraw",
    description = "Generate code to request a redraw of the window's contents on the next event loop iteration."
)]
#[instrument]
async fn window_request_redraw(_p: RequestRedrawParams) -> Result<CallToolResult, ErrorData> {
    text("window.request_redraw();".to_string())
}

#[elicit_tool(
    plugin = "winit_window",
    name = "winit_window__set_cursor_icon",
    description = "Generate code to change the cursor icon displayed when the pointer is over the window."
)]
#[instrument]
async fn window_set_cursor_icon(p: SetCursorIconParams) -> Result<CallToolResult, ErrorData> {
    text(format!(
        "window.set_cursor(winit::window::CursorIcon::{:?});",
        *p.icon
    ))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: winit window creation and configuration code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "winit_window")]
pub struct WinitWindowPlugin;

impl WinitWindowPlugin {
    /// Create a new window plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }

    /// Convenience dispatcher for tests and direct integration.
    ///
    /// Bypasses `RequestContext` (not constructible outside an MCP server).
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<rmcp::model::CallToolResult, rmcp::ErrorData> {
        use elicitation::{NoContext, PluginToolRegistration, inventory};
        let found = inventory::iter::<PluginToolRegistration>()
            .filter(|r| r.plugin == "winit_window")
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

impl Default for WinitWindowPlugin {
    fn default() -> Self {
        Self::new()
    }
}
