//! Creusot proofs for char contract types.

use crate::*;

// Char Contract Proofs
// ============================================================================
//
// Note: These proofs assume creusot-std provides contracts for char methods
// (is_alphabetic, is_numeric, is_alphanumeric). If not, Creusot will treat
// them as uninterpreted functions, which still allows us to prove our wrapper
// logic is correct given the stdlib contracts.

/// Prove that CharAlphabetic construction succeeds for alphabetic chars.
#[requires(value.is_alphabetic())]
#[ensures(result.is_ok())]
pub fn verify_char_alphabetic_valid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharAlphabetic construction fails for non-alphabetic chars.
#[requires(!value.is_alphabetic())]
#[ensures(result.is_err())]
pub fn verify_char_alphabetic_invalid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharNumeric construction succeeds for numeric chars.
#[requires(value.is_numeric())]
#[ensures(result.is_ok())]
pub fn verify_char_numeric_valid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharNumeric construction fails for non-numeric chars.
#[requires(!value.is_numeric())]
#[ensures(result.is_err())]
pub fn verify_char_numeric_invalid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharAlphanumeric construction succeeds for alphanumeric chars.
#[requires(value.is_alphanumeric())]
#[ensures(result.is_ok())]
pub fn verify_char_alphanumeric_valid(value: char) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}

/// Prove that CharAlphanumeric construction fails for non-alphanumeric chars.
#[requires(!value.is_alphanumeric())]
#[ensures(result.is_err())]
pub fn verify_char_alphanumeric_invalid(value: char) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}
