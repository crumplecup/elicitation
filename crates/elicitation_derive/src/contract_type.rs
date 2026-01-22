//! `#[contract_type]` attribute macro for metadata annotation.
//!
//! This macro adds contract metadata to types, allowing the derive macro
//! to extract and compose verification requirements.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{ParseStream, Parser};
use syn::{parse_macro_input, Expr, ExprLit, Item, Lit, Meta};

/// Annotates a type with contract metadata.
///
/// # Attributes
///
/// - `requires = "expr"` - Precondition string (checked at construction)
/// - `ensures = "expr"` - Postcondition string (guaranteed after construction)
///
/// # Example
///
/// ```rust,ignore
/// #[contract_type(
///     requires = "value > 0",
///     ensures = "result.get() > 0"
/// )]
/// pub struct I8Positive(i8);
/// ```
///
/// The metadata is stored via const fns that can be queried at compile time.
pub fn contract_type_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    
    // Parse comma-separated name=value pairs
    let parser = |input: ParseStream| {
        syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)
    };
    
    let metas = match parser.parse(args) {
        Ok(metas) => metas,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut requires_expr = None;
    let mut ensures_expr = None;

    // Parse name=value pairs
    for meta in metas {
        if let Meta::NameValue(nv) = meta {
            let name = nv.path.get_ident().map(|i| i.to_string());
            
            if let Some(name) = name {
                if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = nv.value {
                    match name.as_str() {
                        "requires" => requires_expr = Some(lit_str.value()),
                        "ensures" => ensures_expr = Some(lit_str.value()),
                        _ => {}
                    }
                }
            }
        }
    }

    let requires_expr = requires_expr.unwrap_or_else(|| "true".to_string());
    let ensures_expr = ensures_expr.unwrap_or_else(|| "true".to_string());

    // Extract type name
    let type_name = match &input {
        Item::Struct(s) => &s.ident,
        Item::Enum(e) => &e.ident,
        _ => {
            return syn::Error::new_spanned(
                &input,
                "#[contract_type] only supports structs and enums"
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate metadata methods
    let metadata_impl = quote! {
        #[automatically_derived]
        impl #type_name {
            #[doc(hidden)]
            pub const fn __contract_requires() -> &'static str {
                #requires_expr
            }

            #[doc(hidden)]
            pub const fn __contract_ensures() -> &'static str {
                #ensures_expr
            }
        }
    };

    // Return original item + metadata
    let expanded = quote! {
        #input
        #metadata_impl
    };

    expanded.into()
}
