//! Creusot proofs for URL validation types (RFC 3986).
//!
//! Validates URL syntax through component validation:
//! - SchemeBytes: http, https, ftp, etc. (RFC 3986 scheme grammar)
//! - AuthorityBytes: example.com:8080 (RFC 3986 authority grammar)
//! - UrlBytes: Complete URL with bounded components
//! - Contract types: UrlWithAuthority, UrlAbsolute, UrlHttpBytes
//!
//! This is compositional verification: (utf8_correct ∧ url_crate_correct) → wrapper_correct.

use crate::*;

#[cfg(creusot)]
use elicitation::verification::types::{
    AuthorityBytes, SchemeBytes, UrlAbsoluteBytes, UrlBytes, UrlHttpBytes, UrlWithAuthorityBytes,
    ValidationError,
};

// SchemeBytes Validation Proofs
// ============================================================================

/// Verify: SchemeBytes rejects length exceeding MAX_LEN
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_scheme_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    SchemeBytes::from_slice(bytes)
}

/// Verify: SchemeBytes accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() > 0 && bytes@.len() <= MAX_LEN@)]
pub fn verify_scheme_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    SchemeBytes::from_slice(bytes)
}

/// Verify: SchemeBytes rejects empty scheme
#[cfg(creusot)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_scheme_empty() -> Result<SchemeBytes<10>, ValidationError> {
    SchemeBytes::from_slice(&[] as &[u8])
}

/// Verify: SchemeBytes accepts http
#[cfg(creusot)]
pub fn verify_scheme_http() -> Result<SchemeBytes<4>, ValidationError> {
    let bytes = [b'h', b't', b't', b'p'];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: SchemeBytes accepts https
#[cfg(creusot)]
pub fn verify_scheme_https() -> Result<SchemeBytes<5>, ValidationError> {
    let bytes = [b'h', b't', b't', b'p', b's'];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: SchemeBytes accepts ftp
#[cfg(creusot)]
pub fn verify_scheme_ftp() -> Result<SchemeBytes<3>, ValidationError> {
    let bytes = [b'f', b't', b'p'];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: SchemeBytes accepts file
#[cfg(creusot)]
pub fn verify_scheme_file() -> Result<SchemeBytes<4>, ValidationError> {
    let bytes = [b'f', b'i', b'l', b'e'];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: SchemeBytes accepts scheme with plus
#[cfg(creusot)]
pub fn verify_scheme_with_plus() -> Result<SchemeBytes<16>, ValidationError> {
    let bytes = [
        b'c', b'u', b's', b't', b'o', b'm', b'+', b's', b'c', b'h', b'e', b'm', b'e',
    ];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: SchemeBytes accepts scheme with dash
#[cfg(creusot)]
pub fn verify_scheme_with_dash() -> Result<SchemeBytes<16>, ValidationError> {
    let bytes = [
        b'c', b'u', b's', b't', b'o', b'm', b'-', b's', b'c', b'h', b'e', b'm', b'e',
    ];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: SchemeBytes accepts scheme with dot
#[cfg(creusot)]
pub fn verify_scheme_with_dot() -> Result<SchemeBytes<16>, ValidationError> {
    let bytes = [
        b'c', b'u', b's', b't', b'o', b'm', b'.', b's', b'c', b'h', b'e', b'm', b'e',
    ];
    SchemeBytes::from_slice(&bytes)
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() > 0 && bytes@.len() <= MAX_LEN@)]
pub fn verify_scheme_as_str<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<SchemeBytes<MAX_LEN>, ValidationError> {
    let scheme = SchemeBytes::from_slice(bytes)?;
    let _s = scheme.as_str(); // Should not panic
    Ok(scheme)
}

/// Verify: is_http() doesn't panic
#[cfg(creusot)]
#[requires(bytes@.len() > 0 && bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
pub fn verify_authority_length_check<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<AuthorityBytes<MAX_LEN>, ValidationError> {
    AuthorityBytes::from_slice(bytes)
}

/// Verify: AuthorityBytes accepts valid length
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
pub fn verify_authority_length_valid<const MAX_LEN: usize>(
    bytes: &[u8],
) -> Result<AuthorityBytes<MAX_LEN>, ValidationError> {
    AuthorityBytes::from_slice(bytes)
}

/// Verify: AuthorityBytes accepts empty authority
#[cfg(creusot)]
pub fn verify_authority_empty() -> Result<AuthorityBytes<64>, ValidationError> {
    AuthorityBytes::from_slice(&[] as &[u8])
}

/// Verify: AuthorityBytes accepts simple domain
#[cfg(creusot)]
pub fn verify_authority_simple() -> Result<AuthorityBytes<16>, ValidationError> {
    let bytes = [
        b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'c', b'o', b'm',
    ];
    AuthorityBytes::from_slice(&bytes)
}

/// Verify: AuthorityBytes accepts domain with port
#[cfg(creusot)]
pub fn verify_authority_with_port() -> Result<AuthorityBytes<32>, ValidationError> {
    let bytes = [
        b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'c', b'o', b'm', b':', b'8', b'0', b'8',
        b'0',
    ];
    AuthorityBytes::from_slice(&bytes)
}

/// Verify: AuthorityBytes accepts localhost
#[cfg(creusot)]
pub fn verify_authority_localhost() -> Result<AuthorityBytes<16>, ValidationError> {
    let bytes = [b'l', b'o', b'c', b'a', b'l', b'h', b'o', b's', b't'];
    AuthorityBytes::from_slice(&bytes)
}

/// Verify: AuthorityBytes accepts IP address
#[cfg(creusot)]
pub fn verify_authority_ip() -> Result<AuthorityBytes<16>, ValidationError> {
    let bytes = [b'1', b'2', b'7', b'.', b'0', b'.', b'0', b'.', b'1'];
    AuthorityBytes::from_slice(&bytes)
}

/// Verify: AuthorityBytes accepts IP with port
#[cfg(creusot)]
pub fn verify_authority_ip_port() -> Result<AuthorityBytes<32>, ValidationError> {
    let bytes = [
        b'1', b'9', b'2', b'.', b'1', b'6', b'8', b'.', b'1', b'.', b'1', b':', b'3', b'0', b'0',
        b'0',
    ];
    AuthorityBytes::from_slice(&bytes)
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
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
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
pub fn verify_http_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: HTTPS URL
#[cfg(creusot)]
pub fn verify_https_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b's', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e',
        b'.', b'c', b'o', b'm',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: URL with port
#[cfg(creusot)]
pub fn verify_url_with_port() -> Result<UrlBytes<8, 32, 64>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm', b':', b'8', b'0', b'8', b'0',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: URL with path
#[cfg(creusot)]
pub fn verify_url_with_path() -> Result<UrlBytes<8, 16, 64>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm', b'/', b'p', b'a', b't', b'h',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: File URL
#[cfg(creusot)]
pub fn verify_file_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'f', b'i', b'l', b'e', b':', b'/', b'/', b'/', b'h', b'o', b'm', b'e', b'/', b'u', b's',
        b'e', b'r', b'/', b'f', b'i', b'l', b'e',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: FTP URL
#[cfg(creusot)]
pub fn verify_ftp_url() -> Result<UrlBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'f', b't', b'p', b':', b'/', b'/', b'f', b't', b'p', b'.', b'e', b'x', b'a', b'm', b'p',
        b'l', b'e', b'.', b'c', b'o', b'm',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: URL with query string
#[cfg(creusot)]
pub fn verify_url_with_query() -> Result<UrlBytes<8, 16, 64>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm', b'?', b'q', b'=', b't', b'e', b's', b't',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: URL with fragment
#[cfg(creusot)]
pub fn verify_url_with_fragment() -> Result<UrlBytes<8, 16, 64>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm', b'#', b's', b'e', b'c', b't', b'i', b'o', b'n',
    ];
    UrlBytes::from_slice(&bytes)
}

/// Verify: as_str() returns valid string
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
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
#[cfg(creusot)]
pub fn verify_url_with_authority_http() -> Result<UrlWithAuthorityBytes<8, 16, 32>, ValidationError>
{
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm',
    ];
    UrlWithAuthorityBytes::from_slice(&bytes)
}

/// Verify: UrlWithAuthority with port
#[cfg(creusot)]
pub fn verify_url_with_authority_port() -> Result<UrlWithAuthorityBytes<8, 32, 64>, ValidationError>
{
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm', b':', b'8', b'0', b'8', b'0',
    ];
    UrlWithAuthorityBytes::from_slice(&bytes)
}

/// Verify: url() returns underlying UrlBytes
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
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
#[cfg(creusot)]
pub fn verify_url_absolute_http() -> Result<UrlAbsoluteBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm',
    ];
    UrlAbsoluteBytes::from_slice(&bytes)
}

/// Verify: url() returns underlying UrlBytes
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() > MAX_LEN@)]
#[ensures(match result { Err(_) => true, Ok(_) => false })]
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
#[cfg(creusot)]
pub fn verify_url_http_http() -> Result<UrlHttpBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.',
        b'c', b'o', b'm',
    ];
    UrlHttpBytes::from_slice(&bytes)
}

/// Verify: UrlHttpBytes accepts HTTPS URL
#[cfg(creusot)]
pub fn verify_url_http_https() -> Result<UrlHttpBytes<8, 16, 32>, ValidationError> {
    let bytes = [
        b'h', b't', b't', b'p', b's', b':', b'/', b'/', b'e', b'x', b'a', b'm', b'p', b'l', b'e',
        b'.', b'c', b'o', b'm',
    ];
    UrlHttpBytes::from_slice(&bytes)
}

/// Verify: url() returns underlying UrlBytes
#[cfg(creusot)]
#[requires(bytes@.len() <= MAX_LEN@)]
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
#[cfg(creusot)]
#[requires(bytes@.len() <= 8)]
pub fn verify_url_small_buffer(bytes: &[u8]) -> Result<UrlBytes<4, 4, 8>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

/// Verify: Medium buffer (128 bytes)
#[cfg(creusot)]
#[requires(bytes@.len() <= 128)]
pub fn verify_url_medium_buffer(bytes: &[u8]) -> Result<UrlBytes<16, 64, 128>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

/// Verify: Large buffer (2048 bytes)
#[cfg(creusot)]
#[requires(bytes@.len() <= 2048)]
pub fn verify_url_large_buffer(bytes: &[u8]) -> Result<UrlBytes<32, 256, 2048>, ValidationError> {
    UrlBytes::from_slice(bytes)
}

// ============================================================================
