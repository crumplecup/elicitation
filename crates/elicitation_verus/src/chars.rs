//! Verus proofs for char contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotAlphabetic(char),
    NotNumeric(char),
    NotAlphanumeric(char),
}

// ============================================================================
// CharAlphabetic
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharAlphabetic {
    pub value: char,
}

impl CharAlphabetic {
    /// Creates a CharAlphabetic from a char and validation result.
    ///
    /// For Verus, we abstract the is_alphabetic() check as a parameter
    /// (similar to Kani approach) since we can't verify Unicode tables.
    /// This verifies the wrapper logic is correct.
    pub fn new(value: char, is_alphabetic: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_alphabetic == true ==> (result matches Ok(ca) && ca.value == value),
            is_alphabetic == false ==> (result matches Err(ValidationError::NotAlphabetic(v)) && v == value),
    {
        if is_alphabetic {
            Ok(CharAlphabetic { value })
        } else {
            Err(ValidationError::NotAlphabetic(value))
        }
    }

    /// Gets the wrapped value, which must be alphabetic.
    pub fn get(&self) -> (result: char)
        ensures result == self.value,
    {
        self.value
    }

    /// Unwraps to char (trenchcoat off).
    pub fn into_inner(self) -> (result: char)
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// CharNumeric
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharNumeric {
    pub value: char,
}

impl CharNumeric {
    /// Creates a CharNumeric from a char and validation result.
    pub fn new(value: char, is_numeric: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_numeric == true ==> (result matches Ok(cn) && cn.value == value),
            is_numeric == false ==> (result matches Err(ValidationError::NotNumeric(v)) && v == value),
    {
        if is_numeric {
            Ok(CharNumeric { value })
        } else {
            Err(ValidationError::NotNumeric(value))
        }
    }

    /// Gets the wrapped value, which must be numeric.
    pub fn get(&self) -> (result: char)
        ensures result == self.value,
    {
        self.value
    }

    /// Unwraps to char (trenchcoat off).
    pub fn into_inner(self) -> (result: char)
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// CharAlphanumeric
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharAlphanumeric {
    pub value: char,
}

impl CharAlphanumeric {
    /// Creates a CharAlphanumeric from a char and validation result.
    pub fn new(value: char, is_alphanumeric: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_alphanumeric == true ==> (result matches Ok(can) && can.value == value),
            is_alphanumeric == false ==> (result matches Err(ValidationError::NotAlphanumeric(v)) && v == value),
    {
        if is_alphanumeric {
            Ok(CharAlphanumeric { value })
        } else {
            Err(ValidationError::NotAlphanumeric(value))
        }
    }

    /// Gets the wrapped value, which must be alphanumeric.
    pub fn get(&self) -> (result: char)
        ensures result == self.value,
    {
        self.value
    }

    /// Unwraps to char (trenchcoat off).
    pub fn into_inner(self) -> (result: char)
        ensures result == self.value,
    {
        self.value
    }
}

} // verus!
