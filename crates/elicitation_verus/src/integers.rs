//! Verus proofs for integer contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotPositive(i8),
    Negative(i8),
    Zero,
}

// ============================================================================
// I8Positive (> 0)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I8Positive {
    pub value: i8,
}

impl I8Positive {
    /// Creates an I8Positive from a value.
    /// Returns Ok only if value > 0.
    pub fn new(value: i8) -> (result: Result<Self, ValidationError>)
        ensures
            value > 0 ==> (result matches Ok(p) && p.value == value),
            value <= 0 ==> (result matches Err(ValidationError::NotPositive(v)) && v == value),
    {
        if value > 0 {
            Ok(I8Positive { value })
        } else {
            Err(ValidationError::NotPositive(value))
        }
    }

    /// Gets the wrapped value, which must be positive.
    pub fn get(&self) -> (result: i8)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }

    /// Unwraps to i8 (trenchcoat off).
    pub fn into_inner(self) -> (result: i8)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// I8NonNegative (>= 0)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I8NonNegative {
    pub value: i8,
}

impl I8NonNegative {
    /// Creates an I8NonNegative from a value.
    /// Returns Ok only if value >= 0.
    pub fn new(value: i8) -> (result: Result<Self, ValidationError>)
        ensures
            value >= 0 ==> (result matches Ok(nn) && nn.value == value),
            value < 0 ==> (result matches Err(ValidationError::Negative(v)) && v == value),
    {
        if value >= 0 {
            Ok(I8NonNegative { value })
        } else {
            Err(ValidationError::Negative(value))
        }
    }

    /// Gets the wrapped value, which must be non-negative.
    pub fn get(&self) -> (result: i8)
        requires self.value >= 0,
        ensures result == self.value,
    {
        self.value
    }

    /// Unwraps to i8 (trenchcoat off).
    pub fn into_inner(self) -> (result: i8)
        requires self.value >= 0,
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// I8NonZero (!= 0)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I8NonZero {
    pub value: i8,
}

impl I8NonZero {
    /// Creates an I8NonZero from a value.
    /// Returns Ok only if value != 0.
    pub fn new(value: i8) -> (result: Result<Self, ValidationError>)
        ensures
            value != 0 ==> (result matches Ok(nz) && nz.value == value),
            value == 0 ==> (result matches Err(ValidationError::Zero)),
    {
        if value != 0 {
            Ok(I8NonZero { value })
        } else {
            Err(ValidationError::Zero)
        }
    }

    /// Gets the wrapped value, which must be non-zero.
    pub fn get(&self) -> (result: i8)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }

    /// Unwraps to i8 (trenchcoat off).
    pub fn into_inner(self) -> (result: i8)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }
}

} // verus!
