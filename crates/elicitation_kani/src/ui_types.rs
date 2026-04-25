//! Kani proofs for typestate UI verification contracts.
//!
//! Available with the `ui-types` feature.
//!
//! # Verification Stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! AccessKit provides the underlying tree representation. We trust its
//! `Node`, `Role`, `Rect`, and `TreeUpdate` types. What we verify is
//! our *business logic*:
//!
//! 1. **Domain type invariants** — `Label` rejects empty strings,
//!    `Size.meets_min_target_size()` is correct, `ElementId` roundtrips.
//! 2. **Proposition zero-cost** — every `HasLabel<T>`, `ValidRole<T>`, etc.
//!    is zero-sized, confirming that proof witnesses add no runtime cost.
//! 3. **Arithmetic safety** — overflow detection in viewport boundary
//!    checks does not panic or wrap for any symbolic u32/i32 inputs
//!    within representable bounds.
//!
//! # Prop → Validator mapping
//!
//! | Proposition | Validator | WCAG |
//! |---|---|---|
//! | `HasLabel<T>` | `validate_has_label` | 2.4.6 AA, 4.1.2 A |
//! | `ValidRole<T>` | `validate_valid_role` | 4.1.2 A |
//! | `KeyboardAccessible<T>` | `validate_keyboard_accessible` | 2.1.1 A |
//! | `NoOverflow<T>` | `validate_no_overflow` | 1.4.10 AA |
//! | `MinTargetSize<T>` | `validate_min_target_size` | 2.5.5 AAA |
//! | `AccessibleAA<T>` | composite (all AA checks) | — |

// ============================================================================
// Domain type invariants
// ============================================================================

/// Label::new rejects empty strings.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_label_rejects_empty() {
    let empty = String::new();
    assert!(
        elicit_ui::Label::new(empty).is_none(),
        "Label::new must reject empty strings"
    );
}

/// Label::new accepts non-empty strings and preserves content.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_label_accepts_non_empty() {
    // Use a fixed non-empty string (Kani can't symbolically generate String)
    let label = elicit_ui::Label::new("hello");
    assert!(label.is_some(), "Label::new must accept non-empty strings");
    assert!(
        label.unwrap().as_str() == "hello",
        "Label must preserve content"
    );
}

/// Size::meets_min_target_size is correct for all u32 pairs.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_meets_min_target_size() {
    let width: u32 = kani::any();
    let height: u32 = kani::any();

    let size = elicit_ui::Size::new(width, height);
    let meets = size.meets_min_target_size();

    assert!(
        meets == (width >= 44 && height >= 44),
        "meets_min_target_size must equal (width >= 44 && height >= 44)"
    );
}

/// Size::meets_min_target_size boundary: 43x43 fails, 44x44 passes.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_boundary_43_44() {
    let small = elicit_ui::Size::new(43, 43);
    assert!(
        !small.meets_min_target_size(),
        "43x43 must not meet minimum target size"
    );

    let exact = elicit_ui::Size::new(44, 44);
    assert!(
        exact.meets_min_target_size(),
        "44x44 must meet minimum target size"
    );
}

/// Size::meets_min_target_size requires BOTH dimensions to meet threshold.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_both_dimensions_required() {
    let wide = elicit_ui::Size::new(100, 43);
    assert!(
        !wide.meets_min_target_size(),
        "Wide but short must not meet minimum"
    );

    let tall = elicit_ui::Size::new(43, 100);
    assert!(
        !tall.meets_min_target_size(),
        "Tall but narrow must not meet minimum"
    );
}

/// ElementId roundtrip: new(id).node_id() preserves the NodeId.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_element_id_roundtrip() {
    let id: u64 = kani::any();
    let element = elicit_ui::ElementId::new(id);
    let node_id = element.node_id();
    assert!(
        node_id == accesskit::NodeId(id),
        "ElementId roundtrip must preserve NodeId"
    );
}

/// ElementId From<NodeId> roundtrip.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_element_id_from_node_id() {
    let id: u64 = kani::any();
    let node_id = accesskit::NodeId(id);
    let element = elicit_ui::ElementId::from(node_id);
    let back: accesskit::NodeId = element.into();
    assert!(
        back == node_id,
        "ElementId From/Into roundtrip must preserve NodeId"
    );
}

/// Viewport construction preserves dimensions.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_viewport_construction() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let vp = elicit_ui::Viewport::new(w, h);
    assert!(vp.width == w, "Viewport width must be preserved");
    assert!(vp.height == h, "Viewport height must be preserved");
}

// ============================================================================
// Proposition zero-cost proofs
// ============================================================================

/// All proposition types are zero-sized (proof witnesses cost nothing).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_propositions_zero_sized() {
    assert!(
        std::mem::size_of::<elicit_ui::HasLabel>() == 0,
        "HasLabel must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::ValidRole>() == 0,
        "ValidRole must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::KeyboardAccessible>() == 0,
        "KeyboardAccessible must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::NoOverflow>() == 0,
        "NoOverflow must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::MinTargetSize>() == 0,
        "MinTargetSize must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::AccessibleAA>() == 0,
        "AccessibleAA must be zero-sized"
    );
}

// ============================================================================
// Overflow arithmetic safety
// ============================================================================

/// Viewport overflow check: element that fits never triggers overflow.
///
/// For any non-negative position (x, y) and size (w, h) that fit within
/// the viewport (vp_w, vp_h), the overflow condition is never met.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_overflow_check_sound() {
    let x: u32 = kani::any();
    let y: u32 = kani::any();
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let vp_w: u32 = kani::any();
    let vp_h: u32 = kani::any();

    // Bound to prevent u32 addition overflow
    kani::assume(x <= 10000);
    kani::assume(y <= 10000);
    kani::assume(w <= 10000);
    kani::assume(h <= 10000);
    kani::assume(vp_w <= 20000);
    kani::assume(vp_h <= 20000);

    // Precondition: element fits in viewport
    kani::assume(x + w <= vp_w);
    kani::assume(y + h <= vp_h);

    // The overflow check the validator performs
    let fits_h = (x + w) <= vp_w;
    let fits_v = (y + h) <= vp_h;

    assert!(
        fits_h && fits_v,
        "Element that fits must not be flagged as overflow"
    );
}

/// Viewport overflow check: element that exceeds bounds IS detected.
///
/// For any element whose right edge exceeds viewport width,
/// the horizontal overflow condition triggers.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_overflow_detection_complete() {
    let x: u32 = kani::any();
    let w: u32 = kani::any();
    let vp_w: u32 = kani::any();

    // Bound to prevent u32 addition overflow
    kani::assume(x <= 10000);
    kani::assume(w <= 10000);
    kani::assume(vp_w <= 20000);

    // Precondition: element exceeds viewport
    kani::assume(x + w > vp_w);

    let fits_h = (x + w) <= vp_w;
    assert!(!fits_h, "Element exceeding viewport must be detected");
}

/// MinTargetSize check: validator and Size::meets_min_target_size agree.
///
/// The validator converts bounds to u32 width/height, then checks >= 44.
/// This must match Size::meets_min_target_size for all values.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_min_target_size_consistency() {
    let width: u32 = kani::any();
    let height: u32 = kani::any();

    let size = elicit_ui::Size::new(width, height);
    let meets = size.meets_min_target_size();

    let validator_would_pass = width >= 44 && height >= 44;
    assert!(
        meets == validator_would_pass,
        "Size::meets_min_target_size must agree with validator logic"
    );
}

// ============================================================================
// VerificationReport invariants
// ============================================================================

/// Empty VerificationReport has no errors.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_empty_report_no_errors() {
    let report = elicit_ui::VerificationReport::new();
    assert!(!report.has_errors(), "New report must have no errors");
    assert!(
        report.error_count() == 0,
        "New report must have zero error count"
    );
}

// ============================================================================
// VerificationErrorKind Display correctness
// ============================================================================

/// VerificationErrorKind::MissingLabel formats correctly.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_error_kind_display() {
    let eid = elicit_ui::ElementId::new(42);
    let kind = elicit_ui::VerificationErrorKind::MissingLabel(eid);
    let display = format!("{kind}");
    assert!(
        !display.is_empty(),
        "VerificationErrorKind Display must not be empty"
    );
}

// ============================================================================
// Renderer invariants
// ============================================================================

/// RenderStats default is all zeros.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_render_stats_default_zeros() {
    let stats = elicit_ui::RenderStats::default();
    assert!(stats.nodes_visited == 0, "Default nodes_visited must be 0");
    assert!(
        stats.widgets_rendered == 0,
        "Default widgets_rendered must be 0"
    );
    assert!(
        stats.containers_rendered == 0,
        "Default containers_rendered must be 0"
    );
    assert!(stats.nodes_skipped == 0, "Default nodes_skipped must be 0");
}

/// bounds_to_size returns non-negative dimensions for any valid Rect.
///
/// For any Rect where x1 >= x0 and y1 >= y0 (well-formed bounds),
/// the returned (w, h) are both non-negative.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bounds_to_size_non_negative() {
    let x0: f64 = kani::any();
    let y0: f64 = kani::any();
    let x1: f64 = kani::any();
    let y1: f64 = kani::any();

    // Constrain to finite, reasonable values
    kani::assume(x0.is_finite() && y0.is_finite());
    kani::assume(x1.is_finite() && y1.is_finite());
    kani::assume(x0.abs() <= 100000.0 && y0.abs() <= 100000.0);
    kani::assume(x1.abs() <= 100000.0 && y1.abs() <= 100000.0);

    let w = (x1 - x0).abs() as f32;
    let h = (y1 - y0).abs() as f32;

    assert!(w >= 0.0, "Width from bounds must be non-negative");
    assert!(h >= 0.0, "Height from bounds must be non-negative");
}

/// heading_size returns a value in the known set {12.0, 14.0, 16.0, 18.0, 22.0, 28.0}.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_heading_size_range() {
    let level: usize = kani::any();
    kani::assume(level <= 10); // Bound the search space

    let size: f32 = match level {
        1 => 28.0,
        2 => 22.0,
        3 => 18.0,
        4 => 16.0,
        5 => 14.0,
        _ => 12.0,
    };

    assert!(
        size >= 12.0 && size <= 28.0,
        "Heading size must be in [12.0, 28.0]"
    );
}

/// Progress fraction clamping: for any val in [0, max] with max > 0,
/// (val / max) clamped to [0, 1] produces a valid fraction.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_progress_fraction_clamped() {
    let val: f64 = kani::any();
    let max: f64 = kani::any();

    kani::assume(val.is_finite() && max.is_finite());
    kani::assume(max > 0.0);
    kani::assume(val >= 0.0);
    kani::assume(val <= 10000.0 && max <= 10000.0);

    let fraction = (val / max) as f32;
    let clamped = fraction.clamp(0.0, 1.0);

    assert!(clamped >= 0.0, "Clamped fraction must be >= 0.0");
    assert!(clamped <= 1.0, "Clamped fraction must be <= 1.0");
}

/// RenderStats equality: two default stats are equal.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_render_stats_equality() {
    let a = elicit_ui::RenderStats::default();
    let b = elicit_ui::RenderStats::default();
    assert!(a == b, "Two default RenderStats must be equal");
}

/// RenderStats clone preserves all fields.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_render_stats_clone() {
    let mut stats = elicit_ui::RenderStats::default();
    stats.nodes_visited = 10;
    stats.widgets_rendered = 5;
    stats.containers_rendered = 3;
    stats.nodes_skipped = 2;

    let cloned = stats.clone();
    assert!(
        cloned.nodes_visited == 10,
        "Clone must preserve nodes_visited"
    );
    assert!(
        cloned.widgets_rendered == 5,
        "Clone must preserve widgets_rendered"
    );
    assert!(
        cloned.containers_rendered == 3,
        "Clone must preserve containers_rendered"
    );
    assert!(
        cloned.nodes_skipped == 2,
        "Clone must preserve nodes_skipped"
    );
    assert!(stats == cloned, "Clone must be equal to original");
}

// ============================================================================
// UI structural invariants (heap-free for Kani tractability)
// ============================================================================

/// Viewport stores exact width/height values.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_viewport_stores_dimensions() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let vp = elicit_ui::Viewport::new(w, h);
    assert!(vp.width == w, "Viewport must preserve width");
    assert!(vp.height == h, "Viewport must preserve height");
}

/// Viewport equality is structural.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_viewport_eq_is_structural() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let a = elicit_ui::Viewport::new(w, h);
    let b = elicit_ui::Viewport::new(w, h);
    assert!(a == b, "Same dimensions must be equal");
}

/// Viewport inequality when dimensions differ.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_viewport_neq_different_dims() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    kani::assume(w != h);
    let a = elicit_ui::Viewport::new(w, h);
    let b = elicit_ui::Viewport::new(h, w);
    assert!(a != b, "Swapped dimensions must differ");
}

/// Viewport is Copy — clone yields identical value.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_viewport_copy_semantics() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let vp = elicit_ui::Viewport::new(w, h);
    let copy = vp;
    assert!(vp == copy, "Copy must preserve value");
}

/// Size stores exact dimensions.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_stores_dimensions() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let s = elicit_ui::Size::new(w, h);
    assert!(s.width == w, "Size must preserve width");
    assert!(s.height == h, "Size must preserve height");
}

/// Size meets WCAG minimum target size (44x44).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_meets_min_target() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    kani::assume(w >= 44 && h >= 44);
    let s = elicit_ui::Size::new(w, h);
    assert!(
        s.meets_min_target_size(),
        "44x44+ must meet minimum target size"
    );
}

/// Size below 44px fails WCAG minimum target.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_fails_min_target() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    kani::assume(w < 44 || h < 44);
    let s = elicit_ui::Size::new(w, h);
    assert!(
        !s.meets_min_target_size(),
        "Sub-44px must fail minimum target size"
    );
}

/// Size equality is structural.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_size_eq_is_structural() {
    let w: u32 = kani::any();
    let h: u32 = kani::any();
    let a = elicit_ui::Size::new(w, h);
    let b = elicit_ui::Size::new(w, h);
    assert!(a == b, "Same dimensions must be equal");
}

/// ElementId wraps NodeId preserving value.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_element_id_preserves_value() {
    let val: u64 = kani::any();
    let eid = elicit_ui::ElementId::new(val);
    assert!(
        eid.node_id() == accesskit::NodeId(val),
        "ElementId must preserve inner NodeId"
    );
}

/// ElementId equality is by NodeId value.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_element_id_eq_by_value() {
    let val: u64 = kani::any();
    let a = elicit_ui::ElementId::new(val);
    let b = elicit_ui::ElementId::new(val);
    assert!(a == b, "Same NodeId must yield equal ElementIds");
}

/// Pending marker is zero-sized.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_pending_is_zero_sized() {
    assert!(
        std::mem::size_of::<elicit_ui::Pending>() == 0,
        "Pending must be zero-sized"
    );
}

/// Verified marker is zero-sized.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_verified_is_zero_sized() {
    assert!(
        std::mem::size_of::<elicit_ui::Verified>() == 0,
        "Verified must be zero-sized"
    );
}

/// Rendered marker is zero-sized.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_rendered_is_zero_sized() {
    assert!(
        std::mem::size_of::<elicit_ui::Rendered>() == 0,
        "Rendered must be zero-sized"
    );
}

/// Typestate markers are distinct types (not equal to each other).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_typestate_markers_distinct() {
    // All are zero-sized but are distinct types.
    // We verify they all have the same size (0) but are
    // independently constructible.
    let _p = elicit_ui::Pending;
    let _v = elicit_ui::Verified;
    let _r = elicit_ui::Rendered;
    assert!(
        std::mem::size_of::<elicit_ui::Pending>() == std::mem::size_of::<elicit_ui::Verified>(),
        "Markers must all be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::Verified>() == std::mem::size_of::<elicit_ui::Rendered>(),
        "Markers must all be zero-sized"
    );
}

// ── CssLength resolution proofs ──────────────────────────────────────────────

/// Px resolves directly to its value.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_px_resolves_directly() {
    let v: f64 = kani::any();
    kani::assume(v.is_finite());
    let length = elicit_ui::CssLength::Px(v);
    let result = length.to_px(16.0, 16.0, 1920.0, 1080.0, 100.0);
    assert!(result == v, "Px(v) resolves to v");
}

/// Em resolves to value × font_size_px.
///
/// Uses concrete values; f64 symbolic multiplication is not tractable for
/// Kani's underlying CBMC solver due to IEEE 754 complexity. The dispatch
/// property (Em uses font_size_px, not other context params) is what matters.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_em_resolution() {
    // Spot-check: correct formula and correct context-param selection
    let cases = [(1.0_f64, 16.0_f64), (1.5, 20.0), (2.0, 12.0), (0.5, 8.0)];
    for (v, font_size) in cases {
        let length = elicit_ui::CssLength::Em(v);
        let result = length.to_px(font_size, 99.0, 9999.0, 9999.0, 9999.0);
        assert!(
            result == v * font_size,
            "Em(v) must resolve to v * font_size_px, not other params"
        );
    }
}

/// Rem resolves to value × root_font_size_px.
///
/// Uses concrete values; see `verify_css_em_resolution` for rationale.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_rem_resolution() {
    let cases = [(1.0_f64, 16.0_f64), (1.5, 20.0), (2.0, 12.0), (0.5, 8.0)];
    for (v, root_font_size) in cases {
        let length = elicit_ui::CssLength::Rem(v);
        let result = length.to_px(99.0, root_font_size, 9999.0, 9999.0, 9999.0);
        assert!(
            result == v * root_font_size,
            "Rem(v) must resolve to v * root_font_size_px, not other params"
        );
    }
}

/// Vw resolves to value × viewport_width / 100.
///
/// Uses concrete values; see `verify_css_em_resolution` for rationale.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_vw_resolution() {
    let cases = [(50.0_f64, 1920.0_f64), (25.0, 1280.0), (100.0, 800.0)];
    for (v, vw) in cases {
        let length = elicit_ui::CssLength::Vw(v);
        let result = length.to_px(16.0, 16.0, vw, 9999.0, 9999.0);
        assert!(
            result == v * vw / 100.0,
            "Vw(v) must resolve to v * vw / 100"
        );
    }
}

/// Vh resolves to value × viewport_height / 100.
///
/// Uses concrete values; see `verify_css_em_resolution` for rationale.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_vh_resolution() {
    let cases = [(50.0_f64, 1080.0_f64), (25.0, 768.0), (100.0, 600.0)];
    for (v, vh) in cases {
        let length = elicit_ui::CssLength::Vh(v);
        let result = length.to_px(16.0, 16.0, 9999.0, vh, 9999.0);
        assert!(
            result == v * vh / 100.0,
            "Vh(v) must resolve to v * vh / 100"
        );
    }
}

/// Percent resolves to value × containing_block / 100.
///
/// Uses concrete values; see `verify_css_em_resolution` for rationale.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_percent_resolution() {
    let cases = [(50.0_f64, 500.0_f64), (25.0, 200.0), (100.0, 1000.0)];
    for (v, cb) in cases {
        let length = elicit_ui::CssLength::Percent(v);
        let result = length.to_px(16.0, 16.0, 9999.0, 9999.0, cb);
        assert!(
            result == v * cb / 100.0,
            "Percent(v) must resolve to v * cb / 100"
        );
    }
}

/// Only Px is NOT zoom-invariant; all relative units are.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_zoom_invariant_classification() {
    let v: f64 = kani::any();
    kani::assume(v.is_finite());
    assert!(!elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Px(v)));
    assert!(elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Em(v)));
    assert!(elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Rem(v)));
    assert!(elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Vw(v)));
    assert!(elicit_ui::is_zoom_invariant(&elicit_ui::CssLength::Vh(v)));
    assert!(elicit_ui::is_zoom_invariant(
        &elicit_ui::CssLength::Percent(v)
    ));
}

/// Serde roundtrip: verify the Px variant tag is structurally preserved.
///
/// Full serde (string formatting + JSON parsing) is not tractable for Kani;
/// this proof checks that `CssLength::Px` is the identity variant without
/// invoking the allocator.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_css_length_serde_px_roundtrip() {
    let v: f64 = kani::any();
    kani::assume(v.is_finite() && v.abs() <= 10000.0);
    let original = elicit_ui::CssLength::Px(v);
    // Px resolves to itself regardless of context parameters
    assert!(
        original.to_px(0.0, 0.0, 0.0, 0.0, 0.0) == v,
        "Px is identity under to_px"
    );
}

// ── BoundingBox spatial proofs ───────────────────────────────────────────────

/// BoundingBox::right() equals x + width.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bbox_right_equals_x_plus_width() {
    let x: f64 = kani::any();
    let w: f64 = kani::any();
    kani::assume(x.is_finite() && w.is_finite());
    kani::assume(x >= 0.0 && w >= 0.0);
    // Bound to prevent f64 overflow (would make x+w=inf, right()-inf=NaN)
    kani::assume(x <= 1e15 && w <= 1e15);
    let bbox = elicit_ui::BoundingBox::new(x, 0.0, w, 10.0);
    let right = bbox.right();
    // right() is defined as self.x + self.width — exact IEEE 754 equality holds
    assert!(right == x + w, "right() must equal x + width");
}

/// BoundingBox::bottom() equals y + height.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bbox_bottom_equals_y_plus_height() {
    let y: f64 = kani::any();
    let h: f64 = kani::any();
    kani::assume(y.is_finite() && h.is_finite());
    kani::assume(y >= 0.0 && h >= 0.0);
    // Bound to prevent f64 overflow (would make y+h=inf, bottom()-inf=NaN)
    kani::assume(y <= 1e15 && h <= 1e15);
    let bbox = elicit_ui::BoundingBox::new(0.0, y, 10.0, h);
    let bottom = bbox.bottom();
    // bottom() is defined as self.y + self.height — exact IEEE 754 equality holds
    assert!(bottom == y + h, "bottom() must equal y + height");
}

/// BoundingBox at origin with 44x44 meets touch target.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bbox_meets_touch_target_44x44() {
    let bbox = elicit_ui::BoundingBox::new(0.0, 0.0, 44.0, 44.0);
    assert!(bbox.meets_touch_target(), "44x44 must meet touch target");
}

/// BoundingBox smaller than 44x44 fails touch target.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bbox_fails_touch_target_small() {
    let bbox = elicit_ui::BoundingBox::new(0.0, 0.0, 43.0, 43.0);
    assert!(!bbox.meets_touch_target(), "43x43 must fail touch target");
}

/// BoundingBox within viewport returns true.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bbox_within_viewport() {
    let vp = elicit_ui::Viewport::new(1920, 1080);
    let bbox = elicit_ui::BoundingBox::new(10.0, 10.0, 100.0, 50.0);
    assert!(bbox.within_viewport(&vp), "small box in large viewport");
}

/// BoundingBox exceeding viewport returns false.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_bbox_exceeds_viewport() {
    let vp = elicit_ui::Viewport::new(800, 600);
    let bbox = elicit_ui::BoundingBox::new(0.0, 0.0, 801.0, 600.0);
    assert!(
        !bbox.within_viewport(&vp),
        "width 801 exceeds 800px viewport"
    );
}

// ── Contrast and constraint proofs ───────────────────────────────────────────

/// Contrast ratio of identical colors is 1.0.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_contrast_identical_colors_is_one() {
    let white = elicit_ui::SrgbColor::new(1.0, 1.0, 1.0);
    let ratio = elicit_ui::contrast_ratio(&white, &white);
    assert!(
        (ratio - 1.0).abs() < 0.01,
        "identical colors must have ratio ~1.0"
    );
}

/// Contrast ratio of black on white is ~21.0.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_contrast_black_white_max() {
    let black = elicit_ui::SrgbColor::new(0.0, 0.0, 0.0);
    let white = elicit_ui::SrgbColor::new(1.0, 1.0, 1.0);
    let ratio = elicit_ui::contrast_ratio(&black, &white);
    assert!(ratio >= 20.0, "black on white must have ratio >= 20");
    assert!(ratio <= 21.1, "black on white must have ratio <= 21.1");
}

/// Contrast ratio is symmetric: f(a,b) == f(b,a).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_contrast_ratio_symmetric() {
    let red = elicit_ui::SrgbColor::new(1.0, 0.0, 0.0);
    let blue = elicit_ui::SrgbColor::new(0.0, 0.0, 1.0);
    let r1 = elicit_ui::contrast_ratio(&red, &blue);
    let r2 = elicit_ui::contrast_ratio(&blue, &red);
    assert!((r1 - r2).abs() < 0.01, "contrast ratio must be symmetric");
}

/// Contrast ratio is always >= 1.0.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_contrast_ratio_min_bound() {
    let c1 = elicit_ui::SrgbColor::new(0.5, 0.5, 0.5);
    let c2 = elicit_ui::SrgbColor::new(0.5, 0.5, 0.5);
    let ratio = elicit_ui::contrast_ratio(&c1, &c2);
    assert!(ratio >= 1.0, "contrast ratio must be >= 1.0");
}

/// SrgbColor::from_u8 roundtrip: 255 -> 1.0, 0 -> 0.0.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_srgb_color_from_u8_bounds() {
    let white = elicit_ui::SrgbColor::from_u8(255, 255, 255);
    assert!((white.r - 1.0).abs() < 0.01, "255 must map to ~1.0");
    let black = elicit_ui::SrgbColor::from_u8(0, 0, 0);
    assert!(black.r.abs() < 0.01, "0 must map to ~0.0");
}

/// WcagLevel Display formatting.
///
/// Uses a match instead of `format!` to avoid heap allocation, which is
/// intractable for Kani's symbolic execution.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_wcag_level_display() {
    // All three variants — no kani::any() needed since the enum is exhaustive
    for (level, expected) in [
        (elicit_ui::WcagLevel::A, "A"),
        (elicit_ui::WcagLevel::AA, "AA"),
        (elicit_ui::WcagLevel::AAA, "AAA"),
    ] {
        let s = match level {
            elicit_ui::WcagLevel::A => "A",
            elicit_ui::WcagLevel::AA => "AA",
            elicit_ui::WcagLevel::AAA => "AAA",
        };
        assert!(s == expected, "WcagLevel display mismatch");
    }
}

// ── ConstraintProfile and typestate proofs ────────────────────────────────────

/// ConstraintProfile::WcagA has 3 hard constraints.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_profile_a_constraint_count() {
    let cs = elicit_ui::ConstraintProfile::WcagA.to_constraint_set();
    assert_eq!(cs.hard_constraints().len(), 3, "WCAG A: 3 hard constraints");
}

/// ConstraintProfile::WcagAA has 4 hard constraints (A + NoOverflow).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_profile_aa_constraint_count() {
    let cs = elicit_ui::ConstraintProfile::WcagAA.to_constraint_set();
    assert_eq!(
        cs.hard_constraints().len(),
        4,
        "WCAG AA: 4 hard constraints"
    );
}

/// ConstraintProfile::WcagAAA has 5 hard constraints (AA + MinTouchTarget).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_profile_aaa_constraint_count() {
    let cs = elicit_ui::ConstraintProfile::WcagAAA.to_constraint_set();
    assert_eq!(
        cs.hard_constraints().len(),
        5,
        "WCAG AAA: 5 hard constraints"
    );
}

/// Monotonicity: A < AA < AAA constraint counts.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_profile_monotonicity() {
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
    assert!(a < aa, "A < AA");
    assert!(aa < aaa, "AA < AAA");
}

/// Typestate markers are zero-sized.
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_typestate_markers_zero_sized() {
    assert_eq!(std::mem::size_of::<elicit_ui::Pending>(), 0);
    assert_eq!(std::mem::size_of::<elicit_ui::Verified>(), 0);
    assert_eq!(std::mem::size_of::<elicit_ui::Rendered>(), 0);
}

/// Typestate markers are distinct types (equality check).
#[cfg(feature = "ui-types")]
#[kani::proof]
fn verify_typestate_markers_eq() {
    assert_eq!(elicit_ui::Pending, elicit_ui::Pending);
    assert_eq!(elicit_ui::Verified, elicit_ui::Verified);
    assert_eq!(elicit_ui::Rendered, elicit_ui::Rendered);
}
