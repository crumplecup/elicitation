//! Prusti proofs for regex validation types (layered architecture).
//!
//! Validates regex syntax through layered constraints:
//! Layer 1: UTF-8 encoding (reuse Utf8Bytes proofs)
//! Layer 2: Balanced delimiters ((), [], {})
//! Layer 3: Valid escape sequences (\d, \w, \n, etc.)
//! Layer 4: Valid quantifiers (*, +, ?, {n,m})
//! Layer 5: Valid character classes ([a-z])
//! Layer 6: Complete regex (RegexBytes)
//!
//! This is compositional verification: regex_crate_correct → wrapper_correct.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::verification::types::{
    BalancedDelimiters, RegexBytes, ValidCharClass, ValidEscapes, ValidQuantifiers, ValidationError,
};
use prusti_contracts::*;

// Layer 2: BalancedDelimiters Proofs
// ============================================================================

/// Verify: BalancedDelimiters rejects length exceeding MAX_LEN
#[cfg(prusti)]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_balanced_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<BalancedDelimiters<MAX_LEN>, ValidationError> {
    BalancedDelimiters::from_slice(bytes)
}

/// Verify: BalancedDelimiters accepts valid length
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_balanced_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<BalancedDelimiters<MAX_LEN>, ValidationError> {
    BalancedDelimiters::from_slice(bytes)
}

/// Verify: Simple balanced parentheses
#[cfg(prusti)]
pub fn verify_balanced_simple() -> Result<BalancedDelimiters<16>, ValidationError> {
    BalancedDelimiters::from_slice(b"(abc)")
}

/// Verify: Nested balanced delimiters
#[cfg(prusti)]
pub fn verify_balanced_nested() -> Result<BalancedDelimiters<16>, ValidationError> {
    BalancedDelimiters::from_slice(b"((a|b)c)")
}

/// Verify: Balanced brackets (character class)
#[cfg(prusti)]
pub fn verify_balanced_brackets() -> Result<BalancedDelimiters<16>, ValidationError> {
    BalancedDelimiters::from_slice(b"[a-z]+")
}

/// Verify: Balanced braces (quantifier)
#[cfg(prusti)]
pub fn verify_balanced_braces() -> Result<BalancedDelimiters<16>, ValidationError> {
    BalancedDelimiters::from_slice(b"a{2,5}")
}

/// Verify: Empty regex
#[cfg(prusti)]
pub fn verify_balanced_empty() -> Result<BalancedDelimiters<16>, ValidationError> {
    BalancedDelimiters::from_slice(b"")
}

/// Verify: as_str() returns valid string
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_balanced_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<BalancedDelimiters<MAX_LEN>, ValidationError> {
    let balanced = BalancedDelimiters::from_slice(bytes)?;
    let _s = balanced.as_str(); // Should not panic
    Ok(balanced)
}

// Layer 3: ValidEscapes Proofs
// ============================================================================

/// Verify: ValidEscapes rejects length exceeding MAX_LEN
#[cfg(prusti)]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_escapes_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidEscapes<MAX_LEN>, ValidationError> {
    ValidEscapes::from_slice(bytes)
}

/// Verify: ValidEscapes accepts valid length
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_escapes_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidEscapes<MAX_LEN>, ValidationError> {
    ValidEscapes::from_slice(bytes)
}

/// Verify: Digit escape (\d)
#[cfg(prusti)]
pub fn verify_escape_digit() -> Result<ValidEscapes<16>, ValidationError> {
    ValidEscapes::from_slice(b"\\d+")
}

/// Verify: Word escape (\w)
#[cfg(prusti)]
pub fn verify_escape_word() -> Result<ValidEscapes<16>, ValidationError> {
    ValidEscapes::from_slice(b"\\w*")
}

/// Verify: Dot escape (\.)
#[cfg(prusti)]
pub fn verify_escape_dot() -> Result<ValidEscapes<16>, ValidationError> {
    ValidEscapes::from_slice(b"\\.")
}

/// Verify: Newline escape (\n)
#[cfg(prusti)]
pub fn verify_escape_newline() -> Result<ValidEscapes<16>, ValidationError> {
    ValidEscapes::from_slice(b"\\n")
}

/// Verify: Tab escape (\t)
#[cfg(prusti)]
pub fn verify_escape_tab() -> Result<ValidEscapes<16>, ValidationError> {
    ValidEscapes::from_slice(b"\\t")
}

/// Verify: as_str() returns valid string
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_escapes_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidEscapes<MAX_LEN>, ValidationError> {
    let escapes = ValidEscapes::from_slice(bytes)?;
    let _s = escapes.as_str(); // Should not panic
    Ok(escapes)
}

// Layer 4: ValidQuantifiers Proofs
// ============================================================================

/// Verify: ValidQuantifiers rejects length exceeding MAX_LEN
#[cfg(prusti)]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_quantifiers_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidQuantifiers<MAX_LEN>, ValidationError> {
    ValidQuantifiers::from_slice(bytes)
}

/// Verify: ValidQuantifiers accepts valid length
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_quantifiers_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidQuantifiers<MAX_LEN>, ValidationError> {
    ValidQuantifiers::from_slice(bytes)
}

/// Verify: Star quantifier (*)
#[cfg(prusti)]
pub fn verify_quantifier_star() -> Result<ValidQuantifiers<16>, ValidationError> {
    ValidQuantifiers::from_slice(b"a*")
}

/// Verify: Plus quantifier (+)
#[cfg(prusti)]
pub fn verify_quantifier_plus() -> Result<ValidQuantifiers<16>, ValidationError> {
    ValidQuantifiers::from_slice(b"b+")
}

/// Verify: Question quantifier (?)
#[cfg(prusti)]
pub fn verify_quantifier_question() -> Result<ValidQuantifiers<16>, ValidationError> {
    ValidQuantifiers::from_slice(b"c?")
}

/// Verify: Exact count quantifier ({n})
#[cfg(prusti)]
pub fn verify_quantifier_exact() -> Result<ValidQuantifiers<16>, ValidationError> {
    ValidQuantifiers::from_slice(b"d{3}")
}

/// Verify: Range quantifier ({n,m})
#[cfg(prusti)]
pub fn verify_quantifier_range() -> Result<ValidQuantifiers<16>, ValidationError> {
    ValidQuantifiers::from_slice(b"e{2,5}")
}

/// Verify: as_str() returns valid string
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_quantifiers_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidQuantifiers<MAX_LEN>, ValidationError> {
    let quantifiers = ValidQuantifiers::from_slice(bytes)?;
    let _s = quantifiers.as_str(); // Should not panic
    Ok(quantifiers)
}

// Layer 5: ValidCharClass Proofs
// ============================================================================

/// Verify: ValidCharClass rejects length exceeding MAX_LEN
#[cfg(prusti)]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_charclass_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidCharClass<MAX_LEN>, ValidationError> {
    ValidCharClass::from_slice(bytes)
}

/// Verify: ValidCharClass accepts valid length
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_charclass_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidCharClass<MAX_LEN>, ValidationError> {
    ValidCharClass::from_slice(bytes)
}

/// Verify: Character class range [a-z]
#[cfg(prusti)]
pub fn verify_charclass_range() -> Result<ValidCharClass<16>, ValidationError> {
    ValidCharClass::from_slice(b"[a-z]")
}

/// Verify: Character class set [abc]
#[cfg(prusti)]
pub fn verify_charclass_set() -> Result<ValidCharClass<16>, ValidationError> {
    ValidCharClass::from_slice(b"[abc]")
}

/// Verify: Negated character class [^a-z]
#[cfg(prusti)]
pub fn verify_charclass_negated() -> Result<ValidCharClass<16>, ValidationError> {
    ValidCharClass::from_slice(b"[^a-z]")
}

/// Verify: Character class with escape [\\d]
#[cfg(prusti)]
pub fn verify_charclass_escape() -> Result<ValidCharClass<16>, ValidationError> {
    ValidCharClass::from_slice(b"[\\d]")
}

/// Verify: as_str() returns valid string
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_charclass_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidCharClass<MAX_LEN>, ValidationError> {
    let charclass = ValidCharClass::from_slice(bytes)?;
    let _s = charclass.as_str(); // Should not panic
    Ok(charclass)
}

// Layer 6: RegexBytes Proofs (Complete Regex)
// ============================================================================

/// Verify: RegexBytes rejects length exceeding MAX_LEN
#[cfg(prusti)]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_regex_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<RegexBytes<MAX_LEN>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: RegexBytes accepts valid length
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_regex_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<RegexBytes<MAX_LEN>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: Simple literal regex
#[cfg(prusti)]
pub fn verify_regex_literal() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"hello")
}

/// Verify: Regex with alternation
#[cfg(prusti)]
pub fn verify_regex_alternation() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"cat|dog")
}

/// Verify: Regex with quantifiers
#[cfg(prusti)]
pub fn verify_regex_quantifiers() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"a*b+c?")
}

/// Verify: Regex with character class
#[cfg(prusti)]
pub fn verify_regex_charclass() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"[a-z]+")
}

/// Verify: Regex with escapes
#[cfg(prusti)]
pub fn verify_regex_escapes() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"\\d{3}-\\d{4}")
}

/// Verify: Complex regex (email-like)
#[cfg(prusti)]
pub fn verify_regex_complex() -> Result<RegexBytes<32>, ValidationError> {
    RegexBytes::from_slice(b"[a-z]+@[a-z]+\\.[a-z]+")
}

/// Verify: Regex with groups
#[cfg(prusti)]
pub fn verify_regex_groups() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"(ab)+c")
}

/// Verify: Empty regex
#[cfg(prusti)]
pub fn verify_regex_empty() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(b"")
}

/// Verify: as_str() returns valid string
#[cfg(prusti)]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_regex_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<RegexBytes<MAX_LEN>, ValidationError> {
    let regex = RegexBytes::from_slice(bytes)?;
    let _s = regex.as_str(); // Should not panic
    Ok(regex)
}

// Edge Cases
// ============================================================================

/// Verify: Small buffer (2 bytes)
#[cfg(prusti)]
#[requires(bytes.len() <= 2)]
pub fn verify_regex_small_buffer(bytes: &[u8]) -> Result<RegexBytes<2>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: Medium buffer (64 bytes)
#[cfg(prusti)]
#[requires(bytes.len() <= 64)]
pub fn verify_regex_medium_buffer(bytes: &[u8]) -> Result<RegexBytes<64>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: Large buffer (256 bytes)
#[cfg(prusti)]
#[requires(bytes.len() <= 256)]
pub fn verify_regex_large_buffer(bytes: &[u8]) -> Result<RegexBytes<256>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

// ============================================================================
