//! V13 — `assume_specification` in the multi-tool trust model.
//!
//! Hypothesis: when a transition is opaque to Verus (async fn in another
//! crate), `assume_specification` injects a trusted postcondition.  The
//! composition proof from V12 works identically — it only needs the
//! tag → postcondition model, not how the postcondition was established.
//!
//! The "multi-tool" guarantee: Kani independently verifies that the exec body
//! satisfies the contract; Verus trusts that contract and verifies composition.
//! Neither tool re-does the other's work.
//!
//! V13 mirrors V11/V12 but declares `v13_begin_ext` as `#[verifier::external]`
//! to model an async transition Verus cannot inspect.
//! Expected: ✓ proves.

use vstd::prelude::*;
use verus_builtin_macros::verus;

/// External transition — Verus does not see or verify the body.
/// In production this represents an async fn in `elicit_server` (or any crate
/// that cannot compile under vargo).  Kani independently verifies the contract.
#[verifier::external]
pub fn v13_begin_ext(count: u64) -> V13State {
    V13State::Filtered { count, cursor: 0 }
}

verus! {

#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Same shape as V11State — self-contained for gallery independence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V13State {
    Idle,
    Filtered { count: u64, cursor: u64 },
    Selected { value: u64 },
}

pub open spec fn v13_inv(s: V13State) -> bool {
    match s {
        V13State::Filtered { count, cursor } => cursor < count,
        _ => true,
    }
}

pub open spec fn v13_is_begin(r: V13State, count: u64) -> bool {
    match r {
        V13State::Filtered { count: c, cursor } => c == count && cursor == 0 && count > 0,
        _ => false,
    }
}

// ─── Trusted spec for the external function ──────────────────────────────────
// Verus does NOT verify that `v13_begin_ext` actually satisfies this contract.
// Soundness relies on an independent verification (e.g., `proof_for_contract`
// under Kani) having confirmed the body produces the stated postcondition.
//
// Compare with V11 `v11_begin`: there, Verus verified inline.  Here, the spec
// is declared axiomatically.  The composition proof is identical either way.
pub assume_specification[v13_begin_ext](count: u64) -> (r: V13State)
    requires count > 0,
    ensures  v13_is_begin(r, count);

// ─── Leaf lemmas (same pattern as V11) ───────────────────────────────────────

/// Leaf 1: Filtered at cursor 0 with count > 0 satisfies v13_inv.
pub proof fn v13_leaf_begin(r: V13State, count: u64)
    requires v13_is_begin(r, count),
    ensures v13_inv(r),
{}

/// Leaf 2: any non-Filtered state trivially satisfies v13_inv.
pub proof fn v13_leaf_trivial(r: V13State)
    requires !(r matches V13State::Filtered { .. }),
    ensures  v13_inv(r),
{}

// ─── Composition ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V13Trans {
    BeginExt { count: u64 },
    Reset,
}

pub open spec fn v13_post(r: V13State, tag: V13Trans) -> bool {
    match tag {
        V13Trans::BeginExt { count } => v13_is_begin(r, count),
        V13Trans::Reset              => r matches V13State::Idle,
    }
}

/// Composition: identical structure to V12 despite the external trust model.
/// The proof does not care whether the leaf was verified inline or trusted via
/// `assume_specification` — only the tag → postcondition model matters.
pub proof fn v13_composition(r: V13State, tag: V13Trans)
    requires v13_post(r, tag),
    ensures  v13_inv(r),
{
    match tag {
        V13Trans::BeginExt { count } => v13_leaf_begin(r, count),
        V13Trans::Reset              => v13_leaf_trivial(r),
    }
}

/// Exec caller: uses the trusted spec to prove its own postcondition.
/// The `open spec fn` bodies unfold enough for Z3 to close the proof directly.
pub fn v13_call_begin_ext(count: u64) -> (r: V13State)
    requires count > 0,
    ensures  v13_inv(r),
{
    v13_begin_ext(count)
}

} // verus!
