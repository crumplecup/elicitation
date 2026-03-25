//! `VerifiedWorkflow` validation tests for elicit_serde_json propositions.

#![cfg(feature = "proofs")]

use elicit_serde_json::{IsObject, JsonParsed, PointerResolved, RequiredKeysPresent, UpdateApplied};
use elicitation::contracts::And;
use elicitation::VerifiedWorkflow;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn serde_json_props_non_empty() {
    assert_verified::<JsonParsed>("JsonParsed");
    assert_verified::<IsObject>("IsObject");
    assert_verified::<PointerResolved>("PointerResolved");
    assert_verified::<RequiredKeysPresent>("RequiredKeysPresent");
    assert_verified::<UpdateApplied>("UpdateApplied");
}

#[test]
fn serde_json_and_contains_constituents() {
    type PO = And<JsonParsed, IsObject>;
    type PR = And<JsonParsed, RequiredKeysPresent>;

    assert!(PO::kani_proof_contains::<JsonParsed>());
    assert!(PO::kani_proof_contains::<IsObject>());
    assert!(PR::kani_proof_contains::<RequiredKeysPresent>());
}
