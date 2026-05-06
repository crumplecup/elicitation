//! V9 — `assume_specification` for an external (unverified) function.
//!
//! Hypothesis: `assume_specification[path]` can inject a postcondition
//! contract for a function whose body Verus does not verify, and callers
//! can reason from that contract as a trusted axiom.
//!
//! For production companions this is the mechanism for specifying async
//! transition functions that cannot be called from Verus directly.
//! Expected: ✓ proves.

use verus_builtin_macros::verus;

/// An "external" normalizer whose body Verus does not verify.
/// In production this would be a function from another crate (e.g., an
/// async transition we cannot call directly).
#[verifier::external]
fn v9_normalize(x: u64) -> u64 {
    x.max(1) // always ≥ 1, but Verus doesn't see this
}

verus! {

#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

// Inject a trusted spec for the external function.
// This is the pattern for production async transitions.
pub assume_specification[v9_normalize](x: u64) -> (result: u64)
    ensures result > 0;

/// Caller can now prove `r > 0` even though the body is unverified.
pub fn v9_use_normalize(x: u64) -> (r: u64)
    ensures r > 0,
{
    v9_normalize(x)
}

/// Chain two calls — each governed by the assumed spec.
pub fn v9_chain(a: u64, b: u64) -> (r: u64)
    ensures r > 0,
{
    let _first  = v9_normalize(a);
    let second = v9_normalize(b);
    second
}

} // verus!
