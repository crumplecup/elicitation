//! Wrapper for [`egui::Stroke`] with full elicitation support.

use super::color32::EguiColor32;
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Line stroke (wrapper for `egui::Stroke`).
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
pub struct EguiStroke {
    /// Stroke width in points.
    pub width: f32,
    /// Stroke color.
    pub color: EguiColor32,
}

impl From<egui::Stroke> for EguiStroke {
    fn from(s: egui::Stroke) -> Self {
        Self {
            width: s.width,
            color: EguiColor32::from(s.color),
        }
    }
}

impl From<EguiStroke> for egui::Stroke {
    fn from(s: EguiStroke) -> Self {
        egui::Stroke::new(s.width, egui::Color32::from(s.color))
    }
}

crate::default_style!(EguiStroke => EguiStrokeStyle);

impl Prompt for EguiStroke {
    fn prompt() -> Option<&'static str> {
        Some("Specify a line stroke (width + color):")
    }
}

impl Elicitation for EguiStroke {
    type Style = EguiStrokeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiStroke");
        let width = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let color = EguiColor32::elicit(communicator).await?;
        Ok(Self { width, color })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <f32 as crate::Elicitation>::kani_proof();
        ts.extend(<EguiColor32 as crate::Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <f32 as crate::Elicitation>::verus_proof();
        ts.extend(<EguiColor32 as crate::Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <f32 as crate::Elicitation>::creusot_proof();
        ts.extend(<EguiColor32 as crate::Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for EguiStroke {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Stroke",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "width",
                        type_name: "f32",
                        prompt: Some("Stroke width (points):"),
                    },
                    FieldInfo {
                        name: "color",
                        type_name: "EguiColor32",
                        prompt: Some("Stroke color:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiStroke {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiStroke".to_string(),
            fields: vec![
                ("width".to_string(), Box::new(f32::prompt_tree())),
                ("color".to_string(), Box::new(EguiColor32::prompt_tree())),
            ],
        }
    }
}
