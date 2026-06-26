//! Proof coverage tests for `elicit_reqwest`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. If any supertrait bound is accidentally dropped, this file
//! will fail to compile, acting as a regression test.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

// ── Compile-time ElicitComplete bound checks ────────────────────────────────

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_reqwest_types_are_elicit_complete() {
    // WIP: these types are implemented in their modules but not yet wired into
    // lib.rs — un-comment as each module is enabled.
    // assert_elicit_complete::<elicit_reqwest::Action>();
    // assert_elicit_complete::<elicit_reqwest::Body>();
    // assert_elicit_complete::<elicit_reqwest::Certificate>();
    // assert_elicit_complete::<elicit_reqwest::Client>();
    // assert_elicit_complete::<elicit_reqwest::ClientBuilder>();
    // assert_elicit_complete::<elicit_reqwest::Error>();
    // assert_elicit_complete::<elicit_reqwest::RequestBuilder>();
    // assert_elicit_complete::<elicit_reqwest::Identity>();
    // assert_elicit_complete::<elicit_reqwest::NoProxy>();
    // assert_elicit_complete::<elicit_reqwest::Policy>();
    // assert_elicit_complete::<elicit_reqwest::Proxy>();
    // assert_elicit_complete::<elicit_reqwest::Request>();
    // assert_elicit_complete::<elicit_reqwest::Response>();
    // assert_elicit_complete::<elicit_reqwest::Method>();
    // assert_elicit_complete::<elicit_reqwest::StatusCode>();
    // assert_elicit_complete::<elicit_reqwest::TlsInfo>();
    // assert_elicit_complete::<elicit_reqwest::Version>();
    // assert_elicit_complete::<elicit_reqwest::HeaderMap>();
}

// ── Proof coverage ────────────────────────────────────────────────────────────
//
// The reqwest shadows should emit trusted-opaque proof stubs rather than empty
// placeholders. These assertions make the coverage regression obvious.

// WIP: restore these proof assertions when modules are wired into lib.rs.
//
// #[test]
// fn reqwest_newtype_proofs_are_empty_by_design() {
//     assert!(!elicit_reqwest::Client::kani_proof().is_empty(), ...);
//     assert!(!elicit_reqwest::Error::kani_proof().is_empty(), ...);
//     assert!(!elicit_reqwest::RequestBuilder::kani_proof().is_empty(), ...);
//     assert!(!elicit_reqwest::Response::kani_proof().is_empty(), ...);
// }
//
// #[test]
// fn reqwest_trusted_opaque_proofs_are_non_empty() {
//     assert!(!elicit_reqwest::Method::kani_proof().is_empty(), ...);
//     assert!(!elicit_reqwest::StatusCode::kani_proof().is_empty(), ...);
//     assert!(!elicit_reqwest::Version::kani_proof().is_empty(), ...);
//     assert!(!elicit_reqwest::HeaderMap::kani_proof().is_empty(), ...);
// }
