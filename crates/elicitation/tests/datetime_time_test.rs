//! Tests for time datetime elicitation.

#![cfg(feature = "time")]

use elicitation::{Elicitation, Prompt};
use time::{OffsetDateTime, PrimitiveDateTime};

// Compile-time trait bound checks
#[test]
fn test_offset_datetime_implements_elicitation() {
    fn assert_elicitation<T: Elicitation>() {}
    assert_elicitation::<OffsetDateTime>();
}

#[test]
fn test_primitive_datetime_implements_elicitation() {
    fn assert_elicitation<T: Elicitation>() {}
    assert_elicitation::<PrimitiveDateTime>();
}

#[test]
fn test_offset_datetime_has_prompt() {
    assert!(OffsetDateTime::prompt().is_some());
}

#[test]
fn test_primitive_datetime_has_prompt() {
    assert!(PrimitiveDateTime::prompt().is_some());
}

// Integration tests (require MCP client)
#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_offset_datetime_iso() {
    // Test ISO 8601 with offset: "2024-07-11T15:30:00+05:00"
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_offset_datetime_components() {
    // Test manual components with offset
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_primitive_datetime_iso() {
    // Test ISO 8601 no timezone: "2024-07-11T15:30:00"
}

#[tokio::test]
#[ignore = "Requires MCP client connection"]
async fn test_elicit_primitive_datetime_components() {
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
fn test_time_parses_valid_iso8601_offset() {
    use time::format_description::well_known::Rfc3339;
    let result = OffsetDateTime::parse("2024-07-11T15:30:00+05:00", &Rfc3339);
    assert!(result.is_ok());
}

#[test]
fn test_time_parses_valid_iso8601_primitive() {
    use time::format_description::well_known::Rfc3339;
    let result = PrimitiveDateTime::parse("2024-07-11T15:30:00Z", &Rfc3339);
    assert!(result.is_ok());
}

#[test]
fn test_time_rejects_invalid_iso8601() {
    use time::format_description::well_known::Rfc3339;
    let result = OffsetDateTime::parse("not-a-date", &Rfc3339);
    assert!(result.is_err());
}

#[test]
fn test_time_date_from_components() {
    use time::{Date, Month};
    let result = Date::from_calendar_date(2024, Month::July, 11);
    assert!(result.is_ok());
}

#[test]
fn test_time_rejects_invalid_month() {
    use time::Month;
    let result = Month::try_from(13_u8);
    assert!(result.is_err());
}

#[test]
fn test_time_rejects_invalid_date() {
    use time::{Date, Month};
    let result = Date::from_calendar_date(2024, Month::February, 30);
    assert!(result.is_err());
}
