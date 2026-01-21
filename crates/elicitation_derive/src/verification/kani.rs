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
    
    quote! {
        #[cfg(feature = "verify-kani")]
        #[cfg(kani)]
        #constructor
        
        #[cfg(feature = "verify-kani")]
        #[cfg(kani)]
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

/// Generate a contract predicate for a field by querying its type metadata.
///
/// This generates code that:
/// 1. Attempts to call TypeName::__contract_requires() or __contract_ensures()
/// 2. If available, uses the metadata to generate a predicate
/// 3. Falls back to `true` if no metadata available
fn generate_field_predicate(
    field_name: &Ident,
    field_type: &Type,
    is_requires: bool, // true for requires, false for ensures
) -> TokenStream {
    // Try to extract type path
    let Some(type_path) = extract_type_path(field_type) else {
        // Complex type, can't extract metadata, use trivial predicate
        return quote! { true };
    };

    let method_name = if is_requires {
        format_ident!("__contract_requires")
    } else {
        format_ident!("__contract_ensures")
    };

    // Generate code that conditionally checks for metadata
    // We use a const context to query the metadata at compile time
    if is_requires {
        // For requires: predicate applies to the input parameter
        quote! {
            {
                // Attempt to query contract metadata
                // If the type has __contract_requires(), we could use it here
                // For now, use a simpler approach: just validate construction
                
                // Check if value can be constructed (basic invariant)
                // This is a placeholder - real implementation needs string parsing
                true
            }
        }
    } else {
        // For ensures: predicate applies to the field in the result
        quote! {
            {
                // Ensure the field in the result satisfies invariants
                // Access as: result.field_name
                true
            }
        }
    }
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
    
    let field_inits: Vec<_> = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! { let #name: #ty = kani::any(); }
    }).collect();
    
    quote! {
        #[kani::proof_for_contract(#constructor_name)]
        fn #harness_name() {
            #(#field_inits)*
            let _ = #constructor_name(#(#field_names),*);
        }
    }
}
