//! MCP plugin implementations for all `elicit_reqwest` types.
//!
//! Each module exposes a plugin struct that implements [`ElicitPlugin`][elicitation::ElicitPlugin]
//! and can be registered in a [`PluginRegistry`][elicitation::PluginRegistry].
//!
//! # Available plugins
//!
//! | Plugin | Namespace | Tools |
//! |---|---|---|
//! | [`Plugin`] | `"http"` | `get`, `post`, `put`, `delete`, `patch`, `head` |
//! | [`StatusCodePlugin`] | `"status_code"` | `from_u16`, `as_str`, `canonical_reason`, `is_*` |
//! | [`UrlPlugin`] | `"url"` | `parse`, `scheme`, `host`, `port`, `path`, `query`, `join`, `set_*`, … |
//! | [`MethodPlugin`] | `"method"` | `from_str`, `as_str`, `is_safe`, `is_idempotent` |
//! | [`HeaderMapPlugin`] | `"header_map"` | `new`, `get`, `insert`, `remove`, `keys`, `values`, … |
//! | [`RequestBuilderPlugin`] | `"request_builder"` | `new_get/post/…`, `with_*`, `inspect`, `send` |
//!
//! # Example — full registry
//!
//! ```rust,no_run
//! use elicitation::PluginRegistry;
//! use elicit_reqwest::plugins::{
//!     Plugin, StatusCodePlugin, UrlPlugin, MethodPlugin, HeaderMapPlugin, RequestBuilderPlugin,
//! };
//!
//! let registry = PluginRegistry::new()
//!     .register("http",            Plugin::new())
//!     .register("status_code",     StatusCodePlugin)
//!     .register("url",             UrlPlugin)
//!     .register("method",          MethodPlugin)
//!     .register("header_map",      HeaderMapPlugin)
//!     .register("request_builder", RequestBuilderPlugin::new());
//! ```

mod header_map;
mod http;
mod method;
mod request_builder;
mod status_code;
mod url;
pub(crate) mod util;

pub use header_map::HeaderMapPlugin;
pub use http::Plugin;
pub use method::MethodPlugin;
pub use request_builder::{RequestBuilderPlugin, RequestSpec};
pub use status_code::StatusCodePlugin;
pub use url::UrlPlugin;
