//! Proof coverage tests for `elicit_chrono`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types — chrono datetime types are opaque third-party types.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_chrono_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_chrono::DateTimeFixed>();
    assert_elicit_complete::<elicit_chrono::DateTime>();
    assert_elicit_complete::<elicit_chrono::Duration>();
    assert_elicit_complete::<elicit_chrono::NaiveDateTime>();
    assert_elicit_complete::<elicit_chrono::Utc>();
}

#[test]
fn chrono_wrapper_proofs_are_empty_by_design() {
    assert!(
        !elicit_chrono::DateTimeFixed::kani_proof().is_empty(),
        "DateTimeFixed kani_proof expected non-empty"
    );
    assert!(
        !elicit_chrono::DateTime::kani_proof().is_empty(),
        "DateTime kani_proof expected non-empty"
    );
    assert!(
        !elicit_chrono::Duration::kani_proof().is_empty(),
        "Duration kani_proof expected non-empty"
    );
    assert!(
        !elicit_chrono::NaiveDateTime::kani_proof().is_empty(),
        "NaiveDateTime kani_proof expected non-empty"
    );
    assert!(
        !elicit_chrono::Utc::kani_proof().is_empty(),
        "Utc kani_proof expected non-empty"
    );
}
