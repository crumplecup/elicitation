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
//! | [`WorkflowPlugin`] | `"workflow"` | `url_build`, `fetch`, `fetch_json`, `fetch_auth`, `post_json`, `api_call`, `health_check`, `build_request`, `status_summary`, `paginated_get` |
//!
//! # Example — full registry
//!
//! ```rust,no_run
//! use elicitation::PluginRegistry;
//! use elicit_reqwest::plugins::{
//!     Plugin, StatusCodePlugin, UrlPlugin, MethodPlugin, HeaderMapPlugin,
//!     RequestBuilderPlugin, WorkflowPlugin,
//! };
//!
//! let registry = PluginRegistry::new()
//!     .register("http",            Plugin::new())
//!     .register("status_code",     StatusCodePlugin)
//!     .register("url",             UrlPlugin)
//!     .register("method",          MethodPlugin)
//!     .register("header_map",      HeaderMapPlugin)
//!     .register("request_builder", RequestBuilderPlugin::new())
//!     .register("workflow",        WorkflowPlugin::default_client());
//! ```

mod header_map;
mod http;
mod method;
mod request_builder;
mod status_code;
mod url;
pub(crate) mod util;
mod workflow;

pub use header_map::HeaderMapPlugin;
pub use http::Plugin;
pub use method::MethodPlugin;
pub use request_builder::{RequestBuilderPlugin, RequestSpec};
pub use status_code::StatusCodePlugin;
pub use url::UrlPlugin;
pub use workflow::WorkflowPlugin;
pub use workflow::{
    AuthFetchSucceeded, AuthType, Authorized, BuildRequestParams, BuildRequestParamsBuilder,
    ContentType, FetchResult, FetchSucceeded, RequestCompleted, StatusSuccess, UrlValid,
    apply_auth, do_fetch, do_post, extract_link_next, timeout, urlencoding_simple,
};
