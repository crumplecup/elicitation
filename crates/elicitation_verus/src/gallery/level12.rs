//! V12 — Composition `proof fn`: dispatch table over transition tags.
//!
//! Hypothesis: a single `proof fn` can case-split on a `V12Trans` tag and
//! delegate to the V11 leaf lemmas, proving `v11_inv` for any post-state
//! without re-verifying each transition body.
//!
//! This is the production shape for VSM companions: leaf lemmas proven once
//! (per-transition), composition is a mechanical dispatch table (automatic).
//! Adding a new transition means adding one leaf lemma and one dispatch arm —
//! the rest of the proof never changes.
//! Expected: ✓ proves.

use vstd::prelude::*;
use verus_builtin_macros::verus;

use crate::gallery::level11::{
    V11State, v11_inv, v11_is_begin, v11_is_advance,
    v11_leaf_begin, v11_leaf_advance, v11_leaf_trivial,
};

verus! {

#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Tags naming each transition.  The composition proof dispatches on this tag,
/// not on the transition implementation — the body is irrelevant once proven.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V12Trans {
    Begin   { count: u64 },
    Advance { prev_cursor: u64 },
    Select,
    Reset,
}

/// Maps each tag to the postcondition model of its transition.
/// This is the only spec knowledge the composition proof needs.
pub open spec fn v12_post(r: V11State, tag: V12Trans) -> bool {
    match tag {
        V12Trans::Begin   { count }       => v11_is_begin(r, count),
        V12Trans::Advance { prev_cursor } => v11_is_advance(r, prev_cursor),
        V12Trans::Select                  => r matches V11State::Selected { .. },
        V12Trans::Reset                   => r matches V11State::Idle,
    }
}

/// Composition: any post-state produced by a tagged transition satisfies v11_inv.
///
/// The proof body is a pure dispatch table — each branch calls the pre-proven
/// leaf lemma.  Z3 verifies the composition structurally; leaf bodies are NOT
/// re-verified.  This is the key scalability property: O(n) lemmas + O(1)
/// composition, not O(n²) combined proofs.
pub proof fn v12_composition(r: V11State, tag: V12Trans)
    requires v12_post(r, tag),
    ensures  v11_inv(r),
{
    match tag {
        V12Trans::Begin   { count }       => v11_leaf_begin(r, count),
        V12Trans::Advance { prev_cursor } => v11_leaf_advance(r, prev_cursor),
        V12Trans::Select                  => v11_leaf_trivial(r),
        V12Trans::Reset                   => v11_leaf_trivial(r),
    }
}

} // verus!
