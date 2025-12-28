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
    let custom_prompt = extract_prompt_attr(&input.attrs);

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

    // Generate Prompt impl
    let prompt_impl = generate_prompt_impl(name, custom_prompt);

    // Generate Survey impl
    let survey_impl = generate_survey_impl(name, &field_infos);

    // Generate Elicit impl
    let elicit_impl = generate_elicit_impl(name, &field_infos, &skipped_fields);

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
    prompt: Option<String>,
}

/// Parse field information from a Field.
fn parse_field_info(field: &Field) -> FieldInfo {
    FieldInfo {
        ident: field.ident.clone().expect("Named field has ident"),
        ty: field.ty.clone(),
        prompt: extract_prompt_attr(&field.attrs),
    }
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
            let prompt_expr = match &info.prompt {
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

/// Generate Elicit implementation.
fn generate_elicit_impl(
    name: &syn::Ident,
    elicited_fields: &[FieldInfo],
    skipped_fields: &[FieldInfo],
) -> TokenStream2 {
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
        impl elicitation::Elicitation for #name {
            #[tracing::instrument(skip(client), fields(struct_name = stringify!(#name)))]
            async fn elicit<T: elicitation::pmcp::shared::transport::Transport>(
                client: &elicitation::pmcp::Client<T>,
            ) -> elicitation::ElicitResult<Self> {
                tracing::info!(struct_name = stringify!(#name), "Starting survey");

                #(#elicit_statements)*

                tracing::info!("Survey complete, constructing struct");

                Ok(Self {
                    #all_field_assignments
                })
            }
        }
    }
}
