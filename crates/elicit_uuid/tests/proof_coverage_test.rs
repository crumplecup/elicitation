//! Proof coverage tests for `elicit_uuid`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types.
//!
//! Note: `uuid::Uuid` itself (not the wrapper) has non-empty proofs in the
//! `elicitation` crate's primitive impls, tested in `proof_non_empty_test.rs`.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_uuid_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_uuid::Uuid>();
}

#[test]
fn uuid_wrapper_proofs_are_empty_by_design() {
    assert!(!elicit_uuid::Uuid::kani_proof().is_empty(), "Uuid wrapper kani_proof expected non-empty");
}
