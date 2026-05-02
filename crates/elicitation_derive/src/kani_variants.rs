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
        3 => segs[0].ident == "std" && segs[1].ident == "string" && segs[2].ident == "String",
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
        3 => segs[0].ident == "std" && segs[1].ident == "option" && segs[2].ident == "Option",
        _ => false,
    }
}

/// Extract the first generic type argument from a path type (e.g. `T` from `Vec<T>`).
fn extract_first_generic(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(tp) = ty else {
        return None;
    };
    let last = tp.path.segments.last()?;
    if let syn::PathArguments::AngleBracketed(ab) = &last.arguments
        && let Some(syn::GenericArgument::Type(inner)) = ab.args.first()
    {
        return Some(inner);
    }
    None
}

/// Returns `true` if `ty` is a primitive numeric or bool type.
fn is_primitive_type(ty: &syn::Type) -> bool {
    let syn::Type::Path(tp) = ty else {
        return false;
    };
    let segs = &tp.path.segments;
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

/// Generate construction expressions at depth 0, 1, and 2 for a single field type.
///
/// Returns `(depth0_expr, depth1_expr, depth2_expr)` as Strings suitable for
/// embedding verbatim in a Kani harness source string.
///
/// # Rules
///
/// | Field type          | depth-0              | depth-1                              | depth-2                    |
/// |---------------------|----------------------|--------------------------------------|----------------------------|
/// | `Vec<T>`            | `Vec::new()`         | `vec![<T as KaniCompose>::kani_depth0()]` | two elements          |
/// | `String`            | `String::new()`      | same                                 | same                       |
/// | `Option<T>`         | `None`               | `Some(<T as KaniCompose>::kani_depth0())` | same as depth-1       |
/// | primitive           | `kani::any::<T>()`   | same                                 | same                       |
/// | any other `T`       | `<T as KaniCompose>::kani_depth0()` | `kani_depth1()`     | `kani_depth2()`            |
fn field_construction_exprs(ty: &syn::Type) -> (String, String, String) {
    let ty_str = quote::quote!(#ty).to_string();

    if is_vec_type(ty) {
        let depth0 = ":: std :: vec :: Vec :: new ()".to_string();
        let (depth1, depth2) = if let Some(inner) = extract_first_generic(ty) {
            let inner_str = quote::quote!(#inner).to_string();
            let elem = format!(
                "< {} as :: elicitation :: KaniCompose > :: kani_depth0 ()",
                inner_str
            );
            let d1 = format!("vec ! [{}]", elem);
            let d2 = format!("vec ! [{} , {}]", elem, elem);
            (d1, d2)
        } else {
            (depth0.clone(), depth0.clone())
        };
        (depth0, depth1, depth2)
    } else if is_string_type(ty) {
        let s = ":: std :: string :: String :: new ()".to_string();
        (s.clone(), s.clone(), s)
    } else if is_option_type(ty) {
        let depth0 = ":: core :: option :: Option :: None".to_string();
        let depth1 = if let Some(inner) = extract_first_generic(ty) {
            let inner_str = quote::quote!(#inner).to_string();
            format!(
                ":: core :: option :: Option :: Some (< {} as :: elicitation :: KaniCompose > :: kani_depth0 ())",
                inner_str
            )
        } else {
            depth0.clone()
        };
        // depth-2 for Option: same as depth-1 (still just one Some)
        (depth0, depth1.clone(), depth1)
    } else if is_primitive_type(ty) {
        let s = format!(":: kani :: any :: < {} > ()", ty_str);
        (s.clone(), s.clone(), s)
    } else {
        // User-defined type: delegate to KaniCompose at each depth.
        let d0 = format!(
            "< {} as :: elicitation :: KaniCompose > :: kani_depth0 ()",
            ty_str
        );
        let d1 = format!(
            "< {} as :: elicitation :: KaniCompose > :: kani_depth1 ()",
            ty_str
        );
        let d2 = format!(
            "< {} as :: elicitation :: KaniCompose > :: kani_depth2 ()",
            ty_str
        );
        (d0, d1, d2)
    }
}

/// Build the full variant construction expression at a given depth.
fn variant_expr(ident_str: &str, variant_str: &str, fields: &syn::Fields, depth: u8) -> String {
    match fields {
        Fields::Unit => format!("{ident_str} :: {variant_str}"),
        Fields::Named(named) => {
            let parts: Vec<String> = named
                .named
                .iter()
                .map(|f| {
                    let fname = f.ident.as_ref().unwrap().to_string();
                    let (d0, d1, d2) = field_construction_exprs(&f.ty);
                    let fexpr = match depth {
                        1 => d1,
                        2 => d2,
                        _ => d0,
                    };
                    format!("{fname} : {fexpr}")
                })
                .collect();
            format!("{ident_str} :: {variant_str} {{ {} }}", parts.join(" , "))
        }
        Fields::Unnamed(unnamed) => {
            let parts: Vec<String> = unnamed
                .unnamed
                .iter()
                .map(|f| {
                    let (d0, d1, d2) = field_construction_exprs(&f.ty);
                    match depth {
                        1 => d1,
                        2 => d2,
                        _ => d0,
                    }
                })
                .collect();
            format!("{ident_str} :: {variant_str} ({})", parts.join(" , "))
        }
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

    // Build KaniVariantConstruction entries for each variant.
    let mut entries: Vec<proc_macro2::TokenStream> = Vec::new();
    for variant in &data_enum.variants {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        let variant_snake = to_snake_case(&variant_str);

        let depth0 = variant_expr(&ident_str, &variant_str, &variant.fields, 0);
        let depth1 = variant_expr(&ident_str, &variant_str, &variant.fields, 1);
        let depth2 = variant_expr(&ident_str, &variant_str, &variant.fields, 2);

        entries.push(quote::quote! {
            ::elicitation::KaniVariantConstruction {
                variant_name: #variant_snake,
                depth0: #depth0 .to_string(),
                depth1: #depth1 .to_string(),
                depth2: #depth2 .to_string(),
            }
        });
    }

    quote::quote! {
        impl #impl_generics ::elicitation::KaniVariantState
            for #ident #ty_generics
            #where_clause
        {
            fn kani_variant_constructions() -> ::std::vec::Vec<::elicitation::KaniVariantConstruction> {
                vec![ #(#entries),* ]
            }
        }
    }
    .into()
}
