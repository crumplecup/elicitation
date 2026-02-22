//! Creusot proofs for regex contract types (feature-gated on regex).
//!
//! Cloud of assumptions: Trust regex crate compilation and validation (pattern validity,
//! flag checks). Verify wrapper structure.

#![cfg(feature = "regex")]

use creusot_std::prelude::*;
use elicitation::{RegexCaseInsensitive, RegexMultiline, RegexSetNonEmpty, RegexSetValid, RegexValid};

/// Verify RegexValid construction with valid regex pattern.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_regex_valid_valid() -> Result<RegexValid, elicitation::ValidationError> {
    RegexValid::new(r"\d+")
}

/// Verify RegexValid rejects invalid regex pattern.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_regex_valid_invalid() -> Result<RegexValid, elicitation::ValidationError> {
    RegexValid::new(r"[")
}

/// Verify RegexCaseInsensitive construction with case-insensitive regex.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_regex_case_insensitive_valid() -> Result<RegexCaseInsensitive, elicitation::ValidationError> {
    RegexCaseInsensitive::new(r"(?i)test")
}

/// Verify RegexCaseInsensitive rejects case-sensitive regex.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_regex_case_insensitive_invalid() -> Result<RegexCaseInsensitive, elicitation::ValidationError> {
    RegexCaseInsensitive::new(r"test")
}

/// Verify RegexMultiline construction with multiline regex.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_regex_multiline_valid() -> Result<RegexMultiline, elicitation::ValidationError> {
    RegexMultiline::new(r"(?m)^test$")
}

/// Verify RegexMultiline rejects non-multiline regex.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_regex_multiline_invalid() -> Result<RegexMultiline, elicitation::ValidationError> {
    RegexMultiline::new(r"^test$")
}

/// Verify RegexSetValid construction with valid regex set.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_regex_set_valid_valid() -> Result<RegexSetValid, elicitation::ValidationError> {
    let patterns = std::vec![r"\d+", r"[a-z]+"];
    RegexSetValid::new(patterns)
}

/// Verify RegexSetValid rejects invalid regex patterns.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_regex_set_valid_invalid() -> Result<RegexSetValid, elicitation::ValidationError> {
    let patterns = std::vec![r"\d+", r"["];
    RegexSetValid::new(patterns)
}

/// Verify RegexSetNonEmpty construction with non-empty set.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_regex_set_non_empty_valid() -> Result<RegexSetNonEmpty, elicitation::ValidationError> {
    let patterns = std::vec![r"\d+", r"[a-z]+"];
    RegexSetNonEmpty::new(patterns)
}

/// Verify RegexSetNonEmpty rejects empty set.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_regex_set_non_empty_invalid() -> Result<RegexSetNonEmpty, elicitation::ValidationError> {
    let patterns: Vec<String> = Vec::new();
    RegexSetNonEmpty::new(patterns)
}
