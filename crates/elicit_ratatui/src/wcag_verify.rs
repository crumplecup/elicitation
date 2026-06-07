//! WCAG contrast verification for ratatui.
//!
//! Delegates to the generic [`elicit_ui::verify_wcag_contrast_proofs`]
//! function, which uses `RenderContext::sample_colors` — overridden on
//! [`RatatuiRenderContext`] to scan cells for the first determinate RGB pair.
//!
//! # Scope of post-render verification
//!
//! Buffer scanning can only verify the five contrast-ratio proofs
//! (`contrast_normal`, `contrast_large`, `contrast_enhanced`,
//! `contrast_enhanced_large`, `non_text_contrast`).  All other WCAG proofs are
//! structural — accessible names, heading levels, error labels — and are checked
//! pre-render in [`RatatuiBackend::verify_node`] against AccessKit node metadata.
//!
//! Contrast checks silently skip cells rendered with [`ratatui::style::Color::Reset`]
//! or [`ratatui::style::Color::Indexed`] because their actual RGB values depend on
//! host terminal configuration and cannot be resolved from the buffer alone.
