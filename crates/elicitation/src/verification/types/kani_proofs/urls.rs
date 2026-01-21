//! Kani proofs for URL contract types.

#[cfg(feature = "url")]
use crate::{UrlValid, UrlHttps, UrlHttp, UrlWithHost, UrlCanBeBase};

// ============================================================================
// URL Contract Proofs
// ============================================================================

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_https() {
    // Test known HTTPS URL
    let https_result = UrlHttps::new("https://example.com");
    assert!(https_result.is_ok(), "Valid HTTPS URL accepted");
    
    // Test non-HTTPS URL
    let http_result = UrlHttps::new("http://example.com");
    assert!(http_result.is_err(), "HTTP URL rejected");
}

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_http() {
    // Test known HTTP URL
    let http_result = UrlHttp::new("http://example.com");
    assert!(http_result.is_ok(), "Valid HTTP URL accepted");
    
    // Test non-HTTP URL
    let https_result = UrlHttp::new("https://example.com");
    assert!(https_result.is_err(), "HTTPS URL rejected");
}

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_valid() {
    // Test various valid URL schemes
    assert!(
        UrlValid::new("https://example.com").is_ok(),
        "HTTPS URL is valid"
    );
    assert!(
        UrlValid::new("http://localhost:8080").is_ok(),
        "HTTP with port is valid"
    );
    assert!(
        UrlValid::new("ftp://files.example.com").is_ok(),
        "FTP URL is valid"
    );
    
    // Test invalid URLs
    assert!(
        UrlValid::new("not a url").is_err(),
        "Invalid URL rejected"
    );
}

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_with_host() {
    // Test URLs with hosts
    assert!(
        UrlWithHost::new("https://example.com").is_ok(),
        "URL with domain host accepted"
    );
    assert!(
        UrlWithHost::new("http://192.168.1.1:8080").is_ok(),
        "URL with IP host accepted"
    );
    
    // Test URLs without hosts (like mailto, data)
    assert!(
        UrlWithHost::new("mailto:user@example.com").is_err(),
        "mailto URL has no host, rejected"
    );
    assert!(
        UrlWithHost::new("data:text/plain,hello").is_err(),
        "data URL has no host, rejected"
    );
}

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_can_be_base() {
    // Test URLs that can be base
    assert!(
        UrlCanBeBase::new("https://example.com").is_ok(),
        "HTTP(S) URL can be base"
    );
    assert!(
        UrlCanBeBase::new("http://example.com/path/").is_ok(),
        "URL with path can be base"
    );
    
    // Test URLs that cannot be base
    assert!(
        UrlCanBeBase::new("mailto:user@example.com").is_err(),
        "mailto cannot be base"
    );
    assert!(
        UrlCanBeBase::new("data:text/plain,hello").is_err(),
        "data URL cannot be base"
    );
}

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_https_accessor() {
    // Test accessor methods preserve invariants
    let https = UrlHttps::new("https://secure.example.com").unwrap();
    let url_ref = https.get();
    
    assert!(url_ref.scheme() == "https", "Accessor returns HTTPS URL");
    assert!(url_ref.host_str().is_some(), "HTTPS URL has host");
    
    let url_inner = https.into_inner();
    assert!(url_inner.scheme() == "https", "into_inner() returns HTTPS URL");
}

#[cfg(feature = "url")]
#[kani::proof]
#[kani::unwind(1)]
fn verify_url_trenchcoat_pattern() {
    // Prove trenchcoat pattern: parse → wrap → validate → unwrap → use
    let original = "https://api.example.com/v1/endpoint";
    
    if let Ok(wrapped) = UrlHttps::new(original) {
        let unwrapped = wrapped.into_inner();
        
        // Trenchcoat: The URL string is preserved through wrap/unwrap
        assert!(
            unwrapped.scheme() == "https",
            "Scheme preserved through trenchcoat"
        );
        assert!(
            unwrapped.as_str() == original,
            "Full URL preserved through trenchcoat"
        );
    }
}

// ============================================================================
// Regex Contract Proofs
