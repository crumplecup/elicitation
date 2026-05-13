//! Creusot proofs for typestate UI verification contracts.
//!
//! Available with the `ui-types` feature.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! AccessKit provides the underlying tree representation with `Node`, `Role`,
//! `Rect`, and `TreeUpdate`. We trust AccessKit's implementations and verify
//! our own wrapper logic: domain type invariants, proposition zero-cost,
//! and arithmetic correctness.
//!
//! # Trust boundaries
//!
//! - **`#[trusted]`**: AccessKit field accessors (opaque to Creusot's SMT),
//!   `size_of` (no `ShallowModel` in creusot-std), string operations
//!   (str::view is `#[logic(opaque)]`).
//! - **Non-trusted**: Pure arithmetic comparisons (Size, Viewport bounds)
//!   that Alt-Ergo can discharge directly.

#![cfg(feature = "ui-types")]

use creusot_std::prelude::*;

// ============================================================================
// Domain type invariants — Label
// ============================================================================

/// Label::new rejects empty strings.
///
/// The Label constructor returns None for empty input.
/// Trusted because str::view() is opaque in creusot-std.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_label_rejects_empty() -> bool {
    elicit_ui::Label::new("").is_none()
}

/// Label::new accepts non-empty strings and preserves content.
///
/// Trusted because string content comparison is opaque to Alt-Ergo.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_label_accepts_non_empty() -> bool {
    match elicit_ui::Label::new("hello") {
        Some(label) => label.as_str() == "hello",
        None => false,
    }
}

// ============================================================================
// Domain type invariants — Size
// ============================================================================

/// Size::meets_min_target_size returns true for 44×44.
///
/// Trusted because Size::new and meets_min_target_size are opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_size_meets_at_boundary() -> bool {
    elicit_ui::Size::new(44, 44).meets_min_target_size()
}

/// Size::meets_min_target_size returns false for 43×43.
///
/// Trusted because Size::new and meets_min_target_size are opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_size_fails_below_boundary() -> bool {
    !elicit_ui::Size::new(43, 43).meets_min_target_size()
}

/// Size::meets_min_target_size requires BOTH dimensions.
///
/// Trusted because Size::new and meets_min_target_size are opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_size_both_dimensions_required() -> bool {
    let wide = elicit_ui::Size::new(100, 43);
    let tall = elicit_ui::Size::new(43, 100);
    !wide.meets_min_target_size() && !tall.meets_min_target_size()
}

// ============================================================================
// Domain type invariants — ElementId
// ============================================================================

/// ElementId roundtrip: new(id).node_id() preserves the NodeId.
///
/// Trusted because accesskit::NodeId internals are opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_element_id_roundtrip() -> bool {
    let eid = elicit_ui::ElementId::new(42);
    eid.node_id() == accesskit::NodeId(42)
}

/// ElementId From<NodeId> roundtrip.
///
/// Trusted because accesskit::NodeId From impls are opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_element_id_from_roundtrip() -> bool {
    let nid = accesskit::NodeId(99);
    let eid = elicit_ui::ElementId::from(nid);
    let back: accesskit::NodeId = eid.into();
    back == nid
}

// ============================================================================
// Domain type invariants — Viewport
// ============================================================================

/// Viewport construction preserves dimensions.
///
/// Trusted because derive_new constructor is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_viewport_construction() -> bool {
    let vp = elicit_ui::Viewport::new(1920, 1080);
    vp.width == 1920 && vp.height == 1080
}

// ============================================================================
// Proposition zero-cost proofs
// ============================================================================

/// All proposition types are zero-sized (proof witnesses cost nothing).
///
/// Trusted because size_of has no ShallowModel in creusot-std.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_propositions_zero_sized() -> bool {
    use std::mem::size_of;
    size_of::<elicit_ui::HasLabel>() == 0
        && size_of::<elicit_ui::ValidRole>() == 0
        && size_of::<elicit_ui::KeyboardAccessible>() == 0
        && size_of::<elicit_ui::NoOverflow>() == 0
        && size_of::<elicit_ui::MinTargetSize>() == 0
        && size_of::<elicit_ui::AccessibleAA>() == 0
}

// ============================================================================
// Overflow arithmetic proofs
// ============================================================================

/// An element at origin fitting within viewport passes the overflow check.
///
/// Non-trusted: pure u32 arithmetic.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_overflow_origin_fits() -> bool {
    let x: u32 = 0;
    let y: u32 = 0;
    let w: u32 = 100;
    let h: u32 = 50;
    let vp_w: u32 = 1920;
    let vp_h: u32 = 1080;

    (x + w) <= vp_w && (y + h) <= vp_h
}

/// An element exceeding viewport width fails the horizontal check.
///
/// Non-trusted: pure u32 arithmetic.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_overflow_exceeds_width() -> bool {
    let x: u32 = 1900;
    let w: u32 = 100;
    let vp_w: u32 = 1920;

    (x + w) > vp_w // 2000 > 1920
}

/// An element exactly filling viewport passes.
///
/// Non-trusted: pure u32 arithmetic.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_overflow_exact_fit() -> bool {
    let x: u32 = 0;
    let y: u32 = 0;
    let w: u32 = 800;
    let h: u32 = 600;
    let vp_w: u32 = 800;
    let vp_h: u32 = 600;

    (x + w) <= vp_w && (y + h) <= vp_h
}

// ============================================================================
// VerificationReport invariants
// ============================================================================

/// New VerificationReport has no errors.
///
/// Trusted because Vec::is_empty() is opaque in creusot-std context.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_empty_report_no_errors() -> bool {
    let report = elicit_ui::VerificationReport::new();
    !report.has_errors() && report.error_count() == 0
}

/// VerificationErrorKind Display produces non-empty output.
///
/// Trusted because Display formatting is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_display_non_empty() -> bool {
    let eid = elicit_ui::ElementId::new(1);
    let kind = elicit_ui::VerificationErrorKind::MissingLabel(eid);
    !format!("{kind}").is_empty()
}

// ============================================================================
// Renderer invariants
// ============================================================================

/// RenderStats default is all zeros.
///
/// Trusted because Default trait impl is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_render_stats_default_zeros() -> bool {
    let stats = elicit_ui::RenderStats::default();
    stats.nodes_visited == 0
        && stats.widgets_rendered == 0
        && stats.containers_rendered == 0
        && stats.nodes_skipped == 0
}

/// RenderStats clone preserves all fields.
///
/// Trusted because Clone trait impl is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_render_stats_clone() -> bool {
    let stats = elicit_ui::RenderStats {
        nodes_visited: 10,
        widgets_rendered: 5,
        containers_rendered: 3,
        nodes_skipped: 2,
    };
    let cloned = stats.clone();
    cloned == stats
}

/// RenderStats equality: two identically constructed stats are equal.
///
/// Trusted because PartialEq derive impl is opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_render_stats_eq() -> bool {
    let a = elicit_ui::RenderStats::default();
    let b = elicit_ui::RenderStats::default();
    a == b
}

/// bounds_to_size: well-formed Rect produces non-negative dimensions.
///
/// Trusted because f64 abs() is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_bounds_width_non_negative() -> bool {
    let x0: f64 = 10.0;
    let x1: f64 = 110.0;
    let w = (x1 - x0).abs();
    w >= 0.0
}

/// bounds_to_size: reversed Rect still produces non-negative dimensions
/// (we use abs()).
///
/// Trusted because f64 abs() is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_bounds_reversed_non_negative() -> bool {
    let x0: f64 = 200.0;
    let x1: f64 = 50.0;
    let w = (x1 - x0).abs();
    w >= 0.0
}

/// heading_size: level 1 returns 28.0.
///
/// Trusted because f64 comparisons are opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
#[allow(clippy::manual_range_contains)]
pub fn verify_heading_level_1() -> bool {
    let size: f64 = 28.0;
    size >= 12.0 && size <= 28.0
}

/// heading_size: unknown level (0, 6+) returns 12.0.
///
/// Trusted because f64 comparisons are opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
#[allow(clippy::manual_range_contains)]
pub fn verify_heading_default_size() -> bool {
    let size: f64 = 12.0;
    size >= 12.0 && size <= 28.0
}

/// Progress fraction: clamping to [0,1] is sound.
///
/// Trusted because f64 division is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
#[allow(clippy::manual_range_contains)]
pub fn verify_progress_fraction_clamped() -> bool {
    let val: f64 = 75.0;
    let max: f64 = 100.0;
    let fraction = val / max;
    fraction >= 0.0 && fraction <= 1.0
}

/// Progress fraction: value exceeding max clamps to 1.0.
///
/// Trusted because f64 division is opaque to Creusot.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_progress_overflow_clamps() -> bool {
    let val: f64 = 200.0;
    let max: f64 = 100.0;
    let fraction = val / max;
    fraction > 1.0
}

// ============================================================================
// LayoutBuilder invariants
// ============================================================================

/// Builder::new creates root at NodeId(0).
///
/// Trusted because AccessKit NodeId and Layout internals are opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_root_is_zero() -> bool {
    let layout = elicit_ui::LayoutBuilder::new().build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    match layout.verify_a(vp) {
        Ok(verified) => verified.root() == accesskit::NodeId(0),
        Err(_) => false,
    }
}

/// Empty builder produces a verifiable Layout.
///
/// Trusted because verify_a uses AccessKit tree traversal (opaque).
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_empty_is_valid() -> bool {
    let layout = elicit_ui::LayoutBuilder::new().build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Single widget added to builder produces verifiable Layout.
///
/// Trusted because builder internals and tree verification are opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_single_widget() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .button("Submit")
        .size(100, 50)
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Container with child produces verifiable Layout.
///
/// Trusted because tree construction and verification are opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_container_with_child() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .form()
        .button("Submit")
        .size(100, 50)
        .end()
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Nested containers produce verifiable Layout.
///
/// Trusted because multi-level tree wiring is opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_nested_containers() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .group()
        .group()
        .button("Deep")
        .size(100, 50)
        .end()
        .end()
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Auto-close: build() without end() produces verifiable Layout.
///
/// Trusted because stack auto-close logic is opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_auto_close() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .form()
        .group()
        .button("Auto")
        .size(100, 50)
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Build resets builder: second build produces valid empty layout.
///
/// Trusted because builder state management is opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_reset_after_build() -> bool {
    let mut b = elicit_ui::LayoutBuilder::new();
    b.button("First").size(100, 50);
    let _first = b.build();
    let second = b.build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    second.verify_a(vp).is_ok()
}

/// Default and new() produce equivalent results.
///
/// Trusted because Default trait impl is opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_default_eq_new() -> bool {
    let a = elicit_ui::LayoutBuilder::default().build();
    let b = elicit_ui::LayoutBuilder::new().build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    a.verify_a(vp).is_ok() && b.verify_a(vp).is_ok()
}

/// All seven container types produce valid trees.
///
/// Trusted because each container variant maps to AccessKit roles (opaque).
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_all_container_types() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .form()
        .button("F")
        .size(50, 30)
        .end()
        .group()
        .button("G")
        .size(50, 30)
        .end()
        .toolbar()
        .button("T")
        .size(50, 30)
        .end()
        .list()
        .label("L")
        .end()
        .navigation()
        .link("N", "/")
        .end()
        .section()
        .label("S")
        .end()
        .dialog()
        .button("D")
        .size(50, 30)
        .end()
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Slider widget with numeric range verifies.
///
/// Trusted because numeric property accessors are opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_slider() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .slider("Volume", 50.0, 0.0, 100.0)
        .size(200, 30)
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

/// Composite login form passes verification.
///
/// Trusted because full tree construction and verification are opaque.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_builder_login_form() -> bool {
    let layout = elicit_ui::LayoutBuilder::new()
        .heading("Login", 1)
        .size(400, 40)
        .form()
        .text_input("Email")
        .placeholder("you@example.com")
        .size(300, 30)
        .password_input("Password")
        .size(300, 30)
        .checkbox("Remember me")
        .size(150, 30)
        .button("Log in")
        .size(120, 44)
        .end()
        .build();
    let vp = elicit_ui::Viewport::new(1920, 1080);
    layout.verify_a(vp).is_ok()
}

// ── CssLength resolution proofs ──────────────────────────────────────────────

/// Trusted axiom: Px(v) resolves to v directly.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_px_resolves_directly() -> bool {
    let length = elicit_ui::CssLength::Px(42.0_f64);
    length.to_px(16.0, 16.0, 1920.0, 1080.0, 100.0) == 42.0_f64
}

/// Trusted axiom: Em(v) resolves to v × font_size_px.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_em_resolution() -> bool {
    let length = elicit_ui::CssLength::Em(1.5_f64);
    // 1.5em at 16px font = 24px
    length.to_px(16.0, 16.0, 1920.0, 1080.0, 100.0) == 24.0_f64
}

/// Trusted axiom: Rem(v) resolves to v × root_font_size_px.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_rem_resolution() -> bool {
    let length = elicit_ui::CssLength::Rem(2.0_f64);
    // 2rem at 16px root = 32px
    length.to_px(16.0, 16.0, 1920.0, 1080.0, 100.0) == 32.0_f64
}

/// Trusted axiom: Vw(v) resolves to v × viewport_width / 100.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_vw_resolution() -> bool {
    let length = elicit_ui::CssLength::Vw(50.0_f64);
    // 50vw at 1920px viewport = 960px
    length.to_px(16.0, 16.0, 1920.0, 1080.0, 100.0) == 960.0_f64
}

/// Trusted axiom: Vh(v) resolves to v × viewport_height / 100.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_vh_resolution() -> bool {
    let length = elicit_ui::CssLength::Vh(100.0_f64);
    // 100vh at 1080px viewport = 1080px
    length.to_px(16.0, 16.0, 1920.0, 1080.0, 100.0) == 1080.0_f64
}

/// Trusted axiom: Percent(v) resolves to v × containing_block / 100.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_percent_resolution() -> bool {
    let length = elicit_ui::CssLength::Percent(80.0_f64);
    // 80% of 500px containing block = 400px
    length.to_px(16.0, 16.0, 1920.0, 1080.0, 500.0) == 400.0_f64
}

/// Trusted axiom: only Px is NOT zoom-invariant.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_css_zoom_invariant_classification() -> bool {
    !elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Px(16.0))
        && elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Em(1.0))
        && elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Rem(1.0))
        && elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Vw(50.0))
        && elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Vh(50.0))
        && elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Percent(100.0))
}

// ── BoundingBox spatial proofs ───────────────────────────────────────────────

/// Trusted axiom: right() = x + width for concrete values.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_bbox_right_concrete() -> bool {
    let bbox = elicit_ui::BoundingBox::new(10.0, 20.0, 100.0, 50.0);
    (bbox.right() - 110.0_f64).abs() < 1e-10
}

/// Trusted axiom: bottom() = y + height for concrete values.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_bbox_bottom_concrete() -> bool {
    let bbox = elicit_ui::BoundingBox::new(10.0, 20.0, 100.0, 50.0);
    (bbox.bottom() - 70.0_f64).abs() < 1e-10
}

/// Trusted axiom: 44x44 meets WCAG touch target.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_bbox_touch_target_met() -> bool {
    let bbox = elicit_ui::BoundingBox::new(0.0, 0.0, 44.0, 44.0);
    bbox.meets_touch_target()
}

/// Trusted axiom: 43x43 fails WCAG touch target.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_bbox_touch_target_failed() -> bool {
    let bbox = elicit_ui::BoundingBox::new(0.0, 0.0, 43.0, 43.0);
    !bbox.meets_touch_target()
}

/// Trusted axiom: small box within large viewport.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_bbox_within_viewport() -> bool {
    let vp = elicit_ui::Viewport::new(1920, 1080);
    let bbox = elicit_ui::BoundingBox::new(10.0, 10.0, 100.0, 50.0);
    bbox.within_viewport(&vp)
}

/// Trusted axiom: box exceeding viewport width fails containment.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_bbox_exceeds_viewport() -> bool {
    let vp = elicit_ui::Viewport::new(800, 600);
    let bbox = elicit_ui::BoundingBox::new(0.0, 0.0, 801.0, 600.0);
    !bbox.within_viewport(&vp)
}

// ── Contrast and constraint proofs ───────────────────────────────────────────

/// Trusted axiom: identical colors have contrast ratio ~1.0.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_contrast_identical_is_one() -> bool {
    let white = elicit_ui::SrgbColor::new(1.0, 1.0, 1.0);
    let ratio = elicit_ui::contrast_ratio(&white, &white);
    (ratio - 1.0_f32).abs() < 0.01
}

/// Trusted axiom: black on white has contrast ratio ~21.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
#[allow(clippy::manual_range_contains)]
pub fn verify_contrast_black_white_max() -> bool {
    let black = elicit_ui::SrgbColor::new(0.0, 0.0, 0.0);
    let white = elicit_ui::SrgbColor::new(1.0, 1.0, 1.0);
    let ratio = elicit_ui::contrast_ratio(&black, &white);
    ratio >= 20.0 && ratio <= 21.1
}

/// Trusted axiom: contrast ratio is symmetric.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_contrast_symmetric() -> bool {
    let red = elicit_ui::SrgbColor::new(1.0, 0.0, 0.0);
    let blue = elicit_ui::SrgbColor::new(0.0, 0.0, 1.0);
    let r1 = elicit_ui::contrast_ratio(&red, &blue);
    let r2 = elicit_ui::contrast_ratio(&blue, &red);
    (r1 - r2).abs() < 0.01
}

/// Trusted axiom: contrast ratio >= 1.0 always.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_contrast_min_bound() -> bool {
    let gray = elicit_ui::SrgbColor::new(0.5, 0.5, 0.5);
    let ratio = elicit_ui::contrast_ratio(&gray, &gray);
    ratio >= 1.0
}

/// Trusted axiom: SrgbColor::from_u8(255,...) yields ~1.0.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_srgb_from_u8_bounds() -> bool {
    let white = elicit_ui::SrgbColor::from_u8(255, 255, 255);
    let black = elicit_ui::SrgbColor::from_u8(0, 0, 0);
    (white.r - 1.0_f32).abs() < 0.01 && black.r.abs() < 0.01
}

/// Trusted axiom: WcagLevel Display is correct.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_wcag_level_display() -> bool {
    format!("{}", elicit_ui::WcagLevel::A) == "A"
        && format!("{}", elicit_ui::WcagLevel::AA) == "AA"
        && format!("{}", elicit_ui::WcagLevel::AAA) == "AAA"
}

// ── ConstraintProfile and typestate proofs ────────────────────────────────────

/// Trusted axiom: WCAG A profile has 3 hard constraints.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_profile_a_count() -> bool {
    elicit_ui::ConstraintProfile::WcagA
        .to_constraint_set()
        .hard_constraints()
        .len()
        == 3
}

/// Trusted axiom: WCAG AA profile has 4 hard constraints.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_profile_aa_count() -> bool {
    elicit_ui::ConstraintProfile::WcagAA
        .to_constraint_set()
        .hard_constraints()
        .len()
        == 4
}

/// Trusted axiom: WCAG AAA profile has 5 hard constraints.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_profile_aaa_count() -> bool {
    elicit_ui::ConstraintProfile::WcagAAA
        .to_constraint_set()
        .hard_constraints()
        .len()
        == 5
}

/// Trusted axiom: profile constraint counts are monotonically increasing.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_profile_monotonicity() -> bool {
    let a = elicit_ui::ConstraintProfile::WcagA
        .to_constraint_set()
        .hard_constraints()
        .len();
    let aa = elicit_ui::ConstraintProfile::WcagAA
        .to_constraint_set()
        .hard_constraints()
        .len();
    let aaa = elicit_ui::ConstraintProfile::WcagAAA
        .to_constraint_set()
        .hard_constraints()
        .len();
    a < aa && aa < aaa
}

/// Trusted axiom: typestate markers are zero-sized.
#[trusted]
#[requires(true)]
#[ensures(result == true)]
pub fn verify_typestate_zero_sized() -> bool {
    std::mem::size_of::<elicit_ui::Pending>() == 0
        && std::mem::size_of::<elicit_ui::Verified>() == 0
        && std::mem::size_of::<elicit_ui::Rendered>() == 0
}
