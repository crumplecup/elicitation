//! `VerifiedWorkflow` validation tests for elicit_jiff propositions.

use elicit_jiff::{TimestampFuture, TimestampParsed, TimezoneConverted, ZonedParsed};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn jiff_props_non_empty() {
    assert_verified::<TimestampParsed>("TimestampParsed");
    assert_verified::<TimestampFuture>("TimestampFuture");
    assert_verified::<ZonedParsed>("ZonedParsed");
    assert_verified::<TimezoneConverted>("TimezoneConverted");
}

#[test]
fn jiff_and_contains_constituents() {
    type PF = And<TimestampParsed, TimestampFuture>;
    type PZ = And<TimestampParsed, ZonedParsed>;

    assert!(PF::kani_proof_contains::<TimestampParsed>());
    assert!(PF::kani_proof_contains::<TimestampFuture>());
    assert!(PZ::kani_proof_contains::<ZonedParsed>());
}
