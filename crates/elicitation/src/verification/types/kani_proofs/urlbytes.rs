//! Kani proofs for URL byte validation (RFC 3986).

#![cfg(kani)]

use crate::verification::types::{
    UrlBytes, UrlWithAuthority, UrlAbsolute, UrlHttp,
};

// ============================================================================
// Scheme Validation Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_http_scheme() {
    const MAX_LEN: usize = 64;
    
    let result = UrlBytes::<MAX_LEN>::from_slice(b"http://example.com");
    assert!(result.is_ok());
    
    if let Ok(url) = result {
        assert_eq!(url.scheme(), "http");
        assert!(url.has_authority());
        assert!(url.is_http());
    }
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_https_scheme() {
    const MAX_LEN: usize = 64;
    
    let result = UrlBytes::<MAX_LEN>::from_slice(b"https://example.com");
    assert!(result.is_ok());
    
    if let Ok(url) = result {
        assert_eq!(url.scheme(), "https");
        assert!(url.is_http());
    }
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_ftp_scheme() {
    const MAX_LEN: usize = 64;
    
    let result = UrlBytes::<MAX_LEN>::from_slice(b"ftp://ftp.example.com");
    assert!(result.is_ok());
    
    if let Ok(url) = result {
        assert_eq!(url.scheme(), "ftp");
        assert!(!url.is_http());
    }
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_invalid_scheme_start() {
    const MAX_LEN: usize = 64;
    
    // Scheme must start with letter
    let result = UrlBytes::<MAX_LEN>::from_slice(b"1http://example.com");
    assert!(result.is_err());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_missing_colon() {
    const MAX_LEN: usize = 64;
    
    // Must have ':' after scheme
    let result = UrlBytes::<MAX_LEN>::from_slice(b"http//example.com");
    assert!(result.is_err());
}

// ============================================================================
// Authority Detection Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_authority_present() {
    const MAX_LEN: usize = 64;
    
    let url = UrlBytes::<MAX_LEN>::from_slice(b"http://example.com").unwrap();
    assert!(url.has_authority());
    assert_eq!(url.authority(), Some("example.com"));
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_authority_absent() {
    const MAX_LEN: usize = 64;
    
    let url = UrlBytes::<MAX_LEN>::from_slice(b"mailto:test@example.com").unwrap();
    assert!(!url.has_authority());
    assert_eq!(url.authority(), None);
}

// ============================================================================
// HTTP Scheme Detection Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_http_detection() {
    const MAX_LEN: usize = 64;
    
    let http = UrlBytes::<MAX_LEN>::from_slice(b"http://example.com").unwrap();
    assert!(http.is_http());
    
    let https = UrlBytes::<MAX_LEN>::from_slice(b"https://example.com").unwrap();
    assert!(https.is_http());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_non_http_detection() {
    const MAX_LEN: usize = 64;
    
    let ftp = UrlBytes::<MAX_LEN>::from_slice(b"ftp://example.com").unwrap();
    assert!(!ftp.is_http());
    
    let file = UrlBytes::<MAX_LEN>::from_slice(b"file:///path").unwrap();
    assert!(!file.is_http());
}

// ============================================================================
// Contract Type Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_with_authority_contract() {
    const MAX_LEN: usize = 64;
    
    let result = UrlWithAuthority::<MAX_LEN>::from_slice(b"http://example.com");
    assert!(result.is_ok());
    
    if let Ok(url) = result {
        assert!(url.url().has_authority());
    }
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_without_authority_rejected() {
    const MAX_LEN: usize = 64;
    
    let result = UrlWithAuthority::<MAX_LEN>::from_slice(b"mailto:test@example.com");
    assert!(result.is_err());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_absolute_contract() {
    const MAX_LEN: usize = 64;
    
    let result = UrlAbsolute::<MAX_LEN>::from_slice(b"http://example.com/path");
    assert!(result.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_http_contract_http() {
    const MAX_LEN: usize = 64;
    
    let http = UrlHttp::<MAX_LEN>::from_slice(b"http://example.com");
    assert!(http.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_http_contract_https() {
    const MAX_LEN: usize = 64;
    
    let https = UrlHttp::<MAX_LEN>::from_slice(b"https://example.com");
    assert!(https.is_ok());
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_http_contract_rejects_ftp() {
    const MAX_LEN: usize = 64;
    
    let ftp = UrlHttp::<MAX_LEN>::from_slice(b"ftp://ftp.example.com");
    assert!(ftp.is_err());
}

// ============================================================================
// Component Parsing Proofs
// ============================================================================

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_with_port() {
    const MAX_LEN: usize = 64;
    
    let url = UrlBytes::<MAX_LEN>::from_slice(b"http://example.com:8080").unwrap();
    assert_eq!(url.scheme(), "http");
    assert_eq!(url.authority(), Some("example.com:8080"));
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_url_with_path() {
    const MAX_LEN: usize = 64;
    
    let url = UrlBytes::<MAX_LEN>::from_slice(b"http://example.com/path").unwrap();
    assert_eq!(url.scheme(), "http");
    assert_eq!(url.authority(), Some("example.com"));
}

#[kani::proof]
#[kani::unwind(1)]
fn verify_file_url_empty_authority() {
    const MAX_LEN: usize = 64;
    
    let url = UrlBytes::<MAX_LEN>::from_slice(b"file:///path/to/file").unwrap();
    assert_eq!(url.scheme(), "file");
    assert!(url.has_authority());
}

