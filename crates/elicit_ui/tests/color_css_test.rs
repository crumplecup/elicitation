//! Tests for color contrast and CSS unit modules.

use elicit_ui::{
    contrast_ratio, is_zoom_invariant, Breakpoint, BreakpointSet, CssLength, SrgbColor, TextSize,
};

// ─── Color contrast ───────────────────────────────────────────────────

#[test]
fn black_on_white_has_max_contrast() {
    let black = SrgbColor::new(0.0, 0.0, 0.0);
    let white = SrgbColor::new(1.0, 1.0, 1.0);
    let ratio = contrast_ratio(&black, &white);
    assert!(ratio > 20.0, "Expected ~21:1, got {ratio}");
}

#[test]
fn white_on_white_has_min_contrast() {
    let white = SrgbColor::new(1.0, 1.0, 1.0);
    let ratio = contrast_ratio(&white, &white);
    assert!(ratio < 1.1, "Expected ~1:1, got {ratio}");
}

#[test]
fn contrast_is_symmetric() {
    let red = SrgbColor::new(1.0, 0.0, 0.0);
    let blue = SrgbColor::new(0.0, 0.0, 1.0);
    let r1 = contrast_ratio(&red, &blue);
    let r2 = contrast_ratio(&blue, &red);
    assert!((r1 - r2).abs() < 0.01);
}

#[test]
fn srgb_color_from_u8() {
    let color = SrgbColor::from_u8(255, 128, 0);
    assert!((color.r - 1.0).abs() < 0.01);
    assert!((color.g - 0.502).abs() < 0.01);
    assert!((color.b - 0.0).abs() < 0.01);
}

#[test]
fn srgb_color_to_hex() {
    let color = SrgbColor::from_u8(255, 128, 0);
    assert_eq!(color.to_hex(), "#ff8000");
}

// ─── CSS units ────────────────────────────────────────────────────────

#[test]
fn css_px_resolves_directly() {
    let length = CssLength::Px(16.0);
    assert_eq!(length.to_px(16.0, 16.0, 1920.0, 1080.0, 800.0), 16.0);
}

#[test]
fn css_em_resolves_relative_to_font_size() {
    let length = CssLength::Em(1.5);
    assert_eq!(length.to_px(16.0, 16.0, 1920.0, 1080.0, 800.0), 24.0);
}

#[test]
fn css_rem_resolves_relative_to_root_font_size() {
    let length = CssLength::Rem(2.0);
    assert_eq!(length.to_px(12.0, 16.0, 1920.0, 1080.0, 800.0), 32.0);
}

#[test]
fn css_vw_resolves_to_viewport_width() {
    let length = CssLength::Vw(50.0);
    assert_eq!(length.to_px(16.0, 16.0, 1920.0, 1080.0, 800.0), 960.0);
}

#[test]
fn css_vh_resolves_to_viewport_height() {
    let length = CssLength::Vh(100.0);
    assert_eq!(length.to_px(16.0, 16.0, 1920.0, 1080.0, 800.0), 1080.0);
}

#[test]
fn css_percent_resolves_to_containing_block() {
    let length = CssLength::Percent(50.0);
    assert_eq!(length.to_px(16.0, 16.0, 1920.0, 1080.0, 800.0), 400.0);
}

#[test]
fn zoom_invariant_classification() {
    assert!(!is_zoom_invariant(&CssLength::Px(16.0)));
    assert!(is_zoom_invariant(&CssLength::Em(1.0)));
    assert!(is_zoom_invariant(&CssLength::Rem(1.0)));
    assert!(is_zoom_invariant(&CssLength::Vw(50.0)));
    assert!(is_zoom_invariant(&CssLength::Vh(50.0)));
    assert!(is_zoom_invariant(&CssLength::Percent(100.0)));
}

// ─── Breakpoints ──────────────────────────────────────────────────────

#[test]
fn wcag_breakpoints_include_320() {
    let bps = BreakpointSet::wcag();
    assert!(bps.breakpoints().iter().any(|bp| bp.min_width == 320));
}

#[test]
fn breakpoint_set_custom() {
    let bps = BreakpointSet::wcag()
        .with_breakpoint(Breakpoint::new("ultra-wide", 2560, 3840));
    assert_eq!(bps.breakpoints().len(), 5);
}

// ─── Display formatting ──────────────────────────────────────────────

#[test]
fn css_length_display() {
    assert_eq!(format!("{}", CssLength::Px(16.0)), "16px");
    assert_eq!(format!("{}", CssLength::Em(1.5)), "1.5em");
    assert_eq!(format!("{}", CssLength::Rem(2.0)), "2rem");
    assert_eq!(format!("{}", CssLength::Vw(50.0)), "50vw");
    assert_eq!(format!("{}", CssLength::Vh(100.0)), "100vh");
    assert_eq!(format!("{}", CssLength::Percent(80.0)), "80%");
}
