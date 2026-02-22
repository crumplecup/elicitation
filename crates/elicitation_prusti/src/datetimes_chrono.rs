//! Prusti proofs for chrono datetime contract types (feature-gated on chrono).
//!
//! Cloud of assumptions: Trust chrono crate datetime construction and comparisons.
//! Verify wrapper structure.

#![cfg(feature = "chrono")]

#[cfg(prusti)]
use prusti_contracts::{ensures, trusted};

#[cfg(prusti)]
use elicitation::{DateTimeUtcAfter, DateTimeUtcBefore, NaiveDateTimeAfter};

/// Verify DateTimeUtcAfter construction with datetime after threshold.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_datetime_utc_after_valid() -> Result<DateTimeUtcAfter, elicitation::ValidationError> {
    use chrono::{DateTime, Utc};
    let threshold = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
        .expect("Valid threshold")
        .with_timezone(&Utc);
    let after = DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
        .expect("Valid datetime")
        .with_timezone(&Utc);
    DateTimeUtcAfter::new(after, threshold)
}

/// Verify DateTimeUtcAfter rejects datetime before threshold.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_datetime_utc_after_invalid() -> Result<DateTimeUtcAfter, elicitation::ValidationError>
{
    use chrono::{DateTime, Utc};
    let threshold = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
        .expect("Valid threshold")
        .with_timezone(&Utc);
    let before = DateTime::parse_from_rfc3339("2019-01-01T00:00:00Z")
        .expect("Valid datetime")
        .with_timezone(&Utc);
    DateTimeUtcAfter::new(before, threshold)
}

/// Verify DateTimeUtcBefore construction with datetime before threshold.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_datetime_utc_before_valid() -> Result<DateTimeUtcBefore, elicitation::ValidationError>
{
    use chrono::{DateTime, Utc};
    let threshold = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
        .expect("Valid threshold")
        .with_timezone(&Utc);
    let before = DateTime::parse_from_rfc3339("2019-01-01T00:00:00Z")
        .expect("Valid datetime")
        .with_timezone(&Utc);
    DateTimeUtcBefore::new(before, threshold)
}

/// Verify DateTimeUtcBefore rejects datetime after threshold.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_datetime_utc_before_invalid(
) -> Result<DateTimeUtcBefore, elicitation::ValidationError> {
    use chrono::{DateTime, Utc};
    let threshold = DateTime::parse_from_rfc3339("2020-01-01T00:00:00Z")
        .expect("Valid threshold")
        .with_timezone(&Utc);
    let after = DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
        .expect("Valid datetime")
        .with_timezone(&Utc);
    DateTimeUtcBefore::new(after, threshold)
}

/// Verify NaiveDateTimeAfter construction with datetime after threshold.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_naive_datetime_after_valid(
) -> Result<NaiveDateTimeAfter, elicitation::ValidationError> {
    use chrono::NaiveDateTime;
    let threshold = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Valid threshold");
    let after = NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Valid datetime");
    NaiveDateTimeAfter::new(after, threshold)
}

/// Verify NaiveDateTimeAfter rejects datetime before threshold.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_naive_datetime_after_invalid(
) -> Result<NaiveDateTimeAfter, elicitation::ValidationError> {
    use chrono::NaiveDateTime;
    let threshold = NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Valid threshold");
    let before = NaiveDateTime::parse_from_str("2019-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
        .expect("Valid datetime");
    NaiveDateTimeAfter::new(before, threshold)
}
