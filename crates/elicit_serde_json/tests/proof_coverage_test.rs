//! Proof coverage tests for `elicit_serde_json`.
//!
//! Verifies that every wrapper type satisfies the [`ElicitComplete`] supertrait
//! at compile time. Proof methods are intentionally empty for external wrapper
//! types.
//!
//! [`ElicitComplete`]: elicitation::ElicitComplete

use elicitation::Elicitation as _;

fn assert_elicit_complete<T: elicitation::ElicitComplete>() {}

#[test]
fn all_serde_json_types_are_elicit_complete() {
    assert_elicit_complete::<elicit_serde_json::JsonValue>();
    assert_elicit_complete::<elicit_serde_json::JsonNumber>();
}

#[test]
fn serde_json_wrapper_proofs_are_empty_by_design() {
    assert!(elicit_serde_json::JsonValue::kani_proof().is_empty(),  "JsonValue kani_proof expected empty");
    assert!(elicit_serde_json::JsonNumber::kani_proof().is_empty(), "JsonNumber kani_proof expected empty");
}
