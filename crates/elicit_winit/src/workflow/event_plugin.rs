//! `WinitEventPlugin` — code-generation tools for winit event loop scaffolding.
//!
//! All tools return Rust code strings; no real event loops are run at runtime.
//!
//! # Tool namespace: `winit_event__*`
//!
//! | Tool | Params | Emits |
//! |------|--------|-------|
//! | `app_skeleton` | `app_name`, `title` | full `ApplicationHandler` impl |
//! | `event_loop` | `app_var` | `EventLoop::new()` + `run_app` boilerplate |
//! | `resumed_handler` | `title` | `fn resumed(...)` body with window creation |
//! | `window_event_dispatch` | none | `fn window_event(...)` match-arm skeleton |
//! | `about_to_wait` | none | `fn about_to_wait(...)` body |
//! | `close_requested` | none | `WindowEvent::CloseRequested` handler |
//! | `resized` | none | `WindowEvent::Resized(size)` handler |

use elicitation::{Prop, VerifiedWorkflow, elicit_tool};
use elicitation_derive::ElicitPlugin;
use rmcp::{ErrorData, model::CallToolResult, model::Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: event loop scaffolding code was successfully generated.
#[derive(Prop)]
pub struct WinitEventLoopScaffolded;

impl VerifiedWorkflow for WinitEventLoopScaffolded {}

// ── Param types ───────────────────────────────────────────────────────────────

/// Parameters for `winit_event__app_skeleton`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AppSkeletonParams {
    /// Name of the application struct (e.g. `"MyApp"`).
    pub app_name: String,
    /// Initial window title.
    pub title: String,
}

/// Parameters for `winit_event__event_loop`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EventLoopParams {
    /// Variable name bound to the app struct (e.g. `"app"`).
    pub app_var: String,
}

/// Parameters for `winit_event__resumed_handler`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResumedHandlerParams {
    /// Window title passed to `WindowAttributes::default().with_title(...)`.
    pub title: String,
}

/// Parameters for `winit_event__window_event_dispatch` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WindowEventDispatchParams {}

/// Parameters for `winit_event__about_to_wait` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AboutToWaitParams {}

/// Parameters for `winit_event__close_requested` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CloseRequestedParams {}

/// Parameters for `winit_event__resized` (no user-supplied fields).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResizedParams {}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn text(s: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(s)]))
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__app_skeleton",
    description = "Generate a complete `ApplicationHandler` impl skeleton for a named application struct. \
                   Includes field declarations, `new()`, and all required trait methods."
)]
#[instrument]
async fn event_app_skeleton(p: AppSkeletonParams) -> Result<CallToolResult, ErrorData> {
    let name = &p.app_name;
    let title = &p.title;
    let code = format!(
        r#"use std::sync::Arc;
use winit::{{
    application::ApplicationHandler,
    event::{{Event, WindowEvent}},
    event_loop::{{ActiveEventLoop, ControlFlow, EventLoop}},
    window::{{Window, WindowAttributes}},
}};

pub struct {name} {{
    window: Option<Arc<Window>>,
}}

impl {name} {{
    pub fn new() -> Self {{
        Self {{ window: None }}
    }}
}}

impl ApplicationHandler for {name} {{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {{
        let attrs = WindowAttributes::default().with_title({title:?});
        self.window = Some(Arc::new(
            event_loop.create_window(attrs).expect("Failed to create window"),
        ));
    }}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {{
        match event {{
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {{
                // TODO: render here
            }}
            _ => {{}}
        }}
    }}

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {{
        if let Some(w) = &self.window {{
            w.request_redraw();
        }}
    }}
}}"#
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__event_loop",
    description = "Generate the `main` function body that creates an `EventLoop` and runs an application."
)]
#[instrument]
async fn event_event_loop(p: EventLoopParams) -> Result<CallToolResult, ErrorData> {
    let var = &p.app_var;
    let code = format!(
        r#"fn main() -> Result<(), impl std::error::Error> {{
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut {var} = App::new();
    event_loop.run_app(&mut {var})
}}"#
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__resumed_handler",
    description = "Generate just the `fn resumed` method body for `ApplicationHandler`, \
                   creating a window with the given title."
)]
#[instrument]
async fn event_resumed_handler(p: ResumedHandlerParams) -> Result<CallToolResult, ErrorData> {
    let title = &p.title;
    let code = format!(
        r#"fn resumed(&mut self, event_loop: &ActiveEventLoop) {{
    let attrs = WindowAttributes::default().with_title({title:?});
    self.window = Some(Arc::new(
        event_loop.create_window(attrs).expect("Failed to create window"),
    ));
}}"#
    );
    text(code)
}

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__window_event_dispatch",
    description = "Generate a `fn window_event` method skeleton with the most common `WindowEvent` \
                   match arms (CloseRequested, Resized, RedrawRequested)."
)]
#[instrument]
async fn event_window_event_dispatch(
    _p: WindowEventDispatchParams,
) -> Result<CallToolResult, ErrorData> {
    text(
        r#"fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: WindowEvent,
) {
    match event {
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::Resized(size) => {
            // TODO: handle resize — update viewport / surface
            let _ = size;
        }
        WindowEvent::RedrawRequested => {
            // TODO: render
        }
        _ => {}
    }
}"#
        .to_string(),
    )
}

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__about_to_wait",
    description = "Generate a `fn about_to_wait` body that calls `request_redraw` \
                   to drive a continuous render loop."
)]
#[instrument]
async fn event_about_to_wait(_p: AboutToWaitParams) -> Result<CallToolResult, ErrorData> {
    text(
        r#"fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    if let Some(w) = &self.window {
        w.request_redraw();
    }
}"#
        .to_string(),
    )
}

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__close_requested",
    description = "Generate a `WindowEvent::CloseRequested` match arm that exits the event loop."
)]
#[instrument]
async fn event_close_requested(_p: CloseRequestedParams) -> Result<CallToolResult, ErrorData> {
    text("WindowEvent::CloseRequested => event_loop.exit(),".to_string())
}

#[elicit_tool(
    plugin = "winit_event",
    name = "winit_event__resized",
    description = "Generate a `WindowEvent::Resized(size)` match arm with a TODO placeholder \
                   for updating the rendering surface."
)]
#[instrument]
async fn event_resized(_p: ResizedParams) -> Result<CallToolResult, ErrorData> {
    text(
        r#"WindowEvent::Resized(size) => {
    // TODO: update surface / viewport to new size
    let _ = size;
}"#
        .to_string(),
    )
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// MCP plugin: winit event-loop scaffolding code generation.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "winit_event")]
pub struct WinitEventPlugin;

impl WinitEventPlugin {
    /// Create a new event plugin.
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
            .filter(|r| r.plugin == "winit_event")
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

impl Default for WinitEventPlugin {
    fn default() -> Self {
        Self::new()
    }
}
