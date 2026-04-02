//! Derive macro implementation for `#[derive(Prop)]`.
//!
//! Generates trivial but non-empty, uniquely-named proof harnesses for
//! zero-cost typestate marker propositions. The generated harness function is
//! named `verify_<snake_type_name>_prop_marker`, ensuring no name collisions
//! when multiple proposition types' proofs are assembled into a single
//! verification target.

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use heck::ToSnakeCase;

/// Expand `#[derive(Prop)]` for a struct.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Gate proof method generation on elicitation_derive's own `proofs` feature.
    // This mirrors how `#[derive(Elicit)]` conditionally generates proof methods
    // in struct_impl.rs — using compile-time cfg, NOT `#[cfg(...)]` in generated code.
    let proof_methods = {
        // Convert PascalCase to snake_case at macro-expansion time so the
        // downstream crate needs no snake_case dependency at runtime.
        let snake_name = name.to_string().to_snake_case();
        quote! {
            fn kani_proof() -> ::elicitation::proc_macro2::TokenStream {
                ::elicitation::verification::proof_helpers::kani_trivial_prop(#snake_name)
            }

            fn verus_proof() -> ::elicitation::proc_macro2::TokenStream {
                ::elicitation::verification::proof_helpers::verus_trivial_prop(#snake_name)
            }

            fn creusot_proof() -> ::elicitation::proc_macro2::TokenStream {
                ::elicitation::verification::proof_helpers::creusot_trivial_prop(#snake_name)
            }
        }
    };

    let expanded = quote! {
        impl #impl_generics ::elicitation::contracts::Prop for #name #ty_generics
        #where_clause
        {
            #proof_methods
        }
    };

    expanded.into()
}
