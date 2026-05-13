//! Elicitation support for [`tiff::tags::PlanarConfiguration`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use tiff::tags::PlanarConfiguration;

crate::default_style!(PlanarConfiguration => TiffPlanarConfigurationStyle);

impl Prompt for PlanarConfiguration {
    fn prompt() -> Option<&'static str> {
        Some("Choose the TIFF planar configuration:")
    }
}

impl Select for PlanarConfiguration {
    fn options() -> Vec<Self> {
        vec![Self::Chunky, Self::Planar]
    }

    fn labels() -> Vec<String> {
        vec!["Chunky".to_string(), "Planar".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Chunky" => Some(Self::Chunky),
            "Planar" => Some(Self::Planar),
            _ => None,
        }
    }
}

impl Elicitation for PlanarConfiguration {
    type Style = TiffPlanarConfigurationStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        Self::prompt().unwrap_or("Choose the TIFF planar configuration:"),
                        &Self::labels(),
                    )),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label)
            .ok_or_else(|| ElicitError::new(ElicitErrorKind::InvalidSelection(label)))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "tiff::tags::PlanarConfiguration",
            "Chunky",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "tiff::tags::PlanarConfiguration",
            "Chunky",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "tiff::tags::PlanarConfiguration",
            "Chunky",
        )
    }
}

impl ElicitIntrospect for PlanarConfiguration {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tiff::tags::PlanarConfiguration",
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

impl crate::ElicitPromptTree for PlanarConfiguration {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the TIFF planar configuration:")
                .to_string(),
            type_name: "tiff::tags::PlanarConfiguration".to_string(),
            options: Self::labels(),
            branches: vec![None, None],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for PlanarConfiguration {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Chunky => quote::quote!(::tiff::tags::PlanarConfiguration::Chunky),
            Self::Planar => quote::quote!(::tiff::tags::PlanarConfiguration::Planar),
            _ => panic!("unsupported future PlanarConfiguration variant"),
        }
    }
}
