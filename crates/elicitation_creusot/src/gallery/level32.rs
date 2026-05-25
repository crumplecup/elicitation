//! Gallery level C32: `valinoreth`-style invariant split vs generated wrapper.
//!
//! **Hypothesis**: the remaining downstream ICE is not `#[formal_method]` alone,
//! but the combination of:
//!
//! 1. a proposition whose `creusot_invariant_fn` names a source-module predicate,
//! 2. a plain Rust predicate in the source module,
//! 3. a generated Creusot module that defines a separate `#[logic]` predicate of
//!    the same basename and a `#[requires]`/`#[ensures]` wrapper using it.
//!
//! That shape matches `valinoreth` more closely than C31. This level is split
//! into three feature-gated cases so we can see which ingredient is required.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c32-source-only
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c32-generated-only
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c32-name-collision
//! ```

#[cfg(any(
    feature = "gallery-c32-source-only",
    feature = "gallery-c32-generated-only",
    feature = "gallery-c32-name-collision"
))]
use elicitation::Established;
use elicitation::Prop;
#[cfg(any(
    feature = "gallery-c32-source-only",
    feature = "gallery-c32-generated-only",
    feature = "gallery-c32-name-collision"
))]
use elicitation::formal_method;

/// Minimal lifecycle state mirroring `CombatState`.
pub enum C32State {
    /// Uninitialized lifecycle state.
    Idle,
    /// Active state with a round counter.
    Active {
        /// Round number must stay positive.
        round: u32,
    },
}

/// Contract proposition for the C32 hypothesis.
#[derive(Prop)]
#[prop(creusot_invariant_fn = "c32_consistent", creusot_inv_body = "true")]
pub struct C32Consistent;

/// Dummy step credential.
#[derive(Prop)]
pub struct C32Step;

/// **C32a/C32c**: source-module exec predicate using the same basename as the
/// generated logic predicate in the collision case.
#[cfg(any(
    feature = "gallery-c32-source-only",
    feature = "gallery-c32-name-collision"
))]
pub fn c32_consistent(state: &C32State) -> bool {
    match state {
        C32State::Idle => true,
        C32State::Active { round } => *round > 0,
    }
}

/// **C32a**: `formal_method` plus a source-module exec predicate only.
#[cfg(feature = "gallery-c32-source-only")]
#[formal_method(contracts = [C32Consistent])]
pub fn c32_source_only(
    state: C32State,
    proof: Established<C32Consistent>,
    _step: Established<C32Step>,
) -> (C32State, Established<C32Consistent>) {
    (state, proof)
}

/// **C32b/C32c**: generated-module logic predicate and wrapper, matching the
/// generated Creusot companion shape used downstream.
#[cfg(any(
    feature = "gallery-c32-generated-only",
    feature = "gallery-c32-name-collision"
))]
pub mod generated {
    use super::*;

    #[logic]
    pub fn c32_consistent(_state: &C32State) -> bool {
        pearlite! { true }
    }

    #[requires(c32_consistent(&state))]
    #[ensures(c32_consistent(&result.0))]
    pub(crate) fn c32_generated_wrapper(
        state: C32State,
        proof: Established<C32Consistent>,
        _step: Established<C32Step>,
    ) -> (C32State, Established<C32Consistent>) {
        (state, proof)
    }
}

/// **C32b**: `formal_method` plus generated wrapper, but no source predicate of
/// the same basename.
#[cfg(feature = "gallery-c32-generated-only")]
#[formal_method(contracts = [C32Consistent])]
pub fn c32_generated_only(
    state: C32State,
    proof: Established<C32Consistent>,
    step: Established<C32Step>,
) -> (C32State, Established<C32Consistent>) {
    generated::c32_generated_wrapper(state, proof, step)
}

/// **C32c**: full `valinoreth`-style shape — source exec predicate plus
/// generated logic predicate/wrapper of the same basename.
#[cfg(feature = "gallery-c32-name-collision")]
#[formal_method(contracts = [C32Consistent])]
pub fn c32_name_collision(
    state: C32State,
    proof: Established<C32Consistent>,
    step: Established<C32Step>,
) -> (C32State, Established<C32Consistent>) {
    generated::c32_generated_wrapper(state, proof, step)
}
