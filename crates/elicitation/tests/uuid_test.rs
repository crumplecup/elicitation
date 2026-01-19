//! Tests for UUID elicitation.
//!
//! Requires uuid feature.

#![cfg(feature = "uuid")]

use elicitation::Elicitation;
use uuid::Uuid;

#[test]
fn test_uuid_has_prompt() {
    use elicitation::Prompt;
    let prompt = Uuid::prompt();
    assert!(prompt.is_some());
    let text = prompt.unwrap();
    assert!(text.contains("UUID"));
    assert!(text.contains("generate"));
}

#[test]
fn test_uuid_trait_bounds() {
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}
    fn assert_elicitation<T: Elicitation>() {}

    assert_send::<Uuid>();
    assert_sync::<Uuid>();
    assert_elicitation::<Uuid>();
}

#[test]
fn test_uuid_parse() {
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let uuid = Uuid::parse_str(uuid_str).expect("Valid UUID");
    assert_eq!(uuid.to_string(), uuid_str);
}

#[test]
fn test_uuid_generate() {
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();
    // Extremely unlikely to generate same UUID twice
    assert_ne!(uuid1, uuid2);
}
