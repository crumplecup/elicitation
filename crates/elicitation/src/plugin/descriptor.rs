//! [`ToolDescriptor`] — a self-contained tool definition.
//!
//! A `ToolDescriptor` bundles a tool's name, description, JSON schema, and
//! async handler into one value.
//!
//! # Constructors
//!
//! - [`make_descriptor`] — for handlers that ignore the context
//! - [`make_descriptor_ctx`] — for handlers that use `Arc<Ctx>` where `Ctx: PluginContext`
//!
//! # Example
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use elicitation::plugin::{make_descriptor, make_descriptor_ctx, NoContext};
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
//! struct MyCtx { value: u32 }
//! impl elicitation::plugin::PluginContext for MyCtx {}
//!
//! let ctx_aware = make_descriptor_ctx::<MyCtx, PingParams, _>(
//!     "ping_ctx",
//!     "Echo with context",
//!     |_ctx: Arc<MyCtx>, p| Box::pin(async move {
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

/// Type alias for the async handler stored inside a [`ToolDescriptor`].
///
/// The context is passed as a type-erased `Arc<dyn Any + Send + Sync>`.
/// Context-free handlers ignore it; context-aware handlers downcast it to
/// their concrete `Ctx` type inside the closure created by [`make_descriptor_ctx`].
pub(crate) type ToolHandler = Arc<
    dyn Fn(
            Arc<dyn std::any::Any + Send + Sync>,
            CallToolRequestParams,
        ) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
        + Send
        + Sync,
>;

/// A fully self-contained MCP tool definition.
///
/// Carries the tool's name, description, JSON schema, and an async handler
/// that parses its own params from [`CallToolRequestParams`].
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
    pub(crate) handler: ToolHandler,
}

impl std::fmt::Debug for ToolDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ToolDescriptor")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

impl Clone for ToolDescriptor {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            description: self.description,
            tool: self.tool.clone(),
            handler: Arc::clone(&self.handler),
        }
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
/// Use this when the handler does not need shared resources.  For handlers
/// that require a typed context, use [`make_descriptor_ctx`] instead.
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
        move |_ctx: Arc<dyn std::any::Any + Send + Sync>, params: CallToolRequestParams| {
            let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
            match serde_json::from_value::<T>(value) {
                Ok(typed) => handler(typed),
                Err(e) => {
                    Box::pin(async move { Err(ErrorData::invalid_params(e.to_string(), None)) })
                }
            }
        },
    );
    ToolDescriptor {
        name,
        description,
        tool,
        handler,
    }
}

/// Build a [`ToolDescriptor`] from a context-aware handler.
///
/// The handler receives `Arc<Ctx>` as its first argument, giving access to
/// plugin-specific shared resources (e.g. an HTTP client, a DB pool).
///
/// `Ctx` must implement [`PluginContext`]. The type is inferred from the
/// handler's first argument in most cases.
///
/// # Example
///
/// ```rust,no_run
/// # use std::sync::Arc;
/// # use elicitation::plugin::{PluginContext, make_descriptor_ctx};
/// # use rmcp::model::{CallToolResult, Content};
/// # use schemars::JsonSchema;
/// # use serde::Deserialize;
/// pub struct MyCtx { pub value: u32 }
/// impl PluginContext for MyCtx {}
///
/// #[derive(Deserialize, JsonSchema)]
/// struct Params { input: String }
///
/// let d = make_descriptor_ctx::<MyCtx, Params, _>(
///     "my_tool",
///     "Uses context",
///     |ctx: Arc<MyCtx>, p| Box::pin(async move {
///         let _ = ctx.value;
///         Ok(CallToolResult::success(vec![Content::text(p.input)]))
///     }),
/// );
/// ```
pub fn make_descriptor_ctx<Ctx, T, F>(
    name: &'static str,
    description: &'static str,
    handler: F,
) -> ToolDescriptor
where
    Ctx: PluginContext,
    T: DeserializeOwned + JsonSchema + 'static,
    F: Fn(Arc<Ctx>, T) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
        + Send
        + Sync
        + 'static,
{
    let tool = build_tool::<T>(name, description);
    let handler = Arc::new(
        move |ctx: Arc<dyn std::any::Any + Send + Sync>, params: CallToolRequestParams| {
            let ctx = ctx.downcast::<Ctx>().unwrap_or_else(|_| {
                panic!(
                    "context type mismatch: expected {}",
                    std::any::type_name::<Ctx>()
                )
            });
            let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
            match serde_json::from_value::<T>(value) {
                Ok(typed) => handler(ctx, typed),
                Err(e) => {
                    Box::pin(async move { Err(ErrorData::invalid_params(e.to_string(), None)) })
                }
            }
        },
    );
    ToolDescriptor {
        name,
        description,
        tool,
        handler,
    }
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
    ///
    /// The context is passed as a type-erased `Arc<dyn Any + Send + Sync>`.
    /// Context-aware handlers (built with [`make_descriptor_ctx`]) downcast it
    /// to their concrete `Ctx` type internally.
    pub fn dispatch(
        &self,
        ctx: Arc<dyn std::any::Any + Send + Sync>,
        params: CallToolRequestParams,
    ) -> BoxFuture<'static, Result<CallToolResult, ErrorData>> {
        (self.handler)(ctx, params)
    }
}
