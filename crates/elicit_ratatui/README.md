# elicit_ratatui

Dual-mode MCP tools for building terminal user interfaces with
[ratatui](https://ratatui.rs), plus an AccessKit bridge for WCAG-verified
terminal layouts.

Each tool operates in two modes:

1. **Runtime mode** — returns a `TuiNode` JSON description renderable by a
   ratatui terminal backend.
2. **Emit mode** — generates idiomatic ratatui Rust code via the elicitation
   code-emission pipeline.

---

## What this crate provides

- **MCP tool vocabulary** — every ratatui widget, style primitive, layout
  constraint, and text element exposed as a typed, callable tool
- **TuiNode tree** — composable JSON IR for terminal UIs, parallel to egui's
  `UiNode`
- **AccessKit bridge** — bidirectional `TuiNode ↔ AccessKit` conversion for
  WCAG verification
- **RatatuiBackend** — implements `elicit_ui::RenderBackend`, converting a
  verified `Layout<Verified>` into a `TuiNode` tree ready for ratatui rendering
- **Fragment tools** — generate complete ratatui application scaffolding
  (feature-gated)

---

## AccessKit bridge (dual-frontend IR)

`elicit_ratatui` is the ratatui-side implementation of the `elicit_ui`
dual-frontend architecture.

### RatatuiBackend

Implements `elicit_ui::RenderBackend`, converting a verified `Layout<Verified>`
AccessKit tree into a `TuiNode` tree for terminal rendering:

```rust
use elicit_ratatui::RatatuiBackend;
use elicit_ui::{Layout, Viewport};

let backend = RatatuiBackend::new();
let (rendered, stats) = verified.render(&backend);

// Retrieve the TuiNode tree for passing to ratatui's Frame
if let Some(tui_tree) = backend.last_tui_tree() {
    // render tui_tree to terminal frame...
}
```

### Bidirectional TuiNode ↔ AccessKit conversion

```rust
use elicit_ratatui::{tui_node_to_tree_update, tree_update_to_tui_node};

// Forward: TuiNode tree → AccessKit IR (for WCAG verification)
let tree_update = tui_node_to_tree_update(&tui_root);

// Reverse: AccessKit IR → TuiNode tree (for rendering)
let tui_node = tree_update_to_tui_node(&tree_update, root_id);
```

Full round-trip:

```text
TuiNode ──tui_node_to_tree_update()──→ AccessKit IR
                                             │
                                 elicit_ui constraint verification
                                             │
                                             ▼
AccessKit IR ──tree_update_to_tui_node()──→ TuiNode ──RatatuiBackend──→ Frame
```

### WCAG compliance checking for existing TUI apps

```rust
use elicit_ratatui::tui_node_to_tree_update;
use elicit_ui::{Layout, Viewport, TerminalAccessible};

let tree_update = tui_node_to_tree_update(&my_tui_tree);
let layout = Layout::from_update(tree_update);
let constraints = TerminalAccessible::default().to_constraint_set();
let result = layout.verify_custom(Viewport::new(80, 24), &constraints);
```

---

## Widget tools (12 tools)

| Tool | Widget | Description |
|------|--------|-------------|
| `widget_block` | Block | Bordered container with optional title |
| `widget_paragraph` | Paragraph | Text display with wrapping/scrolling |
| `widget_list` | List | Selectable item list |
| `widget_table` | Table | Multi-column data grid |
| `widget_gauge` | Gauge | Progress indicator (0–100%) |
| `widget_sparkline` | Sparkline | Compact inline chart |
| `widget_bar_chart` | BarChart | Grouped vertical/horizontal bars |
| `widget_chart` | Chart | Line/scatter chart with axes |
| `widget_line_gauge` | LineGauge | Linear progress bar |
| `widget_scrollbar` | Scrollbar | Scroll position indicator |
| `widget_tabs` | Tabs | Horizontal tab selector |
| `widget_clear` | Clear | Clear a rectangular area |

All widget tools return a `WidgetJson` variant. Widgets are leaf nodes in the
`TuiNode` tree.

---

## Style tools (7 tools)

| Tool | Description |
|------|-------------|
| `style_fg` | Set foreground colour |
| `style_bg` | Set background colour |
| `style_modifier` | Add text modifier (Bold, Italic, Underline, etc.) |
| `style_reset` | Reset to default style |
| `color_rgb` | Create RGB colour (r, g, b 0–255) |
| `color_indexed` | Create 256-colour palette colour |
| `color_named` | Create named colour (Red, Green, Blue, etc.) |

---

## Layout tools (8 tools)

| Tool | Description |
|------|-------------|
| `layout_vertical` | Vertical split with constraints |
| `layout_horizontal` | Horizontal split with constraints |
| `constraint_length` | Fixed-length constraint |
| `constraint_percentage` | Percentage constraint |
| `constraint_min` | Minimum length constraint |
| `constraint_max` | Maximum length constraint |
| `constraint_fill` | Fill remaining space |
| `constraint_ratio` | Ratio constraint |

---

## Text tools (6 tools)

| Tool | Description |
|------|-------------|
| `text_raw` | Create plain unstyled text |
| `text_styled` | Create styled text with a single span |
| `span_raw` | Create a plain unstyled span |
| `span_styled` | Create a styled span |
| `line_from_spans` | Create a line from spans |
| `text_from_lines` | Create multi-line text from lines |

---

## Terminal tools (8 tools, `runtime` feature)

Requires `features = ["runtime"]`. Provides crossterm terminal lifecycle
management.

| Tool | Description |
|------|-------------|
| `terminal_create` | Create a crossterm terminal |
| `terminal_destroy` | Destroy terminal and restore state |
| `terminal_clear` | Clear the terminal screen |
| `terminal_size` | Get terminal dimensions (cols × rows) |
| `terminal_hide_cursor` | Hide the cursor |
| `terminal_show_cursor` | Show the cursor |
| `terminal_set_cursor` | Set cursor position |
| `terminal_draw` | Draw a TuiNode tree to the terminal |

## Event tools (3 tools, `runtime` feature)

| Tool | Description |
|------|-------------|
| `event_poll` | Poll for events with timeout |
| `event_read` | Read next terminal event |
| `event_read_key` | Read next key event |

---

## Fragment tools (7 tools, `emit` feature)

Generate complete ratatui application scaffolding as Rust source code.

| Tool | Output |
|------|--------|
| `emit_cargo_toml` | `Cargo.toml` for a ratatui app |
| `emit_main_rs` | `main.rs` with terminal setup/teardown |
| `emit_app_struct` | App state struct with constructor |
| `emit_draw_fn` | Draw function from a TuiNode tree |
| `emit_event_handler` | Keyboard event handler |
| `emit_app_loop` | Main application loop |
| `assemble_ratatui_app` | Complete ratatui application (all of the above) |

---

## TuiNode tree

Agents compose terminal UIs from three node types:

```rust
pub enum TuiNode {
    Widget { widget: WidgetJson },
    Container { block: Option<BlockJson>, children: Vec<TuiNode> },
    Layout { direction: DirectionJson, constraints: Vec<ConstraintJson>, children: Vec<TuiNode> },
}
```

The `RatatuiBackend` and fragment tools both consume `TuiNode` trees.
`terminal_draw` renders a tree directly to the active crossterm terminal.

---

## JSON interchange types

| Type | Description |
|------|-------------|
| `WidgetJson` | 12 widget descriptions |
| `TuiNode` | Widget / Container / Layout tree nodes |
| `StyleJson` | Foreground, background, modifier |
| `ColorJson` | Named / RGB / Indexed |
| `ConstraintJson` | Length, Percentage, Min, Max, Fill, Ratio |
| `DirectionJson` | Horizontal / Vertical |
| `TextJson` / `LineJson` / `SpanJson` | Text primitives |
| `BlockJson` | Border, title, padding |
| `BordersJson` | None / All / specific sides |
| `BorderTypeJson` | Plain, Rounded, Double, Thick, etc. |
| `MarginJson` / `PaddingJson` | Spacing |
| `AlignmentJson` | Left, Center, Right |

---

## Features

| Feature | Dependencies | Description |
|---------|-------------|-------------|
| `emit` | `quote`, `elicitation/emit` | Code generation for ratatui apps |
| `runtime` | `crossterm`, `uuid` | Terminal backend integration |
| `uuid` | `uuid` | UUID handles (implied by `runtime`) |

---

## Verification

ratatui types participate in the elicitation verification pipeline:

- **Kani** — bounded model checking for Select enum roundtrips and composite
  struct field coverage
- **Creusot** — deductive proofs for Select/Composite implementations;
  proof artifacts in `verif/elicitation_creusot_rlib/ratatui_types/`

The trenchcoat pattern (newtype wrappers with `JsonSchema` + verification
traits) enables full `schemars` support and MCP tool schema generation despite
the orphan rule.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.
