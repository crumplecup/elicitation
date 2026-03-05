//! Cross-crate workflow plugins for elicitation.
//!
//! `elicit_server` houses workflows that require visibility across multiple
//! elicitation crates simultaneously — things that can't live in `elicit_reqwest`
//! or `elicit_serde_json` without creating circular dependencies.
//!
//! # Plugins
//!
//! - [`EmitBinaryPlugin`] — recover agent tool compositions as compiled Rust
//!   binaries (requires `feature = "emit"`)
//! - [`SecureFetchPlugin`] — HTTPS-enforced URL validation + HTTP fetch
//!   (`elicit_url` + `elicit_reqwest`)
//! - [`FetchAndParsePlugin`] — HTTP fetch + JSON extraction
//!   (`elicit_reqwest` + `elicit_serde_json`)
//!
//! # Feature flags
//!
//! | Feature | Enables |
//! |---|---|
//! | `emit` | `EmitBinaryPlugin` + full code recovery pipeline |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod fetch_and_parse;
mod secure_fetch;
mod util;

#[cfg(feature = "emit")]
mod emit_plugin;

pub use fetch_and_parse::FetchAndParsePlugin;
pub use secure_fetch::SecureFetchPlugin;

#[cfg(feature = "emit")]
pub use emit_plugin::{EmitBinaryParams, EmitBinaryPlugin, WorkflowStep};
#[cfg(feature = "emit")]
pub use fetch_and_parse::dispatch_fetch_and_parse_emit;
#[cfg(feature = "emit")]
pub use secure_fetch::dispatch_secure_fetch_emit;
