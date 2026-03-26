//! `VerifiedWorkflow` validation tests for elicit_regex propositions.

#![cfg(feature = "proofs")]

use elicit_regex::{PatternMatched, RegexValid};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn regex_props_non_empty() {
    assert_verified::<RegexValid>("RegexValid");
    assert_verified::<PatternMatched>("PatternMatched");
}

#[test]
fn regex_and_contains_constituents() {
    type VP = And<RegexValid, PatternMatched>;
    assert!(VP::kani_proof_contains::<RegexValid>());
    assert!(VP::kani_proof_contains::<PatternMatched>());
}
