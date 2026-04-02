//! Proof coverage tests for `elicit_clap`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. If any supertrait bound is accidentally dropped, this file
//! will fail to compile, acting as a regression test.
//!
//! The proof methods (`kani_proof`, `verus_proof`, `creusot_proof`) are
//! intentionally empty for external-runtime wrapper types — formal verification
//! is not applicable to opaque third-party types. The tests below assert this
//! explicitly so the emptiness is documented and not accidentally "fixed".
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

// ── Compile-time ElicitComplete bound checks ────────────────────────────────

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_clap_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_clap::Arg>();
    assert_elicit_complete::<elicit_clap::ArgAction>();
    assert_elicit_complete::<elicit_clap::ArgGroup>();
    assert_elicit_complete::<elicit_clap::ColorChoice>();
    assert_elicit_complete::<elicit_clap::Command>();
    assert_elicit_complete::<elicit_clap::ErrorKind>();
    assert_elicit_complete::<elicit_clap::Id>();
    assert_elicit_complete::<elicit_clap::PossibleValue>();
    assert_elicit_complete::<elicit_clap::ValueHint>();
    assert_elicit_complete::<elicit_clap::ValueRange>();
    assert_elicit_complete::<elicit_clap::ValueSource>();
}

// ── Proofs are intentionally empty for external wrapper types ───────────────

#[test]
fn clap_wrapper_proofs_are_empty_by_design() {
    assert!(elicit_clap::Arg::kani_proof().is_empty(),       "Arg kani_proof expected empty");
    assert!(elicit_clap::ArgAction::kani_proof().is_empty(), "ArgAction kani_proof expected empty");
    assert!(elicit_clap::ArgGroup::kani_proof().is_empty(),  "ArgGroup kani_proof expected empty");
    assert!(elicit_clap::ColorChoice::kani_proof().is_empty(),"ColorChoice kani_proof expected empty");
    assert!(elicit_clap::Command::kani_proof().is_empty(),   "Command kani_proof expected empty");
    assert!(elicit_clap::ErrorKind::kani_proof().is_empty(), "ErrorKind kani_proof expected empty");
    assert!(elicit_clap::Id::kani_proof().is_empty(),        "Id kani_proof expected empty");
    assert!(elicit_clap::PossibleValue::kani_proof().is_empty(), "PossibleValue kani_proof expected empty");
    assert!(elicit_clap::ValueHint::kani_proof().is_empty(), "ValueHint kani_proof expected empty");
    assert!(elicit_clap::ValueRange::kani_proof().is_empty(),"ValueRange kani_proof expected empty");
    assert!(elicit_clap::ValueSource::kani_proof().is_empty(),"ValueSource kani_proof expected empty");
}
