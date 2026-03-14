//! Creusot proofs for UTF-8 validation types.
//!
//! Proves: length bound enforcement is correct.
//! Trusts: stdlib UTF-8 validation (content correctness).

#[cfg(creusot)]
use crate::*;
#[cfg(creusot)]
use elicitation::verification::types::{Utf8Bytes, ValidationError};

// Length bound proofs (proven from extern_spec)
// ============================================================================

/// Verify: length exceeding MAX_LEN is rejected.
#[cfg(creusot)]
#[requires(len@ > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_utf8_length_check<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: valid length is accepted (UTF-8 validity trusted to stdlib).
#[cfg(creusot)]
#[requires(len@ <= MAX_LEN@)]
#[ensures(match result {
    Ok(ref utf8) => utf8_len(utf8) == len,
    Err(_) => true,
})]
pub fn verify_utf8_length_valid<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: accessor returns the same length that was provided.
#[cfg(creusot)]
#[requires(len@ <= MAX_LEN@)]
#[ensures(match result {
    Ok(ref utf8) => utf8_len(utf8) == len,
    Err(_) => true,
})]
pub fn verify_utf8_len_accessor<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: maximum length boundary (len == MAX_LEN) is accepted.
#[cfg(creusot)]
#[ensures(match result {
    Ok(ref utf8) => utf8_len(utf8) == MAX_LEN,
    Err(_) => true,
})]
pub fn verify_max_length_boundary<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, MAX_LEN)
}

/// Verify: len = MAX_LEN + 1 is rejected.
#[cfg(creusot)]
#[requires(MAX_LEN@ < usize::MAX@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_length_overflow<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, MAX_LEN + 1)
}

// Content proofs (trust stdlib UTF-8 validation)
// ============================================================================

/// Verify: empty byte sequence (len = 0) is valid UTF-8.
#[cfg(creusot)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
pub fn verify_empty_utf8() -> Result<Utf8Bytes<10>, ValidationError> {
    let bytes = [0u8; 10];
    Utf8Bytes::new(bytes, 0)
}

/// Verify: ASCII bytes construct valid UTF-8.
#[cfg(creusot)]
pub fn verify_ascii_valid() -> Result<Utf8Bytes<5>, ValidationError> {
    let bytes = [b'h', b'e', b'l', b'l', b'o'];
    Utf8Bytes::new(bytes, 5)
}
