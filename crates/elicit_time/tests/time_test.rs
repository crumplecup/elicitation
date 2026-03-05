//! Integration tests for `elicit_time` types.

use elicit_time::{OffsetDateTime, PrimitiveDateTime};
use schemars::schema_for;
use serde_json::json;

// ── OffsetDateTime ─────────────────────────────────────────────────────────

#[test]
fn offset_date_time_serialize_rfc3339() {
    let dt = OffsetDateTime::parse("2024-01-15T12:30:00+00:00").unwrap();
    let json = serde_json::to_string(&dt).unwrap();
    // Should round-trip through Rfc3339
    assert!(json.contains("2024-01-15"));
    assert!(json.contains("12:30:00"));
}

#[test]
fn offset_date_time_deserialize_rfc3339() {
    let json = r#""2024-06-01T08:00:00+05:30""#;
    let dt: OffsetDateTime = serde_json::from_str(json).unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 6);
    assert_eq!(dt.day(), 1);
    assert_eq!(dt.hour(), 8);
}

#[test]
fn offset_date_time_roundtrip() {
    let original = "2024-03-15T09:45:00+00:00";
    let dt: OffsetDateTime = serde_json::from_str(&format!("\"{original}\"")).unwrap();
    let back = serde_json::to_string(&dt).unwrap();
    let dt2: OffsetDateTime = serde_json::from_str(&back).unwrap();
    assert_eq!(dt.unix_timestamp(), dt2.unix_timestamp());
}

#[test]
fn offset_date_time_reflect_methods() {
    let dt = OffsetDateTime::parse("2024-11-05T14:22:33+00:00").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 11);
    assert_eq!(dt.day(), 5);
    assert_eq!(dt.hour(), 14);
    assert_eq!(dt.minute(), 22);
    assert_eq!(dt.second(), 33);
    assert_eq!(dt.nanosecond(), 0);
    assert!(dt.unix_timestamp() > 0);
    assert_eq!(dt.utc_offset(), "+00:00:00");
}

#[test]
fn offset_date_time_json_schema_format() {
    let schema = schema_for!(OffsetDateTime);
    let value = serde_json::to_value(&schema).unwrap();
    assert_eq!(value["format"], json!("date-time"));
    assert_eq!(value["type"], json!("string"));
}

// ── PrimitiveDateTime ──────────────────────────────────────────────────────

#[test]
fn primitive_date_time_serialize() {
    let dt = PrimitiveDateTime::parse("2024-07-04T18:30:00").unwrap();
    let json = serde_json::to_string(&dt).unwrap();
    assert_eq!(json, r#""2024-07-04T18:30:00""#);
}

#[test]
fn primitive_date_time_deserialize() {
    let json = r#""2024-12-25T00:00:00""#;
    let dt: PrimitiveDateTime = serde_json::from_str(json).unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 12);
    assert_eq!(dt.day(), 25);
    assert_eq!(dt.hour(), 0);
    assert_eq!(dt.minute(), 0);
    assert_eq!(dt.second(), 0);
}

#[test]
fn primitive_date_time_roundtrip() {
    let original = r#""2024-08-20T15:45:10""#;
    let dt: PrimitiveDateTime = serde_json::from_str(original).unwrap();
    let back = serde_json::to_string(&dt).unwrap();
    assert_eq!(original, back);
}

#[test]
fn primitive_date_time_reflect_methods() {
    let dt = PrimitiveDateTime::parse("2024-02-29T23:59:59").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 2);
    assert_eq!(dt.day(), 29);
    assert_eq!(dt.hour(), 23);
    assert_eq!(dt.minute(), 59);
    assert_eq!(dt.second(), 59);
}

#[test]
fn primitive_date_time_json_schema_is_string() {
    let schema = schema_for!(PrimitiveDateTime);
    let value = serde_json::to_value(&schema).unwrap();
    assert_eq!(value["type"], json!("string"));
}
