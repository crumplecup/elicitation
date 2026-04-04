//! RequestBuilder wrapper for reqwest request builder.
//!
//! Demonstrates consuming method support with non-Clone builder types.
//! The Arc wrapper enables Clone on the newtype while Arc::try_unwrap
//! recovers exclusive ownership for consuming builder methods.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

use crate::{Error, Response};

elicit_newtype!(reqwest::RequestBuilder, as RequestBuilder);

/// Serialize as an empty object — builder state is opaque with no public accessors.
impl serde::Serialize for RequestBuilder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let s = serializer.serialize_struct("RequestBuilder", 0)?;
        s.end()
    }
}

/// Deserialize from any object → reconstruct a placeholder `GET https://example.com` builder.
///
/// The actual URL/method are not recoverable post-construction; this is a snapshot
/// placeholder to satisfy the `ElicitComplete` supertrait.
impl<'de> serde::Deserialize<'de> for RequestBuilder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RequestBuilderVisitor;
        impl<'de> serde::de::Visitor<'de> for RequestBuilderVisitor {
            type Value = RequestBuilder;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(
                    "an object (HTTP request builder is reconstructed as example GET on deserialize)",
                )
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                while map
                    .next_entry::<serde::de::IgnoredAny, serde::de::IgnoredAny>()?
                    .is_some()
                {}
                Ok(reqwest::Client::new().get("https://example.com").into())
            }
        }
        deserializer.deserialize_map(RequestBuilderVisitor)
    }
}

impl elicitation::ElicitComplete for RequestBuilder {}

mod emit_impls {
    use super::RequestBuilder;
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for RequestBuilder {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! {
                ::elicit_reqwest::RequestBuilder::from(
                    ::reqwest::Client::new().get("https://example.com"),
                )
            }
        }
    }
}

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
