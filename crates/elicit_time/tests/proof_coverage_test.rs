//! Proof coverage tests for `elicit_time`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_time_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_time::OffsetDateTime>();
    assert_elicit_complete::<elicit_time::PrimitiveDateTime>();
}

#[test]
fn time_wrapper_proofs_are_empty_by_design() {
    assert!(elicit_time::OffsetDateTime::kani_proof().is_empty(),     "OffsetDateTime kani_proof expected empty");
    assert!(elicit_time::PrimitiveDateTime::kani_proof().is_empty(),  "PrimitiveDateTime kani_proof expected empty");
}
