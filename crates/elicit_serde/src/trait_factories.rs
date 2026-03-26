//! `#[reflect_trait]` factories for serde's generic serialization traits.
//!
//! `serde::Serialize` and `serde::Deserialize` have **method-level** generic
//! parameters (`S: Serializer`, `D: Deserializer`) that cannot be wrapped
//! directly by `#[reflect_trait]`.  These two concrete wrapper traits solve
//! the problem by fixing the format to `serde_json`, erasing the generics:
//!
//! | Wrapper trait      | Wraps                        | Method       |
//! |--------------------|------------------------------|--------------|
//! | [`SerializeJson`]  | `serde::Serialize`           | `to_json`    |
//! | [`DeserializeJson`]| `serde::de::DeserializeOwned`| `from_json`  |
//!
//! Both traits have blanket impls, so any type that derives `serde::Serialize`
//! or `serde::Deserialize` automatically implements the wrapper.  No orphan
//! rule issues arise because both traits are defined in this crate.
//!
//! # Usage
//!
//! At server startup, for each type `T` you want to expose:
//!
//! ```rust,ignore
//! use elicit_serde::{prime_crate__serialize_json, prime_crate__deserialize_json};
//! use elicitation::DynamicToolRegistry;
//!
//! prime_crate__serialize_json::<MyType>();
//! prime_crate__deserialize_json::<MyType>();
//! registry.register_type::<MyType>("myapp").await;
//! ```
//!
//! This exposes two MCP tools per type:
//! - `myapp__to_json`   — serialise a `MyType` value to a compact JSON string
//! - `myapp__from_json` — parse a JSON string into a `MyType` value

use elicitation_macros::reflect_trait;

// ── SerializeJson ─────────────────────────────────────────────────────────────

/// JSON serialization adapter — erases `serde::Serialize`'s generic `Serializer`
/// parameter by fixing the format to `serde_json`.
///
/// Blanket impl provided for every `T: serde::Serialize`.
pub trait SerializeJson {
    /// Serialize `self` to a compact JSON string.
    fn to_json(&self) -> Result<String, String>;
}

impl<T: serde::Serialize> SerializeJson for T {
    fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| e.to_string())
    }
}

// ── DeserializeJson ───────────────────────────────────────────────────────────

/// JSON deserialization adapter — erases `serde::Deserialize`'s generic
/// `Deserializer` parameter by fixing the format to `serde_json`.
///
/// Blanket impl provided for every `T: serde::de::DeserializeOwned`.
pub trait DeserializeJson: Sized {
    /// Parse a JSON string into `Self`.
    fn from_json(json: &str) -> Result<Self, String>;
}

impl<T: serde::de::DeserializeOwned> DeserializeJson for T {
    fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }
}

// ── Factories ─────────────────────────────────────────────────────────────────

/// Expose [`SerializeJson`] as an agent-callable MCP tool factory.
///
/// For each registered type `T`, contributes one tool:
/// - `{prefix}__to_json` — serialize a `T` value (as JSON) to a compact JSON string
///
/// The `target` parameter must be the JSON representation of the `T` value.
#[reflect_trait(crate::SerializeJson)]
pub trait SerializeJsonTools {
    /// Serialize this value to a compact JSON string.
    fn to_json(&self) -> Result<String, String>;
}

/// Expose [`DeserializeJson`] as an agent-callable MCP tool factory.
///
/// For each registered type `T`, contributes one tool:
/// - `{prefix}__from_json` — parse a JSON string into a `T` value
///
/// The `json` parameter must be a valid JSON string encoding a `T` value.
#[reflect_trait(crate::DeserializeJson)]
pub trait DeserializeJsonTools {
    /// Parse a JSON string into this type.
    fn from_json(json: &str) -> Result<Self, String>;
}
