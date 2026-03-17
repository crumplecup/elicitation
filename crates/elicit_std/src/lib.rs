//! MCP macro tools for the Rust standard library.
//!
//! These are **emit-only tools**: each tool takes macro parameters as JSON,
//! validates them, and returns the equivalent Rust source fragment.  There is
//! no runtime execution — calling the tool *is* the code-emission step.
//!
//! # Tools
//!
//! | Tool | Macro | Description |
//! |---|---|---|
//! | `std__format` | `format!` | Emit a `format!(template, args…)` expression |
//! | `std__include_str` | `include_str!` | Emit an `include_str!("path")` expression |
//! | `std__env` | `env!` | Emit an `env!("VAR")` compile-time env expression |
//! | `std__concat` | `concat!` | Emit a `concat!(parts…)` string literal expression |
//!
//! # Usage
//!
//! ```rust,no_run
//! use elicit_std::StdMacrosPlugin;
//! use elicitation::PluginRegistry;
//!
//! #[tokio::main]
//! async fn main() {
//!     let registry = PluginRegistry::new()
//!         .register("std", StdMacrosPlugin);
//! }
//! ```
//!
//! An agent calling `std__format` with
//! `{ "template": "Hello, {}!", "args": ["name"] }` receives back:
//! `format!("Hello, {}!", name)`

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod concat_macro;
mod env_macro;
mod format_macro;
mod include_str_macro;
mod plugin;

pub use concat_macro::ConcatParams;
pub use env_macro::EnvParams;
pub use format_macro::FormatParams;
pub use include_str_macro::IncludeStrParams;
pub use plugin::StdMacrosPlugin;
