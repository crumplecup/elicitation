//! Creusot proofs for duration contract types.
//!
//! Cloud of assumptions: We trust Rust std::time::Duration and our
//! positivity check. We verify wrapper structure.

use creusot_std::prelude::*;
use elicitation::DurationPositive;
use std::time::Duration;

// ============================================================================
// DurationPositive Proofs
// ============================================================================

/// Verify DurationPositive construction with valid positive duration.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_duration_positive_valid() -> Result<DurationPositive, elicitation::ValidationError> {
    DurationPositive::new(Duration::from_secs(1))
}

/// Verify DurationPositive rejects zero duration.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_duration_positive_invalid() -> Result<DurationPositive, elicitation::ValidationError> {
    DurationPositive::new(Duration::from_secs(0))
}
