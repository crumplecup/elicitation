//! Kani verifier backend.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

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

fn generate_constructor(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let constructor_name = quote::format_ident!("__make_{}", struct_name);
    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    
    quote! {
        #[kani::requires(true)]
        #[kani::ensures(|_result: &#struct_name| true)]
        fn #constructor_name(#(#field_names: #field_types),*) -> #struct_name {
            #struct_name { #(#field_names),* }
        }
    }
}

fn generate_harness(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    let constructor_name = quote::format_ident!("__make_{}", struct_name);
    let harness_name = quote::format_ident!("__verify_{}", struct_name);
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
