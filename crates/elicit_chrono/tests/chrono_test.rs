//! Integration tests for `elicit_chrono` types.

use elicit_chrono::{DateTime, DateTimeFixed, NaiveDateTime};
use schemars::schema_for;

// ── DateTime ───────────────────────────────────────────────────────────────

#[test]
fn date_time_utc_parse_and_reflect() {
    let dt = DateTime::parse("2024-01-15T12:30:00Z").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 1);
    assert_eq!(dt.day(), 15);
    assert_eq!(dt.hour(), 12);
    assert_eq!(dt.minute(), 30);
    assert_eq!(dt.second(), 0);
}

#[test]
fn date_time_utc_serde_roundtrip() {
    let dt = DateTime::parse("2024-06-01T00:00:00Z").unwrap();
    let json = serde_json::to_string(&dt).unwrap();
    let dt2: DateTime = serde_json::from_str(&json).unwrap();
    assert_eq!(dt.timestamp(), dt2.timestamp());
}

#[test]
fn date_time_utc_reflect_extras() {
    let dt = DateTime::parse("2024-03-11T00:00:00Z").unwrap(); // Monday
    assert_eq!(dt.weekday(), "Mon");
    assert!(dt.ordinal() > 0);
    assert!(dt.timestamp() > 0);
    assert!(dt.timestamp_millis() > 0);
}

#[test]
fn date_time_utc_to_rfc3339() {
    let dt = DateTime::parse("2024-01-01T00:00:00Z").unwrap();
    let s = dt.to_rfc3339();
    assert!(s.starts_with("2024-01-01"));
}

#[test]
fn date_time_utc_json_schema_is_string() {
    let schema = schema_for!(DateTime);
    let value = serde_json::to_value(&schema).unwrap();
    assert_eq!(value["type"], serde_json::json!("string"));
}

// ── DateTimeFixed ─────────────────────────────────────────────────────────────

#[test]
fn date_time_fixed_parse_and_reflect() {
    let dt = DateTimeFixed::parse("2024-01-15T12:30:00+05:30").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 1);
    assert_eq!(dt.day(), 15);
    assert_eq!(dt.hour(), 12);
    assert_eq!(dt.offset_seconds(), 5 * 3600 + 30 * 60);
}

#[test]
fn date_time_fixed_serde_roundtrip() {
    let s = r#""2024-08-20T15:45:10+00:00""#;
    let dt: DateTimeFixed = serde_json::from_str(s).unwrap();
    let back = serde_json::to_string(&dt).unwrap();
    let dt2: DateTimeFixed = serde_json::from_str(&back).unwrap();
    assert_eq!(dt.timestamp(), dt2.timestamp());
}

// ── NaiveDateTime ─────────────────────────────────────────────────────────────

#[test]
fn naive_date_time_parse_and_reflect() {
    let dt = NaiveDateTime::parse("2024-02-29T23:59:59").unwrap();
    assert_eq!(dt.year(), 2024);
    assert_eq!(dt.month(), 2);
    assert_eq!(dt.day(), 29);
    assert_eq!(dt.hour(), 23);
    assert_eq!(dt.minute(), 59);
    assert_eq!(dt.second(), 59);
}

#[test]
fn naive_date_time_format() {
    let dt = NaiveDateTime::parse("2024-12-25T08:00:00").unwrap();
    let formatted = dt.format("%Y/%m/%d".to_string());
    assert_eq!(formatted, "2024/12/25");
}

#[test]
fn naive_date_time_serde_roundtrip() {
    let dt = NaiveDateTime::parse("2024-07-04T18:00:00").unwrap();
    let json = serde_json::to_string(&dt).unwrap();
    let dt2: NaiveDateTime = serde_json::from_str(&json).unwrap();
    assert_eq!(dt.timestamp(), dt2.timestamp());
}

#[test]
fn naive_date_time_json_schema_is_string() {
    let schema = schema_for!(NaiveDateTime);
    let value = serde_json::to_value(&schema).unwrap();
    assert_eq!(value["type"], serde_json::json!("string"));
}
