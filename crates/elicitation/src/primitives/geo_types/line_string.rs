//! Wrapper for [`geo_types::LineString<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::LineString;

use super::coord::GeoCoord;

/// Elicitable representation of [`geo_types::LineString<f64>`].
///
/// An ordered sequence of 2D coordinates forming an open or closed path.
/// Elicitation collects coordinates iteratively until the user stops adding.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoLineString(pub Vec<GeoCoord>);

impl From<LineString<f64>> for GeoLineString {
    fn from(ls: LineString<f64>) -> Self {
        Self(ls.0.into_iter().map(GeoCoord::from).collect())
    }
}

impl From<GeoLineString> for LineString<f64> {
    fn from(ls: GeoLineString) -> Self {
        LineString(ls.0.into_iter().map(Into::into).collect())
    }
}

crate::default_style!(GeoLineString => GeoLineStringStyle);

impl Prompt for GeoLineString {
    fn prompt() -> Option<&'static str> {
        Some("Build a line string (add coordinates one by one):")
    }
}

impl Elicitation for GeoLineString {
    type Style = GeoLineStringStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoLineString");
        let coords = Vec::<GeoCoord>::elicit(communicator).await?;
        Ok(Self(coords))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<GeoCoord>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<GeoCoord>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<GeoCoord>::creusot_proof()
    }
}

impl ElicitIntrospect for GeoLineString {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::LineString<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "coords",
                    type_name: "Vec<GeoCoord>",
                    prompt: Some("Coordinate sequence (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoLineString {
    fn prompt_tree() -> crate::PromptTree {
        GeoCoord::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for GeoLineString {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.0.iter().map(|c| c.to_code_literal()).collect();
        quote::quote! {
            elicitation::GeoLineString(vec![#(#items),*])
        }
    }
}
