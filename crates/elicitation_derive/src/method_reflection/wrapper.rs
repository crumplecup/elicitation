//! Wrapper method generation for delegating to inner type methods.
//!
//! This module generates wrapper methods that:
//! 1. Accept parameter structs
//! 2. Convert arguments to inner type format
//! 3. Delegate to the wrapped method via Deref
//! 4. Return results in MCP-compatible format

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, GenericArgument, Ident, PathArguments, ReturnType, Type, TypePath};

/// Generates a wrapper method that delegates to the inner type.
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
pub fn generate_wrapper_method(
    method_name: &str,
    params: &[FnArg],
    return_type: &ReturnType,
    is_async: bool,
) -> TokenStream {
    let method_ident = Ident::new(method_name, proc_macro2::Span::call_site());

    // Extract the success type from the return type
    let response_type = extract_return_type(return_type);

    // Generate parameter handling
    let (params_arg, param_conversions) = if params.is_empty() {
        // No parameters - no params argument needed
        (quote! {}, vec![])
    } else {
        let params_struct = Ident::new(
            &format!("{}Params", to_pascal_case(method_name)),
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

        (
            quote! { params: ::rmcp::handler::server::wrapper::Parameters<#params_struct>, },
            conversions,
        )
    };

    // Generate method call
    let method_call = if is_async {
        quote! {
            self.#method_ident(#(#param_conversions),*).await
        }
    } else {
        quote! {
            self.#method_ident(#(#param_conversions),*)
        }
    };

    // Handle return type wrapping
    let result_handling = generate_result_handling(return_type, method_call);

    let async_keyword = if is_async {
        quote! { async }
    } else {
        quote! {}
    };

    let tool_description = format!("{} operation", method_name.replace('_', " "));

    quote! {
        #[doc = concat!("`", #method_name, "` wrapper method.")]
        #[::rmcp::tool(description = #tool_description)]
        pub #async_keyword fn #method_ident(
            &self,
            #params_arg
        ) -> ::std::result::Result<
            ::rmcp::handler::server::wrapper::Json<#response_type>,
            ::rmcp::ErrorData
        > {
            #result_handling
        }
    }
}

/// Generates conversion code for a parameter.
///
/// Examples:
/// - `url: String` → `params.url.as_str()` (for &str parameter)
/// - `data: Vec<u8>` → `&params.data` (for &[u8] parameter)
/// - `value: i32` → `params.value` (for owned parameter)
fn generate_param_conversion(name: &Ident, ty: &Type) -> TokenStream {
    match ty {
        Type::Reference(type_ref) => {
            // &str: use .as_str()
            if let Type::Path(type_path) = &*type_ref.elem {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "str" {
                        return quote! { params.#name.as_str() };
                    }
                }
            }

            // &[T]: use &params.name
            if matches!(&*type_ref.elem, Type::Slice(_)) {
                return quote! { &params.#name };
            }

            // &T: use &params.name
            quote! { &params.#name }
        }
        // Owned type: direct access
        _ => quote! { params.#name },
    }
}

/// Generates result handling code.
///
/// Wraps the result in MCP-compatible format:
/// - Ok values wrapped in Json<T>
/// - Errors converted to ErrorData
fn generate_result_handling(
    return_type: &ReturnType,
    method_call: TokenStream,
) -> TokenStream {
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
    if let Type::Path(TypePath { path, .. }) = ty {
        if let Some(segment) = path.segments.last() {
            return segment.ident == "Result";
        }
    }
    false
}

/// Extracts the success type from Result<T, E> or returns the type itself
fn extract_return_type(return_type: &ReturnType) -> Type {
    match return_type {
        ReturnType::Default => syn::parse_quote! { () },
        ReturnType::Type(_, ty) => {
            if let Type::Path(TypePath { path, .. }) = &**ty {
                if let Some(segment) = path.segments.last() {
                    if segment.ident == "Result" {
                        // Extract T from Result<T, E>
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(success_type)) = args.args.first() {
                                return success_type.clone();
                            }
                        }
                    }
                }
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
        assert_eq!(
            quote! { #conversion }.to_string(),
            quote! { params.url.as_str() }.to_string()
        );
    }

    #[test]
    fn test_param_conversion_owned() {
        let name = Ident::new("count", proc_macro2::Span::call_site());
        let ty: Type = syn::parse_quote! { i32 };
        let conversion = generate_param_conversion(&name, &ty);
        assert_eq!(
            quote! { #conversion }.to_string(),
            quote! { params.count }.to_string()
        );
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

    #[test]
    fn test_generate_wrapper_method_with_params() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { url: &str }];
        let return_ty: ReturnType = syn::parse_quote! { -> Result<Response, Error> };

        let wrapper = generate_wrapper_method("get", &params, &return_ty, false);
        let wrapper_str = wrapper.to_string();

        // Verify key components are present
        assert!(wrapper_str.contains("GetParams"));
        assert!(wrapper_str.contains("pub fn get"));
        assert!(wrapper_str.contains("tool"));  // #[tool] attribute
        assert!(wrapper_str.contains("rmcp"));
        assert!(wrapper_str.contains("as_str")); // params.url.as_str()
    }

    #[test]
    fn test_generate_wrapper_method_no_params() {
        let params: Vec<FnArg> = vec![];
        let return_ty: ReturnType = syn::parse_quote! { -> usize };

        let wrapper = generate_wrapper_method("len", &params, &return_ty, false);
        let wrapper_str = wrapper.to_string();

        // Should not have Parameters argument
        assert!(!wrapper_str.contains("Parameters"));
        assert!(wrapper_str.contains("pub fn len"));
        assert!(wrapper_str.contains("tool"));  // #[tool] attribute
        assert!(wrapper_str.contains("rmcp"));
    }

    #[test]
    fn test_generate_wrapper_method_async() {
        let params: Vec<FnArg> = vec![syn::parse_quote! { url: &str }];
        let return_ty: ReturnType = syn::parse_quote! { -> Result<Response, Error> };

        let wrapper = generate_wrapper_method("fetch", &params, &return_ty, true);
        let wrapper_str = wrapper.to_string();

        // Verify async keyword is present
        assert!(wrapper_str.contains("pub async fn fetch"));
        assert!(wrapper_str.contains(". await"));
    }
}
