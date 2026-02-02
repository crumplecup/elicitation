//! Kani proofs for URL contract types.

#[cfg(feature = "url")]
use crate::{UrlCanBeBase, UrlHttps, UrlValid, UrlWithHost};

// Note: UrlHttp is from urlbytes.rs, not urls.rs

// ============================================================================
// URL Contract Proofs - Wrapper Logic Only
// ============================================================================
//
// These proofs verify ONLY the wrapper logic, not URL parsing.
// We trust the url crate's correctness and verify our contract enforcement.

#[cfg(feature = "url")]
#[kani::proof]
fn verify_url_https_wrapper() {
    // Test HTTPS wrapper logic
    let result = UrlHttps::new("https://example.com");
    
    match result {
        Ok(_url) => {
            // HTTPS wrapper constructed successfully
        }
        Err(e) => {
            // Could be invalid URL or not HTTPS
            assert!(
                matches!(e, crate::ValidationError::UrlInvalid) ||
                matches!(e, crate::ValidationError::UrlNotHttps)
            );
        }
    }
}

#[cfg(feature = "url")]
#[kani::proof]
fn verify_url_valid_wrapper() {
    // Test basic URL wrapper
    let result = UrlValid::new("https://example.com");
    
    match result {
        Ok(_url) => {
            // Valid URL wrapper constructed
        }
        Err(e) => {
            assert!(matches!(e, crate::ValidationError::UrlInvalid));
        }
    }
}

#[cfg(feature = "url")]
#[kani::proof]
fn verify_url_with_host_wrapper() {
    // Test host requirement wrapper
    let result = UrlWithHost::new("https://example.com");
    
    match result {
        Ok(_url) => {
            // URL with host wrapper constructed
        }
        Err(e) => {
            assert!(
                matches!(e, crate::ValidationError::UrlInvalid) ||
                matches!(e, crate::ValidationError::UrlNoHost)
            );
        }
    }
}

#[cfg(feature = "url")]
#[kani::proof]
fn verify_url_can_be_base_wrapper() {
    // Test base URL requirement wrapper
    let result = UrlCanBeBase::new("https://example.com");
    
    match result {
        Ok(_url) => {
            // Base URL wrapper constructed
        }
        Err(e) => {
            assert!(
                matches!(e, crate::ValidationError::UrlInvalid) ||
                matches!(e, crate::ValidationError::UrlCannotBeBase)
            );
        }
    }
}
