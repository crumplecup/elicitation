//! Shadow crate wrapping reqwest HTTP client for elicitation.
//!
//! This crate provides MCP-aware wrappers around reqwest types with full
//! elicitation trait support. Types are drop-in replacements for their reqwest
//! counterparts: same names, same method signatures, same module layout.
//!
//! # Examples
//!
//! ```no_run
//! use elicit_reqwest::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new();
//!     // Use client methods...
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod body;
mod client;
mod client_builder;
mod client_trait;
mod context;
mod error;
mod plugin;
pub mod plugins;
mod proxy;
pub mod redirect;
mod request;
mod request_builder;
mod response;
mod tls;
mod types;

pub use body::Body;
pub use client::Client;
pub use client_builder::ClientBuilder;
pub use client_trait::HttpClient;
pub use context::HttpContext;
pub use error::{Error, ErrorFlags};
pub use plugin::Plugin;
pub use plugins::Plugin as HttpPlugin;
pub use plugins::{
    AuthFetchSucceeded, AuthType, Authorized, BuildRequestParams, BuildRequestParamsBuilder,
    ContentType, FetchResult, FetchSucceeded, HeaderMapPlugin, MethodPlugin, RequestBuilderPlugin,
    RequestCompleted, RequestSpec, StatusCodePlugin, StatusSuccess, UrlPlugin, UrlValid,
    WorkflowPlugin, apply_auth, do_fetch, do_post, extract_link_next, timeout, urlencoding_simple,
};
pub use proxy::{NoProxy, Proxy};
pub use redirect::{Action, Policy};
pub use request::Request;
pub use request_builder::RequestBuilder;
pub use response::Response;
pub use tls::{Certificate, CertificateRevocationList, Identity, TlsInfo, TlsVersion};
pub use types::{HeaderMap, HeaderValue, Method, StatusCode, Url, Version};
