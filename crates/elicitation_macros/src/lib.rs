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
///
/// # MCP Object Schema Requirement
///
/// MCP specification requires tool output schemas to be objects (`"type": "object"`).
/// Enum types generate `"enum": [...]` schemas without a type field, causing validation
/// failures. This macro wraps all types in `ElicitToolOutput<T>` to ensure object schemas.
///
/// The generic wrapper is defined in `elicitation::ElicitToolOutput` and ensures ALL
/// types (structs, enums, primitives) produce valid MCP object schemas.
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

    // Generate methods for each type
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
        // Returns Json<ElicitToolOutput<T>> to ensure object schema for MCP
        let tool_description = format!("Elicit {} via MCP", ty_str);
        let method: syn::ImplItemFn = syn::parse_quote! {
            #[doc = concat!("Elicit `", #ty_str, "` via MCP.")]
            #[tool(description = #tool_description)]
            pub async fn #method_ident(
                peer: ::rmcp::service::Peer<::rmcp::service::RoleServer>,
            ) -> ::std::result::Result<
                ::rmcp::handler::server::wrapper::Json<::elicitation::ElicitToolOutput<#ty>>,
                ::rmcp::ErrorData
            > {
                let value = #ty::elicit_checked(peer).await
                    .map_err(|e| ::rmcp::ErrorData::internal_error(e.to_string(), None))?;
                Ok(::rmcp::handler::server::wrapper::Json(::elicitation::ElicitToolOutput::new(value)))
            }
        };

        // Add method to impl block
        new_impl.items.push(syn::ImplItem::Fn(method));
    }

    // Output the modified impl block
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

/// Generates MCP tool wrappers for trait methods.
///
/// This macro generates delegating wrapper methods for trait methods
/// that can be discovered by `#[tool_router]`.
///
/// # Usage
///
/// ```ignore
/// #[elicit_trait_tools_router(TraitName, field_name, [method1, method2])]
/// #[tool_router(router = my_tools)]
/// impl<T: TraitName> MyServer<T> {
///     // Generated tool methods will be added here
/// }
/// ```
///
/// # Arguments
///
/// 1. `TraitName` - The trait containing methods to expose as tools
/// 2. `field_name` - Field on the impl type holding the trait implementation
/// 3. `[method_list]` - List of trait method names to generate tools for
///
/// # Naming Conventions
///
/// For each method, the macro generates parameter and result types by convention:
/// - Method: `add` → Params: `AddParams`, Result: `AddResult`
/// - Method: `get_user` → Params: `GetUserParams`, Result: `GetUserResult`
///
/// # Example
///
/// ```ignore
/// trait Calculator: Send + Sync {
///     async fn add(&self, params: Parameters<AddParams>)
///         -> Result<Json<AddResult>, rmcp::ErrorData>;
/// }
///
/// struct Server<C: Calculator> {
///     calc: C,
/// }
///
/// #[elicit_trait_tools_router(Calculator, calc, [add, subtract])]
/// #[tool_router]
/// impl<C: Calculator> Server<C> {
///     // Generates:
///     // #[tool(description = "Add operation")]
///     // pub async fn add(&self, params: Parameters<AddParams>)
///     //     -> Result<Json<AddResult>, rmcp::ErrorData>
///     // {
///     //     self.calc.add(params).await
///     // }
/// }
/// ```
#[proc_macro_attribute]
pub fn elicit_trait_tools_router(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attribute: (TraitName, field_name, [method1, method2, ...])
    let attr_str = attr.to_string();

    // Split by comma, but handle array brackets
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;

    for ch in attr_str.chars() {
        match ch {
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth -= 1;
                current.push(ch);
            }
            ',' if bracket_depth == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    if parts.len() != 3 {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "elicit_trait_tools_router requires three arguments: #[elicit_trait_tools_router(TraitName, field_name, [method1, method2])]",
        )
        .to_compile_error()
        .into();
    }

    let _trait_name = &parts[0];
    let field_name = &parts[1];
    let methods_str = &parts[2];

    // Parse method list from [method1, method2, ...]
    let methods_str = methods_str.trim_start_matches('[').trim_end_matches(']');
    let methods: Vec<&str> = methods_str
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if methods.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "elicit_trait_tools_router requires at least one method in the list",
        )
        .to_compile_error()
        .into();
    }

    // Parse the impl block
    let mut impl_block = parse_macro_input!(item as ItemImpl);

    // Generate tool methods for each listed method
    for method_name in methods {
        // Convert method_name to PascalCase for type names
        let pascal_case = to_pascal_case(method_name);
        let params_type = format!("{}Params", pascal_case);
        let result_type = format!("{}Result", pascal_case);

        // Parse type names
        let params_ty: syn::Type = match syn::parse_str(&params_type) {
            Ok(t) => t,
            Err(e) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse params type '{}': {}", params_type, e),
                )
                .to_compile_error()
                .into();
            }
        };

        let result_ty: syn::Type = match syn::parse_str(&result_type) {
            Ok(t) => t,
            Err(e) => {
                return syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse result type '{}': {}", result_type, e),
                )
                .to_compile_error()
                .into();
            }
        };

        let method_ident = syn::Ident::new(method_name, proc_macro2::Span::call_site());
        let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());

        let tool_description = format!("{} operation", method_name.replace('_', " "));

        // Generate the delegating method
        let method: syn::ImplItemFn = syn::parse_quote! {
            #[doc = concat!("`", #method_name, "` operation via trait method delegation.")]
            #[::rmcp::tool(description = #tool_description)]
            pub async fn #method_ident(
                &self,
                params: ::rmcp::handler::server::wrapper::Parameters<#params_ty>,
            ) -> ::std::result::Result<
                ::rmcp::handler::server::wrapper::Json<#result_ty>,
                ::rmcp::ErrorData
            > {
                self.#field_ident.#method_ident(params).await
            }
        };

        impl_block.items.push(syn::ImplItem::Fn(method));
    }

    TokenStream::from(quote! { #impl_block })
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
