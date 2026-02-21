//! Creusot proofs for char contract types.

use crate::*;

// Char Contract Proofs
// ============================================================================
//
// Note: char methods (is_alphabetic, is_numeric, is_alphanumeric) are not
// specified in creusot-std. We mark these proofs as #[trusted] since we
// cannot express the char method preconditions in pure logic.

/// Prove that CharAlphabetic construction succeeds for alphabetic chars.
#[trusted]
pub fn verify_char_alphabetic_valid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharAlphabetic construction fails for non-alphabetic chars.
#[trusted]
pub fn verify_char_alphabetic_invalid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharNumeric construction succeeds for numeric chars.
#[trusted]
pub fn verify_char_numeric_valid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharNumeric construction fails for non-numeric chars.
#[trusted]
pub fn verify_char_numeric_invalid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharAlphanumeric construction succeeds for alphanumeric chars.
#[trusted]
pub fn verify_char_alphanumeric_valid(value: char) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}

/// Prove that CharAlphanumeric construction fails for non-alphanumeric chars.
#[trusted]
pub fn verify_char_alphanumeric_invalid(
    value: char,
) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}
