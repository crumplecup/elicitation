//! Type re-exports and wrappers for reqwest types.
//!
//! NOTE: This is a SHADOW CRATE demonstrating macro usage only.
//! url::Url already has Elicitation support via elicitation feature = "url".

use elicitation::elicit_newtype;

// Re-export enums directly from reqwest (no wrapping needed)
pub use reqwest::{Method, StatusCode, Version};

// url::Url is already supported in elicitation with feature = "url"
// We can re-export it directly:
pub use url::Url;

// HeaderMap wrapping demonstration
elicit_newtype!(http::HeaderMap, as HeaderMap);

// TODO: HeaderMap demonstration blocked pending http feature support in elicitation:
// - http::HeaderMap needs Elicitation + JsonSchema + Prompt impls
