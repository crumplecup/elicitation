//! Apply visual styles — WCAG contrast is enforced at the contract level.

use elicitation::Established;

use crate::{SrgbColor, SufficientContrast, UiResult, WidgetId};

/// Apply visual styles — WCAG contrast is enforced at the contract level.
pub trait UiStyleManager: Send + Sync {
    /// Set foreground/background color pair.
    ///
    /// Returns `Err` if the contrast ratio is below 4.5:1 (WCAG 1.4.3 Level AA).
    fn set_colors(
        &self,
        widget: WidgetId,
        fg: SrgbColor,
        bg: SrgbColor,
    ) -> UiResult<Established<SufficientContrast>>;

    /// Set font size in logical pixels.
    fn set_font_size(&self, widget: WidgetId, size_px: f32) -> UiResult<()>;

    /// Set padding/margin spacing in logical pixels.
    fn set_spacing(&self, widget: WidgetId, px: f32) -> UiResult<()>;

    /// Apply a named theme — verifies all color pairs meet AA contrast.
    fn apply_theme(&self, theme_name: &str) -> UiResult<Established<SufficientContrast>>;
}
