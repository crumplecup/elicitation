//! Kani proofs for URL byte validation (RFC 3986).
//!
//! Uses bounded component validation with proper unwind bounds.
//! Key insight: Unwind must match actual data length, not buffer size.

#![cfg(kani)]

use crate::verification::types::{
    AuthorityBytes, SchemeBytes, UrlAbsolute, UrlBytes, UrlHttp, UrlWithAuthority,
};

// ============================================================================
// Component Validation Proofs (Small Bounds)
// ============================================================================

#[kani::proof]
fn verify_scheme_http() {
    const MAX_LEN: usize = 8; // Small buffer for schemes

    let bytes = b"http";
    let result = SchemeBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(scheme) = result {
        assert_eq!(scheme.as_str(), "http");
        assert!(scheme.is_http());
    }
}

#[kani::proof]
fn verify_scheme_https() {
    const MAX_LEN: usize = 8;

    let bytes = b"https";
    let result = SchemeBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(scheme) = result {
        assert_eq!(scheme.as_str(), "https");
        assert!(scheme.is_http());
    }
}

#[kani::proof]
fn verify_scheme_ftp() {
    const MAX_LEN: usize = 8;

    let bytes = b"ftp";
    let result = SchemeBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(scheme) = result {
        assert_eq!(scheme.as_str(), "ftp");
        assert!(!scheme.is_http());
    }
}

#[kani::proof]
fn verify_scheme_invalid_start() {
    const MAX_LEN: usize = 8;

    let bytes = b"1http";
    let result = SchemeBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

#[kani::proof]
fn verify_scheme_with_plus() {
    const MAX_LEN: usize = 16;

    let bytes = b"custom+scheme";
    let result = SchemeBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
fn verify_authority_simple() {
    const MAX_LEN: usize = 64; // Reasonable authority size

    let bytes = b"example.com";
    let result = AuthorityBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(auth) = result {
        assert_eq!(auth.as_str(), "example.com");
        assert!(!auth.is_empty());
    }
}

#[kani::proof]
fn verify_authority_with_port() {
    const MAX_LEN: usize = 64;

    let bytes = b"example.com:8080";
    let result = AuthorityBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(auth) = result {
        assert_eq!(auth.as_str(), "example.com:8080");
    }
}

#[kani::proof]
fn verify_authority_empty() {
    const MAX_LEN: usize = 64;

    let bytes = b"";
    let result = AuthorityBytes::<MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(auth) = result {
        assert!(auth.is_empty());
    }
}

// ============================================================================
// URL Composition Proofs (Minimal Bounds)
// ============================================================================

#[kani::proof]
fn verify_http_url_composition() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128; // Small total buffer

    let bytes = b"http://example.com";
    let result = UrlBytes::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.authority(), Some("example.com"));
        assert!(url.has_authority());
        assert!(url.is_http());
    }
}

#[kani::proof]
fn verify_https_url_composition() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"https://example.com";
    let result = UrlBytes::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert_eq!(url.scheme(), "https");
        assert!(url.is_http());
    }
}

#[kani::proof]
fn verify_ftp_url_composition() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"ftp://ftp.example.com";
    let result = UrlBytes::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert_eq!(url.scheme(), "ftp");
        assert!(!url.is_http());
    }
}

#[kani::proof]
fn verify_url_no_authority() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"mailto:test@example.com";
    let result = UrlBytes::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert_eq!(url.scheme(), "mailto");
        assert!(!url.has_authority());
        assert_eq!(url.authority(), None);
    }
}

// ============================================================================
// Contract Type Proofs
// ============================================================================

#[kani::proof]
fn verify_url_with_authority_contract() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"http://example.com";
    let result = UrlWithAuthority::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert!(url.url().has_authority());
    }
}

#[kani::proof]
fn verify_url_without_authority_rejected() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"mailto:test@example.com";
    let result = UrlWithAuthority::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_err());
}

#[kani::proof]
fn verify_url_absolute_contract() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"http://example.com/path";
    let result = UrlAbsolute::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());
}

#[kani::proof]
fn verify_url_http_contract_http() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"http://example.com";
    let http = UrlHttp::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(http.is_ok());
}

#[kani::proof]
fn verify_url_http_contract_https() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"https://example.com";
    let https = UrlHttp::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(https.is_ok());
}

#[kani::proof]
fn verify_url_http_contract_rejects_ftp() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"ftp://ftp.example.com";
    let ftp = UrlHttp::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(ftp.is_err());
}

#[kani::proof]
fn verify_url_with_port() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"http://example.com:8080";
    let result = UrlBytes::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.authority(), Some("example.com:8080"));
    }
}

#[kani::proof]
fn verify_file_url_empty_authority() {
    const SCHEME_MAX: usize = 8;
    const AUTHORITY_MAX: usize = 64;
    const MAX_LEN: usize = 128;

    let bytes = b"file:///path/to/file";
    let result = UrlBytes::<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>::from_slice(bytes);
    assert!(result.is_ok());

    if let Ok(url) = result {
        assert_eq!(url.scheme(), "file");
        assert!(url.has_authority());
    }
}
