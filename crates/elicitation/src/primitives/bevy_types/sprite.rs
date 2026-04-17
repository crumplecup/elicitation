//! Bevy sprite type elicitation trenchcoats.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata,
    mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// SpriteScalingMode
// ---------------------------------------------------------------------------

impl Prompt for bevy::sprite::SpriteScalingMode {
    fn prompt() -> Option<&'static str> {
        Some("Sprite scaling mode:")
    }
}

impl Select for bevy::sprite::SpriteScalingMode {
    fn options() -> Vec<Self> {
        vec![
            Self::FillCenter,
            Self::FillStart,
            Self::FillEnd,
            Self::FitCenter,
            Self::FitStart,
            Self::FitEnd,
        ]
    }
    fn labels() -> Vec<String> {
        vec![
            "FillCenter".into(),
            "FillStart".into(),
            "FillEnd".into(),
            "FitCenter".into(),
            "FitStart".into(),
            "FitEnd".into(),
        ]
    }
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "FillCenter" => Some(Self::FillCenter),
            "FillStart" => Some(Self::FillStart),
            "FillEnd" => Some(Self::FillEnd),
            "FitCenter" => Some(Self::FitCenter),
            "FitStart" => Some(Self::FitStart),
            "FitEnd" => Some(Self::FitEnd),
            _ => None,
        }
    }
}

crate::default_style!(bevy::sprite::SpriteScalingMode => SpriteScalingModeStyle);

impl Elicitation for bevy::sprite::SpriteScalingMode {
    type Style = SpriteScalingModeStyle;
    #[tracing::instrument(
        skip(communicator),
        fields(type_name = "bevy::sprite::SpriteScalingMode")
    )]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(Self::prompt().unwrap_or("Choose:"), &Self::labels());
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
                "Invalid SpriteScalingMode: {label}"
            )))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "bevy::sprite::SpriteScalingMode",
            "FillCenter",
        )
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "bevy::sprite::SpriteScalingMode",
            "FillCenter",
        )
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "bevy::sprite::SpriteScalingMode",
            "FillCenter",
        )
    }
}

impl ElicitIntrospect for bevy::sprite::SpriteScalingMode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::sprite::SpriteScalingMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

crate::select_trenchcoat!(bevy::sprite::SpriteScalingMode, as BevySpriteScalingMode);
crate::select_trenchcoat_traits!(BevySpriteScalingMode, bevy::sprite::SpriteScalingMode, [eq]);

// ---------------------------------------------------------------------------
// BevyAnchor
// ---------------------------------------------------------------------------

/// Elicitable trenchcoat for [`bevy::sprite::Anchor`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyAnchor {
    /// Horizontal anchor (-0.5 = left, 0.0 = centre, 0.5 = right).
    pub x: f32,
    /// Vertical anchor (-0.5 = bottom, 0.0 = centre, 0.5 = top).
    pub y: f32,
}

crate::default_style!(BevyAnchor => BevyAnchorStyle);

impl From<bevy::sprite::Anchor> for BevyAnchor {
    fn from(a: bevy::sprite::Anchor) -> Self {
        let v = a.0;
        Self { x: v.x, y: v.y }
    }
}
impl From<BevyAnchor> for bevy::sprite::Anchor {
    fn from(a: BevyAnchor) -> Self {
        bevy::sprite::Anchor(bevy::math::Vec2::new(a.x, a.y))
    }
}

impl Prompt for BevyAnchor {
    fn prompt() -> Option<&'static str> {
        Some("Sprite anchor point (x, y in -0.5..0.5):")
    }
}

impl Elicitation for BevyAnchor {
    type Style = BevyAnchorStyle;
    #[tracing::instrument(skip(communicator), fields(type_name = "BevyAnchor"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            x: f32::elicit(communicator).await?,
            y: f32::elicit(communicator).await?,
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

impl ElicitIntrospect for BevyAnchor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyAnchor",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyAnchor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevyAnchor".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyAnchor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! { bevy::sprite::Anchor(bevy::math::Vec2::new(#x, #y)) }
    }
}

// ---------------------------------------------------------------------------
// BevySpriteConfig
// ---------------------------------------------------------------------------

/// Elicitable trenchcoat for the non-asset fields of [`bevy::sprite::Sprite`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevySpriteConfig {
    /// Red channel (0-1).
    pub color_r: f32,
    /// Green channel (0-1).
    pub color_g: f32,
    /// Blue channel (0-1).
    pub color_b: f32,
    /// Alpha channel (0-1).
    pub color_a: f32,
    /// Flip horizontally.
    pub flip_x: bool,
    /// Flip vertically.
    pub flip_y: bool,
    /// Optional custom width.
    pub custom_width: Option<f32>,
    /// Optional custom height.
    pub custom_height: Option<f32>,
}

crate::default_style!(BevySpriteConfig => BevySpriteConfigStyle);

impl Prompt for BevySpriteConfig {
    fn prompt() -> Option<&'static str> {
        Some("Sprite configuration (colour, flip, size):")
    }
}

impl Elicitation for BevySpriteConfig {
    type Style = BevySpriteConfigStyle;
    #[tracing::instrument(skip(communicator), fields(type_name = "BevySpriteConfig"))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self {
            color_r: f32::elicit(communicator).await?,
            color_g: f32::elicit(communicator).await?,
            color_b: f32::elicit(communicator).await?,
            color_a: f32::elicit(communicator).await?,
            flip_x: bool::elicit(communicator).await?,
            flip_y: bool::elicit(communicator).await?,
            custom_width: <Option<f32>>::elicit(communicator).await?,
            custom_height: <Option<f32>>::elicit(communicator).await?,
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

impl ElicitIntrospect for BevySpriteConfig {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevySpriteConfig",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "color_r",
                        type_name: "f32",
                        prompt: Some("Red:"),
                    },
                    FieldInfo {
                        name: "color_g",
                        type_name: "f32",
                        prompt: Some("Green:"),
                    },
                    FieldInfo {
                        name: "color_b",
                        type_name: "f32",
                        prompt: Some("Blue:"),
                    },
                    FieldInfo {
                        name: "color_a",
                        type_name: "f32",
                        prompt: Some("Alpha:"),
                    },
                    FieldInfo {
                        name: "flip_x",
                        type_name: "bool",
                        prompt: Some("Flip X?"),
                    },
                    FieldInfo {
                        name: "flip_y",
                        type_name: "bool",
                        prompt: Some("Flip Y?"),
                    },
                    FieldInfo {
                        name: "custom_width",
                        type_name: "Option<f32>",
                        prompt: Some("Width:"),
                    },
                    FieldInfo {
                        name: "custom_height",
                        type_name: "Option<f32>",
                        prompt: Some("Height:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevySpriteConfig {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "BevySpriteConfig".to_string(),
            fields: vec![
                ("color_r".to_string(), Box::new(f32::prompt_tree())),
                ("color_g".to_string(), Box::new(f32::prompt_tree())),
                ("color_b".to_string(), Box::new(f32::prompt_tree())),
                ("color_a".to_string(), Box::new(f32::prompt_tree())),
                ("flip_x".to_string(), Box::new(bool::prompt_tree())),
                ("flip_y".to_string(), Box::new(bool::prompt_tree())),
                (
                    "custom_width".to_string(),
                    Box::new(<Option<f32>>::prompt_tree()),
                ),
                (
                    "custom_height".to_string(),
                    Box::new(<Option<f32>>::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevySpriteConfig {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let r = self.color_r;
        let g = self.color_g;
        let b = self.color_b;
        let a = self.color_a;
        let fx = self.flip_x;
        let fy = self.flip_y;
        let cw = match self.custom_width {
            Some(w) => quote::quote! { Some(#w) },
            None => quote::quote! { None },
        };
        let ch = match self.custom_height {
            Some(h) => quote::quote! { Some(#h) },
            None => quote::quote! { None },
        };
        quote::quote! {
            bevy::sprite::Sprite {
                color: bevy::color::Color::srgba(#r, #g, #b, #a),
                flip_x: #fx,
                flip_y: #fy,
                custom_size: if #cw.is_some() && #ch.is_some() {
                    Some(bevy::math::Vec2::new(#cw.unwrap_or(0.0), #ch.unwrap_or(0.0)))
                } else {
                    None
                },
                ..Default::default()
            }
        }
    }
}
