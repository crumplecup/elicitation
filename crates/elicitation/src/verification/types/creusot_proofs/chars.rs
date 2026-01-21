//! Creusot proofs for char contract types.

#![cfg(feature = "verify-creusot")]
#![allow(unused_imports)]

use crate::*;
use creusot_contracts::*;

// Char Contract Proofs
// ============================================================================

/// Prove that CharAlphabetic construction succeeds for alphabetic chars.
#[cfg(feature = "verification")]
#[requires(value.is_alphabetic())]
#[ensures(result.is_ok())]
pub fn verify_char_alphabetic_valid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharAlphabetic construction fails for non-alphabetic chars.
#[cfg(feature = "verification")]
#[requires(!value.is_alphabetic())]
#[ensures(result.is_err())]
pub fn verify_char_alphabetic_invalid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharNumeric construction succeeds for numeric chars.
#[cfg(feature = "verification")]
#[requires(value.is_numeric())]
#[ensures(result.is_ok())]
pub fn verify_char_numeric_valid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharAlphanumeric construction succeeds for alphanumeric chars.
#[cfg(feature = "verification")]
#[requires(value.is_alphanumeric())]
#[ensures(result.is_ok())]
pub fn verify_char_alphanumeric_valid(value: char) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}

// ============================================================================
