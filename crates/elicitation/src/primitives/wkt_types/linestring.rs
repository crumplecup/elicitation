//! Wrapper for [`wkt::types::LineString<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::coord::WktCoord;

/// Elicitable representation of [`wkt::types::LineString<f64>`].
///
/// An ordered sequence of WKT coordinates.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktLineString {
    /// Ordered sequence of coordinates.
    pub coords: Vec<WktCoord>,
}

impl From<wkt::types::LineString<f64>> for WktLineString {
    fn from(ls: wkt::types::LineString<f64>) -> Self {
        Self {
            coords: ls.0.into_iter().map(WktCoord::from).collect(),
        }
    }
}

impl From<WktLineString> for wkt::types::LineString<f64> {
    fn from(ls: WktLineString) -> Self {
        wkt::types::LineString(ls.coords.into_iter().map(wkt::types::Coord::from).collect())
    }
}

crate::default_style!(WktLineString => WktLineStringStyle);

impl Prompt for WktLineString {
    fn prompt() -> Option<&'static str> {
        Some("Build a WKT line string (add coordinates one by one):")
    }
}

impl Elicitation for WktLineString {
    type Style = WktLineStringStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktLineString");
        let coords = Vec::<WktCoord>::elicit(communicator).await?;
        Ok(Self { coords })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<WktCoord>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<WktCoord>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<WktCoord>::creusot_proof()
    }
}

impl ElicitIntrospect for WktLineString {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::LineString<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "coords",
                    type_name: "Vec<WktCoord>",
                    prompt: Some("Coordinate sequence (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktLineString {
    fn prompt_tree() -> crate::PromptTree {
        WktCoord::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for WktLineString {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.coords.iter().map(|c| c.to_code_literal()).collect();
        quote::quote! {
            elicitation::WktLineString { coords: vec![#(#items),*] }
        }
    }
}
