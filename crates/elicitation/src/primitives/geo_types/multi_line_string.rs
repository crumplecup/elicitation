//! Wrapper for [`geo_types::MultiLineString<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::MultiLineString;

use super::line_string::GeoLineString;

/// Elicitable representation of [`geo_types::MultiLineString<f64>`].
///
/// An ordered collection of line strings.
/// Elicitation collects line strings iteratively until the user stops adding.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct GeoMultiLineString(pub Vec<GeoLineString>);

impl From<MultiLineString<f64>> for GeoMultiLineString {
    fn from(mls: MultiLineString<f64>) -> Self {
        Self(mls.0.into_iter().map(GeoLineString::from).collect())
    }
}

impl From<GeoMultiLineString> for MultiLineString<f64> {
    fn from(mls: GeoMultiLineString) -> Self {
        MultiLineString(mls.0.into_iter().map(Into::into).collect())
    }
}

crate::default_style!(GeoMultiLineString => GeoMultiLineStringStyle);

impl Prompt for GeoMultiLineString {
    fn prompt() -> Option<&'static str> {
        Some("Build a multi-line-string collection (add line strings one by one):")
    }
}

impl Elicitation for GeoMultiLineString {
    type Style = GeoMultiLineStringStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoMultiLineString");
        let line_strings = Vec::<GeoLineString>::elicit(communicator).await?;
        Ok(Self(line_strings))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<GeoLineString>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<GeoLineString>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<GeoLineString>::creusot_proof()
    }
}

impl ElicitIntrospect for GeoMultiLineString {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::MultiLineString<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "line_strings",
                    type_name: "Vec<GeoLineString>",
                    prompt: Some("Line string collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for GeoMultiLineString {
    fn prompt_tree() -> crate::PromptTree {
        GeoLineString::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for GeoMultiLineString {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.0.iter().map(|ls| ls.to_code_literal()).collect();
        quote::quote! {
            elicitation::GeoMultiLineString(vec![#(#items),*])
        }
    }
}
