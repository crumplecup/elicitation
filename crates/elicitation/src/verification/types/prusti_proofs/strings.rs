//! Prusti proofs for string contract types.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::*;
use prusti_contracts::*;

// String Contract Proofs
// ============================================================================

/// Prove that StringNonEmpty construction succeeds for non-empty strings.
#[cfg(feature = "verify-prusti")]
#[requires(!value.is_empty())]
#[ensures(result.is_ok())]
pub fn verify_string_non_empty_valid(value: String) -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(value)
}

/// Prove that StringNonEmpty construction fails for empty strings.
#[cfg(feature = "verify-prusti")]
#[requires(value.is_empty())]
#[ensures(result.is_err())]
pub fn verify_string_non_empty_invalid(value: String) -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(value)
}

/// Prove that StringNonEmpty accessor returns the wrapped string.
#[cfg(feature = "verify-prusti")]
#[requires(!value.is_empty())]
#[ensures(match result {
    Ok(ref s) => s.get() == old(value),
    Err(_) => false,
})]
pub fn verify_string_non_empty_accessor(value: String) -> Result<StringNonEmpty, ValidationError> {
    StringNonEmpty::new(value)
}

// ============================================================================
