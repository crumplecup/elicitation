# elicit_egui

MCP-enabled GUI tooling built on [`egui`], [`elicitation`], and [`rmcp`].

Wraps egui's immediate-mode widgets, containers, layout, styling, input, and
code generation as dual-mode MCP tools — giving an agent a complete vocabulary
for constructing verified GUI workflows, either as runtime JSON descriptions or
as emitted Rust source code.

---

## What this crate provides

Two complementary modes, plus code generation:

1. **Runtime mode** — each tool returns a tagged-enum JSON description
   (`WidgetJson`, `ContainerJson`, `LayoutJson`, `StyleJson`, …) that can be
   rendered by the headless egui runtime or inspected as structured data
2. **Emit mode** — each tool's parameters participate in the elicitation
   code-emission pipeline, generating idiomatic egui Rust code
3. **Fragment tools** — generate complete eframe application scaffolding,
   forms, tables, settings panels, and app state boilerplate

---

## Quick start

```toml
[dependencies]
elicit_egui = { version = "0.9" }
rmcp = { version = "0.1", features = ["server"] }
egui = "0.34"
```

```rust,no_run
use elicitation::PluginRegistry;

// All 148 tools are registered via inventory — no manual plugin setup needed.
// The elicit_server crate handles registration automatically.

// To use the headless runtime (requires "runtime" feature):
// elicit_egui = { version = "0.9", features = ["runtime"] }
```

---

## Plugin inventory

| Plugin | Namespace | Tool count | Category |
|---|---|---|---|
| `egui_widgets` | `widget_*` | 31 | Labels, buttons, sliders, text inputs, colour pickers |
| `egui_containers` | `container_*` | 14 | Windows, panels, scroll areas, collapsing, frames |
| `egui_layout` | `layout_*` | 11 | Horizontal, vertical, grid, columns, indent |
| `egui_style` | `style_*` / `egui_set_*` | 29 | Themes, fonts, colours, spacing, interaction, debug |
| `egui_response` | `response_*` | 21 | Click, hover, focus, drag, value-change queries |
| `egui_menus` | `egui_*` | 13 | Context menus, popups, tooltips, modals, dialogs |
| `egui_input` | `egui_*` | 14 | Keyboard, pointer, clipboard, focus management |
| `egui_fragments` | `egui_fragment_*` | 10 | Code generation for complete eframe apps |
| `egui_runtime` | `context_*` | 5 | Headless context lifecycle (feature-gated) |
| **Total** | | **148** | |

---

## How `elicit_egui` satisfies the shadow crate motivation

The shadow crate motivation is stated in `SHADOW_CRATE_MOTIVATION.md`. The core
idea is:

> *Define a vocabulary of atomic, verified operations — let an agent compose
> them into tool chains — the tool chain **is** the GUI.*

Immediate-mode GUI is one of the most natural domains for this approach: an egui
frame is already a sequence of declarative widget calls — exactly the kind of
atomic vocabulary an agent can compose. `elicit_egui` makes every egui widget,
container, layout primitive, and style mutation an explicitly typed, MCP-callable
tool with a machine-checkable contract.

### The dual-mode pattern

Every tool operates in two complementary modes:

```text
Agent call: { "tool": "widget_button", "params": { "text": "Save" } }
                    │
     ┌──────────────┴──────────────┐
     ▼                             ▼
  Runtime mode                  Emit mode
  → WidgetJson::Button          → ui.button("Save");
  → JSON for headless render    → Rust source for compilation
```

**Runtime mode** returns tagged-enum JSON that can be composed into UI trees
(`UiNode`) and rendered by the headless runtime. **Emit mode** generates the
equivalent Rust source code for direct compilation.

### The headless runtime

The `runtime` feature provides a headless `egui::Context` — no windowing system,
no GPU, no eframe dependency. Agents create contexts, compose UI trees from JSON,
and execute frames programmatically:

```text
Agent: context_create → { "context_id": "3f7a…" }
Agent: context_run_frame → { "context_id": "3f7a…", "ui_tree": [...] }
  → Server renders UiNode tree into egui calls
  → Returns { "widgets_rendered": 12, "repaint_needed": false }
```

This enables headless UI testing, screenshot generation, and layout validation
without any display server.

---

## Widget tools (31 tools)

| Tool | Widget | Category |
|---|---|---|
| `widget_label` | Plain text | Display |
| `widget_heading` | Heading text | Display |
| `widget_monospace` | Monospace text | Display |
| `widget_code` | Code with background | Display |
| `widget_small` | Small text | Display |
| `widget_strong` | Bold text | Display |
| `widget_weak` | Faint text | Display |
| `widget_colored_label` | Coloured text | Display |
| `widget_button` | Clickable button | Interactive |
| `widget_small_button` | Compact button | Interactive |
| `widget_checkbox` | Boolean toggle | Interactive |
| `widget_radio_value` | Radio button (auto-update) | Interactive |
| `widget_radio` | Radio button (display only) | Interactive |
| `widget_selectable_label` | Toggle label | Interactive |
| `widget_toggle_value` | Boolean toggle (simple) | Interactive |
| `widget_link` | Clickable text link | Interactive |
| `widget_hyperlink` | Web link | Interactive |
| `widget_separator` | Divider line | Layout |
| `widget_spinner` | Loading spinner | Feedback |
| `widget_text_edit_singleline` | Single-line input | Text |
| `widget_text_edit_multiline` | Multi-line input | Text |
| `widget_code_editor` | Code editor | Text |
| `widget_slider` | Numeric slider | Numeric |
| `widget_slider_vertical` | Vertical slider | Numeric |
| `widget_drag_value` | Drag-to-edit value | Numeric |
| `widget_drag_angle` | Drag-to-edit angle (degrees) | Numeric |
| `widget_drag_angle_tau` | Drag-to-edit angle (tau) | Numeric |
| `widget_progress_bar` | Progress indicator | Feedback |
| `widget_color_edit_button_srgba` | sRGBA colour picker | Colour |
| `widget_color_edit_button_hsva` | HSVA colour picker | Colour |
| `widget_image` | Image display | Media |

All widget tools return a `WidgetJson` variant as tagged JSON. The JSON can be
used standalone or composed into `UiNode::Widget { widget }` tree nodes for the
runtime renderer.

---

## Container tools (14 tools)

| Tool | Container | Notes |
|---|---|---|
| `container_window` | Floating window | Title, position, size, collapsible |
| `container_left_panel` | Left side panel | Resizable with min/max width |
| `container_right_panel` | Right side panel | Resizable |
| `container_top_panel` | Top panel | Resizable |
| `container_bottom_panel` | Bottom panel | Resizable |
| `container_central_panel` | Central panel | Fills remaining space |
| `container_scroll_area` | Scroll region | Vertical/horizontal scroll |
| `container_collapsing` | Collapsible section | Header text, default open |
| `container_group` | Visual group | Box around content |
| `container_frame` | Styled frame | Fill, stroke, margins |
| `container_menu_bar` | Menu bar | Top-level menus |
| `container_menu` | Menu | Within menu bar |
| `container_tooltip` | Tooltip | Hover text |
| `container_popup` | Popup area | Context menu, dropdown |

Containers wrap child `UiNode` lists. Panels and windows that normally require
`Context`-level access degrade gracefully to groups when rendered inside a `Ui`
(e.g., nested in a scroll area).

---

## Layout tools (11 tools)

| Tool | Layout | Notes |
|---|---|---|
| `layout_horizontal` | Left-to-right | Optional alignment |
| `layout_vertical` | Top-to-bottom | Optional alignment |
| `layout_horizontal_centered` | Centred horizontal | — |
| `layout_vertical_centered` | Centred vertical | — |
| `layout_horizontal_justified` | Justified horizontal | Items stretch to fill |
| `layout_vertical_justified` | Justified vertical | — |
| `layout_horizontal_wrapped` | Wrapping horizontal | Next line on overflow |
| `layout_columns` | Column layout | N columns |
| `layout_grid` | Grid layout | Striped, column count |
| `layout_indent` | Indentation | Pixel amount |
| `layout_add_space` | Spacing | Pixel amount |

---

## Style tools (29 tools)

### Theme & core spacing

| Tool | Target | Notes |
|---|---|---|
| `style_spacing` | Global spacing | Item, window, button padding |
| `style_dark_mode` | Dark theme | egui dark visuals |
| `style_light_mode` | Light theme | egui light visuals |

### Font & text

| Tool | Target | Notes |
|---|---|---|
| `egui_set_fonts` | Font families | Proportional, monospace |
| `egui_override_text_style` | Text style override | Family + size per style |
| `egui_set_text_valign` | Text vertical align | Top, center, bottom |
| `egui_set_text_cursor_width` | Cursor blink | Width, on/off duration |

### Colour overrides

| Tool | Target | Notes |
|---|---|---|
| `style_visual` | Named colour property | Hyperlink, bg, panel, etc. |
| `egui_set_hyperlink_color` | Hyperlink colour | — |
| `egui_set_faint_bg_color` | Faint background | Alternating rows |
| `egui_set_extreme_bg_color` | Extreme background | Text input fields |
| `egui_set_code_bg_color` | Code background | Monospace background |
| `egui_set_warn_fg_color` | Warning foreground | — |
| `egui_set_error_fg_color` | Error foreground | — |

### Widget state visuals

| Tool | Target | Notes |
|---|---|---|
| `style_widget_visuals` | Widget fill/stroke per state | Noninteractive, inactive, hovered, active, open |
| `style_selection` | Selection highlight | Background + stroke |
| `style_text_cursor` | Text cursor | Colour, width |
| `egui_set_widget_stroke` | Widget border stroke | Per-state |
| `egui_set_window_stroke` | Window border | Width + colour |

### Window & layout

| Tool | Target | Notes |
|---|---|---|
| `style_window_rounding` | Window corners | Corner radius |
| `style_window_shadow` | Window shadow | Offset, blur, colour |
| `egui_set_menu_margin` | Menu margin | Left, right, top, bottom |
| `egui_set_button_padding` | Button padding | Horizontal, vertical |
| `egui_set_indent` | Indentation | Pixel distance |
| `egui_set_scroll_bar_width` | Scroll bar | Width, handle, margins |
| `egui_set_resize_grip_size` | Resize grip | Corner size |

### Behaviour

| Tool | Target | Notes |
|---|---|---|
| `egui_set_interaction` | Interaction settings | Click time, drag threshold, tooltip delay |
| `egui_set_animation_time` | Animation duration | Transition timing |
| `egui_set_debug_options` | Debug rendering | Widget hits, hover debug |

All style tools return a `StyleJson` variant. The runtime's `apply_style()`
function maps every variant to the corresponding `egui::Context` style mutation.

---

## Response tools (21 tools)

| Tool | Query | Notes |
|---|---|---|
| `response_clicked` | Was clicked | — |
| `response_double_clicked` | Was double-clicked | — |
| `response_secondary_clicked` | Was right-clicked | — |
| `response_clicked_n` | Clicked N times | Count param |
| `response_hovered` | Is hovered | — |
| `response_has_focus` | Has focus | — |
| `response_gained_focus` | Gained focus | — |
| `response_lost_focus` | Lost focus | — |
| `response_request_focus` | Request focus | — |
| `response_surrender_focus` | Release focus | — |
| `response_dragged` | Is dragged | — |
| `response_drag_released` | Drag released | — |
| `response_drag_delta` | Drag delta | Vec2 result |
| `response_changed` | Value changed | — |
| `response_rect` | Bounding rect | Rect result |
| `response_hover_pos` | Hover position | Optional Vec2 |
| `response_show_tooltip` | Show tooltip | Text param |
| `response_set_enabled` | Set enabled state | Bool param |
| `response_highlight` | Highlight widget | — |
| `response_scroll_to_me` | Scroll into view | — |
| `response_context_menu` | Context menu | — |

Response tools return `ResponseQueryJson` (for queries) or `ResponseInfoJson`
(for actions). These are the "glue" tools that let agents react to user
interaction in runtime mode.

---

## Menu & popup tools (13 tools)

| Tool | Action | Notes |
|---|---|---|
| `egui_context_menu` | Right-click menu | Region ID |
| `egui_context_menu_item` | Menu item | Label + optional shortcut |
| `egui_context_menu_separator` | Menu separator | — |
| `egui_popup` | Popup at position | ID, position, content |
| `egui_popup_below_widget` | Popup below widget | Anchor ID |
| `egui_close_popup` | Close popup | — |
| `egui_tooltip` | Hover tooltip | Widget ID, text |
| `egui_tooltip_rich` | Rich tooltip | Custom UI content |
| `egui_tooltip_at_pointer` | Pointer tooltip | Text at cursor |
| `egui_modal` | Modal dialog | Title, content, buttons |
| `egui_confirm_dialog` | Confirm dialog | Yes/no |
| `egui_alert_dialog` | Alert dialog | OK button |
| `egui_notification` | Toast message | Text, duration, position |

---

## Input tools (14 tools)

| Tool | Query | Notes |
|---|---|---|
| `egui_key_pressed` | Key pressed this frame | — |
| `egui_key_released` | Key released this frame | — |
| `egui_key_down` | Key currently held | — |
| `egui_modifiers` | Modifier key state | Ctrl, Shift, Alt, Command |
| `egui_pointer_pos` | Pointer position | — |
| `egui_pointer_button_pressed` | Mouse button pressed | Button name |
| `egui_pointer_button_released` | Mouse button released | Button name |
| `egui_pointer_delta` | Pointer movement delta | This frame |
| `egui_scroll_delta` | Scroll wheel delta | — |
| `egui_clipboard_get` | Get clipboard text | — |
| `egui_clipboard_set` | Set clipboard text | — |
| `egui_request_focus` | Request focus | Widget ID |
| `egui_surrender_focus` | Release focus | Widget ID |
| `egui_has_focus` | Check focus | Widget ID |

---

## Fragment tools (10 tools)

Fragment tools generate complete Rust source code rather than runtime JSON. They
are the code-generation complement to the runtime tools.

| Tool | Output | Notes |
|---|---|---|
| `egui_fragment_native_app` | eframe native app | main.rs + app.rs boilerplate |
| `egui_fragment_web_app` | eframe WASM app | Web canvas setup |
| `egui_fragment_form` | Form widget | Labelled fields (text, number, checkbox, slider, colour) |
| `egui_fragment_table` | Data table | Typed columns + row struct |
| `egui_fragment_settings_panel` | Settings panel | Sections with fields |
| `egui_fragment_sidebar_layout` | Sidebar layout | Sidebar + main content |
| `egui_fragment_tab_panel` | Tabbed panel | Named tabs |
| `egui_fragment_toolbar` | Toolbar | Buttons with tooltips |
| `egui_fragment_app_state` | App state struct | Fields + Default impl |
| `egui_fragment_message_enum` | Message enum | Action variants with payloads |

Example: generating a complete native app:

```json
{
  "tool": "egui_fragment_native_app",
  "params": {
    "app_name": "MyApp",
    "window_title": "My Application",
    "window_width": 1024.0,
    "window_height": 768.0,
    "dark_mode": true
  }
}
```

Returns generated `Cargo.toml`, `src/main.rs`, and `src/app.rs` as JSON strings.

---

## Runtime tools (5 tools, feature-gated)

Requires `features = ["runtime"]`. Provides headless egui context management
using `egui::Context` directly — no eframe, no windowing, no GPU.

| Tool | Action | Notes |
|---|---|---|
| `context_create` | Create context | Returns UUID handle |
| `context_destroy` | Destroy context | Frees resources |
| `context_list` | List contexts | Active context IDs |
| `context_run_frame` | Execute frame | Renders UiNode tree |
| `context_apply_style` | Apply style | StyleJson → Context mutation |

### The UiNode tree

Agents compose UI trees from three node types:

```rust
pub enum UiNode {
    Widget { widget: WidgetJson },
    Container { container: ContainerJson, children: Vec<UiNode> },
    Layout { layout: LayoutJson, children: Vec<UiNode> },
}
```

The runtime's `render_node()` recursively walks the tree, calling the
appropriate egui functions for each node. Containers and layouts wrap their
children; widgets are leaf nodes.

---

## JSON interchange types

Tools communicate via tagged enums that serialize to compact JSON:

| Type | Module | Variants |
|---|---|---|
| `WidgetJson` | `serde_types` | 31 widget descriptions |
| `ContainerJson` | `serde_types` | 13 container descriptions |
| `LayoutJson` | `serde_types` | 11 layout descriptions |
| `StyleJson` | `style_tools` | 20 style mutation descriptions |
| `ResponseQueryJson` | `response_tools` | 16 response queries |
| `ResponseInfoJson` | `response_tools` | 5 response actions |
| `MenuActionJson` | `menu_tools` | 13 menu/popup/dialog descriptions |
| `InputActionJson` | `input_tools` | 14 input/keyboard/pointer queries |
| `UiNode` | `serde_types` | Widget, Container, Layout tree nodes |

Supporting types for colour, geometry, and strokes:

| Type | Fields |
|---|---|
| `ColorJson` | `r, g, b, a` (0–255) |
| `Vec2Json` | `x, y` |
| `RectJson` | `min: Vec2Json, max: Vec2Json` |
| `StrokeJson` | `width: f32, color: ColorJson` |
| `CornerRadiusJson` | `nw, ne, sw, se` |
| `MarginJson` | `left, right, top, bottom` |
| `FontIdJson` | `size: f32, family: String` |

---

## What we chose not to shadow, and why

### `egui::Context` as a live object

`egui::Context` is `Clone` but carries mutable interior state — fonts, textures,
input state, animation clocks. It cannot meaningfully cross the MCP JSON
boundary. We expose it through the server-side registry pattern: UUID handles
reference contexts held in `Arc<Mutex<HashMap<Uuid, egui::Context>>>`, and agents
interact via serializable tool calls.

### `eframe` windowing

eframe pulls in `winit` for OS window management. We deliberately exclude
eframe from the runtime feature — the headless `egui::Context` is sufficient
for all tool operations. A future `windowed` feature can add eframe when needed.
Fragment tools generate eframe code for compilation, bridging the gap.

### `egui::Response` as a return type

`Response` is a frame-local reference tied to the immediate-mode render loop.
We decompose it into individual query tools (`response_clicked`,
`response_hovered`, etc.) that return simple JSON booleans or values, making
response data fully serializable and composable.

### Custom widgets and painters

Custom `Widget` implementations and `Painter` API access require Rust code that
cannot be expressed as tool parameters. Fragment tools handle this by generating
the Rust source code directly.

---

## Feature flags

| Feature | Effect |
|---|---|
| `emit` | Enables `EmitCode` code-recovery support |
| `runtime` | Enables headless egui context management (5 tools) |

---

## Verification

egui types participate in the elicitation verification pipeline:

- **Kani** — bounded model checking for Select enum roundtrips and composite
  struct field coverage (`crates/elicitation_kani/src/egui_types.rs`)
- **Creusot** — deductive proofs for Select/Composite implementations
  (`crates/elicitation_creusot/src/egui_types.rs`)

The trenchcoat pattern (newtype wrappers with `JsonSchema` + verification
traits) enables full `schemars` support and MCP tool schema generation despite
the orphan rule — egui types are wrapped in verification newtypes that derive
the required traits and unwrap to the original type after validation.

[`egui`]: https://docs.rs/egui
[`elicitation`]: https://docs.rs/elicitation
[`rmcp`]: https://docs.rs/rmcp
