//! Verus proofs for regex byte validation types.
//!
//! Validates regex patterns with syntax checking.
//! Simplified stubs for compositional verification.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidSyntax,
    TooLong { max: usize, actual: usize },
}

// ============================================================================
// RegexBytes - Bounded regex pattern bytes
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegexBytes {
    pub length: usize,
    pub max_len: usize,
    pub validated: bool,
}

impl RegexBytes {
    /// Parameters:
    /// - is_valid_regex: Pattern compiles successfully
    /// - length: Pattern length
    /// - max_len: Maximum allowed length
    pub fn new(is_valid_regex: bool, length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_valid_regex) ==> (result matches Err(ValidationError::InvalidSyntax)),
            (is_valid_regex && length <= max_len) ==> (result matches Ok(r) && r.length == length && r.max_len == max_len && r.validated == true),
            (is_valid_regex && length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if !is_valid_regex {
            Err(ValidationError::InvalidSyntax)
        } else if length <= max_len {
            Ok(RegexBytes { length, max_len, validated: true })
        } else {
            Err(ValidationError::TooLong { max: max_len, actual: length })
        }
    }
}

// ============================================================================
// BalancedDelimiters - Regex with balanced parentheses/brackets
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BalancedDelimiters {
    pub length: usize,
    pub validated: bool,
}

impl BalancedDelimiters {
    /// Parameters:
    /// - is_balanced: All delimiters properly paired
    pub fn new(length: usize, is_balanced: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_balanced ==> (result matches Ok(b) && b.length == length && b.validated == true),
            !is_balanced ==> (result matches Err(ValidationError::InvalidSyntax)),
    {
        if is_balanced {
            Ok(BalancedDelimiters { length, validated: true })
        } else {
            Err(ValidationError::InvalidSyntax)
        }
    }
}

} // verus!
