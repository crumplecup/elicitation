//! Elicitation support for [`tiff::ColorType`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use tiff::ColorType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ColorTypeKind {
    Gray,
    Rgb,
    Palette,
    GrayA,
    Rgba,
    Cmyk,
    YCbCr,
}

impl Prompt for ColorTypeKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the TIFF color type:")
    }
}

impl Select for ColorTypeKind {
    fn options() -> Vec<Self> {
        vec![
            Self::Gray,
            Self::Rgb,
            Self::Palette,
            Self::GrayA,
            Self::Rgba,
            Self::Cmyk,
            Self::YCbCr,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Gray".to_string(),
            "RGB".to_string(),
            "Palette".to_string(),
            "GrayA".to_string(),
            "RGBA".to_string(),
            "CMYK".to_string(),
            "YCbCr".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Gray" => Some(Self::Gray),
            "RGB" => Some(Self::Rgb),
            "Palette" => Some(Self::Palette),
            "GrayA" => Some(Self::GrayA),
            "RGBA" => Some(Self::Rgba),
            "CMYK" => Some(Self::Cmyk),
            "YCbCr" => Some(Self::YCbCr),
            _ => None,
        }
    }
}

crate::default_style!(ColorType => TiffColorTypeStyle);

impl Prompt for ColorType {
    fn prompt() -> Option<&'static str> {
        Some("Choose the TIFF color type and bit depth:")
    }
}

impl Elicitation for ColorType {
    type Style = TiffColorTypeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        ColorTypeKind::prompt().unwrap_or("Choose the TIFF color type:"),
                        &ColorTypeKind::labels(),
                    )),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        let bits = u8::elicit(communicator).await?;
        match ColorTypeKind::from_label(&label) {
            Some(ColorTypeKind::Gray) => Ok(Self::Gray(bits)),
            Some(ColorTypeKind::Rgb) => Ok(Self::RGB(bits)),
            Some(ColorTypeKind::Palette) => Ok(Self::Palette(bits)),
            Some(ColorTypeKind::GrayA) => Ok(Self::GrayA(bits)),
            Some(ColorTypeKind::Rgba) => Ok(Self::RGBA(bits)),
            Some(ColorTypeKind::Cmyk) => Ok(Self::CMYK(bits)),
            Some(ColorTypeKind::YCbCr) => Ok(Self::YCbCr(bits)),
            None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("tiff::ColorType")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("tiff::ColorType")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("tiff::ColorType")
    }
}

impl ElicitIntrospect for ColorType {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "tiff::ColorType",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: ColorTypeKind::labels()
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

impl crate::ElicitPromptTree for ColorType {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose the TIFF color type and bit depth:")
                .to_string(),
            type_name: "tiff::ColorType".to_string(),
            options: ColorTypeKind::labels(),
            branches: vec![
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u8::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for ColorType {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Gray(bits) => quote::quote!(::tiff::ColorType::Gray(#bits)),
            Self::RGB(bits) => quote::quote!(::tiff::ColorType::RGB(#bits)),
            Self::Palette(bits) => quote::quote!(::tiff::ColorType::Palette(#bits)),
            Self::GrayA(bits) => quote::quote!(::tiff::ColorType::GrayA(#bits)),
            Self::RGBA(bits) => quote::quote!(::tiff::ColorType::RGBA(#bits)),
            Self::CMYK(bits) => quote::quote!(::tiff::ColorType::CMYK(#bits)),
            Self::YCbCr(bits) => quote::quote!(::tiff::ColorType::YCbCr(#bits)),
        }
    }
}
