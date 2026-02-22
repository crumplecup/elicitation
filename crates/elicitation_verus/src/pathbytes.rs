//! Verus proofs for path byte validation types.
//!
//! Validates file system paths with existence and type checks.
//! Simplified stubs for compositional verification.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidPath,
    TooLong { max: usize, actual: usize },
    NotAbsolute,
    NotRelative,
}

// ============================================================================
// PathBytes - Bounded path bytes
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathBytes {
    pub length: usize,
    pub max_len: usize,
    pub validated: bool,
}

impl PathBytes {
    /// Parameters:
    /// - is_valid_path: Path bytes are valid UTF-8
    /// - length: Actual path length
    /// - max_len: Maximum allowed length
    pub fn new(is_valid_path: bool, length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_valid_path) ==> (result matches Err(ValidationError::InvalidPath)),
            (is_valid_path && length <= max_len) ==> (result matches Ok(p) && p.length == length && p.max_len == max_len && p.validated == true),
            (is_valid_path && length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if !is_valid_path {
            Err(ValidationError::InvalidPath)
        } else if length <= max_len {
            Ok(PathBytes { length, max_len, validated: true })
        } else {
            Err(ValidationError::TooLong { max: max_len, actual: length })
        }
    }
}

// ============================================================================
// PathAbsolute - Absolute path (starts with /)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathAbsolute {
    pub length: usize,
    pub validated: bool,
}

impl PathAbsolute {
    /// Parameters:
    /// - is_absolute: Path starts with /
    pub fn new(length: usize, is_absolute: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_absolute ==> (result matches Ok(p) && p.length == length && p.validated == true),
            !is_absolute ==> (result matches Err(ValidationError::NotAbsolute)),
    {
        if is_absolute {
            Ok(PathAbsolute { length, validated: true })
        } else {
            Err(ValidationError::NotAbsolute)
        }
    }
}

// ============================================================================
// PathRelative - Relative path (does not start with /)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PathRelative {
    pub length: usize,
    pub validated: bool,
}

impl PathRelative {
    /// Parameters:
    /// - is_relative: Path does not start with /
    pub fn new(length: usize, is_relative: bool) -> (result: Result<Self, ValidationError>)
        ensures
            is_relative ==> (result matches Ok(p) && p.length == length && p.validated == true),
            !is_relative ==> (result matches Err(ValidationError::NotRelative)),
    {
        if is_relative {
            Ok(PathRelative { length, validated: true })
        } else {
            Err(ValidationError::NotRelative)
        }
    }
}

} // verus!
