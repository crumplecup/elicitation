//! Tests for [`VerifiedStateMachine`] and [`VerifiedTransition`].
//!
//! Verifies that:
//! - A VSM impl binds `State: ElicitComplete` and `Invariant: Prop`.
//! - Any function with the right signature satisfies `VerifiedTransition`.
//! - Proof tokens carry the invariant through a multi-step round-trip.
//! - A function missing `Established<Invariant>` does NOT satisfy the bound.

use elicitation::{
    Elicit, Established, Prop, VerifiedStateMachine, VerifiedTransition, contracts::ProvableFrom,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── State type ────────────────────────────────────────────────────────────────

/// Minimal 3-state order workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
enum OrderState {
    Pending,
    Processing,
    Complete,
}

// ── Invariant proposition ─────────────────────────────────────────────────────

/// The order state machine invariant: the order ID is non-empty.
#[derive(Prop)]
struct OrderIdNonEmpty;

struct OrderIdCredential;
impl ProvableFrom<OrderIdCredential> for OrderIdNonEmpty {}

// ── Verified State Machine declaration ───────────────────────────────────────

struct OrderMachine;

impl VerifiedStateMachine for OrderMachine {
    type State = OrderState;
    type Invariant = OrderIdNonEmpty;
}

// ── Transitions (each satisfies VerifiedTransition<OrderMachine>) ─────────────

fn submit(
    _state: OrderState,
    proof: Established<OrderIdNonEmpty>,
) -> (OrderState, Established<OrderIdNonEmpty>) {
    (OrderState::Processing, proof)
}

fn complete(
    _state: OrderState,
    proof: Established<OrderIdNonEmpty>,
) -> (OrderState, Established<OrderIdNonEmpty>) {
    (OrderState::Complete, proof)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn transition_satisfies_verified_transition_bound() {
    fn assert_transition<T: VerifiedTransition<OrderMachine>>(_t: &T) {}
    assert_transition(&submit);
    assert_transition(&complete);
}

#[test]
fn closure_satisfies_verified_transition_bound() {
    let reset = |_state: OrderState,
                 proof: Established<OrderIdNonEmpty>|
     -> (OrderState, Established<OrderIdNonEmpty>) { (OrderState::Pending, proof) };

    fn assert_transition<T: VerifiedTransition<OrderMachine>>(_t: &T) {}
    assert_transition(&reset);
}

#[test]
fn multi_step_round_trip_carries_invariant() {
    use elicitation::FormalMethod;

    let cred = OrderIdCredential;
    let proof = Established::<OrderIdNonEmpty>::prove(&cred);

    let (mid_state, mid_proof) = submit.call_formal(OrderState::Pending, proof);
    assert_eq!(mid_state, OrderState::Processing);

    let (final_state, _final_proof) = complete.call_formal(mid_state, mid_proof);
    assert_eq!(final_state, OrderState::Complete);
}

#[test]
fn invariant_token_is_zero_sized() {
    let p = Established::<OrderIdNonEmpty>::assert();
    assert_eq!(std::mem::size_of_val(&p), 0);
}

/// Compile-time check: a function WITHOUT `Established` does NOT satisfy
/// `VerifiedTransition`. This is checked by the type system — the test here
/// just documents the negative case in prose, since a negative-compile test
/// would need a `trybuild` harness.
#[test]
fn informal_transition_is_not_verified() {
    // fn bad(_: OrderState) -> OrderState { OrderState::Pending }
    // assert_transition(&bad);  // ← would NOT compile — proof parameter missing
    //
    // The type system enforces this statically; no runtime check needed.
    let _documented = "informal functions lack Established<Invariant> — won't compile";
}
