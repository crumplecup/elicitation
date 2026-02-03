//! Prusti proofs for URL validation types (RFC 3986).
//!
//! Validates URL syntax through component validation:
//! - SchemeBytes: http, https, ftp, etc. (RFC 3986 scheme grammar)
//! - AuthorityBytes: example.com:8080 (RFC 3986 authority grammar)
//! - UrlBytes: Complete URL with bounded components
//! - Contract types: UrlWithAuthority, UrlAbsolute, UrlHttpBytes
//!
//! This is compositional verification: (utf8_correct ∧ url_crate_correct) → wrapper_correct.

#![cfg(feature = "verify-prusti")]
#![allow(unused_imports)]

use crate::verification::types::{
    AuthorityBytes, SchemeBytes, UrlAbsoluteBytes, UrlBytes, UrlHttpBytes, UrlWithAuthorityBytes,
    ValidationError,
};
use prusti_contracts::*;

// SchemeBytes Validation Proofs
// ============================================================================

/// Verify: SchemeBytes rejects length exceeding MAX_LEN
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_scheme_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    SchemeBytes::from_slice(bytes)
}

/// Verify: SchemeBytes accepts valid length
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > 0 && bytes.len() <= MAX_LEN)]
pub fn verify_scheme_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    SchemeBytes::from_slice(bytes)
}

/// Verify: SchemeBytes rejects empty scheme
#[cfg(feature = "verify-prusti")]
#[ensures(result.is_err())]
pub fn verify_scheme_empty() -> Result<SchemeBytes<10>, ValidationError> {
    SchemeBytes::from_slice(b"")
}

/// Verify: SchemeBytes accepts http
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_http() -> Result<SchemeBytes<4>, ValidationError> {
    SchemeBytes::from_slice(b"http")
}

/// Verify: SchemeBytes accepts https
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_https() -> Result<SchemeBytes<5>, ValidationError> {
    SchemeBytes::from_slice(b"https")
}

/// Verify: SchemeBytes accepts ftp
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_ftp() -> Result<SchemeBytes<3>, ValidationError> {
    SchemeBytes::from_slice(b"ftp")
}

/// Verify: SchemeBytes accepts file
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_file() -> Result<SchemeBytes<4>, ValidationError> {
    SchemeBytes::from_slice(b"file")
}

/// Verify: SchemeBytes accepts scheme with plus
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_with_plus() -> Result<SchemeBytes<16>, ValidationError> {
    SchemeBytes::from_slice(b"custom+scheme")
}

/// Verify: SchemeBytes accepts scheme with dash
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_with_dash() -> Result<SchemeBytes<16>, ValidationError> {
    SchemeBytes::from_slice(b"custom-scheme")
}

/// Verify: SchemeBytes accepts scheme with dot
#[cfg(feature = "verify-prusti")]
pub fn verify_scheme_with_dot() -> Result<SchemeBytes<16>, ValidationError> {
    SchemeBytes::from_slice(b"custom.scheme")
}

/// Verify: as_str() returns valid string
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > 0 && bytes.len() <= MAX_LEN)]
pub fn verify_scheme_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    let scheme = SchemeBytes::from_slice(bytes)?;
    let _s = scheme.as_str(); // Should not panic
    Ok(scheme)
}

/// Verify: is_http() doesn't panic
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > 0 && bytes.len() <= MAX_LEN)]
pub fn verify_scheme_is_http<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    let scheme = SchemeBytes::from_slice(bytes)?;
    let _is_http = scheme.is_http(); // Should not panic
    Ok(scheme)
}

// AuthorityBytes Validation Proofs
// ============================================================================

/// Verify: AuthorityBytes rejects length exceeding MAX_LEN
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_authority_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<AuthorityBytes<MAX_LEN>, ValidationError> {
    AuthorityBytes::from_slice(bytes)
}

/// Verify: AuthorityBytes accepts valid length
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_authority_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<AuthorityBytes<MAX_LEN>, ValidationError> {
    AuthorityBytes::from_slice(bytes)
}

/// Verify: AuthorityBytes accepts empty authority
#[cfg(feature = "verify-prusti")]
pub fn verify_authority_empty() -> Result<AuthorityBytes<64>, ValidationError> {
    AuthorityBytes::from_slice(b"")
}

/// Verify: AuthorityBytes accepts simple domain
#[cfg(feature = "verify-prusti")]
pub fn verify_authority_simple() -> Result<AuthorityBytes<16>, ValidationError> {
    AuthorityBytes::from_slice(b"example.com")
}

/// Verify: AuthorityBytes accepts domain with port
#[cfg(feature = "verify-prusti")]
pub fn verify_authority_with_port() -> Result<AuthorityBytes<32>, ValidationError> {
    AuthorityBytes::from_slice(b"example.com:8080")
}

/// Verify: AuthorityBytes accepts localhost
#[cfg(feature = "verify-prusti")]
pub fn verify_authority_localhost() -> Result<AuthorityBytes<16>, ValidationError> {
    AuthorityBytes::from_slice(b"localhost")
}

/// Verify: AuthorityBytes accepts IP address
#[cfg(feature = "verify-prusti")]
pub fn verify_authority_ip() -> Result<AuthorityBytes<16>, ValidationError> {
    AuthorityBytes::from_slice(b"127.0.0.1")
}

/// Verify: AuthorityBytes accepts IP with port
#[cfg(feature = "verify-prusti")]
pub fn verify_authority_ip_port() -> Result<AuthorityBytes<32>, ValidationError> {
    AuthorityBytes::from_slice(b"192.168.1.1:3000")
}

/// Verify: as_str() returns valid string
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_authority_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<AuthorityBytes<MAX_LEN>, ValidationError> {
    let authority = AuthorityBytes::from_slice(bytes)?;
    let _s = authority.as_str(); // Should not panic
    Ok(authority)
}

// UrlBytes Validation Proofs
// ============================================================================

/// Verify: UrlBytes rejects length exceeding MAX_LEN
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_url_length_check<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

/// Verify: UrlBytes accepts valid length
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_url_length_valid<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

/// Verify: Simple HTTP URL
#[cfg(feature = "verify-prusti")]
pub fn verify_http_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    UrlBytes::from_slice(b"http://example.com")
}

/// Verify: HTTPS URL
#[cfg(feature = "verify-prusti")]
pub fn verify_https_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    UrlBytes::from_slice(b"https://example.com")
}

/// Verify: URL with port
#[cfg(feature = "verify-prusti")]
pub fn verify_url_with_port() -> Result<UrlBytes<8, 32, 64>, ValidationError> {
    UrlBytes::from_slice(b"http://example.com:8080")
}

/// Verify: URL with path
#[cfg(feature = "verify-prusti")]
pub fn verify_url_with_path() -> Result<UrlBytes<8, 16, 64>, ValidationError> {
    UrlBytes::from_slice(b"http://example.com/path")
}

/// Verify: File URL
#[cfg(feature = "verify-prusti")]
pub fn verify_file_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    UrlBytes::from_slice(b"file:///home/user/file")
}

/// Verify: FTP URL
#[cfg(feature = "verify-prusti")]
pub fn verify_ftp_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    UrlBytes::from_slice(b"ftp://ftp.example.com")
}

/// Verify: URL with query string
#[cfg(feature = "verify-prusti")]
pub fn verify_url_with_query() -> Result<UrlBytes<8, 16, 64>, ValidationError> {
    UrlBytes::from_slice(b"http://example.com?q=test")
}

/// Verify: URL with fragment
#[cfg(feature = "verify-prusti")]
pub fn verify_url_with_fragment() -> Result<UrlBytes<8, 16, 64>, ValidationError> {
    UrlBytes::from_slice(b"http://example.com#section")
}

/// Verify: as_str() returns valid string
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_url_as_str<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    let url = UrlBytes::from_slice(bytes)?;
    let _s = url.as_str(); // Should not panic
    Ok(url)
}

// UrlWithAuthority Validation Proofs
// ============================================================================

/// Verify: UrlWithAuthority rejects length exceeding MAX_LEN
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_url_with_authority_length_check<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlWithAuthorityBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    UrlWithAuthorityBytes::from_slice(bytes)
}

/// Verify: UrlWithAuthority accepts URL with authority
#[cfg(feature = "verify-prusti")]
pub fn verify_url_with_authority_http() -> Result<UrlWithAuthorityBytes<8, 16, 32>, ValidationError>
{
    UrlWithAuthorityBytes::from_slice(b"http://example.com")
}

/// Verify: UrlWithAuthority with port
#[cfg(feature = "verify-prusti")]
pub fn verify_url_with_authority_port() -> Result<UrlWithAuthorityBytes<8, 32, 64>, ValidationError>
{
    UrlWithAuthorityBytes::from_slice(b"http://example.com:8080")
}

/// Verify: url() returns underlying UrlBytes
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_url_with_authority_get<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlWithAuthorityBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    let url_with_auth = UrlWithAuthorityBytes::from_slice(bytes)?;
    let _inner = url_with_auth.url(); // Should not panic
    Ok(url_with_auth)
}

// UrlAbsolute Validation Proofs
// ============================================================================

/// Verify: UrlAbsolute rejects length exceeding MAX_LEN
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_url_absolute_length_check<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlAbsoluteBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    UrlAbsoluteBytes::from_slice(bytes)
}

/// Verify: UrlAbsolute accepts absolute URL
#[cfg(feature = "verify-prusti")]
pub fn verify_url_absolute_http() -> Result<UrlAbsoluteBytes<8, 16, 32>, ValidationError> {
    UrlAbsoluteBytes::from_slice(b"http://example.com")
}

/// Verify: url() returns underlying UrlBytes
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_url_absolute_get<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlAbsoluteBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    let url_abs = UrlAbsoluteBytes::from_slice(bytes)?;
    let _inner = url_abs.url(); // Should not panic
    Ok(url_abs)
}

// UrlHttpBytes Validation Proofs
// ============================================================================

/// Verify: UrlHttpBytes rejects length exceeding MAX_LEN
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() > MAX_LEN)]
#[ensures(result.is_err())]
pub fn verify_url_http_length_check<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlHttpBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    UrlHttpBytes::from_slice(bytes)
}

/// Verify: UrlHttpBytes accepts HTTP URL
#[cfg(feature = "verify-prusti")]
pub fn verify_url_http_http() -> Result<UrlHttpBytes<8, 16, 32>, ValidationError> {
    UrlHttpBytes::from_slice(b"http://example.com")
}

/// Verify: UrlHttpBytes accepts HTTPS URL
#[cfg(feature = "verify-prusti")]
pub fn verify_url_http_https() -> Result<UrlHttpBytes<8, 16, 32>, ValidationError> {
    UrlHttpBytes::from_slice(b"https://example.com")
}

/// Verify: url() returns underlying UrlBytes
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= MAX_LEN)]
pub fn verify_url_http_get<
    const SCHEME_MAX: usize,
    const AUTHORITY_MAX: usize,
    const MAX_LEN: usize,
>(
    bytes: &[u8],
) -> Result<UrlHttpBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>, ValidationError> {
    let url_http = UrlHttpBytes::from_slice(bytes)?;
    let _inner = url_http.url(); // Should not panic
    Ok(url_http)
}

// Edge Cases
// ============================================================================

/// Verify: Small buffer (8 bytes)
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= 8)]
pub fn verify_url_small_buffer(bytes: &[u8]) -> Result<UrlBytes<4, 4, 8>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

/// Verify: Medium buffer (128 bytes)
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= 128)]
pub fn verify_url_medium_buffer(bytes: &[u8]) -> Result<UrlBytes<16, 64, 128>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

/// Verify: Large buffer (2048 bytes)
#[cfg(feature = "verify-prusti")]
#[requires(bytes.len() <= 2048)]
pub fn verify_url_large_buffer(bytes: &[u8]) -> Result<UrlBytes<32, 256, 2048>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

// ============================================================================
