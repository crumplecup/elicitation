//! [`ToolDescriptor`] — a self-contained tool definition.
//!
//! A `ToolDescriptor` bundles a tool's name, description, JSON schema, and
//! async handler into one value.  Plugins that implement
//! [`DescriptorPlugin`] expose a static slice of these and get
//! [`ElicitPlugin`] for free.
//!
//! # Example
//!
//! ```rust,no_run
//! use elicitation::plugin::{ToolDescriptor, make_descriptor};
//! use rmcp::model::{CallToolResult, Content};
//! use schemars::JsonSchema;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, JsonSchema)]
//! struct PingParams { message: String }
//!
//! fn ping_descriptor() -> ToolDescriptor {
//!     make_descriptor::<PingParams, _>(
//!         "ping",
//!         "Echo a message back",
//!         |p| Box::pin(async move {
//!             Ok(CallToolResult::success(vec![Content::text(p.message)]))
//!         }),
//!     )
//! }
//! ```

use std::sync::Arc;

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Tool},
};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;

/// A fully self-contained MCP tool definition.
///
/// Carries the tool's name, description, JSON schema, and an async handler
/// that parses its own params from [`CallToolRequestParams`].
///
/// Create via [`make_descriptor`]; use [`DescriptorPlugin`] to expose a
/// static slice as an [`ElicitPlugin`].
pub struct ToolDescriptor {
    /// Bare tool name (no namespace prefix).
    pub name: &'static str,
    /// Human-readable description shown to the agent.
    pub description: &'static str,
    /// rmcp [`Tool`] built from the param type's JSON schema.
    pub(crate) tool: Tool,
    /// Async handler: parses params and executes the tool.
    pub(crate) handler: Arc<
        dyn Fn(CallToolRequestParams) -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
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

/// Build a [`ToolDescriptor`] from a typed param struct and an async handler.
///
/// The helper:
/// - generates the JSON schema from `T` via [`JsonSchema`]
/// - wraps `handler` to parse `T` from raw [`CallToolRequestParams`]
/// - returns a self-contained [`ToolDescriptor`]
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
    let schema_value = serde_json::to_value(schemars::schema_for!(T))
        .unwrap_or(serde_json::Value::Object(Default::default()));
    let schema_obj = match schema_value {
        serde_json::Value::Object(m) => Arc::new(m),
        _ => Arc::new(Default::default()),
    };
    let tool = Tool::new(name, description, schema_obj);
    let handler = Arc::new(move |params: CallToolRequestParams| {
        let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
        let result: Result<T, _> = serde_json::from_value(value);
        match result {
            Ok(typed) => handler(typed),
            Err(e) => Box::pin(async move { Err(ErrorData::invalid_params(e.to_string(), None)) }),
        }
    });
    ToolDescriptor {
        name,
        description,
        tool,
        handler,
    }
}

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

impl ToolDescriptor {
    /// Return the rmcp [`Tool`] (schema + metadata) for this descriptor.
    pub fn as_tool(&self) -> Tool {
        self.tool.clone()
    }

    /// Invoke the handler with the given params.
    pub fn dispatch(
        &self,
        params: CallToolRequestParams,
    ) -> BoxFuture<'static, Result<CallToolResult, ErrorData>> {
        (self.handler)(params)
    }
}
