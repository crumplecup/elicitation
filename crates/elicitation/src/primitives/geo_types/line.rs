//! Wrapper for [`geo_types::Line<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use geo_types::{Coord, Line};

use super::coord::GeoCoord;

/// Elicitable representation of [`geo_types::Line<f64>`].
///
/// A line segment defined by a start and end coordinate.
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct GeoLine {
    /// Start coordinate.
    pub start: GeoCoord,
    /// End coordinate.
    pub end: GeoCoord,
}

impl From<Line<f64>> for GeoLine {
    fn from(l: Line<f64>) -> Self {
        Self {
            start: GeoCoord::from(l.start),
            end: GeoCoord::from(l.end),
        }
    }
}

impl From<GeoLine> for Line<f64> {
    fn from(l: GeoLine) -> Self {
        Line::new(Coord::<f64>::from(l.start), Coord::<f64>::from(l.end))
    }
}

crate::default_style!(GeoLine => GeoLineStyle);

impl Prompt for GeoLine {
    fn prompt() -> Option<&'static str> {
        Some("Specify a line segment (start, end):")
    }
}

impl Elicitation for GeoLine {
    type Style = GeoLineStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting GeoLine");
        let start = GeoCoord::elicit(communicator).await?;
        let end = GeoCoord::elicit(communicator).await?;
        Ok(Self { start, end })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("GeoLine")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("GeoLine")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("GeoLine")
    }
}

impl ElicitIntrospect for GeoLine {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "geo_types::Line<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "start",
                        type_name: "GeoCoord",
                        prompt: Some("Start coordinate:"),
                    },
                    FieldInfo {
                        name: "end",
                        type_name: "GeoCoord",
                        prompt: Some("End coordinate:"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for GeoLine {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "GeoLine".to_string(),
            fields: vec![
                ("start".to_string(), Box::new(GeoCoord::prompt_tree())),
                ("end".to_string(), Box::new(GeoCoord::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for GeoLine {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let start = self.start.to_code_literal();
        let end = self.end.to_code_literal();
        quote::quote! {
            elicitation::GeoLine { start: #start, end: #end }
        }
    }
}
