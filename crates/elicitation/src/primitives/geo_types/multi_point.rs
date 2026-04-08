//! Wrapper for [`geo_types::MultiPoint<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::MultiPoint;

use super::point::GeoPoint;

/// Elicitable representation of [`geo_types::MultiPoint<f64>`].
///
/// An ordered collection of geographic points.
/// Elicitation collects points iteratively until the user stops adding.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoMultiPoint(pub Vec<GeoPoint>);

impl From<MultiPoint<f64>> for GeoMultiPoint {
    fn from(mp: MultiPoint<f64>) -> Self {
        Self(mp.0.into_iter().map(GeoPoint::from).collect())
    }
}

impl From<GeoMultiPoint> for MultiPoint<f64> {
    fn from(mp: GeoMultiPoint) -> Self {
        MultiPoint(mp.0.into_iter().map(Into::into).collect())
    }
}

crate::default_style!(GeoMultiPoint => GeoMultiPointStyle);

impl Prompt for GeoMultiPoint {
    fn prompt() -> Option<&'static str> {
        Some("Build a multi-point collection (add points one by one):")
    }
}

impl Elicitation for GeoMultiPoint {
    type Style = GeoMultiPointStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoMultiPoint");
        let points = Vec::<GeoPoint>::elicit(communicator).await?;
        Ok(Self(points))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<GeoPoint>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<GeoPoint>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<GeoPoint>::creusot_proof()
    }
}

impl ElicitIntrospect for GeoMultiPoint {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::MultiPoint<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "points",
                    type_name: "Vec<GeoPoint>",
                    prompt: Some("Point collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoMultiPoint {
    fn prompt_tree() -> crate::PromptTree {
        GeoPoint::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for GeoMultiPoint {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.0.iter().map(|p| p.to_code_literal()).collect();
        quote::quote! {
            elicitation::GeoMultiPoint(vec![#(#items),*])
        }
    }
}
