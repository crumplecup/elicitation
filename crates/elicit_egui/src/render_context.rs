//! [`RenderContext`] for egui — theme-based colour inspection.
//!
//! egui renders via GPU (not a CPU-accessible buffer), so per-cell colour
//! inspection from a render buffer is not available.  Instead, this context
//! reads the current `ui.visuals()` state at widget-draw time to obtain a
//! best-effort foreground/background pair for WCAG contrast checks.
//!
//! **Limitations:** Widget-specific colour overrides (e.g. `colored_label`,
//! selection highlights, `RichText`) are not captured — the context reflects
//! the theme's default text/background colours only.  When the colour cannot
//! be determined (e.g. alpha < 255 on either channel), `colors_at` returns
//! `None` and the contrast check is silently skipped.

use egui::Rect;
use elicit_ui::{RenderColors, RenderContext};

// ── EguiRenderContext ─────────────────────────────────────────────────────────

/// A [`RenderContext`] for egui that derives colours from `ui.visuals()`.
///
/// Construct via [`EguiRenderContext::from_ui`] inside a widget closure to
/// capture the theme state at draw time.  All egui colour inspection is
/// best-effort: only fully-opaque theme colours are reported.
pub struct EguiRenderContext {
    fg: Option<[u8; 3]>,
    bg: Option<[u8; 3]>,
}

impl EguiRenderContext {
    /// Derive a render context from the current egui `Ui` theme state.
    ///
    /// Reads `ui.visuals().text_color()` and
    /// `ui.visuals().widgets.noninteractive.bg_fill`.  Returns `None` for any
    /// channel whose alpha is less than 255 (translucent colours are
    /// meaningless for WCAG opaque contrast maths).
    pub fn from_ui(ui: &egui::Ui) -> Self {
        let vis = ui.visuals();
        Self {
            fg: color32_to_rgb(vis.text_color()),
            bg: color32_to_rgb(vis.widgets.noninteractive.bg_fill),
        }
    }
}

/// Extract `[r, g, b]` from a `Color32`, returning `None` when alpha < 255.
fn color32_to_rgb(c: egui::Color32) -> Option<[u8; 3]> {
    if c.a() < 255 {
        None
    } else {
        Some([c.r(), c.g(), c.b()])
    }
}

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

    /// Returns the stored theme colours, or `None` if either was translucent.
    ///
    /// `col` and `row` are ignored — egui colour state is uniform across the
    /// widget area; this context stores a single pair derived from visuals.
    fn colors_at(&self, _area: &Rect, _col: u16, _row: u16) -> Option<RenderColors> {
        match (self.fg, self.bg) {
            (Some(fg), Some(bg)) => Some(RenderColors {
                foreground: fg,
                background: bg,
            }),
            _ => None,
        }
    }
}
