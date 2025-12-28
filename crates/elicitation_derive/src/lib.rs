//! Derive macros for elicitation patterns.
//!
//! This crate provides the `#[derive(Elicit)]` macro for automatically
//! implementing elicitation traits on enums and structs.
//!
//! # Usage
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! // Derives Select pattern for enums
//! #[derive(Elicit)]
//! enum Mode {
//!     Fast,
//!     Safe,
//! }
//!
//! // Derives Survey pattern for structs
//! #[derive(Elicit)]
//! struct Config {
//!     #[prompt("Enter timeout in seconds:")]
//!     timeout: u32,
//!     mode: Mode,
//! }
//! ```

#![forbid(unsafe_code)]

// Proc macro implementation will be added in Phase 4-5
// For now, provide a minimal stub to allow workspace compilation

extern crate proc_macro;

use proc_macro::TokenStream;

/// Derive the Elicit trait for enums (→ Select) or structs (→ Survey).
///
/// **Note**: Full implementation coming in Phase 4-5.
#[proc_macro_derive(Elicit, attributes(prompt, alts, skip))]
pub fn derive_elicit(_input: TokenStream) -> TokenStream {
    // Stub implementation - will be replaced in Phase 4
    TokenStream::new()
}
