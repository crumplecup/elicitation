//! Creusot proofs for bool contract types.

#![cfg(feature = "verify-creusot")]
#![allow(unused_imports)]

use crate::*;
use creusot_contracts::prelude::*;

// Bool Contract Proofs
// ============================================================================

/// Prove that BoolTrue construction succeeds for true.
#[requires(value == true)]
#[ensures(result.is_ok())]
pub fn verify_bool_true_valid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolTrue construction fails for false.
#[requires(value == false)]
#[ensures(result.is_err())]
pub fn verify_bool_true_invalid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolFalse construction succeeds for false.
#[requires(value == false)]
#[ensures(result.is_ok())]
pub fn verify_bool_false_valid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}

/// Prove that BoolFalse construction fails for true.
#[requires(value == true)]
#[ensures(result.is_err())]
pub fn verify_bool_false_invalid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}

// ============================================================================
