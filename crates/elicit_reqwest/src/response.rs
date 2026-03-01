//! Response wrapper for reqwest HTTP responses.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Response
//! with MCP tool generation for response reading methods.
//!
//! Most Response methods are non-generic and use `elicit_newtype_methods!`,
//! with only `json<T>()` requiring generic support via `#[reflect_methods]`.

use elicitation::elicit_newtype;

// Note: reqwest::Response does not implement Clone, so we cannot use
// consuming methods with `elicit_newtype_methods!` (same limitation as RequestBuilder).
//
// For Phase 2 demonstration, we use only the basic newtype wrapper.
// Phase 3 will use #[reflect_methods] proc macro for generic methods.

elicit_newtype!(reqwest::Response, as Response);
