# elicit_ratatui — Implementation Plan

> **Premise:** Expose ratatui's terminal UI API as MCP tools for both runtime TUI execution and code generation.
> **Approach:** Completionist harvesting using dual-mode tools (primary), runtime-only tools (TUI display), and fragment tools (TUI app code gen).

---

## Why ratatui is Unique

**Unique characteristics:**
- **Terminal-based** (text/ANSI output, not graphical pixels)
- **Immediate mode** (like egui: widgets rendered every frame)
- **Backend abstraction** (crossterm/termion/termwiz for terminal control)
- **Constraint-based layout** (flexible responsive TUIs)
- **Low resource usage** (<20 MB RAM even for complex UIs)
- **SSH-friendly** (works over remote connections)

**Different from egui:**
- **Output target**: Terminal (80×24 chars) vs GUI (pixels)
- **Widgets**: TUI-specific (borders, tables, gauges) vs GUI (buttons, sliders)
- **Rendering**: ANSI escape codes vs GPU/canvas
- **Use cases**: CLI tools, remote admin vs desktop apps

**Similar to egui:**
- **Immediate mode** (rebuild UI every frame)
- **User-managed state** (app stores data between frames)
- **Synchronous operations** (no async in core rendering)
- **Response pattern** (check interaction state)

**Perfect fit for:**
- AI-driven TUI generation (create terminal dashboards, admin tools)
- Log viewers (monitoring, analysis)
- Database clients (query builders, data browsers)
- System monitors (htop-style, resource usage)
- Configuration editors (interactive setup wizards)
- SSH tools (remote administration panels)
- Development tools (test runners, build monitors)

---

## Core Constraint: Terminal Rendering + Backend Integration

ratatui's rendering pattern:

```rust
// Every frame:
terminal.draw(|frame| {
    let area = frame.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    frame.render_widget(Block::default().borders(Borders::ALL), chunks[0]);
    frame.render_widget(Paragraph::new("Content"), chunks[1]);
})?;
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Widget descriptions | `Block`, `Paragraph`, `Table` params | ✅ Dual-mode (serialize as JSON) |
| Layout constraints | `Length`, `Percentage`, `Min`, `Fill` | ✅ Dual-mode (serialize as JSON) |
| Styling | `Color`, `Style`, `Modifier` | ✅ Dual-mode (serialize as JSON) |
| Terminal state | `Terminal`, per-frame buffer | ⚠️ Runtime-only (backend integration) |
| Stateful widgets | `ListState`, `TableState`, scroll pos | ✅ Dual-mode (serialize state) |
| Backend | crossterm/termion/termwiz events | ⚠️ Runtime-only (terminal I/O) |
| Event loop | Key/mouse event handling | ⚠️ Runtime-only (backend-specific) |

**Key insight:** ratatui is **similar to egui** in dual nature:
- **Widget descriptions** serialize (table columns, chart data)
- **Runtime execution** requires terminal backend
- **Code generation** is primary use case (agents generate TUI apps)

This makes ratatui **dual-mode dominated** (85%+) for widget creation.

---

## Tool Breakdown: 380 Total

### Dual-Mode Tools (320)

Tools that both create widget/layout JSON AND generate code:

#### Core Widgets (30)
- `widget_block` — Bordered container with title
- `widget_paragraph` — Text display with wrapping
- `widget_list` — Selectable item list
- `widget_table` — Multi-column data grid
- `widget_chart` — Line/scatter chart
- `widget_bar_chart` — Grouped bars
- `widget_sparkline` — Compact line chart
- `widget_gauge` — Progress indicator
- `widget_line_gauge` — Linear progress bar
- `widget_tabs` — Horizontal tab selector
- `widget_scrollbar` — Scroll position indicator
- `widget_calendar` — Month calendar view
- `widget_clear` — Clear area
- And 17 more for widget variants

#### Block Properties (25)
- `block_borders` — Set border edges (ALL, TOP, LEFT, etc.)
- `block_border_type` — Set border style (Plain, Rounded, Double, Thick)
- `block_border_style` — Set border color/modifier
- `block_title` — Set title text
- `block_title_alignment` — Center/left/right title
- `block_title_position` — Top/bottom title
- `block_padding` — Set inner padding
- `block_style` — Set background style
- And 17 more for block properties

#### Paragraph Properties (20)
- `paragraph_text` — Set display text
- `paragraph_style` — Set text style
- `paragraph_wrap` — Enable text wrapping
- `paragraph_wrap_trim` — Trim wrapped lines
- `paragraph_scroll` — Set scroll offset
- `paragraph_alignment` — Left/center/right align
- `paragraph_line_spacing` — Set spacing between lines
- And 13 more for paragraph properties

#### List Properties (25)
- `list_items` — Set list items
- `list_block` — Set container block
- `list_style` — Set item style
- `list_highlight_style` — Set selection style
- `list_highlight_symbol` — Set selection marker
- `list_highlight_spacing` — Spacing for highlight
- `list_repeat_highlight_symbol` — Repeat on all lines
- `list_direction` — TopToBottom/BottomToTop
- `list_state_select` — Set selected index
- `list_state_offset` — Set scroll offset
- And 15 more for list properties

#### Table Properties (30)
- `table_rows` — Set table rows
- `table_block` — Set container block
- `table_style` — Set table style
- `table_header` — Set header row
- `table_widths` — Set column widths
- `table_column_spacing` — Set gap between columns
- `table_highlight_style` — Set selection style
- `table_highlight_symbol` — Set selection marker
- `table_highlight_spacing` — Spacing for highlight
- `table_state_select` — Set selected row
- `table_state_offset` — Set scroll offset
- `row_cells` — Create row from cells
- `row_height` — Set row height
- `cell_content` — Set cell text
- `cell_style` — Set cell style
- And 15 more for table properties

#### Chart Properties (35)
- `chart_datasets` — Set chart datasets
- `chart_block` — Set container block
- `chart_x_axis` — Set X axis
- `chart_y_axis` — Set Y axis
- `chart_style` — Set chart style
- `chart_legend_position` — Set legend placement
- `dataset_name` — Set dataset name
- `dataset_marker` — Set point marker (Dot, Braille, Bar)
- `dataset_graph_type` — Line or Scatter
- `dataset_style` — Set line/point style
- `dataset_data` — Set data points
- `axis_title` — Set axis title
- `axis_style` — Set axis style
- `axis_bounds` — Set min/max bounds
- `axis_labels` — Set axis labels
- And 20 more for chart/axis properties

#### Gauge Properties (20)
- `gauge_block` — Set container block
- `gauge_ratio` — Set progress (0.0-1.0)
- `gauge_label` — Set label text
- `gauge_style` — Set gauge style
- `gauge_gauge_style` — Set filled portion style
- `gauge_line_set` — Set line characters
- `line_gauge_ratio` — Set progress
- `line_gauge_label` — Set label
- `line_gauge_style` — Set style
- `line_gauge_line_set` — Set line characters
- And 10 more for gauge properties

#### BarChart Properties (20)
- `bar_chart_data` — Set bar data
- `bar_chart_block` — Set container block
- `bar_chart_max` — Set max value
- `bar_chart_bar_width` — Set bar width
- `bar_chart_bar_gap` — Set gap between bars
- `bar_chart_bar_style` — Set bar style
- `bar_chart_value_style` — Set value text style
- `bar_chart_label_style` — Set label style
- `bar_chart_bar_set` — Set bar characters
- `bar_chart_direction` — Horizontal/Vertical
- And 10 more for bar chart properties

#### Sparkline Properties (15)
- `sparkline_data` — Set data points
- `sparkline_block` — Set container block
- `sparkline_style` — Set sparkline style
- `sparkline_max` — Set max value
- `sparkline_direction` — LeftToRight/RightToLeft
- And 10 more for sparkline properties

#### Tabs Properties (15)
- `tabs_titles` — Set tab titles
- `tabs_block` — Set container block
- `tabs_style` — Set tab style
- `tabs_highlight_style` — Set selected style
- `tabs_select` — Set selected index
- `tabs_divider` — Set divider character
- `tabs_padding` — Set tab padding
- And 8 more for tabs properties

#### Scrollbar Properties (15)
- `scrollbar_orientation` — Vertical/Horizontal
- `scrollbar_style` — Set scrollbar style
- `scrollbar_begin_symbol` — Set start character
- `scrollbar_end_symbol` — Set end character
- `scrollbar_track_symbol` — Set track character
- `scrollbar_thumb_symbol` — Set thumb character
- `scrollbar_state_content_length` — Set total length
- `scrollbar_state_position` — Set current position
- And 7 more for scrollbar properties

#### Calendar Properties (15)
- `calendar_date` — Set displayed date
- `calendar_block` — Set container block
- `calendar_style` — Set calendar style
- `calendar_default_style` — Set date style
- `calendar_show_month_header` — Toggle header
- `calendar_show_weekdays_header` — Toggle weekdays
- And 9 more for calendar properties

#### Layout (30)
- `layout_vertical` — Vertical split
- `layout_horizontal` — Horizontal split
- `layout_constraints` — Set constraint list
- `layout_margin` — Set outer margin
- `layout_flex` — Set flex mode (Start, Center, End, etc.)
- `constraint_length` — Fixed length
- `constraint_percentage` — Percentage of space
- `constraint_ratio` — Ratio (numerator, denominator)
- `constraint_min` — Minimum length
- `constraint_max` — Maximum length
- `constraint_fill` — Fill remaining space
- And 19 more for layout/constraint options

#### Styling (30)
- `style_fg` — Set foreground color
- `style_bg` — Set background color
- `style_add_modifier` — Add text modifier (Bold, Italic, etc.)
- `style_remove_modifier` — Remove modifier
- `style_underline_color` — Set underline color (feature-gated)
- `color_rgb` — RGB color (feature-gated)
- `color_indexed` — 256-color palette
- `color_named` — Named colors (Black, Red, Green, etc.)
- `color_reset` — Reset to default
- `modifier_bold` — Bold modifier
- `modifier_italic` — Italic modifier
- `modifier_underlined` — Underline modifier
- `modifier_dim` — Dim modifier
- `modifier_reversed` — Reverse video
- `modifier_crossed_out` — Strikethrough
- `modifier_slow_blink` — Slow blink
- `modifier_rapid_blink` — Rapid blink
- And 13 more for styling options

#### Text Types (20)
- `text_raw` — Plain string
- `text_styled` — Styled text
- `text_from_spans` — Text from span list
- `text_from_lines` — Text from line list
- `span_raw` — Plain span
- `span_styled` — Styled span
- `line_from_spans` — Line from spans
- `line_styled` — Styled line
- And 12 more for text composition

### Runtime-Only Tools (40)

Terminal integration and stateful TUI execution:

#### Terminal Management (15)
- `terminal_create` — Create Terminal with backend
- `terminal_delete` — Remove terminal
- `terminal_draw` — Draw frame
- `terminal_clear` — Clear screen
- `terminal_hide_cursor` — Hide cursor
- `terminal_show_cursor` — Show cursor
- `terminal_set_cursor` — Set cursor position
- `terminal_size` — Get terminal dimensions
- `terminal_autoresize` — Enable auto resize handling
- And 6 more for terminal operations

#### App State Registry (15)
- `app_state_create` — Create persistent TUI app state
- `app_state_get` — Get state by handle
- `app_state_set` — Update state
- `app_state_delete` — Remove state
- `app_run_frame` — Run TUI with state
- `list_state_create` — Create ListState
- `table_state_create` — Create TableState
- `scrollbar_state_create` — Create ScrollbarState
- And 7 more for state management

#### Event Handling (10)
- `event_read` — Read terminal event
- `event_poll` — Poll for events (timeout)
- `event_key_pressed` — Check key press
- `event_mouse_moved` — Check mouse movement
- `event_resize` — Check terminal resize
- And 5 more for event handling

### Fragment Tools (20)

Code generation for complete TUI applications:

#### Widget Code Generation (10)
- `emit_block` — Generate Block code
- `emit_paragraph` — Generate Paragraph code
- `emit_list` — Generate List code
- `emit_table` — Generate Table code
- `emit_chart` — Generate Chart code
- And 5 more for widget code

#### Complete App Assembly (10)
- `assemble_ratatui_app` — Generate complete TUI app
- `emit_app_struct` — Generate app state struct
- `emit_draw_method` — Generate draw() method
- `emit_event_handler` — Generate event handling
- `emit_main_loop` — Generate main event loop
- And 5 more for app assembly

---

## Serialization Strategy

### Widget JSON

```json
{
  "type": "Block",
  "borders": "ALL",
  "border_type": "Rounded",
  "title": "Log Viewer",
  "title_alignment": "Center",
  "padding": {
    "horizontal": 1,
    "vertical": 1
  },
  "style": {
    "fg": "White",
    "bg": "Black"
  }
}
```

### Table JSON

```json
{
  "type": "Table",
  "block": {
    "borders": "ALL",
    "title": "Users"
  },
  "header": {
    "cells": ["ID", "Name", "Email"],
    "style": { "fg": "Yellow", "modifier": "BOLD" }
  },
  "rows": [
    { "cells": ["1", "Alice", "alice@example.com"] },
    { "cells": ["2", "Bob", "bob@example.com"] }
  ],
  "widths": [
    { "type": "Length", "value": 5 },
    { "type": "Percentage", "value": 30 },
    { "type": "Fill", "value": 1 }
  ],
  "column_spacing": 1,
  "highlight_style": { "fg": "Black", "bg": "Cyan" },
  "highlight_symbol": ">> "
}
```

### Chart JSON

```json
{
  "type": "Chart",
  "block": {
    "borders": "ALL",
    "title": "CPU Usage"
  },
  "datasets": [
    {
      "name": "CPU 1",
      "marker": "Dot",
      "graph_type": "Line",
      "style": { "fg": "Cyan" },
      "data": [
        [0.0, 10.0],
        [1.0, 20.0],
        [2.0, 15.0],
        [3.0, 30.0]
      ]
    }
  ],
  "x_axis": {
    "title": "Time (s)",
    "bounds": [0.0, 10.0],
    "labels": ["0", "5", "10"]
  },
  "y_axis": {
    "title": "Usage (%)",
    "bounds": [0.0, 100.0],
    "labels": ["0", "50", "100"]
  }
}
```

### Layout JSON

```json
{
  "direction": "Vertical",
  "constraints": [
    { "type": "Length", "value": 3 },
    { "type": "Min", "value": 0 },
    { "type": "Length", "value": 3 }
  ],
  "margin": {
    "horizontal": 1,
    "vertical": 1
  }
}
```

### TUI Tree (Declarative Representation)

```json
{
  "terminal_size": { "width": 80, "height": 24 },
  "layout": {
    "direction": "Vertical",
    "constraints": [
      { "type": "Length", "value": 3 },
      { "type": "Min", "value": 0 },
      { "type": "Length", "value": 3 }
    ],
    "areas": [
      {
        "widget": {
          "type": "Block",
          "borders": "ALL",
          "title": "Header"
        }
      },
      {
        "layout": {
          "direction": "Horizontal",
          "constraints": [
            { "type": "Percentage", "value": 30 },
            { "type": "Percentage", "value": 70 }
          ],
          "areas": [
            {
              "widget": {
                "type": "List",
                "items": ["Item 1", "Item 2", "Item 3"],
                "highlight_symbol": ">> "
              }
            },
            {
              "widget": {
                "type": "Paragraph",
                "text": "Content area"
              }
            }
          ]
        }
      },
      {
        "widget": {
          "type": "Block",
          "borders": "ALL",
          "title": "Status"
        }
      }
    ]
  }
}
```

---

## Phase 1: Widget Dual-Mode Tools

**Goal:** Establish dual-mode pattern for widget creation.

### Crate Structure

```
crates/elicit_ratatui/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── widget_tools.rs      # Dual-mode widget creation
    ├── layout_tools.rs      # Dual-mode layout
    ├── style_tools.rs       # Dual-mode styling
    ├── terminal_tools.rs    # Runtime terminal management
    ├── app_registry.rs      # Runtime app state
    ├── fragments.rs         # Code generation
    └── serde_types.rs       # JSON wrappers
```

### Cargo.toml

```toml
[package]
name = "elicit_ratatui"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
ratatui = { version = "0.30", default-features = false }
crossterm = { version = "0.28", optional = true }
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
runtime = ["dep:crossterm", "ratatui/crossterm"]  # Enable runtime TUI display
termion = ["ratatui/termion"]  # Alternative backend
termwiz = ["ratatui/termwiz"]  # Alternative backend
```

### Dual-Mode Tool Example: Block

```rust
use elicitation_derive::elicit_tool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockParams {
    pub borders: String,  // "ALL", "TOP", "LEFT", etc.
    pub border_type: Option<String>,  // "Plain", "Rounded", "Double"
    pub title: Option<String>,
    pub title_alignment: Option<String>,  // "Left", "Center", "Right"
    pub padding: Option<PaddingJson>,
    pub style: Option<StyleJson>,
}

#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_block",
    description = "Create bordered container with optional title",
    emit = Auto
)]
async fn widget_block(p: BlockParams) -> Result<CallToolResult, ErrorData> {
    let widget_json = WidgetJson::Block {
        borders: p.borders.clone(),
        border_type: p.border_type.clone(),
        title: p.title.clone(),
        title_alignment: p.title_alignment.clone(),
        padding: p.padding.clone(),
        style: p.style.clone(),
    };

    Ok(CallToolResult::success(json!({ "widget": widget_json })))
}

impl CustomEmit<BlockParams> for WidgetBlockEmit {
    fn emit_code(params: &BlockParams) -> TokenStream {
        let borders = parse_borders(&params.borders);
        let mut builder_calls = vec![];

        if let Some(ref border_type) = params.border_type {
            let bt = parse_border_type(border_type);
            builder_calls.push(quote! { .border_type(#bt) });
        }
        if let Some(ref title) = params.title {
            builder_calls.push(quote! { .title(#title) });
        }
        if let Some(ref align) = params.title_alignment {
            let alignment = parse_alignment(align);
            builder_calls.push(quote! { .title_alignment(#alignment) });
        }

        quote! {
            Block::default()
                .borders(#borders)
                #(#builder_calls)*
        }
    }
}
```

### WidgetJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WidgetJson {
    Block {
        borders: String,
        border_type: Option<String>,
        title: Option<String>,
        title_alignment: Option<String>,
        padding: Option<PaddingJson>,
        style: Option<StyleJson>,
    },
    Paragraph {
        text: String,
        style: Option<StyleJson>,
        wrap: Option<bool>,
        scroll: Option<(u16, u16)>,
        alignment: Option<String>,
    },
    List {
        items: Vec<String>,
        block: Option<Box<WidgetJson>>,
        style: Option<StyleJson>,
        highlight_style: Option<StyleJson>,
        highlight_symbol: Option<String>,
        state: Option<ListStateJson>,
    },
    Table {
        header: Option<RowJson>,
        rows: Vec<RowJson>,
        block: Option<Box<WidgetJson>>,
        widths: Vec<ConstraintJson>,
        column_spacing: Option<u16>,
        highlight_style: Option<StyleJson>,
        state: Option<TableStateJson>,
    },
    // ... and 10+ more variants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleJson {
    pub fg: Option<String>,
    pub bg: Option<String>,
    pub modifiers: Vec<String>,
    pub underline_color: Option<String>,
}
```

### Dual-Mode Tool Example: Table

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableParams {
    pub header: Option<RowJson>,
    pub rows: Vec<RowJson>,
    pub block: Option<BlockParams>,
    pub widths: Vec<ConstraintJson>,
    pub column_spacing: Option<u16>,
    pub highlight_style: Option<StyleJson>,
}

#[elicit_tool(
    plugin = "ratatui_widgets",
    name = "widget_table",
    description = "Create multi-column data table",
    emit = Auto
)]
async fn widget_table(p: TableParams) -> Result<CallToolResult, ErrorData> {
    let widget_json = WidgetJson::Table {
        header: p.header.clone(),
        rows: p.rows.clone(),
        block: p.block.as_ref().map(|b| Box::new(block_to_json(b))),
        widths: p.widths.clone(),
        column_spacing: p.column_spacing,
        highlight_style: p.highlight_style.clone(),
        state: None,
    };

    Ok(CallToolResult::success(json!({ "widget": widget_json })))
}

impl CustomEmit<TableParams> for WidgetTableEmit {
    fn emit_code(params: &TableParams) -> TokenStream {
        let rows_code = params.rows.iter().map(emit_row);
        let widths_code = params.widths.iter().map(emit_constraint);

        let mut builder_calls = vec![];
        if let Some(ref header) = params.header {
            let header_code = emit_row(header);
            builder_calls.push(quote! { .header(#header_code) });
        }
        if let Some(ref block) = params.block {
            let block_code = emit_block(block);
            builder_calls.push(quote! { .block(#block_code) });
        }
        if let Some(spacing) = params.column_spacing {
            builder_calls.push(quote! { .column_spacing(#spacing) });
        }

        quote! {
            Table::new(vec![#(#rows_code),*], vec![#(#widths_code),*])
                #(#builder_calls)*
        }
    }
}
```

---

## Phase 2: Layout Dual-Mode Tools

**Goal:** Constraint-based responsive layouts.

### Dual-Mode Tool Example: Layout

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutParams {
    pub direction: String,  // "Vertical", "Horizontal"
    pub constraints: Vec<ConstraintJson>,
    pub margin: Option<MarginJson>,
    pub flex: Option<String>,  // "Start", "Center", "End", etc.
}

#[elicit_tool(
    plugin = "ratatui_layout",
    name = "layout_vertical",
    description = "Create vertical layout split",
    emit = Auto
)]
async fn layout_vertical(p: LayoutParams) -> Result<CallToolResult, ErrorData> {
    let layout_json = LayoutJson {
        direction: "Vertical".to_string(),
        constraints: p.constraints.clone(),
        margin: p.margin.clone(),
        flex: p.flex.clone(),
    };

    Ok(CallToolResult::success(json!({ "layout": layout_json })))
}

impl CustomEmit<LayoutParams> for LayoutVerticalEmit {
    fn emit_code(params: &LayoutParams) -> TokenStream {
        let constraints = params.constraints.iter().map(emit_constraint);
        let mut builder_calls = vec![];

        if let Some(ref margin) = params.margin {
            let m = margin.value;
            builder_calls.push(quote! { .margin(#m) });
        }

        quote! {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![#(#constraints),*])
                #(#builder_calls)*
                .split(area)
        }
    }
}
```

---

## Phase 3: Runtime Terminal Management

**Goal:** Actually run TUI apps (requires backend).

### Runtime Tool Example: Create Terminal

```rust
#[cfg(feature = "runtime")]
pub struct RatatuiPlugin {
    terminals: Arc<Mutex<HashMap<Uuid, Terminal<CrosstermBackend<std::io::Stdout>>>>>,
    app_states: Arc<Mutex<HashMap<Uuid, Box<dyn TuiAppState>>>>,
}

#[cfg(feature = "runtime")]
trait TuiAppState: Send + Sync {
    fn draw(&mut self, frame: &mut Frame);
    fn handle_event(&mut self, event: &crossterm::event::Event) -> bool;  // Returns true to quit
}

#[cfg(feature = "runtime")]
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_create",
    description = "Create Terminal with crossterm backend"
)]
async fn terminal_create(_: TerminalCreateParams) -> Result<CallToolResult, ErrorData> {
    use crossterm::{execute, terminal::{enable_raw_mode, EnterAlternateScreen}};
    use std::io;

    enable_raw_mode().map_err(|e| ErrorData::new(e.to_string()))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e| ErrorData::new(e.to_string()))?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).map_err(|e| ErrorData::new(e.to_string()))?;

    let terminal_id = Uuid::new_v4();
    let plugin = get_plugin();
    plugin.terminals.lock().unwrap().insert(terminal_id, terminal);

    Ok(CallToolResult::success(json!({ "terminal_id": terminal_id })))
}

#[cfg(feature = "runtime")]
#[elicit_tool(
    plugin = "ratatui_terminal",
    name = "terminal_draw_frame",
    description = "Draw one TUI frame"
)]
async fn terminal_draw_frame(p: DrawFrameParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut terminals = plugin.terminals.lock().unwrap();
    let terminal = terminals.get_mut(&p.terminal_id)
        .ok_or_else(|| ErrorData::new("Terminal not found"))?;

    // Build TUI from widget JSON
    let ui_tree = p.ui_tree;
    terminal.draw(|frame| {
        render_ui_tree(frame, &ui_tree);
    }).map_err(|e| ErrorData::new(e.to_string()))?;

    Ok(CallToolResult::success(json!({ "success": true })))
}
```

---

## Phase 4: Fragment Tools (Complete TUI Apps)

**Goal:** Generate complete ratatui/crossterm applications.

### Fragment Tool Example: TUI App

```rust
#[elicit_tool(
    plugin = "ratatui_fragments",
    name = "assemble_ratatui_app",
    description = "Generate complete ratatui TUI application",
    emit = Auto
)]
async fn assemble_ratatui_app(p: AssembleTuiParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml_tui(&p);
    let main_rs = generate_main_tui(&p);
    let app_rs = generate_app_struct_tui(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "src/app.rs": app_rs,
        "description": "Complete ratatui TUI app"
    })))
}

fn generate_main_tui(p: &AssembleTuiParams) -> String {
    format!(
        r#"mod app;

use crossterm::{{
    event::{{self, Event, KeyCode}},
    execute,
    terminal::{{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}},
}};
use ratatui::{{backend::CrosstermBackend, Terminal}};
use std::{{error::Error, io}};

fn main() -> Result<(), Box<dyn Error>> {{
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = app::{}::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {{
        println!("Error: {{:?}}", err);
    }}

    Ok(())
}}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut app::{},
) -> io::Result<()> {{
    loop {{
        terminal.draw(|f| app.draw(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {{
            if let Event::Key(key) = event::read()? {{
                match key.code {{
                    KeyCode::Char('q') => return Ok(()),
                    _ => app.handle_key(key),
                }}
            }}
        }}
    }}
}}
"#,
        p.app_struct_name,
        p.app_struct_name
    )
}

fn generate_app_struct_tui(p: &AssembleTuiParams) -> String {
    format!(
        r#"use ratatui::{{
    backend::Backend,
    layout::{{Constraint, Direction, Layout}},
    style::{{Color, Modifier, Style}},
    widgets::{{Block, Borders, List, ListItem, Paragraph}},
    Frame,
}};
use crossterm::event::{{KeyCode, KeyEvent}};

pub struct {} {{
    {}
}}

impl {} {{
    pub fn new() -> Self {{
        Self {{
            {}
        }}
    }}

    pub fn draw<B: Backend>(&mut self, frame: &mut Frame<B>) {{
        {}
    }}

    pub fn handle_key(&mut self, key: KeyEvent) {{
        {}
    }}
}}
"#,
        p.app_struct_name,
        emit_struct_fields_tui(&p.state_fields),
        p.app_struct_name,
        emit_field_init_tui(&p.state_fields),
        emit_draw_code_tui(&p.ui_tree),
        emit_event_handler_tui(&p.key_handlers)
    )
}

fn emit_draw_code_tui(ui_tree: &UiTreeJson) -> String {
    format!(
        r#"let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        // Render widgets
        {}
"#,
        emit_widget_renders_tui(ui_tree)
    )
}
```

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Core widget dual-mode tools: Block, Paragraph (30 tools)
3. **Phase 1c** — Block properties dual-mode tools: (25 tools)
4. **Phase 1d** — `just check elicit_ratatui`
5. **Phase 2a** — Paragraph properties dual-mode tools: (20 tools)
6. **Phase 2b** — List dual-mode tools: (25 tools)
7. **Phase 2c** — Table dual-mode tools: (30 tools)
8. **Phase 2d** — `just check elicit_ratatui`
9. **Phase 3a** — Chart dual-mode tools: (35 tools)
10. **Phase 3b** — Gauge/BarChart/Sparkline dual-mode tools: (55 tools)
11. **Phase 3c** — Tabs/Scrollbar/Calendar dual-mode tools: (45 tools)
12. **Phase 3d** — `just check elicit_ratatui`
13. **Phase 4a** — Layout dual-mode tools: (30 tools)
14. **Phase 4b** — Styling dual-mode tools: (30 tools)
15. **Phase 4c** — Text types dual-mode tools: (20 tools)
16. **Phase 4d** — `just check elicit_ratatui`
17. **Phase 5a** — Runtime terminal management: (15 tools) — requires `runtime` feature
18. **Phase 5b** — Runtime app state registry: (15 tools) — requires `runtime` feature
19. **Phase 5c** — Runtime event handling: (10 tools) — requires `runtime` feature
20. **Phase 5d** — `just check elicit_ratatui --features runtime`
21. **Phase 6a** — Fragment widget code generation: (10 tools)
22. **Phase 6b** — Fragment complete app assembly: (10 tools)
23. **Phase 6c** — `just check elicit_ratatui --all-features`
24. **Phase 7** — Wire into `elicit_server` emit chain

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Dual-Mode Core Widgets | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Block Properties | 25 | `emit = Auto` + CustomEmit |
| Dual-Mode Paragraph Properties | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode List Properties | 25 | `emit = Auto` + CustomEmit |
| Dual-Mode Table Properties | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Chart Properties | 35 | `emit = Auto` + CustomEmit |
| Dual-Mode Gauge Properties | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode BarChart Properties | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Sparkline Properties | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Tabs Properties | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Scrollbar Properties | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Calendar Properties | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Layout | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Styling | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Text Types | 20 | `emit = Auto` + CustomEmit |
| Runtime Terminal Management | 15 | Backend integration (feature-gated) |
| Runtime App State | 15 | UUID → AppState mapping (feature-gated) |
| Runtime Event Handling | 10 | crossterm events (feature-gated) |
| Fragment Widget Code | 10 | Code generation only |
| Fragment App Assembly | 10 | Code generation only |
| **Total** | **380** | |

---

## Key Advantages

1. **Terminal-Based**: SSH-friendly, low bandwidth, remote admin
2. **Immediate Mode**: Simple mental model (like egui)
3. **Cross-Platform Terminals**: Linux, macOS, Windows, WSL
4. **Low Resources**: <20 MB RAM even for complex TUIs
5. **Backend Abstraction**: crossterm (default), termion, termwiz
6. **Rich Widgets**: Tables, charts, gauges, calendars out of the box
7. **Constraint Layout**: Responsive TUIs that adapt to terminal size
8. **AI-Friendly**: Imperative API suits code generation

---

## Comparison to Other Shadow Crates

| Aspect | egui | ratatui |
|--------|------|---------|
| **Output** | GUI (pixels) | Terminal (chars) |
| **Rendering** | GPU/Canvas | ANSI escape codes |
| **Mode** | Immediate | Immediate |
| **Widgets** | Buttons, sliders, color pickers | Blocks, tables, charts, gauges |
| **Use case** | Desktop apps, tools | CLI tools, SSH, remote admin |
| **Resources** | Graphical | Text-only (~20 MB RAM) |
| **Dual-mode %** | 81% | 84% |
| **Runtime %** | 12% | 11% |
| **Total tools** | 420 | 380 |

**Both share immediate mode DNA:**
- Stateless widgets (rebuild every frame)
- User-managed state
- Response/interaction checking
- High dual-mode ratio

**Different output targets:**
- egui: Graphical (native windows, web canvas)
- ratatui: Terminal (80×24 text grid, ANSI colors)

---

## Use Cases

### AI-Generated TUIs

Agents generate terminal UIs from natural language:

```json
{
  "intent": "Create log viewer with filter sidebar",
  "tui": {
    "layout": {
      "direction": "Horizontal",
      "constraints": [
        { "type": "Percentage", "value": 30 },
        { "type": "Percentage", "value": 70 }
      ]
    },
    "widgets": [
      {
        "type": "List",
        "title": "Filters",
        "items": ["ERROR", "WARNING", "INFO"],
        "highlight_symbol": ">> "
      },
      {
        "type": "Paragraph",
        "title": "Logs",
        "text": "Log content here...",
        "scroll": true
      }
    ]
  }
}
```

Agent generates complete TUI app from this JSON.

### System Monitors

htop-style resource monitors:
- CPU/memory usage charts
- Process tables
- Real-time updates
- Sortable columns

### Database Clients

Terminal database browsers:
- Table list sidebar
- Query editor
- Result table
- Row inspector

### Log Viewers

Tail -f with filters:
- Log level filtering
- Search/highlight
- Time-based navigation
- Export to file

### Development Tools

Build monitors, test runners:
- Test results table
- Progress bars
- Color-coded status
- Expandable details

---

## Integration with Terminal Ecosystem

### SSH Workflow

ratatui TUIs work over SSH:
1. SSH into remote server
2. Run TUI app (no X11 needed)
3. Full UI in terminal
4. Low bandwidth usage

### tmux/screen Compatible

Works inside terminal multiplexers:
- Multiple TUI apps in panes
- Detach/reattach sessions
- Persistent remote UIs

### Shell Integration

TUI apps as CLI commands:
```bash
$ my-tui-app --config settings.toml
$ tail -f log.txt | my-log-viewer
$ my-db-client postgres://localhost/db
```

---

## Sources

- [ratatui - Rust (docs.rs)](https://docs.rs/ratatui/latest/ratatui/)
- [ratatui widgets - Rust](https://docs.rs/ratatui/latest/ratatui/widgets/index.html)
- [GitHub - ratatui/ratatui](https://github.com/ratatui/ratatui)
- [Ratatui Official Site](https://ratatui.rs/)
- [ratatui - crates.io](https://crates.io/crates/ratatui)
- [Building Terminal Apps with Ratatui](https://dasroot.net/posts/2026/02/building-terminal-apps-with-ratatui/)
