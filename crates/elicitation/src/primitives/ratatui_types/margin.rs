//! Wrapper for [`ratatui::layout::Margin`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use ratatui::layout::Margin;

/// Elicitable representation of [`ratatui::layout::Margin`].
///
/// Wraps the ratatui `Margin` struct, providing field-by-field elicitation
/// for horizontal and vertical values.
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
pub struct RatatuiMargin {
    /// Horizontal margin.
    #[serde(default)]
    pub horizontal: u16,
    /// Vertical margin.
    #[serde(default)]
    pub vertical: u16,
}

impl From<Margin> for RatatuiMargin {
    fn from(m: Margin) -> Self {
        Self {
            horizontal: m.horizontal,
            vertical: m.vertical,
        }
    }
}

impl From<RatatuiMargin> for Margin {
    fn from(m: RatatuiMargin) -> Self {
        Self::new(m.horizontal, m.vertical)
    }
}

crate::default_style!(RatatuiMargin => RatatuiMarginStyle);

impl Prompt for RatatuiMargin {
    fn prompt() -> Option<&'static str> {
        Some("Configure layout margin:")
    }
}

impl Elicitation for RatatuiMargin {
    type Style = RatatuiMarginStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RatatuiMargin");
        let horizontal = u16::elicit(communicator).await?;
        let vertical = u16::elicit(communicator).await?;
        Ok(Self {
            horizontal,
            vertical,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u16 as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <u16 as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <u16 as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for RatatuiMargin {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::layout::Margin",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "horizontal",
                        type_name: "u16",
                        prompt: Some("Horizontal margin:"),
                    },
                    FieldInfo {
                        name: "vertical",
                        type_name: "u16",
                        prompt: Some("Vertical margin:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for RatatuiMargin {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RatatuiMargin".to_string(),
            fields: vec![
                ("horizontal".to_string(), Box::new(u16::prompt_tree())),
                ("vertical".to_string(), Box::new(u16::prompt_tree())),
            ],
        }
    }
}
