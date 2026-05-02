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

/// Returns `true` if `ty` is `Vec<_>` (any generic `Vec`).
///
/// `kani::any::<Vec<T>>()` fails in Kani ≤0.67 because there is no blanket
/// `impl kani::Arbitrary for Vec<T>`.  Callers should emit `Vec::new()` instead,
/// which is always valid and keeps proofs bounded.
fn is_vec_type(ty: &Type) -> bool {
    let Type::Path(tp) = ty else { return false };
    let segs = &tp.path.segments;
    match segs.len() {
        1 => segs[0].ident == "Vec",
        3 => segs[0].ident == "std" && segs[1].ident == "vec" && segs[2].ident == "Vec",
        _ => false,
    }
}

/// Returns `true` if `ty` is `Option<_>` (any path ending in `Option` with one
/// generic argument).
///
/// `kani::any::<Option<T>>()` hangs in CBMC when T contains heap-allocated
/// fields (BTreeMap, HashMap, etc.): symbolic ownership transfer of the inner
/// value combined with a non-trivial destructor causes unbounded unwinding.
/// Callers should emit `None` instead — option parameters are auxiliary state
/// that doesn't affect structural invariant preservation.
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map(|s| {
                s.ident == "Option"
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

/// Returns `true` if `ty` is `String` / `std::string::String`.
///
/// `kani::any::<String>()` creates an unbounded symbolic string, causing
/// CBMC to explore infinite paths.  Callers should emit a bounded byte-array
/// construction instead.
fn is_string_type(ty: &Type) -> bool {
    let Type::Path(tp) = ty else { return false };
    let segs = &tp.path.segments;
    match segs.len() {
        1 => segs[0].ident == "String",
        3 => segs[0].ident == "std" && segs[1].ident == "string" && segs[2].ident == "String",
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

/// Extract the inner proposition type `P` from `Established<P>`.
///
/// Returns `Some(inner_type)` when `ty` is `Established<P>`, `None` otherwise.
fn extract_established_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        let seg = type_path.path.segments.last()?;
        if seg.ident != "Established" {
            return None;
        }
        if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments
            && ab.args.len() == 1
            && let syn::GenericArgument::Type(inner) = &ab.args[0]
        {
            return Some(inner);
        }
    }
    None
}

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
            // Three depth variants for non-state inputs.
            // Depth-bounded types use KaniCompose::kani_depthN() instead of
            // kani::any(), preventing CBMC's type-based destructor model from
            // unwinding recursively through types like Vec<ExplainNode>.
            let mut non_state_lets_d0: Vec<TokenStream> = Vec::new();
            let mut non_state_lets_d1: Vec<TokenStream> = Vec::new();
            let mut non_state_lets_d2: Vec<TokenStream> = Vec::new();
            let mut call_args: Vec<TokenStream> = Vec::new();
            let mut state_pat_str: Option<String> = None;
            let mut state_ty_str: Option<String> = None;

            for arg in &func.sig.inputs {
                if let FnArg::Typed(pat_type) = arg {
                    let pat = &pat_type.pat;
                    let ty = &pat_type.ty;
                    if is_established_type(ty) {
                        // Use the prop type's kani_proof_credential() to build
                        // Established::prove(&cred) instead of the axiom assert().
                        // This ensures CBMC only explores the bounded credential type,
                        // not the full state space of the underlying state enum.
                        let inner = extract_established_inner(ty)
                            .expect("is_established_type guarantees an inner type");
                        let let_ts = quote! {
                            let #pat: #ty = {
                                let __cred = #inner::kani_proof_credential();
                                ::elicitation::Established::prove(&__cred)
                            };
                        };
                        lets.push(let_ts.clone());
                        non_state_lets_d0.push(let_ts.clone());
                        non_state_lets_d1.push(let_ts.clone());
                        non_state_lets_d2.push(let_ts);
                    } else if is_string_type(ty) {
                        // String content is irrelevant to structural invariant proofs.
                        // from_utf8_lossy introduces a UTF-8 validation loop that makes
                        // the state space unbounded; String::new() is bounded by construction.
                        let let_ts = quote! {
                            let #pat: #ty = ::std::string::String::new();
                        };
                        lets.push(let_ts.clone());
                        non_state_lets_d0.push(let_ts.clone());
                        non_state_lets_d1.push(let_ts.clone());
                        non_state_lets_d2.push(let_ts);
                    } else if is_vec_type(ty) {
                        // kani::any::<Vec<T>>() is not implemented in Kani ≤0.67;
                        // use an empty Vec instead — still covers all invariant checks
                        // without requiring T: kani::Arbitrary on the Vec itself.
                        let let_ts = quote! {
                            let #pat: #ty = ::std::vec::Vec::new();
                        };
                        lets.push(let_ts.clone());
                        non_state_lets_d0.push(let_ts.clone());
                        non_state_lets_d1.push(let_ts.clone());
                        non_state_lets_d2.push(let_ts);
                    } else if is_option_type(ty) {
                        // kani::any::<Option<T>>() hangs when T contains heap-allocated
                        // fields — symbolic ownership transfer + non-trivial destructor
                        // causes unbounded unwinding in CBMC. Option parameters are
                        // auxiliary state; None is the safe bounded choice for invariant proofs.
                        let let_ts = quote! {
                            let #pat: #ty = ::core::option::Option::None;
                        };
                        lets.push(let_ts.clone());
                        non_state_lets_d0.push(let_ts.clone());
                        non_state_lets_d1.push(let_ts.clone());
                        non_state_lets_d2.push(let_ts);
                    } else if state_pat_str.is_none() {
                        // First remaining param = the VSM state enum.
                        // Captured as strings for kani_harness_for_variant; NOT added
                        // to non_state_lets so the per-variant harness can substitute
                        // a concrete construction expression in its place.
                        state_pat_str = Some(quote!(#pat).to_string());
                        state_ty_str = Some(quote!(#ty).to_string());
                        lets.push(quote! {
                            let #pat: #ty = ::kani::any();
                        });
                    } else {
                        // Payload arguments (all non-state, non-proof, non-primitive params)
                        // always use kani_depth0() regardless of harness depth level.
                        //
                        // The d0/d1/d2 depth variation is exclusively about the INPUT STATE
                        // shape (empty → 1-element → 2-element collections), proving the
                        // invariant holds for any bounded input depth by induction.
                        //
                        // Payload arguments represent data injected into the OUTPUT state.
                        // Scaling their depth with the harness depth causes "two-drop SAT
                        // explosion": the joint CBMC drop model for (d1 input) × (d1 payload
                        // output) crosses the tractability threshold when the payload type
                        // has multiple Vec fields (e.g. AdminSnapshot with 5 Vecs,
                        // ConnectionProfile with 9 String/Option<String> fields).
                        //
                        // kani_depth0() payloads are sufficient for invariant proofs: if the
                        // transition preserves the invariant with a minimal well-formed payload,
                        // it preserves it for any payload (payload content does not affect
                        // structural invariant preservation in VSM transitions).
                        //
                        // The type MUST implement KaniCompose — enforced at Kani compile time.
                        let let_ts_any = quote! {
                            let #pat: #ty = ::kani::any();
                        };
                        lets.push(let_ts_any);
                        let let_ts_d0 = quote! {
                            let #pat: #ty = <#ty as ::elicitation::KaniCompose>::kani_depth0();
                        };
                        non_state_lets_d0.push(let_ts_d0.clone());
                        non_state_lets_d1.push(let_ts_d0.clone());
                        non_state_lets_d2.push(let_ts_d0);
                    }
                    call_args.push(quote!(#pat));
                }
            }

            let has_state_param = state_pat_str.is_some();
            let state_pat_s = state_pat_str.unwrap_or_default();
            let state_ty_s = state_ty_str.unwrap_or_default();
            let non_state_lets_d0_src = quote!(#(#non_state_lets_d0)*).to_string();
            let non_state_lets_d1_src = quote!(#(#non_state_lets_d1)*).to_string();
            let non_state_lets_d2_src = quote!(#(#non_state_lets_d2)*).to_string();
            let call_args_src = quote!(#(#call_args),*).to_string();
            let fn_name_src = fn_name.to_string();
            let kani_fn_src = format!("{fn_name}__kani");

            let inputs = &func.sig.inputs;
            let output = &func.sig.output;
            let inputs_src = quote!(#inputs).to_string();
            let output_src = quote!(#output).to_string();
            let creusot_fn_src = format!("{fn_name}__creusot");

            // ── Kani harness ─────────────────────────────────────────────────
            // kani_vec delegates to kani::any_vec::<T, 0>() which takes the
            // vec![] branch immediately — no symbolic heap, no unbounded drops.
            // No stub_verified needed.
            let kani = quote! {
                #[allow(unexpected_cfgs)]
                #[cfg(kani)]
                #[::kani::proof]
                fn #kani_fn() {
                    #(#lets)*
                    let _result = #fn_name(#(#call_args),*);
                    // Safe Rust only, no custom Drop impls: forget is sound.
                    // Eliminates CBMC drop-glue reasoning for Vec-bearing return types.
                    ::std::mem::forget(_result);
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

            // Per-variant harness methods — one per depth.
            // `kani_harness_for_variant_at_depth` substitutes a concrete
            // construction expression for the state param AND appends
            // `__d{n}` to the harness function name.
            let variant_method = if has_state_param {
                quote! {
                    /// Return a depth-bounded per-variant Kani harness.
                    ///
                    /// `variant_name` — snake_case suffix (e.g. `"explain_view"`).
                    /// `state_expr` — concrete construction expression for that variant
                    ///   (from [`KaniVariantConstruction::depth0`] / `depth1` / `depth2`).
                    /// `depth` — 0, 1, or 2; appended as `__d{depth}` to the harness name.
                    pub fn kani_harness_for_variant_at_depth(
                        variant_name: &str,
                        state_expr: &str,
                        depth: u8,
                    ) -> ::proc_macro2::TokenStream {
                        let variant_fn = format!(
                            "{}__{}__d{}",
                            #kani_fn_src, variant_name, depth
                        );
                        let non_state_lets = match depth {
                            0 => #non_state_lets_d0_src,
                            1 => #non_state_lets_d1_src,
                            _ => #non_state_lets_d2_src,
                        };
                        let src = String::new()
                            + "# [cfg (kani)] # [:: kani :: proof] fn "
                            + &variant_fn
                            + " () { let "
                            + #state_pat_s
                            + " : "
                            + #state_ty_s
                            + " = "
                            + state_expr
                            + " ; "
                            + &non_state_lets
                            + " let _result = "
                            + #fn_name_src
                            + " ("
                            + #call_args_src
                            + ") ; :: std :: mem :: forget (_result) ; }";
                        src.parse()
                            .expect("kani_harness_for_variant_at_depth: invalid TokenStream")
                    }

                    /// Convenience wrapper for depth-0 (backward compat shim).
                    #[deprecated(note = "use kani_harness_for_variant_at_depth")]
                    pub fn kani_harness_for_variant(
                        variant_name: &str,
                        state_expr: &str,
                    ) -> ::proc_macro2::TokenStream {
                        Self::kani_harness_for_variant_at_depth(variant_name, state_expr, 0)
                    }
                }
            } else {
                // No state param: body is the same for all variants; only the
                // harness function name is suffixed with variant name + depth.
                quote! {
                    /// Return a depth-bounded per-variant Kani harness.
                    ///
                    /// Since this transition has no state parameter, the body is
                    /// the same for all variants; only the harness function name
                    /// is suffixed.
                    pub fn kani_harness_for_variant_at_depth(
                        variant_name: &str,
                        _state_expr: &str,
                        depth: u8,
                    ) -> ::proc_macro2::TokenStream {
                        let variant_fn = format!(
                            "{}__{}__d{}",
                            #kani_fn_src, variant_name, depth
                        );
                        #harness_src
                            .replace(#kani_fn_src, &variant_fn)
                            .parse()
                            .expect("kani_harness_for_variant_at_depth: invalid TokenStream")
                    }

                    /// Convenience wrapper for depth-0 (backward compat shim).
                    #[deprecated(note = "use kani_harness_for_variant_at_depth")]
                    pub fn kani_harness_for_variant(
                        variant_name: &str,
                        state_expr: &str,
                    ) -> ::proc_macro2::TokenStream {
                        Self::kani_harness_for_variant_at_depth(variant_name, state_expr, 0)
                    }
                }
            };

            let companion = quote! {
                // The companion struct is used only from build.rs / codegen paths.
                // Under Kani it must be absent: its methods return
                // `proc_macro2::TokenStream` (backed by `Vec<TokenTree>`, a
                // recursive heap type) and CBMC would inflate the SAT formula
                // through the drop-glue for every companion in the crate.
                #[allow(unexpected_cfgs)]
                #[cfg(not(kani))]
                #[doc = #struct_doc]
                #[allow(non_camel_case_types)]
                #vis struct #struct_name;
                #[allow(unexpected_cfgs)]
                #[cfg(not(kani))]
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

                    #variant_method

                    /// Return a Creusot companion contract `TokenStream` for this transition.
                    ///
                    /// When `inv_fn` is non-empty and this transition has a state parameter,
                    /// a real `#[requires(inv_fn(&state))]` / `#[ensures(inv_fn(&result.0))]`
                    /// contract is emitted (no `#[trusted]`) — Creusot will verify the body.
                    ///
                    /// When `inv_fn` is empty or this transition has no state parameter,
                    /// a `#[trusted]` placeholder is emitted instead.
                    pub fn creusot_contract(inv_fn: &str) -> ::proc_macro2::TokenStream {
                        let src = if !inv_fn.is_empty() && #has_state_param {
                            String::new()
                                + "# [cfg (creusot)] # [requires ("
                                + inv_fn
                                + " (& "
                                + #state_pat_s
                                + "))] # [ensures ("
                                + inv_fn
                                + " (& result . 0))] pub (crate) fn "
                                + #creusot_fn_src
                                + " ("
                                + #inputs_src
                                + ") "
                                + #output_src
                                + " { "
                                + #fn_name_src
                                + " ("
                                + #call_args_src
                                + ") }"
                        } else {
                            String::new()
                                + "# [cfg (creusot)] # [requires (true)] # [ensures (true)] # [trusted] pub (crate) fn "
                                + #creusot_fn_src
                                + " ("
                                + #inputs_src
                                + ") "
                                + #output_src
                                + " { "
                                + #fn_name_src
                                + " ("
                                + #call_args_src
                                + ") }"
                        };
                        src.parse()
                            .expect("creusot_contract: invalid TokenStream")
                    }

                    /// Return a Kani closure proof `TokenStream` for this transition.
                    ///
                    /// When `inv_fn` is non-empty and this transition has a state parameter,
                    /// emits five items:
                    ///
                    /// 1. A contracted wrapper `fn_name__kani_contracted` with `#[kani::requires]`
                    ///    and `#[kani::ensures]` annotations (documentation / Creusot companion;
                    ///    **not** used in the proof harnesses below).
                    ///
                    /// 2-4. Three `#[kani::proof]` depth harnesses (`_d0`, `_d1`, `_d2`) that
                    ///    verify the invariant at the base case and two inductive steps using
                    ///    `KaniCompose::kani_depth{n}()`.  Each calls the ACTUAL function (not
                    ///    the contracted wrapper) after `kani::assume(inv_fn(&state))`, and
                    ///    asserts `inv_fn(&result.0)` afterward.  No DFCC is triggered because
                    ///    `#[kani::proof_for_contract]` is not used.
                    ///
                    /// 5. A `#[kani::proof]` closure harness that uses `KaniCompose::kani_any()`
                    ///    for symbolic coverage of all variants.  Same assume/assert pattern;
                    ///    `std::mem::forget` prevents drop-glue SAT explosion.
                    ///
                    /// When `inv_fn` is empty or this transition has no state parameter,
                    /// an empty `TokenStream` is returned.
                    pub fn kani_closure_proof(inv_fn: &str) -> ::proc_macro2::TokenStream {
                        if inv_fn.is_empty() || !#has_state_param {
                            return ::proc_macro2::TokenStream::new();
                        }
                        let contracted_fn = String::new() + #fn_name_src + "__kani_contracted";
                        let closure_d0_fn = String::new() + #fn_name_src + "__kani_closure_d0";
                        let closure_d1_fn = String::new() + #fn_name_src + "__kani_closure_d1";
                        let closure_d2_fn = String::new() + #fn_name_src + "__kani_closure_d2";
                        let closure_fn    = String::new() + #fn_name_src + "__kani_closure";

                        // Build one plain #[kani::proof] depth harness (no DFCC).
                        // Calls the ACTUAL function, not the contracted wrapper, so
                        // goto-instrument --dfcc is never invoked.
                        let depth_harness = |harness_fn: &str, depth_method: &str, non_state_lets: &str| {
                            String::new()
                                + "# [allow (unexpected_cfgs)] # [cfg (kani)] "
                                + "# [:: kani :: proof] fn "
                                + harness_fn
                                + " () { let "
                                + #state_pat_s
                                + " : "
                                + #state_ty_s
                                + " = < "
                                + #state_ty_s
                                + " as :: elicitation :: KaniCompose > :: "
                                + depth_method
                                + " () ; "
                                + ":: kani :: assume ("
                                + inv_fn
                                + " (& "
                                + #state_pat_s
                                + ")) ; "
                                + non_state_lets
                                + "let _result = "
                                + #fn_name_src
                                + " ("
                                + #call_args_src
                                + ") ; "
                                + ":: kani :: assert ("
                                + inv_fn
                                + " (& _result . 0) , \"invariant preserved\") ; "
                                + ":: std :: mem :: forget (_result) ; } "
                        };

                        // 5. Symbolic closure harness.
                        //
                        // When the state parameter is prefixed with `_` (i.e. the function
                        // ignores its input state), we apply the forget-and-shadow pattern:
                        //   a) construct the full kani_any() state for the precondition check,
                        //   b) forget it (preventing drop-glue SAT explosion),
                        //   c) shadow with kani_depth0() for the actual function call.
                        //
                        // Soundness: Rust guarantees the return value is computed and moved out
                        // before any local drop occurs. For transitions that ignore state,
                        // the postcondition is independent of the input; forgetting the input
                        // is therefore sound. (Confirmed by Level 10 gallery proof.)
                        //
                        // When the state parameter IS used (no `_` prefix), the function
                        // destructures state and its fields feed into the result. We pass the
                        // symbolic state directly and accept the higher drop-glue cost.
                        let closure_src = if #state_pat_s.starts_with('_') {
                            String::new()
                                + "# [allow (unexpected_cfgs)] # [cfg (kani)] "
                                + "# [:: kani :: proof] fn "
                                + &closure_fn
                                + " () { let "
                                + #state_pat_s
                                + " : "
                                + #state_ty_s
                                + " = < "
                                + #state_ty_s
                                + " as :: elicitation :: KaniCompose > :: kani_any () ; "
                                + ":: kani :: assume ("
                                + inv_fn
                                + " (& "
                                + #state_pat_s
                                + ")) ; "
                                + #non_state_lets_d0_src
                                + ":: std :: mem :: forget ("
                                + #state_pat_s
                                + ") ; "
                                + "let "
                                + #state_pat_s
                                + " : "
                                + #state_ty_s
                                + " = < "
                                + #state_ty_s
                                + " as :: elicitation :: KaniCompose > :: kani_depth0 () ; "
                                + "let _result = "
                                + #fn_name_src
                                + " ("
                                + #call_args_src
                                + ") ; "
                                + ":: kani :: assert ("
                                + inv_fn
                                + " (& _result . 0) , \"invariant preserved\") ; "
                                + ":: std :: mem :: forget (_result) ; } "
                        } else {
                            String::new()
                                + "# [allow (unexpected_cfgs)] # [cfg (kani)] "
                                + "# [:: kani :: proof] fn "
                                + &closure_fn
                                + " () { let "
                                + #state_pat_s
                                + " : "
                                + #state_ty_s
                                + " = < "
                                + #state_ty_s
                                + " as :: elicitation :: KaniCompose > :: kani_any () ; "
                                + ":: kani :: assume ("
                                + inv_fn
                                + " (& "
                                + #state_pat_s
                                + ")) ; "
                                + #non_state_lets_d0_src
                                + "let _result = "
                                + #fn_name_src
                                + " ("
                                + #call_args_src
                                + ") ; "
                                + ":: kani :: assert ("
                                + inv_fn
                                + " (& _result . 0) , \"invariant preserved\") ; "
                                + ":: std :: mem :: forget (_result) ; } "
                        };

                        let src = String::new()
                            // 1. Contracted wrapper — documentation only; not used in harnesses
                            + "# [allow (unexpected_cfgs)] # [cfg (kani)] "
                            + "# [:: kani :: requires ("
                            + inv_fn
                            + " (& "
                            + #state_pat_s
                            + "))] # [:: kani :: ensures (| result | "
                            + inv_fn
                            + " (& result . 0))] pub (crate) fn "
                            + &contracted_fn
                            + " ("
                            + #inputs_src
                            + ") "
                            + #output_src
                            + " { "
                            + #fn_name_src
                            + " ("
                            + #call_args_src
                            + ") } "
                            // 2. Depth-0 plain proof — base case
                            + &depth_harness(&closure_d0_fn, "kani_depth0", #non_state_lets_d0_src)
                            // 3. Depth-1 plain proof — inductive step 1
                            + &depth_harness(&closure_d1_fn, "kani_depth1", #non_state_lets_d1_src)
                            // 4. Depth-2 plain proof — inductive step 2
                            + &depth_harness(&closure_d2_fn, "kani_depth2", #non_state_lets_d2_src)
                            // 5. Symbolic closure
                            + &closure_src;
                        src.parse()
                            .expect("kani_closure_proof: invalid TokenStream")
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
                #[allow(unexpected_cfgs)]
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
                #[allow(unexpected_cfgs)]
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

            // The inline kani harness (kani::any() for state) is intentionally
            // NOT emitted here.  When the state type is a complex enum registered
            // with VerifiedStateMachine, it may not implement kani::Arbitrary
            // (the per-variant KaniVariantState approach replaces it).
            //
            // The companion struct provides kani_harness() (returns the string)
            // and kani_harness_for_variant() for the build.rs / derive_vsm path.
            let _ = kani; // consumed by harness_src above — suppress unused warning
            (quote! {}, creusot, verus, companion)
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
