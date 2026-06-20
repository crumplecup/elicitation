//! Shadow Rust trait + bridge macro generation for `#[reflect_trait]`.
//!
//! When `shadow_trait` is set, the macro generates:
//!
//! 1. A public Rust trait named after the last segment of the real trait path,
//!    with parameter and return types substituted via `type_map` (references
//!    preserved: `&OriginalType` → `&ProxyType`, `&mut OriginalType` → `&mut ProxyType`).
//!    Default method bodies from the input trait are preserved verbatim.
//!
//! 2. A `impl_<snake_trait>!($Type)` declarative macro that, for a concrete type,
//!    generates the real trait impl by bridging from the shadow trait:
//!
//!    - `&mut MappedType` param: `mem::replace` to temporarily own the value,
//!      wrap it as `ProxyType`, call the shadow method, unwrap and restore.
//!    - `&MappedType` param: `RefCast::ref_cast()` for a zero-cost shared borrow.
//!    - owned `MappedType` param: `ProxyType::from(value)`.
//!    - `MappedType` return: `OriginalType::from(result)`.
//!    - Unmapped params/returns: passed through unchanged.
//!
//! # Orphan rule
//!
//! A blanket `impl<T: ShadowTrait> ForeignTrait for T` would violate the orphan
//! rule because the implementing type `T` is an uncovered generic parameter.
//! The `impl_<snake>!` macro works around this by generating a concrete impl for
//! each named type, which is always allowed.

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, format_ident, quote};
use syn::{
    FnArg, Ident, ItemTrait, Path, ReturnType, Signature, TraitItem, TraitItemFn, Type, Visibility,
};

// ── Macro-body path helpers ───────────────────────────────────────────────────

/// Rewrite a `syn::Path` for emission inside a `macro_rules!` body so that
/// the path resolves correctly when the macro is expanded in a downstream crate
/// that does not depend on the external crates directly.
///
/// Transformation rules:
/// - `crate::Foo`         → `$crate::Foo`           (shadow-crate item)
/// - `std::…` / `core::…` / `alloc::…` → `::std::…` (always linked)
/// - Single-segment paths → unchanged (prelude, primitives, type params)
/// - `bevy::app::Plugin`  → `$crate::__bevy::app::Plugin`  (external crate)
///
/// The `$crate::__<crate>` pattern requires the defining crate to provide a
/// hidden re-export:
/// ```rust,ignore
/// #[doc(hidden)] pub use ::bevy as __bevy;
/// ```
fn path_in_macro_body(path: &Path) -> TokenStream {
    let first = &path.segments[0].ident;
    let first_str = first.to_string();

    // Single-segment, unqualified path: prelude type, primitive, or type param.
    if path.segments.len() == 1 && path.leading_colon.is_none() {
        return quote! { #path };
    }

    let rest_segs: Vec<_> = path.segments.iter().skip(1).collect();

    match first_str.as_str() {
        "crate" => crate_to_dollar_crate(quote! { #path }),
        "std" | "core" | "alloc" => quote! { ::#first #(:: #rest_segs)* },
        "self" | "super" => quote! { #path },
        _ => {
            // External crate: `bevy::app::Plugin` → `$crate::__bevy::app::Plugin`
            let hidden = format_ident!("__{}", first);
            crate_to_dollar_crate(quote! { crate::#hidden #(:: #rest_segs)* })
        }
    }
}

/// Rewrite a `syn::Type` for emission inside a `macro_rules!` body.
///
/// Delegates to [`path_in_macro_body`] for path types, recurses into
/// references, and falls back to [`crate_to_dollar_crate`] for anything else.
fn type_in_macro_body(ty: &Type) -> TokenStream {
    match ty {
        Type::Path(tp) if tp.qself.is_none() => path_in_macro_body(&tp.path),
        Type::Reference(r) => {
            let mutability = &r.mutability;
            let inner = type_in_macro_body(&r.elem);
            quote! { & #mutability #inner }
        }
        other => crate_to_dollar_crate(quote! { #other }),
    }
}

use super::naming::last_segment_str;
use super::params::MethodInfo;
use super::type_map::TypeMap;

/// Generate the shadow trait definition and bridge macro.
pub fn shadow_trait_tokens(
    input_trait: &ItemTrait,
    real_trait_path: &Path,
    type_map: &TypeMap,
    vis: &Visibility,
) -> syn::Result<TokenStream> {
    let path_str = real_trait_path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");

    let trait_name = Ident::new(last_segment_str(&path_str), Span::call_site());

    // snake-case macro name: bevy::app::Plugin → impl_bevy__app__plugin!
    let macro_name = format_ident!(
        "impl_{}",
        path_str
            .split("::")
            .map(|seg| {
                // CamelCase → snake_case
                let mut out = String::new();
                for (i, c) in seg.chars().enumerate() {
                    if c.is_uppercase() && i > 0 {
                        out.push('_');
                    }
                    out.extend(c.to_lowercase());
                }
                out
            })
            .collect::<Vec<_>>()
            .join("__")
    );

    let shadow_items = shadow_items(input_trait, type_map)?;
    let bridge_macro = bridge_macro_tokens(
        input_trait,
        real_trait_path,
        &trait_name,
        &macro_name,
        type_map,
    )?;

    let trait_doc = format!(
        "Shadow of `{path}`.\n\nImplement this trait instead of `{path}` so downstream code \
         only imports from this crate.  After implementing this trait on a concrete type, call \
         `{macro_name}!(YourType)` to generate the `{path}` impl automatically.",
        path = path_str,
        macro_name = macro_name,
    );

    Ok(quote! {
        #[doc = #trait_doc]
        #vis trait #trait_name: Send + Sync {
            #(#shadow_items)*
        }

        #bridge_macro
    })
}

// ── Shadow trait items ────────────────────────────────────────────────────────

/// Re-emit each trait method with ref-preserving `type_map` substitutions.
fn shadow_items(input_trait: &ItemTrait, type_map: &TypeMap) -> syn::Result<Vec<TokenStream>> {
    input_trait
        .items
        .iter()
        .map(|item| {
            let TraitItem::Fn(TraitItemFn {
                attrs,
                sig,
                default,
                ..
            }) = item
            else {
                return Err(syn::Error::new_spanned(
                    item,
                    "shadow_trait: trait body may only contain method signatures",
                ));
            };

            let shadow_sig = substitute_sig(sig, type_map);
            let body = match default {
                Some(block) => quote! { #block },
                None => quote! { ; },
            };

            Ok(quote! {
                #(#attrs)*
                #shadow_sig #body
            })
        })
        .collect()
}

/// Apply ref-preserving `type_map` substitutions to a method signature.
fn substitute_sig(sig: &Signature, type_map: &TypeMap) -> Signature {
    let mut out = sig.clone();
    for arg in &mut out.inputs {
        if let FnArg::Typed(pat_type) = arg {
            *pat_type.ty = type_map.apply_to_type_ref_preserving(&pat_type.ty);
        }
    }
    if let ReturnType::Type(_, ty) = &mut out.output {
        **ty = type_map.apply_to_type_ref_preserving(ty);
    }
    out
}

// ── Bridge macro ──────────────────────────────────────────────────────────────

/// Generate a `macro_rules! impl_<snake_name>` that produces the real trait impl
/// for a concrete named type.
fn bridge_macro_tokens(
    input_trait: &ItemTrait,
    real_trait_path: &Path,
    shadow_trait_name: &Ident,
    macro_name: &Ident,
    type_map: &TypeMap,
) -> syn::Result<TokenStream> {
    let methods: Vec<MethodInfo> = MethodInfo::from_trait_items(&input_trait.items)?;

    let method_impls: Vec<TokenStream> = methods
        .iter()
        .map(|m| bridge_method(m, real_trait_path, shadow_trait_name, type_map))
        .collect::<syn::Result<Vec<_>>>()?;

    let macro_doc = format!(
        "Generate `impl {real_trait} for $Type` by bridging from `{shadow}`.\n\n\
         Call this after `impl {shadow} for $Type {{ ... }}`.",
        real_trait = real_trait_path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::"),
        shadow = shadow_trait_name,
    );

    let real_trait_for_macro = path_in_macro_body(real_trait_path);

    Ok(quote! {
        #[doc = #macro_doc]
        #[macro_export]
        macro_rules! #macro_name {
            ($Type:ty) => {
                impl #real_trait_for_macro for $Type {
                    #(#method_impls)*
                }
            };
        }
    })
}

/// Generate one bridged method body (real trait → shadow trait).
fn bridge_method(
    m: &MethodInfo,
    _real_trait_path: &Path,
    shadow_trait_name: &Ident,
    type_map: &TypeMap,
) -> syn::Result<TokenStream> {
    let method_name = &m.name;

    let self_token: TokenStream = if m.has_self {
        if m.consuming_self {
            quote! { self, }
        } else {
            quote! { &self, }
        }
    } else {
        quote! {}
    };

    let real_params: Vec<TokenStream> = m
        .params
        .iter()
        .map(|p| {
            let name = &p.name;
            let ty_ts = type_in_macro_body(p.ty.as_ref());
            quote! { #name: #ty_ts }
        })
        .collect();

    let real_return: TokenStream = match &m.return_type {
        ReturnType::Default => quote! {},
        ReturnType::Type(arrow, ty) => {
            let ty_ts = type_in_macro_body(ty);
            quote! { #arrow #ty_ts }
        }
    };

    // Build per-param conversions (real type → proxy type) and shadow call args.
    let mut conversions: Vec<TokenStream> = Vec::new();
    let mut shadow_args: Vec<TokenStream> = Vec::new();

    for p in &m.params {
        let name = &p.name;
        let ty = &p.ty;

        if type_map.is_mut_ref_to_mapped(ty) {
            // &mut OriginalType → &mut ProxyType via mem::replace
            let inner_ty = inner_ref_type(ty);
            let proxy_ty = type_map
                .find_proxy(inner_ty)
                .expect("is_mut_ref_to_mapped guarantees a map entry");
            let proxy_ty_ts = crate_to_dollar_crate(quote! { #proxy_ty });
            let owned_name = format_ident!("{}_owned", name);
            conversions.push(quote! {
                let mut #owned_name = #proxy_ty_ts::from(::std::mem::take(#name));
            });
            shadow_args.push(quote! { &mut #owned_name });
        } else if type_map.is_ref_to_mapped(ty) {
            // &OriginalType → &ProxyType via RefCast (zero-cost, requires #[repr(transparent)])
            let inner_ty = inner_ref_type(ty);
            let proxy_ty = type_map
                .find_proxy(inner_ty)
                .expect("is_ref_to_mapped guarantees a map entry");
            let proxy_ty_ts = crate_to_dollar_crate(quote! { #proxy_ty });
            // Route ref_cast through the defining crate's hidden re-export so
            // downstream crates don't need ref_cast as a direct dependency.
            let ref_cast_ts = crate_to_dollar_crate(quote! { crate::__ref_cast::RefCast });
            conversions.push(quote! {
                let #name = <#proxy_ty_ts as #ref_cast_ts>::ref_cast(#name);
            });
            shadow_args.push(quote! { #name });
        } else if let Some(proxy_ty) = type_map.find_proxy(ty) {
            // Owned OriginalType → ProxyType
            let proxy_ty_ts = crate_to_dollar_crate(quote! { #proxy_ty });
            conversions.push(quote! {
                let #name = #proxy_ty_ts::from(#name);
            });
            shadow_args.push(quote! { #name });
        } else {
            // Unmapped: pass through unchanged.
            shadow_args.push(quote! { #name });
        }
    }

    // Post-call restores for &mut params.
    let restores: Vec<TokenStream> = m
        .params
        .iter()
        .filter(|p| type_map.is_mut_ref_to_mapped(p.ty.as_ref()))
        .map(|p| {
            let name = &p.name;
            let owned_name = format_ident!("{}_owned", name);
            let inner_ty = inner_ref_type(p.ty.as_ref());
            let inner_ty_ts = type_in_macro_body(inner_ty);
            quote! {
                *#name = #inner_ty_ts::from(#owned_name);
            }
        })
        .collect();

    // Build `$crate::TraitName` so the path resolves to the defining crate when
    // the macro is expanded in a downstream crate.
    let dollar_crate_trait: TokenStream = {
        use proc_macro2::{Punct, Spacing};
        let dollar = TokenTree::Punct(Punct::new('$', Spacing::Alone));
        let krate = TokenTree::Ident(proc_macro2::Ident::new("crate", Span::call_site()));
        let sep1 = TokenTree::Punct(Punct::new(':', Spacing::Joint));
        let sep2 = TokenTree::Punct(Punct::new(':', Spacing::Alone));
        let name = TokenTree::Ident(shadow_trait_name.clone());
        [dollar, krate, sep1, sep2, name].into_iter().collect()
    };

    let shadow_call = if m.has_self {
        quote! { <Self as #dollar_crate_trait>::#method_name(self, #(#shadow_args,)*) }
    } else {
        quote! { <Self as #dollar_crate_trait>::#method_name(#(#shadow_args,)*) }
    };

    let result_and_return = match &m.return_type {
        ReturnType::Default => quote! {
            #shadow_call;
            #(#restores)*
        },
        ReturnType::Type(_, ret_ty) => {
            // `ret_ty` is the original/real type from the input trait.
            // The shadow call returns the proxy type (via type_map substitution).
            // If a proxy exists, convert proxy → original via From.
            let ret_conversion = if type_map.find_proxy(ret_ty).is_some() {
                let ret_ty_ts = type_in_macro_body(ret_ty);
                quote! { #ret_ty_ts::from(__result) }
            } else {
                quote! { __result }
            };
            quote! {
                let __result = #shadow_call;
                #(#restores)*
                #ret_conversion
            }
        }
    };

    Ok(quote! {
        fn #method_name(#self_token #(#real_params,)*) #real_return {
            #(#conversions)*
            #result_and_return
        }
    })
}

fn inner_ref_type(ty: &Type) -> &Type {
    if let Type::Reference(r) = ty {
        r.elem.as_ref()
    } else {
        panic!(
            "inner_ref_type called on non-reference type: {}",
            ty.to_token_stream()
        )
    }
}

/// Replace standalone `crate` idents with `$crate` in a `TokenStream`.
///
/// Paths like `crate::App` in a `macro_rules!` body refer to the *calling*
/// crate, not the crate where the macro is defined.  Replacing `crate` with
/// `$crate` fixes this so the type always resolves to the defining crate.
fn crate_to_dollar_crate(input: TokenStream) -> TokenStream {
    use proc_macro2::{Punct, Spacing};
    input
        .into_iter()
        .flat_map(|tt| -> Vec<TokenTree> {
            match tt {
                TokenTree::Ident(ref id) if id == "crate" => {
                    vec![TokenTree::Punct(Punct::new('$', Spacing::Alone)), tt]
                }
                TokenTree::Group(g) => {
                    let new_inner = crate_to_dollar_crate(g.stream());
                    let mut new_g = proc_macro2::Group::new(g.delimiter(), new_inner);
                    new_g.set_span(g.span());
                    vec![TokenTree::Group(new_g)]
                }
                other => vec![other],
            }
        })
        .collect()
}
