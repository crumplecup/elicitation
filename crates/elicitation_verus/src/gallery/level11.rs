//! V11 — Leaf `proof fn` lemmas for independent invariant proofs.
//!
//! Hypothesis: each transition can be expressed as both an exec fn (verified
//! inline by Verus) and a standalone `proof fn` lemma that captures the same
//! invariant reasoning.  The lemmas become the reusable atoms of the V12
//! composition proof, decoupling verification from dispatch.
//!
//! Machine: a cursor positioned inside an abstract count of items.
//! Invariant: in `Filtered` state, `cursor < count` must always hold.
//! Expected: ✓ proves.

use verus_builtin_macros::verus;
use vstd::prelude::*;

verus! {

#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Cursor machine with a non-trivial index invariant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V11State {
    Idle,
    /// `cursor` is valid: must satisfy `cursor < count`.
    Filtered { count: u64, cursor: u64 },
    Selected { value: u64 },
}

/// The invariant every transition must preserve.
pub open spec fn v11_inv(s: V11State) -> bool {
    match s {
        V11State::Filtered { count, cursor } => cursor < count,
        _ => true,
    }
}

// ─── Postcondition predicates ────────────────────────────────────────────────
// Describe the shape of the post-state without referencing the exec body.
// The composition proof (V12) uses these instead of exec-fn postconditions.

/// Post-shape for `v11_begin`: Filtered at cursor 0 with given count.
/// Embeds `count > 0` so the composition proof need not re-state the precondition.
pub open spec fn v11_is_begin(r: V11State, count: u64) -> bool {
    match r {
        V11State::Filtered { count: c, cursor } => c == count && cursor == 0 && count > 0,
        _ => false,
    }
}

/// Post-shape for `v11_advance`: cursor incremented by one, still in bounds.
pub open spec fn v11_is_advance(r: V11State, prev_cursor: u64) -> bool {
    match r {
        V11State::Filtered { count, cursor } =>
            cursor == prev_cursor + 1 && cursor < count,
        _ => false,
    }
}

// ─── Exec transitions ────────────────────────────────────────────────────────
// Each exec fn proves its own `ensures v11_inv(r)` inline.

/// Transition 1: Idle → Filtered at cursor 0.
pub fn v11_begin(count: u64) -> (r: V11State)
    requires count > 0,
    ensures  v11_inv(r),
{
    V11State::Filtered { count, cursor: 0 }
}

/// Transition 2: advance cursor by one, provided another item exists.
pub fn v11_advance(state: V11State) -> (r: V11State)
    requires
        v11_inv(state),
        state matches V11State::Filtered { count, cursor } && cursor + 1 < count,
    ensures v11_inv(r),
{
    match state {
        V11State::Filtered { count, cursor } =>
            V11State::Filtered { count, cursor: cursor + 1 },
        other => other,
    }
}

/// Transition 3: select the item at the cursor (uses cursor as value proxy).
pub fn v11_select(state: V11State) -> (r: V11State)
    requires v11_inv(state),
    ensures  v11_inv(r),
{
    match state {
        V11State::Filtered { cursor, .. } => V11State::Selected { value: cursor },
        other => other,
    }
}

/// Transition 4: reset to Idle from any valid state.
pub fn v11_reset(_state: V11State) -> (r: V11State)
    ensures v11_inv(r),
{
    V11State::Idle
}

// ─── Leaf lemmas ─────────────────────────────────────────────────────────────
// Each `proof fn` captures invariant preservation for one post-state shape.
// They operate purely in spec space; no exec fn body is involved.
// V12 calls these without re-verifying the transition logic.

/// Leaf 1: Filtered at cursor 0 with count > 0 satisfies v11_inv.
pub proof fn v11_leaf_begin(r: V11State, count: u64)
    requires v11_is_begin(r, count),
    ensures v11_inv(r),
{}

/// Leaf 2: Filtered with cursor just advanced (still in bounds) satisfies v11_inv.
pub proof fn v11_leaf_advance(r: V11State, prev_cursor: u64)
    requires v11_is_advance(r, prev_cursor),
    ensures  v11_inv(r),
{}

/// Leaf 3: any non-Filtered state trivially satisfies v11_inv.
pub proof fn v11_leaf_trivial(r: V11State)
    requires !(r matches V11State::Filtered { .. }),
    ensures  v11_inv(r),
{}

} // verus!
