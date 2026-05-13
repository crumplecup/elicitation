//! V1 — Unit type baseline.
//!
//! Hypothesis: a unit struct with a trivially-true `pub open spec fn inv`
//! can be constructed and passed through an identity function.
//! Expected: ✓ proves — confirms basic Verus toolchain works.

use verus_builtin_macros::verus;

verus! {

/// Unit state: carries no data.
#[derive(Debug, Clone, Copy)]
pub struct GUnit;

/// Trivially-true invariant.
pub open spec fn v1_inv(_s: &GUnit) -> bool {
    true
}

/// Constructor: always produces a valid unit.
pub fn v1_new() -> (r: GUnit)
    ensures v1_inv(&r),
{
    GUnit
}

/// Identity: preserves the invariant.
pub fn v1_identity(s: GUnit) -> (r: GUnit)
    requires v1_inv(&s),
    ensures  v1_inv(&r),
{
    s
}

} // verus!
