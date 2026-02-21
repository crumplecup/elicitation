//! Verus proofs for URL contract types.

use verus_builtin::*;
use verus_builtin_macros::*;
use vstd::prelude::*;

verus! {

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationError {
    UrlParseFailed,
    NotHttps,
    NotHttp,
}

// ============================================================================
// UrlValid - valid URL
// ============================================================================

/// Contract type for valid URLs.
///
/// Abstracts Url::parse() - assume url crate works correctly,
/// verify our wrapper logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlValid {
    pub validated: bool,
}

impl UrlValid {
    /// Creates a UrlValid given parsing result.
    /// 
    /// Parameters abstract the URL parsing:
    /// - parses: result of Url::parse(string).is_ok()
    pub fn new(parses: bool) -> (result: Result<Self, ValidationError>)
        ensures
            parses ==> (result matches Ok(u) && u.validated == true),
            !parses ==> (result matches Err(ValidationError::UrlParseFailed)),
    {
        if parses {
            Ok(UrlValid { validated: true })
        } else {
            Err(ValidationError::UrlParseFailed)
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
// UrlHttps - HTTPS URL
// ============================================================================

/// Contract type for HTTPS URLs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlHttps {
    pub validated: bool,
}

impl UrlHttps {
    /// Creates a UrlHttps given parsing and scheme check.
    /// 
    /// Parameters:
    /// - parses: result of Url::parse(string).is_ok()
    /// - is_https: result of url.scheme() == "https"
    pub fn new(parses: bool, is_https: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::UrlParseFailed)),
            (parses && is_https) ==> (result matches Ok(u) && u.validated == true),
            (parses && !is_https) ==> (result matches Err(ValidationError::NotHttps)),
    {
        if !parses {
            Err(ValidationError::UrlParseFailed)
        } else if is_https {
            Ok(UrlHttps { validated: true })
        } else {
            Err(ValidationError::NotHttps)
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
// UrlHttp - HTTP URL
// ============================================================================

/// Contract type for HTTP URLs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlHttp {
    pub validated: bool,
}

impl UrlHttp {
    /// Creates a UrlHttp given parsing and scheme check.
    pub fn new(parses: bool, is_http: bool) -> (result: Result<Self, ValidationError>)
        ensures
            (!parses) ==> (result matches Err(ValidationError::UrlParseFailed)),
            (parses && is_http) ==> (result matches Ok(u) && u.validated == true),
            (parses && !is_http) ==> (result matches Err(ValidationError::NotHttp)),
    {
        if !parses {
            Err(ValidationError::UrlParseFailed)
        } else if is_http {
            Ok(UrlHttp { validated: true })
        } else {
            Err(ValidationError::NotHttp)
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
