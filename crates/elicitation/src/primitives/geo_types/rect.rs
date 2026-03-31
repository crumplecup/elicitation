//! Wrapper for [`geo_types::Rect<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::{Coord, Rect};

use super::coord::GeoCoord;

/// Elicitable representation of [`geo_types::Rect<f64>`].
///
/// An axis-aligned rectangle defined by its minimum and maximum corners.
/// The constructor normalizes the corners so `min` ≤ `max` on each axis.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct GeoRect {
    /// Minimum corner (lower-left).
    pub min: GeoCoord,
    /// Maximum corner (upper-right).
    pub max: GeoCoord,
}

impl From<Rect<f64>> for GeoRect {
    fn from(r: Rect<f64>) -> Self {
        Self {
            min: GeoCoord::from(r.min()),
            max: GeoCoord::from(r.max()),
        }
    }
}

impl From<GeoRect> for Rect<f64> {
    fn from(r: GeoRect) -> Self {
        Rect::new(
            Coord::<f64>::from(r.min),
            Coord::<f64>::from(r.max),
        )
    }
}

crate::default_style!(GeoRect => GeoRectStyle);

impl Prompt for GeoRect {
    fn prompt() -> Option<&'static str> {
        Some("Specify an axis-aligned rectangle (min corner, max corner):")
    }
}

impl Elicitation for GeoRect {
    type Style = GeoRectStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoRect");
        let min = GeoCoord::elicit(communicator).await?;
        let max = GeoCoord::elicit(communicator).await?;
        Ok(Self { min, max })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("GeoRect")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("GeoRect")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("GeoRect")
    }
}

impl ElicitIntrospect for GeoRect {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Rect<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "min",
                        type_name: "GeoCoord",
                        prompt: Some("Minimum corner (lower-left):"),
                    },
                    FieldInfo {
                        name: "max",
                        type_name: "GeoCoord",
                        prompt: Some("Maximum corner (upper-right):"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for GeoRect {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "GeoRect".to_string(),
            fields: vec![
                ("min".to_string(), Box::new(GeoCoord::prompt_tree())),
                ("max".to_string(), Box::new(GeoCoord::prompt_tree())),
            ],
        }
    }
}
