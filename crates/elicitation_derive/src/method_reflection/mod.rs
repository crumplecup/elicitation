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
mod validation;
mod wrapper;

use discovery::discover_methods;
use params::generate_param_struct;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ItemImpl, ReturnType, Type, TypePath, parse_macro_input};
use validation::validate_generic_bounds;
use wrapper::generate_wrapper_method;

/// Entry point for the #[reflect_methods] attribute macro.
///
/// This macro:
/// 1. Parses the impl block
/// 2. Discovers public methods that the user has added
/// 3. Validates that generic methods have required bounds
/// 4. Generates parameter structs for method arguments
/// 5. Generates MCP tool wrapper methods with #[tool] attributes
/// 6. Keeps the original impl block intact
pub fn expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Preserve the original token stream for output
    let original_item = proc_macro2::TokenStream::from(item.clone());

    let impl_block = parse_macro_input!(item as ItemImpl);

    // Extract impl type for wrapper impl block
    let impl_type = &impl_block.self_ty;

    // Discover methods from the impl block
    let methods = discover_methods(&impl_block);

    // Validate generic bounds for all generic methods
    for method in &methods {
        if method.is_generic()
            && let Err(err) = validate_generic_bounds(&method.generics)
        {
            return TokenStream::from(err.to_compile_error());
        }
    }

    // Generate parameter structs for each method
    let param_structs: Vec<TokenStream2> = methods
        .iter()
        .filter(|method| !method.params.is_empty()) // Only generate if method has parameters
        .map(|method| generate_param_struct(&method.name, &method.params, &method.generics))
        .collect();

    // Generate wrapper methods with #[tool] attributes
    // These have _tool suffix to avoid name collision with original methods
    //
    // NOTE: We skip generating tool wrappers for consuming methods that return Self.
    // These are builder-pattern methods meant for direct Rust usage, not MCP tools.
    // Only consuming methods that return other types (like final build/send methods)
    // get tool wrappers.
    let wrapper_methods: Vec<TokenStream2> = methods
        .iter()
        .filter(|method| {
            // Skip consuming methods that return Self (builder pattern)
            !(method.is_consuming() && returns_self_type(&method.return_type))
        })
        .map(|method| {
            // Check if method is async by looking at signature
            let is_async = method.signature.asyncness.is_some();

            generate_wrapper_method(
                &format!("{}_tool", method.name), // Wrapper method name with _tool suffix
                &method.name, // Original method name (for params and delegation)
                &method.params,
                &method.return_type,
                is_async,
                &method.generics,
                method.is_consuming(), // Whether method takes self or &self
            )
        })
        .collect();

    // Output: parameter structs + original impl block + wrapper impl block
    // NOTE: We use the original_item TokenStream instead of #impl_block to preserve
    // the exact original tokens and avoid any potential issues with ToTokens conversion
    TokenStream::from(quote! {
        #(#param_structs)*

        #original_item

        // MCP tool wrappers
        impl #impl_type {
            #(#wrapper_methods)*
        }
    })
}

/// Helper to check if a return type is `Self`.
fn returns_self_type(return_type: &ReturnType) -> bool {
    if let ReturnType::Type(_, ty) = return_type
        && let Type::Path(TypePath { path, .. }) = &**ty
        && let Some(segment) = path.segments.last()
    {
        return segment.ident == "Self";
    }
    false
}
