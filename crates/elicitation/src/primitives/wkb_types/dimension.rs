//! Wrapper for [`wkb::reader::Dimension`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};

/// Serializable mirror of [`wkb::reader::Dimension`].
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
pub enum WkbDimension {
    /// 2D: X and Y.
    Xy,
    /// 3D: X, Y, and Z.
    Xyz,
    /// 3D: X, Y, and M.
    Xym,
    /// 4D: X, Y, Z, and M.
    Xyzm,
}

impl From<wkb::reader::Dimension> for WkbDimension {
    fn from(value: wkb::reader::Dimension) -> Self {
        match value {
            wkb::reader::Dimension::Xy => Self::Xy,
            wkb::reader::Dimension::Xyz => Self::Xyz,
            wkb::reader::Dimension::Xym => Self::Xym,
            wkb::reader::Dimension::Xyzm => Self::Xyzm,
        }
    }
}

impl From<WkbDimension> for wkb::reader::Dimension {
    fn from(value: WkbDimension) -> Self {
        match value {
            WkbDimension::Xy => Self::Xy,
            WkbDimension::Xyz => Self::Xyz,
            WkbDimension::Xym => Self::Xym,
            WkbDimension::Xyzm => Self::Xyzm,
        }
    }
}

crate::default_style!(WkbDimension => WkbDimensionStyle);

impl Prompt for WkbDimension {
    fn prompt() -> Option<&'static str> {
        Some("Choose the WKB coordinate dimension:")
    }
}

impl Select for WkbDimension {
    fn options() -> Vec<Self> {
        vec![Self::Xy, Self::Xyz, Self::Xym, Self::Xyzm]
    }

    fn labels() -> Vec<String> {
        vec![
            "Xy".to_string(),
            "Xyz".to_string(),
            "Xym".to_string(),
            "Xyzm".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Xy" => Some(Self::Xy),
            "Xyz" => Some(Self::Xyz),
            "Xym" => Some(Self::Xym),
            "Xyzm" => Some(Self::Xyzm),
            _ => None,
        }
    }
}

impl Elicitation for WkbDimension {
    type Style = WkbDimensionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose the WKB coordinate dimension:"),
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
                "Invalid WkbDimension: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("WkbDimension", "Xy")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("WkbDimension", "Xy")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("WkbDimension", "Xy")
    }
}

impl ElicitIntrospect for WkbDimension {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkb::reader::Dimension",
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

impl crate::ElicitPromptTree for WkbDimension {
    fn prompt_tree() -> crate::PromptTree {
        let opts = Self::labels();
        let n = opts.len();
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose coordinate dimension:")
                .to_string(),
            type_name: "WkbDimension".to_string(),
            options: opts,
            branches: vec![None; n],
        }
    }
}
