//! Parameter struct generation for `#[reflect_trait]`.
//!
//! For each method in the `#[reflect_trait]` impl block, we generate a param
//! struct that the agent sends as JSON.  The struct derives
//! `Deserialize + JsonSchema` so it can be used directly in
//! `DynamicToolDescriptor::schema` and deserialized from tool call arguments.
//!
//! # Self receivers
//!
//! Methods with `&self` or `&mut self` receivers do not include `self` in the
//! param struct.  The value of `self` is the registered type `T` and is not
//! provided by the agent — it is the *target* of the tool call.
//!
//! Methods with no receiver (associated functions) include all parameters.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{FnArg, ReturnType, Signature, Type};

use super::naming::param_struct_name;
use super::type_map::TypeMap;

/// A parsed method extracted from the `#[reflect_trait]` impl block.
pub struct MethodInfo {
    /// Original method name (e.g. `insert`).
    pub name: syn::Ident,
    /// Non-self parameters in declaration order.
    pub params: Vec<ParamInfo>,
    /// Return type, stored for future use in result serialization.
    #[allow(dead_code)]
    pub return_type: ReturnType,
    /// True if the first argument is `&self` or `&mut self`.
    pub has_self: bool,
}

/// A single non-self parameter.
pub struct ParamInfo {
    pub name: syn::Ident,
    pub ty: Box<Type>,
}

impl MethodInfo {
    /// Parse all method signatures from a trait block's items.
    ///
    /// Only `TraitItem::Fn` items are accepted; anything else is an error.
    pub fn from_trait_items(items: &[syn::TraitItem]) -> syn::Result<Vec<Self>> {
        items
            .iter()
            .map(|item| match item {
                syn::TraitItem::Fn(f) => Self::from_sig(&f.sig),
                _ => Err(syn::Error::new_spanned(
                    item,
                    "#[reflect_trait] trait body may only contain method signatures",
                )),
            })
            .collect()
    }

    /// Parse all method signatures from an impl block's items.
    ///
    /// Only `ImplItem::Fn` items are accepted; anything else is an error.
    pub fn from_impl_items(items: &[syn::ImplItem]) -> syn::Result<Vec<Self>> {
        items
            .iter()
            .map(|item| match item {
                syn::ImplItem::Fn(f) => Self::from_sig(&f.sig),
                _ => Err(syn::Error::new_spanned(
                    item,
                    "#[reflect_trait] impl body may only contain method signatures (fn items)",
                )),
            })
            .collect()
    }

    fn from_sig(sig: &Signature) -> syn::Result<Self> {
        let name = sig.ident.clone();
        let mut has_self = false;
        let mut params = Vec::new();

        for arg in &sig.inputs {
            match arg {
                FnArg::Receiver(_) => {
                    has_self = true;
                }
                FnArg::Typed(pat_type) => {
                    let param_name = match pat_type.pat.as_ref() {
                        syn::Pat::Ident(id) => id.ident.clone(),
                        other => {
                            return Err(syn::Error::new_spanned(
                                other,
                                "#[reflect_trait]: parameter patterns must be simple identifiers",
                            ));
                        }
                    };
                    params.push(ParamInfo {
                        name: param_name,
                        ty: pat_type.ty.clone(),
                    });
                }
            }
        }

        Ok(Self {
            name,
            params,
            return_type: sig.output.clone(),
            has_self,
        })
    }

    /// Generate the param struct for this method.
    ///
    /// For `&self` methods, a `target: serde_json::Value` field is automatically
    /// prepended so the agent can supply the concrete instance to operate on.
    ///
    /// ```text
    /// #[derive(Debug, Clone, serde::Deserialize, schemars::JsonSchema)]
    /// struct IsValidNameParams {
    ///     target: serde_json::Value,  // auto-added for &self methods
    ///     name: String,
    /// }
    /// ```
    pub fn param_struct_tokens(&self, vis: &syn::Visibility, type_map: &TypeMap) -> TokenStream {
        let struct_name = param_struct_name(&self.name.to_string());
        let method_name_str = self.name.to_string();
        let struct_doc = format!("Parameters for the `{method_name_str}` tool method.");

        let target_field = if self.has_self {
            quote! {
                /// Serialized target instance (the `Self` value to call this method on).
                pub target: ::serde_json::Value,
            }
        } else {
            quote! {}
        };

        let fields: Vec<TokenStream> = self
            .params
            .iter()
            .map(|p| {
                let field_name = &p.name;
                let field_ty = &p.ty;
                // &str params: the agent sends a String
                if is_str_ref(field_ty) {
                    let doc = format!("`{field_name}` parameter (as owned String).");
                    return quote! {
                        #[doc = #doc]
                        pub #field_name: String,
                    };
                }
                let substituted = type_map.apply_to_type(field_ty);
                let doc = format!("`{field_name}` parameter.");
                if substituted.to_token_stream().to_string()
                    != field_ty.to_token_stream().to_string()
                {
                    quote! {
                        #[doc = #doc]
                        pub #field_name: #substituted,
                    }
                } else {
                    quote! {
                        #[doc = #doc]
                        pub #field_name: <#field_ty as ::elicitation::ElicitProxy>::Proxy,
                    }
                }
            })
            .collect();

        quote! {
            #[doc = #struct_doc]
            #[derive(Debug, Clone, ::serde::Deserialize, ::schemars::JsonSchema)]
            #vis struct #struct_name {
                #target_field
                #(#fields)*
            }
        }
    }
}

/// Returns `true` if the type is `&str` or `&'_ str`.
pub fn is_str_ref(ty: &Type) -> bool {
    if let Type::Reference(r) = ty {
        if let Type::Path(tp) = r.elem.as_ref() {
            if tp.qself.is_none() {
                if let Some(seg) = tp.path.segments.last() {
                    return seg.ident == "str" && seg.arguments.is_none();
                }
            }
        }
    }
    false
}
