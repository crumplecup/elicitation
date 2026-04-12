//! Ratatui terminal frontend for the archive CLI.
//!
//! Provides an interactive pgAdmin-style database browser with keyboard
//! navigation: `↑`/`↓` move selection, `Enter` expands/collapses schemas,
//! `r` refreshes, `?` toggles the keybinding help overlay, `q`/`Esc` quits.
//!
//! Key bindings are sourced from [`ArchiveNavModel::bindings`] (the
//! AccessKit IR), keeping all frontends consistent.

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

use crate::archive::nav_model::{ArchiveNavModel, FlatItem};
use crate::archive::nav_tree::NavTree;
use crate::archive::{
    ArchiveResult, TableType,
    errors::{ArchiveError, ArchiveErrorKind},
};

// ── Thin ratatui wrapper ──────────────────────────────────────────────────────

/// Wraps the frontend-agnostic [`ArchiveNavModel`] with the ratatui-specific
/// [`ListState`] needed for stateful list rendering.
struct TuiApp {
    model: ArchiveNavModel,
    list_state: ListState,
}

impl TuiApp {
    fn new(nav: NavTree) -> Self {
        let model = ArchiveNavModel::new(nav);
        let mut list_state = ListState::default();
        if !model.flat.is_empty() {
            list_state.select(Some(model.cursor));
        }
        Self { model, list_state }
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
    }

    fn move_down(&mut self) {
        self.model.move_down();
        self.model.flash = None;
        self.sync_list_state();
    }

    fn toggle_expand(&mut self) {
        self.model.toggle_expand();
        self.sync_list_state();
    }

    fn refresh(&mut self) {
        self.model.refresh();
    }

    fn toggle_help(&mut self) {
        self.model.toggle_help();
    }
}

// ── Drawing ───────────────────────────────────────────────────────────────────

fn draw_app(frame: &mut Frame, app: &mut TuiApp) {
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

    draw_header(frame, chunks[0], &app.model);
    draw_nav(frame, chunks[1], app);
    draw_status_bar(frame, chunks[2]);

    if app.model.show_help {
        draw_help(frame, area);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, model: &ArchiveNavModel) {
    let ver = model.version.as_deref().unwrap_or("unknown");
    let flash = model.flash.as_deref().unwrap_or("");
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
    // Chips sourced from the AccessKit IR — single source of truth.
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

    let mut app = TuiApp::new(nav);

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
                        KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                        KeyCode::Down | KeyCode::Char('j') => app.move_down(),
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
