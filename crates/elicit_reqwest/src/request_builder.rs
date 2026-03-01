//! RequestBuilder wrapper for reqwest request builder.
//!
//! Demonstrates consuming method support with non-Clone builder types.
//! The Arc wrapper enables Clone on the newtype while Arc::try_unwrap
//! recovers exclusive ownership for consuming builder methods.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

use crate::{Error, Response};

elicit_newtype!(reqwest::RequestBuilder, as RequestBuilder);

#[reflect_methods]
impl RequestBuilder {
    /// Set request timeout.
    pub fn timeout(self, timeout: std::time::Duration) -> Self {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.timeout(timeout).into()
    }

    /// Set Bearer token authorization header.
    pub fn bearer_auth(self, token: String) -> Self {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.bearer_auth(token).into()
    }

    /// Set Basic authorization credentials.
    pub fn basic_auth(self, username: String, password: Option<String>) -> Self {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.basic_auth(username, password).into()
    }

    /// Append a header to the request.
    pub fn header(self, key: String, value: String) -> Self {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.header(key, value).into()
    }

    /// Set the JSON body (serializes `value` as JSON).
    pub fn json<T>(self, value: &T) -> Self
    where
        T: elicitation::Elicitation + schemars::JsonSchema + serde::Serialize,
    {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.json(value).into()
    }

    /// Send the request and await the response.
    pub async fn send(self) -> Result<Response, Error> {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.send().await.map(Response::from).map_err(Error::from)
    }
}
