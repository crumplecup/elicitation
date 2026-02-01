//! URL byte validation following RFC 3986.
//!
//! This module provides byte-level validation for URLs using bounded components,
//! demonstrating that complex parsing can be verified tractably by decomposing
//! into constrained, bounded types.

use crate::verification::types::{Utf8Bytes, ValidationError};

// ============================================================================
// Bounded Component Types
// ============================================================================

/// URL scheme with bounded length (e.g., "http", "https", "ftp").
///
/// RFC 3986: scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemeBytes<const MAX_LEN: usize = 32> {
    utf8: Utf8Bytes<MAX_LEN>,
}

impl<const MAX_LEN: usize> SchemeBytes<MAX_LEN> {
    /// Create validated scheme from slice.
    ///
    /// RFC 3986 constraints:
    /// - Must start with letter
    /// - Contains only letters, digits, +, -, .
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();

        if len == 0 {
            return Err(ValidationError::InvalidUrlSyntax);
        }

        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }

        // For Kani: assert len is bounded to enable tractable verification
        #[cfg(kani)]
        kani::assume(len <= MAX_LEN);

        // First character must be letter
        if !bytes[0].is_ascii_alphabetic() {
            return Err(ValidationError::InvalidUrlSyntax);
        }

        // Validate all characters (manual loop for Kani)
        let mut i = 0;
        while i < len {
            if !is_valid_scheme_char(bytes[i]) {
                return Err(ValidationError::InvalidUrlSyntax);
            }
            i += 1;
        }

        // Copy to fixed array
        let mut fixed = [0u8; MAX_LEN];
        fixed[..len].copy_from_slice(bytes);

        let utf8 = Utf8Bytes::new(fixed, len)?;
        Ok(Self { utf8 })
    }

    /// Get scheme as string slice.
    pub fn as_str(&self) -> &str {
        self.utf8.as_str()
    }

    /// Check if scheme is HTTP or HTTPS.
    pub fn is_http(&self) -> bool {
        let s = self.as_str();
        s == "http" || s == "https"
    }
}

/// URL authority with bounded length (e.g., "example.com:8080").
///
/// RFC 3986: authority = [ userinfo "@" ] host [ ":" port ]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityBytes<const MAX_LEN: usize = 256> {
    utf8: Utf8Bytes<MAX_LEN>,
}

impl<const MAX_LEN: usize> AuthorityBytes<MAX_LEN> {
    /// Create validated authority from slice.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, ValidationError> {
        let len = bytes.len();

        if len > MAX_LEN {
            return Err(ValidationError::TooLong {
                max: MAX_LEN,
                actual: len,
            });
        }

        // Copy to fixed array
        let mut fixed = [0u8; MAX_LEN];
        if len > 0 {
            fixed[..len].copy_from_slice(bytes);
        }

        let utf8 = Utf8Bytes::new(fixed, len)?;
        Ok(Self { utf8 })
    }

    /// Get authority as string slice.
    pub fn as_str(&self) -> &str {
        self.utf8.as_str()
    }

    /// Check if authority is empty.
    pub fn is_empty(&self) -> bool {
        self.utf8.is_empty()
    }
}

// ============================================================================
// Core URL Type (Bounded Components)
// ============================================================================

/// Validated URL bytes with bounded components (RFC 3986 syntax).
///
/// Architecture:
/// - Scheme: Bounded, validated per RFC 3986
/// - Authority: Bounded, optional
/// - Path/Query/Fragment: Stored as offsets into original buffer
///
/// This enables tractable Kani proofs by bounding component exploration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlBytes<
    const SCHEME_MAX: usize = 32,
    const AUTHORITY_MAX: usize = 256,
    const MAX_LEN: usize = 2048,
> {
    /// Full URL (UTF-8 validated)
    utf8: Utf8Bytes<MAX_LEN>,
    /// Validated scheme
    scheme: SchemeBytes<SCHEME_MAX>,
    /// Optional authority
    authority: Option<AuthorityBytes<AUTHORITY_MAX>>,
}

// ============================================================================
// URL Implementation
// ============================================================================

impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
    UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
{
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

        // Copy to fixed array and validate UTF-8
        let mut fixed = [0u8; MAX_LEN];
        fixed[..len].copy_from_slice(bytes);
        let utf8 = Utf8Bytes::new(fixed, len)?;

        // Parse components (bounded)
        let (scheme, authority) = parse_url_bounded::<SCHEME_MAX, AUTHORITY_MAX>(bytes)?;

        Ok(Self {
            utf8,
            scheme,
            authority,
        })
    }

    /// Create from Vec (user-facing API, delegates to from_slice).
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

    /// Get the scheme.
    pub fn scheme(&self) -> &str {
        self.scheme.as_str()
    }

    /// Get the authority if present.
    pub fn authority(&self) -> Option<&str> {
        self.authority.as_ref().map(|a| a.as_str())
    }

    /// Check if URL has authority (starts with //).
    pub fn has_authority(&self) -> bool {
        self.authority.is_some()
    }

    /// Check if URL has HTTP or HTTPS scheme.
    pub fn is_http(&self) -> bool {
        self.scheme.is_http()
    }

    /// Convert to String.
    pub fn to_string(&self) -> String {
        self.utf8.to_string()
    }
}

// ============================================================================
// Contract Types
// ============================================================================

/// URL with authority (has //).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlWithAuthority<
    const SCHEME_MAX: usize = 32,
    const AUTHORITY_MAX: usize = 256,
    const MAX_LEN: usize = 2048,
> {
    url: UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>,
}

impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
    UrlWithAuthority<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
{
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
    pub fn url(&self) -> &UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN> {
        &self.url
    }
}

/// Absolute URL (has scheme + authority).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlAbsolute<
    const SCHEME_MAX: usize = 32,
    const AUTHORITY_MAX: usize = 256,
    const MAX_LEN: usize = 2048,
> {
    url: UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>,
}

impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
    UrlAbsolute<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
{
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
    pub fn url(&self) -> &UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN> {
        &self.url
    }
}

/// URL with HTTP/HTTPS scheme.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UrlHttp<
    const SCHEME_MAX: usize = 32,
    const AUTHORITY_MAX: usize = 256,
    const MAX_LEN: usize = 2048,
> {
    url: UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>,
}

impl<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize, const MAX_LEN: usize>
    UrlHttp<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN>
{
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
    pub fn url(&self) -> &UrlBytes<SCHEME_MAX, AUTHORITY_MAX, MAX_LEN> {
        &self.url
    }
}

// ============================================================================
// Bounded URL Parsing
// ============================================================================

/// Parse URL into bounded components.
/// Parse URL into bounded components.
fn parse_url_bounded<const SCHEME_MAX: usize, const AUTHORITY_MAX: usize>(
    bytes: &[u8],
) -> Result<
    (
        SchemeBytes<SCHEME_MAX>,
        Option<AuthorityBytes<AUTHORITY_MAX>>,
    ),
    ValidationError,
> {
    if bytes.is_empty() {
        return Err(ValidationError::InvalidUrlSyntax);
    }

    // Find scheme end (before ':')
    let scheme_end = find_scheme_end(bytes)?;

    // Extract scheme into bounded buffer
    let scheme = SchemeBytes::from_slice(&bytes[..scheme_end])?;

    // Check for authority (//)
    let authority = if scheme_end + 2 < bytes.len()
        && bytes[scheme_end + 1] == b'/'
        && bytes[scheme_end + 2] == b'/'
    {
        let auth_start = scheme_end + 3;
        let auth_end = find_authority_end(bytes, auth_start);

        if auth_end > auth_start {
            Some(AuthorityBytes::from_slice(&bytes[auth_start..auth_end])?)
        } else {
            // Empty authority is valid (e.g., file:///)
            Some(AuthorityBytes::from_slice(&[])?)
        }
    } else {
        None
    };

    Ok((scheme, authority))
}

/// Find the end of the scheme (position of first ':').
fn find_scheme_end(bytes: &[u8]) -> Result<usize, ValidationError> {
    if bytes.is_empty() || !bytes[0].is_ascii_alphabetic() {
        return Err(ValidationError::InvalidUrlSyntax);
    }

    let mut i = 1;
    while i < bytes.len() {
        let ch = bytes[i];

        if ch == b':' {
            return Ok(i);
        }

        if !is_valid_scheme_char(ch) {
            return Err(ValidationError::InvalidUrlSyntax);
        }

        i += 1;
    }

    Err(ValidationError::InvalidUrlSyntax)
}

/// Find the end of authority and start of path.
fn find_authority_end(bytes: &[u8], start: usize) -> usize {
    let mut i = start;

    while i < bytes.len() {
        let ch = bytes[i];

        // Authority ends at '/', '?', '#'
        if ch == b'/' || ch == b'?' || ch == b'#' {
            return i;
        }

        i += 1;
    }

    // Authority extends to end
    bytes.len()
}

/// Check if character is valid in scheme.
fn is_valid_scheme_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'+' || b == b'-' || b == b'.'
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_http_url() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"http://example.com").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.authority(), Some("example.com"));
        assert!(url.has_authority());
        assert!(url.is_http());
    }

    #[test]
    fn test_valid_https_url_with_path() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"https://example.com/path").unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.authority(), Some("example.com"));
        assert!(url.is_http());
    }

    #[test]
    fn test_url_with_port() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"http://example.com:8080/").unwrap();
        assert_eq!(url.scheme(), "http");
        assert_eq!(url.authority(), Some("example.com:8080"));
    }

    #[test]
    fn test_url_with_query() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"http://example.com?key=value").unwrap();
        assert_eq!(url.scheme(), "http");
        assert!(url.has_authority());
    }

    #[test]
    fn test_url_with_fragment() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"http://example.com#section").unwrap();
        assert_eq!(url.scheme(), "http");
    }

    #[test]
    fn test_ftp_url() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"ftp://ftp.example.com/file.txt").unwrap();
        assert_eq!(url.scheme(), "ftp");
        assert!(!url.is_http());
    }

    #[test]
    fn test_file_url() {
        let url = UrlBytes::<32, 256, 2048>::from_slice(b"file:///path/to/file").unwrap();
        assert_eq!(url.scheme(), "file");
        assert!(url.has_authority());
    }

    #[test]
    fn test_invalid_scheme_start() {
        let result = UrlBytes::<32, 256, 2048>::from_slice(b"1http://example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_scheme() {
        let result = UrlBytes::<32, 256, 2048>::from_slice(b"//example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_url_with_authority_contract() {
        let url = UrlWithAuthority::<32, 256, 2048>::from_slice(b"http://example.com").unwrap();
        assert!(url.url().has_authority());
    }

    #[test]
    fn test_url_without_authority_rejected() {
        let result = UrlWithAuthority::<32, 256, 2048>::from_slice(b"mailto:test@example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_url_http_contract() {
        let url = UrlHttp::<32, 256, 2048>::from_slice(b"https://example.com").unwrap();
        assert!(url.url().is_http());
    }

    #[test]
    fn test_non_http_rejected() {
        let result = UrlHttp::<32, 256, 2048>::from_slice(b"ftp://example.com");
        assert!(result.is_err());
    }
}
