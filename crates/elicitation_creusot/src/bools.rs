//! Creusot proofs for bool contract types.

use crate::*;
use elicitation::{BoolFalse, BoolTrue, ValidationError};

// Bool Contract Proofs
// ============================================================================
//
// extern_spec contracts for BoolTrue::new and BoolFalse::new are in
// extern_specs.rs. The proof obligations here follow by simple implication
// from those axioms and are discharged by Alt-Ergo automatically.

/// Prove that BoolTrue construction succeeds for true.
#[requires(value)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_bool_true_valid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolTrue construction fails for false.
#[requires(!value)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
pub fn verify_bool_true_invalid(value: bool) -> Result<BoolTrue, ValidationError> {
    BoolTrue::new(value)
}

/// Prove that BoolFalse construction succeeds for false.
#[requires(!value)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_bool_false_valid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}

/// Prove that BoolFalse construction fails for true.
#[requires(value)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
pub fn verify_bool_false_invalid(value: bool) -> Result<BoolFalse, ValidationError> {
    BoolFalse::new(value)
}
