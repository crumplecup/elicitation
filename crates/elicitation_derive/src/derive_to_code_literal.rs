//! Derive macro implementation for `#[derive(ToCodeLiteral)]`.
//!
//! Auto-generates [`ToCodeLiteral`] implementations that convert struct/enum
//! values into `TokenStream` expressions reconstructing them. Generated impls
//! are gated on `#[cfg(feature = "emit")]` at the call site.

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Punct, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields};

/// Expand `#[derive(ToCodeLiteral)]` for a struct or enum.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let to_code_literal_body = match &input.data {
        Data::Struct(data) => gen_struct_body(name, &data.fields),
        Data::Enum(data) => gen_enum_body(name, data),
        Data::Union(_) => {
            return syn::Error::new_spanned(name, "ToCodeLiteral cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        #[cfg(feature = "emit")]
        impl #impl_generics ::elicitation::emit_code::ToCodeLiteral
            for #name #ty_generics #where_clause
        {
            fn to_code_literal(&self) -> ::elicitation::proc_macro2::TokenStream {
                #to_code_literal_body
            }

            fn type_tokens() -> ::elicitation::proc_macro2::TokenStream {
                ::quote::quote! { #name }
            }
        }
    };

    expanded.into()
}

/// Emit a literal `#` token that will be interpreted by the runtime `quote!`.
fn hash_punct() -> TokenTree {
    TokenTree::Punct(Punct::new('#', Spacing::Alone))
}

/// Build `field: #local, field2: #local2, ...` as raw tokens for inner quote body.
fn build_named_fields_pattern(
    field_names: &[&syn::Ident],
    local_vars: &[syn::Ident],
) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    for (i, (field, local)) in field_names.iter().zip(local_vars.iter()).enumerate() {
        if i > 0 {
            tokens.extend(std::iter::once(TokenTree::Punct(Punct::new(
                ',',
                Spacing::Alone,
            ))));
        }
        tokens.extend(quote! { #field : });
        tokens.extend(std::iter::once(hash_punct()));
        tokens.extend(quote! { #local });
    }
    tokens
}

/// Build `#local0, #local1, ...` as raw tokens for tuple fields in inner quote body.
fn build_tuple_fields_pattern(local_vars: &[syn::Ident]) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    for (i, local) in local_vars.iter().enumerate() {
        if i > 0 {
            tokens.extend(std::iter::once(TokenTree::Punct(Punct::new(
                ',',
                Spacing::Alone,
            ))));
        }
        tokens.extend(std::iter::once(hash_punct()));
        tokens.extend(quote! { #local });
    }
    tokens
}

/// Wrap tokens in `::quote::quote! { <inner> }`.
fn wrap_in_quote(inner: TokenStream2) -> TokenStream2 {
    let group = Group::new(Delimiter::Brace, inner);
    let mut result = quote! { ::quote::quote! };
    result.extend(std::iter::once(TokenTree::Group(group)));
    result
}

/// Generate body for a struct (named, tuple, or unit).
fn gen_struct_body(name: &syn::Ident, fields: &Fields) -> TokenStream2 {
    match fields {
        Fields::Named(named) => {
            let field_names: Vec<_> = named
                .named
                .iter()
                .map(|f| f.ident.as_ref().expect("named field"))
                .collect();
            let local_vars: Vec<_> = field_names
                .iter()
                .map(|f| format_ident!("__tcl_{f}"))
                .collect();

            let fields_pattern = build_named_fields_pattern(&field_names, &local_vars);
            let brace_group = Group::new(Delimiter::Brace, fields_pattern);
            let mut inner = quote! { #name };
            inner.extend(std::iter::once(TokenTree::Group(brace_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #(
                    let #local_vars =
                        ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#field_names);
                )*
                #quote_call
            }
        }
        Fields::Unnamed(unnamed) => {
            let indices: Vec<_> = (0..unnamed.unnamed.len()).map(syn::Index::from).collect();
            let local_vars: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__tcl_{i}"))
                .collect();

            let tuple_pattern = build_tuple_fields_pattern(&local_vars);
            let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
            let mut inner = quote! { #name };
            inner.extend(std::iter::once(TokenTree::Group(paren_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #(
                    let #local_vars =
                        ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#indices);
                )*
                #quote_call
            }
        }
        Fields::Unit => {
            let quote_call = wrap_in_quote(quote! { #name });
            quote! { #quote_call }
        }
    }
}

/// Generate match body for an enum.
fn gen_enum_body(name: &syn::Ident, data: &syn::DataEnum) -> TokenStream2 {
    let arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let vname = &variant.ident;
            gen_variant_arm(name, vname, &variant.fields)
        })
        .collect();

    quote! {
        match self {
            #( #arms )*
        }
    }
}

/// Generate a single match arm for an enum variant.
fn gen_variant_arm(
    enum_name: &syn::Ident,
    variant_name: &syn::Ident,
    fields: &Fields,
) -> TokenStream2 {
    match fields {
        Fields::Named(named) => {
            let field_names: Vec<_> = named
                .named
                .iter()
                .map(|f| f.ident.as_ref().expect("named field"))
                .collect();
            let local_vars: Vec<_> = field_names
                .iter()
                .map(|f| format_ident!("__tcl_{f}"))
                .collect();

            let fields_pattern = build_named_fields_pattern(&field_names, &local_vars);
            let brace_group = Group::new(Delimiter::Brace, fields_pattern);
            let mut inner = quote! { #enum_name :: #variant_name };
            inner.extend(std::iter::once(TokenTree::Group(brace_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #enum_name::#variant_name { #( #field_names ),* } => {
                    #(
                        let #local_vars =
                            ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#field_names);
                    )*
                    #quote_call
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            let bindings: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__field_{i}"))
                .collect();
            let local_vars: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__tcl_{i}"))
                .collect();

            let tuple_pattern = build_tuple_fields_pattern(&local_vars);
            let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
            let mut inner = quote! { #enum_name :: #variant_name };
            inner.extend(std::iter::once(TokenTree::Group(paren_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #enum_name::#variant_name( #( #bindings ),* ) => {
                    #(
                        let #local_vars =
                            ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#bindings);
                    )*
                    #quote_call
                }
            }
        }
        Fields::Unit => {
            let quote_call = wrap_in_quote(quote! { #enum_name :: #variant_name });
            quote! {
                #enum_name::#variant_name => {
                    #quote_call
                }
            }
        }
    }
}
