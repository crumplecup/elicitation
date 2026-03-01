//! Client wrapper for reqwest HTTP client.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Client
//! with MCP tool generation for all HTTP methods.
//!
//! All Client methods are generic over `U: IntoUrl`, making this module
//! an excellent test of the `#[reflect_methods]` generic support.

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
