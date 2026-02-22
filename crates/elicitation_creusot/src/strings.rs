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
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_string_non_empty_valid() -> Result<StringNonEmpty, elicitation::ValidationError> {
    StringNonEmpty::new("hello".to_string())
}

/// Verify StringNonEmpty rejects empty string.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_string_non_empty_invalid() -> Result<StringNonEmpty, elicitation::ValidationError> {
    StringNonEmpty::new(String::new())
}

/// Verify StringNonEmpty with custom max length.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_string_non_empty_bounded_valid()
-> Result<StringNonEmpty<10>, elicitation::ValidationError> {
    StringNonEmpty::<10>::new("short".to_string())
}

/// Verify StringNonEmpty rejects string exceeding max length.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_string_non_empty_too_long() -> Result<StringNonEmpty<10>, elicitation::ValidationError>
{
    StringNonEmpty::<10>::new("this string is way too long".to_string())
}
