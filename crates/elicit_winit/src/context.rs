//! [`WinitCtx`], [`WinitCmd`], and [`WinitError`] — the integration bridge
//! between plugin tools and the embedding app's winit event loop.

use elicitation::plugin::PluginContext;
use elicitation::{
    WinitCursorIconSelect, WinitPhysicalSize, WinitWindowAttributes, WinitWindowLevelSelect,
};
use rmcp::ErrorData;
use tokio::sync::oneshot;
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;

use crate::Window;

// ── Error ─────────────────────────────────────────────────────────────────────

/// Errors produced by winit shadow tools.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
pub enum WinitError {
    /// The referenced window UUID is not in the event-loop state.
    #[display("window not found: {}", _0)]
    WindowNotFound(#[error(not(source))] String),
    /// The winit event loop has shut down.
    #[display("winit event loop closed")]
    EventLoopClosed,
    /// The reply channel was dropped before the event loop responded.
    #[display("reply channel closed unexpectedly")]
    ReplyChannelClosed,
    /// The OS refused to create or configure the window.
    #[display("OS error: {}", _0)]
    OsError(#[error(not(source))] String),
}

// ── Commands ──────────────────────────────────────────────────────────────────

/// Commands sent from MCP tool functions to the embedding app's event loop.
///
/// Each variant carries a `reply` oneshot channel so the tool can await the
/// result without sharing a single channel across concurrent calls.
pub enum WinitCmd {
    /// Create a new OS window.
    CreateWindow {
        /// Window creation attributes.
        attrs: WinitWindowAttributes,
        /// Reply channel for the new [`Window`] handle.
        reply: oneshot::Sender<Result<Window, WinitError>>,
    },
    /// Change the window's title bar text.
    SetTitle {
        /// Target window UUID.
        id: Uuid,
        /// New title string.
        title: String,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Show or hide the window.
    SetVisible {
        /// Target window UUID.
        id: Uuid,
        /// `true` to show, `false` to hide.
        visible: bool,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Enable or disable user resizing.
    SetResizable {
        /// Target window UUID.
        id: Uuid,
        /// `true` to allow resizing.
        resizable: bool,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Show or hide OS window decorations (title bar, borders).
    SetDecorations {
        /// Target window UUID.
        id: Uuid,
        /// `true` to show decorations.
        decorations: bool,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Enter or exit borderless-fullscreen mode.
    SetFullscreen {
        /// Target window UUID.
        id: Uuid,
        /// `true` to enter fullscreen.
        fullscreen: bool,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Set the window stacking level.
    SetWindowLevel {
        /// Target window UUID.
        id: Uuid,
        /// New window level.
        level: WinitWindowLevelSelect,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Change the cursor icon shown over this window.
    SetCursorIcon {
        /// Target window UUID.
        id: Uuid,
        /// New cursor icon.
        icon: WinitCursorIconSelect,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Show or hide the cursor while it is over this window.
    SetCursorVisible {
        /// Target window UUID.
        id: Uuid,
        /// `true` to show the cursor.
        visible: bool,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Request a redraw on the next event-loop iteration.
    RequestRedraw {
        /// Target window UUID.
        id: Uuid,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Focus the window (bring to front and give keyboard input).
    Focus {
        /// Target window UUID.
        id: Uuid,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
    /// Query the window's current inner (content area) size in physical pixels.
    InnerSize {
        /// Target window UUID.
        id: Uuid,
        /// Reply channel.
        reply: oneshot::Sender<Result<WinitPhysicalSize, WinitError>>,
    },
    /// Query the window's DPI scale factor.
    ScaleFactor {
        /// Target window UUID.
        id: Uuid,
        /// Reply channel.
        reply: oneshot::Sender<Result<f64, WinitError>>,
    },
    /// Query the window's current title.
    Title {
        /// Target window UUID.
        id: Uuid,
        /// Reply channel.
        reply: oneshot::Sender<Result<String, WinitError>>,
    },
    /// Destroy the window and remove it from the event-loop state.
    Destroy {
        /// Target window UUID.
        id: Uuid,
        /// Reply channel.
        reply: oneshot::Sender<Result<(), WinitError>>,
    },
}

// ── Context ───────────────────────────────────────────────────────────────────

/// Shared context holding the event-loop proxy injected by the embedding app.
///
/// `EventLoopProxy<WinitCmd>` is `Clone + Send`, so `WinitCtx` needs no
/// interior mutability — the proxy itself is the thread-safe sender.
pub struct WinitCtx {
    proxy: EventLoopProxy<WinitCmd>,
}

impl WinitCtx {
    /// Create a new context from an injected event-loop proxy.
    ///
    /// Call `EventLoop::create_proxy()` in the embedding app and pass the
    /// result here before registering the plugin with the MCP server.
    pub fn new(proxy: EventLoopProxy<WinitCmd>) -> Self {
        Self { proxy }
    }

    /// Send a command to the event loop.
    pub(crate) fn send(&self, cmd: WinitCmd) -> Result<(), ErrorData> {
        self.proxy
            .send_event(cmd)
            .map_err(|_| ErrorData::internal_error("winit event loop closed", None))
    }
}

impl PluginContext for WinitCtx {}

impl std::fmt::Debug for WinitCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WinitCtx").finish_non_exhaustive()
    }
}
