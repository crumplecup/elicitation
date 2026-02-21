//! Verus proofs for boolean contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotTrue,
    NotFalse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolTrue {
    pub value: bool,
}

impl BoolTrue {
    /// Creates a BoolTrue from a boolean value.
    /// Returns Ok only if value is true.
    pub fn new(value: bool) -> (result: Result<Self, ValidationError>)
        ensures
            value == true ==> (result matches Ok(bt) && bt.value == true),
            value == false ==> (result matches Err(_)),
    {
        if value {
            Ok(BoolTrue { value })
        } else {
            Err(ValidationError::NotTrue)
        }
    }

    /// Gets the value, which must be true.
    pub fn get(&self) -> (result: bool)
        requires self.value == true,
        ensures result == true,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoolFalse {
    pub value: bool,
}

impl BoolFalse {
    /// Creates a BoolFalse from a boolean value.
    /// Returns Ok only if value is false.
    pub fn new(value: bool) -> (result: Result<Self, ValidationError>)
        ensures
            value == false ==> (result matches Ok(bf) && bf.value == false),
            value == true ==> (result matches Err(_)),
    {
        if !value {
            Ok(BoolFalse { value })
        } else {
            Err(ValidationError::NotFalse)
        }
    }

    /// Gets the value, which must be false.
    pub fn get(&self) -> (result: bool)
        requires self.value == false,
        ensures result == false,
    {
        self.value
    }
}

} // verus!
