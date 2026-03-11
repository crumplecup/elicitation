//! Derive implementation for enums (Select pattern).

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Fields};

/// Information about an enum variant and its fields.
struct VariantInfo {
    ident: syn::Ident,
    fields: VariantFields,
}

/// Classification of variant field types.
enum VariantFields {
    Unit,
    Tuple(Vec<syn::Type>),
    Struct(Vec<FieldInfo>),
}

/// Information about a single field in a struct variant.
struct FieldInfo {
    ident: syn::Ident,
    ty: syn::Type,
}

/// Expand #[derive(Elicit)] for enums.
///
/// Generates implementations of:
/// - Prompt (with optional custom prompt from #[prompt] attribute)
/// - Select (options, labels, from_label)
/// - Elicit (calls elicit_select MCP tool, then elicits fields)
///
/// Supports unit variants, tuple variants, and struct variants.
///
/// **Generic Support:**
/// Supports generic type parameters. All type parameters will have `Elicitation` bounds
/// added to ensure their fields can be elicited.
pub fn expand_enum(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    // Extract generics for trait implementations
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract custom prompt from #[prompt("...")] attribute
    let custom_prompt = extract_prompt_attr(&input.attrs);

    let data_enum = match &input.data {
        syn::Data::Enum(e) => e,
        _ => unreachable!("expand_enum called on non-enum"),
    };

    if data_enum.variants.is_empty() {
        let error = syn::Error::new_spanned(name, "Enum must have at least one variant");
        return error.to_compile_error().into();
    }

    // Parse all variants, categorizing by field type
    let variants: Vec<VariantInfo> = data_enum.variants.iter().map(parse_variant).collect();

    // For Select trait, only unit variants can be in options()
    let unit_variant_idents: Vec<_> = variants
        .iter()
        .filter(|v| matches!(v.fields, VariantFields::Unit))
        .map(|v| &v.ident)
        .collect();

    // All variants contribute to labels (for selection)
    let variant_labels: Vec<String> = variants.iter().map(|v| v.ident.to_string()).collect();

    // Generate Prompt impl
    let prompt_impl = generate_prompt_impl(
        name,
        custom_prompt,
        &impl_generics,
        &ty_generics,
        &where_clause,
    );

    // Generate Select impl (only for unit variants in options())
    let select_impl = generate_select_impl(
        name,
        &unit_variant_idents,
        &variant_labels,
        &impl_generics,
        &ty_generics,
        &where_clause,
    );

    // Generate style enum
    let style_enum = generate_style_enum(name);

    // Generate Elicit impl (handles all variant types)
    let elicit_impl =
        generate_elicit_impl(name, &variants, &impl_generics, &ty_generics, &where_clause);

    // Generate ElicitIntrospect impl
    let introspect_impl =
        generate_introspect_impl(name, &variants, &impl_generics, &ty_generics, &where_clause);

    // Generate TypeGraphKey submission for non-generic enums.
    let graph_key_emission = if generics.params.is_empty() {
        let name_str = name.to_string();
        quote! {
            elicitation::inventory::submit!(elicitation::TypeGraphKey::new(
                #name_str,
                <#name as elicitation::ElicitIntrospect>::metadata,
            ));
        }
    } else {
        quote! {}
    };

    // Note: Verification code is NOT generated for user types.
    // Users can write verification harnesses manually if needed.
    // Verification is primarily for elicitation's own contract types.

    let expanded = quote! {
        #style_enum
        #prompt_impl
        #select_impl
        #elicit_impl
        #introspect_impl
        #graph_key_emission
    };

    TokenStream::from(expanded)
}

/// Extract custom prompt from #[prompt("...")] attribute.
fn extract_prompt_attr(attrs: &[syn::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("prompt") {
            // Handle #[prompt("text")]
            if let Ok(value) = attr.parse_args::<syn::LitStr>() {
                return Some(value.value());
            }
        }
    }
    None
}

/// Parse a variant into structured information.
fn parse_variant(variant: &syn::Variant) -> VariantInfo {
    let fields = match &variant.fields {
        Fields::Unit => VariantFields::Unit,

        Fields::Unnamed(f) => {
            let types = f.unnamed.iter().map(|field| field.ty.clone()).collect();
            VariantFields::Tuple(types)
        }

        Fields::Named(f) => {
            let fields = f
                .named
                .iter()
                .map(|field| FieldInfo {
                    ident: field.ident.clone().expect("Named field must have ident"),
                    ty: field.ty.clone(),
                })
                .collect();
            VariantFields::Struct(fields)
        }
    };

    VariantInfo {
        ident: variant.ident.clone(),
        fields,
    }
}

/// Generate Prompt trait implementation.
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
        let default_prompt = format!("Please select a {}:", name);
        quote! {
            impl #impl_generics elicitation::Prompt for #name #ty_generics #where_clause {
                fn prompt() -> Option<&'static str> {
                    Some(#default_prompt)
                }
            }
        }
    }
}

/// Generate Select trait implementation.
fn generate_select_impl(
    name: &syn::Ident,
    unit_variant_idents: &[&syn::Ident],
    variant_labels: &[String],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    quote! {
        impl #impl_generics elicitation::Select for #name #ty_generics #where_clause {
            fn options() -> Vec<Self> {
                vec![#(Self::#unit_variant_idents),*]
            }

            fn labels() -> Vec<String> {
                vec![#(#variant_labels.to_string()),*]
            }

            fn from_label(label: &str) -> Option<Self> {
                match label {
                    #(#variant_labels => Some(Self::#unit_variant_idents),)*
                    _ => None,
                }
            }
        }
    }
}

/// Generate match arm for a single variant.
fn generate_variant_match_arm(variant: &VariantInfo, enum_ident: &syn::Ident) -> TokenStream2 {
    let variant_ident = &variant.ident;
    let label = variant_ident.to_string();

    match &variant.fields {
        VariantFields::Unit => {
            // No fields - just construct variant
            quote! {
                #label => {
                    tracing::debug!(variant = #label, "Constructing unit variant");
                    Ok(#enum_ident::#variant_ident)
                }
            }
        }

        VariantFields::Tuple(fields) => {
            // Generate sequential field elicitation
            let field_names: Vec<_> = (0..fields.len())
                .map(|i| syn::Ident::new(&format!("field_{}", i), variant_ident.span()))
                .collect();

            let elicit_stmts = fields.iter().enumerate().map(|(i, field_ty)| {
                let field_name = &field_names[i];

                quote! {
                    tracing::debug!(
                        variant = #label,
                        field_index = #i,
                        field_type = stringify!(#field_ty),
                        "Eliciting tuple field"
                    );
                    let #field_name = <#field_ty as elicitation::Elicitation>::elicit(communicator).await
                        .map_err(|e| {
                            tracing::error!(
                                variant = #label,
                                field_index = #i,
                                error = %e,
                                "Field elicitation failed"
                            );
                            e
                        })?;
                }
            });

            let field_count = fields.len();
            quote! {
                #label => {
                    tracing::debug!(variant = #label, field_count = #field_count, "Eliciting tuple variant");
                    #(#elicit_stmts)*
                    Ok(#enum_ident::#variant_ident(#(#field_names),*))
                }
            }
        }

        VariantFields::Struct(fields) => {
            // Generate named field elicitation
            let field_idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

            let elicit_stmts = fields.iter().map(|field| {
                let field_ident = &field.ident;
                let field_ty = &field.ty;
                let field_name_str = field_ident.to_string();

                quote! {
                    tracing::debug!(
                        variant = #label,
                        field = #field_name_str,
                        field_type = stringify!(#field_ty),
                        "Eliciting struct field"
                    );
                    let #field_ident = <#field_ty as elicitation::Elicitation>::elicit(communicator).await
                        .map_err(|e| {
                            tracing::error!(
                                variant = #label,
                                field = #field_name_str,
                                error = %e,
                                "Field elicitation failed"
                            );
                            e
                        })?;
                }
            });

            let field_count = fields.len();
            quote! {
                #label => {
                    tracing::debug!(variant = #label, field_count = #field_count, "Eliciting struct variant");
                    #(#elicit_stmts)*
                    Ok(#enum_ident::#variant_ident { #(#field_idents),* })
                }
            }
        }
    }
}

/// Generate the Elicitation implementation for an enum.
fn generate_elicit_impl(
    name: &syn::Ident,
    variants: &[VariantInfo],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let style_name = quote::format_ident!("{}Style", name);
    let variant_labels: Vec<String> = variants.iter().map(|v| v.ident.to_string()).collect();

    // Collect all field types across all variants for kani_proof
    let mut all_field_types: Vec<&syn::Type> = Vec::new();
    for variant in variants {
        match &variant.fields {
            VariantFields::Unit => {
                // No fields to verify
            }
            VariantFields::Tuple(types) => {
                all_field_types.extend(types.iter());
            }
            VariantFields::Struct(fields) => {
                all_field_types.extend(fields.iter().map(|f| &f.ty));
            }
        }
    }

    // Phase 1: Variant selection
    let selection_code = quote! {
        let base_prompt = Self::prompt().unwrap();
        let labels = Self::labels();

        tracing::debug!(
            enum_name = stringify!(#name),
            options = ?labels,
            "Eliciting enum variant selection"
        );

        // Format prompt with options for server-side elicitation
        let options_text = labels.iter()
            .enumerate()
            .map(|(i, label)| format!("{}. {}", i + 1, label))
            .collect::<Vec<_>>()
            .join("\n");

        let full_prompt = format!("{}\n\nOptions:\n{}\n\nRespond with the number (1-{}) or exact label:",
            base_prompt, options_text, labels.len());

        // Use send_prompt for server-side compatibility
        let response = communicator.send_prompt(&full_prompt).await
            .map_err(|e| {
                tracing::error!(error = ?e, "Prompt send failed");
                elicitation::ElicitError::from(e)
            })?;

        // Parse response - try as number first, then as label
        let selected = response.trim();
        let selected = if let Ok(num) = selected.parse::<usize>() {
            // User gave a number (1-indexed)
            if num > 0 && num <= labels.len() {
                labels[num - 1].to_string()
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
            selected.to_string()
        };

        tracing::debug!(
            selected = %selected,
            "User selected variant"
        );
    };

    // Phase 2: Field elicitation based on variant
    let match_arms = variants.iter().map(|v| generate_variant_match_arm(v, name));

    #[cfg(feature = "proofs")]
    let proof_methods = quote! {
        fn kani_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#all_field_types as elicitation::Elicitation>::kani_proof());
            )*
            ts
        }

        fn verus_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#all_field_types as elicitation::Elicitation>::verus_proof());
            )*
            ts
        }

        fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
            let mut ts = elicitation::proc_macro2::TokenStream::new();
            #(
                ts.extend(<#all_field_types as elicitation::Elicitation>::creusot_proof());
            )*
            ts
        }
    };
    #[cfg(not(feature = "proofs"))]
    let proof_methods = quote! {};

    quote! {
        #[automatically_derived]
        #[allow(unexpected_cfgs)]
        impl #impl_generics elicitation::Elicitation for #name #ty_generics #where_clause {
            type Style = #style_name;

            #[tracing::instrument(
                skip(communicator),
                fields(
                    enum_name = stringify!(#name),
                    variant = tracing::field::Empty
                )
            )]
            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                #selection_code

                // Record selected variant in span
                tracing::Span::current().record("variant", &selected.as_str());

                // Match on selected variant and elicit fields
                match selected.as_str() {
                    #(#match_arms,)*
                    _ => {
                        let options_str = vec![#(#variant_labels.to_string()),*].join(", ");
                        tracing::error!(
                            selected = %selected,
                            valid_options = ?Self::labels(),
                            "Invalid variant selected"
                        );
                        Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::InvalidOption {
                                value: selected.clone(),
                                options: options_str,
                            }
                        ))
                    }
                }
            }

            #proof_methods
        }
    }
}

/// Generate a default-only style enum for the type.
///
/// Creates a simple enum with a single Default variant, following the pattern
/// used by built-in types.
fn generate_style_enum(name: &syn::Ident) -> TokenStream2 {
    let style_name = quote::format_ident!("{}Style", name);

    quote! {
        /// Style enum for this type (default-only for now).
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
    }
}

/// Generate ElicitIntrospect implementation for an enum.
fn generate_introspect_impl(
    name: &syn::Ident,
    variants: &[VariantInfo],
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: &Option<&syn::WhereClause>,
) -> TokenStream2 {
    let name_str = name.to_string();

    // Build VariantMetadata for each variant, capturing per-variant fields.
    let variant_metadata: Vec<TokenStream2> = variants
        .iter()
        .map(|v| {
            let label = v.ident.to_string();
            let fields: Vec<TokenStream2> = match &v.fields {
                VariantFields::Unit => vec![],
                VariantFields::Tuple(types) => types
                    .iter()
                    .enumerate()
                    .map(|(i, ty)| {
                        let field_name = i.to_string();
                        let type_name = quote!(#ty).to_string().replace(' ', "");
                        quote! {
                            elicitation::FieldInfo {
                                name: #field_name,
                                prompt: None,
                                type_name: #type_name,
                            }
                        }
                    })
                    .collect(),
                VariantFields::Struct(fields) => fields
                    .iter()
                    .map(|f| {
                        let field_name = f.ident.to_string();
                        let ty = &f.ty;
                        let type_name = quote!(#ty).to_string().replace(' ', "");
                        quote! {
                            elicitation::FieldInfo {
                                name: #field_name,
                                prompt: None,
                                type_name: #type_name,
                            }
                        }
                    })
                    .collect(),
            };
            quote! {
                elicitation::VariantMetadata {
                    label: #label.to_string(),
                    fields: vec![#(#fields),*],
                }
            }
        })
        .collect();

    quote! {
        impl #impl_generics elicitation::ElicitIntrospect for #name #ty_generics #where_clause {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Select
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: #name_str,
                    description: <Self as elicitation::Prompt>::prompt(),
                    details: elicitation::PatternDetails::Select {
                        variants: vec![#(#variant_metadata),*],
                    },
                }
            }
        }
    }
}
