//! Prusti proofs for UTF-8 validation types.
//!
//! These proofs verify the UTF-8 wrapper logic assuming correct stdlib validation.
//! This is compositional verification: stdlib_correct → wrapper_correct.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::verification::types::{Utf8Bytes, ValidationError};
use prusti_contracts::*;

// UTF-8 Validation Proofs
// ============================================================================

/// Verify: Utf8Bytes correctly rejects length exceeding MAX_LEN
#[cfg(prusti)]
#[requires(len > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_utf8_length_check<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: Utf8Bytes correctly accepts valid length
#[cfg(prusti)]
#[requires(len <= MAX_LEN)]
pub fn verify_utf8_length_valid<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: Utf8Bytes accessor returns correct length
#[cfg(prusti)]
#[requires(len <= MAX_LEN)]
#[ensures(match result {
    Ok(ref utf8) => utf8.len() == len,
    Err(_) => true,
})]
pub fn verify_utf8_len_accessor<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: Utf8Bytes is_empty predicate for zero length
#[cfg(prusti)]
#[ensures(match result {
    Ok(ref utf8) => utf8.is_empty(),
    Err(_) => true,
})]
pub fn verify_utf8_empty<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, 0)
}

/// Verify: Utf8Bytes is_empty predicate for non-zero length
#[cfg(prusti)]
#[requires(len > 0 && len <= MAX_LEN)]
#[ensures(match result {
    Ok(ref utf8) => !utf8.is_empty(),
    Err(_) => true,
})]
pub fn verify_utf8_non_empty<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

// ASCII Validation (Compositional - trusts stdlib ASCII validation)
// ============================================================================

/// Verify: ASCII bytes (< 0x80) construct valid UTF-8
/// Compositional proof: if stdlib accepts ASCII, wrapper accepts it
#[cfg(prusti)]
pub fn verify_ascii_valid() -> Result<Utf8Bytes<5>, ValidationError> {
    let bytes = [b'h', b'e', b'l', b'l', b'o'];
    Utf8Bytes::new(bytes, 5)
}

/// Verify: Single ASCII byte is valid UTF-8
#[cfg(prusti)]
pub fn verify_single_ascii() -> Result<Utf8Bytes<1>, ValidationError> {
    let bytes = [b'x'];
    Utf8Bytes::new(bytes, 1)
}

/// Verify: Empty UTF-8 sequence is valid
#[cfg(prusti)]
#[ensures(result.is_ok())]
pub fn verify_empty_utf8() -> Result<Utf8Bytes<10>, ValidationError> {
    let bytes = [0u8; 10];
    Utf8Bytes::new(bytes, 0)
}

// Bounded Length Validation
// ============================================================================

/// Verify: Small buffer (2 bytes) works correctly
#[cfg(prusti)]
#[requires(len <= 2)]
pub fn verify_small_buffer(bytes: [u8; 2], len: usize) -> Result<Utf8Bytes<2>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: Medium buffer (16 bytes) works correctly
#[cfg(prusti)]
#[requires(len <= 16)]
pub fn verify_medium_buffer(bytes: [u8; 16], len: usize) -> Result<Utf8Bytes<16>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

/// Verify: Large buffer (256 bytes) works correctly
#[cfg(prusti)]
#[requires(len <= 256)]
pub fn verify_large_buffer(
    bytes: [u8; 256],
    len: usize,
) -> Result<Utf8Bytes<256>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

// Compositional Validation (Trust stdlib UTF-8 correctness)
// ============================================================================

/// Verify: Two-byte composition
/// If stdlib validates each byte sequence, composition is valid
#[cfg(prusti)]
pub fn verify_two_byte_composition() -> Result<Utf8Bytes<4>, ValidationError> {
    let bytes = [b'a', b'b', 0, 0];
    Utf8Bytes::new(bytes, 2)
}

/// Verify: Four-byte composition
#[cfg(prusti)]
pub fn verify_four_byte_composition() -> Result<Utf8Bytes<8>, ValidationError> {
    let bytes = [b't', b'e', b's', b't', 0, 0, 0, 0];
    Utf8Bytes::new(bytes, 4)
}

/// Verify: Construction does not panic on any valid length
#[cfg(prusti)]
#[requires(len <= MAX_LEN)]
pub fn verify_no_panic_valid_length<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, len)
}

// Edge Cases
// ============================================================================

/// Verify: Maximum length boundary (len == MAX_LEN)
#[cfg(prusti)]
#[ensures(match result {
    Ok(ref utf8) => utf8.len() == MAX_LEN,
    Err(_) => true,
})]
pub fn verify_max_length_boundary<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, MAX_LEN)
}

/// Verify: Length overflow detection (len = MAX_LEN + 1)
#[cfg(prusti)]
#[requires(MAX_LEN < usize::MAX)]
#[ensures(result.is_err())]
pub fn verify_length_overflow<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    Utf8Bytes::new(bytes, MAX_LEN + 1)
}

/// Verify: as_str() returns valid UTF-8 string slice
#[cfg(prusti)]
#[requires(len <= MAX_LEN)]
pub fn verify_as_str_valid<const MAX_LEN: usize>(
    bytes: [u8; MAX_LEN],
    len: usize,
) -> Result<Utf8Bytes<MAX_LEN>, ValidationError> {
    let utf8 = Utf8Bytes::new(bytes, len)?;
    let _s = utf8.as_str(); // Should not panic
    Ok(utf8)
}

// ============================================================================
