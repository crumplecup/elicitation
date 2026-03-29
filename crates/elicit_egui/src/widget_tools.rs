//! Dual-mode widget creation tools.
//!
//! Each tool returns a [`WidgetJson`] variant describing the widget.
//! In emit mode, the JSON can be converted to idiomatic egui code.

use elicitation::elicit_tool;
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use crate::serde_types::{ColorJson, RangeJson, StrokeJson, WidgetJson};

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

/// Empty params for tools that take no arguments.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyParams {}

fn widget_result(widget: &WidgetJson) -> CallToolResult {
    match serde_json::to_string(widget) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ---------------------------------------------------------------------------
// Basic display widgets
// ---------------------------------------------------------------------------

/// Parameters for [`widget_label`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct LabelParams {
    /// Display text.
    pub text: String,
    /// Whether to wrap long text.
    #[serde(default)]
    pub wrap: bool,
    /// Optional text colour (RGBA).
    pub color: Option<ColorJson>,
}

/// Create a plain text label.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_label",
    description = "Create a plain text label. Returns WidgetJson::Label."
)]
#[instrument(skip_all)]
async fn widget_label(p: LabelParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Label {
        text: p.text,
        wrap: p.wrap,
        color: p.color,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_heading`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HeadingParams {
    /// Heading text.
    pub text: String,
}

/// Create a heading (large, bold text).
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_heading",
    description = "Create a heading (large bold text). Returns WidgetJson::Heading."
)]
#[instrument(skip_all)]
async fn widget_heading(p: HeadingParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Heading { text: p.text };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_monospace`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct MonospaceParams {
    /// Text content.
    pub text: String,
}

/// Create monospace text.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_monospace",
    description = "Create monospace (fixed-width) text. Returns WidgetJson::Monospace."
)]
#[instrument(skip_all)]
async fn widget_monospace(p: MonospaceParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Monospace { text: p.text };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_code`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CodeParams {
    /// Code text content.
    pub text: String,
}

/// Create code text (monospace with background).
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_code",
    description = "Create code text (monospace with background). Returns WidgetJson::Code."
)]
#[instrument(skip_all)]
async fn widget_code(p: CodeParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Code { text: p.text };
    Ok(widget_result(&w))
}

/// Parameters for simple text widgets (small, strong, weak).
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SimpleTextParams {
    /// Text content.
    pub text: String,
}

/// Create small text.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_small",
    description = "Create small text. Returns WidgetJson::Small."
)]
#[instrument(skip_all)]
async fn widget_small(p: SimpleTextParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Small { text: p.text };
    Ok(widget_result(&w))
}

/// Create strong (bold) text.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_strong",
    description = "Create strong (bold) text. Returns WidgetJson::Strong."
)]
#[instrument(skip_all)]
async fn widget_strong(p: SimpleTextParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Strong { text: p.text };
    Ok(widget_result(&w))
}

/// Create weak (faint) text.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_weak",
    description = "Create weak (faint) text. Returns WidgetJson::Weak."
)]
#[instrument(skip_all)]
async fn widget_weak(p: SimpleTextParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Weak { text: p.text };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_colored_label`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ColoredLabelParams {
    /// Text content.
    pub text: String,
    /// Text colour.
    pub color: ColorJson,
}

/// Create a coloured text label.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_colored_label",
    description = "Create a coloured text label. Returns WidgetJson::ColoredLabel."
)]
#[instrument(skip_all)]
async fn widget_colored_label(p: ColoredLabelParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::ColoredLabel {
        text: p.text,
        color: p.color,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Interactive widgets
// ---------------------------------------------------------------------------

/// Parameters for [`widget_button`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ButtonParams {
    /// Button label text.
    pub text: String,
    /// Whether to wrap long text.
    #[serde(default)]
    pub wrap: bool,
    /// Optional fill colour.
    pub fill: Option<ColorJson>,
    /// Optional border stroke.
    pub stroke: Option<StrokeJson>,
}

/// Create a clickable button.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_button",
    description = "Create a clickable button. Returns WidgetJson::Button."
)]
#[instrument(skip_all)]
async fn widget_button(p: ButtonParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Button {
        text: p.text,
        wrap: p.wrap,
        fill: p.fill,
        stroke: p.stroke,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_small_button`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SmallButtonParams {
    /// Button label text.
    pub text: String,
}

/// Create a small button (less padding).
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_small_button",
    description = "Create a small button with less padding. Returns WidgetJson::SmallButton."
)]
#[instrument(skip_all)]
async fn widget_small_button(p: SmallButtonParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::SmallButton { text: p.text };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_checkbox`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckboxParams {
    /// Label text beside the checkbox.
    pub text: String,
    /// Current checked state.
    #[serde(default)]
    pub checked: bool,
}

/// Create a boolean checkbox.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_checkbox",
    description = "Create a boolean checkbox. Returns WidgetJson::Checkbox."
)]
#[instrument(skip_all)]
async fn widget_checkbox(p: CheckboxParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Checkbox {
        text: p.text,
        checked: p.checked,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_radio_value`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct RadioValueParams {
    /// Label text.
    pub text: String,
    /// Whether this radio is currently selected.
    #[serde(default)]
    pub selected: bool,
}

/// Create a radio button.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_radio_value",
    description = "Create a radio button (one of a group). Returns WidgetJson::RadioValue."
)]
#[instrument(skip_all)]
async fn widget_radio_value(p: RadioValueParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::RadioValue {
        text: p.text,
        selected: p.selected,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_selectable_label`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectableLabelParams {
    /// Label text.
    pub text: String,
    /// Whether currently selected.
    #[serde(default)]
    pub selected: bool,
}

/// Create a selectable label (click to toggle selection).
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_selectable_label",
    description = "Create a selectable label (click to toggle). Returns WidgetJson::SelectableLabel."
)]
#[instrument(skip_all)]
async fn widget_selectable_label(p: SelectableLabelParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::SelectableLabel {
        text: p.text,
        selected: p.selected,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_hyperlink`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct HyperlinkParams {
    /// Display text.
    pub text: String,
    /// Target URL.
    pub url: String,
}

/// Create a hyperlink.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_hyperlink",
    description = "Create a hyperlink that opens a URL. Returns WidgetJson::Hyperlink."
)]
#[instrument(skip_all)]
async fn widget_hyperlink(p: HyperlinkParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Hyperlink {
        text: p.text,
        url: p.url,
    };
    Ok(widget_result(&w))
}

/// Create a horizontal/vertical separator.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_separator",
    description = "Create a separator line. Returns WidgetJson::Separator."
)]
#[instrument(skip_all)]
async fn widget_separator(p: EmptyParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(widget_result(&WidgetJson::Separator))
}

/// Create a loading spinner.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_spinner",
    description = "Create a loading spinner animation. Returns WidgetJson::Spinner."
)]
#[instrument(skip_all)]
async fn widget_spinner(p: EmptyParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(widget_result(&WidgetJson::Spinner))
}

// ---------------------------------------------------------------------------
// Text input widgets
// ---------------------------------------------------------------------------

/// Parameters for [`widget_text_edit_singleline`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextEditSinglelineParams {
    /// Current text value.
    pub text: String,
    /// Placeholder hint text.
    pub hint: Option<String>,
    /// Whether the input is interactive.
    #[serde(default = "default_true")]
    pub interactive: bool,
}

fn default_true() -> bool {
    true
}

/// Create a single-line text input.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_text_edit_singleline",
    description = "Create a single-line text input. Returns WidgetJson::TextEditSingleline."
)]
#[instrument(skip_all)]
async fn widget_text_edit_singleline(
    p: TextEditSinglelineParams,
) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::TextEditSingleline {
        text: p.text,
        hint: p.hint,
        interactive: p.interactive,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_text_edit_multiline`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextEditMultilineParams {
    /// Current text value.
    pub text: String,
    /// Placeholder hint text.
    pub hint: Option<String>,
    /// Whether the input is interactive.
    #[serde(default = "default_true")]
    pub interactive: bool,
}

/// Create a multi-line text input.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_text_edit_multiline",
    description = "Create a multi-line text input. Returns WidgetJson::TextEditMultiline."
)]
#[instrument(skip_all)]
async fn widget_text_edit_multiline(
    p: TextEditMultilineParams,
) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::TextEditMultiline {
        text: p.text,
        hint: p.hint,
        interactive: p.interactive,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_code_editor`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct CodeEditorParams {
    /// Current code text.
    pub text: String,
    /// Programming language hint.
    pub language: Option<String>,
    /// Whether the editor is interactive.
    #[serde(default = "default_true")]
    pub interactive: bool,
}

/// Create a code editor (monospace with tab support).
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_code_editor",
    description = "Create a code editor (monospace, tab support). Returns WidgetJson::CodeEditor."
)]
#[instrument(skip_all)]
async fn widget_code_editor(p: CodeEditorParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::CodeEditor {
        text: p.text,
        language: p.language,
        interactive: p.interactive,
    };
    Ok(widget_result(&w))
}

// ---------------------------------------------------------------------------
// Numeric widgets
// ---------------------------------------------------------------------------

/// Parameters for [`widget_slider`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SliderParams {
    /// Current value.
    pub value: f64,
    /// Minimum value (inclusive).
    pub min: f64,
    /// Maximum value (inclusive).
    pub max: f64,
    /// Step size between values.
    pub step: Option<f64>,
    /// Label text beside the slider.
    pub text: Option<String>,
    /// Suffix appended to the value display.
    pub suffix: Option<String>,
    /// Whether to use logarithmic scale.
    #[serde(default)]
    pub logarithmic: bool,
    /// Whether to clamp value to range.
    #[serde(default = "default_true")]
    pub clamping: bool,
}

/// Create a numeric slider.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_slider",
    description = "Create a numeric slider. Returns WidgetJson::Slider."
)]
#[instrument(skip_all)]
async fn widget_slider(p: SliderParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Slider {
        value: p.value,
        range: RangeJson {
            min: p.min,
            max: p.max,
        },
        step: p.step,
        text: p.text,
        suffix: p.suffix,
        logarithmic: p.logarithmic,
        clamping: p.clamping,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_drag_value`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct DragValueParams {
    /// Current value.
    pub value: f64,
    /// Optional value range.
    pub range: Option<RangeJson>,
    /// Drag speed multiplier.
    pub speed: Option<f64>,
    /// Label prefix.
    pub prefix: Option<String>,
    /// Label suffix.
    pub suffix: Option<String>,
}

/// Create a drag-to-edit numeric value.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_drag_value",
    description = "Create a drag-to-edit numeric value. Returns WidgetJson::DragValue."
)]
#[instrument(skip_all)]
async fn widget_drag_value(p: DragValueParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::DragValue {
        value: p.value,
        range: p.range,
        speed: p.speed,
        prefix: p.prefix,
        suffix: p.suffix,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_progress_bar`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProgressBarParams {
    /// Progress fraction (0.0 = empty, 1.0 = full).
    pub progress: f32,
    /// Optional overlay text.
    pub text: Option<String>,
}

/// Create a progress bar.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_progress_bar",
    description = "Create a progress bar (0.0–1.0). Returns WidgetJson::ProgressBar."
)]
#[instrument(skip_all)]
async fn widget_progress_bar(p: ProgressBarParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::ProgressBar {
        progress: p.progress,
        text: p.text,
    };
    Ok(widget_result(&w))
}

/// Parameters for [`widget_image`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ImageParams {
    /// Image URI.
    pub uri: String,
    /// Optional size constraint.
    pub size: Option<crate::serde_types::Vec2Json>,
}

/// Create an image display.
#[elicit_tool(
    plugin = "egui_widgets",
    name = "widget_image",
    description = "Display an image by URI. Returns WidgetJson::Image."
)]
#[instrument(skip_all)]
async fn widget_image(p: ImageParams) -> Result<CallToolResult, ErrorData> {
    let w = WidgetJson::Image {
        uri: p.uri,
        size: p.size,
    };
    Ok(widget_result(&w))
}
