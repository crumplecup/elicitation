//! Gallery level C31: isolate `formal_method` vs raw `#[instrument]` under Creusot.
//!
//! **Goal**: reproduce the current downstream `cargo creusot prove` crash from a
//! minimal, in-repo case so we can tell whether the trigger is:
//!
//! 1. raw `#[instrument]` on a free function,
//! 2. `#[formal_method]` by itself, or
//! 3. the combination `#[formal_method] + #[instrument]`.
//!
//! This level is **feature-gated on purpose** so each case can be compiled in
//! isolation:
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c31-raw-instrument
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c31-formal-only
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c31-formal-instrument
//! ```
//!
//! ## Case layout
//!
//! | Feature | What it enables | Expected signal |
//! |---------|------------------|-----------------|
//! | `gallery-c31-raw-instrument` | Bare `#[instrument]` free function | Tells us whether tracing alone is enough |
//! | `gallery-c31-formal-only` | `#[formal_method]` with no tracing | Tells us whether the macro itself is enough |
//! | `gallery-c31-formal-instrument` | `#[formal_method]` + `#[instrument]` | Tells us whether the interaction is the trigger |
//!
//! The function body is deliberately trivial: `(state, proof)`. That keeps the
//! repro focused on macro expansion shape rather than body complexity.

use creusot_std::prelude::*;
#[cfg(any(
    feature = "gallery-c31-raw-instrument",
    feature = "gallery-c31-formal-only",
    feature = "gallery-c31-formal-instrument"
))]
use elicitation::Established;
use elicitation::Prop;
#[cfg(any(
    feature = "gallery-c31-formal-only",
    feature = "gallery-c31-formal-instrument"
))]
use elicitation::formal_method;

/// Minimal state machine state for the C31 repro.
pub enum C31State {
    /// Idle state.
    Idle,
    /// Active state with a positive count.
    Active {
        /// Must stay positive.
        count: u32,
    },
}

/// Minimal invariant proposition used by the `formal_method` contract list.
#[derive(Prop)]
#[prop(
    creusot_invariant_fn = "c31_consistent",
    creusot_inv_body = "match state { C31State::Idle => true, C31State::Active { count } => count@ > 0 }"
)]
pub struct C31Consistent;

/// Dummy proof token for a transition credential.
#[derive(Prop)]
pub struct C31StepBegun;

/// Same-file invariant predicate.
#[logic]
pub fn c31_consistent(state: &C31State) -> bool {
    pearlite! {
        match state {
            C31State::Idle => true,
            C31State::Active { count } => count@ > 0,
        }
    }
}

/// **C31a**: bare `#[instrument]` with no `formal_method`.
#[cfg(feature = "gallery-c31-raw-instrument")]
#[tracing::instrument(skip_all)]
pub fn c31_raw_instrument(
    state: C31State,
    proof: Established<C31Consistent>,
    _step: Established<C31StepBegun>,
) -> (C31State, Established<C31Consistent>) {
    (state, proof)
}

/// **C31b**: `#[formal_method]` with no tracing.
#[cfg(feature = "gallery-c31-formal-only")]
#[formal_method(contracts = [C31Consistent])]
pub fn c31_formal_only(
    state: C31State,
    proof: Established<C31Consistent>,
    _step: Established<C31StepBegun>,
) -> (C31State, Established<C31Consistent>) {
    (state, proof)
}

/// **C31c**: `#[formal_method]` plus raw `#[instrument]`.
#[cfg(feature = "gallery-c31-formal-instrument")]
#[formal_method(contracts = [C31Consistent])]
#[tracing::instrument(skip_all)]
pub fn c31_formal_instrument(
    state: C31State,
    proof: Established<C31Consistent>,
    _step: Established<C31StepBegun>,
) -> (C31State, Established<C31Consistent>) {
    (state, proof)
}
