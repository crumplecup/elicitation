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
//! impl PathBuf {
//!     pub fn exists(&self) -> bool {
//!         self.0.exists()
//!     }
//! }
//! ```
//!
//! This generates:
//! - Parameter structs for each method (with #[derive(Elicit)])
//! - Wrapper methods that delegate to inner type
//! - MCP tool registrations via #[tool] attributes

mod discovery;
mod params;
mod wrapper;

use discovery::discover_methods;
use params::generate_param_struct;
use wrapper::generate_wrapper_method;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

/// Entry point for the #[reflect_methods] attribute macro.
///
/// This macro:
/// 1. Parses the impl block
/// 2. Discovers public methods that the user has added
/// 3. Generates parameter structs for method arguments
/// 4. Generates MCP tool wrapper methods with #[tool] attributes
/// 5. Keeps the original impl block intact
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    // Extract impl type for wrapper impl block
    let impl_type = &impl_block.self_ty;

    // Discover methods from the impl block
    let methods = discover_methods(&impl_block);

    // Generate parameter structs for each method
    let param_structs: Vec<TokenStream2> = methods
        .iter()
        .filter(|method| !method.params.is_empty()) // Only generate if method has parameters
        .map(|method| generate_param_struct(&method.name, &method.params))
        .collect();

    // Generate wrapper methods with #[tool] attributes
    // These have _tool suffix to avoid name collision with original methods
    let wrapper_methods: Vec<TokenStream2> = methods
        .iter()
        .map(|method| {
            // Check if method is async by looking at signature
            let is_async = method.signature.asyncness.is_some();

            generate_wrapper_method(
                &format!("{}_tool", method.name),  // Wrapper method name with _tool suffix
                &method.name,                       // Original method name (for params and delegation)
                &method.params,
                &method.return_type,
                is_async,
            )
        })
        .collect();

    // Output: parameter structs + original impl block + wrapper impl block
    TokenStream::from(quote! {
        #(#param_structs)*

        #impl_block

        // MCP tool wrappers
        impl #impl_type {
            #(#wrapper_methods)*
        }
    })
}
