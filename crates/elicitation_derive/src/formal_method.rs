//! `#[formal_method]` — mark a function as a contract-honoring formal method
//! and generate backend verification harnesses.
//!
//! # What the macro does
//!
//! The attribute is **not required** for type enforcement — any function with
//! the signature `Fn(In, Established<PIn>) -> (Out, Established<POut>)` already
//! satisfies the [`FormalMethod`] bound automatically via the blanket impl.
//!
//! This macro adds:
//! 1. A `#[doc]` annotation declaring the function as formal and listing
//!    its contracts.
//! 2. A `#[kani::proof]` harness under `#[cfg(kani)]`, using `kani::any()`
//!    for non-proof inputs and `Established::assert()` for proof tokens.
//! 3. A companion **unit struct** `{PascalCase}Transition` whose
//!    `kani_harness()` method returns the proof harness as a
//!    `proc_macro2::TokenStream`, enabling `build.rs` composition via
//!    [`VerifiedStateMachine::transition_harnesses`].
//! 4. A `#[requires(true)] #[ensures(true)] #[trusted]` Creusot companion
//!    under `#[cfg(creusot)]`.
//! 5. A `requires true, ensures true,` Verus companion inside `verus! { }`
//!    under `#[cfg(verus)]`.
//!
//! # Syntax
//!
//! ```rust,ignore
//! use elicitation::formal_method;
//! use elicitation::{Established, contracts::Prop};
//!
//! #[formal_method(contracts = [InvariantHolds])]
//! fn advance(state: MyState, proof: Established<InvariantHolds>)
//!     -> (MyState, Established<InvariantHolds>)
//! {
//!     (state.next(), proof)
//! }
//! ```
//!
//! The `contracts = [...]` argument is optional. Without it the macro still
//! adds the doc annotation and harnesses.
//!
//! # Generated harness (Kani)
//!
//! ```rust,ignore
//! #[cfg(kani)]
//! #[kani::proof]
//! fn advance__kani() {
//!     let state: MyState = kani::any();
//!     let proof: Established<InvariantHolds> = ::elicitation::Established::assert();
//!     let _result = advance(state, proof);
//! }
//! ```
//!
//! # Generated companion (Creusot)
//!
//! ```rust,ignore
//! #[cfg(creusot)]
//! #[requires(true)]
//! #[ensures(true)]
//! #[trusted]
//! fn advance__creusot(state: MyState, proof: Established<InvariantHolds>)
//!     -> (MyState, Established<InvariantHolds>)
//! {
//!     advance(state, proof)
//! }
//! ```
//!
//! # Generated companion (Verus)
//!
//! ```rust,ignore
//! #[cfg(verus)]
//! verus! {
//! fn advance__verus(state: MyState, proof: Established<InvariantHolds>)
//!     -> (MyState, Established<InvariantHolds>)
//!     requires true,
//!     ensures true,
//! {
//!     advance(state, proof)
//! }
//! }
//! ```
//!
//! [`FormalMethod`]: elicitation::contracts::FormalMethod

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    FnArg, ItemFn, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// ── String helpers ─────────────────────────────────────────────────────────────

/// Convert a `snake_case` identifier string to `PascalCase`.
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

// ── Attribute argument parsing ─────────────────────────────────────────────────

/// Parsed arguments from `#[formal_method(contracts = [C1, C2, ...])]`.
struct FormalMethodArgs {
    contracts: Vec<syn::Path>,
}

impl Parse for FormalMethodArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(FormalMethodArgs {
                contracts: Vec::new(),
            });
        }
        let ident: syn::Ident = input.parse()?;
        if ident != "contracts" {
            return Err(syn::Error::new(
                ident.span(),
                "expected `contracts = [ContractType, ...]`",
            ));
        }
        let _: Token![=] = input.parse()?;
        let content;
        syn::bracketed!(content in input);
        let contracts = Punctuated::<syn::Path, Token![,]>::parse_terminated(&content)?;
        Ok(FormalMethodArgs {
            contracts: contracts.into_iter().collect(),
        })
    }
}

// ── Type helpers ───────────────────────────────────────────────────────────────

/// Returns `true` if `ty` resolves to `String` / `std::string::String`.
///
/// `kani::any::<String>()` creates an unbounded symbolic string, causing
/// CBMC to explore infinite paths.  Callers should emit a bounded byte-array
/// construction instead.
fn is_string_type(ty: &Type) -> bool {
    let Type::Path(tp) = ty else { return false };
    let segs = &tp.path.segments;
    match segs.len() {
        1 => segs[0].ident == "String",
        3 => {
            segs[0].ident == "std"
                && segs[1].ident == "string"
                && segs[2].ident == "String"
        }
        _ => false,
    }
}

/// Returns `true` if `ty` is `Established<_>` (any path ending in `Established`
/// with exactly one generic argument).
fn is_established_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map(|s| {
                s.ident == "Established"
                    && matches!(
                        &s.arguments,
                        syn::PathArguments::AngleBracketed(ab) if ab.args.len() == 1
                    )
            })
            .unwrap_or(false)
    } else {
        false
    }
}

// ── Expansion ─────────────────────────────────────────────────────────────────

pub fn expand(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let parsed_args: FormalMethodArgs = syn::parse2(args)?;
    let mut func: ItemFn = syn::parse2(item)?;

    let fn_name = &func.sig.ident;

    // ── Doc annotation ────────────────────────────────────────────────────────
    let contracts_doc = if parsed_args.contracts.is_empty() {
        "**\\[formal\\_method\\]**".to_string()
    } else {
        let names: Vec<String> = parsed_args
            .contracts
            .iter()
            .map(|p| {
                let s = quote!(#p).to_string();
                s.replace(' ', "")
            })
            .collect();
        format!(
            "**\\[formal\\_method\\]** Contracts: `{}`",
            names.join("`, `")
        )
    };
    let doc_attr: syn::Attribute = syn::parse_quote!(#[doc = #contracts_doc]);
    func.attrs.push(doc_attr);

    // ── Backend harnesses ─────────────────────────────────────────────────────
    // Only generated for free functions (no `self`) and non-async functions.
    // Kani doesn't support async proofs, and `self` requires a concrete receiver.
    let has_receiver = func
        .sig
        .inputs
        .iter()
        .any(|a| matches!(a, FnArg::Receiver(_)));
    let is_async = func.sig.asyncness.is_some();

    let contracts_str = if parsed_args.contracts.is_empty() {
        String::new()
    } else {
        parsed_args
            .contracts
            .iter()
            .map(|p| quote!(#p).to_string().replace(' ', ""))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let (kani_harness, creusot_companion, verus_companion, companion_struct) =
        if !is_async && !has_receiver {
            let kani_fn = format_ident!("{fn_name}__kani");
            let creusot_fn = format_ident!("{fn_name}__creusot");
            let verus_fn = format_ident!("{fn_name}__verus");

            let mut lets: Vec<TokenStream> = Vec::new();
            let mut call_args: Vec<TokenStream> = Vec::new();

            for arg in &func.sig.inputs {
                if let FnArg::Typed(pat_type) = arg {
                    let pat = &pat_type.pat;
                    let ty = &pat_type.ty;
                    if is_established_type(ty) {
                        lets.push(quote! {
                            let #pat: #ty = ::elicitation::Established::assert();
                        });
                    } else if is_string_type(ty) {
                        // kani::any::<String>() creates an unbounded symbolic string;
                        // derive a bounded one from a fixed-size byte array instead.
                        lets.push(quote! {
                            let #pat: #ty = {
                                let bytes: [u8; 16] = ::kani::any();
                                ::std::string::String::from_utf8_lossy(&bytes).into_owned()
                            };
                        });
                    } else {
                        lets.push(quote! {
                            let #pat: #ty = ::kani::any();
                        });
                    }
                    call_args.push(quote!(#pat));
                }
            }

            let inputs = &func.sig.inputs;
            let output = &func.sig.output;

            // ── Kani inline harness ──────────────────────────────────────────
            let kani = quote! {
                #[cfg(kani)]
                #[::kani::proof]
                fn #kani_fn() {
                    #(#lets)*
                    let _result = #fn_name(#(#call_args),*);
                }
            };

            // ── Companion struct: returns the harness as a runtime TokenStream ──
            //
            // The harness source is captured here (at proc-macro expansion time)
            // as a string literal, then parsed back into a TokenStream when
            // `kani_harness()` is called (e.g. from a `build.rs` or
            // `VerifiedStateMachine::vsm_kani_proof()`).
            let harness_src = kani.to_string();
            let struct_name = format_ident!("{}Transition", to_pascal_case(&fn_name.to_string()));
            let vis = &func.vis;
            let struct_doc = format!(
                "Kani harness companion for [`{fn_name}`].\n\
                 \n\
                 Call [`{struct_name}::kani_harness`] from a `build.rs` or \
                 [`VerifiedStateMachine::transition_harnesses`] to obtain the \
                 `#[kani::proof]` harness as a `proc_macro2::TokenStream`."
            );
            let companion = quote! {
                #[doc = #struct_doc]
                #[allow(non_camel_case_types)]
                #vis struct #struct_name;
                impl #struct_name {
                    /// Return the Kani harness `TokenStream` for this transition.
                    ///
                    /// The string was captured at macro-expansion time; parsing it
                    /// at runtime guarantees the harness stays in sync with the
                    /// function signature.
                    pub fn kani_harness() -> ::proc_macro2::TokenStream {
                        #harness_src
                            .parse()
                            .expect("formal_method companion: invalid kani harness source")
                    }
                }
            };

            // ── Creusot companion ────────────────────────────────────────────
            let creusot_doc = if contracts_str.is_empty() {
                "Creusot companion.".to_string()
            } else {
                format!("Creusot companion. Contracts: `{contracts_str}`.")
            };
            let creusot = quote! {
                #[cfg(creusot)]
                #[doc = #creusot_doc]
                #[requires(true)]
                #[ensures(true)]
                #[trusted]
                fn #creusot_fn(#inputs) #output {
                    #fn_name(#(#call_args),*)
                }
            };

            // ── Verus companion ──────────────────────────────────────────────
            let verus_doc = if contracts_str.is_empty() {
                "Verus companion.".to_string()
            } else {
                format!("Verus companion. Contracts: `{contracts_str}`.")
            };
            let verus = quote! {
                #[cfg(verus)]
                verus! {
                    #[doc = #verus_doc]
                    fn #verus_fn(#inputs) #output
                        requires true,
                        ensures true,
                    {
                        #fn_name(#(#call_args),*)
                    }
                }
            };

            (kani, creusot, verus, companion)
        } else {
            (quote! {}, quote! {}, quote! {}, quote! {})
        };

    Ok(quote! {
        #func
        #kani_harness
        #creusot_companion
        #verus_companion
        #companion_struct
    })
}
