//! Verification contract generation for #[derive(Elicit)].

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident};

mod creusot;
mod kani;
mod prusti;
mod verus;

/// Generate verification code for a struct.
pub fn generate_struct_verification(struct_name: &Ident, fields: &[&Field]) -> TokenStream {
    // Only generate Kani verification for user crates (it's the only one with proper cfg support)
    // Other verifiers (creusot, prusti, verus) are only used in elicitation's own codebase
    let kani_code = kani::generate_kani_verification(struct_name, fields);

    quote! {
        #kani_code
    }
}

/// Generate verification code for an enum.
pub fn generate_enum_verification(enum_name: &Ident, variants: &[&syn::Variant]) -> TokenStream {
    // Only generate Kani verification for user crates
    let kani_code = kani::generate_kani_enum_verification(enum_name, variants);

    quote! {
        #kani_code
    }
}
