//! Type re-exports and wrappers for reqwest types.

// Re-export enums directly from reqwest (now with full Elicitation support)
pub use reqwest::{Method, StatusCode, Version};

// url::Url has Elicitation support via elicitation feature = "url"
pub use url::Url;

// http::HeaderMap has Elicitation support via elicitation feature = "reqwest"
pub use http::HeaderMap;
