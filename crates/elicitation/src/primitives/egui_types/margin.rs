//! Wrapper for [`egui::Margin`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Spacing margin on each side (wrapper for `egui::Margin`).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct EguiMargin {
    /// Left margin.
    pub left: i8,
    /// Right margin.
    pub right: i8,
    /// Top margin.
    pub top: i8,
    /// Bottom margin.
    pub bottom: i8,
}

impl From<egui::Margin> for EguiMargin {
    fn from(m: egui::Margin) -> Self {
        Self {
            left: m.left,
            right: m.right,
            top: m.top,
            bottom: m.bottom,
        }
    }
}

impl From<EguiMargin> for egui::Margin {
    fn from(m: EguiMargin) -> Self {
        egui::Margin {
            left: m.left,
            right: m.right,
            top: m.top,
            bottom: m.bottom,
        }
    }
}

crate::default_style!(EguiMargin => EguiMarginStyle);

impl Prompt for EguiMargin {
    fn prompt() -> Option<&'static str> {
        Some("Specify margins (left, right, top, bottom):")
    }
}

impl Elicitation for EguiMargin {
    type Style = EguiMarginStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiMargin");
        let left = crate::verification::types::I8Default::elicit(communicator)
            .await?
            .get();
        let right = crate::verification::types::I8Default::elicit(communicator)
            .await?
            .get();
        let top = crate::verification::types::I8Default::elicit(communicator)
            .await?
            .get();
        let bottom = crate::verification::types::I8Default::elicit(communicator)
            .await?
            .get();
        Ok(Self {
            left,
            right,
            top,
            bottom,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiMargin")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiMargin")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiMargin")
    }
}

impl ElicitIntrospect for EguiMargin {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::Margin",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "left",
                        type_name: "i8",
                        prompt: Some("Left margin:"),
                    },
                    FieldInfo {
                        name: "right",
                        type_name: "i8",
                        prompt: Some("Right margin:"),
                    },
                    FieldInfo {
                        name: "top",
                        type_name: "i8",
                        prompt: Some("Top margin:"),
                    },
                    FieldInfo {
                        name: "bottom",
                        type_name: "i8",
                        prompt: Some("Bottom margin:"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for EguiMargin {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiMargin".to_string(),
            fields: vec![
                ("left".to_string(), Box::new(i8::prompt_tree())),
                ("right".to_string(), Box::new(i8::prompt_tree())),
                ("top".to_string(), Box::new(i8::prompt_tree())),
                ("bottom".to_string(), Box::new(i8::prompt_tree())),
            ],
        }
    }
}
