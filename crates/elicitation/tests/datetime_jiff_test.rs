//! Tests for jiff datetime elicitation.

#![cfg(feature = "jiff")]

use elicitation::{Elicitation, Prompt};
use jiff::{Timestamp, Zoned, civil::DateTime as CivilDateTime};

// Compile-time trait bound checks
#[test]
fn test_timestamp_implements_elicitation() {
    fn assert_elicitation<T: Elicitation>() {}
    assert_elicitation::<Timestamp>();
}

#[test]
fn test_zoned_implements_elicitation() {
    fn assert_elicitation<T: Elicitation>() {}
    assert_elicitation::<Zoned>();
}

#[test]
fn test_civil_datetime_implements_elicitation() {
    fn assert_elicitation<T: Elicitation>() {}
    assert_elicitation::<CivilDateTime>();
}

#[test]
fn test_timestamp_has_prompt() {
    assert!(Timestamp::prompt().is_some());
}

#[test]
fn test_zoned_has_prompt() {
    assert!(Zoned::prompt().is_some());
}

#[test]
fn test_civil_datetime_has_prompt() {
    assert!(CivilDateTime::prompt().is_some());
}

// Integration tests (require MCP client)
#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_timestamp_iso() {
    // Test ISO 8601: "2024-07-11T15:30:00Z"
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_timestamp_components() {
    // Test manual components
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_zoned_iso() {
    // Test ISO 8601 with timezone: "2024-07-11T15:30:00-05[America/New_York]"
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_zoned_components() {
    // Test manual components with IANA timezone
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_civil_datetime_iso() {
    // Test ISO 8601 no timezone: "2024-07-11T15:30:00"
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_civil_datetime_components() {
    // Test manual components
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_invalid_iso_format() {
    // Test error handling for malformed ISO strings
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_invalid_components() {
    // Test error handling for invalid date components
}

// Unit tests for datetime parsing
#[test]
fn test_jiff_parses_valid_timestamp() {
    let result = "2024-07-11T15:30:00Z".parse::<Timestamp>();
    assert!(result.is_ok());
}

#[test]
fn test_jiff_parses_valid_zoned() {
    // Use -04 for EDT (America/New_York in July)
    let result = "2024-07-11T15:30:00-04:00[America/New_York]".parse::<Zoned>();
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert!(result.is_ok());
}

#[test]
fn test_jiff_parses_valid_civil() {
    let result = "2024-07-11T15:30:00".parse::<CivilDateTime>();
    assert!(result.is_ok());
}

#[test]
fn test_jiff_rejects_invalid_timestamp() {
    let result = "not-a-date".parse::<Timestamp>();
    assert!(result.is_err());
}

#[test]
fn test_jiff_constructs_civil_from_components() {
    let result = CivilDateTime::new(2024, 7, 11, 15, 30, 0, 0);
    assert!(result.is_ok());
}

#[test]
fn test_jiff_rejects_invalid_month() {
    let result = CivilDateTime::new(2024, 13, 11, 15, 30, 0, 0);
    assert!(result.is_err());
}

#[test]
fn test_jiff_rejects_invalid_day() {
    let result = CivilDateTime::new(2024, 2, 30, 15, 30, 0, 0);
    assert!(result.is_err());
}
