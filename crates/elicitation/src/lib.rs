//! Conversational elicitation of strongly-typed Rust values via MCP.
//!
//! The `elicitation` library provides a trait-based system for eliciting
//! strongly-typed values from users through conversational interaction via
//! the Model Context Protocol (MCP). It transforms LLM conversations into
//! type-safe Rust values with compile-time guarantees.
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
//! ```rust,no_run
//! use elicitation::{Elicit, ElicitResult};
//!
//! # async fn example(client: &pmcp::Client) -> ElicitResult<()> {
//! // Elicit a simple integer
//! let age: i32 = i32::elicit(client).await?;
//!
//! // Elicit an optional value
//! let nickname: Option<String> = Option::<String>::elicit(client).await?;
//!
//! // Elicit a collection
//! let scores: Vec<i32> = Vec::<i32>::elicit(client).await?;
//! # Ok(())
//! # }
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
//! The library uses the [pmcp](https://crates.io/crates/pmcp) crate for
//! high-performance MCP client integration. All elicitation happens through
//! asynchronous MCP tool calls.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod containers;
mod error;
mod mcp;
mod paradigm;
mod primitives;
mod traits;

// Error types
pub use error::{ElicitError, ElicitErrorKind, ElicitResult};

// Core traits
pub use traits::{Elicit, Prompt};

// Interaction paradigm traits
pub use paradigm::{Affirm, Authorize, FieldInfo, Select, Survey};

// Re-export pmcp for user convenience
pub use pmcp;

// Re-export derive macro (will be implemented in Phase 4-5)
// pub use elicitation_derive::Elicit as DeriveElicit;
