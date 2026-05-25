//! Gallery level C34: simplest real `VerifiedStateMachine`.
//!
//! **Hypothesis**: if C33's one-transition variant still ICEs, the crash may no
//! longer depend on `Vec`-backed state at all. This level keeps the same proof
//! derivation architecture but reduces the state to a tiny enum with no
//! collections and a single `#[formal_method]` transition.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c34-simple-vsm
//! ```

use elicitation::{
    Elicit, Established, KaniVariantState, Prop, VerifiedStateMachine, formal_method,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Smallest real VSM state with the full `ElicitComplete` surface.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniVariantState)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub enum C34State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Shared invariant for the tiny machine.
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "c34_consistent",
    creusot_invariant_fn = "c34_consistent",
    creusot_inv_body = "true",
    verus_inv_body = "true"
)]
pub struct C34Consistent;

/// Step credential for the only transition.
#[derive(Prop)]
pub struct C34Step;

/// Source invariant function referenced by `formal_method`.
pub fn c34_consistent(_state: &C34State) -> bool {
    true
}

/// Minimal faithful VSM repro: real derive, real state, one formal transition.
#[cfg(feature = "gallery-c34-simple-vsm")]
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [c34_finish])]
pub struct C34Machine;

/// Single transition preserving the invariant token.
#[cfg(feature = "gallery-c34-simple-vsm")]
#[formal_method(contracts = [C34Consistent])]
pub fn c34_finish(
    _state: C34State,
    proof: Established<C34Consistent>,
    _step: Established<C34Step>,
) -> (C34State, Established<C34Consistent>) {
    (C34State::Done, proof)
}
