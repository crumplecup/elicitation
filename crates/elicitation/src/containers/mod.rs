//! Container type implementations.
//!
//! This module provides `Elicit` implementations for standard Rust containers:
//! - Option<T>: Optional values
//! - Vec<T>: Collections of values
//!
//! All container types implement:
//! - [`Prompt`](crate::Prompt) - Provides default prompts
//! - [`Elicit`](crate::Elicit) - Async elicitation via MCP
//!
//! Container implementations are generic over any `T: Elicit`, allowing
//! composition of elicitation patterns.

mod option;
mod vec;
