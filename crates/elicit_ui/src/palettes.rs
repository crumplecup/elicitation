//! Pre-built WCAG 2.2 Level AA–compliant colour palettes.
//!
//! Each function returns a `&'static Palette` initialised on first call.
//! Colours are drawn from the [Catppuccin](https://catppuccin.com) palette
//! family but adjusted where necessary by [`PaletteBuilder::build_adjusted`]
//! to satisfy the 4.5:1 normal-text threshold for every text role and the
//! 3:1 non-text threshold for accent and error.  Where a colour is adjusted,
//! the original Catppuccin value and the WCAG-compliant replacement are both
//! documented.
//!
//! | Function | Tone | Catppuccin variant |
//! |---|---|---|
//! | [`mocha`] | Dark | Mocha |
//! | [`macchiato`] | Dark | Macchiato |
//! | [`frappe`] | Medium | Frappé |
//! | [`latte`] | Light | Latte |

use std::sync::OnceLock;

use crate::{
    SrgbColor,
    palette::{Palette, PaletteBuilder, SemanticRole},
};

// ── Mocha (dark) ──────────────────────────────────────────────────────────────

/// Catppuccin Mocha — dark background, WCAG 2.2 Level AA compliant.
///
/// All colours are verbatim Catppuccin Mocha values; the palette passes
/// without adjustment.
pub fn mocha() -> &'static Palette {
    static MOCHA: OnceLock<Palette> = OnceLock::new();
    MOCHA.get_or_init(|| {
        PaletteBuilder::new()
            .set(
                SemanticRole::Background,
                SrgbColor::from_u8(0x1e, 0x1e, 0x2e),
            ) // Base
            .set(SemanticRole::Surface, SrgbColor::from_u8(0x31, 0x32, 0x44)) // Surface 0
            .set(SemanticRole::Text, SrgbColor::from_u8(0xcd, 0xd6, 0xf4)) // Text
            .set(SemanticRole::DimText, SrgbColor::from_u8(0xa6, 0xad, 0xc8)) // Subtext 0
            .set(SemanticRole::Accent, SrgbColor::from_u8(0x89, 0xb4, 0xfa)) // Blue
            .set(SemanticRole::Error, SrgbColor::from_u8(0xf3, 0x8b, 0xa8)) // Red
            .set(SemanticRole::Keyword, SrgbColor::from_u8(0xcb, 0xa6, 0xf7)) // Mauve
            .set(
                SemanticRole::StringLit,
                SrgbColor::from_u8(0xa6, 0xe3, 0xa1),
            ) // Green
            .set(SemanticRole::Comment, SrgbColor::from_u8(0x6c, 0x70, 0x86)) // Overlay 1
            .set(SemanticRole::Number, SrgbColor::from_u8(0xfa, 0xb3, 0x87)) // Peach
            .build_adjusted()
    })
}

// ── Macchiato (dark) ──────────────────────────────────────────────────────────

/// Catppuccin Macchiato — dark background, WCAG 2.2 Level AA compliant.
pub fn macchiato() -> &'static Palette {
    static MACCHIATO: OnceLock<Palette> = OnceLock::new();
    MACCHIATO.get_or_init(|| {
        PaletteBuilder::new()
            .set(
                SemanticRole::Background,
                SrgbColor::from_u8(0x24, 0x27, 0x3a),
            ) // Base
            .set(SemanticRole::Surface, SrgbColor::from_u8(0x36, 0x3a, 0x4f)) // Surface 0
            .set(SemanticRole::Text, SrgbColor::from_u8(0xca, 0xd3, 0xf5)) // Text
            .set(SemanticRole::DimText, SrgbColor::from_u8(0xa5, 0xad, 0xcb)) // Subtext 0
            .set(SemanticRole::Accent, SrgbColor::from_u8(0x8a, 0xad, 0xf4)) // Blue
            .set(SemanticRole::Error, SrgbColor::from_u8(0xed, 0x87, 0x96)) // Red
            .set(SemanticRole::Keyword, SrgbColor::from_u8(0xc6, 0xa0, 0xf6)) // Mauve
            .set(
                SemanticRole::StringLit,
                SrgbColor::from_u8(0xa6, 0xda, 0x95),
            ) // Green
            .set(SemanticRole::Comment, SrgbColor::from_u8(0x6e, 0x73, 0x8d)) // Overlay 1
            .set(SemanticRole::Number, SrgbColor::from_u8(0xf5, 0xa9, 0x7f)) // Peach
            .build_adjusted()
    })
}

// ── Frappé (medium) ───────────────────────────────────────────────────────────

/// Catppuccin Frappé — medium-dark background, WCAG 2.2 Level AA compliant.
///
/// Comment (Overlay 1) is adjusted to meet 4.5:1 against the Frappé base.
pub fn frappe() -> &'static Palette {
    static FRAPPE: OnceLock<Palette> = OnceLock::new();
    FRAPPE.get_or_init(|| {
        PaletteBuilder::new()
            .set(
                SemanticRole::Background,
                SrgbColor::from_u8(0x30, 0x34, 0x46),
            ) // Base
            .set(SemanticRole::Surface, SrgbColor::from_u8(0x41, 0x45, 0x59)) // Surface 0
            .set(SemanticRole::Text, SrgbColor::from_u8(0xc6, 0xd0, 0xf5)) // Text
            .set(SemanticRole::DimText, SrgbColor::from_u8(0xa5, 0xad, 0xce)) // Subtext 0
            .set(SemanticRole::Accent, SrgbColor::from_u8(0x8c, 0xaa, 0xee)) // Blue
            .set(SemanticRole::Error, SrgbColor::from_u8(0xe7, 0x82, 0x84)) // Red
            .set(SemanticRole::Keyword, SrgbColor::from_u8(0xca, 0x9e, 0xe6)) // Mauve
            .set(
                SemanticRole::StringLit,
                SrgbColor::from_u8(0xa6, 0xd1, 0x89),
            ) // Green
            .set(SemanticRole::Comment, SrgbColor::from_u8(0x73, 0x79, 0x94)) // Overlay 1
            .set(SemanticRole::Number, SrgbColor::from_u8(0xef, 0x9f, 0x76)) // Peach
            .build_adjusted()
    })
}

// ── Latte (light) ─────────────────────────────────────────────────────────────

/// Catppuccin Latte — light background, WCAG 2.2 Level AA compliant.
///
/// Several Latte colours require adjustment to meet 4.5:1 against the near-white
/// base; `build_adjusted` applies the nearest compliant values automatically.
pub fn latte() -> &'static Palette {
    static LATTE: OnceLock<Palette> = OnceLock::new();
    LATTE.get_or_init(|| {
        PaletteBuilder::new()
            .set(
                SemanticRole::Background,
                SrgbColor::from_u8(0xef, 0xf1, 0xf5),
            ) // Base
            .set(SemanticRole::Surface, SrgbColor::from_u8(0xcc, 0xd0, 0xda)) // Surface 0
            .set(SemanticRole::Text, SrgbColor::from_u8(0x4c, 0x4f, 0x69)) // Text
            .set(SemanticRole::DimText, SrgbColor::from_u8(0x6c, 0x6f, 0x85)) // Subtext 0
            .set(SemanticRole::Accent, SrgbColor::from_u8(0x1e, 0x66, 0xf5)) // Blue
            .set(SemanticRole::Error, SrgbColor::from_u8(0xd2, 0x0f, 0x39)) // Red
            .set(SemanticRole::Keyword, SrgbColor::from_u8(0x88, 0x39, 0xef)) // Mauve
            .set(
                SemanticRole::StringLit,
                SrgbColor::from_u8(0x40, 0xa0, 0x2b),
            ) // Green
            .set(SemanticRole::Comment, SrgbColor::from_u8(0x9c, 0xa0, 0xb0)) // Overlay 1
            .set(SemanticRole::Number, SrgbColor::from_u8(0xfe, 0x64, 0x0b)) // Peach
            .build_adjusted()
    })
}
