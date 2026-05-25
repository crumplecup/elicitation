//! V14 — State enum derive rules to avoid Verus internal panic.
//!
//! # The bug
//!
//! Verus panics at `vir/src/context.rs` with
//! `assertion failed: !trait_impl_map.contains_key(&trait_impl.x.impl_path)`
//! in two scenarios involving `Copy` or `Clone` inside `verus! {}`:
//!
//! 1. **Two `Copy` enums** in the same `verus! {}` block — two `PartialEq`/`Copy`
//!    impls collide in Verus's trait-impl map.
//! 2. **`Clone` without `Copy` on a unit-only enum** + any `Copy` enum — Verus
//!    tries to spec the Clone impl, fails internally for unit variants.
//!
//! The second scenario is the one triggered by the generator for VSMs with
//! trivial invariants (no `verus_state_body`), where the state enum is
//! `_Unspecified` (a single unit variant).
//!
//! # The rules
//!
//! - **State enum with struct-variant fields**: `#[derive(Debug, Clone, PartialEq, Eq)]`
//!   — `Clone` without `Copy` is safe; Verus warns but continues (see gallery L11–L13).
//! - **State enum with only unit variants** (`_Unspecified` / no `verus_state_body`):
//!   `#[derive(Debug, PartialEq, Eq)]` — no `Clone`, no `Copy`.
//! - **Trans/tag enum**: always `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`.
//!
//! These rules are enforced in `verus_gen.rs`.
//!
//! # Worked example
//!
//! A two-transition VSM with trivial invariant: no `verus_state_body`, so the
//! state enum is unit-only.  It verifies only because `V14State` omits `Clone`.
//!
//! Expected: ✓ proves.

use verus_builtin_macros::verus;

#[verifier::external]
pub fn v14_start_stub(state: V14State) -> V14State {
    todo!()
}

#[verifier::external]
pub fn v14_stop_stub(state: V14State) -> V14State {
    todo!()
}

verus! {

// State enum: no `Clone`, no `Copy` — unit-only enum inside verus! {} block.
// Deriving `Clone` (without `Copy`) on a unit-only enum combined with a `Copy`
// Trans enum triggers a Verus internal panic.  Omitting `Clone` entirely is safe
// since the proof functions take `V14State` by value (it's Copy-sized anyway).
#[derive(Debug, PartialEq, Eq)]
pub enum V14State {
    Idle,
    Running,
}

pub open spec fn v14_inv(s: V14State) -> bool { true }

pub open spec fn v14_post_trivial(post: V14State) -> bool { true }

// ─── Assume specifications ─────────────────────────────────────────────────

pub assume_specification[v14_start_stub](state: V14State) -> (r: V14State)
    requires v14_inv(state),
    ensures  v14_post_trivial(r);

pub assume_specification[v14_stop_stub](state: V14State) -> (r: V14State)
    requires v14_inv(state),
    ensures  v14_post_trivial(r);

// ─── Leaf + composition ────────────────────────────────────────────────────

pub proof fn v14_leaf_trivial(post: V14State)
    requires v14_post_trivial(post),
    ensures  v14_inv(post),
{}

// Trans enum: `Copy` is correct here — it carries no heap data.
// Only one Copy enum in the verus! block → no Verus panic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V14Trans {
    Start,
    Stop,
}

pub open spec fn v14_post(post: V14State, tag: V14Trans) -> bool {
    match tag {
        V14Trans::Start => v14_post_trivial(post),
        V14Trans::Stop  => v14_post_trivial(post),
    }
}

pub proof fn v14_composition(post: V14State, tag: V14Trans)
    requires v14_post(post, tag),
    ensures  v14_inv(post),
{
    match tag {
        V14Trans::Start => v14_leaf_trivial(post),
        V14Trans::Stop  => v14_leaf_trivial(post),
    }
}

pub fn v14_start_verified(state: V14State) -> (r: V14State)
    requires v14_inv(state),
    ensures  v14_inv(r),
{
    v14_start_stub(state)
}

pub fn v14_stop_verified(state: V14State) -> (r: V14State)
    requires v14_inv(state),
    ensures  v14_inv(r),
{
    v14_stop_stub(state)
}

} // verus!
