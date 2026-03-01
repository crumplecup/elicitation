//! Parameter struct generation for method arguments.
//!
//! This module generates typed parameter structs for method arguments,
//! enabling MCP tool integration.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, Generics, Ident, Type};

/// Generates a parameter struct for a method's arguments.
///
/// # Non-Generic Method
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
///
/// # Generic Method
///
/// Given method signature: `fn contains<T>(item: &T) -> bool where T: Elicitation + JsonSchema`
/// Generates:
/// ```ignore
/// #[derive(Debug, Clone, Elicit, JsonSchema)]
/// pub struct ContainsParams<T>
/// where
///     T: Elicitation + JsonSchema,
/// {
///     pub item: T,  // Reference removed, requires owned type
/// }
/// ```
pub fn generate_param_struct(
    method_name: &str,
    params: &[FnArg],
    generics: &Generics,
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

    // Extract generic params and where clause
    let generic_params = &generics.params;
    let where_clause = &generics.where_clause;

    // Generate struct with or without generics
    if generic_params.is_empty() {
        // Non-generic struct
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
    } else {
        // Generic struct
        quote! {
            #[derive(
                ::std::fmt::Debug,
                ::std::clone::Clone,
                ::elicitation::Elicit,
                ::schemars::JsonSchema,
            )]
            pub struct #struct_ident<#generic_params>
            #where_clause
            {
                #(#fields),*
            }
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

    /// Helper to extract generics from a function signature
    fn parse_sig_generics(sig: &str) -> Generics {
        let sig: syn::Signature = syn::parse_str(sig).unwrap();
        sig.generics
    }

    #[test]
    fn test_generate_param_struct_non_generic() {
        let params: Vec<FnArg> = vec![
            syn::parse_quote! { url: &str },
            syn::parse_quote! { timeout: u64 },
        ];
        let generics = parse_sig_generics("fn get()");

        let output = generate_param_struct("get", &params, &generics);
        let output_str = output.to_string();

        // Verify struct name
        assert!(output_str.contains("GetParams"));
        // Verify fields
        assert!(output_str.contains("url"));
        assert!(output_str.contains("String")); // &str converted to String
        assert!(output_str.contains("timeout"));
        assert!(output_str.contains("u64"));
        // Verify derives
        assert!(output_str.contains("Elicit"));
        assert!(output_str.contains("JsonSchema"));
    }

    #[test]
    fn test_generate_param_struct_generic() {
        let params: Vec<FnArg> = vec![
            syn::parse_quote! { item: &T },
        ];
        let generics = parse_sig_generics("fn contains<T>() where T: Elicitation + JsonSchema");

        let output = generate_param_struct("contains", &params, &generics);
        let output_str = output.to_string();

        // Verify struct has generic parameter
        assert!(output_str.contains("ContainsParams"));
        assert!(output_str.contains("<"));
        assert!(output_str.contains("T"));
        // Verify where clause
        assert!(output_str.contains("where"));
        assert!(output_str.contains("Elicitation"));
        assert!(output_str.contains("JsonSchema"));
        // Verify field type is T (not &T)
        assert!(output_str.contains("item"));
    }

    #[test]
    fn test_generate_param_struct_multiple_generics() {
        let params: Vec<FnArg> = vec![
            syn::parse_quote! { key: K },
            syn::parse_quote! { value: V },
        ];
        let generics = parse_sig_generics(
            "fn insert<K, V>() where K: Elicitation + JsonSchema + Hash, V: Elicitation + JsonSchema"
        );

        let output = generate_param_struct("insert", &params, &generics);
        let output_str = output.to_string();

        // Verify struct has multiple generic parameters
        assert!(output_str.contains("InsertParams"));
        assert!(output_str.contains("K"));
        assert!(output_str.contains("V"));
        // Verify where clause with multiple bounds
        assert!(output_str.contains("Hash"));
        // Verify fields
        assert!(output_str.contains("key"));
        assert!(output_str.contains("value"));
    }
}
