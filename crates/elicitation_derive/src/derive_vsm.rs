//! Derive macro implementation for `#[derive(VerifiedStateMachine)]`.
//!
//! Generates the `VerifiedStateMachine` impl by inferring associated types from
//! naming conventions and converting the transition list to per-variant, per-depth
//! companion struct calls.
//!
//! # Naming conventions
//!
//! Given `struct FooBarMachine`, the macro infers:
//! - `type State = FooBarState`
//! - `type Invariant = FooBarConsistent`
//!
//! Both can be overridden via `#[vsm(state = MyState, invariant = MyInvariant)]`.
//!
//! # Transition list
//!
//! Each `snake_case` name in `#[vsm(transitions = [...])]` is converted to a
//! companion struct call using `kani_harness_for_variant_at_depth`.
//!
//! The state type must implement [`KaniVariantState`] (via
//! `#[derive(KaniVariantState)]`) to supply per-variant, per-depth construction
//! expressions.  Three harnesses are generated per (transition × variant) — one at
//! each compositional depth (0/1/2) — giving CBMC a concrete discriminant and
//! bounded collection sizes for each proof.
//!
//! # Example
//!
//! ```rust,ignore
//! #[derive(VerifiedStateMachine)]
//! #[vsm(transitions = [begin_connect, disconnect, reconnect])]
//! pub struct ArchiveConnectionMachine;
//! ```
//!
//! Expands to:
//!
//! ```rust,ignore
//! impl VerifiedStateMachine for ArchiveConnectionMachine {
//!     type State     = ArchiveConnectionState;
//!     type Invariant = ArchiveConnectionConsistent;
//!
//!     fn transition_harnesses() -> Vec<::proc_macro2::TokenStream> {
//!         let mut __harnesses = Vec::new();
//!         for __vc in
//!             <ArchiveConnectionState as KaniVariantState>::kani_variant_constructions()
//!         {
//!             __harnesses.push(BeginConnectTransition::kani_harness_for_variant_at_depth(__vc.variant_name, &__vc.depth0, 0));
//!             __harnesses.push(BeginConnectTransition::kani_harness_for_variant_at_depth(__vc.variant_name, &__vc.depth1, 1));
//!             __harnesses.push(BeginConnectTransition::kani_harness_for_variant_at_depth(__vc.variant_name, &__vc.depth2, 2));
//!             // ... same for each transition
//!         }
//!         __harnesses
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    DeriveInput, Ident, Path, Token,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// ── Attribute parsing ──────────────────────────────────────────────────────────

/// Parsed content of `#[vsm(...)]`.
#[derive(Default)]
struct VsmArgs {
    /// `state = MySate` — override inferred state type.
    state: Option<Path>,
    /// `invariant = MyInvariant` — override inferred invariant type.
    invariant: Option<Path>,
    /// `transitions = [t1, t2, ...]` — required list of transition fn names.
    transitions: Vec<Ident>,
}

impl Parse for VsmArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut args = VsmArgs::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let _eq: Token![=] = input.parse()?;

            match key.to_string().as_str() {
                "state" => {
                    args.state = Some(input.parse()?);
                }
                "invariant" => {
                    args.invariant = Some(input.parse()?);
                }
                "transitions" => {
                    let content;
                    syn::bracketed!(content in input);
                    let list: Punctuated<Ident, Token![,]> =
                        content.parse_terminated(Ident::parse, Token![,])?;
                    args.transitions = list.into_iter().collect();
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "unknown vsm key `{other}`; expected `state`, `invariant`, or `transitions`"
                        ),
                    ));
                }
            }

            // Optional trailing comma between key=value pairs.
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(args)
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────────

/// Strip a `Machine` suffix from a struct name to get the common prefix.
///
/// `ArchiveConnectionMachine` → `ArchiveConnection`
fn strip_machine_suffix(name: &str) -> &str {
    name.strip_suffix("Machine").unwrap_or(name)
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

// ── Expansion ─────────────────────────────────────────────────────────────────

/// Expand `#[derive(VerifiedStateMachine)]`.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Collect `#[vsm(...)]` attributes.
    let mut vsm_args = VsmArgs::default();
    for attr in &input.attrs {
        if attr.path().is_ident("vsm") {
            let parsed: VsmArgs = match attr.parse_args() {
                Ok(a) => a,
                Err(e) => return e.to_compile_error().into(),
            };
            if parsed.state.is_some() {
                vsm_args.state = parsed.state;
            }
            if parsed.invariant.is_some() {
                vsm_args.invariant = parsed.invariant;
            }
            vsm_args.transitions.extend(parsed.transitions);
        }
    }

    // Infer State and Invariant from naming convention if not overridden.
    let struct_name_str = struct_name.to_string();
    let prefix = strip_machine_suffix(&struct_name_str);

    let state_type: Path = vsm_args.state.unwrap_or_else(|| {
        let ident = Ident::new(&format!("{prefix}State"), Span::call_site());
        syn::parse_quote!(#ident)
    });

    let invariant_type: Path = vsm_args.invariant.unwrap_or_else(|| {
        let ident = Ident::new(&format!("{prefix}Consistent"), Span::call_site());
        syn::parse_quote!(#ident)
    });

    // Build the `transition_harnesses()` body using per-variant, per-depth construction.
    //
    // For each (variant × transition × depth in [0, 1, 2]) we emit one harness,
    // giving CBMC a concrete discriminant AND bounded collection sizes.
    let harness_pushes: Vec<_> = vsm_args
        .transitions
        .iter()
        .map(|t| {
            let companion = Ident::new(
                &format!("{}Transition", to_pascal_case(&t.to_string())),
                t.span(),
            );
            quote! {
                __harnesses.push(
                    #companion::kani_harness_for_variant_at_depth(
                        __vc.variant_name,
                        &__vc.depth0,
                        0,
                    )
                );
                __harnesses.push(
                    #companion::kani_harness_for_variant_at_depth(
                        __vc.variant_name,
                        &__vc.depth1,
                        1,
                    )
                );
                __harnesses.push(
                    #companion::kani_harness_for_variant_at_depth(
                        __vc.variant_name,
                        &__vc.depth2,
                        2,
                    )
                );
            }
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics ::elicitation::contracts::VerifiedStateMachine
            for #struct_name #ty_generics
        #where_clause
        {
            type State     = #state_type;
            type Invariant = #invariant_type;

            fn transition_harnesses() -> ::std::vec::Vec<::proc_macro2::TokenStream> {
                let mut __harnesses = ::std::vec::Vec::new();
                for __vc in
                    <#state_type as ::elicitation::KaniVariantState>::kani_variant_constructions()
                {
                    #( #harness_pushes )*
                }
                __harnesses
            }
        }
    };

    expanded.into()
}
