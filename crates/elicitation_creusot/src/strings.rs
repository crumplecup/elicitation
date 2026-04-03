//! Creusot proofs for string contract types.
//!
//! Cloud of assumptions: We trust Rust stdlib String, UTF-8 validation,
//! and our length/non-empty checks. We verify wrapper type structure.

use creusot_std::prelude::*;
use elicitation::StringNonEmpty;

// ============================================================================
// StringNonEmpty Proofs
// ============================================================================

/// Verify StringNonEmpty construction with valid non-empty string.
///
/// Parameterized proof: any string with 0 < len ≤ 4096 bytes produces Ok.
#[requires(value@.len() > 0 && value@.len() <= 4096)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_string_non_empty_valid(
    value: String,
) -> Result<StringNonEmpty, elicitation::ValidationError> {
    StringNonEmpty::new(value)
}

/// Verify StringNonEmpty rejects empty string.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
pub fn verify_string_non_empty_invalid() -> Result<StringNonEmpty, elicitation::ValidationError> {
    StringNonEmpty::new(String::new())
}

/// Verify StringNonEmpty with custom max length.
///
/// Parameterized proof: any string with 0 < len ≤ 10 bytes produces Ok.
#[requires(value@.len() > 0 && value@.len() <= 10)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_string_non_empty_bounded_valid(
    value: String,
) -> Result<StringNonEmpty<10>, elicitation::ValidationError> {
    StringNonEmpty::<10>::new(value)
}

/// Verify StringNonEmpty rejects string exceeding max length.
///
/// Parameterized proof: any string with len > 10 bytes produces Err.
#[requires(value@.len() > 10)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
pub fn verify_string_non_empty_too_long(
    value: String,
) -> Result<StringNonEmpty<10>, elicitation::ValidationError> {
    StringNonEmpty::<10>::new(value)
}
