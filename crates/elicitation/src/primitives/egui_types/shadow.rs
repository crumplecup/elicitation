//! Wrapper for [`egui::Shadow`] with full elicitation support.

use super::color32::EguiColor32;
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Drop-shadow parameters (wrapper for `egui::Shadow`).
///
/// The `offset` array is flattened to `offset_x` / `offset_y` for JSON ergonomics.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct EguiShadow {
    /// Horizontal offset (positive = right).
    pub offset_x: i8,
    /// Vertical offset (positive = down).
    pub offset_y: i8,
    /// Blur radius.
    pub blur: u8,
    /// Spread radius.
    pub spread: u8,
    /// Shadow color.
    pub color: EguiColor32,
}

impl From<egui::Shadow> for EguiShadow {
    fn from(s: egui::Shadow) -> Self {
        Self {
            offset_x: s.offset[0],
            offset_y: s.offset[1],
            blur: s.blur,
            spread: s.spread,
            color: EguiColor32::from(s.color),
        }
    }
}

impl From<EguiShadow> for egui::Shadow {
    fn from(s: EguiShadow) -> Self {
        egui::Shadow {
            offset: [s.offset_x, s.offset_y],
            blur: s.blur,
            spread: s.spread,
            color: egui::Color32::from(s.color),
        }
    }
}

crate::default_style!(EguiShadow => EguiShadowStyle);

impl Prompt for EguiShadow {
    fn prompt() -> Option<&'static str> {
        Some("Specify drop-shadow parameters:")
    }
}

impl Elicitation for EguiShadow {
    type Style = EguiShadowStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiShadow");
        let offset_x = crate::verification::types::I8Default::elicit(communicator)
            .await?
            .get();
        let offset_y = crate::verification::types::I8Default::elicit(communicator)
            .await?
            .get();
        let blur = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let spread = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let color = EguiColor32::elicit(communicator).await?;
        Ok(Self {
            offset_x,
            offset_y,
            blur,
            spread,
            color,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiShadow")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiShadow")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiShadow")
    }
}

impl ElicitIntrospect for EguiShadow {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Shadow",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "offset_x",
                        type_name: "i8",
                        prompt: Some("Horizontal offset:"),
                    },
                    FieldInfo {
                        name: "offset_y",
                        type_name: "i8",
                        prompt: Some("Vertical offset:"),
                    },
                    FieldInfo {
                        name: "blur",
                        type_name: "u8",
                        prompt: Some("Blur radius:"),
                    },
                    FieldInfo {
                        name: "spread",
                        type_name: "u8",
                        prompt: Some("Spread radius:"),
                    },
                    FieldInfo {
                        name: "color",
                        type_name: "EguiColor32",
                        prompt: Some("Shadow color:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiShadow {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiShadow".to_string(),
            fields: vec![
                ("offset_x".to_string(), Box::new(i8::prompt_tree())),
                ("offset_y".to_string(), Box::new(i8::prompt_tree())),
                ("blur".to_string(), Box::new(u8::prompt_tree())),
                ("spread".to_string(), Box::new(u8::prompt_tree())),
                ("color".to_string(), Box::new(EguiColor32::prompt_tree())),
            ],
        }
    }
}
