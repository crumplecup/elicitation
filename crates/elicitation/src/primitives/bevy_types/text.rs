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
    prompt   = "Text justification (Left/Center/Right/Justified/Start/End):",
    kani_var = "Left",
    variants = [
        bevy::text::Justify::Left,
        bevy::text::Justify::Center,
        bevy::text::Justify::Right,
        bevy::text::Justify::Justified,
        bevy::text::Justify::Start,
        bevy::text::Justify::End,
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
            bevy::text::Justify::Start => quote::quote! { bevy::text::Justify::Start },
            bevy::text::Justify::End => quote::quote! { bevy::text::Justify::End },
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

// ── BevyFontSize ──────────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::text::FontSize`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "unit", rename_all = "snake_case")]
pub enum BevyFontSize {
    /// Logical pixels.
    Px {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of viewport width.
    Vw {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of viewport height.
    Vh {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the smaller viewport dimension.
    VMin {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the larger viewport dimension.
    VMax {
        /// The numeric value.
        value: f32,
    },
    /// Relative to the `RemSize` resource.
    Rem {
        /// The numeric value.
        value: f32,
    },
}

impl From<bevy::text::FontSize> for BevyFontSize {
    fn from(v: bevy::text::FontSize) -> Self {
        match v {
            bevy::text::FontSize::Px(x) => Self::Px { value: x },
            bevy::text::FontSize::Vw(x) => Self::Vw { value: x },
            bevy::text::FontSize::Vh(x) => Self::Vh { value: x },
            bevy::text::FontSize::VMin(x) => Self::VMin { value: x },
            bevy::text::FontSize::VMax(x) => Self::VMax { value: x },
            bevy::text::FontSize::Rem(x) => Self::Rem { value: x },
        }
    }
}

impl From<BevyFontSize> for bevy::text::FontSize {
    fn from(v: BevyFontSize) -> Self {
        match v {
            BevyFontSize::Px { value } => Self::Px(value),
            BevyFontSize::Vw { value } => Self::Vw(value),
            BevyFontSize::Vh { value } => Self::Vh(value),
            BevyFontSize::VMin { value } => Self::VMin(value),
            BevyFontSize::VMax { value } => Self::VMax(value),
            BevyFontSize::Rem { value } => Self::Rem(value),
        }
    }
}

impl Prompt for BevyFontSize {
    fn prompt() -> Option<&'static str> {
        Some("Font size:")
    }
}

impl Select for BevyFontSize {
    fn options() -> Vec<Self> {
        vec![
            Self::Px { value: 16.0 },
            Self::Rem { value: 1.0 },
            Self::Vw { value: 2.5 },
            Self::Vh { value: 2.5 },
            Self::VMin { value: 2.5 },
            Self::VMax { value: 2.5 },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Px".to_string(),
            "Rem".to_string(),
            "Vw".to_string(),
            "Vh".to_string(),
            "VMin".to_string(),
            "VMax".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Px" => Some(Self::Px { value: 16.0 }),
            "Rem" => Some(Self::Rem { value: 1.0 }),
            "Vw" => Some(Self::Vw { value: 2.5 }),
            "Vh" => Some(Self::Vh { value: 2.5 }),
            "VMin" => Some(Self::VMin { value: 2.5 }),
            "VMax" => Some(Self::VMax { value: 2.5 }),
            _ => None,
        }
    }
}

crate::default_style!(BevyFontSize => BevyFontSizeStyle);

impl Elicitation for BevyFontSize {
    type Style = BevyFontSizeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Font size unit:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        let v = f32::elicit(communicator).await?;
        match label.as_str() {
            "Px" => Ok(Self::Px { value: v }),
            "Rem" => Ok(Self::Rem { value: v }),
            "Vw" => Ok(Self::Vw { value: v }),
            "Vh" => Ok(Self::Vh { value: v }),
            "VMin" => Ok(Self::VMin { value: v }),
            "VMax" => Ok(Self::VMax { value: v }),
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid FontSize: {label}"
            )))),
        }
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

impl ElicitIntrospect for BevyFontSize {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::text::FontSize",
            description: Some("Font size with unit"),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|l| VariantMetadata {
                        label: l,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyFontSize {
    fn prompt_tree() -> crate::PromptTree {
        let f32_leaf = || {
            Some(Box::new(crate::PromptTree::Leaf {
                prompt: "Value:".to_string(),
                type_name: "f32".to_string(),
            }))
        };
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Font size:").to_string(),
            type_name: "bevy::text::FontSize".to_string(),
            options: Self::labels(),
            branches: vec![
                f32_leaf(),
                f32_leaf(),
                f32_leaf(),
                f32_leaf(),
                f32_leaf(),
                f32_leaf(),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyFontSize {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Px { value } => quote::quote! { bevy::text::FontSize::Px(#value) },
            Self::Rem { value } => quote::quote! { bevy::text::FontSize::Rem(#value) },
            Self::Vw { value } => quote::quote! { bevy::text::FontSize::Vw(#value) },
            Self::Vh { value } => quote::quote! { bevy::text::FontSize::Vh(#value) },
            Self::VMin { value } => quote::quote! { bevy::text::FontSize::VMin(#value) },
            Self::VMax { value } => quote::quote! { bevy::text::FontSize::VMax(#value) },
        }
    }
}

// ── BevyFontStyle ─────────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::text::FontStyle`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BevyFontStyle {
    /// Upright.
    Normal,
    /// Italic.
    Italic,
    /// Oblique with optional slant angle in degrees.
    Oblique {
        /// Slant angle in degrees; `None` uses the default angle.
        angle: Option<f32>,
    },
}

impl From<bevy::text::FontStyle> for BevyFontStyle {
    fn from(v: bevy::text::FontStyle) -> Self {
        match v {
            bevy::text::FontStyle::Normal => Self::Normal,
            bevy::text::FontStyle::Italic => Self::Italic,
            bevy::text::FontStyle::Oblique(a) => Self::Oblique { angle: a },
        }
    }
}

impl From<BevyFontStyle> for bevy::text::FontStyle {
    fn from(v: BevyFontStyle) -> Self {
        match v {
            BevyFontStyle::Normal => Self::Normal,
            BevyFontStyle::Italic => Self::Italic,
            BevyFontStyle::Oblique { angle } => Self::Oblique(angle),
        }
    }
}

impl Prompt for BevyFontStyle {
    fn prompt() -> Option<&'static str> {
        Some("Font style:")
    }
}

impl Select for BevyFontStyle {
    fn options() -> Vec<Self> {
        vec![Self::Normal, Self::Italic, Self::Oblique { angle: None }]
    }

    fn labels() -> Vec<String> {
        vec![
            "Normal".to_string(),
            "Italic".to_string(),
            "Oblique".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Normal" => Some(Self::Normal),
            "Italic" => Some(Self::Italic),
            "Oblique" => Some(Self::Oblique { angle: None }),
            _ => None,
        }
    }
}

crate::default_style!(BevyFontStyle => BevyFontStyleStyle);

impl Elicitation for BevyFontStyle {
    type Style = BevyFontStyleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(Self::prompt().unwrap_or("Font style:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match label.as_str() {
            "Normal" => Ok(Self::Normal),
            "Italic" => Ok(Self::Italic),
            "Oblique" => {
                let angle = Option::<f32>::elicit(communicator).await?;
                Ok(Self::Oblique { angle })
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid FontStyle: {label}"
            )))),
        }
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

impl ElicitIntrospect for BevyFontStyle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::text::FontStyle",
            description: Some("Font slant style"),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|l| VariantMetadata {
                        label: l,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyFontStyle {
    fn prompt_tree() -> crate::PromptTree {
        let angle_leaf = Box::new(crate::PromptTree::Leaf {
            prompt: "Angle (degrees, optional):".to_string(),
            type_name: "Option<f32>".to_string(),
        });
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Font style:").to_string(),
            type_name: "bevy::text::FontStyle".to_string(),
            options: Self::labels(),
            branches: vec![None, None, Some(angle_leaf)],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyFontStyle {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Normal => quote::quote! { bevy::text::FontStyle::Normal },
            Self::Italic => quote::quote! { bevy::text::FontStyle::Italic },
            Self::Oblique { angle: Some(a) } => {
                quote::quote! { bevy::text::FontStyle::Oblique(Some(#a)) }
            }
            Self::Oblique { angle: None } => quote::quote! { bevy::text::FontStyle::Oblique(None) },
        }
    }
}

// ── BevyFontWeight ────────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::text::FontWeight`].
///
/// Font weight as a value from 1 to 1000. Common values: 400 (Normal), 700 (Bold).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BevyFontWeight(pub u16);

impl From<bevy::text::FontWeight> for BevyFontWeight {
    fn from(v: bevy::text::FontWeight) -> Self {
        Self(v.0)
    }
}
impl From<BevyFontWeight> for bevy::text::FontWeight {
    fn from(v: BevyFontWeight) -> Self {
        Self(v.0)
    }
}

crate::default_style!(BevyFontWeight => BevyFontWeightStyle);

impl Prompt for BevyFontWeight {
    fn prompt() -> Option<&'static str> {
        Some("Font weight (100-1000, 400=Normal, 700=Bold):")
    }
}

impl Elicitation for BevyFontWeight {
    type Style = BevyFontWeightStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self(u16::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <u16 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyFontWeight {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::text::FontWeight",
            description: Some("Font stroke thickness (1-1000)"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "u16",
                    prompt: Some("Weight (e.g. 400=Normal, 700=Bold):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyFontWeight {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::text::FontWeight".to_string(),
            fields: vec![("0".to_string(), Box::new(u16::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyFontWeight {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v = self.0;
        quote::quote! { bevy::text::FontWeight(#v) }
    }
}

// ── BevyFontWidth ─────────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::text::FontWidth`].
///
/// Horizontal condensing/expanding ratio. 1.0 = normal, 0.75 = condensed, 1.25 = expanded.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyFontWidth(pub f32);

impl From<bevy::text::FontWidth> for BevyFontWidth {
    fn from(v: bevy::text::FontWidth) -> Self {
        Self(v.0)
    }
}
impl From<BevyFontWidth> for bevy::text::FontWidth {
    fn from(v: BevyFontWidth) -> Self {
        Self(v.0)
    }
}

crate::default_style!(BevyFontWidth => BevyFontWidthStyle);

impl Prompt for BevyFontWidth {
    fn prompt() -> Option<&'static str> {
        Some("Font width ratio (1.0 = normal):")
    }
}

impl Elicitation for BevyFontWidth {
    type Style = BevyFontWidthStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self(f32::elicit(communicator).await?))
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

impl ElicitIntrospect for BevyFontWidth {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::text::FontWidth",
            description: Some("Horizontal condensing/expanding ratio"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "f32",
                    prompt: Some("Width ratio:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyFontWidth {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::text::FontWidth".to_string(),
            fields: vec![("0".to_string(), Box::new(f32::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyFontWidth {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v = self.0;
        quote::quote! { bevy::text::FontWidth(#v) }
    }
}

// ── BevyUiText ────────────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::widget::Text`].
///
/// The string content of a UI text widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiText(pub String);

impl From<bevy::ui::widget::Text> for BevyUiText {
    fn from(t: bevy::ui::widget::Text) -> Self {
        Self(t.0)
    }
}
impl From<BevyUiText> for bevy::ui::widget::Text {
    fn from(t: BevyUiText) -> Self {
        Self(t.0)
    }
}

crate::default_style!(BevyUiText => BevyUiTextStyle);

impl Prompt for BevyUiText {
    fn prompt() -> Option<&'static str> {
        Some("Text content:")
    }
}

impl Elicitation for BevyUiText {
    type Style = BevyUiTextStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self(String::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <String as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyUiText {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::widget::Text",
            description: Some("Text string for a UI widget"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "String",
                    prompt: Some("Text:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyUiText {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::ui::widget::Text".to_string(),
            fields: vec![("0".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyUiText {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let s = &self.0;
        quote::quote! { bevy::ui::widget::Text(#s.to_string()) }
    }
}
