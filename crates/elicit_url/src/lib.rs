//! `elicit_url` — elicitation-enabled wrapper around `url::Url`.
//!
//! Provides a [`Url`] newtype with:
//! - [`schemars::JsonSchema`] (delegated to `url::Url`)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent)
//! - MCP reflect methods for URL decomposition and manipulation

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod url_type;

pub use url_type::Url;
