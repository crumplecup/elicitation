//! Proof coverage tests for `elicit_reqwest`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. If any supertrait bound is accidentally dropped, this file
//! will fail to compile, acting as a regression test.
//!
//! The proof methods are intentionally empty for external-runtime wrapper
//! types — HTTP clients, request builders, and responses are opaque runtime
//! resources that cannot be formally verified. The tests below assert this
//! explicitly.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

// ── Compile-time ElicitComplete bound checks ────────────────────────────────

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_reqwest_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_reqwest::Client>();
    assert_elicit_complete::<elicit_reqwest::Error>();
    assert_elicit_complete::<elicit_reqwest::RequestBuilder>();
    assert_elicit_complete::<elicit_reqwest::Response>();
    assert_elicit_complete::<elicit_reqwest::Method>();
    assert_elicit_complete::<elicit_reqwest::StatusCode>();
    assert_elicit_complete::<elicit_reqwest::Version>();
    assert_elicit_complete::<elicit_reqwest::HeaderMap>();
}

// ── Proof emptiness: elicit_newtype! wrappers vs trusted-opaque types ───────
//
// Client, Error, RequestBuilder, Response use `elicit_newtype!` which emits
// empty proof methods — these are opaque runtime resources with no formal proof.
//
// Method, StatusCode, Version, HeaderMap use `impl_elicitation_for_reqwest_newtype!`
// which emits `kani_trusted_opaque` — non-empty trusted axiom proofs.

#[test]
fn reqwest_newtype_proofs_are_empty_by_design() {
    assert!(elicit_reqwest::Client::kani_proof().is_empty(),         "Client kani_proof expected empty");
    assert!(elicit_reqwest::Error::kani_proof().is_empty(),          "Error kani_proof expected empty");
    assert!(elicit_reqwest::RequestBuilder::kani_proof().is_empty(), "RequestBuilder kani_proof expected empty");
    assert!(elicit_reqwest::Response::kani_proof().is_empty(),       "Response kani_proof expected empty");
}

#[test]
fn reqwest_trusted_opaque_proofs_are_non_empty() {
    assert!(!elicit_reqwest::Method::kani_proof().is_empty(),     "Method kani_proof expected non-empty");
    assert!(!elicit_reqwest::StatusCode::kani_proof().is_empty(), "StatusCode kani_proof expected non-empty");
    assert!(!elicit_reqwest::Version::kani_proof().is_empty(),    "Version kani_proof expected non-empty");
    assert!(!elicit_reqwest::HeaderMap::kani_proof().is_empty(),  "HeaderMap kani_proof expected non-empty");
}
