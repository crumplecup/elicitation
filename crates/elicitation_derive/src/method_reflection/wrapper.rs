//! Wrapper method generation for delegating to inner type methods.
//!
//! This module generates wrapper methods that:
//! 1. Accept parameter structs
//! 2. Convert arguments to inner type format
//! 3. Delegate to the wrapped method via Deref
//! 4. Return results in MCP-compatible format

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, ReturnType, Type};

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
    let params_struct = Ident::new(
        &format!("{}Params", to_pascal_case(method_name)),
        proc_macro2::Span::call_site(),
    );

    // Generate parameter conversions
    let param_conversions: Vec<TokenStream> = params
        .iter()
        .filter_map(|param| {
            if let FnArg::Typed(pat_type) = param {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    let name = &pat_ident.ident;
                    // TODO: Smart conversion based on type
                    // For now, simple passthrough
                    Some(generate_param_conversion(name, &pat_type.ty))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

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
            params: ::rmcp::handler::server::wrapper::Parameters<#params_struct>,
        ) -> ::std::result::Result<
            ::rmcp::handler::server::wrapper::Json<ResponseType>,
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
    _return_type: &ReturnType,
    method_call: TokenStream,
) -> TokenStream {
    // TODO: Parse return type to determine proper wrapping
    // For now, assume Result<T, E> and wrap accordingly
    quote! {
        #method_call
            .map(::rmcp::handler::server::wrapper::Json)
            .map_err(|e| ::rmcp::ErrorData::internal_error(e.to_string(), None))
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
}
