//! Runtime terminal management tools for ratatui.
//!
//! These tools manage terminal lifecycle, rendering, and cursor control.
//! Requires the `runtime` feature (crossterm backend).

use std::collections::HashMap;
use std::io;
use std::sync::{Mutex, OnceLock};

use ratatui::Frame;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::layout::Rect;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

use crate::serde_types::{BlockJson, ParagraphText, TuiNode, WidgetJson};
use crate::wcag_verify::verify_wcag_contrast_proofs;
use elicit_ui::ColorTheme;
use elicitation::elicit_tool;

/// Crossterm terminal type alias.
type TerminalType = Terminal<CrosstermBackend<io::Stdout>>;

/// Global terminal registry.
static TERMINALS: OnceLock<Mutex<HashMap<Uuid, TerminalType>>> = OnceLock::new();

/// Access the global terminal registry.
fn terminals() -> &'static Mutex<HashMap<Uuid, TerminalType>> {
    TERMINALS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Serialise a JSON value to a `CallToolResult`.
fn json_result(value: &impl serde::Serialize) -> CallToolResult {
    match serde_json::to_string(value) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Parse a terminal ID from a string parameter.
fn parse_terminal_id(id: &str) -> Result<Uuid, Box<CallToolResult>> {
    Uuid::parse_str(id).map_err(|e| {
        Box::new(CallToolResult::error(vec![Content::text(format!(
            "invalid terminal_id: {e}"
        ))]))
    })
}

// ---------------------------------------------------------------------------
// terminal_create
// ---------------------------------------------------------------------------

/// Parameters for `terminal_create`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalCreateParams {
    /// Optional window title.
    #[serde(default)]
    pub title: Option<String>,
}

/// Create a new terminal with crossterm backend.
///
/// Enables raw mode, enters alternate screen, and registers the terminal.
/// Returns the terminal ID as a UUID string.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_create",
    description = "Create a new crossterm terminal. Enables raw mode and alternate screen. Returns { terminal_id }."
)]
#[instrument(skip_all)]
async fn terminal_create(p: TerminalCreateParams) -> Result<CallToolResult, ErrorData> {
    let result: Result<Uuid, String> = (|| {
        enable_raw_mode().map_err(|e| format!("enable raw mode: {e}"))?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)
            .map_err(|e| format!("enter alternate screen: {e}"))?;
        if let Some(title) = &p.title {
            execute!(
                stdout,
                ratatui::crossterm::terminal::SetTitle(title.as_str())
            )
            .map_err(|e| format!("set title: {e}"))?;
        }
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).map_err(|e| format!("create terminal: {e}"))?;
        let id = Uuid::new_v4();
        terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?
            .insert(id, terminal);
        Ok(id)
    })();

    match result {
        Ok(id) => Ok(json_result(
            &serde_json::json!({ "terminal_id": id.to_string() }),
        )),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_destroy
// ---------------------------------------------------------------------------

/// Parameters for `terminal_destroy`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalDestroyParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
}

/// Destroy a terminal and restore terminal state.
///
/// Leaves alternate screen, disables raw mode, and removes the terminal
/// from the registry.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_destroy",
    description = "Destroy a terminal, leave alternate screen, and disable raw mode."
)]
#[instrument(skip_all)]
async fn terminal_destroy(p: TerminalDestroyParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<(), String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let _terminal = guard
            .remove(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        // Drop terminal before restoring state
        drop(_terminal);
        disable_raw_mode().map_err(|e| format!("disable raw mode: {e}"))?;
        execute!(io::stdout(), LeaveAlternateScreen)
            .map_err(|e| format!("leave alternate screen: {e}"))?;
        Ok(())
    })();

    match result {
        Ok(()) => Ok(json_result(&serde_json::json!({ "success": true }))),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_clear
// ---------------------------------------------------------------------------

/// Parameters for `terminal_clear`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalClearParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
}

/// Clear the terminal screen.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_clear",
    description = "Clear the terminal screen."
)]
#[instrument(skip_all)]
async fn terminal_clear(p: TerminalClearParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<(), String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let terminal = guard
            .get_mut(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        terminal
            .clear()
            .map_err(|e| format!("clear terminal: {e}"))?;
        Ok(())
    })();

    match result {
        Ok(()) => Ok(json_result(&serde_json::json!({ "success": true }))),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_size
// ---------------------------------------------------------------------------

/// Parameters for `terminal_size`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalSizeParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
}

/// Get the terminal dimensions.
///
/// Returns `{ width, height }` in columns and rows.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_size",
    description = "Get terminal dimensions. Returns { width, height }."
)]
#[instrument(skip_all)]
async fn terminal_size(p: TerminalSizeParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<ratatui::layout::Size, String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let terminal = guard
            .get_mut(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        terminal.size().map_err(|e| format!("get size: {e}"))
    })();

    match result {
        Ok(size) => Ok(json_result(
            &serde_json::json!({ "width": size.width, "height": size.height }),
        )),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_hide_cursor
// ---------------------------------------------------------------------------

/// Parameters for `terminal_hide_cursor`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalHideCursorParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
}

/// Hide the terminal cursor.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_hide_cursor",
    description = "Hide the terminal cursor."
)]
#[instrument(skip_all)]
async fn terminal_hide_cursor(p: TerminalHideCursorParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<(), String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let terminal = guard
            .get_mut(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        terminal
            .hide_cursor()
            .map_err(|e| format!("hide cursor: {e}"))?;
        Ok(())
    })();

    match result {
        Ok(()) => Ok(json_result(&serde_json::json!({ "success": true }))),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_show_cursor
// ---------------------------------------------------------------------------

/// Parameters for `terminal_show_cursor`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalShowCursorParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
}

/// Show the terminal cursor.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_show_cursor",
    description = "Show the terminal cursor."
)]
#[instrument(skip_all)]
async fn terminal_show_cursor(p: TerminalShowCursorParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<(), String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let terminal = guard
            .get_mut(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        terminal
            .show_cursor()
            .map_err(|e| format!("show cursor: {e}"))?;
        Ok(())
    })();

    match result {
        Ok(()) => Ok(json_result(&serde_json::json!({ "success": true }))),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_set_cursor
// ---------------------------------------------------------------------------

/// Parameters for `terminal_set_cursor`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalSetCursorParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
    /// Column position (0-based).
    pub x: u16,
    /// Row position (0-based).
    pub y: u16,
}

/// Set the terminal cursor position.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_set_cursor",
    description = "Set the terminal cursor position at (x, y)."
)]
#[instrument(skip_all)]
async fn terminal_set_cursor(p: TerminalSetCursorParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<(), String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let terminal = guard
            .get_mut(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        terminal
            .set_cursor_position((p.x, p.y))
            .map_err(|e| format!("set cursor: {e}"))?;
        Ok(())
    })();

    match result {
        Ok(()) => Ok(json_result(&serde_json::json!({ "success": true }))),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// terminal_draw
// ---------------------------------------------------------------------------

/// Parameters for `terminal_draw`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TerminalDrawParams {
    /// Terminal ID returned by `terminal_create`.
    pub terminal_id: String,
    /// Root TuiNode tree to render.
    pub root: TuiNode,
}

/// Draw a TuiNode tree to the terminal.
///
/// Recursively renders the node tree into the terminal frame.
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_draw",
    description = "Draw a TuiNode tree to the terminal. Recursively renders widgets and layouts."
)]
#[instrument(skip_all)]
async fn terminal_draw(p: TerminalDrawParams) -> Result<CallToolResult, ErrorData> {
    let id = match parse_terminal_id(&p.terminal_id) {
        Ok(id) => id,
        Err(r) => return Ok(*r),
    };
    let result: Result<(), String> = (|| {
        let mut guard = terminals()
            .lock()
            .map_err(|e| format!("lock registry: {e}"))?;
        let terminal = guard
            .get_mut(&id)
            .ok_or_else(|| format!("terminal {id} not found"))?;
        let root = p.root;
        terminal
            .draw(|frame| {
                let area = frame.area();
                render_node(frame, area, &root);
            })
            .map_err(|e| format!("draw: {e}"))?;
        Ok(())
    })();

    match result {
        Ok(()) => Ok(json_result(&serde_json::json!({ "success": true }))),
        Err(msg) => Ok(CallToolResult::error(vec![Content::text(msg)])),
    }
}

// ---------------------------------------------------------------------------
// Rendering helpers
// ---------------------------------------------------------------------------

/// Recursively render a `TuiNode` tree into a ratatui frame.
///
/// Callers with a `TuiNode` tree (e.g. from `tui_node_to_tree_update` /
/// `tree_update_to_tui_node` or built directly) can call this to drive a
/// `ratatui::Frame` without re-implementing the layout/widget dispatch.
pub fn render_node(frame: &mut Frame, area: Rect, node: &TuiNode) {
    match node {
        TuiNode::Widget { widget, proofs } => {
            render_widget(frame, area, widget);
            let buf = frame.buffer_mut();
            let ctx = crate::RatatuiRenderContext::new(buf);
            verify_wcag_contrast_proofs(&ctx, &area, proofs);
        }
        TuiNode::Layout {
            direction,
            constraints,
            children,
            margin,
        } => {
            let dir: ratatui::layout::Direction = (*direction).into();
            let layout_constraints: Vec<ratatui::layout::Constraint> =
                constraints.iter().map(|c| (*c).into()).collect();
            let mut layout = ratatui::layout::Layout::default()
                .direction(dir)
                .constraints(layout_constraints);
            if let Some(m) = margin {
                layout = layout
                    .horizontal_margin(m.horizontal)
                    .vertical_margin(m.vertical);
            }
            let chunks = layout.split(area);
            for (i, child) in children.iter().enumerate() {
                if i < chunks.len() {
                    render_node(frame, chunks[i], child);
                }
            }
        }
        TuiNode::StatusBar { chips, theme } => render_status_bar(frame, area, chips, *theme),
    }
}

/// Render a Zellij-style status bar into a single-line area.
///
/// Each chip pair `(key, action)` is rendered as a styled key label followed
/// by the action description.  Colors are chosen by `theme`.
fn render_status_bar(frame: &mut Frame, area: Rect, chips: &[(String, String)], theme: ColorTheme) {
    use ratatui::style::{Color, Modifier, Style};
    use ratatui::text::{Line, Span};
    use ratatui::widgets::Paragraph;

    let (key_bg, key_fg, action_fg, bar_bg) = match theme {
        ColorTheme::Dark => (Color::Cyan, Color::Black, Color::Gray, Color::DarkGray),
        ColorTheme::Light => (Color::Blue, Color::White, Color::DarkGray, Color::Gray),
        ColorTheme::HighContrast => (Color::Yellow, Color::Black, Color::White, Color::Black),
        ColorTheme::Solarized => (
            Color::Rgb(38, 139, 210),
            Color::White,
            Color::Rgb(147, 161, 161),
            Color::Rgb(0, 43, 54),
        ),
    };

    let mut spans: Vec<Span<'static>> = vec![Span::styled(" ", Style::default().bg(bar_bg))];
    for (key, action) in chips {
        spans.push(Span::styled(
            format!(" {key} "),
            Style::default()
                .bg(key_bg)
                .fg(key_fg)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            format!(" {action}  "),
            Style::default().fg(action_fg).bg(bar_bg),
        ));
    }

    let line = Line::from(spans);
    let bar = Paragraph::new(line).style(Style::default().bg(bar_bg));
    frame.render_widget(bar, area);
}

/// Render a single `WidgetJson` into a frame area.
///
/// Exposed so callers can render individual widgets by hand when they
/// do not need the full recursive `TuiNode` tree dispatch.
pub fn render_widget(frame: &mut Frame, area: Rect, widget: &WidgetJson) {
    match widget {
        WidgetJson::Block { block } => {
            render_block_widget(frame, area, block);
        }
        WidgetJson::Paragraph {
            text,
            style,
            wrap,
            scroll,
            alignment,
            block,
        } => {
            let ratatui_text: ratatui::text::Text<'static> = match text {
                ParagraphText::Plain(s) => ratatui::text::Text::raw(s.clone()),
                ParagraphText::Rich(t) => {
                    let lines: Vec<ratatui::text::Line<'static>> = t
                        .lines
                        .iter()
                        .map(|l| {
                            let spans: Vec<ratatui::text::Span<'static>> = l
                                .spans
                                .iter()
                                .map(|s| {
                                    let style = s
                                        .style
                                        .clone()
                                        .map(ratatui::style::Style::from)
                                        .unwrap_or_default();
                                    ratatui::text::Span::styled(s.content.clone(), style)
                                })
                                .collect();
                            let mut line = ratatui::text::Line::from(spans);
                            if let Some(ls) = &l.style {
                                line = line.style(ratatui::style::Style::from(ls.clone()));
                            }
                            if let Some(a) = l.alignment {
                                line = line.alignment(ratatui::layout::Alignment::from(a));
                            }
                            line
                        })
                        .collect();
                    let mut rich = ratatui::text::Text::from(lines);
                    if let Some(ts) = &t.style {
                        rich = rich.style(ratatui::style::Style::from(ts.clone()));
                    }
                    if let Some(a) = t.alignment {
                        rich = rich.alignment(ratatui::layout::Alignment::from(a));
                    }
                    rich
                }
            };
            let mut p = ratatui::widgets::Paragraph::new(ratatui_text);
            if let Some(s) = style {
                p = p.style(ratatui::style::Style::from(s.clone()));
            }
            if *wrap {
                p = p.wrap(ratatui::widgets::Wrap { trim: false });
            }
            if let Some((row, col)) = scroll {
                p = p.scroll((*row, *col));
            }
            if let Some(a) = alignment {
                let align = match a.as_str() {
                    "Center" => ratatui::layout::Alignment::Center,
                    "Right" => ratatui::layout::Alignment::Right,
                    _ => ratatui::layout::Alignment::Left,
                };
                p = p.alignment(align);
            }
            if let Some(b) = block {
                p = p.block(build_block(b));
            }
            frame.render_widget(p, area);
        }
        WidgetJson::List {
            items,
            block,
            style,
            highlight_style,
            highlight_symbol,
            ..
        } => {
            let list_items: Vec<ratatui::text::Line> = items
                .iter()
                .map(|s| ratatui::text::Line::raw(s.as_str()))
                .collect();
            let mut list = ratatui::widgets::List::new(list_items);
            if let Some(s) = style {
                list = list.style(ratatui::style::Style::from(s.clone()));
            }
            if let Some(hs) = highlight_style {
                list = list.highlight_style(ratatui::style::Style::from(hs.clone()));
            }
            if let Some(sym) = highlight_symbol {
                list = list.highlight_symbol(sym.as_str());
            }
            if let Some(b) = block {
                list = list.block(build_block(b));
            }
            frame.render_widget(list, area);
        }
        WidgetJson::Table {
            header,
            rows,
            widths,
            column_spacing,
            block,
            highlight_style,
            ..
        } => {
            let make_row = |r: &crate::serde_types::RowJson| -> ratatui::widgets::Row<'static> {
                let cells: Vec<ratatui::text::Text<'static>> = r
                    .cells
                    .iter()
                    .map(|c| ratatui::text::Text::from(c.content.clone()))
                    .collect();
                ratatui::widgets::Row::new(cells)
            };
            let data_rows: Vec<ratatui::widgets::Row<'static>> =
                rows.iter().map(&make_row).collect();
            let constraints: Vec<ratatui::layout::Constraint> =
                widths.iter().map(|w| (*w).into()).collect();
            let mut table = ratatui::widgets::Table::new(data_rows, constraints);
            if let Some(h) = header {
                table = table.header(make_row(h));
            }
            if let Some(cs) = column_spacing {
                table = table.column_spacing(*cs);
            }
            if let Some(hs) = highlight_style {
                table = table.row_highlight_style(ratatui::style::Style::from(hs.clone()));
            }
            if let Some(b) = block {
                table = table.block(build_block(b));
            }
            frame.render_widget(table, area);
        }
        WidgetJson::Gauge {
            ratio,
            label,
            block,
            style,
            gauge_style,
        } => {
            let mut g = ratatui::widgets::Gauge::default().ratio(*ratio);
            if let Some(l) = label {
                g = g.label(l.as_str());
            }
            if let Some(s) = style {
                g = g.style(ratatui::style::Style::from(s.clone()));
            }
            if let Some(gs) = gauge_style {
                g = g.gauge_style(ratatui::style::Style::from(gs.clone()));
            }
            if let Some(b) = block {
                g = g.block(build_block(b));
            }
            frame.render_widget(g, area);
        }
        WidgetJson::Sparkline {
            data,
            block,
            style,
            max,
            ..
        } => {
            let mut s = ratatui::widgets::Sparkline::default().data(data);
            if let Some(st) = style {
                s = s.style(ratatui::style::Style::from(st.clone()));
            }
            if let Some(m) = max {
                s = s.max(*m);
            }
            if let Some(b) = block {
                s = s.block(build_block(b));
            }
            frame.render_widget(s, area);
        }
        WidgetJson::Tabs {
            titles,
            selected,
            block,
            style,
            highlight_style,
            ..
        } => {
            let tab_titles: Vec<&str> = titles.iter().map(|s| s.as_str()).collect();
            let mut t = ratatui::widgets::Tabs::new(tab_titles);
            if let Some(sel) = selected {
                t = t.select(*sel);
            }
            if let Some(s) = style {
                t = t.style(ratatui::style::Style::from(s.clone()));
            }
            if let Some(hs) = highlight_style {
                t = t.highlight_style(ratatui::style::Style::from(hs.clone()));
            }
            if let Some(b) = block {
                t = t.block(build_block(b));
            }
            frame.render_widget(t, area);
        }
        WidgetJson::Clear => {
            frame.render_widget(ratatui::widgets::Clear, area);
        }
        // Complex widgets (BarChart, Chart, LineGauge, Scrollbar) render a placeholder.
        _ => {
            let p = ratatui::widgets::Paragraph::new("(complex widget)")
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray));
            frame.render_widget(p, area);
        }
    }
}

/// Build a ratatui `Block` from a `BlockJson` descriptor.
fn build_block(b: &BlockJson) -> ratatui::widgets::Block<'_> {
    let mut blk = ratatui::widgets::Block::default().borders(b.borders.into());
    if let Some(bt) = &b.border_type {
        blk = blk.border_type((*bt).into());
    }
    if let Some(t) = &b.title {
        blk = blk.title(t.as_str());
    }
    if let Some(s) = &b.style {
        blk = blk.style(ratatui::style::Style::from(s.clone()));
    }
    if let Some(bs) = &b.border_style {
        blk = blk.border_style(ratatui::style::Style::from(bs.clone()));
    }
    if let Some(p) = &b.padding {
        blk = blk.padding(ratatui::widgets::Padding::from(*p));
    }
    blk
}

/// Render a standalone Block widget.
fn render_block_widget(frame: &mut Frame, area: Rect, block: &BlockJson) {
    frame.render_widget(build_block(block), area);
}
