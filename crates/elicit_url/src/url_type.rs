//! `Url` — elicitation-enabled wrapper around `url::Url`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(url::Url, as Url, serde);
elicit_newtype_traits!(Url, url::Url, [cmp, display, from_str]);

#[reflect_methods]
impl Url {
    /// Returns the full URL string.
    #[instrument(skip(self))]
    pub fn as_str(&self) -> String {
        self.0.as_str().to_string()
    }

    /// Returns the scheme (e.g. `"https"`, `"ftp"`).
    #[instrument(skip(self))]
    pub fn scheme(&self) -> String {
        self.0.scheme().to_string()
    }

    /// Returns the host string, or `None` for URLs without a host.
    #[instrument(skip(self))]
    pub fn host(&self) -> Option<String> {
        self.0.host_str().map(str::to_string)
    }

    /// Returns the explicit port, or `None` if using the default for the scheme.
    #[instrument(skip(self))]
    pub fn port(&self) -> Option<u16> {
        self.0.port()
    }

    /// Returns the port, falling back to the well-known default for the scheme.
    #[instrument(skip(self))]
    pub fn port_or_default(&self) -> Option<u16> {
        self.0.port_or_known_default()
    }

    /// Returns the path component (e.g. `"/api/v1/users"`).
    #[instrument(skip(self))]
    pub fn path(&self) -> String {
        self.0.path().to_string()
    }

    /// Returns the query string, or `None` if absent.
    #[instrument(skip(self))]
    pub fn query(&self) -> Option<String> {
        self.0.query().map(str::to_string)
    }

    /// Returns the fragment (the part after `#`), or `None` if absent.
    #[instrument(skip(self))]
    pub fn fragment(&self) -> Option<String> {
        self.0.fragment().map(str::to_string)
    }

    /// Returns the username, or an empty string if absent.
    #[instrument(skip(self))]
    pub fn username(&self) -> String {
        self.0.username().to_string()
    }

    /// Returns `true` if the URL has an authority component (host/user/port).
    #[instrument(skip(self))]
    pub fn has_authority(&self) -> bool {
        self.0.has_authority()
    }

    /// Resolves `input` relative to this URL. Returns `None` if parsing fails.
    #[instrument(skip(self))]
    pub fn join(&self, input: String) -> Option<String> {
        self.0.join(&input).ok().map(|u| u.to_string())
    }

    /// Returns the origin as a string (e.g. `"https://example.com:443"`).
    #[instrument(skip(self))]
    pub fn origin(&self) -> String {
        self.0.origin().ascii_serialization()
    }
}

impl Url {
    /// Parse a URL string. Returns `None` if the string is not a valid URL.
    pub fn parse(s: &str) -> Option<Self> {
        url::Url::parse(s)
            .ok()
            .map(|u| std::sync::Arc::new(u).into())
    }
}
