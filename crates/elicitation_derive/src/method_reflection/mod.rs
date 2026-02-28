//! Automatic method reflection for generating MCP tool wrappers.
//!
//! This module provides the `#[reflect_methods]` proc macro for automatically
//! discovering methods on newtype wrappers and generating MCP tool interfaces.
//!
//! # Example
//!
//! ```ignore
//! use elicitation::elicit_newtype;
//!
//! // Create newtype wrapper
//! elicit_newtype!(::std::path::PathBuf, as PathBuf);
//!
//! // Automatically discover and wrap methods
//! #[reflect_methods]
//! impl PathBuf { }
//! ```
//!
//! This generates:
//! - Parameter structs for each method (with #[derive(Elicit)])
//! - Wrapper methods that delegate to inner type
//! - MCP tool registrations via #[tool] attributes

mod discovery;
mod params;
mod wrapper;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

/// Entry point for the #[reflect_methods] attribute macro.
///
/// This macro:
/// 1. Parses the impl block
/// 2. Discovers public methods on the wrapped type
/// 3. Generates parameter structs for method arguments
/// 4. Generates wrapper methods with #[tool] attributes
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    // TODO: Implement method discovery
    // TODO: Generate parameter structs
    // TODO: Generate wrapper methods

    // For now, return the impl block unchanged
    TokenStream::from(quote::quote! { #impl_block })
}
