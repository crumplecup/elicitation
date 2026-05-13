//! `elicit_axum` — axum MCP tools.
//!
//! Provides 22 MCP tools across 4 plugins for describing and emitting
//! axum web service configurations. All config objects are stored
//! server-side in UUID-keyed registries; only serializable handles
//! (UUIDs, primitive config values) cross the MCP boundary.
//!
//! # Plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`AxumRouterPlugin`] | `axum_router__*` | Router descriptor: routes, layers, fallback |
//! | [`AxumHandlerPlugin`] | `axum_handler__*` | Handler descriptor: extractors, body |
//! | [`AxumResponsePlugin`] | `axum_response__*` | Response descriptor: JSON, HTML, redirect, status |
//! | [`AxumServePlugin`] | `axum_serve__*` | Serve descriptor: bind address, graceful shutdown |
//!
//! # Feature flags
//!
//! - `emit` — enables `EmitCode` code-recovery support.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod handler;
pub mod response;
pub mod router;
pub mod serve;

pub use handler::{AxumExtractorAdded, AxumHandlerDefined, AxumHandlerPlugin};
pub use response::{AxumResponseDefined, AxumResponsePlugin};
pub use router::{AxumRouteAdded, AxumRouterCreated, AxumRouterPlugin};
pub use serve::{AxumServePlugin, AxumServerConfigured};
