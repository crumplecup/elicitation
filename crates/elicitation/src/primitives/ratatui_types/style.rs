//! Wrapper for [`ratatui::style::Style`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use ratatui::style::{Color, Modifier, Style};

/// Elicitable representation of [`ratatui::style::Style`].
///
/// Wraps the ratatui `Style` struct, providing field-by-field elicitation
/// for foreground colour, background colour, and text modifier.
///
/// Uses string representations for `Color` and `Modifier` fields because
/// those ratatui types don't implement `JsonSchema`.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct RatatuiStyle {
    /// Foreground colour (e.g. "Red", "#FF00AA", "Reset").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fg: Option<String>,
    /// Background colour (e.g. "Blue", "#00FF00").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg: Option<String>,
    /// Whether to apply bold modifier.
    #[serde(default)]
    pub bold: bool,
    /// Whether to apply italic modifier.
    #[serde(default)]
    pub italic: bool,
    /// Whether to apply underline modifier.
    #[serde(default)]
    pub underlined: bool,
}

impl From<Style> for RatatuiStyle {
    fn from(s: Style) -> Self {
        Self {
            fg: s.fg.map(|c| c.to_string()),
            bg: s.bg.map(|c| c.to_string()),
            bold: s.add_modifier.contains(Modifier::BOLD),
            italic: s.add_modifier.contains(Modifier::ITALIC),
            underlined: s.add_modifier.contains(Modifier::UNDERLINED),
        }
    }
}

impl TryFrom<RatatuiStyle> for Style {
    type Error = String;

    fn try_from(s: RatatuiStyle) -> Result<Self, Self::Error> {
        let mut style = Self::default();
        if let Some(ref fg_str) = s.fg {
            let c: Color = fg_str
                .parse()
                .map_err(|_| format!("invalid foreground color: {fg_str}"))?;
            style = style.fg(c);
        }
        if let Some(ref bg_str) = s.bg {
            let c: Color = bg_str
                .parse()
                .map_err(|_| format!("invalid background color: {bg_str}"))?;
            style = style.bg(c);
        }
        let mut mods = Modifier::empty();
        if s.bold {
            mods |= Modifier::BOLD;
        }
        if s.italic {
            mods |= Modifier::ITALIC;
        }
        if s.underlined {
            mods |= Modifier::UNDERLINED;
        }
        if !mods.is_empty() {
            style = style.add_modifier(mods);
        }
        Ok(style)
    }
}

crate::default_style!(RatatuiStyle => RatatuiStyleStyle);

impl Prompt for RatatuiStyle {
    fn prompt() -> Option<&'static str> {
        Some("Configure a ratatui style:")
    }
}

impl Elicitation for RatatuiStyle {
    type Style = RatatuiStyleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting RatatuiStyle");
        let fg = Option::<String>::elicit(communicator).await?;
        let bg = Option::<String>::elicit(communicator).await?;
        let bold = bool::elicit(communicator).await?;
        let italic = bool::elicit(communicator).await?;
        let underlined = bool::elicit(communicator).await?;
        Ok(Self {
            fg,
            bg,
            bold,
            italic,
            underlined,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as crate::Elicitation>::kani_proof();
        ts.extend(<Option<String> as crate::Elicitation>::kani_proof());
        ts
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as crate::Elicitation>::verus_proof();
        ts.extend(<Option<String> as crate::Elicitation>::verus_proof());
        ts
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        let mut ts = <bool as crate::Elicitation>::creusot_proof();
        ts.extend(<Option<String> as crate::Elicitation>::creusot_proof());
        ts
    }
}

impl ElicitIntrospect for RatatuiStyle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::style::Style",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "fg",
                        type_name: "Option<String>",
                        prompt: Some("Foreground colour (e.g. Red, #FF00AA):"),
                    },
                    FieldInfo {
                        name: "bg",
                        type_name: "Option<String>",
                        prompt: Some("Background colour:"),
                    },
                    FieldInfo {
                        name: "bold",
                        type_name: "bool",
                        prompt: Some("Bold?"),
                    },
                    FieldInfo {
                        name: "italic",
                        type_name: "bool",
                        prompt: Some("Italic?"),
                    },
                    FieldInfo {
                        name: "underlined",
                        type_name: "bool",
                        prompt: Some("Underlined?"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for RatatuiStyle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "RatatuiStyle".to_string(),
            fields: vec![
                ("fg".to_string(), Box::new(<Option<String>>::prompt_tree())),
                ("bg".to_string(), Box::new(<Option<String>>::prompt_tree())),
                ("bold".to_string(), Box::new(bool::prompt_tree())),
                ("italic".to_string(), Box::new(bool::prompt_tree())),
                ("underlined".to_string(), Box::new(bool::prompt_tree())),
            ],
        }
    }
}
