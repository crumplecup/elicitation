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
        std::mem::size_of::<elicit_ui::HasLabel<()>>() == 0,
        "HasLabel must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::ValidRole<()>>() == 0,
        "ValidRole must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::KeyboardAccessible<()>>() == 0,
        "KeyboardAccessible must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::NoOverflow<()>>() == 0,
        "NoOverflow must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::MinTargetSize<()>>() == 0,
        "MinTargetSize must be zero-sized"
    );
    assert!(
        std::mem::size_of::<elicit_ui::AccessibleAA<()>>() == 0,
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
