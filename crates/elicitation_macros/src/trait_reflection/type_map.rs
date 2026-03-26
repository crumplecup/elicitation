//! `type_map(OldType => NewType, ...)` attribute parsing and code generation.
//!
//! `type_map` lets `#[reflect_trait]` bridge non-serializable third-party types
//! to their serializable newtype proxies, working around the orphan rule.
//!
//! # Usage
//!
//! ```rust,ignore
//! #[reflect_trait(clap::CommandFactory,
//!     type_map(clap::Command => crate::Command))]
//! trait CommandFactoryTools {
//!     fn command() -> clap::Command;
//! }
//! ```
//!
//! The macro generates `crate::Command::from(result)` for return values and
//! `clap::Command::from(p.cmd)` for parameters, using the `From` impls that
//! `elicit_newtype!` already provides.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Token, Type,
    parse::{Parse, ParseStream},
};

/// A single `OldType => NewType` mapping entry.
#[derive(Clone)]
pub struct TypeMapEntry {
    pub original: Type,
    pub proxy: Type,
}

/// The complete `type_map(A => B, C => D)` argument parsed from the attribute.
#[derive(Clone, Default)]
pub struct TypeMap(pub Vec<TypeMapEntry>);

impl Parse for TypeMapEntry {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let original: Type = input.parse()?;
        let _arrow: Token![=>] = input.parse()?;
        let proxy: Type = input.parse()?;
        Ok(TypeMapEntry { original, proxy })
    }
}

impl Parse for TypeMap {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // Parses: type_map( A => B , C => D )
        let content;
        syn::parenthesized!(content in input);
        let mut entries = Vec::new();
        while !content.is_empty() {
            entries.push(content.parse::<TypeMapEntry>()?);
            if content.peek(Token![,]) {
                let _: Token![,] = content.parse()?;
            }
        }
        Ok(TypeMap(entries))
    }
}

impl TypeMap {
    /// True if there are no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Find the proxy type for `ty` if a direct mapping exists.
    ///
    /// Comparison is done on the stringified type to avoid span differences.
    pub fn find_proxy<'a>(&'a self, ty: &Type) -> Option<&'a Type> {
        let needle = type_str(ty);
        self.0.iter().find_map(|e| {
            if type_str(&e.original) == needle {
                Some(&e.proxy)
            } else {
                None
            }
        })
    }

    /// Find the original type for `ty` (reverse lookup from proxy).
    pub fn find_original<'a>(&'a self, ty: &Type) -> Option<&'a Type> {
        let needle = type_str(ty);
        self.0.iter().find_map(|e| {
            if type_str(&e.proxy) == needle {
                Some(&e.original)
            } else {
                None
            }
        })
    }

    /// Substitute type_map entries inside `ty`, recursively traversing
    /// `Option<T>`, `Vec<T>`, and `Result<T, E>`.
    ///
    /// Returns the substituted type.
    pub fn apply_to_type(&self, ty: &Type) -> Type {
        if self.is_empty() {
            return ty.clone();
        }
        // Direct hit
        if let Some(proxy) = self.find_proxy(ty) {
            return proxy.clone();
        }
        // Recurse into generic wrappers (Option<T>, Vec<T>, Result<T,E>)
        if let Type::Path(type_path) = ty
            && let Some(last) = type_path.path.segments.last()
        {
            let name = last.ident.to_string();
            if matches!(name.as_str(), "Option" | "Vec" | "Result")
                && let syn::PathArguments::AngleBracketed(ref ab) = last.arguments
            {
                let new_args: Vec<syn::GenericArgument> = ab
                    .args
                    .iter()
                    .map(|arg| {
                        if let syn::GenericArgument::Type(inner) = arg {
                            syn::GenericArgument::Type(self.apply_to_type(inner))
                        } else {
                            arg.clone()
                        }
                    })
                    .collect();

                let mut new_path = type_path.clone();
                if let Some(last_seg) = new_path.path.segments.last_mut() {
                    last_seg.arguments =
                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                            colon2_token: ab.colon2_token,
                            lt_token: ab.lt_token,
                            args: new_args.into_iter().collect(),
                            gt_token: ab.gt_token,
                        });
                }
                return Type::Path(new_path);
            }
        }
        ty.clone()
    }

    /// Generate a conversion expression from `original_ty` to its serializable proxy form.
    ///
    /// - Direct mapping:        `ProxyType::from(expr)`
    /// - `Option<Mapped>`:      `(expr).map(ProxyType::from)`
    /// - `Vec<Mapped>`:         `(expr).into_iter().map(ProxyType::from).collect()`
    /// - `Result<Mapped, E>`:   `(expr).map(ProxyType::from)`
    /// - `&[T]` (slice ref):    `(expr).to_vec()` — makes the slice owned for async moves
    /// - No mapping:            identity (T is Serialize via factory bounds)
    pub fn proxy_encode(&self, expr: TokenStream, ty: &Type) -> TokenStream {
        if let Some(proxy) = self.find_proxy(ty) {
            return quote! { #proxy::from(#expr) };
        }
        // &[T] → .to_vec() so the value can be moved into a 'static async block
        if let Type::Reference(r) = ty
            && matches!(r.elem.as_ref(), Type::Slice(_))
        {
            return quote! { (#expr).to_vec() };
        }
        // Check generic wrappers
        if let Type::Path(type_path) = ty
            && let Some(last) = type_path.path.segments.last()
        {
            let name = last.ident.to_string();
            if let syn::PathArguments::AngleBracketed(ref ab) = last.arguments {
                let inner_types: Vec<&Type> = ab
                    .args
                    .iter()
                    .filter_map(|arg| {
                        if let syn::GenericArgument::Type(t) = arg {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect();

                if !inner_types.is_empty()
                    && let Some(proxy_inner) = self.find_proxy(inner_types[0])
                {
                    match name.as_str() {
                        "Option" => {
                            return quote! { (#expr).map(#proxy_inner::from) };
                        }
                        "Vec" => {
                            return quote! {
                                (#expr).into_iter().map(#proxy_inner::from).collect()
                            };
                        }
                        "Result" => {
                            return quote! { (#expr).map(#proxy_inner::from) };
                        }
                        _ => {}
                    }
                }
            }
        }
        // Fall through: result is T or a stdlib type — both are Serialize.
        // No proxy conversion needed for return values.
        quote! { #expr }
    }

    /// Generate a conversion expression from `proxy_ty` back to the original type.
    ///
    /// - Direct mapping:        `OriginalType::from(expr)`
    /// - `Option<Mapped>`:      `(expr).map(OriginalType::from)`
    /// - etc.
    /// - No mapping:            `::elicitation::ElicitProxy::from_proxy(expr)`
    pub fn proxy_decode(&self, expr: TokenStream, proxy_ty: &Type) -> TokenStream {
        if let Some(orig) = self.find_original(proxy_ty) {
            return quote! { #orig::from(#expr) };
        }
        // Check generic wrappers
        if let Type::Path(type_path) = proxy_ty
            && let Some(last) = type_path.path.segments.last()
        {
            let name = last.ident.to_string();
            if let syn::PathArguments::AngleBracketed(ref ab) = last.arguments {
                let inner_types: Vec<&Type> = ab
                    .args
                    .iter()
                    .filter_map(|arg| {
                        if let syn::GenericArgument::Type(t) = arg {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect();

                if !inner_types.is_empty()
                    && let Some(orig_inner) = self.find_original(inner_types[0])
                {
                    match name.as_str() {
                        "Option" => {
                            return quote! { (#expr).map(#orig_inner::from) };
                        }
                        "Vec" => {
                            return quote! {
                                (#expr).into_iter().map(#orig_inner::from).collect()
                            };
                        }
                        "Result" => {
                            return quote! { (#expr).map(#orig_inner::from) };
                        }
                        _ => {}
                    }
                }
            }
        }
        // Fall through: use ElicitProxy
        quote! { ::elicitation::ElicitProxy::from_proxy(#expr) }
    }
}

fn type_str(ty: &Type) -> String {
    quote!(#ty).to_string().replace(" ", "")
}
