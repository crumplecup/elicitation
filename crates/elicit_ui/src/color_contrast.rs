//! WCAG color contrast verification using the `palette` crate.
//!
//! Provides constraints for WCAG 2.1 contrast requirements:
//! - SC 1.4.3 Contrast (Minimum) — Level AA: 4.5:1 normal, 3:1 large
//! - SC 1.4.6 Contrast (Enhanced) — Level AAA: 7:1 normal, 4.5:1 large
//! - SC 1.4.11 Non-text Contrast — Level AA: 3:1 for UI components

use crate::constraints::{Constraint, ConstraintContext, SpecReference, Violation, WcagLevel};
use accesskit::NodeId;

/// sRGB color for contrast checking.
///
/// This is our domain type that can be converted to palette's Srgb
/// when the `color` feature is enabled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SrgbColor {
    /// Red channel (0.0–1.0).
    pub r: f32,
    /// Green channel (0.0–1.0).
    pub g: f32,
    /// Blue channel (0.0–1.0).
    pub b: f32,
}

impl SrgbColor {
    /// Create from floating-point channels (0.0–1.0).
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Create from 8-bit channels (0–255).
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
        }
    }

    /// Convert to a hex string like `#rrggbb`.
    pub fn to_hex(&self) -> String {
        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}

/// Convert a [`crate::UiColor`] to an sRGB triple for contrast checking.
///
/// Returns `None` for `UiColor::Reset` because it inherits its colour from
/// the host theme at render time; no single sRGB value can represent it.
/// For `Rgba` the alpha channel is discarded — WCAG contrast is defined for
/// fully opaque colours; callers should verify opacity independently.
impl crate::UiColor {
    /// Convert to [`SrgbColor`], or `None` for `UiColor::Reset`.
    pub fn to_srgb(self) -> Option<SrgbColor> {
        // VS Code terminal palette — same values as `From<UiColor> for peniko::Color`.
        let (r, g, b): (u8, u8, u8) = match self {
            Self::Reset => return None,
            Self::Black => (12, 12, 12),
            Self::Red => (197, 15, 31),
            Self::Green => (19, 161, 14),
            Self::Yellow => (193, 156, 0),
            Self::Blue => (0, 55, 218),
            Self::Magenta => (136, 23, 152),
            Self::Cyan => (58, 150, 221),
            Self::White => (204, 204, 204),
            Self::DarkGray => (118, 118, 118),
            Self::LightRed => (231, 72, 86),
            Self::LightGreen => (22, 198, 12),
            Self::LightYellow => (249, 241, 165),
            Self::LightBlue => (59, 120, 255),
            Self::LightMagenta => (180, 0, 158),
            Self::LightCyan => (97, 214, 214),
            Self::Gray => (242, 242, 242),
            Self::Rgb { r, g, b } | Self::Rgba { r, g, b, .. } => (r, g, b),
            Self::Indexed { index } => ansi256_to_rgb(index),
        };
        Some(SrgbColor::from_u8(r, g, b))
    }
}

/// Expand an ANSI-256 palette index to `(r, g, b)` bytes.
///
/// Indices 0–15 follow the standard 16-colour terminal palette.
/// Indices 16–231 are the 6×6×6 colour cube.  Indices 232–255 are
/// the 24-step grayscale ramp.
fn ansi256_to_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        0 => (0, 0, 0),
        1 => (128, 0, 0),
        2 => (0, 128, 0),
        3 => (128, 128, 0),
        4 => (0, 0, 128),
        5 => (128, 0, 128),
        6 => (0, 128, 128),
        7 => (192, 192, 192),
        8 => (128, 128, 128),
        9 => (255, 0, 0),
        10 => (0, 255, 0),
        11 => (255, 255, 0),
        12 => (0, 0, 255),
        13 => (255, 0, 255),
        14 => (0, 255, 255),
        15 => (255, 255, 255),
        16..=231 => {
            let n = index - 16;
            let r = n / 36;
            let g = (n % 36) / 6;
            let b = n % 6;
            let to_byte = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
            (to_byte(r), to_byte(g), to_byte(b))
        }
        232..=255 => {
            let gray = 8 + (index - 232) * 10;
            (gray, gray, gray)
        }
    }
}

/// Compute the WCAG 2.1 contrast ratio between two colors.
///
/// Uses palette's `Wcag21RelativeContrast` for accurate luminance-based
/// computation. Returns a value between 1.0 and 21.0.
#[cfg(feature = "color")]
#[tracing::instrument(level = "debug")]
pub fn contrast_ratio(fg: &SrgbColor, bg: &SrgbColor) -> f32 {
    use palette::Srgb;
    use palette::color_difference::Wcag21RelativeContrast;

    let fg_srgb: Srgb<f32> = Srgb::new(fg.r, fg.g, fg.b);
    let bg_srgb: Srgb<f32> = Srgb::new(bg.r, bg.g, bg.b);

    fg_srgb.relative_contrast(bg_srgb)
}

/// Compute contrast ratio without the `color` feature (pure math fallback).
///
/// Uses the WCAG 2.1 relative luminance formula directly.
#[cfg(not(feature = "color"))]
#[tracing::instrument(level = "debug")]
pub fn contrast_ratio(fg: &SrgbColor, bg: &SrgbColor) -> f32 {
    fn linearize(c: f32) -> f32 {
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }

    fn luminance(color: &SrgbColor) -> f32 {
        0.2126 * linearize(color.r) + 0.7152 * linearize(color.g) + 0.0722 * linearize(color.b)
    }

    let l1 = luminance(fg);
    let l2 = luminance(bg);
    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

/// Whether text is "large" per WCAG definitions.
///
/// Large text: ≥18pt (24px) normal weight, or ≥14pt (18.66px) bold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextSize {
    /// Normal text (below large thresholds).
    Normal,
    /// Large text (≥18pt or ≥14pt bold).
    Large,
}

/// WCAG 1.4.3 — Contrast (Minimum), Level AA.
///
/// - Normal text: 4.5:1
/// - Large text: 3:1
#[derive(Debug, Clone)]
pub struct ContrastMinimum {
    /// Foreground color.
    pub foreground: SrgbColor,
    /// Background color.
    pub background: SrgbColor,
    /// Text size category.
    pub text_size: TextSize,
}

impl Constraint for ContrastMinimum {
    #[tracing::instrument(level = "debug", skip(self, _ctx))]
    fn check(&self, _node_id: NodeId, _ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let ratio = contrast_ratio(&self.foreground, &self.background);
        let required = match self.text_size {
            TextSize::Normal => 4.5,
            TextSize::Large => 3.0,
        };

        if ratio >= required {
            Ok(())
        } else {
            Err(Violation::ContrastInsufficient {
                actual: ratio,
                required,
                foreground: self.foreground.to_hex(),
                background: self.background.to_hex(),
            })
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.3",
            level: WcagLevel::AA,
            url: "https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum",
        }
    }
}

/// WCAG 1.4.6 — Contrast (Enhanced), Level AAA.
///
/// - Normal text: 7:1
/// - Large text: 4.5:1
#[derive(Debug, Clone)]
pub struct ContrastEnhanced {
    /// Foreground color.
    pub foreground: SrgbColor,
    /// Background color.
    pub background: SrgbColor,
    /// Text size category.
    pub text_size: TextSize,
}

impl Constraint for ContrastEnhanced {
    #[tracing::instrument(level = "debug", skip(self, _ctx))]
    fn check(&self, _node_id: NodeId, _ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let ratio = contrast_ratio(&self.foreground, &self.background);
        let required = match self.text_size {
            TextSize::Normal => 7.0,
            TextSize::Large => 4.5,
        };

        if ratio >= required {
            Ok(())
        } else {
            Err(Violation::ContrastInsufficient {
                actual: ratio,
                required,
                foreground: self.foreground.to_hex(),
                background: self.background.to_hex(),
            })
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.6",
            level: WcagLevel::AAA,
            url: "https://www.w3.org/WAI/WCAG21/Understanding/contrast-enhanced",
        }
    }
}

/// WCAG 1.4.11 — Non-text Contrast, Level AA.
///
/// UI components and graphical objects require 3:1 contrast.
#[derive(Debug, Clone)]
pub struct NonTextContrast {
    /// Foreground (component) color.
    pub foreground: SrgbColor,
    /// Background color.
    pub background: SrgbColor,
}

impl Constraint for NonTextContrast {
    #[tracing::instrument(level = "debug", skip(self, _ctx))]
    fn check(&self, _node_id: NodeId, _ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let ratio = contrast_ratio(&self.foreground, &self.background);
        let required = 3.0_f32;

        if ratio >= required {
            Ok(())
        } else {
            Err(Violation::ContrastInsufficient {
                actual: ratio,
                required,
                foreground: self.foreground.to_hex(),
                background: self.background.to_hex(),
            })
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.11",
            level: WcagLevel::AA,
            url: "https://www.w3.org/WAI/WCAG21/Understanding/non-text-contrast",
        }
    }
}
