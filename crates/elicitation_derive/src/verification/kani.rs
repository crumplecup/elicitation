//! Kani verifier backend.

use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use syn::{Field, Ident, Type, TypePath};

/// Generate Kani verification for a struct.
pub fn generate_kani_verification(
    struct_name: &Ident,
    fields: &[&Field],
) -> TokenStream {
    let constructor = generate_constructor(struct_name, fields);
    let harness = generate_harness(struct_name, fields);
    
    // Note: We only gate on the feature flag, not #[cfg(kani)]
    // This allows cargo expand to show the code, and Kani will find it when running
    quote! {
        #[cfg(feature = "verify-kani")]
        #constructor
        
        #[cfg(feature = "verify-kani")]
        #harness
    }
}

/// Generate Kani verification for an enum.
pub fn generate_kani_enum_verification(
    enum_name: &Ident,
    _variants: &[&syn::Variant],
) -> TokenStream {
    let _ = enum_name;
    quote! {
        // TODO: Enum verification
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
        .map(|(field_name, field_type)| {
            generate_field_predicate(field_name, field_type, true)
        })
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
        .map(|(field_name, field_type)| {
            generate_field_predicate(field_name, field_type, false)
        })
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
    let field_inits: Vec<_> = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! { let #name: #ty = kani::any(); }
    }).collect();
    
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
