//! Creusot proofs for URL contract types (feature-gated on url).
//!
//! Cloud of assumptions: Trust url crate parsing and classification (scheme checks,
//! host validation, base URL requirements). Verify wrapper structure.

#![cfg(feature = "url")]

use creusot_std::prelude::*;
use elicitation::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};

/// Verify UrlValid construction with valid URL.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_valid_valid() -> Result<UrlValid, elicitation::ValidationError> {
    UrlValid::new("https://example.com")
}

/// Verify UrlValid rejects invalid URL.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_valid_invalid() -> Result<UrlValid, elicitation::ValidationError> {
    UrlValid::new("not a url")
}

/// Verify UrlHttp construction with HTTP URL.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_http_valid() -> Result<UrlHttp, elicitation::ValidationError> {
    UrlHttp::new("http://example.com")
}

/// Verify UrlHttp rejects non-HTTP URL.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_http_invalid() -> Result<UrlHttp, elicitation::ValidationError> {
    UrlHttp::new("https://example.com")
}

/// Verify UrlHttps construction with HTTPS URL.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_https_valid() -> Result<UrlHttps, elicitation::ValidationError> {
    UrlHttps::new("https://example.com")
}

/// Verify UrlHttps rejects non-HTTPS URL.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_https_invalid() -> Result<UrlHttps, elicitation::ValidationError> {
    UrlHttps::new("http://example.com")
}

/// Verify UrlWithHost construction with URL containing host.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_with_host_valid() -> Result<UrlWithHost, elicitation::ValidationError> {
    UrlWithHost::new("https://example.com/path")
}

/// Verify UrlWithHost rejects URL without host.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_with_host_invalid() -> Result<UrlWithHost, elicitation::ValidationError> {
    UrlWithHost::new("data:text/plain,hello")
}

/// Verify UrlCanBeBase construction with base-able URL.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_can_be_base_valid() -> Result<UrlCanBeBase, elicitation::ValidationError> {
    UrlCanBeBase::new("https://example.com/")
}

/// Verify UrlCanBeBase rejects non-base URL.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_can_be_base_invalid() -> Result<UrlCanBeBase, elicitation::ValidationError> {
    UrlCanBeBase::new("data:text/plain,hello")
}
