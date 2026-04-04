//! Wrapper for [`egui::Pos2`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// 2D point (wrapper for `egui::Pos2`).
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
pub struct EguiPos2 {
    /// X coordinate.
    pub x: f32,
    /// Y coordinate.
    pub y: f32,
}

impl From<egui::Pos2> for EguiPos2 {
    fn from(p: egui::Pos2) -> Self {
        Self { x: p.x, y: p.y }
    }
}

impl From<EguiPos2> for egui::Pos2 {
    fn from(p: EguiPos2) -> Self {
        egui::pos2(p.x, p.y)
    }
}

crate::default_style!(EguiPos2 => EguiPos2Style);

impl Prompt for EguiPos2 {
    fn prompt() -> Option<&'static str> {
        Some("Specify a 2D point:")
    }
}

impl Elicitation for EguiPos2 {
    type Style = EguiPos2Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiPos2");
        let x = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let y = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        Ok(Self { x, y })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiPos2")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiPos2")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiPos2")
    }
}

impl ElicitIntrospect for EguiPos2 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Pos2",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X coordinate:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y coordinate:"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for EguiPos2 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiPos2".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
