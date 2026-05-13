//! Trenchcoat wrapper for [`rstar::primitives::Rectangle<[f64; 2]>`].

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use rstar::{AABB, PointDistance, RTreeObject, primitives::Rectangle};

use super::point2::{elicit_point2, point2_prompt_tree};

/// Elicitable wrapper for [`rstar::primitives::Rectangle<[f64; 2]>`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct RstarRectangle {
    /// Lower corner of the rectangle.
    pub lower: [f64; 2],
    /// Upper corner of the rectangle.
    pub upper: [f64; 2],
}

impl RstarRectangle {
    /// Converts this wrapper into the upstream `rstar` type.
    pub fn into_inner(self) -> Rectangle<[f64; 2]> {
        self.into()
    }
}

impl From<Rectangle<[f64; 2]>> for RstarRectangle {
    fn from(value: Rectangle<[f64; 2]>) -> Self {
        Self {
            lower: value.lower(),
            upper: value.upper(),
        }
    }
}

impl From<RstarRectangle> for Rectangle<[f64; 2]> {
    fn from(value: RstarRectangle) -> Self {
        Self::from_corners(value.lower, value.upper)
    }
}

impl RTreeObject for RstarRectangle {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        Rectangle::from(*self).envelope()
    }
}

impl PointDistance for RstarRectangle {
    fn distance_2(
        &self,
        point: &<Self::Envelope as rstar::Envelope>::Point,
    ) -> <<Self::Envelope as rstar::Envelope>::Point as rstar::Point>::Scalar {
        Rectangle::from(*self).distance_2(point)
    }

    fn contains_point(&self, point: &<Self::Envelope as rstar::Envelope>::Point) -> bool {
        Rectangle::from(*self).contains_point(point)
    }
}

crate::default_style!(RstarRectangle => RstarRectangleStyle);

impl Prompt for RstarRectangle {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2D rstar rectangle by lower and upper corners:")
    }
}

impl Elicitation for RstarRectangle {
    type Style = RstarRectangleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            lower: elicit_point2(communicator).await?,
            upper: elicit_point2(communicator).await?,
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

impl ElicitIntrospect for RstarRectangle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "rstar::primitives::Rectangle<[f64; 2]>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "lower",
                        type_name: "[f64; 2]",
                        prompt: Some("Lower corner:"),
                    },
                    FieldInfo {
                        name: "upper",
                        type_name: "[f64; 2]",
                        prompt: Some("Upper corner:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for RstarRectangle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "RstarRectangle".to_string(),
            fields: vec![
                (
                    "lower".to_string(),
                    Box::new(point2_prompt_tree("Lower corner:")),
                ),
                (
                    "upper".to_string(),
                    Box::new(point2_prompt_tree("Upper corner:")),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for RstarRectangle {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let lower = <[f64; 2] as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.lower);
        let upper = <[f64; 2] as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.upper);
        quote::quote! {
            ::elicitation::RstarRectangle { lower: #lower, upper: #upper }
        }
    }
}
