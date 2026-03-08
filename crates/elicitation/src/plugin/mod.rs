//! Type-erased plugin interface for the elicitation tool registry.
//!
//! Each shadow crate (e.g., `elicit_reqwest`) provides a `Plugin` struct that
//! implements [`ElicitPlugin`]. The [`PluginRegistry`](crate::PluginRegistry)
//! collects these and serves them as a single MCP server.
//!
//! # Implementing a plugin
//!
//! **Simple path** — implement [`DescriptorPlugin`] and expose a slice of
//! [`ToolDescriptor`]s built with [`make_descriptor`].  The blanket impl
//! provides [`ElicitPlugin`] for free.
//!
//! **Full control** — implement [`ElicitPlugin`] directly.

pub mod context;
pub mod descriptor;
pub mod descriptor_plugin;

pub use context::PluginContext;
pub use descriptor::{PluginToolRegistration, ToolDescriptor, make_descriptor, make_descriptor_ctx};
pub use descriptor_plugin::DescriptorPlugin;

use std::borrow::Cow;
use std::sync::Arc;

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Tool},
    service::RequestContext,
};

use crate::rmcp::RoleServer;

/// Type-erased interface for a shadow-crate tool plugin.
///
/// # Object Safety
///
/// This trait is object-safe: all async methods return `BoxFuture`.
///
/// Prefer implementing [`DescriptorPlugin`] over this trait directly unless
/// you need custom dispatch logic.
pub trait ElicitPlugin: Send + Sync + 'static {
    /// Human-readable plugin name, used as the namespace prefix.
    ///
    /// E.g. `"http"` produces tools named `http__get`, `http__post`, etc.
    fn name(&self) -> &'static str;

    /// List all tools provided by this plugin (without namespace prefix).
    fn list_tools(&self) -> Vec<Tool>;

    /// Dispatch a tool call to this plugin.
    ///
    /// `params.name` will already have the namespace prefix stripped by
    /// `PluginRegistry` before this is called.
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>>;
}

/// A type-erased, cheaply-cloneable plugin reference.
pub type ArcPlugin = Arc<dyn ElicitPlugin>;

/// Apply the namespace prefix to a tool name.
///
/// `"http"` + `"get"` → `"http__get"`.
pub(crate) fn prefixed_name(prefix: &str, name: &str) -> Cow<'static, str> {
    Cow::Owned(format!("{prefix}__{name}"))
}

/// Strip the namespace prefix from a tool name, returning the bare name.
///
/// `"http__get"` with prefix `"http"` → `"get"`.
/// Returns `None` if the name does not start with `{prefix}__`.
pub(crate) fn strip_prefix<'a>(prefix: &str, name: &'a str) -> Option<&'a str> {
    let sep = format!("{prefix}__");
    name.strip_prefix(sep.as_str())
}
