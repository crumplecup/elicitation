//! Wrapper for [`egui::FontId`] with full elicitation support.

use super::trenchcoats::FontFamilySelect;
use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

/// Font identifier (wrapper for `egui::FontId`).
///
/// Uses [`FontFamilySelect`] for the family field, which covers the
/// unit variants of `egui::FontFamily` (Monospace, Proportional).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct EguiFontId {
    /// Font size in points.
    pub size: f32,
    /// Font family (Monospace or Proportional).
    pub family: FontFamilySelect,
}

impl From<egui::FontId> for EguiFontId {
    fn from(f: egui::FontId) -> Self {
        Self {
            size: f.size,
            family: FontFamilySelect::from(f.family),
        }
    }
}

impl From<EguiFontId> for egui::FontId {
    fn from(f: EguiFontId) -> Self {
        egui::FontId {
            size: f.size,
            family: f.family.into_inner(),
        }
    }
}

crate::default_style!(EguiFontId => EguiFontIdStyle);

impl Prompt for EguiFontId {
    fn prompt() -> Option<&'static str> {
        Some("Specify a font (size + family):")
    }
}

impl Elicitation for EguiFontId {
    type Style = EguiFontIdStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting EguiFontId");
        let size = crate::verification::types::F32Default::elicit(communicator)
            .await?
            .get();
        let family = FontFamilySelect::elicit(communicator).await?;
        Ok(Self { size, family })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("EguiFontId")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("EguiFontId")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("EguiFontId")
    }
}

impl ElicitIntrospect for EguiFontId {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "egui::FontId",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "size",
                        type_name: "f32",
                        prompt: Some("Font size (points):"),
                    },
                    FieldInfo {
                        name: "family",
                        type_name: "FontFamilySelect",
                        prompt: Some("Font family:"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for EguiFontId {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "EguiFontId".to_string(),
            fields: vec![
                ("size".to_string(), Box::new(f32::prompt_tree())),
                (
                    "family".to_string(),
                    Box::new(FontFamilySelect::prompt_tree()),
                ),
            ],
        }
    }
}
