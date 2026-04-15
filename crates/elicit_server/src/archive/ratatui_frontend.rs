//! Ratatui terminal frontend for the archive CLI.
//!
//! Provides an interactive pgAdmin-style database browser with keyboard
//! navigation: `↑`/`↓` move selection, `Enter` expands/collapses schemas or
//! opens a data grid for a table, `r` refreshes the tree, `?` toggles help,
//! `/` opens a search/filter bar, `q`/`Esc` quits.
//!
//! Key bindings are sourced from [`ArchiveNavModel::bindings`] (the
//! AccessKit IR), keeping all frontends consistent.
//!
//! ## Layout
//!
//! ```text
//! ┌──────────────── header (3 rows) ───────────────────────────────────┐
//! │ nav panel (30%) │ content panel (70%)                              │
//! │   schema tree   │   column detail / data grid / loading            │
//! ├─────────────────┴──────────────────────────────────────────────────┤
//! │ status bar (1 row)                                                 │
//! └────────────────────────────────────────────────────────────────────┘
//! ```

use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use elicit_ratatui::{RatatuiBackend, render_node};
use elicit_ui::UiTreeRenderer as _;
use futures::StreamExt as _;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;
use tracing::instrument;

use crate::archive::nav_model::{ArchiveNavModel, FetchRequest, PanelEvent, PanelMode};
use crate::archive::nav_tree::{NavTree, build_nav_tree};
use crate::archive::{
    ArchiveDbBackend, ArchiveResult, HistoryStore, SavedQueryStore,
    errors::{ArchiveError, ArchiveErrorKind},
    plugins::export::export_query_result,
    plugins::query::{execute_sql_direct, preview_table_direct},
};

// ── Thin ratatui wrapper ──────────────────────────────────────────────────────

/// Wraps [`ArchiveNavModel`] with the ratatui bridge backend.
///
/// Rendering uses the IR pipeline: `model.to_verified_tree()` →
/// `backend.render_from_ir()` → `render_node()`.  Event handling remains
/// in this module; only presentation logic has moved into the IR.
struct TuiApp {
    model: ArchiveNavModel,
    backend: RatatuiBackend,
    /// Sender to the background fetch task.
    req_tx: mpsc::Sender<FetchRequest>,
    /// Persistent query history (None if unavailable).
    history: Option<HistoryStore>,
    /// Saved-query store (None if unavailable).
    saved: Option<SavedQueryStore>,
    /// Timestamp of the last SQL execution start (for duration tracking).
    exec_start: Option<std::time::Instant>,
    /// SQL text of the last `ExecuteSql` dispatch (for history recording).
    pending_sql: Option<String>,
}

impl TuiApp {
    fn new(
        nav: NavTree,
        req_tx: mpsc::Sender<FetchRequest>,
        history: Option<HistoryStore>,
        saved: Option<SavedQueryStore>,
    ) -> Self {
        let model = ArchiveNavModel::new(nav);
        Self {
            model,
            backend: RatatuiBackend::new(),
            req_tx,
            history,
            saved,
            exec_start: None,
            pending_sql: None,
        }
    }

    fn move_up(&mut self) {
        self.model.move_up();
        self.model.flash = None;
        if let Some(req) = self.model.inspect_request() {
            let _ = self.req_tx.try_send(req);
        }
        if let Some(req) = self.model.stats_request() {
            let _ = self.req_tx.try_send(req);
        }
    }

    fn move_down(&mut self) {
        self.model.move_down();
        self.model.flash = None;
        if let Some(req) = self.model.inspect_request() {
            let _ = self.req_tx.try_send(req);
        }
        if let Some(req) = self.model.stats_request() {
            let _ = self.req_tx.try_send(req);
        }
    }

    /// Toggle expand/select.  If a table is selected, dispatches a fetch request.
    fn toggle_expand(&mut self) {
        if let Some(req) = self.model.toggle_expand() {
            let _ = self.req_tx.try_send(req);
        }
    }

    fn refresh(&mut self) {
        let req = self.model.request_refresh();
        let _ = self.req_tx.try_send(req);
    }

    fn toggle_help(&mut self) {
        self.model.toggle_help();
    }

    fn request_ddl(&mut self) {
        if let Some(req) = self.model.ddl_request() {
            let _ = self.req_tx.try_send(req);
        }
    }

    fn request_explain(&mut self) {
        if let Some(req) = self.model.explain_request() {
            let _ = self.req_tx.try_send(req);
        }
    }

    fn confirm_export(&mut self) {
        let format = self.model.confirm_export_picker();
        if let Some(req) = self.model.export_request(format) {
            let _ = self.req_tx.try_send(req);
        }
    }

    /// Execute the current SQL editor content.
    fn run_sql(&mut self) {
        if let PanelMode::SqlEditor { text, running, .. } = &mut self.model.panel {
            if *running || text.trim().is_empty() {
                return;
            }
            let sql = text.trim().to_string();
            *running = true;
            self.exec_start = Some(std::time::Instant::now());
            self.pending_sql = Some(sql.clone());
            // Reset history navigation on new execution
            self.model.history_idx = None;
            let _ = self.req_tx.try_send(FetchRequest::ExecuteSql { sql });
        }
    }

    fn apply_panel_event(&mut self, event: PanelEvent) {
        match event {
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
            }
            PanelEvent::NavRefreshed(nav) => {
                self.model.apply_refresh(nav);
            }
            PanelEvent::FetchError(e) => {
                // If this error is from a SQL execution, record it in history.
                if let Some(sql) = self.pending_sql.take() {
                    let duration_ms = self
                        .exec_start
                        .take()
                        .map(|s| s.elapsed().as_millis() as u64)
                        .unwrap_or(0);
                    if let Some(ref store) = self.history {
                        store.append_spawn(sql, duration_ms, None, Some(e.to_string()));
                    }
                    // Reset running flag in SqlEditor panel
                    if let PanelMode::SqlEditor { running, .. } = &mut self.model.panel {
                        *running = false;
                    }
                } else {
                    self.model.panel = PanelMode::ColumnDetail;
                }
                self.model.flash = Some(format!("error: {e}"));
            }
            PanelEvent::SqlResult(result) => {
                let duration_ms = self
                    .exec_start
                    .take()
                    .map(|s| s.elapsed().as_millis() as u64)
                    .unwrap_or(0);
                let row_count = result.row_count;
                let current_text = match &self.model.panel {
                    PanelMode::SqlEditor { text, .. } => text.clone(),
                    _ => String::new(),
                };
                if let Some(sql) = self.pending_sql.take() {
                    let new_entry = crate::archive::QueryHistoryEntry {
                        id: 0,
                        executed_at: chrono::Utc::now(),
                        sql: sql.clone(),
                        duration_ms,
                        row_count: Some(row_count),
                        error: None,
                    };
                    self.model.history_cache.insert(0, new_entry);
                    if let Some(ref store) = self.history {
                        store.append_spawn(sql, duration_ms, Some(row_count), None);
                    }
                }
                self.model.panel = PanelMode::SqlEditor {
                    text: current_text,
                    result: Some(result),
                    running: false,
                    error: None,
                };
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

// ── Background fetch task ─────────────────────────────────────────────────────

/// Spawns a tokio task that handles [`FetchRequest`]s and sends back
/// [`PanelEvent`]s.  Returns `(req_tx, event_rx)`.
fn spawn_fetch_task(
    url: Option<String>,
) -> (mpsc::Sender<FetchRequest>, mpsc::Receiver<PanelEvent>) {
    let (req_tx, mut req_rx) = mpsc::channel::<FetchRequest>(16);
    let (event_tx, event_rx) = mpsc::channel::<PanelEvent>(16);

    tokio::spawn(async move {
        while let Some(req) = req_rx.recv().await {
            let Some(ref url) = url else {
                // Demo mode — no live DB.
                let _ = event_tx
                    .send(PanelEvent::FetchError(
                        "No database URL — run 'archive serve' instead of 'archive demo'."
                            .to_string(),
                    ))
                    .await;
                continue;
            };
            match req {
                FetchRequest::PreviewTable { schema, table } => {
                    let result = preview_table_direct(url, &schema, &table, 200).await;
                    let ev = match result {
                        Ok(r) => PanelEvent::DataGrid {
                            schema,
                            table,
                            result: r,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::Refresh => match ArchiveDbBackend::connect(url).await {
                    Ok(backend) => match build_nav_tree(&backend, url).await {
                        Ok(nav) => {
                            let _ = event_tx.send(PanelEvent::NavRefreshed(nav)).await;
                        }
                        Err(e) => {
                            let _ = event_tx.send(PanelEvent::FetchError(e.to_string())).await;
                        }
                    },
                    Err(e) => {
                        let _ = event_tx.send(PanelEvent::FetchError(e.to_string())).await;
                    }
                },
                FetchRequest::ExecuteSql { sql } => {
                    let result = execute_sql_direct(url, &sql).await;
                    let ev = match result {
                        Ok(r) => PanelEvent::SqlResult(r),
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::InspectTable { schema, table } => {
                    use crate::archive::plugins::inspect::inspect_table_direct;
                    let ev = match inspect_table_direct(url, &schema, &table).await {
                        Ok(inspection) => PanelEvent::TableInspected {
                            schema,
                            table,
                            inspection,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::GetDdl { schema, table } => {
                    use crate::archive::plugins::inspect::generate_ddl_direct;
                    let ev = match generate_ddl_direct(url, &schema, &table).await {
                        Ok(ddl) => PanelEvent::DdlReady {
                            schema,
                            table,
                            ddl: ddl.ddl,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::GetColumnStats { schema, table } => {
                    use crate::archive::plugins::inspect::get_column_stats_direct;
                    let ev = match get_column_stats_direct(url, &schema, &table).await {
                        Ok(stats) => PanelEvent::ColumnStatsReady {
                            schema,
                            table,
                            stats,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::ExplainSql { schema, table, sql } => {
                    use crate::archive::plugins::inspect::explain_sql_direct;
                    let ev = match explain_sql_direct(url, &sql).await {
                        Ok(root) => PanelEvent::ExplainReady {
                            schema,
                            table,
                            root,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::ExportData {
                    schema,
                    table,
                    result,
                    format,
                } => {
                    let export = export_query_result(&result, format.clone());
                    let ev = PanelEvent::ExportReady {
                        schema,
                        table,
                        content: export.content,
                        row_count: export.row_count,
                        format,
                    };
                    let _ = event_tx.send(ev).await;
                }
            }
        }
    });

    (req_tx, event_rx)
}

// ── Event loop ────────────────────────────────────────────────────────────────

/// Run the interactive archive browser in a crossterm terminal.
///
/// Accepts an optional `url` for live data fetching; pass `None` for demo mode.
#[instrument(skip(nav))]
pub async fn run_tui(nav: NavTree, url: Option<String>) -> ArchiveResult<()> {
    enable_raw_mode().map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    let backend_term = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend_term).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    let (req_tx, mut event_rx) = spawn_fetch_task(url);

    let history = HistoryStore::open().await.ok();
    let history_cache = if let Some(ref store) = history {
        store
            .recent(crate::archive::plugins::history::MAX_HISTORY)
            .await
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let saved = SavedQueryStore::open().await.ok();
    let saved_cache = if let Some(ref store) = saved {
        store.all().await.unwrap_or_default()
    } else {
        Vec::new()
    };
    let mut app = TuiApp::new(nav, req_tx, history, saved);
    app.model.history_cache = history_cache;
    app.model.saved_cache = saved_cache;
    let mut reader = EventStream::new();
    let mut quit = false;

    let result: ArchiveResult<()> = async {
        loop {
            // IR pipeline: mint verified tree → bridge render → draw frame.
            let (tree, _ir_proof) = app.model.to_verified_tree().map_err(|e| {
                ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
            })?;
            let (tui_node, _stats, _render_proof) =
                app.backend.render(&tree).map_err(|e| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })?;
            terminal
                .draw(|frame| {
                    render_node(frame, frame.area(), &tui_node);
                })
                .map_err(|e: std::io::Error| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })?;

            if quit {
                break;
            }

            tokio::select! {
                Some(ev) = event_rx.recv() => {
                    app.apply_panel_event(ev);
                }
                Some(Ok(event)) = reader.next() => {
                    if let Event::Key(key) = event {
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }
                        if app.model.filter_active {
                            match key.code {
                                KeyCode::Esc => app.model.close_filter(),
                                KeyCode::Backspace => app.model.filter_backspace(),
                                KeyCode::Char(c) => app.model.filter_push(c),
                                _ => {}
                            }
                        } else if app.model.save_prompt_active {
                            match key.code {
                                KeyCode::Esc => app.model.close_save_prompt(),
                                KeyCode::Backspace => app.model.save_prompt_backspace(),
                                KeyCode::Enter => {
                                    if let Some(name) = app.model.take_save_prompt() {
                                        if let PanelMode::SqlEditor { text, .. } = &app.model.panel {
                                            let sql = text.trim().to_string();
                                            if let Some(ref store) = app.saved {
                                                store.save_spawn(name.clone(), sql.clone());
                                            }
                                            use crate::archive::SavedQuery;
                                            let existing = app.model.saved_cache
                                                .iter()
                                                .position(|q| q.name == name);
                                            let now = chrono::Utc::now();
                                            if let Some(idx) = existing {
                                                app.model.saved_cache[idx].sql = sql;
                                                app.model.saved_cache[idx].updated_at = now;
                                            } else {
                                                let new_q = SavedQuery {
                                                    id: 0,
                                                    name: name.clone(),
                                                    sql,
                                                    created_at: now,
                                                    updated_at: now,
                                                };
                                                let ins = app.model.saved_cache
                                                    .partition_point(|q| q.name < name);
                                                app.model.saved_cache.insert(ins, new_q);
                                            }
                                            app.model.flash = Some(format!("saved \"{name}\""));
                                        }
                                    }
                                }
                                KeyCode::Char(c) => app.model.save_prompt_push(c),
                                _ => {}
                            }
                        } else if app.model.saved_browser_active {
                            let len = app.model.saved_cache.len();
                            match key.code {
                                KeyCode::Esc | KeyCode::Char('q') => {
                                    app.model.toggle_saved_browser();
                                }
                                KeyCode::Up | KeyCode::Char('k') => {
                                    app.model.saved_browser_prev();
                                }
                                KeyCode::Down | KeyCode::Char('j') => {
                                    if len > 0 {
                                        app.model.saved_browser_next();
                                    }
                                }
                                KeyCode::Enter => {
                                    let idx = app.model.saved_browser_idx;
                                    if let Some(q) = app.model.saved_cache.get(idx) {
                                        let sql = q.sql.clone();
                                        app.model.panel = PanelMode::SqlEditor {
                                            text: sql,
                                            result: None,
                                            running: false,
                                            error: None,
                                        };
                                        app.model.toggle_saved_browser();
                                    }
                                }
                                KeyCode::Char('d') | KeyCode::Delete => {
                                    let idx = app.model.saved_browser_idx;
                                    if let Some(q) = app.model.saved_cache.get(idx) {
                                        let id = q.id;
                                        let name = q.name.clone();
                                        if let Some(ref store) = app.saved {
                                            store.delete_spawn(id);
                                        }
                                        app.model.saved_cache.remove(idx);
                                        if idx > 0 && idx >= app.model.saved_cache.len() {
                                            app.model.saved_browser_prev();
                                        }
                                        app.model.flash = Some(format!("deleted \"{name}\""));
                                    }
                                }
                                _ => {}
                            }
                        } else if app.model.export_picker {
                            match key.code {
                                KeyCode::Esc => app.model.toggle_export_picker(),
                                KeyCode::Up | KeyCode::Char('k') => {
                                    app.model.export_picker_prev();
                                }
                                KeyCode::Down | KeyCode::Char('j') => {
                                    app.model.export_picker_next();
                                }
                                KeyCode::Enter => app.confirm_export(),
                                _ => {}
                            }
                        } else if matches!(app.model.panel, PanelMode::SqlEditor { .. }) {
                            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                            match (key.code, ctrl) {
                                (KeyCode::Enter, true) | (KeyCode::F(5), _) => app.run_sql(),
                                (KeyCode::Char('s'), true) => app.model.open_save_prompt(),
                                (KeyCode::Up, true) => { app.model.history_prev(); }
                                (KeyCode::Down, true) => { app.model.history_next(); }
                                (KeyCode::Esc, _) => {
                                    app.model.panel = PanelMode::ColumnDetail;
                                }
                                (KeyCode::Char(c), false) => {
                                    if let PanelMode::SqlEditor { text, .. } = &mut app.model.panel {
                                        text.push(c);
                                    }
                                }
                                (KeyCode::Backspace, _) => {
                                    if let PanelMode::SqlEditor { text, .. } = &mut app.model.panel {
                                        text.pop();
                                    }
                                }
                                (KeyCode::Enter, false) => {
                                    if let PanelMode::SqlEditor { text, .. } = &mut app.model.panel {
                                        text.push('\n');
                                    }
                                }
                                _ => {}
                            }
                        } else {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Esc => quit = true,
                                KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                                KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                                KeyCode::Enter => app.toggle_expand(),
                                KeyCode::Char('r') => app.refresh(),
                                KeyCode::Char('?') => app.toggle_help(),
                                KeyCode::Char('/') => app.model.open_filter(),
                                KeyCode::Char('d') => app.request_ddl(),
                                KeyCode::Char('e') => app.request_explain(),
                                KeyCode::Char('s') => {
                                    app.model.panel = PanelMode::SqlEditor {
                                        text: String::new(),
                                        result: None,
                                        running: false,
                                        error: None,
                                    };
                                }
                                KeyCode::F(2) => app.model.toggle_saved_browser(),
                                KeyCode::Char('x') => {
                                    if app.model.panel.is_data_grid() {
                                        app.model.toggle_export_picker();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                else => break,
            }
        }
        Ok(())
    }
    .await;

    disable_raw_mode().map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    result
}
