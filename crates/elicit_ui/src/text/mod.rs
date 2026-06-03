//! Cross-backend text IR — the linebender half of the `elicit_ui` IR.
//!
//! These types form the portable text intermediate representation used
//! throughout the `elicit_*` pipeline.  They are intentionally independent
//! of any concrete frontend (ratatui, egui, leptos) and convert *to* the
//! concrete backend types inside the corresponding bridge crates.
//!
//! # Linebender integration
//!
//! - [`UiColor`] converts to [`peniko::Color`] via `From<UiColor>`.
//! - [`FontWeight`] converts to [`parley::style::FontWeight`].
//! - [`FontStyle`] converts to [`parley::style::FontStyle`].
//!
//! # Usage
//!
//! ```rust
//! use elicit_ui::text::{ParagraphText, RichText, TextLine, TextSpan, TextStyle, UiColor, TextModifier};
//!
//! let cursor_style = TextStyle {
//!     fg: None,
//!     bg: None,
//!     modifiers: vec![TextModifier::Reversed, TextModifier::Bold],
//!     ..Default::default()
//! };
//! let span = TextSpan { content: " X ".to_string(), style: Some(cursor_style) };
//! let line = TextLine { spans: vec![span], style: None, alignment: None };
//! let rich = RichText { lines: vec![line], style: None, alignment: None };
//! let para = ParagraphText::Rich(rich);
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Colour
// ---------------------------------------------------------------------------

/// Cross-backend colour.
///
/// Covers the full ANSI palette plus 24-bit RGB and 256-colour indexed modes.
/// Converts to [`peniko::Color`] via `From<UiColor>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum UiColor {
    /// Reset to terminal / theme default.
    Reset,
    /// Standard ANSI black.
    Black,
    /// Standard ANSI red.
    Red,
    /// Standard ANSI green.
    Green,
    /// Standard ANSI yellow.
    Yellow,
    /// Standard ANSI blue.
    Blue,
    /// Standard ANSI magenta.
    Magenta,
    /// Standard ANSI cyan.
    Cyan,
    /// Standard ANSI white.
    White,
    /// Dark gray (bright black).
    DarkGray,
    /// Light (bright) red.
    LightRed,
    /// Light (bright) green.
    LightGreen,
    /// Light (bright) yellow.
    LightYellow,
    /// Light (bright) blue.
    LightBlue,
    /// Light (bright) magenta.
    LightMagenta,
    /// Light (bright) cyan.
    LightCyan,
    /// Bright white / gray.
    Gray,
    /// 24-bit RGB colour.
    Rgb {
        /// Red channel (0–255).
        r: u8,
        /// Green channel (0–255).
        g: u8,
        /// Blue channel (0–255).
        b: u8,
    },
    /// 24-bit RGB with alpha (transparency).
    Rgba {
        /// Red channel (0–255).
        r: u8,
        /// Green channel (0–255).
        g: u8,
        /// Blue channel (0–255).
        b: u8,
        /// Alpha channel (0 = transparent, 255 = opaque).
        a: u8,
    },
    /// 256-colour palette index.
    Indexed {
        /// Palette index (0–255).
        index: u8,
    },
}

impl From<UiColor> for peniko::Color {
    fn from(c: UiColor) -> Self {
        // VS Code terminal palette (reference sRGB values).
        match c {
            UiColor::Reset => peniko::Color::from_rgba8(0, 0, 0, 0),
            UiColor::Black => peniko::Color::from_rgb8(12, 12, 12),
            UiColor::Red => peniko::Color::from_rgb8(197, 15, 31),
            UiColor::Green => peniko::Color::from_rgb8(19, 161, 14),
            UiColor::Yellow => peniko::Color::from_rgb8(193, 156, 0),
            UiColor::Blue => peniko::Color::from_rgb8(0, 55, 218),
            UiColor::Magenta => peniko::Color::from_rgb8(136, 23, 152),
            UiColor::Cyan => peniko::Color::from_rgb8(58, 150, 221),
            UiColor::White => peniko::Color::from_rgb8(204, 204, 204),
            UiColor::DarkGray => peniko::Color::from_rgb8(118, 118, 118),
            UiColor::LightRed => peniko::Color::from_rgb8(231, 72, 86),
            UiColor::LightGreen => peniko::Color::from_rgb8(22, 198, 12),
            UiColor::LightYellow => peniko::Color::from_rgb8(249, 241, 165),
            UiColor::LightBlue => peniko::Color::from_rgb8(59, 120, 255),
            UiColor::LightMagenta => peniko::Color::from_rgb8(180, 0, 158),
            UiColor::LightCyan => peniko::Color::from_rgb8(97, 214, 214),
            UiColor::Gray => peniko::Color::from_rgb8(242, 242, 242),
            UiColor::Rgb { r, g, b } => peniko::Color::from_rgb8(r, g, b),
            UiColor::Rgba { r, g, b, a } => peniko::Color::from_rgba8(r, g, b, a),
            UiColor::Indexed { index } => ansi256_to_peniko(index),
        }
    }
}

/// Approximate ANSI 256-colour index to sRGB.
fn ansi256_to_peniko(idx: u8) -> peniko::Color {
    match idx {
        // 0-15: standard ANSI colours (use the named variants above)
        0 => peniko::Color::from_rgb8(12, 12, 12),
        1 => peniko::Color::from_rgb8(197, 15, 31),
        2 => peniko::Color::from_rgb8(19, 161, 14),
        3 => peniko::Color::from_rgb8(193, 156, 0),
        4 => peniko::Color::from_rgb8(0, 55, 218),
        5 => peniko::Color::from_rgb8(136, 23, 152),
        6 => peniko::Color::from_rgb8(58, 150, 221),
        7 => peniko::Color::from_rgb8(204, 204, 204),
        8 => peniko::Color::from_rgb8(118, 118, 118),
        9 => peniko::Color::from_rgb8(231, 72, 86),
        10 => peniko::Color::from_rgb8(22, 198, 12),
        11 => peniko::Color::from_rgb8(249, 241, 165),
        12 => peniko::Color::from_rgb8(59, 120, 255),
        13 => peniko::Color::from_rgb8(180, 0, 158),
        14 => peniko::Color::from_rgb8(97, 214, 214),
        15 => peniko::Color::from_rgb8(242, 242, 242),
        // 16-231: 6×6×6 colour cube
        16..=231 => {
            let n = idx - 16;
            let b = n % 6;
            let g = (n / 6) % 6;
            let r = n / 36;
            let channel = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            peniko::Color::from_rgb8(channel(r), channel(g), channel(b))
        }
        // 232-255: greyscale ramp
        232..=255 => {
            let v = 8 + (idx - 232) * 10;
            peniko::Color::from_rgb8(v, v, v)
        }
    }
}

// ---------------------------------------------------------------------------
// Text modifiers
// ---------------------------------------------------------------------------

/// Text rendering attribute.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TextModifier {
    /// Bold text.
    Bold,
    /// Dim / faint text.
    Dim,
    /// Italic text.
    Italic,
    /// Underlined text.
    Underlined,
    /// Slow blink.
    SlowBlink,
    /// Rapid blink.
    RapidBlink,
    /// Reversed foreground / background.
    Reversed,
    /// Hidden text.
    Hidden,
    /// Crossed-out (strikethrough) text.
    CrossedOut,
}

// ---------------------------------------------------------------------------
// Font weight
// ---------------------------------------------------------------------------

/// Portable font weight (matches `parley::style::FontWeight`).
///
/// Converts to `parley::style::FontWeight` via `From<FontWeight>`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FontWeight(pub f32);

impl FontWeight {
    /// 100 — Thin.
    pub const THIN: Self = Self(100.0);
    /// 200 — Extra-light.
    pub const EXTRA_LIGHT: Self = Self(200.0);
    /// 300 — Light.
    pub const LIGHT: Self = Self(300.0);
    /// 400 — Normal / Regular.
    pub const NORMAL: Self = Self(400.0);
    /// 500 — Medium.
    pub const MEDIUM: Self = Self(500.0);
    /// 600 — Semi-bold.
    pub const SEMI_BOLD: Self = Self(600.0);
    /// 700 — Bold.
    pub const BOLD: Self = Self(700.0);
    /// 800 — Extra-bold.
    pub const EXTRA_BOLD: Self = Self(800.0);
    /// 900 — Black.
    pub const BLACK: Self = Self(900.0);
}

impl From<FontWeight> for parley::style::FontWeight {
    fn from(w: FontWeight) -> Self {
        Self::new(w.0)
    }
}

// ---------------------------------------------------------------------------
// Font style
// ---------------------------------------------------------------------------

/// Portable font style (matches `parley::style::FontStyle`).
///
/// Converts to `parley::style::FontStyle` via `From<FontStyle>`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum FontStyle {
    /// Upright text.
    Normal,
    /// Italic text.
    Italic,
    /// Oblique text with optional angle in degrees.
    Oblique(Option<f32>),
}

impl From<FontStyle> for parley::style::FontStyle {
    fn from(s: FontStyle) -> Self {
        match s {
            FontStyle::Normal => Self::Normal,
            FontStyle::Italic => Self::Italic,
            FontStyle::Oblique(angle) => Self::Oblique(angle),
        }
    }
}

// ---------------------------------------------------------------------------
// Text decoration
// ---------------------------------------------------------------------------

/// Text decoration attribute (for GUI frontends; complements [`TextModifier`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TextDecoration {
    /// Underline.
    Underline,
    /// Strikethrough.
    Strikethrough,
    /// Overline.
    Overline,
}

// ---------------------------------------------------------------------------
// Font family
// ---------------------------------------------------------------------------

/// Portable font family.
///
/// Bridges map this to the closest native type:
/// - egui: [`egui::FontFamily`]
/// - CSS/leptos: `font-family` property (`sans-serif`, `monospace`, or quoted name)
/// - parley: [`parley::style::GenericFamily`] / [`parley::style::FontFamilyName`]
/// - ratatui: no-op (terminal uses its own font)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum FontFamily {
    /// Default proportional (sans-serif) font.
    Proportional,
    /// Monospace font.
    Monospace,
    /// Named font family (e.g. `"Arial"`, `"JetBrains Mono"`).
    Named {
        /// Font family name.
        name: String,
    },
}

// ---------------------------------------------------------------------------
// Line height
// ---------------------------------------------------------------------------

/// Portable line height.
///
/// Mirrors [`parley::style::LineHeight`] exactly.  Bridges map this to:
/// - parley: direct 1-to-1 via [`From<LineHeight>`]
/// - egui: `TextFormat::line_height` — `MetricsRelative` and `FontSizeRelative`
///   are approximated as `font_size × factor`; `Absolute` is used directly
/// - CSS/leptos: `line-height` property — relative variants emit a unitless number,
///   `Absolute` emits a `px` value
/// - ratatui: no-op (terminal line spacing is fixed)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum LineHeight {
    /// Multiple of the font's metrics-defined line height (ascender + descender + leading).
    ///
    /// Default CSS equivalent: `line-height: normal` ≈ `MetricsRelative(1.0)`.
    MetricsRelative {
        /// Multiplier, e.g. `1.2` for 120% of metrics height.
        factor: f32,
    },
    /// Multiple of the font size (unitless CSS `line-height` behaviour).
    FontSizeRelative {
        /// Multiplier, e.g. `1.5` for 150% of font size.
        factor: f32,
    },
    /// Absolute line height in points / pixels.
    Absolute {
        /// Height in points or CSS pixels.
        value: f32,
    },
}

impl From<LineHeight> for parley::style::LineHeight {
    fn from(lh: LineHeight) -> Self {
        match lh {
            LineHeight::MetricsRelative { factor } => Self::MetricsRelative(factor),
            LineHeight::FontSizeRelative { factor } => Self::FontSizeRelative(factor),
            LineHeight::Absolute { value } => Self::Absolute(value),
        }
    }
}

// ---------------------------------------------------------------------------
// Vertical alignment
// ---------------------------------------------------------------------------

/// Vertical alignment of a text span within its line.
///
/// Used for superscript (`Top`) and subscript (`Bottom`) effects.
/// Bridges map this to:
/// - egui: `TextFormat::valign`
/// - CSS/leptos: `vertical-align` (`super`, `middle`, `sub`)
/// - parley / ratatui: best-effort or no-op
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum VerticalAlign {
    /// Align to top of the line — superscript effect when combined with a small font.
    Top,
    /// Align to the centre of the line (default for mixed-size runs).
    Center,
    /// Align to the bottom of the line — subscript effect when combined with a small font.
    Bottom,
}

// ---------------------------------------------------------------------------
// Style
// ---------------------------------------------------------------------------

/// Cross-backend text style.
///
/// Core fields (`fg`, `bg`, `modifiers`) are universally supported.
/// Extended fields (`font_weight`, `font_style`, `decorations`) are used by
/// GUI frontends (egui, wgpu) via the linebender bridge.
///
/// Rich typography fields (`font_size`, `font_family`, `letter_spacing`,
/// `word_spacing`, `line_height`, `vertical_align`) target GUI frontends
/// that support the full linebender/CSS rendering model.  Terminal frontends
/// (ratatui) treat these as no-ops.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TextStyle {
    /// Foreground colour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fg: Option<UiColor>,
    /// Background colour.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<UiColor>,
    /// Active text modifiers (bold, italic, reversed, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modifiers: Vec<TextModifier>,
    /// Font weight override (GUI frontends only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
    /// Font style override (GUI frontends only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_style: Option<FontStyle>,
    /// Text decorations (GUI frontends only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub decorations: Vec<TextDecoration>,
    /// Font size in points (GUI frontends only; ratatui: no-op).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f32>,
    /// Font family override (GUI frontends only; ratatui: no-op).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<FontFamily>,
    /// Extra letter spacing in points (GUI frontends only; ratatui: no-op).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<f32>,
    /// Extra word spacing in points (GUI frontends only; ratatui: no-op).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_spacing: Option<f32>,
    /// Line height override (GUI frontends only; ratatui: no-op).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_height: Option<LineHeight>,
    /// Vertical alignment within the line (GUI frontends only; ratatui: no-op).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertical_align: Option<VerticalAlign>,
}

// ---------------------------------------------------------------------------
// Alignment
// ---------------------------------------------------------------------------

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum TextAlign {
    /// Left-aligned.
    Left,
    /// Centred.
    Center,
    /// Right-aligned.
    Right,
    /// Justified (expand inter-word spacing to fill the line width).
    ///
    /// GUI frontends that support justify (egui, CSS/leptos) honour this directly.
    /// Terminal frontends (ratatui) fall back to `Left`.
    Justify,
}

// ---------------------------------------------------------------------------
// Span / Line / RichText
// ---------------------------------------------------------------------------

/// A styled run of text within a line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TextSpan {
    /// Span text content.
    pub content: String,
    /// Per-span style override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,
}

/// A line of styled spans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TextLine {
    /// Spans composing this line.
    pub spans: Vec<TextSpan>,
    /// Style applied to the whole line (merged with span styles).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,
    /// Line alignment override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<TextAlign>,
}

/// Multi-line rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RichText {
    /// Lines of text.
    pub lines: Vec<TextLine>,
    /// Style applied to the entire block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,
    /// Alignment for the entire block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alignment: Option<TextAlign>,
}

// ---------------------------------------------------------------------------
// ParagraphText — the portable paragraph IR
// ---------------------------------------------------------------------------

/// Paragraph content: a plain string or richly-styled [`RichText`].
///
/// Serialises as a JSON string for [`ParagraphText::Plain`] or a JSON object
/// for [`ParagraphText::Rich`].  This asymmetry is intentional: existing
/// consumers that pass plain strings continue to work, while new consumers
/// can pass a [`RichText`] object for per-span styling.
///
/// # Examples
///
/// Plain text:
/// ```json
/// "Hello world"
/// ```
///
/// Rich text:
/// ```json
/// {"lines":[{"spans":[{"content":" X ","style":{"modifiers":["Reversed","Bold"]}}]}]}
/// ```
#[derive(Debug, Clone, PartialEq, JsonSchema)]
pub enum ParagraphText {
    /// Plain unstyled text.
    Plain(String),
    /// Multi-line text with per-span styling.
    Rich(RichText),
}

impl serde::Serialize for ParagraphText {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Plain(s) => s.serialize(serializer),
            Self::Rich(t) => t.serialize(serializer),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ParagraphText {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::String(s) => Ok(Self::Plain(s)),
            serde_json::Value::Object(_) => serde_json::from_value::<RichText>(value)
                .map(Self::Rich)
                .map_err(serde::de::Error::custom),
            other => Err(serde::de::Error::custom(format!(
                "expected string or object for ParagraphText, got {other}"
            ))),
        }
    }
}

impl ParagraphText {
    /// Returns all text content joined with newlines between lines.
    pub fn to_plain_string(&self) -> String {
        match self {
            Self::Plain(s) => s.clone(),
            Self::Rich(t) => t
                .lines
                .iter()
                .map(|l| {
                    l.spans
                        .iter()
                        .map(|s| s.content.as_str())
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n"),
        }
    }
}

impl From<String> for ParagraphText {
    fn from(s: String) -> Self {
        Self::Plain(s)
    }
}

impl From<&str> for ParagraphText {
    fn from(s: &str) -> Self {
        Self::Plain(s.to_string())
    }
}

impl From<RichText> for ParagraphText {
    fn from(t: RichText) -> Self {
        Self::Rich(t)
    }
}
