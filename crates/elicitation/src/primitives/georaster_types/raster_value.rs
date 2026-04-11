//! Elicitation support for [`georaster::geotiff::RasterValue`].

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use georaster::geotiff::RasterValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RasterValueKind {
    NoData,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    Rgb8,
    Rgba8,
    Rgb16,
    Rgba16,
}

impl Prompt for RasterValueKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose the raster pixel value variant:")
    }
}

impl Select for RasterValueKind {
    fn options() -> Vec<Self> {
        vec![
            Self::NoData,
            Self::U8,
            Self::U16,
            Self::U32,
            Self::U64,
            Self::F32,
            Self::F64,
            Self::I8,
            Self::I16,
            Self::I32,
            Self::I64,
            Self::Rgb8,
            Self::Rgba8,
            Self::Rgb16,
            Self::Rgba16,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "NoData".to_string(),
            "U8".to_string(),
            "U16".to_string(),
            "U32".to_string(),
            "U64".to_string(),
            "F32".to_string(),
            "F64".to_string(),
            "I8".to_string(),
            "I16".to_string(),
            "I32".to_string(),
            "I64".to_string(),
            "Rgb8".to_string(),
            "Rgba8".to_string(),
            "Rgb16".to_string(),
            "Rgba16".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "NoData" => Some(Self::NoData),
            "U8" => Some(Self::U8),
            "U16" => Some(Self::U16),
            "U32" => Some(Self::U32),
            "U64" => Some(Self::U64),
            "F32" => Some(Self::F32),
            "F64" => Some(Self::F64),
            "I8" => Some(Self::I8),
            "I16" => Some(Self::I16),
            "I32" => Some(Self::I32),
            "I64" => Some(Self::I64),
            "Rgb8" => Some(Self::Rgb8),
            "Rgba8" => Some(Self::Rgba8),
            "Rgb16" => Some(Self::Rgb16),
            "Rgba16" => Some(Self::Rgba16),
            _ => None,
        }
    }
}

crate::default_style!(RasterValue => GeoRasterRasterValueStyle);

impl Prompt for RasterValue {
    fn prompt() -> Option<&'static str> {
        Some("Choose a georaster pixel value:")
    }
}

impl Elicitation for RasterValue {
    type Style = GeoRasterRasterValueStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(mcp::select_params(
                        RasterValueKind::prompt()
                            .unwrap_or("Choose the raster pixel value variant:"),
                        &RasterValueKind::labels(),
                    )),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match RasterValueKind::from_label(&label) {
            Some(RasterValueKind::NoData) => Ok(Self::NoData),
            Some(RasterValueKind::U8) => Ok(Self::U8(u8::elicit(communicator).await?)),
            Some(RasterValueKind::U16) => Ok(Self::U16(u16::elicit(communicator).await?)),
            Some(RasterValueKind::U32) => Ok(Self::U32(u32::elicit(communicator).await?)),
            Some(RasterValueKind::U64) => Ok(Self::U64(u64::elicit(communicator).await?)),
            Some(RasterValueKind::F32) => Ok(Self::F32(f32::elicit(communicator).await?)),
            Some(RasterValueKind::F64) => Ok(Self::F64(f64::elicit(communicator).await?)),
            Some(RasterValueKind::I8) => Ok(Self::I8(i8::elicit(communicator).await?)),
            Some(RasterValueKind::I16) => Ok(Self::I16(i16::elicit(communicator).await?)),
            Some(RasterValueKind::I32) => Ok(Self::I32(i32::elicit(communicator).await?)),
            Some(RasterValueKind::I64) => Ok(Self::I64(i64::elicit(communicator).await?)),
            Some(RasterValueKind::Rgb8) => Ok(Self::Rgb8(
                u8::elicit(communicator).await?,
                u8::elicit(communicator).await?,
                u8::elicit(communicator).await?,
            )),
            Some(RasterValueKind::Rgba8) => Ok(Self::Rgba8(
                u8::elicit(communicator).await?,
                u8::elicit(communicator).await?,
                u8::elicit(communicator).await?,
                u8::elicit(communicator).await?,
            )),
            Some(RasterValueKind::Rgb16) => Ok(Self::Rgb16(
                u16::elicit(communicator).await?,
                u16::elicit(communicator).await?,
                u16::elicit(communicator).await?,
            )),
            Some(RasterValueKind::Rgba16) => Ok(Self::Rgba16(
                u16::elicit(communicator).await?,
                u16::elicit(communicator).await?,
                u16::elicit(communicator).await?,
                u16::elicit(communicator).await?,
            )),
            None => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(label))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("georaster::geotiff::RasterValue")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("georaster::geotiff::RasterValue")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque(
            "georaster::geotiff::RasterValue",
        )
    }
}

impl ElicitIntrospect for RasterValue {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "georaster::geotiff::RasterValue",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: RasterValueKind::labels()
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

impl crate::ElicitPromptTree for RasterValue {
    fn prompt_tree() -> crate::PromptTree {
        let rgb8_tree = crate::PromptTree::Survey {
            prompt: Some("Enter RGB8 channel values:".to_string()),
            type_name: "(u8, u8, u8)".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(u8::prompt_tree())),
                ("g".to_string(), Box::new(u8::prompt_tree())),
                ("b".to_string(), Box::new(u8::prompt_tree())),
            ],
        };
        let rgba8_tree = crate::PromptTree::Survey {
            prompt: Some("Enter RGBA8 channel values:".to_string()),
            type_name: "(u8, u8, u8, u8)".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(u8::prompt_tree())),
                ("g".to_string(), Box::new(u8::prompt_tree())),
                ("b".to_string(), Box::new(u8::prompt_tree())),
                ("a".to_string(), Box::new(u8::prompt_tree())),
            ],
        };
        let rgb16_tree = crate::PromptTree::Survey {
            prompt: Some("Enter RGB16 channel values:".to_string()),
            type_name: "(u16, u16, u16)".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(u16::prompt_tree())),
                ("g".to_string(), Box::new(u16::prompt_tree())),
                ("b".to_string(), Box::new(u16::prompt_tree())),
            ],
        };
        let rgba16_tree = crate::PromptTree::Survey {
            prompt: Some("Enter RGBA16 channel values:".to_string()),
            type_name: "(u16, u16, u16, u16)".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(u16::prompt_tree())),
                ("g".to_string(), Box::new(u16::prompt_tree())),
                ("b".to_string(), Box::new(u16::prompt_tree())),
                ("a".to_string(), Box::new(u16::prompt_tree())),
            ],
        };
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Choose a georaster pixel value:")
                .to_string(),
            type_name: "georaster::geotiff::RasterValue".to_string(),
            options: RasterValueKind::labels(),
            branches: vec![
                None,
                Some(Box::new(u8::prompt_tree())),
                Some(Box::new(u16::prompt_tree())),
                Some(Box::new(u32::prompt_tree())),
                Some(Box::new(u64::prompt_tree())),
                Some(Box::new(f32::prompt_tree())),
                Some(Box::new(f64::prompt_tree())),
                Some(Box::new(i8::prompt_tree())),
                Some(Box::new(i16::prompt_tree())),
                Some(Box::new(i32::prompt_tree())),
                Some(Box::new(i64::prompt_tree())),
                Some(Box::new(rgb8_tree)),
                Some(Box::new(rgba8_tree)),
                Some(Box::new(rgb16_tree)),
                Some(Box::new(rgba16_tree)),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for RasterValue {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::NoData => quote::quote!(::georaster::geotiff::RasterValue::NoData),
            Self::U8(value) => quote::quote!(::georaster::geotiff::RasterValue::U8(#value)),
            Self::U16(value) => quote::quote!(::georaster::geotiff::RasterValue::U16(#value)),
            Self::U32(value) => quote::quote!(::georaster::geotiff::RasterValue::U32(#value)),
            Self::U64(value) => quote::quote!(::georaster::geotiff::RasterValue::U64(#value)),
            Self::F32(value) => quote::quote!(::georaster::geotiff::RasterValue::F32(#value)),
            Self::F64(value) => quote::quote!(::georaster::geotiff::RasterValue::F64(#value)),
            Self::I8(value) => quote::quote!(::georaster::geotiff::RasterValue::I8(#value)),
            Self::I16(value) => quote::quote!(::georaster::geotiff::RasterValue::I16(#value)),
            Self::I32(value) => quote::quote!(::georaster::geotiff::RasterValue::I32(#value)),
            Self::I64(value) => quote::quote!(::georaster::geotiff::RasterValue::I64(#value)),
            Self::Rgb8(r, g, b) => {
                quote::quote!(::georaster::geotiff::RasterValue::Rgb8(#r, #g, #b))
            }
            Self::Rgba8(r, g, b, a) => {
                quote::quote!(::georaster::geotiff::RasterValue::Rgba8(#r, #g, #b, #a))
            }
            Self::Rgb16(r, g, b) => {
                quote::quote!(::georaster::geotiff::RasterValue::Rgb16(#r, #g, #b))
            }
            Self::Rgba16(r, g, b, a) => {
                quote::quote!(::georaster::geotiff::RasterValue::Rgba16(#r, #g, #b, #a))
            }
            _ => panic!("unsupported future RasterValue variant"),
        }
    }
}
