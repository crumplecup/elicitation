//! Derive macros for elicitation patterns.
//!
//! This crate provides the `#[derive(Elicit)]` macro for automatically
//! implementing elicitation traits on enums and structs.
//!
//! # Enum Derivation (Select Pattern)
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! // Derives Select pattern for enums (Phase 4)
//! #[derive(Elicit)]
//! enum Mode {
//!     Fast,
//!     Safe,
//! }
//!
//! // Custom prompt
//! #[derive(Elicit)]
//! #[prompt("Choose your preferred mode:")]
//! enum Color {
//!     Red,
//!     Green,
//!     Blue,
//! }
//! ```
//!
//! # Struct Derivation (Survey Pattern)
//!
//! **Note**: Struct derivation planned for Phase 5.
//!
//! ```rust,ignore
//! use elicitation::Elicit;
//!
//! // Derives Survey pattern for structs (Phase 5)
//! #[derive(Elicit)]
//! struct Config {
//!     #[prompt("Enter timeout in seconds:")]
//!     timeout: u32,
//!     mode: Mode,
//! }
//! ```

#![forbid(unsafe_code)]

extern crate proc_macro;

mod derive_elicit;
mod enum_impl;
mod struct_impl;

use proc_macro::TokenStream;

/// Derive the Elicit trait for enums (→ Select) or structs (→ Survey).
///
/// # Supported Attributes
///
/// - `#[prompt("...")]` - Custom prompt text (applies to type or fields)
/// - `#[alts([...])]` - Synonym mappings for enum variants (planned for v0.2.0)
/// - `#[skip]` - Skip a struct field during elicitation (planned for Phase 5)
///
/// # Enum Derivation
///
/// For enums, generates implementations of:
/// - `Prompt` - Provides prompt text
/// - `Select` - Finite options pattern
/// - `Elicit` - Calls `elicit_select` MCP tool
///
/// Only unit variants (no fields) are supported in v0.1.0.
///
/// # Struct Derivation
///
/// **Status**: Planned for Phase 5.
///
/// For structs, will generate implementations of:
/// - `Prompt` - Provides prompt text
/// - `Survey` - Multi-field elicitation pattern
/// - `Elicit` - Sequential field elicitation
///
/// # Examples
///
/// ```rust,ignore
/// use elicitation::Elicit;
///
/// #[derive(Elicit)]
/// enum Status {
///     Active,
///     Inactive,
/// }
///
/// #[derive(Elicit)]
/// #[prompt("Select your favorite color:")]
/// enum Color {
///     Red,
///     Green,
///     Blue,
/// }
/// ```
#[proc_macro_derive(Elicit, attributes(prompt, alts, skip))]
pub fn derive_elicit(input: TokenStream) -> TokenStream {
    derive_elicit::expand(input)
}
