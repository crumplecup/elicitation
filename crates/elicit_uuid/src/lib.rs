//! Elicitation-enabled `Uuid` newtype — JsonSchema-compatible wrapper around `uuid::Uuid`.
//!
//! `uuid::Uuid` does not implement `JsonSchema`, which prevents it from being
//! used directly in MCP tool registrations. This crate provides a transparent
//! newtype `Uuid` that:
//!
//! - Derives `JsonSchema` via [`schemars`]'s `uuid1` feature (emits `{ "type": "string", "format": "uuid" }`).
//! - Derives `Serialize`/`Deserialize` transparently.
//! - Implements `Deref`/`DerefMut` so all `uuid::Uuid` methods are accessible without unwrapping.
//! - Converts losslessly to/from `uuid::Uuid` via `From`/`Into`.
//!
//! # Replacing `uuid::Uuid` in your types
//!
//! ```rust,ignore
//! use elicit_uuid::Uuid;
//! use elicitation_derive::Elicit;
//!
//! #[derive(Debug, Clone, Elicit)]
//! pub struct Record {
//!     id: Uuid,
//!     name: String,
//! }
//! ```
//!
//! To obtain the inner `uuid::Uuid`, dereference or convert:
//!
//! ```rust
//! use elicit_uuid::Uuid;
//!
//! let id: Uuid = uuid::Uuid::new_v4().into();
//! let inner: uuid::Uuid = *id;
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod uuid_type;

pub use uuid_type::Uuid;
