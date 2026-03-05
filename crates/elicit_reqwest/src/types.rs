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
