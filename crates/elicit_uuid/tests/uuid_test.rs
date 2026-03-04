//! Basic tests for elicit_uuid::Uuid newtype.

use elicit_uuid::Uuid;
use schemars::JsonSchema as _;

#[test]
fn converts_from_uuid() {
    let raw = uuid::Uuid::new_v4();
    let wrapped: Uuid = raw.into();
    assert_eq!(*wrapped, raw);
}

#[test]
fn roundtrips_via_deref() {
    let raw = uuid::Uuid::new_v4();
    let wrapped: Uuid = raw.into();
    let back: uuid::Uuid = *wrapped;
    assert_eq!(raw, back);
}

#[test]
fn parse_valid() {
    let s = "550e8400-e29b-41d4-a716-446655440000";
    let id = Uuid::parse(s).expect("valid UUID");
    assert_eq!(id.to_hyphenated(), s);
}

#[test]
fn parse_invalid_returns_none() {
    assert!(Uuid::parse("not-a-uuid").is_none());
}

#[test]
fn nil_is_nil() {
    let nil: Uuid = uuid::Uuid::nil().into();
    assert!(nil.is_nil());
    assert_eq!(nil.version(), None);
}

#[test]
fn max_is_max() {
    let max: Uuid = uuid::Uuid::max().into();
    assert!(max.is_max());
}

#[test]
fn v4_version_is_4() {
    let id: Uuid = uuid::Uuid::new_v4().into();
    assert_eq!(id.version(), Some(4));
}

#[test]
fn simple_and_urn() {
    let id: Uuid = uuid::Uuid::new_v4().into();
    let simple = id.to_simple();
    assert_eq!(simple.len(), 32);
    assert!(id.to_urn().starts_with("urn:uuid:"));
}

#[test]
fn bytes_hex_is_32_chars() {
    let id: Uuid = uuid::Uuid::new_v4().into();
    let hex = id.as_bytes_hex();
    assert_eq!(hex.len(), 32);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn json_schema_has_uuid_format() {
    use schemars::SchemaGenerator;
    let schema = Uuid::json_schema(&mut SchemaGenerator::default());
    let json = serde_json::to_value(&schema).unwrap();
    assert_eq!(json["format"], "uuid");
}

#[test]
fn serde_roundtrip() {
    let id: Uuid = uuid::Uuid::new_v4().into();
    let json = serde_json::to_string(&id).unwrap();
    let back: Uuid = serde_json::from_str(&json).unwrap();
    assert_eq!(*id, *back);
}
