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
    ArchiveDbBackend, ArchiveResult, BackendKind, ConnectionProfile, ConnectionSet, HistoryStore,
    SavedQueryStore,
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
    model: ConnectionSet,
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
        connections: ConnectionSet,
        req_tx: mpsc::Sender<FetchRequest>,
        event_rx: mpsc::Receiver<PanelEvent>,
        history: Option<HistoryStore>,
        saved: Option<SavedQueryStore>,
    ) -> Self {
        Self {
            model: connections,
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
                        grid_row: 0,
                        grid_col: 0,
                        edit_state: None,
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
                            grid_row: 0,
                            grid_col: 0,
                            edit_state: None,
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
                PanelEvent::MonitorReady(snapshot) => {
                    self.model.apply_monitor_snapshot(snapshot);
                }
                PanelEvent::AdminReady(snapshot) => {
                    self.model.apply_admin_snapshot(snapshot);
                }
            }
        }
    }

    fn render_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        // Drain pending data events before rendering.
        self.poll_events();

        // ── Keyboard navigation ───────────────────────────────────────────────
        {
            use crate::archive::frontend_trait::ArchiveFrontend as _;
            let keymap = crate::archive::ArchiveKeyMap::default_map();
            let mode = self.model.current_mode();
            let combos = egui_events_to_combos(&ctx);
            for combo in combos {
                if let Some(action) = keymap.resolve(&combo, mode) {
                    if self.dispatch_action(action) {
                        self.should_quit = true;
                    }
                    break;
                }
            }
            // Text input for modes that accept printable characters
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
                self.dispatch_text(&typed);
            }
        }

        // ── IR pipeline: verified tree → egui bridge ─────────────────────────
        match self.model.to_verified_tree() {
            Ok((tree, ir_proof)) => {
                let clicked = render_egui_from_ir(ui, &tree, ir_proof);
                for node_id in clicked {
                    if let Some(node) = tree.nodes().get(&node_id) {
                        self.dispatch_toolbar_click(node);
                    }
                }
            }
            Err(e) => {
                ui.label(format!("IR error: {e}"));
            }
        }
    }

    /// Dispatch a toolbar button click to the appropriate model action.
    fn dispatch_toolbar_click(&mut self, node: &accesskit::Node) {
        use crate::archive::nav_model::PanelMode;
        let label = node.label().unwrap_or("").to_string();
        match label.as_str() {
            "SQL Editor" => {
                let text = if let PanelMode::SqlEditor { text, .. } = &self.model.panel {
                    text.clone()
                } else {
                    String::new()
                };
                self.model.panel = PanelMode::SqlEditor {
                    text,
                    result: None,
                    running: false,
                    error: None,
                };
            }
            "DDL" => {
                if let Some(req) = self.model.ddl_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            "EXPLAIN" => {
                if let Some(req) = self.model.explain_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            "Col Detail" => {
                self.model.panel = PanelMode::ColumnDetail;
            }
            "History" => {
                self.model.panel = PanelMode::HistoryPanel {
                    entries: self.model.history_cache.clone(),
                };
            }
            "Saved" => {
                self.model.toggle_saved_browser();
            }
            "Save SQL" => {
                self.model.open_save_prompt();
            }
            "Export" => {
                self.model.toggle_export_picker();
            }
            "⟳ Refresh" => {
                let req = self.model.request_refresh();
                let _ = self.req_tx.try_send(req);
            }
            "?" => {
                self.model.toggle_help();
            }
            _ => {}
        }
    }
}

/// Gate function: proof token ensures the tree was minted by [`ArchiveNavModel::to_verified_tree`].
///
/// Returns the [`accesskit::NodeId`]s of any toolbar buttons clicked this frame.
fn render_egui_from_ir(
    ui: &mut egui::Ui,
    tree: &elicit_ui::VerifiedTree,
    _proof: elicitation::Established<elicit_ui::IrSourced>,
) -> Vec<accesskit::NodeId> {
    let (_stats, clicked) = elicit_egui::render_tree(ui, tree.nodes(), tree.root());
    clicked
}

// ── Egui key-combo helper ─────────────────────────────────────────────────────

/// Collect all key-press events from the current egui frame into
/// platform-neutral [`KeyCombo`]s that the [`ArchiveKeyMap`] can resolve.
fn egui_events_to_combos(ctx: &egui::Context) -> Vec<crate::archive::KeyCombo> {
    use crate::archive::{ArchiveKey, KeyCombo};
    ctx.input(|i| {
        let mut out = Vec::new();
        for ev in &i.events {
            if let egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } = ev
            {
                let ak = match key {
                    Key::ArrowUp | Key::K => ArchiveKey::Up,
                    Key::ArrowDown | Key::J => ArchiveKey::Down,
                    Key::Enter => ArchiveKey::Enter,
                    Key::Escape => ArchiveKey::Esc,
                    Key::Backspace => ArchiveKey::Backspace,
                    Key::Delete => ArchiveKey::Delete,
                    Key::Tab => ArchiveKey::Tab,
                    Key::F1 => ArchiveKey::F(1),
                    Key::F2 => ArchiveKey::F(2),
                    Key::F3 => ArchiveKey::F(3),
                    Key::F4 => ArchiveKey::F(4),
                    Key::F5 => ArchiveKey::F(5),
                    Key::A => ArchiveKey::Char('a'),
                    Key::B => ArchiveKey::Char('b'),
                    Key::C => ArchiveKey::Char('c'),
                    Key::D => ArchiveKey::Char('d'),
                    Key::E => ArchiveKey::Char('e'),
                    Key::F => ArchiveKey::Char('f'),
                    Key::G => ArchiveKey::Char('g'),
                    Key::H => ArchiveKey::Char('h'),
                    Key::I => ArchiveKey::Char('i'),
                    Key::L => ArchiveKey::Char('l'),
                    Key::M => ArchiveKey::Char('m'),
                    Key::N => ArchiveKey::Char('n'),
                    Key::O => ArchiveKey::Char('o'),
                    Key::P => ArchiveKey::Char('p'),
                    Key::Q => ArchiveKey::Char('q'),
                    Key::R => ArchiveKey::Char('r'),
                    Key::S => ArchiveKey::Char('s'),
                    Key::T => ArchiveKey::Char('t'),
                    Key::U => ArchiveKey::Char('u'),
                    Key::V => ArchiveKey::Char('v'),
                    Key::W => ArchiveKey::Char('w'),
                    Key::X => ArchiveKey::Char('x'),
                    Key::Y => ArchiveKey::Char('y'),
                    Key::Z => ArchiveKey::Char('z'),
                    Key::Slash => ArchiveKey::Char('/'),
                    Key::Questionmark => ArchiveKey::Char('?'),
                    _ => continue,
                };
                out.push(KeyCombo {
                    key: ak,
                    ctrl: modifiers.ctrl,
                    shift: modifiers.shift,
                    alt: modifiers.alt,
                });
            }
        }
        out
    })
}

// ── ArchiveFrontend implementation ───────────────────────────────────────────

impl crate::archive::frontend_trait::ArchiveFrontend for ArchiveEguiApp {
    fn dispatch_action(&mut self, action: crate::archive::ArchiveAction) -> bool {
        use crate::archive::ArchiveAction as A;
        use crate::archive::nav_model::PanelMode;
        match action {
            A::MoveUp => {
                self.model.move_up();
                if let Some(req) = self.model.inspect_request() {
                    let _ = self.req_tx.try_send(req);
                }
                if let Some(req) = self.model.stats_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::MoveDown => {
                self.model.move_down();
                if let Some(req) = self.model.inspect_request() {
                    let _ = self.req_tx.try_send(req);
                }
                if let Some(req) = self.model.stats_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::Select => {
                if let Some(req) = self.model.toggle_expand() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::Refresh => {
                let req = self.model.request_refresh();
                let _ = self.req_tx.try_send(req);
            }
            A::ToggleHelp => self.model.toggle_help(),
            A::OpenFilter => self.model.open_filter(),
            A::OpenSqlEditor => {
                self.model.panel = PanelMode::SqlEditor {
                    text: String::new(),
                    result: None,
                    running: false,
                    error: None,
                };
            }
            A::OpenSavedBrowser => self.model.toggle_saved_browser(),
            A::OpenMonitor => {
                if let Some(req) = self.model.toggle_monitor_panel() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::OpenAdmin => {
                if let Some(req) = self.model.toggle_admin_panel() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::AdminTabNext => self.model.admin_tab_next(),
            A::AdminTabPrev => self.model.admin_tab_prev(),
            A::ToggleExportPicker => {
                if self.model.panel.is_data_grid() {
                    self.model.toggle_export_picker();
                }
            }
            A::RequestDdl => {
                if let Some(req) = self.model.ddl_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::RequestExplain => {
                if let Some(req) = self.model.explain_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::ConnNext => {
                self.model.conn_next();
                if let Some(url) = self.model.conn_active_url() {
                    let _ = self.req_tx.try_send(FetchRequest::UpdateUrl(url));
                }
            }
            A::ConnPrev => {
                self.model.conn_prev();
                if let Some(url) = self.model.conn_active_url() {
                    let _ = self.req_tx.try_send(FetchRequest::UpdateUrl(url));
                }
            }
            A::Quit => return true,
            A::FilterClose => self.model.close_filter(),
            A::FilterBackspace => self.model.filter_backspace(),
            A::SavePromptClose => self.model.close_save_prompt(),
            A::SavePromptBackspace => self.model.save_prompt_backspace(),
            A::SavePromptConfirm => {
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
                            let ins = self.model.saved_cache.partition_point(|q| q.name < name);
                            self.model.saved_cache.insert(
                                ins,
                                SavedQuery {
                                    id: 0,
                                    name: name.clone(),
                                    sql,
                                    created_at: now,
                                    updated_at: now,
                                },
                            );
                        }
                        self.model.flash = Some(format!("saved \"{name}\""));
                    }
                }
            }
            A::SavedBrowserClose => self.model.toggle_saved_browser(),
            A::SavedBrowserUp => self.model.saved_browser_prev(),
            A::SavedBrowserDown => {
                if !self.model.saved_cache.is_empty() {
                    self.model.saved_browser_next();
                }
            }
            A::SavedBrowserSelect => {
                if let Some(sql) = self.model.load_focused_saved_query_text() {
                    self.model.panel = PanelMode::SqlEditor {
                        text: sql,
                        result: None,
                        running: false,
                        error: None,
                    };
                    self.model.toggle_saved_browser();
                }
            }
            A::SavedBrowserDelete => {
                if let Some((id, name)) = self.model.remove_focused_saved_query() {
                    if let Some(ref store) = self.saved {
                        store.delete_spawn(id);
                    }
                    self.model.flash = Some(format!("deleted \"{name}\""));
                }
            }
            A::ExportPickerClose => self.model.toggle_export_picker(),
            A::ExportPickerUp => self.model.export_picker_prev(),
            A::ExportPickerDown => self.model.export_picker_next(),
            A::ExportPickerConfirm => {
                let format = self.model.confirm_export_picker();
                if let Some(req) = self.model.export_request(format) {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::SqlRun => self.run_sql(),
            A::SqlHistoryPrev => {
                self.model.history_prev();
            }
            A::SqlHistoryNext => {
                self.model.history_next();
            }
            A::SqlClose => {
                self.model.panel = PanelMode::ColumnDetail;
            }
            A::SqlSave => self.model.open_save_prompt(),
            A::SqlBackspace => {
                if let PanelMode::SqlEditor { text, .. } = &mut self.model.panel {
                    text.pop();
                }
            }
            A::SqlNewline => {
                if let PanelMode::SqlEditor { text, .. } = &mut self.model.panel {
                    text.push('\n');
                }
            }
        }
        false
    }

    fn dispatch_text(&mut self, text: &str) {
        use crate::archive::actions::KeyMapMode;
        use crate::archive::nav_model::PanelMode;
        match self.model.current_mode() {
            KeyMapMode::Filter => {
                for ch in text.chars() {
                    self.model.filter_push(ch);
                }
            }
            KeyMapMode::SavePrompt => {
                for ch in text.chars() {
                    self.model.save_prompt_push(ch);
                }
            }
            KeyMapMode::SqlEditor => {
                if let PanelMode::SqlEditor { text: buf, .. } = &mut self.model.panel {
                    buf.push_str(text);
                }
            }
            _ => {}
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

    // Spawn the background fetch task. The URL can be updated dynamically via
    // FetchRequest::UpdateUrl for multi-connection switching.
    {
        let tx = event_tx.clone();
        let url_for_task = url.clone();
        tokio::spawn(async move {
            let mut current_url = url_for_task;
            while let Some(req) = req_rx.recv().await {
                // Handle URL switch before any DB access.
                if let FetchRequest::UpdateUrl(new_url) = req {
                    current_url = Some(new_url.clone());
                    // Auto-refresh on the new connection.
                    match ArchiveDbBackend::connect(&new_url).await {
                        Ok(b) => match build_nav_tree(&b, &new_url).await {
                            Ok(t) => {
                                let _ = tx.send(PanelEvent::NavRefreshed(t)).await;
                            }
                            Err(e) => {
                                let _ = tx.send(PanelEvent::FetchError(e.to_string())).await;
                            }
                        },
                        Err(e) => {
                            let _ = tx.send(PanelEvent::FetchError(e.to_string())).await;
                        }
                    }
                    continue;
                }
                let Some(ref url_str) = current_url else {
                    let _ = tx
                        .send(PanelEvent::FetchError(
                            "No database URL — run 'archive serve' instead of 'archive demo'."
                                .to_string(),
                        ))
                        .await;
                    continue;
                };
                let result = match req {
                    FetchRequest::PreviewTable { schema, table } => {
                        match preview_table_direct(url_str, &schema, &table, 200).await {
                            Ok(r) => PanelEvent::DataGrid {
                                schema,
                                table,
                                result: r,
                            },
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::ExecuteSql { sql } => {
                        match execute_sql_direct(url_str, &sql).await {
                            Ok(r) => PanelEvent::SqlResult(r),
                            Err(e) => PanelEvent::FetchError(e),
                        }
                    }
                    FetchRequest::Refresh => match ArchiveDbBackend::connect(url_str).await {
                        Ok(b) => match build_nav_tree(&b, url_str).await {
                            Ok(t) => PanelEvent::NavRefreshed(t),
                            Err(e) => PanelEvent::FetchError(e.to_string()),
                        },
                        Err(e) => PanelEvent::FetchError(e.to_string()),
                    },
                    FetchRequest::InspectTable { schema, table } => {
                        use crate::archive::plugins::inspect::inspect_table_direct;
                        match inspect_table_direct(url_str, &schema, &table).await {
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
                    // Already handled above before the URL check.
                    FetchRequest::UpdateUrl(_) => {
                        unreachable!("UpdateUrl handled before this match")
                    }
                    FetchRequest::FetchMonitor => {
                        use crate::archive::MonitorSnapshot;
                        use elicit_db::{DbMonitor, DbRoleManager};
                        match ArchiveDbBackend::connect(url_str).await {
                            Ok(backend) => {
                                let sessions = backend
                                    .active_sessions()
                                    .await
                                    .map(|a| a.sessions)
                                    .unwrap_or_default();
                                let roles = backend.list_roles().await.unwrap_or_default();
                                let cache_hit = backend.cache_hit_ratio().await.ok();
                                PanelEvent::MonitorReady(MonitorSnapshot {
                                    sessions,
                                    roles,
                                    cache_hit,
                                    backups: Vec::new(),
                                })
                            }
                            Err(e) => PanelEvent::FetchError(e.to_string()),
                        }
                    }
                    FetchRequest::FetchAdmin => {
                        use crate::archive::{AdminSnapshot, AdminTab};
                        use elicit_db::{DbBackupManager, DbRoleManager, DbServerAdmin};
                        match ArchiveDbBackend::connect(url_str).await {
                            Ok(backend) => {
                                let roles = backend.list_roles().await.unwrap_or_default();
                                let backups = backend.list_backups().await.unwrap_or_default();
                                let wal_ready = backend.wal_status().await.is_ok();
                                let server_version =
                                    backend.server_version().await.unwrap_or_default();
                                let extensions =
                                    backend.list_extensions().await.unwrap_or_default();
                                let settings = backend.list_settings().await.unwrap_or_default();
                                PanelEvent::AdminReady(AdminSnapshot {
                                    roles,
                                    backups,
                                    wal_ready,
                                    server_version,
                                    extensions,
                                    settings,
                                    active_tab: AdminTab::Roles,
                                })
                            }
                            Err(e) => PanelEvent::FetchError(e.to_string()),
                        }
                    }
                };
                let _ = tx.send(result).await;
            }
        });
    }

    let event_loop = EventLoop::new().expect("create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    // Initialize history store synchronously. `block_in_place` yields the
    // tokio thread so `block_on` doesn't panic inside an async runtime.
    let (history, history_cache, saved, saved_cache) = tokio::task::block_in_place(|| {
        let handle = tokio::runtime::Handle::current();
        let history = handle.block_on(HistoryStore::open()).ok();
        let history_cache = if let Some(ref store) = history {
            handle
                .block_on(store.recent(crate::archive::plugins::history::MAX_HISTORY))
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let saved = handle.block_on(SavedQueryStore::open()).ok();
        let saved_cache = if let Some(ref store) = saved {
            handle.block_on(store.all()).unwrap_or_default()
        } else {
            Vec::new()
        };
        (history, history_cache, saved, saved_cache)
    });

    let profile = ConnectionProfile {
        name: "primary".to_string(),
        url_env_key: url.clone().unwrap_or_default(),
        backend: BackendKind::Postgres,
        color: None,
    };
    let connections = ConnectionSet::from_single(profile, ArchiveNavModel::new(nav), url);
    let mut app = ArchiveEguiApp::new(connections, req_tx, event_rx, history, saved);
    app.model.history_cache = history_cache;
    app.model.saved_cache = saved_cache;
    event_loop
        .run_app(&mut app)
        .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))
}
