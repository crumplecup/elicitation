//! Wrapper for [`wkt::types::MultiPolygon<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::polygon::WktPolygon;

/// Elicitable representation of [`wkt::types::MultiPolygon<f64>`].
///
/// An ordered collection of WKT polygons.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktMultiPolygon {
    /// Collection of WKT polygons.
    pub polygons: Vec<WktPolygon>,
}

impl From<wkt::types::MultiPolygon<f64>> for WktMultiPolygon {
    fn from(mp: wkt::types::MultiPolygon<f64>) -> Self {
        Self {
            polygons: mp.0.into_iter().map(WktPolygon::from).collect(),
        }
    }
}

impl From<WktMultiPolygon> for wkt::types::MultiPolygon<f64> {
    fn from(mp: WktMultiPolygon) -> Self {
        wkt::types::MultiPolygon(
            mp.polygons
                .into_iter()
                .map(wkt::types::Polygon::from)
                .collect(),
        )
    }
}

crate::default_style!(WktMultiPolygon => WktMultiPolygonStyle);

impl Prompt for WktMultiPolygon {
    fn prompt() -> Option<&'static str> {
        Some("Build a WKT multi-polygon (add polygons one by one):")
    }
}

impl Elicitation for WktMultiPolygon {
    type Style = WktMultiPolygonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktMultiPolygon");
        let polygons = Vec::<WktPolygon>::elicit(communicator).await?;
        Ok(Self { polygons })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<WktPolygon>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<WktPolygon>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<WktPolygon>::creusot_proof()
    }
}

impl ElicitIntrospect for WktMultiPolygon {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::MultiPolygon<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "polygons",
                    type_name: "Vec<WktPolygon>",
                    prompt: Some("Polygon collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktMultiPolygon {
    fn prompt_tree() -> crate::PromptTree {
        WktPolygon::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for WktMultiPolygon {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.polygons.iter().map(|p| p.to_code_literal()).collect();
        quote::quote! {
            elicitation::WktMultiPolygon { polygons: vec![#(#items),*] }
        }
    }
}
