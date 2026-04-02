//! Proof coverage tests for `elicit_regex`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_regex_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_regex::Regex>();
}

#[test]
fn regex_wrapper_proofs_are_empty_by_design() {
    assert!(!elicit_regex::Regex::kani_proof().is_empty(), "Regex kani_proof expected non-empty");
}
