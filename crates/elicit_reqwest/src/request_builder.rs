//! RequestBuilder wrapper for reqwest request builder.
//!
//! Provides an elicitation-enabled wrapper around reqwest::RequestBuilder
//! with MCP tool generation for request configuration methods.
//!
//! This module demonstrates mixed macro usage:
//! - Non-generic methods use `elicit_newtype_methods!`
//! - Generic methods use `#[reflect_methods]`

use elicitation::elicit_newtype;

// Note: reqwest::RequestBuilder does not implement Clone, so we cannot use
// consuming methods with `elicit_newtype_methods!` (the fallback clone path
// would fail compilation even if never executed).
//
// This is a known limitation of declarative macros - proc macros could handle
// this by generating conditional code based on trait bounds.
//
// For Phase 2 demonstration, we use only the basic newtype wrapper.
// Phase 3 will use #[reflect_methods] proc macro for generic methods.

elicit_newtype!(reqwest::RequestBuilder, as RequestBuilder);
