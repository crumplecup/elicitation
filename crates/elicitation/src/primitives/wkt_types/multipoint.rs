//! Wrapper for [`wkt::types::MultiPoint<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::point::WktPoint;

/// Elicitable representation of [`wkt::types::MultiPoint<f64>`].
///
/// An ordered collection of WKT points.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktMultiPoint {
    /// Collection of WKT points.
    pub points: Vec<WktPoint>,
}

impl From<wkt::types::MultiPoint<f64>> for WktMultiPoint {
    fn from(mp: wkt::types::MultiPoint<f64>) -> Self {
        Self {
            points: mp.0.into_iter().map(WktPoint::from).collect(),
        }
    }
}

impl From<WktMultiPoint> for wkt::types::MultiPoint<f64> {
    fn from(mp: WktMultiPoint) -> Self {
        wkt::types::MultiPoint(mp.points.into_iter().map(wkt::types::Point::from).collect())
    }
}

crate::default_style!(WktMultiPoint => WktMultiPointStyle);

impl Prompt for WktMultiPoint {
    fn prompt() -> Option<&'static str> {
        Some("Build a WKT multi-point (add points one by one):")
    }
}

impl Elicitation for WktMultiPoint {
    type Style = WktMultiPointStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktMultiPoint");
        let points = Vec::<WktPoint>::elicit(communicator).await?;
        Ok(Self { points })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<WktPoint>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<WktPoint>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<WktPoint>::creusot_proof()
    }
}

impl ElicitIntrospect for WktMultiPoint {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::MultiPoint<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "points",
                    type_name: "Vec<WktPoint>",
                    prompt: Some("Point collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktMultiPoint {
    fn prompt_tree() -> crate::PromptTree {
        WktPoint::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for WktMultiPoint {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.points.iter().map(|p| p.to_code_literal()).collect();
        quote::quote! {
            elicitation::WktMultiPoint { points: vec![#(#items),*] }
        }
    }
}
