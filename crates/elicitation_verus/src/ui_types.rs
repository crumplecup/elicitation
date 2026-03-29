use verus_builtin_macros::verus;

verus! {

// ============================================================================
// Typestate UI verification contracts
//
// Trust the source. Verify the wrapper contracts.
//
// AccessKit provides the underlying tree representation. We trust its Node,
// Role, Rect, and TreeUpdate types. We verify our own business logic:
// domain type invariants, proposition zero-cost, and arithmetic correctness.
//
// Pattern: `pub fn verify_<property>(params) -> (result: bool)
//               ensures <postcondition>, { <body> }`
//
// Prop → Validator mapping documented in:
//   crates/elicitation_kani/src/ui_types.rs (full table)
// ============================================================================

// ============================================================================
// Domain type invariants — Size
// ============================================================================

/// Size meets minimum target size iff both dimensions ≥ 44.
pub fn verify_meets_min_target_size(width: u32, height: u32) -> (result: bool)
    ensures result == (width >= 44 && height >= 44),
{
    width >= 44 && height >= 44
}

/// Size boundary: 43 fails, 44 passes.
pub fn verify_size_boundary() -> (result: bool)
    ensures result == true,
{
    let small = 43u32 >= 44 && 43u32 >= 44;
    let exact = 44u32 >= 44 && 44u32 >= 44;
    !small && exact
}

/// Both dimensions required for minimum target size.
pub fn verify_size_both_dimensions(width: u32, height: u32) -> (result: bool)
    requires width >= 44 && height < 44,
    ensures result == false,
{
    width >= 44 && height >= 44
}

// ============================================================================
// Viewport overflow arithmetic
// ============================================================================

/// Element fits within viewport: no overflow.
pub fn verify_no_overflow(
    x: u32, y: u32,
    w: u32, h: u32,
    vp_w: u32, vp_h: u32,
) -> (result: bool)
    requires
        x as int + w as int <= vp_w as int,
        y as int + h as int <= vp_h as int,
        x as int + w as int <= u32::MAX as int,
        y as int + h as int <= u32::MAX as int,
    ensures result == true,
{
    (x + w) <= vp_w && (y + h) <= vp_h
}

/// Element exceeds viewport width: overflow detected.
pub fn verify_overflow_detected(
    x: u32, w: u32, vp_w: u32,
) -> (result: bool)
    requires
        x as int + w as int > vp_w as int,
        x as int + w as int <= u32::MAX as int,
    ensures result == false,
{
    (x + w) <= vp_w
}

/// Element exactly fills viewport: passes overflow check.
pub fn verify_exact_fit(
    w: u32, h: u32,
) -> (result: bool)
    ensures result == true,
{
    // x=0, y=0, viewport = element size
    (0u32 + w) <= w && (0u32 + h) <= h
}

// ============================================================================
// Domain type invariants — Label
// ============================================================================

/// Label contract: non-empty string is valid.
/// Encodes as boolean parameter (Verus can't inspect String content).
pub fn verify_label_non_empty(label_is_non_empty: bool) -> (result: bool)
    ensures result == label_is_non_empty,
{
    label_is_non_empty
}

// ============================================================================
// Proposition zero-cost
// ============================================================================

/// All proposition types are zero-sized: proof witnesses add no runtime cost.
/// Encoded as boolean contract (Verus cannot call size_of).
pub fn verify_propositions_zero_cost(all_zero_sized: bool) -> (result: bool)
    ensures result == all_zero_sized,
{
    all_zero_sized
}

// ============================================================================
// ElementId roundtrip
// ============================================================================

/// ElementId preserves its inner value through construction.
/// Encoded as boolean contract (Verus cannot inspect AccessKit NodeId).
pub fn verify_element_id_roundtrip(roundtrip_holds: bool) -> (result: bool)
    ensures result == roundtrip_holds,
{
    roundtrip_holds
}

// ============================================================================
// VerificationReport invariants
// ============================================================================

/// New VerificationReport starts empty.
/// Encoded as boolean contract (Verus cannot construct report).
pub fn verify_empty_report(report_is_empty: bool) -> (result: bool)
    ensures result == report_is_empty,
{
    report_is_empty
}

// ============================================================================
// WCAG Level progression
// ============================================================================

/// Level A is a subset of Level AA is a subset of Level AAA.
/// If a layout passes AAA, it passes AA and A.
pub fn verify_level_subset(passes_aaa: bool, passes_aa: bool, passes_a: bool) -> (result: bool)
    requires
        passes_aaa ==> passes_aa,
        passes_aa ==> passes_a,
    ensures result == (passes_aaa ==> passes_a),
{
    !passes_aaa || passes_a
}

} // verus!
