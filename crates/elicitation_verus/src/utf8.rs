//! Verus proofs for UTF-8 byte contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidUtf8,
    TooLong { max: usize, actual: usize },
}

// ============================================================================
// Utf8Valid - valid UTF-8 bytes
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Valid {
    pub validated: bool,
}

impl Utf8Valid {
    /// Parameters:
    /// - is_valid_utf8: std::str::from_utf8(bytes).is_ok()
    pub fn new(is_valid_utf8: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_valid_utf8 ==> (result matches Ok(u) && u.validated == true),
            !is_valid_utf8 ==> (result matches Err(ValidationError::InvalidUtf8)),
    {
        if is_valid_utf8 {
            Ok(Utf8Valid { validated: true })
        } else {
            Err(ValidationError::InvalidUtf8)
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

// ============================================================================
// Utf8Bounded - UTF-8 with max length
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Utf8Bounded {
    pub validated: bool,
    pub max_len: usize,
}

impl Utf8Bounded {
    /// Parameters:
    /// - is_valid_utf8: std::str::from_utf8(bytes).is_ok()
    /// - length: bytes.len()
    /// - max_len: maximum allowed length
    pub fn new(is_valid_utf8: bool, length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_valid_utf8) ==> (result matches Err(ValidationError::InvalidUtf8)),
            (is_valid_utf8 && length <= max_len) ==> (result matches Ok(u) && u.validated == true && u.max_len == max_len),
            (is_valid_utf8 && length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if !is_valid_utf8 {
            Err(ValidationError::InvalidUtf8)
        } else if length <= max_len {
            Ok(Utf8Bounded { validated: true, max_len })
        } else {
            Err(ValidationError::TooLong { max: max_len, actual: length })
        }
    }

    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

} // verus!
