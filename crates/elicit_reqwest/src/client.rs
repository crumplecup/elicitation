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

/// Serialize as an empty object — the client holds a connection pool with no
/// observable configuration that survives serialization.
impl serde::Serialize for Client {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let s = serializer.serialize_struct("Client", 0)?;
        s.end()
    }
}

/// Deserialize from any object → reconstruct a default `Client`.
impl<'de> serde::Deserialize<'de> for Client {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ClientVisitor;
        impl<'de> serde::de::Visitor<'de> for ClientVisitor {
            type Value = Client;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an object (HTTP client is reconstructed as default on deserialize)")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                while map
                    .next_entry::<serde::de::IgnoredAny, serde::de::IgnoredAny>()?
                    .is_some()
                {}
                Ok(Client::new())
            }
        }
        deserializer.deserialize_map(ClientVisitor)
    }
}

impl elicitation::ElicitComplete for Client {}

mod emit_impls {
    use super::Client;
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for Client {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_reqwest::Client::new() }
        }
    }
}

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
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.0.get(url).into()
    }

    /// Start building a POST request to `url`.
    pub fn post<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.0.post(url).into()
    }

    /// Start building a PUT request to `url`.
    pub fn put<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.0.put(url).into()
    }

    /// Start building a DELETE request to `url`.
    pub fn delete<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.0.delete(url).into()
    }

    /// Start building a PATCH request to `url`.
    pub fn patch<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.0.patch(url).into()
    }

    /// Start building a HEAD request to `url`.
    pub fn head<U>(&self, url: U) -> RequestBuilder
    where
        U: elicitation::ElicitComplete + reqwest::IntoUrl,
    {
        self.0.head(url).into()
    }
}
