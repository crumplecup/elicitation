//! WCAG contrast verification for ratatui.
//!
//! Delegates to the generic [`elicit_ui::verify_wcag_contrast_proofs`]
//! function, which uses `RenderContext::sample_colors` — overridden on
//! [`RatatuiRenderContext`] to scan cells for the first determinate RGB pair.
