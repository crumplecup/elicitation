//! Wrapper for [`geo_types::Coord<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::Coord;

/// Elicitable representation of [`geo_types::Coord<f64>`].
///
/// A 2D coordinate with x and y fields, commonly used for spatial positioning.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct GeoCoord {
    /// X coordinate (longitude or horizontal position).
    #[serde(default)]
    pub x: f64,
    /// Y coordinate (latitude or vertical position).
    #[serde(default)]
    pub y: f64,
}

impl From<Coord<f64>> for GeoCoord {
    fn from(c: Coord<f64>) -> Self {
        Self { x: c.x, y: c.y }
    }
}

impl From<GeoCoord> for Coord<f64> {
    fn from(c: GeoCoord) -> Self {
        Coord { x: c.x, y: c.y }
    }
}

crate::default_style!(GeoCoord => GeoCoordStyle);

impl Prompt for GeoCoord {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2D coordinate (x, y):")
    }
}

impl Elicitation for GeoCoord {
    type Style = GeoCoordStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoCoord");
        let x = f64::elicit(communicator).await?;
        let y = f64::elicit(communicator).await?;
        Ok(Self { x, y })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("GeoCoord")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("GeoCoord")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("GeoCoord")
    }
}

impl ElicitIntrospect for GeoCoord {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Coord<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f64",
                        prompt: Some("X coordinate:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f64",
                        prompt: Some("Y coordinate:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoCoord {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "GeoCoord".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f64::prompt_tree())),
                ("y".to_string(), Box::new(f64::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoCoord {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! {
            elicitation::GeoCoord { x: #x, y: #y }
        }
    }
}
