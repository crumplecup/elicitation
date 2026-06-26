//! Newtypes for reqwest/http types that add `JsonSchema` + `Serialize` + `Deserialize`.

use elicitation::{
    Elicit, ElicitCommunicator, ElicitIntrospect, ElicitPromptTree, ElicitResult, ElicitSpec,
    Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, proc_macro2::TokenStream,
};
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP header value:")]
struct HeaderValueData {
    #[prompt("Raw header-value bytes:")]
    bytes: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("UTF-8 header text, when representable:")]
    text: Option<String>,
}

impl From<&http::HeaderValue> for HeaderValueData {
    fn from(value: &http::HeaderValue) -> Self {
        Self {
            bytes: value.as_bytes().to_vec(),
            text: value.to_str().ok().map(str::to_string),
        }
    }
}

/// HTTP header value newtype with byte-faithful serde and schema support.
#[derive(Debug, Clone)]
pub struct HeaderValue(pub Arc<http::HeaderValue>);

impl Serialize for HeaderValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        HeaderValueData::from(self.0.as_ref()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HeaderValue {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = HeaderValueData::deserialize(d)?;
        http::HeaderValue::from_bytes(&value.bytes)
            .map(Self::from)
            .map_err(serde::de::Error::custom)
    }
}

impl JsonSchema for HeaderValue {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HeaderValue".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        HeaderValueData::json_schema(generator)
    }
}

impl Prompt for HeaderValue {
    fn prompt() -> Option<&'static str> {
        Some("Describe an HTTP header value:")
    }
}

impl Elicitation for HeaderValue {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let data = HeaderValueData::elicit(communicator).await?;
        let value = http::HeaderValue::from_bytes(&data.bytes).map_err(|error| {
            elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                error.to_string(),
            ))
        })?;
        Ok(Self::from(value))
    }

    fn kani_proof() -> TokenStream {
        HeaderValueData::kani_proof()
    }

    fn verus_proof() -> TokenStream {
        HeaderValueData::verus_proof()
    }

    fn creusot_proof() -> TokenStream {
        HeaderValueData::creusot_proof()
    }
}

impl ElicitIntrospect for HeaderValue {
    fn pattern() -> elicitation::ElicitationPattern {
        HeaderValueData::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "HeaderValue",
            description: Self::prompt(),
            details: HeaderValueData::metadata().details,
        }
    }
}

impl ElicitPromptTree for HeaderValue {
    fn prompt_tree() -> PromptTree {
        match HeaderValueData::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "HeaderValue".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for HeaderValue {
    fn type_spec() -> TypeSpec {
        let base = HeaderValueData::type_spec();
        TypeSpec::new(
            "HeaderValue",
            "HTTP header value stored as raw bytes with optional UTF-8 text for display.",
            base.categories().clone(),
        )
    }
}

impl std::ops::Deref for HeaderValue {
    type Target = http::HeaderValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HeaderValue {
    /// Construct a header value from a static string.
    pub fn from_static(src: &'static str) -> Self {
        Self::from(http::HeaderValue::from_static(src))
    }

    /// Construct a header value from a validated string.
    pub fn from_str(src: &str) -> Result<Self, http::header::InvalidHeaderValue> {
        http::HeaderValue::from_str(src).map(Self::from)
    }

    /// Construct a header value from raw bytes.
    pub fn from_bytes(src: &[u8]) -> Result<Self, http::header::InvalidHeaderValue> {
        http::HeaderValue::from_bytes(src).map(Self::from)
    }

    /// Borrow the raw header bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Borrow the UTF-8 header text when representable.
    pub fn to_str(&self) -> Result<&str, http::header::ToStrError> {
        self.0.to_str()
    }
}

impl From<http::HeaderValue> for HeaderValue {
    fn from(value: http::HeaderValue) -> Self {
        Self(Arc::new(value))
    }
}

impl From<HeaderValue> for http::HeaderValue {
    fn from(value: HeaderValue) -> Self {
        Arc::try_unwrap(value.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

/// HTTP header map newtype with `JsonSchema` and serde support.
///
/// Serializes as a JSON object mapping header names to their first string value.
/// Multiple values for the same header are collapsed to the first.
#[derive(Debug, Clone)]
pub struct HeaderMap(pub Arc<http::HeaderMap>);

impl Default for HeaderMap {
    fn default() -> Self {
        Self(Arc::new(http::HeaderMap::new()))
    }
}

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

// ── Elicitation impls ─────────────────────────────────────────────────────────
//
// Method, StatusCode, Version, and HeaderMap are hand-crafted newtypes that
// predate `elicit_newtype!`.  We manually add the Elicitation family of traits
// so each type participates in the elicitation family without relying on the
// derive macros yet.

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
                elicitation::TypeSpec::new($tn, $desc, Vec::new())
            }
        }

        impl elicitation::ElicitPromptTree for $Type {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: $desc.to_string(),
                    type_name: $tn.to_string(),
                }
            }
        }
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
