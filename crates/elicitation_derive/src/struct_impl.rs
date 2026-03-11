//! Derive implementation for structs (Survey pattern).

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Field, Fields, punctuated::Punctuated, token::Comma};

/// Expand #[derive(Elicit)] for structs.
///
/// Generates implementations of:
/// - Prompt (with optional custom prompt from #[prompt] attribute)
/// - Survey (field metadata)
/// - Elicit (sequential field elicitation)
///
/// **Generic Support:**
/// Supports generic type parameters. All type parameters will have `Elicitation` bounds
/// added to ensure their fields can be elicited.
pub fn expand_struct(input: DeriveInput) -> TokenStream {
    // Dispatch to tuple struct handler before borrowing from input
    let data_struct = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unreachable!("expand_struct called on non-struct"),
    };
    if let Fields::Unnamed(f) = &data_struct.fields {
        // Need to clone unnamed before consuming input
        let unnamed = f.unnamed.clone();
        return expand_tuple_struct(input, unnamed);
    }
    if let Fields::Unit = &data_struct.fields {
        return expand_unit_struct(input);
    }

    let name = &input.ident;

    // Extract generics for trait implementations
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract custom prompt from #[prompt("...")] attribute
    let (custom_prompt, _) = extract_prompts(&input.attrs);
    // Extract spec attrs from the struct itself
    let spec_summary = extract_spec_summary(&input.attrs);
    let struct_spec_requires = extract_spec_requires(&input.attrs);

    let data_struct = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unreachable!("expand_struct called on non-struct"),
    };

    // Extract named fields (Unnamed already handled above)
    let fields = match &data_struct.fields {
        Fields::Named(f) => &f.named,
        Fields::Unnamed(_) => unreachable!("Unnamed already dispatched"),
        Fields::Unit => unreachable!("Unit already dispatched"),
    };

    // Parse field information - separate elicited and skipped fields
    let mut elicited_fields = Vec::new();
    let mut skipped_fields = Vec::new();

    for field in fields.iter() {
        let info = parse_field_info(field);
        if has_skip_attr(&field.attrs) {
            skipped_fields.push(info);
        } else {
            elicited_fields.push(info);
        }
    }

    let field_infos = elicited_fields;

    if field_infos.is_empty() {
        let error = syn::Error::new_spanned(
            name,
            "Elicit derive for structs requires at least one non-skipped field.",
        );
        return error.to_compile_error().into();
    }

    // Collect all unique style names across all fields
    let mut all_styles = std::collections::HashSet::new();
    for field in &field_infos {
        for style_name in field.styled_prompts.keys() {
            all_styles.insert(style_name.clone());
        }
    }
    let all_styles: Vec<String> = all_styles.into_iter().collect();

    // Generate Prompt impl
    let prompt_impl = generate_prompt_impl(
        name,
        custom_prompt,
        &impl_generics,
        &ty_generics,
        &where_clause,
    );

    // Generate Survey impl
    let survey_impl = generate_survey_impl(
        name,
        &field_infos,
        &impl_generics,
        &ty_generics,
        &where_clause,
    );

    // Generate Elicit impl (style-aware if styles present)
    let elicit_impl = if all_styles.is_empty() {
        generate_elicit_impl_simple(
            name,
            &field_infos,
            &skipped_fields,
            &impl_generics,
            &ty_generics,
            &where_clause,
        )
    } else {
        generate_elicit_impl_styled(
            name,
            &field_infos,
            &skipped_fields,
            &all_styles,
            &impl_generics,
            &ty_generics,
            &where_clause,
        )
    };

    // Generate ElicitIntrospect impl
    let introspect_impl =
        generate_introspect_impl(name, &impl_generics, &ty_generics, &where_clause);

    // Generate ElicitSpec impl (composed from field type specs + user attributes)
    // Skip for generic structs: inventory::submit! cannot register generic types.
    let elicit_spec_impl = if generics.params.is_empty() {
        generate_elicit_spec_impl(
            name,
            &field_infos,
            &spec_summary,
            &struct_spec_requires,
            &impl_generics,
            &ty_generics,
            &where_clause,
        )
    } else {
        quote! {}
    };

    // Generate TypeGraphKey submission for the structural registry.
    // Skip for generic structs (same reason as ElicitSpec).
    let graph_key_emission = if generics.params.is_empty() {
        generate_graph_key_emission(name)
    } else {
        quote! {}
    };

    // Note: Verification code is NOT generated for user types.
    // Users can write verification harnesses manually if needed.
    // Verification is primarily for elicitation's own contract types.

    let expanded = quote! {
        #prompt_impl
        #survey_impl
        #elicit_impl
        #introspect_impl
        #elicit_spec_impl
        #graph_key_emission
    };

    TokenStream::from(expanded)
}

/// Expand #[derive(Elicit)] for tuple structs (newtype and multi-field).
///
/// Generates Survey-pattern implementations where each positional field gets
/// the index string ("0", "1", ...) as its field name.  Construction uses
/// positional syntax: `Ok(Self(_field_0, _field_1, ...))`.
fn expand_tuple_struct(input: DeriveInput, unnamed: Punctuated<syn::Field, Comma>) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    if unnamed.is_empty() {
        let error = syn::Error::new_spanned(name, "Elicit derive requires at least one field.");
        return error.to_compile_error().into();
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let style_name = quote::format_ident!("{}Style", name);
    let name_str = name.to_string();

    let custom_prompt = extract_prompts(&input.attrs).0;
    let spec_summary = extract_spec_summary(&input.attrs);
    let struct_spec_requires = extract_spec_requires(&input.attrs);

    // Per-field variable idents (_field_0, _field_1, ...) and types
    let var_idents: Vec<_> = (0..unnamed.len())
        .map(|i| quote::format_ident!("_field_{}", i))
        .collect();
    let field_types: Vec<_> = unnamed.iter().map(|f| &f.ty).collect();
    let index_strs: Vec<String> = (0..unnamed.len()).map(|i| i.to_string()).collect();

    // Prompt impl
    let prompt_expr = match custom_prompt {
        Some(ref p) => quote! { Some(#p) },
        None => quote! { None },
    };

    // Survey fields
    let field_metadata: Vec<_> = index_strs
        .iter()
        .zip(field_types.iter())
        .map(|(idx, ty)| {
            quote! {
                elicitation::FieldInfo {
                    name: #idx,
                    prompt: None,
                    type_name: stringify!(#ty),
                }
            }
        })
        .collect();

    // Elicit statements: let _field_i = <Ti>::elicit(communicator).await?;
    let elicit_stmts: Vec<_> = var_idents
        .iter()
        .zip(index_strs.iter())
        .zip(field_types.iter())
        .map(|((var, idx), ty)| {
            quote! {
                tracing::debug!(field = #idx, "Eliciting field");
                let #var = <#ty>::elicit(communicator).await?;
            }
        })
        .collect();

    // ElicitSpec field blocks (same shape as named-field version)
    let field_category_blocks: Vec<TokenStream2> = index_strs
        .iter()
        .zip(field_types.iter())
        .map(|(idx, ty)| {
            let cat_name = format!("fields.{idx}");
            quote! {
                {
                    let type_id = std::any::TypeId::of::<#ty>();
                    let inherited: Vec<elicitation::SpecEntry> =
                        elicitation::lookup_type_spec_by_id(type_id)
                            .map(|spec| {
                                spec.categories().iter()
                                    .flat_map(|c| c.entries().iter().cloned())
                                    .collect()
                            })
                            .unwrap_or_default();
                    if !inherited.is_empty() {
                        Some(
                            elicitation::SpecCategoryBuilder::default()
                                .name(#cat_name.to_string())
                                .entries(inherited)
                                .build()
                                .expect("valid SpecCategory"),
                        )
                    } else {
                        None
                    }
                }
            }
        })
        .collect();

    let field_count = unnamed.len();
    let summary_expr = match spec_summary {
        Some(ref s) => quote! { #s.to_string() },
        None => {
            let auto = format!(
                "User-defined tuple type with {} field{}.",
                field_count,
                if field_count == 1 { "" } else { "s" }
            );
            quote! { #auto.to_string() }
        }
    };

    // struct-level requires (same pattern as named structs)
    let struct_requires_block = if struct_spec_requires.is_empty() {
        quote! {}
    } else {
        let exprs: Vec<&str> = struct_spec_requires.iter().map(String::as_str).collect();
        quote! {
            {
                let exprs: &[&str] = &[#(#exprs),*];
                let entries: Vec<elicitation::SpecEntry> = exprs.iter().enumerate().map(|(i, expr)| {
                    elicitation::SpecEntryBuilder::default()
                        .label(format!("struct_invariant_{}", i + 1))
                        .description(format!("Struct-level invariant: {}", expr))
                        .expression(Some((*expr).to_string()))
                        .build()
                        .expect("valid SpecEntry")
                }).collect();
                categories.push(
                    elicitation::SpecCategoryBuilder::default()
                        .name("requires".to_string())
                        .entries(entries)
                        .build()
                        .expect("valid SpecCategory"),
                );
            }
        }
    };

    let elicit_spec_impl = if generics.params.is_empty() {
        quote! {
            impl elicitation::ElicitSpec for #name {
                fn type_spec() -> elicitation::TypeSpec {
                    let mut categories: Vec<elicitation::SpecCategory> = Vec::new();
                    #(
                        if let Some(cat) = #field_category_blocks {
                            categories.push(cat);
                        }
                    )*
                    #struct_requires_block
                    elicitation::TypeSpecBuilder::default()
                        .type_name(#name_str.to_string())
                        .summary(#summary_expr)
                        .categories(categories)
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            elicitation::inventory::submit!(elicitation::TypeSpecInventoryKey::new(
                #name_str,
                <#name as elicitation::ElicitSpec>::type_spec,
                std::any::TypeId::of::<#name>
            ));
        }
    } else {
        quote! {}
    };

    let graph_key_emission = if generics.params.is_empty() {
        generate_graph_key_emission(name)
    } else {
        quote! {}
    };

    #[cfg(feature = "proofs")]
    let proof_methods = quote! {
        fn kani_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(ts.extend(<#field_types as elicitation::Elicitation>::kani_proof());)*
            ts
        }

        fn verus_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(ts.extend(<#field_types as elicitation::Elicitation>::verus_proof());)*
            ts
        }

        fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(ts.extend(<#field_types as elicitation::Elicitation>::creusot_proof());)*
            ts
        }
    };
    #[cfg(not(feature = "proofs"))]
    let proof_methods = quote! {};

    let expanded = quote! {
        impl elicitation::Prompt for #name #ty_generics #where_clause {
            fn prompt() -> Option<&'static str> {
                #prompt_expr
            }
        }

        impl #impl_generics elicitation::Survey for #name #ty_generics #where_clause {
            fn fields() -> Vec<elicitation::FieldInfo> {
                vec![#(#field_metadata),*]
            }
        }

        /// Style enum for this type (default-only).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum #style_name {
            /// Default elicitation style.
            #[default]
            Default,
        }

        impl elicitation::Prompt for #style_name {
            fn prompt() -> Option<&'static str> { None }
        }

        impl elicitation::Elicitation for #style_name {
            type Style = #style_name;
            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self::Default)
            }
        }

        #[allow(unexpected_cfgs)]
        impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
            type Style = #style_name;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                tracing::debug!(struct_name = #name_str, "Eliciting tuple struct");
                #(#elicit_stmts)*
                Ok(Self(#(#var_idents),*))
            }

            #proof_methods
        }

        impl #impl_generics elicitation::ElicitIntrospect for #name #ty_generics #where_clause {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Survey
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: #name_str,
                    description: <Self as elicitation::Prompt>::prompt(),
                    details: elicitation::PatternDetails::Survey {
                        fields: <Self as elicitation::Survey>::fields(),
                    },
                }
            }
        }

        #elicit_spec_impl
        #graph_key_emission
    };

    TokenStream::from(expanded)
}

/// Expand #[derive(Elicit)] for unit structs.
///
/// Unit structs have exactly one value, so `elicit()` returns `Ok(Self)` immediately
/// with no user interaction needed.
fn expand_unit_struct(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let style_name = quote::format_ident!("{}Style", name);
    let name_str = name.to_string();

    let custom_prompt = extract_prompts(&input.attrs).0;
    let spec_summary = extract_spec_summary(&input.attrs);
    let struct_spec_requires = extract_spec_requires(&input.attrs);

    let prompt_expr = match custom_prompt {
        Some(ref p) => quote! { Some(#p) },
        None => quote! { None },
    };

    let summary_expr = match spec_summary {
        Some(ref s) => quote! { #s.to_string() },
        None => quote! { "Unit type with a single value.".to_string() },
    };

    let struct_requires_block = if struct_spec_requires.is_empty() {
        quote! {}
    } else {
        let exprs: Vec<&str> = struct_spec_requires.iter().map(String::as_str).collect();
        quote! {
            {
                let exprs: &[&str] = &[#(#exprs),*];
                let entries: Vec<elicitation::SpecEntry> = exprs.iter().enumerate().map(|(i, expr)| {
                    elicitation::SpecEntryBuilder::default()
                        .label(format!("invariant_{}", i + 1))
                        .description(format!("Invariant: {}", expr))
                        .expression(Some((*expr).to_string()))
                        .build()
                        .expect("valid SpecEntry")
                }).collect();
                categories.push(
                    elicitation::SpecCategoryBuilder::default()
                        .name("requires".to_string())
                        .entries(entries)
                        .build()
                        .expect("valid SpecCategory"),
                );
            }
        }
    };

    let elicit_spec_impl = if generics.params.is_empty() {
        quote! {
            impl elicitation::ElicitSpec for #name {
                fn type_spec() -> elicitation::TypeSpec {
                    let mut categories: Vec<elicitation::SpecCategory> = Vec::new();
                    #struct_requires_block
                    elicitation::TypeSpecBuilder::default()
                        .type_name(#name_str.to_string())
                        .summary(#summary_expr)
                        .categories(categories)
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            elicitation::inventory::submit!(elicitation::TypeSpecInventoryKey::new(
                #name_str,
                <#name as elicitation::ElicitSpec>::type_spec,
                std::any::TypeId::of::<#name>
            ));
        }
    } else {
        quote! {}
    };

    let graph_key_emission = if generics.params.is_empty() {
        generate_graph_key_emission(name)
    } else {
        quote! {}
    };

    #[cfg(feature = "proofs")]
    let proof_methods = quote! {
        fn kani_proof() -> elicitation::proc_macro2::TokenStream {
            elicitation::proc_macro2::TokenStream::new()
        }

        fn verus_proof() -> elicitation::proc_macro2::TokenStream {
            elicitation::proc_macro2::TokenStream::new()
        }

        fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
            elicitation::proc_macro2::TokenStream::new()
        }
    };
    #[cfg(not(feature = "proofs"))]
    let proof_methods = quote! {};

    let expanded = quote! {
        impl elicitation::Prompt for #name #ty_generics #where_clause {
            fn prompt() -> Option<&'static str> {
                #prompt_expr
            }
        }

        impl #impl_generics elicitation::Survey for #name #ty_generics #where_clause {
            fn fields() -> Vec<elicitation::FieldInfo> {
                vec![]
            }
        }

        /// Style enum for this type (default-only).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum #style_name {
            /// Default elicitation style.
            #[default]
            Default,
        }

        impl elicitation::Prompt for #style_name {
            fn prompt() -> Option<&'static str> { None }
        }

        impl elicitation::Elicitation for #style_name {
            type Style = #style_name;
            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self::Default)
            }
        }

        #[allow(unexpected_cfgs)]
        impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
            type Style = #style_name;

            #[tracing::instrument(skip(_communicator))]
            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                tracing::debug!(struct_name = #name_str, "Eliciting unit struct");
                Ok(Self)
            }

            #proof_methods
        }

        impl #impl_generics elicitation::ElicitIntrospect for #name #ty_generics #where_clause {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Survey
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: #name_str,
                    description: <Self as elicitation::Prompt>::prompt(),
                    details: elicitation::PatternDetails::Survey {
                        fields: vec![],
                    },
                }
            }
        }

        #elicit_spec_impl
        #graph_key_emission
    };

    TokenStream::from(expanded)
}

/// Field information for code generation.
struct FieldInfo {
    ident: syn::Ident,
    ty: syn::Type,
    default_prompt: Option<String>,
    styled_prompts: std::collections::HashMap<String, String>, // style_name -> prompt_text
    /// Extra requires expressions from `#[spec_requires(expr)]` on this field.
    spec_requires: Vec<String>,
}

/// Parse field information from a Field.
fn parse_field_info(field: &Field) -> FieldInfo {
    let (default_prompt, styled_prompts) = extract_prompts(&field.attrs);
    let spec_requires = extract_spec_requires(&field.attrs);

    FieldInfo {
        ident: field.ident.clone().expect("Named field has ident"),
        ty: field.ty.clone(),
        default_prompt,
        styled_prompts,
        spec_requires,
    }
}

/// Extract prompts from attributes.
/// Returns (default_prompt, style_specific_prompts).
fn extract_prompts(
    attrs: &[syn::Attribute],
) -> (Option<String>, std::collections::HashMap<String, String>) {
    let mut default_prompt = None;
    let mut styled_prompts = std::collections::HashMap::new();

    for attr in attrs {
        if !attr.path().is_ident("prompt") {
            continue;
        }

        // Parse the attribute arguments
        let parsed = attr.parse_args_with(|input: syn::parse::ParseStream| {
            // First argument: prompt text (required)
            let prompt_text: syn::LitStr = input.parse()?;

            // Check if there's a comma for additional arguments
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;

                // Parse: style = "name"
                if input.peek(syn::Ident) {
                    let ident: syn::Ident = input.parse()?;
                    if ident == "style" {
                        input.parse::<syn::Token![=]>()?;
                        let style_name: syn::LitStr = input.parse()?;
                        return Ok((prompt_text.value(), Some(style_name.value())));
                    }
                }
            }

            Ok((prompt_text.value(), None))
        });

        match parsed {
            Ok((prompt, Some(style))) => {
                styled_prompts.insert(style, prompt);
            }
            Ok((prompt, None)) => {
                default_prompt = Some(prompt);
            }
            Err(_) => {
                // Failed to parse, skip this attribute
                continue;
            }
        }
    }

    (default_prompt, styled_prompts)
}

/// Check if field has #[skip] attribute.
fn has_skip_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("skip"))
}

/// Extract `#[spec_requires(expr, ...)]` attribute values from a list of attributes.
///
/// Each `#[spec_requires(...)]` contributes one or more expression strings.
fn extract_spec_requires(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut exprs = Vec::new();
    for attr in attrs {
        if !attr.path().is_ident("spec_requires") {
            continue;
        }
        // Parse the token stream inside the attribute as comma-separated expressions.
        let tokens = attr.meta.require_list().map(|l| l.tokens.clone());
        if let Ok(ts) = tokens {
            // Split on commas at depth 0 and collect each fragment as a string.
            let mut current = proc_macro2::TokenStream::new();
            let mut depth: usize = 0;
            for tt in ts {
                match &tt {
                    proc_macro2::TokenTree::Group(g) => {
                        depth += 1;
                        current.extend(std::iter::once(tt.clone()));
                        let _ = g; // used implicitly via depth tracking
                        depth -= 1;
                    }
                    proc_macro2::TokenTree::Punct(p) if p.as_char() == ',' && depth == 0 => {
                        let s = current.to_string().trim().to_string();
                        if !s.is_empty() {
                            exprs.push(s);
                        }
                        current = proc_macro2::TokenStream::new();
                    }
                    _ => current.extend(std::iter::once(tt.clone())),
                }
            }
            let s = current.to_string().trim().to_string();
            if !s.is_empty() {
                exprs.push(s);
            }
        }
    }
    exprs
}

/// Extract `#[spec_summary = "..."]` from struct-level attributes.
///
/// Returns `None` if no such attribute is present.
fn extract_spec_summary(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("spec_summary") {
            continue;
        }
        if let syn::Meta::NameValue(syn::MetaNameValue {
            value:
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }),
            ..
        }) = &attr.meta
        {
            return Some(s.value());
        }
    }
    None
}

/// Generate Prompt implementation.
fn generate_prompt_impl(
    name: &syn::Ident,
    custom_prompt: Option<String>,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    if let Some(prompt) = custom_prompt {
        quote! {
            impl #impl_generics elicitation::Prompt for #name #ty_generics #where_clause {
                fn prompt() -> Option<&'static str> {
                    Some(#prompt)
                }
            }
        }
    } else {
        let default_prompt = format!("Let's create a {}:", name);
        quote! {
            impl #impl_generics elicitation::Prompt for #name #ty_generics #where_clause {
                fn prompt() -> Option<&'static str> {
                    Some(#default_prompt)
                }
            }
        }
    }
}

/// Generate Survey implementation.
fn generate_survey_impl(
    name: &syn::Ident,
    field_infos: &[FieldInfo],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let field_metadata: Vec<_> = field_infos
        .iter()
        .map(|info| {
            let field_name = info.ident.to_string();
            let field_ty = &info.ty;
            let prompt_expr = match &info.default_prompt {
                Some(p) => quote! { Some(#p) },
                None => quote! { None },
            };

            // Generate inline FieldInfo construction with stringify! in generated code
            quote! {
                elicitation::FieldInfo {
                    name: #field_name,
                    prompt: #prompt_expr,
                    type_name: stringify!(#field_ty),
                }
            }
        })
        .collect();

    quote! {
        impl #impl_generics elicitation::Survey for #name #ty_generics #where_clause {
            fn fields() -> Vec<elicitation::FieldInfo> {
                vec![#(#field_metadata),*]
            }
        }
    }
}

/// Generate simple Elicit implementation (no styles).
fn generate_elicit_impl_simple(
    name: &syn::Ident,
    elicited_fields: &[FieldInfo],
    skipped_fields: &[FieldInfo],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let style_name = quote::format_ident!("{}Style", name);
    let elicited_names: Vec<_> = elicited_fields.iter().map(|info| &info.ident).collect();
    let elicited_types: Vec<_> = elicited_fields.iter().map(|info| &info.ty).collect();

    let elicit_statements: Vec<_> = elicited_names
        .iter()
        .zip(elicited_types.iter())
        .map(|(name, ty)| {
            let name_str = name.to_string();
            quote! {
                tracing::debug!(field = #name_str, "Eliciting field");
                let #name = <#ty>::elicit(communicator).await?;
            }
        })
        .collect();

    // For skipped fields, use Default::default()
    let skipped_names: Vec<_> = skipped_fields.iter().map(|info| &info.ident).collect();
    let skipped_defaults: Vec<_> = skipped_names
        .iter()
        .map(|name| {
            quote! {
                #name: Default::default()
            }
        })
        .collect();

    // Combine elicited and skipped field assignments
    let all_field_assignments = if skipped_fields.is_empty() {
        quote! { #(#elicited_names),* }
    } else {
        quote! {
            #(#elicited_names,)*
            #(#skipped_defaults),*
        }
    };

    #[cfg(feature = "proofs")]
    let proof_methods = quote! {
        fn kani_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#elicited_types as elicitation::Elicitation>::kani_proof());
            )*
            ts
        }

        fn verus_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#elicited_types as elicitation::Elicitation>::verus_proof());
            )*
            ts
        }

        fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#elicited_types as elicitation::Elicitation>::creusot_proof());
            )*
            ts
        }
    };
    #[cfg(not(feature = "proofs"))]
    let proof_methods = quote! {};

    quote! {
        /// Style enum for this type (default-only).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub enum #style_name {
            /// Default elicitation style.
            #[default]
            Default,
        }

        impl elicitation::Prompt for #style_name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for #style_name {
            type Style = #style_name;

            async fn elicit<C: elicitation::ElicitCommunicator>(_communicator: &C) -> elicitation::ElicitResult<Self> {
                Ok(Self::Default)
            }
        }

        #[allow(unexpected_cfgs)]
        impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
            type Style = #style_name;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                tracing::debug!(struct_name = stringify!(#name), "Eliciting struct");
                #(#elicit_statements)*
                Ok(Self {
                    #all_field_assignments
                })
            }

            #proof_methods
        }
    }
}

/// Generate styled Elicit implementation (with style selection).
fn generate_elicit_impl_styled(
    name: &syn::Ident,
    elicited_fields: &[FieldInfo],
    skipped_fields: &[FieldInfo],
    all_styles: &[String],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    // Generate style enum name
    let style_enum_name = syn::Ident::new(&format!("{}ElicitStyle", name), name.span());

    // Generate style enum variants
    let style_variants: Vec<_> = std::iter::once("Default".to_string())
        .chain(all_styles.iter().map(|s| capitalize_first(s)))
        .map(|variant_name| syn::Ident::new(&variant_name, name.span()))
        .collect();

    let style_labels: Vec<_> = std::iter::once("default".to_string())
        .chain(all_styles.iter().cloned())
        .collect();

    // Generate from_label match arms
    let from_label_arms: Vec<_> = style_variants
        .iter()
        .zip(style_labels.iter())
        .map(|(variant, label)| {
            quote! {
                #label => Some(Self::#variant),
            }
        })
        .collect();

    // Generate field elicitation with style matching
    let field_elicit_statements: Vec<_> = elicited_fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_ty = &field.ty;
            let field_name_str = field_name.to_string();

            // Build match arms for each style
            let mut match_arms = Vec::new();

            // Add styled prompts
            for (style_name, prompt_text) in &field.styled_prompts {
                let style_variant = syn::Ident::new(&capitalize_first(style_name), name.span());
                match_arms.push(quote! {
                    #style_enum_name::#style_variant => #prompt_text,
                });
            }

            // Add default fallback
            let default_prompt = field.default_prompt.as_deref()
                .unwrap_or(field_name_str.as_str());
            match_arms.push(quote! {
                _ => #default_prompt,
            });

            // Check type and generate appropriate inline elicitation
            let type_path = match field_ty {
                syn::Type::Path(p) => Some(p),
                _ => None,
            };

            let last_segment = type_path.and_then(|p| p.path.segments.last());
            let type_ident = last_segment.map(|seg| &seg.ident);

            // Determine if this type supports inline elicitation
            let supports_inline = if let Some(ident) = type_ident {
                let ident_str = ident.to_string();
                matches!(ident_str.as_str(),
                    "String" | "bool" |
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                    "f32" | "f64"
                )
            } else {
                false
            };

            if supports_inline && !field.styled_prompts.is_empty() {
                let type_ident = type_ident.unwrap();
                let type_str = type_ident.to_string();

                // Generate inline elicitation based on type
                match type_str.as_str() {
                    "String" => {
                        // String: use send_prompt directly
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting string field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let #field_name = communicator.send_prompt(prompt).await?;
                        }
                    }
                    "bool" => {
                        // Boolean: use send_prompt and parse
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting bool field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let response = communicator.send_prompt(prompt).await?;
                            let #field_name = response.trim().parse::<bool>()
                                .map_err(|e| elicitation::ElicitError::new(
                                    elicitation::ElicitErrorKind::ParseError(
                                        format!("Failed to parse boolean: {}", e)
                                    )
                                ))?;
                        }
                    }
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
                        // Integer types: use send_prompt and parse
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting integer field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let response = communicator.send_prompt(prompt).await?;
                            let #field_name = response.trim().parse::<#field_ty>()
                                .map_err(|e| elicitation::ElicitError::new(
                                    elicitation::ElicitErrorKind::ParseError(
                                        format!("Failed to parse {}: {}", stringify!(#field_ty), e)
                                    )
                                ))?;
                        }
                    }
                    "f32" | "f64" => {
                        // Float types: use send_prompt and parse
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting float field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let response = communicator.send_prompt(prompt).await?;
                            let #field_name = response.trim().parse::<#field_ty>()
                                .map_err(|e| elicitation::ElicitError::new(
                                    elicitation::ElicitErrorKind::ParseError(
                                        format!("Failed to parse {}: {}", stringify!(#field_ty), e)
                                    )
                                ))?;
                        }
                    }
                    _ => {
                        // Fallback for unsupported types
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting field (no inline style support for this type)");
                            let #field_name = <#field_ty>::elicit(communicator).await?;
                        }
                    }
                }
            } else {
                // For complex types or fields without styled prompts, fall back to their own elicit()
                quote! {
                    tracing::debug!(field = #field_name_str, "Eliciting field via standard elicit()");
                    let #field_name = <#field_ty>::elicit(communicator).await?;
                }
            }
        })
        .collect();

    // For skipped fields, use Default::default()
    let skipped_names: Vec<_> = skipped_fields.iter().map(|info| &info.ident).collect();
    let skipped_defaults: Vec<_> = skipped_names
        .iter()
        .map(|name| {
            quote! {
                #name: Default::default()
            }
        })
        .collect();

    let elicited_names: Vec<_> = elicited_fields.iter().map(|info| &info.ident).collect();

    // Combine elicited and skipped field assignments
    let all_field_assignments = if skipped_fields.is_empty() {
        quote! { #(#elicited_names),* }
    } else {
        quote! {
            #(#elicited_names,)*
            #(#skipped_defaults),*
        }
    };

    #[cfg(feature = "proofs")]
    let elicited_types: Vec<_> = elicited_fields.iter().map(|info| &info.ty).collect();
    #[cfg(feature = "proofs")]
    let proof_methods = quote! {
        fn kani_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#elicited_types as elicitation::Elicitation>::kani_proof());
            )*
            ts
        }

        fn verus_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#elicited_types as elicitation::Elicitation>::verus_proof());
            )*
            ts
        }

        fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#elicited_types as elicitation::Elicitation>::creusot_proof());
            )*
            ts
        }
    };
    #[cfg(not(feature = "proofs"))]
    let proof_methods = quote! {};

    // Generate enum with first variant as default
    let default_variant = &style_variants[0];
    let other_variants = &style_variants[1..];

    quote! {
        // Generate style selection enum
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        enum #style_enum_name {
            #[default]
            #default_variant,
            #(#other_variants),*
        }

        impl elicitation::Prompt for #style_enum_name {
            fn prompt() -> Option<&'static str> {
                Some("Select elicitation style:")
            }
        }

        impl elicitation::Select for #style_enum_name {
            fn options() -> &'static [Self] {
                &[#(Self::#style_variants),*]
            }

            fn labels() -> &'static [&'static str] {
                &[#(#style_labels),*]
            }

            fn from_label(label: &str) -> Option<Self> {
                match label {
                    #(#from_label_arms)*
                    _ => None,
                }
            }
        }

        impl elicitation::Elicitation for #style_enum_name {
            type Style = #style_enum_name;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let prompt = <Self as elicitation::Prompt>::prompt().unwrap();
                tracing::debug!("Eliciting style selection");

                // Use send_prompt for server-side compatibility
                let options_text = <Self as elicitation::Select>::labels().iter()
                    .enumerate()
                    .map(|(i, label)| format!("{}. {}", i + 1, label))
                    .collect::<Vec<_>>()
                    .join("\n");

                let full_prompt = format!("{}\n\nOptions:\n{}\n\nRespond with the number (1-{}) or exact label:",
                    prompt, options_text, <Self as elicitation::Select>::labels().len());

                let response = communicator.send_prompt(&full_prompt).await?;

                // Parse response - try as number first, then as label
                let selected = response.trim();
                let label = if let Ok(num) = selected.parse::<usize>() {
                    // User gave a number (1-indexed)
                    let labels = <Self as elicitation::Select>::labels();
                    if num > 0 && num <= labels.len() {
                        labels[num - 1]
                    } else {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::InvalidOption {
                                value: selected.to_string(),
                                options: labels.join(", "),
                            }
                        ));
                    }
                } else {
                    // User gave a label directly
                    selected
                };

                <Self as elicitation::Select>::from_label(label).ok_or_else(|| {
                    elicitation::ElicitError::new(elicitation::ElicitErrorKind::InvalidSelection(label))
                })
            }
        }

        #[allow(unexpected_cfgs)]
        impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
            type Style = #style_enum_name;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                tracing::debug!(struct_name = stringify!(#name), "Eliciting struct with style");

                // Step 1: Get style (use pre-set or elicit)
                let elicit_style = communicator.style_or_elicit::<#name #ty_generics>().await?;
                tracing::debug!(?elicit_style, "Style selected");

                // Step 2: Elicit fields with chosen style
                #(#field_elicit_statements)*

                Ok(Self {
                    #all_field_assignments
                })
            }

            #proof_methods
        }
    }
}

/// Generate ElicitIntrospect implementation for a struct.
fn generate_introspect_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let name_str = name.to_string();

    quote! {
        impl #impl_generics elicitation::ElicitIntrospect for #name #ty_generics #where_clause {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Survey
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: #name_str,
                    description: <Self as elicitation::Prompt>::prompt(),
                    details: elicitation::PatternDetails::Survey {
                        fields: <Self as elicitation::Survey>::fields(),
                    },
                }
            }
        }
    }
}

/// Capitalize first character of a string.
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

/// Generate `ElicitSpec` impl for a derived struct.
///
/// The composed spec has:
/// - One `"fields.<name>"` sub-category per non-skipped field, populated by a runtime
///   `lookup_type_spec_by_id` call on the field's type (plus any `#[spec_requires]` extras).
/// - An optional top-level `"requires"` category for struct-level `#[spec_requires]` entries.
/// - A summary from `#[spec_summary = "..."]` or an auto-generated fallback.
/// - An `inventory::submit!` registration so `lookup_type_spec("MyType")` works.
fn generate_elicit_spec_impl(
    name: &syn::Ident,
    field_infos: &[FieldInfo],
    spec_summary: &Option<String>,
    struct_spec_requires: &[String],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let name_str = name.to_string();
    let field_count = field_infos.len();

    let summary_expr = match spec_summary {
        Some(s) => quote! { #s.to_string() },
        None => {
            let auto = format!(
                "User-defined type with {} field{}.",
                field_count,
                if field_count == 1 { "" } else { "s" }
            );
            quote! { #auto.to_string() }
        }
    };

    // Build one block per field that resolves to an `Option<SpecCategory>`
    let field_category_blocks: Vec<TokenStream2> = field_infos.iter().map(|f| {
        let field_name = f.ident.to_string();
        let cat_name = format!("fields.{field_name}");
        let ty = &f.ty;
        let extra_exprs: Vec<&str> = f.spec_requires.iter().map(String::as_str).collect();
        let extra_count = extra_exprs.len();
        let extra_label = format!("{field_name}_invariant");

        quote! {
            {
                let type_id = std::any::TypeId::of::<#ty>();
                let inherited: Vec<elicitation::SpecEntry> = elicitation::lookup_type_spec_by_id(type_id)
                    .map(|spec| {
                        spec.categories().iter()
                            .flat_map(|c| c.entries().iter().cloned())
                            .collect()
                    })
                    .unwrap_or_default();

                let mut entries = inherited;

                // Append user #[spec_requires] extras for this field
                let extra_exprs: &[&str] = &[#(#extra_exprs),*];
                for (i, expr) in extra_exprs.iter().enumerate() {
                    let label = if #extra_count == 1 {
                        #extra_label.to_string()
                    } else {
                        format!("{}_{}", #extra_label, i + 1)
                    };
                    entries.push(
                        elicitation::SpecEntryBuilder::default()
                            .label(label)
                            .description(format!("Additional invariant: {}", expr))
                            .expression(Some((*expr).to_string()))
                            .build()
                            .expect("valid SpecEntry"),
                    );
                }

                if !entries.is_empty() {
                    Some(
                        elicitation::SpecCategoryBuilder::default()
                            .name(#cat_name.to_string())
                            .entries(entries)
                            .build()
                            .expect("valid SpecCategory"),
                    )
                } else {
                    None
                }
            }
        }
    }).collect();

    // Build struct-level "requires" entries from #[spec_requires] on the struct
    let struct_requires_block = if struct_spec_requires.is_empty() {
        quote! {}
    } else {
        let exprs: Vec<&str> = struct_spec_requires.iter().map(String::as_str).collect();
        let count = exprs.len();
        quote! {
            {
                let exprs: &[&str] = &[#(#exprs),*];
                let entries: Vec<elicitation::SpecEntry> = exprs.iter().enumerate().map(|(i, expr)| {
                    let label = if #count == 1 {
                        "invariant".to_string()
                    } else {
                        format!("invariant_{}", i + 1)
                    };
                    elicitation::SpecEntryBuilder::default()
                        .label(label)
                        .description(format!("Struct-level invariant: {}", expr))
                        .expression(Some((*expr).to_string()))
                        .build()
                        .expect("valid SpecEntry")
                }).collect();
                categories.push(
                    elicitation::SpecCategoryBuilder::default()
                        .name("requires".to_string())
                        .entries(entries)
                        .build()
                        .expect("valid SpecCategory"),
                );
            }
        }
    };

    quote! {
        impl #impl_generics elicitation::ElicitSpec for #name #ty_generics #where_clause {
            fn type_spec() -> elicitation::TypeSpec {
                let mut categories: Vec<elicitation::SpecCategory> = Vec::new();

                // Per-field sub-categories
                #(
                    if let Some(cat) = #field_category_blocks {
                        categories.push(cat);
                    }
                )*

                // Struct-level requires
                #struct_requires_block

                elicitation::TypeSpecBuilder::default()
                    .type_name(#name_str.to_string())
                    .summary(#summary_expr)
                    .categories(categories)
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        elicitation::inventory::submit!(elicitation::TypeSpecInventoryKey::new(
            #name_str,
            <#name #ty_generics as elicitation::ElicitSpec>::type_spec,
            std::any::TypeId::of::<#name #ty_generics>
        ));
    }
}

/// Emit an `inventory::submit!` call registering this struct in the
/// `TypeGraphKey` structural registry. Gated on `cfg(feature = "graph")`.
///
/// Called for all non-generic structs (named, tuple, unit).
fn generate_graph_key_emission(name: &syn::Ident) -> TokenStream2 {
    let name_str = name.to_string();
    quote! {
        #[cfg(feature = "graph")]
        elicitation::inventory::submit!(elicitation::TypeGraphKey::new(
            #name_str,
            <#name as elicitation::ElicitIntrospect>::metadata,
        ));
    }
}
