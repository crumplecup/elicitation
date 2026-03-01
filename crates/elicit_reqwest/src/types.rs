//! Type re-exports and wrappers for reqwest types.
//!
//! This module provides elicitation-enabled wrappers for reqwest's
//! supporting types like URLs, headers, and HTTP methods.

use elicitation::elicit_newtype;

// Re-export enums directly from reqwest (no wrapping needed)
pub use reqwest::{Method, StatusCode, Version};

// Wrap types we control for elicitation support
elicit_newtype!(url::Url, as Url);
elicit_newtype!(http::HeaderMap, as HeaderMap);
