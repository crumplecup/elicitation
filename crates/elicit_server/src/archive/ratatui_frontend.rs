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
    event::{Event, EventStream, KeyCode, KeyEventKind},
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
    ArchiveDbBackend, ArchiveResult, QueryResult, TableType,
    errors::{ArchiveError, ArchiveErrorKind},
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
}

impl TuiApp {
    fn new(nav: NavTree, req_tx: mpsc::Sender<FetchRequest>) -> Self {
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
        // Eagerly kick off an inspection fetch when landing on a table row.
        if let Some(req) = self.model.inspect_request() {
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
                self.model.panel = PanelMode::ColumnDetail;
                self.model.flash = Some(format!("error: {e}"));
            }
            PanelEvent::SqlResult(result) => {
                self.model.panel = PanelMode::DataGrid {
                    schema: String::new(),
                    table: "(query result)".to_string(),
                    result,
                    page: 0,
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
            }
            lines.push(Line::default());
            lines.push(Line::from(Span::styled(
                " Press Enter to preview table data.  d → DDL",
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
    let mut app = TuiApp::new(nav, req_tx);
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
