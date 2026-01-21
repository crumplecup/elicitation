//! Verification contract generation for #[derive(Elicit)].

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

mod kani;
mod creusot;

/// Generate verification code for a struct.
pub fn generate_struct_verification(
    struct_name: &Ident,
    fields: &[&Field],
) -> TokenStream {
    let kani_code = kani::generate_kani_verification(struct_name, fields);
    let creusot_code = creusot::generate_creusot_verification(struct_name, fields);
    
    quote! {
        #kani_code
        #creusot_code
    }
}

/// Generate verification code for an enum.
pub fn generate_enum_verification(
    enum_name: &Ident,
    variants: &[&syn::Variant],
) -> TokenStream {
    let kani_code = kani::generate_kani_enum_verification(enum_name, variants);
    let creusot_code = creusot::generate_creusot_enum_verification(enum_name, variants);
    
    quote! {
        #kani_code
        #creusot_code
    }
}
