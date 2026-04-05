//! Wrapper for [`egui::CornerRadius`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Per-corner radii (wrapper for `egui::CornerRadius`).
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
pub struct EguiCornerRadius {
    /// North-west (top-left) radius.
    pub nw: u8,
    /// North-east (top-right) radius.
    pub ne: u8,
    /// South-west (bottom-left) radius.
    pub sw: u8,
    /// South-east (bottom-right) radius.
    pub se: u8,
}

impl From<egui::CornerRadius> for EguiCornerRadius {
    fn from(r: egui::CornerRadius) -> Self {
        Self {
            nw: r.nw,
            ne: r.ne,
            sw: r.sw,
            se: r.se,
        }
    }
}

impl From<EguiCornerRadius> for egui::CornerRadius {
    fn from(r: EguiCornerRadius) -> Self {
        egui::CornerRadius {
            nw: r.nw,
            ne: r.ne,
            sw: r.sw,
            se: r.se,
        }
    }
}

crate::default_style!(EguiCornerRadius => EguiCornerRadiusStyle);

impl Prompt for EguiCornerRadius {
    fn prompt() -> Option<&'static str> {
        Some("Specify corner radii (NW, NE, SW, SE):")
    }
}

impl Elicitation for EguiCornerRadius {
    type Style = EguiCornerRadiusStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiCornerRadius");
        let nw = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let ne = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let sw = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        let se = crate::verification::types::U8Default::elicit(communicator)
            .await?
            .get();
        Ok(Self { nw, ne, sw, se })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiCornerRadius")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiCornerRadius")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiCornerRadius")
    }
}

impl ElicitIntrospect for EguiCornerRadius {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::CornerRadius",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "nw",
                        type_name: "u8",
                        prompt: Some("North-west radius:"),
                    },
                    FieldInfo {
                        name: "ne",
                        type_name: "u8",
                        prompt: Some("North-east radius:"),
                    },
                    FieldInfo {
                        name: "sw",
                        type_name: "u8",
                        prompt: Some("South-west radius:"),
                    },
                    FieldInfo {
                        name: "se",
                        type_name: "u8",
                        prompt: Some("South-east radius:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for EguiCornerRadius {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiCornerRadius".to_string(),
            fields: vec![
                ("nw".to_string(), Box::new(u8::prompt_tree())),
                ("ne".to_string(), Box::new(u8::prompt_tree())),
                ("sw".to_string(), Box::new(u8::prompt_tree())),
                ("se".to_string(), Box::new(u8::prompt_tree())),
            ],
        }
    }
}
