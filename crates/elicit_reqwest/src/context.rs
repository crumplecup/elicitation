//! Plugin context for reqwest-backed tools.
//!
//! [`HttpContext`] holds a shared [`reqwest::Client`] that is reused across all
//! tool invocations within a single plugin instance.

use std::sync::Arc;

use elicitation::PluginContext;

/// Plugin context carrying a shared HTTP client.
#[derive(Debug)]
pub struct HttpContext {
    /// The underlying reqwest HTTP client.
    pub http: reqwest::Client,
}

impl PluginContext for HttpContext {}

impl HttpContext {
    /// Create a context wrapping a freshly-constructed default HTTP client.
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            http: reqwest::Client::new(),
        })
    }
}
