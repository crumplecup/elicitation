//! Wrapper for [`palette::Srgb<f32>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};
use palette::Srgb;

/// Elicitable representation of [`palette::Srgb<f32>`].
///
/// An RGB color in the sRGB color space with three floating-point channels
/// ranging from 0.0 (dark) to 1.0 (bright).
#[derive(
    Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct PaletteSrgb {
    /// Red channel (0.0–1.0).
    #[serde(default)]
    pub r: f32,
    /// Green channel (0.0–1.0).
    #[serde(default)]
    pub g: f32,
    /// Blue channel (0.0–1.0).
    #[serde(default)]
    pub b: f32,
}

impl From<Srgb<f32>> for PaletteSrgb {
    fn from(c: Srgb<f32>) -> Self {
        Self {
            r: c.red,
            g: c.green,
            b: c.blue,
        }
    }
}

impl From<PaletteSrgb> for Srgb<f32> {
    fn from(c: PaletteSrgb) -> Self {
        Srgb::new(c.r, c.g, c.b)
    }
}

crate::default_style!(PaletteSrgb => PaletteSrgbStyle);

impl Prompt for PaletteSrgb {
    fn prompt() -> Option<&'static str> {
        Some("Specify an sRGB color (r, g, b channels from 0.0–1.0):")
    }
}

impl Elicitation for PaletteSrgb {
    type Style = PaletteSrgbStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PaletteSrgb");
        let r = f32::elicit(communicator).await?;
        let g = f32::elicit(communicator).await?;
        let b = f32::elicit(communicator).await?;
        Ok(Self { r, g, b })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <f32 as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        <f32 as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        <f32 as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for PaletteSrgb {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "palette::Srgb<f32>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "r",
                        type_name: "f32",
                        prompt: Some("Red channel (0.0–1.0):"),
                    },
                    FieldInfo {
                        name: "g",
                        type_name: "f32",
                        prompt: Some("Green channel (0.0–1.0):"),
                    },
                    FieldInfo {
                        name: "b",
                        type_name: "f32",
                        prompt: Some("Blue channel (0.0–1.0):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for PaletteSrgb {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "PaletteSrgb".to_string(),
            fields: vec![
                ("r".to_string(), Box::new(f32::prompt_tree())),
                ("g".to_string(), Box::new(f32::prompt_tree())),
                ("b".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for PaletteSrgb {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        quote::quote! {
            elicitation::PaletteSrgb { r: #r, g: #g, b: #b }
        }
    }
}
