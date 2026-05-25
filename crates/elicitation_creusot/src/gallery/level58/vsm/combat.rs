//! Source combat module for C58.

use elicitation::{Established, Prop, formal_method};

/// Minimal lifecycle state.
#[derive(Debug)]
pub enum C58State {
    /// Initial state.
    Idle,
    /// Active state with a positive round counter.
    Active {
        /// Round number.
        round: u32,
    },
}

/// Shared consistency proposition.
#[derive(Prop)]
#[prop(creusot_invariant_fn = "c58_consistent", creusot_inv_body = "true")]
pub struct C58Consistent;

/// Dummy step credential.
#[derive(Prop)]
pub struct C58Step;

/// Source-module exec predicate.
pub fn c58_consistent(state: &C58State) -> bool {
    match state {
        C58State::Idle => true,
        C58State::Active { round } => *round > 0,
    }
}

/// Source transition with downstream-like formal surface.
#[formal_method(contracts = [C58Consistent])]
#[tracing::instrument(skip_all)]
pub fn c58_begin(
    state: C58State,
    proof: Established<C58Consistent>,
    _step: Established<C58Step>,
) -> (C58State, Established<C58Consistent>) {
    (state, proof)
}
