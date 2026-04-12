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
use elicit_db::DbValue;
use elicit_ratatui::render_node;
use elicit_ui::ColorTheme;
use futures::StreamExt as _;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, Borders, Cell, Clear, List, ListItem, ListState, Paragraph, Row, Table, TableState,
};
use ratatui::{Frame, Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;
use tracing::instrument;

use crate::archive::nav_model::{
    ArchiveNavModel, FetchRequest, FlatItem, PanelEvent, PanelMode, column_widths,
};
use crate::archive::nav_tree::{NavTree, build_nav_tree};
use crate::archive::{
    ArchiveDbBackend, ArchiveResult, ExportFormat, HistoryStore, QueryResult, TableType,
    errors::{ArchiveError, ArchiveErrorKind},
    plugins::export::export_query_result,
    plugins::query::{execute_sql_direct, preview_table_direct},
};

// ── Thin ratatui wrapper ──────────────────────────────────────────────────────

/// Wraps [`ArchiveNavModel`] with ratatui-specific stateful widgets.
struct TuiApp {
    model: ArchiveNavModel,
    list_state: ListState,
    table_state: TableState,
    /// Sender to the background fetch task.
    req_tx: mpsc::Sender<FetchRequest>,
    /// When `Some`, show the export format picker overlay.
    export_picker: bool,
    /// Currently highlighted option in the export picker (0–3).
    export_picker_idx: usize,
    /// Persistent query history (None if unavailable).
    history: Option<HistoryStore>,
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
    ) -> Self {
        let model = ArchiveNavModel::new(nav);
        let mut list_state = ListState::default();
        if !model.flat.is_empty() {
            list_state.select(Some(model.cursor));
        }
        Self {
            model,
            list_state,
            table_state: TableState::default(),
            req_tx,
            export_picker: false,
            export_picker_idx: 0,
            history,
            exec_start: None,
            pending_sql: None,
        }
    }

    fn sync_list_state(&mut self) {
        if self.model.flat.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select(Some(self.model.cursor));
        }
    }

    fn move_up(&mut self) {
        self.model.move_up();
        self.model.flash = None;
        self.sync_list_state();
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
        self.sync_list_state();
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
        self.sync_list_state();
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
                self.table_state = TableState::default().with_selected(Some(0));
            }
            PanelEvent::NavRefreshed(nav) => {
                self.model.apply_refresh(nav);
                self.sync_list_state();
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
                };
                self.table_state = TableState::default().with_selected(Some(0));
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

// ── Drawing ───────────────────────────────────────────────────────────────────

fn draw_app(frame: &mut Frame, app: &mut TuiApp) {
    let area = frame.area();

    // Outer: header | body | status
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    draw_header(frame, outer[0], &app.model);

    // Body: nav (30%) | content (70%)
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(outer[1]);

    draw_nav(frame, body[0], app);
    draw_content(frame, body[1], app);
    draw_status_bar(frame, outer[2]);

    if app.model.show_help {
        draw_help(frame, area);
    }
    if app.export_picker {
        draw_export_picker(frame, area, app.export_picker_idx);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, model: &ArchiveNavModel) {
    let ver = model.version.as_deref().unwrap_or("unknown");
    let flash = model.flash.as_deref().unwrap_or("");
    let filter_hint = if model.filter_active {
        format!("  /filter: {}_", model.filter)
    } else {
        String::new()
    };
    let title_line = Line::from(vec![
        Span::styled(
            format!(" {} ", model.backend_label),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{ver} "), Style::default().fg(Color::White)),
        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} ", model.db_name),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(filter_hint, Style::default().fg(Color::Green)),
    ]);
    let flash_line = if flash.is_empty() {
        Line::default()
    } else {
        Line::from(Span::styled(
            format!(" {flash}"),
            Style::default().fg(Color::Green),
        ))
    };
    let header = Paragraph::new(vec![title_line, flash_line])
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title(" Archive ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black));
    frame.render_widget(header, area);
}

fn draw_nav(frame: &mut Frame, area: Rect, app: &mut TuiApp) {
    let items: Vec<ListItem> = app
        .model
        .flat
        .iter()
        .map(|fi| match fi {
            FlatItem::Schema(i) => {
                let s = &app.model.schemas[*i];
                let arrow = if s.expanded { "▼" } else { "▶" };
                let table_count = s.entry.tables.len();
                let count_label = if table_count == 0 {
                    "empty".to_string()
                } else if table_count == 1 {
                    "1 table".to_string()
                } else {
                    format!("{table_count} tables")
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {arrow} "), Style::default().fg(Color::Cyan)),
                    Span::styled(
                        s.entry.name.clone(),
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("  {count_label}"),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]))
            }
            FlatItem::Table(si, ti) => {
                let s = &app.model.schemas[*si];
                let t = &s.entry.tables[*ti];
                let is_last = *ti + 1 == s.entry.tables.len();
                let prefix = if is_last { "   └─" } else { "   ├─" };
                let (type_label, type_color) = match t.table_type {
                    TableType::Table => ("TABLE", Color::Blue),
                    TableType::View => ("VIEW ", Color::Magenta),
                    TableType::MaterializedView => ("MATV ", Color::Yellow),
                    TableType::Unknown => ("?    ", Color::DarkGray),
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!("{prefix} "), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{type_label} "), Style::default().fg(type_color)),
                    Span::styled(t.table_name.clone(), Style::default().fg(Color::Gray)),
                ]))
            }
        })
        .collect();

    let block = Block::default()
        .borders(Borders::RIGHT)
        .title(" Navigator ")
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▌");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_content(frame: &mut Frame, area: Rect, app: &mut TuiApp) {
    match &app.model.panel {
        PanelMode::ColumnDetail => draw_column_detail(frame, area, &app.model),
        PanelMode::Loading { schema, table } => draw_loading(frame, area, schema, table),
        PanelMode::DataGrid {
            schema,
            table,
            result,
            ..
        } => {
            let result = result.clone();
            let schema = schema.clone();
            let table = table.clone();
            draw_data_grid(frame, area, &schema, &table, &result, &mut app.table_state);
        }
        PanelMode::SqlEditor {
            text,
            result,
            running,
        } => {
            let text = text.clone();
            let result = result.clone();
            let running = *running;
            draw_sql_editor(frame, area, &text, result.as_ref(), running);
        }
        PanelMode::Ddl { schema, table, ddl } => {
            let schema = schema.clone();
            let table = table.clone();
            let ddl = ddl.clone();
            draw_ddl_panel(frame, area, &schema, &table, &ddl);
        }
        PanelMode::ExplainPlan {
            schema,
            table,
            root,
        } => {
            let schema = schema.clone();
            let table = table.clone();
            let root = root.clone();
            draw_explain_panel(frame, area, &schema, &table, &root);
        }
    }
}

fn draw_column_detail(frame: &mut Frame, area: Rect, model: &ArchiveNavModel) {
    let content = match model.selected() {
        Some(FlatItem::Schema(si)) => {
            let s = &model.schemas[si];
            let mut lines = vec![
                Line::from(vec![
                    Span::styled(" schema: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        s.entry.name.clone(),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::styled(" owner:  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(s.entry.owner.clone(), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled(" tables: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        s.entry.tables.len().to_string(),
                        Style::default().fg(Color::White),
                    ),
                ]),
                Line::default(),
                Line::from(Span::styled(
                    " Press Enter to expand / collapse.",
                    Style::default().fg(Color::DarkGray),
                )),
            ];
            for t in &s.entry.tables {
                lines.push(Line::from(vec![
                    Span::styled("   • ", Style::default().fg(Color::DarkGray)),
                    Span::styled(t.table_name.clone(), Style::default().fg(Color::Gray)),
                ]));
            }
            lines
        }
        Some(FlatItem::Table(si, ti)) => {
            let s = &model.schemas[si];
            let t = &s.entry.tables[ti];
            let mut lines = vec![
                Line::from(vec![Span::styled(
                    format!(" {}.{}", t.schema, t.table_name),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(vec![
                    Span::styled(" type:   ", Style::default().fg(Color::DarkGray)),
                    Span::styled(t.table_type.to_string(), Style::default().fg(Color::White)),
                ]),
            ];
            if let Some(rows) = t.estimated_rows {
                lines.push(Line::from(vec![
                    Span::styled(" ~rows:  ", Style::default().fg(Color::DarkGray)),
                    Span::styled(rows.to_string(), Style::default().fg(Color::White)),
                ]));
            }
            lines.push(Line::default());
            lines.push(Line::from(Span::styled(
                " Columns:",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
            for col in &t.columns {
                let flags: Vec<&str> = [
                    col.is_primary_key.then_some("PK"),
                    col.is_foreign_key.then_some("FK"),
                    col.is_spatial.then_some("spatial"),
                    (!col.nullable).then_some("NOT NULL"),
                ]
                .into_iter()
                .flatten()
                .collect();
                let flag_str = if flags.is_empty() {
                    String::new()
                } else {
                    format!("  [{}]", flags.join(", "))
                };
                lines.push(Line::from(vec![
                    Span::styled("   ", Style::default()),
                    Span::styled(col.name.clone(), Style::default().fg(Color::White)),
                    Span::styled(
                        format!("  {}", col.sql_type),
                        Style::default().fg(Color::Blue),
                    ),
                    Span::styled(flag_str, Style::default().fg(Color::Yellow)),
                ]));
                // Inline column stats (if available)
                if let Some(col_stats) = model
                    .column_stats_for(&t.schema, &t.table_name)
                    .and_then(|sv| sv.iter().find(|s| s.column_name == col.name))
                {
                    let mut stat_parts = vec![];
                    let null_pct = (col_stats.null_fraction * 100.0).round() as u8;
                    if null_pct > 0 {
                        stat_parts.push(format!("null:{null_pct}%"));
                    }
                    if col_stats.avg_width_bytes > 0 {
                        stat_parts.push(format!("avg:{}B", col_stats.avg_width_bytes));
                    }
                    if col_stats.n_distinct != 0.0 {
                        if col_stats.n_distinct < 0.0 {
                            stat_parts
                                .push(format!("~{:.0}%unique", -col_stats.n_distinct * 100.0));
                        } else {
                            stat_parts.push(format!("~{:.0}distinct", col_stats.n_distinct));
                        }
                    }
                    if let Some(corr) = col_stats.correlation {
                        stat_parts.push(format!("corr:{corr:.2}"));
                    }
                    if !stat_parts.is_empty() {
                        lines.push(Line::from(vec![
                            Span::styled("      ", Style::default()),
                            Span::styled(
                                stat_parts.join("  "),
                                Style::default().fg(Color::DarkGray),
                            ),
                        ]));
                    }
                }
            }
            lines.push(Line::default());
            lines.push(Line::from(Span::styled(
                " Enter: preview  d: DDL  e: explain",
                Style::default().fg(Color::DarkGray),
            )));
            // Enrichment: FK / constraints / indexes (if already loaded)
            if let Some(inspection) = model.inspection(&t.schema, &t.table_name) {
                if !inspection.foreign_keys.is_empty() {
                    lines.push(Line::default());
                    lines.push(Line::from(Span::styled(
                        " Foreign Keys:",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )));
                    for fk in &inspection.foreign_keys {
                        lines.push(Line::from(vec![
                            Span::styled("   ", Style::default()),
                            Span::styled(fk.from_column.clone(), Style::default().fg(Color::White)),
                            Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                            Span::styled(
                                format!("{}.{}.{}", fk.to_schema, fk.to_table, fk.to_column),
                                Style::default().fg(Color::Blue),
                            ),
                        ]));
                    }
                }
                if !inspection.constraints.is_empty() {
                    lines.push(Line::default());
                    lines.push(Line::from(Span::styled(
                        " Constraints:",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )));
                    for c in &inspection.constraints {
                        let cols = c.columns.join(", ");
                        lines.push(Line::from(vec![
                            Span::styled("   ", Style::default()),
                            Span::styled(
                                format!("{:?}", c.kind),
                                Style::default().fg(Color::Magenta),
                            ),
                            Span::styled(format!("  {}", c.name), Style::default().fg(Color::Gray)),
                            Span::styled(
                                if cols.is_empty() {
                                    String::new()
                                } else {
                                    format!("  ({cols})")
                                },
                                Style::default().fg(Color::White),
                            ),
                        ]));
                    }
                }
                if !inspection.indexes.is_empty() {
                    lines.push(Line::default());
                    lines.push(Line::from(Span::styled(
                        " Indexes:",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )));
                    for idx in &inspection.indexes {
                        let cols = idx.column_names.join(", ");
                        let unique = if idx.is_unique { " UNIQUE" } else { "" };
                        lines.push(Line::from(vec![
                            Span::styled("   ", Style::default()),
                            Span::styled(idx.index_name.clone(), Style::default().fg(Color::Gray)),
                            Span::styled(
                                format!("  ({cols}){unique} [{}]", idx.index_method),
                                Style::default().fg(Color::Blue),
                            ),
                        ]));
                    }
                }
            }
            lines
        }
        None => vec![Line::from(Span::styled(
            " No selection.",
            Style::default().fg(Color::DarkGray),
        ))],
    };

    let para = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .title(" Detail ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black));
    frame.render_widget(para, area);
}

fn draw_ddl_panel(frame: &mut Frame, area: Rect, schema: &str, table: &str, ddl: &str) {
    let lines: Vec<Line> = ddl.lines().map(|l| Line::from(l.to_string())).collect();
    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .title(format!(" DDL: {schema}.{table} "))
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black))
        .wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(para, area);
}

fn draw_explain_panel(
    frame: &mut Frame,
    area: Rect,
    _schema: &str,
    _table: &str,
    root: &crate::archive::ExplainNode,
) {
    let mut lines = Vec::new();
    render_explain_node(&mut lines, root, 0);
    lines.push(Line::default());
    lines.push(Line::from(Span::styled(
        " Esc: back",
        Style::default().fg(Color::DarkGray),
    )));
    let para = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .title(" EXPLAIN Plan ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black))
        .wrap(ratatui::widgets::Wrap { trim: false });
    frame.render_widget(para, area);
}

fn render_explain_node(
    lines: &mut Vec<Line<'static>>,
    node: &crate::archive::ExplainNode,
    depth: usize,
) {
    let indent = "  ".repeat(depth);
    let relation = node
        .relation_name
        .as_deref()
        .map(|r| format!(" on {r}"))
        .unwrap_or_default();
    let alias = node
        .alias
        .as_deref()
        .filter(|a| Some(*a) != node.relation_name.as_deref())
        .map(|a| format!(" as {a}"))
        .unwrap_or_default();
    let header = format!(
        "{indent}▸ {}{}{}  cost={:.1}..{:.1}  rows={}",
        node.node_type, relation, alias, node.startup_cost, node.total_cost, node.plan_rows,
    );
    lines.push(Line::from(vec![Span::styled(
        header,
        Style::default().fg(Color::White),
    )]));
    if let (Some(at), Some(ar)) = (node.actual_total_time, node.actual_rows) {
        let stats = format!("{indent}  actual: {at:.2}ms  rows={ar}");
        lines.push(Line::from(vec![Span::styled(
            stats,
            Style::default().fg(Color::DarkGray),
        )]));
    }
    for child in &node.children {
        render_explain_node(lines, child, depth + 1);
    }
}

fn draw_loading(frame: &mut Frame, area: Rect, schema: &str, table: &str) {
    let para = Paragraph::new(Line::from(vec![
        Span::styled("  ⟳ Loading ", Style::default().fg(Color::Yellow)),
        Span::styled(
            format!("{schema}.{table}"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("…", Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::NONE))
    .style(Style::default().bg(Color::Black));
    frame.render_widget(para, area);
}

fn draw_data_grid(
    frame: &mut Frame,
    area: Rect,
    schema: &str,
    table: &str,
    result: &QueryResult,
    table_state: &mut TableState,
) {
    const MAX_COL_W: usize = 30;
    let widths = column_widths(result, MAX_COL_W);

    let header_cells: Vec<Cell> = result
        .columns
        .iter()
        .map(|col| {
            Cell::from(col.name.clone()).style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
        })
        .collect();
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);

    let rows: Vec<Row> = result
        .rows
        .rows
        .iter()
        .map(|row| {
            let cells: Vec<Cell> = result
                .columns
                .iter()
                .enumerate()
                .map(|(ci, _)| {
                    let val = row
                        .0
                        .get(ci)
                        .map(|(_, v)| cell_display(v))
                        .unwrap_or_default();
                    let truncated = if val.len() > MAX_COL_W {
                        format!("{}…", &val[..MAX_COL_W - 1])
                    } else {
                        val
                    };
                    Cell::from(truncated).style(Style::default().fg(Color::White))
                })
                .collect();
            Row::new(cells).height(1)
        })
        .collect();

    let constraints: Vec<Constraint> = widths
        .iter()
        .map(|&w| Constraint::Length(w as u16 + 2))
        .collect();

    let title = format!(
        " {}{}.{}  ({} rows) ",
        if schema.is_empty() { "" } else { "" },
        schema,
        table,
        result.row_count
    );
    let tbl = Table::new(rows, constraints)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .title(title)
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .row_highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(Color::Black));

    frame.render_stateful_widget(tbl, area, table_state);
}

fn draw_sql_editor(
    frame: &mut Frame,
    area: Rect,
    text: &str,
    result: Option<&QueryResult>,
    running: bool,
) {
    let header = if running {
        "⟳ Running…"
    } else {
        "SQL Editor — Ctrl+Enter to run"
    };
    let para = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title(format!(" {header} "))
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black).fg(Color::White));

    if let Some(res) = result {
        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);
        frame.render_widget(para, split[0]);
        let mut ts = TableState::default();
        draw_data_grid(frame, split[1], "", "(result)", res, &mut ts);
    } else {
        frame.render_widget(para, area);
    }
}

fn cell_display(v: &DbValue) -> String {
    match v {
        DbValue::Null => "NULL".to_string(),
        DbValue::Bool(b) => if *b { "t" } else { "f" }.to_string(),
        DbValue::Int(n) => n.to_string(),
        DbValue::Float(f) => format!("{f:.4}"),
        DbValue::Text(s) => s.clone(),
        DbValue::Bytes(b) => format!("\\x{}", hex_short(b)),
        DbValue::Json(j) => j.to_string(),
        DbValue::Geometry(s) | DbValue::Geography(s) => match s {
            elicit_db::DbSpatialValue::Wkt(w) => format!("<geo wkt:{}>", &w[..w.len().min(20)]),
            elicit_db::DbSpatialValue::Wkb(b) => format!("<geo wkb:{} bytes>", b.len()),
        },
    }
}

fn hex_short(bytes: &[u8]) -> String {
    let s: String = bytes.iter().take(8).map(|b| format!("{b:02x}")).collect();
    if bytes.len() > 8 {
        format!("{s}…")
    } else {
        s
    }
}

fn draw_status_bar(frame: &mut Frame, area: Rect) {
    use elicit_ratatui::TuiNode;
    let chips = ArchiveNavModel::bindings()
        .into_iter()
        .map(|b| (b.key, b.action))
        .collect();
    let bar = TuiNode::StatusBar {
        chips,
        theme: ColorTheme::Dark,
    };
    render_node(frame, area, &bar);
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let bindings: &[(&str, &str)] = &[
        ("q / Esc  ", "Quit"),
        ("↑ / k    ", "Move up"),
        ("↓ / j    ", "Move down"),
        ("Enter    ", "Expand schema / preview table"),
        ("r        ", "Refresh tree"),
        ("/        ", "Filter / search"),
        ("Esc      ", "(in filter) clear filter"),
        ("?        ", "Toggle this help"),
    ];

    let height = bindings.len() as u16 + 4;
    let width = 48u16;
    let popup = centered_rect(width, height, area);

    let lines: Vec<Line> = bindings
        .iter()
        .map(|(k, a)| {
            Line::from(vec![
                Span::styled(
                    format!("  {k}  "),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(*a, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let help = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Keybindings ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black));

    frame.render_widget(Clear, popup);
    frame.render_widget(help, popup);
}

fn draw_export_picker(frame: &mut Frame, area: Rect, selected: usize) {
    const FORMATS: &[(&str, &str)] = &[
        ("CSV  ", "Comma-separated values (.csv)"),
        ("JSON ", "JSON array of objects (.json)"),
        ("NDJSON", "Newline-delimited JSON (.ndjson)"),
        ("TSV  ", "Tab-separated values (.tsv)"),
    ];
    let height = FORMATS.len() as u16 + 4;
    let width = 46u16;
    let popup = centered_rect(width, height, area);

    let lines: Vec<Line> = FORMATS
        .iter()
        .enumerate()
        .map(|(i, (fmt, desc))| {
            let (key_style, desc_style) = if i == selected {
                (
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                )
            } else {
                (
                    Style::default().fg(Color::Cyan),
                    Style::default().fg(Color::White),
                )
            };
            Line::from(vec![
                Span::styled(format!("  {fmt}  "), key_style),
                Span::styled(*desc, desc_style),
            ])
        })
        .collect();

    let picker = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Export Format (↑↓ Enter Esc) ")
                .title_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
        )
        .style(Style::default().bg(Color::Black));

    frame.render_widget(Clear, popup);
    frame.render_widget(picker, popup);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
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

    // Initialize history store (best-effort — None on failure).
    let history = HistoryStore::open().await.ok();
    let history_cache = if let Some(ref store) = history {
        store
            .recent(crate::archive::plugins::history::MAX_HISTORY)
            .await
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let mut app = TuiApp::new(nav, req_tx, history);
    app.model.history_cache = history_cache;
    let mut reader = EventStream::new();
    let mut quit = false;

    let result: ArchiveResult<()> = async {
        loop {
            terminal
                .draw(|frame| draw_app(frame, &mut app))
                .map_err(|e: std::io::Error| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })?;

            if quit {
                break;
            }

            tokio::select! {
                // Panel data from background task
                Some(ev) = event_rx.recv() => {
                    app.apply_panel_event(ev);
                }
                // Terminal events
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
                        } else if app.export_picker {
                            match key.code {
                                KeyCode::Esc => app.export_picker = false,
                                KeyCode::Up | KeyCode::Char('k') => {
                                    if app.export_picker_idx > 0 {
                                        app.export_picker_idx -= 1;
                                    }
                                }
                                KeyCode::Down | KeyCode::Char('j') => {
                                    app.export_picker_idx =
                                        (app.export_picker_idx + 1).min(3);
                                }
                                KeyCode::Enter => app.confirm_export(),
                                _ => {}
                            }
                        } else if matches!(app.model.panel, PanelMode::SqlEditor { .. }) {
                            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                            match (key.code, ctrl) {
                                // Ctrl+Enter or F5 → execute
                                (KeyCode::Enter, true) | (KeyCode::F(5), _) => app.run_sql(),
                                // Ctrl+Up → older history entry
                                (KeyCode::Up, true) => { app.model.history_prev(); }
                                // Ctrl+Down → newer history entry
                                (KeyCode::Down, true) => { app.model.history_next(); }
                                // Esc → leave SQL editor
                                (KeyCode::Esc, _) => {
                                    app.model.panel = PanelMode::ColumnDetail;
                                }
                                // Typing updates the SQL text
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
                                // s → open SQL editor
                                KeyCode::Char('s') => {
                                    app.model.panel = PanelMode::SqlEditor {
                                        text: String::new(),
                                        result: None,
                                        running: false,
                                    };
                                }
                                KeyCode::Char('x') => {
                                    if app.model.panel.is_data_grid() {
                                        app.export_picker = true;
                                        app.export_picker_idx = 0;
                                    }
                                }
                                // Data grid navigation
                                KeyCode::PageDown => {
                                    if let PanelMode::DataGrid { result, .. } = &app.model.panel {
                                        let max = result.rows.rows.len().saturating_sub(1);
                                        let sel = app.table_state.selected().unwrap_or(0);
                                        app.table_state.select(Some((sel + 10).min(max)));
                                    }
                                }
                                KeyCode::PageUp => {
                                    if app.model.panel.is_data_grid() {
                                        let sel = app.table_state.selected().unwrap_or(0);
                                        app.table_state.select(Some(sel.saturating_sub(10)));
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
