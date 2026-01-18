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
pub fn expand_enum(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

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
    let prompt_impl = generate_prompt_impl(name, custom_prompt);

    // Generate Select impl (only for unit variants in options())
    let select_impl = generate_select_impl(name, &unit_variant_idents, &variant_labels);

    // Generate Elicit impl (handles all variant types)
    let elicit_impl = generate_elicit_impl(name, &variants);

    let expanded = quote! {
        #prompt_impl
        #select_impl
        #elicit_impl
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
fn generate_prompt_impl(name: &syn::Ident, custom_prompt: Option<String>) -> TokenStream2 {
    if let Some(prompt) = custom_prompt {
        quote! {
            impl elicitation::Prompt for #name {
                fn prompt() -> Option<&'static str> {
                    Some(#prompt)
                }
            }
        }
    } else {
        let default_prompt = format!("Please select a {}:", name);
        quote! {
            impl elicitation::Prompt for #name {
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
) -> TokenStream2 {
    quote! {
        impl elicitation::Select for #name {
            fn options() -> &'static [Self] {
                &[#(Self::#unit_variant_idents),*]
            }

            fn labels() -> &'static [&'static str] {
                &[#(#variant_labels),*]
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
                    let #field_name = <#field_ty as elicitation::Elicitation>::elicit(client).await
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
                    let #field_ident = <#field_ty as elicitation::Elicitation>::elicit(client).await
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
fn generate_elicit_impl(name: &syn::Ident, variants: &[VariantInfo]) -> TokenStream2 {
    let variant_labels: Vec<String> = variants.iter().map(|v| v.ident.to_string()).collect();

    // Phase 1: Variant selection
    let selection_code = quote! {
        let prompt = Self::prompt().unwrap();
        let labels = Self::labels();

        tracing::debug!(
            enum_name = stringify!(#name),
            options = ?labels,
            "Eliciting enum variant selection"
        );

        let params = elicitation::mcp::select_params(prompt, labels);
        let result = client
            .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                name: elicitation::mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "MCP tool call failed");
                elicitation::ElicitError::from(e)
            })?;

        let value = elicitation::mcp::extract_value(result)?;
        let selected = elicitation::mcp::parse_string(value)?;

        tracing::debug!(
            selected = %selected,
            "User selected variant"
        );
    };

    // Phase 2: Field elicitation based on variant
    let match_arms = variants.iter().map(|v| generate_variant_match_arm(v, name));

    quote! {
        #[automatically_derived]
        impl elicitation::Elicitation for #name {
            #[tracing::instrument(
                skip(client),
                fields(
                    enum_name = stringify!(#name),
                    variant = tracing::field::Empty
                )
            )]
            async fn elicit(
                client: &elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleClient>,
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
        }
    }
}
