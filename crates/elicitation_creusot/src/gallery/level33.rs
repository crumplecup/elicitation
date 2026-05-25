//! Gallery level C33: tiny combat-like `VerifiedStateMachine` with `Vec` state.
//!
//! **Hypothesis**: the remaining `valinoreth` ICE needs more than `formal_method`
//! and invariant-name plumbing. It may require a real `VerifiedStateMachine`
//! whose transitions all share one contract over an enum carrying `Vec` fields.
//!
//! This level mirrors the combat shape in a minimal way:
//!
//! - enum lifecycle state
//! - `Vec`-backed active variant
//! - plain Rust source invariant function named by `creusot_invariant_fn`
//! - multiple `#[formal_method]` transitions sharing that contract
//! - one `#[derive(VerifiedStateMachine)]` machine registering those transitions
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c33-one-transition
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c33-three-transitions
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c33-vsm-combat
//! ```

use elicitation::{
    Elicit, Established, KaniVariantState, Prop, VerifiedStateMachine, formal_method,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Minimal combatant data.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub struct C33Combatant {
    /// Hit points.
    pub hp: i32,
}

/// Tiny combat lifecycle state.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniVariantState)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub enum C33State {
    /// Combat not started yet.
    #[default]
    Idle,
    /// Active combat with turn order and current actor.
    Active {
        /// Active combatants.
        combatants: Vec<C33Combatant>,
        /// Turn order indexing into `combatants`.
        turn_order: Vec<usize>,
        /// Current actor in `turn_order`.
        current_actor: usize,
        /// Current round number.
        round: u32,
    },
    /// Combat finished.
    Done,
}

/// Shared consistency proposition for the machine transitions.
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "c33_consistent",
    creusot_invariant_fn = "c33_consistent",
    creusot_inv_body = "true",
    verus_inv_body = "true"
)]
pub struct C33Consistent;

/// Step credential: combat was initialized.
#[derive(Prop)]
pub struct C33Initialized;

/// Step credential: a turn started.
#[derive(Prop)]
pub struct C33TurnBegan;

/// Step credential: damage was applied.
#[derive(Prop)]
pub struct C33DamageApplied;

/// Step credential: a no-op transition executed.
#[derive(Prop)]
pub struct C33NoopStep;

/// Source-module exec invariant, matching the `valinoreth` pattern.
pub fn c33_consistent(state: &C33State) -> bool {
    match state {
        C33State::Idle | C33State::Done => true,
        C33State::Active {
            combatants,
            turn_order,
            current_actor,
            round,
        } => {
            if combatants.is_empty() {
                return false;
            }
            if turn_order.is_empty() {
                return false;
            }
            if *current_actor >= turn_order.len() {
                return false;
            }
            if !turn_order.iter().all(|&idx| idx < combatants.len()) {
                return false;
            }
            *round > 0
        }
    }
}

/// Smallest VSM slice: one shared-contract transition over the combat state.
#[cfg(feature = "gallery-c33-one-transition")]
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [c33_noop])]
pub struct C33Machine;

/// Mid-size slice: multiple shared-contract transitions, but no Vec mutation.
#[cfg(feature = "gallery-c33-three-transitions")]
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [c33_initialize, c33_begin_turn, c33_noop])]
pub struct C33Machine;

/// Full tiny combat VSM including the Vec-mutating damage transition.
#[cfg(feature = "gallery-c33-vsm-combat")]
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [c33_initialize, c33_begin_turn, c33_apply_damage])]
pub struct C33Machine;

/// No-op transition preserving the current state.
#[cfg(any(
    feature = "gallery-c33-one-transition",
    feature = "gallery-c33-three-transitions"
))]
#[formal_method(contracts = [C33Consistent])]
pub fn c33_noop(
    state: C33State,
    proof: Established<C33Consistent>,
    _step: Established<C33NoopStep>,
) -> (C33State, Established<C33Consistent>) {
    (state, proof)
}

/// Initialize combat and establish the first turn.
#[cfg(any(
    feature = "gallery-c33-three-transitions",
    feature = "gallery-c33-vsm-combat"
))]
#[formal_method(contracts = [C33Consistent], kani_requires = [
    "!combatants.is_empty()",
    "combatants.iter().all(|c| c.hp > 0)",
])]
pub fn c33_initialize(
    _state: C33State,
    proof: Established<C33Consistent>,
    combatants: Vec<C33Combatant>,
    _init: Established<C33Initialized>,
) -> (C33State, Established<C33Consistent>) {
    let turn_order: Vec<usize> = (0..combatants.len()).collect();
    (
        C33State::Active {
            combatants,
            turn_order,
            current_actor: 0,
            round: 1,
        },
        proof,
    )
}

/// Begin the current actor's turn.
#[cfg(any(
    feature = "gallery-c33-three-transitions",
    feature = "gallery-c33-vsm-combat"
))]
#[formal_method(contracts = [C33Consistent])]
pub fn c33_begin_turn(
    state: C33State,
    proof: Established<C33Consistent>,
    _step: Established<C33TurnBegan>,
) -> (C33State, Established<C33Consistent>) {
    (state, proof)
}

/// Apply damage to a target selected through `turn_order`.
#[cfg(feature = "gallery-c33-vsm-combat")]
#[formal_method(contracts = [C33Consistent])]
pub fn c33_apply_damage(
    state: C33State,
    proof: Established<C33Consistent>,
    target_id: usize,
    damage: i32,
    _step: Established<C33DamageApplied>,
) -> (C33State, Established<C33Consistent>) {
    match state {
        C33State::Active {
            mut combatants,
            turn_order,
            current_actor,
            round,
        } => {
            if let Some(&idx) = turn_order.get(target_id)
                && let Some(combatant) = combatants.get_mut(idx)
            {
                combatant.hp -= damage;
            }

            (
                C33State::Active {
                    combatants,
                    turn_order,
                    current_actor,
                    round,
                },
                proof,
            )
        }
        other => (other, proof),
    }
}
