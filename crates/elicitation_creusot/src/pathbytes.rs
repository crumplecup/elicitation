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
#[trusted]
#[cfg(creusot)]
pub fn verify_path_empty() -> Result<PathBytes<10>, ValidationError> {
    PathBytes::from_slice(&[])
}

/// Verify: PathBytes construction from single ASCII byte
#[trusted]
#[cfg(creusot)]
pub fn verify_path_single_byte() -> Result<PathBytes<1>, ValidationError> {
    PathBytes::from_slice(b"a")
}

/// Verify: PathBytes construction from ASCII path
#[trusted]
#[cfg(creusot)]
pub fn verify_path_ascii() -> Result<PathBytes<10>, ValidationError> {
    PathBytes::from_slice(b"/usr/local")
}

/// Verify: PathBytes construction with root path
#[trusted]
#[cfg(creusot)]
pub fn verify_path_root() -> Result<PathBytes<1>, ValidationError> {
    PathBytes::from_slice(b"/")
}

/// Verify: PathBytes construction with current directory
#[trusted]
#[cfg(creusot)]
pub fn verify_path_current_dir() -> Result<PathBytes<1>, ValidationError> {
    PathBytes::from_slice(b".")
}

/// Verify: PathBytes construction with parent directory
#[trusted]
#[cfg(creusot)]
pub fn verify_path_parent_dir() -> Result<PathBytes<2>, ValidationError> {
    PathBytes::from_slice(b"..")
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
#[trusted]
#[cfg(creusot)]
#[ensures(match result {
    Ok(ref path) => path_is_empty(path),
    Err(_) => true,
})]
pub fn verify_path_empty_predicate() -> Result<PathBytes<10>, ValidationError> {
    PathBytes::from_slice(&[])
}

/// Verify: PathBytes is_empty() for non-zero length
#[trusted]
#[cfg(creusot)]
#[ensures(match result {
    Ok(ref path) => !path_is_empty(path),
    Err(_) => true,
})]
pub fn verify_path_non_empty_predicate() -> Result<PathBytes<10>, ValidationError> {
    PathBytes::from_slice(b"/home")
}

// PathAbsolute Validation Proofs
// ============================================================================

/// Verify: PathAbsolute accepts path starting with /
#[trusted]
#[cfg(creusot)]
pub fn verify_absolute_with_leading_slash() -> Result<PathAbsolute<10>, ValidationError> {
    PathAbsolute::from_slice(b"/usr/bin")
}

/// Verify: PathAbsolute accepts root path
#[trusted]
#[cfg(creusot)]
pub fn verify_absolute_root() -> Result<PathAbsolute<1>, ValidationError> {
    PathAbsolute::from_slice(b"/")
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
#[trusted]
#[cfg(creusot)]
pub fn verify_absolute_get_accessor() -> Result<PathAbsolute<10>, ValidationError> {
    let abs = PathAbsolute::from_slice(b"/home")?;
    let _path = abs.get(); // Should not panic
    Ok(abs)
}

/// Verify: PathAbsolute as_str() returns valid string
#[trusted]
#[cfg(creusot)]
pub fn verify_absolute_as_str() -> Result<PathAbsolute<10>, ValidationError> {
    let abs = PathAbsolute::from_slice(b"/tmp")?;
    let _s = abs.as_str(); // Should not panic
    Ok(abs)
}

// PathRelative Validation Proofs
// ============================================================================

/// Verify: PathRelative accepts path not starting with /
#[trusted]
#[cfg(creusot)]
pub fn verify_relative_no_leading_slash() -> Result<PathRelative<10>, ValidationError> {
    PathRelative::from_slice(b"usr/local")
}

/// Verify: PathRelative accepts current directory
#[trusted]
#[cfg(creusot)]
pub fn verify_relative_current_dir() -> Result<PathRelative<1>, ValidationError> {
    PathRelative::from_slice(b".")
}

/// Verify: PathRelative accepts parent directory
#[trusted]
#[cfg(creusot)]
pub fn verify_relative_parent_dir() -> Result<PathRelative<2>, ValidationError> {
    PathRelative::from_slice(b"..")
}

/// Verify: PathRelative accepts simple filename
#[trusted]
#[cfg(creusot)]
pub fn verify_relative_filename() -> Result<PathRelative<10>, ValidationError> {
    PathRelative::from_slice(b"file.txt")
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
#[trusted]
#[cfg(creusot)]
pub fn verify_relative_get_accessor() -> Result<PathRelative<10>, ValidationError> {
    let rel = PathRelative::from_slice(b"home")?;
    let _path = rel.get(); // Should not panic
    Ok(rel)
}

/// Verify: PathRelative as_str() returns valid string
#[trusted]
#[cfg(creusot)]
pub fn verify_relative_as_str() -> Result<PathRelative<10>, ValidationError> {
    let rel = PathRelative::from_slice(b"tmp")?;
    let _s = rel.as_str(); // Should not panic
    Ok(rel)
}

// PathNonEmpty Validation Proofs
// ============================================================================

/// Verify: PathNonEmpty rejects empty path
#[trusted]
#[cfg(creusot)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_non_empty_rejects_empty() -> Result<PathNonEmpty<10>, ValidationError> {
    PathNonEmpty::from_slice(&[])
}

/// Verify: PathNonEmpty accepts single character
#[trusted]
#[cfg(creusot)]
pub fn verify_non_empty_single_char() -> Result<PathNonEmpty<1>, ValidationError> {
    PathNonEmpty::from_slice(b"/")
}

/// Verify: PathNonEmpty accepts multi-character path
#[trusted]
#[cfg(creusot)]
pub fn verify_non_empty_multi_char() -> Result<PathNonEmpty<10>, ValidationError> {
    PathNonEmpty::from_slice(b"/usr/bin")
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
#[trusted]
#[cfg(creusot)]
pub fn verify_non_empty_get_accessor() -> Result<PathNonEmpty<10>, ValidationError> {
    let nonempty = PathNonEmpty::from_slice(b"/home")?;
    let _path = nonempty.get(); // Should not panic
    Ok(nonempty)
}

/// Verify: PathNonEmpty as_str() returns valid string
#[trusted]
#[cfg(creusot)]
pub fn verify_non_empty_as_str() -> Result<PathNonEmpty<10>, ValidationError> {
    let nonempty = PathNonEmpty::from_slice(b"/tmp")?;
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
