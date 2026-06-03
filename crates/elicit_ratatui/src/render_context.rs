//! [`RenderContext`] implementation for ratatui's [`Buffer`].
//!
//! Import [`RatatuiRenderContext`] and wrap a `&Buffer` + `Rect` pair to
//! perform post-render invariant checks via [`verify_in_debug`].

use elicit_ui::{RenderColors, RenderContext};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;

// Re-export so callers can write `elicit_ratatui::verify_in_debug`.
pub use elicit_ui::RenderVerifiable;
pub use elicit_ui::verify_in_debug as ratatui_verify_in_debug;

// ── RatatuiRenderContext ──────────────────────────────────────────────────────

/// A [`RenderContext`] that inspects a ratatui [`Buffer`] over a [`Rect`] area.
///
/// Wrap the buffer reference produced by `Frame::buffer_mut()` (or any
/// in-memory buffer used in unit tests) to run [`RenderVerifiable`]
/// implementations via [`elicit_ui::verify_in_debug`].
pub struct RatatuiRenderContext<'a> {
    buf: &'a Buffer,
}

impl<'a> RatatuiRenderContext<'a> {
    /// Creates a new context backed by `buf`.
    pub fn new(buf: &'a Buffer) -> Self {
        Self { buf }
    }
}

impl<'a> RenderContext for RatatuiRenderContext<'a> {
    /// [`ratatui::layout::Rect`] is the natural area type for ratatui buffers.
    type Area = Rect;

    fn symbol_at(&self, area: &Rect, col: u16, row: u16) -> &str {
        let x = area.x.saturating_add(col);
        let y = area.y.saturating_add(row);
        if x >= area.right() || y >= area.bottom() {
            return "";
        }
        self.buf.cell((x, y)).map_or("", |c| c.symbol())
    }

    fn area_width(&self, area: &Rect) -> u16 {
        area.width
    }

    fn area_height(&self, area: &Rect) -> u16 {
        area.height
    }

    fn colors_at(&self, area: &Rect, col: u16, row: u16) -> Option<RenderColors> {
        let x = area.x.saturating_add(col);
        let y = area.y.saturating_add(row);
        if x >= area.right() || y >= area.bottom() {
            return None;
        }
        let cell = self.buf.cell((x, y))?;
        let fg = color_to_rgb(cell.fg)?;
        let bg = color_to_rgb(cell.bg)?;
        Some(RenderColors {
            foreground: fg,
            background: bg,
        })
    }

    /// Scan all cells in `area` for the first one with determinate RGB colours.
    ///
    /// Ratatui cells default to `Color::Reset` (terminal default) until a
    /// widget writes to them.  Scanning finds the first cell that a widget
    /// actually coloured, giving a representative sample for contrast checks.
    fn sample_colors(&self, area: &Rect) -> Option<RenderColors> {
        for row in 0..area.height {
            for col in 0..area.width {
                if let Some(colors) = self.colors_at(area, col, row) {
                    return Some(colors);
                }
            }
        }
        None
    }
}

/// Convert a ratatui [`Color`] to `[r, g, b]`, returning `None` for colours
/// that require terminal-specific lookup (Reset, Indexed).
fn color_to_rgb(color: Color) -> Option<[u8; 3]> {
    match color {
        Color::Rgb(r, g, b) => Some([r, g, b]),
        Color::Black => Some([0, 0, 0]),
        Color::Red => Some([128, 0, 0]),
        Color::Green => Some([0, 128, 0]),
        Color::Yellow => Some([128, 128, 0]),
        Color::Blue => Some([0, 0, 128]),
        Color::Magenta => Some([128, 0, 128]),
        Color::Cyan => Some([0, 128, 128]),
        Color::Gray => Some([192, 192, 192]),
        Color::DarkGray => Some([128, 128, 128]),
        Color::LightRed => Some([255, 0, 0]),
        Color::LightGreen => Some([0, 255, 0]),
        Color::LightYellow => Some([255, 255, 0]),
        Color::LightBlue => Some([0, 0, 255]),
        Color::LightMagenta => Some([255, 0, 255]),
        Color::LightCyan => Some([0, 255, 255]),
        Color::White => Some([255, 255, 255]),
        // Terminal default or palette index — cannot determine RGB without runtime lookup.
        Color::Reset | Color::Indexed(_) => None,
    }
}
