//! Code generation tools for egui/eframe application fragments.
//!
//! Each tool generates idiomatic Rust source code strings that users can
//! paste directly into their egui/eframe projects.  The generated code
//! includes the necessary `use` imports so fragments are self-contained.

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

fn code_result(code: &str) -> CallToolResult {
    match serde_json::to_string(&serde_json::json!({ "code": code })) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

fn to_snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// App scaffolding
// ---------------------------------------------------------------------------

/// Parameters for [`egui_fragment_native_app`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct NativeAppParams {
    /// Application struct name (e.g. `MyApp`).
    pub app_name: String,
    /// Window title shown in the title bar.
    pub window_title: String,
    /// Initial window width in logical pixels.
    pub window_width: f32,
    /// Initial window height in logical pixels.
    pub window_height: f32,
    /// Whether to use dark mode (true) or light mode (false).
    #[serde(default = "default_true")]
    pub dark_mode: bool,
}

fn default_true() -> bool {
    true
}

/// Generate complete eframe native app boilerplate (main.rs + app.rs).
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_native_app",
    description = "Generate complete eframe native application boilerplate with main.rs and app.rs.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_native_app(p: NativeAppParams) -> Result<CallToolResult, ErrorData> {
    let theme_call = if p.dark_mode {
        "egui::Visuals::dark()"
    } else {
        "egui::Visuals::light()"
    };

    let main_rs = format!(
        r#"use eframe::{{self, egui}};

mod app;
use app::{app_name};

fn main() -> eframe::Result {{
    let options = eframe::NativeOptions {{
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([{width:.1}, {height:.1}]),
        ..Default::default()
    }};
    eframe::run_native(
        "{title}",
        options,
        Box::new(|cc| Ok(Box::new({app_name}::new(cc)))),
    )
}}"#,
        app_name = p.app_name,
        title = p.window_title,
        width = p.window_width,
        height = p.window_height,
    );

    let app_rs = format!(
        r#"use eframe::egui;

pub struct {app_name} {{
    // Add your application state here.
}}

impl {app_name} {{
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {{
        cc.egui_ctx.set_visuals({theme});
        Self {{}}
    }}
}}

impl eframe::App for {app_name} {{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {{
        egui::CentralPanel::default().show(ctx, |ui| {{
            ui.heading("{title}");
            ui.label("Hello from {app_name}!");
        }});
    }}
}}"#,
        app_name = p.app_name,
        theme = theme_call,
        title = p.window_title,
    );

    let json = serde_json::json!({
        "main_rs": main_rs,
        "app_rs": app_rs,
    });
    match serde_json::to_string(&json) {
        Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "serialize error: {e}"
        ))])),
    }
}

/// Parameters for [`egui_fragment_web_app`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WebAppParams {
    /// Application struct name (e.g. `MyWebApp`).
    pub app_name: String,
    /// HTML canvas element ID for rendering.
    pub canvas_id: String,
}

/// Generate eframe WASM web app boilerplate.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_web_app",
    description = "Generate eframe WASM web application boilerplate for browser deployment.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_web_app(p: WebAppParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"use eframe::{{self, egui}};

pub struct {app_name} {{
    // Add your application state here.
}}

impl {app_name} {{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {{
        Self {{}}
    }}
}}

impl eframe::App for {app_name} {{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {{
        egui::CentralPanel::default().show(ctx, |ui| {{
            ui.heading("Hello from {app_name}!");
        }});
    }}
}}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{{self, prelude::*}};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {{
    let web_options = eframe::WebOptions::default();
    eframe::WebRunner::new()
        .start(
            canvas_id,
            web_options,
            Box::new(|cc| Ok(Box::new({app_name}::new(cc)))),
        )
        .await
}}

// Call from HTML/JS:
// ```html
// <canvas id="{canvas_id}"></canvas>
// <script>wasm_bindgen("./my_app_bg.wasm").then(() => start("{canvas_id}"));</script>
// ```"#,
        app_name = p.app_name,
        canvas_id = p.canvas_id,
    );
    Ok(code_result(&code))
}

// ---------------------------------------------------------------------------
// Widget code generation
// ---------------------------------------------------------------------------

/// A single field in a generated form.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct FormFieldDef {
    /// Rust field name (snake_case).
    pub name: String,
    /// Field type: `"text"`, `"number"`, `"checkbox"`, `"slider"`, or `"color"`.
    pub field_type: String,
    /// Human-readable label shown in the UI.
    pub label: String,
}

/// Parameters for [`egui_fragment_form`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct FormParams {
    /// Struct name for the form state (e.g. `UserForm`).
    pub struct_name: String,
    /// Fields to generate in the form.
    pub fields: Vec<FormFieldDef>,
}

/// Generate code for a form with labelled fields.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_form",
    description = "Generate Rust code for an egui form with labelled input fields (text, number, checkbox, slider, color).",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_form(p: FormParams) -> Result<CallToolResult, ErrorData> {
    let mut struct_fields = String::new();
    let mut defaults = String::new();
    let mut ui_rows = String::new();

    for f in &p.fields {
        let (rust_type, default_val, widget_code) = match f.field_type.as_str() {
            "text" => (
                "String",
                "String::new()".to_owned(),
                format!("ui.text_edit_singleline(&mut self.{name});", name = f.name),
            ),
            "number" => (
                "f64",
                "0.0".to_owned(),
                format!(
                    "ui.add(egui::DragValue::new(&mut self.{name}));",
                    name = f.name
                ),
            ),
            "checkbox" => (
                "bool",
                "false".to_owned(),
                format!("ui.checkbox(&mut self.{name}, \"\");", name = f.name),
            ),
            "slider" => (
                "f64",
                "0.0".to_owned(),
                format!(
                    "ui.add(egui::Slider::new(&mut self.{name}, 0.0..=100.0));",
                    name = f.name
                ),
            ),
            "color" => (
                "[f32; 4]",
                "[1.0, 1.0, 1.0, 1.0]".to_owned(),
                format!(
                    "ui.color_edit_button_rgba_unmultiplied(&mut self.{name});",
                    name = f.name
                ),
            ),
            other => (
                "String",
                "String::new()".to_owned(),
                format!(
                    "ui.label(\"unsupported type: {other}\"); // field: {name}",
                    other = other,
                    name = f.name,
                ),
            ),
        };

        struct_fields.push_str(&format!("    pub {}: {},\n", f.name, rust_type));
        defaults.push_str(&format!("            {}: {},\n", f.name, default_val));
        ui_rows.push_str(&format!(
            "            ui.label(\"{label}\");\n            {widget}\n",
            label = f.label,
            widget = widget_code,
        ));
    }

    let code = format!(
        r#"use eframe::egui;

pub struct {name} {{
{fields}}}

impl Default for {name} {{
    fn default() -> Self {{
        Self {{
{defaults}        }}
    }}
}}

impl {name} {{
    pub fn ui(&mut self, ui: &mut egui::Ui) {{
        egui::Grid::new("{name}_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {{
{ui_rows}        }});
    }}
}}"#,
        name = p.struct_name,
        fields = struct_fields,
        defaults = defaults,
        ui_rows = ui_rows
            .lines()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|pair| format!("{}\n{}\n                ui.end_row();\n", pair[0], pair[1]))
            .collect::<String>(),
    );
    Ok(code_result(&code))
}

/// A column definition for a generated table.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct TableColumnDef {
    /// Rust field name on the row struct (snake_case).
    pub name: String,
    /// Column header text shown in the UI.
    pub header: String,
}

/// Parameters for [`egui_fragment_table`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TableParams {
    /// Widget struct name (e.g. `UserTable`).
    pub struct_name: String,
    /// Columns to display.
    pub columns: Vec<TableColumnDef>,
    /// Name of the row data type (e.g. `UserRow`).
    pub row_type_name: String,
}

/// Generate code for a data table widget.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_table",
    description = "Generate Rust code for an egui data table with typed rows and column headers.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_table(p: TableParams) -> Result<CallToolResult, ErrorData> {
    let mut row_fields = String::new();
    let mut header_cells = String::new();
    let mut body_cells = String::new();

    for col in &p.columns {
        row_fields.push_str(&format!("    pub {}: String,\n", col.name));
        header_cells.push_str(&format!(
            "                        header.col(|ui| {{ ui.strong(\"{}\"); }});\n",
            col.header,
        ));
        body_cells.push_str(&format!(
            "                            row.col(|ui| {{ ui.label(&item.{}); }});\n",
            col.name,
        ));
    }

    let num_cols = p.columns.len();
    let code = format!(
        r#"use eframe::egui;
use egui_extras::{{TableBuilder, Column}};

#[derive(Debug, Clone)]
pub struct {row_type} {{
{row_fields}}}

pub struct {name} {{
    pub rows: Vec<{row_type}>,
}}

impl {name} {{
    pub fn new(rows: Vec<{row_type}>) -> Self {{
        Self {{ rows }}
    }}

    pub fn ui(&self, ui: &mut egui::Ui) {{
        let available = ui.available_size();
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
{columns}            .header(20.0, |mut header| {{
{header_cells}            }})
            .body(|body| {{
                body.rows(18.0, self.rows.len(), |mut row| {{
                    let item = &self.rows[row.index()];
{body_cells}                }});
            }});
    }}
}}"#,
        name = p.struct_name,
        row_type = p.row_type_name,
        row_fields = row_fields,
        columns = (0..num_cols)
            .map(|_| "            .column(Column::auto().resizable(true))\n".to_owned())
            .collect::<String>(),
        header_cells = header_cells,
        body_cells = body_cells,
    );
    Ok(code_result(&code))
}

/// A field definition within a settings section.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct SettingsFieldDef {
    /// Rust field name (snake_case).
    pub name: String,
    /// Field type: `"text"`, `"number"`, `"checkbox"`, `"slider"`, or `"color"`.
    pub field_type: String,
    /// Human-readable label.
    pub label: String,
    /// Default value as a string literal (e.g. `"true"`, `"42"`, `"hello"`).
    pub default_value: String,
}

/// A named section within a settings panel.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct SettingsSectionDef {
    /// Section name (used as collapsing header text).
    pub name: String,
    /// Fields in this section.
    pub fields: Vec<SettingsFieldDef>,
}

/// Parameters for [`egui_fragment_settings_panel`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SettingsPanelParams {
    /// Panel title shown at the top.
    pub title: String,
    /// Settings sections.
    pub sections: Vec<SettingsSectionDef>,
}

/// Generate code for a settings panel with collapsible sections.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_settings_panel",
    description = "Generate Rust code for an egui settings panel with collapsible sections and typed fields.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_settings_panel(p: SettingsPanelParams) -> Result<CallToolResult, ErrorData> {
    let mut struct_fields = String::new();
    let mut defaults = String::new();
    let mut sections_ui = String::new();

    for section in &p.sections {
        let mut section_fields = String::new();
        for f in &section.fields {
            let (rust_type, default_val, widget) = match f.field_type.as_str() {
                "text" => (
                    "String",
                    format!("\"{}\".to_owned()", f.default_value),
                    format!("ui.text_edit_singleline(&mut self.{});", f.name),
                ),
                "number" => (
                    "f64",
                    f.default_value.clone(),
                    format!("ui.add(egui::DragValue::new(&mut self.{}));", f.name),
                ),
                "checkbox" => (
                    "bool",
                    f.default_value.clone(),
                    format!("ui.checkbox(&mut self.{}, \"\");", f.name),
                ),
                "slider" => (
                    "f64",
                    f.default_value.clone(),
                    format!(
                        "ui.add(egui::Slider::new(&mut self.{}, 0.0..=100.0));",
                        f.name
                    ),
                ),
                "color" => (
                    "[f32; 4]",
                    format!("[{}]", f.default_value),
                    format!(
                        "ui.color_edit_button_rgba_unmultiplied(&mut self.{});",
                        f.name
                    ),
                ),
                _ => (
                    "String",
                    format!("\"{}\".to_owned()", f.default_value),
                    format!("ui.text_edit_singleline(&mut self.{});", f.name),
                ),
            };
            struct_fields.push_str(&format!("    pub {}: {},\n", f.name, rust_type));
            defaults.push_str(&format!("            {}: {},\n", f.name, default_val));
            section_fields.push_str(&format!(
                "                    ui.label(\"{label}\");\n\
                 \x20                   {widget}\n\
                 \x20                   ui.end_row();\n",
                label = f.label,
                widget = widget,
            ));
        }

        sections_ui.push_str(&format!(
            r#"        ui.collapsing("{section_name}", |ui| {{
            egui::Grid::new("{section_name}_grid")
                .num_columns(2)
                .spacing([8.0, 4.0])
                .show(ui, |ui| {{
{fields}                }});
        }});
"#,
            section_name = section.name,
            fields = section_fields,
        ));
    }

    let code = format!(
        r#"use eframe::egui;

pub struct SettingsPanel {{
{struct_fields}}}

impl Default for SettingsPanel {{
    fn default() -> Self {{
        Self {{
{defaults}        }}
    }}
}}

impl SettingsPanel {{
    pub fn ui(&mut self, ui: &mut egui::Ui) {{
        ui.heading("{title}");
        ui.separator();
{sections_ui}    }}
}}"#,
        struct_fields = struct_fields,
        defaults = defaults,
        title = p.title,
        sections_ui = sections_ui,
    );
    Ok(code_result(&code))
}

// ---------------------------------------------------------------------------
// Layout code generation
// ---------------------------------------------------------------------------

/// Parameters for [`egui_fragment_sidebar_layout`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SidebarLayoutParams {
    /// Sidebar width in logical pixels.
    pub sidebar_width: f32,
    /// Menu items displayed in the sidebar.
    pub sidebar_items: Vec<String>,
}

/// Generate code for a sidebar + main content layout.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_sidebar_layout",
    description = "Generate Rust code for an egui sidebar + main content panel layout.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_sidebar_layout(p: SidebarLayoutParams) -> Result<CallToolResult, ErrorData> {
    let mut items_code = String::new();
    for (i, item) in p.sidebar_items.iter().enumerate() {
        items_code.push_str(&format!(
            "                if ui.selectable_label(self.selected == {i}, \"{item}\").clicked() {{\n\
             \x20                   self.selected = {i};\n\
             \x20               }}\n",
            i = i,
            item = item,
        ));
    }

    let mut match_arms = String::new();
    for (i, item) in p.sidebar_items.iter().enumerate() {
        match_arms.push_str(&format!(
            "                {i} => ui.label(\"Content for: {item}\"),\n",
            i = i,
            item = item,
        ));
    }
    match_arms.push_str("                _ => ui.label(\"Select an item\"),\n");

    let code = format!(
        r#"use eframe::egui;

pub struct SidebarLayout {{
    pub selected: usize,
}}

impl Default for SidebarLayout {{
    fn default() -> Self {{
        Self {{ selected: 0 }}
    }}
}}

impl SidebarLayout {{
    pub fn ui(&mut self, ctx: &egui::Context) {{
        egui::SidePanel::left("sidebar")
            .default_width({width:.1})
            .show(ctx, |ui| {{
                ui.heading("Menu");
                ui.separator();
{items}            }});

        egui::CentralPanel::default().show(ctx, |ui| {{
            match self.selected {{
{match_arms}            }};
        }});
    }}
}}"#,
        width = p.sidebar_width,
        items = items_code,
        match_arms = match_arms,
    );
    Ok(code_result(&code))
}

/// A single tab definition.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct TabDef {
    /// Identifier for the tab (used in match arms).
    pub name: String,
    /// Label shown on the tab button.
    pub label: String,
}

/// Parameters for [`egui_fragment_tab_panel`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TabPanelParams {
    /// Tabs to generate.
    pub tabs: Vec<TabDef>,
}

/// Generate code for a tabbed panel.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_tab_panel",
    description = "Generate Rust code for an egui tabbed panel with selectable tabs.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_tab_panel(p: TabPanelParams) -> Result<CallToolResult, ErrorData> {
    let mut variant_list = String::new();
    let mut tab_buttons = String::new();
    let mut tab_content = String::new();

    for tab in &p.tabs {
        let variant = {
            let mut chars = tab.name.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => tab.name.clone(),
            }
        };
        variant_list.push_str(&format!("    {},\n", variant));
        tab_buttons.push_str(&format!(
            "            if ui.selectable_label(matches!(self.active_tab, Tab::{variant}), \"{label}\").clicked() {{\n\
             \x20               self.active_tab = Tab::{variant};\n\
             \x20           }}\n",
            variant = variant,
            label = tab.label,
        ));
        tab_content.push_str(&format!(
            "            Tab::{variant} => {{\n\
             \x20               ui.heading(\"{label}\");\n\
             \x20               ui.label(\"Content for {label}\");\n\
             \x20           }}\n",
            variant = variant,
            label = tab.label,
        ));
    }

    let first_variant = if let Some(tab) = p.tabs.first() {
        let mut chars = tab.name.chars();
        match chars.next() {
            Some(c) => c.to_uppercase().to_string() + chars.as_str(),
            None => tab.name.clone(),
        }
    } else {
        String::new()
    };

    let code = format!(
        r#"use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {{
{variants}}}

pub struct TabPanel {{
    pub active_tab: Tab,
}}

impl Default for TabPanel {{
    fn default() -> Self {{
        Self {{ active_tab: Tab::{first} }}
    }}
}}

impl TabPanel {{
    pub fn ui(&mut self, ui: &mut egui::Ui) {{
        ui.horizontal(|ui| {{
{tab_buttons}        }});
        ui.separator();
        match self.active_tab {{
{tab_content}        }}
    }}
}}"#,
        variants = variant_list,
        first = first_variant,
        tab_buttons = tab_buttons,
        tab_content = tab_content,
    );
    Ok(code_result(&code))
}

/// A single toolbar button definition.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct ToolbarButtonDef {
    /// Button identifier (snake_case, used as method suffix).
    pub name: String,
    /// Button label text.
    pub label: String,
    /// Tooltip text shown on hover.
    pub tooltip: String,
}

/// Parameters for [`egui_fragment_toolbar`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ToolbarParams {
    /// Buttons to include in the toolbar.
    pub buttons: Vec<ToolbarButtonDef>,
}

/// Generate code for a toolbar with buttons.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_toolbar",
    description = "Generate Rust code for an egui toolbar with labelled, tooltipped buttons.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_toolbar(p: ToolbarParams) -> Result<CallToolResult, ErrorData> {
    let mut variant_list = String::new();
    let mut button_code = String::new();

    for btn in &p.buttons {
        let variant = {
            let mut chars = btn.name.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => btn.name.clone(),
            }
        };
        variant_list.push_str(&format!("    {},\n", variant));
        button_code.push_str(&format!(
            "            if ui.button(\"{label}\").on_hover_text(\"{tooltip}\").clicked() {{\n\
             \x20               actions.push(ToolbarAction::{variant});\n\
             \x20           }}\n",
            label = btn.label,
            tooltip = btn.tooltip,
            variant = variant,
        ));
    }

    let code = format!(
        r#"use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarAction {{
{variants}}}

pub struct Toolbar;

impl Toolbar {{
    /// Render the toolbar and return any actions triggered this frame.
    pub fn ui(ui: &mut egui::Ui) -> Vec<ToolbarAction> {{
        let mut actions = Vec::new();
        ui.horizontal(|ui| {{
{buttons}        }});
        actions
    }}
}}"#,
        variants = variant_list,
        buttons = button_code,
    );
    Ok(code_result(&code))
}

// ---------------------------------------------------------------------------
// State management
// ---------------------------------------------------------------------------

/// A single field in a generated state struct.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct StateFieldDef {
    /// Rust field name (snake_case).
    pub name: String,
    /// Rust type expression (e.g. `String`, `Vec<u32>`, `bool`).
    pub rust_type: String,
    /// Default value as a Rust expression (e.g. `String::new()`, `false`, `42`).
    pub default_value: String,
}

/// Parameters for [`egui_fragment_app_state`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AppStateParams {
    /// Struct name (e.g. `AppState`).
    pub struct_name: String,
    /// Fields in the state struct.
    pub fields: Vec<StateFieldDef>,
}

/// Generate an application state struct with defaults.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_app_state",
    description = "Generate a Rust app state struct with typed fields and Default implementation for egui apps.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_app_state(p: AppStateParams) -> Result<CallToolResult, ErrorData> {
    let mut struct_fields = String::new();
    let mut defaults = String::new();

    for f in &p.fields {
        struct_fields.push_str(&format!("    pub {}: {},\n", f.name, f.rust_type));
        defaults.push_str(&format!("            {}: {},\n", f.name, f.default_value));
    }

    let snake = to_snake(&p.struct_name);
    let code = format!(
        r#"/// Application state for the egui app.
#[derive(Debug, Clone)]
pub struct {name} {{
{fields}}}

impl Default for {name} {{
    fn default() -> Self {{
        Self {{
{defaults}        }}
    }}
}}

impl {name} {{
    /// Create a new instance with default values.
    pub fn new() -> Self {{
        Self::default()
    }}
}}

// Usage in your eframe::App:
// ```
// struct MyApp {{
//     state: {name},
// }}
//
// impl eframe::App for MyApp {{
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {{
//         let {snake} = &mut self.state;
//         // use {snake}.field_name ...
//     }}
// }}
// ```"#,
        name = p.struct_name,
        fields = struct_fields,
        defaults = defaults,
        snake = snake,
    );
    Ok(code_result(&code))
}

/// A single variant in a generated message enum.
#[derive(Debug, Clone, Deserialize, JsonSchema, ToCodeLiteral)]
pub struct MessageVariantDef {
    /// Variant name (PascalCase).
    pub name: String,
    /// Optional payload type (e.g. `String`, `u64`). If `None`, unit variant.
    pub payload_type: Option<String>,
}

/// Parameters for [`egui_fragment_message_enum`].
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct MessageEnumParams {
    /// Enum name (e.g. `AppMessage`).
    pub enum_name: String,
    /// Variants of the message enum.
    pub variants: Vec<MessageVariantDef>,
}

/// Generate a message/action enum for application events.
#[elicit_tool(
    plugin = "egui_fragments",
    name = "egui_fragment_message_enum",
    description = "Generate a Rust message/action enum for egui app event handling.",
    emit = Auto
)]
#[instrument(skip_all)]
async fn egui_fragment_message_enum(p: MessageEnumParams) -> Result<CallToolResult, ErrorData> {
    let mut variants = String::new();
    let mut match_arms = String::new();

    for v in &p.variants {
        match &v.payload_type {
            Some(ty) => {
                variants.push_str(&format!("    {}({}),\n", v.name, ty));
                match_arms.push_str(&format!(
                    "            {enum_name}::{name}(value) => {{\n\
                     \x20               // Handle {name} with value\n\
                     \x20               let _ = value;\n\
                     \x20           }}\n",
                    enum_name = p.enum_name,
                    name = v.name,
                ));
            }
            None => {
                variants.push_str(&format!("    {},\n", v.name));
                match_arms.push_str(&format!(
                    "            {enum_name}::{name} => {{\n\
                     \x20               // Handle {name}\n\
                     \x20           }}\n",
                    enum_name = p.enum_name,
                    name = v.name,
                ));
            }
        }
    }

    let code = format!(
        r#"/// Messages (actions) that drive application state transitions.
#[derive(Debug, Clone)]
pub enum {enum_name} {{
{variants}}}

impl {enum_name} {{
    /// Process the message, updating application state.
    pub fn handle(self) {{
        match self {{
{match_arms}        }}
    }}
}}

// Usage pattern:
// ```
// let mut messages: Vec<{enum_name}> = Vec::new();
//
// // In your UI code, push messages:
// // if ui.button("Save").clicked() {{
// //     messages.push({enum_name}::Save);
// // }}
//
// // After UI rendering, process messages:
// for msg in messages {{
//     msg.handle();
// }}
// ```"#,
        enum_name = p.enum_name,
        variants = variants,
        match_arms = match_arms,
    );
    Ok(code_result(&code))
}
