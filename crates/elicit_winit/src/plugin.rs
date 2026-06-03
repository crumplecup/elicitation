//! [`WinitPlugin`] — stateful MCP plugin for winit windowing.

use std::sync::Arc;

use elicitation::ElicitPlugin;
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;

use crate::context::WinitCtx;
use winit::event_loop::EventLoopProxy;

use crate::WinitCmd;

// ── Plugin ────────────────────────────────────────────────────────────────────

/// Stateful MCP plugin for winit window management.
///
/// Holds a shared [`WinitCtx`] wrapping the event-loop proxy injected by the
/// embedding app. Register a single instance with your MCP server; all
/// `window__*` tools share the same context.
///
/// # Example
///
/// ```no_run
/// # use winit::event_loop::EventLoop;
/// # use elicit_winit::{WinitCmd, WinitPlugin};
/// let event_loop = EventLoop::<WinitCmd>::with_user_event()
///     .build()
///     .expect("event loop");
/// let proxy = event_loop.create_proxy();
/// let plugin = WinitPlugin::new(proxy);
/// ```
#[derive(ElicitPlugin)]
#[plugin(name = "winit")]
pub struct WinitPlugin(pub Arc<WinitCtx>);

impl WinitPlugin {
    /// Create a new plugin from an event-loop proxy injected by the embedding app.
    pub fn new(proxy: EventLoopProxy<WinitCmd>) -> Self {
        Self(Arc::new(WinitCtx::new(proxy)))
    }

    /// Return a shared reference to the underlying context.
    pub fn ctx(&self) -> Arc<WinitCtx> {
        Arc::clone(&self.0)
    }
}

impl std::fmt::Debug for WinitPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("WinitPlugin").field(&self.0).finish()
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Wrap a text message in a successful [`CallToolResult`].
pub(crate) fn ok_text(msg: impl Into<String>) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(msg.into())]))
}

/// Serialize a value as JSON in a successful [`CallToolResult`].
pub(crate) fn ok_json<T: serde::Serialize>(v: &T) -> Result<CallToolResult, ErrorData> {
    match serde_json::to_string(v) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => Err(ErrorData::internal_error(
            format!("serialization error: {e}"),
            None,
        )),
    }
}
