//! `elicit_axum_core` — axum-core trait factory MCP tools.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{AxumCoreFromRefPlugin, AxumCoreFromRequestPlugin, AxumCoreIntoResponsePlugin};
