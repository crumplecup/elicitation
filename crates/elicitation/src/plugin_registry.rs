//! `PluginRegistry` — an MCP `ServerHandler` that aggregates tool plugins.
//!
//! # Usage
//!
//! ```rust,no_run
//! use elicitation::PluginRegistry;
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! // let registry = PluginRegistry::new()
//! //     .register("http", elicit_reqwest::Plugin::new(reqwest::Client::new()))
//! //     .register("fs",   elicit_tokio_fs::Plugin::new(root_dir));
//! // registry.serve(rmcp::transport::stdio()).await?;
//! # Ok(())
//! # }
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, ListToolsResult, PaginatedRequestParams},
    service::RequestContext,
};

use crate::{
    plugin::{ArcPlugin, ElicitPlugin, prefixed_name, strip_prefix},
    rmcp::RoleServer,
};

/// Aggregates multiple `ElicitPlugin` instances into one MCP server.
///
/// Tools are namespaced as `{prefix}__{tool_name}` to prevent collisions
/// across crates. The registry dispatches incoming `call_tool` requests to
/// the appropriate plugin after stripping the prefix.
///
/// # Cloneability
///
/// `PluginRegistry` is cheaply cloneable — plugins are held behind `Arc`.
#[derive(Clone, Default)]
pub struct PluginRegistry {
    /// Registered plugins in order.
    plugins: Vec<(String, ArcPlugin)>,
    /// `full_tool_name` → `plugin_index` for O(1) dispatch.
    dispatch: HashMap<String, usize>,
}

impl PluginRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin under a namespace prefix.
    ///
    /// Tools will be listed as `{prefix}__{original_name}`.
    ///
    /// # Panics
    ///
    /// Panics if any tool name in the plugin collides with an already-registered
    /// tool (after prefix application).
    #[tracing::instrument(skip(self, plugin), fields(prefix))]
    pub fn register(mut self, prefix: impl Into<String>, plugin: impl ElicitPlugin) -> Self {
        let prefix = prefix.into();
        let plugin: ArcPlugin = Arc::new(plugin);
        let idx = self.plugins.len();

        for tool in plugin.list_tools() {
            let full = format!("{prefix}__{}", tool.name);
            assert!(
                !self.dispatch.contains_key(&full),
                "tool name collision: `{full}` already registered"
            );
            self.dispatch.insert(full, idx);
        }

        tracing::debug!(prefix = %prefix, tool_count = plugin.list_tools().len(), "Registered plugin");
        self.plugins.push((prefix, plugin));
        self
    }

    /// Register a plugin without a prefix (tools are listed with their bare names).
    ///
    /// Use with care: collisions are possible when mixing flat and prefixed plugins.
    #[tracing::instrument(skip(self, plugin))]
    pub fn register_flat(mut self, plugin: impl ElicitPlugin) -> Self {
        let plugin: ArcPlugin = Arc::new(plugin);
        let idx = self.plugins.len();

        for tool in plugin.list_tools() {
            let name = tool.name.to_string();
            assert!(
                !self.dispatch.contains_key(&name),
                "tool name collision: `{name}` already registered"
            );
            self.dispatch.insert(name, idx);
        }

        tracing::debug!(
            tool_count = plugin.list_tools().len(),
            "Registered flat plugin"
        );
        // Use empty string as sentinel for flat (no prefix)
        self.plugins.push((String::new(), plugin));
        self
    }

    /// Build all tool listings with prefixes applied.
    fn all_tools(&self) -> Vec<rmcp::model::Tool> {
        let mut tools = Vec::new();
        for (prefix, plugin) in &self.plugins {
            for mut tool in plugin.list_tools() {
                if !prefix.is_empty() {
                    tool.name = prefixed_name(prefix, &tool.name);
                }
                tools.push(tool);
            }
        }
        tools
    }

    /// Wrap `self` in a `Toolchain` with a filter predicate.
    ///
    /// Only tools for which `filter` returns `true` will be visible to agents.
    pub fn filter<F>(self, filter: F) -> Toolchain<F>
    where
        F: Fn(&rmcp::model::Tool) -> bool + Send + Sync + 'static,
    {
        Toolchain {
            registry: self,
            filter,
        }
    }
}

impl rmcp::ServerHandler for PluginRegistry {
    #[tracing::instrument(skip(self, _request, _context))]
    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        let tools = self.all_tools();
        tracing::debug!(count = tools.len(), "Listing tools");
        std::future::ready(Ok(ListToolsResult {
            tools,
            ..Default::default()
        }))
    }

    #[tracing::instrument(skip(self, context), fields(tool = %request.name))]
    fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        async move {
            let idx = self
                .dispatch
                .get(request.name.as_ref())
                .copied()
                .ok_or_else(|| {
                    ErrorData::invalid_params(format!("tool `{}` not found", request.name), None)
                })?;

            let (prefix, plugin) = &self.plugins[idx];

            // Strip prefix from the request name before forwarding
            let bare_name: String = if prefix.is_empty() {
                request.name.as_ref().to_string()
            } else {
                strip_prefix(prefix, request.name.as_ref())
                    .ok_or_else(|| ErrorData::invalid_params("prefix mismatch", None))?
                    .to_string()
            };

            let mut forwarded = request;
            forwarded.name = std::borrow::Cow::Owned(bare_name.clone());

            tracing::debug!(bare = %bare_name, "Dispatching to plugin");
            plugin.call_tool(forwarded, context).await
        }
    }
}

// ============================================================================
// Toolchain — filtered view of a PluginRegistry
// ============================================================================

/// A curated subset of a `PluginRegistry`, visible to agents.
///
/// Built via [`PluginRegistry::filter`]. Only tools for which the predicate
/// returns `true` appear in `list_tools` and are routable via `call_tool`.
pub struct Toolchain<F = fn(&rmcp::model::Tool) -> bool>
where
    F: Fn(&rmcp::model::Tool) -> bool + Send + Sync + 'static,
{
    registry: PluginRegistry,
    filter: F,
}

impl<F> Toolchain<F>
where
    F: Fn(&rmcp::model::Tool) -> bool + Send + Sync + 'static,
{
    /// Build a toolchain from an existing registry with a filter.
    pub fn new(registry: PluginRegistry, filter: F) -> Self {
        Self { registry, filter }
    }
}

impl<F> rmcp::ServerHandler for Toolchain<F>
where
    F: Fn(&rmcp::model::Tool) -> bool + Send + Sync + 'static,
{
    #[tracing::instrument(skip(self, _request, _context))]
    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, ErrorData>> + Send + '_ {
        let tools: Vec<_> = self
            .registry
            .all_tools()
            .into_iter()
            .filter(|t| (self.filter)(t))
            .collect();
        tracing::debug!(visible = tools.len(), "Listing toolchain tools");
        std::future::ready(Ok(ListToolsResult {
            tools,
            ..Default::default()
        }))
    }

    #[tracing::instrument(skip(self, context), fields(tool = %request.name))]
    fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<CallToolResult, ErrorData>> + Send + '_ {
        async move {
            // Reject calls to tools not in the filtered set
            let visible = self
                .registry
                .all_tools()
                .into_iter()
                .any(|t| t.name == request.name && (self.filter)(&t));

            if !visible {
                return Err(ErrorData::invalid_params(
                    format!("tool `{}` not in toolchain", request.name),
                    None,
                ));
            }

            self.registry.call_tool(request, context).await
        }
    }
}
