//! Prusti proofs for duration contract types.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::*;
use prusti_contracts::*;

// Duration Contract Proofs
// ============================================================================

/// Prove that DurationPositive construction succeeds for non-zero durations.
#[cfg(prusti)]
#[requires(!value.is_zero())]
#[ensures(result.is_ok())]
pub fn verify_duration_positive_valid(
    value: std::time::Duration,
) -> Result<DurationPositive, ValidationError> {
    DurationPositive::new(value)
}

/// Prove that DurationPositive construction fails for zero duration.
#[cfg(prusti)]
#[requires(value.is_zero())]
#[ensures(result.is_err())]
pub fn verify_duration_positive_invalid(
    value: std::time::Duration,
) -> Result<DurationPositive, ValidationError> {
    DurationPositive::new(value)
}

// ============================================================================
