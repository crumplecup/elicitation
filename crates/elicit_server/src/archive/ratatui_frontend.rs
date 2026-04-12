//! Ratatui terminal frontend for the archive CLI.
//!
//! Provides an interactive pgAdmin-style database browser with keyboard
//! navigation: `↑`/`↓` move selection, `Enter` expands/collapses schemas,
//! `r` refreshes, `?` toggles the keybinding help overlay, `q`/`Esc` quits.

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use elicit_ratatui::render_node;
use elicit_ui::ColorTheme;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};
use ratatui::{Frame, Terminal, backend::CrosstermBackend};
use tracing::instrument;

use crate::archive::nav_tree::{NavTree, SchemaEntry};
use crate::archive::{
    ArchiveResult, TableType,
    errors::{ArchiveError, ArchiveErrorKind},
};

// ── Flat navigation item ──────────────────────────────────────────────────────

/// One visible row in the navigation list.
#[derive(Clone)]
enum FlatItem {
    /// A schema row.  Carries the schema index so we can look it up.
    Schema(usize),
    /// A table/view row.  Carries (schema_idx, table_idx).
    Table(usize, usize),
}

// ── App state ────────────────────────────────────────────────────────────────

struct AppState {
    db_name: String,
    version: Option<String>,
    backend_label: String,
    schemas: Vec<SchemaWithExpand>,
    flat: Vec<FlatItem>,
    list_state: ListState,
    show_help: bool,
    flash: Option<String>,
}

struct SchemaWithExpand {
    entry: SchemaEntry,
    expanded: bool,
}

impl AppState {
    fn new(nav: NavTree) -> Self {
        let schemas: Vec<SchemaWithExpand> = nav
            .schemas
            .into_iter()
            .map(|e| SchemaWithExpand {
                entry: e,
                expanded: false,
            })
            .collect();

        let mut app = Self {
            db_name: nav.db_name,
            version: nav.version,
            backend_label: nav.backend.to_string(),
            schemas,
            flat: Vec::new(),
            list_state: ListState::default(),
            show_help: false,
            flash: None,
        };
        app.rebuild_flat();
        if !app.flat.is_empty() {
            app.list_state.select(Some(0));
        }
        app
    }

    fn rebuild_flat(&mut self) {
        self.flat.clear();
        for (i, s) in self.schemas.iter().enumerate() {
            self.flat.push(FlatItem::Schema(i));
            if s.expanded {
                for j in 0..s.entry.tables.len() {
                    self.flat.push(FlatItem::Table(i, j));
                }
            }
        }
    }

    fn move_up(&mut self) {
        let cur = self.list_state.selected().unwrap_or(0);
        let next = if cur == 0 {
            self.flat.len().saturating_sub(1)
        } else {
            cur - 1
        };
        self.list_state.select(Some(next));
    }

    fn move_down(&mut self) {
        let cur = self.list_state.selected().unwrap_or(0);
        let next = if cur + 1 >= self.flat.len() {
            0
        } else {
            cur + 1
        };
        self.list_state.select(Some(next));
    }

    fn toggle_expand(&mut self) {
        let selected = self.list_state.selected().unwrap_or(0);
        let Some(item) = self.flat.get(selected).cloned() else {
            return;
        };
        match item {
            FlatItem::Schema(i) => {
                self.schemas[i].expanded = !self.schemas[i].expanded;
                self.rebuild_flat();
                // Keep cursor on the same schema row after rebuild.
                let new_pos = self
                    .flat
                    .iter()
                    .position(|f| matches!(f, FlatItem::Schema(j) if *j == i))
                    .unwrap_or(0);
                self.list_state.select(Some(new_pos));
            }
            FlatItem::Table(si, ti) => {
                let t = &self.schemas[si].entry.tables[ti];
                let cols = t.columns.len();
                let rows = t
                    .estimated_rows
                    .map(|r| format!("~{r} rows"))
                    .unwrap_or_else(|| "rows: ?".to_string());
                self.flash = Some(format!(
                    "{}.{} — {cols} columns, {rows}",
                    t.schema, t.table_name
                ));
            }
        }
    }

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        self.flash = None;
    }

    fn refresh(&mut self) {
        self.flash = Some("↺ Refreshed (demo)".to_string());
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

fn draw_app(frame: &mut Frame, app: &mut AppState) {
    let area = frame.area();

    // Three-row vertical split: header | nav list | status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    draw_header(frame, chunks[0], app);
    draw_nav(frame, chunks[1], app);
    draw_status_bar(frame, chunks[2]);

    if app.show_help {
        draw_help(frame, area);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &AppState) {
    let ver = app.version.as_deref().unwrap_or("unknown");
    let flash = app.flash.as_deref().unwrap_or("");
    let title_line = Line::from(vec![
        Span::styled(
            format!(" {} ", app.backend_label),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{ver} "), Style::default().fg(Color::White)),
        Span::styled("│ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{} ", app.db_name),
            Style::default().fg(Color::Yellow),
        ),
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

fn draw_nav(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let items: Vec<ListItem> = app
        .flat
        .iter()
        .map(|fi| match fi {
            FlatItem::Schema(i) => {
                let s = &app.schemas[*i];
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
                let s = &app.schemas[*si];
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

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▌");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn draw_status_bar(frame: &mut Frame, area: Rect) {
    use elicit_ratatui::TuiNode;
    let bar = TuiNode::StatusBar {
        chips: vec![
            ("q".to_string(), "Quit".to_string()),
            ("↑↓".to_string(), "Navigate".to_string()),
            ("Enter".to_string(), "Select".to_string()),
            ("r".to_string(), "Refresh".to_string()),
            ("?".to_string(), "Help".to_string()),
        ],
        theme: ColorTheme::Dark,
    };
    render_node(frame, area, &bar);
}

fn draw_help(frame: &mut Frame, area: Rect) {
    let bindings: &[(&str, &str)] = &[
        ("q / Esc  ", "Quit"),
        ("↑ / k    ", "Move up"),
        ("↓ / j    ", "Move down"),
        ("Enter    ", "Expand / collapse schema"),
        ("r        ", "Refresh"),
        ("?        ", "Toggle this help"),
    ];

    let height = bindings.len() as u16 + 4; // lines + borders + title + padding
    let width = 42u16;
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

// ── Event loop ────────────────────────────────────────────────────────────────

/// Run the interactive archive browser in a crossterm terminal.
///
/// Blocks until the user presses `q` or `Esc`.
#[instrument(skip(nav))]
pub fn run_tui(nav: NavTree) -> ArchiveResult<()> {
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

    let mut app = AppState::new(nav);

    let result = (|| -> ArchiveResult<()> {
        loop {
            terminal
                .draw(|frame| draw_app(frame, &mut app))
                .map_err(|e: std::io::Error| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })?;

            if event::poll(std::time::Duration::from_millis(100)).map_err(|e: std::io::Error| {
                ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
            })? {
                if let Event::Key(key) = event::read().map_err(|e: std::io::Error| {
                    ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
                })? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.move_up();
                            app.flash = None;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.move_down();
                            app.flash = None;
                        }
                        KeyCode::Enter => app.toggle_expand(),
                        KeyCode::Char('r') => app.refresh(),
                        KeyCode::Char('?') => app.toggle_help(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode().map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen).map_err(|e: std::io::Error| {
        ArchiveError::new(ArchiveErrorKind::Frontend(e.to_string()))
    })?;

    result
}
