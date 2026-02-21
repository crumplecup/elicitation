//! Verus proofs for UUID contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    ParseFailed,
    NotV4,
    IsNil,
}

// ============================================================================
// UuidV4 - UUID version 4
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UuidV4 {
    pub validated: bool,
}

impl UuidV4 {
    /// Parameters:
    /// - parses: Uuid::parse_str(string).is_ok()
    /// - is_v4: uuid.get_version_num() == 4
    pub fn new(parses: bool, is_v4: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::ParseFailed)),
            (parses && is_v4) ==> (result matches Ok(u) && u.validated == true),
            (parses && !is_v4) ==> (result matches Err(ValidationError::NotV4)),
    {
        if !parses {
            Err(ValidationError::ParseFailed)
        } else if is_v4 {
            Ok(UuidV4 { validated: true })
        } else {
            Err(ValidationError::NotV4)
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
// UuidNonNil - Non-nil UUID
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UuidNonNil {
    pub validated: bool,
}

impl UuidNonNil {
    /// Parameters:
    /// - parses: Uuid::parse_str(string).is_ok()
    /// - is_nil: uuid.is_nil()
    pub fn new(parses: bool, is_nil: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::ParseFailed)),
            (parses && !is_nil) ==> (result matches Ok(u) && u.validated == true),
            (parses && is_nil) ==> (result matches Err(ValidationError::IsNil)),
    {
        if !parses {
            Err(ValidationError::ParseFailed)
        } else if !is_nil {
            Ok(UuidNonNil { validated: true })
        } else {
            Err(ValidationError::IsNil)
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
