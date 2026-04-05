//! Wrapper for [`egui::Vec2`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// 2D vector / size (wrapper for `egui::Vec2`).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct EguiVec2 {
    /// X component (width or horizontal direction).
    pub x: f32,
    /// Y component (height or vertical direction).
    pub y: f32,
}

impl From<egui::Vec2> for EguiVec2 {
    fn from(v: egui::Vec2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<EguiVec2> for egui::Vec2 {
    fn from(v: EguiVec2) -> Self {
        egui::vec2(v.x, v.y)
    }
}

crate::default_style!(EguiVec2 => EguiVec2Style);

impl Prompt for EguiVec2 {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2D size or direction:")
    }
}

impl Elicitation for EguiVec2 {
    type Style = EguiVec2Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiVec2");
        let x = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let y = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        Ok(Self { x, y })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiVec2")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiVec2")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiVec2")
    }
}

impl ElicitIntrospect for EguiVec2 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Vec2",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X component:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y component:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiVec2 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiVec2".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
