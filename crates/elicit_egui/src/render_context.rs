//! Stub [`RenderContext`] for egui.
//!
//! egui renders via GPU (not a CPU-accessible buffer), so cell-level colour
//! inspection is not available at the ratatui level.  This stub satisfies the
//! trait bound so egui crates can be compiled against the same abstractions;
//! all methods return `None` or zero-size.
//!
//! A full implementation would require reading back the rendered texture from
//! the GPU, which is beyond the current scope.

use egui::Rect;
use elicit_ui::{RenderColors, RenderContext};

// ── EguiRenderContext ─────────────────────────────────────────────────────────

/// A [`RenderContext`] stub for egui frames.
///
/// Colour inspection is not implemented (returns `None` for `colors_at`).
/// Structural checks via `symbol_at` are also stubs — egui is a retained
/// immediate-mode renderer and does not expose a character grid.
pub struct EguiRenderContext;

impl RenderContext for EguiRenderContext {
    /// egui uses floating-point [`egui::Rect`] as its area type.
    type Area = Rect;

    fn symbol_at(&self, _area: &Rect, _col: u16, _row: u16) -> &str {
        ""
    }

    fn area_width(&self, area: &Rect) -> u16 {
        area.width() as u16
    }

    fn area_height(&self, area: &Rect) -> u16 {
        area.height() as u16
    }

    /// Always returns `None` — egui colour data is not available from the CPU.
    fn colors_at(&self, _area: &Rect, _col: u16, _row: u16) -> Option<RenderColors> {
        None
    }
}
