//! Newtypes for reqwest/http types that add `JsonSchema` + `Serialize` + `Deserialize`.

use std::sync::Arc;

use schemars::{JsonSchema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// url::Url has Elicitation support via elicitation feature = "url"
pub use url::Url;

// ── Method ────────────────────────────────────────────────────────────────────

/// HTTP method newtype with `JsonSchema` and serde support.
///
/// Serializes to/from the uppercase method string (e.g. `"GET"`, `"POST"`).
#[derive(Debug, Clone)]
pub struct Method(pub Arc<reqwest::Method>);

impl JsonSchema for Method {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Method".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "description": "HTTP method (e.g. \"GET\", \"POST\", \"PUT\", \"DELETE\")"
        })
    }
}

impl Serialize for Method {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Method {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        reqwest::Method::from_bytes(s.as_bytes())
            .map(|m| m.into())
            .map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for Method {
    type Target = reqwest::Method;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<reqwest::Method> for Method {
    fn from(m: reqwest::Method) -> Self {
        Self(Arc::new(m))
    }
}

impl From<Method> for reqwest::Method {
    fn from(m: Method) -> Self {
        Arc::try_unwrap(m.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

// ── StatusCode ────────────────────────────────────────────────────────────────

/// HTTP status code newtype with `JsonSchema` and serde support.
///
/// Serializes to/from the integer status code (e.g. `200`, `404`).
#[derive(Debug, Clone)]
pub struct StatusCode(pub Arc<reqwest::StatusCode>);

impl JsonSchema for StatusCode {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "StatusCode".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "integer",
            "minimum": 100,
            "maximum": 599,
            "description": "HTTP status code (100–599)"
        })
    }
}

impl Serialize for StatusCode {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u16(self.0.as_u16())
    }
}

impl<'de> Deserialize<'de> for StatusCode {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let code = u16::deserialize(d)?;
        reqwest::StatusCode::from_u16(code)
            .map(|sc| sc.into())
            .map_err(serde::de::Error::custom)
    }
}

impl std::ops::Deref for StatusCode {
    type Target = reqwest::StatusCode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StatusCode {
    /// Construct a `StatusCode` from a raw `u16`.
    ///
    /// Returns an error if the value is not in the range 100–599.
    pub fn from_u16(src: u16) -> Result<Self, <reqwest::StatusCode as TryFrom<u16>>::Error> {
        reqwest::StatusCode::from_u16(src).map(|sc| Self(Arc::new(sc)))
    }
}

impl From<reqwest::StatusCode> for StatusCode {
    fn from(sc: reqwest::StatusCode) -> Self {
        Self(Arc::new(sc))
    }
}

impl From<StatusCode> for reqwest::StatusCode {
    fn from(sc: StatusCode) -> Self {
        Arc::try_unwrap(sc.0).unwrap_or_else(|arc| *arc)
    }
}

// ── Version ───────────────────────────────────────────────────────────────────

/// HTTP version newtype with `JsonSchema` and serde support.
///
/// Serializes to/from the canonical string (e.g. `"HTTP/1.1"`, `"HTTP/2.0"`).
#[derive(Debug, Clone)]
pub struct Version(pub Arc<reqwest::Version>);

impl JsonSchema for Version {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "Version".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "string",
            "enum": ["HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"],
            "description": "HTTP protocol version"
        })
    }
}

impl Serialize for Version {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{:?}", *self.0))
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let v = match s.as_str() {
            "HTTP/0.9" => reqwest::Version::HTTP_09,
            "HTTP/1.0" => reqwest::Version::HTTP_10,
            "HTTP/1.1" => reqwest::Version::HTTP_11,
            "HTTP/2.0" => reqwest::Version::HTTP_2,
            "HTTP/3.0" => reqwest::Version::HTTP_3,
            other => {
                return Err(serde::de::Error::custom(format!(
                    "unknown HTTP version: {other}"
                )));
            }
        };
        Ok(v.into())
    }
}

impl std::ops::Deref for Version {
    type Target = reqwest::Version;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<reqwest::Version> for Version {
    fn from(v: reqwest::Version) -> Self {
        Self(Arc::new(v))
    }
}

impl From<Version> for reqwest::Version {
    fn from(v: Version) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| *arc)
    }
}

// ── HeaderMap ─────────────────────────────────────────────────────────────────

/// HTTP header map newtype with `JsonSchema` and serde support.
///
/// Serializes as a JSON object mapping header names to their first string value.
/// Multiple values for the same header are collapsed to the first.
#[derive(Debug, Clone)]
pub struct HeaderMap(pub Arc<http::HeaderMap>);

impl JsonSchema for HeaderMap {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HeaderMap".into()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "type": "object",
            "additionalProperties": { "type": "string" },
            "description": "HTTP headers as a string-to-string map (first value per header name)"
        })
    }
}

impl Serialize for HeaderMap {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(self.0.len()))?;
        for (key, value) in self.0.iter() {
            let v = value.to_str().unwrap_or("<binary>");
            map.serialize_entry(key.as_str(), v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for HeaderMap {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw: std::collections::HashMap<String, String> =
            std::collections::HashMap::deserialize(d)?;
        let mut map = http::HeaderMap::new();
        for (k, v) in raw {
            let name = http::header::HeaderName::from_bytes(k.as_bytes())
                .map_err(serde::de::Error::custom)?;
            let value = http::HeaderValue::from_str(&v).map_err(serde::de::Error::custom)?;
            map.insert(name, value);
        }
        Ok(map.into())
    }
}

impl std::ops::Deref for HeaderMap {
    type Target = http::HeaderMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<http::HeaderMap> for HeaderMap {
    fn from(m: http::HeaderMap) -> Self {
        Self(Arc::new(m))
    }
}

impl From<HeaderMap> for http::HeaderMap {
    fn from(m: HeaderMap) -> Self {
        Arc::try_unwrap(m.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

// ── Elicitation / ElicitComplete impls ────────────────────────────────────────
//
// Method, StatusCode, Version, and HeaderMap are hand-crafted newtypes that
// predate `elicit_newtype!`.  We manually add the Elicitation family of traits
// so each type can satisfy the `ElicitComplete` supertrait.

macro_rules! impl_elicitation_for_reqwest_newtype {
    (
        $Type:ident,
        inner = $inner:expr,
        description = $desc:literal,
        type_name_str = $tn:literal $(,)?
    ) => {
        impl elicitation::Prompt for $Type {
            fn prompt() -> Option<&'static str> {
                Some($desc)
            }
        }

        impl elicitation::Elicitation for $Type {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Err(elicitation::ElicitError::new(
                    elicitation::ElicitErrorKind::ParseError(
                        concat!(
                            "`",
                            $tn,
                            "` cannot be interactively elicited — construct it directly."
                        )
                        .to_string(),
                    ),
                ))
            }

            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque($tn)
            }

            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque($tn)
            }

            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque($tn)
            }
        }

        impl elicitation::ElicitIntrospect for $Type {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: $tn,
                    description: Some($desc),
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }

        impl elicitation::ElicitSpec for $Type {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name($tn.to_string())
                    .summary($desc.to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $Type {}
    };
}

impl_elicitation_for_reqwest_newtype!(
    Method,
    inner = "reqwest::Method",
    description = "HTTP method (e.g. GET, POST, PUT, DELETE)",
    type_name_str = "Method",
);

impl_elicitation_for_reqwest_newtype!(
    StatusCode,
    inner = "reqwest::StatusCode",
    description = "HTTP status code (100–599)",
    type_name_str = "StatusCode",
);

impl_elicitation_for_reqwest_newtype!(
    Version,
    inner = "reqwest::Version",
    description = "HTTP protocol version (e.g. HTTP/1.1, HTTP/2.0)",
    type_name_str = "Version",
);

impl_elicitation_for_reqwest_newtype!(
    HeaderMap,
    inner = "http::HeaderMap",
    description = "HTTP headers as a string-to-string map",
    type_name_str = "HeaderMap",
);

// ── ToCodeLiteral impls ────────────────────────────────────────────────────────

mod emit_impls {
    use super::{HeaderMap, Method, StatusCode, Version};
    use elicitation::emit_code::ToCodeLiteral;
    use elicitation::proc_macro2::TokenStream;

    impl ToCodeLiteral for Method {
        fn to_code_literal(&self) -> TokenStream {
            let s = self.0.as_str().to_string();
            quote::quote! {
                ::elicit_reqwest::Method::from(
                    ::reqwest::Method::from_bytes(#s.as_bytes()).expect("valid HTTP method")
                )
            }
        }
    }

    impl ToCodeLiteral for StatusCode {
        fn to_code_literal(&self) -> TokenStream {
            let n = self.0.as_u16();
            quote::quote! {
                ::elicit_reqwest::StatusCode::from_u16(#n).expect("valid status code")
            }
        }
    }

    impl ToCodeLiteral for Version {
        fn to_code_literal(&self) -> TokenStream {
            let s = format!("{:?}", *self.0);
            let variant: TokenStream = match s.as_str() {
                "HTTP/0.9" => quote::quote! { ::reqwest::Version::HTTP_09 },
                "HTTP/1.0" => quote::quote! { ::reqwest::Version::HTTP_10 },
                "HTTP/1.1" => quote::quote! { ::reqwest::Version::HTTP_11 },
                "HTTP/2.0" => quote::quote! { ::reqwest::Version::HTTP_2 },
                "HTTP/3.0" => quote::quote! { ::reqwest::Version::HTTP_3 },
                _ => quote::quote! { ::reqwest::Version::HTTP_11 },
            };
            quote::quote! { ::elicit_reqwest::Version::from(#variant) }
        }
    }

    impl ToCodeLiteral for HeaderMap {
        fn to_code_literal(&self) -> TokenStream {
            let entries: Vec<_> = self
                .0
                .iter()
                .map(|(k, v)| {
                    let key = k.as_str();
                    let val = v.to_str().unwrap_or("");
                    quote::quote! { (#key, #val) }
                })
                .collect();
            quote::quote! {
                {
                    let mut __map = ::http::HeaderMap::new();
                    #(
                        __map.insert(
                            ::http::header::HeaderName::from_bytes(#entries.0.as_bytes())
                                .expect("valid header name"),
                            ::http::HeaderValue::from_str(#entries.1)
                                .expect("valid header value"),
                        );
                    )*
                    ::elicit_reqwest::HeaderMap::from(__map)
                }
            }
        }
    }
}
