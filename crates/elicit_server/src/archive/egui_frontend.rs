//! Egui native-window frontend for the archive CLI.
//!
//! Renders a pgAdmin-style database browser in a native OS window using the
//! winit 0.30 `ApplicationHandler` pattern, `egui-winit` for event
//! integration, and `egui-wgpu` for GPU-accelerated rendering.
//!
//! The UI mirrors the ratatui frontend: Catppuccin Mocha dark theme, a
//! collapsible schema/table tree on the left, a column-detail panel in the
//! centre, and a keybinding bar at the bottom.
//!
//! Key bindings are sourced from [`ArchiveNavModel::bindings`] (the AccessKit
//! IR), keeping all frontends consistent.
//!
//! [`run_egui`] runs the event loop directly on the calling thread.
//! winit on Linux requires the event loop on the OS main thread; callers
//! must ensure they are not inside a spawned worker thread.  The archive
//! binary satisfies this because `#[tokio::main]` calls `Runtime::block_on`
//! on the OS main thread, so the surrounding `async fn main` body (and every
//! `await`-free call within it) executes there.

use std::sync::Arc;

use egui::{Color32, Key, RichText, ScrollArea, Vec2};
use egui_winit::State as EguiWinitState;
use tracing::instrument;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::archive::{
    ArchiveResult,
    errors::{ArchiveError, ArchiveErrorKind},
    nav_model::{ArchiveNavModel, FlatItem},
    nav_tree::NavTree,
    types::TableType,
};

// ── Catppuccin Mocha palette ──────────────────────────────────────────────────

const BASE: Color32 = Color32::from_rgb(30, 30, 46);
const SURFACE0: Color32 = Color32::from_rgb(49, 50, 68);
const SURFACE1: Color32 = Color32::from_rgb(69, 71, 90);
const TEXT: Color32 = Color32::from_rgb(205, 214, 244);
const SUBTEXT0: Color32 = Color32::from_rgb(166, 173, 200);
const BLUE: Color32 = Color32::from_rgb(137, 180, 250);
const MAUVE: Color32 = Color32::from_rgb(203, 166, 247);
const YELLOW: Color32 = Color32::from_rgb(249, 226, 175);
const OVERLAY0: Color32 = Color32::from_rgb(108, 112, 134);

// ── Application state ─────────────────────────────────────────────────────────

struct ArchiveEguiApp {
    model: ArchiveNavModel,
    should_quit: bool,
    // wgpu / egui-winit resources (None until `resumed`)
    window: Option<Arc<Window>>,
    egui_state: Option<EguiWinitState>,
    surface: Option<wgpu::Surface<'static>>,
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
    renderer: Option<egui_wgpu::Renderer>,
    surface_config: Option<wgpu::SurfaceConfiguration>,
}

impl ArchiveEguiApp {
    fn new(nav: NavTree) -> Self {
        Self {
            model: ArchiveNavModel::new(nav),
            should_quit: false,
            window: None,
            egui_state: None,
            surface: None,
            device: None,
            queue: None,
            renderer: None,
            surface_config: None,
        }
    }

    fn apply_theme(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = BASE;
        visuals.window_fill = SURFACE0;
        visuals.extreme_bg_color = SURFACE0;
        visuals.code_bg_color = SURFACE0;
        visuals.override_text_color = Some(TEXT);
        visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(137, 180, 250, 60);
        visuals.selection.stroke = egui::Stroke::new(1.0, BLUE);
        visuals.widgets.noninteractive.bg_fill = BASE;
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, SUBTEXT0);
        visuals.widgets.inactive.bg_fill = SURFACE0;
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, TEXT);
        visuals.widgets.hovered.bg_fill = SURFACE1;
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, TEXT);
        visuals.widgets.active.bg_fill = SURFACE1;
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, BLUE);
        ctx.set_visuals(visuals);
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        // ── Keyboard navigation (sourced from AccessKit IR) ───────────────────
        let (up, down, enter, refresh, toggle_help, quit) = ctx.input(|i| {
            (
                i.key_pressed(Key::ArrowUp) || i.key_pressed(Key::K),
                i.key_pressed(Key::ArrowDown) || i.key_pressed(Key::J),
                i.key_pressed(Key::Enter),
                i.key_pressed(Key::R),
                i.key_pressed(Key::Questionmark),
                i.key_pressed(Key::Q) || i.key_pressed(Key::Escape),
            )
        });
        if up {
            self.model.move_up();
        }
        if down {
            self.model.move_down();
        }
        if enter {
            self.model.toggle_expand();
        }
        if refresh {
            self.model.refresh();
        }
        if toggle_help {
            self.model.toggle_help();
        }
        if quit {
            self.should_quit = true;
        }

        // ── Status bar (chips from AccessKit IR) ──────────────────────────────
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(SURFACE1)
                    .inner_margin(egui::Margin::symmetric(12i8, 4i8)),
            )
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    for binding in ArchiveNavModel::bindings() {
                        ui.label(RichText::new(&binding.key).color(YELLOW).monospace());
                        ui.add_space(2.0);
                        ui.label(
                            RichText::new(binding.action.to_lowercase())
                                .color(SUBTEXT0)
                                .small(),
                        );
                        ui.add_space(8.0);
                    }
                    // Flash message in the status bar
                    if let Some(flash) = &self.model.flash {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(flash)
                                    .color(Color32::from_rgb(166, 227, 161))
                                    .small(),
                            );
                        });
                    }
                });
            });

        // ── Navigation panel (flat keyboard-navigable list) ───────────────────
        let cursor = self.model.cursor;
        egui::Panel::left("nav")
            .resizable(true)
            .default_size(260.0)
            .frame(
                egui::Frame::new()
                    .fill(BASE)
                    .inner_margin(egui::Margin::same(8i8)),
            )
            .show_inside(ui, |ui| {
                // Header: db name + version
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&self.model.db_name)
                            .color(BLUE)
                            .strong()
                            .size(14.0),
                    );
                    if let Some(v) = &self.model.version {
                        ui.label(RichText::new(format!("({})", v)).color(SUBTEXT0).small());
                    }
                });
                ui.separator();

                // Flat list with keyboard selection highlight
                // Collect any mouse-clicked row index first to avoid borrow conflicts.
                let mut clicked_row: Option<usize> = None;
                ScrollArea::vertical().show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    for (row_idx, item) in self.model.flat.iter().enumerate() {
                        let is_selected = row_idx == cursor;
                        match item {
                            FlatItem::Schema(si) => {
                                let s = &self.model.schemas[*si];
                                let arrow = if s.expanded { "▼ " } else { "▶ " };
                                let label = RichText::new(format!(
                                    "{arrow}{}  ({})",
                                    s.entry.name,
                                    s.entry.tables.len()
                                ))
                                .color(if is_selected { BLUE } else { MAUVE })
                                .strong();
                                let resp = ui.selectable_label(is_selected, label);
                                if resp.clicked() {
                                    clicked_row = Some(row_idx);
                                }
                                if is_selected {
                                    resp.scroll_to_me(Some(egui::Align::Center));
                                }
                            }
                            FlatItem::Table(si, ti) => {
                                let t = &self.model.schemas[*si].entry.tables[*ti];
                                let icon = match t.table_type {
                                    TableType::Table => "📋",
                                    TableType::View => "👁",
                                    TableType::MaterializedView => "💾",
                                    TableType::Unknown => "•",
                                };
                                let label = RichText::new(format!("   {icon} {}", t.table_name))
                                    .color(if is_selected { BLUE } else { TEXT })
                                    .small();
                                let resp = ui.selectable_label(is_selected, label);
                                if resp.clicked() {
                                    clicked_row = Some(row_idx);
                                }
                                if is_selected {
                                    resp.scroll_to_me(Some(egui::Align::Center));
                                }
                            }
                        }
                    }
                });
                // Apply any mouse-click after the borrow on flat ends.
                if let Some(row) = clicked_row {
                    self.model.cursor = row;
                    self.model.toggle_expand();
                }
            });

        // ── Central panel ─────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BASE)
                    .inner_margin(egui::Margin::same(16i8)),
            )
            .show_inside(ui, |ui| match self.model.selected() {
                Some(FlatItem::Schema(si)) => {
                    let s = &self.model.schemas[si];
                    ui.label(
                        RichText::new(format!("schema: {}", s.entry.name))
                            .color(BLUE)
                            .heading(),
                    );
                    ui.separator();
                    ui.label(RichText::new(format!("Owner: {}", s.entry.owner)).color(SUBTEXT0));
                    ui.label(
                        RichText::new(format!("{} tables / views", s.entry.tables.len()))
                            .color(SUBTEXT0),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new("Press Enter to expand/collapse")
                            .color(OVERLAY0)
                            .small(),
                    );
                }
                Some(FlatItem::Table(si, ti)) => {
                    let schema = &self.model.schemas[si];
                    let table = &schema.entry.tables[ti];
                    ui.label(
                        RichText::new(format!("{}.{}", schema.entry.name, table.table_name))
                            .color(BLUE)
                            .heading(),
                    );
                    ui.separator();
                    ui.label(RichText::new(format!("Type: {}", table.table_type)).color(SUBTEXT0));
                    if let Some(rows) = table.estimated_rows {
                        ui.label(RichText::new(format!("~{} rows", rows)).color(SUBTEXT0));
                    }
                    if !table.columns.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Columns").color(MAUVE).strong());
                        ui.separator();
                        egui::Grid::new("columns")
                            .num_columns(3)
                            .spacing([16.0, 2.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label(RichText::new("Name").color(SUBTEXT0).small());
                                ui.label(RichText::new("Type").color(SUBTEXT0).small());
                                ui.label(RichText::new("Nullable").color(SUBTEXT0).small());
                                ui.end_row();
                                for col in &table.columns {
                                    ui.label(RichText::new(&col.name).color(TEXT));
                                    ui.label(
                                        RichText::new(&col.sql_type)
                                            .color(BLUE)
                                            .monospace()
                                            .small(),
                                    );
                                    ui.label(if col.nullable {
                                        RichText::new("yes").color(OVERLAY0).small()
                                    } else {
                                        RichText::new("no").color(YELLOW).small()
                                    });
                                    ui.end_row();
                                }
                            });
                    }
                }
                None => {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            RichText::new("← Select a schema or table to inspect")
                                .color(OVERLAY0)
                                .size(16.0),
                        );
                    });
                }
            });

        // ── Help modal ────────────────────────────────────────────────────────
        if self.model.show_help {
            egui::Window::new("Keyboard Shortcuts")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(&ctx, |ui| {
                    egui::Grid::new("help_grid")
                        .num_columns(2)
                        .spacing([16.0, 4.0])
                        .show(ui, |ui| {
                            for binding in ArchiveNavModel::bindings() {
                                ui.label(RichText::new(&binding.key).color(YELLOW).monospace());
                                ui.label(RichText::new(&binding.action).color(TEXT));
                                ui.end_row();
                            }
                        });
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.model.show_help = false;
                    }
                });
        }
    }
}

// ── winit ApplicationHandler ──────────────────────────────────────────────────

impl ApplicationHandler for ArchiveEguiApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = WindowAttributes::default()
            .with_title(format!("Archive — {}", self.model.db_name))
            .with_inner_size(winit::dpi::LogicalSize::new(1280_f64, 720_f64));
        let window = Arc::new(event_loop.create_window(attrs).expect("create window"));

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance
            .create_surface(window.clone())
            .expect("create wgpu surface");

        // wgpu async init — safe to block here: running on a plain std thread,
        // not inside a Tokio executor.
        let (adapter, device, queue) = futures::executor::block_on(async {
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .expect(
                    "no suitable wgpu adapter — ensure Vulkan/Metal/DX12/GL drivers are installed",
                );
            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor::default())
                .await
                .expect("could not create wgpu device");
            (adapter, device, queue)
        });
        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let egui_ctx = egui::Context::default();
        Self::apply_theme(&egui_ctx);
        let egui_state = EguiWinitState::new(
            egui_ctx,
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(device.limits().max_texture_dimension_2d as usize),
        );
        let renderer =
            egui_wgpu::Renderer::new(&device, format, egui_wgpu::RendererOptions::default());

        self.window = Some(window);
        self.egui_state = Some(egui_state);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.renderer = Some(renderer);
        self.surface_config = Some(config);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = match self.window.as_ref() {
            Some(w) => w.clone(),
            None => return,
        };
        let state = match self.egui_state.as_mut() {
            Some(s) => s,
            None => return,
        };

        let response = state.on_window_event(&window, &event);
        if response.repaint {
            window.request_redraw();
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                if let (Some(surface), Some(device), Some(cfg)) = (
                    self.surface.as_ref(),
                    self.device.as_ref(),
                    self.surface_config.as_mut(),
                ) {
                    cfg.width = size.width.max(1);
                    cfg.height = size.height.max(1);
                    surface.configure(device, cfg);
                }
                window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                // Extract raw input and ctx without holding a borrow on `self`.
                let raw = self.egui_state.as_mut().unwrap().take_egui_input(&window);
                let ctx = self.egui_state.as_ref().unwrap().egui_ctx().clone();

                let out = ctx.run_ui(raw, |ui| self.render_ui(ui));

                if self.should_quit {
                    event_loop.exit();
                    return;
                }

                self.egui_state
                    .as_mut()
                    .unwrap()
                    .handle_platform_output(&window, out.platform_output);

                let (surface, device, queue, renderer, cfg) = match (
                    self.surface.as_ref(),
                    self.device.as_ref(),
                    self.queue.as_ref(),
                    self.renderer.as_mut(),
                    self.surface_config.as_ref(),
                ) {
                    (Some(s), Some(d), Some(q), Some(r), Some(c)) => (s, d, q, r, c),
                    _ => return,
                };

                let surface_tex = surface.get_current_texture();
                let texture = match surface_tex {
                    wgpu::CurrentSurfaceTexture::Success(t) => t,
                    _ => return,
                };
                let view = texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let clipped = ctx.tessellate(out.shapes, ctx.pixels_per_point());
                let screen = egui_wgpu::ScreenDescriptor {
                    size_in_pixels: [cfg.width, cfg.height],
                    pixels_per_point: ctx.pixels_per_point(),
                };
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                for (id, delta) in &out.textures_delta.set {
                    renderer.update_texture(device, queue, *id, delta);
                }
                renderer.update_buffers(device, queue, &mut encoder, &clipped, &screen);
                {
                    // `forget_lifetime()` converts RenderPass<'encoder> to RenderPass<'static>
                    // as required by egui-wgpu 0.34's Renderer::render signature.
                    let mut rpass = encoder
                        .begin_render_pass(&wgpu::RenderPassDescriptor {
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &view,
                                resolve_target: None,
                                depth_slice: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color {
                                        r: 30.0 / 255.0,
                                        g: 30.0 / 255.0,
                                        b: 46.0 / 255.0,
                                        a: 1.0,
                                    }),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            ..Default::default()
                        })
                        .forget_lifetime();
                    renderer.render(&mut rpass, &clipped, &screen);
                }
                queue.submit(std::iter::once(encoder.finish()));
                texture.present();
                for id in &out.textures_delta.free {
                    renderer.free_texture(id);
                }
                window.request_redraw();
            }

            _ => {}
        }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Run the egui native-window frontend, blocking until the user closes it.
///
/// Must be called from the OS main thread.  On Linux, winit refuses to build
/// an event loop on any other thread.  The archive binary satisfies this
/// constraint because `#[tokio::main]` drives `async fn main` on the main
/// OS thread via `Runtime::block_on`.
#[instrument(skip(nav), fields(db = %nav.db_name))]
pub fn run_egui(nav: NavTree) -> ArchiveResult<()> {
    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = ArchiveEguiApp::new(nav);
    event_loop
        .run_app(&mut app)
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))
}
