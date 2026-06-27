//! `reqwest::tls::Version` elicitation (Select pattern).

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitPromptTree,
    ElicitResult, Elicitation, ElicitationPattern, PatternDetails, Prompt, PromptTree, Select,
    TypeMetadata, VariantMetadata, mcp,
};

crate::default_style!(reqwest::tls::Version => TlsVersionStyle);

impl Select for reqwest::tls::Version {
    fn options() -> Vec<Self> {
        vec![
            reqwest::tls::Version::TLS_1_0,
            reqwest::tls::Version::TLS_1_1,
            reqwest::tls::Version::TLS_1_2,
            reqwest::tls::Version::TLS_1_3,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "TLS 1.0".to_string(),
            "TLS 1.1".to_string(),
            "TLS 1.2".to_string(),
            "TLS 1.3".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "TLS 1.0" => Some(reqwest::tls::Version::TLS_1_0),
            "TLS 1.1" => Some(reqwest::tls::Version::TLS_1_1),
            "TLS 1.2" => Some(reqwest::tls::Version::TLS_1_2),
            "TLS 1.3" => Some(reqwest::tls::Version::TLS_1_3),
            _ => None,
        }
    }
}

impl Prompt for reqwest::tls::Version {
    fn prompt() -> Option<&'static str> {
        Some("Select TLS protocol version:")
    }
}

impl Elicitation for reqwest::tls::Version {
    type Style = TlsVersionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::tls::Version");

        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select TLS protocol version:"),
            &Self::labels(),
        );

        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid TLS protocol version: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("reqwest::tls::Version", "TLS 1.2")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("reqwest::tls::Version", "TLS 1.2")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "reqwest::tls::Version",
            "TLS 1.2",
        )
    }
}

impl ElicitIntrospect for reqwest::tls::Version {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::tls::Version",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for reqwest::tls::Version {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let branch_count = labels.len();
        PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Select TLS protocol version:")
                .to_string(),
            type_name: "reqwest::tls::Version".to_string(),
            options: labels,
            branches: vec![None; branch_count],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for reqwest::tls::Version {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match *self {
            reqwest::tls::Version::TLS_1_0 => quote::quote! { reqwest::tls::Version::TLS_1_0 },
            reqwest::tls::Version::TLS_1_1 => quote::quote! { reqwest::tls::Version::TLS_1_1 },
            reqwest::tls::Version::TLS_1_2 => quote::quote! { reqwest::tls::Version::TLS_1_2 },
            reqwest::tls::Version::TLS_1_3 => quote::quote! { reqwest::tls::Version::TLS_1_3 },
            _ => quote::quote! {{
                compile_error!("unsupported reqwest::tls::Version variant in code recovery");
            }},
        }
    }
}
