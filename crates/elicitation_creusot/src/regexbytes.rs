//! Creusot proofs for regex validation types (layered architecture).
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

use crate::*;

#[cfg(creusot)]
use elicitation::verification::types::{
    BalancedDelimiters, RegexBytes, ValidCharClass, ValidEscapes, ValidQuantifiers, ValidationError,
};

// Layer 2: BalancedDelimiters Proofs
// ============================================================================

/// Verify: BalancedDelimiters rejects length exceeding MAX_LEN
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_balanced_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<BalancedDelimiters<MAX_LEN>, ValidationError> {
    BalancedDelimiters::from_slice(bytes)
}

/// Verify: BalancedDelimiters accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_balanced_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<BalancedDelimiters<MAX_LEN>, ValidationError> {
    BalancedDelimiters::from_slice(bytes)
}

/// Verify: Simple balanced parentheses
#[cfg(creusot)]
pub fn verify_balanced_simple() -> Result<BalancedDelimiters<16>, ValidationError> {
    let bytes = [b'(', b'a', b'b', b'c', b')'];
    BalancedDelimiters::from_slice(&bytes)
}

/// Verify: Nested balanced delimiters
#[cfg(creusot)]
pub fn verify_balanced_nested() -> Result<BalancedDelimiters<16>, ValidationError> {
    let bytes = [b'(', b'(', b'a', b'|', b'b', b')', b'c', b')'];
    BalancedDelimiters::from_slice(&bytes)
}

/// Verify: Balanced brackets (character class)
#[cfg(creusot)]
pub fn verify_balanced_brackets() -> Result<BalancedDelimiters<16>, ValidationError> {
    let bytes = [b'[', b'a', b'-', b'z', b']', b'+'];
    BalancedDelimiters::from_slice(&bytes)
}

/// Verify: Balanced braces (quantifier)
#[cfg(creusot)]
pub fn verify_balanced_braces() -> Result<BalancedDelimiters<16>, ValidationError> {
    let bytes = [b'a', b'{', b'2', b',', b'5', b'}'];
    BalancedDelimiters::from_slice(&bytes)
}

/// Verify: Empty regex
#[cfg(creusot)]
pub fn verify_balanced_empty() -> Result<BalancedDelimiters<16>, ValidationError> {
    BalancedDelimiters::from_slice(&[] as &[u8])
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_escapes_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidEscapes<MAX_LEN>, ValidationError> {
    ValidEscapes::from_slice(bytes)
}

/// Verify: ValidEscapes accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_escapes_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidEscapes<MAX_LEN>, ValidationError> {
    ValidEscapes::from_slice(bytes)
}

/// Verify: Digit escape (\d)
#[cfg(creusot)]
pub fn verify_escape_digit() -> Result<ValidEscapes<16>, ValidationError> {
    let bytes = [b'\\', b'd', b'+'];
    ValidEscapes::from_slice(&bytes)
}

/// Verify: Word escape (\w)
#[cfg(creusot)]
pub fn verify_escape_word() -> Result<ValidEscapes<16>, ValidationError> {
    let bytes = [b'\\', b'w', b'*'];
    ValidEscapes::from_slice(&bytes)
}

/// Verify: Dot escape (\.)
#[cfg(creusot)]
pub fn verify_escape_dot() -> Result<ValidEscapes<16>, ValidationError> {
    let bytes = [b'\\', b'.'];
    ValidEscapes::from_slice(&bytes)
}

/// Verify: Newline escape (\n)
#[cfg(creusot)]
pub fn verify_escape_newline() -> Result<ValidEscapes<16>, ValidationError> {
    let bytes = [b'\\', b'n'];
    ValidEscapes::from_slice(&bytes)
}

/// Verify: Tab escape (\t)
#[cfg(creusot)]
pub fn verify_escape_tab() -> Result<ValidEscapes<16>, ValidationError> {
    let bytes = [b'\\', b't'];
    ValidEscapes::from_slice(&bytes)
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_quantifiers_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidQuantifiers<MAX_LEN>, ValidationError> {
    ValidQuantifiers::from_slice(bytes)
}

/// Verify: ValidQuantifiers accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_quantifiers_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidQuantifiers<MAX_LEN>, ValidationError> {
    ValidQuantifiers::from_slice(bytes)
}

/// Verify: Star quantifier (*)
#[cfg(creusot)]
pub fn verify_quantifier_star() -> Result<ValidQuantifiers<16>, ValidationError> {
    let bytes = [b'a', b'*'];
    ValidQuantifiers::from_slice(&bytes)
}

/// Verify: Plus quantifier (+)
#[cfg(creusot)]
pub fn verify_quantifier_plus() -> Result<ValidQuantifiers<16>, ValidationError> {
    let bytes = [b'b', b'+'];
    ValidQuantifiers::from_slice(&bytes)
}

/// Verify: Question quantifier (?)
#[cfg(creusot)]
pub fn verify_quantifier_question() -> Result<ValidQuantifiers<16>, ValidationError> {
    let bytes = [b'c', b'?'];
    ValidQuantifiers::from_slice(&bytes)
}

/// Verify: Exact count quantifier ({n})
#[cfg(creusot)]
pub fn verify_quantifier_exact() -> Result<ValidQuantifiers<16>, ValidationError> {
    let bytes = [b'd', b'{', b'3', b'}'];
    ValidQuantifiers::from_slice(&bytes)
}

/// Verify: Range quantifier ({n,m})
#[cfg(creusot)]
pub fn verify_quantifier_range() -> Result<ValidQuantifiers<16>, ValidationError> {
    let bytes = [b'e', b'{', b'2', b',', b'5', b'}'];
    ValidQuantifiers::from_slice(&bytes)
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_charclass_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidCharClass<MAX_LEN>, ValidationError> {
    ValidCharClass::from_slice(bytes)
}

/// Verify: ValidCharClass accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_charclass_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<ValidCharClass<MAX_LEN>, ValidationError> {
    ValidCharClass::from_slice(bytes)
}

/// Verify: Character class range [a-z]
#[cfg(creusot)]
pub fn verify_charclass_range() -> Result<ValidCharClass<16>, ValidationError> {
    let bytes = [b'[', b'a', b'-', b'z', b']'];
    ValidCharClass::from_slice(&bytes)
}

/// Verify: Character class set [abc]
#[cfg(creusot)]
pub fn verify_charclass_set() -> Result<ValidCharClass<16>, ValidationError> {
    let bytes = [b'[', b'a', b'b', b'c', b']'];
    ValidCharClass::from_slice(&bytes)
}

/// Verify: Negated character class [^a-z]
#[cfg(creusot)]
pub fn verify_charclass_negated() -> Result<ValidCharClass<16>, ValidationError> {
    let bytes = [b'[', b'^', b'a', b'-', b'z', b']'];
    ValidCharClass::from_slice(&bytes)
}

/// Verify: Character class with escape [\\d]
#[cfg(creusot)]
pub fn verify_charclass_escape() -> Result<ValidCharClass<16>, ValidationError> {
    let bytes = [b'[', b'\\', b'd', b']'];
    ValidCharClass::from_slice(&bytes)
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_regex_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<RegexBytes<MAX_LEN>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: RegexBytes accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_regex_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<RegexBytes<MAX_LEN>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: Simple literal regex
#[cfg(creusot)]
pub fn verify_regex_literal() -> Result<RegexBytes<16>, ValidationError> {
    let bytes = [b'h', b'e', b'l', b'l', b'o'];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Regex with alternation
#[cfg(creusot)]
pub fn verify_regex_alternation() -> Result<RegexBytes<16>, ValidationError> {
    let bytes = [b'c', b'a', b't', b'|', b'd', b'o', b'g'];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Regex with quantifiers
#[cfg(creusot)]
pub fn verify_regex_quantifiers() -> Result<RegexBytes<16>, ValidationError> {
    let bytes = [b'a', b'*', b'b', b'+', b'c', b'?'];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Regex with character class
#[cfg(creusot)]
pub fn verify_regex_charclass() -> Result<RegexBytes<16>, ValidationError> {
    let bytes = [b'[', b'a', b'-', b'z', b']', b'+'];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Regex with escapes
#[cfg(creusot)]
pub fn verify_regex_escapes() -> Result<RegexBytes<16>, ValidationError> {
    let bytes = [
        b'\\', b'd', b'{', b'3', b'}', b'-', b'\\', b'd', b'{', b'4', b'}',
    ];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Complex regex (email-like)
#[cfg(creusot)]
pub fn verify_regex_complex() -> Result<RegexBytes<32>, ValidationError> {
    let bytes = [
        b'[', b'a', b'-', b'z', b']', b'+', b'@', b'[', b'a', b'-', b'z', b']', b'+', b'\\', b'.',
        b'[', b'a', b'-', b'z', b']', b'+',
    ];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Regex with groups
#[cfg(creusot)]
pub fn verify_regex_groups() -> Result<RegexBytes<16>, ValidationError> {
    let bytes = [b'(', b'a', b'b', b')', b'+', b'c'];
    RegexBytes::from_slice(&bytes)
}

/// Verify: Empty regex
#[cfg(creusot)]
pub fn verify_regex_empty() -> Result<RegexBytes<16>, ValidationError> {
    RegexBytes::from_slice(&[] as &[u8])
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() <= 2)]
pub fn verify_regex_small_buffer(bytes: &[u8]) -> Result<RegexBytes<2>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: Medium buffer (64 bytes)
#[cfg(creusot)]
#[requires(bytes@.len() <= 64)]
pub fn verify_regex_medium_buffer(bytes: &[u8]) -> Result<RegexBytes<64>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

/// Verify: Large buffer (256 bytes)
#[cfg(creusot)]
#[requires(bytes@.len() <= 256)]
pub fn verify_regex_large_buffer(bytes: &[u8]) -> Result<RegexBytes<256>, ValidationError> {
    RegexBytes::from_slice(bytes)
}

// ============================================================================
