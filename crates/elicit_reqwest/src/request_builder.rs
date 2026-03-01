//! RequestBuilder wrapper for reqwest request builder.
//!
//! Provides an elicitation-enabled wrapper around reqwest::RequestBuilder
//! with MCP tool generation for request configuration methods.
//!
//! This module demonstrates mixed macro usage:
//! - Non-generic methods use `elicit_newtype_methods!`
//! - Generic methods use `#[reflect_methods]`

use elicitation::elicit_newtype;

elicit_newtype!(reqwest::RequestBuilder, as RequestBuilder);
