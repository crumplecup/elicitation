//! [`RenderVerifiable`] impls for WCAG contrast propositions in ratatui.
//!
//! Implements post-render sanity checks for the five contrast-related WCAG
//! Success Criteria.  Each implementation uses [`RatatuiRenderContext::colors_at`]
//! to sample cell colours and asserts the required contrast ratio holds.
//!
//! # Thresholds
//!
//! | Prop | SC | Ratio |
//! |------|----|-------|
//! | [`WcagContrastMinimumNormalText`] | 1.4.3 AA | 4.5 : 1 |
//! | [`WcagContrastMinimumLargeText`] | 1.4.3 AA | 3.0 : 1 |
//! | [`WcagContrastEnhancedNormalText`] | 1.4.6 AAA | 7.0 : 1 |
//! | [`WcagContrastEnhancedLargeText`] | 1.4.6 AAA | 4.5 : 1 |
//! | [`WcagNonTextContrastMinimum`] | 1.4.11 AA | 3.0 : 1 |

use crate::render_context::RatatuiRenderContext;
use elicit_ui::{
    RenderContext, RenderVerifiable, WcagContrastEnhancedLargeText, WcagContrastEnhancedNormalText,
    WcagContrastMinimumLargeText, WcagContrastMinimumNormalText, WcagNodeProofs,
    WcagNonTextContrastMinimum,
};
use ratatui::layout::Rect;

// ── Contrast ratio helper ─────────────────────────────────────────────────────

/// Compute the WCAG relative luminance of a linear sRGB component in [0, 1].
#[inline]
fn linearise(c: u8) -> f64 {
    let c = c as f64 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Compute WCAG relative luminance for an `[r, g, b]` triple.
#[inline]
fn luminance(rgb: [u8; 3]) -> f64 {
    let [r, g, b] = rgb;
    0.2126 * linearise(r) + 0.7152 * linearise(g) + 0.0722 * linearise(b)
}

/// WCAG contrast ratio: `(L1 + 0.05) / (L2 + 0.05)` where L1 ≥ L2.
///
/// Returns `None` if either colour cannot be determined from the buffer.
pub fn wcag_contrast_ratio(fg: [u8; 3], bg: [u8; 3]) -> f64 {
    let l1 = luminance(fg);
    let l2 = luminance(bg);
    let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

// ── Shared helper ─────────────────────────────────────────────────────────────

/// Sample the first cell with determinate colours in `area`.
///
/// Returns `None` when all cells use terminal-default or indexed colours.
fn sample_contrast(ctx: &RatatuiRenderContext<'_>, area: &Rect) -> Option<f64> {
    for row in 0..area.height {
        for col in 0..area.width {
            if let Some(colors) = ctx.colors_at(area, col, row) {
                return Some(wcag_contrast_ratio(colors.foreground, colors.background));
            }
        }
    }
    None
}

// ── Macro to reduce boilerplate ───────────────────────────────────────────────

macro_rules! impl_contrast_verifiable {
    ($prop:ty, $threshold:expr, $sc:literal) => {
        impl<'a> RenderVerifiable<RatatuiRenderContext<'a>> for $prop {
            fn verify_rendered(ctx: &RatatuiRenderContext<'a>, area: &Rect) {
                if let Some(ratio) = sample_contrast(ctx, area) {
                    if ratio < $threshold {
                        tracing::error!(
                            area = ?area,
                            ratio = ratio,
                            required = $threshold,
                            sc = $sc,
                            "WCAG contrast ratio below threshold",
                        );
                        debug_assert!(
                            ratio >= $threshold,
                            "WCAG {sc} contrast ratio {ratio:.2} below required {threshold:.1}",
                            sc = $sc,
                            ratio = ratio,
                            threshold = $threshold,
                        );
                    }
                }
                // No determinate colour → skip check (terminal default colours
                // are the user's responsibility; we can only verify RGB/named cells).
            }
        }
    };
}

impl_contrast_verifiable!(WcagContrastMinimumNormalText, 4.5_f64, "1.4.3 AA normal");
impl_contrast_verifiable!(WcagContrastMinimumLargeText, 3.0_f64, "1.4.3 AA large");
impl_contrast_verifiable!(WcagContrastEnhancedNormalText, 7.0_f64, "1.4.6 AAA normal");
impl_contrast_verifiable!(WcagContrastEnhancedLargeText, 4.5_f64, "1.4.6 AAA large");
impl_contrast_verifiable!(WcagNonTextContrastMinimum, 3.0_f64, "1.4.11 AA non-text");

// ── Dispatch helper ───────────────────────────────────────────────────────────

/// Run all populated WCAG proof checks for a node after it has been drawn.
///
/// Called by `render_node` for every [`crate::serde_types::TuiNode::Widget`]
/// after `render_widget` completes.  In release builds the compiler elides
/// all debug_assert paths to zero cost.
#[inline]
pub fn verify_wcag_proofs(ctx: &RatatuiRenderContext<'_>, area: &Rect, proofs: &WcagNodeProofs) {
    use elicit_ui::verify_in_debug;

    if proofs.contrast_normal.is_some() {
        verify_in_debug::<WcagContrastMinimumNormalText, _>(ctx, area);
    }
    if proofs.contrast_large.is_some() {
        verify_in_debug::<WcagContrastMinimumLargeText, _>(ctx, area);
    }
    if proofs.contrast_enhanced.is_some() {
        verify_in_debug::<WcagContrastEnhancedNormalText, _>(ctx, area);
    }
    if proofs.contrast_enhanced_large.is_some() {
        verify_in_debug::<WcagContrastEnhancedLargeText, _>(ctx, area);
    }
    if proofs.non_text_contrast.is_some() {
        verify_in_debug::<WcagNonTextContrastMinimum, _>(ctx, area);
    }
}
