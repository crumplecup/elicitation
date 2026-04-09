//! MCP workflow plugins for axum-core traits.

pub mod from_ref_plugin;
pub mod from_request_plugin;
pub mod into_response_plugin;

pub use from_ref_plugin::AxumCoreFromRefPlugin;
pub use from_request_plugin::AxumCoreFromRequestPlugin;
pub use into_response_plugin::AxumCoreIntoResponsePlugin;
