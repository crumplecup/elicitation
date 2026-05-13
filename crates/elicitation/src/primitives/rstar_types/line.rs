//! Trenchcoat wrapper for [`rstar::primitives::Line<[f64; 2]>`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use rstar::{AABB, PointDistance, RTreeObject, primitives::Line};

use super::point2::{elicit_point2, point2_prompt_tree};

/// Elicitable wrapper for [`rstar::primitives::Line<[f64; 2]>`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct RstarLine {
    /// Start point of the line.
    pub from: [f64; 2],
    /// End point of the line.
    pub to: [f64; 2],
}

impl RstarLine {
    /// Converts this wrapper into the upstream `rstar` type.
    pub fn into_inner(self) -> Line<[f64; 2]> {
        self.into()
    }
}

impl From<Line<[f64; 2]>> for RstarLine {
    fn from(value: Line<[f64; 2]>) -> Self {
        Self {
            from: value.from,
            to: value.to,
        }
    }
}

impl From<RstarLine> for Line<[f64; 2]> {
    fn from(value: RstarLine) -> Self {
        Self::new(value.from, value.to)
    }
}

impl RTreeObject for RstarLine {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        Line::from(*self).envelope()
    }
}

impl PointDistance for RstarLine {
    fn distance_2(
        &self,
        point: &<Self::Envelope as rstar::Envelope>::Point,
    ) -> <<Self::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar {
        Line::from(*self).distance_2(point)
    }

    fn contains_point(&self, point: &<Self::Envelope as rstar::Envelope>::Point) -> bool {
        Line::from(*self).contains_point(point)
    }
}

crate::default_style!(RstarLine => RstarLineStyle);

impl Prompt for RstarLine {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2D line by start and end points:")
    }
}

impl Elicitation for RstarLine {
    type Style = RstarLineStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            from: elicit_point2(communicator).await?,
            to: elicit_point2(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <[f64; 2] as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <[f64; 2] as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <[f64; 2] as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for RstarLine {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "rstar::primitives::Line<[f64; 2]>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "from",
                        type_name: "[f64; 2]",
                        prompt: Some("Start point:"),
                    },
                    FieldInfo {
                        name: "to",
                        type_name: "[f64; 2]",
                        prompt: Some("End point:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for RstarLine {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "RstarLine".to_string(),
            fields: vec![
                (
                    "from".to_string(),
                    Box::new(point2_prompt_tree("Start point:")),
                ),
                ("to".to_string(), Box::new(point2_prompt_tree("End point:"))),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for RstarLine {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let from = <[f64; 2] as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.from);
        let to = <[f64; 2] as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.to);
        quote::quote! {
            ::elicitation::RstarLine { from: #from, to: #to }
        }
    }
}
