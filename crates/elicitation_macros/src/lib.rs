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
                            let param_names: Vec<_> = method
                                .sig
                                .inputs
                                .iter()
                                .filter_map(|arg| {
                                    if let syn::FnArg::Typed(pat_type) = arg
                                        && let syn::Pat::Ident(ident) = &*pat_type.pat
                                    {
                                        return Some(ident.ident.clone());
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

/// Generates elicitation tool methods inside an impl block for rmcp tool_router.
///
/// This proc macro runs BEFORE `#[tool_router]`, generating methods with `#[tool]` markers
/// that `#[tool_router]` will discover and register.
///
/// # How It Works
///
/// For each type T, generates a method in the correct shape:
/// - Already transformed to `Pin<Box<dyn Future>>` (not async)
/// - Marked with `#[tool(description = "...")]` for discovery
/// - rmcp's `#[tool]` processes it, generating metadata and async transformation
///
/// # Macro Execution Order
///
/// ```text
/// #[elicit_tools(Type1, Type2)]  ← 1. Runs first, adds methods with #[tool] markers
/// #[tool_router]                  ← 2. Runs second, discovers #[tool] methods
/// impl MyServer { }               ← 3. #[tool] on each method generates metadata
/// ```
///
/// # Example
///
/// ```rust,ignore
/// use elicitation_macros::elicit_tools;
/// use rmcp::{tool_router, Peer, RoleServer, ErrorData, Json};
///
/// #[elicit_tools(CacheKeyNewParams, StorageNewParams)]
/// #[tool_router]
/// impl BotticelliServer {
///     // Your existing tool methods...
/// }
/// ```
///
/// For each type, generates:
/// ```rust,ignore
/// #[tool(description = "Elicit CacheKeyNewParams via MCP")]
/// pub async fn elicit_cache_key_new_params(
///     peer: Peer<RoleServer>,
/// ) -> Result<Json<CacheKeyNewParams>, ErrorData> {
///     CacheKeyNewParams::elicit_checked(peer)
///         .await
///         .map(Json)
///         .map_err(|e| ErrorData::internal_error(e.to_string(), None))
/// }
/// // Note: *_tool_attr() generated by rmcp's #[tool] macro
/// ```
///
/// # Key Requirements
///
/// - **No `&self`**: Methods are standalone functions for rmcp's parameter extraction
/// - **Return `Json<T>`**: Wrapper implements `IntoCallToolResult` for structured responses
/// - **Use `ErrorData`**: rmcp's standard error type
/// - **`Peer<RoleServer>`**: Parameter extracted via `FromContextPart` trait
#[proc_macro_attribute]
pub fn elicit_tools(attr: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = parse_macro_input!(item as ItemImpl);

    // Parse comma-separated list of type names
    let types_input = attr.to_string();
    let types: Vec<&str> = types_input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if types.is_empty() {
        return syn::Error::new_spanned(
            &impl_block,
            "elicit_tools requires at least one type: #[elicit_tools(Type1, Type2)]",
        )
        .to_compile_error()
        .into();
    }

    // Clone the impl block to modify
    let mut new_impl = impl_block.clone();

    // Generate methods and metadata for each type
    for ty_str in types {
        // Parse the type
        let ty: syn::Type = match syn::parse_str(ty_str) {
            Ok(t) => t,
            Err(e) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse type '{}': {}", ty_str, e),
                )
                .to_compile_error()
                .into();
            }
        };

        // Convert type name to snake_case for method name
        let method_name = to_snake_case(ty_str);
        let method_ident = syn::Ident::new(
            &format!("elicit_{}", method_name),
            proc_macro2::Span::call_site(),
        );

        // Generate async method with #[tool] marker
        // NO &self parameter - use peer as standalone function parameter
        // Return Json<T> wrapper for proper rmcp integration (IntoCallToolResult)
        let tool_description = format!("Elicit {} via MCP", ty_str);
        let method: syn::ImplItemFn = syn::parse_quote! {
            #[doc = concat!("Elicit `", #ty_str, "` via MCP.")]
            #[tool(description = #tool_description)]
            pub async fn #method_ident(
                peer: ::rmcp::service::Peer<::rmcp::service::RoleServer>,
            ) -> ::std::result::Result<::rmcp::handler::server::wrapper::Json<#ty>, ::rmcp::ErrorData> {
                #ty::elicit_checked(peer)
                    .await
                    .map(::rmcp::handler::server::wrapper::Json)
                    .map_err(|e| ::rmcp::ErrorData::internal_error(e.to_string(), None))
            }
        };

        // Add method to impl block
        new_impl.items.push(syn::ImplItem::Fn(method));
    }

    // Output the modified impl block (with all original attributes preserved)
    TokenStream::from(quote! { #new_impl })
}

/// Convert PascalCase to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_lowercase = false;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            // Add underscore before uppercase if previous was lowercase
            if i > 0 && prev_was_lowercase {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
            prev_was_lowercase = false;
        } else {
            result.push(ch);
            prev_was_lowercase = ch.is_lowercase();
        }
    }

    result
}
