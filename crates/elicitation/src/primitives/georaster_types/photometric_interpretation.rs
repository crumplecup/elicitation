//! Elicitation support for [`tiff::tags::PhotometricInterpretation`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use tiff::tags::PhotometricInterpretation;

crate::default_style!(PhotometricInterpretation => TiffPhotometricInterpretationStyle);

impl Prompt for PhotometricInterpretation {
    fn prompt() -> Option<&'static str> {
        Some("Choose the TIFF photometric interpretation:")
    }
}

impl Select for PhotometricInterpretation {
    fn options() -> Vec<Self> {
        vec![
            Self::WhiteIsZero,
            Self::BlackIsZero,
            Self::RGB,
            Self::RGBPalette,
            Self::TransparencyMask,
            Self::CMYK,
            Self::YCbCr,
            Self::CIELab,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "WhiteIsZero".to_string(),
            "BlackIsZero".to_string(),
            "RGB".to_string(),
            "RGBPalette".to_string(),
            "TransparencyMask".to_string(),
            "CMYK".to_string(),
            "YCbCr".to_string(),
            "CIELab".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "WhiteIsZero" => Some(Self::WhiteIsZero),
            "BlackIsZero" => Some(Self::BlackIsZero),
            "RGB" => Some(Self::RGB),
            "RGBPalette" => Some(Self::RGBPalette),
            "TransparencyMask" => Some(Self::TransparencyMask),
            "CMYK" => Some(Self::CMYK),
            "YCbCr" => Some(Self::YCbCr),
            "CIELab" => Some(Self::CIELab),
            _ => None,
        }
    }
}

impl Elicitation for PhotometricInterpretation {
    type Style = TiffPhotometricInterpretationStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        Self::prompt().unwrap_or("Choose the TIFF photometric interpretation:"),
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
            "tiff::tags::PhotometricInterpretation",
            "RGB",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "tiff::tags::PhotometricInterpretation",
            "RGB",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "tiff::tags::PhotometricInterpretation",
            "RGB",
        )
    }
}

impl ElicitIntrospect for PhotometricInterpretation {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tiff::tags::PhotometricInterpretation",
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

impl crate::ElicitPromptTree for PhotometricInterpretation {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the TIFF photometric interpretation:")
                .to_string(),
            type_name: "tiff::tags::PhotometricInterpretation".to_string(),
            options: Self::labels(),
            branches: vec![None, None, None, None, None, None, None, None],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for PhotometricInterpretation {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::WhiteIsZero => {
                quote::quote!(::tiff::tags::PhotometricInterpretation::WhiteIsZero)
            }
            Self::BlackIsZero => {
                quote::quote!(::tiff::tags::PhotometricInterpretation::BlackIsZero)
            }
            Self::RGB => quote::quote!(::tiff::tags::PhotometricInterpretation::RGB),
            Self::RGBPalette => quote::quote!(::tiff::tags::PhotometricInterpretation::RGBPalette),
            Self::TransparencyMask => {
                quote::quote!(::tiff::tags::PhotometricInterpretation::TransparencyMask)
            }
            Self::CMYK => quote::quote!(::tiff::tags::PhotometricInterpretation::CMYK),
            Self::YCbCr => quote::quote!(::tiff::tags::PhotometricInterpretation::YCbCr),
            Self::CIELab => quote::quote!(::tiff::tags::PhotometricInterpretation::CIELab),
            _ => panic!("unsupported future PhotometricInterpretation variant"),
        }
    }
}
