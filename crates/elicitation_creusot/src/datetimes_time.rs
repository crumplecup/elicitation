//! Creusot proofs for time crate datetime contract types (feature-gated on time).
//!
//! Cloud of assumptions: Trust time crate datetime construction and comparisons.
//! Verify wrapper structure.

#![cfg(feature = "time")]

use creusot_std::prelude::*;
use elicitation::{OffsetDateTimeAfter, OffsetDateTimeBefore};

/// Verify OffsetDateTimeAfter construction with datetime after threshold.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_offset_datetime_after_valid() -> Result<OffsetDateTimeAfter, elicitation::ValidationError> {
    use time::macros::datetime;
    let threshold = datetime!(2020-01-01 0:00 UTC);
    let after = datetime!(2021-01-01 0:00 UTC);
    OffsetDateTimeAfter::new(after, threshold)
}

/// Verify OffsetDateTimeAfter rejects datetime before threshold.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_offset_datetime_after_invalid() -> Result<OffsetDateTimeAfter, elicitation::ValidationError> {
    use time::macros::datetime;
    let threshold = datetime!(2020-01-01 0:00 UTC);
    let before = datetime!(2019-01-01 0:00 UTC);
    OffsetDateTimeAfter::new(before, threshold)
}

/// Verify OffsetDateTimeBefore construction with datetime before threshold.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_offset_datetime_before_valid() -> Result<OffsetDateTimeBefore, elicitation::ValidationError> {
    use time::macros::datetime;
    let threshold = datetime!(2020-01-01 0:00 UTC);
    let before = datetime!(2019-01-01 0:00 UTC);
    OffsetDateTimeBefore::new(before, threshold)
}

/// Verify OffsetDateTimeBefore rejects datetime after threshold.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_offset_datetime_before_invalid() -> Result<OffsetDateTimeBefore, elicitation::ValidationError> {
    use time::macros::datetime;
    let threshold = datetime!(2020-01-01 0:00 UTC);
    let after = datetime!(2021-01-01 0:00 UTC);
    OffsetDateTimeBefore::new(after, threshold)
}
