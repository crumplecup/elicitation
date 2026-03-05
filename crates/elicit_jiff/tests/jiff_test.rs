//! Integration tests for `elicit_jiff` types.

use elicit_jiff::{Timestamp, Zoned};
use schemars::schema_for;
use serde_json::json;

// ── Zoned ──────────────────────────────────────────────────────────────────

#[test]
fn zoned_parse_and_reflect() {
    let z = Zoned::parse("2024-01-15T12:30:00+00:00[UTC]").unwrap();
    assert_eq!(z.year(), 2024);
    assert_eq!(z.month(), 1);
    assert_eq!(z.day(), 15);
    assert_eq!(z.hour(), 12);
    assert_eq!(z.minute(), 30);
    assert_eq!(z.second(), 0);
}

#[test]
fn zoned_serde_roundtrip() {
    let z = Zoned::parse("2024-06-01T00:00:00+00:00[UTC]").unwrap();
    let json = serde_json::to_string(&z).unwrap();
    let z2: Zoned = serde_json::from_str(&json).unwrap();
    assert_eq!(z.timestamp_seconds(), z2.timestamp_seconds());
}

#[test]
fn zoned_timezone_name() {
    let z = Zoned::parse("2024-01-15T12:30:00+00:00[UTC]").unwrap();
    assert_eq!(z.timezone_name(), "UTC");
}

#[test]
fn zoned_weekday() {
    let z = Zoned::parse("2024-01-15T00:00:00+00:00[UTC]").unwrap(); // Monday
    let wd = z.weekday();
    assert_eq!(wd, "Monday");
}

#[test]
fn zoned_in_tz() {
    let z = Zoned::parse("2024-01-15T12:00:00+00:00[UTC]").unwrap();
    let result = z.in_tz("UTC".to_string());
    assert!(result.is_some());
    assert!(result.unwrap().contains("2024-01-15"));
}

#[test]
fn zoned_day_of_year() {
    let z = Zoned::parse("2024-01-15T00:00:00+00:00[UTC]").unwrap();
    assert_eq!(z.day_of_year(), 15);
}

#[test]
fn zoned_json_schema() {
    let schema = schema_for!(Zoned);
    let value = serde_json::to_value(&schema).unwrap();
    // jiff::Zoned serializes as a string
    assert!(value.get("type").is_some() || value.get("$ref").is_some());
}

// ── Timestamp ─────────────────────────────────────────────────────────────

#[test]
fn timestamp_from_second_and_reflect() {
    let ts = Timestamp::from_second(0).unwrap();
    assert_eq!(ts.as_second(), 0);
    assert!(ts.is_zero());
    assert_eq!(ts.signum(), 0);
}

#[test]
fn timestamp_positive() {
    let ts = Timestamp::from_second(1_700_000_000).unwrap();
    assert_eq!(ts.as_second(), 1_700_000_000);
    assert!(!ts.is_zero());
    assert_eq!(ts.signum(), 1);
    assert!(ts.as_millisecond() > ts.as_second());
}

#[test]
fn timestamp_in_tz() {
    let ts = Timestamp::from_second(0).unwrap();
    let result = ts.in_tz("UTC".to_string());
    assert!(result.is_some());
}

#[test]
fn timestamp_serde_roundtrip() {
    let ts = Timestamp::from_second(1_700_000_000).unwrap();
    let json = serde_json::to_string(&ts).unwrap();
    let ts2: Timestamp = serde_json::from_str(&json).unwrap();
    assert_eq!(ts.as_second(), ts2.as_second());
}

#[test]
fn timestamp_json_schema() {
    let schema = schema_for!(Timestamp);
    let value = serde_json::to_value(&schema).unwrap();
    assert!(value.get("type").is_some() || value.get("$ref").is_some());
}
