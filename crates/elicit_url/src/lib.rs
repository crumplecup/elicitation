//! `elicit_url` — elicitation-enabled wrapper around `url::Url`.
//!
//! Provides a [`Url`] newtype with:
//! - [`schemars::JsonSchema`] (delegated to `url::Url`)
//! - [`serde::Serialize`] / [`serde::Deserialize`] (transparent)
//! - MCP reflect methods for URL decomposition and manipulation
//! - [`UrlWorkflowPlugin`]: contract-verified URL composition tools

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod url_type;
pub mod workflow;

pub use url_type::Url;
pub use workflow::{
    AssertHttpsParams, BuildUrlParams, HttpsRequired, JoinUrlParams, ParseUrlParams, ParsedUrl,
    SchemeAllowed, SecureUrl, SecureUrlState, UnvalidatedUrl, UrlParsed, UrlWorkflowPlugin,
    ValidateSchemeParams,
};

#[cfg(feature = "emit")]
pub use workflow::dispatch_emit as dispatch_url_emit;
