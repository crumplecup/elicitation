//! Verus proofs for string contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    EmptyString,
    TooLong { max: usize, actual: usize },
}

// ============================================================================
// StringNonEmpty - non-empty string validation
// ============================================================================

/// Contract type for non-empty strings.
///
/// For Verus, we abstract the string and its length check.
/// We verify wrapper logic, not stdlib String implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringNonEmpty {
    // We don't store the string itself - just track that validation passed
    // In real code, this would hold a String, but Verus doesn't have specs for it
    pub validated: bool,
}

impl StringNonEmpty {
    /// Creates a StringNonEmpty given validation results.
    ///
    /// Parameters abstract the string properties we'd check:
    /// - is_empty: result of string.is_empty()
    ///
    /// This verifies our wrapper logic is correct.
    pub fn new(is_empty: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_empty) ==> (result matches Ok(s) && s.validated == true),
            is_empty ==> (result matches Err(ValidationError::EmptyString)),
    {
        if !is_empty {
            Ok(StringNonEmpty { validated: true })
        } else {
            Err(ValidationError::EmptyString)
        }
    }

    /// Check if validated (always true for valid instances).
    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

// ============================================================================
// StringBounded - bounded length string
// ============================================================================

/// Contract type for strings with maximum length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringBounded {
    pub validated: bool,
    pub max_len: usize,
}

impl StringBounded {
    /// Creates a StringBounded given length and max.
    ///
    /// Parameters abstract the string properties:
    /// - length: result of string.len()
    /// - max_len: maximum allowed length
    ///
    /// Verifies: length <= max_len check is correct.
    pub fn new(length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            length <= max_len ==> (result matches Ok(s) && s.validated == true && s.max_len == max_len),
            length > max_len ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if length <= max_len {
            Ok(StringBounded { validated: true, max_len })
        } else {
            Err(ValidationError::TooLong {
                max: max_len,
                actual: length,
            })
        }
    }

    /// Check if validated (always true for valid instances).
    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

// ============================================================================
// StringNonEmptyBounded - combined constraints
// ============================================================================

/// Contract type for non-empty AND bounded strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringNonEmptyBounded {
    pub validated: bool,
    pub max_len: usize,
}

impl StringNonEmptyBounded {
    /// Creates a StringNonEmptyBounded given validation results.
    ///
    /// Parameters abstract the string properties:
    /// - length: result of string.len()
    /// - max_len: maximum allowed length
    ///
    /// Verifies: both non-empty AND bounded checks are correct.
    pub fn new(length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (length > 0 && length <= max_len) ==> (result matches Ok(s) && s.validated == true && s.max_len == max_len),
            (length == 0) ==> (result matches Err(ValidationError::EmptyString)),
            (length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if length == 0 {
            Err(ValidationError::EmptyString)
        } else if length > max_len {
            Err(ValidationError::TooLong {
                max: max_len,
                actual: length,
            })
        } else {
            Ok(StringNonEmptyBounded { validated: true, max_len })
        }
    }

    /// Check if validated (always true for valid instances).
    pub fn is_validated(&self) -> (result: bool)
        requires self.validated == true,
        ensures result == true,
    {
        self.validated
    }
}

} // verus!
