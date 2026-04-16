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

use crate::archive::frontend_trait::ArchiveFrontend;
use crate::archive::nav_model::{ArchiveNavModel, FetchRequest, PanelEvent, PanelMode};
use crate::archive::nav_tree::{NavTree, build_nav_tree};
use crate::archive::{
    ArchiveDbBackend, ArchiveResult, BackendKind, ConnectionProfile, ConnectionSet, HistoryStore,
    SavedQueryStore,
    errors::{ArchiveError, ArchiveErrorKind},
    plugins::export::export_query_result,
    plugins::query::{execute_sql_direct, preview_table_direct},
};

// ── Thin ratatui wrapper ──────────────────────────────────────────────────────

/// Wraps [`ConnectionSet`] (wrapping [`ArchiveNavModel`]) with the ratatui bridge backend.
///
/// Rendering uses the IR pipeline: `model.to_verified_tree()` →
/// `backend.render_from_ir()` → `render_node()`.  Event handling remains
/// in this module; only presentation logic has moved into the IR.
///
/// `model` is a [`ConnectionSet`] that derefs transparently to the active
/// [`ArchiveNavModel`], so all existing `self.model.*` call sites remain
/// unchanged while gaining multi-connection switching via `conn_*` methods.
struct TuiApp {
    model: ConnectionSet,
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
        connections: ConnectionSet,
        req_tx: mpsc::Sender<FetchRequest>,
        history: Option<HistoryStore>,
        saved: Option<SavedQueryStore>,
    ) -> Self {
        Self {
            model: connections,
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
                    grid_row: 0,
                    grid_col: 0,
                    edit_state: None,
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
            PanelEvent::MonitorReady(snapshot) => {
                self.model.apply_monitor_snapshot(snapshot);
            }
            PanelEvent::AdminReady(snapshot) => {
                self.model.apply_admin_snapshot(snapshot);
            }
            PanelEvent::ErdReady(diagram) => {
                self.model.apply_erd_diagram(diagram);
            }
            PanelEvent::ConstraintsReady {
                schema,
                table,
                constraints,
            } => {
                self.model.apply_constraints(schema, table, constraints);
            }
            PanelEvent::IndexesReady {
                schema,
                table,
                indexes,
            } => {
                self.model.apply_indexes(schema, table, indexes);
            }
        }
    }
}

// ── ArchiveFrontend implementation ───────────────────────────────────────────

impl crate::archive::ArchiveFrontend for TuiApp {
    /// Dispatch a named action.  Every [`ArchiveAction`] variant must be
    /// handled here — the compiler enforces exhaustiveness.
    ///
    /// Returns `true` when the application should quit.
    fn dispatch_action(&mut self, action: crate::archive::ArchiveAction) -> bool {
        use crate::archive::ArchiveAction as A;
        use crate::archive::nav_model::PanelMode;
        match action {
            A::MoveUp => self.move_up(),
            A::MoveDown => self.move_down(),
            A::Select => self.toggle_expand(),
            A::Refresh => self.refresh(),
            A::ToggleHelp => self.toggle_help(),
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
            A::OpenErd => {
                if let Some(req) = self.model.toggle_erd_panel() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::OpenConstraints => {
                if let Some(req) = self.model.toggle_constraint_panel() {
                    let _ = self.req_tx.try_send(req);
                }
            }
            A::OpenIndexes => {
                if let Some(req) = self.model.toggle_index_panel() {
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
            A::RequestDdl => self.request_ddl(),
            A::RequestExplain => self.request_explain(),
            A::PageNext => self.model.page_next(),
            A::PagePrev => self.model.page_prev(),
            A::PageFirst => self.model.page_first(),
            A::PageLast => self.model.page_last(),
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
            A::ExportPickerConfirm => self.confirm_export(),
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

/// Spawns a tokio task that handles [`FetchRequest`]s and sends back
/// [`PanelEvent`]s.  Returns `(req_tx, event_rx)`.
fn spawn_fetch_task(
    url: Option<String>,
) -> (mpsc::Sender<FetchRequest>, mpsc::Receiver<PanelEvent>) {
    let (req_tx, mut req_rx) = mpsc::channel::<FetchRequest>(16);
    let (event_tx, event_rx) = mpsc::channel::<PanelEvent>(16);

    tokio::spawn(async move {
        let mut current_url = url;
        while let Some(req) = req_rx.recv().await {
            // Handle URL switch before any DB access.
            if let FetchRequest::UpdateUrl(new_url) = req {
                current_url = Some(new_url);
                // Trigger a refresh on the new connection.
                if let Some(ref url) = current_url {
                    match ArchiveDbBackend::connect(url).await {
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
                    }
                }
                continue;
            }

            let Some(ref url) = current_url else {
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
                // Already handled above before the URL check.
                FetchRequest::UpdateUrl(_) => unreachable!("UpdateUrl handled before this match"),
                FetchRequest::FetchMonitor { schema } => {
                    use crate::archive::MonitorSnapshot;
                    use elicit_db::{DbMonitor, DbRoleManager};
                    let schema = schema.clone();
                    let ev = match ArchiveDbBackend::connect(url).await {
                        Ok(backend) => {
                            let sessions = backend
                                .active_sessions()
                                .await
                                .map(|a| a.sessions)
                                .unwrap_or_default();
                            let roles = backend.list_roles().await.unwrap_or_default();
                            let cache_hit = backend.cache_hit_ratio().await.ok();
                            let slow_queries =
                                backend.slow_queries(1_000).await.unwrap_or_default();
                            let lock_waits = backend.lock_waits().await.unwrap_or_default();
                            let table_bloat =
                                backend.table_bloat(&schema).await.unwrap_or_default();
                            let index_usage =
                                backend.index_usage(&schema).await.unwrap_or_default();
                            PanelEvent::MonitorReady(MonitorSnapshot {
                                sessions,
                                roles,
                                cache_hit,
                                backups: Vec::new(),
                                slow_queries,
                                lock_waits,
                                table_bloat,
                                index_usage,
                                active_tab: crate::archive::MonitorTab::Sessions,
                            })
                        }
                        Err(e) => PanelEvent::FetchError(e.to_string()),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::FetchAdmin => {
                    use crate::archive::{AdminSnapshot, AdminTab};
                    use elicit_db::{DbBackupManager, DbRoleManager, DbServerAdmin};
                    let ev = match ArchiveDbBackend::connect(url).await {
                        Ok(backend) => {
                            let roles = backend.list_roles().await.unwrap_or_default();
                            let backups = backend.list_backups().await.unwrap_or_default();
                            let wal_ready = backend.wal_status().await.is_ok();
                            let server_version = backend.server_version().await.unwrap_or_default();
                            let extensions = backend.list_extensions().await.unwrap_or_default();
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
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::FetchErd { schema } => {
                    use crate::archive::nav_tree::fetch_erd;
                    let ev = match fetch_erd(
                        &ArchiveDbBackend::connect(url).await.unwrap(),
                        url,
                        &schema,
                    )
                    .await
                    {
                        Ok(diagram) => PanelEvent::ErdReady(diagram),
                        Err(e) => PanelEvent::FetchError(e.to_string()),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::FetchConstraints { schema, table } => {
                    use crate::archive::plugins::inspect::inspect_table_direct;
                    let ev = match inspect_table_direct(url, &schema, &table).await {
                        Ok(insp) => PanelEvent::ConstraintsReady {
                            schema,
                            table,
                            constraints: insp.constraints,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
                FetchRequest::FetchIndexes { schema, table } => {
                    use crate::archive::plugins::inspect::inspect_table_direct;
                    let ev = match inspect_table_direct(url, &schema, &table).await {
                        Ok(insp) => PanelEvent::IndexesReady {
                            schema,
                            table,
                            indexes: insp.indexes,
                        },
                        Err(e) => PanelEvent::FetchError(e),
                    };
                    let _ = event_tx.send(ev).await;
                }
            }
        }
    });

    (req_tx, event_rx)
}

// ── Event loop ────────────────────────────────────────────────────────────────

/// Convert a crossterm [`KeyEvent`] to an [`ArchiveKeyCombo`] if it is a
/// recognisable key press that the key map can resolve.
fn crossterm_key_to_combo(key: &crossterm::event::KeyEvent) -> Option<crate::archive::KeyCombo> {
    use crate::archive::{ArchiveKey, KeyCombo};
    use crossterm::event::{KeyCode, KeyModifiers};
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
    let shift = key.modifiers.contains(KeyModifiers::SHIFT);
    let ak = match key.code {
        KeyCode::Up => ArchiveKey::Up,
        KeyCode::Down => ArchiveKey::Down,
        KeyCode::Enter => ArchiveKey::Enter,
        KeyCode::Esc => ArchiveKey::Esc,
        KeyCode::Backspace => ArchiveKey::Backspace,
        KeyCode::Delete => ArchiveKey::Delete,
        KeyCode::Tab => ArchiveKey::Tab,
        KeyCode::BackTab => ArchiveKey::BackTab,
        KeyCode::PageDown => ArchiveKey::PageDown,
        KeyCode::PageUp => ArchiveKey::PageUp,
        KeyCode::Home => ArchiveKey::Home,
        KeyCode::End => ArchiveKey::End,
        KeyCode::F(n) => ArchiveKey::F(n),
        KeyCode::Char(c) => ArchiveKey::Char(c),
        _ => return None,
    };
    Some(KeyCombo {
        key: ak,
        ctrl,
        shift,
        alt: false,
    })
}

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

    let (req_tx, mut event_rx) = spawn_fetch_task(url.clone());

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
    let profile = ConnectionProfile {
        name: "primary".to_string(),
        url_env_key: url.clone().unwrap_or_default(),
        backend: BackendKind::Postgres,
        color: None,
    };
    let connections = ConnectionSet::from_single(profile, ArchiveNavModel::new(nav), url);
    let mut app = TuiApp::new(connections, req_tx, history, saved);
    app.model.history_cache = history_cache;
    app.model.saved_cache = saved_cache;
    let mut reader = EventStream::new();
    let mut quit = false;

    let result: ArchiveResult<()> = async {
        loop {
            // IR pipeline: mint verified tree → bridge render → draw frame.
            let (tree, _ir_proof) = app
                .model
                .to_verified_tree()
                .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;
            let (tui_node, _stats, _render_proof) = app
                .backend
                .render(&tree)
                .map_err(|e| ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string())))?;
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
                        let keymap = crate::archive::ArchiveKeyMap::default_map();
                        let mode = app.model.current_mode();
                        if let Some(combo) = crossterm_key_to_combo(&key) {
                            if let Some(action) = keymap.resolve(&combo, mode) {
                                if app.dispatch_action(action) {
                                    quit = true;
                                }
                                continue;
                            }
                        }
                        // Text input for modes that accept printable characters
                        if let KeyCode::Char(c) = key.code {
                            if !key.modifiers.contains(KeyModifiers::CONTROL)
                                && !key.modifiers.contains(KeyModifiers::ALT)
                            {
                                let mut buf = [0u8; 4];
                                app.dispatch_text(c.encode_utf8(&mut buf));
                            }
                        }
                        if let KeyCode::Enter = key.code {
                            // Bare Enter in text modes: newline
                            if !key.modifiers.contains(KeyModifiers::CONTROL) {
                                app.dispatch_text("\n");
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
