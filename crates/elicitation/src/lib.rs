//! Conversational elicitation of strongly-typed Rust values via MCP.
//!
//! The `elicitation` library provides a trait-based system for eliciting
//! strongly-typed values from users through conversational interaction via
//! the Model Context Protocol (MCP). It transforms LLM conversations into
//! type-safe Rust values with compile-time guarantees.
//!
//! # MCP Setup Required
//!
//! This library runs as an **MCP server** and requires an **MCP client**
//! (like Claude Desktop or Claude CLI) to provide the elicitation tools.
//! Your application won't work standalone - it must be invoked by an MCP client.
//!
//! See the [README](https://github.com/crumplecup/elicitation) for setup instructions.
//!
//! # Core Concepts
//!
//! ## Traits
//!
//! - [`Prompt`] - Provides prompt metadata for a type
//! - [`Elicit`] - Main trait for eliciting values
//!
//! ## Interaction Paradigms
//!
//! - [`Select`] - Choose from finite options (enum pattern)
//! - [`Affirm`] - Yes/no confirmation (bool pattern)
//! - [`Survey`] - Multi-field elicitation (struct pattern)
//! - [`Authorize`] - Permission policies (planned for v0.2.0)
//!
//! # Example
//!
//! ```rust,ignore
//! use elicitation::{Elicitation, ElicitResult};
//! use rmcp::service::{Peer, RoleClient};
//!
//! async fn example(client: &Peer<RoleClient>) -> ElicitResult<()> {
//!     // Elicit a simple integer
//!     let age: i32 = i32::elicit(client).await?;
//!
//!     // Elicit an optional value
//!     let nickname: Option<String> = Option::<String>::elicit(client).await?;
//!
//!     // Elicit a collection
//!     let scores: Vec<i32> = Vec::<i32>::elicit(client).await?;
//!     Ok(())
//! }
//! ```
//!
//! # Derive Macros
//!
//! The library provides derive macros for automatic implementation:
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! // Enums automatically use the Select paradigm
//! #[derive(Elicit)]
//! enum Color {
//!     Red,
//!     Green,
//!     Blue,
//! }
//!
//! // Structs automatically use the Survey paradigm
//! #[derive(Elicit)]
//! struct Person {
//!     #[prompt("What is your name?")]
//!     name: String,
//!     #[prompt("What is your age?")]
//!     age: u8,
//! }
//! ```
//!
//! # MCP Integration
//!
//! The library uses the [rmcp](https://crates.io/crates/rmcp) crate - the
//! official Rust MCP SDK - for MCP client integration. All elicitation
//! happens through asynchronous MCP tool calls.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod collections;
mod containers;
mod error;
pub mod mcp;
mod paradigm;
mod primitives;
pub mod style;
mod traits;

#[cfg(feature = "serde_json")]
mod value_impl;

#[cfg(any(feature = "chrono", feature = "time", feature = "jiff"))]
mod datetime_common;

#[cfg(feature = "chrono")]
mod datetime_chrono;

#[cfg(feature = "time")]
mod datetime_time;

#[cfg(feature = "jiff")]
mod datetime_jiff;

// Error types
pub use error::{ElicitError, ElicitErrorKind, ElicitResult, JsonError, RmcpError, ServiceError};

// Core traits
pub use traits::{Elicitation, Prompt};

// Interaction paradigm traits
pub use paradigm::{Affirm, Authorize, FieldInfo, Select, Survey};

// Re-export rmcp for user convenience
pub use rmcp;

// Re-export derive macro with user-friendly name
pub use elicitation_derive::Elicit;
