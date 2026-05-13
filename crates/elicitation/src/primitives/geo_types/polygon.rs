//! Wrapper for [`geo_types::Polygon<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::Polygon;

use super::{coord::GeoCoord, line_string::GeoLineString};

/// Elicitable representation of [`geo_types::Polygon<f64>`].
///
/// A polygon defined by an exterior ring and zero or more interior rings (holes).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoPolygon {
    /// Exterior ring of the polygon.
    pub exterior: GeoLineString,
    /// Interior rings (holes), if any.
    pub interiors: Vec<GeoLineString>,
}

impl From<Polygon<f64>> for GeoPolygon {
    fn from(p: Polygon<f64>) -> Self {
        let (exterior, interiors) = p.into_inner();
        Self {
            exterior: GeoLineString::from(exterior),
            interiors: interiors.into_iter().map(GeoLineString::from).collect(),
        }
    }
}

impl From<GeoPolygon> for Polygon<f64> {
    fn from(p: GeoPolygon) -> Self {
        Polygon::new(
            p.exterior.into(),
            p.interiors.into_iter().map(Into::into).collect(),
        )
    }
}

crate::default_style!(GeoPolygon => GeoPolygonStyle);

impl Prompt for GeoPolygon {
    fn prompt() -> Option<&'static str> {
        Some("Specify a polygon (exterior ring, then optional holes):")
    }
}

impl Elicitation for GeoPolygon {
    type Style = GeoPolygonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoPolygon");
        let exterior = GeoLineString::elicit(communicator).await?;
        let interiors = Vec::<GeoLineString>::elicit(communicator).await?;
        Ok(Self {
            exterior,
            interiors,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        // GeoPolygon has exterior + interiors: both GeoLineString — delegate to compose.
        GeoLineString::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        GeoLineString::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        GeoLineString::creusot_proof()
    }
}

impl ElicitIntrospect for GeoPolygon {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Polygon<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "exterior",
                        type_name: "GeoLineString",
                        prompt: Some("Exterior ring:"),
                    },
                    FieldInfo {
                        name: "interiors",
                        type_name: "Vec<GeoLineString>",
                        prompt: Some("Interior rings / holes (add iteratively):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoPolygon {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "GeoPolygon".to_string(),
            fields: vec![
                (
                    "exterior".to_string(),
                    Box::new(GeoLineString::prompt_tree()),
                ),
                ("interiors".to_string(), Box::new(GeoCoord::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoPolygon {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let exterior = self.exterior.to_code_literal();
        let interiors: Vec<_> = self.interiors.iter().map(|r| r.to_code_literal()).collect();
        quote::quote! {
            elicitation::GeoPolygon {
                exterior: #exterior,
                interiors: vec![#(#interiors),*],
            }
        }
    }
}
