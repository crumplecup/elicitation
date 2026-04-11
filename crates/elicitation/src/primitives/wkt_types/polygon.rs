//! Wrapper for [`wkt::types::Polygon<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::{coord::WktCoord, linestring::WktLineString};

/// Elicitable representation of [`wkt::types::Polygon<f64>`].
///
/// A WKT polygon with an exterior ring and optional interior rings (holes).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktPolygon {
    /// Exterior ring of the polygon.
    pub exterior: WktLineString,
    /// Interior rings (holes), if any.
    pub interiors: Vec<WktLineString>,
}

impl From<wkt::types::Polygon<f64>> for WktPolygon {
    fn from(p: wkt::types::Polygon<f64>) -> Self {
        // wkt::types::Polygon is a tuple struct: first ring is exterior, rest are holes.
        let mut rings = p.0.into_iter();
        let exterior = rings
            .next()
            .map(WktLineString::from)
            .unwrap_or_else(|| WktLineString { coords: vec![] });
        let interiors = rings.map(WktLineString::from).collect();
        Self {
            exterior,
            interiors,
        }
    }
}

impl From<WktPolygon> for wkt::types::Polygon<f64> {
    fn from(p: WktPolygon) -> Self {
        let mut rings = vec![wkt::types::LineString::from(p.exterior)];
        rings.extend(p.interiors.into_iter().map(wkt::types::LineString::from));
        wkt::types::Polygon(rings)
    }
}

crate::default_style!(WktPolygon => WktPolygonStyle);

impl Prompt for WktPolygon {
    fn prompt() -> Option<&'static str> {
        Some("Specify a WKT polygon (exterior ring, then optional holes):")
    }
}

impl Elicitation for WktPolygon {
    type Style = WktPolygonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktPolygon");
        let exterior = WktLineString::elicit(communicator).await?;
        let interiors = Vec::<WktLineString>::elicit(communicator).await?;
        Ok(Self {
            exterior,
            interiors,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        WktLineString::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        WktLineString::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        WktLineString::creusot_proof()
    }
}

impl ElicitIntrospect for WktPolygon {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::Polygon<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "exterior",
                        type_name: "WktLineString",
                        prompt: Some("Exterior ring:"),
                    },
                    FieldInfo {
                        name: "interiors",
                        type_name: "Vec<WktLineString>",
                        prompt: Some("Interior rings / holes (add iteratively):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktPolygon {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "WktPolygon".to_string(),
            fields: vec![
                (
                    "exterior".to_string(),
                    Box::new(WktLineString::prompt_tree()),
                ),
                ("interiors".to_string(), Box::new(WktCoord::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for WktPolygon {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let exterior = self.exterior.to_code_literal();
        let interiors: Vec<_> = self.interiors.iter().map(|r| r.to_code_literal()).collect();
        quote::quote! {
            elicitation::WktPolygon {
                exterior: #exterior,
                interiors: vec![#(#interiors),*],
            }
        }
    }
}
