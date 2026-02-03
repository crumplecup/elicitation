//! Creusot proofs for string contract types.

#![cfg(feature = "verify-creusot")]
#![allow(unused_imports)]

use crate::*;
// Contracts provided by creusot_std;

// String Contract Proofs
// ============================================================================

/// Prove that StringNonEmpty construction succeeds for non-empty strings.
#[requires(!value.is_empty())]
#[ensures(result.is_ok())]
pub fn verify_string_non_empty_valid(value: String) -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(value)
}

/// Prove that StringNonEmpty construction fails for empty strings.
#[requires(value.is_empty())]
#[ensures(result.is_err())]
pub fn verify_string_non_empty_invalid(value: String) -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(value)
}

/// Prove that StringNonEmpty accessor returns the wrapped string.
#[requires(!value.is_empty())]
#[ensures(match ^result {
    Ok(s) => s.get() == value,
    Err(_) => false,
})]
pub fn verify_string_non_empty_accessor(value: String) -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(value)
}

// ============================================================================
