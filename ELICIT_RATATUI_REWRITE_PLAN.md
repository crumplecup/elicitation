# elicit_ratatui Rewrite Plan

## Problem Statement

`elicit_ratatui` is not a shadow crate. It invented a parallel JSON IR
(`WidgetJson`, `ColorJson`, `StyleJson`, ~1,500 lines of custom DSL) and
bespoke builder/setter tools that have no correspondence to ratatui's actual
API. An agent using the current crate learns a custom vocabulary that
transfers to nothing. The goal of this rewrite is to delete the DSL entirely
and replace it with a real shadow following the three-layer pattern:
**trenchcoat types**, **`#[reflect_methods]`**, **`StatefulPlugin`**.

**Reference implementations:** `elicit_clap` (trenchcoat + `#[reflect_methods]`),
`elicit_redb` (StatefulPlugin + UUID handles).

---

## What Survives Unchanged

These are correctly implemented consumers of ratatui, not DSL tools. Keep
every line:

| File | Role |
|------|------|
| `render_backend.rs` | `RatatuiBackend` — `UiNodeBridge` impl for AccessKit → TuiNode |
| `tui_communicator.rs` | `TuiCommunicator` — `ElicitCommunicator` for terminal apps |
| `tui_accesskit_convert.rs` | `tree_update_to_tui_node` / `tui_node_to_tree_update` |
| `wcag_verify.rs` | WCAG contrast proof helpers |
| `render_context.rs` | `RatatuiRenderContext`, `RenderVerifiable` |

---

## What Gets Deleted

Every file in the current DSL layer is removed in full:

- `serde_types.rs` — `WidgetJson`, `ColorJson`, `StyleJson`, all `*Json` types
- `widget_tools.rs` — custom constructors (`widget_block`, `widget_paragraph`, …)
- `property_tools.rs` — custom setters (`block_set_title`, `block_set_borders`, …)
- `layout_tools.rs` — custom layout tools
- `style_tools.rs` — custom color/style tools
- `text_tools.rs` — custom text/span/line tools
- `fragment_tools.rs` — emit scaffolding tools (reimplementable via real shadow later)
- `event_tools.rs` — rewritten properly (crossterm events as trenchcoats)

---

## Module Structure After Rewrite

```
crates/elicit_ratatui/src/
├── lib.rs                       mod + pub use only
├── plugin.rs                    RatatuiCtx, RatatuiPlugin (stateful)
├── primitives/
│   ├── mod.rs
│   ├── color.rs                 Color — hand-written trenchcoat enum
│   ├── style.rs                 Style — elicit_newtype! + reflect_methods
│   ├── modifier.rs              Modifier — hand-written bitflags trenchcoat
│   ├── constraint.rs            Constraint — hand-written trenchcoat enum
│   ├── rect.rs                  Rect — hand-written trenchcoat struct
│   ├── direction.rs             Direction — hand-written trenchcoat enum
│   ├── alignment.rs             Alignment — hand-written trenchcoat enum
│   ├── borders.rs               Borders, BorderType — bitflags + enum trenchcoats
│   └── padding.rs               Padding, Margin — hand-written trenchcoat structs
├── text/
│   ├── mod.rs
│   ├── span.rs                  Span<'static> — elicit_newtype! + reflect_methods
│   ├── line.rs                  Line<'static> — elicit_newtype! + reflect_methods
│   └── text.rs                  Text<'static> — elicit_newtype! + reflect_methods
├── layout.rs                    Layout — elicit_newtype! + reflect_methods
├── widgets/
│   ├── mod.rs
│   ├── block.rs                 Block<'static> — elicit_newtype! + reflect_methods
│   ├── paragraph.rs             Paragraph<'static> — elicit_newtype! + reflect_methods
│   ├── list.rs                  List<'static>, ListItem<'static> — newtypes
│   ├── table.rs                 Table<'static>, Row<'static>, Cell<'static> — newtypes
│   ├── gauge.rs                 Gauge<'static>, LineGauge<'static> — newtypes
│   ├── chart.rs                 Chart<'static>, Dataset<'static>, Axis<'static> — newtypes
│   ├── sparkline.rs             Sparkline<'static> — newtype
│   ├── bar_chart.rs             BarChart — newtype
│   ├── tabs.rs                  Tabs<'static> — newtype
│   └── scrollbar.rs             Scrollbar — newtype
├── state/
│   ├── mod.rs
│   ├── list_state.rs            ListState UUID tools
│   ├── table_state.rs           TableState UUID tools
│   └── scrollbar_state.rs       ScrollbarState UUID tools
├── terminal.rs                  Terminal UUID tools (StatefulPlugin)
├── events.rs                    Event trenchcoats + tools
├── render_backend.rs            KEEP
├── tui_communicator.rs          KEEP
├── tui_accesskit_convert.rs     KEEP
└── wcag_verify.rs               KEEP
```

---

## Phase 0 — Delete the DSL

Remove all files listed under "What Gets Deleted" above. Remove all
`pub use` exports in `lib.rs` that point to deleted modules. This phase
intentionally breaks compilation — subsequent phases restore it with
correct code.

---

## Phase 1 — Primitive Trenchcoats

These are all hand-written (not `elicit_newtype!`) because they are small
value types where explicit field mapping is cleaner than Arc-backing.
Every type gets `Debug + Clone + PartialEq + Serialize + Deserialize +
JsonSchema + From<upstream> + Into<upstream>`.

### `primitives/color.rs`

Mirror `ratatui::style::Color` exactly. The upstream enum has 20+ variants;
match them 1:1 and implement bidirectional `From`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Color {
    Reset, Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
    DarkGray, LightRed, LightGreen, LightYellow, LightBlue,
    LightMagenta, LightCyan, Gray,
    Rgb { r: u8, g: u8, b: u8 },
    Indexed { index: u8 },
}

impl From<Color> for ratatui::style::Color { /* match arm per variant */ }
impl From<ratatui::style::Color> for Color { /* match arm per variant */ }
```

### `primitives/modifier.rs`

`ratatui::style::Modifier` is bitflags. Represent it as a serializable
struct of booleans (or a set-of-strings). Bidirectional `From` via
`Modifier::from_bits_truncate`.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Modifier {
    pub bold: bool, pub dim: bool, pub italic: bool, pub underlined: bool,
    pub slow_blink: bool, pub rapid_blink: bool, pub reversed: bool,
    pub hidden: bool, pub crossed_out: bool,
}
```

### `primitives/constraint.rs`

`ratatui::layout::Constraint` is an enum with fields. Hand-written
trenchcoat:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum Constraint {
    Percentage(u16), Ratio(u32, u32), Length(u16), Fill(u16),
    Max(u16), Min(u16),
}
```

### `primitives/rect.rs`

`ratatui::layout::Rect` has four `u16` fields. Direct struct mirror.

### `primitives/direction.rs`, `alignment.rs`, `borders.rs`, `padding.rs`

Same pattern — enum/struct mirrors with 1:1 variant mapping.

---

## Phase 2 — Text Primitive Trenchcoats

ratatui's `Span<'a>`, `Line<'a>`, `Text<'a>` all have lifetime parameters.
The shadow types wrap the `'static` version. Since ratatui's builders accept
`impl Into<Cow<'static, str>>`, passing `String` works natively.

Use `elicit_newtype!` for Arc-backed cloning, then custom
`Serialize`/`Deserialize`/`JsonSchema` (same pattern as `elicit_clap::Arg`),
then `#[reflect_methods]` for the builder API.

### `text/span.rs`

```rust
elicit_newtype!(ratatui::text::Span<'static>, as Span);

// Custom serde: serialize to {content, style?}; deserialize from same
// Custom JsonSchema: {content: string, style?: Style}

#[reflect_methods]
impl Span {
    pub fn content(&self) -> String { self.0.content.to_string() }
    // patch_style takes our Style trenchcoat, converts to ratatui::style::Style
    pub fn patch_style(&self, style: crate::Style) -> Span {
        Span::from((*self.0).clone().patch_style(style.into()))
    }
    pub fn reset_style(&self) -> Span {
        Span::from((*self.0).clone().reset_style())
    }
}
```

### `text/line.rs`

Same pattern — wraps `ratatui::text::Line<'static>`. Reflect:
`spans()`, `width()`, `alignment()`, `patch_style()`.

### `text/text.rs`

Same — wraps `ratatui::text::Text<'static>`. Reflect:
`lines()`, `width()`, `height()`, `patch_style()`, `alignment()`.

---

## Phase 3 — Layout Trenchcoat

```rust
elicit_newtype!(ratatui::layout::Layout, as Layout);

// Custom serde: serialize direction + constraints + margin + flex settings

#[reflect_methods]
impl Layout {
    /// Create a new horizontal layout.
    pub fn horizontal(constraints: Vec<crate::Constraint>) -> Layout {
        Layout::from(ratatui::layout::Layout::horizontal(
            constraints.into_iter().map(Into::into)
        ))
    }
    /// Create a new vertical layout.
    pub fn vertical(constraints: Vec<crate::Constraint>) -> Layout {
        Layout::from(ratatui::layout::Layout::vertical(
            constraints.into_iter().map(Into::into)
        ))
    }
    /// Split a Rect into areas.
    pub fn areas(&self, area: crate::Rect) -> Vec<crate::Rect> {
        self.0.clone().areas::<9>(area.into())   // returns [Rect; N]; collect to Vec
            .iter().map(|r| crate::Rect::from(*r)).collect()
    }
    pub fn direction(&self, d: crate::Direction) -> Layout {
        Layout::from((*self.0).clone().direction(d.into()))
    }
    pub fn margin(&self, m: u16) -> Layout {
        Layout::from((*self.0).clone().margin(m))
    }
    pub fn spacing(&self, s: u16) -> Layout {
        Layout::from((*self.0).clone().spacing(s))
    }
}
```

Note: ratatui 0.30's `Layout::areas` is generic over const `N`. Expose a
`split_n(area, n)` free tool for the general case.

---

## Phase 4 — Widget Trenchcoats with `#[reflect_methods]`

Each widget follows the same pattern. `Block` is shown in full; the others
are identical in structure.

### `widgets/block.rs`

```rust
elicit_newtype!(ratatui::widgets::Block<'static>, as Block);

// Serde: serialize {title?, borders, border_type?, style?, padding?}
// Deserialize: reconstruct Block::new() + apply fields

#[reflect_methods]
impl Block {
    /// Create a new empty block.
    pub fn new() -> Block {
        Block::from(ratatui::widgets::Block::new())
    }
    /// Set the block title.
    pub fn title(&self, title: String) -> Block {
        Block::from((*self.0).clone().title(title))
    }
    /// Set the borders (use Borders trenchcoat).
    pub fn borders(&self, borders: crate::Borders) -> Block {
        Block::from((*self.0).clone().borders(borders.into()))
    }
    /// Set the border type.
    pub fn border_type(&self, bt: crate::BorderType) -> Block {
        Block::from((*self.0).clone().border_type(bt.into()))
    }
    /// Set the block style.
    pub fn style(&self, style: crate::Style) -> Block {
        Block::from((*self.0).clone().style(style.into()))
    }
    /// Set the padding.
    pub fn padding(&self, padding: crate::Padding) -> Block {
        Block::from((*self.0).clone().padding(padding.into()))
    }
}
```

`#[reflect_methods]` generates MCP tools:
`block__new`, `block__title`, `block__borders`, `block__border_type`,
`block__style`, `block__padding`.

The agent composes: `block__new()` → `block__title("My App")` →
`block__borders("All")` → pass to `terminal__render_widget`.

### Remaining widgets — same pattern

| Widget | Key builder methods to reflect |
|--------|-------------------------------|
| `Paragraph` | `new(text)`, `block(block)`, `wrap(wrap)`, `scroll(offset)`, `alignment(align)` |
| `List` | `new(items)`, `block(block)`, `style(style)`, `highlight_style(style)`, `highlight_symbol(sym)` |
| `Table` | `new(rows)`, `header(row)`, `block(block)`, `widths(constraints)`, `style(style)`, `highlight_style(style)` |
| `Gauge` | `new()`, `block(block)`, `ratio(f64)`, `percent(u16)`, `label(text)`, `style(style)`, `gauge_style(style)` |
| `LineGauge` | `new()`, `block(block)`, `ratio(f64)`, `label(text)`, `style(style)` |
| `Sparkline` | `new()`, `block(block)`, `data(data)`, `max(u64)`, `direction(dir)`, `style(style)` |
| `BarChart` | `new()`, `block(block)`, `data(group)`, `max(u64)`, `bar_width(u16)`, `bar_gap(u16)`, `style(style)` |
| `Chart` | `new(datasets)`, `block(block)`, `x_axis(axis)`, `y_axis(axis)`, `style(style)` |
| `Tabs` | `new(titles)`, `block(block)`, `select(usize)`, `style(style)`, `highlight_style(style)`, `divider(span)` |
| `Scrollbar` | `new(orient)`, `track_symbol(sym)`, `thumb_symbol(sym)`, `begin_symbol(sym)`, `end_symbol(sym)`, `style(style)` |

---

## Phase 5 — StatefulPlugin

Ratatui requires a live `Terminal` that persists across tool calls. Stateful
widgets (`List`, `Table`, `Scrollbar`) require mutable state objects.
Both go into `RatatuiCtx`.

### `plugin.rs`

```rust
pub struct RatatuiCtx {
    #[cfg(feature = "runtime")]
    pub(crate) terminals:        Mutex<HashMap<Uuid, Terminal<CrosstermBackend<Stdout>>>>,
    pub(crate) list_states:      Mutex<HashMap<Uuid, ratatui::widgets::ListState>>,
    pub(crate) table_states:     Mutex<HashMap<Uuid, ratatui::widgets::TableState>>,
    pub(crate) scrollbar_states: Mutex<HashMap<Uuid, ratatui::widgets::ScrollbarState>>,
}

impl PluginContext for RatatuiCtx {}

#[derive(ElicitPlugin)]
#[plugin(name = "ratatui")]
pub struct RatatuiPlugin(pub Arc<RatatuiCtx>);
```

Lock ordering: `list_states` < `table_states` < `scrollbar_states` < `terminals`.

### `terminal.rs` tools

```rust
// terminal__create — create crossterm terminal, store in ctx, return UUID
#[elicit_tool(plugin = "ratatui", name = "terminal__create", description = "…")]
async fn terminal_create(ctx: Arc<RatatuiCtx>, p: TerminalCreateParams)
    -> Result<CallToolResult, ErrorData>

// terminal__size — return (cols, rows) for a terminal UUID
#[elicit_tool(plugin = "ratatui", name = "terminal__size", description = "…")]
async fn terminal_size(ctx: Arc<RatatuiCtx>, p: TerminalSelfParams)
    -> Result<CallToolResult, ErrorData>

// terminal__render_widget — render a stateless widget at a Rect
#[elicit_tool(plugin = "ratatui", name = "terminal__render_widget", description = "…")]
async fn terminal_render_widget(ctx: Arc<RatatuiCtx>, p: TerminalRenderWidgetParams)
    -> Result<CallToolResult, ErrorData>

// terminal__render_stateful — render a stateful widget with a state UUID
#[elicit_tool(plugin = "ratatui", name = "terminal__render_stateful", description = "…")]
async fn terminal_render_stateful(ctx: Arc<RatatuiCtx>, p: TerminalRenderStatefulParams)
    -> Result<CallToolResult, ErrorData>

// terminal__clear, terminal__hide_cursor, terminal__show_cursor
// terminal__set_cursor(col, row)
// terminal__destroy — restore raw mode, remove from ctx
```

The render tools reconstruct the real ratatui widget from the trenchcoat via
`Into`, then call `terminal.draw(|frame| frame.render_widget(widget, area))`.
This is the conversion site — the only place where `WidgetEnum::Block(b) =>
frame.render_widget(ratatui::Block::from(b), area)` etc. live. There is a
single `WidgetEnum` (not `WidgetJson`) used only in render params:

```rust
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(tag = "kind")]
pub enum WidgetParam {
    Block(Block), Paragraph(Paragraph), List(List), Table(Table),
    Gauge(Gauge), LineGauge(LineGauge), Sparkline(Sparkline),
    BarChart(BarChart), Chart(Chart), Tabs(Tabs), Clear,
}
```

This enum exists only as a tool parameter to `terminal__render_widget`. It
is NOT the general-purpose `WidgetJson` the old code used everywhere. Widgets
flow as their own types through the chain; only at render time is a union
needed.

### `state/` tools

```rust
// list_state__new() → UUID
// list_state__select(state_id, Option<usize>)
// list_state__offset(state_id) → usize
// list_state__destroy(state_id)
//
// table_state__new() → UUID
// table_state__select_row(state_id, Option<usize>)
// table_state__select_column(state_id, Option<usize>)
// table_state__destroy(state_id)
//
// scrollbar_state__new(content_length) → UUID
// scrollbar_state__position(state_id) → usize
// scrollbar_state__set_position(state_id, usize)
// scrollbar_state__destroy(state_id)
```

---

## Phase 6 — Events

`crossterm::event::Event`, `KeyEvent`, `KeyCode`, `MouseEvent` get
hand-written trenchcoat enums (same Copy/value pattern as Phase 1). The
`event__poll(timeout_ms)` and `event__read()` tools return these trenchcoats
as JSON.

---

## Phase 7 — Style Trenchcoat with `#[reflect_methods]`

`ratatui::style::Style` is a struct with `fg`, `bg`, `underline_color`,
`add_modifier`, `sub_modifier`. Use `elicit_newtype!` for consistency with
the other builder types.

```rust
elicit_newtype!(ratatui::style::Style, as Style);

#[reflect_methods]
impl Style {
    pub fn new() -> Style { Style::from(ratatui::style::Style::new()) }
    pub fn fg(&self, color: crate::Color) -> Style {
        Style::from((*self.0).fg(color.into()))
    }
    pub fn bg(&self, color: crate::Color) -> Style {
        Style::from((*self.0).bg(color.into()))
    }
    pub fn bold(&self) -> Style {
        Style::from((*self.0).bold())
    }
    pub fn italic(&self) -> Style {
        Style::from((*self.0).italic())
    }
    // underlined, dim, rapid_blink, slow_blink, reversed, hidden, crossed_out
    pub fn reset(&self) -> Style {
        Style::from((*self.0).reset())
    }
    pub fn patch(&self, other: crate::Style) -> Style {
        Style::from((*self.0).patch((*other.0)))
    }
}
```

---

## Typical Agent Tool Chain After Rewrite

```
style__new()
  → style__fg(Color::Rgb {r:0,g:255,b:128})
  → style__bold()
  → s (Style UUID or inline)

block__new()
  → block__title("My App")
  → block__borders(Borders::All)
  → block__style(s)
  → b (Block)

text__from_str("Hello, world!")
  → t (Text)

paragraph__new(t)
  → paragraph__block(b)
  → paragraph__alignment(Alignment::Center)
  → p (Paragraph)

terminal__create()              → term_id
terminal__size(term_id)         → {cols: 80, rows: 24}
rect__new(0, 0, 80, 24)         → area
terminal__render_widget(term_id, Paragraph(p), area)
terminal__destroy(term_id)
```

Every step calls the actual ratatui API. The vocabulary IS ratatui's
vocabulary. An agent that learns this tool chain can read and write ratatui
code directly — there is no DSL to unlearn.

---

## Cargo.toml Changes

Add `uuid` to non-optional deps (already present as optional — promote it):

```toml
[dependencies]
uuid = { workspace = true, features = ["v4"] }  # remove optional

[features]
runtime = ["dep:crossterm", "dep:tokio", "ratatui/crossterm"]
# uuid is no longer feature-gated — every plugin session needs UUIDs for state
```

---

## Implementation Order

1. Phase 0 — Delete DSL files
2. Phase 1 — Primitive trenchcoats (Color, Modifier, Constraint, Rect, Direction, Alignment, Borders, Padding)
3. Phase 7 — Style trenchcoat (`#[reflect_methods]`)
4. Phase 2 — Text primitives (Span, Line, Text)
5. Phase 3 — Layout
6. Phase 4 — Widget trenchcoats, one file at a time, starting with Block
7. Phase 5 — StatefulPlugin + terminal tools + state tools
8. Phase 6 — Events
9. Restore `lib.rs` exports, verify compile, run `just check elicit_ratatui`

---

## Key Rules

- `elicit_newtype!` for builder types that need Arc-backed Clone
- Hand-written struct/enum mirrors for Copy/value primitives
- `#[reflect_methods]` on EVERY impl block — no manual `#[elicit_tool]`
  inside widget files; tools emerge from reflection
- Free-standing constructor tools (`block__new()`, `style__new()`) come
  from `#[reflect_methods]` on associated functions
- `StatefulPlugin` + `Arc<RatatuiCtx>` for terminal and state lifecycle
- `WidgetParam` enum exists only in `TerminalRenderWidgetParams` — it is
  NOT a general interchange type
- `#[instrument]` on every reflected method (already enforced by CLAUDE.md;
  `#[reflect_methods]` should propagate the attribute or each method carries it)
