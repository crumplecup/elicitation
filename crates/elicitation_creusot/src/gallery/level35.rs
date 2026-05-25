//! Gallery level C35: same formal surface, no `VerifiedStateMachine`.
//!
//! **Hypothesis**: if C34 ICEs because of how same-crate VSM proof packaging is
//! assembled, then removing only the `VerifiedStateMachine` derive should make
//! the crash disappear while keeping the tiny `ElicitComplete` state and
//! `#[formal_method]` transition surface intact.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c35-formal-no-vsm
//! ```

use elicitation::{Elicit, Established, KaniVariantState, Prop, formal_method};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Same tiny state shape as C34, but without a machine derive.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniVariantState)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub enum C35State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Shared invariant carried through the transition.
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "c35_consistent",
    creusot_invariant_fn = "c35_consistent",
    creusot_inv_body = "true",
    verus_inv_body = "true"
)]
pub struct C35Consistent;

/// Step credential for the only transition.
#[derive(Prop)]
pub struct C35Step;

/// Source invariant function referenced by `formal_method`.
pub fn c35_consistent(_state: &C35State) -> bool {
    true
}

/// Same transition surface as C34, but no surrounding machine derive.
#[cfg(feature = "gallery-c35-formal-no-vsm")]
#[formal_method(contracts = [C35Consistent])]
pub fn c35_finish(
    _state: C35State,
    proof: Established<C35Consistent>,
    _step: Established<C35Step>,
) -> (C35State, Established<C35Consistent>) {
    (C35State::Done, proof)
}
