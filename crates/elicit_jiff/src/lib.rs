//! `elicit_jiff` — elicitation-enabled wrappers around `jiff` datetime types.
//!
//! Provides [`Zoned`] and [`Timestamp`] newtypes with:
//! - [`schemars::JsonSchema`] (delegated to inner jiff types)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent)
//! - MCP reflect methods for datetime field access and timezone operations

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod timestamp;
mod zoned;

pub use timestamp::Timestamp;
pub use zoned::Zoned;
