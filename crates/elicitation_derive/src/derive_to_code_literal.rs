//! Derive macro implementation for `#[derive(ToCodeLiteral)]`.
//!
//! # Overview
//!
//! [`ToCodeLiteral`] is the trait that lets a value describe itself as Rust
//! source code — a `TokenStream` expression whose evaluation reconstructs the
//! original value.  The `#[elicit_tool]` macro relies on it: when auto-deriving
//! an [`EmitCode`] impl for a tool's params struct, it calls
//! `ToCodeLiteral::to_code_literal(&self.field)` for every field so the emitted
//! workflow code contains concrete literals rather than opaque runtime values.
//!
//! Writing these impls by hand is tedious for simple types and impractical for
//! complex ones.  This module provides `#[derive(ToCodeLiteral)]` which
//! auto-generates the impl for any struct or enum, including recursive types
//! like `UiNode` (whose variants contain `Vec<UiNode>`).
//!
//! # What Gets Generated
//!
//! ## Named Structs
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! struct ColorJson { r: f32, g: f32, b: f32, a: f32 }
//!
//! // Generates (simplified):
//! #[cfg(feature = "emit")]
//! impl ToCodeLiteral for ColorJson {
//!     fn to_code_literal(&self) -> TokenStream {
//!         let r = ToCodeLiteral::to_code_literal(&self.r);
//!         let g = ToCodeLiteral::to_code_literal(&self.g);
//!         let b = ToCodeLiteral::to_code_literal(&self.b);
//!         let a = ToCodeLiteral::to_code_literal(&self.a);
//!         quote! { ColorJson { r: #r, g: #g, b: #b, a: #a } }
//!     }
//!     fn type_tokens() -> TokenStream { quote! { ColorJson } }
//! }
//! ```
//!
//! ## Tuple Structs
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! struct Wrapper(String);
//! // → quote! { Wrapper(#__tcl_0) }
//! ```
//!
//! Named proxy structs can also target tuple constructors with
//! `#[to_code_literal(tuple)]`:
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! #[to_code_literal(path = "::bevy::mesh::Mesh3d", tuple)]
//! struct Mesh3dParams {
//!     #[to_code_literal(expr)]
//!     mesh_expr: String,
//! }
//! // → quote! { ::bevy::mesh::Mesh3d(meshes.add(...)) }
//! ```
//!
//! Named helper structs can also emit a raw tuple literal with
//! `#[to_code_literal(raw_tuple)]`:
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! #[to_code_literal(raw_tuple)]
//! struct TextStyleParts {
//!     font: TextFontJson,
//!     color: TextColorJson,
//!     layout: TextLayoutJson,
//! }
//! // → quote! { (TextFont { ... }, TextColor(...), TextLayout { ... }) }
//! ```
//!
//! ## Unit Structs
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! struct Marker;
//! // → quote! { Marker }
//! ```
//!
//! ## Enums
//!
//! Handles all three variant forms — unit, tuple, and struct:
//!
//! ```rust,ignore
//! #[derive(ToCodeLiteral)]
//! enum UiNode {
//!     Widget { widget: WidgetJson },                           // struct variant
//!     Container { container: ContainerJson, children: Vec<UiNode> }, // recursive
//!     Leaf(String),                                            // tuple variant
//!     Empty,                                                   // unit variant
//! }
//! ```
//!
//! Each variant becomes a match arm that destructures, converts each field
//! via `ToCodeLiteral`, and reassembles:
//!
//! ```text
//! UiNode::Widget { widget } => {
//!     let __tcl_widget = ToCodeLiteral::to_code_literal(widget);
//!     quote! { UiNode::Widget { widget: #__tcl_widget } }
//! }
//! UiNode::Empty => quote! { UiNode::Empty }
//! ```
//!
//! # Recursion Through Blanket Impls
//!
//! The derive does **not** need special handling for `Vec<T>`, `Option<T>`, or
//! `HashMap<String, V>`.  Those types already have blanket `ToCodeLiteral`
//! impls in `elicitation::emit_code` that delegate element-wise.  So
//! `Vec<UiNode>` works automatically as long as `UiNode` itself implements the
//! trait — which the derive provides.
//!
//! # Feature Gating
//!
//! The generated impl is always wrapped in `#[cfg(feature = "emit")]`.  This
//! means:
//!
//! - The derive attribute is **always available** (proc macros run regardless
//!   of features).
//! - The generated impl **silently vanishes** when `emit` is disabled — no
//!   compile errors, no dead code.
//! - Downstream crates need their own `emit` feature that activates
//!   `elicitation/emit` (e.g. `elicit_egui` has
//!   `emit = ["dep:quote", "elicitation/emit"]`).
//!
//! # Nested Quote Technique
//!
//! This derive faces a classic proc-macro challenge: it must generate code
//! that itself calls `::quote::quote!`.  The outer `quote!` (in the derive)
//! interprets `#ident` as interpolation, but the inner `quote!` (in the
//! generated code) also needs `#ident` markers for *its* interpolation.
//!
//! The solution: build the inner `quote!` body as raw `TokenStream` with
//! literal `#` [`Punct`] tokens, then interpolate that stream into the outer
//! `quote!`.  The outer macro copies the `#` tokens verbatim; the inner
//! `::quote::quote!` at the user's compile time interprets them as
//! interpolation points.
//!
//! ```text
//! Derive expansion time (proc macro):
//!   outer quote! { ::quote::quote! { #inner_body } }
//!                                     ↑ interpolated as TokenStream
//!
//! inner_body contains:
//!   ColorJson { r : # __tcl_r , g : # __tcl_g , ... }
//!                   ↑ literal Punct('#')
//!
//! User compile time:
//!   ::quote::quote! { ColorJson { r: #__tcl_r, g: #__tcl_g, ... } }
//!                                    ↑ now interpreted as interpolation
//! ```
//!
//! # Requirements
//!
//! Every field type must itself implement `ToCodeLiteral`.  If a field's type
//! does not, the compiler will report a trait-bound error pointing at the
//! derive — add `#[derive(ToCodeLiteral)]` to that type or provide a manual
//! impl.
//!
//! The crate using the derive must have `quote` available (typically as an
//! optional dependency gated on the `emit` feature).

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Punct, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, LitStr};

use crate::struct_impl::has_skip_attr;

/// Expand `#[derive(ToCodeLiteral)]` for a struct or enum.
pub fn expand(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let options = match parse_options(&input.attrs) {
        Ok(options) => options,
        Err(error) => return error.to_compile_error().into(),
    };
    let literal_type = options
        .type_path
        .clone()
        .unwrap_or_else(|| quote! { #name });

    if options.transparent && (options.tuple || options.raw_tuple) {
        return syn::Error::new_spanned(
            name,
            "#[to_code_literal(transparent)] cannot be combined with tuple-emission options",
        )
        .to_compile_error()
        .into();
    }

    if options.transparent
        && (options.default_update
            || options.update_expr.is_some()
            || options.default_expr.is_some())
    {
        return syn::Error::new_spanned(
            name,
            "#[to_code_literal(transparent)] cannot be combined with default-constructor options",
        )
        .to_compile_error()
        .into();
    }

    if (options.tuple || options.raw_tuple)
        && (options.default_update
            || options.update_expr.is_some()
            || options.default_expr.is_some())
    {
        return syn::Error::new_spanned(
            name,
            "tuple-emission to_code_literal options cannot be combined with default-constructor options",
        )
        .to_compile_error()
        .into();
    }

    let to_code_literal_body = match (&input.data, options.transparent) {
        (Data::Struct(data), false) => {
            let update_expr = if options.default_update {
                Some(quote! { ::std::default::Default::default() })
            } else {
                options.update_expr.clone()
            };
            match gen_struct_body(
                &literal_type,
                &data.fields,
                options.tuple,
                options.raw_tuple,
                update_expr.as_ref(),
                options.default_expr.as_ref(),
            ) {
                Ok(body) => body,
                Err(error) => return error.to_compile_error().into(),
            }
        }
        (Data::Struct(data), true) => match gen_transparent_body(&data.fields) {
            Ok(body) => body,
            Err(error) => return error.to_compile_error().into(),
        },
        (Data::Enum(data), false) => gen_enum_body(name, &literal_type, data),
        (Data::Enum(_), true) => {
            return syn::Error::new_spanned(
                name,
                "#[to_code_literal(transparent)] only supports structs",
            )
            .to_compile_error()
            .into();
        }
        (Data::Union(_), _) => {
            return syn::Error::new_spanned(name, "ToCodeLiteral cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };
    let type_tokens_expr = match options.type_path {
        Some(type_path) => quote! { ::elicitation::quote::quote! { #type_path } },
        None if options.raw_tuple => match raw_tuple_type_tokens(&input.data) {
            Ok(tokens) => tokens,
            Err(error) => return error.to_compile_error().into(),
        },
        None if options.transparent => match transparent_type_tokens(&input.data) {
            Ok(tokens) => tokens,
            Err(error) => return error.to_compile_error().into(),
        },
        None => quote! { ::elicitation::quote::quote! { #name } },
    };
    let type_tokens_literal = type_tokens_expr.to_string();

    let expanded = quote! {
        impl #impl_generics ::elicitation::emit_code::ToCodeLiteral
            for #name #ty_generics #where_clause
        {
            fn to_code_literal(&self) -> ::elicitation::proc_macro2::TokenStream {
                #to_code_literal_body
            }

            fn type_tokens() -> ::elicitation::proc_macro2::TokenStream {
                ::elicitation::emit_code::CodeLiteralEmitter::type_tokens(#type_tokens_literal)
            }
        }
    };

    expanded.into()
}

/// Generate a complete `ToCodeLiteral` impl block for use inside other derive macros.
///
/// Called by `expand_named_struct`, `expand_tuple_struct`, `expand_unit_struct`,
/// and `expand_enum` to emit the impl alongside the `Elicitation` impl.
pub fn generate_to_code_literal_impl(
    name: &syn::Ident,
    data: &syn::Data,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let body = match data {
        syn::Data::Struct(data) => {
            match gen_struct_body(&quote! { #name }, &data.fields, false, false, None, None) {
                Ok(body) => body,
                Err(error) => return error.to_compile_error(),
            }
        }
        syn::Data::Enum(data) => gen_enum_body(name, &quote! { #name }, data),
        syn::Data::Union(_) => {
            return syn::Error::new_spanned(name, "ToCodeLiteral cannot be derived for unions")
                .to_compile_error();
        }
    };

    // Collect field types to add ToCodeLiteral bounds in where clause.
    // Skip fields marked with `#[skip]` — those use Default::default() and
    // don't need a ToCodeLiteral bound (and including them can cause overflow
    // for recursive types like `children: Vec<Self>`).
    let field_types: Vec<syn::Type> = match data {
        syn::Data::Struct(data) => {
            let mut field_types = Vec::new();
            for field in &data.fields {
                if has_skip_attr(&field.attrs) {
                    continue;
                }

                let options = match parse_field_options(&field.attrs) {
                    Ok(options) => options,
                    Err(error) => return error.to_compile_error(),
                };
                if !options.expr && options.to_tokens.is_none() {
                    field_types.push(field.ty.clone());
                }
            }
            field_types
        }
        syn::Data::Enum(data) => {
            let mut field_types = Vec::new();
            for variant in &data.variants {
                for field in &variant.fields {
                    if has_skip_attr(&field.attrs) {
                        continue;
                    }
                    let options = match parse_field_options(&field.attrs) {
                        Ok(options) => options,
                        Err(error) => return error.to_compile_error(),
                    };
                    if !options.expr && options.to_tokens.is_none() {
                        field_types.push(field.ty.clone());
                    }
                }
            }
            field_types
        }
        _ => vec![],
    };
    let tcl_bounds: Vec<_> = field_types
        .iter()
        .map(|ty| quote! { #ty: ::elicitation::emit_code::ToCodeLiteral })
        .collect();
    let existing: Vec<_> = match where_clause {
        Some(wc) => wc.predicates.iter().collect(),
        None => vec![],
    };
    let where_tokens = if existing.is_empty() && tcl_bounds.is_empty() {
        quote! {}
    } else {
        quote! { where #(#existing,)* #(#tcl_bounds,)* }
    };

    quote! {
        #[automatically_derived]
        impl #impl_generics ::elicitation::emit_code::ToCodeLiteral
            for #name #ty_generics #where_tokens
        {
            fn to_code_literal(&self) -> ::elicitation::proc_macro2::TokenStream {
                #body
            }

            fn type_tokens() -> ::elicitation::proc_macro2::TokenStream {
                ::elicitation::emit_code::CodeLiteralEmitter::type_tokens(stringify!(#name))
            }
        }
    }
}

#[derive(Default)]
struct ToCodeLiteralOptions {
    type_path: Option<TokenStream2>,
    transparent: bool,
    tuple: bool,
    raw_tuple: bool,
    default_update: bool,
    update_expr: Option<TokenStream2>,
    default_expr: Option<TokenStream2>,
}

#[derive(Default)]
struct ToCodeLiteralFieldOptions {
    rename: Option<syn::Ident>,
    expr: bool,
    optional: bool,
    to_tokens: Option<TokenStream2>,
}

#[derive(Default)]
struct ToCodeLiteralVariantOptions {
    tuple: bool,
}

fn parse_options(attrs: &[syn::Attribute]) -> syn::Result<ToCodeLiteralOptions> {
    let mut options = ToCodeLiteralOptions::default();

    for attr in attrs {
        if !attr.path().is_ident("to_code_literal") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("path") {
                let value = meta.value()?;
                let path = value.parse::<LitStr>()?;
                let parsed = path.value().parse::<TokenStream2>().map_err(|error| {
                    meta.error(format!(
                        "invalid to_code_literal path `{}`: {error}",
                        path.value()
                    ))
                })?;
                options.type_path = Some(parsed);
                return Ok(());
            }

            if meta.path.is_ident("transparent") {
                options.transparent = true;
                return Ok(());
            }

            if meta.path.is_ident("tuple") {
                options.tuple = true;
                return Ok(());
            }

            if meta.path.is_ident("raw_tuple") {
                options.raw_tuple = true;
                return Ok(());
            }

            if meta.path.is_ident("default_update") {
                options.default_update = true;
                return Ok(());
            }

            if meta.path.is_ident("update") {
                let value = meta.value()?;
                let expr = value.parse::<LitStr>()?;
                let parsed = expr.value().parse::<TokenStream2>().map_err(|error| {
                    meta.error(format!(
                        "invalid to_code_literal update `{}`: {error}",
                        expr.value()
                    ))
                })?;
                options.update_expr = Some(parsed);
                return Ok(());
            }

            if meta.path.is_ident("default_expr") {
                let value = meta.value()?;
                let expr = value.parse::<LitStr>()?;
                let parsed = expr.value().parse::<TokenStream2>().map_err(|error| {
                    meta.error(format!(
                        "invalid to_code_literal default_expr `{}`: {error}",
                        expr.value()
                    ))
                })?;
                options.default_expr = Some(parsed);
                return Ok(());
            }

            Err(meta.error(
                "unsupported to_code_literal option; expected `path = \"...\"`, `transparent`, `tuple`, `raw_tuple`, `default_update`, `update = \"...\"`, or `default_expr = \"...\"`",
            ))
        })?;
    }

    Ok(options)
}

fn parse_field_options(attrs: &[Attribute]) -> syn::Result<ToCodeLiteralFieldOptions> {
    let mut options = ToCodeLiteralFieldOptions::default();

    for attr in attrs {
        if !attr.path().is_ident("to_code_literal") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                let value = meta.value()?;
                let rename = value.parse::<LitStr>()?;
                options.rename = Some(syn::parse_str::<syn::Ident>(&rename.value()).map_err(
                    |error| meta.error(format!("invalid to_code_literal rename `{}`: {error}", rename.value())),
                )?);
                return Ok(());
            }

            if meta.path.is_ident("expr") {
                options.expr = true;
                return Ok(());
            }

            if meta.path.is_ident("optional") {
                options.optional = true;
                return Ok(());
            }

            if meta.path.is_ident("to_tokens") {
                let value = meta.value()?;
                let path = value.parse::<LitStr>()?;
                let parsed = path.value().parse::<TokenStream2>().map_err(|error| {
                    meta.error(format!(
                        "invalid to_code_literal to_tokens `{}`: {error}",
                        path.value()
                    ))
                })?;
                options.to_tokens = Some(parsed);
                return Ok(());
            }

            Err(meta.error(
                "unsupported field to_code_literal option; expected `rename = \"...\"`, `expr`, `optional`, or `to_tokens = \"...\"`",
            ))
        })?;
    }

    Ok(options)
}

fn parse_variant_options(attrs: &[Attribute]) -> syn::Result<ToCodeLiteralVariantOptions> {
    let mut options = ToCodeLiteralVariantOptions::default();

    for attr in attrs {
        if !attr.path().is_ident("to_code_literal") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("tuple") {
                options.tuple = true;
                return Ok(());
            }

            Err(meta.error("unsupported enum variant to_code_literal option; expected `tuple`"))
        })?;
    }

    Ok(options)
}

fn is_option_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "Option")
            .unwrap_or(false),
        _ => false,
    }
}

fn gen_transparent_body(fields: &Fields) -> syn::Result<TokenStream2> {
    match fields {
        Fields::Named(named) => {
            let present: Vec<_> = named
                .named
                .iter()
                .filter(|field| !has_skip_attr(&field.attrs))
                .collect();
            if present.len() != 1 {
                return Err(syn::Error::new_spanned(
                    &named.named,
                    "#[to_code_literal(transparent)] requires exactly one non-#[skip] field",
                ));
            }
            let field = present[0];
            let field_ident = field.ident.as_ref().expect("named field");
            let options = parse_field_options(&field.attrs)?;
            if options.optional {
                return Err(syn::Error::new_spanned(
                    field,
                    "#[to_code_literal(optional)] is not supported on transparent fields",
                ));
            }
            if options.expr && options.to_tokens.is_some() {
                return Err(syn::Error::new_spanned(
                    field,
                    "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                ));
            }
            let context = field_ident.to_string();
            let access = quote! { &self.#field_ident };
            let value_tokens = if let Some(to_tokens) = options.to_tokens {
                quote! { #to_tokens(#access) }
            } else if options.expr {
                quote! { ::elicitation::emit_code::parse_expr_tokens(#access, #context) }
            } else {
                quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#access) }
            };
            Ok(value_tokens)
        }
        Fields::Unnamed(unnamed) => {
            let present: Vec<_> = unnamed
                .unnamed
                .iter()
                .enumerate()
                .filter(|(_, field)| !has_skip_attr(&field.attrs))
                .collect();
            if present.len() != 1 {
                return Err(syn::Error::new_spanned(
                    &unnamed.unnamed,
                    "#[to_code_literal(transparent)] requires exactly one non-#[skip] field",
                ));
            }
            let (position, field) = present[0];
            let index = syn::Index::from(position);
            let options = parse_field_options(&field.attrs)?;
            if options.optional {
                return Err(syn::Error::new_spanned(
                    field,
                    "#[to_code_literal(optional)] is not supported on transparent fields",
                ));
            }
            if options.expr && options.to_tokens.is_some() {
                return Err(syn::Error::new_spanned(
                    field,
                    "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                ));
            }
            let context = format!("field_{position}");
            let access = quote! { &self.#index };
            let value_tokens = if let Some(to_tokens) = options.to_tokens {
                quote! { #to_tokens(#access) }
            } else if options.expr {
                quote! { ::elicitation::emit_code::parse_expr_tokens(#access, #context) }
            } else {
                quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#access) }
            };
            Ok(value_tokens)
        }
        Fields::Unit => Err(syn::Error::new_spanned(
            fields,
            "#[to_code_literal(transparent)] does not support unit structs",
        )),
    }
}

fn transparent_type_tokens(data: &Data) -> syn::Result<TokenStream2> {
    let ty = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => named
                .named
                .iter()
                .find(|field| !has_skip_attr(&field.attrs))
                .map(|field| field.ty.clone()),
            Fields::Unnamed(unnamed) => unnamed
                .unnamed
                .iter()
                .find(|field| !has_skip_attr(&field.attrs))
                .map(|field| field.ty.clone()),
            Fields::Unit => None,
        },
        _ => None,
    }
    .ok_or_else(|| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[to_code_literal(transparent)] requires exactly one non-#[skip] field",
        )
    })?;

    Ok(quote! { <#ty as ::elicitation::emit_code::ToCodeLiteral>::type_tokens() })
}

fn raw_tuple_type_tokens(data: &Data) -> syn::Result<TokenStream2> {
    let field_types = match data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(named) => named
                .named
                .iter()
                .filter(|field| !has_skip_attr(&field.attrs))
                .map(|field| field.ty.clone())
                .collect::<Vec<_>>(),
            Fields::Unnamed(_) | Fields::Unit => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "#[to_code_literal(raw_tuple)] only supports named structs",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "#[to_code_literal(raw_tuple)] only supports structs",
            ));
        }
    };

    let local_vars: Vec<_> = (0..field_types.len())
        .map(|idx| format_ident!("__tcl_type_{idx}"))
        .collect();
    let tuple_pattern = build_tuple_fields_pattern(&local_vars);
    let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
    let inner = TokenStream2::from(TokenTree::Group(paren_group));
    let quote_call = wrap_in_quote(inner);

    Ok(quote! {
        {
            #( let #local_vars = <#field_types as ::elicitation::emit_code::ToCodeLiteral>::type_tokens(); )*
            #quote_call
        }
    })
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

/// Build `#field0 #field1 ..Default::default()` as raw tokens for named field fragments.
fn build_named_field_fragments_pattern(
    local_vars: &[syn::Ident],
    update_expr: Option<&TokenStream2>,
) -> TokenStream2 {
    let mut tokens = TokenStream2::new();
    for local in local_vars {
        tokens.extend(std::iter::once(hash_punct()));
        tokens.extend(quote! { #local });
    }
    if let Some(update_expr) = update_expr {
        tokens.extend(quote! { ..#update_expr });
    }
    tokens
}

/// Wrap tokens in `::elicitation::quote::quote! { <inner> }`.
///
/// Uses the re-exported `quote` from `elicitation` so downstream crates
/// don't need `quote` as a direct dependency.
fn wrap_in_quote(inner: TokenStream2) -> TokenStream2 {
    let group = Group::new(Delimiter::Brace, inner);
    let mut result = quote! { ::elicitation::quote::quote! };
    result.extend(std::iter::once(TokenTree::Group(group)));
    result
}

/// Generate body for a struct (named, tuple, or unit).
///
/// Fields marked `#[skip]` emit `Default::default()` in the reconstructed
/// literal rather than calling `ToCodeLiteral::to_code_literal`, so their
/// types don't need a `ToCodeLiteral` bound (and recursive types like
/// `children: Vec<Self>` don't overflow the trait solver).
fn gen_struct_body(
    type_tokens: &TokenStream2,
    fields: &Fields,
    tuple_constructor: bool,
    raw_tuple: bool,
    update_expr: Option<&TokenStream2>,
    default_expr: Option<&TokenStream2>,
) -> syn::Result<TokenStream2> {
    match fields {
        Fields::Named(named) => {
            if tuple_constructor || raw_tuple {
                let all_fields: Vec<_> = named.named.iter().collect();
                let field_names: Vec<_> = all_fields
                    .iter()
                    .map(|f| f.ident.as_ref().expect("named field"))
                    .collect();
                let local_vars: Vec<_> = field_names
                    .iter()
                    .map(|f| format_ident!("__tcl_{f}"))
                    .collect();

                let bindings: Vec<TokenStream2> = all_fields
                    .iter()
                    .zip(field_names.iter())
                    .zip(local_vars.iter())
                    .map(|((field, fname), lvar)| -> syn::Result<TokenStream2> {
                        let options = parse_field_options(&field.attrs)?;
                        if options.rename.is_some() {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(rename = \"...\")] is not supported with tuple-emission options",
                            ));
                        }
                        if options.optional {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(optional)] is not supported with tuple-emission options",
                            ));
                        }
                        if options.expr && options.to_tokens.is_some() {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                            ));
                        }

                        if has_skip_attr(&field.attrs) {
                            return Ok(quote! {
                                let #lvar = ::elicitation::quote::quote! { ::std::default::Default::default() };
                            });
                        }

                        let context = fname.to_string();
                        let value_tokens = if let Some(to_tokens) = &options.to_tokens {
                            quote! { #to_tokens(&self.#fname) }
                        } else if options.expr {
                            quote! { ::elicitation::emit_code::parse_expr_tokens(&self.#fname, #context) }
                        } else {
                            quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#fname) }
                        };

                        Ok(quote! {
                            let #lvar = #value_tokens;
                        })
                    })
                    .collect::<syn::Result<_>>()?;

                let tuple_pattern = build_tuple_fields_pattern(&local_vars);
                let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
                let inner = if raw_tuple {
                    TokenStream2::from(TokenTree::Group(paren_group))
                } else {
                    let mut inner = quote! { #type_tokens };
                    inner.extend(std::iter::once(TokenTree::Group(paren_group)));
                    inner
                };
                let quote_call = wrap_in_quote(inner);

                return Ok(quote! {
                    #( #bindings )*
                    #quote_call
                });
            }

            let all_fields: Vec<_> = named.named.iter().collect();
            let field_names: Vec<_> = all_fields
                .iter()
                .map(|f| f.ident.as_ref().expect("named field"))
                .collect();
            let local_vars: Vec<_> = field_names
                .iter()
                .map(|f| format_ident!("__tcl_{f}"))
                .collect();
            let presence_vars: Vec<_> = field_names
                .iter()
                .map(|f| format_ident!("__tcl_has_{f}"))
                .collect();

            let bindings: Vec<TokenStream2> = all_fields
                .iter()
                .zip(field_names.iter())
                .zip(local_vars.iter().zip(presence_vars.iter()))
                .map(|((field, fname), (lvar, pvar))| -> syn::Result<TokenStream2> {
                    let options = parse_field_options(&field.attrs)?;
                    if options.expr && options.to_tokens.is_some() {
                        return Err(syn::Error::new_spanned(
                            field,
                            "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                        ));
                    }
                    let emitted_name = options.rename.as_ref().unwrap_or(fname);
                    let context = emitted_name.to_string();

                    if has_skip_attr(&field.attrs) {
                        return Ok(quote! {
                            let #pvar = true;
                            let #lvar = ::elicitation::quote::quote! { #emitted_name: Default::default(), };
                        });
                    }

                    if options.optional {
                        if !is_option_type(&field.ty) {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(optional)] requires an Option<T> field",
                            ));
                        }

                        let value_tokens = if let Some(to_tokens) = &options.to_tokens {
                            quote! { #to_tokens(value) }
                        } else if options.expr {
                            quote! { ::elicitation::emit_code::parse_expr_tokens(value, #context) }
                        } else {
                            quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(value) }
                        };

                        return Ok(quote! {
                            let #pvar = self.#fname.is_some();
                            let #lvar = match &self.#fname {
                                Some(value) => {
                                    let __tcl_value = #value_tokens;
                                    let mut __tcl_field =
                                        ::elicitation::quote::quote! { #emitted_name: };
                                    __tcl_field.extend(__tcl_value);
                                    __tcl_field.extend(::elicitation::quote::quote! { , });
                                    __tcl_field
                                }
                                None => ::elicitation::proc_macro2::TokenStream::new(),
                            };
                        });
                    }

                    let value_tokens = if let Some(to_tokens) = &options.to_tokens {
                        quote! {
                            #to_tokens(&self.#fname)
                        }
                    } else if options.expr {
                        quote! {
                            ::elicitation::emit_code::parse_expr_tokens(&self.#fname, #context)
                        }
                    } else {
                        quote! {
                            ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#fname)
                        }
                    };

                    Ok(quote! {
                        let #pvar = true;
                        let #lvar = {
                            let __tcl_value = #value_tokens;
                            let mut __tcl_field =
                                ::elicitation::quote::quote! { #emitted_name: };
                            __tcl_field.extend(__tcl_value);
                            __tcl_field.extend(::elicitation::quote::quote! { , });
                            __tcl_field
                        };
                    })
                })
                .collect::<syn::Result<_>>()?;

            let fields_pattern = build_named_field_fragments_pattern(&local_vars, update_expr);
            let brace_group = Group::new(Delimiter::Brace, fields_pattern);
            let mut inner = quote! { #type_tokens };
            inner.extend(std::iter::once(TokenTree::Group(brace_group)));
            let quote_call = wrap_in_quote(inner);

            let has_fields = if presence_vars.is_empty() {
                quote! { false }
            } else {
                quote! { #( #presence_vars )||* }
            };
            let emit_result = if let Some(default_expr) = default_expr {
                let default_expr = wrap_in_quote(default_expr.clone());
                quote! {
                    if !(#has_fields) {
                        #default_expr
                    } else {
                        #quote_call
                    }
                }
            } else {
                quote! { #quote_call }
            };

            Ok(quote! {
                #( #bindings )*
                #emit_result
            })
        }
        Fields::Unnamed(unnamed) => {
            if tuple_constructor || raw_tuple {
                return Err(syn::Error::new_spanned(
                    unnamed,
                    "tuple-emission to_code_literal options only support named structs",
                ));
            }
            if update_expr.is_some() || default_expr.is_some() {
                return Err(syn::Error::new_spanned(
                    unnamed,
                    "default-constructor to_code_literal options only support named structs",
                ));
            }
            let indices: Vec<_> = (0..unnamed.unnamed.len()).map(syn::Index::from).collect();
            let local_vars: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__tcl_{i}"))
                .collect();

            let tuple_pattern = build_tuple_fields_pattern(&local_vars);
            let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
            let mut inner = quote! { #type_tokens };
            inner.extend(std::iter::once(TokenTree::Group(paren_group)));
            let quote_call = wrap_in_quote(inner);

            Ok(quote! {
                #(
                    let #local_vars =
                        ::elicitation::emit_code::ToCodeLiteral::to_code_literal(&self.#indices);
                )*
                #quote_call
            })
        }
        Fields::Unit => {
            if update_expr.is_some() || default_expr.is_some() {
                return Err(syn::Error::new_spanned(
                    fields,
                    "default-constructor to_code_literal options only support named structs",
                ));
            }
            let quote_call = wrap_in_quote(quote! { #type_tokens });
            Ok(quote! { #quote_call })
        }
    }
}

/// Generate match body for an enum.
fn gen_enum_body(
    local_name: &syn::Ident,
    emitted_type: &TokenStream2,
    data: &syn::DataEnum,
) -> TokenStream2 {
    let arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| gen_variant_arm(local_name, emitted_type, variant))
        .collect();

    quote! {
        match self {
            #( #arms )*
        }
    }
}

/// Generate a single match arm for an enum variant.
fn gen_variant_arm(
    local_name: &syn::Ident,
    emitted_name: &TokenStream2,
    variant: &syn::Variant,
) -> TokenStream2 {
    let variant_name = &variant.ident;
    let fields = &variant.fields;
    let variant_options = match parse_variant_options(&variant.attrs) {
        Ok(options) => options,
        Err(error) => return error.to_compile_error(),
    };

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

            if variant_options.tuple {
                let bindings: Vec<_> = match named
                    .named
                    .iter()
                    .zip(field_names.iter())
                    .zip(local_vars.iter())
                    .map(|((field, fname), local)| -> syn::Result<TokenStream2> {
                        let options = parse_field_options(&field.attrs)?;
                        if options.rename.is_some() {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(rename = \"...\")] is not supported with #[to_code_literal(tuple)] on enum variants",
                            ));
                        }
                        if options.optional {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(optional)] is not supported with #[to_code_literal(tuple)] on enum variants",
                            ));
                        }
                        if options.expr && options.to_tokens.is_some() {
                            return Err(syn::Error::new_spanned(
                                field,
                                "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                            ));
                        }

                        if has_skip_attr(&field.attrs) {
                            return Ok(quote! {
                                let #local = ::elicitation::quote::quote! { ::std::default::Default::default() };
                            });
                        }

                        let context = fname.to_string();
                        let value_tokens = if let Some(to_tokens) = &options.to_tokens {
                            quote! { #to_tokens(#fname) }
                        } else if options.expr {
                            quote! { ::elicitation::emit_code::parse_expr_tokens(#fname, #context) }
                        } else {
                            quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#fname) }
                        };

                        Ok(quote! {
                            let #local = #value_tokens;
                        })
                    })
                    .collect::<syn::Result<Vec<_>>>()
                {
                    Ok(bindings) => bindings,
                    Err(error) => return error.to_compile_error(),
                };

                let tuple_pattern = build_tuple_fields_pattern(&local_vars);
                let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
                let mut inner = quote! { #emitted_name :: #variant_name };
                inner.extend(std::iter::once(TokenTree::Group(paren_group)));
                let quote_call = wrap_in_quote(inner);

                return quote! {
                    #local_name::#variant_name { #( #field_names ),* } => {
                        #( #bindings )*
                        #quote_call
                    }
                };
            }

            let fields_pattern = build_named_fields_pattern(&field_names, &local_vars);
            let brace_group = Group::new(Delimiter::Brace, fields_pattern);
            let mut inner = quote! { #emitted_name :: #variant_name };
            inner.extend(std::iter::once(TokenTree::Group(brace_group)));
            let quote_call = wrap_in_quote(inner);

            let value_bindings: Vec<_> = match named
                .named
                .iter()
                .zip(field_names.iter())
                .zip(local_vars.iter())
                .map(|((field, field_name), local)| -> syn::Result<TokenStream2> {
                    let options = parse_field_options(&field.attrs)?;
                    if options.expr && options.to_tokens.is_some() {
                        return Err(syn::Error::new_spanned(
                            field,
                            "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                        ));
                    }

                    if has_skip_attr(&field.attrs) {
                        return Ok(quote! {
                            let #local = ::elicitation::quote::quote! { ::std::default::Default::default() };
                        });
                    }

                    let context = options
                        .rename
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| field_name.to_string());
                    let value_tokens = if let Some(to_tokens) = &options.to_tokens {
                        quote! { #to_tokens(#field_name) }
                    } else if options.expr {
                        quote! { ::elicitation::emit_code::parse_expr_tokens(#field_name, #context) }
                    } else {
                        quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#field_name) }
                    };

                    Ok(quote! {
                        let #local = #value_tokens;
                    })
                })
                .collect::<syn::Result<Vec<_>>>()
            {
                Ok(bindings) => bindings,
                Err(error) => return error.to_compile_error(),
            };

            quote! {
                #local_name::#variant_name { #( #field_names ),* } => {
                    #( #value_bindings )*
                    #quote_call
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            if variant_options.tuple {
                return syn::Error::new_spanned(
                    variant,
                    "#[to_code_literal(tuple)] is only supported on named enum variants",
                )
                .to_compile_error();
            }
            let bindings: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__field_{i}"))
                .collect();
            let local_vars: Vec<_> = (0..unnamed.unnamed.len())
                .map(|i| format_ident!("__tcl_{i}"))
                .collect();
            let value_bindings: Vec<_> = match unnamed
                .unnamed
                .iter()
                .zip(bindings.iter())
                .zip(local_vars.iter())
                .enumerate()
                .map(|(i, ((field, binding), local))| -> syn::Result<TokenStream2> {
                    let options = parse_field_options(&field.attrs)?;
                    if options.rename.is_some() {
                        return Err(syn::Error::new_spanned(
                            field,
                            "#[to_code_literal(rename = \"...\")] is not supported on tuple variant fields",
                        ));
                    }
                    if options.optional {
                        return Err(syn::Error::new_spanned(
                            field,
                            "#[to_code_literal(optional)] is not supported on tuple variant fields",
                        ));
                    }
                    if options.expr && options.to_tokens.is_some() {
                        return Err(syn::Error::new_spanned(
                            field,
                            "#[to_code_literal(expr)] cannot be combined with `to_tokens`",
                        ));
                    }

                    if has_skip_attr(&field.attrs) {
                        return Ok(quote! {
                            let #local = ::elicitation::quote::quote! { ::std::default::Default::default() };
                        });
                    }

                    let context = format!("{variant_name}_{i}");
                    let value_tokens = if let Some(to_tokens) = &options.to_tokens {
                        quote! { #to_tokens(#binding) }
                    } else if options.expr {
                        quote! { ::elicitation::emit_code::parse_expr_tokens(#binding, #context) }
                    } else {
                        quote! { ::elicitation::emit_code::ToCodeLiteral::to_code_literal(#binding) }
                    };

                    Ok(quote! {
                        let #local = #value_tokens;
                    })
                })
                .collect::<syn::Result<Vec<_>>>()
            {
                Ok(bindings) => bindings,
                Err(error) => return error.to_compile_error(),
            };

            let tuple_pattern = build_tuple_fields_pattern(&local_vars);
            let paren_group = Group::new(Delimiter::Parenthesis, tuple_pattern);
            let mut inner = quote! { #emitted_name :: #variant_name };
            inner.extend(std::iter::once(TokenTree::Group(paren_group)));
            let quote_call = wrap_in_quote(inner);

            quote! {
                #local_name::#variant_name( #( #bindings ),* ) => {
                    #( #value_bindings )*
                    #quote_call
                }
            }
        }
        Fields::Unit => {
            if variant_options.tuple {
                return syn::Error::new_spanned(
                    variant,
                    "#[to_code_literal(tuple)] is only supported on named enum variants",
                )
                .to_compile_error();
            }
            let emitted_name_literal = emitted_name.to_string();
            let variant_name_literal = variant_name.to_string();
            quote! {
                #local_name::#variant_name => {
                    ::elicitation::emit_code::CodeLiteralEmitter::enum_unit_variant_literal(
                        #emitted_name_literal,
                        #variant_name_literal,
                    )
                }
            }
        }
    }
}
