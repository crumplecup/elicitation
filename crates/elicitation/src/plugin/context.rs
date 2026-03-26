//! [`PluginContext`] — marker trait for plugin-specific server-side state.
//!
//! Each shadow crate that needs long-lived resources (connection pools,
//! credentials, transaction registries) defines its own context struct and
//! implements [`PluginContext`].  The context is held behind `Arc` and shared
//! across all tool calls on the same server instance.
//!
//! Stateless plugins that require no server-side state use [`NoContext`].

/// Marker trait for plugin-specific server-side state.
///
/// Implement this on a struct that holds the resources a plugin needs across
/// tool calls (e.g. an HTTP client, a database pool).  The struct is wrapped
/// in `Arc` and passed to every context-aware tool handler.
///
/// # Example
///
/// ```rust
/// use elicitation::plugin::PluginContext;
///
/// pub struct MyContext {
///     pub value: u32,
/// }
///
/// impl PluginContext for MyContext {}
/// ```
pub trait PluginContext: std::any::Any + Send + Sync + 'static {}

/// No-op context for stateless plugins.
///
/// Plugins that require no server-side state use `NoContext` as their context
/// type.  `#[derive(ElicitPlugin)]` on a unit struct automatically uses this.
#[derive(Debug, Default, Clone)]
pub struct NoContext;

impl PluginContext for NoContext {}
