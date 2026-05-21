//! [`DescriptorPlugin`] — blanket [`StatefulPlugin`] impl for descriptor-based plugins.
//!
//! Implement this trait instead of [`ElicitPlugin`](crate::ElicitPlugin) or [`StatefulPlugin`] directly when
//! your plugin exposes a static slice of [`ToolDescriptor`]s.  The blanket impl routes
//! through [`StatefulPlugin`] (with [`NoContext`]) and provides `list_tools` and
//! `call_tool` for free.

use rmcp::model::Tool;

use super::{StatefulPlugin, descriptor::ToolDescriptor};
use crate::plugin::context::NoContext;

/// A plugin that exposes a static slice of [`ToolDescriptor`]s.
///
/// Implement this instead of [`ElicitPlugin`](crate::ElicitPlugin) to eliminate manual
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

impl<T: DescriptorPlugin> StatefulPlugin for T {
    type Context = NoContext;

    fn name(&self) -> &'static str {
        DescriptorPlugin::name(self)
    }

    fn list_tools(&self) -> Vec<Tool> {
        self.descriptors().iter().map(|d| d.as_tool()).collect()
    }

    fn tool_descriptors(&self) -> Vec<ToolDescriptor> {
        self.descriptors().to_vec()
    }

    fn context(&self) -> std::sync::Arc<NoContext> {
        std::sync::Arc::new(NoContext)
    }
}
