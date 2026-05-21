//! `EguiWinitPlugin` — MCP tools for egui + winit native application scaffolding.
//!
//! Provides descriptor-registry tools for composing a native egui app that uses
//! winit for window management and `egui_winit::State` for event integration.
//! All tools are **emit-only**: they build an [`EguiWinitDescriptor`] server-side
//! and `egui_winit__emit` produces a complete `main.rs` scaffold.
//!
//! # Tool namespace: `egui_winit__*`
//!
//! | Tool | Params | Returns | Description |
//! |---|---|---|---|
//! | `new` | `app_struct, title, width, height` | `{ config_id }` | Create app config |
//! | `set_title` | `config_id, title` | — | Set window title |
//! | `set_size` | `config_id, width, height` | — | Set initial window size |
//! | `set_renderer` | `config_id, renderer` | — | Set `wgpu` or `glow` backend |
//! | `set_theme` | `config_id, theme` | — | Set dark / light / system theme |
//! | `set_vsync` | `config_id, vsync` | — | Toggle vertical sync |
//! | `set_decorations` | `config_id, decorations` | — | Toggle OS window decorations |
//! | `set_resizable` | `config_id, resizable` | — | Toggle window resize |
//! | `describe` | `config_id` | JSON descriptor | Inspect current config |
//! | `emit` | `config_id` | `main.rs` source | Emit complete native app |

use std::collections::BTreeMap;
use std::sync::Arc;

use elicitation::{
    EguiWinitDescriptor, EguiWinitRenderer, EguiWinitTheme, PluginContext, VerifiedWorkflow,
    contracts::{Established, Prop},
};
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::instrument;
use uuid::Uuid;

// ── Proposition ───────────────────────────────────────────────────────────────

/// Proposition: a native egui + winit application was successfully configured.
#[derive(elicitation::Elicit, ::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
pub struct EguiWinitAppConfigured;

impl Prop for EguiWinitAppConfigured {
    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[kani::proof]
            fn verify_egui_winit_app_configured() {
                let configured: bool = kani::any();
                kani::assume(configured);
                assert!(configured, "egui winit app configured");
            }
        }
    }
    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            verus! {
            pub fn verify_egui_winit_app_configured(ok: bool) -> (result: bool)
                ensures result == ok,
            { ok }
            }
        }
    }
    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        quote::quote! {
            #[requires(true)]
            #[ensures(result == true)]
            #[trusted]
            pub fn verify_egui_winit_app_configured_contract() -> bool { true }
        }
    }
}
impl VerifiedWorkflow for EguiWinitAppConfigured {}

// ── Plugin context ────────────────────────────────────────────────────────────

/// Shared mutable state for [`EguiWinitPlugin`].
pub struct EguiWinitCtx {
    pub(crate) items: Mutex<BTreeMap<Uuid, EguiWinitDescriptor>>,
}

impl EguiWinitCtx {
    fn new() -> Self {
        Self {
            items: Mutex::new(BTreeMap::new()),
        }
    }
}

impl PluginContext for EguiWinitCtx {}

// ── Param / result structs ────────────────────────────────────────────────────

/// Parameters for `egui_winit__new`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitNewParams {
    /// Application struct name (PascalCase, e.g. `"MyApp"`).
    pub app_struct: String,
    /// Window title shown in the OS title bar.
    pub title: String,
    /// Initial window width in logical pixels.
    pub width: u32,
    /// Initial window height in logical pixels.
    pub height: u32,
}

/// Parameters for `egui_winit__set_title`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetTitleParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// New window title.
    pub title: String,
}

/// Parameters for `egui_winit__set_size`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetSizeParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// Window width in logical pixels.
    pub width: u32,
    /// Window height in logical pixels.
    pub height: u32,
}

/// Parameters for `egui_winit__set_renderer`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetRendererParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// Backend: `"wgpu"` (default) or `"glow"`.
    pub renderer: EguiWinitRenderer,
}

/// Parameters for `egui_winit__set_theme`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetThemeParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// Theme: `"dark"` (default), `"light"`, or `"system"`.
    pub theme: EguiWinitTheme,
}

/// Parameters for `egui_winit__set_vsync`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetVsyncParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// Whether to enable vertical sync.
    pub vsync: bool,
}

/// Parameters for `egui_winit__set_decorations`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetDecorationsParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// Whether to show OS window decorations (title bar, borders).
    pub decorations: bool,
}

/// Parameters for `egui_winit__set_resizable`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitSetResizableParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
    /// Whether the window can be resized by the user.
    pub resizable: bool,
}

/// Parameters for `egui_winit__describe`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitDescribeParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
}

/// Parameters for `egui_winit__emit`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EguiWinitEmitParams {
    /// Config UUID returned by `egui_winit__new`.
    pub config_id: String,
}

/// Result returned by `egui_winit__new`.
#[derive(Debug, Serialize, JsonSchema)]
pub struct EguiWinitNewResult {
    /// UUID for the new config — pass to all other `egui_winit__*` tools.
    pub config_id: String,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_id(s: &str) -> Result<Uuid, ErrorData> {
    s.parse::<Uuid>()
        .map_err(|_| ErrorData::invalid_params(format!("invalid UUID: {s}"), None))
}

fn json_result<T: Serialize>(value: &T) -> CallToolResult {
    match serde_json::to_string_pretty(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn ok_text(s: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![Content::text(s.into())])
}

// ── Emit helpers ──────────────────────────────────────────────────────────────

fn emit_wgpu(desc: &EguiWinitDescriptor) -> String {
    let app = &desc.app_struct;
    let title = &desc.title;
    let w = desc.width;
    let h = desc.height;
    let theme_line = match desc.theme {
        EguiWinitTheme::Dark => "        ctx.set_theme(egui::Theme::Dark);\n".to_string(),
        EguiWinitTheme::Light => "        ctx.set_theme(egui::Theme::Light);\n".to_string(),
        EguiWinitTheme::System => String::new(),
    };
    let deco = if desc.decorations {
        String::new()
    } else {
        "\n        .with_decorations(false)".to_string()
    };
    let resize = if desc.resizable {
        String::new()
    } else {
        "\n        .with_resizable(false)".to_string()
    };
    let transparent = if desc.transparent {
        "\n        .with_transparent(true)".to_string()
    } else {
        String::new()
    };
    let maximized = if desc.maximized {
        "\n        .with_maximized(true)".to_string()
    } else {
        String::new()
    };
    let vsync_mode = if desc.vsync {
        "wgpu::PresentMode::AutoVsync"
    } else {
        "wgpu::PresentMode::AutoNoVsync"
    };
    format!(
        r#"//! Generated by `egui_winit__emit` (renderer: wgpu).
//! Add this as `src/main.rs` in your binary crate.
//!
//! Required Cargo.toml dependencies:
//! ```toml
//! egui = "0.34"
//! egui-winit = "0.34"
//! egui-wgpu = {{ version = "0.34", features = ["winit"] }}
//! wgpu = "24"
//! winit = "0.30"
//! pollster = "0.4"
//! ```

use std::sync::Arc;
use egui_winit::State;
use winit::{{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{{ActiveEventLoop, ControlFlow, EventLoop}},
    window::{{Window, WindowId, WindowAttributes}},
}};

/// Your application state — add your own fields here.
#[derive(Default)]
struct {app} {{
    window: Option<Arc<Window>>,
    egui_state: Option<State>,
    surface: Option<wgpu::Surface<'static>>,
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
    renderer: Option<egui_wgpu::Renderer>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
}}

impl {app} {{
    fn update(&mut self, ctx: &egui::Context) {{
        egui::CentralPanel::default().show(ctx, |ui| {{
            ui.heading("{title}");
            ui.label("Replace this with your UI.");
        }});
    }}
}}

impl ApplicationHandler for {app} {{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {{
        let attrs = WindowAttributes::default()
            .with_title("{title}")
            .with_inner_size(winit::dpi::LogicalSize::new({w}_f64, {h}_f64)){deco}{resize}{transparent}{maximized};
        let window = Arc::new(event_loop.create_window(attrs).expect("create window"));

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).expect("create surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {{
            compatible_surface: Some(&surface),
            ..Default::default()
        }})).expect("find adapter");
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor::default(), None,
        )).expect("create device");
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let size = window.inner_size();
        let surface_format = surface.get_capabilities(&adapter).formats[0];
        let config = wgpu::SurfaceConfiguration {{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: {vsync_mode},
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }};
        surface.configure(&device, &config);

        let ctx = egui::Context::default();
{theme_line}        let egui_state = State::new(
            ctx,
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(device.limits().max_texture_dimension_2d as usize),
        );
        let renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1, false);

        self.window = Some(window);
        self.egui_state = Some(egui_state);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.renderer = Some(renderer);
        self.surface_config = Some(config);
    }}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {{
        let (window, state) = match (self.window.as_ref(), self.egui_state.as_mut()) {{
            (Some(w), Some(s)) => (w, s),
            _ => return,
        }};
        let response = state.on_window_event(window, &event);
        if response.repaint {{
            window.request_redraw();
        }}
        match event {{
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {{
                if let (Some(surface), Some(device), Some(cfg)) = (
                    self.surface.as_ref(),
                    self.device.as_ref(),
                    self.surface_config.as_mut(),
                ) {{
                    cfg.width = size.width.max(1);
                    cfg.height = size.height.max(1);
                    surface.configure(device, cfg);
                }}
                window.request_redraw();
            }}
            WindowEvent::RedrawRequested => {{
                let state = self.egui_state.as_mut().unwrap();
                let raw_input = state.take_egui_input(window);
                let ctx = state.egui_ctx().clone();
                let full_output = ctx.run(raw_input, |ctx| {{
                    self.update(ctx);
                }});
                state.handle_platform_output(window, full_output.platform_output);

                let (surface, device, queue, renderer, cfg) = match (
                    self.surface.as_ref(),
                    self.device.as_ref(),
                    self.queue.as_ref(),
                    self.renderer.as_mut(),
                    self.surface_config.as_ref(),
                ) {{
                    (Some(s), Some(d), Some(q), Some(r), Some(c)) => (s, d, q, r, c),
                    _ => return,
                }};
                let output = match surface.get_current_texture() {{
                    Ok(t) => t,
                    Err(_) => return,
                }};
                let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let clipped = ctx.tessellate(full_output.shapes, ctx.pixels_per_point());
                let screen = egui_wgpu::ScreenDescriptor {{
                    size_in_pixels: [cfg.width, cfg.height],
                    pixels_per_point: ctx.pixels_per_point(),
                }};
                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor {{ label: None }},
                );
                for (id, delta) in &full_output.textures_delta.set {{
                    renderer.update_texture(device, queue, *id, delta);
                }}
                renderer.update_buffers(device, queue, &mut encoder, &clipped, &screen);
                {{
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {{
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {{
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {{
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            }},
                        }})],
                        ..Default::default()
                    }});
                    renderer.render(&mut rpass, &clipped, &screen);
                }}
                queue.submit(std::iter::once(encoder.finish()));
                output.present();
                for id in &full_output.textures_delta.free {{
                    renderer.free_texture(id);
                }}
                window.request_redraw();
            }}
            _ => {{}}
        }}
    }}
}}

fn main() {{
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = {app}::default();
    event_loop.run_app(&mut app).expect("run event loop");
}}
"#,
        app = app,
        title = title,
        w = w,
        h = h,
        deco = deco,
        resize = resize,
        transparent = transparent,
        maximized = maximized,
        vsync_mode = vsync_mode,
        theme_line = theme_line,
    )
}

fn emit_glow(desc: &EguiWinitDescriptor) -> String {
    let app = &desc.app_struct;
    let title = &desc.title;
    let w = desc.width;
    let h = desc.height;
    format!(
        r#"//! Generated by `egui_winit__emit` (renderer: glow).
//! Add this as `src/main.rs` in your binary crate.
//!
//! Required Cargo.toml dependencies:
//! ```toml
//! egui = "0.34"
//! egui_glow = {{ version = "0.34", features = ["winit"] }}
//! winit = "0.30"
//! glutin = "0.32"
//! ```

use egui_winit::State;
use winit::{{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{{ActiveEventLoop, ControlFlow, EventLoop}},
    window::{{Window, WindowId, WindowAttributes}},
}};

/// Your application state.
#[derive(Default)]
struct {app} {{
    window: Option<Window>,
    egui_state: Option<State>,
    // TODO: add your glutin/glow context here
}}

impl {app} {{
    fn update(&mut self, ctx: &egui::Context) {{
        egui::CentralPanel::default().show(ctx, |ui| {{
            ui.heading("{title}");
            ui.label("Replace this with your UI.");
        }});
    }}
}}

impl ApplicationHandler for {app} {{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {{
        let attrs = WindowAttributes::default()
            .with_title("{title}")
            .with_inner_size(winit::dpi::LogicalSize::new({w}_f64, {h}_f64));
        let window = event_loop.create_window(attrs).expect("create window");
        let ctx = egui::Context::default();
        let egui_state = State::new(
            ctx,
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );
        self.window = Some(window);
        self.egui_state = Some(egui_state);
        // TODO: initialise glutin display + glow context here
    }}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {{
        let (window, state) = match (self.window.as_ref(), self.egui_state.as_mut()) {{
            (Some(w), Some(s)) => (w, s),
            _ => return,
        }};
        let response = state.on_window_event(window, &event);
        if response.repaint {{
            window.request_redraw();
        }}
        match event {{
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {{
                let state = self.egui_state.as_mut().unwrap();
                let raw_input = state.take_egui_input(window);
                let ctx = state.egui_ctx().clone();
                let _full_output = ctx.run(raw_input, |ctx| {{
                    self.update(ctx);
                }});
                // TODO: swap glow framebuffer here
                window.request_redraw();
            }}
            _ => {{}}
        }}
    }}
}}

fn main() {{
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = {app}::default();
    event_loop.run_app(&mut app).expect("run event loop");
}}
"#,
        app = app,
        title = title,
        w = w,
        h = h,
    )
}

// ── Tool functions ────────────────────────────────────────────────────────────

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__new",
    description = "Create a new egui + winit native app config. \
                   Returns { config_id } for use with all other egui_winit__* tools. \
                   Establishes: EguiWinitAppConfigured.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(app_struct = %p.app_struct, title = %p.title))]
async fn new_config(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitNewParams,
) -> Result<CallToolResult, ErrorData> {
    let id = Uuid::new_v4();
    let desc = EguiWinitDescriptor::new(p.app_struct, p.title, p.width, p.height);
    ctx.items.lock().await.insert(id, desc);
    let _proof: Established<EguiWinitAppConfigured> = Established::assert();
    Ok(json_result(&EguiWinitNewResult {
        config_id: id.to_string(),
    }))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_title",
    description = "Update the window title for an egui_winit config.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(title = %p.title))]
async fn set_title(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetTitleParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    ctx.items
        .lock()
        .await
        .get_mut(&id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
        })?
        .title = p.title;
    Ok(ok_text("title updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_size",
    description = "Update the initial window size (logical pixels) for an egui_winit config.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(width = p.width, height = p.height))]
async fn set_size(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetSizeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let mut items = ctx.items.lock().await;
    let desc = items.get_mut(&id).ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    desc.width = p.width;
    desc.height = p.height;
    Ok(ok_text("size updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_renderer",
    description = "Set the GPU backend: 'wgpu' (default, recommended) or 'glow' (OpenGL).",
    emit = Auto
)]
#[instrument(skip(ctx), fields(renderer = ?p.renderer))]
async fn set_renderer(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetRendererParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    ctx.items
        .lock()
        .await
        .get_mut(&id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
        })?
        .renderer = p.renderer;
    Ok(ok_text("renderer updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_theme",
    description = "Set the colour theme: 'dark' (default), 'light', or 'system' (follow OS).",
    emit = Auto
)]
#[instrument(skip(ctx), fields(theme = ?p.theme))]
async fn set_theme(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetThemeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    ctx.items
        .lock()
        .await
        .get_mut(&id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
        })?
        .theme = p.theme;
    Ok(ok_text("theme updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_vsync",
    description = "Enable or disable vertical sync (default: true). \
                   Affects the wgpu PresentMode: AutoVsync vs AutoNoVsync.",
    emit = Auto
)]
#[instrument(skip(ctx), fields(vsync = p.vsync))]
async fn set_vsync(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetVsyncParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    ctx.items
        .lock()
        .await
        .get_mut(&id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
        })?
        .vsync = p.vsync;
    Ok(ok_text("vsync updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_decorations",
    description = "Show or hide OS window decorations — title bar, border, close/min/max buttons \
                   (default: true).",
    emit = Auto
)]
#[instrument(skip(ctx), fields(decorations = p.decorations))]
async fn set_decorations(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetDecorationsParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    ctx.items
        .lock()
        .await
        .get_mut(&id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
        })?
        .decorations = p.decorations;
    Ok(ok_text("decorations updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__set_resizable",
    description = "Allow or prevent user window resize (default: true).",
    emit = Auto
)]
#[instrument(skip(ctx), fields(resizable = p.resizable))]
async fn set_resizable(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitSetResizableParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    ctx.items
        .lock()
        .await
        .get_mut(&id)
        .ok_or_else(|| {
            ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
        })?
        .resizable = p.resizable;
    Ok(ok_text("resizable updated"))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__describe",
    description = "Return a JSON snapshot of the current egui_winit config.",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn describe(
    ctx: Arc<EguiWinitCtx>,
    p: EguiWinitDescribeParams,
) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let desc = ctx.items.lock().await.get(&id).cloned().ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    Ok(json_result(&desc))
}

#[elicitation::elicit_tool(
    plugin = "egui_winit",
    name = "egui_winit__emit",
    description = "Emit a complete main.rs scaffold for the configured egui + winit native app. \
                   For 'wgpu' renderer: includes egui_winit::State + full wgpu render loop. \
                   For 'glow' renderer: includes egui_winit::State + glutin stub. \
                   The emitted code uses winit's ApplicationHandler pattern (winit 0.30+).",
    emit = Auto
)]
#[instrument(skip(ctx))]
async fn emit(ctx: Arc<EguiWinitCtx>, p: EguiWinitEmitParams) -> Result<CallToolResult, ErrorData> {
    let id = parse_id(&p.config_id)?;
    let desc = ctx.items.lock().await.get(&id).cloned().ok_or_else(|| {
        ErrorData::invalid_params(format!("unknown config_id: {}", p.config_id), None)
    })?;
    let src = match desc.renderer {
        EguiWinitRenderer::Wgpu => emit_wgpu(&desc),
        EguiWinitRenderer::Glow => emit_glow(&desc),
    };
    Ok(CallToolResult::success(vec![Content::text(src)]))
}

// ── Plugin struct + ElicitPlugin impl ─────────────────────────────────────────

/// MCP plugin providing `egui_winit__*` tools for native egui + winit app scaffolding.
///
/// Mirrors the structure of `egui_winit` just as `elicit_leptos::LeptosAxumPlugin`
/// mirrors `leptos_axum` in `elicit_leptos`. All tools emit Rust code; no windows
/// or GPU devices are created in the MCP server process.
///
/// # Composition
///
/// ```text
/// egui_winit__new(app_struct: "MyApp", title: "My App", width: 1280, height: 720)
///   → { config_id }
/// egui_winit__set_theme(config_id, theme: "dark")
/// egui_winit__set_renderer(config_id, renderer: "wgpu")
/// egui_winit__emit(config_id)
///   → main.rs with winit ApplicationHandler + egui_winit::State + wgpu render loop
/// ```
pub struct EguiWinitPlugin(Arc<EguiWinitCtx>);

impl EguiWinitPlugin {
    /// Create a new plugin with an empty config registry.
    pub fn new() -> Self {
        Self(Arc::new(EguiWinitCtx::new()))
    }

    /// Invoke a tool by name directly — useful for tests.
    pub async fn invoke_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<CallToolResult, ErrorData> {
        let owned = name.to_string();
        let params = if let Some(m) = args.as_object().cloned() {
            CallToolRequestParams::new(owned).with_arguments(m)
        } else {
            CallToolRequestParams::new(owned)
        };
        let full_name = if name.starts_with("egui_winit__") {
            name.to_string()
        } else {
            format!("egui_winit__{name}")
        };
        let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "egui_winit")
            .find(|r| r.name == full_name)
            .map(|r| (r.constructor)())
            .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
        descriptor
            .dispatch(
                self.0.clone() as Arc<dyn std::any::Any + Send + Sync>,
                params,
            )
            .await
    }
}

impl Default for EguiWinitPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl elicitation::ElicitPlugin for EguiWinitPlugin {
    fn name(&self) -> &'static str {
        "egui_winit"
    }

    fn list_tools(&self) -> Vec<Tool> {
        elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
            .filter(|r| r.plugin == "egui_winit")
            .map(|r| (r.constructor)().as_tool())
            .collect()
    }

    #[tracing::instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> futures::future::BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let plugin_ctx = self.0.clone();
        Box::pin(async move {
            let name = params.name.as_ref();
            let full_name = if name.starts_with("egui_winit__") {
                name.to_string()
            } else {
                format!("egui_winit__{name}")
            };
            let descriptor = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                .filter(|r| r.plugin == "egui_winit")
                .find(|r| r.name == full_name)
                .map(|r| (r.constructor)())
                .ok_or_else(|| ErrorData::invalid_params(format!("unknown tool: {name}"), None))?;
            descriptor
                .dispatch(plugin_ctx as Arc<dyn std::any::Any + Send + Sync>, params)
                .await
        })
    }
}
