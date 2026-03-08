//! `#[derive(ElicitPlugin)]` — generate an [`ElicitPlugin`] impl from a unit struct.
//!
//! Collects all [`PluginToolRegistration`]s for the named plugin via `inventory`
//! and dispatches to them, eliminating all handwritten `list_tools` / `call_tool`
//! boilerplate.
//!
//! # Struct shapes
//!
//! - **Unit struct** `struct MyPlugin;` — a fresh `PluginContext` is created per call
//! - **Newtype** `struct MyPlugin(Arc<PluginContext>);` — the stored context is cloned per call,
//!   enabling resource sharing (e.g. a single `reqwest::Client` connection pool)
//!
//! [`ElicitPlugin`]: elicitation::plugin::ElicitPlugin
//! [`PluginToolRegistration`]: elicitation::plugin::PluginToolRegistration

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Lit, Meta, Result};

/// Expand `#[derive(ElicitPlugin)]` with `#[plugin(name = "...")]`.
pub fn expand(input: TokenStream) -> TokenStream {
    match expand_inner(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(input: TokenStream) -> Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(input)?;
    let type_ident = &ast.ident;

    // Extract plugin name from #[plugin(name = "...")]
    let plugin_name = extract_plugin_name(&ast)?;

    // Determine context source based on struct shape.
    let ctx_expr = context_source_expr(&ast)?;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics elicitation::ElicitPlugin for #type_ident #ty_generics #where_clause {
            fn name(&self) -> &'static str {
                #plugin_name
            }

            fn list_tools(&self) -> ::std::vec::Vec<elicitation::rmcp::model::Tool> {
                elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                    .filter(|r| r.plugin == #plugin_name)
                    .map(|r| (r.constructor)().as_tool())
                    .collect()
            }

            fn call_tool<'__a>(
                &'__a self,
                params: elicitation::rmcp::model::CallToolRequestParams,
                _ctx: elicitation::rmcp::service::RequestContext<elicitation::rmcp::RoleServer>,
            ) -> elicitation::futures::future::BoxFuture<
                '__a,
                ::std::result::Result<
                    elicitation::rmcp::model::CallToolResult,
                    elicitation::rmcp::ErrorData,
                >,
            > {
                let prefix = ::std::format!("{plugin}__", plugin = #plugin_name);
                let bare = params
                    .name
                    .strip_prefix(prefix.as_str())
                    .map(::std::string::String::from)
                    .unwrap_or_else(|| params.name.to_string());

                let found = elicitation::inventory::iter::<elicitation::PluginToolRegistration>()
                    .filter(|r| r.plugin == #plugin_name)
                    .find(|r| r.name == bare)
                    .map(|r| (r.constructor)());

                let plugin_ctx = #ctx_expr;

                ::std::boxed::Box::pin(async move {
                    match found {
                        ::std::option::Option::Some(descriptor) => {
                            descriptor.dispatch(plugin_ctx, params).await
                        }
                        ::std::option::Option::None => {
                            ::std::result::Result::Err(
                                elicitation::rmcp::ErrorData::invalid_params(
                                    ::std::format!("unknown tool: {bare}"),
                                    ::std::option::Option::None,
                                )
                            )
                        }
                    }
                })
            }
        }
    })
}

/// Return a `TokenStream` expression that evaluates to `Arc<PluginContext>`.
///
/// - Unit struct → `Arc::new(PluginContext::default())`
/// - One unnamed field → `self.0.clone()` (assumes `Arc<PluginContext>` newtype)
fn context_source_expr(ast: &DeriveInput) -> Result<TokenStream> {
    let Data::Struct(ref data) = ast.data else {
        return Err(Error::new_spanned(
            ast,
            "#[derive(ElicitPlugin)] is only supported on structs",
        ));
    };

    match &data.fields {
        Fields::Unit => Ok(quote! {
            ::std::sync::Arc::new(elicitation::PluginContext::default())
        }),
        Fields::Unnamed(unnamed) if unnamed.unnamed.len() == 1 => Ok(quote! {
            self.0.clone()
        }),
        other => Err(Error::new_spanned(
            other,
            "#[derive(ElicitPlugin)] supports only unit structs or newtypes with one \
             unnamed field (e.g. `struct MyPlugin(Arc<PluginContext>);`)",
        )),
    }
}

fn extract_plugin_name(ast: &DeriveInput) -> Result<String> {
    for attr in &ast.attrs {
        if !attr.path().is_ident("plugin") {
            continue;
        }
        let meta = attr.parse_args::<Meta>()?;
        let Meta::NameValue(nv) = meta else {
            return Err(Error::new_spanned(
                attr,
                "expected `#[plugin(name = \"...\")]`",
            ));
        };
        if !nv.path.is_ident("name") {
            return Err(Error::new_spanned(&nv.path, "expected `name = \"...\"`"));
        }
        let syn::Expr::Lit(expr_lit) = &nv.value else {
            return Err(Error::new_spanned(&nv.value, "expected a string literal"));
        };
        let Lit::Str(s) = &expr_lit.lit else {
            return Err(Error::new_spanned(
                &expr_lit.lit,
                "expected a string literal",
            ));
        };
        return Ok(s.value());
    }

    Err(Error::new_spanned(
        ast,
        "#[derive(ElicitPlugin)] requires `#[plugin(name = \"...\")]`",
    ))
}
