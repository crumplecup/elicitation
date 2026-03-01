//! Response wrapper for reqwest HTTP responses.
//!
//! Demonstrates async consuming generic methods — the crown jewel of
//! `#[reflect_methods]` capabilities. Includes `Serialize` for use as
//! an MCP `Json<>` return type.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;
use serde::Serialize;

use crate::Error;

elicit_newtype!(reqwest::Response, as Response);

/// Serialize as a metadata summary for MCP tool return values.
impl Serialize for Response {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Response", 2)?;
        s.serialize_field("status", &self.0.status().as_u16())?;
        s.serialize_field("url", self.0.url().as_str())?;
        s.end()
    }
}

#[reflect_methods]
impl Response {
    /// Returns the HTTP status code as a `u16`.
    pub fn status(&self) -> u16 {
        self.0.status().as_u16()
    }

    /// Returns the final URL of the response (after redirects).
    pub fn url(&self) -> String {
        self.0.url().to_string()
    }

    /// Consume the response and return the body as text.
    pub async fn text(self) -> Result<String, Error> {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.text().await.map_err(Error::from)
    }

    /// Consume the response and deserialize the body as JSON.
    pub async fn json<T>(self) -> Result<T, Error>
    where
        T: elicitation::Elicitation + schemars::JsonSchema + serde::de::DeserializeOwned,
    {
        let inner = std::sync::Arc::try_unwrap(self.0)
            .expect("Consuming method requires exclusive ownership");
        inner.json::<T>().await.map_err(Error::from)
    }
}
