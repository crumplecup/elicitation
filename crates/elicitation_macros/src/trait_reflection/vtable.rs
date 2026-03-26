//! VTable struct generation for `#[reflect_trait]`.
//!
//! The vtable holds one `Arc<dyn Fn(Value) -> BoxFuture<Result<CallToolResult>>>` per
//! method.  It is created inside `register_type::<T>()` — that is where
//! monomorphization happens and the concrete `T` is captured in a closure.
//!
//! The generated code looks like:
//!
//! ```rust,ignore
//! struct InsertableVTable {
//!     insert: Arc<dyn Fn(serde_json::Value)
//!         -> BoxFuture<'static, Result<CallToolResult, ErrorData>>
//!         + Send + Sync>,
//!     // ... one field per method
//! }
//!
//! impl InsertableVTable {
//!     fn for_type<T>(prefix: &str) -> Self
//!     where
//!         T: diesel::Insertable + Serialize + DeserializeOwned + JsonSchema
//!              + Elicitation + Send + Sync + 'static,
//!     {
//!         Self {
//!             insert: Arc::new(|params| Box::pin(async move {
//!                 let p: InsertParams = serde_json::from_value(params)?;
//!                 let result = <T as diesel::Insertable>::insert(p.item)?;
//!                 Ok(CallToolResult::success(vec![Content::text(
//!                     serde_json::to_string(&result)?
//!                 )]))
//!             })),
//!         }
//!     }
//! }
//! ```

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Path, ReturnType};

use super::{
    naming::{param_struct_name, vtable_struct_name},
    params::MethodInfo,
    type_map::TypeMap,
};

/// Generate the vtable struct definition and `for_type::<T>()` constructor.
pub fn vtable_tokens(
    trait_path: &Path,
    trait_path_str: &str,
    methods: &[MethodInfo],
    vis: &syn::Visibility,
    type_map: &TypeMap,
) -> TokenStream {
    let vtable_name = vtable_struct_name(trait_path_str);

    // Field declarations: one Arc<dyn Fn> per method
    let fields: Vec<TokenStream> = methods
        .iter()
        .map(|m| {
            let field = &m.name;
            quote! {
                #field: ::std::sync::Arc<
                    dyn Fn(::serde_json::Value)
                        -> ::futures::future::BoxFuture<
                            'static,
                            ::std::result::Result<
                                ::rmcp::model::CallToolResult,
                                ::rmcp::ErrorData,
                            >
                        >
                        + Send + Sync
                >,
            }
        })
        .collect();

    // for_type::<T>() constructor — one closure per method
    let closure_fields: Vec<TokenStream> =
        methods
            .iter()
            .map(|m| {
                let method_name = &m.name;
                let param_struct = param_struct_name(&method_name.to_string());
                let param_names: Vec<&syn::Ident> = m.params.iter().map(|p| &p.name).collect();
                let param_types: Vec<&syn::Type> = m.params.iter().map(|p| p.ty.as_ref()).collect();

                // Convert each param from its proxy/substituted form back to the original type.
                let param_conversions: Vec<TokenStream> = m.params.iter().map(|p| {
                let name = &p.name;
                let ty = &p.ty;
                // &str params: the struct holds a String, convert with .as_str()
                if super::params::is_str_ref(ty) {
                    return quote! { let #name = p.#name.as_str(); };
                }
                let substituted = type_map.apply_to_type(ty);
                if substituted.to_token_stream().to_string() != ty.to_token_stream().to_string() {
                    // type_map handled this: use proxy_decode for the substituted type
                    let field_access = quote! { p.#name };
                    let conversion = type_map.proxy_decode(field_access, &substituted);
                    quote! { let #name = #conversion; }
                } else {
                    quote! {
                        let #name = <#ty as ::elicitation::ElicitProxy>::from_proxy(p.#name);
                    }
                }
            }).collect();

                // Return type: extract the actual type (strip the `->`)
                let ret_ty = match &m.return_type {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => Some(ty.as_ref()),
                };

                // Generate the return value conversion expression.
                let result_conversion = if let Some(ret) = ret_ty {
                    type_map.proxy_encode(quote! { result }, ret)
                } else {
                    // Unit return: serialize as `null`
                    quote! { ::elicitation::ElicitProxy::into_proxy(result) }
                };

                let call = if m.has_self {
                    // For &self methods the param struct contains `target: serde_json::Value`.
                    // Deserialize T from that value, then convert other params via from_proxy.
                    quote! {
                        #method_name: ::std::sync::Arc::new(|params| {
                            ::std::boxed::Box::pin(async move {
                                let p: #param_struct = ::serde_json::from_value(params)
                                    .map_err(|e| ::rmcp::ErrorData::invalid_params(
                                        e.to_string(), None
                                    ))?;
                                let target: T = ::serde_json::from_value(p.target)
                                    .map_err(|e| ::rmcp::ErrorData::invalid_params(
                                        format!("failed to deserialize target: {e}"), None
                                    ))?;
                                // Convert proxy params to their original types
                                #(#param_conversions)*
                                let result = <T as #trait_path>::#method_name(
                                    &target,
                                    #(#param_names,)*
                                );
                                // Convert return value to its serializable proxy
                                let proxied = #result_conversion;
                                let text = ::serde_json::to_string(&proxied)
                                    .map_err(|e| ::rmcp::ErrorData::internal_error(
                                        e.to_string(), None
                                    ))?;
                                Ok(::rmcp::model::CallToolResult::success(vec![
                                    ::rmcp::model::Content::new(
                                        ::rmcp::model::RawContent::text(text),
                                        None,
                                    )
                                ]))
                            })
                        }),
                    }
                } else {
                    quote! {
                        #method_name: ::std::sync::Arc::new(|params| {
                            ::std::boxed::Box::pin(async move {
                                let p: #param_struct = ::serde_json::from_value(params)
                                    .map_err(|e| ::rmcp::ErrorData::invalid_params(
                                        e.to_string(), None
                                    ))?;
                                // Convert proxy params to their original types
                                #(#param_conversions)*
                                let result = <T as #trait_path>::#method_name(#(#param_names,)*);
                                // Convert return value to its serializable proxy
                                let proxied = #result_conversion;
                                let text = ::serde_json::to_string(&proxied)
                                    .map_err(|e| ::rmcp::ErrorData::internal_error(
                                        e.to_string(), None
                                    ))?;
                                Ok(::rmcp::model::CallToolResult::success(vec![
                                    ::rmcp::model::Content::new(
                                        ::rmcp::model::RawContent::text(text),
                                        None,
                                    )
                                ]))
                            })
                        }),
                    }
                };
                // Suppress unused variable warning for param_types (used for future bounds)
                let _ = param_types;
                call
            })
            .collect();

    let vtable_doc = format!("Vtable holding dispatch closures for `{trait_path_str}` tools.");

    quote! {
        #[doc = #vtable_doc]
        #vis struct #vtable_name {
            #(#fields)*
        }

        impl #vtable_name {
            /// Build a vtable capturing monomorphized dispatch for `T`.
            ///
            /// Called inside `register_type::<T>()` — T is concrete here.
            fn for_type<T>() -> Self
            where
                T: #trait_path
                    + ::serde::Serialize
                    + ::serde::de::DeserializeOwned
                    + ::schemars::JsonSchema
                    + ::elicitation::Elicitation
                    + Send + Sync + 'static,
            {
                Self {
                    #(#closure_fields)*
                }
            }
        }
    }
}
