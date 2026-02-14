//! URL contract types for formal verification.
//!
//! This module provides contract types for URL validation using the `url` crate.

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
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts url crate's
/// parsing logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlValid(Url);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct UrlValid(std::marker::PhantomData<Url>);

#[cfg(not(kani))]
impl UrlValid {
    /// Create a new UrlValid from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlInvalid` if the URL cannot be parsed.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        Url::parse(value)
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

#[cfg(kani)]
impl UrlValid {
    /// Create a new UrlValid from a string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without URL parsing.
    pub fn new(_value: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        if is_valid {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlInvalid)
        }
    }

    /// Create a new UrlValid from an existing Url (Kani mode).
    pub fn from_url(_url: Url) -> Self {
        Self(std::marker::PhantomData)
    }

    /// Get a reference to the wrapped URL (not available in Kani mode).
    pub fn get(&self) -> &Url {
        panic!("UrlValid::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the URL (not available in Kani mode).
    pub fn into_inner(self) -> Url {
        panic!("UrlValid::into_inner() not available in Kani mode - use symbolic validation")
    }
}

/// A URL with HTTPS scheme.
///
/// This contract ensures the URL uses the HTTPS protocol for secure
/// communication.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts url crate's
/// parsing logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlHttps(Url);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct UrlHttps(std::marker::PhantomData<Url>);

#[cfg(not(kani))]
impl UrlHttps {
    /// Create a new UrlHttps from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNotHttps` if the URL scheme is not HTTPS.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let url = Url::parse(value).map_err(|_| ValidationError::UrlInvalid)?;

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

#[cfg(kani)]
impl UrlHttps {
    /// Create a new UrlHttps from a string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without URL parsing.
    pub fn new(_value: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        let is_https: bool = kani::any();

        if !is_valid {
            Err(ValidationError::UrlInvalid)
        } else if is_https {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlNotHttps)
        }
    }

    /// Create a new UrlHttps from an existing Url (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic.
    pub fn from_url(_url: Url) -> Result<Self, ValidationError> {
        let is_https: bool = kani::any();
        if is_https {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlNotHttps)
        }
    }

    /// Get a reference to the wrapped URL (not available in Kani mode).
    pub fn get(&self) -> &Url {
        panic!("UrlHttps::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the URL (not available in Kani mode).
    pub fn into_inner(self) -> Url {
        panic!("UrlHttps::into_inner() not available in Kani mode - use symbolic validation")
    }
}

/// A URL with HTTP scheme.
///
/// This contract ensures the URL uses the HTTP protocol.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts url crate's
/// parsing logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlHttp(Url);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct UrlHttp(std::marker::PhantomData<Url>);

#[cfg(not(kani))]
impl UrlHttp {
    /// Create a new UrlHttp from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNotHttp` if the URL scheme is not HTTP.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let url = Url::parse(value).map_err(|_| ValidationError::UrlInvalid)?;

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

#[cfg(kani)]
impl UrlHttp {
    /// Create a new UrlHttp from a string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without URL parsing.
    pub fn new(_value: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        let is_http: bool = kani::any();

        if !is_valid {
            Err(ValidationError::UrlInvalid)
        } else if is_http {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlNotHttp)
        }
    }

    /// Create a new UrlHttp from an existing Url (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic.
    pub fn from_url(_url: Url) -> Result<Self, ValidationError> {
        let is_http: bool = kani::any();
        if is_http {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlNotHttp)
        }
    }

    /// Get a reference to the wrapped URL (not available in Kani mode).
    pub fn get(&self) -> &Url {
        panic!("UrlHttp::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the URL (not available in Kani mode).
    pub fn into_inner(self) -> Url {
        panic!("UrlHttp::into_inner() not available in Kani mode - use symbolic validation")
    }
}

/// A URL with a host component.
///
/// This contract ensures the URL has a valid host (domain or IP address).
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts url crate's
/// parsing logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlWithHost(Url);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct UrlWithHost(std::marker::PhantomData<Url>);

#[cfg(not(kani))]
impl UrlWithHost {
    /// Create a new UrlWithHost from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlNoHost` if the URL has no host component.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let url = Url::parse(value).map_err(|_| ValidationError::UrlInvalid)?;

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

#[cfg(kani)]
impl UrlWithHost {
    /// Create a new UrlWithHost from a string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without URL parsing.
    pub fn new(_value: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        let has_host: bool = kani::any();

        if !is_valid {
            Err(ValidationError::UrlInvalid)
        } else if has_host {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlNoHost)
        }
    }

    /// Create a new UrlWithHost from an existing Url (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic.
    pub fn from_url(_url: Url) -> Result<Self, ValidationError> {
        let has_host: bool = kani::any();
        if has_host {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlNoHost)
        }
    }

    /// Get a reference to the wrapped URL (not available in Kani mode).
    pub fn get(&self) -> &Url {
        panic!("UrlWithHost::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the URL (not available in Kani mode).
    pub fn into_inner(self) -> Url {
        panic!("UrlWithHost::into_inner() not available in Kani mode - use symbolic validation")
    }
}

/// A URL that can be used as a base for relative URLs.
///
/// This contract ensures the URL can act as a base for resolving relative URLs.
///
/// # Kani Verification
///
/// In Kani mode, uses PhantomData and symbolic validation. Trusts url crate's
/// parsing logic, verifies only wrapper invariants.
#[cfg(not(kani))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UrlCanBeBase(Url);

#[cfg(kani)]
#[derive(Debug, Clone)]
pub struct UrlCanBeBase(std::marker::PhantomData<Url>);

#[cfg(not(kani))]
impl UrlCanBeBase {
    /// Create a new UrlCanBeBase from a string.
    ///
    /// # Errors
    ///
    /// Returns `ValidationError::UrlCannotBeBase` if the URL cannot be a base.
    pub fn new(value: &str) -> Result<Self, ValidationError> {
        let url = Url::parse(value).map_err(|_| ValidationError::UrlInvalid)?;

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

#[cfg(kani)]
impl UrlCanBeBase {
    /// Create a new UrlCanBeBase from a string (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic without URL parsing.
    pub fn new(_value: &str) -> Result<Self, ValidationError> {
        let is_valid: bool = kani::any();
        let can_be_base: bool = kani::any();

        if !is_valid {
            Err(ValidationError::UrlInvalid)
        } else if can_be_base {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlCannotBeBase)
        }
    }

    /// Create a new UrlCanBeBase from an existing Url (Kani mode).
    ///
    /// Uses symbolic boolean to verify wrapper logic.
    pub fn from_url(_url: Url) -> Result<Self, ValidationError> {
        let can_be_base: bool = kani::any();
        if can_be_base {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::UrlCannotBeBase)
        }
    }

    /// Get a reference to the wrapped URL (not available in Kani mode).
    pub fn get(&self) -> &Url {
        panic!("UrlCanBeBase::get() not available in Kani mode - use symbolic validation")
    }

    /// Unwrap the URL (not available in Kani mode).
    pub fn into_inner(self) -> Url {
        panic!("UrlCanBeBase::into_inner() not available in Kani mode - use symbolic validation")
    }
}

// ============================================================================
// Elicitation Implementations
// ============================================================================

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};

// Re-export UrlStyle from primitives
pub use crate::primitives::url::UrlStyle;

impl Prompt for UrlValid {
    fn prompt() -> Option<&'static str> {
        Some("Enter a valid URL:")
    }
}

impl Elicitation for UrlValid {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let prompt = Self::prompt().unwrap();
        tracing::debug!("Eliciting UrlValid with server-side send_prompt");

        // Use send_prompt for server-side compatibility
        let response = communicator.send_prompt(prompt).await?;

        // Parse the string as a URL
        let url = url::Url::parse(response.trim()).map_err(|e| {
            crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                "Invalid URL: {}",
                e
            )))
        })?;

        Ok(Self::from_url(url))
    }
}

impl Prompt for UrlHttps {
    fn prompt() -> Option<&'static str> {
        Some("Enter an HTTPS URL:")
    }
}

impl Elicitation for UrlHttps {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let value = url::Url::elicit(communicator).await?;
        Self::from_url(value).map_err(crate::ElicitError::from)
    }
}

impl Prompt for UrlHttp {
    fn prompt() -> Option<&'static str> {
        Some("Enter an HTTP or HTTPS URL:")
    }
}

impl Elicitation for UrlHttp {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let value = url::Url::elicit(communicator).await?;
        Self::from_url(value).map_err(crate::ElicitError::from)
    }
}

impl Prompt for UrlWithHost {
    fn prompt() -> Option<&'static str> {
        Some("Enter a URL with a host:")
    }
}

impl Elicitation for UrlWithHost {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let value = url::Url::elicit(communicator).await?;
        Self::from_url(value).map_err(crate::ElicitError::from)
    }
}

impl Prompt for UrlCanBeBase {
    fn prompt() -> Option<&'static str> {
        Some("Enter a base URL:")
    }
}

impl Elicitation for UrlCanBeBase {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let value = url::Url::elicit(communicator).await?;
        Self::from_url(value).map_err(crate::ElicitError::from)
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

        // Test from_url, get, and into_inner
        let url = Url::parse("http://example.com").unwrap();
        let http_url = UrlHttp::from_url(url).unwrap();
        assert_eq!(http_url.get().scheme(), "http");
        let inner = http_url.into_inner();
        assert_eq!(inner.as_str(), "http://example.com/");
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
