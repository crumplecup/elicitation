//! Fragment code generation tools for ratatui.
//!
//! These tools generate complete ratatui application Rust source code
//! from widget/layout descriptions. Requires the `emit` feature.

use crate::serde_types::{ConstraintJson, DirectionJson, TuiNode, WidgetJson};
use elicitation::ToCodeLiteral;
use elicitation::elicit_tool;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Wrap generated code in a JSON `CallToolResult`.
fn code_result(code: &str) -> CallToolResult {
    match serde_json::to_string(&serde_json::json!({ "code": code })) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ---------------------------------------------------------------------------
// Helper types
// ---------------------------------------------------------------------------

/// A field definition for generated app structs.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct AppFieldJson {
    /// Field name.
    pub name: String,
    /// Rust type as string (e.g. `"Vec<String>"`, `"usize"`, `"bool"`).
    pub field_type: String,
    /// Default value expression as string (e.g. "0", "true", "Vec::new()").
    #[serde(default)]
    pub default: Option<String>,
}

/// A key handler mapping for event generation.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct KeyHandlerJson {
    /// Key to handle (e.g. "q", "Up", "Down", "Enter").
    pub key: String,
    /// Rust code to execute (e.g. "app.quit = true", "app.index += 1").
    pub action: String,
}

// ---------------------------------------------------------------------------
// emit_cargo_toml
// ---------------------------------------------------------------------------

/// Parameters for `emit_cargo_toml`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitCargoTomlParams {
    /// Package name (e.g. "my-tui-app").
    pub name: String,
    /// Package version (default "0.1.0").
    #[serde(default)]
    pub version: Option<String>,
    /// ratatui dependency version (default "0.29").
    #[serde(default)]
    pub ratatui_version: Option<String>,
    /// crossterm dependency version (default "0.28").
    #[serde(default)]
    pub crossterm_version: Option<String>,
}

/// Generate a Cargo.toml for a ratatui application.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "emit_cargo_toml",
    description = "Generate Cargo.toml for a ratatui terminal application with crossterm backend."
)]
#[instrument(skip_all)]
async fn emit_cargo_toml(p: EmitCargoTomlParams) -> Result<CallToolResult, ErrorData> {
    let version = p.version.as_deref().unwrap_or("0.1.0");
    let ratatui_ver = p.ratatui_version.as_deref().unwrap_or("0.29");
    let crossterm_ver = p.crossterm_version.as_deref().unwrap_or("0.28");

    let code = format!(
        r#"[package]
name = "{name}"
version = "{version}"
edition = "2021"

[dependencies]
ratatui = {{ version = "{ratatui_ver}", features = ["crossterm"] }}
crossterm = "{crossterm_ver}"
"#,
        name = p.name,
    );
    Ok(code_result(&code))
}

// ---------------------------------------------------------------------------
// emit_main_rs
// ---------------------------------------------------------------------------

/// Parameters for `emit_main_rs`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitMainRsParams {
    /// Application module name (default "app").
    #[serde(default)]
    pub app_module: Option<String>,
}

/// Generate main.rs with terminal setup and teardown.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "emit_main_rs",
    description = "Generate main.rs with crossterm terminal setup, app execution, and cleanup."
)]
#[instrument(skip_all)]
async fn emit_main_rs(p: EmitMainRsParams) -> Result<CallToolResult, ErrorData> {
    let app_mod = p.app_module.as_deref().unwrap_or("app");
    let code = format!(
        r#"mod {app_mod};

use crossterm::{{
    execute,
    terminal::{{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}},
}};
use ratatui::{{backend::CrosstermBackend, Terminal}};
use std::{{error::Error, io}};

fn main() -> Result<(), Box<dyn Error>> {{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = {app_mod}::run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {{
        eprintln!("Error: {{err:?}}");
    }}
    Ok(())
}}
"#
    );
    Ok(code_result(&code))
}

// ---------------------------------------------------------------------------
// emit_app_struct
// ---------------------------------------------------------------------------

/// Parameters for `emit_app_struct`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitAppStructParams {
    /// Struct name (e.g. "App").
    pub name: String,
    /// Struct fields.
    pub fields: Vec<AppFieldJson>,
}

/// Generate an application state struct with a `new()` constructor.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "emit_app_struct",
    description = "Generate a Rust struct for application state with fields and a new() constructor."
)]
#[instrument(skip_all)]
async fn emit_app_struct(p: EmitAppStructParams) -> Result<CallToolResult, ErrorData> {
    let mut field_defs = String::new();
    let mut field_inits = String::new();

    for f in &p.fields {
        field_defs.push_str(&format!("    pub {}: {},\n", f.name, f.field_type));
        let default = f
            .default
            .as_deref()
            .unwrap_or_else(|| default_for_type(&f.field_type));
        field_inits.push_str(&format!("            {}: {},\n", f.name, default));
    }

    let code = format!(
        r#"/// Application state.
pub struct {name} {{
{field_defs}}}

impl {name} {{
    /// Create a new instance with default values.
    pub fn new() -> Self {{
        Self {{
{field_inits}        }}
    }}
}}
"#,
        name = p.name,
    );
    Ok(code_result(&code))
}

/// Return a sensible default expression for common Rust types.
fn default_for_type(ty: &str) -> &str {
    match ty {
        "bool" => "false",
        "usize" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "isize" => "0",
        "f32" | "f64" => "0.0",
        "String" => "String::new()",
        _ if ty.starts_with("Vec") => "Vec::new()",
        _ if ty.starts_with("Option") => "None",
        _ => "Default::default()",
    }
}

// ---------------------------------------------------------------------------
// emit_draw_fn
// ---------------------------------------------------------------------------

/// Parameters for `emit_draw_fn`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitDrawFnParams {
    /// Root TUI node tree to render.
    pub root: TuiNode,
}

/// Generate a draw function that renders a `TuiNode` tree.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "emit_draw_fn",
    description = "Generate a draw function from a TuiNode widget/layout tree."
)]
#[instrument(skip_all)]
async fn emit_draw_fn(p: EmitDrawFnParams) -> Result<CallToolResult, ErrorData> {
    let body = tui_node_to_code(&p.root, 1);
    let code = format!(
        r#"use ratatui::{{
    Frame,
    layout::{{Constraint, Direction, Layout, Rect}},
    widgets::{{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs}},
}};

/// Draw the UI into the given frame.
pub fn draw(frame: &mut Frame) {{
    let area = frame.area();
{body}}}
"#,
    );
    Ok(code_result(&code))
}

/// Recursively convert a `TuiNode` tree into rendering code.
fn tui_node_to_code(node: &TuiNode, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    match node {
        TuiNode::Widget { widget, .. } => widget_to_code(widget, indent),
        TuiNode::Layout {
            direction,
            constraints,
            children,
            margin,
        } => {
            let dir = match direction {
                DirectionJson::Vertical => "Direction::Vertical",
                DirectionJson::Horizontal => "Direction::Horizontal",
            };
            let constraints_code: Vec<String> =
                constraints.iter().map(constraint_to_code).collect();
            let mut code = format!("{pad}let chunks = Layout::default()\n");
            code += &format!("{pad}    .direction({dir})\n");
            code += &format!("{pad}    .constraints([{}])\n", constraints_code.join(", "));
            if let Some(m) = margin {
                code += &format!("{pad}    .horizontal_margin({})\n", m.horizontal);
                code += &format!("{pad}    .vertical_margin({})\n", m.vertical);
            }
            code += &format!("{pad}    .split(area);\n");
            for (i, child) in children.iter().enumerate() {
                code += &format!("{pad}// chunk {i}\n");
                code += &format!("{pad}{{\n");
                code += &format!("{pad}    let area = chunks[{i}];\n");
                code += &tui_node_to_code(child, indent + 1);
                code += &format!("{pad}}}\n");
            }
            code
        }
        TuiNode::StatusBar { chips, theme } => {
            let chips_code = chips
                .iter()
                .map(|(k, a)| format!("({:?}.to_string(), {:?}.to_string())", k, a))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "{pad}render_status_bar(frame, area, &[{chips_code}], ColorTheme::{theme:?});\n"
            )
        }
    }
}

/// Convert a single `WidgetJson` variant into rendering code.
fn widget_to_code(widget: &WidgetJson, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    match widget {
        WidgetJson::Paragraph {
            text, block, wrap, ..
        } => {
            let mut code = format!("{pad}let widget = Paragraph::new({text:?})");
            if let Some(b) = block {
                code += &format!("\n{pad}    .block({})", block_to_code(b));
            }
            if *wrap {
                code += &format!("\n{pad}    .wrap(ratatui::widgets::Wrap {{ trim: true }})");
            }
            code += ";\n";
            code += &format!("{pad}frame.render_widget(widget, area);\n");
            code
        }
        WidgetJson::List {
            items,
            block,
            highlight_symbol,
            ..
        } => {
            let items_code: Vec<String> = items.iter().map(|i| format!("{i:?}")).collect();
            let mut code = format!(
                "{pad}let items: Vec<ListItem> = vec![{}]\n{pad}    .into_iter().map(ListItem::new).collect();\n",
                items_code.join(", ")
            );
            code += &format!("{pad}let widget = List::new(items)");
            if let Some(b) = block {
                code += &format!("\n{pad}    .block({})", block_to_code(b));
            }
            if let Some(sym) = highlight_symbol {
                code += &format!("\n{pad}    .highlight_symbol({sym:?})");
            }
            code += ";\n";
            code += &format!("{pad}frame.render_widget(widget, area);\n");
            code
        }
        WidgetJson::Gauge {
            ratio,
            label,
            block,
            ..
        } => {
            let mut code = format!("{pad}let widget = Gauge::default()\n{pad}    .ratio({ratio})");
            if let Some(lbl) = label {
                code += &format!("\n{pad}    .label({lbl:?})");
            }
            if let Some(b) = block {
                code += &format!("\n{pad}    .block({})", block_to_code(b));
            }
            code += ";\n";
            code += &format!("{pad}frame.render_widget(widget, area);\n");
            code
        }
        WidgetJson::Tabs {
            titles,
            selected,
            block,
            ..
        } => {
            let titles_code: Vec<String> = titles.iter().map(|t| format!("{t:?}")).collect();
            let mut code = format!(
                "{pad}let widget = Tabs::new(vec![{}])",
                titles_code.join(", ")
            );
            if let Some(sel) = selected {
                code += &format!("\n{pad}    .select({sel})");
            }
            if let Some(b) = block {
                code += &format!("\n{pad}    .block({})", block_to_code(b));
            }
            code += ";\n";
            code += &format!("{pad}frame.render_widget(widget, area);\n");
            code
        }
        WidgetJson::Block { block } => {
            let mut code = format!("{pad}let widget = {};\n", block_to_code(block));
            code += &format!("{pad}frame.render_widget(widget, area);\n");
            code
        }
        WidgetJson::Clear => {
            format!("{pad}frame.render_widget(ratatui::widgets::Clear, area);\n")
        }
        _ => {
            // Fallback: emit a placeholder comment for complex widgets
            format!("{pad}// TODO: render {:?} widget\n", widget_tag(widget))
        }
    }
}

/// Return a short tag name for a `WidgetJson` variant.
fn widget_tag(w: &WidgetJson) -> &'static str {
    match w {
        WidgetJson::Block { .. } => "Block",
        WidgetJson::Paragraph { .. } => "Paragraph",
        WidgetJson::List { .. } => "List",
        WidgetJson::Table { .. } => "Table",
        WidgetJson::Gauge { .. } => "Gauge",
        WidgetJson::Sparkline { .. } => "Sparkline",
        WidgetJson::Tabs { .. } => "Tabs",
        WidgetJson::Clear => "Clear",
        WidgetJson::BarChart { .. } => "BarChart",
        WidgetJson::Chart { .. } => "Chart",
        WidgetJson::LineGauge { .. } => "LineGauge",
        WidgetJson::Scrollbar { .. } => "Scrollbar",
    }
}

/// Generate a `Block::default()` builder expression from `BlockJson`.
fn block_to_code(b: &crate::serde_types::BlockJson) -> String {
    let mut code = "Block::default().borders(Borders::ALL)".to_owned();
    if let Some(title) = &b.title {
        code += &format!(".title({title:?})");
    }
    code
}

/// Convert a `ConstraintJson` into a Rust expression string.
fn constraint_to_code(c: &ConstraintJson) -> String {
    match c {
        ConstraintJson::Length { value } => format!("Constraint::Length({value})"),
        ConstraintJson::Percentage { value } => format!("Constraint::Percentage({value})"),
        ConstraintJson::Min { value } => format!("Constraint::Min({value})"),
        ConstraintJson::Max { value } => format!("Constraint::Max({value})"),
        ConstraintJson::Fill { value } => format!("Constraint::Fill({value})"),
        ConstraintJson::Ratio { num, den } => format!("Constraint::Ratio({num}, {den})"),
    }
}

// ---------------------------------------------------------------------------
// emit_event_handler
// ---------------------------------------------------------------------------

/// Parameters for `emit_event_handler`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitEventHandlerParams {
    /// Quit key (default "q").
    #[serde(default)]
    pub quit_key: Option<String>,
    /// Additional key handlers.
    #[serde(default)]
    pub handlers: Vec<KeyHandlerJson>,
}

/// Generate a keyboard event handler function.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "emit_event_handler",
    description = "Generate an event handler function that maps keyboard input to app state changes."
)]
#[instrument(skip_all)]
async fn emit_event_handler(p: EmitEventHandlerParams) -> Result<CallToolResult, ErrorData> {
    let quit_key = p.quit_key.as_deref().unwrap_or("q");

    let mut match_arms = String::new();
    // Always generate the quit key arm
    match_arms.push_str(&format!(
        "            KeyCode::Char('{quit_key}') => return Ok(true),\n"
    ));
    for h in &p.handlers {
        let arm = key_to_match_arm(&h.key);
        match_arms.push_str(&format!(
            "            {arm} => {{ {action} }}\n",
            action = h.action
        ));
    }
    match_arms.push_str("            _ => {}\n");

    let code = format!(
        r#"use crossterm::event::{{self, Event, KeyCode}};
use std::io;

/// Handle keyboard events. Returns `true` when the app should quit.
pub fn handle_events(app: &mut App) -> io::Result<bool> {{
    if event::poll(std::time::Duration::from_millis(50))? {{
        if let Event::Key(key) = event::read()? {{
            if key.kind == crossterm::event::KeyEventKind::Press {{
                match key.code {{
{match_arms}                }}
            }}
        }}
    }}
    Ok(false)
}}
"#,
    );
    Ok(code_result(&code))
}

/// Convert a key name string into a `KeyCode::*` match pattern.
fn key_to_match_arm(key: &str) -> String {
    match key {
        "Up" => "KeyCode::Up".to_owned(),
        "Down" => "KeyCode::Down".to_owned(),
        "Left" => "KeyCode::Left".to_owned(),
        "Right" => "KeyCode::Right".to_owned(),
        "Enter" => "KeyCode::Enter".to_owned(),
        "Esc" => "KeyCode::Esc".to_owned(),
        "Tab" => "KeyCode::Tab".to_owned(),
        "Backspace" => "KeyCode::Backspace".to_owned(),
        s if s.len() == 1 => format!("KeyCode::Char('{s}')"),
        other => format!("KeyCode::Char('{}')", other.chars().next().unwrap_or('?')),
    }
}

// ---------------------------------------------------------------------------
// emit_app_loop
// ---------------------------------------------------------------------------

/// Parameters for `emit_app_loop`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmitAppLoopParams {
    /// Application struct name (e.g. "App").
    pub app_name: String,
}

/// Generate the main application loop with draw and event handling.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "emit_app_loop",
    description = "Generate the main application loop that draws the UI and handles events each frame."
)]
#[instrument(skip_all)]
async fn emit_app_loop(p: EmitAppLoopParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"use ratatui::{{backend::CrosstermBackend, Terminal}};
use std::io;

/// Run the application loop.
pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut {app_name},
) -> io::Result<()> {{
    loop {{
        terminal.draw(|frame| draw(frame))?;
        if handle_events(app)? {{
            return Ok(());
        }}
    }}
}}
"#,
        app_name = p.app_name,
    );
    Ok(code_result(&code))
}

// ---------------------------------------------------------------------------
// assemble_ratatui_app
// ---------------------------------------------------------------------------

/// Parameters for `assemble_ratatui_app`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct AssembleRatatuiAppParams {
    /// Application/package name.
    pub name: String,
    /// Window title shown in terminal (default: name).
    #[serde(default)]
    pub title: Option<String>,
    /// Root TUI layout tree (generates draw body). If absent a placeholder is used.
    #[serde(default)]
    pub root: Option<TuiNode>,
    /// Quit key (default "q").
    #[serde(default)]
    pub quit_key: Option<String>,
}

/// Generate a complete ratatui application with all files.
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "assemble_ratatui_app",
    description = "Generate a complete ratatui application with Cargo.toml and src/main.rs."
)]
#[instrument(skip_all)]
async fn assemble_ratatui_app(p: AssembleRatatuiAppParams) -> Result<CallToolResult, ErrorData> {
    let title = p.title.as_deref().unwrap_or(&p.name);
    let quit_key = p.quit_key.as_deref().unwrap_or("q");

    let cargo_toml = format!(
        r#"[package]
name = "{name}"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = {{ version = "0.29", features = ["crossterm"] }}
crossterm = "0.28"
"#,
        name = p.name,
    );

    let draw_body = match &p.root {
        Some(node) => tui_node_to_code(node, 1),
        None => format!(
            r#"    let widget = Paragraph::new("{title}")
        .block(Block::default().borders(Borders::ALL).title("{title}"));
    frame.render_widget(widget, area);
"#,
        ),
    };

    let main_rs = format!(
        r#"use crossterm::{{
    event::{{self, Event, KeyCode}},
    execute,
    terminal::{{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}},
}};
use ratatui::{{
    backend::CrosstermBackend,
    layout::{{Constraint, Direction, Layout, Rect}},
    widgets::{{Block, Borders, Gauge, List, ListItem, Paragraph, Tabs}},
    Frame, Terminal,
}};
use std::{{error::Error, io}};

fn main() -> Result<(), Box<dyn Error>> {{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {{
        eprintln!("Error: {{err:?}}");
    }}
    Ok(())
}}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {{
    loop {{
        terminal.draw(|frame| draw(frame))?;
        if event::poll(std::time::Duration::from_millis(50))? {{
            if let Event::Key(key) = event::read()? {{
                if key.kind == crossterm::event::KeyEventKind::Press {{
                    if let KeyCode::Char('{quit_key}') = key.code {{
                        return Ok(());
                    }}
                }}
            }}
        }}
    }}
}}

fn draw(frame: &mut Frame) {{
    let area = frame.area();
{draw_body}}}
"#,
    );

    let json = serde_json::json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
    });
    match serde_json::to_string(&json) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "serialize error: {e}"
        ))])),
    }
}
