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
use quote::quote;
use syn::Path;

use super::{
    naming::{param_struct_name, vtable_struct_name},
    params::MethodInfo,
};

/// Generate the vtable struct definition and `for_type::<T>()` constructor.
pub fn vtable_tokens(
    trait_path: &Path,
    trait_path_str: &str,
    methods: &[MethodInfo],
    vis: &syn::Visibility,
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
    let closure_fields: Vec<TokenStream> = methods
        .iter()
        .map(|m| {
            let method_name = &m.name;
            let param_struct = param_struct_name(&method_name.to_string());
            let param_names: Vec<&syn::Ident> = m.params.iter().map(|p| &p.name).collect();

            let call = if m.has_self {
                // For &self methods the param struct contains `target: serde_json::Value`.
                // Deserialize T from that value inside the closure.
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
                            let result = <T as #trait_path>::#method_name(
                                &target,
                                #(p.#param_names,)*
                            );
                            let text = ::serde_json::to_string(&result)
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
                            let result = <T as #trait_path>::#method_name(#(p.#param_names,)*);
                            let text = ::serde_json::to_string(&result)
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
            call
        })
        .collect();

    quote! {
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
