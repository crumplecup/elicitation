//! Primitive type implementations.
//!
//! This module provides `Elicitation` implementations for all Rust primitive types:
//! - Integer types: i8, i16, i32, i64, u8, u16, u32, u64
//! - Floating-point types: f32, f64
//! - Boolean: bool
//! - Character: char
//! - String: String
//! - PathBuf: std::path::PathBuf
//! - Network types: IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, etc.
//! - Duration: std::time::Duration
//! - Tuples: (T1, T2), (T1, T2, T3), up to arity 12
//!
//! All primitive types implement:
//! - [`Prompt`](crate::Prompt) - Provides default prompts
//! - [`Elicitation`](crate::Elicitation) - Async elicitation via MCP
//!
//! Integer and float types use generic macros to eliminate duplication.

mod boolean;
mod char;
pub mod duration;
mod floats;
mod integers;
mod network;
mod pathbuf;
mod string;
pub mod systemtime;
mod tuples;

#[cfg(feature = "uuid")]
pub mod uuid;
