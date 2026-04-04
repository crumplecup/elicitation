//! `VerifiedWorkflow` validation tests for elicit_chrono propositions.
//!
//! Asserts that every Prop has non-empty proofs and that composite
//! `And<P, Q>` types delegate to their constituent proofs.

use elicit_chrono::{DateTimeFuture, DateTimeInRange, DateTimeParsed};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn chrono_props_non_empty() {
    assert_verified::<DateTimeParsed>("DateTimeParsed");
    assert_verified::<DateTimeFuture>("DateTimeFuture");
    assert_verified::<DateTimeInRange>("DateTimeInRange");
}

#[test]
fn chrono_and_contains_constituents() {
    type PF = And<DateTimeParsed, DateTimeFuture>;
    type PR = And<DateTimeParsed, DateTimeInRange>;

    assert!(PF::kani_proof_contains::<DateTimeParsed>());
    assert!(PF::kani_proof_contains::<DateTimeFuture>());
    assert!(PR::kani_proof_contains::<DateTimeParsed>());
    assert!(PR::kani_proof_contains::<DateTimeInRange>());
}
