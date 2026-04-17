//! Bevy text type elicitation trenchcoats.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Macro for simple unit-enum bevy text selects ──────────────────────────────

macro_rules! impl_bevy_text_select {
    (
        type       = $ty:ty,
        style      = $style:ident,
        prompt     = $prompt:literal,
        kani_var   = $kani_var:literal,
        variants   = [ $($v:expr),+ $(,)? ]
    ) => {
        impl Prompt for $ty {
            fn prompt() -> Option<&'static str> { Some($prompt) }
        }

        impl Select for $ty {
            fn options() -> Vec<Self> { vec![$($v),+] }

            fn labels() -> Vec<String> {
                Self::options()
                    .iter()
                    .map(|v| serde_json::to_string(v).unwrap().trim_matches('"').to_string())
                    .collect()
            }

            fn from_label(label: &str) -> Option<Self> {
                serde_json::from_str(&format!("\"{}\"", label)).ok()
            }
        }

        crate::default_style!($ty => $style);

        impl Elicitation for $ty {
            type Style = $style;

            #[tracing::instrument(skip(communicator))]
            async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                let params = mcp::select_params(
                    Self::prompt().unwrap_or("Choose a value:"),
                    &Self::labels(),
                );
                let result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                            .with_arguments(params),
                    )
                    .await?;
                let value = mcp::extract_value(result)?;
                let label = mcp::parse_string(value)?;
                Self::from_label(&label).ok_or_else(|| {
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Invalid {}: {label}", stringify!($ty)
                    )))
                })
            }

            fn kani_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::kani_select_wrapper(stringify!($ty), $kani_var)
            }
            fn verus_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::verus_select_wrapper(stringify!($ty), $kani_var)
            }
            fn creusot_proof() -> proc_macro2::TokenStream {
                crate::verification::proof_helpers::creusot_select_wrapper(stringify!($ty), $kani_var)
            }
        }

        impl ElicitIntrospect for $ty {
            fn pattern() -> ElicitationPattern { ElicitationPattern::Select }

            fn metadata() -> TypeMetadata {
                TypeMetadata {
                    type_name: stringify!($ty),
                    description: Self::prompt(),
                    details: PatternDetails::Select {
                        variants: Self::labels()
                            .into_iter()
                            .map(|label| VariantMetadata { label, fields: vec![] })
                            .collect(),
                    },
                }
            }
        }
    };
}

// ── Justify ───────────────────────────────────────────────────────────────────

impl_bevy_text_select! {
    type     = bevy::text::Justify,
    style    = JustifyStyle,
    prompt   = "Text justification (Left/Center/Right/Justified):",
    kani_var = "Left",
    variants = [
        bevy::text::Justify::Left,
        bevy::text::Justify::Center,
        bevy::text::Justify::Right,
        bevy::text::Justify::Justified,
    ]
}

crate::select_trenchcoat!(bevy::text::Justify, as BevyJustify, serde);
crate::select_trenchcoat_traits!(BevyJustify, bevy::text::Justify, [eq]);

// ── LineBreak ─────────────────────────────────────────────────────────────────

impl_bevy_text_select! {
    type     = bevy::text::LineBreak,
    style    = LineBreakStyle,
    prompt   = "Text line-break mode:",
    kani_var = "WordBoundary",
    variants = [
        bevy::text::LineBreak::WordBoundary,
        bevy::text::LineBreak::AnyCharacter,
        bevy::text::LineBreak::WordOrCharacter,
        bevy::text::LineBreak::NoWrap,
    ]
}

crate::select_trenchcoat!(bevy::text::LineBreak, as BevyLineBreak, serde);
crate::select_trenchcoat_traits!(BevyLineBreak, bevy::text::LineBreak, [eq]);

// ── FontSmoothing ─────────────────────────────────────────────────────────────

impl_bevy_text_select! {
    type     = bevy::text::FontSmoothing,
    style    = FontSmoothingStyle,
    prompt   = "Font smoothing mode (None / AntiAliased):",
    kani_var = "AntiAliased",
    variants = [
        bevy::text::FontSmoothing::None,
        bevy::text::FontSmoothing::AntiAliased,
    ]
}

crate::select_trenchcoat!(bevy::text::FontSmoothing, as BevyFontSmoothing, serde);
crate::select_trenchcoat_traits!(BevyFontSmoothing, bevy::text::FontSmoothing, [eq]);

// ── BevyTextLayout ────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for [`bevy::text::TextLayout`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyTextLayout {
    /// Text justification.
    pub justify: BevyJustify,
    /// Line-break mode.
    pub linebreak: BevyLineBreak,
}

crate::default_style!(BevyTextLayout => BevyTextLayoutStyle);

impl From<bevy::text::TextLayout> for BevyTextLayout {
    fn from(t: bevy::text::TextLayout) -> Self {
        Self {
            justify: BevyJustify(t.justify),
            linebreak: BevyLineBreak(t.linebreak),
        }
    }
}

impl From<BevyTextLayout> for bevy::text::TextLayout {
    fn from(t: BevyTextLayout) -> Self {
        bevy::text::TextLayout {
            justify: t.justify.into_inner(),
            linebreak: t.linebreak.into_inner(),
        }
    }
}

impl Prompt for BevyTextLayout {
    fn prompt() -> Option<&'static str> {
        Some("Text layout (justify, line break):")
    }
}

impl Elicitation for BevyTextLayout {
    type Style = BevyTextLayoutStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyTextLayout"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            justify: BevyJustify::elicit(communicator).await?,
            linebreak: BevyLineBreak::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <bevy::text::Justify as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <bevy::text::Justify as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <bevy::text::Justify as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyTextLayout {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyTextLayout",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "justify",
                        type_name: "bevy::text::Justify",
                        prompt: Some("Justification:"),
                    },
                    FieldInfo {
                        name: "linebreak",
                        type_name: "bevy::text::LineBreak",
                        prompt: Some("Line break:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyTextLayout {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyTextLayout".to_string(),
            fields: vec![
                ("justify".to_string(), Box::new(BevyJustify::prompt_tree())),
                (
                    "linebreak".to_string(),
                    Box::new(BevyLineBreak::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyTextLayout {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let j = match self.justify.0 {
            bevy::text::Justify::Left => quote::quote! { bevy::text::Justify::Left },
            bevy::text::Justify::Center => quote::quote! { bevy::text::Justify::Center },
            bevy::text::Justify::Right => quote::quote! { bevy::text::Justify::Right },
            bevy::text::Justify::Justified => quote::quote! { bevy::text::Justify::Justified },
        };
        let lb = match self.linebreak.0 {
            bevy::text::LineBreak::WordBoundary => {
                quote::quote! { bevy::text::LineBreak::WordBoundary }
            }
            bevy::text::LineBreak::AnyCharacter => {
                quote::quote! { bevy::text::LineBreak::AnyCharacter }
            }
            bevy::text::LineBreak::WordOrCharacter => {
                quote::quote! { bevy::text::LineBreak::WordOrCharacter }
            }
            bevy::text::LineBreak::NoWrap => quote::quote! { bevy::text::LineBreak::NoWrap },
        };
        quote::quote! {
            bevy::text::TextLayout { justify: #j, linebreak: #lb }
        }
    }
}

// ── BevyTextFont ──────────────────────────────────────────────────────────────

/// Elicitable trenchcoat for the non-asset fields of [`bevy::text::TextFont`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyTextFont {
    /// Font size in logical pixels.
    pub font_size: f32,
    /// Font smoothing.
    pub font_smoothing: BevyFontSmoothing,
}

crate::default_style!(BevyTextFont => BevyTextFontStyle);

impl Prompt for BevyTextFont {
    fn prompt() -> Option<&'static str> {
        Some("Text font settings (size, smoothing):")
    }
}

impl Elicitation for BevyTextFont {
    type Style = BevyTextFontStyle;

    #[tracing::instrument(skip(communicator), fields(type_name = "BevyTextFont"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            font_size: f32::elicit(communicator).await?,
            font_smoothing: BevyFontSmoothing::elicit(communicator).await?,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyTextFont {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyTextFont",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "font_size",
                        type_name: "f32",
                        prompt: Some("Font size (px):"),
                    },
                    FieldInfo {
                        name: "font_smoothing",
                        type_name: "bevy::text::FontSmoothing",
                        prompt: Some("Smoothing:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyTextFont {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyTextFont".to_string(),
            fields: vec![
                ("font_size".to_string(), Box::new(f32::prompt_tree())),
                (
                    "font_smoothing".to_string(),
                    Box::new(BevyFontSmoothing::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyTextFont {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let sz = self.font_size;
        let sm = match self.font_smoothing.0 {
            bevy::text::FontSmoothing::None => quote::quote! { bevy::text::FontSmoothing::None },
            bevy::text::FontSmoothing::AntiAliased => {
                quote::quote! { bevy::text::FontSmoothing::AntiAliased }
            }
        };
        quote::quote! {
            bevy::text::TextFont { font_size: #sz, font_smoothing: #sm, ..Default::default() }
        }
    }
}
