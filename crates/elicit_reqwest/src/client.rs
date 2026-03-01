//! Client wrapper for reqwest HTTP client.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Client
//! with MCP tool generation for all HTTP methods.
//!
//! All HTTP method wrappers are generic over `U: IntoUrl + Elicitation + JsonSchema`,
//! demonstrating the `#[reflect_methods]` generic support added to the derive macro.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

use crate::RequestBuilder;

elicit_newtype!(reqwest::Client, as Client);

impl Client {
    /// Creates a new HTTP client with default settings.
    pub fn new() -> Self {
        reqwest::Client::new().into()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[reflect_methods]
impl Client {
    /// Start building a GET request to `url`.
    pub fn get<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
    {
        self.0.get(url).into()
    }

    /// Start building a POST request to `url`.
    pub fn post<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
    {
        self.0.post(url).into()
    }

    /// Start building a PUT request to `url`.
    pub fn put<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
    {
        self.0.put(url).into()
    }

    /// Start building a DELETE request to `url`.
    pub fn delete<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
    {
        self.0.delete(url).into()
    }

    /// Start building a PATCH request to `url`.
    pub fn patch<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
    {
        self.0.patch(url).into()
    }

    /// Start building a HEAD request to `url`.
    pub fn head<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
    {
        self.0.head(url).into()
    }
}
