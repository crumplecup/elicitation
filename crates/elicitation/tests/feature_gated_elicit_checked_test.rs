//! Test that all feature-gated types have elicit_checked() method.
//!
//! This test verifies that the default `elicit_checked()` implementation
//! on the `Elicitation` trait works correctly for all feature-gated types.

use elicitation::Elicitation;

/// Test that url::Url has elicit_checked method.
#[test]
#[cfg(feature = "url")]
fn test_url_has_elicit_checked() {
    // This test just verifies compilation - the method exists
    let _: fn(_) -> _ = url::Url::elicit_checked;
}

/// Test that uuid::Uuid has elicit_checked method.
#[test]
#[cfg(feature = "uuid")]
fn test_uuid_has_elicit_checked() {
    let _: fn(_) -> _ = uuid::Uuid::elicit_checked;
}

/// Test that UuidGenerationMode has elicit_checked method.
#[test]
#[cfg(feature = "uuid")]
fn test_uuid_generation_mode_has_elicit_checked() {
    use elicitation::UuidGenerationMode;
    let _: fn(_) -> _ = UuidGenerationMode::elicit_checked;
}

/// Test that chrono datetime types have elicit_checked method.
#[test]
#[cfg(feature = "chrono")]
fn test_chrono_types_have_elicit_checked() {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

    let _: fn(_) -> _ = DateTime::<Utc>::elicit_checked;
    let _: fn(_) -> _ = DateTime::<FixedOffset>::elicit_checked;
    let _: fn(_) -> _ = NaiveDateTime::elicit_checked;
}

/// Test that time datetime types have elicit_checked method.
#[test]
#[cfg(feature = "time")]
fn test_time_types_have_elicit_checked() {
    use time::{OffsetDateTime, PrimitiveDateTime};

    // Note: time::Instant is deprecated, use std::time::Instant instead
    let _: fn(_) -> _ = std::time::Instant::elicit_checked;
    let _: fn(_) -> _ = OffsetDateTime::elicit_checked;
    let _: fn(_) -> _ = PrimitiveDateTime::elicit_checked;
}

/// Test that jiff datetime types have elicit_checked method.
#[test]
#[cfg(feature = "jiff")]
fn test_jiff_types_have_elicit_checked() {
    use jiff::civil::DateTime as CivilDateTime;
    use jiff::{Timestamp, Zoned};

    let _: fn(_) -> _ = Timestamp::elicit_checked;
    let _: fn(_) -> _ = Zoned::elicit_checked;
    let _: fn(_) -> _ = CivilDateTime::elicit_checked;
}

/// Test that serde_json::Value has elicit_checked method.
#[test]
#[cfg(feature = "serde_json")]
fn test_serde_json_value_has_elicit_checked() {
    use serde_json::Value;
    let _: fn(_) -> _ = Value::elicit_checked;
}

/// Test that primitive types have elicit_checked method.
#[test]
fn test_primitives_have_elicit_checked() {
    let _: fn(_) -> _ = bool::elicit_checked;
    let _: fn(_) -> _ = String::elicit_checked;
    let _: fn(_) -> _ = i32::elicit_checked;
    let _: fn(_) -> _ = u64::elicit_checked;
    let _: fn(_) -> _ = f64::elicit_checked;
}

/// Test that collection types have elicit_checked method.
#[test]
fn test_collections_have_elicit_checked() {
    use std::collections::{HashMap, HashSet};

    let _: fn(_) -> _ = Vec::<String>::elicit_checked;
    let _: fn(_) -> _ = HashSet::<i32>::elicit_checked;
    let _: fn(_) -> _ = HashMap::<String, i32>::elicit_checked;
}

/// Test that container types have elicit_checked method.
#[test]
fn test_containers_have_elicit_checked() {
    let _: fn(_) -> _ = Option::<String>::elicit_checked;
    let _: fn(_) -> _ = Result::<i32, String>::elicit_checked;
}
