//! Wrapper for [`wkb::Endianness`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

/// Serializable mirror of [`wkb::Endianness`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub enum WkbEndianness {
    /// Big endian byte order.
    BigEndian,
    /// Little endian byte order.
    LittleEndian,
}

impl From<wkb::Endianness> for WkbEndianness {
    fn from(value: wkb::Endianness) -> Self {
        match value {
            wkb::Endianness::BigEndian => Self::BigEndian,
            wkb::Endianness::LittleEndian => Self::LittleEndian,
        }
    }
}

impl From<WkbEndianness> for wkb::Endianness {
    fn from(value: WkbEndianness) -> Self {
        match value {
            WkbEndianness::BigEndian => Self::BigEndian,
            WkbEndianness::LittleEndian => Self::LittleEndian,
        }
    }
}

crate::default_style!(WkbEndianness => WkbEndiannessStyle);

impl Prompt for WkbEndianness {
    fn prompt() -> Option<&'static str> {
        Some("Choose the WKB byte order:")
    }
}

impl Select for WkbEndianness {
    fn options() -> Vec<Self> {
        vec![Self::BigEndian, Self::LittleEndian]
    }

    fn labels() -> Vec<String> {
        vec!["BigEndian".to_string(), "LittleEndian".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "BigEndian" => Some(Self::BigEndian),
            "LittleEndian" => Some(Self::LittleEndian),
            _ => None,
        }
    }
}

impl Elicitation for WkbEndianness {
    type Style = WkbEndiannessStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the WKB byte order:"),
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
                "Invalid WkbEndianness: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("WkbEndianness", "BigEndian")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("WkbEndianness", "BigEndian")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("WkbEndianness", "BigEndian")
    }
}

impl ElicitIntrospect for WkbEndianness {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkb::Endianness",
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

impl crate::ElicitPromptTree for WkbEndianness {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the WKB byte order:")
                .to_string(),
            type_name: "WkbEndianness".to_string(),
            options: Self::labels(),
            branches: vec![None, None],
        }
    }
}
