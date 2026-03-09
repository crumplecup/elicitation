//! `#[elicit_tool]` — generate a [`ToolDescriptor`] companion from an async function.
//!
//! [`ToolDescriptor`]: elicitation::plugin::ToolDescriptor

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Error, Expr, FnArg, ItemFn, Lit, LitStr, Meta, Pat, PatType, Path, Result, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

// ── Attribute args ─────────────────────────────────────────────────────────────

/// Helper to parse the inner tokens of `emit_ctx("lhs" => "rhs")`.
struct EmitCtxPair {
    lhs: LitStr,
    rhs: LitStr,
}

impl Parse for EmitCtxPair {
    fn parse(input: ParseStream) -> Result<Self> {
        let lhs: LitStr = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let rhs: LitStr = input.parse()?;
        Ok(EmitCtxPair { lhs, rhs })
    }
}

/// Parsed arguments from `#[elicit_tool(name = "...", description = "...", plugin = "...",
/// emit_ctx("ctx.field" => "replacement_expr"))]`.
struct ElicitToolArgs {
    name: String,
    description: String,
    /// Optional owning plugin name. When set, emits `inventory::submit!`.
    plugin: Option<String>,
    /// How to generate `impl EmitCode`. Default: auto-derive from handler body.
    emit: EmitMode,
    /// Context substitutions: `("ctx.field", "replacement_expr_source")`.
    emit_ctx_subs: Vec<(String, String)>,
}

/// Controls how `impl EmitCode` is generated for a handler.
enum EmitMode {
    /// Auto-derive by rewriting the handler body (default).
    Auto,
    /// Delegate to a user-supplied type implementing `CustomEmit<Params>`.
    Custom(Path),
}

impl Parse for ElicitToolArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let pairs: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut name = None;
        let mut description = None;
        let mut plugin = None;
        let mut emit: EmitMode = EmitMode::Auto;
        let mut emit_ctx_subs = Vec::new();

        for meta in pairs {
            match &meta {
                Meta::List(ml) => {
                    let key = ml
                        .path
                        .get_ident()
                        .map(|i| i.to_string())
                        .unwrap_or_default();
                    if key == "emit_ctx" {
                        let pair: EmitCtxPair = syn::parse2(ml.tokens.clone())?;
                        emit_ctx_subs.push((pair.lhs.value(), pair.rhs.value()));
                    } else {
                        return Err(Error::new_spanned(
                            &ml.path,
                            format!("unknown list attribute `{key}`; expected `emit_ctx(...)`"),
                        ));
                    }
                }
                Meta::NameValue(nv) => {
                    let key = nv
                        .path
                        .get_ident()
                        .map(|i| i.to_string())
                        .unwrap_or_default();

                    match key.as_str() {
                        "name" => {
                            let Expr::Lit(expr_lit) = &nv.value else {
                                return Err(Error::new_spanned(
                                    &nv.value,
                                    "expected a string literal",
                                ));
                            };
                            let Lit::Str(s) = &expr_lit.lit else {
                                return Err(Error::new_spanned(
                                    &expr_lit.lit,
                                    "expected a string literal",
                                ));
                            };
                            name = Some(s.value());
                        }
                        "description" => {
                            let Expr::Lit(expr_lit) = &nv.value else {
                                return Err(Error::new_spanned(
                                    &nv.value,
                                    "expected a string literal",
                                ));
                            };
                            let Lit::Str(s) = &expr_lit.lit else {
                                return Err(Error::new_spanned(
                                    &expr_lit.lit,
                                    "expected a string literal",
                                ));
                            };
                            description = Some(s.value());
                        }
                        "plugin" => {
                            let Expr::Lit(expr_lit) = &nv.value else {
                                return Err(Error::new_spanned(
                                    &nv.value,
                                    "expected a string literal",
                                ));
                            };
                            let Lit::Str(s) = &expr_lit.lit else {
                                return Err(Error::new_spanned(
                                    &expr_lit.lit,
                                    "expected a string literal",
                                ));
                            };
                            plugin = Some(s.value());
                        }
                        "emit" => {
                            match &nv.value {
                                // emit = false / emit = true → both rejected
                                Expr::Lit(expr_lit) => {
                                    let msg = match &expr_lit.lit {
                                        Lit::Bool(b) if !b.value() => {
                                            "cannot opt out with `emit = false`; \
                                             provide `emit = T` where T: CustomEmit<Params>, \
                                             or fix the handler body so auto-derive works"
                                        }
                                        Lit::Bool(_) => {
                                            "redundant `emit = true`; auto-derive is the default, \
                                             just remove the `emit` attribute"
                                        }
                                        _ => {
                                            "expected a type path for `emit`, e.g. `emit = MyEmit`"
                                        }
                                    };
                                    return Err(Error::new_spanned(&nv.value, msg));
                                }
                                // emit = some::Type
                                Expr::Path(p) => {
                                    emit = EmitMode::Custom(p.path.clone());
                                }
                                other => {
                                    return Err(Error::new_spanned(
                                        other,
                                        "expected a type path for `emit`, e.g. `emit = MyEmit`",
                                    ));
                                }
                            }
                        }
                        other => {
                            return Err(Error::new_spanned(
                                &nv.path,
                                format!(
                                    "unknown key `{other}`; expected `name`, `description`, \
                                     `plugin`, `emit`, or `emit_ctx(...)`"
                                ),
                            ));
                        }
                    }
                }
                other => {
                    return Err(Error::new_spanned(
                        other,
                        "expected `name = \"...\"`, `description = \"...\"`, or `emit_ctx(...)`",
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
            emit,
            emit_ctx_subs,
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
        emit,
        emit_ctx_subs,
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

    let mut expanded = quote! {
        #func

        /// Auto-generated [`ToolDescriptor`] constructor for [`#fn_ident`].
        ///
        /// [`ToolDescriptor`]: elicitation::plugin::ToolDescriptor
        pub fn #descriptor_ident() -> elicitation::plugin::ToolDescriptor {
            #descriptor_body
        }

        #inventory_submit
    };

    // Phase 4: generate `impl EmitCode` — auto-derive or custom delegation.
    match emit {
        EmitMode::Auto => {
            use crate::emit_rewriter::EmitRewriter;
            use quote::ToTokens as _;

            // Collect and rewrite the function body tokens.
            let body_ts: TokenStream = func
                .block
                .stmts
                .iter()
                .map(|s| s.to_token_stream())
                .collect();
            let mut rewriter = EmitRewriter::new(emit_ctx_subs);
            let rewritten = rewriter.rewrite(body_ts);

            // Generate `let __field = ToCodeLiteral::to_code_literal(&self.field);` bindings.
            let field_bindings: Vec<_> = rewriter
                .param_fields
                .iter()
                .map(|f| {
                    let local = format_ident!("__{f}");
                    let field = format_ident!("{f}");
                    quote! {
                        let #local =
                            elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#field);
                    }
                })
                .collect();

            // Infer crate deps from path prefixes in the rewritten body AND from
            // emit_ctx substitution values (e.g. "reqwest::Client::new()").
            let sub_ts: TokenStream = rewriter
                .ctx_subs
                .iter()
                .flat_map(|(_, v)| v.clone())
                .collect();
            let mut crate_names = EmitRewriter::infer_crate_names(&rewritten);
            crate_names.extend(EmitRewriter::infer_crate_names(&sub_ts));
            let elicitation_version = EmitRewriter::resolve_workspace_version("elicitation")
                .unwrap_or_else(|| "0.0".to_string());
            // The crate defining this handler — its types are used bare (without prefix),
            // so infer_crate_names can't detect them; include it explicitly.
            let own_crate = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
            let own_crate_version = EmitRewriter::resolve_workspace_version(&own_crate)
                .unwrap_or_else(|| "0.0".to_string());
            let mut crate_deps: Vec<_> = vec![
                // Always required: ToCodeLiteral impls emit elicitation:: paths at runtime.
                quote! { elicitation::emit_code::CrateDep::new("elicitation", #elicitation_version) },
                // Own crate — handler uses its types unqualified.
                quote! { elicitation::emit_code::CrateDep::new(#own_crate, #own_crate_version) },
            ];
            crate_deps.extend(crate_names.iter().map(|cname| {
                let version = EmitRewriter::resolve_workspace_version(cname)
                    .unwrap_or_else(|| "0.0".to_string());
                quote! { elicitation::emit_code::CrateDep::new(#cname, #version) }
            }));

            let emit_block = quote! {
                #[cfg(feature = "emit")]
                impl elicitation::emit_code::EmitCode for #params_ty {
                    fn emit_code(&self) -> elicitation::proc_macro2::TokenStream {
                        #(#field_bindings)*
                        ::quote::quote! { #rewritten }
                    }

                    fn crate_deps(&self) -> ::std::vec::Vec<elicitation::emit_code::CrateDep> {
                        ::std::vec![ #(#crate_deps),* ]
                    }
                }

                #[cfg(feature = "emit")]
                elicitation::register_emit!(#name, #params_ty);
            };

            expanded = quote! { #expanded #emit_block };
        }
        EmitMode::Custom(custom_ty) => {
            let emit_block = quote! {
                #[cfg(feature = "emit")]
                impl elicitation::emit_code::EmitCode for #params_ty {
                    fn emit_code(&self) -> elicitation::proc_macro2::TokenStream {
                        <#custom_ty as elicitation::emit_code::CustomEmit<#params_ty>>::emit_code(self)
                    }

                    fn crate_deps(&self) -> ::std::vec::Vec<elicitation::emit_code::CrateDep> {
                        <#custom_ty as elicitation::emit_code::CustomEmit<#params_ty>>::crate_deps()
                    }
                }

                #[cfg(feature = "emit")]
                elicitation::register_emit!(#name, #params_ty);
            };

            expanded = quote! { #expanded #emit_block };
        }
    }

    Ok(expanded)
}

/// Returns `true` if the first non-`self` parameter looks like `ctx: Arc<PluginContext>`.
///
/// Detection is name-based (`ctx`) as a heuristic; type path is also checked
/// for a segment ending in `PluginContext`.
fn first_param_is_context(func: &ItemFn) -> bool {
    let Some(first) = func.sig.inputs.iter().find_map(|arg| {
        if let FnArg::Typed(pt) = arg
            && let Pat::Ident(p) = pt.pat.as_ref()
            && p.ident != "self"
        {
            return Some(pt);
        }
        None
    }) else {
        return false;
    };

    // Name heuristic: parameter named `ctx`
    if let Pat::Ident(p) = first.pat.as_ref()
        && p.ident == "ctx"
    {
        return true;
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
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg
                && let Pat::Ident(p) = pat.as_ref()
                && p.ident != "self"
            {
                return Some(ty.as_ref());
            }
            None
        })
        .nth(n)
}
