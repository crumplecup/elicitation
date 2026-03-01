//! Wrapper method generation for delegating to inner type methods.
//!
//! This module generates wrapper methods that:
//! 1. Accept parameter structs
//! 2. Convert arguments to inner type format
//! 3. Delegate to the wrapped method via Deref
//! 4. Return results in MCP-compatible format

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    FnArg, GenericArgument, GenericParam, Generics, Ident, PathArguments, ReturnType, Type,
    TypePath,
};

/// Generates a wrapper method that delegates to the inner type.
///
/// # Non-Generic Method
///
/// Given method: `fn get(url: &str) -> Result<Response, Error>`
/// Generates:
/// ```ignore
/// #[tool(description = "Get resource from URL")]
/// pub async fn get(
///     &self,
///     params: rmcp::Parameters<GetParams>,
/// ) -> Result<rmcp::Json<Response>, rmcp::ErrorData> {
///     let url = params.url.as_str();  // String → &str conversion
///     self.0.get(url)
///         .await
///         .map(rmcp::Json)
///         .map_err(|e| rmcp::ErrorData::internal_error(e.to_string(), None))
/// }
/// ```
///
/// # Generic Method
///
/// Given method: `fn contains<T>(item: &T) -> bool where T: Elicitation + JsonSchema`
/// Generates:
/// ```ignore
/// #[tool(description = "contains operation")]
/// pub fn contains_tool<T>(
///     &self,
///     params: Parameters<ContainsParams<T>>,
/// ) -> Result<Json<bool>, ErrorData>
/// where
///     T: Elicitation + JsonSchema,
/// {
///     let item = &params.0.item;
///     self.contains::<T>(item)
///         .map(Json)
///         .map_err(|e| ErrorData::internal_error(e.to_string(), None))
/// }
/// ```
pub fn generate_wrapper_method(
    wrapper_method_name: &str,
    original_method_name: &str,
    params: &[FnArg],
    return_type: &ReturnType,
    is_async: bool,
    generics: &Generics,
    is_consuming: bool,
) -> TokenStream {
    let method_ident = Ident::new(wrapper_method_name, proc_macro2::Span::call_site());
    let original_method_ident = Ident::new(original_method_name, proc_macro2::Span::call_site());

    // Extract the success type from the return type
    let response_type = extract_return_type(return_type);

    // Extract generic components
    let generic_params = &generics.params;
    let where_clause = &generics.where_clause;
    let is_generic = !generic_params.is_empty();

    // Extract just type parameter names for turbofish (T, U, V...)
    let turbofish_params: Vec<_> = generic_params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(&type_param.ident)
            } else {
                None
            }
        })
        .collect();

    // Generate parameter handling
    let (params_arg, param_conversions) = if params.is_empty() {
        // No parameters - no params argument needed
        (quote! {}, vec![])
    } else {
        // Use original method name for parameter struct (not wrapper name)
        let params_struct = Ident::new(
            &format!("{}Params", to_pascal_case(original_method_name)),
            proc_macro2::Span::call_site(),
        );

        let conversions: Vec<TokenStream> = params
            .iter()
            .filter_map(|param| {
                if let FnArg::Typed(pat_type) = param {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        let name = &pat_ident.ident;
                        Some(generate_param_conversion(name, &pat_type.ty))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Add generic parameters to params struct if method is generic
        let params_type = if is_generic {
            quote! { params: ::rmcp::handler::server::wrapper::Parameters<#params_struct<#(#turbofish_params),*>>, }
        } else {
            quote! { params: ::rmcp::handler::server::wrapper::Parameters<#params_struct>, }
        };

        (params_type, conversions)
    };

    // Check if method returns Self (needs wrapping for consuming methods)
    let returns_self = matches!(return_type, ReturnType::Type(_, ty) if is_self_type(ty));

    // Generate method call - different strategy for consuming vs borrowing methods
    let method_call = if is_consuming {
        // Consuming methods use hybrid Arc unwrap-or-clone strategy
        let inner_call = if is_generic && is_async {
            quote! {
                inner.#original_method_ident::<#(#turbofish_params),*>(#(#param_conversions),*).await
            }
        } else if is_generic {
            quote! {
                inner.#original_method_ident::<#(#turbofish_params),*>(#(#param_conversions),*)
            }
        } else if is_async {
            quote! {
                inner.#original_method_ident(#(#param_conversions),*).await
            }
        } else {
            quote! {
                inner.#original_method_ident(#(#param_conversions),*)
            }
        };

        // If method returns Self, wrap the result back in the wrapper type
        let wrapped_call = if returns_self {
            quote! { Self::from(#inner_call) }
        } else {
            inner_call
        };

        quote! {
            {
                // Consuming methods require exclusive ownership of the wrapped value.
                // This will panic if the Arc has multiple references (refcount > 1).
                // For exclusive ownership, this is the correct behavior regardless of
                // whether the inner type implements Clone.
                let inner = ::std::sync::Arc::try_unwrap(self.0)
                    .expect("Consuming method requires exclusive ownership (Arc refcount must be 1)");
                #wrapped_call
            }
        }
    } else {
        // Borrowing methods call through Deref
        if is_generic && is_async {
            quote! {
                self.#original_method_ident::<#(#turbofish_params),*>(#(#param_conversions),*).await
            }
        } else if is_generic {
            quote! {
                self.#original_method_ident::<#(#turbofish_params),*>(#(#param_conversions),*)
            }
        } else if is_async {
            quote! {
                self.#original_method_ident(#(#param_conversions),*).await
            }
        } else {
            quote! {
                self.#original_method_ident(#(#param_conversions),*)
            }
        }
    };

    // Handle return type wrapping
    let result_handling = generate_result_handling(return_type, method_call);

    let async_keyword = if is_async {
        quote! { async }
    } else {
        quote! {}
    };

    let tool_description = format!("{} operation", original_method_name.replace('_', " "));

    // Determine receiver type: self or &self
    let receiver = if is_consuming {
        quote! { self }
    } else {
        quote! { &self }
    };

    // Generate method signature with or without generics
    if is_generic {
        // NOTE: We don't use #[tool] attribute for generic methods because:
        // 1. The #[tool] macro generates helper functions that reference generic types
        //    but those helpers are not themselves generic, causing compilation errors
        // 2. MCP tool registration requires concrete JSON schemas at registration time
        // 3. Generic methods need to be monomorphized to concrete types before use
        //
        // To use generic methods as MCP tools, you'll need to either:
        // - Create non-generic wrapper methods that call the generic method with concrete types
        // - Manually register the monomorphized versions as separate tools
        quote! {
            #[doc = concat!("`", #original_method_name, "` MCP tool wrapper method (generic - requires manual registration).")]
            #[doc = ""]
            #[doc = "**Note:** This is a generic method. It cannot be automatically registered as an MCP tool."]
            #[doc = "You must create non-generic wrappers or manually register monomorphized versions."]
            pub #async_keyword fn #method_ident<#generic_params>(
                #receiver,
                #params_arg
            ) -> ::std::result::Result<
                ::rmcp::handler::server::wrapper::Json<#response_type>,
                ::rmcp::ErrorData
            >
            #where_clause
            {
                #result_handling
            }
        }
    } else {
        quote! {
            #[doc = concat!("`", #original_method_name, "` MCP tool wrapper method.")]
            #[::rmcp::tool(description = #tool_description)]
            pub #async_keyword fn #method_ident(
                #receiver,
                #params_arg
            ) -> ::std::result::Result<
                ::rmcp::handler::server::wrapper::Json<#response_type>,
                ::rmcp::ErrorData
            > {
                #result_handling
            }
        }
    }
}

/// Generates conversion code for a parameter.
///
/// Examples:
/// - `url: String` → `params.0.url.as_str()` (for &str parameter)
/// - `data: Vec<u8>` → `&params.0.data` (for &[u8] parameter)
/// - `value: i32` → `params.0.value` (for owned parameter)
///
/// Note: Parameters<T> is a tuple struct, so we access the inner value via `.0`
fn generate_param_conversion(name: &Ident, ty: &Type) -> TokenStream {
    match ty {
        Type::Reference(type_ref) => {
            // &str: use .as_str()
            if let Type::Path(type_path) = &*type_ref.elem
                && let Some(segment) = type_path.path.segments.last()
                && segment.ident == "str"
            {
                return quote! { params.0.#name.as_str() };
            }

            // &[T]: use &params.0.name
            if matches!(&*type_ref.elem, Type::Slice(_)) {
                return quote! { &params.0.#name };
            }

            // &T: use &params.0.name
            quote! { &params.0.#name }
        }
        // Owned type: direct access
        _ => quote! { params.0.#name },
    }
}

/// Generates result handling code.
///
/// Wraps the result in MCP-compatible format:
/// - Ok values wrapped in Json<T>
/// - Errors converted to ErrorData
fn generate_result_handling(return_type: &ReturnType, method_call: TokenStream) -> TokenStream {
    match return_type {
        ReturnType::Default => {
            // No return value - wrap in Ok(())
            quote! {
                #method_call;
                Ok(::rmcp::handler::server::wrapper::Json(()))
            }
        }
        ReturnType::Type(_, ty) => {
            if is_result_type(ty) {
                // Result<T, E> - map both sides
                quote! {
                    #method_call
                        .map(::rmcp::handler::server::wrapper::Json)
                        .map_err(|e| ::rmcp::ErrorData::internal_error(e.to_string(), None))
                }
            } else {
                // Plain type T - wrap in Ok(Json(T))
                quote! {
                    let result = #method_call;
                    Ok(::rmcp::handler::server::wrapper::Json(result))
                }
            }
        }
    }
}

/// Checks if a type is Result<T, E>
fn is_result_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty
        && let Some(segment) = path.segments.last()
    {
        return segment.ident == "Result";
    }
    false
}

/// Checks if a type is `Self`
fn is_self_type(ty: &Type) -> bool {
    if let Type::Path(TypePath { path, .. }) = ty
        && let Some(segment) = path.segments.last()
    {
        return segment.ident == "Self";
    }
    false
}

/// Extracts the success type from Result<T, E> or returns the type itself
fn extract_return_type(return_type: &ReturnType) -> Type {
    match return_type {
        ReturnType::Default => syn::parse_quote! { () },
        ReturnType::Type(_, ty) => {
            if let Type::Path(TypePath { path, .. }) = &**ty
                && let Some(segment) = path.segments.last()
                && segment.ident == "Result"
                && let PathArguments::AngleBracketed(args) = &segment.arguments
                && let Some(GenericArgument::Type(success_type)) = args.args.first()
            {
                return success_type.clone();
            }
            // Not a Result, return the type itself
            (**ty).clone()
        }
    }
}

/// Converts snake_case to PascalCase
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("get_user"), "GetUser");
        assert_eq!(to_pascal_case("fetch_data"), "FetchData");
    }

    #[test]
    fn test_param_conversion_str() {
        let name = Ident::new("url", proc_macro2::Span::call_site());
        let ty: Type = syn::parse_quote! { &str };
        let conversion = generate_param_conversion(&name, &ty);
        let conversion_str = quote! { #conversion }.to_string();
        // Check that it accesses params.0.url and calls as_str()
        assert!(conversion_str.contains("params"));
        assert!(conversion_str.contains("url"));
        assert!(conversion_str.contains("as_str"));
    }

    #[test]
    fn test_param_conversion_owned() {
        let name = Ident::new("count", proc_macro2::Span::call_site());
        let ty: Type = syn::parse_quote! { i32 };
        let conversion = generate_param_conversion(&name, &ty);
        let conversion_str = quote! { #conversion }.to_string();
        // Check that it accesses params.0.count
        assert!(conversion_str.contains("params"));
        assert!(conversion_str.contains("count"));
    }

    #[test]
    fn test_is_result_type() {
        let result_ty: Type = syn::parse_quote! { Result<String, Error> };
        assert!(is_result_type(&result_ty));

        let plain_ty: Type = syn::parse_quote! { String };
        assert!(!is_result_type(&plain_ty));

        let option_ty: Type = syn::parse_quote! { Option<String> };
        assert!(!is_result_type(&option_ty));
    }

    #[test]
    fn test_extract_return_type_from_result() {
        let return_ty: ReturnType = syn::parse_quote! { -> Result<String, Error> };
        let extracted = extract_return_type(&return_ty);

        let expected: Type = syn::parse_quote! { String };
        assert_eq!(
            quote! { #extracted }.to_string(),
            quote! { #expected }.to_string()
        );
    }

    #[test]
    fn test_extract_return_type_plain() {
        let return_ty: ReturnType = syn::parse_quote! { -> bool };
        let extracted = extract_return_type(&return_ty);

        let expected: Type = syn::parse_quote! { bool };
        assert_eq!(
            quote! { #extracted }.to_string(),
            quote! { #expected }.to_string()
        );
    }

    #[test]
    fn test_extract_return_type_unit() {
        let return_ty: ReturnType = ReturnType::Default;
        let extracted = extract_return_type(&return_ty);

        let expected: Type = syn::parse_quote! { () };
        assert_eq!(
            quote! { #extracted }.to_string(),
            quote! { #expected }.to_string()
        );
    }

    /// Helper to extract generics from a function signature
    fn parse_sig_generics(sig: &str) -> Generics {
        let sig: syn::Signature = syn::parse_str(sig).unwrap();
        sig.generics
    }

    #[test]
    fn test_generate_wrapper_method_with_params() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { url: &str }];
        let return_ty: ReturnType = syn::parse_quote! { -> Result<Response, Error> };
        let generics = parse_sig_generics("fn get()");

        let wrapper =
            generate_wrapper_method("get_tool", "get", &params, &return_ty, false, &generics, false);
        let wrapper_str = wrapper.to_string();

        // Verify key components are present
        assert!(wrapper_str.contains("GetParams")); // Based on original name
        assert!(wrapper_str.contains("pub fn get_tool")); // Wrapper name
        assert!(wrapper_str.contains("tool")); // #[tool] attribute
        assert!(wrapper_str.contains("rmcp"));
        assert!(wrapper_str.contains("as_str")); // params.url.as_str()
        assert!(wrapper_str.contains("self . get")); // Calls original method
    }

    #[test]
    fn test_generate_wrapper_method_no_params() {
        let params: Vec<FnArg> = vec![];
        let return_ty: ReturnType = syn::parse_quote! { -> usize };
        let generics = parse_sig_generics("fn len()");

        let wrapper =
            generate_wrapper_method("len_tool", "len", &params, &return_ty, false, &generics, false);
        let wrapper_str = wrapper.to_string();

        // Should not have Parameters argument
        assert!(!wrapper_str.contains("Parameters"));
        assert!(wrapper_str.contains("pub fn len_tool")); // Wrapper name
        assert!(wrapper_str.contains("tool")); // #[tool] attribute
        assert!(wrapper_str.contains("rmcp"));
        assert!(wrapper_str.contains("self . len")); // Calls original method
    }

    #[test]
    fn test_generate_wrapper_method_async() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { url: &str }];
        let return_ty: ReturnType = syn::parse_quote! { -> Result<Response, Error> };
        let generics = parse_sig_generics("fn fetch()");

        let wrapper =
            generate_wrapper_method("fetch_tool", "fetch", &params, &return_ty, true, &generics, false);
        let wrapper_str = wrapper.to_string();

        // Verify async keyword is present
        assert!(wrapper_str.contains("pub async fn fetch_tool")); // Wrapper name
        assert!(wrapper_str.contains(". await"));
        assert!(wrapper_str.contains("self . fetch")); // Calls original method
    }

    #[test]
    fn test_generate_wrapper_method_generic() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { item: &T }];
        let return_ty: ReturnType = syn::parse_quote! { -> bool };
        let generics = parse_sig_generics("fn contains<T>() where T: Elicitation + JsonSchema");

        let wrapper = generate_wrapper_method(
            "contains_tool",
            "contains",
            &params,
            &return_ty,
            false,
            &generics,
            false,
        );
        let wrapper_str = wrapper.to_string();

        // Verify generic method components
        assert!(wrapper_str.contains("ContainsParams"));
        assert!(wrapper_str.contains("< T >")); // Spaces in token stream
        assert!(wrapper_str.contains("where"));
        assert!(wrapper_str.contains("Elicitation"));
        assert!(wrapper_str.contains("JsonSchema"));
        // Verify turbofish in method call: "contains :: < T >"
        assert!(wrapper_str.contains("contains :: <"));
        // Verify parameter type includes generic
        assert!(wrapper_str.contains("ContainsParams < T >"));
    }

    #[test]
    fn test_generate_wrapper_method_multiple_generics() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { key: K }, syn::parse_quote! { value: V }];
        let return_ty: ReturnType = syn::parse_quote! { -> Option<V> };
        let generics = parse_sig_generics(
            "fn insert<K, V>() where K: Elicitation + JsonSchema + Hash, V: Elicitation + JsonSchema",
        );

        let wrapper = generate_wrapper_method(
            "insert_tool",
            "insert",
            &params,
            &return_ty,
            false,
            &generics,
            false,
        );
        let wrapper_str = wrapper.to_string();

        // Verify multiple generic parameters
        assert!(wrapper_str.contains("InsertParams"));
        assert!(wrapper_str.contains("K"));
        assert!(wrapper_str.contains("V"));
        assert!(wrapper_str.contains("Hash"));
        // Verify turbofish with multiple params: "insert :: < K , V >"
        assert!(wrapper_str.contains("insert :: <"));
        // Verify parameter type includes generics
        assert!(wrapper_str.contains("InsertParams < K , V >"));
    }

    #[test]
    fn test_generate_wrapper_method_generic_async() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { url: &str }];
        let return_ty: ReturnType = syn::parse_quote! { -> Result<T, Error> };
        let generics = parse_sig_generics("fn fetch<T>() where T: Elicitation + JsonSchema");

        let wrapper =
            generate_wrapper_method("fetch_tool", "fetch", &params, &return_ty, true, &generics, false);
        let wrapper_str = wrapper.to_string();

        // Verify async generic method
        assert!(wrapper_str.contains("pub async fn fetch_tool"));
        assert!(wrapper_str.contains("< T >"));
        assert!(wrapper_str.contains("where"));
        assert!(wrapper_str.contains(". await"));
        // Verify turbofish: "fetch :: < T >"
        assert!(wrapper_str.contains("fetch :: <"));
    }
}
