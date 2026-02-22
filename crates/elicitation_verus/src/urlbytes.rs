//! Verus proofs for URL byte validation types (RFC 3986).
//!
//! Validates URL syntax through component validation.
//! Simplified stubs for compositional verification.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    InvalidSyntax,
    TooLong { max: usize, actual: usize },
    MissingScheme,
    MissingAuthority,
}

// ============================================================================
// UrlBytes - Complete URL with bounded components
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlBytes {
    pub length: usize,
    pub max_len: usize,
    pub validated: bool,
}

impl UrlBytes {
    /// Parameters:
    /// - is_valid_url: URL parses successfully
    /// - length: URL length
    /// - max_len: Maximum allowed length
    pub fn new(is_valid_url: bool, length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_valid_url) ==> (result matches Err(ValidationError::InvalidSyntax)),
            (is_valid_url && length <= max_len) ==> (result matches Ok(u) && u.length == length && u.max_len == max_len && u.validated == true),
            (is_valid_url && length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if !is_valid_url {
            Err(ValidationError::InvalidSyntax)
        } else if length <= max_len {
            Ok(UrlBytes { length, max_len, validated: true })
        } else {
            Err(ValidationError::TooLong { max: max_len, actual: length })
        }
    }
}

// ============================================================================
// SchemeBytes - URL scheme (http, https, ftp, etc.)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemeBytes {
    pub length: usize,
    pub max_len: usize,
    pub validated: bool,
}

impl SchemeBytes {
    /// Parameters:
    /// - is_valid_scheme: Scheme matches RFC 3986 grammar
    pub fn new(is_valid_scheme: bool, length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_valid_scheme) ==> (result matches Err(ValidationError::InvalidSyntax)),
            (is_valid_scheme && length <= max_len) ==> (result matches Ok(s) && s.length == length && s.max_len == max_len && s.validated == true),
            (is_valid_scheme && length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if !is_valid_scheme {
            Err(ValidationError::InvalidSyntax)
        } else if length <= max_len {
            Ok(SchemeBytes { length, max_len, validated: true })
        } else {
            Err(ValidationError::TooLong { max: max_len, actual: length })
        }
    }
}

// ============================================================================
// AuthorityBytes - URL authority (example.com:8080)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthorityBytes {
    pub length: usize,
    pub max_len: usize,
    pub validated: bool,
}

impl AuthorityBytes {
    /// Parameters:
    /// - is_valid_authority: Authority matches RFC 3986 grammar
    pub fn new(is_valid_authority: bool, length: usize, max_len: usize) -> (result: Result<Self, ValidationError>)
        ensures
            (!is_valid_authority) ==> (result matches Err(ValidationError::InvalidSyntax)),
            (is_valid_authority && length <= max_len) ==> (result matches Ok(a) && a.length == length && a.max_len == max_len && a.validated == true),
            (is_valid_authority && length > max_len) ==> (result matches Err(ValidationError::TooLong { max, actual }) && max == max_len && actual == length),
    {
        if !is_valid_authority {
            Err(ValidationError::InvalidSyntax)
        } else if length <= max_len {
            Ok(AuthorityBytes { length, max_len, validated: true })
        } else {
            Err(ValidationError::TooLong { max: max_len, actual: length })
        }
    }
}

} // verus!
