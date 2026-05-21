//! Dual-mode styling tools.
//!
//! Tools for configuring egui visual appearance — spacing, colours,
//! text styles, widget visuals, etc. Each returns a [`StyleJson`]
//! description that can be applied at runtime or emitted as code.

use elicitation::ToCodeLiteral;
use elicitation::elicit_tool;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::serde_types::{ColorJson, CornerRadiusJson, MarginJson, StrokeJson, Vec2Json};

// ---------------------------------------------------------------------------
// Style JSON types
// ---------------------------------------------------------------------------

/// Serializable style configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
#[serde(tag = "type")]
pub enum StyleJson {
    /// Set global spacing values.
    Spacing {
        /// Space between widgets.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        item_spacing: Option<Vec2Json>,
        /// Space between a widget and its label.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        window_margin: Option<Vec2Json>,
        /// Button padding.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        button_padding: Option<Vec2Json>,
        /// Indentation amount.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        indent: Option<f32>,
    },

    /// Switch to dark mode.
    DarkMode,

    /// Switch to light mode.
    LightMode,

    /// Set a visual property.
    Visual {
        /// Property name.
        property: VisualProperty,
        /// Colour value.
        color: ColorJson,
    },

    /// Set window rounding.
    WindowRounding {
        /// Corner radius.
        corner_radius: CornerRadiusJson,
    },

    /// Set window shadow.
    WindowShadow {
        /// Shadow x-offset.
        offset_x: f32,
        /// Shadow y-offset.
        offset_y: f32,
        /// Blur radius.
        blur: f32,
        /// Shadow colour.
        color: ColorJson,
    },

    /// Override widget visuals (stroke, fill, corner radius).
    WidgetVisuals {
        /// Which widget state to override.
        state: WidgetState,
        /// Background fill.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bg_fill: Option<ColorJson>,
        /// Weak background fill.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        weak_bg_fill: Option<ColorJson>,
        /// Border stroke.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bg_stroke: Option<StrokeJson>,
        /// Corner rounding.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        corner_radius: Option<CornerRadiusJson>,
        /// Foreground stroke.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fg_stroke: Option<StrokeJson>,
    },

    /// Set selection colour.
    SelectionColor {
        /// Background colour for selected items.
        bg_fill: ColorJson,
        /// Stroke for selected items.
        stroke: StrokeJson,
    },

    /// Set text cursor colour and width.
    TextCursor {
        /// Cursor colour.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color: Option<ColorJson>,
        /// Cursor width.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<f32>,
    },

    /// Configure font families with named font data.
    SetFonts {
        /// Font family to configure.
        family: FontFamily,
        /// Ordered list of font data names for this family.
        font_names: Vec<String>,
    },

    /// Override a named text style with font family and size.
    OverrideTextStyle {
        /// Which text style to override.
        style: TextStyleName,
        /// Font family for this style.
        family: FontFamily,
        /// Font size in points.
        size: f32,
    },

    /// Set vertical text alignment.
    SetTextValign {
        /// Vertical alignment.
        valign: TextValign,
    },

    /// Configure interaction timing and thresholds.
    Interaction {
        /// Time window for a click (seconds).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sense_click_time: Option<f32>,
        /// Drag distance threshold (logical pixels).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sense_drag_threshold: Option<f32>,
        /// Delay before tooltips appear (seconds).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        tooltip_delay: Option<f32>,
        /// Only show tooltips when pointer is still.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        show_tooltips_only_when_still: Option<bool>,
    },

    /// Set UI animation duration.
    AnimationTime {
        /// Duration in seconds.
        duration: f32,
    },

    /// Toggle debug rendering options.
    DebugOptions {
        /// Show widget hit-test rectangles.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        show_widget_hits: Option<bool>,
        /// Debug information on hover.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        debug_on_hover: Option<bool>,
        /// Show resize handle indicators.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        show_resize: Option<bool>,
        /// Show interactive widget outlines.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        show_interactive_widgets: Option<bool>,
        /// Show blocking-widget overlay.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        show_blocking_widget: Option<bool>,
    },

    /// Set window border stroke.
    WindowStroke {
        /// Stroke for window borders.
        stroke: StrokeJson,
    },

    /// Set menu margin.
    MenuMargin {
        /// Margin around menus.
        margin: MarginJson,
    },

    /// Configure scroll bar appearance.
    ScrollBar {
        /// Scroll bar track width (logical pixels).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bar_width: Option<f32>,
        /// Minimum scroll handle length.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        handle_min_length: Option<f32>,
        /// Inner margin between bar and content.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bar_inner_margin: Option<f32>,
        /// Outer margin for the scroll bar.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bar_outer_margin: Option<f32>,
        /// Whether the scroll bar floats over content.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        floating: Option<bool>,
    },

    /// Set resize grip/handle size.
    ResizeGripSize {
        /// Side length of the resize corner (logical pixels).
        size: f32,
    },

    /// Configure text cursor blink behaviour.
    TextCursorBlink {
        /// Cursor width (logical pixels).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<f32>,
        /// Blink-on duration (seconds, None = no blink).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blink_on: Option<f32>,
        /// Blink-off duration (seconds).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        blink_off: Option<f32>,
        /// Whether to preview text cursor.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        preview: Option<bool>,
    },
}

/// Visual property names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum VisualProperty {
    /// Hyperlink colour.
    HyperlinkColor,
    /// Faint background colour.
    FaintBgColor,
    /// Extreme background colour (e.g. text input).
    ExtremeBgColor,
    /// Code background colour.
    CodeBgColor,
    /// Warning foreground colour.
    WarnFgColor,
    /// Error foreground colour.
    ErrorFgColor,
    /// Window fill colour.
    WindowFill,
    /// Panel fill colour.
    PanelFill,
}

/// Widget interaction state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum WidgetState {
    /// Normal (no interaction).
    Noninteractive,
    /// Hovered.
    Inactive,
    /// Being hovered.
    Hovered,
    /// Being clicked/dragged.
    Active,
    /// Open (e.g. combo box).
    Open,
}

/// Named font family in egui.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum FontFamily {
    /// Proportional (variable-width) font.
    Proportional,
    /// Monospace (fixed-width) font.
    Monospace,
}

/// Named text style in egui.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum TextStyleName {
    /// Heading text style.
    Heading,
    /// Body text style.
    Body,
    /// Monospace text style.
    Monospace,
    /// Button text style.
    Button,
    /// Small text style.
    Small,
}

/// Vertical text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToCodeLiteral)]
pub enum TextValign {
    /// Align to top.
    Top,
    /// Centre vertically.
    Center,
    /// Align to bottom.
    Bottom,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn style_result(style: &StyleJson) -> CallToolResult {
    match serde_json::to_string(style) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Empty params for no-argument style tools.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct EmptyStyleParams {}

// ---------------------------------------------------------------------------
// Style tools
// ---------------------------------------------------------------------------

/// Parameters for `style_spacing`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpacingParams {
    /// Space between widgets.
    pub item_spacing: Option<Vec2Json>,
    /// Window margin.
    pub window_margin: Option<Vec2Json>,
    /// Button padding.
    pub button_padding: Option<Vec2Json>,
    /// Indentation amount.
    pub indent: Option<f32>,
}

/// Set global spacing values.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_spacing",
    description = "Set global spacing values (item spacing, margins, padding). Returns StyleJson::Spacing."
)]
#[instrument(skip_all)]
async fn style_spacing(p: SpacingParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Spacing {
        item_spacing: p.item_spacing,
        window_margin: p.window_margin,
        button_padding: p.button_padding,
        indent: p.indent,
    };
    Ok(style_result(&s))
}

/// Switch to dark mode.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_dark_mode",
    description = "Switch to dark visual theme. Returns StyleJson::DarkMode."
)]
#[instrument(skip_all)]
async fn style_dark_mode(p: EmptyStyleParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(style_result(&StyleJson::DarkMode))
}

/// Switch to light mode.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_light_mode",
    description = "Switch to light visual theme. Returns StyleJson::LightMode.",
    emit = None
)]
#[instrument(skip_all)]
async fn style_light_mode(p: EmptyStyleParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(style_result(&StyleJson::LightMode))
}

/// Parameters for `style_visual`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct VisualParams {
    /// Which visual property to set.
    pub property: VisualProperty,
    /// Colour value.
    pub color: ColorJson,
}

/// Set a named visual colour property.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_visual",
    description = "Set a visual colour property (hyperlink, background, etc.). Returns StyleJson::Visual."
)]
#[instrument(skip_all)]
async fn style_visual(p: VisualParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: p.property,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Parameters for `style_window_rounding`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowRoundingParams {
    /// Corner radius.
    pub corner_radius: CornerRadiusJson,
}

/// Set window corner rounding.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_window_rounding",
    description = "Set window corner rounding. Returns StyleJson::WindowRounding."
)]
#[instrument(skip_all)]
async fn style_window_rounding(p: WindowRoundingParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::WindowRounding {
        corner_radius: p.corner_radius,
    };
    Ok(style_result(&s))
}

/// Parameters for `style_window_shadow`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WindowShadowParams {
    /// Shadow x-offset.
    pub offset_x: f32,
    /// Shadow y-offset.
    pub offset_y: f32,
    /// Blur radius.
    pub blur: f32,
    /// Shadow colour.
    pub color: ColorJson,
}

/// Set window shadow.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_window_shadow",
    description = "Set window drop shadow. Returns StyleJson::WindowShadow."
)]
#[instrument(skip_all)]
async fn style_window_shadow(p: WindowShadowParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::WindowShadow {
        offset_x: p.offset_x,
        offset_y: p.offset_y,
        blur: p.blur,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Parameters for `style_widget_visuals`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct WidgetVisualsParams {
    /// Which widget state to override.
    pub state: WidgetState,
    /// Background fill.
    pub bg_fill: Option<ColorJson>,
    /// Weak background fill.
    pub weak_bg_fill: Option<ColorJson>,
    /// Border stroke.
    pub bg_stroke: Option<StrokeJson>,
    /// Corner rounding.
    pub corner_radius: Option<CornerRadiusJson>,
    /// Foreground stroke.
    pub fg_stroke: Option<StrokeJson>,
}

/// Override widget visuals for a specific interaction state.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_widget_visuals",
    description = "Override widget visuals (fill, stroke, rounding) for a state. Returns StyleJson::WidgetVisuals."
)]
#[instrument(skip_all)]
async fn style_widget_visuals(p: WidgetVisualsParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::WidgetVisuals {
        state: p.state,
        bg_fill: p.bg_fill,
        weak_bg_fill: p.weak_bg_fill,
        bg_stroke: p.bg_stroke,
        corner_radius: p.corner_radius,
        fg_stroke: p.fg_stroke,
    };
    Ok(style_result(&s))
}

/// Parameters for `style_selection`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SelectionParams {
    /// Background colour for selected items.
    pub bg_fill: ColorJson,
    /// Stroke for selected items.
    pub stroke: StrokeJson,
}

/// Set selection highlight colours.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_selection",
    description = "Set selection highlight colours. Returns StyleJson::SelectionColor."
)]
#[instrument(skip_all)]
async fn style_selection(p: SelectionParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::SelectionColor {
        bg_fill: p.bg_fill,
        stroke: p.stroke,
    };
    Ok(style_result(&s))
}

/// Parameters for `style_text_cursor`.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct TextCursorParams {
    /// Cursor colour.
    pub color: Option<ColorJson>,
    /// Cursor width.
    pub width: Option<f32>,
}

/// Set text cursor appearance.
#[elicit_tool(
    plugin = "egui_style",
    name = "style_text_cursor",
    description = "Set text cursor colour and width. Returns StyleJson::TextCursor."
)]
#[instrument(skip_all)]
async fn style_text_cursor(p: TextCursorParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::TextCursor {
        color: p.color,
        width: p.width,
    };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Font management tools
// ---------------------------------------------------------------------------

/// Parameters for `egui_set_fonts`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SetFontsParams {
    /// Font family to configure.
    pub family: FontFamily,
    /// Ordered list of font data names for this family.
    pub font_names: Vec<String>,
}

/// Configure font families with named font data.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_fonts",
    description = "Configure font families (proportional/monospace) with font data names. Returns StyleJson::SetFonts."
)]
#[instrument(skip_all)]
async fn egui_set_fonts(p: SetFontsParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::SetFonts {
        family: p.family,
        font_names: p.font_names,
    };
    Ok(style_result(&s))
}

/// Parameters for `egui_override_text_style`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct OverrideTextStyleParams {
    /// Which text style to override.
    pub style: TextStyleName,
    /// Font family for this style.
    pub family: FontFamily,
    /// Font size in points.
    pub size: f32,
}

/// Override a named text style with font family and size.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_override_text_style",
    description = "Override a named text style (Heading, Body, Monospace, Button, Small) with font family and size. Returns StyleJson::OverrideTextStyle."
)]
#[instrument(skip_all)]
async fn egui_override_text_style(p: OverrideTextStyleParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::OverrideTextStyle {
        style: p.style,
        family: p.family,
        size: p.size,
    };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_text_valign`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct SetTextValignParams {
    /// Vertical text alignment.
    pub valign: TextValign,
}

/// Set vertical text alignment.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_text_valign",
    description = "Set vertical text alignment (Top, Center, Bottom). Returns StyleJson::SetTextValign."
)]
#[instrument(skip_all)]
async fn egui_set_text_valign(p: SetTextValignParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::SetTextValign { valign: p.valign };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Interaction settings tools
// ---------------------------------------------------------------------------

/// Parameters for `egui_set_interaction`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct InteractionParams {
    /// Time window for a click (seconds).
    pub sense_click_time: Option<f32>,
    /// Drag distance threshold (logical pixels).
    pub sense_drag_threshold: Option<f32>,
    /// Delay before tooltips appear (seconds).
    pub tooltip_delay: Option<f32>,
    /// Only show tooltips when pointer is still.
    pub show_tooltips_only_when_still: Option<bool>,
}

/// Configure interaction settings.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_interaction",
    description = "Configure interaction settings (click time, drag threshold, tooltip delay). Returns StyleJson::Interaction."
)]
#[instrument(skip_all)]
async fn egui_set_interaction(p: InteractionParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Interaction {
        sense_click_time: p.sense_click_time,
        sense_drag_threshold: p.sense_drag_threshold,
        tooltip_delay: p.tooltip_delay,
        show_tooltips_only_when_still: p.show_tooltips_only_when_still,
    };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_animation_time`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct AnimationTimeParams {
    /// Animation duration in seconds.
    pub duration: f32,
}

/// Set animation duration for UI transitions.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_animation_time",
    description = "Set animation duration for UI transitions (seconds). Returns StyleJson::AnimationTime."
)]
#[instrument(skip_all)]
async fn egui_set_animation_time(p: AnimationTimeParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::AnimationTime {
        duration: p.duration,
    };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Debug settings tools
// ---------------------------------------------------------------------------

/// Parameters for `egui_set_debug_options`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct DebugOptionsParams {
    /// Show widget hit-test rectangles.
    pub show_widget_hits: Option<bool>,
    /// Debug information on hover.
    pub debug_on_hover: Option<bool>,
    /// Show resize handle indicators.
    pub show_resize: Option<bool>,
    /// Show interactive widget outlines.
    pub show_interactive_widgets: Option<bool>,
    /// Show blocking-widget overlay.
    pub show_blocking_widget: Option<bool>,
}

/// Toggle debug rendering options.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_debug_options",
    description = "Toggle debug rendering (widget hits, debug on hover, resize indicators). Returns StyleJson::DebugOptions."
)]
#[instrument(skip_all)]
async fn egui_set_debug_options(p: DebugOptionsParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::DebugOptions {
        show_widget_hits: p.show_widget_hits,
        debug_on_hover: p.debug_on_hover,
        show_resize: p.show_resize,
        show_interactive_widgets: p.show_interactive_widgets,
        show_blocking_widget: p.show_blocking_widget,
    };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Individual colour override tools
// ---------------------------------------------------------------------------

/// Parameters for individual colour override tools.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ColorOverrideParams {
    /// Colour value.
    pub color: ColorJson,
}

/// Set hyperlink colour.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_hyperlink_color",
    description = "Set hyperlink colour. Returns StyleJson::Visual with HyperlinkColor."
)]
#[instrument(skip_all)]
async fn egui_set_hyperlink_color(p: ColorOverrideParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: VisualProperty::HyperlinkColor,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Set faint background colour for alternating rows.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_faint_bg_color",
    description = "Set faint background colour for alternating rows/subtle backgrounds. Returns StyleJson::Visual with FaintBgColor.",
    emit = None
)]
#[instrument(skip_all)]
async fn egui_set_faint_bg_color(p: ColorOverrideParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: VisualProperty::FaintBgColor,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Set extreme background colour.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_extreme_bg_color",
    description = "Set extreme background colour (e.g. text input fields). Returns StyleJson::Visual with ExtremeBgColor.",
    emit = None
)]
#[instrument(skip_all)]
async fn egui_set_extreme_bg_color(p: ColorOverrideParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: VisualProperty::ExtremeBgColor,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Set code/monospace background colour.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_code_bg_color",
    description = "Set code/monospace background colour. Returns StyleJson::Visual with CodeBgColor.",
    emit = None
)]
#[instrument(skip_all)]
async fn egui_set_code_bg_color(p: ColorOverrideParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: VisualProperty::CodeBgColor,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Set warning foreground colour.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_warn_fg_color",
    description = "Set warning foreground colour. Returns StyleJson::Visual with WarnFgColor.",
    emit = None
)]
#[instrument(skip_all)]
async fn egui_set_warn_fg_color(p: ColorOverrideParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: VisualProperty::WarnFgColor,
        color: p.color,
    };
    Ok(style_result(&s))
}

/// Set error foreground colour.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_error_fg_color",
    description = "Set error foreground colour. Returns StyleJson::Visual with ErrorFgColor.",
    emit = None
)]
#[instrument(skip_all)]
async fn egui_set_error_fg_color(p: ColorOverrideParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Visual {
        property: VisualProperty::ErrorFgColor,
        color: p.color,
    };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Stroke customisation tools
// ---------------------------------------------------------------------------

/// Parameters for `egui_set_widget_stroke`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WidgetStrokeParams {
    /// Which widget state to set stroke for.
    pub state: WidgetState,
    /// Stroke (width + colour).
    pub stroke: StrokeJson,
}

/// Set stroke for a specific widget state.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_widget_stroke",
    description = "Set border stroke (width + colour) for a widget state (Inactive, Hovered, Active, Open). Returns StyleJson::WidgetVisuals."
)]
#[instrument(skip_all)]
async fn egui_set_widget_stroke(p: WidgetStrokeParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::WidgetVisuals {
        state: p.state,
        bg_fill: None,
        weak_bg_fill: None,
        bg_stroke: Some(p.stroke),
        corner_radius: None,
        fg_stroke: None,
    };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_window_stroke`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WindowStrokeParams {
    /// Window border stroke (width + colour).
    pub stroke: StrokeJson,
}

/// Set window border stroke.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_window_stroke",
    description = "Set window border stroke (width + colour). Returns StyleJson::WindowStroke."
)]
#[instrument(skip_all)]
async fn egui_set_window_stroke(p: WindowStrokeParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::WindowStroke { stroke: p.stroke };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Margin / padding tools
// ---------------------------------------------------------------------------

/// Parameters for `egui_set_menu_margin`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct MenuMarginParams {
    /// Menu margin (left, right, top, bottom).
    pub margin: MarginJson,
}

/// Set margin for menus.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_menu_margin",
    description = "Set margin around menus. Returns StyleJson::MenuMargin."
)]
#[instrument(skip_all)]
async fn egui_set_menu_margin(p: MenuMarginParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::MenuMargin { margin: p.margin };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_button_padding`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ButtonPaddingParams {
    /// Button padding (x = horizontal, y = vertical).
    pub padding: Vec2Json,
}

/// Set padding for buttons.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_button_padding",
    description = "Set button padding (horizontal, vertical). Returns StyleJson::Spacing."
)]
#[instrument(skip_all)]
async fn egui_set_button_padding(p: ButtonPaddingParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Spacing {
        item_spacing: None,
        window_margin: None,
        button_padding: Some(p.padding),
        indent: None,
    };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_indent`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct StyleIndentParams {
    /// Indentation distance in logical pixels.
    pub indent: f32,
}

/// Set indentation distance.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_indent",
    description = "Set indentation distance in logical pixels. Returns StyleJson::Spacing."
)]
#[instrument(skip_all)]
async fn egui_set_indent(p: StyleIndentParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::Spacing {
        item_spacing: None,
        window_margin: None,
        button_padding: None,
        indent: Some(p.indent),
    };
    Ok(style_result(&s))
}

// ---------------------------------------------------------------------------
// Miscellaneous style tools
// ---------------------------------------------------------------------------

/// Parameters for `egui_set_scroll_bar_width`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ScrollBarWidthParams {
    /// Scroll bar track width (logical pixels).
    pub bar_width: Option<f32>,
    /// Minimum scroll handle length.
    pub handle_min_length: Option<f32>,
    /// Inner margin between bar and content.
    pub bar_inner_margin: Option<f32>,
    /// Outer margin for the scroll bar.
    pub bar_outer_margin: Option<f32>,
    /// Whether the scroll bar floats over content.
    pub floating: Option<bool>,
}

/// Set scroll bar width and related settings.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_scroll_bar_width",
    description = "Set scroll bar width, handle length, margins, and floating mode. Returns StyleJson::ScrollBar."
)]
#[instrument(skip_all)]
async fn egui_set_scroll_bar_width(p: ScrollBarWidthParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::ScrollBar {
        bar_width: p.bar_width,
        handle_min_length: p.handle_min_length,
        bar_inner_margin: p.bar_inner_margin,
        bar_outer_margin: p.bar_outer_margin,
        floating: p.floating,
    };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_resize_grip_size`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ResizeGripSizeParams {
    /// Size of the resize grip corner (logical pixels).
    pub size: f32,
}

/// Set resize handle/grip visual size.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_resize_grip_size",
    description = "Set resize grip/handle corner size. Returns StyleJson::ResizeGripSize."
)]
#[instrument(skip_all)]
async fn egui_set_resize_grip_size(p: ResizeGripSizeParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::ResizeGripSize { size: p.size };
    Ok(style_result(&s))
}

/// Parameters for `egui_set_text_cursor_width`.
#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct TextCursorBlinkParams {
    /// Cursor width (logical pixels).
    pub width: Option<f32>,
    /// Blink-on duration (seconds, None = no blink).
    pub blink_on: Option<f32>,
    /// Blink-off duration (seconds).
    pub blink_off: Option<f32>,
    /// Whether to preview text cursor.
    pub preview: Option<bool>,
}

/// Set text cursor blink settings.
#[elicit_tool(
    plugin = "egui_style",
    name = "egui_set_text_cursor_width",
    description = "Set text cursor blink settings (width, blink on/off durations, preview). Returns StyleJson::TextCursorBlink."
)]
#[instrument(skip_all)]
async fn egui_set_text_cursor_width(p: TextCursorBlinkParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson::TextCursorBlink {
        width: p.width,
        blink_on: p.blink_on,
        blink_off: p.blink_off,
        preview: p.preview,
    };
    Ok(style_result(&s))
}
