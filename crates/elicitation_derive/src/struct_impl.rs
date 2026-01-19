//! Derive implementation for structs (Survey pattern).

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Field, Fields};

/// Expand #[derive(Elicit)] for structs.
///
/// Generates implementations of:
/// - Prompt (with optional custom prompt from #[prompt] attribute)
/// - Survey (field metadata)
/// - Elicit (sequential field elicitation)
pub fn expand_struct(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    // Extract custom prompt from #[prompt("...")] attribute
    let (custom_prompt, _) = extract_prompts(&input.attrs);

    let data_struct = match &input.data {
        syn::Data::Struct(s) => s,
        _ => unreachable!("expand_struct called on non-struct"),
    };

    // Extract named fields
    let fields = match &data_struct.fields {
        Fields::Named(f) => &f.named,
        Fields::Unnamed(_) => {
            let error = syn::Error::new_spanned(
                name,
                "Elicit derive for structs requires named fields. \
                 Tuple structs are not supported.",
            );
            return error.to_compile_error().into();
        }
        Fields::Unit => {
            let error =
                syn::Error::new_spanned(name, "Elicit derive for unit structs is not supported.");
            return error.to_compile_error().into();
        }
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
    let prompt_impl = generate_prompt_impl(name, custom_prompt);

    // Generate Survey impl
    let survey_impl = generate_survey_impl(name, &field_infos);

    // Generate Elicit impl (style-aware if styles present)
    let elicit_impl = if all_styles.is_empty() {
        generate_elicit_impl_simple(name, &field_infos, &skipped_fields)
    } else {
        generate_elicit_impl_styled(name, &field_infos, &skipped_fields, &all_styles)
    };

    let expanded = quote! {
        #prompt_impl
        #survey_impl
        #elicit_impl
    };

    TokenStream::from(expanded)
}

/// Field information for code generation.
struct FieldInfo {
    ident: syn::Ident,
    ty: syn::Type,
    default_prompt: Option<String>,
    styled_prompts: std::collections::HashMap<String, String>, // style_name -> prompt_text
}

/// Parse field information from a Field.
fn parse_field_info(field: &Field) -> FieldInfo {
    let (default_prompt, styled_prompts) = extract_prompts(&field.attrs);
    
    FieldInfo {
        ident: field.ident.clone().expect("Named field has ident"),
        ty: field.ty.clone(),
        default_prompt,
        styled_prompts,
    }
}

/// Extract prompts from attributes.
/// Returns (default_prompt, style_specific_prompts).
fn extract_prompts(attrs: &[syn::Attribute]) -> (Option<String>, std::collections::HashMap<String, String>) {
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

/// Generate Prompt implementation.
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
        let default_prompt = format!("Let's create a {}:", name);
        quote! {
            impl elicitation::Prompt for #name {
                fn prompt() -> Option<&'static str> {
                    Some(#default_prompt)
                }
            }
        }
    }
}

/// Generate Survey implementation.
fn generate_survey_impl(name: &syn::Ident, field_infos: &[FieldInfo]) -> TokenStream2 {
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
        impl elicitation::Survey for #name {
            fn fields() -> &'static [elicitation::FieldInfo] {
                &[#(#field_metadata),*]
            }
        }
    }
}

/// Generate simple Elicit implementation (no styles).
fn generate_elicit_impl_simple(
    name: &syn::Ident,
    elicited_fields: &[FieldInfo],
    skipped_fields: &[FieldInfo],
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
                let #name = <#ty>::elicit(client).await?;
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

            async fn elicit(_client: &elicitation::ElicitClient<'_>) -> elicitation::ElicitResult<Self> {
                Ok(Self::Default)
            }
        }

        impl elicitation::Elicitation for #name {
            type Style = #style_name;

            #[tracing::instrument(skip(client))]
            async fn elicit(
                client: &elicitation::ElicitClient<'_>,
            ) -> elicitation::ElicitResult<Self> {
                tracing::debug!(struct_name = stringify!(#name), "Eliciting struct");
                #(#elicit_statements)*
                Ok(Self {
                    #all_field_assignments
                })
            }
        }
    }
}

/// Generate styled Elicit implementation (with style selection).
fn generate_elicit_impl_styled(
    name: &syn::Ident,
    elicited_fields: &[FieldInfo],
    skipped_fields: &[FieldInfo],
    all_styles: &[String],
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
                        // String: use elicit_text
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting string field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let params = elicitation::mcp::text_params(prompt);
                            let result = client
                                .peer()
                                .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                                    name: elicitation::mcp::tool_names::elicit_text().into(),
                                    arguments: Some(params),
                                    task: None,
                                })
                                .await?;
                            let value = elicitation::mcp::extract_value(result)?;
                            let #field_name = elicitation::mcp::parse_string(value)?;
                        }
                    }
                    "bool" => {
                        // Boolean: use elicit_bool
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting bool field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let params = elicitation::mcp::bool_params(prompt);
                            let result = client
                                .peer()
                                .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                                    name: elicitation::mcp::tool_names::elicit_bool().into(),
                                    arguments: Some(params),
                                    task: None,
                                })
                                .await?;
                            let value = elicitation::mcp::extract_value(result)?;
                            let #field_name = elicitation::mcp::parse_bool(value)?;
                        }
                    }
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
                        // Integer types: use elicit_number
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting integer field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let params = elicitation::mcp::number_params(
                                prompt,
                                #field_ty::MIN as i64,
                                #field_ty::MAX as i64,
                            );
                            let result = client
                                .peer()
                                .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                                    name: elicitation::mcp::tool_names::elicit_number().into(),
                                    arguments: Some(params),
                                    task: None,
                                })
                                .await?;
                            let value = elicitation::mcp::extract_value(result)?;
                            let #field_name = elicitation::mcp::parse_integer::<#field_ty>(value)?;
                        }
                    }
                    "f32" | "f64" => {
                        // Float types: use elicit_number with f64 min/max
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting float field with styled prompt");
                            let prompt = match elicit_style {
                                #(#match_arms)*
                            };
                            let params = elicitation::mcp::number_params(
                                prompt,
                                i64::MIN,
                                i64::MAX,
                            );
                            let result = client
                                .peer()
                                .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                                    name: elicitation::mcp::tool_names::elicit_number().into(),
                                    arguments: Some(params),
                                    task: None,
                                })
                                .await?;
                            let value = elicitation::mcp::extract_value(result)?;
                            let #field_name = elicitation::mcp::parse_integer::<i64>(value)? as #field_ty;
                        }
                    }
                    _ => {
                        // Fallback for unsupported types
                        quote! {
                            tracing::debug!(field = #field_name_str, "Eliciting field (no inline style support for this type)");
                            let #field_name = <#field_ty>::elicit(client).await?;
                        }
                    }
                }
            } else {
                // For complex types or fields without styled prompts, fall back to their own elicit()
                quote! {
                    tracing::debug!(field = #field_name_str, "Eliciting field via standard elicit()");
                    let #field_name = <#field_ty>::elicit(client).await?;
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

            #[tracing::instrument(skip(client))]
            async fn elicit(
                client: &elicitation::ElicitClient<'_>,
            ) -> elicitation::ElicitResult<Self> {
                let prompt = <Self as elicitation::Prompt>::prompt().unwrap();
                tracing::debug!("Eliciting style selection");

                let params = elicitation::mcp::select_params(
                    prompt,
                    <Self as elicitation::Select>::labels()
                );
                let result = client
                    .peer()
                    .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                        name: elicitation::mcp::tool_names::elicit_select().into(),
                        arguments: Some(params),
                        task: None,
                    })
                    .await?;

                let value = elicitation::mcp::extract_value(result)?;
                let label = elicitation::mcp::parse_string(value)?;

                <Self as elicitation::Select>::from_label(&label).ok_or_else(|| {
                    elicitation::ElicitError::new(elicitation::ElicitErrorKind::InvalidSelection(label))
                })
            }
        }
        
        impl elicitation::Elicitation for #name {
            type Style = #style_enum_name;

            #[tracing::instrument(skip(client))]
            async fn elicit(
                client: &elicitation::ElicitClient<'_>,
            ) -> elicitation::ElicitResult<Self> {
                tracing::debug!(struct_name = stringify!(#name), "Eliciting struct with style");
                
                // Step 1: Elicit style choice
                let elicit_style = #style_enum_name::elicit(client).await?;
                tracing::debug!(?elicit_style, "Style selected");
                
                // Step 2: Elicit fields with chosen style
                #(#field_elicit_statements)*
                
                Ok(Self {
                    #all_field_assignments
                })
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
