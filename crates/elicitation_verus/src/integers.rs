use verus_builtin_macros::verus;
// Required by verus! macro for comparison operators (<=, >, etc.)
// Cargo cannot detect this usage as it occurs during macro expansion
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotPositiveI8(i8),
    NegativeI8(i8),
    NotPositiveI16(i16),
    NegativeI16(i16),
    NotPositiveU8(u8),
    NotPositiveU16(u16),
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
            value <= 0 ==> (result matches Err(ValidationError::NotPositiveI8(v)) && v == value),
    {
        if value > 0 {
            Ok(I8Positive { value })
        } else {
            Err(ValidationError::NotPositiveI8(value))
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
            value < 0 ==> (result matches Err(ValidationError::NegativeI8(v)) && v == value),
    {
        if value >= 0 {
            Ok(I8NonNegative { value })
        } else {
            Err(ValidationError::NegativeI8(value))
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

// ============================================================================
// I16 Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I16Positive {
    pub value: i16,
}

impl I16Positive {
    pub fn new(value: i16) -> (result: Result<Self, ValidationError>)
        ensures
            value > 0 ==> (result matches Ok(p) && p.value == value),
            value <= 0 ==> (result matches Err(ValidationError::NotPositiveI16(v)) && v == value),
    {
        if value > 0 {
            Ok(I16Positive { value })
        } else {
            Err(ValidationError::NotPositiveI16(value))
        }
    }

    pub fn get(&self) -> (result: i16)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: i16)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I16NonNegative {
    pub value: i16,
}

impl I16NonNegative {
    pub fn new(value: i16) -> (result: Result<Self, ValidationError>)
        ensures
            value >= 0 ==> (result matches Ok(nn) && nn.value == value),
            value < 0 ==> (result matches Err(ValidationError::NegativeI16(v)) && v == value),
    {
        if value >= 0 {
            Ok(I16NonNegative { value })
        } else {
            Err(ValidationError::NegativeI16(value))
        }
    }

    pub fn get(&self) -> (result: i16)
        requires self.value >= 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: i16)
        requires self.value >= 0,
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I16NonZero {
    pub value: i16,
}

impl I16NonZero {
    pub fn new(value: i16) -> (result: Result<Self, ValidationError>)
        ensures
            value != 0 ==> (result matches Ok(nz) && nz.value == value),
            value == 0 ==> (result matches Err(ValidationError::Zero)),
    {
        if value != 0 {
            Ok(I16NonZero { value })
        } else {
            Err(ValidationError::Zero)
        }
    }

    pub fn get(&self) -> (result: i16)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: i16)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// U8 Types (unsigned)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U8Positive {
    pub value: u8,
}

impl U8Positive {
    pub fn new(value: u8) -> (result: Result<Self, ValidationError>)
        ensures
            value > 0 ==> (result matches Ok(p) && p.value == value),
            value == 0 ==> (result matches Err(ValidationError::NotPositiveU8(v)) && v == value),
    {
        if value > 0 {
            Ok(U8Positive { value })
        } else {
            Err(ValidationError::NotPositiveU8(value))
        }
    }

    pub fn get(&self) -> (result: u8)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: u8)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U8NonZero {
    pub value: u8,
}

impl U8NonZero {
    pub fn new(value: u8) -> (result: Result<Self, ValidationError>)
        ensures
            value != 0 ==> (result matches Ok(nz) && nz.value == value),
            value == 0 ==> (result matches Err(ValidationError::Zero)),
    {
        if value != 0 {
            Ok(U8NonZero { value })
        } else {
            Err(ValidationError::Zero)
        }
    }

    pub fn get(&self) -> (result: u8)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: u8)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// U16 Types (unsigned)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U16Positive {
    pub value: u16,
}

impl U16Positive {
    pub fn new(value: u16) -> (result: Result<Self, ValidationError>)
        ensures
            value > 0 ==> (result matches Ok(p) && p.value == value),
            value == 0 ==> (result matches Err(ValidationError::NotPositiveU16(v)) && v == value),
    {
        if value > 0 {
            Ok(U16Positive { value })
        } else {
            Err(ValidationError::NotPositiveU16(value))
        }
    }

    pub fn get(&self) -> (result: u16)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: u16)
        requires self.value > 0,
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U16NonZero {
    pub value: u16,
}

impl U16NonZero {
    pub fn new(value: u16) -> (result: Result<Self, ValidationError>)
        ensures
            value != 0 ==> (result matches Ok(nz) && nz.value == value),
            value == 0 ==> (result matches Err(ValidationError::Zero)),
    {
        if value != 0 {
            Ok(U16NonZero { value })
        } else {
            Err(ValidationError::Zero)
        }
    }

    pub fn get(&self) -> (result: u16)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: u16)
        requires self.value != 0,
        ensures result == self.value,
    {
        self.value
    }
}

} // verus!
