//! MCP tool transport for serde — type-aware JSON serialization and deserialization.
//!
//! Exposes [`serde::Serialize`] and [`serde::de::DeserializeOwned`] as per-type MCP
//! tool factories using `#[reflect_trait]`.  The generic `Serializer`/`Deserializer`
//! parameters are erased by concrete wrapper traits that fix the format to `serde_json`.
//!
//! # Trait Factories
//!
//! ```rust,no_run
//! use elicit_serde::{prime_serialize_json, prime_deserialize_json};
//! use elicitation::DynamicToolRegistry;
//! use serde::{Serialize, Deserialize};
//! use schemars::JsonSchema;
//! use elicitation_derive::Elicit;
//!
//! #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
//! struct Point { x: f64, y: f64 }
//!
//! #[tokio::main]
//! async fn main() {
//!     prime_serialize_json::<Point>();
//!     prime_deserialize_json::<Point>();
//!     // then register with DynamicToolRegistry::register_type::<Point>("geo")
//!     // and instantiate("crate::SerializeJson", "geo").await
//! }
//! ```
//!
//! This exposes `geo__to_json` and `geo__from_json` as MCP tools.
//!
//! # Type-to-Type Conversion
//!
//! [`DynamicToolRegistry::register_convert`] enables structural conversion between
//! any two types that share a compatible serde data model (schema migration,
//! newtype unwrapping, field renaming via `#[serde]` attributes):
//!
//! ```rust,no_run
//! use elicit_serde::DynamicToolRegistry;
//! use serde::{Serialize, Deserialize};
//! use schemars::JsonSchema;
//!
//! #[derive(Serialize, Deserialize, JsonSchema)]
//! struct ConfigV1 { name: String, value: i32 }
//!
//! #[derive(Serialize, Deserialize, JsonSchema)]
//! struct ConfigV2 { name: String, value: i64, #[serde(default)] tag: Option<String> }
//!
//! let registry = DynamicToolRegistry::new()
//!     .register_convert::<ConfigV1, ConfigV2>();
//! // Exposes `convert__config_v1__to__config_v2` as an MCP tool.
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod trait_factories;

pub use elicitation::DynamicToolRegistry;
pub use trait_factories::{
    DeserializeJson, DeserializeJsonFactory, SerializeJson, SerializeJsonFactory,
    prime_crate__deserialize_json as prime_deserialize_json,
    prime_crate__serialize_json as prime_serialize_json,
};
