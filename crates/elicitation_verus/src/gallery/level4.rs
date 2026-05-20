//! V4 — `spec fn` body visibility tiers.
//!
//! Three sub-experiments:
//!
//! * **V4a** (`pub open spec fn`) — body transparent in every module; direct
//!   use of the definition works without a lemma.
//! * **V4b** (`pub closed spec fn`) — body opaque outside the declaring
//!   module; a caller that tries to reason about the body **cannot**.
//!   Demonstrated by relying only on the function's boolean return value
//!   (which is always valid) while leaving the body unknown.
//! * **V4c** — same closed fn, but a `pub proof fn` lemma exports the
//!   key fact so external callers can still prove things.
//!
//! Expected: all three sub-experiments verify (V4b verifies because the
//! external caller only uses what is provable from the opaque spec).

use verus_builtin_macros::verus;

// ---------------------------------------------------------------------------
// Inner module: declares both an open and a closed spec fn
// ---------------------------------------------------------------------------
pub mod inner {
    use verus_builtin_macros::verus;

    verus! {

    // Required for `>` in spec fn bodies — comparison operators use SpecOrd.
    #[allow(unused_imports)]
    use vstd::prelude::SpecOrd;

    /// Open: body is visible everywhere.
    pub open spec fn v4a_open_inv(x: u64) -> bool {
        x > 0
    }

    /// Closed: body is private to this module.
    pub closed spec fn v4b_closed_inv(x: u64) -> bool {
        x > 0
    }

    /// Lemma: exports the key implication for V4c callers.
    pub proof fn v4c_lemma_positive_implies_inv(x: u64)
        requires x > 0,
        ensures  v4b_closed_inv(x),
    {
        reveal(v4b_closed_inv);
    }

    } // verus!
}

// ---------------------------------------------------------------------------
// Outer module: exercises each visibility tier
// ---------------------------------------------------------------------------
verus! {

// Required for `>` in requires/ensures with u64 — comparison operators use SpecOrd.
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;
#[cfg(verus_keep_ghost)]
use self::inner::v4a_open_inv;
#[cfg(verus_keep_ghost)]
use self::inner::v4b_closed_inv;
#[cfg(verus_keep_ghost)]
use self::inner::v4c_lemma_positive_implies_inv;

// V4a — open spec fn: body transparent, direct proof works.
pub fn v4a_construct(x: u64) -> (r: u64)
    requires x > 0,
    ensures  v4a_open_inv(r),
{
    x
}

// V4b — closed spec fn: we can only assert the postcondition as a boolean.
// We cannot prove *why* it is true from the outside; we just forward the
// value unchanged and rely on the callee's contract.
pub fn v4b_forward(x: u64) -> (r: u64)
    requires v4b_closed_inv(x),
    ensures  v4b_closed_inv(r),
{
    x
}

// V4c — closed spec fn + lemma: external caller obtains the invariant via
// the published lemma, then passes it to a function that needs it.
pub fn v4c_with_lemma(x: u64) -> (r: u64)
    requires x > 0,
    ensures  v4b_closed_inv(r),
{
    proof { v4c_lemma_positive_implies_inv(x); }
    x
}

} // verus!
