//! Derive implementation for enums (Select pattern).

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DeriveInput, Fields, Lit, Meta};

/// Expand #[derive(Elicit)] for enums.
///
/// Generates implementations of:
/// - Prompt (with optional custom prompt from #[prompt] attribute)
/// - Select (options, labels, from_label)
/// - Elicit (calls elicit_select MCP tool)
pub fn expand_enum(input: DeriveInput) -> TokenStream {
    let name = &input.ident;

    // Extract custom prompt from #[prompt("...")] attribute
    let custom_prompt = extract_prompt_attr(&input.attrs);

    let data_enum = match &input.data {
        syn::Data::Enum(e) => e,
        _ => unreachable!("expand_enum called on non-enum"),
    };

    // Extract only unit variants (no fields)
    let unit_variants: Vec<_> = data_enum
        .variants
        .iter()
        .filter(|v| matches!(v.fields, Fields::Unit))
        .collect();

    if unit_variants.is_empty() {
        let error = syn::Error::new_spanned(
            name,
            "Elicit derive for enums requires at least one unit variant. \
             Variants with fields are not supported in v0.1.0.",
        );
        return error.to_compile_error().into();
    }

    // Check for non-unit variants and warn
    let non_unit_count = data_enum.variants.len() - unit_variants.len();
    if non_unit_count > 0 {
        // Note: We could emit a warning here, but proc macros can't emit warnings
        // Users will see their enum partially implemented
    }

    // Generate variant names and labels
    let variant_idents: Vec<_> = unit_variants.iter().map(|v| &v.ident).collect();
    let variant_labels: Vec<String> = variant_idents
        .iter()
        .map(|ident| ident.to_string())
        .collect();

    // Generate Prompt impl
    let prompt_impl = if let Some(prompt) = custom_prompt {
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
    };

    // Generate Select impl
    let select_impl = quote! {
        impl elicitation::Select for #name {
            fn options() -> &'static [Self] {
                &[#(Self::#variant_idents),*]
            }

            fn labels() -> &'static [&'static str] {
                &[#(#variant_labels),*]
            }

            fn from_label(label: &str) -> Option<Self> {
                match label {
                    #(#variant_labels => Some(Self::#variant_idents),)*
                    _ => None,
                }
            }
        }
    };

    // Generate Elicit impl
    let elicit_impl = generate_elicit_impl(name, &variant_labels);

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

/// Generate the Elicit implementation for an enum.
fn generate_elicit_impl(name: &syn::Ident, variant_labels: &[String]) -> TokenStream2 {
    let options_display = variant_labels.join(", ");

    quote! {
        impl elicitation::Elicit for #name {
            #[tracing::instrument(skip(client), fields(enum_name = stringify!(#name)))]
            async fn elicit<T: elicitation::pmcp::shared::transport::Transport>(
                client: &elicitation::pmcp::Client<T>,
            ) -> elicitation::ElicitResult<Self> {
                let prompt = Self::prompt().unwrap();
                let labels = Self::labels();

                tracing::debug!(options = ?labels, "Eliciting enum selection");

                let params = elicitation::mcp::select_params(prompt, labels);
                let result = client
                    .call_tool(
                        elicitation::mcp::tool_names::elicit_select(),
                        params,
                    )
                    .await?;

                let value = elicitation::mcp::extract_value(result)?;
                let selected = elicitation::mcp::parse_string(value)?;

                tracing::debug!(selected = %selected, "User selected option");

                Self::from_label(&selected).ok_or_else(|| {
                    elicitation::ElicitError::new(
                        elicitation::ElicitErrorKind::InvalidOption {
                            value: selected,
                            options: #options_display.to_string(),
                        },
                    )
                })
            }
        }
    }
}
