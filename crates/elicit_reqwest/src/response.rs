//! Response wrapper for reqwest HTTP responses.
//!
//! Provides an elicitation-enabled wrapper around reqwest::Response
//! with MCP tool generation for response reading methods.
//!
//! Most Response methods are non-generic and use `elicit_newtype_methods!`,
//! with only `json<T>()` requiring generic support via `#[reflect_methods]`.

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::Response, as Response);
