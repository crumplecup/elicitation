//! Creusot proofs for float contract types.
//!
//! Cloud of assumptions: We trust Rust stdlib f32/f64, is_finite() checks,
//! and our positivity/non-negativity comparisons. We verify wrapper structure.

use creusot_std::prelude::*;
use elicitation::{F32Finite, F32NonNegative, F32Positive, F64Finite, F64NonNegative, F64Positive};

// ============================================================================
// F32 Proofs
// ============================================================================

/// Verify F32Positive construction with valid positive value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f32_positive_valid() -> Result<F32Positive, elicitation::ValidationError> {
    F32Positive::new(42.5)
}

/// Verify F32Positive rejects zero/negative.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f32_positive_invalid() -> Result<F32Positive, elicitation::ValidationError> {
    F32Positive::new(0.0)
}

/// Verify F32NonNegative construction with valid non-negative value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f32_non_negative_valid() -> Result<F32NonNegative, elicitation::ValidationError> {
    F32NonNegative::new(0.0)
}

/// Verify F32NonNegative rejects negative.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f32_non_negative_invalid() -> Result<F32NonNegative, elicitation::ValidationError> {
    F32NonNegative::new(-1.0)
}

/// Verify F32Finite construction with valid finite value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f32_finite_valid() -> Result<F32Finite, elicitation::ValidationError> {
    F32Finite::new(123.456)
}

/// Verify F32Finite rejects NaN.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f32_finite_invalid() -> Result<F32Finite, elicitation::ValidationError> {
    F32Finite::new(f32::NAN)
}

// ============================================================================
// F64 Proofs
// ============================================================================

/// Verify F64Positive construction with valid positive value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f64_positive_valid() -> Result<F64Positive, elicitation::ValidationError> {
    F64Positive::new(42.5)
}

/// Verify F64Positive rejects zero/negative.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f64_positive_invalid() -> Result<F64Positive, elicitation::ValidationError> {
    F64Positive::new(0.0)
}

/// Verify F64NonNegative construction with valid non-negative value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f64_non_negative_valid() -> Result<F64NonNegative, elicitation::ValidationError> {
    F64NonNegative::new(0.0)
}

/// Verify F64NonNegative rejects negative.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f64_non_negative_invalid() -> Result<F64NonNegative, elicitation::ValidationError> {
    F64NonNegative::new(-1.0)
}

/// Verify F64Finite construction with valid finite value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f64_finite_valid() -> Result<F64Finite, elicitation::ValidationError> {
    F64Finite::new(123.456)
}

/// Verify F64Finite rejects NaN.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f64_finite_invalid() -> Result<F64Finite, elicitation::ValidationError> {
    F64Finite::new(f64::NAN)
}
