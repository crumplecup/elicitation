//! Creusot proofs for duration contract types.

#![cfg(feature = "verify-creusot")]
#![allow(unused_imports)]

use crate::*;
use creusot_contracts::*;

// Duration Contract Proofs
// ============================================================================

/// Prove that DurationPositive construction succeeds for non-zero durations.
#[requires(!value.is_zero())]
#[ensures(result.is_ok())]
pub fn verify_duration_positive_valid(
    value: ::std::time::Duration,
) -> Result<DurationPositive, ValidationError> {
    DurationPositive::new(value)
}

/// Prove that DurationPositive construction fails for zero duration.
#[requires(value.is_zero())]
#[ensures(result.is_err())]
pub fn verify_duration_positive_invalid(
    value: ::std::time::Duration,
) -> Result<DurationPositive, ValidationError> {
    DurationPositive::new(value)
}

// ============================================================================
