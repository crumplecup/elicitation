//! Wrapper for [`egui::Rect`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Axis-aligned rectangle (wrapper for `egui::Rect`).
///
/// Flattened to four `f32` fields for JSON simplicity.
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
pub struct EguiRect {
    /// Minimum X (left edge).
    pub min_x: f32,
    /// Minimum Y (top edge).
    pub min_y: f32,
    /// Maximum X (right edge).
    pub max_x: f32,
    /// Maximum Y (bottom edge).
    pub max_y: f32,
}

impl From<egui::Rect> for EguiRect {
    fn from(r: egui::Rect) -> Self {
        Self {
            min_x: r.min.x,
            min_y: r.min.y,
            max_x: r.max.x,
            max_y: r.max.y,
        }
    }
}

impl From<EguiRect> for egui::Rect {
    fn from(r: EguiRect) -> Self {
        egui::Rect::from_min_max(egui::pos2(r.min_x, r.min_y), egui::pos2(r.max_x, r.max_y))
    }
}

crate::default_style!(EguiRect => EguiRectStyle);

impl Prompt for EguiRect {
    fn prompt() -> Option<&'static str> {
        Some("Specify a rectangle (min/max corners):")
    }
}

impl Elicitation for EguiRect {
    type Style = EguiRectStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiRect");
        let min_x = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let min_y = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let max_x = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let max_y = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        Ok(Self {
            min_x,
            min_y,
            max_x,
            max_y,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiRect")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiRect")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiRect")
    }
}

impl ElicitIntrospect for EguiRect {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Rect",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "min_x",
                        type_name: "f32",
                        prompt: Some("Minimum X (left):"),
                    },
                    FieldInfo {
                        name: "min_y",
                        type_name: "f32",
                        prompt: Some("Minimum Y (top):"),
                    },
                    FieldInfo {
                        name: "max_x",
                        type_name: "f32",
                        prompt: Some("Maximum X (right):"),
                    },
                    FieldInfo {
                        name: "max_y",
                        type_name: "f32",
                        prompt: Some("Maximum Y (bottom):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiRect {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiRect".to_string(),
            fields: vec![
                ("min_x".to_string(), Box::new(f32::prompt_tree())),
                ("min_y".to_string(), Box::new(f32::prompt_tree())),
                ("max_x".to_string(), Box::new(f32::prompt_tree())),
                ("max_y".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}
