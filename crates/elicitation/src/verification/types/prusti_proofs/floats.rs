//! Prusti proofs for float contract types.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::*;
use prusti_contracts::*;

// Float Contract Proofs
// ============================================================================

/// Prove that F32Finite construction succeeds for finite values.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_finite())]
#[ensures(result.is_ok())]
pub fn verify_f32_finite_valid(value: f32) -> Result<F32Finite, ValidationError> {
    F32Finite::new(value)
}

/// Prove that F32Finite construction fails for non-finite values.
#[cfg(feature = "verify-prusti")]
#[requires(!value.is_finite())]
#[ensures(result.is_err())]
pub fn verify_f32_finite_invalid(value: f32) -> Result<F32Finite, ValidationError> {
    F32Finite::new(value)
}

/// Prove that F32Positive construction succeeds for positive finite values.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_finite() && value > 0.0)]
#[ensures(result.is_ok())]
pub fn verify_f32_positive_valid(value: f32) -> Result<F32Positive, ValidationError> {
    F32Positive::new(value)
}

/// Prove that F32NonNegative construction succeeds for non-negative finite values.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_finite() && value >= 0.0)]
#[ensures(result.is_ok())]
pub fn verify_f32_non_negative_valid(value: f32) -> Result<F32NonNegative, ValidationError> {
    F32NonNegative::new(value)
}

/// Prove that F64Finite construction succeeds for finite values.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_finite())]
#[ensures(result.is_ok())]
pub fn verify_f64_finite_valid(value: f64) -> Result<F64Finite, ValidationError> {
    F64Finite::new(value)
}

/// Prove that F64Positive construction succeeds for positive finite values.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_finite() && value > 0.0)]
#[ensures(result.is_ok())]
pub fn verify_f64_positive_valid(value: f64) -> Result<F64Positive, ValidationError> {
    F64Positive::new(value)
}

/// Prove that F64NonNegative construction succeeds for non-negative finite values.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_finite() && value >= 0.0)]
#[ensures(result.is_ok())]
pub fn verify_f64_non_negative_valid(value: f64) -> Result<F64NonNegative, ValidationError> {
    F64NonNegative::new(value)
}

// ============================================================================
