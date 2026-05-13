//! Wrapper for [`geo_types::Point<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::{Coord, Point};

use super::coord::GeoCoord;

/// Elicitable representation of [`geo_types::Point<f64>`].
///
/// A geographic point defined by a single 2D coordinate.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct GeoPoint {
    /// The underlying coordinate.
    pub coord: GeoCoord,
}

impl From<Point<f64>> for GeoPoint {
    fn from(p: Point<f64>) -> Self {
        Self {
            coord: GeoCoord::from(p.0),
        }
    }
}

impl From<GeoPoint> for Point<f64> {
    fn from(p: GeoPoint) -> Self {
        Point(Coord::<f64>::from(p.coord))
    }
}

crate::default_style!(GeoPoint => GeoPointStyle);

impl Prompt for GeoPoint {
    fn prompt() -> Option<&'static str> {
        Some("Specify a geographic point (coordinate):")
    }
}

impl Elicitation for GeoPoint {
    type Style = GeoPointStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoPoint");
        let coord = GeoCoord::elicit(communicator).await?;
        Ok(Self { coord })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        // GeoPoint wraps one GeoCoord — delegate to compose the proof chain.
        GeoCoord::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        GeoCoord::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        GeoCoord::creusot_proof()
    }
}

impl ElicitIntrospect for GeoPoint {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Point<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "coord",
                    type_name: "GeoCoord",
                    prompt: Some("Coordinate (x, y):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoPoint {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "GeoPoint".to_string(),
            fields: vec![("coord".to_string(), Box::new(GeoCoord::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoPoint {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let coord = self.coord.to_code_literal();
        quote::quote! {
            elicitation::GeoPoint { coord: #coord }
        }
    }
}
