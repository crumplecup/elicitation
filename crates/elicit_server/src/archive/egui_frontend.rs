//! Egui native-window frontend for the archive CLI.
//!
//! Renders a pgAdmin-style database browser in a native OS window using the
//! winit 0.30 `ApplicationHandler` pattern, `egui-winit` for event
//! integration, and `egui-wgpu` for GPU-accelerated rendering.
//!
//! Key bindings are sourced from [`ArchiveNavModel::bindings`] (the AccessKit
//! IR), keeping all frontends consistent.
//!
//! [`run_egui`] runs the event loop directly on the calling thread.

use std::sync::Arc;

use egui::Key;
use egui_winit::State as EguiWinitState;
use tokio::sync::mpsc;
use tracing::instrument;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::archive::{
    ArchiveDbBackend, ArchiveResult, HistoryStore, SavedQueryStore,
    errors::{ArchiveError, ArchiveErrorKind},
    nav_model::{ArchiveNavModel, FetchRequest, PanelEvent, PanelMode},
    nav_tree::{NavTree, build_nav_tree},
    plugins::export::export_query_result,
    plugins::query::{execute_sql_direct, preview_table_direct},
};

// ── Catppuccin Mocha palette (used by apply_theme) ────────────────────────────

use egui::Color32;
const BASE: Color32 = Color32::from_rgb(30, 30, 46);
const SURFACE0: Color32 = Color32::from_rgb(49, 50, 68);
const SURFACE1: Color32 = Color32::from_rgb(69, 71, 90);
const TEXT: Color32 = Color32::from_rgb(205, 214, 244);
const SUBTEXT0: Color32 = Color32::from_rgb(166, 173, 200);
const BLUE: Color32 = Color32::from_rgb(137, 180, 250);

// ── Application state ─────────────────────────────────────────────────────────

struct ArchiveEguiApp {
    model: ArchiveNavModel,
    should_quit: bool,
    /// Sender to the background fetch task.
    req_tx: mpsc::Sender<FetchRequest>,
    /// Receiver for panel events from the fetch task.
    event_rx: mpsc::Receiver<PanelEvent>,
    /// SQLite-backed query history store (None if DB unavailable).
    history: Option<HistoryStore>,
    /// SQL text that is currently executing (for history recording).
    pending_sql: Option<String>,
    /// Instant the current SQL execution started.
    exec_start: Option<std::time::Instant>,
    /// Saved-query store (None if DB unavailable).
    saved: Option<SavedQueryStore>,
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
    fn new(
        nav: NavTree,
        req_tx: mpsc::Sender<FetchRequest>,
        event_rx: mpsc::Receiver<PanelEvent>,
        history: Option<HistoryStore>,
        saved: Option<SavedQueryStore>,
    ) -> Self {
        Self {
            model: ArchiveNavModel::new(nav),
            should_quit: false,
            req_tx,
            event_rx,
            history,
            pending_sql: None,
            exec_start: None,
            saved,
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

    fn run_sql(&mut self) {
        if let PanelMode::SqlEditor { text, running, .. } = &mut self.model.panel {
            if *running || text.trim().is_empty() {
                return;
            }
            let sql = text.trim().to_string();
            *running = true;
            self.exec_start = Some(std::time::Instant::now());
            self.pending_sql = Some(sql.clone());
            self.model.history_idx = None;
            let _ = self.req_tx.try_send(FetchRequest::ExecuteSql { sql });
        }
    }

    /// Drain the event channel and apply any pending panel events.
    fn poll_events(&mut self) {
        while let Ok(ev) = self.event_rx.try_recv() {
            match ev {
                PanelEvent::DataGrid {
                    schema,
                    table,
                    result,
                } => {
                    self.model.panel = PanelMode::DataGrid {
                        schema,
                        table,
                        result,
                        page: 0,
                    };
                    self.model.flash = None;
                }
                PanelEvent::NavRefreshed(nav) => {
                    self.model.apply_refresh(nav);
                }
                PanelEvent::FetchError(e) => {
                    if self.pending_sql.is_some() {
                        // SQL execution failed — keep editor open
                        if let PanelMode::SqlEditor { running, .. } = &mut self.model.panel {
                            *running = false;
                        }
                        self.pending_sql = None;
                        self.exec_start = None;
                    } else {
                        self.model.panel = PanelMode::ColumnDetail;
                    }
                    self.model.flash = Some(format!("⚠ {e}"));
                }
                PanelEvent::SqlResult(result) => {
                    // Record to history
                    if let Some(sql) = self.pending_sql.take() {
                        let duration_ms = self
                            .exec_start
                            .take()
                            .map(|t| t.elapsed().as_millis() as u64)
                            .unwrap_or(0);
                        let row_count = result.rows.rows.len() as u64;
                        let entry = crate::archive::QueryHistoryEntry {
                            id: 0,
                            sql: sql.clone(),
                            executed_at: chrono::Utc::now(),
                            duration_ms,
                            row_count: Some(row_count),
                            error: None,
                        };
                        if let Some(store) = &self.history {
                            store.append_spawn(sql, duration_ms, Some(row_count), None);
                        }
                        self.model.history_cache.insert(0, entry);
                    } else {
                        self.exec_start = None;
                    }
                    // Keep editor open, embed result
                    if let PanelMode::SqlEditor {
                        running,
                        result: res,
                        ..
                    } = &mut self.model.panel
                    {
                        *running = false;
                        *res = Some(result);
                    } else {
                        self.model.panel = PanelMode::DataGrid {
                            schema: String::new(),
                            table: "(query result)".to_string(),
                            result,
                            page: 0,
                        };
                    }
                }
                PanelEvent::TableInspected {
                    schema,
                    table,
                    inspection,
                } => {
                    self.model.store_inspection(schema, table, inspection);
                }
                PanelEvent::DdlReady { schema, table, ddl } => {
                    self.model.panel = PanelMode::Ddl { schema, table, ddl };
                }
                PanelEvent::ColumnStatsReady {
                    schema,
                    table,
                    stats,
                } => {
                    self.model.store_column_stats(schema, table, stats);
                }
                PanelEvent::ExplainReady {
                    schema,
                    table,
                    root,
                } => {
                    self.model.panel = PanelMode::ExplainPlan {
                        schema,
                        table,
                        root,
                    };
                }
                PanelEvent::ExportReady {
                    schema,
                    table,
                    content,
                    format,
                    row_count,
                } => {
                    self.model.last_export =
                        Some((schema.clone(), table.clone(), content, format.clone()));
                    let ext = format.extension();
                    self.model.flash = Some(format!(
                        "exported {row_count} rows from {schema}.{table} as .{ext}"
                    ));
                }
            }
        }
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        // Drain pending data events before rendering.
        self.poll_events();

        // ── Keyboard navigation ───────────────────────────────────────────────
        let (up, down, enter, refresh_key, toggle_help, quit, slash, esc_key) = ctx.input(|i| {
            (
                i.key_pressed(Key::ArrowUp) || i.key_pressed(Key::K),
                i.key_pressed(Key::ArrowDown) || i.key_pressed(Key::J),
                i.key_pressed(Key::Enter),
                i.key_pressed(Key::R),
                i.key_pressed(Key::Questionmark),
                i.key_pressed(Key::Q),
                i.key_pressed(Key::Slash),
                i.key_pressed(Key::Escape),
            )
        });

        if self.model.filter_active {
            let typed: String = ctx.input(|i| {
                i.events
                    .iter()
                    .filter_map(|e| {
                        if let egui::Event::Text(t) = e {
                            Some(t.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            for ch in typed.chars() {
                self.model.filter_push(ch);
            }
            if ctx.input(|i| i.key_pressed(Key::Backspace)) {
                self.model.filter_backspace();
            }
            if esc_key {
                self.model.close_filter();
            }
        } else if self.model.save_prompt_active {
            let typed: String = ctx.input(|i| {
                i.events
                    .iter()
                    .filter_map(|e| {
                        if let egui::Event::Text(t) = e {
                            Some(t.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            for ch in typed.chars() {
                self.model.save_prompt_push(ch);
            }
            if ctx.input(|i| i.key_pressed(Key::Backspace)) {
                self.model.save_prompt_backspace();
            }
            if esc_key {
                self.model.close_save_prompt();
            }
            if enter {
                if let Some(name) = self.model.take_save_prompt() {
                    if let PanelMode::SqlEditor { text, .. } = &self.model.panel {
                        let sql = text.trim().to_string();
                        if let Some(ref store) = self.saved {
                            store.save_spawn(name.clone(), sql.clone());
                        }
                        use crate::archive::SavedQuery;
                        let existing = self.model.saved_cache.iter().position(|q| q.name == name);
                        let now = chrono::Utc::now();
                        if let Some(idx) = existing {
                            self.model.saved_cache[idx].sql = sql;
                            self.model.saved_cache[idx].updated_at = now;
                        } else {
                            let new_q = SavedQuery {
                                id: 0,
                                name: name.clone(),
                                sql,
                                created_at: now,
                                updated_at: now,
                            };
                            let ins = self.model.saved_cache.partition_point(|q| q.name < name);
                            self.model.saved_cache.insert(ins, new_q);
                        }
                        self.model.flash = Some(format!("saved \"{name}\""));
                    }
                }
            }
        } else if self.model.saved_browser_active {
            let len = self.model.saved_cache.len();
            if esc_key || quit {
                self.model.toggle_saved_browser();
            }
            if up {
                self.model.saved_browser_prev();
            }
            if down && len > 0 {
                self.model.saved_browser_next();
            }
            if enter {
                let idx = self.model.saved_browser_idx;
                if let Some(q) = self.model.saved_cache.get(idx) {
                    let sql = q.sql.clone();
                    self.model.panel = PanelMode::SqlEditor {
                        text: sql,
                        result: None,
                        running: false,
                        error: None,
                    };
                }
            }
            if ctx.input(|i| i.key_pressed(Key::D)) {
                let idx = self.model.saved_browser_idx;
                if let Some(q) = self.model.saved_cache.get(idx) {
                    let id = q.id;
                    let name = q.name.clone();
                    if let Some(ref store) = self.saved {
                        store.delete_spawn(id);
                    }
                    self.model.saved_cache.remove(idx);
                    if idx > 0 && idx >= self.model.saved_cache.len() {
                        self.model.saved_browser_prev();
                    }
                    self.model.flash = Some(format!("deleted \"{name}\""));
                }
            }
        } else if self.model.export_picker {
            if esc_key {
                self.model.toggle_export_picker();
            }
            if up {
                self.model.export_picker_prev();
            }
            if down {
                self.model.export_picker_next();
            }
            if enter {
                let format = self.model.confirm_export_picker();
                if let Some(req) = self.model.export_request(format) {
                    let _ = self.req_tx.try_send(req);
                }
            }
        } else if matches!(self.model.panel, PanelMode::SqlEditor { .. }) {
            let (ctrl_enter, ctrl_up, ctrl_down) = ctx.input(|i| {
                let ctrl = i.modifiers.ctrl;
                (
                    ctrl && i.key_pressed(Key::Enter),
                    ctrl && i.key_pressed(Key::ArrowUp),
                    ctrl && i.key_pressed(Key::ArrowDown),
                )
            });
            if ctrl_enter || ctx.input(|i| i.key_pressed(Key::F5)) {
                self.run_sql();
            }
            if ctrl_up {
                self.model.history_prev();
            }
            if ctrl_down {
                self.model.history_next();
            }
            if esc_key {
                self.model.panel = PanelMode::ColumnDetail;
            }
            let typed: String = ctx.input(|i| {
                i.events
                    .iter()
                    .filter_map(|e| {
                        if let egui::Event::Text(t) = e {
                            Some(t.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            if !typed.is_empty() {
                if let PanelMode::SqlEditor { text, .. } = &mut self.model.panel {
                    text.push_str(&typed);
                }
            }
            if ctx.input(|i| i.key_pressed(Key::Backspace)) {
                if let PanelMode::SqlEditor { text, .. } = &mut self.model.panel {
                    text.pop();
                }
            }
            if !ctrl_enter && enter {
                if let PanelMode::SqlEditor { text, .. } = &mut self.model.panel {
                    text.push('\n');
                }
            }
            if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S)) {
                self.model.open_save_prompt();
            }
        } else {
            if up {
                self.model.move_up();
                if let Some(req) = self.model.inspect_request() {
                    let _ = self.req_tx.try_send(req);
                }
                if let Some(req) = self.model.stats_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            if down {
                self.model.move_down();
                if let Some(req) = self.model.inspect_request() {
                    let _ = self.req_tx.try_send(req);
                }
                if let Some(req) = self.model.stats_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            if enter {
                if let Some(req) = self.model.toggle_expand() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            if refresh_key {
                let req = self.model.request_refresh();
                let _ = self.req_tx.try_send(req);
            }
            if toggle_help {
                self.model.toggle_help();
            }
            if slash {
                self.model.open_filter();
            }
            if ctx.input(|i| i.key_pressed(Key::D)) {
                if let Some(req) = self.model.ddl_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            if ctx.input(|i| i.key_pressed(Key::E)) {
                if let Some(req) = self.model.explain_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            if ctx.input(|i| i.key_pressed(Key::S)) {
                self.model.panel = PanelMode::SqlEditor {
                    text: String::new(),
                    result: None,
                    running: false,
                    error: None,
                };
                self.model.toggle_saved_browser();
            }
            if ctx.input(|i| i.key_pressed(Key::X)) && self.model.panel.is_data_grid() {
                self.model.toggle_export_picker();
            }
            if quit || esc_key {
                self.should_quit = true;
            }
        }

        // ── IR pipeline: verified tree → egui bridge ─────────────────────────
        match self.model.to_verified_tree() {
            Ok((tree, ir_proof)) => {
                render_egui_from_ir(ui, &tree, ir_proof);
            }
            Err(e) => {
                ui.label(format!("IR error: {e}"));
            }
        }
    }
}

/// Gate function: proof token ensures the tree was minted by [`ArchiveNavModel::to_verified_tree`].
fn render_egui_from_ir(
    ui: &mut egui::Ui,
    tree: &elicit_ui::VerifiedTree,
    _proof: elicitation::Established<elicit_ui::IrSourced>,
) {
    elicit_egui::render_tree(ui, tree.nodes(), tree.root());
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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Non-blocking channel drain — if data arrived, request a repaint.
        if !self.event_rx.is_empty() {
            if let Some(window) = self.window.as_ref() {
                window.request_redraw();
            }
        }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Run the egui native-window frontend, blocking until the user closes it.
///
/// Must be called from the OS main thread.
#[instrument(skip(nav), fields(db = %nav.db_name))]
pub fn run_egui(nav: NavTree, url: Option<String>) -> ArchiveResult<()> {
    let (req_tx, mut req_rx) = mpsc::channel::<FetchRequest>(32);
    let (event_tx, event_rx) = mpsc::channel::<PanelEvent>(32);

    // Spawn the background fetch task if we have a URL.
    if let Some(url_str) = url.clone() {
        let tx = event_tx.clone();
        tokio::spawn(async move {
            while let Some(req) = req_rx.recv().await {
                let result = match req {
                    FetchRequest::PreviewTable { schema, table } => {
                        match preview_table_direct(&url_str, &schema, &table, 200).await {
                            Ok(r) => PanelEvent::DataGrid {
                                schema,
                                table,
                                result: r,
                            },
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::ExecuteSql { sql } => {
                        match execute_sql_direct(&url_str, &sql).await {
                            Ok(r) => PanelEvent::SqlResult(r),
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::Refresh => match ArchiveDbBackend::connect(&url_str).await {
                        Ok(b) => match build_nav_tree(&b, &url_str).await {
                            Ok(t) => PanelEvent::NavRefreshed(t),
                            Err(e) => PanelEvent::FetchError(e.to_string()),
                        },
                        Err(e) => PanelEvent::FetchError(e.to_string()),
                    },
                    FetchRequest::InspectTable { schema, table } => {
                        use crate::archive::plugins::inspect::inspect_table_direct;
                        match inspect_table_direct(&url_str, &schema, &table).await {
                            Ok(inspection) => PanelEvent::TableInspected {
                                schema,
                                table,
                                inspection,
                            },
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::GetDdl { schema, table } => {
                        use crate::archive::plugins::inspect::generate_ddl_direct;
                        match generate_ddl_direct(&url_str, &schema, &table).await {
                            Ok(ddl) => PanelEvent::DdlReady {
                                schema,
                                table,
                                ddl: ddl.ddl,
                            },
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::GetColumnStats { schema, table } => {
                        use crate::archive::plugins::inspect::get_column_stats_direct;
                        match get_column_stats_direct(&url_str, &schema, &table).await {
                            Ok(stats) => PanelEvent::ColumnStatsReady {
                                schema,
                                table,
                                stats,
                            },
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::ExplainSql { schema, table, sql } => {
                        use crate::archive::plugins::inspect::explain_sql_direct;
                        match explain_sql_direct(&url_str, &sql).await {
                            Ok(root) => PanelEvent::ExplainReady {
                                schema,
                                table,
                                root,
                            },
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::ExportData {
                        schema,
                        table,
                        result,
                        format,
                    } => {
                        let export = export_query_result(&result, format.clone());
                        PanelEvent::ExportReady {
                            schema,
                            table,
                            content: export.content,
                            row_count: export.row_count,
                            format,
                        }
                    }
                };
                let _ = tx.send(result).await;
            }
        });
    }

    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    // Initialize history store synchronously using the current tokio runtime.
    let history = tokio::runtime::Handle::current()
        .block_on(HistoryStore::open())
        .ok();
    let history_cache = if let Some(ref store) = history {
        tokio::runtime::Handle::current()
            .block_on(store.recent(crate::archive::plugins::history::MAX_HISTORY))
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    // Saved-query store reuses the same SQLite file.
    let saved = tokio::runtime::Handle::current()
        .block_on(SavedQueryStore::open())
        .ok();
    let saved_cache = if let Some(ref store) = saved {
        tokio::runtime::Handle::current()
            .block_on(store.all())
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    let mut app = ArchiveEguiApp::new(nav, req_tx, event_rx, history, saved);
    app.model.history_cache = history_cache;
    app.model.saved_cache = saved_cache;
    event_loop
        .run_app(&mut app)
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))
}
