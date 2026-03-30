//! Derive macro implementation for `#[derive(ToCodeLiteral)]`.
//!
//! # Overview
//!
//! [`ToCodeLiteral`] is the trait that lets a value describe itself as Rust
//! source code — a `TokenStream` expression whose evaluation reconstructs the
//! original value.  The `#[elicit_tool]` macro relies on it: when auto-deriving
//! an [`EmitCode`] impl for a tool's params struct, it calls
//! `ToCodeLiteral::to_code_literal(&self.field)` for every field so the emitted
//! workflow code contains concrete literals rather than opaque runtime values.
//!
//! Writing these impls by hand is tedious for simple types and impractical for
//! complex ones.  This module provides `#[derive(ToCodeLiteral)]` which
//! auto-generates the impl for any struct or enum, including recursive types
//! like `UiNode` (whose variants contain `Vec<UiNode>`).
//!
//! # What Gets Generated
//!
//! ## Named Structs
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! struct ColorJson { r: f32, g: f32, b: f32, a: f32 }
//!
//! // Generates (simplified):
//! #[cfg(feature = "emit")]
//! impl ToCodeLiteral for ColorJson {
//!     fn to_code_literal(&self) -> TokenStream {
//!         let r = ToCodeLiteral::to_code_literal(&self.r);
//!         let g = ToCodeLiteral::to_code_literal(&self.g);
//!         let b = ToCodeLiteral::to_code_literal(&self.b);
//!         let a = ToCodeLiteral::to_code_literal(&self.a);
//!         quote! { ColorJson { r: #r, g: #g, b: #b, a: #a } }
//!     }
//!     fn type_tokens() -> TokenStream { quote! { ColorJson } }
//! }
//! ```
//!
//! ## Tuple Structs
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! struct Wrapper(String);
//! // → quote! { Wrapper(#__tcl_0) }
//! ```
//!
//! ## Unit Structs
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! struct Marker;
//! // → quote! { Marker }
//! ```
//!
//! ## Enums
//!
//! Handles all three variant forms — unit, tuple, and struct:
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! enum UiNode {
//!     Widget { widget: WidgetJson },                           // struct variant
//!     Container { container: ContainerJson, children: Vec<UiNode> }, // recursive
//!     Leaf(String),                                            // tuple variant
//!     Empty,                                                   // unit variant
//! }
//! ```
//!
//! Each variant becomes a match arm that destructures, converts each field
//! via `ToCodeLiteral`, and reassembles:
//!
//! ```text
//! UiNode::Widget { widget } => {
//!     let __tcl_widget = ToCodeLiteral::to_code_literal(widget);
//!     quote! { UiNode::Widget { widget: #__tcl_widget } }
//! }
//! UiNode::Empty => quote! { UiNode::Empty }
//! ```
//!
//! # Recursion Through Blanket Impls
//!
//! The derive does **not** need special handling for `Vec<T>`, `Option<T>`, or
//! `HashMap<String, V>`.  Those types already have blanket `ToCodeLiteral`
//! impls in `elicitation::emit_code` that delegate element-wise.  So
//! `Vec<UiNode>` works automatically as long as `UiNode` itself implements the
//! trait — which the derive provides.
//!
//! # Feature Gating
//!
//! The generated impl is always wrapped in `#[cfg(feature = "emit")]`.  This
//! means:
//!
//! - The derive attribute is **always available** (proc macros run regardless
//!   of features).
//! - The generated impl **silently vanishes** when `emit` is disabled — no
//!   compile errors, no dead code.
//! - Downstream crates need their own `emit` feature that activates
//!   `elicitation/emit` (e.g. `elicit_egui` has
//!   `emit = ["dep:quote", "elicitation/emit"]`).
//!
//! # Nested Quote Technique
//!
//! This derive faces a classic proc-macro challenge: it must generate code
//! that itself calls `::quote::quote!`.  The outer `quote!` (in the derive)
//! interprets `#ident` as interpolation, but the inner `quote!` (in the
//! generated code) also needs `#ident` markers for *its* interpolation.
//!
//! The solution: build the inner `quote!` body as raw `TokenStream` with
//! literal `#` [`Punct`] tokens, then interpolate that stream into the outer
//! `quote!`.  The outer macro copies the `#` tokens verbatim; the inner
//! `::quote::quote!` at the user's compile time interprets them as
//! interpolation points.
//!
//! ```text
//! Derive expansion time (proc macro):
//!   outer quote! { ::quote::quote! { #inner_body } }
//!                                     ↑ interpolated as TokenStream
//!
//! inner_body contains:
//!   ColorJson { r : # __tcl_r , g : # __tcl_g , ... }
//!                   ↑ literal Punct('#')
//!
//! User compile time:
//!   ::quote::quote! { ColorJson { r: #__tcl_r, g: #__tcl_g, ... } }
//!                                    ↑ now interpreted as interpolation
//! ```
//!
//! # Requirements
//!
//! Every field type must itself implement `ToCodeLiteral`.  If a field's type
//! does not, the compiler will report a trait-bound error pointing at the
//! derive — add `#[derive(ToCodeLiteral)]` to that type or provide a manual
//! impl.
//!
//! The crate using the derive must have `quote` available (typically as an
//! optional dependency gated on the `emit` feature).

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Punct, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};

/// Expand `#[derive(ToCodeLiteral)]` for a struct or enum.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let to_code_literal_body = match &input.data {
        Data::Struct(data) => gen_struct_body(name, &data.fields),
        Data::Enum(data) => gen_enum_body(name, data),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "ToCodeLiteral cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        #[cfg(feature = "emit")]
        impl #impl_generics ::elicitation::emit_code::ToCodeLiteral
            for #name #ty_generics #where_clause
        {
            fn to_code_literal(&self) -> ::elicitation::proc_macro2::TokenStream {
                #to_code_literal_body
            }

            fn type_tokens() -> ::elicitation::proc_macro2::TokenStream {
                ::quote::quote! { #name }
            }
        }
    };

    expanded.into()
}

/// Emit a literal `#` token that will be interpreted by the runtime `quote!`.
fn hash_punct() -> TokenTree {
    TokenTree::Punct(Punct::new('#', Spacing::Alone))
}

/// Build `field: #local, field2: #local2, ...` as raw tokens for inner quote body.
fn build_named_fields_pattern(
    field_names: &[&syn::Ident],
    local_vars: &[syn::Ident],
) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    for (i, (field, local)) in field_names.iter().zip(local_vars.iter()).enumerate() {
        if i > 0 {
            tokens.extend(std::iter::once(TokenTree::Punct(Punct::new(
                ',',
                Spacing::Alone,
            ))));
        }
        tokens.extend(quote! { #field : });
        tokens.extend(std::iter::once(hash_punct()));
        tokens.extend(quote! { #local });
    }
    tokens
}

/// Build `#local0, #local1, ...` as raw tokens for tuple fields in inner quote body.
fn build_tuple_fields_pattern(local_vars: &[syn::Ident]) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    for (i, local) in local_vars.iter().enumerate() {
        if i > 0 {
            tokens.extend(std::iter::once(TokenTree::Punct(Punct::new(
                ',',
                Spacing::Alone,
            ))));
        }
        tokens.extend(std::iter::once(hash_punct()));
        tokens.extend(quote! { #local });
    }
    tokens
}

/// Wrap tokens in `::quote::quote! { <inner> }`.
fn wrap_in_quote(inner: TokenStream2) -> TokenStream2 {
    let group = Group::new(Delimiter::Brace, inner);
    let mut result = quote! { ::quote::quote! };
    result.extend(std::iter::once(TokenTree::Group(group)));
    result
}

/// Generate body for a struct (named, tuple, or unit).
fn gen_struct_body(name: &syn::Ident, fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(named) => {
            let field_names: Vec<_> = named
                .named
                .iter()
                .map(|f| f.ident.as_ref().expect("named field"))
                .collect();
            let local_vars: Vec<_> = field_names
                .iter()
                .map(|f| format_ident!("__tcl_{f}"))
                .collect();

            let fields_pattern = build_named_fields_pattern(&field_names, &local_vars);
            let brace_group = Group::new(Delimiter::Brace, fields_pattern);
            let mut inner = quote! { #name };
            inner.extend(std::iter::once(TokenTree::Group(brace_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #(
                    let #local_vars =
                        ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#field_names);
                )*
                #quote_call
            }
        }
        Fields::Unnamed(unnamed) => {
            let indices: Vec<_> = (0..unnamed.unnamed.len()).map(syn::Index::from).collect();
            let local_vars: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__tcl_{i}"))
                .collect();

            let tuple_pattern = build_tuple_fields_pattern(&local_vars);
            let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
            let mut inner = quote! { #name };
            inner.extend(std::iter::once(TokenTree::Group(paren_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #(
                    let #local_vars =
                        ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#indices);
                )*
                #quote_call
            }
        }
        Fields::Unit => {
            let quote_call = wrap_in_quote(quote! { #name });
            quote! { #quote_call }
        }
    }
}

/// Generate match body for an enum.
fn gen_enum_body(name: &syn::Ident, data: &syn::DataEnum) -> TokenStream2 {
    let arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let vname = &variant.ident;
            gen_variant_arm(name, vname, &variant.fields)
        })
        .collect();

    quote! {
        match self {
            #( #arms )*
        }
    }
}

/// Generate a single match arm for an enum variant.
fn gen_variant_arm(
    enum_name: &syn::Ident,
    variant_name: &syn::Ident,
    fields: &Fields,
) -> TokenStream2 {
    match fields {
        Fields::Named(named) => {
            let field_names: Vec<_> = named
                .named
                .iter()
                .map(|f| f.ident.as_ref().expect("named field"))
                .collect();
            let local_vars: Vec<_> = field_names
                .iter()
                .map(|f| format_ident!("__tcl_{f}"))
                .collect();

            let fields_pattern = build_named_fields_pattern(&field_names, &local_vars);
            let brace_group = Group::new(Delimiter::Brace, fields_pattern);
            let mut inner = quote! { #enum_name :: #variant_name };
            inner.extend(std::iter::once(TokenTree::Group(brace_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #enum_name::#variant_name { #( #field_names ),* } => {
                    #(
                        let #local_vars =
                            ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#field_names);
                    )*
                    #quote_call
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            let bindings: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__field_{i}"))
                .collect();
            let local_vars: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__tcl_{i}"))
                .collect();

            let tuple_pattern = build_tuple_fields_pattern(&local_vars);
            let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
            let mut inner = quote! { #enum_name :: #variant_name };
            inner.extend(std::iter::once(TokenTree::Group(paren_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #enum_name::#variant_name( #( #bindings ),* ) => {
                    #(
                        let #local_vars =
                            ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#bindings);
                    )*
                    #quote_call
                }
            }
        }
        Fields::Unit => {
            let quote_call = wrap_in_quote(quote! { #enum_name :: #variant_name });
            quote! {
                #enum_name::#variant_name => {
                    #quote_call
                }
            }
        }
    }
}
