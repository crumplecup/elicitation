//! Primitive type implementations.
//!
//! This module provides `Elicitation` implementations for all Rust primitive types:
//! - Integer types: i8, i16, i32, i64, u8, u16, u32, u64
//! - Floating-point types: f32, f64
//! - Boolean: bool
//! - String: String
//! - PathBuf: std::path::PathBuf
//! - Network types: IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, etc.
//!
//! All primitive types implement:
//! - [`Prompt`](crate::Prompt) - Provides default prompts
//! - [`Elicitation`](crate::Elicitation) - Async elicitation via MCP
//!
//! Integer and float types use generic macros to eliminate duplication.

mod boolean;
mod floats;
mod integers;
mod network;
mod pathbuf;
mod string;
