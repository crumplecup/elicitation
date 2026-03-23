# elicit_egui — Implementation Plan

> **Premise:** Expose egui's immediate mode GUI API as MCP tools for both runtime UI execution and code generation.
> **Approach:** Completionist harvesting using dual-mode tools (primary), runtime-only tools (UI display), and fragment tools (app code gen).

---

## Why egui is Different

**Unique characteristics:**
- **Immediate mode** (widgets recreated every frame, not retained)
- **Imperative API** (build UI with function calls, not declarative markup)
- **User-managed state** (egui doesn't store widget values)
- **Platform integration** (needs eframe/egui_web/egui_winit for rendering)
- **Every widget returns Response** (interaction state: clicked, hovered, dragged)

**Different from previous shadow crates:**
- **Not pure data** (like accesskit) — has runtime execution
- **Not computation** (like nalgebra/ndarray) — builds UIs
- **Not layout engine** (like taffy) — uses internal layout
- **Not text engine** (like parley) — uses internal text rendering

**Similar characteristics:**
- **Natural serialization**: Widget descriptions are primitives/enums
- **Synchronous operations**: UI building is deterministic per frame
- **Clear API flow**: Create context → Build UI → Get response
- **Builder patterns**: Widgets use builder API

**Perfect fit for:**
- AI-driven UI generation (generate egui UIs from descriptions)
- Interactive tools (agents create custom UIs for users)
- Dashboard generation (monitoring, admin panels)
- Form builders (generate data entry UIs)
- Debug UIs (inspection panels, profilers)
- Prototyping (rapid UI iteration)

---

## Core Constraint: Immediate Mode + Platform Integration

egui's immediate mode pattern:

```rust
// Every frame (~60 FPS):
ctx.run(raw_input, |ctx| {
    egui::Window::new("My Window").show(ctx, |ui| {
        if ui.button("Click me").clicked() {
            // Handle click
        }
    });
});
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Widget descriptions | `Button`, `Label`, `Slider` params | ✅ Dual-mode (serialize as JSON) |
| Container descriptions | `Window`, `Panel`, `ScrollArea` | ✅ Dual-mode (serialize as JSON) |
| Response state | `clicked()`, `hovered()`, `dragged()` | ✅ Dual-mode (serialize boolean) |
| Context state | `Context`, per-frame state | ⚠️ Runtime-only (platform integration) |
| Closures | `Window::show(ctx, \|ui\| { ... })` | ⚠️ Cannot serialize (use fragment tools) |
| User state | Application data between frames | ✅ Runtime-only (UUID-keyed registry) |
| Event loop | Platform event handling | ⚠️ Runtime-only (eframe/egui_web) |

**Key insight:** egui is **dual nature**:
- **Widget descriptions** serialize (Button with text, Slider with range)
- **Runtime execution** requires platform (can't run headless in MCP)
- **Code generation** is primary use case (agents generate egui apps)

This makes egui **dual-mode dominated** for widget creation, with **fragment tools** for complete apps.

---

## Tool Breakdown: 420 Total

### Dual-Mode Tools (340)

Tools that both create widget/container JSON AND generate code:

#### Basic Widgets (50)
- `widget_label` — Display text
- `widget_button` — Clickable button
- `widget_small_button` — Small button
- `widget_checkbox` — Boolean toggle
- `widget_radio` — Radio button
- `widget_radio_value` — Radio with value assignment
- `widget_selectable_label` — Clickable label
- `widget_selectable_value` — Label with value
- `widget_link` — Hyperlink-styled text
- `widget_hyperlink` — Web link
- `widget_image` — Display image
- `widget_separator` — Horizontal/vertical divider
- `widget_spinner` — Loading spinner
- `widget_heading` — Large text
- `widget_monospace` — Fixed-width text
- `widget_code` — Code text with background
- `widget_small` — Small text
- `widget_strong` — Bold text
- `widget_weak` — Faint text
- `widget_colored_label` — Colored text
- And 30 more for widget variants

#### Text Input Widgets (20)
- `widget_text_edit_singleline` — Single-line input
- `widget_text_edit_multiline` — Multi-line input
- `widget_code_editor` — Code editor with tabs
- `text_edit_hint` — Set placeholder text
- `text_edit_interactive` — Set editable state
- `text_edit_text_color` — Set text color
- `text_edit_font` — Set font
- `text_edit_margin` — Set margins
- And 12 more for text edit properties

#### Numeric Widgets (30)
- `widget_slider` — Numeric slider
- `widget_slider_vertical` — Vertical slider
- `widget_slider_range` — Range slider (min/max)
- `widget_drag_value` — Draggable numeric input
- `widget_drag_angle` — Angle in degrees
- `widget_drag_angle_tau` — Angle as tau fraction
- `slider_set_range` — Set min/max
- `slider_set_step` — Set increment
- `slider_show_value` — Toggle value display
- `slider_text` — Set label text
- `slider_suffix` — Add unit suffix
- `slider_prefix` — Add prefix
- `drag_value_speed` — Set drag speed
- `drag_value_clamp_range` — Clamp to range
- And 16 more for numeric widget properties

#### Color Widgets (15)
- `widget_color_edit_button_srgba` — sRGB color picker
- `widget_color_edit_button_hsva` — HSV color picker
- `widget_color_edit_button_rgb` — Linear RGB picker
- `widget_color_edit_button_srgb` — sRGB picker
- `widget_color_edit_button_rgba` — RGBA picker
- `color_picker_alpha` — Enable alpha channel
- `color_picker_format` — Set color format
- And 8 more for color picker options

#### Progress/Status Widgets (10)
- `widget_progress_bar` — Show progress (0.0-1.0)
- `progress_bar_text` — Set text overlay
- `progress_bar_animate` — Enable animation
- `progress_bar_fill` — Set fill color
- And 6 more for progress options

#### Containers (40)
- `container_window` — Floating window
- `container_area` — Positioned overlay
- `container_side_panel` — Left/right panel
- `container_top_panel` — Top panel
- `container_bottom_panel` — Bottom panel
- `container_central_panel` — Central fill panel
- `container_scroll_area` — Scrollable region
- `container_collapsing` — Collapsible section
- `container_group` — Visual grouping
- `container_frame` — Framed region
- `window_title` — Set window title
- `window_default_pos` — Set initial position
- `window_default_size` — Set initial size
- `window_resizable` — Enable resizing
- `window_collapsible` — Enable collapse
- `window_scroll` — Enable scrolling
- `window_vscroll` — Vertical scroll only
- `window_hscroll` — Horizontal scroll only
- `window_title_bar` — Show/hide title bar
- `scroll_area_max_height` — Set max height
- `scroll_area_max_width` — Set max width
- `scroll_area_auto_shrink` — Shrink to content
- And 18 more for container properties

#### Layout (35)
- `layout_horizontal` — Horizontal layout
- `layout_vertical` — Vertical layout
- `layout_horizontal_centered` — Centered horizontal
- `layout_vertical_centered` — Centered vertical
- `layout_horizontal_justified` — Justified horizontal
- `layout_vertical_justified` — Justified vertical
- `layout_spacing` — Set spacing between widgets
- `layout_indent` — Add indentation
- `layout_add_space` — Add explicit spacing
- `layout_separator` — Add separator
- `layout_columns` — Create columns
- `layout_grid` — Grid layout
- `layout_allocate_space` — Reserve space
- `layout_allocate_exact_size` — Reserve exact size
- `layout_end_row` — End current row (grid)
- And 20 more for layout control

#### Grid Layout (20)
- `grid_new` — Create grid
- `grid_num_columns` — Set column count
- `grid_min_col_width` — Set minimum width
- `grid_max_col_width` — Set maximum width
- `grid_spacing` — Set cell spacing
- `grid_striped` — Alternate row colors
- `grid_row` — Add row
- `grid_col` — Add column
- And 12 more for grid options

#### Styling (40)
- `style_spacing` — Set spacing values
- `style_visuals` — Set visual theme
- `style_text_styles` — Set text styles
- `style_override` — Override style
- `style_dark_mode` — Enable dark mode
- `style_light_mode` — Enable light mode
- `visuals_widgets` — Widget visual settings
- `visuals_selection` — Selection colors
- `visuals_hyperlink_color` — Hyperlink color
- `visuals_faint_bg_color` — Faint background
- `visuals_extreme_bg_color` — Extreme background
- `visuals_code_bg_color` — Code background
- `visuals_window_rounding` — Window corner rounding
- `visuals_window_shadow` — Window shadow
- `visuals_popup_shadow` — Popup shadow
- And 25 more for visual properties

#### Response Checking (30)
- `response_clicked` — Check if clicked
- `response_double_clicked` — Check double click
- `response_triple_clicked` — Check triple click
- `response_middle_clicked` — Check middle button
- `response_secondary_clicked` — Check right click
- `response_hovered` — Check if hovered
- `response_contains_pointer` — Check pointer inside
- `response_dragged` — Check if being dragged
- `response_drag_started` — Check drag start
- `response_drag_stopped` — Check drag stop
- `response_drag_delta` — Get drag movement
- `response_is_pointer_button_down_on` — Check button held
- `response_interact_pointer_pos` — Get pointer position
- `response_changed` — Check if value changed
- `response_gained_focus` — Check focus gain
- `response_lost_focus` — Check focus loss
- `response_has_focus` — Check current focus
- `response_request_focus` — Request focus
- `response_surrender_focus` — Release focus
- And 11 more for response queries

#### Context Menu & Tooltips (20)
- `response_context_menu` — Add context menu
- `response_on_hover_text` — Add hover tooltip
- `response_on_hover_ui` — Add hover UI
- `response_on_disabled_hover_text` — Disabled tooltip
- `tooltip_text` — Show tooltip with text
- `tooltip_ui` — Show tooltip with UI
- And 14 more for menu/tooltip options

#### Input/Events (10)
- `input_key_pressed` — Check key press
- `input_key_down` — Check key held
- `input_modifiers` — Check modifier keys
- `input_pointer_pos` — Get pointer position
- `input_scroll_delta` — Get scroll amount
- And 5 more for input queries

### Runtime-Only Tools (50)

Platform integration and stateful UI execution:

#### Context Management (15)
- `context_create` — Create egui Context
- `context_delete` — Remove context
- `context_begin_frame` — Start frame
- `context_end_frame` — End frame, get output
- `context_run` — Run UI closure for frame
- `context_set_pixels_per_point` — Set DPI scale
- `context_request_repaint` — Request redraw
- `context_memory` — Access persistent memory
- And 7 more for context operations

#### App State Registry (20)
- `app_state_create` — Create persistent app state
- `app_state_get` — Get state by handle
- `app_state_set` — Update state
- `app_state_delete` — Remove state
- `app_run_frame` — Run UI with state
- And 15 more for state management

#### Platform Integration (15)
- `platform_run_native` — Run with eframe (native)
- `platform_run_web` — Run with egui_web
- `platform_handle_input` — Process input events
- `platform_paint` — Render to screen
- `platform_clipboard_get` — Get clipboard
- `platform_clipboard_set` — Set clipboard
- And 9 more for platform operations

### Fragment Tools (30)

Code generation for complete egui applications:

#### Widget Code Generation (10)
- `emit_widget_builder` — Generate widget builder code
- `emit_button` — Generate button code
- `emit_slider` — Generate slider code
- `emit_text_edit` — Generate text edit code
- And 6 more for widget code

#### Container Code Generation (10)
- `emit_window` — Generate window code
- `emit_panel` — Generate panel code
- `emit_scroll_area` — Generate scroll area code
- And 7 more for container code

#### Complete App Assembly (10)
- `assemble_egui_native_app` — Generate eframe app
- `assemble_egui_web_app` — Generate web app
- `emit_app_struct` — Generate app state struct
- `emit_update_method` — Generate update() method
- And 6 more for app assembly

---

## Serialization Strategy

### Widget JSON

```json
{
  "type": "Button",
  "text": "Click Me",
  "wrap": false,
  "fill": null,
  "stroke": null,
  "sense": "click"
}
```

### Slider JSON

```json
{
  "type": "Slider",
  "value": 50.0,
  "range": { "min": 0.0, "max": 100.0 },
  "step": 1.0,
  "text": "Volume",
  "suffix": "%",
  "show_value": true,
  "logarithmic": false,
  "smart_aim": true
}
```

### Window JSON

```json
{
  "type": "Window",
  "title": "Settings",
  "default_pos": { "x": 100.0, "y": 100.0 },
  "default_size": { "width": 400.0, "height": 300.0 },
  "resizable": true,
  "collapsible": true,
  "title_bar": true,
  "scroll": true,
  "children": [
    {
      "type": "Label",
      "text": "Volume Settings"
    },
    {
      "type": "Slider",
      "value": 75.0,
      "range": { "min": 0.0, "max": 100.0 }
    }
  ]
}
```

### Response JSON

```json
{
  "clicked": true,
  "double_clicked": false,
  "hovered": false,
  "dragged": false,
  "drag_delta": { "x": 0.0, "y": 0.0 },
  "changed": true,
  "gained_focus": false,
  "lost_focus": false,
  "rect": {
    "min": { "x": 10.0, "y": 20.0 },
    "max": { "x": 110.0, "y": 50.0 }
  }
}
```

### UI Tree (Declarative Representation)

```json
{
  "windows": [
    {
      "title": "Main Window",
      "children": [
        {
          "layout": "Vertical",
          "children": [
            { "type": "Heading", "text": "User Profile" },
            { "type": "Separator" },
            {
              "layout": "Horizontal",
              "children": [
                { "type": "Label", "text": "Name:" },
                { "type": "TextEdit", "id": "name_input", "value": "John Doe" }
              ]
            },
            {
              "type": "Button",
              "text": "Save",
              "action": "save_profile"
            }
          ]
        }
      ]
    }
  ]
}
```

---

## Phase 1: Widget Dual-Mode Tools

**Goal:** Establish dual-mode pattern for widget creation.

### Crate Structure

```
crates/elicit_egui/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── widget_tools.rs      # Dual-mode widget creation
    ├── container_tools.rs   # Dual-mode containers
    ├── layout_tools.rs      # Dual-mode layout
    ├── style_tools.rs       # Dual-mode styling
    ├── response_tools.rs    # Dual-mode response checking
    ├── context_tools.rs     # Runtime context management
    ├── app_registry.rs      # Runtime app state
    ├── fragments.rs         # Code generation
    └── serde_types.rs       # JSON wrappers
```

### Cargo.toml

```toml
[package]
name = "elicit_egui"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
egui = "0.33"
eframe = { version = "0.33", optional = true }  # For native runtime
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
runtime = ["dep:eframe"]  # Enable runtime UI display
```

### Dual-Mode Tool Example: Button

```rust
use elicitation_derive::elicit_tool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonParams {
    pub text: String,
    pub wrap: Option<bool>,
    pub fill: Option<ColorJson>,
    pub stroke: Option<StrokeJson>,
}

#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_button",
    description = "Create clickable button",
    emit = Auto
)]
async fn widget_button(p: ButtonParams) -> Result<CallToolResult, ErrorData> {
    let widget_json = WidgetJson::Button {
        text: p.text.clone(),
        wrap: p.wrap.unwrap_or(false),
        fill: p.fill.clone(),
        stroke: p.stroke.clone(),
    };

    Ok(CallToolResult::success(json!({ "widget": widget_json })))
}

impl CustomEmit<ButtonParams> for WidgetButtonEmit {
    fn emit_code(params: &ButtonParams) -> TokenStream {
        let text = &params.text;
        let mut builder_calls = vec![];

        if let Some(wrap) = params.wrap {
            builder_calls.push(quote! { .wrap(#wrap) });
        }

        quote! {
            ui.button(#text)#(#builder_calls)*
        }
    }
}
```

### WidgetJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WidgetJson {
    Label {
        text: String,
        wrap: bool,
        color: Option<ColorJson>,
    },
    Button {
        text: String,
        wrap: bool,
        fill: Option<ColorJson>,
        stroke: Option<StrokeJson>,
    },
    Checkbox {
        text: String,
        checked: bool,
    },
    Slider {
        value: f64,
        range: RangeJson,
        step: Option<f64>,
        text: Option<String>,
        suffix: Option<String>,
    },
    TextEdit {
        id: String,
        value: String,
        multiline: bool,
        hint: Option<String>,
    },
    // ... and 40+ more variants
}
```

### Dual-Mode Tool Example: Slider

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderParams {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: Option<f64>,
    pub text: Option<String>,
    pub suffix: Option<String>,
    pub logarithmic: Option<bool>,
}

#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_slider",
    description = "Create numeric slider",
    emit = Auto
)]
async fn widget_slider(p: SliderParams) -> Result<CallToolResult, ErrorData> {
    let widget_json = WidgetJson::Slider {
        value: p.value,
        range: RangeJson { min: p.min, max: p.max },
        step: p.step,
        text: p.text.clone(),
        suffix: p.suffix.clone(),
    };

    Ok(CallToolResult::success(json!({ "widget": widget_json })))
}

impl CustomEmit<SliderParams> for WidgetSliderEmit {
    fn emit_code(params: &SliderParams) -> TokenStream {
        let min = params.min;
        let max = params.max;
        let mut builder_calls = vec![];

        if let Some(ref text) = params.text {
            builder_calls.push(quote! { .text(#text) });
        }
        if let Some(ref suffix) = params.suffix {
            builder_calls.push(quote! { .suffix(#suffix) });
        }
        if let Some(step) = params.step {
            builder_calls.push(quote! { .step_by(#step) });
        }
        if let Some(logarithmic) = params.logarithmic {
            if logarithmic {
                builder_calls.push(quote! { .logarithmic(true) });
            }
        }

        quote! {
            ui.add(egui::Slider::new(&mut value, #min..=#max)#(#builder_calls)*)
        }
    }
}
```

---

## Phase 2: Container Dual-Mode Tools

**Goal:** Windows, panels, scroll areas, etc.

### Dual-Mode Tool Example: Window

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowParams {
    pub title: String,
    pub default_pos: Option<Pos2Json>,
    pub default_size: Option<Vec2Json>,
    pub resizable: Option<bool>,
    pub collapsible: Option<bool>,
    pub scroll: Option<bool>,
}

#[elicit_tool(
    plugin = "egui_containers",
    name = "container_window",
    description = "Create floating window",
    emit = Auto
)]
async fn container_window(p: WindowParams) -> Result<CallToolResult, ErrorData> {
    let container_json = ContainerJson::Window {
        title: p.title.clone(),
        default_pos: p.default_pos.clone(),
        default_size: p.default_size.clone(),
        resizable: p.resizable.unwrap_or(true),
        collapsible: p.collapsible.unwrap_or(true),
        scroll: p.scroll.unwrap_or(false),
        children: vec![],
    };

    Ok(CallToolResult::success(json!({ "container": container_json })))
}

impl CustomEmit<WindowParams> for ContainerWindowEmit {
    fn emit_code(params: &WindowParams) -> TokenStream {
        let title = &params.title;
        let mut builder_calls = vec![];

        if let Some(ref pos) = params.default_pos {
            let x = pos.x;
            let y = pos.y;
            builder_calls.push(quote! { .default_pos([#x, #y]) });
        }
        if let Some(ref size) = params.default_size {
            let w = size.x;
            let h = size.y;
            builder_calls.push(quote! { .default_size([#w, #h]) });
        }
        if let Some(resizable) = params.resizable {
            builder_calls.push(quote! { .resizable(#resizable) });
        }
        if let Some(collapsible) = params.collapsible {
            builder_calls.push(quote! { .collapsible(#collapsible) });
        }
        if let Some(scroll) = params.scroll {
            if scroll {
                builder_calls.push(quote! { .scroll(true) });
            }
        }

        quote! {
            egui::Window::new(#title)#(#builder_calls)*
                .show(ctx, |ui| {
                    // Children widgets go here
                })
        }
    }
}
```

---

## Phase 3: Runtime Context Management

**Goal:** Actually run egui UIs (requires eframe).

### Runtime Tool Example: Create Context

```rust
#[cfg(feature = "runtime")]
pub struct EguiPlugin {
    contexts: Arc<Mutex<HashMap<Uuid, egui::Context>>>,
    app_states: Arc<Mutex<HashMap<Uuid, Box<dyn AppState>>>>,
}

#[cfg(feature = "runtime")]
trait AppState: Send + Sync {
    fn update(&mut self, ctx: &egui::Context);
}

#[cfg(feature = "runtime")]
#[elicit_tool(
    plugin = "egui_context",
    name = "context_create",
    description = "Create egui Context for UI rendering"
)]
async fn context_create(_: ContextCreateParams) -> Result<CallToolResult, ErrorData> {
    let ctx = egui::Context::default();
    let ctx_id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.contexts.lock().unwrap().insert(ctx_id, ctx);

    Ok(CallToolResult::success(json!({ "context_id": ctx_id })))
}

#[cfg(feature = "runtime")]
#[elicit_tool(
    plugin = "egui_context",
    name = "context_run_frame",
    description = "Run one UI frame"
)]
async fn context_run_frame(p: RunFrameParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let contexts = plugin.contexts.lock().unwrap();
    let ctx = contexts.get(&p.context_id)
        .ok_or_else(|| ErrorData::new("Context not found"))?;

    // Build UI from widget JSON
    let ui_tree = p.ui_tree;
    let full_output = ctx.run(egui::RawInput::default(), |ctx| {
        // Render ui_tree into egui calls
        render_ui_tree(ctx, &ui_tree);
    });

    // Serialize output
    let output_json = FullOutputJson::from_output(&full_output);
    Ok(CallToolResult::success(json!({ "output": output_json })))
}
```

---

## Phase 4: Fragment Tools (Complete Apps)

**Goal:** Generate complete egui/eframe applications.

### Fragment Tool Example: Native App

```rust
#[elicit_tool(
    plugin = "egui_fragments",
    name = "assemble_egui_native_app",
    description = "Generate complete eframe native application",
    emit = Auto
)]
async fn assemble_egui_native_app(p: AssembleNativeParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml_native(&p);
    let main_rs = generate_main_native(&p);
    let app_rs = generate_app_struct(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "src/app.rs": app_rs,
        "description": "Complete eframe native app"
    })))
}

fn generate_main_native(p: &AssembleNativeParams) -> String {
    format!(
        r#"mod app;

fn main() -> Result<(), eframe::Error> {{
    let options = eframe::NativeOptions {{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([{}, {}]),
        ..Default::default()
    }};

    eframe::run_native(
        "{}",
        options,
        Box::new(|cc| Ok(Box::new(app::{}::new(cc)))),
    )
}}
"#,
        p.window_width,
        p.window_height,
        p.app_name,
        p.app_struct_name
    )
}

fn generate_app_struct(p: &AssembleNativeParams) -> String {
    format!(
        r#"use eframe::egui;

pub struct {} {{
    {}
}}

impl {} {{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {{
        Self {{
            {}
        }}
    }}
}}

impl eframe::App for {} {{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {{
        {}
    }}
}}
"#,
        p.app_struct_name,
        emit_struct_fields(&p.state_fields),
        p.app_struct_name,
        emit_field_init(&p.state_fields),
        p.app_struct_name,
        emit_ui_code(&p.ui_tree)
    )
}

fn emit_ui_code(ui_tree: &UiTreeJson) -> String {
    let mut code = String::new();

    for window in &ui_tree.windows {
        code.push_str(&format!(
            r#"        egui::Window::new("{}")
            .show(ctx, |ui| {{
{}
            }});
"#,
            window.title,
            emit_window_children(&window.children, 4)
        ));
    }

    code
}

fn emit_window_children(children: &[WidgetJson], indent: usize) -> String {
    let indent_str = " ".repeat(indent);
    let mut code = String::new();

    for child in children {
        match child {
            WidgetJson::Label { text, .. } => {
                code.push_str(&format!("{}ui.label(\"{}\");\n", indent_str, text));
            }
            WidgetJson::Button { text, .. } => {
                code.push_str(&format!(
                    "{}if ui.button(\"{}\").clicked() {{\n",
                    indent_str, text
                ));
                code.push_str(&format!("{}    // Handle click\n", indent_str));
                code.push_str(&format!("{}}}\n", indent_str));
            }
            WidgetJson::Slider { value, range, text, .. } => {
                code.push_str(&format!(
                    "{}ui.add(egui::Slider::new(&mut self.{}_value, {}..={})",
                    indent_str,
                    text.as_ref().map(|s| s.to_lowercase()).unwrap_or("slider".to_string()),
                    range.min,
                    range.max
                ));
                if let Some(ref label) = text {
                    code.push_str(&format!(".text(\"{}\")", label));
                }
                code.push_str(");\n");
            }
            // ... handle all widget types
            _ => {}
        }
    }

    code
}
```

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Basic widget dual-mode tools: (50 tools)
3. **Phase 1c** — Text input widget dual-mode tools: (20 tools)
4. **Phase 1d** — `just check elicit_egui`
5. **Phase 2a** — Numeric widget dual-mode tools: (30 tools)
6. **Phase 2b** — Color widget dual-mode tools: (15 tools)
7. **Phase 2c** — Progress/status widget dual-mode tools: (10 tools)
8. **Phase 2d** — `just check elicit_egui`
9. **Phase 3a** — Container dual-mode tools: (40 tools)
10. **Phase 3b** — Layout dual-mode tools: (35 tools)
11. **Phase 3c** — Grid layout dual-mode tools: (20 tools)
12. **Phase 3d** — `just check elicit_egui`
13. **Phase 4a** — Styling dual-mode tools: (40 tools)
14. **Phase 4b** — Response checking dual-mode tools: (30 tools)
15. **Phase 4c** — Context menu/tooltip dual-mode tools: (20 tools)
16. **Phase 4d** — `just check elicit_egui`
17. **Phase 5a** — Input/event dual-mode tools: (10 tools)
18. **Phase 5b** — `just check elicit_egui`
19. **Phase 6a** — Runtime context management: (15 tools) — requires `runtime` feature
20. **Phase 6b** — Runtime app state registry: (20 tools) — requires `runtime` feature
21. **Phase 6c** — Runtime platform integration: (15 tools) — requires `runtime` feature
22. **Phase 6d** — `just check elicit_egui --features runtime`
23. **Phase 7a** — Fragment widget code generation: (10 tools)
24. **Phase 7b** — Fragment container code generation: (10 tools)
25. **Phase 7c** — Fragment complete app assembly: (10 tools)
26. **Phase 7d** — `just check elicit_egui --all-features`
27. **Phase 8** — Wire into `elicit_server` emit chain

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Dual-Mode Basic Widgets | 50 | `emit = Auto` + CustomEmit |
| Dual-Mode Text Input | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Numeric Widgets | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Color Widgets | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Progress Widgets | 10 | `emit = Auto` + CustomEmit |
| Dual-Mode Containers | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Layout | 35 | `emit = Auto` + CustomEmit |
| Dual-Mode Grid | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Styling | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Response Checking | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Menus/Tooltips | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Input/Events | 10 | `emit = Auto` + CustomEmit |
| Runtime Context Management | 15 | UUID → Context mapping (feature-gated) |
| Runtime App State | 20 | UUID → AppState mapping (feature-gated) |
| Runtime Platform Integration | 15 | eframe/egui_web (feature-gated) |
| Fragment Widget Code | 10 | Code generation only |
| Fragment Container Code | 10 | Code generation only |
| Fragment App Assembly | 10 | Code generation only |
| **Total** | **420** | |

---

## Key Advantages

1. **Immediate Mode**: Simple mental model (rebuild UI every frame)
2. **Cross-Platform**: Native (Windows/macOS/Linux) and web (WASM)
3. **No External Dependencies**: Pure Rust, no OS widgets
4. **Rapid Prototyping**: Quick UI iteration
5. **AI-Friendly**: Imperative API suits code generation
6. **Self-Contained**: egui doesn't need external UI frameworks
7. **Accessible**: Optional accesskit integration
8. **Productive**: Rich widget library out of the box

---

## Comparison to Other Shadow Crates

| Aspect | taffy | accesskit | egui |
|--------|-------|-----------|------|
| **Domain** | Box layout | Accessibility | GUI framework |
| **Purpose** | Layout algorithm | Semantic metadata | Complete UIs |
| **Statefulness** | Tree mutations | Optional | Per-frame (immediate mode) |
| **Runtime tools** | 53% | 7% | 12% |
| **Dual-mode tools** | 35% | 87% | 81% |
| **Total tools** | 340 | 450 | 420 |
| **Output** | Box positions | Accessibility tree | Interactive UI |
| **Use case** | UI layout engine | Screen readers | Complete apps |

**egui is unique:**
- **Complete framework** (not just one piece like layout/text/accessibility)
- **Immediate mode** (stateless widgets recreated every frame)
- **Feature-gated runtime** (can run headless for code gen, or with display)
- **High dual-mode ratio** (81%) — widget descriptions serialize well

**Shares "straightforward" DNA:**
- Natural JSON serialization for widget descriptions
- Synchronous operations
- Builder patterns
- Clear API taxonomy

---

## Integration with Linebender Stack

### Complementary Usage

egui is an **alternative** to the linebender stack (taffy/parley/accesskit/vello):

| Stack | Approach | Use Case |
|-------|----------|----------|
| **Linebender** | Composition (taffy + parley + accesskit + vello) | Custom renderers, game engines, specialized UIs |
| **egui** | All-in-one immediate mode | Tools, editors, debug UIs, dashboards |

Both can use accesskit:
- egui has built-in accesskit support
- Linebender apps use accesskit directly

### Hybrid Use Cases

Possible to combine:
- Use egui for tool UI, vello for custom graphics
- Use taffy for main layout, egui for inspector panels
- Use parley for document text, egui for controls

---

## Use Cases

### AI-Generated UIs

Agents can generate egui UIs from natural language:

```json
{
  "intent": "Create settings panel with volume slider and theme toggle",
  "ui": {
    "windows": [
      {
        "title": "Settings",
        "children": [
          { "type": "Heading", "text": "Audio" },
          {
            "type": "Slider",
            "value": 75.0,
            "range": { "min": 0.0, "max": 100.0 },
            "text": "Volume",
            "suffix": "%"
          },
          { "type": "Separator" },
          { "type": "Heading", "text": "Appearance" },
          {
            "type": "Checkbox",
            "text": "Dark Mode",
            "checked": true
          }
        ]
      }
    ]
  }
}
```

Agent generates complete eframe app from this JSON.

### Interactive Tools

Agents create custom UIs for specific tasks:
- Database query builder
- Log file viewer with filters
- Configuration editor
- Asset browser

### Dashboard Generation

Monitoring dashboards from data sources:
- Real-time metrics visualization
- System status panels
- Performance graphs
- Alert displays

### Form Builders

Generate data entry UIs from schemas:
- User registration forms
- Survey builders
- Admin panels
- CRUD interfaces

---

## Sources

- [egui - Rust (docs.rs)](https://docs.rs/egui/latest/egui/)
- [Ui - Rust](https://docs.rs/egui/latest/egui/struct.Ui.html)
- [widgets - Rust](https://docs.rs/egui/latest/egui/widgets/index.html)
- [GitHub - emilk/egui](https://github.com/emilk/egui)
- [egui Official Site](https://www.egui.rs/)
- [egui - crates.io](https://crates.io/crates/egui)
