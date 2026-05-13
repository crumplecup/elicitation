//! Elicitation support for [`georaster::geotiff::ImageInfo`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use georaster::geotiff::ImageInfo;

crate::default_style!(ImageInfo => GeoRasterImageInfoStyle);

impl Prompt for ImageInfo {
    fn prompt() -> Option<&'static str> {
        Some("Describe a GeoTIFF image info record:")
    }
}

impl Elicitation for ImageInfo {
    type Style = GeoRasterImageInfoStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            dimensions: Option::<(u32, u32)>::elicit(communicator).await?,
            colortype: Option::<tiff::ColorType>::elicit(communicator).await?,
            photometric_interpretation: Option::<tiff::tags::PhotometricInterpretation>::elicit(
                communicator,
            )
            .await?,
            planar_config: Option::<tiff::tags::PlanarConfiguration>::elicit(communicator).await?,
            samples: u8::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("georaster::geotiff::ImageInfo")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("georaster::geotiff::ImageInfo")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("georaster::geotiff::ImageInfo")
    }
}

impl ElicitIntrospect for ImageInfo {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "georaster::geotiff::ImageInfo",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "dimensions",
                        type_name: "Option<(u32, u32)>",
                        prompt: Some("Optional image width/height dimensions:"),
                    },
                    FieldInfo {
                        name: "colortype",
                        type_name: "Option<tiff::ColorType>",
                        prompt: Some("Optional TIFF color type:"),
                    },
                    FieldInfo {
                        name: "photometric_interpretation",
                        type_name: "Option<tiff::tags::PhotometricInterpretation>",
                        prompt: Some("Optional TIFF photometric interpretation:"),
                    },
                    FieldInfo {
                        name: "planar_config",
                        type_name: "Option<tiff::tags::PlanarConfiguration>",
                        prompt: Some("Optional TIFF planar configuration:"),
                    },
                    FieldInfo {
                        name: "samples",
                        type_name: "u8",
                        prompt: Some("Samples per pixel:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for ImageInfo {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "georaster::geotiff::ImageInfo".to_string(),
            fields: vec![
                (
                    "dimensions".to_string(),
                    Box::new(crate::PromptTree::Leaf {
                        prompt: "Optionally enter image dimensions as (width, height).".to_string(),
                        type_name: "Option<(u32, u32)>".to_string(),
                    }),
                ),
                (
                    "colortype".to_string(),
                    Box::new(Option::<tiff::ColorType>::prompt_tree()),
                ),
                (
                    "photometric_interpretation".to_string(),
                    Box::new(Option::<tiff::tags::PhotometricInterpretation>::prompt_tree()),
                ),
                (
                    "planar_config".to_string(),
                    Box::new(Option::<tiff::tags::PlanarConfiguration>::prompt_tree()),
                ),
                ("samples".to_string(), Box::new(u8::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for ImageInfo {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let dimensions = self.dimensions.to_code_literal();
        let colortype = self.colortype.to_code_literal();
        let photometric_interpretation = self.photometric_interpretation.to_code_literal();
        let planar_config = self.planar_config.to_code_literal();
        let samples = self.samples;
        quote::quote! {
            ::georaster::geotiff::ImageInfo {
                dimensions: #dimensions,
                colortype: #colortype,
                photometric_interpretation: #photometric_interpretation,
                planar_config: #planar_config,
                samples: #samples,
            }
        }
    }
}
