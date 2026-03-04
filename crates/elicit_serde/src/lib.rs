//! MCP tool transport for serde — serialize/deserialize any registered Elicitation type as JSON.
//!
//! Uses `erased-serde` to erase the generic Serializer/Deserializer parameters,
//! making serde's capabilities available as concrete MCP tool calls.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elicit_serde::SerdePlugin;
//! use elicitation::PluginRegistry;
//!
//! #[tokio::main]
//! async fn main() {
//!     let registry = PluginRegistry::new()
//!         .register("serde", SerdePlugin);
//!     // registry.serve(rmcp::transport::stdio()).await.unwrap();
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod serde_plugin;

pub use serde_plugin::SerdePlugin;
