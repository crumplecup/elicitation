//! Dual-mode styling tools.
//!
//! Tools for configuring egui visual appearance — spacing, colours,
//! text styles, widget visuals, etc. Each returns a [`StyleJson`]
//! description that can be applied at runtime or emitted as code.

use elicitation::elicit_tool;
use rmcp::model::{CallToolResult, Content};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::serde_types::{ColorJson, CornerRadiusJson, StrokeJson, Vec2Json};

// ---------------------------------------------------------------------------
// Style JSON types
// ---------------------------------------------------------------------------

/// Serializable style configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
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
}

/// Visual property names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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

/// Parameters for [`style_spacing`].
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
    description = "Switch to light visual theme. Returns StyleJson::LightMode."
)]
#[instrument(skip_all)]
async fn style_light_mode(p: EmptyStyleParams) -> Result<CallToolResult, ErrorData> {
    let _ = p;
    Ok(style_result(&StyleJson::LightMode))
}

/// Parameters for [`style_visual`].
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

/// Parameters for [`style_window_rounding`].
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

/// Parameters for [`style_window_shadow`].
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

/// Parameters for [`style_widget_visuals`].
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

/// Parameters for [`style_selection`].
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

/// Parameters for [`style_text_cursor`].
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
