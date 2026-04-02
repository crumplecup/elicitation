//! Proof coverage tests for `elicit_jiff`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_jiff_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_jiff::Timestamp>();
    assert_elicit_complete::<elicit_jiff::Zoned>();
}

#[test]
fn jiff_wrapper_proofs_are_empty_by_design() {
    assert!(elicit_jiff::Timestamp::kani_proof().is_empty(), "Timestamp kani_proof expected empty");
    assert!(elicit_jiff::Zoned::kani_proof().is_empty(),     "Zoned kani_proof expected empty");
}
