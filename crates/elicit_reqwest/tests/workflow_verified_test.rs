//! `VerifiedWorkflow` validation tests for elicit_reqwest propositions.

#![cfg(feature = "proofs")]

use elicit_reqwest::{Authorized, RequestCompleted, StatusSuccess, UrlValid};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn reqwest_props_non_empty() {
    assert_verified::<UrlValid>("UrlValid");
    assert_verified::<RequestCompleted>("RequestCompleted");
    assert_verified::<StatusSuccess>("StatusSuccess");
    assert_verified::<Authorized>("Authorized");
}

#[test]
fn reqwest_and_contains_constituents() {
    type RC = And<RequestCompleted, StatusSuccess>;
    type RA = And<RequestCompleted, Authorized>;

    assert!(RC::kani_proof_contains::<RequestCompleted>());
    assert!(RC::kani_proof_contains::<StatusSuccess>());
    assert!(RA::kani_proof_contains::<Authorized>());
}
