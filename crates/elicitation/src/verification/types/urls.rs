//! URL contract types for formal verification.
//!
//! This module provides contract types for URL validation using the `url` crate.

#![cfg(feature = "url")]

use crate::verification::types::ValidationError;
#[cfg(feature = "url")]
use url::Url;

// ============================================================================
// URL Contract Types
// ============================================================================

/// A valid URL.
///
/// This contract ensures the value is a valid, parseable URL according to
/// the WHATWG URL Standard.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlValid(Url);

impl UrlValid {
    /// Create a new UrlValid from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlInvalid` if the URL cannot be parsed.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        Url::parse(value.as_ref())
            .map(Self)
            .map_err(|_| ValidationError::UrlInvalid)
    }

    /// Create a new UrlValid from an existing Url.
    pub fn from_url(url: Url) -> Self {
        Self(url)
    }

    /// Get a reference to the wrapped URL.
    pub fn get(&self) -> &Url {
        &self.0
    }

    /// Unwrap the URL.
    pub fn into_inner(self) -> Url {
        self.0
    }
}

/// A URL with HTTPS scheme.
///
/// This contract ensures the URL uses the HTTPS protocol for secure
/// communication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlHttps(Url);

impl UrlHttps {
    /// Create a new UrlHttps from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNotHttps` if the URL scheme is not HTTPS.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let url = Url::parse(value.as_ref()).map_err(|_| ValidationError::UrlInvalid)?;

        if url.scheme() == "https" {
            Ok(Self(url))
        } else {
            Err(ValidationError::UrlNotHttps)
        }
    }

    /// Create a new UrlHttps from an existing Url.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNotHttps` if the URL scheme is not HTTPS.
    pub fn from_url(url: Url) -> Result<Self, ValidationError> {
        if url.scheme() == "https" {
            Ok(Self(url))
        } else {
            Err(ValidationError::UrlNotHttps)
        }
    }

    /// Get a reference to the wrapped URL.
    pub fn get(&self) -> &Url {
        &self.0
    }

    /// Unwrap the URL.
    pub fn into_inner(self) -> Url {
        self.0
    }
}

/// A URL with HTTP scheme.
///
/// This contract ensures the URL uses the HTTP protocol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlHttp(Url);

impl UrlHttp {
    /// Create a new UrlHttp from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNotHttp` if the URL scheme is not HTTP.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let url = Url::parse(value.as_ref()).map_err(|_| ValidationError::UrlInvalid)?;

        if url.scheme() == "http" {
            Ok(Self(url))
        } else {
            Err(ValidationError::UrlNotHttp)
        }
    }

    /// Create a new UrlHttp from an existing Url.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNotHttp` if the URL scheme is not HTTP.
    pub fn from_url(url: Url) -> Result<Self, ValidationError> {
        if url.scheme() == "http" {
            Ok(Self(url))
        } else {
            Err(ValidationError::UrlNotHttp)
        }
    }

    /// Get a reference to the wrapped URL.
    pub fn get(&self) -> &Url {
        &self.0
    }

    /// Unwrap the URL.
    pub fn into_inner(self) -> Url {
        self.0
    }
}

/// A URL with a host component.
///
/// This contract ensures the URL has a valid host (domain or IP address).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlWithHost(Url);

impl UrlWithHost {
    /// Create a new UrlWithHost from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNoHost` if the URL has no host component.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let url = Url::parse(value.as_ref()).map_err(|_| ValidationError::UrlInvalid)?;

        if url.host().is_some() {
            Ok(Self(url))
        } else {
            Err(ValidationError::UrlNoHost)
        }
    }

    /// Create a new UrlWithHost from an existing Url.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNoHost` if the URL has no host component.
    pub fn from_url(url: Url) -> Result<Self, ValidationError> {
        if url.host().is_some() {
            Ok(Self(url))
        } else {
            Err(ValidationError::UrlNoHost)
        }
    }

    /// Get a reference to the wrapped URL.
    pub fn get(&self) -> &Url {
        &self.0
    }

    /// Unwrap the URL.
    pub fn into_inner(self) -> Url {
        self.0
    }
}

/// A URL that can be used as a base for relative URLs.
///
/// This contract ensures the URL can act as a base for resolving relative URLs.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlCanBeBase(Url);

impl UrlCanBeBase {
    /// Create a new UrlCanBeBase from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlCannotBeBase` if the URL cannot be a base.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ValidationError> {
        let url = Url::parse(value.as_ref()).map_err(|_| ValidationError::UrlInvalid)?;

        if url.cannot_be_a_base() {
            Err(ValidationError::UrlCannotBeBase)
        } else {
            Ok(Self(url))
        }
    }

    /// Create a new UrlCanBeBase from an existing Url.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlCannotBeBase` if the URL cannot be a base.
    pub fn from_url(url: Url) -> Result<Self, ValidationError> {
        if url.cannot_be_a_base() {
            Err(ValidationError::UrlCannotBeBase)
        } else {
            Ok(Self(url))
        }
    }

    /// Get a reference to the wrapped URL.
    pub fn get(&self) -> &Url {
        &self.0
    }

    /// Unwrap the URL.
    pub fn into_inner(self) -> Url {
        self.0
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_valid() {
        assert!(UrlValid::new("https://example.com").is_ok());
        assert!(UrlValid::new("http://localhost:8080/path").is_ok());
        assert!(UrlValid::new("ftp://files.example.com").is_ok());
        assert!(UrlValid::new("not a url").is_err());
        assert!(UrlValid::new("").is_err());
    }

    #[test]
    fn test_url_https() {
        assert!(UrlHttps::new("https://example.com").is_ok());
        assert!(UrlHttps::new("https://secure.example.com/path?query=1").is_ok());
        assert!(UrlHttps::new("http://example.com").is_err());
        assert!(UrlHttps::new("ftp://example.com").is_err());
        assert!(UrlHttps::new("not a url").is_err());
    }

    #[test]
    fn test_url_http() {
        assert!(UrlHttp::new("http://example.com").is_ok());
        assert!(UrlHttp::new("http://localhost:8080/api").is_ok());
        assert!(UrlHttp::new("https://example.com").is_err());
        assert!(UrlHttp::new("ftp://example.com").is_err());
        assert!(UrlHttp::new("not a url").is_err());
    }

    #[test]
    fn test_url_with_host() {
        assert!(UrlWithHost::new("https://example.com").is_ok());
        assert!(UrlWithHost::new("http://192.168.1.1:8080").is_ok());
        assert!(UrlWithHost::new("mailto:user@example.com").is_err()); // No host
        assert!(UrlWithHost::new("data:text/plain,hello").is_err()); // No host
    }

    #[test]
    fn test_url_can_be_base() {
        assert!(UrlCanBeBase::new("https://example.com").is_ok());
        assert!(UrlCanBeBase::new("http://example.com/path/").is_ok());
        assert!(UrlCanBeBase::new("mailto:user@example.com").is_err()); // Cannot be base
        assert!(UrlCanBeBase::new("data:text/plain,hello").is_err()); // Cannot be base
    }

    #[test]
    fn test_url_accessors() {
        let url = UrlValid::new("https://example.com/path").unwrap();
        assert_eq!(url.get().scheme(), "https");
        assert_eq!(url.get().host_str(), Some("example.com"));

        let inner = url.into_inner();
        assert_eq!(inner.as_str(), "https://example.com/path");
    }
}
