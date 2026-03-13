//! Creusot proofs for char contract types.

use crate::*;
use elicitation::{CharAlphabetic, CharAlphanumeric, CharNumeric, ValidationError};

/// Prove that CharAlphabetic construction succeeds for alphabetic chars.
#[cfg(creusot)]
#[requires(char_is_alphabetic(value))]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_char_alphabetic_valid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharAlphabetic construction fails for non-alphabetic chars.
#[cfg(creusot)]
#[requires(!char_is_alphabetic(value))]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_char_alphabetic_invalid(value: char) -> Result<CharAlphabetic, ValidationError> {
    CharAlphabetic::new(value)
}

/// Prove that CharNumeric construction succeeds for numeric chars.
#[cfg(creusot)]
#[requires(char_is_numeric(value))]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_char_numeric_valid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharNumeric construction fails for non-numeric chars.
#[cfg(creusot)]
#[requires(!char_is_numeric(value))]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_char_numeric_invalid(value: char) -> Result<CharNumeric, ValidationError> {
    CharNumeric::new(value)
}

/// Prove that CharAlphanumeric construction succeeds for alphanumeric chars.
#[cfg(creusot)]
#[requires(char_is_alphanumeric(value))]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_char_alphanumeric_valid(value: char) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}

/// Prove that CharAlphanumeric construction fails for non-alphanumeric chars.
#[cfg(creusot)]
#[requires(!char_is_alphanumeric(value))]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_char_alphanumeric_invalid(value: char) -> Result<CharAlphanumeric, ValidationError> {
    CharAlphanumeric::new(value)
}
