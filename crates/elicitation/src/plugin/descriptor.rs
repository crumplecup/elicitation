//! [`ToolDescriptor`] — a self-contained tool definition.
//!
//! A `ToolDescriptor` bundles a tool's name, description, JSON schema, and
//! async handler into one value.  The handler always receives a
//! [`Arc<PluginContext>`](super::PluginContext) so plugins can share resources
//! (e.g., `reqwest::Client`) across calls.
//!
//! # Constructors
//!
//! - [`make_descriptor`] — for handlers that ignore the context
//! - [`make_descriptor_ctx`] — for handlers that use `Arc<PluginContext>`
//!
//! # Example
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use elicitation::plugin::{PluginContext, make_descriptor, make_descriptor_ctx};
//! use rmcp::model::{CallToolResult, Content};
//! use schemars::JsonSchema;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, JsonSchema)]
//! struct PingParams { message: String }
//!
//! // Context-free
//! let ctx_free = make_descriptor::<PingParams, _>(
//!     "ping",
//!     "Echo a message back",
//!     |p| Box::pin(async move {
//!         Ok(CallToolResult::success(vec![Content::text(p.message)]))
//!     }),
//! );
//!
//! // Context-aware (e.g. uses ctx.http)
//! let ctx_aware = make_descriptor_ctx::<PingParams, _>(
//!     "ping_ctx",
//!     "Echo via HTTP client",
//!     |_ctx: Arc<PluginContext>, p| Box::pin(async move {
//!         Ok(CallToolResult::success(vec![Content::text(p.message)]))
//!     }),
//! );
//! ```

use std::sync::Arc;

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Tool},
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

use super::PluginContext;

/// A fully self-contained MCP tool definition.
///
/// Carries the tool's name, description, JSON schema, and an async handler
/// that parses its own params from [`CallToolRequestParams`].  The handler
/// always receives an [`Arc<PluginContext>`] so long-lived resources can be
/// shared across calls.
///
/// Create via [`make_descriptor`] (context-free) or [`make_descriptor_ctx`]
/// (context-aware).
pub struct ToolDescriptor {
    /// Bare tool name (no namespace prefix).
    pub name: &'static str,
    /// Human-readable description shown to the agent.
    pub description: &'static str,
    /// rmcp [`Tool`] built from the param type's JSON schema.
    pub(crate) tool: Tool,
    /// Async handler: receives context + raw params, returns result.
    pub(crate) handler: Arc<
        dyn Fn(Arc<PluginContext>, CallToolRequestParams)
                -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
            + Send
            + Sync,
    >,
}

impl std::fmt::Debug for ToolDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolDescriptor")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

// ── Schema helper ──────────────────────────────────────────────────────────────

fn build_tool<T: JsonSchema>(name: &'static str, description: &'static str) -> Tool {
    let schema_value = serde_json::to_value(schemars::schema_for!(T))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let schema_obj = match schema_value {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    Tool::new(name, description, schema_obj)
}

// ── Constructors ───────────────────────────────────────────────────────────────

/// Build a [`ToolDescriptor`] from a context-free handler.
///
/// The context parameter is ignored; use this when the handler does not need
/// shared resources.  For handlers that require `Arc<PluginContext>`, use
/// [`make_descriptor_ctx`] instead.
///
/// # Example
///
/// ```rust,no_run
/// # use elicitation::plugin::make_descriptor;
/// # use rmcp::model::{CallToolResult, Content};
/// # use schemars::JsonSchema;
/// # use serde::Deserialize;
/// #[derive(Deserialize, JsonSchema)]
/// struct MyParams { value: u32 }
///
/// let d = make_descriptor::<MyParams, _>(
///     "my_tool",
///     "Does something with a u32",
///     |p| Box::pin(async move {
///         Ok(CallToolResult::success(vec![Content::text(p.value.to_string())]))
///     }),
/// );
/// ```
pub fn make_descriptor<T, F>(
    name: &'static str,
    description: &'static str,
    handler: F,
) -> ToolDescriptor
where
    T: DeserializeOwned + JsonSchema + 'static,
    F: Fn(T) -> BoxFuture<'static, Result<CallToolResult, ErrorData>> + Send + Sync + 'static,
{
    let tool = build_tool::<T>(name, description);
    let handler = Arc::new(
        move |_ctx: Arc<PluginContext>, params: CallToolRequestParams| {
            let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
            match serde_json::from_value::<T>(value) {
                Ok(typed) => handler(typed),
                Err(e) => {
                    Box::pin(async move { Err(ErrorData::invalid_params(e.to_string(), None)) })
                }
            }
        },
    );
    ToolDescriptor { name, description, tool, handler }
}

/// Build a [`ToolDescriptor`] from a context-aware handler.
///
/// The handler receives `Arc<PluginContext>` as its first argument, giving
/// access to shared resources such as `ctx.http` (the `reqwest::Client`).
///
/// # Example
///
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use elicitation::plugin::{PluginContext, make_descriptor_ctx};
/// # use rmcp::model::{CallToolResult, Content};
/// # use schemars::JsonSchema;
/// # use serde::Deserialize;
/// #[derive(Deserialize, JsonSchema)]
/// struct FetchParams { url: String }
///
/// let d = make_descriptor_ctx::<FetchParams, _>(
///     "fetch",
///     "Fetch a URL using the shared client",
///     |ctx: Arc<PluginContext>, p| Box::pin(async move {
///         let _resp = ctx.http.get(&p.url).send().await;
///         Ok(CallToolResult::success(vec![]))
///     }),
/// );
/// ```
pub fn make_descriptor_ctx<T, F>(
    name: &'static str,
    description: &'static str,
    handler: F,
) -> ToolDescriptor
where
    T: DeserializeOwned + JsonSchema + 'static,
    F: Fn(Arc<PluginContext>, T) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
        + Send
        + Sync
        + 'static,
{
    let tool = build_tool::<T>(name, description);
    let handler = Arc::new(
        move |ctx: Arc<PluginContext>, params: CallToolRequestParams| {
            let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
            match serde_json::from_value::<T>(value) {
                Ok(typed) => handler(ctx, typed),
                Err(e) => {
                    Box::pin(async move { Err(ErrorData::invalid_params(e.to_string(), None)) })
                }
            }
        },
    );
    ToolDescriptor { name, description, tool, handler }
}

// ── PluginToolRegistration ─────────────────────────────────────────────────────

/// Lightweight inventory registration connecting a tool to its plugin.
///
/// Submitted via `inventory::submit!` by the `#[elicit_tool(plugin = "...")]`
/// macro.  Collected by `#[derive(ElicitPlugin)]` to discover all tools that
/// belong to a given plugin at link time.
///
/// The `constructor` is a plain function pointer (zero-cost, `'static`) that
/// builds the full [`ToolDescriptor`] on demand.
#[derive(Debug)]
pub struct PluginToolRegistration {
    /// Name of the owning plugin (e.g. `"secure_fetch"`).
    pub plugin: &'static str,
    /// Bare tool name (no namespace prefix).
    pub name: &'static str,
    /// Builds the [`ToolDescriptor`] for this tool.
    pub constructor: fn() -> ToolDescriptor,
}

inventory::collect!(PluginToolRegistration);

// ── ToolDescriptor impl ────────────────────────────────────────────────────────

impl ToolDescriptor {
    /// Return the rmcp [`Tool`] (schema + metadata) for this descriptor.
    pub fn as_tool(&self) -> Tool {
        self.tool.clone()
    }

    /// Invoke the handler with the given context and params.
    pub fn dispatch(
        &self,
        ctx: Arc<PluginContext>,
        params: CallToolRequestParams,
    ) -> BoxFuture<'static, Result<CallToolResult, ErrorData>> {
        (self.handler)(ctx, params)
    }
}

