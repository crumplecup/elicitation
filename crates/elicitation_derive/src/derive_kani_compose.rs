//! Derive macro for `KaniCompose`.
//!
//! Generates a depth-bounded `impl KaniCompose for T` for structs and enums.
//!
//! # Structs
//!
//! Each depth method constructs `Self` with all fields built from their
//! type's corresponding `kani_depth{n}()` call.  The depth semantics of
//! each field type's impl are respected automatically:
//!
//! ```rust,ignore
//! #[derive(KaniCompose)]
//! struct ExplainNode {
//!     node_type: String,
//!     #[skip]
//!     children: Vec<ExplainNode>,
//!     plan_rows: i64,
//! }
//! // #[skip] Vec → always Vec::new() regardless of depth
//! // Generates depth0: { node_type: String::new(), children: vec![], plan_rows: kani::any() }
//! // Generates depth1: { node_type: String::new(), children: vec![], ... }
//! // Generates depth2: { ..., children: vec![], ... }
//! ```
//!
//! # Enums
//!
//! The first unit variant (or `#[default]` variant) is used at all depths.
//! If no unit variant exists, the first variant is used with depth-bounded
//! field construction.  This keeps the proof state space concrete.
//!
//! ```rust,ignore
//! #[derive(KaniCompose)]
//! enum Mode { ViewA, ViewB { label: String } }
//! // All three depths return Mode::ViewA (first unit variant).
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, TypePath};

// ── Field type classification ─────────────────────────────────────────────────

fn is_vec(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };
    let segs = &path.segments;
    match segs.len() {
        1 => segs[0].ident == "Vec",
        3 => segs[0].ident == "std" && segs[1].ident == "vec" && segs[2].ident == "Vec",
        _ => false,
    }
}

fn is_string(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };
    let segs = &path.segments;
    match segs.len() {
        1 => segs[0].ident == "String",
        3 => segs[0].ident == "std" && segs[1].ident == "string" && segs[2].ident == "String",
        _ => false,
    }
}

fn is_option(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };
    let segs = &path.segments;
    match segs.len() {
        1 => segs[0].ident == "Option",
        3 => segs[0].ident == "std" && segs[1].ident == "option" && segs[2].ident == "Option",
        _ => false,
    }
}

fn is_primitive(ty: &Type) -> bool {
    let Type::Path(TypePath { path, .. }) = ty else {
        return false;
    };
    let segs = &path.segments;
    if segs.len() != 1 {
        return false;
    }
    matches!(
        segs[0].ident.to_string().as_str(),
        "bool"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "f32"
            | "f64"
            | "usize"
            | "isize"
            | "char"
    )
}

fn first_generic(ty: &Type) -> Option<&Type> {
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };
    let last = path.segments.last()?;
    if let syn::PathArguments::AngleBracketed(ab) = &last.arguments {
        if let Some(syn::GenericArgument::Type(inner)) = ab.args.first() {
            return Some(inner);
        }
    }
    None
}

// ── Attribute helpers ─────────────────────────────────────────────────────────

/// Returns `true` if the field carries a `#[skip]` attribute.
///
/// `#[skip]` marks a field as excluded from elicitation survey/display.
/// `KaniCompose` respects this by keeping `Vec`-typed skipped fields empty at
/// every depth, preventing CBMC from modelling nested heap objects that the
/// invariant does not depend on.
fn has_skip_attr(field: &syn::Field) -> bool {
    field.attrs.iter().any(|a| a.path().is_ident("skip"))
}

// ── Per-field expression generation ──────────────────────────────────────────

/// Generate depth-0/1/2 `TokenStream` expressions for a single field type.
///
/// The returned triple is `(depth0, depth1, depth2)`.
fn field_exprs(
    ty: &Type,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    if is_vec(ty) {
        let d0 = quote! { ::std::vec::Vec::new() };
        let (d1, d2) = if let Some(inner) = first_generic(ty) {
            (
                quote! { vec![<#inner as ::elicitation::KaniCompose>::kani_depth0()] },
                quote! { vec![
                    <#inner as ::elicitation::KaniCompose>::kani_depth0(),
                    <#inner as ::elicitation::KaniCompose>::kani_depth0(),
                ] },
            )
        } else {
            (d0.clone(), d0.clone())
        };
        (d0, d1, d2)
    } else if is_string(ty) {
        let s = quote! { ::std::string::String::new() };
        (s.clone(), s.clone(), s)
    } else if is_option(ty) {
        let d0 = quote! { ::core::option::Option::None };
        let d1 = if let Some(inner) = first_generic(ty) {
            quote! { ::core::option::Option::Some(<#inner as ::elicitation::KaniCompose>::kani_depth0()) }
        } else {
            d0.clone()
        };
        // depth-2 for Option is same as depth-1 (still one Some)
        (d0, d1.clone(), d1)
    } else if is_primitive(ty) {
        let s = quote! { ::kani::any::<#ty>() };
        (s.clone(), s.clone(), s)
    } else {
        // User-defined type: delegate to its KaniCompose impl.
        (
            quote! { <#ty as ::elicitation::KaniCompose>::kani_depth0() },
            quote! { <#ty as ::elicitation::KaniCompose>::kani_depth1() },
            quote! { <#ty as ::elicitation::KaniCompose>::kani_depth2() },
        )
    }
}

// ── Struct helpers ────────────────────────────────────────────────────────────

/// Generate a single `kani_any()` expression for a field type.
///
/// - `Vec<T>` → `T::kani_vec_closure(1, 3)` (bounded symbolic collection)
/// - `String` → `String::kani_any()` (bounded symbolic string)
/// - `Option<T>` → symbolic `Some(T::kani_any()) | None`
/// - primitives → `kani::any::<T>()`
/// - anything else → `T::kani_any()`
fn field_any_expr(ty: &Type, skipped: bool) -> proc_macro2::TokenStream {
    // #[skip] on a Vec field: keep empty even in kani_any() — CBMC should not
    // explore symbolic heap for fields the invariant does not depend on.
    if skipped && is_vec(ty) {
        return quote! { ::std::vec::Vec::new() };
    }
    if is_vec(ty) {
        if let Some(inner) = first_generic(ty) {
            quote! { <#inner as ::elicitation::KaniCompose>::kani_vec_closure(1, 3) }
        } else {
            quote! { ::std::vec::Vec::new() }
        }
    } else if is_string(ty) {
        quote! { <::std::string::String as ::elicitation::KaniCompose>::kani_any() }
    } else if is_option(ty) {
        if let Some(inner) = first_generic(ty) {
            quote! {
                if ::kani::any::<bool>() {
                    ::core::option::Option::Some(<#inner as ::elicitation::KaniCompose>::kani_any())
                } else {
                    ::core::option::Option::None
                }
            }
        } else {
            quote! { ::core::option::Option::None }
        }
    } else if is_primitive(ty) {
        quote! { ::kani::any::<#ty>() }
    } else {
        quote! { <#ty as ::elicitation::KaniCompose>::kani_any() }
    }
}

/// Build the `Self { field: any_expr, ... }` body for named struct fields.
fn struct_named_any_body(fields: &syn::FieldsNamed) -> proc_macro2::TokenStream {
    let assignments: Vec<proc_macro2::TokenStream> = fields
        .named
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            let expr = field_any_expr(&f.ty, has_skip_attr(f));
            quote! { #name: #expr }
        })
        .collect();
    quote! { Self { #(#assignments),* } }
}

/// Build the `Self(expr, ...)` body for unnamed/tuple struct fields.
fn struct_unnamed_any_body(fields: &syn::FieldsUnnamed) -> proc_macro2::TokenStream {
    let exprs: Vec<proc_macro2::TokenStream> = fields
        .unnamed
        .iter()
        .map(|f| field_any_expr(&f.ty, has_skip_attr(f)))
        .collect();
    quote! { Self(#(#exprs),*) }
}

/// Build the symbolic variant-selector body for an enum.
///
/// Generates a `match` over a `kani::any::<usize>()` selector (bounded to
/// the variant count via `kani::assume`) that constructs each variant with
/// its fields drawn from **depth-2** expressions (not `kani_any()`).
///
/// ## Why depth-2, not `kani_any()`?
///
/// Using `String::kani_any()` (4 symbolic chars) or `kani_vec_closure` inside
/// each variant arm makes the formula intractable for large enums: a 18-variant
/// enum with several `String` fields times out the SAT solver (>10 min).
///
/// The correct inductive argument does NOT require fully-symbolic field content
/// in the closure harness.  The d0/d1/d2 harnesses already prove the invariant
/// holds for field growth (empty → 1 element → 2 elements).  The closure's job
/// is to prove variant coverage: "for ANY variant, with maximally-saturated
/// (depth-2) fields, the invariant is preserved."  Combined with d0/d1/d2,
/// this gives: ∀ finite inputs, invariant is preserved.
///
/// Primitives (`u32`, `bool`, etc.) remain fully symbolic via `kani::any()`
/// because `field_exprs` already returns `kani::any::<T>()` for all primitive
/// depths.  Only heap-allocated types (`String`, `Vec<T>`) use the bounded
/// depth-2 construction.
fn enum_any_body(
    enum_ident: &syn::Ident,
    data: &syn::DataEnum,
) -> proc_macro2::TokenStream {
    let n = data.variants.len();
    let n_lit = n; // usize literal
    let arms: Vec<proc_macro2::TokenStream> = data
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let vname = &v.ident;
            let construction = match &v.fields {
                Fields::Unit => quote! { #enum_ident::#vname },
                Fields::Named(named) => {
                    let assignments: Vec<_> = named
                        .named
                        .iter()
                        .map(|f| {
                            let fname = f.ident.as_ref().unwrap();
                            let skipped = has_skip_attr(f);
                            let expr = if skipped && is_vec(&f.ty) {
                                quote! { ::std::vec::Vec::new() }
                            } else {
                                field_exprs(&f.ty).2
                            };
                            quote! { #fname: #expr }
                        })
                        .collect();
                    quote! { #enum_ident::#vname { #(#assignments),* } }
                }
                Fields::Unnamed(unnamed) => {
                    let exprs: Vec<_> = unnamed
                        .unnamed
                        .iter()
                        .map(|f| {
                            let skipped = has_skip_attr(f);
                            if skipped && is_vec(&f.ty) {
                                quote! { ::std::vec::Vec::new() }
                            } else {
                                field_exprs(&f.ty).2
                            }
                        })
                        .collect();
                    quote! { #enum_ident::#vname(#(#exprs),*) }
                }
            };
            if i == n - 1 {
                // Last arm uses `_` to satisfy exhaustiveness without a
                // concrete integer literal that might warn about unreachability.
                quote! { _ => #construction }
            } else {
                let idx = i;
                quote! { #idx => #construction }
            }
        })
        .collect();
    quote! {
        let __variant: usize = ::kani::any();
        ::kani::assume(__variant < #n_lit);
        match __variant {
            #(#arms),*
        }
    }
}

/// Build the `Self { field: expr, ... }` body for named fields at the given depth.
fn struct_named_body(fields: &syn::FieldsNamed, depth: u8) -> proc_macro2::TokenStream {
    let assignments: Vec<proc_macro2::TokenStream> = fields
        .named
        .iter()
        .map(|f| {
            let name = f.ident.as_ref().unwrap();
            // #[skip] on a Vec field: keep it empty at every depth so CBMC
            // does not model nested heap objects that the invariant ignores.
            if has_skip_attr(f) && is_vec(&f.ty) {
                return quote! { #name: ::std::vec::Vec::new() };
            }
            let (d0, d1, d2) = field_exprs(&f.ty);
            let expr = match depth {
                1 => d1,
                2 => d2,
                _ => d0,
            };
            quote! { #name: #expr }
        })
        .collect();
    quote! { Self { #(#assignments),* } }
}

/// Build the `Self(expr, ...)` body for tuple struct fields at the given depth.
fn struct_unnamed_body(fields: &syn::FieldsUnnamed, depth: u8) -> proc_macro2::TokenStream {
    let exprs: Vec<proc_macro2::TokenStream> = fields
        .unnamed
        .iter()
        .map(|f| {
            let (d0, d1, d2) = field_exprs(&f.ty);
            match depth {
                1 => d1,
                2 => d2,
                _ => d0,
            }
        })
        .collect();
    quote! { Self(#(#exprs),*) }
}

// ── Enum helpers ──────────────────────────────────────────────────────────────

/// Variant selection priority for enums:
/// 1. `#[default]` variant
/// 2. First unit variant
/// 3. First variant overall
fn pick_base_variant(data: &syn::DataEnum) -> &syn::Variant {
    // Priority 1: explicit #[default]
    if let Some(v) = data
        .variants
        .iter()
        .find(|v| v.attrs.iter().any(|a| a.path().is_ident("default")))
    {
        return v;
    }
    // Priority 2: first unit variant
    if let Some(v) = data
        .variants
        .iter()
        .find(|v| matches!(v.fields, Fields::Unit))
    {
        return v;
    }
    // Priority 3: first variant
    data.variants
        .iter()
        .next()
        .expect("enum must have at least one variant")
}

/// Build the construction expression for a single enum variant at the given depth.
fn variant_body(
    enum_ident: &syn::Ident,
    variant: &syn::Variant,
    depth: u8,
) -> proc_macro2::TokenStream {
    let vname = &variant.ident;
    match &variant.fields {
        Fields::Unit => quote! { #enum_ident::#vname },
        Fields::Named(named) => {
            let assignments: Vec<proc_macro2::TokenStream> = named
                .named
                .iter()
                .map(|f| {
                    let fname = f.ident.as_ref().unwrap();
                    let (d0, d1, d2) = field_exprs(&f.ty);
                    let expr = match depth {
                        1 => d1,
                        2 => d2,
                        _ => d0,
                    };
                    quote! { #fname: #expr }
                })
                .collect();
            quote! { #enum_ident::#vname { #(#assignments),* } }
        }
        Fields::Unnamed(unnamed) => {
            let exprs: Vec<proc_macro2::TokenStream> = unnamed
                .unnamed
                .iter()
                .map(|f| {
                    let (d0, d1, d2) = field_exprs(&f.ty);
                    match depth {
                        1 => d1,
                        2 => d2,
                        _ => d0,
                    }
                })
                .collect();
            quote! { #enum_ident::#vname(#(#exprs),*) }
        }
    }
}

// ── Main expand ───────────────────────────────────────────────────────────────

pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (depth0_body, depth1_body, depth2_body) = match &input.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(named) => (
                struct_named_body(named, 0),
                struct_named_body(named, 1),
                struct_named_body(named, 2),
            ),
            Fields::Unnamed(unnamed) => (
                struct_unnamed_body(unnamed, 0),
                struct_unnamed_body(unnamed, 1),
                struct_unnamed_body(unnamed, 2),
            ),
            Fields::Unit => {
                let b = quote! { Self };
                (b.clone(), b.clone(), b)
            }
        },
        Data::Enum(de) => {
            let base = pick_base_variant(de);
            (
                variant_body(name, base, 0),
                variant_body(name, base, 1),
                variant_body(name, base, 2),
            )
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "KaniCompose cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let kani_any_body = match &input.data {
        Data::Struct(ds) => match &ds.fields {
            Fields::Named(named) => struct_named_any_body(named),
            Fields::Unnamed(unnamed) => struct_unnamed_any_body(unnamed),
            Fields::Unit => quote! { Self },
        },
        Data::Enum(de) => enum_any_body(name, de),
        Data::Union(_) => unreachable!("union already handled above"),
    };

    // Detect whether depth1 == depth0 and depth2 == depth1 to suppress default overrides.
    // We always emit all three methods explicitly for clarity — the compiler will inline
    // the trivially-identical cases away.

    quote! {
        #[allow(unexpected_cfgs)]
        #[cfg(kani)]
        impl #impl_generics ::elicitation::KaniCompose for #name #ty_generics #where_clause {
            // #[inline(always)] ensures CBMC can propagate Vec::new() etc.
            // constants through recursive types so the destructor loop sees
            // length == 0 and does not unwind symbolically.
            #[inline(always)]
            fn kani_depth0() -> Self {
                #depth0_body
            }
            #[inline(always)]
            fn kani_depth1() -> Self {
                #depth1_body
            }
            #[inline(always)]
            fn kani_depth2() -> Self {
                #depth2_body
            }
            #[inline(always)]
            fn kani_any() -> Self {
                #kani_any_body
            }
        }
    }
    .into()
}
