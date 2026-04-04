//! Wrapper for [`egui::Color32`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// sRGBA color (wrapper for `egui::Color32`).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct EguiColor32 {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
    /// Alpha channel (0–255, 255 = opaque).
    pub a: u8,
}

impl From<egui::Color32> for EguiColor32 {
    fn from(c: egui::Color32) -> Self {
        Self {
            r: c.r(),
            g: c.g(),
            b: c.b(),
            a: c.a(),
        }
    }
}

impl From<EguiColor32> for egui::Color32 {
    fn from(c: EguiColor32) -> Self {
        egui::Color32::from_rgba_unmultiplied(c.r, c.g, c.b, c.a)
    }
}

crate::default_style!(EguiColor32 => EguiColor32Style);

impl Prompt for EguiColor32 {
    fn prompt() -> Option<&'static str> {
        Some("Specify an RGBA color:")
    }
}

impl Elicitation for EguiColor32 {
    type Style = EguiColor32Style;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiColor32");
        let r = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let g = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let b = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let a = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        Ok(Self { r, g, b, a })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiColor32")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiColor32")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiColor32")
    }
}

impl ElicitIntrospect for EguiColor32 {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Color32",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "r",
                        type_name: "u8",
                        prompt: Some("Red (0–255):"),
                    },
                    FieldInfo {
                        name: "g",
                        type_name: "u8",
                        prompt: Some("Green (0–255):"),
                    },
                    FieldInfo {
                        name: "b",
                        type_name: "u8",
                        prompt: Some("Blue (0–255):"),
                    },
                    FieldInfo {
                        name: "a",
                        type_name: "u8",
                        prompt: Some("Alpha (0–255):"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for EguiColor32 {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiColor32".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(u8::prompt_tree())),
                ("g".to_string(), Box::new(u8::prompt_tree())),
                ("b".to_string(), Box::new(u8::prompt_tree())),
                ("a".to_string(), Box::new(u8::prompt_tree())),
            ],
        }
    }
}
