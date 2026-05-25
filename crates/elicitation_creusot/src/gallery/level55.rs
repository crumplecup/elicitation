//! Gallery level C55: generated wrapper over a re-exported function path.
//!
//! **Hypothesis**: the remaining downstream crash is not the generated wrapper
//! shape itself, but attaching Creusot specs to a function reached through a
//! `pub use` re-export rather than its defining module path.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c55-generated-reexport-path
//! ```

use creusot_std::prelude::*;
use elicitation::{Established, Prop, formal_method};

/// Minimal lifecycle state.
pub enum C55State {
    /// Initial state.
    Idle,
    /// Concluded state with optional victor.
    Done {
        /// Winning side or combatant.
        victor: Option<String>,
    },
}

/// Shared consistency proposition.
#[derive(Prop)]
#[prop(creusot_invariant_fn = "c55_consistent", creusot_inv_body = "true")]
pub struct C55Consistent;

/// Victory credential.
#[derive(Prop)]
pub struct C55Victory;

mod source {
    use super::*;

    /// Defining-module predicate.
    pub fn c55_consistent(state: &C55State) -> bool {
        match state {
            C55State::Idle => true,
            C55State::Done { victor: _ } => true,
        }
    }

    /// Defining-module source transition.
    #[formal_method(contracts = [C55Consistent])]
    pub fn c55_conclude(
        _state: C55State,
        proof: Established<C55Consistent>,
        victor: Option<String>,
        _victory: Established<C55Victory>,
    ) -> (C55State, Established<C55Consistent>) {
        (C55State::Done { victor }, proof)
    }
}

pub use source::{c55_conclude, c55_consistent};

/// Generated companion shape that targets the re-export path instead of the
/// defining module path.
pub mod generated {
    use super::*;

    #[logic]
    pub fn c55_consistent_creusot_logic(_state: &C55State) -> bool {
        pearlite! { true }
    }

    extern_spec! {
        #[requires(c55_consistent_creusot_logic(&state))]
        #[ensures(c55_consistent_creusot_logic(&result.0))]
        fn c55_conclude(state: C55State, proof: Established<C55Consistent>, victor: Option<String>, _victory: Established<C55Victory>) -> (C55State, Established<C55Consistent>);
    }

    #[requires(c55_consistent_creusot_logic(&state))]
    #[ensures(c55_consistent_creusot_logic(&result.0))]
    pub fn c55_conclude_creusot(
        state: C55State,
        proof: Established<C55Consistent>,
        victor: Option<String>,
        _victory: Established<C55Victory>,
    ) -> (C55State, Established<C55Consistent>) {
        c55_conclude(state, proof, victor, _victory)
    }
}
