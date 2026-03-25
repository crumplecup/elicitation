//! `VerifiedWorkflow` validation tests for elicit_time propositions.

#![cfg(feature = "proofs")]

use elicit_time::{OffsetDateTimeFuture, OffsetDateTimeParsed, PrimitiveDateTimeParsed};
use elicitation::contracts::And;
use elicitation::VerifiedWorkflow;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn time_props_non_empty() {
    assert_verified::<OffsetDateTimeParsed>("OffsetDateTimeParsed");
    assert_verified::<OffsetDateTimeFuture>("OffsetDateTimeFuture");
    assert_verified::<PrimitiveDateTimeParsed>("PrimitiveDateTimeParsed");
}

#[test]
fn time_and_contains_constituents() {
    type PF = And<OffsetDateTimeParsed, OffsetDateTimeFuture>;
    assert!(PF::kani_proof_contains::<OffsetDateTimeParsed>());
    assert!(PF::kani_proof_contains::<OffsetDateTimeFuture>());
}
