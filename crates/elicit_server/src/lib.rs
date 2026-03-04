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
//!
//! # Feature flags
//!
//! | Feature | Enables |
//! |---|---|
//! | `emit` | `EmitBinaryPlugin` + full code recovery pipeline |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "emit")]
mod util;

#[cfg(feature = "emit")]
mod emit_plugin;

#[cfg(feature = "emit")]
pub use emit_plugin::{EmitBinaryParams, EmitBinaryPlugin, WorkflowStep};
