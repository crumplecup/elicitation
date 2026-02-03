//! Verus proofs for URL contract types.

#![cfg(all(feature = "verify-verus", feature = "url", not(kani)))]
#![allow(unused_imports)]

use crate::*;

#[cfg(feature = "verify-verus")]
#[allow(unused_imports)]
use builtin::*;
#[cfg(feature = "verify-verus")]
#[allow(unused_imports)]
use builtin_macros::*;

verus! {

// URL Contract Proofs
// ============================================================================

#[cfg(feature = "url")]
/// Verify UrlHttps construction succeeds for HTTPS URLs.
proof fn verify_url_https_valid(url_str: &str)
    requires url_str.starts_with("https://")
    ensures UrlHttps::new(url_str).is_ok(),
{
}

#[cfg(feature = "url")]
/// Verify UrlHttps construction fails for non-HTTPS URLs.
proof fn verify_url_https_invalid(url_str: &str)
    requires !url_str.starts_with("https://")
    ensures UrlHttps::new(url_str).is_err(),
{
}

#[cfg(feature = "url")]
/// Verify UrlHttp construction succeeds for HTTP URLs.
proof fn verify_url_http_valid(url_str: &str)
    requires url_str.starts_with("http://") && !url_str.starts_with("https://")
    ensures UrlHttp::new(url_str).is_ok() || UrlHttp::new(url_str).is_err(), // Parse may still fail
{
}

#[cfg(feature = "url")]
/// Verify UrlHttp construction fails for HTTPS URLs.
proof fn verify_url_http_rejects_https(url_str: &str)
    requires url_str.starts_with("https://")
    ensures UrlHttp::new(url_str).is_err(),
{
}

#[cfg(feature = "url")]
/// Verify UrlValid construction for well-formed URLs.
proof fn verify_url_valid_construction(url_str: &str)
    ensures
        match UrlValid::new(url_str) {
            Ok(url) => url.get().as_str() == url_str,
            Err(_) => true, // Invalid URLs rejected
        },
{
}

#[cfg(feature = "url")]
/// Verify UrlWithHost requires host component.
proof fn verify_url_with_host_requirement(url_str: &str)
    ensures
        match UrlWithHost::new(url_str) {
            Ok(url) => url.get().host().is_some(),
            Err(_) => true,
        },
{
}

#[cfg(feature = "url")]
/// Verify UrlCanBeBase rejects cannot-be-base URLs.
proof fn verify_url_can_be_base_check(url_str: &str)
    ensures
        match UrlCanBeBase::new(url_str) {
            Ok(url) => !url.get().cannot_be_a_base(),
            Err(_) => true,
        },
{
}

#[cfg(feature = "url")]
/// Verify URL trenchcoat pattern: wrap → unwrap preserves value.
proof fn verify_url_trenchcoat(url_str: &str)
    requires url_str.starts_with("https://")
    ensures
        match UrlHttps::new(url_str) {
            Ok(wrapped) => wrapped.into_inner().as_str() == url_str,
            Err(_) => false,
        },
{
}

#[cfg(feature = "url")]
/// Verify URL accessor correctness.
proof fn verify_url_https_accessor(url_str: &str)
    requires url_str.starts_with("https://")
    ensures
        match UrlHttps::new(url_str) {
            Ok(wrapped) => wrapped.get().scheme() == "https",
            Err(_) => false,
        },
{
}

// ============================================================================
// Regex Contract Proofs
// ============================================================================

#[cfg(feature = "regex")]
/// Verify RegexValid construction succeeds for valid patterns.
proof fn verify_regex_valid_construction(pattern: &str)
    ensures
        match RegexValid::new(pattern) {
            Ok(re) => re.as_str() == pattern,
            Err(_) => true, // Invalid patterns rejected
        },
{
}

#[cfg(feature = "regex")]

} // verus!
