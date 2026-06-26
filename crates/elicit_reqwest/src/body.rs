//! Shadow for `reqwest::Body`.

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Error;

/// Owned request-body snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP request body:")]
#[to_code_literal(path = "::elicit_reqwest::Body::from_bytes", tuple)]
pub struct Body {
    #[serde(default)]
    #[prompt("Raw request body bytes:")]
    bytes: Vec<u8>,
}

impl Body {
    /// Create a body snapshot from raw bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Borrow the raw body bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Rebuild a raw `reqwest::Body`.
    pub fn build_raw(&self) -> reqwest::Body {
        reqwest::Body::from(self.bytes.clone())
    }
}

impl TryFrom<reqwest::Body> for Body {
    type Error = Error;

    fn try_from(value: reqwest::Body) -> Result<Self, Self::Error> {
        let bytes = value
            .as_bytes()
            .ok_or_else(|| Error::builder("streaming reqwest::Body cannot be snapshotted"))?
            .to_vec();
        Ok(Self::from_bytes(bytes))
    }
}

impl From<Body> for reqwest::Body {
    fn from(value: Body) -> Self {
        value.build_raw()
    }
}
