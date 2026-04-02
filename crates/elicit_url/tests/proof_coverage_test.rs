//! Proof coverage tests for `elicit_url`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_url_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_url::Url>();
}

#[test]
fn url_wrapper_proofs_are_empty_by_design() {
    assert!(elicit_url::Url::kani_proof().is_empty(), "Url wrapper kani_proof expected empty");
}
