//! Bevy color type elicitation.
//!
//! Covers `bevy::color::Color` (outer enum) and the ten inner color-space
//! struct variants: `Srgba`, `LinearRgba`, `Hsla`, `Hsva`, `Hwba`, `Laba`,
//! `Lcha`, `Oklaba`, `Oklcha`, and `Xyza`.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Shared macro for 4-field f32 color-space structs ─────────────────────────

/// Generates a color-space struct trenchcoat with fields `f1`, `f2`, `f3`, `alpha: f32`.
macro_rules! bevy_color_struct {
    (
        $name:ident,
        $upstream:path,
        $type_name:literal,
        $code_path:literal,
        $f1:ident, $f1_prompt:literal,
        $f2:ident, $f2_prompt:literal,
        $f3:ident, $f3_prompt:literal,
        $description:literal
    ) => {
        paste::paste! {
            #[doc = $description]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
            pub struct $name {
                /// First component.
                pub $f1: f32,
                /// Second component.
                pub $f2: f32,
                /// Third component.
                pub $f3: f32,
                /// Alpha channel [0, 1].
                pub alpha: f32,
            }

            crate::default_style!($name => [<$name Style>]);

            impl $name {
                /// Constructs a new instance.
                pub fn new($f1: f32, $f2: f32, $f3: f32, alpha: f32) -> Self {
                    Self { $f1, $f2, $f3, alpha }
                }

                /// Converts this wrapper into the upstream type.
                pub fn into_inner(self) -> $upstream { self.into() }
            }

            impl From<$upstream> for $name {
                fn from(c: $upstream) -> Self {
                    Self { $f1: c.$f1, $f2: c.$f2, $f3: c.$f3, alpha: c.alpha }
                }
            }

            impl From<$name> for $upstream {
                fn from(c: $name) -> Self {
                    Self { $f1: c.$f1, $f2: c.$f2, $f3: c.$f3, alpha: c.alpha }
                }
            }

            impl Prompt for $name {
                fn prompt() -> Option<&'static str> {
                    Some($description)
                }
            }

            impl Elicitation for $name {
                type Style = [<$name Style>];

                #[tracing::instrument(skip(communicator), fields(type_name = $type_name))]
                async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
                    Ok(Self {
                        $f1: f32::elicit(communicator).await?,
                        $f2: f32::elicit(communicator).await?,
                        $f3: f32::elicit(communicator).await?,
                        alpha: f32::elicit(communicator).await?,
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

            impl ElicitIntrospect for $name {
                fn pattern() -> ElicitationPattern { ElicitationPattern::Survey }
                fn metadata() -> TypeMetadata {
                    TypeMetadata {
                        type_name: $type_name,
                        description: Self::prompt(),
                        details: PatternDetails::Survey {
                            fields: vec![
                                FieldInfo { name: stringify!($f1), type_name: "f32", prompt: Some($f1_prompt) },
                                FieldInfo { name: stringify!($f2), type_name: "f32", prompt: Some($f2_prompt) },
                                FieldInfo { name: stringify!($f3), type_name: "f32", prompt: Some($f3_prompt) },
                                FieldInfo { name: "alpha", type_name: "f32", prompt: Some("Alpha [0,1]:") },
                            ],
                        },
                    }
                }
            }

            impl crate::ElicitPromptTree for $name {
                fn prompt_tree() -> crate::PromptTree {
                    crate::PromptTree::Survey {
                        prompt: Self::prompt().map(str::to_string),
                        type_name: $type_name.to_string(),
                        fields: vec![
                            (stringify!($f1).to_string(), Box::new(f32::prompt_tree())),
                            (stringify!($f2).to_string(), Box::new(f32::prompt_tree())),
                            (stringify!($f3).to_string(), Box::new(f32::prompt_tree())),
                            ("alpha".to_string(), Box::new(f32::prompt_tree())),
                        ],
                    }
                }
            }

            impl crate::emit_code::ToCodeLiteral for $name {
                fn to_code_literal(&self) -> proc_macro2::TokenStream {
                    let f1 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.$f1);
                    let f2 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.$f2);
                    let f3 = crate::emit_code::ToCodeLiteral::to_code_literal(&self.$f3);
                    let a  = crate::emit_code::ToCodeLiteral::to_code_literal(&self.alpha);
                    let path: proc_macro2::TokenStream = $code_path.parse().unwrap();
                    quote::quote! { #path { $f1: #f1, $f2: #f2, $f3: #f3, alpha: #a } }
                }
            }
        }
    };
}

// ── Ten color-space struct trenchcoats ───────────────────────────────────────

bevy_color_struct!(
    BevySrgba,
    bevy::color::Srgba,
    "bevy::color::Srgba",
    "bevy::color::Srgba",
    red,
    "Red channel [0,1]:",
    green,
    "Green channel [0,1]:",
    blue,
    "Blue channel [0,1]:",
    "sRGB color with alpha."
);

bevy_color_struct!(
    BevyLinearRgba,
    bevy::color::LinearRgba,
    "bevy::color::LinearRgba",
    "bevy::color::LinearRgba",
    red,
    "Red channel [0,1]:",
    green,
    "Green channel [0,1]:",
    blue,
    "Blue channel [0,1]:",
    "Linear RGB color with alpha."
);

bevy_color_struct!(
    BevyHsla,
    bevy::color::Hsla,
    "bevy::color::Hsla",
    "bevy::color::Hsla",
    hue,
    "Hue [0,360):",
    saturation,
    "Saturation [0,1]:",
    lightness,
    "Lightness [0,1]:",
    "HSL color with alpha."
);

bevy_color_struct!(
    BevyHsva,
    bevy::color::Hsva,
    "bevy::color::Hsva",
    "bevy::color::Hsva",
    hue,
    "Hue [0,360):",
    saturation,
    "Saturation [0,1]:",
    value,
    "Value [0,1]:",
    "HSV color with alpha."
);

bevy_color_struct!(
    BevyHwba,
    bevy::color::Hwba,
    "bevy::color::Hwba",
    "bevy::color::Hwba",
    hue,
    "Hue [0,360):",
    whiteness,
    "Whiteness [0,1]:",
    blackness,
    "Blackness [0,1]:",
    "HWB color with alpha."
);

bevy_color_struct!(
    BevyLaba,
    bevy::color::Laba,
    "bevy::color::Laba",
    "bevy::color::Laba",
    lightness,
    "Lightness [0,1]:",
    a,
    "Green-red axis a:",
    b,
    "Blue-yellow axis b:",
    "CIE L*a*b* color with alpha."
);

bevy_color_struct!(
    BevyLcha,
    bevy::color::Lcha,
    "bevy::color::Lcha",
    "bevy::color::Lcha",
    lightness,
    "Lightness [0,1]:",
    chroma,
    "Chroma ≥ 0:",
    hue,
    "Hue [0,360):",
    "CIE L*C*h* color with alpha."
);

bevy_color_struct!(
    BevyOklaba,
    bevy::color::Oklaba,
    "bevy::color::Oklaba",
    "bevy::color::Oklaba",
    lightness,
    "Lightness [0,1]:",
    a,
    "Green-red axis a:",
    b,
    "Blue-yellow axis b:",
    "Oklab perceptual color with alpha."
);

bevy_color_struct!(
    BevyOklcha,
    bevy::color::Oklcha,
    "bevy::color::Oklcha",
    "bevy::color::Oklcha",
    lightness,
    "Lightness [0,1]:",
    chroma,
    "Chroma ≥ 0:",
    hue,
    "Hue [0,360):",
    "Oklch perceptual polar color with alpha."
);

bevy_color_struct!(
    BevyXyza,
    bevy::color::Xyza,
    "bevy::color::Xyza",
    "bevy::color::Xyza",
    x,
    "X component:",
    y,
    "Y (luminance):",
    z,
    "Z component:",
    "CIE XYZ color with alpha."
);

// ── BevyColor outer enum ──────────────────────────────────────────────────────

/// Internal variant-selection enum for [`BevyColor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BevyColorKind {
    Srgba,
    LinearRgba,
    Hsla,
    Hsva,
    Hwba,
    Laba,
    Lcha,
    Oklaba,
    Oklcha,
    Xyza,
}

impl Prompt for BevyColorKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose a color space:")
    }
}

impl Select for BevyColorKind {
    fn options() -> Vec<Self> {
        vec![
            Self::Srgba,
            Self::LinearRgba,
            Self::Hsla,
            Self::Hsva,
            Self::Hwba,
            Self::Laba,
            Self::Lcha,
            Self::Oklaba,
            Self::Oklcha,
            Self::Xyza,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Srgba".to_string(),
            "LinearRgba".to_string(),
            "Hsla".to_string(),
            "Hsva".to_string(),
            "Hwba".to_string(),
            "Laba".to_string(),
            "Lcha".to_string(),
            "Oklaba".to_string(),
            "Oklcha".to_string(),
            "Xyza".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Srgba" => Some(Self::Srgba),
            "LinearRgba" => Some(Self::LinearRgba),
            "Hsla" => Some(Self::Hsla),
            "Hsva" => Some(Self::Hsva),
            "Hwba" => Some(Self::Hwba),
            "Laba" => Some(Self::Laba),
            "Lcha" => Some(Self::Lcha),
            "Oklaba" => Some(Self::Oklaba),
            "Oklcha" => Some(Self::Oklcha),
            "Xyza" => Some(Self::Xyza),
            _ => None,
        }
    }
}

/// Owned trenchcoat for `bevy::color::Color`.
///
/// Wraps all ten Bevy color-space variants so that the outer enum can satisfy
/// [`Elicitation`] and [`schemars::JsonSchema`] under orphan rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "space")]
pub enum BevyColor {
    /// sRGB with alpha.
    Srgba(BevySrgba),
    /// Linear RGB with alpha.
    LinearRgba(BevyLinearRgba),
    /// HSL with alpha.
    Hsla(BevyHsla),
    /// HSV with alpha.
    Hsva(BevyHsva),
    /// HWB with alpha.
    Hwba(BevyHwba),
    /// CIE L*a*b* with alpha.
    Laba(BevyLaba),
    /// CIE L*C*h* with alpha.
    Lcha(BevyLcha),
    /// Oklab with alpha.
    Oklaba(BevyOklaba),
    /// Oklch with alpha.
    Oklcha(BevyOklcha),
    /// CIE XYZ with alpha.
    Xyza(BevyXyza),
}

impl From<bevy::color::Color> for BevyColor {
    fn from(c: bevy::color::Color) -> Self {
        match c {
            bevy::color::Color::Srgba(v) => Self::Srgba(v.into()),
            bevy::color::Color::LinearRgba(v) => Self::LinearRgba(v.into()),
            bevy::color::Color::Hsla(v) => Self::Hsla(v.into()),
            bevy::color::Color::Hsva(v) => Self::Hsva(v.into()),
            bevy::color::Color::Hwba(v) => Self::Hwba(v.into()),
            bevy::color::Color::Laba(v) => Self::Laba(v.into()),
            bevy::color::Color::Lcha(v) => Self::Lcha(v.into()),
            bevy::color::Color::Oklaba(v) => Self::Oklaba(v.into()),
            bevy::color::Color::Oklcha(v) => Self::Oklcha(v.into()),
            bevy::color::Color::Xyza(v) => Self::Xyza(v.into()),
        }
    }
}

impl From<BevyColor> for bevy::color::Color {
    fn from(c: BevyColor) -> Self {
        match c {
            BevyColor::Srgba(v) => Self::Srgba(v.into()),
            BevyColor::LinearRgba(v) => Self::LinearRgba(v.into()),
            BevyColor::Hsla(v) => Self::Hsla(v.into()),
            BevyColor::Hsva(v) => Self::Hsva(v.into()),
            BevyColor::Hwba(v) => Self::Hwba(v.into()),
            BevyColor::Laba(v) => Self::Laba(v.into()),
            BevyColor::Lcha(v) => Self::Lcha(v.into()),
            BevyColor::Oklaba(v) => Self::Oklaba(v.into()),
            BevyColor::Oklcha(v) => Self::Oklcha(v.into()),
            BevyColor::Xyza(v) => Self::Xyza(v.into()),
        }
    }
}

crate::default_style!(BevyColor => BevyColorStyle);

impl Prompt for BevyColor {
    fn prompt() -> Option<&'static str> {
        Some("Choose a Bevy color:")
    }
}

impl Elicitation for BevyColor {
    type Style = BevyColorStyle;

    #[tracing::instrument(skip(communicator))]
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send {
        let comm = communicator.clone();
        Box::pin(async move {
            let communicator = &comm;
            let result = communicator
                .call_tool(
                    rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                        .with_arguments(mcp::select_params(
                            BevyColorKind::prompt().unwrap_or("Choose a color space:"),
                            &BevyColorKind::labels(),
                        )),
                )
                .await?;
            let value = mcp::extract_value(result)?;
            let label = mcp::parse_string(value)?;
            match BevyColorKind::from_label(&label) {
                Some(BevyColorKind::Srgba) => {
                    Ok(Self::Srgba(BevySrgba::elicit(communicator).await?))
                }
                Some(BevyColorKind::LinearRgba) => Ok(Self::LinearRgba(
                    BevyLinearRgba::elicit(communicator).await?,
                )),
                Some(BevyColorKind::Hsla) => Ok(Self::Hsla(BevyHsla::elicit(communicator).await?)),
                Some(BevyColorKind::Hsva) => Ok(Self::Hsva(BevyHsva::elicit(communicator).await?)),
                Some(BevyColorKind::Hwba) => Ok(Self::Hwba(BevyHwba::elicit(communicator).await?)),
                Some(BevyColorKind::Laba) => Ok(Self::Laba(BevyLaba::elicit(communicator).await?)),
                Some(BevyColorKind::Lcha) => Ok(Self::Lcha(BevyLcha::elicit(communicator).await?)),
                Some(BevyColorKind::Oklaba) => {
                    Ok(Self::Oklaba(BevyOklaba::elicit(communicator).await?))
                }
                Some(BevyColorKind::Oklcha) => {
                    Ok(Self::Oklcha(BevyOklcha::elicit(communicator).await?))
                }
                Some(BevyColorKind::Xyza) => Ok(Self::Xyza(BevyXyza::elicit(communicator).await?)),
                None => Err(crate::ElicitError::new(
                    crate::ElicitErrorKind::InvalidSelection(label),
                )),
            }
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevySrgba as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevySrgba as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevySrgba as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyColor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::color::Color",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: vec![
                    VariantMetadata {
                        label: "Srgba".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "LinearRgba".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Hsla".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Hsva".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Hwba".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Laba".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Lcha".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Oklaba".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Oklcha".to_string(),
                        fields: vec![],
                    },
                    VariantMetadata {
                        label: "Xyza".to_string(),
                        fields: vec![],
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyColor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose a Bevy color:").to_string(),
            type_name: "bevy::color::Color".to_string(),
            options: BevyColorKind::labels(),
            branches: vec![
                Some(Box::new(BevySrgba::prompt_tree())),
                Some(Box::new(BevyLinearRgba::prompt_tree())),
                Some(Box::new(BevyHsla::prompt_tree())),
                Some(Box::new(BevyHsva::prompt_tree())),
                Some(Box::new(BevyHwba::prompt_tree())),
                Some(Box::new(BevyLaba::prompt_tree())),
                Some(Box::new(BevyLcha::prompt_tree())),
                Some(Box::new(BevyOklaba::prompt_tree())),
                Some(Box::new(BevyOklcha::prompt_tree())),
                Some(Box::new(BevyXyza::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyColor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            BevyColor::Srgba(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Srgba(#inner) }
            }
            BevyColor::LinearRgba(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::LinearRgba(#inner) }
            }
            BevyColor::Hsla(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Hsla(#inner) }
            }
            BevyColor::Hsva(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Hsva(#inner) }
            }
            BevyColor::Hwba(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Hwba(#inner) }
            }
            BevyColor::Laba(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Laba(#inner) }
            }
            BevyColor::Lcha(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Lcha(#inner) }
            }
            BevyColor::Oklaba(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Oklaba(#inner) }
            }
            BevyColor::Oklcha(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Oklcha(#inner) }
            }
            BevyColor::Xyza(v) => {
                let inner = crate::emit_code::ToCodeLiteral::to_code_literal(v);
                quote::quote! { bevy::color::Color::Xyza(#inner) }
            }
        }
    }
}
