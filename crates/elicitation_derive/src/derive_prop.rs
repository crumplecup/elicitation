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

/// Parsed content of `#[prop(credential = SomeType, creusot_invariant_fn = "fn_name", kani_invariant_fn = "fn_name", verus_invariant_fn = "fn_name")]`.
struct PropArgs {
    credential: Option<Path>,
    creusot_invariant_fn: Option<String>,
    kani_invariant_fn: Option<String>,
    verus_invariant_fn: Option<String>,
}

impl Parse for PropArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(PropArgs {
                credential: None,
                creusot_invariant_fn: None,
                kani_invariant_fn: None,
                verus_invariant_fn: None,
            });
        }
        let mut credential = None;
        let mut creusot_invariant_fn = None;
        let mut kani_invariant_fn = None;
        let mut verus_invariant_fn = None;
        loop {
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            match ident.to_string().as_str() {
                "credential" => {
                    let path: Path = input.parse()?;
                    credential = Some(path);
                }
                "creusot_invariant_fn" => {
                    let lit: syn::LitStr = input.parse()?;
                    creusot_invariant_fn = Some(lit.value());
                }
                "kani_invariant_fn" => {
                    let lit: syn::LitStr = input.parse()?;
                    kani_invariant_fn = Some(lit.value());
                }
                "verus_invariant_fn" => {
                    let lit: syn::LitStr = input.parse()?;
                    verus_invariant_fn = Some(lit.value());
                }
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!(
                            "unknown prop key `{other}`; expected `credential`, `creusot_invariant_fn`, `kani_invariant_fn`, or `verus_invariant_fn`"
                        ),
                    ));
                }
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
                if input.is_empty() {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(PropArgs {
            credential,
            creusot_invariant_fn,
            kani_invariant_fn,
            verus_invariant_fn,
        })
    }
}

/// Extract the `#[prop(...)]` attribute from the derive input, if present.
fn extract_prop_args(input: &DeriveInput) -> syn::Result<PropArgs> {
    for attr in &input.attrs {
        if attr.path().is_ident("prop") {
            let args: PropArgs = attr.parse_args()?;
            return Ok(args);
        }
    }
    Ok(PropArgs {
        credential: None,
        creusot_invariant_fn: None,
        kani_invariant_fn: None,
        verus_invariant_fn: None,
    })
}

// ── Expand ────────────────────────────────────────────────────────────────────

/// Expand `#[derive(Prop)]` for a struct.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let prop_args = match extract_prop_args(&input) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error().into(),
    };
    let credential = prop_args.credential;
    let creusot_fn_name = prop_args.creusot_invariant_fn;
    let kani_fn_name = prop_args.kani_invariant_fn;
    let verus_fn_name = prop_args.verus_invariant_fn;

    let snake_name = name.to_string().to_snake_case();

    // Generate the optional `creusot_invariant_fn_name()` override when specified.
    let creusot_fn_override = match &creusot_fn_name {
        Some(fn_name) => quote! {
            fn creusot_invariant_fn_name() -> &'static str {
                #fn_name
            }
        },
        None => quote! {},
    };

    // Generate the optional `kani_invariant_fn_name()` override when specified.
    let kani_fn_override = match &kani_fn_name {
        Some(fn_name) => quote! {
            fn kani_invariant_fn_name() -> &'static str {
                #fn_name
            }
        },
        None => quote! {},
    };

    // Generate the optional `verus_invariant_fn_name()` override when specified.
    let verus_fn_override = match &verus_fn_name {
        Some(fn_name) => quote! {
            fn verus_invariant_fn_name() -> &'static str {
                #fn_name
            }
        },
        None => quote! {},
    };

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

        #creusot_fn_override

        #kani_fn_override

        #verus_fn_override
    };

    // Generate `kani_proof_credential()` inherent method and, for the
    // no-credential case, a blanket `ProvableFrom<()>` impl.
    let (credential_impl, provable_from_impl) = match credential {
        Some(cred_ty) => {
            // User-declared credential: formal_method will call prove(&P::kani_proof_credential()).
            // The user must separately declare impl ProvableFrom<cred_ty> for Self.
            let cred_impl = quote! {
                #[allow(unexpected_cfgs)]
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
                #[allow(unexpected_cfgs)]
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
