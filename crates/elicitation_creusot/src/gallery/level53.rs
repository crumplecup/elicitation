//! Gallery level C53: generated Creusot wrapper with `Option<String>` argument.
//!
//! **Hypothesis**: the remaining downstream ICE is not the invariant split alone,
//! but the exact generated companion shape once a transition argument includes
//! `Option<String>`.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c53-generated-option-string
//! ```

use creusot_std::prelude::*;
use elicitation::{Established, Prop, formal_method};

/// Minimal lifecycle state.
pub enum C53State {
    /// Initial state.
    Idle,
    /// Concluded state with optional victor.
    Done {
        /// Winning combatant or side.
        victor: Option<String>,
    },
}

/// Shared consistency proposition.
#[derive(Prop)]
#[prop(creusot_invariant_fn = "c53_consistent", creusot_inv_body = "true")]
pub struct C53Consistent;

/// Victory credential.
#[derive(Prop)]
pub struct C53Victory;

/// Source-side exec predicate named by the proposition.
pub fn c53_consistent(state: &C53State) -> bool {
    match state {
        C53State::Idle => true,
        C53State::Done { victor: _ } => true,
    }
}

/// Source transition mirroring the downstream `conclude_combat` shape.
#[formal_method(contracts = [C53Consistent])]
pub fn c53_conclude(
    _state: C53State,
    proof: Established<C53Consistent>,
    victor: Option<String>,
    _victory: Established<C53Victory>,
) -> (C53State, Established<C53Consistent>) {
    (C53State::Done { victor }, proof)
}

/// Generated companion shape: distinct logic predicate plus `extern_spec!` and
/// a non-trusted call-through wrapper.
pub mod generated {
    use super::*;

    #[logic]
    pub fn c53_consistent_creusot_logic(_state: &C53State) -> bool {
        pearlite! { true }
    }

    extern_spec! {
        #[requires(c53_consistent_creusot_logic(&state))]
        #[ensures(c53_consistent_creusot_logic(&result.0))]
        fn c53_conclude(state: C53State, proof: Established<C53Consistent>, victor: Option<String>, _victory: Established<C53Victory>) -> (C53State, Established<C53Consistent>);
    }

    #[requires(c53_consistent_creusot_logic(&state))]
    #[ensures(c53_consistent_creusot_logic(&result.0))]
    pub fn c53_conclude_creusot(
        state: C53State,
        proof: Established<C53Consistent>,
        victor: Option<String>,
        _victory: Established<C53Victory>,
    ) -> (C53State, Established<C53Consistent>) {
        c53_conclude(state, proof, victor, _victory)
    }
}
