//! Verus proofs for path contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    EmptyPath,
    NotAbsolute,
    NotRelative,
}

// ============================================================================
// PathNonEmpty - non-empty path
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathNonEmpty {
    pub validated: bool,
}

impl PathNonEmpty {
    /// Parameters:
    /// - is_empty: path.as_os_str().is_empty()
    pub fn new(is_empty: bool) -> (result: Result<Self, ValidationError>)
        ensures
            !is_empty ==> (result matches Ok(p) && p.validated == true),
            is_empty ==> (result matches Err(ValidationError::EmptyPath)),
    {
        if !is_empty {
            Ok(PathNonEmpty { validated: true })
        } else {
            Err(ValidationError::EmptyPath)
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
// PathAbsolute - absolute path
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathAbsolute {
    pub validated: bool,
}

impl PathAbsolute {
    /// Parameters:
    /// - is_absolute: path.is_absolute()
    pub fn new(is_absolute: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_absolute ==> (result matches Ok(p) && p.validated == true),
            !is_absolute ==> (result matches Err(ValidationError::NotAbsolute)),
    {
        if is_absolute {
            Ok(PathAbsolute { validated: true })
        } else {
            Err(ValidationError::NotAbsolute)
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
// PathRelative - relative path
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathRelative {
    pub validated: bool,
}

impl PathRelative {
    /// Parameters:
    /// - is_relative: path.is_relative()
    pub fn new(is_relative: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_relative ==> (result matches Ok(p) && p.validated == true),
            !is_relative ==> (result matches Err(ValidationError::NotRelative)),
    {
        if is_relative {
            Ok(PathRelative { validated: true })
        } else {
            Err(ValidationError::NotRelative)
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
