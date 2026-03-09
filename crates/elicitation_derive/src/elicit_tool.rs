//! `#[elicit_tool]` — generate a [`ToolDescriptor`] companion from an async function.
//!
//! [`ToolDescriptor`]: elicitation::plugin::ToolDescriptor

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Error, Expr, FnArg, ItemFn, Lit, Meta, Pat, PatType, Result, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// ── Attribute args ─────────────────────────────────────────────────────────────

/// Parsed arguments from `#[elicit_tool(name = "...", description = "...", plugin = "...")]`.
struct ElicitToolArgs {
    name: String,
    description: String,
    /// Optional owning plugin name. When set, emits `inventory::submit!`.
    plugin: Option<String>,
}

impl Parse for ElicitToolArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let pairs: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut name = None;
        let mut description = None;
        let mut plugin = None;

        for meta in pairs {
            let Meta::NameValue(nv) = meta else {
                return Err(Error::new_spanned(
                    meta,
                    "expected `name = \"...\"` or `description = \"...\"`",
                ));
            };

            let key = nv
                .path
                .get_ident()
                .map(|i| i.to_string())
                .unwrap_or_default();
            let Expr::Lit(expr_lit) = &nv.value else {
                return Err(Error::new_spanned(&nv.value, "expected a string literal"));
            };
            let Lit::Str(s) = &expr_lit.lit else {
                return Err(Error::new_spanned(
                    &expr_lit.lit,
                    "expected a string literal",
                ));
            };

            match key.as_str() {
                "name" => name = Some(s.value()),
                "description" => description = Some(s.value()),
                "plugin" => plugin = Some(s.value()),
                other => {
                    return Err(Error::new_spanned(
                        &nv.path,
                        format!(
                            "unknown key `{other}`; expected `name`, `description`, or `plugin`"
                        ),
                    ));
                }
            }
        }

        Ok(ElicitToolArgs {
            name: name.ok_or_else(|| {
                Error::new(proc_macro2::Span::call_site(), "missing `name = \"...\"`")
            })?,
            description: description.ok_or_else(|| {
                Error::new(
                    proc_macro2::Span::call_site(),
                    "missing `description = \"...\"`",
                )
            })?,
            plugin,
        })
    }
}

// ── Main expansion ─────────────────────────────────────────────────────────────

/// Expand `#[elicit_tool(name = "...", description = "...")]` on an async fn.
///
/// Emits the original function unchanged, plus a companion
/// `{fn_name}_descriptor() -> elicitation::plugin::ToolDescriptor` function.
///
/// The first positional parameter of the annotated function must be the typed
/// params struct (e.g. `p: SecureFetchParams`).  That type is used as `T` in
/// `make_descriptor::<T, _>(...)`.
pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    match expand_inner(args, item) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let ElicitToolArgs {
        name,
        description,
        plugin,
    } = syn::parse2(args)?;
    let func: ItemFn = syn::parse2(item.clone())?;

    // Check if the first param is a context param (Arc<PluginContext>).
    let is_ctx_aware = first_param_is_context(&func);

    // Extract the params type: skip the context param if present.
    let params_ty = nth_params_type(&func, if is_ctx_aware { 1 } else { 0 }).ok_or_else(|| {
        Error::new_spanned(
            &func.sig,
            "#[elicit_tool] requires a typed params parameter (the params struct)",
        )
    })?;

    let fn_ident = &func.sig.ident;
    let descriptor_ident = format_ident!("{fn_ident}_descriptor");

    // Optional inventory registration when `plugin = "..."` is specified.
    let inventory_submit = plugin.map(|plugin_name| {
        quote! {
            elicitation::inventory::submit! {
                elicitation::PluginToolRegistration {
                    plugin: #plugin_name,
                    name: #name,
                    constructor: #descriptor_ident,
                }
            }
        }
    });

    // Generate the descriptor constructor, using the context-aware variant if needed.
    let descriptor_body = if is_ctx_aware {
        quote! {
            elicitation::make_descriptor_ctx::<#params_ty, _>(
                #name,
                #description,
                |ctx, p| ::std::boxed::Box::pin(#fn_ident(ctx, p)),
            )
        }
    } else {
        quote! {
            elicitation::make_descriptor::<#params_ty, _>(
                #name,
                #description,
                |p| ::std::boxed::Box::pin(#fn_ident(p)),
            )
        }
    };

    let expanded = quote! {
        #func

        /// Auto-generated [`ToolDescriptor`] constructor for [`#fn_ident`].
        ///
        /// [`ToolDescriptor`]: elicitation::plugin::ToolDescriptor
        pub fn #descriptor_ident() -> elicitation::plugin::ToolDescriptor {
            #descriptor_body
        }

        #inventory_submit
    };

    Ok(expanded)
}

/// Returns `true` if the first non-`self` parameter looks like `ctx: Arc<PluginContext>`.
///
/// Detection is name-based (`ctx`) as a heuristic; type path is also checked
/// for a segment ending in `PluginContext`.
fn first_param_is_context(func: &ItemFn) -> bool {
    let Some(first) = func.sig.inputs.iter().find_map(|arg| {
        if let FnArg::Typed(pt) = arg {
            if let Pat::Ident(p) = pt.pat.as_ref() {
                if p.ident != "self" {
                    return Some(pt);
                }
            }
        }
        None
    }) else {
        return false;
    };

    // Name heuristic: parameter named `ctx`
    if let Pat::Ident(p) = first.pat.as_ref() {
        if p.ident == "ctx" {
            return true;
        }
    }

    // Type heuristic: last path segment is `PluginContext`
    type_path_ends_with(first.ty.as_ref(), "PluginContext")
}

/// Check whether any segment in a type path ends with `name`.
fn type_path_ends_with(ty: &Type, name: &str) -> bool {
    match ty {
        Type::Path(tp) => tp.path.segments.iter().any(|seg| seg.ident == name),
        Type::Reference(r) => type_path_ends_with(&r.elem, name),
        _ => false,
    }
}

/// Pull the type from the Nth non-`self` parameter of a function signature.
fn nth_params_type(func: &ItemFn, n: usize) -> Option<&Type> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                if let Pat::Ident(p) = pat.as_ref() {
                    if p.ident != "self" {
                        return Some(ty.as_ref());
                    }
                }
            }
            None
        })
        .nth(n)
}
