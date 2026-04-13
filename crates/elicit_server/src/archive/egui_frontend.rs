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

use egui::{Color32, Key, RichText, ScrollArea, Vec2};
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
    ArchiveDbBackend, ArchiveResult, ExportFormat, HistoryStore, SavedQueryStore,
    errors::{ArchiveError, ArchiveErrorKind},
    nav_model::{ArchiveNavModel, FetchRequest, FlatItem, PanelEvent, PanelMode},
    nav_tree::{NavTree, build_nav_tree},
    plugins::export::export_query_result,
    plugins::query::{execute_sql_direct, preview_table_direct},
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
const GREEN: Color32 = Color32::from_rgb(166, 227, 161);
const RED: Color32 = Color32::from_rgb(243, 139, 168);

// ── Application state ─────────────────────────────────────────────────────────

struct ArchiveEguiApp {
    model: ArchiveNavModel,
    should_quit: bool,
    /// Sender to the background fetch task.
    req_tx: mpsc::Sender<FetchRequest>,
    /// Receiver for panel events from the fetch task.
    event_rx: mpsc::Receiver<PanelEvent>,
    /// When true, the export format picker overlay is shown.
    export_picker: bool,
    /// Currently highlighted option in the export picker (0–3).
    export_picker_idx: usize,
    /// SQLite-backed query history store (None if DB unavailable).
    history: Option<HistoryStore>,
    /// SQL text that is currently executing (for history recording).
    pending_sql: Option<String>,
    /// Instant the current SQL execution started.
    exec_start: Option<std::time::Instant>,
    /// Saved-query store (None if DB unavailable).
    saved: Option<SavedQueryStore>,
    /// When true the save-name prompt modal is shown.
    save_prompt_active: bool,
    /// Text being typed into the save-name prompt.
    save_prompt_text: String,
    /// When true the saved-queries browser window is shown.
    saved_browser_active: bool,
    /// Currently highlighted row in the saved-queries browser.
    saved_browser_idx: usize,
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
            export_picker: false,
            export_picker_idx: 0,
            history,
            pending_sql: None,
            exec_start: None,
            saved,
            save_prompt_active: false,
            save_prompt_text: String::new(),
            saved_browser_active: false,
            saved_browser_idx: 0,
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

        // ── Keyboard navigation (sourced from AccessKit IR) ───────────────────
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
            // In filter mode: collect typed characters
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
        } else if self.save_prompt_active {
            // Save-name prompt
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
            self.save_prompt_text.push_str(&typed);
            if ctx.input(|i| i.key_pressed(Key::Backspace)) {
                self.save_prompt_text.pop();
            }
            if esc_key {
                self.save_prompt_active = false;
                self.save_prompt_text.clear();
            }
            if enter {
                let name = self.save_prompt_text.trim().to_string();
                if !name.is_empty() {
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
                self.save_prompt_active = false;
                self.save_prompt_text.clear();
            }
        } else if self.saved_browser_active {
            // Saved-queries browser keyboard
            let len = self.model.saved_cache.len();
            if esc_key || quit {
                self.saved_browser_active = false;
            }
            if up && self.saved_browser_idx > 0 {
                self.saved_browser_idx -= 1;
            }
            if down && len > 0 {
                self.saved_browser_idx = (self.saved_browser_idx + 1).min(len - 1);
            }
            if enter {
                if let Some(q) = self.model.saved_cache.get(self.saved_browser_idx) {
                    let sql = q.sql.clone();
                    self.model.panel = PanelMode::SqlEditor {
                        text: sql,
                        result: None,
                        running: false,
                    };
                    self.saved_browser_active = false;
                }
            }
            if ctx.input(|i| i.key_pressed(Key::D)) {
                if let Some(q) = self.model.saved_cache.get(self.saved_browser_idx) {
                    let id = q.id;
                    let name = q.name.clone();
                    if let Some(ref store) = self.saved {
                        store.delete_spawn(id);
                    }
                    self.model.saved_cache.remove(self.saved_browser_idx);
                    if self.saved_browser_idx > 0
                        && self.saved_browser_idx >= self.model.saved_cache.len()
                    {
                        self.saved_browser_idx -= 1;
                    }
                    self.model.flash = Some(format!("deleted \"{name}\""));
                }
            }
        } else if self.export_picker {
            if esc_key {
                self.export_picker = false;
            }
            if up && self.export_picker_idx > 0 {
                self.export_picker_idx -= 1;
            }
            if down {
                self.export_picker_idx = (self.export_picker_idx + 1).min(3);
            }
            if enter {
                let format = match self.export_picker_idx {
                    0 => ExportFormat::Csv,
                    1 => ExportFormat::Json,
                    2 => ExportFormat::Ndjson,
                    _ => ExportFormat::Tsv,
                };
                self.export_picker = false;
                if let Some(req) = self.model.export_request(format) {
                    let _ = self.req_tx.try_send(req);
                }
            }
        } else if matches!(self.model.panel, PanelMode::SqlEditor { .. }) {
            // SQL editor keyboard handling
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
            // Text input for SQL editing
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
            // Ctrl+S → save prompt
            if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S)) {
                self.save_prompt_active = true;
                self.save_prompt_text.clear();
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
            // d → DDL viewer
            if ctx.input(|i| i.key_pressed(Key::D)) {
                if let Some(req) = self.model.ddl_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            // e → EXPLAIN viewer
            if ctx.input(|i| i.key_pressed(Key::E)) {
                if let Some(req) = self.model.explain_request() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            // s → open SQL editor
            if ctx.input(|i| i.key_pressed(Key::S)) {
                self.model.panel = PanelMode::SqlEditor {
                    text: String::new(),
                    result: None,
                    running: false,
                };
            }
            // F2 → saved queries browser
            if ctx.input(|i| i.key_pressed(Key::F2)) {
                self.saved_browser_active = true;
                self.saved_browser_idx = 0;
            }
            // x → export format picker
            if ctx.input(|i| i.key_pressed(Key::X)) && self.model.panel.is_data_grid() {
                self.export_picker = true;
                self.export_picker_idx = 0;
            }
            if quit || esc_key {
                self.should_quit = true;
            }
        }

        // ── Status bar ────────────────────────────────────────────────────────
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
                    // Extra bindings not in the core IR
                    ui.label(RichText::new("/").color(YELLOW).monospace());
                    ui.add_space(2.0);
                    ui.label(RichText::new("filter").color(SUBTEXT0).small());
                    ui.add_space(8.0);

                    if let Some(flash) = &self.model.flash {
                        let color = if flash.starts_with('⚠') { RED } else { GREEN };
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(flash).color(color).small());
                        });
                    }
                });
            });

        // ── Navigation panel ──────────────────────────────────────────────────
        let cursor = self.model.cursor;
        let filter_active = self.model.filter_active;
        let filter_text = self.model.filter.clone();

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

                // Filter bar
                if filter_active {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("/").color(YELLOW).monospace());
                        ui.label(
                            RichText::new(format!("{filter_text}_"))
                                .color(GREEN)
                                .monospace()
                                .small(),
                        );
                    });
                }

                ui.separator();

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
                // Mouse click: set cursor then toggle expand
                if let Some(row) = clicked_row {
                    self.model.cursor = row;
                    if let Some(req) = self.model.toggle_expand() {
                        let _ = self.req_tx.try_send(req);
                    }
                }
            });

        // ── Central panel ─────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BASE)
                    .inner_margin(egui::Margin::same(12i8)),
            )
            .show_inside(ui, |ui| {
                // Clone to avoid borrow issues on panel match
                let panel = std::mem::replace(&mut self.model.panel, PanelMode::ColumnDetail);
                match &panel {
                    PanelMode::ColumnDetail => {
                        self.model.panel = panel;
                        self.render_column_detail(ui);
                    }
                    PanelMode::Loading { schema, table } => {
                        ui.centered_and_justified(|ui| {
                            ui.label(
                                RichText::new(format!("⟳ Loading {schema}.{table}…"))
                                    .color(YELLOW)
                                    .size(16.0),
                            );
                        });
                        self.model.panel = PanelMode::Loading {
                            schema: schema.clone(),
                            table: table.clone(),
                        };
                        // Request redraw to poll for the result
                        ctx.request_repaint();
                    }
                    PanelMode::DataGrid {
                        schema,
                        table,
                        result,
                        page,
                    } => {
                        let schema = schema.clone();
                        let table = table.clone();
                        let result = result.clone();
                        let page = *page;
                        self.render_data_grid(ui, &schema, &table, &result, page);
                        self.model.panel = PanelMode::DataGrid {
                            schema,
                            table,
                            result,
                            page,
                        };
                    }
                    PanelMode::SqlEditor {
                        text,
                        result,
                        running,
                    } => {
                        let text = text.clone();
                        let result = result.clone();
                        let running = *running;
                        // Minimal SQL editor — Phase 1.2
                        ui.label(RichText::new("SQL Editor (Phase 1.2)").color(MAUVE));
                        self.model.panel = PanelMode::SqlEditor {
                            text,
                            result,
                            running,
                        };
                    }
                    PanelMode::Ddl { schema, table, ddl } => {
                        let schema = schema.clone();
                        let table = table.clone();
                        let ddl = ddl.clone();
                        ui.label(
                            RichText::new(format!("DDL: {schema}.{table}"))
                                .color(BLUE)
                                .size(14.0)
                                .strong(),
                        );
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut ddl.as_str())
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(f32::INFINITY)
                                    .code_editor(),
                            );
                        });
                        self.model.panel = PanelMode::Ddl { schema, table, ddl };
                    }
                    PanelMode::ExplainPlan {
                        schema,
                        table,
                        root,
                    } => {
                        let schema = schema.clone();
                        let table = table.clone();
                        let root = root.clone();
                        ui.label(
                            RichText::new("EXPLAIN Plan")
                                .color(BLUE)
                                .size(14.0)
                                .strong(),
                        );
                        ui.separator();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            render_explain_node_egui(ui, &root, 0);
                        });
                        self.model.panel = PanelMode::ExplainPlan {
                            schema,
                            table,
                            root,
                        };
                    }
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
                            ui.label(RichText::new("/").color(YELLOW).monospace());
                            ui.label(RichText::new("Filter nav tree").color(TEXT));
                            ui.end_row();
                            ui.label(RichText::new("Esc").color(YELLOW).monospace());
                            ui.label(RichText::new("Close filter / Quit").color(TEXT));
                            ui.end_row();
                        });
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.model.show_help = false;
                    }
                });
        }

        // ── Export format picker ──────────────────────────────────────────────
        if self.export_picker {
            const FORMATS: &[(&str, &str)] = &[
                ("CSV", "Comma-separated values (.csv)"),
                ("JSON", "JSON array of objects (.json)"),
                ("NDJSON", "Newline-delimited JSON (.ndjson)"),
                ("TSV", "Tab-separated values (.tsv)"),
            ];
            egui::Window::new("Export Format")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(&ctx, |ui| {
                    ui.label(
                        RichText::new("Select export format (↑↓ Enter Esc):")
                            .color(SUBTEXT0)
                            .small(),
                    );
                    ui.separator();
                    for (i, (fmt, desc)) in FORMATS.iter().enumerate() {
                        let selected = i == self.export_picker_idx;
                        let fmt_text = if selected {
                            RichText::new(format!("▶ {fmt}"))
                                .color(BLUE)
                                .strong()
                                .monospace()
                        } else {
                            RichText::new(format!("  {fmt}")).color(TEXT).monospace()
                        };
                        let desc_text = RichText::new(*desc).color(SUBTEXT0).small();
                        ui.horizontal(|ui| {
                            if ui.selectable_label(selected, fmt_text).clicked() {
                                self.export_picker_idx = i;
                            }
                            ui.label(desc_text);
                        });
                    }
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Export").clicked() {
                            let format = match self.export_picker_idx {
                                0 => ExportFormat::Csv,
                                1 => ExportFormat::Json,
                                2 => ExportFormat::Ndjson,
                                _ => ExportFormat::Tsv,
                            };
                            self.export_picker = false;
                            if let Some(req) = self.model.export_request(format) {
                                let _ = self.req_tx.try_send(req);
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.export_picker = false;
                        }
                    });
                });
        }

        // ── Save-query prompt ─────────────────────────────────────────────────
        if self.save_prompt_active {
            egui::Window::new("Save Query")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(&ctx, |ui| {
                    ui.label(
                        RichText::new(
                            "Enter a name for this query (Enter to save, Esc to cancel):",
                        )
                        .color(SUBTEXT0)
                        .small(),
                    );
                    ui.separator();
                    let response = ui.text_edit_singleline(&mut self.save_prompt_text);
                    response.request_focus();
                    ui.separator();
                    ui.horizontal(|ui| {
                        let can_save = !self.save_prompt_text.trim().is_empty();
                        if ui
                            .add_enabled(can_save, egui::Button::new("Save"))
                            .clicked()
                        {
                            let name = self.save_prompt_text.trim().to_string();
                            if let PanelMode::SqlEditor { text, .. } = &self.model.panel {
                                let sql = text.trim().to_string();
                                if let Some(ref store) = self.saved {
                                    store.save_spawn(name.clone(), sql.clone());
                                }
                                use crate::archive::SavedQuery;
                                let existing =
                                    self.model.saved_cache.iter().position(|q| q.name == name);
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
                                    let ins =
                                        self.model.saved_cache.partition_point(|q| q.name < name);
                                    self.model.saved_cache.insert(ins, new_q);
                                }
                                self.model.flash = Some(format!("saved \"{name}\""));
                            }
                            self.save_prompt_active = false;
                            self.save_prompt_text.clear();
                        }
                        if ui.button("Cancel").clicked() {
                            self.save_prompt_active = false;
                            self.save_prompt_text.clear();
                        }
                    });
                });
        }

        // ── Saved queries browser ─────────────────────────────────────────────
        if self.saved_browser_active {
            egui::Window::new("Saved Queries")
                .collapsible(false)
                .resizable(true)
                .min_width(480.0)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(&ctx, |ui| {
                    ui.label(
                        RichText::new("↑↓ navigate  Enter load  D delete  Esc close")
                            .color(SUBTEXT0)
                            .small(),
                    );
                    ui.separator();
                    if self.model.saved_cache.is_empty() {
                        ui.label(
                            RichText::new("(no saved queries)")
                                .color(SUBTEXT0)
                                .italics(),
                        );
                    } else {
                        egui::ScrollArea::vertical()
                            .max_height(300.0)
                            .show(ui, |ui| {
                                let len = self.model.saved_cache.len();
                                let mut load_idx: Option<usize> = None;
                                let mut del_idx: Option<usize> = None;
                                for (i, q) in self.model.saved_cache.iter().enumerate() {
                                    let selected = i == self.saved_browser_idx;
                                    let preview: String = q
                                        .sql
                                        .lines()
                                        .next()
                                        .unwrap_or("")
                                        .chars()
                                        .take(48)
                                        .collect();
                                    ui.horizontal(|ui| {
                                        let name_text = if selected {
                                            RichText::new(format!("▶ {:.<20}", q.name))
                                                .color(BLUE)
                                                .strong()
                                                .monospace()
                                        } else {
                                            RichText::new(format!("  {:.<20}", q.name))
                                                .color(TEXT)
                                                .monospace()
                                        };
                                        if ui.selectable_label(selected, name_text).clicked() {
                                            self.saved_browser_idx = i;
                                        }
                                        ui.label(RichText::new(&preview).color(SUBTEXT0).small());
                                        if ui
                                            .small_button(RichText::new("load").color(BLUE))
                                            .clicked()
                                        {
                                            load_idx = Some(i);
                                        }
                                        if ui
                                            .small_button(RichText::new("del").color(RED))
                                            .clicked()
                                        {
                                            del_idx = Some(i);
                                        }
                                    });
                                }
                                if let Some(i) = load_idx {
                                    if let Some(q) = self.model.saved_cache.get(i) {
                                        let sql = q.sql.clone();
                                        self.model.panel = PanelMode::SqlEditor {
                                            text: sql,
                                            result: None,
                                            running: false,
                                        };
                                        self.saved_browser_active = false;
                                    }
                                }
                                if let Some(i) = del_idx {
                                    if let Some(q) = self.model.saved_cache.get(i) {
                                        let id = q.id;
                                        let name = q.name.clone();
                                        if let Some(ref store) = self.saved {
                                            store.delete_spawn(id);
                                        }
                                        self.model.saved_cache.remove(i);
                                        if self.saved_browser_idx > 0
                                            && self.saved_browser_idx >= len - 1
                                        {
                                            self.saved_browser_idx -= 1;
                                        }
                                        self.model.flash = Some(format!("deleted \"{name}\""));
                                    }
                                }
                            });
                    }
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.saved_browser_active = false;
                    }
                });
        }
    }

    fn render_column_detail(&self, ui: &mut egui::Ui) {
        match self.model.selected() {
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
                    ui.label(RichText::new(format!("~{rows} rows")).color(SUBTEXT0));
                }
                if !table.columns.is_empty() {
                    ui.add_space(8.0);
                    ui.label(RichText::new("Columns").color(MAUVE).strong());
                    ui.separator();
                    egui::Grid::new("columns")
                        .num_columns(4)
                        .spacing([16.0, 2.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label(RichText::new("Name").color(SUBTEXT0).small());
                            ui.label(RichText::new("Type").color(SUBTEXT0).small());
                            ui.label(RichText::new("Nullable").color(SUBTEXT0).small());
                            ui.label(RichText::new("Flags").color(SUBTEXT0).small());
                            ui.end_row();
                            for col in &table.columns {
                                ui.label(RichText::new(&col.name).color(TEXT));
                                ui.label(
                                    RichText::new(&col.sql_type).color(BLUE).monospace().small(),
                                );
                                ui.label(if col.nullable {
                                    RichText::new("yes").color(OVERLAY0).small()
                                } else {
                                    RichText::new("no").color(YELLOW).small()
                                });
                                let flags: Vec<&str> = [
                                    col.is_primary_key.then_some("PK"),
                                    col.is_foreign_key.then_some("FK"),
                                    col.is_spatial.then_some("spatial"),
                                ]
                                .into_iter()
                                .flatten()
                                .collect();
                                ui.label(RichText::new(flags.join(" ")).color(YELLOW).small());
                                ui.end_row();
                            }
                        });
                }
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Enter to preview rows  |  d for DDL  |  e for EXPLAIN")
                        .color(OVERLAY0)
                        .small(),
                );

                // Column statistics (if already loaded)
                if let Some(col_stats_vec) = self
                    .model
                    .column_stats_for(&table.schema, &table.table_name)
                {
                    if !col_stats_vec.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Column Statistics").color(YELLOW).strong());
                        ui.separator();
                        egui::Grid::new("col_stats_grid")
                            .num_columns(5)
                            .spacing([12.0, 2.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label(RichText::new("Column").color(SUBTEXT0).small());
                                ui.label(RichText::new("Null%").color(SUBTEXT0).small());
                                ui.label(RichText::new("AvgWidth").color(SUBTEXT0).small());
                                ui.label(RichText::new("~Distinct").color(SUBTEXT0).small());
                                ui.label(RichText::new("Correlation").color(SUBTEXT0).small());
                                ui.end_row();
                                for s in col_stats_vec {
                                    ui.label(RichText::new(&s.column_name).color(TEXT).small());
                                    ui.label(
                                        RichText::new(format!("{:.1}%", s.null_fraction * 100.0))
                                            .color(OVERLAY0)
                                            .small(),
                                    );
                                    ui.label(
                                        RichText::new(format!("{}B", s.avg_width_bytes))
                                            .color(OVERLAY0)
                                            .small(),
                                    );
                                    let distinct_label = if s.n_distinct < 0.0 {
                                        format!("{:.0}%", -s.n_distinct * 100.0)
                                    } else if s.n_distinct == 0.0 {
                                        "—".to_string()
                                    } else {
                                        format!("{:.0}", s.n_distinct)
                                    };
                                    ui.label(RichText::new(distinct_label).color(OVERLAY0).small());
                                    let corr_label = s
                                        .correlation
                                        .map(|c| format!("{c:.2}"))
                                        .unwrap_or_else(|| "—".to_string());
                                    ui.label(RichText::new(corr_label).color(OVERLAY0).small());
                                    ui.end_row();
                                }
                            });
                    }
                }

                // Enrichment: FK / constraints / indexes
                if let Some(inspection) = self.model.inspection(&table.schema, &table.table_name) {
                    if !inspection.foreign_keys.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Foreign Keys").color(YELLOW).strong());
                        ui.separator();
                        egui::Grid::new("fk_grid")
                            .num_columns(3)
                            .spacing([12.0, 2.0])
                            .show(ui, |ui| {
                                ui.label(RichText::new("Column").color(SUBTEXT0).small());
                                ui.label(RichText::new("→ Target").color(SUBTEXT0).small());
                                ui.label(RichText::new("Actions").color(SUBTEXT0).small());
                                ui.end_row();
                                for fk in &inspection.foreign_keys {
                                    ui.label(RichText::new(&fk.from_column).color(TEXT));
                                    ui.label(
                                        RichText::new(format!(
                                            "{}.{}.{}",
                                            fk.to_schema, fk.to_table, fk.to_column
                                        ))
                                        .color(BLUE)
                                        .small(),
                                    );
                                    ui.label(
                                        RichText::new(format!(
                                            "del:{} upd:{}",
                                            fk.on_delete, fk.on_update
                                        ))
                                        .color(OVERLAY0)
                                        .small(),
                                    );
                                    ui.end_row();
                                }
                            });
                    }
                    if !inspection.constraints.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Constraints").color(YELLOW).strong());
                        ui.separator();
                        for c in &inspection.constraints {
                            let cols = c.columns.join(", ");
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(format!("{:?}", c.kind)).color(MAUVE).small(),
                                );
                                ui.label(RichText::new(&c.name).color(TEXT).small());
                                if !cols.is_empty() {
                                    ui.label(
                                        RichText::new(format!("({cols})")).color(SUBTEXT0).small(),
                                    );
                                }
                            });
                        }
                    }
                    if !inspection.indexes.is_empty() {
                        ui.add_space(8.0);
                        ui.label(RichText::new("Indexes").color(YELLOW).strong());
                        ui.separator();
                        for idx in &inspection.indexes {
                            let cols = idx.column_names.join(", ");
                            let unique = if idx.is_unique { " UNIQUE" } else { "" };
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&idx.index_name).color(TEXT).small());
                                ui.label(
                                    RichText::new(format!(
                                        "({cols}){unique} [{}]",
                                        idx.index_method
                                    ))
                                    .color(BLUE)
                                    .small(),
                                );
                            });
                        }
                    }
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
        }
    }

    fn render_data_grid(
        &self,
        ui: &mut egui::Ui,
        schema: &str,
        table: &str,
        result: &crate::archive::QueryResult,
        _page: u32,
    ) {
        let title = if schema.is_empty() {
            format!("{table}  ({} rows)", result.row_count)
        } else {
            format!("{schema}.{table}  ({} rows)", result.row_count)
        };
        ui.label(RichText::new(title).color(BLUE).heading());
        ui.separator();

        // Column headers
        egui::Grid::new("data_grid_headers")
            .num_columns(result.columns.len())
            .spacing([8.0, 2.0])
            .show(ui, |ui| {
                for col in &result.columns {
                    ui.label(
                        RichText::new(&col.name)
                            .color(MAUVE)
                            .monospace()
                            .small()
                            .strong(),
                    );
                }
                ui.end_row();
            });

        ui.separator();

        ScrollArea::both().id_salt("data_grid_body").show(ui, |ui| {
            egui::Grid::new("data_grid_rows")
                .num_columns(result.columns.len())
                .spacing([8.0, 1.0])
                .striped(true)
                .show(ui, |ui| {
                    for row in &result.rows.rows {
                        for (ci, _col) in result.columns.iter().enumerate() {
                            let val = row
                                .0
                                .get(ci)
                                .map(|(_, v)| egui_cell_display(v))
                                .unwrap_or_default();
                            let truncated = if val.len() > 40 {
                                format!("{}…", &val[..39])
                            } else {
                                val
                            };
                            ui.label(RichText::new(truncated).color(TEXT).monospace().small());
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

fn egui_cell_display(v: &elicit_db::DbValue) -> String {
    use elicit_db::DbValue;
    match v {
        DbValue::Null => "NULL".to_string(),
        DbValue::Bool(b) => if *b { "true" } else { "false" }.to_string(),
        DbValue::Int(n) => n.to_string(),
        DbValue::Float(f) => format!("{f:.4}"),
        DbValue::Text(s) => s.clone(),
        DbValue::Bytes(b) => format!(
            "\\x{}",
            b.iter()
                .take(8)
                .map(|x| format!("{x:02x}"))
                .collect::<String>()
        ),
        DbValue::Json(j) => j.to_string(),
        DbValue::Geometry(s) | DbValue::Geography(s) => match s {
            elicit_db::DbSpatialValue::Wkt(w) => format!("<geo wkt:{}>", &w[..w.len().min(20)]),
            elicit_db::DbSpatialValue::Wkb(b) => format!("<geo wkb:{} bytes>", b.len()),
        },
    }
}

fn render_explain_node_egui(ui: &mut egui::Ui, node: &crate::archive::ExplainNode, depth: usize) {
    let relation = node
        .relation_name
        .as_deref()
        .map(|r| format!(" on {r}"))
        .unwrap_or_default();
    let header = format!(
        "{}  cost={:.1}..{:.1}  rows={}",
        node.node_type, node.startup_cost, node.total_cost, node.plan_rows,
    );
    let id = egui::Id::new(("explain_node", depth, &node.node_type, &relation));
    egui::CollapsingHeader::new(
        egui::RichText::new(format!("{}{header}{relation}", "  ".repeat(depth)))
            .color(TEXT)
            .monospace(),
    )
    .id_salt(id)
    .default_open(true)
    .show(ui, |ui| {
        if let (Some(at), Some(ar)) = (node.actual_total_time, node.actual_rows) {
            ui.label(
                egui::RichText::new(format!("actual: {at:.2}ms  rows={ar}"))
                    .color(OVERLAY0)
                    .small(),
            );
        }
        for child in &node.children {
            render_explain_node_egui(ui, child, depth + 1);
        }
    });
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
