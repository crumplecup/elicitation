//! Wrapper for [`geo_types::MultiPolygon<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::MultiPolygon;

use super::polygon::GeoPolygon;

/// Elicitable representation of [`geo_types::MultiPolygon<f64>`].
///
/// An ordered collection of polygons.
/// Elicitation collects polygons iteratively until the user stops adding.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoMultiPolygon(pub Vec<GeoPolygon>);

impl From<MultiPolygon<f64>> for GeoMultiPolygon {
    fn from(mp: MultiPolygon<f64>) -> Self {
        Self(mp.0.into_iter().map(GeoPolygon::from).collect())
    }
}

impl From<GeoMultiPolygon> for MultiPolygon<f64> {
    fn from(mp: GeoMultiPolygon) -> Self {
        MultiPolygon(mp.0.into_iter().map(Into::into).collect())
    }
}

crate::default_style!(GeoMultiPolygon => GeoMultiPolygonStyle);

impl Prompt for GeoMultiPolygon {
    fn prompt() -> Option<&'static str> {
        Some("Build a multi-polygon collection (add polygons one by one):")
    }
}

impl Elicitation for GeoMultiPolygon {
    type Style = GeoMultiPolygonStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoMultiPolygon");
        let polygons = Vec::<GeoPolygon>::elicit(communicator).await?;
        Ok(Self(polygons))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<GeoPolygon>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<GeoPolygon>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<GeoPolygon>::creusot_proof()
    }
}

impl ElicitIntrospect for GeoMultiPolygon {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::MultiPolygon<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "polygons",
                    type_name: "Vec<GeoPolygon>",
                    prompt: Some("Polygon collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoMultiPolygon {
    fn prompt_tree() -> crate::PromptTree {
        GeoPolygon::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for GeoMultiPolygon {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.0.iter().map(|p| p.to_code_literal()).collect();
        quote::quote! {
            elicitation::GeoMultiPolygon(vec![#(#items),*])
        }
    }
}
