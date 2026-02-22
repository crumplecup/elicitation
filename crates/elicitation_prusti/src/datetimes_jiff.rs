//! Prusti proofs for jiff timestamp contract types (feature-gated on jiff).
//!
//! Cloud of assumptions: Trust jiff crate timestamp construction and comparisons.
//! Verify wrapper structure.

#![cfg(feature = "jiff")]

#[cfg(prusti)]
use prusti_contracts::{ensures, trusted};

#[cfg(prusti)]
use elicitation::{TimestampAfter, TimestampBefore};

/// Verify TimestampAfter construction with timestamp after threshold.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_timestamp_after_valid() -> Result<TimestampAfter, elicitation::ValidationError> {
    use jiff::Timestamp;
    let threshold = Timestamp::from_second(1577836800).expect("Valid threshold"); // 2020-01-01
    let after = Timestamp::from_second(1609459200).expect("Valid timestamp"); // 2021-01-01
    TimestampAfter::new(after, threshold)
}

/// Verify TimestampAfter rejects timestamp before threshold.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_timestamp_after_invalid() -> Result<TimestampAfter, elicitation::ValidationError> {
    use jiff::Timestamp;
    let threshold = Timestamp::from_second(1577836800).expect("Valid threshold"); // 2020-01-01
    let before = Timestamp::from_second(1546300800).expect("Valid timestamp"); // 2019-01-01
    TimestampAfter::new(before, threshold)
}

/// Verify TimestampBefore construction with timestamp before threshold.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_timestamp_before_valid() -> Result<TimestampBefore, elicitation::ValidationError> {
    use jiff::Timestamp;
    let threshold = Timestamp::from_second(1577836800).expect("Valid threshold"); // 2020-01-01
    let before = Timestamp::from_second(1546300800).expect("Valid timestamp"); // 2019-01-01
    TimestampBefore::new(before, threshold)
}

/// Verify TimestampBefore rejects timestamp after threshold.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_timestamp_before_invalid() -> Result<TimestampBefore, elicitation::ValidationError> {
    use jiff::Timestamp;
    let threshold = Timestamp::from_second(1577836800).expect("Valid threshold"); // 2020-01-01
    let after = Timestamp::from_second(1609459200).expect("Valid timestamp"); // 2021-01-01
    TimestampBefore::new(after, threshold)
}
