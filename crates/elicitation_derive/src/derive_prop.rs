//! Derive macro implementation for `#[derive(Prop)]`.
//!
//! Generates trivial but non-empty, uniquely-named proof harnesses for
//! zero-cost typestate marker propositions. The generated harness function is
//! named `verify_<snake_type_name>_prop_marker`, ensuring no name collisions
//! when multiple proposition types' proofs are assembled into a single
//! verification target.
//!
//! # Credential attribute
//!
//! To wire a proposition to a credential type from `elicit_db` or `elicit_ui`,
//! annotate the struct with `#[prop(credential = SomeCredentialType)]`:
//!
//! ```rust,ignore
//! #[derive(Prop)]
//! #[prop(credential = WcagVerified)]
//! pub struct ArchivePanelConsistent;
//! ```
//!
//! This generates a `kani_proof_credential()` inherent method that returns the
//! credential type (via `kani::any()`). `#[formal_method]` harnesses call this
//! method to produce `Established::prove(&credential)` instead of the weaker
//! `Established::assert()`.
//!
//! Without `#[prop(credential = ...)]`, the credential defaults to `()` and a
//! blanket `impl ProvableFrom<()> for Self` is generated so that `assert()`-
//! equivalent proofs still compile.

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Path, Token, parse::Parse, parse::ParseStream};

use heck::ToSnakeCase;

// ── Attribute parsing ──────────────────────────────────────────────────────────

/// Parsed content of `#[prop(credential = SomeType)]`.
struct PropArgs {
    credential: Option<Path>,
}

impl Parse for PropArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(PropArgs { credential: None });
        }
        let ident: syn::Ident = input.parse()?;
        if ident != "credential" {
            return Err(syn::Error::new(
                ident.span(),
                "expected `credential = SomeType`",
            ));
        }
        let _: Token![=] = input.parse()?;
        let path: Path = input.parse()?;
        Ok(PropArgs {
            credential: Some(path),
        })
    }
}

/// Extract the `#[prop(credential = T)]` attribute from the derive input, if present.
fn extract_credential(input: &DeriveInput) -> syn::Result<Option<Path>> {
    for attr in &input.attrs {
        if attr.path().is_ident("prop") {
            let args: PropArgs = attr.parse_args()?;
            return Ok(args.credential);
        }
    }
    Ok(None)
}

// ── Expand ────────────────────────────────────────────────────────────────────

/// Expand `#[derive(Prop)]` for a struct.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let credential = match extract_credential(&input) {
        Ok(c) => c,
        Err(e) => return e.to_compile_error().into(),
    };

    let snake_name = name.to_string().to_snake_case();

    let proof_methods = quote! {
        fn kani_proof() -> ::elicitation::proc_macro2::TokenStream {
            ::elicitation::verification::proof_helpers::kani_trivial_prop(#snake_name)
        }

        fn verus_proof() -> ::elicitation::proc_macro2::TokenStream {
            ::elicitation::verification::proof_helpers::verus_trivial_prop(#snake_name)
        }

        fn creusot_proof() -> ::elicitation::proc_macro2::TokenStream {
            ::elicitation::verification::proof_helpers::creusot_trivial_prop(#snake_name)
        }
    };

    // Generate `kani_proof_credential()` inherent method and, for the
    // no-credential case, a blanket `ProvableFrom<()>` impl.
    let (credential_impl, provable_from_impl) = match credential {
        Some(cred_ty) => {
            // User-declared credential: formal_method will call prove(&P::kani_proof_credential()).
            // The user must separately declare impl ProvableFrom<cred_ty> for Self.
            let cred_impl = quote! {
                #[cfg(kani)]
                impl #impl_generics #name #ty_generics #where_clause {
                    /// Return a Kani symbolic credential for use in proof harnesses.
                    ///
                    /// Called by `#[formal_method]`-generated harnesses to produce
                    /// `Established::prove(&credential)` instead of `Established::assert()`.
                    #[allow(dead_code)]
                    pub fn kani_proof_credential() -> #cred_ty {
                        ::kani::any::<#cred_ty>()
                    }
                }
            };
            (cred_impl, quote! {})
        }
        None => {
            // No credential: use `()` as credential and auto-derive ProvableFrom<()>.
            // This preserves backward-compatible assert()-equivalent proofs.
            let cred_impl = quote! {
                #[cfg(kani)]
                impl #impl_generics #name #ty_generics #where_clause {
                    /// Return the trivial unit credential used for marker propositions.
                    #[allow(dead_code)]
                    pub fn kani_proof_credential() -> () {
                        ()
                    }
                }
            };
            let pf_impl = quote! {
                impl #impl_generics ::elicitation::contracts::ProvableFrom<()>
                    for #name #ty_generics #where_clause {}
            };
            (cred_impl, pf_impl)
        }
    };

    let expanded = quote! {
        impl #impl_generics ::elicitation::contracts::Prop for #name #ty_generics
        #where_clause
        {
            #proof_methods
        }

        #provable_from_impl

        #credential_impl
    };

    expanded.into()
}
