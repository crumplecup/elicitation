//! Creusot shadow-workspace preparation.
//!
//! The regular `cargo creusot` flow compiles the entire target crate, not just
//! generated proof companions. User crates often contain runtime-only
//! instrumentation and integration derives (`tracing::instrument`, serde,
//! schemars, derive_builder, derive_more::Display/Error) that are harmless in
//! normal builds but hostile to Creusot translation.
//!
//! This module creates a sanitized shadow copy of a workspace under
//! `.creusot-shadow/`, preserving relative path dependencies while gating those
//! proof-hostile surfaces away from the Creusot build.

use anyhow::Context as _;
use proc_macro2::Span;
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit_mut::{self, VisitMut};
use walkdir::{DirEntry, WalkDir};

/// Create or refresh a sanitized `.creusot-shadow` workspace copy.
pub fn prepare_shadow_workspace(workspace_root: &Path) -> anyhow::Result<PathBuf> {
    let workspace_name = workspace_root
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("workspace root must have a terminal directory name"))?;
    let shadow_root = workspace_root
        .parent()
        .ok_or_else(|| anyhow::anyhow!("workspace root must have a parent directory"))?
        .join(format!("{workspace_name}-creusot-shadow"));
    if shadow_root.exists() {
        fs::remove_dir_all(&shadow_root).with_context(|| {
            format!(
                "failed to remove stale shadow workspace {}",
                shadow_root.display()
            )
        })?;
    }
    fs::create_dir_all(&shadow_root).with_context(|| {
        format!(
            "failed to create shadow workspace {}",
            shadow_root.display()
        )
    })?;

    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_entry(|entry| should_descend(entry, &shadow_root))
    {
        let entry = entry?;
        let src_path = entry.path();
        if src_path == workspace_root {
            continue;
        }

        let rel = src_path.strip_prefix(workspace_root).with_context(|| {
            format!(
                "failed to compute relative path for {} from {}",
                src_path.display(),
                workspace_root.display()
            )
        })?;
        let dst_path = shadow_root.join(rel);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dst_path)
                .with_context(|| format!("failed to create {}", dst_path.display()))?;
            continue;
        }

        if entry.file_type().is_file() {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            if src_path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                let source = fs::read_to_string(src_path)
                    .with_context(|| format!("failed to read {}", src_path.display()))?;
                let sanitized = sanitize_rust_source(&source)
                    .with_context(|| format!("failed to sanitize {}", src_path.display()))?;
                fs::write(&dst_path, sanitized)
                    .with_context(|| format!("failed to write {}", dst_path.display()))?;
            } else {
                fs::copy(src_path, &dst_path).with_context(|| {
                    format!(
                        "failed to copy {} to {}",
                        src_path.display(),
                        dst_path.display()
                    )
                })?;
            }
        }
    }

    Ok(shadow_root)
}

/// Rewrite a Rust source file so Creusot sees a tracing/serde-free surface.
pub fn sanitize_rust_source(source: &str) -> anyhow::Result<String> {
    let mut file = syn::parse_file(source).context("failed to parse Rust source")?;
    let mut sanitizer = CreusotSanitizer;
    sanitizer.visit_file_mut(&mut file);
    Ok(prettyplease::unparse(&file))
}

fn should_descend(entry: &DirEntry, shadow_root: &Path) -> bool {
    let path = entry.path();
    if path == shadow_root {
        return false;
    }
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return true;
    };
    !matches!(name, ".git" | "target" | "verif" | ".creusot-shadow")
}

struct CreusotSanitizer;

#[derive(Clone, Copy, Default)]
struct AttrContext {
    gate_eq_like: bool,
}

impl VisitMut for CreusotSanitizer {
    fn visit_item_struct_mut(&mut self, item: &mut syn::ItemStruct) {
        sanitize_attrs(
            &mut item.attrs,
            AttrContext {
                gate_eq_like: item
                    .fields
                    .iter()
                    .any(|field| type_is_float(&field.ty) || type_blocks_deep_model(&field.ty)),
            },
        );
        if struct_needs_deep_model(item) && !has_deep_model_derive(&item.attrs) {
            make_fields_public(&mut item.fields);
            item.attrs.push(
                syn::parse_quote!(#[cfg_attr(creusot, derive(creusot_std::model::DeepModel))]),
            );
        }
        visit_mut::visit_item_struct_mut(self, item);
    }

    fn visit_item_enum_mut(&mut self, item: &mut syn::ItemEnum) {
        let has_float_fields = item_enum_has_float_fields(item);
        sanitize_attrs(
            &mut item.attrs,
            AttrContext {
                gate_eq_like: has_float_fields,
            },
        );
        if !has_float_fields && enum_needs_deep_model(item) && !has_deep_model_derive(&item.attrs) {
            item.attrs.push(
                syn::parse_quote!(#[cfg_attr(creusot, derive(creusot_std::model::DeepModel))]),
            );
        }
        visit_mut::visit_item_enum_mut(self, item);
    }

    fn visit_item_fn_mut(&mut self, item: &mut syn::ItemFn) {
        sanitize_attrs(&mut item.attrs, AttrContext::default());
        if item.sig.ident == "main" {
            let original = item.block.stmts.clone();
            item.block = syn::parse_quote!({
                #[cfg(creusot)]
                {
                    return;
                }
                #[cfg(not(creusot))]
                {
                    #(#original)*
                }
            });
        }
        if block_uses_runtime_parse_helper(&item.block) && !has_cfg_not_creusot(&item.attrs) {
            item.attrs.push(syn::parse_quote!(#[cfg(not(creusot))]));
        }
        visit_mut::visit_item_fn_mut(self, item);
    }

    fn visit_impl_item_fn_mut(&mut self, item: &mut syn::ImplItemFn) {
        sanitize_attrs(&mut item.attrs, AttrContext::default());
        if block_uses_runtime_parse_helper(&item.block) && !has_cfg_not_creusot(&item.attrs) {
            item.attrs.push(syn::parse_quote!(#[cfg(not(creusot))]));
        }
        visit_mut::visit_impl_item_fn_mut(self, item);
    }

    fn visit_stmt_mut(&mut self, stmt: &mut syn::Stmt) {
        if let syn::Stmt::Macro(stmt_macro) = stmt
            && is_tracing_stmt_macro(&stmt_macro.mac)
            && !has_cfg_not_creusot(&stmt_macro.attrs)
        {
            stmt_macro
                .attrs
                .push(syn::parse_quote!(#[cfg(not(creusot))]));
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_expr_mut(&mut self, expr: &mut syn::Expr) {
        if let syn::Expr::Macro(node) = expr {
            if is_tracing_stmt_macro(&node.mac) {
                *expr = syn::parse_quote!(());
                visit_mut::visit_expr_mut(self, expr);
                return;
            }
            if let Some(rewritten) = rewrite_vec_macro_expr(node) {
                *expr = rewritten;
                visit_mut::visit_expr_mut(self, expr);
                return;
            }
            if is_panic_like_macro(&node.mac) {
                node.mac.tokens = proc_macro2::TokenStream::new();
            }
        }
        visit_mut::visit_expr_mut(self, expr);
    }
}

fn sanitize_attrs(attrs: &mut Vec<syn::Attribute>, context: AttrContext) {
    let had_display_attr = attrs.iter().any(is_display_attr);
    let mut out = Vec::with_capacity(attrs.len() + 1);
    for attr in attrs.drain(..) {
        if is_instrument_attr(&attr) {
            let meta = attr.meta.clone();
            out.push(syn::parse_quote!(#[cfg_attr(not(creusot), #meta)]));
            continue;
        }
        if is_display_attr(&attr) {
            let meta = attr.meta.clone();
            out.push(syn::parse_quote!(#[cfg_attr(not(creusot), #meta)]));
            continue;
        }
        if attr.path().is_ident("derive")
            && let Ok(paths) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
        {
            let (hostile, safe): (Vec<_>, Vec<_>) = paths.into_iter().partition(|path| {
                is_creusot_hostile_derive(path, had_display_attr, context.gate_eq_like)
            });
            if !safe.is_empty() {
                out.push(syn::parse_quote!(#[derive(#(#safe),*)]));
            }
            if !hostile.is_empty() {
                out.push(syn::parse_quote!(#[cfg_attr(not(creusot), derive(#(#hostile),*))]));
            }
            continue;
        }
        out.push(attr);
    }
    *attrs = out;
}

fn is_instrument_attr(attr: &syn::Attribute) -> bool {
    attr.path()
        .segments
        .last()
        .map(|segment| segment.ident == "instrument")
        .unwrap_or(false)
}

fn is_tracing_stmt_macro(mac: &syn::Macro) -> bool {
    mac.path
        .segments
        .last()
        .map(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "trace" | "debug" | "info" | "warn" | "error" | "event" | "span"
            )
        })
        .unwrap_or(false)
}

fn is_panic_like_macro(mac: &syn::Macro) -> bool {
    mac.path
        .segments
        .last()
        .map(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "panic" | "todo" | "unimplemented" | "unreachable"
            )
        })
        .unwrap_or(false)
}

fn is_vec_macro(mac: &syn::Macro) -> bool {
    mac.path
        .segments
        .last()
        .map(|segment| segment.ident == "vec")
        .unwrap_or(false)
}

fn has_cfg_not_creusot(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("cfg")
            && attr
                .meta
                .to_token_stream()
                .to_string()
                .contains("not ( creusot )")
    })
}

fn is_display_attr(attr: &syn::Attribute) -> bool {
    attr.path()
        .segments
        .last()
        .map(|segment| segment.ident == "display")
        .unwrap_or(false)
}

fn is_creusot_hostile_derive(path: &syn::Path, had_display_attr: bool, gate_eq_like: bool) -> bool {
    path.segments
        .last()
        .map(|segment| {
            matches!(
                segment.ident.to_string().as_str(),
                "Serialize"
                    | "Deserialize"
                    | "JsonSchema"
                    | "Builder"
                    | "FromStr"
                    | "EnumIter"
                    | "PartialOrd"
                    | "Ord"
            ) || (gate_eq_like
                && matches!(
                    segment.ident.to_string().as_str(),
                    "PartialEq" | "Eq" | "Hash"
                ))
                || (had_display_attr && segment.ident == "Display")
        })
        .unwrap_or(false)
}

fn item_enum_has_float_fields(item: &syn::ItemEnum) -> bool {
    item.variants
        .iter()
        .flat_map(|variant| variant.fields.iter())
        .any(|field| type_is_float(&field.ty))
}

fn type_is_float(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| matches!(segment.ident.to_string().as_str(), "f32" | "f64"))
            .unwrap_or(false),
        syn::Type::Group(group) => type_is_float(&group.elem),
        syn::Type::Paren(paren) => type_is_float(&paren.elem),
        _ => false,
    }
}

fn enum_needs_deep_model(item: &syn::ItemEnum) -> bool {
    item.variants
        .iter()
        .all(|variant| variant.fields.is_empty())
        || has_eq_like_derive(&item.attrs)
}

fn struct_needs_deep_model(item: &syn::ItemStruct) -> bool {
    has_eq_like_derive(&item.attrs)
        && !item
            .fields
            .iter()
            .any(|field| type_is_float(&field.ty) || type_blocks_deep_model(&field.ty))
}

fn make_fields_public(fields: &mut syn::Fields) {
    match fields {
        syn::Fields::Named(named) => {
            for field in &mut named.named {
                field.vis = syn::Visibility::Public(syn::token::Pub::default());
            }
        }
        syn::Fields::Unnamed(unnamed) => {
            for field in &mut unnamed.unnamed {
                field.vis = syn::Visibility::Public(syn::token::Pub::default());
            }
        }
        syn::Fields::Unit => {}
    }
}

fn has_eq_like_derive(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path().is_ident("derive")
            && attr
                .parse_args_with(
                    syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
                )
                .map(|paths| {
                    paths.iter().any(|path| {
                        path.segments
                            .last()
                            .map(|seg| {
                                matches!(
                                    seg.ident.to_string().as_str(),
                                    "PartialEq" | "Eq" | "Hash"
                                )
                            })
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
    })
}

fn type_blocks_deep_model(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path.path.segments.iter().any(|segment| {
            segment.ident == "Established"
                || angle_bracketed_args_block_deep_model(&segment.arguments)
        }),
        syn::Type::Reference(reference) => type_blocks_deep_model(&reference.elem),
        syn::Type::Group(group) => type_blocks_deep_model(&group.elem),
        syn::Type::Paren(paren) => type_blocks_deep_model(&paren.elem),
        syn::Type::Tuple(tuple) => tuple.elems.iter().any(type_blocks_deep_model),
        _ => false,
    }
}

fn angle_bracketed_args_block_deep_model(args: &syn::PathArguments) -> bool {
    match args {
        syn::PathArguments::AngleBracketed(angle) => angle.args.iter().any(|arg| match arg {
            syn::GenericArgument::Type(ty) => type_blocks_deep_model(ty),
            _ => false,
        }),
        _ => false,
    }
}

fn rewrite_vec_macro_expr(node: &syn::ExprMacro) -> Option<syn::Expr> {
    if !is_vec_macro(&node.mac) {
        return None;
    }

    let elems = node
        .mac
        .parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
        .ok()?;

    let binding = syn::Ident::new("__elicitation_shadow_vec", Span::call_site());
    let pushes: Vec<syn::Stmt> = elems
        .into_iter()
        .map(|expr| syn::parse_quote!(#binding.push(#expr);))
        .collect();

    Some(syn::parse_quote!({
        let mut #binding = ::std::vec::Vec::new();
        #(#pushes)*
        #binding
    }))
}

fn block_uses_runtime_parse_helper(block: &syn::Block) -> bool {
    let tokens = block.to_token_stream().to_string();
    tokens.contains("from_str") || tokens.contains("to_lowercase")
}

fn has_deep_model_derive(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.meta
            .to_token_stream()
            .to_string()
            .contains("DeepModel")
    })
}

use quote::ToTokens;
