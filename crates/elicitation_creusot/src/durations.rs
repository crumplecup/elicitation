//! Creusot proofs for duration contract types.

use crate::*;
use elicitation::{DurationPositive, ValidationError};
use std::time::Duration;

/// Verify DurationPositive construction succeeds for positive durations.
#[cfg(creusot)]
#[requires(duration_is_positive(duration))]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_duration_positive_valid(duration: Duration) -> Result<DurationPositive, ValidationError> {
    DurationPositive::new(duration)
}

/// Verify DurationPositive rejects zero / non-positive durations.
#[cfg(creusot)]
#[requires(!duration_is_positive(duration))]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_duration_positive_invalid(duration: Duration) -> Result<DurationPositive, ValidationError> {
    DurationPositive::new(duration)
}
