//! Derive macro implementation for `#[derive(KaniVariantState)]`.
//!
//! Generates an impl of `elicitation::KaniVariantState` for a state enum,
//! providing one concrete construction expression per variant.  Used by the
//! `#[derive(VerifiedStateMachine)]` harness generator to emit per-variant
//! Kani proofs (one harness per transition × variant) instead of a single
//! symbolic-enum harness that CBMC cannot verify in bounded time.
//!
//! # Field construction rules
//!
//! | Field type | Generated expression |
//! |-----------|---------------------|
//! | `Vec<T>` | `Vec::new()` |
//! | `String` | `String::new()` |
//! | `Option<T>` | `None` |
//! | anything else | `kani::any()` |
//!
//! These choices match the manual `impl kani::Arbitrary` patterns that existed
//! before this derive was introduced, and avoid the symbolic-length destructor
//! problem described in `elicitation::verification::kani::kani_vec`.

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields};

/// Convert a PascalCase or camelCase identifier to snake_case.
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

/// Returns `true` if `ty` is `Vec<_>` (any form: `Vec`, `std::vec::Vec`).
fn is_vec_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(tp) = ty else {
        return false;
    };
    let segs = &tp.path.segments;
    match segs.len() {
        1 => segs[0].ident == "Vec",
        3 => segs[0].ident == "std" && segs[1].ident == "vec" && segs[2].ident == "Vec",
        _ => false,
    }
}

/// Returns `true` if `ty` is `String` (any form: `String`, `std::string::String`).
fn is_string_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(tp) = ty else {
        return false;
    };
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

/// Returns `true` if `ty` is `Option<_>` (any form: `Option`, `std::option::Option`).
fn is_option_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(tp) = ty else {
        return false;
    };
    let segs = &tp.path.segments;
    match segs.len() {
        1 => segs[0].ident == "Option",
        3 => {
            segs[0].ident == "std"
                && segs[1].ident == "option"
                && segs[2].ident == "Option"
        }
        _ => false,
    }
}

/// Generate a concrete construction expression for a single field.
///
/// Produces a token string (with spaces between tokens, matching
/// `proc_macro2::TokenStream::to_string()` format) suitable for embedding
/// in a generated Kani harness source string.
fn field_construction_expr(ty: &syn::Type) -> &'static str {
    if is_vec_type(ty) {
        ":: std :: vec :: Vec :: new ()"
    } else if is_string_type(ty) {
        ":: std :: string :: String :: new ()"
    } else if is_option_type(ty) {
        // Always use None for Option<T>: avoids kani::any() on T which may
        // not implement kani::Arbitrary (e.g. Option<String>, Option<Vec<T>>).
        "None"
    } else {
        ":: kani :: any ()"
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let ident_str = ident.to_string();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let data_enum = match &input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "KaniVariantState can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    // Build static string pairs: ("variant_snake", "Ident :: Variant { ... }")
    let mut entries: Vec<proc_macro2::TokenStream> = Vec::new();
    for variant in &data_enum.variants {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        let variant_snake = to_snake_case(&variant_str);

        let expr_str: String = match &variant.fields {
            Fields::Unit => {
                format!("{ident_str} :: {variant_str}")
            }
            Fields::Named(named) => {
                let field_strs: Vec<String> = named
                    .named
                    .iter()
                    .map(|f| {
                        let fname = f.ident.as_ref().unwrap().to_string();
                        let fexpr = field_construction_expr(&f.ty);
                        format!("{fname} : {fexpr}")
                    })
                    .collect();
                format!(
                    "{ident_str} :: {variant_str} {{ {} }}",
                    field_strs.join(" , ")
                )
            }
            Fields::Unnamed(unnamed) => {
                let field_strs: Vec<&str> = unnamed
                    .unnamed
                    .iter()
                    .map(|f| field_construction_expr(&f.ty))
                    .collect();
                format!(
                    "{ident_str} :: {variant_str} ({})",
                    field_strs.join(" , ")
                )
            }
        };

        entries.push(quote::quote! {
            (#variant_snake, #expr_str)
        });
    }

    quote::quote! {
        impl #impl_generics ::elicitation::KaniVariantState
            for #ident #ty_generics
            #where_clause
        {
            fn kani_variant_constructions() -> ::std::vec::Vec<(&'static str, &'static str)> {
                vec![ #(#entries),* ]
            }
        }
    }
    .into()
}
