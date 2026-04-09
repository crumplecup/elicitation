//! MCP workflow plugins for axum code generation.

pub mod error_gen_plugin;
pub mod handler_gen_plugin;
pub mod middleware_gen_plugin;
pub mod route_gen_plugin;
pub mod service_gen_plugin;
pub mod state_gen_plugin;
pub mod test_gen_plugin;

pub use error_gen_plugin::AxumErrorGenPlugin;
pub use handler_gen_plugin::AxumHandlerGenPlugin;
pub use middleware_gen_plugin::AxumMiddlewareGenPlugin;
pub use route_gen_plugin::AxumRouteGenPlugin;
pub use service_gen_plugin::AxumServiceGenPlugin;
pub use state_gen_plugin::AxumStateGenPlugin;
pub use test_gen_plugin::AxumTestGenPlugin;
