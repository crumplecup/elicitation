//! MCP fragment tools for the Rust standard library.
//!
//! These are **fragment tools**: each tool takes macro parameters as JSON,
//! validates them, and returns the equivalent Rust source fragment.  Fragments
//! are composable — an agent can pass a fragment string returned by one tool
//! as an input expression to another, building up complex expressions before
//! assembling them into a binary.
//!
//! The final step is [`std__assemble`](AssembleParams): it takes an ordered
//! list of statement-level fragments and assembles them into a complete,
//! compilable `main.rs` plus a matching `Cargo.toml`.
//!
//! # Fragment pipeline
//!
//! ```text
//! std__env { var: "USER" }          → env!("USER")
//!     ↓ pass as arg to ↓
//! std__format { template: "Hello, {}!", args: ["env!(\"USER\")"] }
//!                                   → format!("Hello, {}!", env!("USER"))
//!     ↓ pass as step to ↓
//! std__assemble { steps: ["let msg = format!(...); println!(\"{}\", msg);"] }
//!                                   → main.rs + Cargo.toml
//! ```
//!
//! # Tools
//!
//! | Tool | Kind | Description |
//! |---|---|---|
//! | `std__format` | fragment | Emit a `format!(template, args…)` expression |
//! | `std__include_str` | fragment | Emit an `include_str!("path")` expression |
//! | `std__env` | fragment | Emit an `env!("VAR")` compile-time env expression |
//! | `std__concat` | fragment | Emit a `concat!(parts…)` string literal expression |
//! | `std__assemble` | terminal | Assemble statement fragments into a compilable binary |
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

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod assemble;
mod concat_macro;
mod env_macro;
mod format_macro;
mod include_str_macro;
mod plugin;

pub use assemble::AssembleParams;
pub use concat_macro::ConcatParams;
pub use env_macro::EnvParams;
pub use format_macro::FormatParams;
pub use include_str_macro::IncludeStrParams;
pub use plugin::StdMacrosPlugin;
