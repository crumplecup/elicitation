//! [`Window`] — UUID handle for a live `winit::window::Window`, plus all
//! `window__*` MCP tools.

use std::sync::Arc;

use elicitation::{
    WinitCursorIconSelect, WinitWindowAttributes, WinitWindowLevelSelect,
    elicit_tool,
};
use rmcp::{ErrorData, model::CallToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    WinitCmd, WinitError,
    context::WinitCtx,
    plugin::{ok_json, ok_text},
};

// ── shadow type ───────────────────────────────────────────────────────────────

/// Shadow of `winit::window::Window`.
///
/// A UUID handle identifying a live `winit::window::Window` owned by the
/// embedding app's event-loop thread. Pass this handle to all `window__*`
/// tools.
#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Window(pub Uuid);

impl Serialize for Window {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Window {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse::<Uuid>()
            .map(Window)
            .map_err(serde::de::Error::custom)
    }
}

// ── receive helper ────────────────────────────────────────────────────────────

async fn recv<T>(rx: oneshot::Receiver<Result<T, WinitError>>) -> Result<T, ErrorData> {
    rx.await
        .map_err(|_| ErrorData::internal_error("reply channel closed", None))?
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))
}

// ── params structs ────────────────────────────────────────────────────────────

/// Params for `window__create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowCreateParams {
    /// Attributes for the new window.
    pub attrs: WinitWindowAttributes,
}

/// Params for tools that operate on an existing window.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSelfParams {
    /// Window handle returned by `window__create`.
    pub window: Window,
}

/// Params for `window__set_title`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetTitleParams {
    /// Window handle.
    pub window: Window,
    /// New title bar text.
    pub title: String,
}

/// Params for `window__set_visible`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetVisibleParams {
    /// Window handle.
    pub window: Window,
    /// `true` to show the window, `false` to hide it.
    pub visible: bool,
}

/// Params for `window__set_resizable`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetResizableParams {
    /// Window handle.
    pub window: Window,
    /// `true` to allow user resizing.
    pub resizable: bool,
}

/// Params for `window__set_decorations`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetDecorationsParams {
    /// Window handle.
    pub window: Window,
    /// `true` to show OS decorations (title bar, borders).
    pub decorations: bool,
}

/// Params for `window__set_fullscreen`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetFullscreenParams {
    /// Window handle.
    pub window: Window,
    /// `true` to enter borderless fullscreen.
    pub fullscreen: bool,
}

/// Params for `window__set_window_level`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetWindowLevelParams {
    /// Window handle.
    pub window: Window,
    /// New window stacking level.
    pub level: WinitWindowLevelSelect,
}

/// Params for `window__set_cursor_icon`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetCursorIconParams {
    /// Window handle.
    pub window: Window,
    /// Cursor icon to display over this window.
    pub icon: WinitCursorIconSelect,
}

/// Params for `window__set_cursor_visible`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowSetCursorVisibleParams {
    /// Window handle.
    pub window: Window,
    /// `true` to show the cursor, `false` to hide it.
    pub visible: bool,
}

// ── tools ─────────────────────────────────────────────────────────────────────

/// Mirrors `ActiveEventLoop::create_window` — creates a new OS window.
#[elicit_tool(
    plugin = "winit",
    name = "window__create",
    description = "Create a new OS window with the given attributes. Returns a Window handle.",
    emit = None
)]
#[instrument(skip(ctx), fields(title = %p.attrs.title))]
pub async fn window_create(
    ctx: Arc<WinitCtx>,
    p: WindowCreateParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::CreateWindow {
        attrs: p.attrs,
        reply: tx,
    })?;
    ok_json(&recv(rx).await?)
}

/// Mirrors `Window::set_title`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_title",
    description = "Change the window's title bar text.",
    emit = None
)]
#[instrument(skip(ctx), fields(title = %p.title))]
pub async fn window_set_title(
    ctx: Arc<WinitCtx>,
    p: WindowSetTitleParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetTitle {
        id: p.window.0,
        title: p.title,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_visible`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_visible",
    description = "Show or hide the window.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_visible(
    ctx: Arc<WinitCtx>,
    p: WindowSetVisibleParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetVisible {
        id: p.window.0,
        visible: p.visible,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_resizable`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_resizable",
    description = "Enable or disable user resizing of the window.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_resizable(
    ctx: Arc<WinitCtx>,
    p: WindowSetResizableParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetResizable {
        id: p.window.0,
        resizable: p.resizable,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_decorations`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_decorations",
    description = "Show or hide OS window decorations (title bar, borders).",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_decorations(
    ctx: Arc<WinitCtx>,
    p: WindowSetDecorationsParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetDecorations {
        id: p.window.0,
        decorations: p.decorations,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_fullscreen`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_fullscreen",
    description = "Enter or exit borderless fullscreen mode.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_fullscreen(
    ctx: Arc<WinitCtx>,
    p: WindowSetFullscreenParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetFullscreen {
        id: p.window.0,
        fullscreen: p.fullscreen,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_window_level`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_window_level",
    description = "Set the window stacking level (normal, always-on-top, always-on-bottom).",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_window_level(
    ctx: Arc<WinitCtx>,
    p: WindowSetWindowLevelParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetWindowLevel {
        id: p.window.0,
        level: p.level,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_cursor`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_cursor_icon",
    description = "Change the cursor icon displayed over this window.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_cursor_icon(
    ctx: Arc<WinitCtx>,
    p: WindowSetCursorIconParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetCursorIcon {
        id: p.window.0,
        icon: p.icon,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::set_cursor_visible`.
#[elicit_tool(
    plugin = "winit",
    name = "window__set_cursor_visible",
    description = "Show or hide the cursor while it is over this window.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_set_cursor_visible(
    ctx: Arc<WinitCtx>,
    p: WindowSetCursorVisibleParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::SetCursorVisible {
        id: p.window.0,
        visible: p.visible,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::request_redraw`.
#[elicit_tool(
    plugin = "winit",
    name = "window__request_redraw",
    description = "Request a redraw on the next event-loop iteration.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_request_redraw(
    ctx: Arc<WinitCtx>,
    p: WindowSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::RequestRedraw {
        id: p.window.0,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::focus_window`.
#[elicit_tool(
    plugin = "winit",
    name = "window__focus",
    description = "Bring the window to the front and give it keyboard focus.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_focus(
    ctx: Arc<WinitCtx>,
    p: WindowSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::Focus {
        id: p.window.0,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("ok")
}

/// Mirrors `Window::inner_size` — returns the content area in physical pixels.
#[elicit_tool(
    plugin = "winit",
    name = "window__inner_size",
    description = "Query the window's inner (content area) size in physical pixels.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_inner_size(
    ctx: Arc<WinitCtx>,
    p: WindowSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::InnerSize {
        id: p.window.0,
        reply: tx,
    })?;
    ok_json(&recv(rx).await?)
}

/// Mirrors `Window::scale_factor` — returns the DPI scale factor.
#[elicit_tool(
    plugin = "winit",
    name = "window__scale_factor",
    description = "Query the window's DPI scale factor (logical-to-physical pixel ratio).",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_scale_factor(
    ctx: Arc<WinitCtx>,
    p: WindowSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::ScaleFactor {
        id: p.window.0,
        reply: tx,
    })?;
    ok_json(&recv(rx).await?)
}

/// Mirrors `Window::title` — returns the current title bar text.
#[elicit_tool(
    plugin = "winit",
    name = "window__title",
    description = "Query the window's current title bar text.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_title(
    ctx: Arc<WinitCtx>,
    p: WindowSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::Title {
        id: p.window.0,
        reply: tx,
    })?;
    ok_text(recv(rx).await?)
}

/// Destroy the window and release its OS resources.
#[elicit_tool(
    plugin = "winit",
    name = "window__destroy",
    description = "Destroy the window and release its OS resources.",
    emit = None
)]
#[instrument(skip(ctx))]
pub async fn window_destroy(
    ctx: Arc<WinitCtx>,
    p: WindowSelfParams,
) -> Result<CallToolResult, ErrorData> {
    let (tx, rx) = oneshot::channel();
    ctx.send(WinitCmd::Destroy {
        id: p.window.0,
        reply: tx,
    })?;
    recv(rx).await?;
    ok_text("destroyed")
}
