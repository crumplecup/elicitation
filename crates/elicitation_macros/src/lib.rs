//! Procedural macros for the elicitation library.
//!
//! This crate provides attribute macros for automatic instrumentation of contract types.

use proc_macro::TokenStream;
use quote::quote;
use syn::{ImplItem, ItemImpl, parse_macro_input};

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
/// # Kani Compatibility
///
/// When compiling under Kani (formal verification), this macro becomes a no-op.
/// Instrumentation is for runtime observability, not formal verification.
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
    let impl_block = parse_macro_input!(item as ItemImpl);

    // Under Kani, return impl block unchanged (no instrumentation needed)
    #[cfg(kani)]
    {
        return TokenStream::from(quote! { #impl_block });
    }

    // Normal compilation: add instrumentation
    #[cfg(not(kani))]
    {
        let mut impl_block = impl_block;

        // Process each method in the impl block
        for item in &mut impl_block.items {
            if let ImplItem::Fn(method) = item {
                // Only instrument public methods
                if matches!(method.vis, syn::Visibility::Public(_)) {
                    let method_name = method.sig.ident.to_string();
                    let has_generics = !method.sig.generics.params.is_empty();

                    // Determine instrumentation strategy based on method name
                    let instrument_attr = if is_constructor(&method_name) {
                        // Constructors: track errors and parameters
                        if has_generics {
                            // Skip generic parameters (can't guarantee Debug)
                            let param_names: Vec<_> = method.sig.inputs.iter()
                                .filter_map(|arg| {
                                    if let syn::FnArg::Typed(pat_type) = arg {
                                        if let syn::Pat::Ident(ident) = &*pat_type.pat {
                                            return Some(ident.ident.clone());
                                        }
                                    }
                                    None
                                })
                                .collect();
                            
                            // For constructors with generics, skip params but track errors
                            quote! {
                                #[tracing::instrument(skip(#(#param_names),*), err)]
                            }
                        } else {
                            // For constructors without generics, log all params and errors
                            quote! {
                                #[tracing::instrument(err)]
                            }
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
}

/// Check if method name indicates a constructor.
fn is_constructor(name: &str) -> bool {
    name == "new" || name.starts_with("from_") || name.starts_with("try_") || name == "default"
}

/// Check if method name indicates an accessor.
fn is_accessor(name: &str) -> bool {
    name == "get"
        || name == "into_inner"
        || name.starts_with("as_")
        || name.starts_with("to_")
        || name.starts_with("get_")
}
