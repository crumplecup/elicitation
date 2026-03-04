//! `Uuid` — elicitation-enabled wrapper around `uuid::Uuid`.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

elicit_newtype!(uuid::Uuid, as Uuid, serde);

#[reflect_methods]
impl Uuid {
    /// Returns the hyphenated string representation (e.g. `"550e8400-e29b-41d4-a716-446655440000"`).
    pub fn to_hyphenated(&self) -> String {
        self.0.hyphenated().to_string()
    }

    /// Returns the simple (no-hyphens) string representation.
    pub fn to_simple(&self) -> String {
        self.0.simple().to_string()
    }

    /// Returns the URN string representation (e.g. `"urn:uuid:550e8400..."`).
    pub fn to_urn(&self) -> String {
        self.0.urn().to_string()
    }

    /// Returns `true` if this is the nil UUID (all zeros).
    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }

    /// Returns `true` if this is the max UUID (all ones).
    pub fn is_max(&self) -> bool {
        self.0.is_max()
    }

    /// Returns the UUID version number, or `None` for nil/max.
    pub fn version(&self) -> Option<u8> {
        let n = self.0.get_version_num();
        if n == 0 { None } else { Some(n as u8) }
    }

    /// Returns the raw bytes of this UUID as a lowercase hex string (32 chars, no hyphens).
    pub fn as_bytes_hex(&self) -> String {
        hex::encode(self.0.as_bytes())
    }
}

impl Uuid {
    /// Parse a UUID from a string in any supported format.
    ///
    /// Returns `None` if the string is not a valid UUID.
    pub fn parse(s: &str) -> Option<Uuid> {
        uuid::Uuid::parse_str(s)
            .ok()
            .map(|u| std::sync::Arc::new(u).into())
    }
}
