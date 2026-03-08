//! [`DescriptorPlugin`] — blanket [`ElicitPlugin`] impl for descriptor-based plugins.
//!
//! Implement this trait instead of [`ElicitPlugin`] directly when your plugin
//! exposes a static slice of [`ToolDescriptor`]s.  The blanket impl provides
//! `list_tools` and `call_tool` for free.

use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Tool},
    service::RequestContext,
};
use tracing::instrument;

use crate::rmcp::RoleServer;

use super::{ElicitPlugin, descriptor::ToolDescriptor};

/// A plugin that exposes a static slice of [`ToolDescriptor`]s.
///
/// Implement this instead of [`ElicitPlugin`] to eliminate manual
/// `list_tools` and `call_tool` dispatch.  A blanket impl provides
/// both for free.
///
/// # Example
///
/// ```rust,no_run
/// use elicitation::plugin::{DescriptorPlugin, ToolDescriptor, make_descriptor};
/// use rmcp::model::{CallToolResult, Content};
/// use schemars::JsonSchema;
/// use serde::Deserialize;
///
/// #[derive(Deserialize, JsonSchema)]
/// struct PingParams { message: String }
///
/// pub struct MyPlugin;
///
/// impl DescriptorPlugin for MyPlugin {
///     fn name(&self) -> &'static str { "my_plugin" }
///     fn descriptors(&self) -> &'static [ToolDescriptor] { &TOOLS }
/// }
///
/// static TOOLS: std::sync::LazyLock<Vec<ToolDescriptor>> = std::sync::LazyLock::new(|| {
///     vec![make_descriptor::<PingParams, _>(
///         "ping", "Echo a message",
///         |p| Box::pin(async move {
///             Ok(CallToolResult::success(vec![Content::text(p.message)]))
///         }),
///     )]
/// });
/// ```
pub trait DescriptorPlugin: Send + Sync + 'static {
    /// Plugin namespace prefix (e.g. `"secure_fetch"`).
    fn name(&self) -> &'static str;

    /// Static list of tool descriptors for this plugin.
    fn descriptors(&self) -> &[ToolDescriptor];
}

impl<T: DescriptorPlugin> ElicitPlugin for T {
    fn name(&self) -> &'static str {
        DescriptorPlugin::name(self)
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.descriptors().iter().map(|d| d.as_tool()).collect()
    }

    #[instrument(skip(self), fields(plugin = self.name(), tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        let bare = params
            .name
            .strip_prefix(&format!("{name}__", name = self.name()))
            .map(|s| s.to_owned())
            .unwrap_or_else(|| params.name.to_string());

        match self.descriptors().iter().find(|d| d.name == bare.as_str()) {
            Some(descriptor) => descriptor.dispatch(params),
            None => Box::pin(async move {
                Err(ErrorData::invalid_params(
                    format!("unknown tool: {bare}"),
                    None,
                ))
            }),
        }
    }
}
