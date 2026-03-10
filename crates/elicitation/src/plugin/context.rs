//! [`PluginContext`] — shared execution context for plugin tools.
//!
//! Plugins that need long-lived resources (HTTP connection pools, credentials,
//! telemetry handles) store them here and receive an `Arc<PluginContext>` on
//! every tool invocation.
//!
//! # Feature gates
//!
//! The `http` field is available when the `reqwest` feature is enabled
//! (part of the default `full` feature bundle).

use std::sync::Arc;

/// Shared execution context passed to every tool handler.
///
/// Holding the context behind [`Arc`] lets multiple plugins share the same
/// `reqwest::Client` connection pool, which is the primary motivation for
/// this type.
///
/// # Example
///
/// ```rust,no_run
/// use std::sync::Arc;
/// use elicitation::plugin::PluginContext;
///
/// let ctx = Arc::new(PluginContext::default());
/// ```
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Shared HTTP client. Present when the `reqwest` feature is enabled.
    #[cfg(feature = "reqwest")]
    pub http: reqwest::Client,
}

impl Default for PluginContext {
    fn default() -> Self {
        Self {
            #[cfg(feature = "reqwest")]
            http: reqwest::Client::new(),
        }
    }
}

impl PluginContext {
    /// Create a new context with default settings.
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}
