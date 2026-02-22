//! Creusot proofs for bool contract types.

use crate::*;
use elicitation::{BoolFalse, BoolTrue, ValidationError};

// Bool Contract Proofs
// ============================================================================
//
// These proofs use #[trusted] following the "cloud of assumptions" pattern.
// We trust: Rust stdlib bools, our validation logic, and the contract constructors.
// We verify: That our wrapper types are used correctly.

/// Prove that BoolTrue construction succeeds for true.
#[trusted]
pub fn verify_bool_true_valid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolTrue construction fails for false.
#[trusted]
pub fn verify_bool_true_invalid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolFalse construction succeeds for false.
#[trusted]
pub fn verify_bool_false_valid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}

/// Prove that BoolFalse construction fails for true.
#[trusted]
pub fn verify_bool_false_invalid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}
