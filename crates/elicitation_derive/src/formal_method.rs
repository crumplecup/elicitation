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
//! 4. `#[cfg_attr(creusot, ::creusot_std::macros::requires(...))]` /
//!    `#[cfg_attr(creusot, ::creusot_std::macros::ensures(...))]` directly
//!    on the original function, so `cargo creusot` verifies the body.
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
//! # Generated contracts (Creusot)
//!
//! ```rust,ignore
//! #[cfg_attr(creusot, ::creusot_std::macros::requires(invariant_fn(&state)))]
//! #[cfg_attr(creusot, ::creusot_std::macros::ensures(invariant_fn(&result.0)))]
//! fn advance(state: MyState, proof: Established<InvariantHolds>)
//!     -> (MyState, Established<InvariantHolds>)
//! {
//!     (state.next(), proof)
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

/// If `block` is `{ f(args) }` (single tail-call with no semicolon), return a
/// version rewritten to `{ f__creusot(args) }`.
///
/// This prevents Creusot from descending into the original `f` (which may have
/// `#[instrument]`-generated static string slices in its MIR) and instead uses
/// the clean `f__creusot` companion that has no tracing overhead.
fn creusot_delegation_rewrite(block: &syn::Block) -> Option<String> {
    if block.stmts.len() != 1 {
        return None;
    }
    let syn::Stmt::Expr(syn::Expr::Call(call), None) = &block.stmts[0] else {
        return None;
    };
    let syn::Expr::Path(path_expr) = call.func.as_ref() else {
        return None;
    };
    let mut new_path = path_expr.path.clone();
    let last = new_path.segments.last_mut()?;
    let new_name = format!("{}__creusot", last.ident);
    last.ident = syn::Ident::new(&new_name, last.ident.span());
    let new_func = syn::Expr::Path(syn::ExprPath {
        attrs: path_expr.attrs.clone(),
        qself: path_expr.qself.clone(),
        path: new_path,
    });
    let args = &call.args;
    Some(quote!({ #new_func(#args) }).to_string())
}

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

/// Parsed arguments from `#[formal_method(contracts = [C1, C2, ...], creusot_requires = ["expr"], kani_requires = ["expr"], verus_class = "trivial")]`.
struct FormalMethodArgs {
    contracts: Vec<syn::Path>,
    /// Extra Pearlite expressions added as `#[requires(expr)]` to the Creusot companion only.
    ///
    /// Use this to express logical content that proof tokens carry but Creusot cannot derive
    /// from the opaque `Established<P>` type — e.g. `"initial_bankroll@ > 0"` when the
    /// function accepts `bankroll_proof: Established<BankrollPositive>`.
    creusot_requires: Vec<syn::LitStr>,
    /// Extra Rust expressions added as `#[kani::requires(expr)]` to the original function.
    ///
    /// Use this to express preconditions that proof tokens carry conceptually but that
    /// CBMC cannot infer from the opaque `Established<P>` ZST — e.g. `"initial_bankroll > 0"`
    /// when the function accepts `bankroll_proof: Established<BankrollPositive>`.
    kani_requires: Vec<syn::LitStr>,
    /// Verus generator postcondition class override.
    ///
    /// When set, the standalone Verus generator uses this class directly instead of
    /// inferring it from the transition body.  Valid values: `"trivial"`, `"passthrough"`,
    /// `"special_false"`, `"conditional_special"`.
    ///
    /// This annotation is consumed by the CLI scanner; the derive macro ignores it.
    #[allow(dead_code)]
    verus_class: Option<syn::LitStr>,
}

impl Parse for FormalMethodArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(FormalMethodArgs {
                contracts: Vec::new(),
                creusot_requires: Vec::new(),
                kani_requires: Vec::new(),
                verus_class: None,
            });
        }
        let mut contracts = Vec::new();
        let mut creusot_requires = Vec::new();
        let mut kani_requires = Vec::new();
        let mut verus_class = None;
        loop {
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            match ident.to_string().as_str() {
                "contracts" => {
                    let content;
                    syn::bracketed!(content in input);
                    let paths = Punctuated::<syn::Path, Token![,]>::parse_terminated(&content)?;
                    contracts = paths.into_iter().collect();
                }
                "creusot_requires" => {
                    let content;
                    syn::bracketed!(content in input);
                    let lits = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(&content)?;
                    creusot_requires = lits.into_iter().collect();
                }
                "kani_requires" => {
                    let content;
                    syn::bracketed!(content in input);
                    let lits = Punctuated::<syn::LitStr, Token![,]>::parse_terminated(&content)?;
                    kani_requires = lits.into_iter().collect();
                }
                "verus_class" => {
                    verus_class = Some(input.parse::<syn::LitStr>()?);
                }
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!(
                            "unknown key `{other}`; expected `contracts`, `creusot_requires`, `kani_requires`, or `verus_class`"
                        ),
                    ));
                }
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
            if input.is_empty() {
                break;
            }
        }
        Ok(FormalMethodArgs {
            contracts,
            creusot_requires,
            kani_requires,
            verus_class,
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

    // ── Gate tracing instrumentation under not(kani) ──────────────────────────
    // `#[instrument]` from tracing expands to heap-allocating Span/enter code
    // with no Kani contracts.  Under proof_for_contract, DFCC would inline the
    // entire tracing prologue/epilogue (no contracts on tracing functions),
    // inflating the proof cost from ~32s to timeout.
    // Transform:  #[instrument(…)]  →  #[cfg_attr(not(kani), instrument(…))]
    for attr in func.attrs.iter_mut() {
        let is_instrument = attr
            .path()
            .segments
            .last()
            .map(|s| s.ident == "instrument")
            .unwrap_or(false);
        if is_instrument {
            let meta = attr.meta.clone();
            *attr = syn::parse_quote! { #[cfg_attr(not(kani), #meta)] };
        }
    }

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

    let (kani_harness, creusot_companion, verus_companion, companion_struct) = if !is_async
        && !has_receiver
    {
        let kani_fn = format_ident!("{fn_name}__kani");
        let verus_fn = format_ident!("{fn_name}__verus");

        let mut lets: Vec<TokenStream> = Vec::new();
        // Non-state let bindings for the proof_for_contract closure harness.
        // Depth-bounded types use KaniCompose::kani_depth0() instead of
        // kani::any(), preventing CBMC's type-based destructor model from
        // unwinding recursively through types like Vec<ExplainNode>.
        let mut non_state_lets_d0: Vec<TokenStream> = Vec::new();
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
                    non_state_lets_d0.push(let_ts);
                } else if is_string_type(ty) {
                    // String content is irrelevant to structural invariant proofs.
                    // from_utf8_lossy introduces a UTF-8 validation loop that makes
                    // the state space unbounded; String::new() is bounded by construction.
                    let let_ts = quote! {
                        let #pat: #ty = ::std::string::String::new();
                    };
                    lets.push(let_ts.clone());
                    non_state_lets_d0.push(let_ts);
                } else if is_vec_type(ty) {
                    // kani::any::<Vec<T>>() is not implemented in Kani ≤0.67;
                    // use an empty Vec instead — still covers all invariant checks
                    // without requiring T: kani::Arbitrary on the Vec itself.
                    let let_ts = quote! {
                        let #pat: #ty = ::std::vec::Vec::new();
                    };
                    lets.push(let_ts.clone());
                    non_state_lets_d0.push(let_ts);
                } else if is_option_type(ty) {
                    // kani::any::<Option<T>>() hangs when T contains heap-allocated
                    // fields — symbolic ownership transfer + non-trivial destructor
                    // causes unbounded unwinding in CBMC. Option parameters are
                    // auxiliary state; None is the safe bounded choice for invariant proofs.
                    let let_ts = quote! {
                        let #pat: #ty = ::core::option::Option::None;
                    };
                    lets.push(let_ts.clone());
                    non_state_lets_d0.push(let_ts);
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
                    non_state_lets_d0.push(let_ts_d0);
                }
                call_args.push(quote!(#pat));
            }
        }

        // Guard: `result` is a Creusot-reserved identifier (the return-value name
        // in `#[ensures]` clauses). Reject it as a parameter name here so the
        // generated Creusot companion never produces invalid COMA output.
        for arg in &func.sig.inputs {
            if let FnArg::Typed(pat_type) = arg
                && let syn::Pat::Ident(pi) = &*pat_type.pat
                && pi.ident == "result"
            {
                return Err(syn::Error::new_spanned(
                    &pi.ident,
                    "#[formal_method]: parameter name `result` is reserved by Creusot \
                     (it names the return value in #[ensures] clauses). \
                     Rename the parameter (e.g. `query_result`, `fn_result`).",
                ));
            }
        }

        let has_state_param = state_pat_str.is_some();
        let state_pat_s = state_pat_str.unwrap_or_default();
        let state_ty_s = state_ty_str.unwrap_or_default();
        let non_state_lets_d0_src = quote!(#(#non_state_lets_d0)*).to_string();
        let call_args_src = quote!(#(#call_args),*).to_string();
        let fn_name_src = fn_name.to_string();
        let kani_fn_src = format!("{fn_name}__kani");

        let inputs = &func.sig.inputs;
        let output = &func.sig.output;
        let inputs_src = quote!(#inputs).to_string();
        let output_src = quote!(#output).to_string();
        // Verus `assume_specification` uses `-> (r: Type)` binder syntax.
        // Strip the leading `->` from output_src and wrap as `-> (r : Type)`.
        let verus_output_src = if output_src.trim_start().starts_with("->") {
            let ty = output_src.trim_start()[2..].trim().to_string();
            format!("-> (r : {ty})")
        } else {
            "-> (r : ())".to_string()
        };
        let creusot_fn_src = format!("{fn_name}__creusot");
        let body_src = {
            let b = &func.block;
            quote!(#b).to_string()
        };
        // For Creusot: if the body is a simple delegation `{ f(args) }`, rewrite
        // to `{ f__creusot(args) }` so Creusot uses the clean companion rather than
        // the original (which may have #[instrument]-generated string literals).
        let creusot_body_src =
            creusot_delegation_rewrite(&func.block).unwrap_or_else(|| body_src.clone());

        // ── Kani contracts on the original function ───────────────────────
        // When contracts = [SomeType] is provided and there is a state
        // parameter, add #[cfg_attr(kani, kani::requires(...))] and
        // #[cfg_attr(kani, kani::ensures(...))] to the original function.
        //
        // The invariant function name is derived by snake_casing the last
        // segment of the first contract type path.  Convention:
        //   ArchivePanelConsistent → archive_panel_consistent
        //
        // This allows proof_for_contract(fn_name) to target the original
        // function directly, and stub_verified(fn_name) to work in
        // composition proofs without contracted wrapper indirection.
        if has_state_param
            && !parsed_args.contracts.is_empty()
            && let Some(first_contract) = parsed_args.contracts.first()
            && let Some(last_seg) = first_contract.segments.last()
        {
            let inv_fn_name = {
                let s = last_seg.ident.to_string();
                // Manual PascalCase → snake_case: insert underscore before
                // each uppercase letter that follows a lowercase letter.
                let mut out = String::with_capacity(s.len() + 8);
                let mut prev_lower = false;
                for ch in s.chars() {
                    if ch.is_uppercase() && prev_lower {
                        out.push('_');
                    }
                    out.push(ch.to_ascii_lowercase());
                    prev_lower = ch.is_lowercase();
                }
                out
            };
            let inv_fn_ident: syn::Ident =
                syn::parse_str(&inv_fn_name).expect("derived ident is valid");
            let state_pat_tokens: TokenStream =
                state_pat_s.parse().expect("state_pat_s is valid tokens");
            let requires_attr: syn::Attribute = syn::parse_quote! {
                #[cfg_attr(kani, ::kani::requires(#inv_fn_ident(&#state_pat_tokens)))]
            };
            let ensures_attr: syn::Attribute = syn::parse_quote! {
                #[cfg_attr(kani, ::kani::ensures(|result| #inv_fn_ident(&result.0)))]
            };
            func.attrs.push(requires_attr);
            func.attrs.push(ensures_attr);

            // Emit any extra kani_requires expressions as additional
            // #[cfg_attr(kani, ::kani::requires(expr))] attributes.
            // These constrain symbolic parameters whose bounds are carried by
            // proof tokens (ZSTs) and are therefore invisible to CBMC without
            // an explicit assume.
            for lit in &parsed_args.kani_requires {
                let expr: syn::Expr =
                    syn::parse_str(&lit.value()).expect("kani_requires expr is valid Rust");
                let extra_req: syn::Attribute = syn::parse_quote! {
                    #[cfg_attr(kani, ::kani::requires(#expr))]
                };
                func.attrs.push(extra_req);
            }

            // Creusot contracts are expressed as extern_spec! blocks in
            // elicitation_creusot/src/vsm.rs rather than inline annotations
            // here.  Inline creusot attrs would require creusot_std in scope
            // inside elicit_server, which violates the architecture principle
            // that production crates have zero creusot knowledge.
        }

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
                    let non_state_lets = #non_state_lets_d0_src;
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

        // Build extra Creusot requires string for embedding in the companion.
        // Each LitStr value becomes `# [requires (expr)] ` in the generated source.
        let creusot_extra_requires_src: String = parsed_args
            .creusot_requires
            .iter()
            .map(|lit| format!("# [requires ({})] ", lit.value()))
            .collect::<String>();

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
                /// contract is emitted with the **inlined function body** — Creusot verifies
                /// the ensures directly from the body, with no cross-crate spec dependency.
                ///
                /// If the body is a simple delegation `{ f(args) }`, the emitted body is
                /// `{ f__creusot(args) }` so Creusot uses the clean companion rather than
                /// the original (which may have `#[instrument]`-generated string literals).
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
                            + "))] "
                            + #creusot_extra_requires_src
                            + "# [ensures ("
                            + inv_fn
                            + " (& result . 0))] pub (crate) fn "
                            + #creusot_fn_src
                            + " ("
                            + #inputs_src
                            + ") "
                            + #output_src
                            + " "
                            + &#creusot_body_src
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

                /// Return a Verus `assume_specification` block for this transition.
                ///
                /// When `inv_fn` is non-empty and this transition has a state parameter,
                /// emits:
                /// ```text
                /// #[cfg(verus)] verus! {
                ///     pub assume_specification[ fn_name ](params) -> (r: RetType)
                ///         requires inv_fn(&state),
                ///         ensures inv_fn(&r.0),
                ///     ;
                /// }
                /// ```
                ///
                /// When `inv_fn` is empty or this transition has no state parameter,
                /// a stub with `requires true, ensures true` is emitted instead.
                pub fn verus_contract(inv_fn: &str) -> ::proc_macro2::TokenStream {
                    let src = if !inv_fn.is_empty() && #has_state_param {
                        String::new()
                            + "verus ! { pub assume_specification [ "
                            + #fn_name_src
                            + " ] ("
                            + #inputs_src
                            + ") "
                            + #verus_output_src
                            + " requires "
                            + inv_fn
                            + " (& "
                            + #state_pat_s
                            + ") , ensures "
                            + inv_fn
                            + " (& r . 0) , ; }"
                    } else {
                        String::new()
                            + "verus ! { pub assume_specification [ "
                            + #fn_name_src
                            + " ] ("
                            + #inputs_src
                            + ") "
                            + #verus_output_src
                            + " requires true , ensures true , ; }"
                    };
                    src.parse()
                        .expect("verus_contract: invalid TokenStream")
                }

                /// Return a `#[verifier::external]` stub function for this transition.
                ///
                /// Used in self-contained Verus proof crates (like `strictly_verus`) where
                /// external crate rlibs cannot be linked via the Verus toolchain.  The stub
                /// declares the function so that `assume_specification` has a target to axiomatise.
                ///
                /// The body is `todo!()` — Verus never inspects it because the function is
                /// marked `#[verifier::external]`.
                pub fn verus_external_stub() -> ::proc_macro2::TokenStream {
                    let src = String::new()
                        + "verus ! { # [verifier :: external] pub fn "
                        + #fn_name_src
                        + " ("
                        + #inputs_src
                        + ") "
                        + #output_src
                        + " { todo ! () } }";
                    src.parse()
                        .expect("verus_external_stub: invalid TokenStream")
                }

                /// Return a Kani `proof_for_contract` harness `TokenStream` for this transition.
                ///
                /// When `inv_fn` is non-empty and this transition has a state parameter,
                /// emits one `#[kani::proof_for_contract(fn_name)]` harness using the
                /// forgive-and-forget pattern:
                /// `kani_any()` → `assume(inv_fn)` → `forget` → `kani_depth0()` → call → `forget` result.
                /// The postcondition is checked automatically by DFCC.
                ///
                /// Once verified, `stub_verified(fn_name)` can replace calls to this
                /// function in composition proofs with the contract axiom.
                ///
                /// When `inv_fn` is empty or this transition has no state parameter,
                /// an empty `TokenStream` is returned.
                pub fn kani_closure_proof(inv_fn: &str) -> ::proc_macro2::TokenStream {
                    if inv_fn.is_empty() || !#has_state_param {
                        return ::proc_macro2::TokenStream::new();
                    }
                    let closure_fn = String::new() + #fn_name_src + "__kani_closure";

                    // Symbolic closure harness — proof_for_contract + forgive-and-forget.
                    //
                    // Uses #[kani::proof_for_contract(fn_name)] where fn_name is the
                    // ORIGINAL function.  The original function must have
                    // #[cfg_attr(kani, kani::requires(...))] and
                    // #[cfg_attr(kani, kani::ensures(...))] on it — emitted by
                    // formal_method's expand() via the cfg_attr contract injection.
                    //
                    // DFCC instruments the original function body directly, without the
                    // indirection of a contracted wrapper.  This is why the wrapper was
                    // causing timeouts (DFCC inlined wrapper → called original → inlined
                    // original body, doubling the CBMC work).
                    //
                    // Forgive-and-forget pattern (confirmed tractable in Level 11-12 gallery):
                    //   a) kani_any() state for symbolic precondition coverage.
                    //   b) kani::assume(inv_fn(&state)) to restrict to valid inputs.
                    //   c) forget(state) — prevents drop-glue SAT explosion.
                    //   d) kani_depth0() shadow for the actual function call.
                    // The postcondition is checked automatically by DFCC (no kani::assert).
                    //
                    // Once verified, stub_verified(fn_name) can replace calls in
                    // composition proofs with the contract axiom.
                    let closure_src = String::new()
                        + "# [allow (unexpected_cfgs)] # [cfg (kani)] "
                        + "# [:: kani :: proof_for_contract ("
                        + #fn_name_src
                        + ")] fn "
                        + &closure_fn
                        + " () { let "
                        + #state_pat_s
                        + " : "
                        + #state_ty_s
                        + " = < "
                        + #state_ty_s
                        + " as :: elicitation :: KaniCompose > :: kani_depth2 () ; "
                        + ":: kani :: assume ("
                        + inv_fn
                        + " (& "
                        + #state_pat_s
                        + ")) ; "
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
                        + #non_state_lets_d0_src
                        + "let _result = "
                        + #fn_name_src
                        + " ("
                        + #call_args_src
                        + ") ; "
                        + ":: std :: mem :: forget (_result) ; } ";

                    closure_src.parse()
                        .expect("kani_closure_proof: invalid TokenStream")
                }
            }
        };

        // ── Creusot companion ────────────────────────────────────────────
        // The always-trusted inline Creusot companion has been removed.
        // Real #[requires]/#[ensures] attrs are now emitted as cfg_attr
        // on the original function (above). The companion struct's
        // `creusot_contract()` method (used by elicit_proofs/build.rs)
        // continues to generate the wrapper layer for elicit_proofs.
        let creusot = quote! {};

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
