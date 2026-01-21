//! Procedural macros for the elicitation library.
//!
//! This crate provides attribute macros for automatic instrumentation of contract types.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ItemImpl};

/// Attribute macro to add tracing instrumentation to impl blocks.
///
/// Apply to `impl` blocks to automatically instrument all public methods.
///
/// # Strategy
///
/// - **Constructors** (`new`, `from_*`, `try_*`): `#[instrument(ret, err)]`
/// - **Accessors** (`get`, `into_inner`, `as_*`, `to_*`): `#[instrument(level = "trace", ret)]`
/// - **Other methods**: `#[instrument(skip(self))]`
///
/// # Example
///
/// ```rust,ignore
/// #[instrumented_impl]
/// impl I8Positive {
///     pub fn new(value: i8) -> Result<Self, ValidationError> {
///         if value > 0 {
///             Ok(Self(value))
///         } else {
///             Err(ValidationError::NotPositive(value as i128))
///         }
///     }
///
///     pub fn get(&self) -> i8 {
///         self.0
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn instrumented_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(item as ItemImpl);
    
    // Process each method in the impl block
    for item in &mut impl_block.items {
        if let ImplItem::Fn(method) = item {
            // Only instrument public methods
            if matches!(method.vis, syn::Visibility::Public(_)) {
                let method_name = method.sig.ident.to_string();
                
                // Determine instrumentation strategy based on method name
                let instrument_attr = if is_constructor(&method_name) {
                    // Constructors: log args and return, with error tracking
                    quote! {
                        #[tracing::instrument(ret, err)]
                    }
                } else if is_accessor(&method_name) {
                    // Accessors: trace level to avoid noise
                    quote! {
                        #[tracing::instrument(level = "trace", ret)]
                    }
                } else {
                    // Other methods: standard debug
                    quote! {
                        #[tracing::instrument(skip(self))]
                    }
                };
                
                // Add instrumentation attribute at the beginning
                let attr: syn::Attribute = syn::parse_quote! { #instrument_attr };
                method.attrs.insert(0, attr);
            }
        }
    }
    
    TokenStream::from(quote! { #impl_block })
}

/// Check if method name indicates a constructor.
fn is_constructor(name: &str) -> bool {
    name == "new" 
        || name.starts_with("from_") 
        || name.starts_with("try_")
        || name == "default"
}

/// Check if method name indicates an accessor.
fn is_accessor(name: &str) -> bool {
    name == "get" 
        || name == "into_inner"
        || name.starts_with("as_")
        || name.starts_with("to_")
        || name.starts_with("get_")
}
