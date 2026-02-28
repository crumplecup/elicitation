//! Parameter struct generation for method arguments.
//!
//! This module generates typed parameter structs for method arguments,
//! enabling MCP tool integration.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Ident, Type};

/// Generates a parameter struct for a method's arguments.
///
/// Given method signature: `fn get(url: &str, timeout: u64) -> Response`
/// Generates:
/// ```ignore
/// #[derive(Debug, Clone, Elicit, JsonSchema)]
/// pub struct GetParams {
///     pub url: String,  // &str converted to String
///     pub timeout: u64,
/// }
/// ```
pub fn generate_param_struct(
    method_name: &str,
    params: &[FnArg],
) -> TokenStream {
    let struct_name = format!("{}Params", to_pascal_case(method_name));
    let struct_ident = Ident::new(&struct_name, proc_macro2::Span::call_site());

    // Extract parameter names and types
    let fields: Vec<TokenStream> = params
        .iter()
        .filter_map(|param| {
            if let FnArg::Typed(pat_type) = param {
                // Extract parameter name
                let name = if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    &pat_ident.ident
                } else {
                    return None;
                };

                // Convert type (e.g., &str → String)
                let ty = convert_param_type(&pat_type.ty);

                Some(quote! {
                    pub #name: #ty
                })
            } else {
                None
            }
        })
        .collect();

    quote! {
        #[derive(
            ::std::fmt::Debug,
            ::std::clone::Clone,
            ::elicitation::Elicit,
            ::schemars::JsonSchema,
        )]
        pub struct #struct_ident {
            #(#fields),*
        }
    }
}

/// Converts a parameter type to its owned equivalent.
///
/// Conversions:
/// - `&str` → `String`
/// - `&[T]` → `Vec<T>`
/// - `&T` → `T` (with clone warning if large)
/// - `T` → `T` (passthrough)
fn convert_param_type(ty: &Type) -> Type {
    match ty {
        Type::Reference(type_ref) => {
            // Handle &str → String
            if let Type::Path(type_path) = &*type_ref.elem {
                if let Some(segment) = type_path.path.segments.last() {
                    if segment.ident == "str" {
                        return syn::parse_quote! { String };
                    }
                }
            }

            // Handle &[T] → Vec<T>
            if let Type::Slice(type_slice) = &*type_ref.elem {
                let elem = &type_slice.elem;
                return syn::parse_quote! { Vec<#elem> };
            }

            // Handle &T → T (requires Clone)
            // TODO: Add warning for large types
            (*type_ref.elem).clone()
        }
        // Passthrough for owned types
        _ => ty.clone(),
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
        assert_eq!(to_pascal_case("fetch"), "Fetch");
        assert_eq!(to_pascal_case("parse_json_data"), "ParseJsonData");
    }

    #[test]
    fn test_convert_str_reference() {
        let ty: Type = syn::parse_quote! { &str };
        let converted = convert_param_type(&ty);
        let expected: Type = syn::parse_quote! { String };
        assert_eq!(quote! { #converted }.to_string(), quote! { #expected }.to_string());
    }

    #[test]
    fn test_convert_slice_reference() {
        let ty: Type = syn::parse_quote! { &[u8] };
        let converted = convert_param_type(&ty);
        let expected: Type = syn::parse_quote! { Vec<u8> };
        assert_eq!(quote! { #converted }.to_string(), quote! { #expected }.to_string());
    }
}
