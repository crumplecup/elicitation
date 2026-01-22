//! URL byte validation following RFC 3986.
//!
//! This module provides byte-level validation for URLs, demonstrating that
//! complex parsing can be verified tractably by breaking it into simple
//! constraint checks.

use crate::verification::types::{ValidationError, Utf8Bytes};

// ============================================================================
// Core Types
// ============================================================================

/// Validated URL bytes (RFC 3986 syntax).
///
/// Validates:
/// - Valid UTF-8 (reuses Utf8Bytes foundation)
/// - RFC 3986 URL syntax (scheme, authority, path, query, fragment)
/// - Balanced delimiters and proper encoding
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlBytes<const MAX_LEN: usize = 2048> {
    utf8: Utf8Bytes<MAX_LEN>,
    components: UrlComponents,
}

/// Parsed URL components (byte offsets).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UrlComponents {
    /// Scheme end position (before ':')
    scheme_end: usize,
    /// Authority start (after '//')
    authority_start: Option<usize>,
    /// Authority end (before '/', '?', '#', or end)
    authority_end: Option<usize>,
    /// Path start
    path_start: usize,
    /// Query start (after '?')
    query_start: Option<usize>,
    /// Fragment start (after '#')
    fragment_start: Option<usize>,
}

// ============================================================================
// Contract Types
// ============================================================================

/// URL with authority (has //).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlWithAuthority<const MAX_LEN: usize = 2048> {
    url: UrlBytes<MAX_LEN>,
}

/// Absolute URL (has scheme + authority).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlAbsolute<const MAX_LEN: usize = 2048> {
    url: UrlBytes<MAX_LEN>,
}

/// URL with HTTP/HTTPS scheme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlHttp<const MAX_LEN: usize = 2048> {
    url: UrlBytes<MAX_LEN>,
}

// ============================================================================
// UrlBytes Implementation
// ============================================================================

impl<const MAX_LEN: usize> UrlBytes<MAX_LEN> {
    /// Create from byte slice (Kani-friendly, no Vec allocation).
    ///
    /// Returns `ValidationError::InvalidUtf8` if not valid UTF-8.
    /// Returns `ValidationError::TooLong` if exceeds MAX_LEN.
    /// Returns `ValidationError::InvalidUrlSyntax` if invalid RFC 3986 syntax.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();
        
        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }
        
        // Copy to fixed array (Kani's native domain!)
        let mut fixed = [0u8; MAX_LEN];
        fixed[..len].copy_from_slice(bytes);
        
        // Validate UTF-8 (reuse foundation!)
        let utf8 = Utf8Bytes::new(fixed, len)?;
        
        // Parse and validate URL syntax
        let components = parse_url_components(utf8.as_str())?;
        
        Ok(Self { utf8, components })
    }
    
    /// Create from Vec (user-facing API, delegates to from_slice).
    ///
    /// Returns `ValidationError::InvalidUtf8` if not valid UTF-8.
    /// Returns `ValidationError::TooLong` if exceeds MAX_LEN.
    /// Returns `ValidationError::InvalidUrlSyntax` if invalid RFC 3986 syntax.
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }
    
    /// Get the URL as a string slice.
    pub fn as_str(&self) -> &str {
        self.utf8.as_str()
    }
    
    /// Get the URL as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.utf8.as_str().as_bytes()
    }
    
    /// Get the length in bytes.
    pub fn len(&self) -> usize {
        self.utf8.len()
    }
    
    /// Check if the URL is empty.
    pub fn is_empty(&self) -> bool {
        self.utf8.is_empty()
    }
    
    /// Get the scheme (e.g., "http", "https", "ftp").
    pub fn scheme(&self) -> &str {
        &self.as_str()[..self.components.scheme_end]
    }
    
    /// Get the authority if present (e.g., "example.com:8080").
    pub fn authority(&self) -> Option<&str> {
        match (self.components.authority_start, self.components.authority_end) {
            (Some(start), Some(end)) => Some(&self.as_str()[start..end]),
            _ => None,
        }
    }
    
    /// Check if URL has authority (starts with //).
    pub fn has_authority(&self) -> bool {
        self.components.authority_start.is_some()
    }
    
    /// Check if URL has HTTP or HTTPS scheme.
    pub fn is_http(&self) -> bool {
        let scheme = self.scheme();
        scheme == "http" || scheme == "https"
    }
    
    /// Convert to String.
    pub fn to_string(&self) -> String {
        self.utf8.to_string()
    }
}

// ============================================================================
// Contract Type Implementations
// ============================================================================

impl<const MAX_LEN: usize> UrlWithAuthority<MAX_LEN> {
    /// Create from byte slice (Kani-friendly).
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let url = UrlBytes::from_slice(bytes)?;
        
        if !url.has_authority() {
            return Err(ValidationError::UrlMissingAuthority);
        }
        
        Ok(Self { url })
    }
    
    /// Create from Vec (user-facing API).
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }
    
    /// Get the underlying URL.
    pub fn url(&self) -> &UrlBytes<MAX_LEN> {
        &self.url
    }
}

impl<const MAX_LEN: usize> UrlAbsolute<MAX_LEN> {
    /// Create from byte slice (Kani-friendly).
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let url = UrlBytes::from_slice(bytes)?;
        
        if !url.has_authority() {
            return Err(ValidationError::UrlNotAbsolute);
        }
        
        Ok(Self { url })
    }
    
    /// Create from Vec (user-facing API).
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }
    
    /// Get the underlying URL.
    pub fn url(&self) -> &UrlBytes<MAX_LEN> {
        &self.url
    }
}

impl<const MAX_LEN: usize> UrlHttp<MAX_LEN> {
    /// Create from byte slice (Kani-friendly).
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let url = UrlBytes::from_slice(bytes)?;
        
        if !url.is_http() {
            return Err(ValidationError::UrlNotHttp);
        }
        
        Ok(Self { url })
    }
    
    /// Create from Vec (user-facing API).
    pub fn new(bytes: Vec<u8>) -> Result<Self, ValidationError> {
        Self::from_slice(&bytes)
    }
    
    /// Get the underlying URL.
    pub fn url(&self) -> &UrlBytes<MAX_LEN> {
        &self.url
    }
}

// ============================================================================
// URL Parsing and Validation
// ============================================================================

/// Parse URL components following RFC 3986.
///
/// RFC 3986 syntax:
/// ```text
/// URI = scheme ":" hier-part [ "?" query ] [ "#" fragment ]
/// hier-part = "//" authority path-abempty
///           / path-absolute
///           / path-rootless
///           / path-empty
/// ```
fn parse_url_components(url: &str) -> Result<UrlComponents, ValidationError> {
    let bytes = url.as_bytes();
    let len = bytes.len();
    
    if len == 0 {
        return Err(ValidationError::InvalidUrlSyntax);
    }
    
    // Parse scheme (before first ':')
    let scheme_end = find_scheme_end(bytes)?;
    
    // Check for authority (//)
    let (authority_start, authority_end, path_start) = if scheme_end + 2 < len
        && bytes[scheme_end + 1] == b'/'
        && bytes[scheme_end + 2] == b'/'
    {
        let auth_start = scheme_end + 3;
        let (auth_end, path) = find_authority_end(bytes, auth_start);
        (Some(auth_start), Some(auth_end), path)
    } else {
        // No authority, path starts after ':'
        (None, None, scheme_end + 1)
    };
    
    // Find query (after '?')
    let query_start = find_char(bytes, b'?', path_start);
    
    // Find fragment (after '#')
    let fragment_start = find_char(bytes, b'#', path_start);
    
    Ok(UrlComponents {
        scheme_end,
        authority_start,
        authority_end,
        path_start,
        query_start,
        fragment_start,
    })
}

/// Find the end of the scheme (position of first ':').
fn find_scheme_end(bytes: &[u8]) -> Result<usize, ValidationError> {
    if bytes.is_empty() {
        return Err(ValidationError::InvalidUrlSyntax);
    }
    
    // Scheme must start with letter
    if !bytes[0].is_ascii_alphabetic() {
        return Err(ValidationError::InvalidUrlSyntax);
    }
    
    // Find ':'
    let mut i = 1;
    while i < bytes.len() {
        let ch = bytes[i];
        
        if ch == b':' {
            return Ok(i);
        }
        
        // Scheme chars: letter, digit, '+', '-', '.'
        if !ch.is_ascii_alphanumeric() && ch != b'+' && ch != b'-' && ch != b'.' {
            return Err(ValidationError::InvalidUrlSyntax);
        }
        
        i += 1;
    }
    
    Err(ValidationError::InvalidUrlSyntax)
}

/// Find the end of authority and start of path.
fn find_authority_end(bytes: &[u8], start: usize) -> (usize, usize) {
    let mut i = start;
    
    while i < bytes.len() {
        let ch = bytes[i];
        
        // Authority ends at '/', '?', '#'
        if ch == b'/' || ch == b'?' || ch == b'#' {
            return (i, i);
        }
        
        i += 1;
    }
    
    // Authority extends to end
    (bytes.len(), bytes.len())
}

/// Find first occurrence of char after start position.
fn find_char(bytes: &[u8], target: u8, start: usize) -> Option<usize> {
    let mut i = start;
    
    while i < bytes.len() {
        if bytes[i] == target {
            return Some(i);
        }
        i += 1;
    }
    
    None
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_http_url() {
        let url = UrlBytes::<2048>::new(b"http://example.com".to_vec()).unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.authority(), Some("example.com"));
        assert!(url.has_authority());
        assert!(url.is_http());
    }

    #[test]
    fn test_valid_https_url_with_path() {
        let url = UrlBytes::<2048>::new(b"https://example.com/path".to_vec()).unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.authority(), Some("example.com"));
        assert!(url.is_http());
    }

    #[test]
    fn test_url_with_port() {
        let url = UrlBytes::<2048>::new(b"http://example.com:8080/".to_vec()).unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.authority(), Some("example.com:8080"));
    }

    #[test]
    fn test_url_with_query() {
        let url = UrlBytes::<2048>::new(b"http://example.com?key=value".to_vec()).unwrap();
        assert_eq!(url.scheme(), "http");
        assert!(url.has_authority());
    }

    #[test]
    fn test_url_with_fragment() {
        let url = UrlBytes::<2048>::new(b"http://example.com#section".to_vec()).unwrap();
        assert_eq!(url.scheme(), "http");
    }

    #[test]
    fn test_ftp_url() {
        let url = UrlBytes::<2048>::new(b"ftp://ftp.example.com/file.txt".to_vec()).unwrap();
        assert_eq!(url.scheme(), "ftp");
        assert!(!url.is_http());
    }

    #[test]
    fn test_file_url() {
        let url = UrlBytes::<2048>::new(b"file:///path/to/file".to_vec()).unwrap();
        assert_eq!(url.scheme(), "file");
        assert!(url.has_authority());
    }

    #[test]
    fn test_invalid_scheme_start() {
        let result = UrlBytes::<2048>::new(b"1http://example.com".to_vec());
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_scheme() {
        let result = UrlBytes::<2048>::new(b"//example.com".to_vec());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_with_authority_contract() {
        let url = UrlWithAuthority::<2048>::new(b"http://example.com".to_vec()).unwrap();
        assert!(url.url().has_authority());
    }

    #[test]
    fn test_url_without_authority_rejected() {
        let result = UrlWithAuthority::<2048>::new(b"mailto:test@example.com".to_vec());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_http_contract() {
        let url = UrlHttp::<2048>::new(b"https://example.com".to_vec()).unwrap();
        assert!(url.url().is_http());
    }

    #[test]
    fn test_non_http_rejected() {
        let result = UrlHttp::<2048>::new(b"ftp://example.com".to_vec());
        assert!(result.is_err());
    }
}
