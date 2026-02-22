//! Prusti proofs for bool contract types.

// Proof modules import types directly from elicitation
// use crate::*;

// Bool Contract Proofs
// ============================================================================

/// Prove that BoolTrue construction succeeds for true.
#[cfg(prusti)]
#[requires(value == true)]
#[ensures(result.is_ok())]
pub fn verify_bool_true_valid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolTrue construction fails for false.
#[cfg(prusti)]
#[requires(value == false)]
#[ensures(result.is_err())]
pub fn verify_bool_true_invalid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolFalse construction succeeds for false.
#[cfg(prusti)]
#[requires(value == false)]
#[ensures(result.is_ok())]
pub fn verify_bool_false_valid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}

/// Prove that BoolFalse construction fails for true.
#[cfg(prusti)]
#[requires(value == true)]
#[ensures(result.is_err())]
pub fn verify_bool_false_invalid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}

// ============================================================================
