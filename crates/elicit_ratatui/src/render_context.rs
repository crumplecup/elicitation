//! [`RenderContext`] implementation for ratatui's [`Buffer`].
//!
//! Import [`RatatuiRenderContext`] and wrap a `&Buffer` + `Rect` pair to
//! perform post-render invariant checks via [`verify_in_debug`].

use elicit_ui::RenderContext;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

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
}
