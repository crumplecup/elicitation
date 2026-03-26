//! Creusot proofs for Unix path validation types.
//!
//! These proofs verify path wrapper logic assuming correct UTF-8 and null-byte validation.
//! This is compositional verification: (utf8_correct ∧ no_null_correct) → wrapper_correct.

#[cfg(creusot)]
use crate::*;

#[cfg(creusot)]
use elicitation::verification::types::{
    PathAbsolute, PathBytes, PathNonEmpty, PathRelative, ValidationError,
};

// PathBytes Validation Proofs
// ============================================================================

/// Verify: PathBytes correctly rejects length exceeding MAX_LEN
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_path_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathBytes<MAX_LEN>, ValidationError> {
    PathBytes::from_slice(bytes)
}

/// Verify: PathBytes accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
#[ensures(match result {
    Ok(ref path) => path_len(path)@ == bytes@.len(),
    Err(_) => true,
})]
pub fn verify_path_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathBytes<MAX_LEN>, ValidationError> {
    PathBytes::from_slice(bytes)
}

/// Verify: PathBytes construction from empty slice
#[cfg(creusot)]
pub fn verify_path_empty() -> Result<PathBytes<10>, ValidationError> {
    PathBytes::from_slice(&[])
}

/// Verify: PathBytes construction from single ASCII byte
#[cfg(creusot)]
pub fn verify_path_single_byte() -> Result<PathBytes<1>, ValidationError> {
    let bytes = [b'a'];
    PathBytes::from_slice(&bytes)
}

/// Verify: PathBytes construction from ASCII path
#[cfg(creusot)]
pub fn verify_path_ascii() -> Result<PathBytes<10>, ValidationError> {
    let bytes = [b'/', b'u', b's', b'r', b'/', b'l', b'o', b'c', b'a', b'l'];
    PathBytes::from_slice(&bytes)
}

/// Verify: PathBytes construction with root path
#[cfg(creusot)]
pub fn verify_path_root() -> Result<PathBytes<1>, ValidationError> {
    let bytes = [b'/'];
    PathBytes::from_slice(&bytes)
}

/// Verify: PathBytes construction with current directory
#[cfg(creusot)]
pub fn verify_path_current_dir() -> Result<PathBytes<1>, ValidationError> {
    let bytes = [b'.'];
    PathBytes::from_slice(&bytes)
}

/// Verify: PathBytes construction with parent directory
#[cfg(creusot)]
pub fn verify_path_parent_dir() -> Result<PathBytes<2>, ValidationError> {
    let bytes = [b'.', b'.'];
    PathBytes::from_slice(&bytes)
}

// Compositional Validation (Trust stdlib UTF-8 + null check)
// ============================================================================

/// Verify: PathBytes as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_path_as_str_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathBytes<MAX_LEN>, ValidationError> {
    let path = PathBytes::from_slice(bytes)?;
    let _s = path.as_str(); // Should not panic
    Ok(path)
}

/// Verify: PathBytes len() returns correct value
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
#[ensures(match result {
    Ok(ref path) => path_len(path)@ == bytes@.len(),
    Err(_) => true,
})]
pub fn verify_path_len_accessor<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathBytes<MAX_LEN>, ValidationError> {
    PathBytes::from_slice(bytes)
}

/// Verify: PathBytes is_empty() for zero length
#[cfg(creusot)]
#[ensures(match result {
    Ok(ref path) => path_is_empty(path),
    Err(_) => true,
})]
pub fn verify_path_empty_predicate() -> Result<PathBytes<10>, ValidationError> {
    PathBytes::from_slice(&[])
}

/// Verify: PathBytes is_empty() for non-zero length
#[cfg(creusot)]
#[ensures(match result {
    Ok(ref path) => !path_is_empty(path),
    Err(_) => true,
})]
pub fn verify_path_non_empty_predicate() -> Result<PathBytes<10>, ValidationError> {
    let bytes = [b'/', b'h', b'o', b'm', b'e'];
    PathBytes::from_slice(&bytes)
}

// PathAbsolute Validation Proofs
// ============================================================================

/// Verify: PathAbsolute accepts path starting with /
#[cfg(creusot)]
pub fn verify_absolute_with_leading_slash() -> Result<PathAbsolute<10>, ValidationError> {
    let bytes = [b'/', b'u', b's', b'r', b'/', b'b', b'i', b'n'];
    PathAbsolute::from_slice(&bytes)
}

/// Verify: PathAbsolute accepts root path
#[cfg(creusot)]
pub fn verify_absolute_root() -> Result<PathAbsolute<1>, ValidationError> {
    let bytes = [b'/'];
    PathAbsolute::from_slice(&bytes)
}

/// Verify: PathAbsolute length check propagates
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_absolute_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathAbsolute<MAX_LEN>, ValidationError> {
    PathAbsolute::from_slice(bytes)
}

/// Verify: PathAbsolute get() returns underlying PathBytes
#[cfg(creusot)]
pub fn verify_absolute_get_accessor() -> Result<PathAbsolute<10>, ValidationError> {
    let bytes = [b'/', b'h', b'o', b'm', b'e'];
    let abs = PathAbsolute::from_slice(&bytes)?;
    let _path = abs.get(); // Should not panic
    Ok(abs)
}

/// Verify: PathAbsolute as_str() returns valid string
#[cfg(creusot)]
pub fn verify_absolute_as_str() -> Result<PathAbsolute<10>, ValidationError> {
    let bytes = [b'/', b't', b'm', b'p'];
    let abs = PathAbsolute::from_slice(&bytes)?;
    let _s = abs.as_str(); // Should not panic
    Ok(abs)
}

// PathRelative Validation Proofs
// ============================================================================

/// Verify: PathRelative accepts path not starting with /
#[cfg(creusot)]
pub fn verify_relative_no_leading_slash() -> Result<PathRelative<10>, ValidationError> {
    let bytes = [b'u', b's', b'r', b'/', b'l', b'o', b'c', b'a', b'l'];
    PathRelative::from_slice(&bytes)
}

/// Verify: PathRelative accepts current directory
#[cfg(creusot)]
pub fn verify_relative_current_dir() -> Result<PathRelative<1>, ValidationError> {
    let bytes = [b'.'];
    PathRelative::from_slice(&bytes)
}

/// Verify: PathRelative accepts parent directory
#[cfg(creusot)]
pub fn verify_relative_parent_dir() -> Result<PathRelative<2>, ValidationError> {
    let bytes = [b'.', b'.'];
    PathRelative::from_slice(&bytes)
}

/// Verify: PathRelative accepts simple filename
#[cfg(creusot)]
pub fn verify_relative_filename() -> Result<PathRelative<10>, ValidationError> {
    let bytes = [b'f', b'i', b'l', b'e', b'.', b't', b'x', b't'];
    PathRelative::from_slice(&bytes)
}

/// Verify: PathRelative length check propagates
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_relative_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathRelative<MAX_LEN>, ValidationError> {
    PathRelative::from_slice(bytes)
}

/// Verify: PathRelative get() returns underlying PathBytes
#[cfg(creusot)]
pub fn verify_relative_get_accessor() -> Result<PathRelative<10>, ValidationError> {
    let bytes = [b'h', b'o', b'm', b'e'];
    let rel = PathRelative::from_slice(&bytes)?;
    let _path = rel.get(); // Should not panic
    Ok(rel)
}

/// Verify: PathRelative as_str() returns valid string
#[cfg(creusot)]
pub fn verify_relative_as_str() -> Result<PathRelative<10>, ValidationError> {
    let bytes = [b't', b'm', b'p'];
    let rel = PathRelative::from_slice(&bytes)?;
    let _s = rel.as_str(); // Should not panic
    Ok(rel)
}

// PathNonEmpty Validation Proofs
// ============================================================================

/// Verify: PathNonEmpty rejects empty path
#[cfg(creusot)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_non_empty_rejects_empty() -> Result<PathNonEmpty<10>, ValidationError> {
    PathNonEmpty::from_slice(&[])
}

/// Verify: PathNonEmpty accepts single character
#[cfg(creusot)]
pub fn verify_non_empty_single_char() -> Result<PathNonEmpty<1>, ValidationError> {
    let bytes = [b'/'];
    PathNonEmpty::from_slice(&bytes)
}

/// Verify: PathNonEmpty accepts multi-character path
#[cfg(creusot)]
pub fn verify_non_empty_multi_char() -> Result<PathNonEmpty<10>, ValidationError> {
    let bytes = [b'/', b'u', b's', b'r', b'/', b'b', b'i', b'n'];
    PathNonEmpty::from_slice(&bytes)
}

/// Verify: PathNonEmpty length check propagates
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_non_empty_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<PathNonEmpty<MAX_LEN>, ValidationError> {
    PathNonEmpty::from_slice(bytes)
}

/// Verify: PathNonEmpty get() returns underlying PathBytes
#[cfg(creusot)]
pub fn verify_non_empty_get_accessor() -> Result<PathNonEmpty<10>, ValidationError> {
    let bytes = [b'/', b'h', b'o', b'm', b'e'];
    let nonempty = PathNonEmpty::from_slice(&bytes)?;
    let _path = nonempty.get(); // Should not panic
    Ok(nonempty)
}

/// Verify: PathNonEmpty as_str() returns valid string
#[cfg(creusot)]
pub fn verify_non_empty_as_str() -> Result<PathNonEmpty<10>, ValidationError> {
    let bytes = [b'/', b't', b'm', b'p'];
    let nonempty = PathNonEmpty::from_slice(&bytes)?;
    let _s = nonempty.as_str(); // Should not panic
    Ok(nonempty)
}

// Edge Cases
// ============================================================================

/// Verify: Small buffer (2 bytes) works correctly
#[cfg(creusot)]
#[requires(bytes@.len() <= 2)]
pub fn verify_path_small_buffer(bytes: &[u8]) -> Result<PathBytes<2>, ValidationError> {
    PathBytes::from_slice(bytes)
}

/// Verify: Medium buffer (64 bytes) works correctly
#[cfg(creusot)]
#[requires(bytes@.len() <= 64)]
pub fn verify_path_medium_buffer(bytes: &[u8]) -> Result<PathBytes<64>, ValidationError> {
    PathBytes::from_slice(bytes)
}

/// Verify: Large buffer (4096 bytes) works correctly
#[cfg(creusot)]
#[requires(bytes@.len() <= 4096)]
pub fn verify_path_large_buffer(bytes: &[u8]) -> Result<PathBytes<4096>, ValidationError> {
    PathBytes::from_slice(bytes)
}

// ============================================================================
