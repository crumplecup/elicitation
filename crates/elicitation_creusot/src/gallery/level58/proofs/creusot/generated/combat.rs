//! Generated combat companion for C58.

use crate::gallery::level58::{C58Consistent, C58State, C58Step, c58_begin};
use creusot_std::prelude::*;
use elicitation::Established;

#[logic]
pub fn c58_consistent_creusot_logic(_state: &C58State) -> bool {
    pearlite! { true }
}

extern_spec! {
    #[requires(c58_consistent_creusot_logic(&state))]
    #[ensures(c58_consistent_creusot_logic(&result.0))]
    fn c58_begin(
        state: C58State,
        proof: Established<C58Consistent>,
        _step: Established<C58Step>,
    ) -> (C58State, Established<C58Consistent>);
}

fn c58_begin_creusot_local(
    state: C58State,
    proof: Established<C58Consistent>,
    step: Established<C58Step>,
) -> (C58State, Established<C58Consistent>) {
    c58_begin_creusot(state, proof, step)
}

#[requires(c58_consistent_creusot_logic(&state))]
#[ensures(c58_consistent_creusot_logic(&result.0))]
pub fn c58_begin_creusot(
    state: C58State,
    proof: Established<C58Consistent>,
    _step: Established<C58Step>,
) -> (C58State, Established<C58Consistent>) {
    c58_begin(state, proof, _step)
}

#[requires(c58_consistent_creusot_logic(&state))]
#[ensures(c58_consistent_creusot_logic(&result.0))]
pub fn c58_begin_generated(
    state: C58State,
    proof: Established<C58Consistent>,
    step: Established<C58Step>,
) -> (C58State, Established<C58Consistent>) {
    c58_begin_creusot_local(state, proof, step)
}
