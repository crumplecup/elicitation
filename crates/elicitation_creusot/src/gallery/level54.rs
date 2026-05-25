//! Gallery level C54: combat-like generated module without tracing.
//!
//! **Hypothesis**: if the single generated wrapper shape is fine, the remaining
//! downstream crash may require the larger same-crate combat module shape:
//! several shared-contract transitions over a `Vec`-backed enum state plus a
//! generated companion module that attaches extensional Creusot specs.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c54-generated-combat-module
//! ```

use creusot_std::prelude::*;
use elicitation::{Established, Prop, formal_method};

/// Minimal combatant payload.
pub struct C54Combatant {
    /// Current hit points.
    pub hp: i32,
    /// Initiative sort key.
    pub basic_speed: i32,
}

/// Combat-like lifecycle state.
pub enum C54State {
    /// Combat not yet started.
    Idle,
    /// Active combat.
    Active {
        /// Active combatants.
        combatants: Vec<C54Combatant>,
        /// Turn-order index into `combatants`.
        turn_order: Vec<usize>,
        /// Current actor in the turn order.
        current_actor: usize,
        /// Current round number.
        round: u32,
    },
    /// Combat has concluded.
    Done {
        /// Optional victor description.
        victor: Option<String>,
    },
}

/// Shared consistency proposition.
#[derive(Prop)]
#[prop(creusot_invariant_fn = "c54_consistent", creusot_inv_body = "true")]
pub struct C54Consistent;

/// Initialization credential.
#[derive(Prop)]
pub struct C54Initialized;

/// Turn-start credential.
#[derive(Prop)]
pub struct C54TurnBegan;

/// Damage credential.
#[derive(Prop)]
pub struct C54DamageApplied;

/// Turn-end credential.
#[derive(Prop)]
pub struct C54TurnEnded;

/// Victory credential.
#[derive(Prop)]
pub struct C54Victory;

/// Source-side exec invariant.
pub fn c54_consistent(state: &C54State) -> bool {
    match state {
        C54State::Idle | C54State::Done { .. } => true,
        C54State::Active {
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

/// Initialize combat and establish turn order.
#[formal_method(contracts = [C54Consistent])]
pub fn c54_initialize(
    _state: C54State,
    proof: Established<C54Consistent>,
    combatants: Vec<C54Combatant>,
    _init: Established<C54Initialized>,
) -> (C54State, Established<C54Consistent>) {
    let mut turn_order: Vec<usize> = (0..combatants.len()).collect();
    turn_order.sort_by(|&a, &b| combatants[b].basic_speed.cmp(&combatants[a].basic_speed));
    (
        C54State::Active {
            combatants,
            turn_order,
            current_actor: 0,
            round: 1,
        },
        proof,
    )
}

/// Begin a turn.
#[formal_method(contracts = [C54Consistent])]
pub fn c54_begin_turn(
    state: C54State,
    proof: Established<C54Consistent>,
    _step: Established<C54TurnBegan>,
) -> (C54State, Established<C54Consistent>) {
    (state, proof)
}

/// Apply damage to the selected target.
#[formal_method(contracts = [C54Consistent])]
pub fn c54_apply_damage(
    state: C54State,
    proof: Established<C54Consistent>,
    target_id: usize,
    damage: i32,
    _step: Established<C54DamageApplied>,
) -> (C54State, Established<C54Consistent>) {
    match state {
        C54State::Active {
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
                C54State::Active {
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

/// Advance to the next turn.
#[formal_method(contracts = [C54Consistent])]
pub fn c54_end_turn(
    state: C54State,
    proof: Established<C54Consistent>,
    _step: Established<C54TurnEnded>,
) -> (C54State, Established<C54Consistent>) {
    match state {
        C54State::Active {
            combatants,
            turn_order,
            current_actor,
            round,
        } => {
            let next_actor = current_actor + 1;
            let (new_actor, new_round) = if next_actor >= turn_order.len() {
                (0, round + 1)
            } else {
                (next_actor, round)
            };

            (
                C54State::Active {
                    combatants,
                    turn_order,
                    current_actor: new_actor,
                    round: new_round,
                },
                proof,
            )
        }
        other => (other, proof),
    }
}

/// Conclude combat with an optional victor.
#[formal_method(contracts = [C54Consistent])]
pub fn c54_conclude(
    _state: C54State,
    proof: Established<C54Consistent>,
    victor: Option<String>,
    _victory: Established<C54Victory>,
) -> (C54State, Established<C54Consistent>) {
    (C54State::Done { victor }, proof)
}

/// Generated companion module matching the extensional downstream pattern.
pub mod generated {
    use super::*;

    #[logic]
    pub fn c54_consistent_creusot_logic(_state: &C54State) -> bool {
        pearlite! { true }
    }

    extern_spec! {
        #[requires(c54_consistent_creusot_logic(&state))]
        #[ensures(c54_consistent_creusot_logic(&result.0))]
        fn c54_initialize(state: C54State, proof: Established<C54Consistent>, combatants: Vec<C54Combatant>, _init: Established<C54Initialized>) -> (C54State, Established<C54Consistent>);
    }

    #[requires(c54_consistent_creusot_logic(&state))]
    #[ensures(c54_consistent_creusot_logic(&result.0))]
    pub fn c54_initialize_creusot(
        state: C54State,
        proof: Established<C54Consistent>,
        combatants: Vec<C54Combatant>,
        _init: Established<C54Initialized>,
    ) -> (C54State, Established<C54Consistent>) {
        c54_initialize(state, proof, combatants, _init)
    }

    extern_spec! {
        #[requires(c54_consistent_creusot_logic(&state))]
        #[ensures(c54_consistent_creusot_logic(&result.0))]
        fn c54_begin_turn(state: C54State, proof: Established<C54Consistent>, _step: Established<C54TurnBegan>) -> (C54State, Established<C54Consistent>);
    }

    #[requires(c54_consistent_creusot_logic(&state))]
    #[ensures(c54_consistent_creusot_logic(&result.0))]
    pub fn c54_begin_turn_creusot(
        state: C54State,
        proof: Established<C54Consistent>,
        _step: Established<C54TurnBegan>,
    ) -> (C54State, Established<C54Consistent>) {
        c54_begin_turn(state, proof, _step)
    }

    extern_spec! {
        #[requires(c54_consistent_creusot_logic(&state))]
        #[ensures(c54_consistent_creusot_logic(&result.0))]
        fn c54_apply_damage(state: C54State, proof: Established<C54Consistent>, target_id: usize, damage: i32, _step: Established<C54DamageApplied>) -> (C54State, Established<C54Consistent>);
    }

    #[requires(c54_consistent_creusot_logic(&state))]
    #[ensures(c54_consistent_creusot_logic(&result.0))]
    pub fn c54_apply_damage_creusot(
        state: C54State,
        proof: Established<C54Consistent>,
        target_id: usize,
        damage: i32,
        _step: Established<C54DamageApplied>,
    ) -> (C54State, Established<C54Consistent>) {
        c54_apply_damage(state, proof, target_id, damage, _step)
    }

    extern_spec! {
        #[requires(c54_consistent_creusot_logic(&state))]
        #[ensures(c54_consistent_creusot_logic(&result.0))]
        fn c54_end_turn(state: C54State, proof: Established<C54Consistent>, _step: Established<C54TurnEnded>) -> (C54State, Established<C54Consistent>);
    }

    #[requires(c54_consistent_creusot_logic(&state))]
    #[ensures(c54_consistent_creusot_logic(&result.0))]
    pub fn c54_end_turn_creusot(
        state: C54State,
        proof: Established<C54Consistent>,
        _step: Established<C54TurnEnded>,
    ) -> (C54State, Established<C54Consistent>) {
        c54_end_turn(state, proof, _step)
    }

    extern_spec! {
        #[requires(c54_consistent_creusot_logic(&state))]
        #[ensures(c54_consistent_creusot_logic(&result.0))]
        fn c54_conclude(state: C54State, proof: Established<C54Consistent>, victor: Option<String>, _victory: Established<C54Victory>) -> (C54State, Established<C54Consistent>);
    }

    #[requires(c54_consistent_creusot_logic(&state))]
    #[ensures(c54_consistent_creusot_logic(&result.0))]
    pub fn c54_conclude_creusot(
        state: C54State,
        proof: Established<C54Consistent>,
        victor: Option<String>,
        _victory: Established<C54Victory>,
    ) -> (C54State, Established<C54Consistent>) {
        c54_conclude(state, proof, victor, _victory)
    }
}
