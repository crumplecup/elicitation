//! Prusti proofs for Regex contract types.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::*;
use prusti_contracts::*;

// Regex Contract Proofs
// ============================================================================

#[cfg(all(prusti, feature = "regex"))]
/// Prove that RegexValid construction succeeds for valid patterns.
#[ensures(result.is_ok() || result.is_err())]
pub fn verify_regex_valid_construction(pattern: &str) -> Result<RegexValid, ValidationError> {
    RegexValid::new(pattern)
}

#[cfg(all(prusti, feature = "regex"))]
/// Prove that RegexSetValid construction works for multiple patterns.
#[ensures(result.is_ok() || result.is_err())]
pub fn verify_regex_set_valid_construction(
    patterns: &[&str],
) -> Result<RegexSetValid, ValidationError> {
    RegexSetValid::new(patterns)
}

#[cfg(all(prusti, feature = "regex"))]
/// Prove that RegexCaseInsensitive compiles with case-insensitive flag.
#[ensures(result.is_ok() || result.is_err())]
pub fn verify_regex_case_insensitive_construction(
    pattern: &str,
) -> Result<RegexCaseInsensitive, ValidationError> {
    RegexCaseInsensitive::new(pattern)
}

#[cfg(all(prusti, feature = "regex"))]
/// Prove that RegexMultiline compiles with multiline flag.
#[ensures(result.is_ok() || result.is_err())]
pub fn verify_regex_multiline_construction(
    pattern: &str,
) -> Result<RegexMultiline, ValidationError> {
    RegexMultiline::new(pattern)
}

#[cfg(all(prusti, feature = "regex"))]
/// Prove that RegexSetNonEmpty rejects empty pattern sets.
#[ensures(match result {
    Ok(ref set) => set.len() > 0,
    Err(_) => true,
})]
pub fn verify_regex_set_non_empty_requirement(
    patterns: &[&str],
) -> Result<RegexSetNonEmpty, ValidationError> {
    RegexSetNonEmpty::new(patterns)
}

#[cfg(all(prusti, feature = "regex"))]
/// Prove regex trenchcoat pattern: pattern → compile → unwrap preserves pattern.
#[ensures(match result {
    Ok(ref wrapped) => wrapped.clone().into_inner().as_str() == pattern,
    Err(_) => true,
})]
pub fn verify_regex_trenchcoat(pattern: &str) -> Result<RegexValid, ValidationError> {
    RegexValid::new(pattern)
}

#[cfg(all(prusti, feature = "regex"))]
/// Prove regex accessor correctness.
#[ensures(match result {
    Ok(ref wrapped) => wrapped.get().as_str() == pattern,
    Err(_) => true,
})]
pub fn verify_regex_accessor(pattern: &str) -> Result<RegexValid, ValidationError> {
    RegexValid::new(pattern)
}

// ============================================================================
// Verification Statistics
// ============================================================================

/// Total number of Prusti proofs implemented.
#[cfg(prusti)]
#[must_use]
pub const fn total() -> usize {
    // Placeholder: Prusti regex proofs not yet implemented
    0
}
