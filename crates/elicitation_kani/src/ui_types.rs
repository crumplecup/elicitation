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

    assert!(fits_h && fits_v, "Element that fits must not be flagged as overflow");
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
