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

    // Extract the params type from the first parameter.
    let params_ty = first_param_type(&func).ok_or_else(|| {
        Error::new_spanned(
            &func.sig,
            "#[elicit_tool] requires at least one typed parameter (the params struct)",
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

    let expanded = quote! {
        #func

        /// Auto-generated [`ToolDescriptor`] constructor for [`#fn_ident`].
        ///
        /// [`ToolDescriptor`]: elicitation::plugin::ToolDescriptor
        pub fn #descriptor_ident() -> elicitation::plugin::ToolDescriptor {
            elicitation::make_descriptor::<#params_ty, _>(
                #name,
                #description,
                |p| ::std::boxed::Box::pin(#fn_ident(p)),
            )
        }

        #inventory_submit
    };

    Ok(expanded)
}

/// Pull the type from the first non-`self` parameter of a function signature.
fn first_param_type(func: &ItemFn) -> Option<&Type> {
    for arg in &func.sig.inputs {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            // Skip if it's a `self` pattern somehow typed
            if let Pat::Ident(p) = pat.as_ref() {
                if p.ident == "self" {
                    continue;
                }
            }
            return Some(ty.as_ref());
        }
    }
    None
}
