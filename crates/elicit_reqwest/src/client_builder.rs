//! Shadow for `reqwest::ClientBuilder`.

use elicitation::Elicit;
use elicitation_derive::reflect_methods;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Error, HeaderMap, Policy, Proxy};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe a client-builder timeout:")]
struct DurationSpec {
    #[prompt("Whole seconds component:")]
    secs: u64,
    #[prompt("Nanoseconds component:")]
    nanos: u32,
}

impl DurationSpec {
    fn from_duration(timeout: std::time::Duration) -> Self {
        Self {
            secs: timeout.as_secs(),
            nanos: timeout.subsec_nanos(),
        }
    }

    fn to_duration(&self) -> std::time::Duration {
        std::time::Duration::new(self.secs, self.nanos)
    }
}

/// Owned `reqwest::ClientBuilder` recipe.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP client builder recipe:")]
pub struct ClientBuilder {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional User-Agent header value:")]
    user_agent: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional default header map:")]
    default_headers: Option<HeaderMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional overall request timeout:")]
    timeout: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional connect timeout:")]
    connect_timeout: Option<DurationSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Optional redirect policy:")]
    redirect: Option<Policy>,
    #[serde(default)]
    #[prompt("Zero or more configured proxies:")]
    proxies: Vec<Proxy>,
}

impl ClientBuilder {
    /// Construct a new client-builder recipe with reqwest defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the default User-Agent value.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Set default request headers.
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = Some(headers);
        self
    }

    /// Set the overall request timeout.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = Some(DurationSpec::from_duration(timeout));
        self
    }

    /// Set the connect timeout.
    pub fn connect_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.connect_timeout = Some(DurationSpec::from_duration(timeout));
        self
    }

    /// Set the redirect policy.
    pub fn redirect(mut self, policy: Policy) -> Self {
        self.redirect = Some(policy);
        self
    }

    /// Append a configured proxy.
    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxies.push(proxy);
        self
    }

    /// Rebuild a raw `reqwest::Client`.
    pub fn build(&self) -> Result<reqwest::Client, Error> {
        let mut builder = reqwest::Client::builder();

        if let Some(user_agent) = &self.user_agent {
            builder = builder.user_agent(user_agent.clone());
        }

        if let Some(headers) = &self.default_headers {
            builder = builder.default_headers(http::HeaderMap::from(headers.clone()));
        }

        if let Some(timeout) = &self.timeout {
            builder = builder.timeout(timeout.to_duration());
        }

        if let Some(connect_timeout) = &self.connect_timeout {
            builder = builder.connect_timeout(connect_timeout.to_duration());
        }

        if let Some(redirect) = &self.redirect {
            builder = builder.redirect(redirect.build_raw()?);
        }

        for proxy in &self.proxies {
            builder = builder.proxy(proxy.build_raw()?);
        }

        builder.build().map_err(Error::from)
    }
}

#[reflect_methods]
impl ClientBuilder {
    /// Borrow the configured proxies.
    pub fn proxies(&self) -> &[Proxy] {
        &self.proxies
    }
}
