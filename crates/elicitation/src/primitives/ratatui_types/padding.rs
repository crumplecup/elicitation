//! Wrapper for [`ratatui::widgets::Padding`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use ratatui::widgets::Padding;

/// Elicitable representation of [`ratatui::widgets::Padding`].
///
/// Wraps the ratatui `Padding` struct, providing field-by-field elicitation
/// for left, right, top, and bottom values.
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
)]
pub struct RatatuiPadding {
    /// Left padding.
    #[serde(default)]
    pub left: u16,
    /// Right padding.
    #[serde(default)]
    pub right: u16,
    /// Top padding.
    #[serde(default)]
    pub top: u16,
    /// Bottom padding.
    #[serde(default)]
    pub bottom: u16,
}

impl From<Padding> for RatatuiPadding {
    fn from(p: Padding) -> Self {
        Self {
            left: p.left,
            right: p.right,
            top: p.top,
            bottom: p.bottom,
        }
    }
}

impl From<RatatuiPadding> for Padding {
    fn from(p: RatatuiPadding) -> Self {
        Self::new(p.left, p.right, p.top, p.bottom)
    }
}

crate::default_style!(RatatuiPadding => RatatuiPaddingStyle);

impl Prompt for RatatuiPadding {
    fn prompt() -> Option<&'static str> {
        Some("Configure inner padding:")
    }
}

impl Elicitation for RatatuiPadding {
    type Style = RatatuiPaddingStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RatatuiPadding");
        let left = u16::elicit(communicator).await?;
        let right = u16::elicit(communicator).await?;
        let top = u16::elicit(communicator).await?;
        let bottom = u16::elicit(communicator).await?;
        Ok(Self {
            left,
            right,
            top,
            bottom,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_composite_wrapper("RatatuiPadding")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_composite_wrapper("RatatuiPadding")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_composite_wrapper("RatatuiPadding")
    }
}

impl ElicitIntrospect for RatatuiPadding {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::widgets::Padding",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "left",
                        type_name: "u16",
                        prompt: Some("Left padding:"),
                    },
                    FieldInfo {
                        name: "right",
                        type_name: "u16",
                        prompt: Some("Right padding:"),
                    },
                    FieldInfo {
                        name: "top",
                        type_name: "u16",
                        prompt: Some("Top padding:"),
                    },
                    FieldInfo {
                        name: "bottom",
                        type_name: "u16",
                        prompt: Some("Bottom padding:"),
                    },
                ],
            },
        }
    }
}

#[cfg(feature = "prompt-tree")]
impl crate::ElicitPromptTree for RatatuiPadding {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RatatuiPadding".to_string(),
            fields: vec![
                ("left".to_string(), Box::new(u16::prompt_tree())),
                ("right".to_string(), Box::new(u16::prompt_tree())),
                ("top".to_string(), Box::new(u16::prompt_tree())),
                ("bottom".to_string(), Box::new(u16::prompt_tree())),
            ],
        }
    }
}
