//! Wrapper for [`wkt::types::Point<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::coord::WktCoord;

/// Elicitable representation of [`wkt::types::Point<f64>`].
///
/// A WKT point — optionally contains a coordinate (empty points are valid WKT).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktPoint {
    /// The coordinate, or `None` for an empty point.
    pub coord: Option<WktCoord>,
}

impl From<wkt::types::Point<f64>> for WktPoint {
    fn from(p: wkt::types::Point<f64>) -> Self {
        let (coord, _dim) = p.into_inner();
        Self {
            coord: coord.map(WktCoord::from),
        }
    }
}

impl From<WktPoint> for wkt::types::Point<f64> {
    fn from(p: WktPoint) -> Self {
        wkt::types::Point::new(
            p.coord.map(wkt::types::Coord::from),
            wkt::types::Dimension::XY,
        )
    }
}

crate::default_style!(WktPoint => WktPointStyle);

impl Prompt for WktPoint {
    fn prompt() -> Option<&'static str> {
        Some("Specify a WKT point (coordinate, or empty):")
    }
}

impl Elicitation for WktPoint {
    type Style = WktPointStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktPoint");
        let coord = WktCoord::elicit(communicator).await?;
        Ok(Self { coord: Some(coord) })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        WktCoord::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        WktCoord::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        WktCoord::creusot_proof()
    }
}

impl ElicitIntrospect for WktPoint {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::Point<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "coord",
                    type_name: "Option<WktCoord>",
                    prompt: Some("Coordinate (x, y):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktPoint {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "WktPoint".to_string(),
            fields: vec![("coord".to_string(), Box::new(WktCoord::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for WktPoint {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let coord_tokens = match &self.coord {
            Some(c) => {
                let inner = c.to_code_literal();
                quote::quote! { Some(#inner) }
            }
            None => quote::quote! { None },
        };
        quote::quote! {
            elicitation::WktPoint { coord: #coord_tokens }
        }
    }
}
