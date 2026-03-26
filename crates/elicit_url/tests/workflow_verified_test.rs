//! `VerifiedWorkflow` validation tests for elicit_url propositions.

#![cfg(feature = "proofs")]

use elicit_url::{HttpsRequired, SchemeAllowed, UrlParsed};
use elicitation::VerifiedWorkflow;
use elicitation::contracts::And;

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

#[test]
fn url_props_non_empty() {
    assert_verified::<UrlParsed>("UrlParsed");
    assert_verified::<HttpsRequired>("HttpsRequired");
    assert_verified::<SchemeAllowed>("SchemeAllowed");
}

#[test]
fn url_and_contains_constituents() {
    // SecureUrl = And<UrlParsed, HttpsRequired> — the production composite
    type SecureUrl = And<UrlParsed, HttpsRequired>;
    type AllowedUrl = And<UrlParsed, SchemeAllowed>;

    assert!(SecureUrl::kani_proof_contains::<UrlParsed>());
    assert!(SecureUrl::kani_proof_contains::<HttpsRequired>());
    assert!(AllowedUrl::kani_proof_contains::<UrlParsed>());
    assert!(AllowedUrl::kani_proof_contains::<SchemeAllowed>());
}
