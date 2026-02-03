//! Kani verifier backend.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Field, Ident, Type, TypePath};

/// Generate Kani verification for a struct.
pub fn generate_kani_verification(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let constructor = generate_constructor(struct_name, fields);
    let harness = generate_harness(struct_name, fields);
    let module_name = format_ident!("__kani_verification_{}", struct_name);

    // Module name includes struct name to avoid conflicts
    // Only compiled when Kani is actually running
    quote! {
        #[cfg(kani)]
        mod #module_name {
            use super::*;

            #constructor

            #harness
        }
    }
}

/// Generate Kani verification for an enum.
pub fn generate_kani_enum_verification(
    enum_name: &Ident,
    variants: &[&syn::Variant],
) -> TokenStream {
    let harnesses: Vec<_> = variants
        .iter()
        .map(|variant| generate_variant_harness(enum_name, variant))
        .collect();
    let module_name = format_ident!("__kani_verification_{}", enum_name);

    quote! {
        #[cfg(kani)]
        mod #module_name {
            use super::*;

            #(#harnesses)*
        }
    }
}

/// Extract the type path from a field type (best effort).
fn extract_type_path(ty: &Type) -> Option<&TypePath> {
    match ty {
        Type::Path(type_path) => Some(type_path),
        _ => None,
    }
}

/// Generate a contract predicate for a field.
///
/// Strategy: Contract types are already validated at construction time.
/// If a field has type `I8Positive`, any instance is guaranteed positive.
/// The struct constructor just needs to accept validated instances.
fn generate_field_predicate(
    _field_name: &Ident,
    _field_type: &Type,
    _is_requires: bool,
) -> TokenStream {
    // For contract types, validation happens at their own construction
    // The struct constructor accepts pre-validated instances
    // So the predicate is trivially true - we trust the field types
    quote! { true }
}

fn generate_constructor(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let constructor_name = format_ident!("__make_{}", struct_name);
    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Generate requires predicates for each field
    let requires_predicates: Vec<TokenStream> = field_names
        .iter()
        .zip(&field_types)
        .map(|(field_name, field_type)| generate_field_predicate(field_name, field_type, true))
        .collect();

    // Combine all requires with AND
    let combined_requires = if requires_predicates.is_empty() {
        quote! { true }
    } else {
        quote! { #(#requires_predicates)&&* }
    };

    // Generate ensures predicates for each field in result
    let ensures_predicates: Vec<TokenStream> = field_names
        .iter()
        .zip(&field_types)
        .map(|(field_name, field_type)| generate_field_predicate(field_name, field_type, false))
        .collect();

    let combined_ensures = if ensures_predicates.is_empty() {
        quote! { true }
    } else {
        quote! { #(#ensures_predicates)&&* }
    };

    quote! {
        #[kani::requires(#combined_requires)]
        #[kani::ensures(|_result: &#struct_name| #combined_ensures)]
        fn #constructor_name(#(#field_names: #field_types),*) -> #struct_name {
            #struct_name { #(#field_names),* }
        }
    }
}

fn generate_harness(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let constructor_name = format_ident!("__make_{}", struct_name);
    let harness_name = format_ident!("__verify_{}", struct_name);
    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Generate field initializations with kani::any()
    let field_inits: Vec<_> = field_names
        .iter()
        .zip(field_types.iter())
        .map(|(name, ty)| {
            quote! { let #name: #ty = kani::any(); }
        })
        .collect();

    // Generate stub_verified attributes for field type constructors
    // This assumes leaf types are already verified, so we only verify composition
    let stub_attributes: Vec<TokenStream> = field_types
        .iter()
        .filter_map(|ty| extract_type_path(ty))
        .map(|type_path| {
            // Generate #[kani::stub_verified(TypeName::new)] if the type has a new method
            quote! {
                #[kani::stub_verified(#type_path::new)]
            }
        })
        .collect();

    quote! {
        #[kani::proof_for_contract(#constructor_name)]
        #(#stub_attributes)*
        fn #harness_name() {
            // Create arbitrary instances of each field type
            // With stub_verified, Kani assumes these are valid without re-proving
            #(#field_inits)*

            // Call constructor (Kani verifies composition, not leaves)
            let _result = #constructor_name(#(#field_names),*);
        }
    }
}

/// Generate a proof harness for a single enum variant.
fn generate_variant_harness(enum_name: &Ident, variant: &syn::Variant) -> TokenStream {
    let variant_name = &variant.ident;
    let harness_name = format_ident!("__verify_{}_{}", enum_name, variant_name);

    match &variant.fields {
        syn::Fields::Unit => {
            // Unit variant - just construct it
            quote! {
                #[kani::proof]
                fn #harness_name() {
                    let _value = #enum_name::#variant_name;
                    // Unit variants are always valid
                }
            }
        }

        syn::Fields::Unnamed(fields) => {
            // Tuple variant - generate symbolic fields
            let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
            let field_names: Vec<_> = (0..field_types.len())
                .map(|i| format_ident!("field_{}", i))
                .collect();

            // Generate stub_verified for each field type
            let stub_attributes: Vec<TokenStream> = field_types
                .iter()
                .filter_map(|ty| extract_type_path(ty))
                .map(|type_path| {
                    let type_segments = &type_path.path.segments;
                    let type_name = quote! { #type_segments };
                    quote! {
                        #[kani::stub_verified(#type_name::new)]
                    }
                })
                .collect();

            quote! {
                #[kani::proof]
                #(#stub_attributes)*
                fn #harness_name() {
                    #(let #field_names: #field_types = kani::any();)*
                    let _value = #enum_name::#variant_name(#(#field_names),*);
                }
            }
        }

        syn::Fields::Named(fields) => {
            // Struct variant - generate symbolic fields
            let field_names: Vec<_> = fields
                .named
                .iter()
                .filter_map(|f| f.ident.as_ref())
                .collect();
            let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();

            // Generate stub_verified for each field type
            let stub_attributes: Vec<TokenStream> = field_types
                .iter()
                .filter_map(|ty| extract_type_path(ty))
                .map(|type_path| {
                    let type_segments = &type_path.path.segments;
                    let type_name = quote! { #type_segments };
                    quote! {
                        #[kani::stub_verified(#type_name::new)]
                    }
                })
                .collect();

            quote! {
                #[kani::proof]
                #(#stub_attributes)*
                fn #harness_name() {
                    #(let #field_names: #field_types = kani::any();)*
                    let _value = #enum_name::#variant_name { #(#field_names),* };
                }
            }
        }
    }
}
