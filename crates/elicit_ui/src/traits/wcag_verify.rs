//! Generic WCAG contrast verification for any [`RenderContext`].
//!
//! Provides `impl<C: RenderContext> RenderVerifiable<C>` for the five
//! contrast-related WCAG Success Criteria, plus the
//! [`verify_wcag_contrast_proofs`] dispatch helper that exercises all five
//! fields on a [`WcagNodeProofs`] sidecar.
//!
//! The implementations call [`RenderContext::sample_colors`] to obtain a
//! representative colour pair for the widget area.  Frontends that can
//! inspect their render buffer should override `sample_colors` to scan for
//! the first cell with determinate colours; frontends that derive colours
//! from theme state (egui) or AccessKit node metadata (leptos) should store
//! colours on their context struct and return them from `colors_at`.
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

use crate::{
    RenderContext, RenderVerifiable, WcagContrastEnhancedLargeText, WcagContrastEnhancedNormalText,
    WcagContrastMinimumLargeText, WcagContrastMinimumNormalText, WcagNodeProofs,
    WcagNonTextContrastMinimum,
};

use super::render_verify::verify_in_debug;

// ── Contrast ratio math ───────────────────────────────────────────────────────

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
/// Suitable for use in custom [`RenderVerifiable`] implementations as well
/// as the built-in contrast checks.
pub fn wcag_contrast_ratio(fg: [u8; 3], bg: [u8; 3]) -> f64 {
    let l1 = luminance(fg);
    let l2 = luminance(bg);
    let (lighter, darker) = if l1 >= l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

// ── Generic RenderVerifiable impls ────────────────────────────────────────────

macro_rules! impl_contrast_verifiable {
    ($prop:ty, $threshold:expr, $sc:literal) => {
        impl<C: RenderContext> RenderVerifiable<C> for $prop {
            fn verify_rendered(ctx: &C, area: &C::Area) {
                if let Some(colors) = ctx.sample_colors(area) {
                    let ratio = wcag_contrast_ratio(colors.foreground, colors.background);
                    if ratio < $threshold {
                        tracing::error!(
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
                // No determinate colour → skip check (theme/terminal defaults are
                // the integrator's responsibility; we can only verify explicit RGB).
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

/// Run all populated WCAG contrast proof checks for a node.
///
/// Dispatches the five contrast-related fields in [`WcagNodeProofs`] through
/// [`verify_in_debug`], which compiles to nothing in release builds.
///
/// Call this after the widget has been drawn so that the render buffer
/// (for buffer-backed frontends) or context state (for theme-backed frontends)
/// reflects what was actually displayed.
#[inline]
pub fn verify_wcag_contrast_proofs<C: RenderContext>(
    ctx: &C,
    area: &C::Area,
    proofs: &WcagNodeProofs,
) {
    if proofs.contrast_normal.is_some() {
        verify_in_debug::<WcagContrastMinimumNormalText, C>(ctx, area);
    }
    if proofs.contrast_large.is_some() {
        verify_in_debug::<WcagContrastMinimumLargeText, C>(ctx, area);
    }
    if proofs.contrast_enhanced.is_some() {
        verify_in_debug::<WcagContrastEnhancedNormalText, C>(ctx, area);
    }
    if proofs.contrast_enhanced_large.is_some() {
        verify_in_debug::<WcagContrastEnhancedLargeText, C>(ctx, area);
    }
    if proofs.non_text_contrast.is_some() {
        verify_in_debug::<WcagNonTextContrastMinimum, C>(ctx, area);
    }
}
