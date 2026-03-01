//! Shadow crate wrapping reqwest HTTP client for elicitation.
//!
//! This crate demonstrates and tests all elicitation macro capabilities,
//! especially the new generic support. It provides transparent wrappers
//! around reqwest types with automatic MCP tool generation.
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

mod client;
mod error;
mod request_builder;
mod response;
mod types;

pub use client::Client;
pub use error::Error;
pub use request_builder::RequestBuilder;
pub use response::Response;
pub use types::{HeaderMap, Method, StatusCode, Url, Version};
