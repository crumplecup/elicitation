//! Verus proofs for float contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    NotFinite,
    FloatNotPositive,
    FloatNegative,
}

// ============================================================================
// F32 Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32Positive {
    pub value: f32,
}

impl F32Positive {
    /// Creates an F32Positive from a value.
    /// 
    /// For Verus, we abstract is_finite() and > 0.0 checks as parameters
    /// since we can't verify float IEEE 754 properties directly.
    pub fn new(value: f32, is_finite: bool, is_positive: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_finite) ==> (result matches Err(ValidationError::NotFinite)),
            (is_finite && is_positive) ==> (result matches Ok(p) && p.value == value),
            (is_finite && !is_positive) ==> (result matches Err(ValidationError::FloatNotPositive)),
    {
        if !is_finite {
            Err(ValidationError::NotFinite)
        } else if is_positive {
            Ok(F32Positive { value })
        } else {
            Err(ValidationError::FloatNotPositive)
        }
    }

    pub fn get(&self) -> (result: f32)
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: f32)
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32NonNegative {
    pub value: f32,
}

impl F32NonNegative {
    pub fn new(value: f32, is_finite: bool, is_non_negative: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_finite) ==> (result matches Err(ValidationError::NotFinite)),
            (is_finite && is_non_negative) ==> (result matches Ok(nn) && nn.value == value),
            (is_finite && !is_non_negative) ==> (result matches Err(ValidationError::FloatNegative)),
    {
        if !is_finite {
            Err(ValidationError::NotFinite)
        } else if is_non_negative {
            Ok(F32NonNegative { value })
        } else {
            Err(ValidationError::FloatNegative)
        }
    }

    pub fn get(&self) -> (result: f32)
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: f32)
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32Finite {
    pub value: f32,
}

impl F32Finite {
    pub fn new(value: f32, is_finite: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_finite ==> (result matches Ok(f) && f.value == value),
            !is_finite ==> (result matches Err(ValidationError::NotFinite)),
    {
        if is_finite {
            Ok(F32Finite { value })
        } else {
            Err(ValidationError::NotFinite)
        }
    }

    pub fn get(&self) -> (result: f32)
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: f32)
        ensures result == self.value,
    {
        self.value
    }
}

// ============================================================================
// F64 Types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64Positive {
    pub value: f64,
}

impl F64Positive {
    pub fn new(value: f64, is_finite: bool, is_positive: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_finite) ==> (result matches Err(ValidationError::NotFinite)),
            (is_finite && is_positive) ==> (result matches Ok(p) && p.value == value),
            (is_finite && !is_positive) ==> (result matches Err(ValidationError::FloatNotPositive)),
    {
        if !is_finite {
            Err(ValidationError::NotFinite)
        } else if is_positive {
            Ok(F64Positive { value })
        } else {
            Err(ValidationError::FloatNotPositive)
        }
    }

    pub fn get(&self) -> (result: f64)
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: f64)
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64NonNegative {
    pub value: f64,
}

impl F64NonNegative {
    pub fn new(value: f64, is_finite: bool, is_non_negative: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_finite) ==> (result matches Err(ValidationError::NotFinite)),
            (is_finite && is_non_negative) ==> (result matches Ok(nn) && nn.value == value),
            (is_finite && !is_non_negative) ==> (result matches Err(ValidationError::FloatNegative)),
    {
        if !is_finite {
            Err(ValidationError::NotFinite)
        } else if is_non_negative {
            Ok(F64NonNegative { value })
        } else {
            Err(ValidationError::FloatNegative)
        }
    }

    pub fn get(&self) -> (result: f64)
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: f64)
        ensures result == self.value,
    {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64Finite {
    pub value: f64,
}

impl F64Finite {
    pub fn new(value: f64, is_finite: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_finite ==> (result matches Ok(f) && f.value == value),
            !is_finite ==> (result matches Err(ValidationError::NotFinite)),
    {
        if is_finite {
            Ok(F64Finite { value })
        } else {
            Err(ValidationError::NotFinite)
        }
    }

    pub fn get(&self) -> (result: f64)
        ensures result == self.value,
    {
        self.value
    }

    pub fn into_inner(self) -> (result: f64)
        ensures result == self.value,
    {
        self.value
    }
}

} // verus!
