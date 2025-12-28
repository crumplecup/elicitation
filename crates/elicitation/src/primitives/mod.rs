//! Primitive type implementations.
//!
//! This module provides `Elicit` implementations for all Rust primitive types:
//! - Integer types: i8, i16, i32, i64, u8, u16, u32, u64
//! - Floating-point types: f32, f64
//! - Boolean: bool
//! - String: String
//!
//! All primitive types implement:
//! - [`Prompt`](crate::Prompt) - Provides default prompts
//! - [`Elicit`](crate::Elicit) - Async elicitation via MCP
//!
//! Integer and float types use generic macros to eliminate duplication.

mod boolean;
mod floats;
mod integers;
mod string;
