//! Dual-mode style and colour tools for ratatui.
//!
//! Each tool returns a [`StyleJson`] or [`ColorJson`] description.

use crate::serde_types::{ColorJson, ModifierJson, StyleJson};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

use elicitation::elicit_tool;

/// Serialise a style to a JSON `CallToolResult`.
fn style_result(style: &StyleJson) -> CallToolResult {
    match serde_json::to_string(style) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

/// Serialise a colour to a JSON `CallToolResult`.
fn color_result(color: &ColorJson) -> CallToolResult {
    match serde_json::to_string(color) {
        Ok(s) => CallToolResult::success(vec![Content::text(s)]),
        Err(e) => CallToolResult::error(vec![Content::text(format!("serialize error: {e}"))]),
    }
}

// ---------------------------------------------------------------------------
// Style tools
// ---------------------------------------------------------------------------

/// Parameters for [`style_fg`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StyleFgParams {
    /// Foreground colour.
    pub color: ColorJson,
}

/// Create a style with a foreground colour.
#[elicit_tool(
    plugin = "ratatui_style",
    name = "style_fg",
    description = "Create a style with a foreground colour. Returns StyleJson."
)]
#[instrument(skip_all)]
async fn style_fg(p: StyleFgParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson {
        fg: Some(p.color),
        ..Default::default()
    };
    Ok(style_result(&s))
}

/// Parameters for [`style_bg`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StyleBgParams {
    /// Background colour.
    pub color: ColorJson,
}

/// Create a style with a background colour.
#[elicit_tool(
    plugin = "ratatui_style",
    name = "style_bg",
    description = "Create a style with a background colour. Returns StyleJson."
)]
#[instrument(skip_all)]
async fn style_bg(p: StyleBgParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson {
        bg: Some(p.color),
        ..Default::default()
    };
    Ok(style_result(&s))
}

/// Parameters for [`style_modifier`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ModifierParams {
    /// Text modifier to apply.
    pub modifier: ModifierJson,
}

/// Create a style with a text modifier (Bold, Italic, etc.).
#[elicit_tool(
    plugin = "ratatui_style",
    name = "style_modifier",
    description = "Create a style with a text modifier (Bold, Italic, Underlined, etc.). Returns StyleJson."
)]
#[instrument(skip_all)]
async fn style_modifier(p: ModifierParams) -> Result<CallToolResult, ErrorData> {
    let s = StyleJson {
        modifiers: vec![p.modifier],
        ..Default::default()
    };
    Ok(style_result(&s))
}

/// Parameters for [`style_reset`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct StyleResetParams;

/// Create a default (reset) style with no colours or modifiers.
#[elicit_tool(
    plugin = "ratatui_style",
    name = "style_reset",
    description = "Create a default (reset) style with no colours or modifiers. Returns StyleJson."
)]
#[instrument(skip_all)]
async fn style_reset(_p: StyleResetParams) -> Result<CallToolResult, ErrorData> {
    Ok(style_result(&StyleJson::default()))
}

// ---------------------------------------------------------------------------
// Colour tools
// ---------------------------------------------------------------------------

/// Parameters for [`color_rgb`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ColorRgbParams {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
}

/// Create an RGB colour.
#[elicit_tool(
    plugin = "ratatui_style",
    name = "color_rgb",
    description = "Create a 24-bit RGB colour. Returns ColorJson::Rgb."
)]
#[instrument(skip_all)]
async fn color_rgb(p: ColorRgbParams) -> Result<CallToolResult, ErrorData> {
    let c = ColorJson::Rgb {
        r: p.r,
        g: p.g,
        b: p.b,
    };
    Ok(color_result(&c))
}

/// Parameters for [`color_indexed`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ColorIndexedParams {
    /// 256-colour palette index (0–255).
    pub index: u8,
}

/// Create a 256-colour palette colour.
#[elicit_tool(
    plugin = "ratatui_style",
    name = "color_indexed",
    description = "Create a 256-colour palette colour by index (0–255). Returns ColorJson::Indexed."
)]
#[instrument(skip_all)]
async fn color_indexed(p: ColorIndexedParams) -> Result<CallToolResult, ErrorData> {
    let c = ColorJson::Indexed { index: p.index };
    Ok(color_result(&c))
}

/// Parameters for [`color_named`].
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ColorNamedParams {
    /// Named colour: "Red", "Green", "Blue", "Yellow", "Magenta", "Cyan",
    /// "White", "Black", "Gray", "DarkGray", "LightRed", "LightGreen",
    /// "LightYellow", "LightBlue", "LightMagenta", "LightCyan".
    pub name: String,
}

/// Create a named ANSI colour.
#[elicit_tool(
    plugin = "ratatui_style",
    name = "color_named",
    description = "Create a named ANSI colour (Red, Green, Blue, etc.). Returns ColorJson variant."
)]
#[instrument(skip_all)]
async fn color_named(p: ColorNamedParams) -> Result<CallToolResult, ErrorData> {
    let c = match p.name.to_lowercase().as_str() {
        "black" => ColorJson::Black,
        "red" => ColorJson::Red,
        "green" => ColorJson::Green,
        "yellow" => ColorJson::Yellow,
        "blue" => ColorJson::Blue,
        "magenta" => ColorJson::Magenta,
        "cyan" => ColorJson::Cyan,
        "white" => ColorJson::White,
        "gray" | "grey" => ColorJson::Gray,
        "darkgray" | "darkgrey" | "dark_gray" | "dark_grey" => ColorJson::DarkGray,
        "lightred" | "light_red" => ColorJson::LightRed,
        "lightgreen" | "light_green" => ColorJson::LightGreen,
        "lightyellow" | "light_yellow" => ColorJson::LightYellow,
        "lightblue" | "light_blue" => ColorJson::LightBlue,
        "lightmagenta" | "light_magenta" => ColorJson::LightMagenta,
        "lightcyan" | "light_cyan" => ColorJson::LightCyan,
        "reset" => ColorJson::Reset,
        _ => {
            return Err(ErrorData::internal_error(
                format!("Unknown colour name: {}", p.name),
                None,
            ));
        }
    };
    Ok(color_result(&c))
}
