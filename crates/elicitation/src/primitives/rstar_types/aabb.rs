//! Trenchcoat wrapper for [`rstar::AABB<[f64; 2]>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use rstar::AABB;

use super::point2::{elicit_point2, point2_prompt_tree};

/// Elicitable wrapper for [`rstar::AABB<[f64; 2]>`].
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct RstarAabb {
    /// Lower corner of the envelope.
    pub lower: [f64; 2],
    /// Upper corner of the envelope.
    pub upper: [f64; 2],
}

impl RstarAabb {
    /// Converts this wrapper into the upstream `rstar` type.
    pub fn into_inner(self) -> AABB<[f64; 2]> {
        self.into()
    }
}

impl From<AABB<[f64; 2]>> for RstarAabb {
    fn from(value: AABB<[f64; 2]>) -> Self {
        Self {
            lower: value.lower(),
            upper: value.upper(),
        }
    }
}

impl From<RstarAabb> for AABB<[f64; 2]> {
    fn from(value: RstarAabb) -> Self {
        Self::from_corners(value.lower, value.upper)
    }
}

crate::default_style!(RstarAabb => RstarAabbStyle);

impl Prompt for RstarAabb {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2D axis-aligned bounding box by lower and upper corners:")
    }
}

impl Elicitation for RstarAabb {
    type Style = RstarAabbStyle;

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

impl ElicitIntrospect for RstarAabb {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "rstar::AABB<[f64; 2]>",
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

impl crate::ElicitPromptTree for RstarAabb {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "RstarAabb".to_string(),
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

impl crate::emit_code::ToCodeLiteral for RstarAabb {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let lower = <[f64; 2] as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.lower);
        let upper = <[f64; 2] as crate::emit_code::ToCodeLiteral>::to_code_literal(&self.upper);
        quote::quote! {
            ::elicitation::RstarAabb { lower: #lower, upper: #upper }
        }
    }
}
