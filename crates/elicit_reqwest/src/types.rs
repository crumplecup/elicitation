//! Newtypes for reqwest/http types that add `JsonSchema` + `Serialize` + `Deserialize`.

use elicitation::{
    ElicitCommunicator, ElicitComplete, ElicitError, ElicitErrorKind, ElicitResult, elicit_newtype,
    emit_code::ToCodeLiteral, proc_macro2::TokenStream,
};
use schemars::SchemaGenerator;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

// elicit_url::Url is the ecosystem shadow type for url::Url
pub use elicit_url::Url;

// ── Schema helpers ────────────────────────────────────────────────────────────

fn method_json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "description": "HTTP method (e.g. \"GET\", \"POST\", \"PUT\", \"DELETE\")"
    })
}

fn status_code_json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "integer",
        "minimum": 100,
        "maximum": 599,
        "description": "HTTP status code (100–599)"
    })
}

fn version_json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "string",
        "enum": ["HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0", "HTTP/3.0"],
        "description": "HTTP protocol version"
    })
}

fn header_map_json_schema(_gen: &mut SchemaGenerator) -> schemars::Schema {
    schemars::json_schema!({
        "type": "object",
        "additionalProperties": { "type": "string" },
        "description": "HTTP headers as a string-to-string map (first value per header name)"
    })
}

// ── Method ────────────────────────────────────────────────────────────────────

elicit_newtype!(reqwest::Method, as Method, json_schema = method_json_schema);

impl Serialize for Method {
    #[tracing::instrument(skip(self, s), level = "trace")]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.0.as_str())
    }
}

impl<'de> Deserialize<'de> for Method {
    #[tracing::instrument(skip(d), level = "trace")]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        reqwest::Method::from_bytes(s.as_bytes())
            .map(Self::from)
            .map_err(serde::de::Error::custom)
    }
}

impl From<Method> for reqwest::Method {
    fn from(m: Method) -> Self {
        Arc::try_unwrap(m.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl ElicitComplete for Method {}

impl ToCodeLiteral for Method {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let s = self.0.as_str();
        quote::quote! {
            ::elicit_reqwest::Method::from(
                match ::reqwest::Method::from_bytes(#s.as_bytes()) {
                    ::std::result::Result::Ok(m) => m,
                    ::std::result::Result::Err(error) => {
                        return ::std::result::Result::Err(error.into());
                    }
                }
            )
        }
    }
}

// ── StatusCode ────────────────────────────────────────────────────────────────

elicit_newtype!(reqwest::StatusCode, as StatusCode, json_schema = status_code_json_schema);

impl Serialize for StatusCode {
    #[tracing::instrument(skip(self, s), level = "trace")]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u16(self.0.as_u16())
    }
}

impl<'de> Deserialize<'de> for StatusCode {
    #[tracing::instrument(skip(d), level = "trace")]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let code = u16::deserialize(d)?;
        reqwest::StatusCode::from_u16(code)
            .map(Self::from)
            .map_err(serde::de::Error::custom)
    }
}

impl StatusCode {
    /// Construct a `StatusCode` from a raw `u16`.
    ///
    /// Returns an error if the value is not in the range 100–599.
    #[tracing::instrument(level = "debug")]
    pub fn from_u16(src: u16) -> Result<Self, <reqwest::StatusCode as TryFrom<u16>>::Error> {
        reqwest::StatusCode::from_u16(src).map(Self::from)
    }
}

impl From<StatusCode> for reqwest::StatusCode {
    fn from(sc: StatusCode) -> Self {
        Arc::try_unwrap(sc.0).unwrap_or_else(|arc| *arc)
    }
}

impl ElicitComplete for StatusCode {}

impl ToCodeLiteral for StatusCode {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let code = self.0.as_u16();
        quote::quote! {
            match ::elicit_reqwest::StatusCode::from_u16(#code) {
                ::std::result::Result::Ok(s) => s,
                ::std::result::Result::Err(error) => {
                    return ::std::result::Result::Err(error.into());
                }
            }
        }
    }
}

// ── Version ───────────────────────────────────────────────────────────────────

elicit_newtype!(reqwest::Version, as Version, json_schema = version_json_schema);

impl Serialize for Version {
    #[tracing::instrument(skip(self, s), level = "trace")]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&format!("{:?}", *self.0))
    }
}

impl<'de> Deserialize<'de> for Version {
    #[tracing::instrument(skip(d), level = "trace")]
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

impl From<Version> for reqwest::Version {
    fn from(v: Version) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| *arc)
    }
}

impl ElicitComplete for Version {}

impl ToCodeLiteral for Version {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let variant = match *self.0 {
            reqwest::Version::HTTP_09 => quote::quote! { ::reqwest::Version::HTTP_09 },
            reqwest::Version::HTTP_10 => quote::quote! { ::reqwest::Version::HTTP_10 },
            reqwest::Version::HTTP_11 => quote::quote! { ::reqwest::Version::HTTP_11 },
            reqwest::Version::HTTP_2 => quote::quote! { ::reqwest::Version::HTTP_2 },
            reqwest::Version::HTTP_3 => quote::quote! { ::reqwest::Version::HTTP_3 },
            _ => quote::quote! { ::reqwest::Version::HTTP_11 },
        };
        quote::quote! { ::elicit_reqwest::Version::from(#variant) }
    }
}

// ── HeaderValue ───────────────────────────────────────────────────────────────
// HeaderValue serializes as { bytes: Vec<u8>, text: Option<String> } — it has
// named fields in its serialized form, so it is a hand-written trenchcoat per
// the "fields need names" branch of the SUPPORT_PATTERNS.md decision tree.

use elicitation::{Elicitation, ElicitIntrospect, ElicitPromptTree, ElicitSpec, Prompt, PromptTree,
    TypeMetadata, TypeSpec};
use elicitation::Elicit;

#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema, Elicit)]
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

/// HTTP header value with byte-faithful serde and elicitation via `HeaderValueData`.
#[derive(Debug, Clone)]
pub struct HeaderValue(pub Arc<http::HeaderValue>);

impl Serialize for HeaderValue {
    #[tracing::instrument(skip(self, serializer), level = "trace")]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        HeaderValueData::from(self.0.as_ref()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HeaderValue {
    #[tracing::instrument(skip(d), level = "trace")]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let data = HeaderValueData::deserialize(d)?;
        http::HeaderValue::from_bytes(&data.bytes)
            .map(Self::from)
            .map_err(serde::de::Error::custom)
    }
}

impl schemars::JsonSchema for HeaderValue {
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

    #[tracing::instrument(skip(communicator), level = "debug")]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let data = HeaderValueData::elicit(communicator).await?;
        let value = http::HeaderValue::from_bytes(&data.bytes).map_err(|error| {
            ElicitError::new(ElicitErrorKind::ParseError(error.to_string()))
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
    #[tracing::instrument(level = "debug")]
    pub fn from_static(src: &'static str) -> Self {
        Self::from(http::HeaderValue::from_static(src))
    }

    /// Construct a header value from raw bytes.
    #[tracing::instrument(skip(src), level = "debug")]
    pub fn from_bytes(src: &[u8]) -> Result<Self, http::header::InvalidHeaderValue> {
        http::HeaderValue::from_bytes(src).map(Self::from)
    }

    /// Borrow the raw header bytes.
    #[tracing::instrument(skip(self), level = "trace")]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Borrow the UTF-8 header text when representable.
    #[tracing::instrument(skip(self), level = "trace")]
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

impl std::str::FromStr for HeaderValue {
    type Err = http::header::InvalidHeaderValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        http::HeaderValue::from_str(s).map(Self::from)
    }
}

impl ElicitComplete for HeaderValue {}

impl ToCodeLiteral for HeaderValue {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let bytes = self.0.as_bytes().to_vec();
        quote::quote! {
            match ::elicit_reqwest::HeaderValue::from_bytes(&[#(#bytes),*]) {
                ::std::result::Result::Ok(v) => v,
                ::std::result::Result::Err(error) => {
                    return ::std::result::Result::Err(error.into());
                }
            }
        }
    }
}

// ── HeaderMap ─────────────────────────────────────────────────────────────────

elicit_newtype!(http::HeaderMap, as HeaderMap, json_schema = header_map_json_schema);

impl Default for HeaderMap {
    fn default() -> Self {
        Self(Arc::new(http::HeaderMap::new()))
    }
}

impl Serialize for HeaderMap {
    #[tracing::instrument(skip(self, s), level = "trace")]
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
    #[tracing::instrument(skip(d), level = "trace")]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw: std::collections::HashMap<String, String> =
            std::collections::HashMap::deserialize(d)?;
        let mut map = http::HeaderMap::new();
        for (k, v) in raw {
            let name = http::header::HeaderName::from_bytes(k.as_bytes())
                .map_err(serde::de::Error::custom)?;
            let value =
                http::HeaderValue::from_str(&v).map_err(serde::de::Error::custom)?;
            map.insert(name, value);
        }
        Ok(map.into())
    }
}

impl From<HeaderMap> for http::HeaderMap {
    fn from(m: HeaderMap) -> Self {
        Arc::try_unwrap(m.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl ElicitComplete for HeaderMap {}

impl ToCodeLiteral for HeaderMap {
    #[tracing::instrument(skip(self), level = "trace")]
    fn to_code_literal(&self) -> TokenStream {
        let entries: Vec<_> = self
            .0
            .iter()
            .map(|(name, value)| {
                let name_str = name.as_str();
                let value_ts = HeaderValue::from(value.clone()).to_code_literal();
                quote::quote! {
                    map.insert(
                        match ::http::header::HeaderName::from_bytes(#name_str.as_bytes()) {
                            ::std::result::Result::Ok(n) => n,
                            ::std::result::Result::Err(error) => {
                                return ::std::result::Result::Err(error.into());
                            }
                        },
                        #value_ts,
                    )
                }
            })
            .collect();
        quote::quote! {
            {
                let mut map = ::elicit_reqwest::HeaderMap::default();
                #(#entries;)*
                map
            }
        }
    }
}
