//! Client wrapper for reqwest HTTP client.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Client
//! with MCP tool generation for all HTTP methods.
//!
//! All Client methods are generic over `U: IntoUrl`, making this module
//! an excellent test of the `#[reflect_methods]` generic support.
//!
//! NOTE: This is a SHADOW CRATE demonstrating macro usage only.
//! All Elicitation/JsonSchema/Prompt impls should be in the main
//! elicitation crate under appropriate feature flags.

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::Client, as Client);

impl Client {
    /// Creates a new HTTP client.
    pub fn new() -> Self {
        reqwest::Client::new().into()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Generic HTTP methods demonstration
// Blocked pending reqwest feature support in elicitation crate:
// - reqwest::Client needs Elicitation + JsonSchema + Prompt impls
// - url::Url already has impls (feature = "url")
// - RequestBuilder needs Elicitation + JsonSchema + Prompt impls
//
// #[reflect_methods]
// impl Client {
//     pub fn get<U>(&self, url: U) -> RequestBuilder
//     where
//         U: elicitation::Elicitation + schemars::JsonSchema + reqwest::IntoUrl,
//     { ... }
// }
