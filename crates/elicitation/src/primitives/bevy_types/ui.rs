//! Bevy UI type elicitation trenchcoats.
//!
//! Covers:
//! - [`BevyVal`] — owned trenchcoat for `bevy::ui::Val`
//! - [`BevyAlignItems`], [`BevyJustifyItems`], [`BevyAlignSelf`], [`BevyJustifySelf`],
//!   [`BevyAlignContent`], [`BevyJustifyContent`], [`BevyDisplay`], [`BevyBoxSizing`],
//!   [`BevyFlexDirection`], [`BevyFlexWrap`], [`BevyPositionType`], [`BevyOverflowAxis`],
//!   [`BevyVisualBox`] — select-trenchcoats for layout enums
//! - [`BevyOverflowClipMargin`] — survey trenchcoat for `bevy::ui::OverflowClipMargin`
//! - [`BevyOverflow`] — survey trenchcoat for `bevy::ui::Overflow`
//! - [`BevyInlineDirection`] — select-trenchcoat for `bevy::ui::InlineDirection`
//! - [`BevyGridAutoFlow`] — select-trenchcoat for `bevy::ui::GridAutoFlow`
//! - [`BevyUiRect`] — survey trenchcoat for `bevy::ui::UiRect`
//! - [`BevyBorderRadius`] — survey trenchcoat for `bevy::ui::BorderRadius`

use crate::{
    BevyColor, ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult,
    Elicitation, ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, Survey,
    TypeMetadata, VariantMetadata, mcp,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Shared helper macro for unit-enum UI layout selects ──────────────────────

macro_rules! impl_ui_select {
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
                    description: None,
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

// ── AlignItems ────────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::AlignItems,
    style    = BevyAlignItemsStyle,
    prompt   = "Choose cross-axis item alignment:",
    kani_var = "bevy::ui::AlignItems::Center",
    variants = [
        Self::Default, Self::Start, Self::End, Self::FlexStart, Self::FlexEnd,
        Self::Center, Self::Baseline, Self::Stretch,
    ]
);

crate::select_trenchcoat!(bevy::ui::AlignItems, as BevyAlignItems, serde);
crate::select_trenchcoat_traits!(BevyAlignItems, bevy::ui::AlignItems, [copy, eq]);

// ── JustifyItems ──────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::JustifyItems,
    style    = BevyJustifyItemsStyle,
    prompt   = "Choose inline-axis item justification:",
    kani_var = "bevy::ui::JustifyItems::Center",
    variants = [
        Self::Default, Self::Start, Self::End, Self::Center, Self::Baseline, Self::Stretch,
    ]
);

crate::select_trenchcoat!(bevy::ui::JustifyItems, as BevyJustifyItems, serde);
crate::select_trenchcoat_traits!(BevyJustifyItems, bevy::ui::JustifyItems, [copy, eq]);

// ── AlignSelf ─────────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::AlignSelf,
    style    = BevyAlignSelfStyle,
    prompt   = "Choose per-item cross-axis alignment:",
    kani_var = "bevy::ui::AlignSelf::Center",
    variants = [
        Self::Auto, Self::Start, Self::End, Self::FlexStart, Self::FlexEnd,
        Self::Center, Self::Baseline, Self::Stretch,
    ]
);

crate::select_trenchcoat!(bevy::ui::AlignSelf, as BevyAlignSelf, serde);
crate::select_trenchcoat_traits!(BevyAlignSelf, bevy::ui::AlignSelf, [copy, eq]);

// ── JustifySelf ───────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::JustifySelf,
    style    = BevyJustifySelfStyle,
    prompt   = "Choose per-item inline-axis justification:",
    kani_var = "bevy::ui::JustifySelf::Center",
    variants = [
        Self::Auto, Self::Start, Self::End, Self::Center, Self::Baseline, Self::Stretch,
    ]
);

crate::select_trenchcoat!(bevy::ui::JustifySelf, as BevyJustifySelf, serde);
crate::select_trenchcoat_traits!(BevyJustifySelf, bevy::ui::JustifySelf, [copy, eq]);

// ── AlignContent ──────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::AlignContent,
    style    = BevyAlignContentStyle,
    prompt   = "Choose multi-line cross-axis content alignment:",
    kani_var = "bevy::ui::AlignContent::Center",
    variants = [
        Self::Default, Self::Start, Self::End, Self::FlexStart, Self::FlexEnd,
        Self::Center, Self::Stretch, Self::SpaceBetween, Self::SpaceEvenly, Self::SpaceAround,
    ]
);

crate::select_trenchcoat!(bevy::ui::AlignContent, as BevyAlignContent, serde);
crate::select_trenchcoat_traits!(BevyAlignContent, bevy::ui::AlignContent, [copy, eq]);

// ── JustifyContent ────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::JustifyContent,
    style    = BevyJustifyContentStyle,
    prompt   = "Choose main-axis content justification:",
    kani_var = "bevy::ui::JustifyContent::Center",
    variants = [
        Self::Default, Self::Start, Self::End, Self::FlexStart, Self::FlexEnd,
        Self::Center, Self::Stretch, Self::SpaceBetween, Self::SpaceEvenly, Self::SpaceAround,
    ]
);

crate::select_trenchcoat!(bevy::ui::JustifyContent, as BevyJustifyContent, serde);
crate::select_trenchcoat_traits!(BevyJustifyContent, bevy::ui::JustifyContent, [copy, eq]);

// ── Display ───────────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::Display,
    style    = BevyDisplayStyle,
    prompt   = "Choose layout display model:",
    kani_var = "bevy::ui::Display::Flex",
    variants = [Self::Flex, Self::Grid, Self::Block, Self::None]
);

crate::select_trenchcoat!(bevy::ui::Display, as BevyDisplay, serde);
crate::select_trenchcoat_traits!(BevyDisplay, bevy::ui::Display, [copy, eq]);

// ── BoxSizing ─────────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::BoxSizing,
    style    = BevyBoxSizingStyle,
    prompt   = "Choose box sizing model:",
    kani_var = "bevy::ui::BoxSizing::BorderBox",
    variants = [Self::BorderBox, Self::ContentBox]
);

crate::select_trenchcoat!(bevy::ui::BoxSizing, as BevyBoxSizing, serde);
crate::select_trenchcoat_traits!(BevyBoxSizing, bevy::ui::BoxSizing, [copy, eq]);

// ── FlexDirection ─────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::FlexDirection,
    style    = BevyFlexDirectionStyle,
    prompt   = "Choose flex main axis direction:",
    kani_var = "bevy::ui::FlexDirection::Row",
    variants = [Self::Row, Self::Column, Self::RowReverse, Self::ColumnReverse]
);

crate::select_trenchcoat!(bevy::ui::FlexDirection, as BevyFlexDirection, serde);
crate::select_trenchcoat_traits!(BevyFlexDirection, bevy::ui::FlexDirection, [copy, eq]);

// ── FlexWrap ──────────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::FlexWrap,
    style    = BevyFlexWrapStyle,
    prompt   = "Choose flex wrapping behavior:",
    kani_var = "bevy::ui::FlexWrap::NoWrap",
    variants = [Self::NoWrap, Self::Wrap, Self::WrapReverse]
);

crate::select_trenchcoat!(bevy::ui::FlexWrap, as BevyFlexWrap, serde);
crate::select_trenchcoat_traits!(BevyFlexWrap, bevy::ui::FlexWrap, [copy, eq]);

// ── PositionType ──────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::PositionType,
    style    = BevyPositionTypeStyle,
    prompt   = "Choose node positioning type:",
    kani_var = "bevy::ui::PositionType::Relative",
    variants = [Self::Relative, Self::Absolute]
);

crate::select_trenchcoat!(bevy::ui::PositionType, as BevyPositionType, serde);
crate::select_trenchcoat_traits!(BevyPositionType, bevy::ui::PositionType, [copy, eq]);

// ── OverflowAxis ──────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::OverflowAxis,
    style    = BevyOverflowAxisStyle,
    prompt   = "Choose overflow behavior on this axis:",
    kani_var = "bevy::ui::OverflowAxis::Visible",
    variants = [Self::Visible, Self::Clip, Self::Hidden, Self::Scroll]
);

crate::select_trenchcoat!(bevy::ui::OverflowAxis, as BevyOverflowAxis, serde);
crate::select_trenchcoat_traits!(BevyOverflowAxis, bevy::ui::OverflowAxis, [copy, eq]);

// ── VisualBox ─────────────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::VisualBox,
    style    = BevyVisualBoxStyle,
    prompt   = "Choose the clip boundary box:",
    kani_var = "bevy::ui::VisualBox::PaddingBox",
    variants = [Self::ContentBox, Self::PaddingBox, Self::BorderBox]
);

crate::select_trenchcoat!(bevy::ui::VisualBox, as BevyVisualBox, serde);
crate::select_trenchcoat_traits!(BevyVisualBox, bevy::ui::VisualBox, [copy, eq]);

// ── BevyOverflowClipMargin ────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::OverflowClipMargin`].
///
/// Specifies the visible area for clipped overflow: a [`BevyVisualBox`] reference
/// box (ContentBox, PaddingBox, or BorderBox) plus an optional margin in logical
/// pixels that widens the clipping region.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyOverflowClipMargin {
    /// Which box model box to clip against.
    pub visual_box: BevyVisualBox,
    /// Extra unclipped margin beyond the visual box, in logical pixels.
    pub margin: f32,
}

impl From<bevy::ui::OverflowClipMargin> for BevyOverflowClipMargin {
    fn from(o: bevy::ui::OverflowClipMargin) -> Self {
        Self {
            visual_box: BevyVisualBox(o.visual_box),
            margin: o.margin,
        }
    }
}

impl From<BevyOverflowClipMargin> for bevy::ui::OverflowClipMargin {
    fn from(b: BevyOverflowClipMargin) -> Self {
        Self {
            visual_box: b.visual_box.into_inner(),
            margin: b.margin,
        }
    }
}

crate::default_style!(BevyOverflowClipMargin => BevyOverflowClipMarginStyle);

impl Prompt for BevyOverflowClipMargin {
    fn prompt() -> Option<&'static str> {
        Some("Overflow clip margin (visual box + margin):")
    }
}

impl Elicitation for BevyOverflowClipMargin {
    type Style = BevyOverflowClipMarginStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let visual_box = BevyVisualBox::elicit(communicator).await?;
        let margin = f32::elicit(communicator).await?;
        Ok(Self { visual_box, margin })
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

impl ElicitIntrospect for BevyOverflowClipMargin {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::OverflowClipMargin",
            description: Some("Overflow clip boundary: visual box + margin in logical pixels"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "visual_box",
                        type_name: "bevy::ui::VisualBox",
                        prompt: Some("Clip reference box (ContentBox/PaddingBox/BorderBox):"),
                    },
                    FieldInfo {
                        name: "margin",
                        type_name: "f32",
                        prompt: Some("Extra clip margin in logical pixels:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyOverflowClipMargin {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Overflow clip margin:".to_string()),
            type_name: "bevy::ui::OverflowClipMargin".to_string(),
            fields: vec![
                (
                    "visual_box".to_string(),
                    Box::new(BevyVisualBox::prompt_tree()),
                ),
                ("margin".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyOverflowClipMargin {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let vb = match self.visual_box.0 {
            bevy::ui::VisualBox::ContentBox => {
                quote::quote! { bevy::ui::VisualBox::ContentBox }
            }
            bevy::ui::VisualBox::PaddingBox => {
                quote::quote! { bevy::ui::VisualBox::PaddingBox }
            }
            bevy::ui::VisualBox::BorderBox => {
                quote::quote! { bevy::ui::VisualBox::BorderBox }
            }
        };
        let margin = self.margin;
        quote::quote! {
            bevy::ui::OverflowClipMargin { visual_box: #vb, margin: #margin }
        }
    }
}

// ── BevyVal ───────────────────────────────────────────────────────────────────
//
// bevy::ui::Val has data variants (Px, Percent, Vw, Vh, VMin, VMax) so we
// use the owned enum trenchcoat pattern.

/// Owned trenchcoat for [`bevy::ui::Val`].
///
/// Covers all seven variants: Auto, Px, Percent, Vw, Vh, VMin, VMax.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "unit", rename_all = "snake_case")]
pub enum BevyVal {
    /// Automatically determined by layout.
    Auto,
    /// Logical pixels.
    Px {
        /// Value in logical pixels.
        value: f32,
    },
    /// Percentage of parent.
    Percent {
        /// Percentage value.
        value: f32,
    },
    /// Viewport width percentage.
    Vw {
        /// Percentage of viewport width.
        value: f32,
    },
    /// Viewport height percentage.
    Vh {
        /// Percentage of viewport height.
        value: f32,
    },
    /// Percentage of the smaller viewport dimension.
    VMin {
        /// Percentage of viewport min-dimension.
        value: f32,
    },
    /// Percentage of the larger viewport dimension.
    VMax {
        /// Percentage of viewport max-dimension.
        value: f32,
    },
}

impl From<BevyVal> for bevy::ui::Val {
    fn from(v: BevyVal) -> Self {
        match v {
            BevyVal::Auto => Self::Auto,
            BevyVal::Px { value } => Self::Px(value),
            BevyVal::Percent { value } => Self::Percent(value),
            BevyVal::Vw { value } => Self::Vw(value),
            BevyVal::Vh { value } => Self::Vh(value),
            BevyVal::VMin { value } => Self::VMin(value),
            BevyVal::VMax { value } => Self::VMax(value),
        }
    }
}

impl From<bevy::ui::Val> for BevyVal {
    fn from(v: bevy::ui::Val) -> Self {
        match v {
            bevy::ui::Val::Auto => Self::Auto,
            bevy::ui::Val::Px(x) => Self::Px { value: x },
            bevy::ui::Val::Percent(x) => Self::Percent { value: x },
            bevy::ui::Val::Vw(x) => Self::Vw { value: x },
            bevy::ui::Val::Vh(x) => Self::Vh { value: x },
            bevy::ui::Val::VMin(x) => Self::VMin { value: x },
            bevy::ui::Val::VMax(x) => Self::VMax { value: x },
        }
    }
}

impl Prompt for BevyVal {
    fn prompt() -> Option<&'static str> {
        Some("Enter a UI measurement value:")
    }
}

// ── Internal kind enum for BevyVal variant selection ─────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BevyValKind {
    Auto,
    Px,
    Percent,
    Vw,
    Vh,
    VMin,
    VMax,
}

impl Prompt for BevyValKind {
    fn prompt() -> Option<&'static str> {
        Some("Choose a UI measurement unit:")
    }
}

impl Select for BevyValKind {
    fn options() -> Vec<Self> {
        vec![
            Self::Auto,
            Self::Px,
            Self::Percent,
            Self::Vw,
            Self::Vh,
            Self::VMin,
            Self::VMax,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Auto".into(),
            "Px".into(),
            "Percent".into(),
            "Vw".into(),
            "Vh".into(),
            "VMin".into(),
            "VMax".into(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Auto" => Some(Self::Auto),
            "Px" => Some(Self::Px),
            "Percent" => Some(Self::Percent),
            "Vw" => Some(Self::Vw),
            "Vh" => Some(Self::Vh),
            "VMin" => Some(Self::VMin),
            "VMax" => Some(Self::VMax),
            _ => None,
        }
    }
}

/// Elicit a single f32 value for a BevyVal numeric variant.
async fn elicit_val_float<C: ElicitCommunicator>(
    communicator: &C,
    _prompt: &str,
) -> ElicitResult<f32> {
    f32::elicit(communicator).await
}

crate::default_style!(BevyVal => BevyValStyle);

impl Elicitation for BevyVal {
    type Style = BevyValStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let kind_params = mcp::select_params(
            BevyValKind::prompt().unwrap_or("Choose unit:"),
            &BevyValKind::labels(),
        );
        let kind_result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(kind_params),
            )
            .await?;
        let kind_value = mcp::extract_value(kind_result)?;
        let kind_label = mcp::parse_string(kind_value)?;
        let kind = BevyValKind::from_label(&kind_label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid BevyVal unit: {kind_label}"
            )))
        })?;

        match kind {
            BevyValKind::Auto => Ok(Self::Auto),
            BevyValKind::Px => {
                let v = elicit_val_float(communicator, "Value in logical pixels:").await?;
                Ok(Self::Px { value: v })
            }
            BevyValKind::Percent => {
                let v = elicit_val_float(communicator, "Percentage of parent (0–100):").await?;
                Ok(Self::Percent { value: v })
            }
            BevyValKind::Vw => {
                let v =
                    elicit_val_float(communicator, "Viewport width percentage (0–100):").await?;
                Ok(Self::Vw { value: v })
            }
            BevyValKind::Vh => {
                let v =
                    elicit_val_float(communicator, "Viewport height percentage (0–100):").await?;
                Ok(Self::Vh { value: v })
            }
            BevyValKind::VMin => {
                let v =
                    elicit_val_float(communicator, "Viewport min-dimension percentage (0–100):")
                        .await?;
                Ok(Self::VMin { value: v })
            }
            BevyValKind::VMax => {
                let v =
                    elicit_val_float(communicator, "Viewport max-dimension percentage (0–100):")
                        .await?;
                Ok(Self::VMax { value: v })
            }
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("BevyVal", "BevyVal::Auto")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("BevyVal", "BevyVal::Auto")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("BevyVal", "BevyVal::Auto")
    }
}

impl ElicitIntrospect for BevyVal {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyVal",
            description: Some("UI measurement value (Auto, Px, Percent, Vw, Vh, VMin, VMax)"),
            details: PatternDetails::Select {
                variants: BevyValKind::labels()
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

impl crate::ElicitPromptTree for BevyVal {
    fn prompt_tree() -> crate::PromptTree {
        let f_leaf = crate::PromptTree::Leaf {
            prompt: "Value:".to_string(),
            type_name: "f32".to_string(),
        };
        crate::PromptTree::Select {
            prompt: "Choose a UI measurement unit:".to_string(),
            type_name: "BevyVal".to_string(),
            options: BevyValKind::labels(),
            branches: vec![
                None,
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyVal::Px".to_string(),
                    fields: vec![("value".to_string(), Box::new(f_leaf.clone()))],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyVal::Percent".to_string(),
                    fields: vec![("value".to_string(), Box::new(f_leaf.clone()))],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyVal::Vw".to_string(),
                    fields: vec![("value".to_string(), Box::new(f_leaf.clone()))],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyVal::Vh".to_string(),
                    fields: vec![("value".to_string(), Box::new(f_leaf.clone()))],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyVal::VMin".to_string(),
                    fields: vec![("value".to_string(), Box::new(f_leaf.clone()))],
                })),
                Some(Box::new(crate::PromptTree::Survey {
                    prompt: None,
                    type_name: "BevyVal::VMax".to_string(),
                    fields: vec![("value".to_string(), Box::new(f_leaf))],
                })),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyVal {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Auto => quote::quote! { bevy::ui::Val::Auto },
            Self::Px { value } => quote::quote! { bevy::ui::Val::Px(#value) },
            Self::Percent { value } => quote::quote! { bevy::ui::Val::Percent(#value) },
            Self::Vw { value } => quote::quote! { bevy::ui::Val::Vw(#value) },
            Self::Vh { value } => quote::quote! { bevy::ui::Val::Vh(#value) },
            Self::VMin { value } => quote::quote! { bevy::ui::Val::VMin(#value) },
            Self::VMax { value } => quote::quote! { bevy::ui::Val::VMax(#value) },
        }
    }
}

// ── BevyUiRect ────────────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::UiRect`].
///
/// Holds four [`BevyVal`] fields for left, right, top, and bottom edges.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyUiRect {
    /// Left edge.
    pub left: BevyVal,
    /// Right edge.
    pub right: BevyVal,
    /// Top edge.
    pub top: BevyVal,
    /// Bottom edge.
    pub bottom: BevyVal,
}

impl From<BevyUiRect> for bevy::ui::UiRect {
    fn from(r: BevyUiRect) -> Self {
        Self::new(r.left.into(), r.right.into(), r.top.into(), r.bottom.into())
    }
}

impl From<bevy::ui::UiRect> for BevyUiRect {
    fn from(r: bevy::ui::UiRect) -> Self {
        Self {
            left: r.left.into(),
            right: r.right.into(),
            top: r.top.into(),
            bottom: r.bottom.into(),
        }
    }
}

impl Prompt for BevyUiRect {
    fn prompt() -> Option<&'static str> {
        Some("Enter a UI rect (left, right, top, bottom):")
    }
}

impl Survey for BevyUiRect {
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "left",
                prompt: Some("Left edge:"),
                type_name: "BevyVal",
            },
            FieldInfo {
                name: "right",
                prompt: Some("Right edge:"),
                type_name: "BevyVal",
            },
            FieldInfo {
                name: "top",
                prompt: Some("Top edge:"),
                type_name: "BevyVal",
            },
            FieldInfo {
                name: "bottom",
                prompt: Some("Bottom edge:"),
                type_name: "BevyVal",
            },
        ]
    }
}

crate::default_style!(BevyUiRect => BevyUiRectStyle);

impl Elicitation for BevyUiRect {
    type Style = BevyUiRectStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let left = BevyVal::elicit(communicator).await?;
        let right = BevyVal::elicit(communicator).await?;
        let top = BevyVal::elicit(communicator).await?;
        let bottom = BevyVal::elicit(communicator).await?;
        Ok(Self {
            left,
            right,
            top,
            bottom,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyUiRect {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyUiRect",
            description: Some("UI rect with four Val edges"),
            details: PatternDetails::Survey {
                fields: Self::fields(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyUiRect {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: None,
            type_name: "BevyUiRect".to_string(),
            fields: vec![
                ("left".to_string(), Box::new(BevyVal::prompt_tree())),
                ("right".to_string(), Box::new(BevyVal::prompt_tree())),
                ("top".to_string(), Box::new(BevyVal::prompt_tree())),
                ("bottom".to_string(), Box::new(BevyVal::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyUiRect {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let left = crate::emit_code::ToCodeLiteral::to_code_literal(&self.left);
        let right = crate::emit_code::ToCodeLiteral::to_code_literal(&self.right);
        let top = crate::emit_code::ToCodeLiteral::to_code_literal(&self.top);
        let bottom = crate::emit_code::ToCodeLiteral::to_code_literal(&self.bottom);
        quote::quote! {
            bevy::ui::UiRect {
                left: #left,
                right: #right,
                top: #top,
                bottom: #bottom,
            }
        }
    }
}

// ── BevyBorderRadius ──────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::BorderRadius`].
///
/// Holds four [`BevyVal`] corner radii.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyBorderRadius {
    /// Top-left corner radius.
    pub top_left: BevyVal,
    /// Top-right corner radius.
    pub top_right: BevyVal,
    /// Bottom-right corner radius.
    pub bottom_right: BevyVal,
    /// Bottom-left corner radius.
    pub bottom_left: BevyVal,
}

impl From<BevyBorderRadius> for bevy::ui::BorderRadius {
    fn from(r: BevyBorderRadius) -> Self {
        Self {
            top_left: r.top_left.into(),
            top_right: r.top_right.into(),
            bottom_right: r.bottom_right.into(),
            bottom_left: r.bottom_left.into(),
        }
    }
}

impl From<bevy::ui::BorderRadius> for BevyBorderRadius {
    fn from(r: bevy::ui::BorderRadius) -> Self {
        Self {
            top_left: r.top_left.into(),
            top_right: r.top_right.into(),
            bottom_right: r.bottom_right.into(),
            bottom_left: r.bottom_left.into(),
        }
    }
}

impl Prompt for BevyBorderRadius {
    fn prompt() -> Option<&'static str> {
        Some("Enter border radii (top-left, top-right, bottom-right, bottom-left):")
    }
}

impl Survey for BevyBorderRadius {
    fn fields() -> Vec<FieldInfo> {
        vec![
            FieldInfo {
                name: "top_left",
                prompt: Some("Top-left radius:"),
                type_name: "BevyVal",
            },
            FieldInfo {
                name: "top_right",
                prompt: Some("Top-right radius:"),
                type_name: "BevyVal",
            },
            FieldInfo {
                name: "bottom_right",
                prompt: Some("Bottom-right radius:"),
                type_name: "BevyVal",
            },
            FieldInfo {
                name: "bottom_left",
                prompt: Some("Bottom-left radius:"),
                type_name: "BevyVal",
            },
        ]
    }
}

crate::default_style!(BevyBorderRadius => BevyBorderRadiusStyle);

impl Elicitation for BevyBorderRadius {
    type Style = BevyBorderRadiusStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let top_left = BevyVal::elicit(communicator).await?;
        let top_right = BevyVal::elicit(communicator).await?;
        let bottom_right = BevyVal::elicit(communicator).await?;
        let bottom_left = BevyVal::elicit(communicator).await?;
        Ok(Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyBorderRadius {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "BevyBorderRadius",
            description: Some("Border radii for the four corners of a UI node"),
            details: PatternDetails::Survey {
                fields: Self::fields(),
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyBorderRadius {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: None,
            type_name: "BevyBorderRadius".to_string(),
            fields: vec![
                ("top_left".to_string(), Box::new(BevyVal::prompt_tree())),
                ("top_right".to_string(), Box::new(BevyVal::prompt_tree())),
                ("bottom_right".to_string(), Box::new(BevyVal::prompt_tree())),
                ("bottom_left".to_string(), Box::new(BevyVal::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyBorderRadius {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let tl = crate::emit_code::ToCodeLiteral::to_code_literal(&self.top_left);
        let tr = crate::emit_code::ToCodeLiteral::to_code_literal(&self.top_right);
        let br = crate::emit_code::ToCodeLiteral::to_code_literal(&self.bottom_right);
        let bl = crate::emit_code::ToCodeLiteral::to_code_literal(&self.bottom_left);
        quote::quote! {
            bevy::ui::BorderRadius {
                top_left: #tl,
                top_right: #tr,
                bottom_right: #br,
                bottom_left: #bl,
            }
        }
    }
}

// ── BevyOverflow ──────────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::Overflow`].
///
/// Controls how overflowing content is handled on each axis independently.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyOverflow {
    /// Overflow behavior on the horizontal axis.
    pub x: BevyOverflowAxis,
    /// Overflow behavior on the vertical axis.
    pub y: BevyOverflowAxis,
}

impl From<bevy::ui::Overflow> for BevyOverflow {
    fn from(o: bevy::ui::Overflow) -> Self {
        Self {
            x: BevyOverflowAxis(o.x),
            y: BevyOverflowAxis(o.y),
        }
    }
}

impl From<BevyOverflow> for bevy::ui::Overflow {
    fn from(b: BevyOverflow) -> Self {
        Self {
            x: b.x.into_inner(),
            y: b.y.into_inner(),
        }
    }
}

crate::default_style!(BevyOverflow => BevyOverflowStyle);

impl Prompt for BevyOverflow {
    fn prompt() -> Option<&'static str> {
        Some("Overflow behavior (x and y axes):")
    }
}

impl Elicitation for BevyOverflow {
    type Style = BevyOverflowStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let x = BevyOverflowAxis::elicit(communicator).await?;
        let y = BevyOverflowAxis::elicit(communicator).await?;
        Ok(Self { x, y })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyOverflowAxis as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyOverflowAxis as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyOverflowAxis as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyOverflow {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::Overflow",
            description: Some("Overflow behavior on the horizontal and vertical axes"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "bevy::ui::OverflowAxis",
                        prompt: Some("Horizontal overflow:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "bevy::ui::OverflowAxis",
                        prompt: Some("Vertical overflow:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyOverflow {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Overflow:".to_string()),
            type_name: "bevy::ui::Overflow".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(BevyOverflowAxis::prompt_tree())),
                ("y".to_string(), Box::new(BevyOverflowAxis::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyOverflow {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x.0;
        let y = self.y.0;
        let x_tok = match x {
            bevy::ui::OverflowAxis::Visible => quote::quote! { bevy::ui::OverflowAxis::Visible },
            bevy::ui::OverflowAxis::Clip => quote::quote! { bevy::ui::OverflowAxis::Clip },
            bevy::ui::OverflowAxis::Hidden => quote::quote! { bevy::ui::OverflowAxis::Hidden },
            bevy::ui::OverflowAxis::Scroll => quote::quote! { bevy::ui::OverflowAxis::Scroll },
        };
        let y_tok = match y {
            bevy::ui::OverflowAxis::Visible => quote::quote! { bevy::ui::OverflowAxis::Visible },
            bevy::ui::OverflowAxis::Clip => quote::quote! { bevy::ui::OverflowAxis::Clip },
            bevy::ui::OverflowAxis::Hidden => quote::quote! { bevy::ui::OverflowAxis::Hidden },
            bevy::ui::OverflowAxis::Scroll => quote::quote! { bevy::ui::OverflowAxis::Scroll },
        };
        quote::quote! { bevy::ui::Overflow { x: #x_tok, y: #y_tok } }
    }
}

// ── BevyInlineDirection ───────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::InlineDirection,
    style    = BevyInlineDirectionStyle,
    prompt   = "Choose inline text direction:",
    kani_var = "bevy::ui::InlineDirection::Ltr",
    variants = [Self::Ltr, Self::Rtl]
);

crate::select_trenchcoat!(bevy::ui::InlineDirection, as BevyInlineDirection, serde);
crate::select_trenchcoat_traits!(BevyInlineDirection, bevy::ui::InlineDirection, [copy, eq]);

// ── BevyGridAutoFlow ──────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::GridAutoFlow,
    style    = BevyGridAutoFlowStyle,
    prompt   = "Choose grid auto-placement flow direction:",
    kani_var = "bevy::ui::GridAutoFlow::Row",
    variants = [Self::Row, Self::Column, Self::RowDense, Self::ColumnDense]
);

crate::select_trenchcoat!(bevy::ui::GridAutoFlow, as BevyGridAutoFlow, serde);
crate::select_trenchcoat_traits!(BevyGridAutoFlow, bevy::ui::GridAutoFlow, [copy, eq]);

// ── BevyMinTrackSizingFunction ────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::MinTrackSizingFunction`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BevyMinTrackSizingFunction {
    /// Fixed pixel size.
    Px {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the available space.
    Percent {
        /// The numeric value.
        value: f32,
    },
    /// Sized under a min-content constraint.
    MinContent,
    /// Sized under a max-content constraint.
    MaxContent,
    /// Automatically sized.
    Auto,
    /// Percentage of the viewport's smaller dimension.
    VMin {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport's larger dimension.
    VMax {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport height.
    Vh {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport width.
    Vw {
        /// The numeric value.
        value: f32,
    },
}

impl From<bevy::ui::MinTrackSizingFunction> for BevyMinTrackSizingFunction {
    fn from(v: bevy::ui::MinTrackSizingFunction) -> Self {
        match v {
            bevy::ui::MinTrackSizingFunction::Px(x) => Self::Px { value: x },
            bevy::ui::MinTrackSizingFunction::Percent(x) => Self::Percent { value: x },
            bevy::ui::MinTrackSizingFunction::MinContent => Self::MinContent,
            bevy::ui::MinTrackSizingFunction::MaxContent => Self::MaxContent,
            bevy::ui::MinTrackSizingFunction::Auto => Self::Auto,
            bevy::ui::MinTrackSizingFunction::VMin(x) => Self::VMin { value: x },
            bevy::ui::MinTrackSizingFunction::VMax(x) => Self::VMax { value: x },
            bevy::ui::MinTrackSizingFunction::Vh(x) => Self::Vh { value: x },
            bevy::ui::MinTrackSizingFunction::Vw(x) => Self::Vw { value: x },
        }
    }
}

impl From<BevyMinTrackSizingFunction> for bevy::ui::MinTrackSizingFunction {
    fn from(v: BevyMinTrackSizingFunction) -> Self {
        match v {
            BevyMinTrackSizingFunction::Px { value } => Self::Px(value),
            BevyMinTrackSizingFunction::Percent { value } => Self::Percent(value),
            BevyMinTrackSizingFunction::MinContent => Self::MinContent,
            BevyMinTrackSizingFunction::MaxContent => Self::MaxContent,
            BevyMinTrackSizingFunction::Auto => Self::Auto,
            BevyMinTrackSizingFunction::VMin { value } => Self::VMin(value),
            BevyMinTrackSizingFunction::VMax { value } => Self::VMax(value),
            BevyMinTrackSizingFunction::Vh { value } => Self::Vh(value),
            BevyMinTrackSizingFunction::Vw { value } => Self::Vw(value),
        }
    }
}

impl Prompt for BevyMinTrackSizingFunction {
    fn prompt() -> Option<&'static str> {
        Some("Minimum track sizing function:")
    }
}

impl Select for BevyMinTrackSizingFunction {
    fn options() -> Vec<Self> {
        vec![
            Self::Auto,
            Self::MinContent,
            Self::MaxContent,
            Self::Px { value: 0.0 },
            Self::Percent { value: 0.0 },
            Self::VMin { value: 0.0 },
            Self::VMax { value: 0.0 },
            Self::Vh { value: 0.0 },
            Self::Vw { value: 0.0 },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Auto".to_string(),
            "MinContent".to_string(),
            "MaxContent".to_string(),
            "Px".to_string(),
            "Percent".to_string(),
            "VMin".to_string(),
            "VMax".to_string(),
            "Vh".to_string(),
            "Vw".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Auto" => Some(Self::Auto),
            "MinContent" => Some(Self::MinContent),
            "MaxContent" => Some(Self::MaxContent),
            "Px" => Some(Self::Px { value: 0.0 }),
            "Percent" => Some(Self::Percent { value: 0.0 }),
            "VMin" => Some(Self::VMin { value: 0.0 }),
            "VMax" => Some(Self::VMax { value: 0.0 }),
            "Vh" => Some(Self::Vh { value: 0.0 }),
            "Vw" => Some(Self::Vw { value: 0.0 }),
            _ => None,
        }
    }
}

crate::default_style!(BevyMinTrackSizingFunction => BevyMinTrackSizingFunctionStyle);

impl Elicitation for BevyMinTrackSizingFunction {
    type Style = BevyMinTrackSizingFunctionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose a value:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match label.as_str() {
            "Auto" => Ok(Self::Auto),
            "MinContent" => Ok(Self::MinContent),
            "MaxContent" => Ok(Self::MaxContent),
            lbl @ ("Px" | "Percent" | "VMin" | "VMax" | "Vh" | "Vw") => {
                let v = f32::elicit(communicator).await?;
                match lbl {
                    "Px" => Ok(Self::Px { value: v }),
                    "Percent" => Ok(Self::Percent { value: v }),
                    "VMin" => Ok(Self::VMin { value: v }),
                    "VMax" => Ok(Self::VMax { value: v }),
                    "Vh" => Ok(Self::Vh { value: v }),
                    "Vw" => Ok(Self::Vw { value: v }),
                    _ => unreachable!(),
                }
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid MinTrackSizingFunction: {label}"
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

impl ElicitIntrospect for BevyMinTrackSizingFunction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::MinTrackSizingFunction",
            description: Some("Minimum sizing function for a CSS Grid track"),
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

impl crate::ElicitPromptTree for BevyMinTrackSizingFunction {
    fn prompt_tree() -> crate::PromptTree {
        let leaf = Box::new(crate::PromptTree::Leaf {
            prompt: "Value:".to_string(),
            type_name: "f32".to_string(),
        });
        // Auto, MinContent, MaxContent are unit variants; the rest take an f32.
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Minimum track sizing:")
                .to_string(),
            type_name: "bevy::ui::MinTrackSizingFunction".to_string(),
            options: Self::labels(),
            branches: vec![
                None,
                None,
                None,
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyMinTrackSizingFunction {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Auto => quote::quote! { bevy::ui::MinTrackSizingFunction::Auto },
            Self::MinContent => quote::quote! { bevy::ui::MinTrackSizingFunction::MinContent },
            Self::MaxContent => quote::quote! { bevy::ui::MinTrackSizingFunction::MaxContent },
            Self::Px { value } => quote::quote! { bevy::ui::MinTrackSizingFunction::Px(#value) },
            Self::Percent { value } => {
                quote::quote! { bevy::ui::MinTrackSizingFunction::Percent(#value) }
            }
            Self::VMin { value } => {
                quote::quote! { bevy::ui::MinTrackSizingFunction::VMin(#value) }
            }
            Self::VMax { value } => {
                quote::quote! { bevy::ui::MinTrackSizingFunction::VMax(#value) }
            }
            Self::Vh { value } => quote::quote! { bevy::ui::MinTrackSizingFunction::Vh(#value) },
            Self::Vw { value } => quote::quote! { bevy::ui::MinTrackSizingFunction::Vw(#value) },
        }
    }
}

// ── BevyMaxTrackSizingFunction ────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::MaxTrackSizingFunction`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BevyMaxTrackSizingFunction {
    /// Fixed pixel size.
    Px {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the available space.
    Percent {
        /// The numeric value.
        value: f32,
    },
    /// Sized under a min-content constraint.
    MinContent,
    /// Sized under a max-content constraint.
    MaxContent,
    /// fit-content formula with a fixed pixel limit.
    FitContentPx {
        /// The numeric value.
        value: f32,
    },
    /// fit-content formula with a percentage limit.
    FitContentPercent {
        /// The numeric value.
        value: f32,
    },
    /// Automatically sized.
    Auto,
    /// Fraction of the remaining free space (`fr` units).
    Fraction {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport's smaller dimension.
    VMin {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport's larger dimension.
    VMax {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport height.
    Vh {
        /// The numeric value.
        value: f32,
    },
    /// Percentage of the viewport width.
    Vw {
        /// The numeric value.
        value: f32,
    },
}

impl From<bevy::ui::MaxTrackSizingFunction> for BevyMaxTrackSizingFunction {
    fn from(v: bevy::ui::MaxTrackSizingFunction) -> Self {
        match v {
            bevy::ui::MaxTrackSizingFunction::Px(x) => Self::Px { value: x },
            bevy::ui::MaxTrackSizingFunction::Percent(x) => Self::Percent { value: x },
            bevy::ui::MaxTrackSizingFunction::MinContent => Self::MinContent,
            bevy::ui::MaxTrackSizingFunction::MaxContent => Self::MaxContent,
            bevy::ui::MaxTrackSizingFunction::FitContentPx(x) => Self::FitContentPx { value: x },
            bevy::ui::MaxTrackSizingFunction::FitContentPercent(x) => {
                Self::FitContentPercent { value: x }
            }
            bevy::ui::MaxTrackSizingFunction::Auto => Self::Auto,
            bevy::ui::MaxTrackSizingFunction::Fraction(x) => Self::Fraction { value: x },
            bevy::ui::MaxTrackSizingFunction::VMin(x) => Self::VMin { value: x },
            bevy::ui::MaxTrackSizingFunction::VMax(x) => Self::VMax { value: x },
            bevy::ui::MaxTrackSizingFunction::Vh(x) => Self::Vh { value: x },
            bevy::ui::MaxTrackSizingFunction::Vw(x) => Self::Vw { value: x },
        }
    }
}

impl From<BevyMaxTrackSizingFunction> for bevy::ui::MaxTrackSizingFunction {
    fn from(v: BevyMaxTrackSizingFunction) -> Self {
        match v {
            BevyMaxTrackSizingFunction::Px { value } => Self::Px(value),
            BevyMaxTrackSizingFunction::Percent { value } => Self::Percent(value),
            BevyMaxTrackSizingFunction::MinContent => Self::MinContent,
            BevyMaxTrackSizingFunction::MaxContent => Self::MaxContent,
            BevyMaxTrackSizingFunction::FitContentPx { value } => Self::FitContentPx(value),
            BevyMaxTrackSizingFunction::FitContentPercent { value } => {
                Self::FitContentPercent(value)
            }
            BevyMaxTrackSizingFunction::Auto => Self::Auto,
            BevyMaxTrackSizingFunction::Fraction { value } => Self::Fraction(value),
            BevyMaxTrackSizingFunction::VMin { value } => Self::VMin(value),
            BevyMaxTrackSizingFunction::VMax { value } => Self::VMax(value),
            BevyMaxTrackSizingFunction::Vh { value } => Self::Vh(value),
            BevyMaxTrackSizingFunction::Vw { value } => Self::Vw(value),
        }
    }
}

impl Prompt for BevyMaxTrackSizingFunction {
    fn prompt() -> Option<&'static str> {
        Some("Maximum track sizing function:")
    }
}

impl Select for BevyMaxTrackSizingFunction {
    fn options() -> Vec<Self> {
        vec![
            Self::Auto,
            Self::MinContent,
            Self::MaxContent,
            Self::Px { value: 0.0 },
            Self::Percent { value: 0.0 },
            Self::FitContentPx { value: 0.0 },
            Self::FitContentPercent { value: 0.0 },
            Self::Fraction { value: 1.0 },
            Self::VMin { value: 0.0 },
            Self::VMax { value: 0.0 },
            Self::Vh { value: 0.0 },
            Self::Vw { value: 0.0 },
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Auto".to_string(),
            "MinContent".to_string(),
            "MaxContent".to_string(),
            "Px".to_string(),
            "Percent".to_string(),
            "FitContentPx".to_string(),
            "FitContentPercent".to_string(),
            "Fraction".to_string(),
            "VMin".to_string(),
            "VMax".to_string(),
            "Vh".to_string(),
            "Vw".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Auto" => Some(Self::Auto),
            "MinContent" => Some(Self::MinContent),
            "MaxContent" => Some(Self::MaxContent),
            "Px" => Some(Self::Px { value: 0.0 }),
            "Percent" => Some(Self::Percent { value: 0.0 }),
            "FitContentPx" => Some(Self::FitContentPx { value: 0.0 }),
            "FitContentPercent" => Some(Self::FitContentPercent { value: 0.0 }),
            "Fraction" => Some(Self::Fraction { value: 1.0 }),
            "VMin" => Some(Self::VMin { value: 0.0 }),
            "VMax" => Some(Self::VMax { value: 0.0 }),
            "Vh" => Some(Self::Vh { value: 0.0 }),
            "Vw" => Some(Self::Vw { value: 0.0 }),
            _ => None,
        }
    }
}

crate::default_style!(BevyMaxTrackSizingFunction => BevyMaxTrackSizingFunctionStyle);

impl Elicitation for BevyMaxTrackSizingFunction {
    type Style = BevyMaxTrackSizingFunctionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose a value:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match label.as_str() {
            "Auto" => Ok(Self::Auto),
            "MinContent" => Ok(Self::MinContent),
            "MaxContent" => Ok(Self::MaxContent),
            lbl @ ("Px" | "Percent" | "FitContentPx" | "FitContentPercent" | "Fraction"
            | "VMin" | "VMax" | "Vh" | "Vw") => {
                let v = f32::elicit(communicator).await?;
                match lbl {
                    "Px" => Ok(Self::Px { value: v }),
                    "Percent" => Ok(Self::Percent { value: v }),
                    "FitContentPx" => Ok(Self::FitContentPx { value: v }),
                    "FitContentPercent" => Ok(Self::FitContentPercent { value: v }),
                    "Fraction" => Ok(Self::Fraction { value: v }),
                    "VMin" => Ok(Self::VMin { value: v }),
                    "VMax" => Ok(Self::VMax { value: v }),
                    "Vh" => Ok(Self::Vh { value: v }),
                    "Vw" => Ok(Self::Vw { value: v }),
                    _ => unreachable!(),
                }
            }
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid MaxTrackSizingFunction: {label}"
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

impl ElicitIntrospect for BevyMaxTrackSizingFunction {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::MaxTrackSizingFunction",
            description: Some("Maximum sizing function for a CSS Grid track"),
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

impl crate::ElicitPromptTree for BevyMaxTrackSizingFunction {
    fn prompt_tree() -> crate::PromptTree {
        let leaf = Box::new(crate::PromptTree::Leaf {
            prompt: "Value:".to_string(),
            type_name: "f32".to_string(),
        });
        // Auto, MinContent, MaxContent are unit variants; the rest take an f32.
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Maximum track sizing:")
                .to_string(),
            type_name: "bevy::ui::MaxTrackSizingFunction".to_string(),
            options: Self::labels(),
            branches: vec![
                None,
                None,
                None,
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf.clone()),
                Some(leaf),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyMaxTrackSizingFunction {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Auto => quote::quote! { bevy::ui::MaxTrackSizingFunction::Auto },
            Self::MinContent => quote::quote! { bevy::ui::MaxTrackSizingFunction::MinContent },
            Self::MaxContent => quote::quote! { bevy::ui::MaxTrackSizingFunction::MaxContent },
            Self::Px { value } => quote::quote! { bevy::ui::MaxTrackSizingFunction::Px(#value) },
            Self::Percent { value } => {
                quote::quote! { bevy::ui::MaxTrackSizingFunction::Percent(#value) }
            }
            Self::FitContentPx { value } => {
                quote::quote! { bevy::ui::MaxTrackSizingFunction::FitContentPx(#value) }
            }
            Self::FitContentPercent { value } => {
                quote::quote! { bevy::ui::MaxTrackSizingFunction::FitContentPercent(#value) }
            }
            Self::Fraction { value } => {
                quote::quote! { bevy::ui::MaxTrackSizingFunction::Fraction(#value) }
            }
            Self::VMin { value } => {
                quote::quote! { bevy::ui::MaxTrackSizingFunction::VMin(#value) }
            }
            Self::VMax { value } => {
                quote::quote! { bevy::ui::MaxTrackSizingFunction::VMax(#value) }
            }
            Self::Vh { value } => quote::quote! { bevy::ui::MaxTrackSizingFunction::Vh(#value) },
            Self::Vw { value } => quote::quote! { bevy::ui::MaxTrackSizingFunction::Vw(#value) },
        }
    }
}

// ── BevyGridTrackRepetition ───────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::GridTrackRepetition`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BevyGridTrackRepetition {
    /// Repeat the track a fixed number of times.
    Count {
        /// Number of repetitions.
        count: u16,
    },
    /// Repeat to fill available space (auto-fill).
    AutoFill,
    /// Repeat to fill available space, collapsing empty tracks (auto-fit).
    AutoFit,
}

impl From<bevy::ui::GridTrackRepetition> for BevyGridTrackRepetition {
    fn from(v: bevy::ui::GridTrackRepetition) -> Self {
        match v {
            bevy::ui::GridTrackRepetition::Count(n) => Self::Count { count: n },
            bevy::ui::GridTrackRepetition::AutoFill => Self::AutoFill,
            bevy::ui::GridTrackRepetition::AutoFit => Self::AutoFit,
        }
    }
}

impl From<BevyGridTrackRepetition> for bevy::ui::GridTrackRepetition {
    fn from(v: BevyGridTrackRepetition) -> Self {
        match v {
            BevyGridTrackRepetition::Count { count } => Self::Count(count),
            BevyGridTrackRepetition::AutoFill => Self::AutoFill,
            BevyGridTrackRepetition::AutoFit => Self::AutoFit,
        }
    }
}

impl Prompt for BevyGridTrackRepetition {
    fn prompt() -> Option<&'static str> {
        Some("Grid track repetition:")
    }
}

impl Select for BevyGridTrackRepetition {
    fn options() -> Vec<Self> {
        vec![Self::Count { count: 1 }, Self::AutoFill, Self::AutoFit]
    }

    fn labels() -> Vec<String> {
        vec![
            "Count".to_string(),
            "AutoFill".to_string(),
            "AutoFit".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Count" => Some(Self::Count { count: 1 }),
            "AutoFill" => Some(Self::AutoFill),
            "AutoFit" => Some(Self::AutoFit),
            _ => None,
        }
    }
}

crate::default_style!(BevyGridTrackRepetition => BevyGridTrackRepetitionStyle);

impl Elicitation for BevyGridTrackRepetition {
    type Style = BevyGridTrackRepetitionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose a value:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        match label.as_str() {
            "Count" => {
                let count = u16::elicit(communicator).await?;
                Ok(Self::Count { count })
            }
            "AutoFill" => Ok(Self::AutoFill),
            "AutoFit" => Ok(Self::AutoFit),
            _ => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid GridTrackRepetition: {label}"
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

impl ElicitIntrospect for BevyGridTrackRepetition {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::GridTrackRepetition",
            description: Some("How many times a grid track repeats"),
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

impl crate::ElicitPromptTree for BevyGridTrackRepetition {
    fn prompt_tree() -> crate::PromptTree {
        let u16_leaf = Box::new(crate::PromptTree::Leaf {
            prompt: "Count:".to_string(),
            type_name: "u16".to_string(),
        });
        crate::PromptTree::Select {
            prompt: Self::prompt()
                .unwrap_or("Grid track repetition:")
                .to_string(),
            type_name: "bevy::ui::GridTrackRepetition".to_string(),
            options: Self::labels(),
            branches: vec![Some(u16_leaf), None, None],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGridTrackRepetition {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Count { count } => quote::quote! { bevy::ui::GridTrackRepetition::Count(#count) },
            Self::AutoFill => quote::quote! { bevy::ui::GridTrackRepetition::AutoFill },
            Self::AutoFit => quote::quote! { bevy::ui::GridTrackRepetition::AutoFit },
        }
    }
}

// ── Helper: emit a bevy enum path from its Debug repr ────────────────────────

fn bevy_ui_enum_tok(module: &str, value: &dyn std::fmt::Debug) -> proc_macro2::TokenStream {
    let variant = format!("{:?}", value);
    let full = format!("bevy::ui::{module}::{variant}");
    full.parse().expect("valid enum path")
}

// ── BevyGridTrack ─────────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::GridTrack`].
///
/// Stores the min and max sizing functions explicitly. Converts one-way to
/// `bevy::ui::GridTrack` because bevy's type has private fields.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyGridTrack {
    /// Minimum sizing function for this track.
    pub min: BevyMinTrackSizingFunction,
    /// Maximum sizing function for this track.
    pub max: BevyMaxTrackSizingFunction,
}

impl From<BevyGridTrack> for bevy::ui::GridTrack {
    fn from(v: BevyGridTrack) -> Self {
        bevy::ui::GridTrack::minmax(
            bevy::ui::MinTrackSizingFunction::from(v.min),
            bevy::ui::MaxTrackSizingFunction::from(v.max),
        )
    }
}

crate::default_style!(BevyGridTrack => BevyGridTrackStyle);

impl Prompt for BevyGridTrack {
    fn prompt() -> Option<&'static str> {
        Some("Grid track sizing (min and max):")
    }
}

impl Elicitation for BevyGridTrack {
    type Style = BevyGridTrackStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let min = BevyMinTrackSizingFunction::elicit(communicator).await?;
        let max = BevyMaxTrackSizingFunction::elicit(communicator).await?;
        Ok(Self { min, max })
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

impl ElicitIntrospect for BevyGridTrack {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::GridTrack",
            description: Some("A CSS Grid track with min and max sizing functions"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "min",
                        type_name: "bevy::ui::MinTrackSizingFunction",
                        prompt: Some("Minimum track size:"),
                    },
                    FieldInfo {
                        name: "max",
                        type_name: "bevy::ui::MaxTrackSizingFunction",
                        prompt: Some("Maximum track size:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyGridTrack {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Grid track:".to_string()),
            type_name: "bevy::ui::GridTrack".to_string(),
            fields: vec![
                (
                    "min".to_string(),
                    Box::new(BevyMinTrackSizingFunction::prompt_tree()),
                ),
                (
                    "max".to_string(),
                    Box::new(BevyMaxTrackSizingFunction::prompt_tree()),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGridTrack {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let min = self.min.to_code_literal();
        let max = self.max.to_code_literal();
        quote::quote! { bevy::ui::GridTrack::minmax(#min, #max) }
    }
}

// ── BevyRepeatedGridTrack ─────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::RepeatedGridTrack`].
///
/// Represents a single-track pattern with a repetition count. This covers the
/// common CSS Grid pattern `repeat(N, <track-def>)`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyRepeatedGridTrack {
    /// How many times to repeat.
    pub repetition: BevyGridTrackRepetition,
    /// The track definition to repeat.
    pub track: BevyGridTrack,
}

impl From<BevyRepeatedGridTrack> for bevy::ui::RepeatedGridTrack {
    fn from(v: BevyRepeatedGridTrack) -> Self {
        bevy::ui::RepeatedGridTrack::minmax(
            bevy::ui::GridTrackRepetition::from(v.repetition),
            bevy::ui::MinTrackSizingFunction::from(v.track.min),
            bevy::ui::MaxTrackSizingFunction::from(v.track.max),
        )
    }
}

crate::default_style!(BevyRepeatedGridTrack => BevyRepeatedGridTrackStyle);

impl Prompt for BevyRepeatedGridTrack {
    fn prompt() -> Option<&'static str> {
        Some("Repeated grid track (repetition + track definition):")
    }
}

impl Elicitation for BevyRepeatedGridTrack {
    type Style = BevyRepeatedGridTrackStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let repetition = BevyGridTrackRepetition::elicit(communicator).await?;
        let track = BevyGridTrack::elicit(communicator).await?;
        Ok(Self { repetition, track })
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

impl ElicitIntrospect for BevyRepeatedGridTrack {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::RepeatedGridTrack",
            description: Some("A repeated CSS Grid track pattern"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "repetition",
                        type_name: "bevy::ui::GridTrackRepetition",
                        prompt: Some("Number of repetitions:"),
                    },
                    FieldInfo {
                        name: "track",
                        type_name: "bevy::ui::GridTrack",
                        prompt: Some("Track definition:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyRepeatedGridTrack {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Repeated grid track:".to_string()),
            type_name: "bevy::ui::RepeatedGridTrack".to_string(),
            fields: vec![
                (
                    "repetition".to_string(),
                    Box::new(BevyGridTrackRepetition::prompt_tree()),
                ),
                ("track".to_string(), Box::new(BevyGridTrack::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyRepeatedGridTrack {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let repetition = self.repetition.to_code_literal();
        let min = self.track.min.to_code_literal();
        let max = self.track.max.to_code_literal();
        quote::quote! { bevy::ui::RepeatedGridTrack::minmax(#repetition, #min, #max) }
    }
}

// ── BevyGridPlacement ─────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::GridPlacement`].
///
/// Stores start, span, and end as plain optionals. Zero is not valid for any
/// field — use `None` instead.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyGridPlacement {
    /// Grid line at which the item should start (1-indexed, negative counts from end).
    pub start: Option<i16>,
    /// Number of grid tracks to span.
    pub span: Option<u16>,
    /// Grid line at which the item should end (1-indexed, negative counts from end).
    pub end: Option<i16>,
}

impl From<bevy::ui::GridPlacement> for BevyGridPlacement {
    fn from(v: bevy::ui::GridPlacement) -> Self {
        Self {
            start: v.get_start(),
            span: v.get_span(),
            end: v.get_end(),
        }
    }
}

impl From<BevyGridPlacement> for bevy::ui::GridPlacement {
    fn from(v: BevyGridPlacement) -> Self {
        let mut p = Self::default();
        if let Some(start) = v.start {
            p = p.set_start(start);
        }
        if let Some(span) = v.span {
            p = p.set_span(span);
        }
        if let Some(end) = v.end {
            p = p.set_end(end);
        }
        p
    }
}

crate::default_style!(BevyGridPlacement => BevyGridPlacementStyle);

impl Prompt for BevyGridPlacement {
    fn prompt() -> Option<&'static str> {
        Some("Grid placement (start, span, end):")
    }
}

impl Elicitation for BevyGridPlacement {
    type Style = BevyGridPlacementStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let start = Option::<i16>::elicit(communicator).await?;
        let span = Option::<u16>::elicit(communicator).await?;
        let end = Option::<i16>::elicit(communicator).await?;
        Ok(Self { start, span, end })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <i16 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <i16 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <i16 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyGridPlacement {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::GridPlacement",
            description: Some("Grid item placement on a single axis (start, span, end)"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "start",
                        type_name: "Option<i16>",
                        prompt: Some("Start grid line (optional):"),
                    },
                    FieldInfo {
                        name: "span",
                        type_name: "Option<u16>",
                        prompt: Some("Track span (optional):"),
                    },
                    FieldInfo {
                        name: "end",
                        type_name: "Option<i16>",
                        prompt: Some("End grid line (optional):"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyGridPlacement {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Grid placement:".to_string()),
            type_name: "bevy::ui::GridPlacement".to_string(),
            fields: vec![
                ("start".to_string(), Box::new(<Option<i16>>::prompt_tree())),
                ("span".to_string(), Box::new(<Option<u16>>::prompt_tree())),
                ("end".to_string(), Box::new(<Option<i16>>::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGridPlacement {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match (self.start, self.span, self.end) {
            (Some(start), Some(span), None) => {
                quote::quote! { bevy::ui::GridPlacement::start_span(#start, #span) }
            }
            (None, Some(span), Some(end)) => {
                quote::quote! { bevy::ui::GridPlacement::end_span(#end, #span) }
            }
            (Some(start), None, Some(end)) => {
                quote::quote! { bevy::ui::GridPlacement::start_end(#start, #end) }
            }
            (Some(start), None, None) => {
                quote::quote! { bevy::ui::GridPlacement::start(#start) }
            }
            (None, None, Some(end)) => {
                quote::quote! { bevy::ui::GridPlacement::end(#end) }
            }
            (None, Some(span), None) => {
                quote::quote! { bevy::ui::GridPlacement::span(#span) }
            }
            _ => quote::quote! { bevy::ui::GridPlacement::auto() },
        }
    }
}

// ── BevyNode ──────────────────────────────────────────────────────────────────

/// Owned survey trenchcoat for [`bevy::ui::Node`].
///
/// Covers all 42 layout fields. Use `From<bevy::ui::Node>` / `Into<bevy::ui::Node>`
/// for conversion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyNode {
    /// Layout algorithm to use.
    pub display: BevyDisplay,
    /// Box sizing model.
    pub box_sizing: BevyBoxSizing,
    /// Positioning strategy.
    pub position_type: BevyPositionType,
    /// Overflow behavior on x/y axes.
    pub overflow: BevyOverflow,
    /// Scrollbar width in logical pixels.
    pub scrollbar_width: f32,
    /// Overflow clip boundary.
    pub overflow_clip_margin: BevyOverflowClipMargin,
    /// Left edge offset.
    pub left: BevyVal,
    /// Right edge offset.
    pub right: BevyVal,
    /// Top edge offset.
    pub top: BevyVal,
    /// Bottom edge offset.
    pub bottom: BevyVal,
    /// Preferred width.
    pub width: BevyVal,
    /// Preferred height.
    pub height: BevyVal,
    /// Minimum width.
    pub min_width: BevyVal,
    /// Minimum height.
    pub min_height: BevyVal,
    /// Maximum width.
    pub max_width: BevyVal,
    /// Maximum height.
    pub max_height: BevyVal,
    /// Aspect ratio (width / height).
    pub aspect_ratio: Option<f32>,
    /// Cross-axis alignment of children.
    pub align_items: BevyAlignItems,
    /// Inline-axis alignment of children (grid only).
    pub justify_items: BevyJustifyItems,
    /// Cross-axis alignment of this item.
    pub align_self: BevyAlignSelf,
    /// Inline-axis alignment of this item.
    pub justify_self: BevyJustifySelf,
    /// Multi-line cross-axis content alignment.
    pub align_content: BevyAlignContent,
    /// Main-axis content justification.
    pub justify_content: BevyJustifyContent,
    /// Inline text direction.
    pub direction: BevyInlineDirection,
    /// Margin outside the border.
    pub margin: BevyUiRect,
    /// Padding inside the border.
    pub padding: BevyUiRect,
    /// Border widths.
    pub border: BevyUiRect,
    /// Corner radii.
    pub border_radius: BevyBorderRadius,
    /// Flex main-axis direction.
    pub flex_direction: BevyFlexDirection,
    /// Flex wrapping.
    pub flex_wrap: BevyFlexWrap,
    /// Flex grow factor.
    pub flex_grow: f32,
    /// Flex shrink factor.
    pub flex_shrink: f32,
    /// Flex basis.
    pub flex_basis: BevyVal,
    /// Gap between rows.
    pub row_gap: BevyVal,
    /// Gap between columns.
    pub column_gap: BevyVal,
    /// Grid auto-placement flow.
    pub grid_auto_flow: BevyGridAutoFlow,
    /// Explicitly sized grid rows.
    pub grid_template_rows: Vec<BevyRepeatedGridTrack>,
    /// Explicitly sized grid columns.
    pub grid_template_columns: Vec<BevyRepeatedGridTrack>,
    /// Implicitly created row sizes.
    pub grid_auto_rows: Vec<BevyGridTrack>,
    /// Implicitly created column sizes.
    pub grid_auto_columns: Vec<BevyGridTrack>,
    /// Row placement of this grid item.
    pub grid_row: BevyGridPlacement,
    /// Column placement of this grid item.
    pub grid_column: BevyGridPlacement,
}

impl From<bevy::ui::Node> for BevyNode {
    fn from(n: bevy::ui::Node) -> Self {
        Self {
            display: BevyDisplay(n.display),
            box_sizing: BevyBoxSizing(n.box_sizing),
            position_type: BevyPositionType(n.position_type),
            overflow: BevyOverflow::from(n.overflow),
            scrollbar_width: n.scrollbar_width,
            overflow_clip_margin: BevyOverflowClipMargin::from(n.overflow_clip_margin),
            left: BevyVal::from(n.left),
            right: BevyVal::from(n.right),
            top: BevyVal::from(n.top),
            bottom: BevyVal::from(n.bottom),
            width: BevyVal::from(n.width),
            height: BevyVal::from(n.height),
            min_width: BevyVal::from(n.min_width),
            min_height: BevyVal::from(n.min_height),
            max_width: BevyVal::from(n.max_width),
            max_height: BevyVal::from(n.max_height),
            aspect_ratio: n.aspect_ratio,
            align_items: BevyAlignItems(n.align_items),
            justify_items: BevyJustifyItems(n.justify_items),
            align_self: BevyAlignSelf(n.align_self),
            justify_self: BevyJustifySelf(n.justify_self),
            align_content: BevyAlignContent(n.align_content),
            justify_content: BevyJustifyContent(n.justify_content),
            direction: BevyInlineDirection(n.direction),
            margin: BevyUiRect::from(n.margin),
            padding: BevyUiRect::from(n.padding),
            border: BevyUiRect::from(n.border),
            border_radius: BevyBorderRadius::from(n.border_radius),
            flex_direction: BevyFlexDirection(n.flex_direction),
            flex_wrap: BevyFlexWrap(n.flex_wrap),
            flex_grow: n.flex_grow,
            flex_shrink: n.flex_shrink,
            flex_basis: BevyVal::from(n.flex_basis),
            row_gap: BevyVal::from(n.row_gap),
            column_gap: BevyVal::from(n.column_gap),
            grid_auto_flow: BevyGridAutoFlow(n.grid_auto_flow),
            // bevy::ui::GridTrack / RepeatedGridTrack have private fields with no
            // public getters, so grid template/auto fields cannot be reconstructed.
            grid_template_rows: vec![],
            grid_template_columns: vec![],
            grid_auto_rows: vec![],
            grid_auto_columns: vec![],
            grid_row: BevyGridPlacement::from(n.grid_row),
            grid_column: BevyGridPlacement::from(n.grid_column),
        }
    }
}

impl From<BevyNode> for bevy::ui::Node {
    fn from(b: BevyNode) -> Self {
        Self {
            display: b.display.into_inner(),
            box_sizing: b.box_sizing.into_inner(),
            position_type: b.position_type.into_inner(),
            overflow: b.overflow.into(),
            scrollbar_width: b.scrollbar_width,
            overflow_clip_margin: b.overflow_clip_margin.into(),
            left: b.left.into(),
            right: b.right.into(),
            top: b.top.into(),
            bottom: b.bottom.into(),
            width: b.width.into(),
            height: b.height.into(),
            min_width: b.min_width.into(),
            min_height: b.min_height.into(),
            max_width: b.max_width.into(),
            max_height: b.max_height.into(),
            aspect_ratio: b.aspect_ratio,
            align_items: b.align_items.into_inner(),
            justify_items: b.justify_items.into_inner(),
            align_self: b.align_self.into_inner(),
            justify_self: b.justify_self.into_inner(),
            align_content: b.align_content.into_inner(),
            justify_content: b.justify_content.into_inner(),
            direction: b.direction.into_inner(),
            margin: b.margin.into(),
            padding: b.padding.into(),
            border: b.border.into(),
            border_radius: b.border_radius.into(),
            flex_direction: b.flex_direction.into_inner(),
            flex_wrap: b.flex_wrap.into_inner(),
            flex_grow: b.flex_grow,
            flex_shrink: b.flex_shrink,
            flex_basis: b.flex_basis.into(),
            row_gap: b.row_gap.into(),
            column_gap: b.column_gap.into(),
            grid_auto_flow: b.grid_auto_flow.into_inner(),
            grid_template_rows: b.grid_template_rows.into_iter().map(Into::into).collect(),
            grid_template_columns: b
                .grid_template_columns
                .into_iter()
                .map(Into::into)
                .collect(),
            grid_auto_rows: b.grid_auto_rows.into_iter().map(Into::into).collect(),
            grid_auto_columns: b.grid_auto_columns.into_iter().map(Into::into).collect(),
            grid_row: b.grid_row.into(),
            grid_column: b.grid_column.into(),
        }
    }
}

crate::default_style!(BevyNode => BevyNodeStyle);

impl Prompt for BevyNode {
    fn prompt() -> Option<&'static str> {
        Some("UI node layout properties:")
    }
}

impl Elicitation for BevyNode {
    type Style = BevyNodeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let display = BevyDisplay::elicit(communicator).await?;
        let box_sizing = BevyBoxSizing::elicit(communicator).await?;
        let position_type = BevyPositionType::elicit(communicator).await?;
        let overflow = BevyOverflow::elicit(communicator).await?;
        let scrollbar_width = f32::elicit(communicator).await?;
        let overflow_clip_margin = BevyOverflowClipMargin::elicit(communicator).await?;
        let left = BevyVal::elicit(communicator).await?;
        let right = BevyVal::elicit(communicator).await?;
        let top = BevyVal::elicit(communicator).await?;
        let bottom = BevyVal::elicit(communicator).await?;
        let width = BevyVal::elicit(communicator).await?;
        let height = BevyVal::elicit(communicator).await?;
        let min_width = BevyVal::elicit(communicator).await?;
        let min_height = BevyVal::elicit(communicator).await?;
        let max_width = BevyVal::elicit(communicator).await?;
        let max_height = BevyVal::elicit(communicator).await?;
        let aspect_ratio = Option::<f32>::elicit(communicator).await?;
        let align_items = BevyAlignItems::elicit(communicator).await?;
        let justify_items = BevyJustifyItems::elicit(communicator).await?;
        let align_self = BevyAlignSelf::elicit(communicator).await?;
        let justify_self = BevyJustifySelf::elicit(communicator).await?;
        let align_content = BevyAlignContent::elicit(communicator).await?;
        let justify_content = BevyJustifyContent::elicit(communicator).await?;
        let direction = BevyInlineDirection::elicit(communicator).await?;
        let margin = BevyUiRect::elicit(communicator).await?;
        let padding = BevyUiRect::elicit(communicator).await?;
        let border = BevyUiRect::elicit(communicator).await?;
        let border_radius = BevyBorderRadius::elicit(communicator).await?;
        let flex_direction = BevyFlexDirection::elicit(communicator).await?;
        let flex_wrap = BevyFlexWrap::elicit(communicator).await?;
        let flex_grow = f32::elicit(communicator).await?;
        let flex_shrink = f32::elicit(communicator).await?;
        let flex_basis = BevyVal::elicit(communicator).await?;
        let row_gap = BevyVal::elicit(communicator).await?;
        let column_gap = BevyVal::elicit(communicator).await?;
        let grid_auto_flow = BevyGridAutoFlow::elicit(communicator).await?;
        let grid_template_rows = Vec::<BevyRepeatedGridTrack>::elicit(communicator).await?;
        let grid_template_columns = Vec::<BevyRepeatedGridTrack>::elicit(communicator).await?;
        let grid_auto_rows = Vec::<BevyGridTrack>::elicit(communicator).await?;
        let grid_auto_columns = Vec::<BevyGridTrack>::elicit(communicator).await?;
        let grid_row = BevyGridPlacement::elicit(communicator).await?;
        let grid_column = BevyGridPlacement::elicit(communicator).await?;
        Ok(Self {
            display,
            box_sizing,
            position_type,
            overflow,
            scrollbar_width,
            overflow_clip_margin,
            left,
            right,
            top,
            bottom,
            width,
            height,
            min_width,
            min_height,
            max_width,
            max_height,
            aspect_ratio,
            align_items,
            justify_items,
            align_self,
            justify_self,
            align_content,
            justify_content,
            direction,
            margin,
            padding,
            border,
            border_radius,
            flex_direction,
            flex_wrap,
            flex_grow,
            flex_shrink,
            flex_basis,
            row_gap,
            column_gap,
            grid_auto_flow,
            grid_template_rows,
            grid_template_columns,
            grid_auto_rows,
            grid_auto_columns,
            grid_row,
            grid_column,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyNode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::Node",
            description: Some("UI node layout: positioning, sizing, flex, grid"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "display",
                        type_name: "bevy::ui::Display",
                        prompt: Some("Layout algorithm:"),
                    },
                    FieldInfo {
                        name: "box_sizing",
                        type_name: "bevy::ui::BoxSizing",
                        prompt: Some("Box sizing model:"),
                    },
                    FieldInfo {
                        name: "position_type",
                        type_name: "bevy::ui::PositionType",
                        prompt: Some("Positioning type:"),
                    },
                    FieldInfo {
                        name: "overflow",
                        type_name: "bevy::ui::Overflow",
                        prompt: Some("Overflow behavior:"),
                    },
                    FieldInfo {
                        name: "scrollbar_width",
                        type_name: "f32",
                        prompt: Some("Scrollbar width (px):"),
                    },
                    FieldInfo {
                        name: "overflow_clip_margin",
                        type_name: "bevy::ui::OverflowClipMargin",
                        prompt: Some("Overflow clip margin:"),
                    },
                    FieldInfo {
                        name: "left",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Left:"),
                    },
                    FieldInfo {
                        name: "right",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Right:"),
                    },
                    FieldInfo {
                        name: "top",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Top:"),
                    },
                    FieldInfo {
                        name: "bottom",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Bottom:"),
                    },
                    FieldInfo {
                        name: "width",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Width:"),
                    },
                    FieldInfo {
                        name: "height",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Height:"),
                    },
                    FieldInfo {
                        name: "min_width",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Min width:"),
                    },
                    FieldInfo {
                        name: "min_height",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Min height:"),
                    },
                    FieldInfo {
                        name: "max_width",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Max width:"),
                    },
                    FieldInfo {
                        name: "max_height",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Max height:"),
                    },
                    FieldInfo {
                        name: "aspect_ratio",
                        type_name: "Option<f32>",
                        prompt: Some("Aspect ratio:"),
                    },
                    FieldInfo {
                        name: "align_items",
                        type_name: "bevy::ui::AlignItems",
                        prompt: Some("Align items:"),
                    },
                    FieldInfo {
                        name: "justify_items",
                        type_name: "bevy::ui::JustifyItems",
                        prompt: Some("Justify items:"),
                    },
                    FieldInfo {
                        name: "align_self",
                        type_name: "bevy::ui::AlignSelf",
                        prompt: Some("Align self:"),
                    },
                    FieldInfo {
                        name: "justify_self",
                        type_name: "bevy::ui::JustifySelf",
                        prompt: Some("Justify self:"),
                    },
                    FieldInfo {
                        name: "align_content",
                        type_name: "bevy::ui::AlignContent",
                        prompt: Some("Align content:"),
                    },
                    FieldInfo {
                        name: "justify_content",
                        type_name: "bevy::ui::JustifyContent",
                        prompt: Some("Justify content:"),
                    },
                    FieldInfo {
                        name: "direction",
                        type_name: "bevy::ui::InlineDirection",
                        prompt: Some("Text direction:"),
                    },
                    FieldInfo {
                        name: "margin",
                        type_name: "bevy::ui::UiRect",
                        prompt: Some("Margin:"),
                    },
                    FieldInfo {
                        name: "padding",
                        type_name: "bevy::ui::UiRect",
                        prompt: Some("Padding:"),
                    },
                    FieldInfo {
                        name: "border",
                        type_name: "bevy::ui::UiRect",
                        prompt: Some("Border:"),
                    },
                    FieldInfo {
                        name: "border_radius",
                        type_name: "bevy::ui::BorderRadius",
                        prompt: Some("Border radius:"),
                    },
                    FieldInfo {
                        name: "flex_direction",
                        type_name: "bevy::ui::FlexDirection",
                        prompt: Some("Flex direction:"),
                    },
                    FieldInfo {
                        name: "flex_wrap",
                        type_name: "bevy::ui::FlexWrap",
                        prompt: Some("Flex wrap:"),
                    },
                    FieldInfo {
                        name: "flex_grow",
                        type_name: "f32",
                        prompt: Some("Flex grow:"),
                    },
                    FieldInfo {
                        name: "flex_shrink",
                        type_name: "f32",
                        prompt: Some("Flex shrink:"),
                    },
                    FieldInfo {
                        name: "flex_basis",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Flex basis:"),
                    },
                    FieldInfo {
                        name: "row_gap",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Row gap:"),
                    },
                    FieldInfo {
                        name: "column_gap",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Column gap:"),
                    },
                    FieldInfo {
                        name: "grid_auto_flow",
                        type_name: "bevy::ui::GridAutoFlow",
                        prompt: Some("Grid auto flow:"),
                    },
                    FieldInfo {
                        name: "grid_template_rows",
                        type_name: "Vec<bevy::ui::RepeatedGridTrack>",
                        prompt: Some("Grid template rows:"),
                    },
                    FieldInfo {
                        name: "grid_template_columns",
                        type_name: "Vec<bevy::ui::RepeatedGridTrack>",
                        prompt: Some("Grid template columns:"),
                    },
                    FieldInfo {
                        name: "grid_auto_rows",
                        type_name: "Vec<bevy::ui::GridTrack>",
                        prompt: Some("Grid auto rows:"),
                    },
                    FieldInfo {
                        name: "grid_auto_columns",
                        type_name: "Vec<bevy::ui::GridTrack>",
                        prompt: Some("Grid auto columns:"),
                    },
                    FieldInfo {
                        name: "grid_row",
                        type_name: "bevy::ui::GridPlacement",
                        prompt: Some("Grid row placement:"),
                    },
                    FieldInfo {
                        name: "grid_column",
                        type_name: "bevy::ui::GridPlacement",
                        prompt: Some("Grid column placement:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyNode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("UI node:".to_string()),
            type_name: "bevy::ui::Node".to_string(),
            fields: vec![
                ("display".to_string(), Box::new(BevyDisplay::prompt_tree())),
                (
                    "box_sizing".to_string(),
                    Box::new(BevyBoxSizing::prompt_tree()),
                ),
                (
                    "position_type".to_string(),
                    Box::new(BevyPositionType::prompt_tree()),
                ),
                (
                    "overflow".to_string(),
                    Box::new(BevyOverflow::prompt_tree()),
                ),
                ("scrollbar_width".to_string(), Box::new(f32::prompt_tree())),
                (
                    "overflow_clip_margin".to_string(),
                    Box::new(BevyOverflowClipMargin::prompt_tree()),
                ),
                ("left".to_string(), Box::new(BevyVal::prompt_tree())),
                ("right".to_string(), Box::new(BevyVal::prompt_tree())),
                ("top".to_string(), Box::new(BevyVal::prompt_tree())),
                ("bottom".to_string(), Box::new(BevyVal::prompt_tree())),
                ("width".to_string(), Box::new(BevyVal::prompt_tree())),
                ("height".to_string(), Box::new(BevyVal::prompt_tree())),
                ("min_width".to_string(), Box::new(BevyVal::prompt_tree())),
                ("min_height".to_string(), Box::new(BevyVal::prompt_tree())),
                ("max_width".to_string(), Box::new(BevyVal::prompt_tree())),
                ("max_height".to_string(), Box::new(BevyVal::prompt_tree())),
                (
                    "aspect_ratio".to_string(),
                    Box::new(<Option<f32>>::prompt_tree()),
                ),
                (
                    "align_items".to_string(),
                    Box::new(BevyAlignItems::prompt_tree()),
                ),
                (
                    "justify_items".to_string(),
                    Box::new(BevyJustifyItems::prompt_tree()),
                ),
                (
                    "align_self".to_string(),
                    Box::new(BevyAlignSelf::prompt_tree()),
                ),
                (
                    "justify_self".to_string(),
                    Box::new(BevyJustifySelf::prompt_tree()),
                ),
                (
                    "align_content".to_string(),
                    Box::new(BevyAlignContent::prompt_tree()),
                ),
                (
                    "justify_content".to_string(),
                    Box::new(BevyJustifyContent::prompt_tree()),
                ),
                (
                    "direction".to_string(),
                    Box::new(BevyInlineDirection::prompt_tree()),
                ),
                ("margin".to_string(), Box::new(BevyUiRect::prompt_tree())),
                ("padding".to_string(), Box::new(BevyUiRect::prompt_tree())),
                ("border".to_string(), Box::new(BevyUiRect::prompt_tree())),
                (
                    "border_radius".to_string(),
                    Box::new(BevyBorderRadius::prompt_tree()),
                ),
                (
                    "flex_direction".to_string(),
                    Box::new(BevyFlexDirection::prompt_tree()),
                ),
                (
                    "flex_wrap".to_string(),
                    Box::new(BevyFlexWrap::prompt_tree()),
                ),
                ("flex_grow".to_string(), Box::new(f32::prompt_tree())),
                ("flex_shrink".to_string(), Box::new(f32::prompt_tree())),
                ("flex_basis".to_string(), Box::new(BevyVal::prompt_tree())),
                ("row_gap".to_string(), Box::new(BevyVal::prompt_tree())),
                ("column_gap".to_string(), Box::new(BevyVal::prompt_tree())),
                (
                    "grid_auto_flow".to_string(),
                    Box::new(BevyGridAutoFlow::prompt_tree()),
                ),
                (
                    "grid_template_rows".to_string(),
                    Box::new(<Vec<BevyRepeatedGridTrack>>::prompt_tree()),
                ),
                (
                    "grid_template_columns".to_string(),
                    Box::new(<Vec<BevyRepeatedGridTrack>>::prompt_tree()),
                ),
                (
                    "grid_auto_rows".to_string(),
                    Box::new(<Vec<BevyGridTrack>>::prompt_tree()),
                ),
                (
                    "grid_auto_columns".to_string(),
                    Box::new(<Vec<BevyGridTrack>>::prompt_tree()),
                ),
                (
                    "grid_row".to_string(),
                    Box::new(BevyGridPlacement::prompt_tree()),
                ),
                (
                    "grid_column".to_string(),
                    Box::new(BevyGridPlacement::prompt_tree()),
                ),
            ],
        }
    }
}

// ── BevyShadowStyle ───────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::ShadowStyle`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyShadowStyle {
    /// Shadow color.
    pub color: BevyColor,
    /// Horizontal offset.
    pub x_offset: BevyVal,
    /// Vertical offset.
    pub y_offset: BevyVal,
    /// Outward spread radius.
    pub spread_radius: BevyVal,
    /// Blur radius.
    pub blur_radius: BevyVal,
}

impl From<bevy::ui::ShadowStyle> for BevyShadowStyle {
    fn from(s: bevy::ui::ShadowStyle) -> Self {
        Self {
            color: BevyColor::from(s.color),
            x_offset: BevyVal::from(s.x_offset),
            y_offset: BevyVal::from(s.y_offset),
            spread_radius: BevyVal::from(s.spread_radius),
            blur_radius: BevyVal::from(s.blur_radius),
        }
    }
}

impl From<BevyShadowStyle> for bevy::ui::ShadowStyle {
    fn from(b: BevyShadowStyle) -> Self {
        Self {
            color: b.color.into(),
            x_offset: b.x_offset.into(),
            y_offset: b.y_offset.into(),
            spread_radius: b.spread_radius.into(),
            blur_radius: b.blur_radius.into(),
        }
    }
}

crate::default_style!(BevyShadowStyle => BevyShadowStyleStyle);

impl Prompt for BevyShadowStyle {
    fn prompt() -> Option<&'static str> {
        Some("Drop shadow style (color, offsets, spread, blur):")
    }
}

impl Elicitation for BevyShadowStyle {
    type Style = BevyShadowStyleStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let color = BevyColor::elicit(communicator).await?;
        let x_offset = BevyVal::elicit(communicator).await?;
        let y_offset = BevyVal::elicit(communicator).await?;
        let spread_radius = BevyVal::elicit(communicator).await?;
        let blur_radius = BevyVal::elicit(communicator).await?;
        Ok(Self {
            color,
            x_offset,
            y_offset,
            spread_radius,
            blur_radius,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyShadowStyle {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::ShadowStyle",
            description: Some("CSS-style drop shadow"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "color",
                        type_name: "bevy::color::Color",
                        prompt: Some("Shadow color:"),
                    },
                    FieldInfo {
                        name: "x_offset",
                        type_name: "bevy::ui::Val",
                        prompt: Some("X offset:"),
                    },
                    FieldInfo {
                        name: "y_offset",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Y offset:"),
                    },
                    FieldInfo {
                        name: "spread_radius",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Spread radius:"),
                    },
                    FieldInfo {
                        name: "blur_radius",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Blur radius:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyShadowStyle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Shadow style:".to_string()),
            type_name: "bevy::ui::ShadowStyle".to_string(),
            fields: vec![
                ("color".to_string(), Box::new(BevyColor::prompt_tree())),
                ("x_offset".to_string(), Box::new(BevyVal::prompt_tree())),
                ("y_offset".to_string(), Box::new(BevyVal::prompt_tree())),
                (
                    "spread_radius".to_string(),
                    Box::new(BevyVal::prompt_tree()),
                ),
                ("blur_radius".to_string(), Box::new(BevyVal::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyShadowStyle {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let color = self.color.to_code_literal();
        let x_offset = self.x_offset.to_code_literal();
        let y_offset = self.y_offset.to_code_literal();
        let spread_radius = self.spread_radius.to_code_literal();
        let blur_radius = self.blur_radius.to_code_literal();
        quote::quote! {
            bevy::ui::ShadowStyle { color: #color, x_offset: #x_offset, y_offset: #y_offset, spread_radius: #spread_radius, blur_radius: #blur_radius }
        }
    }
}

// ── BevyBoxShadow ─────────────────────────────────────────────────────────────

/// Owned trenchcoat for [`bevy::ui::BoxShadow`].
///
/// A list of drop shadows applied to a UI node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyBoxShadow(pub Vec<BevyShadowStyle>);

impl From<bevy::ui::BoxShadow> for BevyBoxShadow {
    fn from(b: bevy::ui::BoxShadow) -> Self {
        Self(b.0.into_iter().map(BevyShadowStyle::from).collect())
    }
}

impl From<BevyBoxShadow> for bevy::ui::BoxShadow {
    fn from(b: BevyBoxShadow) -> Self {
        Self(b.0.into_iter().map(Into::into).collect())
    }
}

crate::default_style!(BevyBoxShadow => BevyBoxShadowStyle);

impl Prompt for BevyBoxShadow {
    fn prompt() -> Option<&'static str> {
        Some("Box shadows (list of shadow styles):")
    }
}

impl Elicitation for BevyBoxShadow {
    type Style = BevyBoxShadowStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let shadows = Vec::<BevyShadowStyle>::elicit(communicator).await?;
        Ok(Self(shadows))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyShadowStyle as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyShadowStyle as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyShadowStyle as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyBoxShadow {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::BoxShadow",
            description: Some("List of drop shadows for a UI node"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "Vec<bevy::ui::ShadowStyle>",
                    prompt: Some("Shadows:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyBoxShadow {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Box shadow:".to_string()),
            type_name: "bevy::ui::BoxShadow".to_string(),
            fields: vec![(
                "0".to_string(),
                Box::new(<Vec<BevyShadowStyle>>::prompt_tree()),
            )],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyBoxShadow {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let shadows = self.0.to_code_literal();
        quote::quote! { bevy::ui::BoxShadow(#shadows) }
    }
}

// ── BevyZIndex ────────────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::ZIndex`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BevyZIndex(pub i32);

impl From<bevy::ui::ZIndex> for BevyZIndex {
    fn from(z: bevy::ui::ZIndex) -> Self {
        Self(z.0)
    }
}
impl From<BevyZIndex> for bevy::ui::ZIndex {
    fn from(z: BevyZIndex) -> Self {
        Self(z.0)
    }
}

crate::default_style!(BevyZIndex => BevyZIndexStyle);

impl Prompt for BevyZIndex {
    fn prompt() -> Option<&'static str> {
        Some("Z-index (local stacking order):")
    }
}

impl Elicitation for BevyZIndex {
    type Style = BevyZIndexStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self(i32::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <i32 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <i32 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <i32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyZIndex {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::ZIndex",
            description: Some("Local z-order within siblings"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "i32",
                    prompt: Some("Z value:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyZIndex {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::ui::ZIndex".to_string(),
            fields: vec![("0".to_string(), Box::new(i32::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyZIndex {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v = self.0;
        quote::quote! { bevy::ui::ZIndex(#v) }
    }
}

// ── BevyGlobalZIndex ──────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::GlobalZIndex`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BevyGlobalZIndex(pub i32);

impl From<bevy::ui::GlobalZIndex> for BevyGlobalZIndex {
    fn from(z: bevy::ui::GlobalZIndex) -> Self {
        Self(z.0)
    }
}
impl From<BevyGlobalZIndex> for bevy::ui::GlobalZIndex {
    fn from(z: BevyGlobalZIndex) -> Self {
        Self(z.0)
    }
}

crate::default_style!(BevyGlobalZIndex => BevyGlobalZIndexStyle);

impl Prompt for BevyGlobalZIndex {
    fn prompt() -> Option<&'static str> {
        Some("Global Z-index (absolute stacking order):")
    }
}

impl Elicitation for BevyGlobalZIndex {
    type Style = BevyGlobalZIndexStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self(i32::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <i32 as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <i32 as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <i32 as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyGlobalZIndex {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::GlobalZIndex",
            description: Some("Absolute z-order across the entire UI"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "i32",
                    prompt: Some("Z value:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyGlobalZIndex {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::ui::GlobalZIndex".to_string(),
            fields: vec![("0".to_string(), Box::new(i32::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyGlobalZIndex {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v = self.0;
        quote::quote! { bevy::ui::GlobalZIndex(#v) }
    }
}

// ── BevyScrollPosition ────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::ScrollPosition`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyScrollPosition {
    /// Horizontal scroll offset.
    pub x: f32,
    /// Vertical scroll offset.
    pub y: f32,
}

impl From<bevy::ui::ScrollPosition> for BevyScrollPosition {
    fn from(s: bevy::ui::ScrollPosition) -> Self {
        Self { x: s.0.x, y: s.0.y }
    }
}

impl From<BevyScrollPosition> for bevy::ui::ScrollPosition {
    fn from(b: BevyScrollPosition) -> Self {
        Self(bevy::math::Vec2::new(b.x, b.y))
    }
}

crate::default_style!(BevyScrollPosition => BevyScrollPositionStyle);

impl Prompt for BevyScrollPosition {
    fn prompt() -> Option<&'static str> {
        Some("Scroll position (x, y):")
    }
}

impl Elicitation for BevyScrollPosition {
    type Style = BevyScrollPositionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let x = f32::elicit(communicator).await?;
        let y = f32::elicit(communicator).await?;
        Ok(Self { x, y })
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

impl ElicitIntrospect for BevyScrollPosition {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::ScrollPosition",
            description: Some("Scroll offset in logical pixels"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "x",
                        type_name: "f32",
                        prompt: Some("X offset:"),
                    },
                    FieldInfo {
                        name: "y",
                        type_name: "f32",
                        prompt: Some("Y offset:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyScrollPosition {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::ui::ScrollPosition".to_string(),
            fields: vec![
                ("x".to_string(), Box::new(f32::prompt_tree())),
                ("y".to_string(), Box::new(f32::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyScrollPosition {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let x = self.x;
        let y = self.y;
        quote::quote! { bevy::ui::ScrollPosition(bevy::math::Vec2::new(#x, #y)) }
    }
}

// ── BevyOutline ───────────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::Outline`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyOutline {
    /// Outline width.
    pub width: BevyVal,
    /// Gap between node edge and outline.
    pub offset: BevyVal,
    /// Outline color.
    pub color: BevyColor,
}

impl From<bevy::ui::Outline> for BevyOutline {
    fn from(o: bevy::ui::Outline) -> Self {
        Self {
            width: BevyVal::from(o.width),
            offset: BevyVal::from(o.offset),
            color: BevyColor::from(o.color),
        }
    }
}

impl From<BevyOutline> for bevy::ui::Outline {
    fn from(b: BevyOutline) -> Self {
        Self {
            width: b.width.into(),
            offset: b.offset.into(),
            color: b.color.into(),
        }
    }
}

crate::default_style!(BevyOutline => BevyOutlineStyle);

impl Prompt for BevyOutline {
    fn prompt() -> Option<&'static str> {
        Some("Outline (width, offset, color):")
    }
}

impl Elicitation for BevyOutline {
    type Style = BevyOutlineStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let width = BevyVal::elicit(communicator).await?;
        let offset = BevyVal::elicit(communicator).await?;
        let color = BevyColor::elicit(communicator).await?;
        Ok(Self {
            width,
            offset,
            color,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyVal as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyOutline {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::Outline",
            description: Some("Node outline (drawn outside the border)"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "width",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Width:"),
                    },
                    FieldInfo {
                        name: "offset",
                        type_name: "bevy::ui::Val",
                        prompt: Some("Offset:"),
                    },
                    FieldInfo {
                        name: "color",
                        type_name: "bevy::color::Color",
                        prompt: Some("Color:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyOutline {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Outline:".to_string()),
            type_name: "bevy::ui::Outline".to_string(),
            fields: vec![
                ("width".to_string(), Box::new(BevyVal::prompt_tree())),
                ("offset".to_string(), Box::new(BevyVal::prompt_tree())),
                ("color".to_string(), Box::new(BevyColor::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyOutline {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let width = self.width.to_code_literal();
        let offset = self.offset.to_code_literal();
        let color = self.color.to_code_literal();
        quote::quote! { bevy::ui::Outline { width: #width, offset: #offset, color: #color } }
    }
}

// ── BevyBackgroundColor ───────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::BackgroundColor`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyBackgroundColor(pub BevyColor);

impl From<bevy::ui::BackgroundColor> for BevyBackgroundColor {
    fn from(b: bevy::ui::BackgroundColor) -> Self {
        Self(BevyColor::from(b.0))
    }
}
impl From<BevyBackgroundColor> for bevy::ui::BackgroundColor {
    fn from(b: BevyBackgroundColor) -> Self {
        Self(b.0.into())
    }
}

crate::default_style!(BevyBackgroundColor => BevyBackgroundColorStyle);

impl Prompt for BevyBackgroundColor {
    fn prompt() -> Option<&'static str> {
        Some("Background color:")
    }
}

impl Elicitation for BevyBackgroundColor {
    type Style = BevyBackgroundColorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        Ok(Self(BevyColor::elicit(communicator).await?))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyColor as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyColor as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyColor as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyBackgroundColor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::BackgroundColor",
            description: Some("Fill color of a UI node"),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "0",
                    type_name: "bevy::color::Color",
                    prompt: Some("Color:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyBackgroundColor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "bevy::ui::BackgroundColor".to_string(),
            fields: vec![("0".to_string(), Box::new(BevyColor::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyBackgroundColor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let color = self.0.to_code_literal();
        quote::quote! { bevy::ui::BackgroundColor(#color) }
    }
}

// ── BevyBorderColor ───────────────────────────────────────────────────────────

/// Trenchcoat for [`bevy::ui::BorderColor`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BevyBorderColor {
    /// Top border color.
    pub top: BevyColor,
    /// Right border color.
    pub right: BevyColor,
    /// Bottom border color.
    pub bottom: BevyColor,
    /// Left border color.
    pub left: BevyColor,
}

impl From<bevy::ui::BorderColor> for BevyBorderColor {
    fn from(b: bevy::ui::BorderColor) -> Self {
        Self {
            top: BevyColor::from(b.top),
            right: BevyColor::from(b.right),
            bottom: BevyColor::from(b.bottom),
            left: BevyColor::from(b.left),
        }
    }
}

impl From<BevyBorderColor> for bevy::ui::BorderColor {
    fn from(b: BevyBorderColor) -> Self {
        Self {
            top: b.top.into(),
            right: b.right.into(),
            bottom: b.bottom.into(),
            left: b.left.into(),
        }
    }
}

crate::default_style!(BevyBorderColor => BevyBorderColorStyle);

impl Prompt for BevyBorderColor {
    fn prompt() -> Option<&'static str> {
        Some("Border colors (top, right, bottom, left):")
    }
}

impl Elicitation for BevyBorderColor {
    type Style = BevyBorderColorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let top = BevyColor::elicit(communicator).await?;
        let right = BevyColor::elicit(communicator).await?;
        let bottom = BevyColor::elicit(communicator).await?;
        let left = BevyColor::elicit(communicator).await?;
        Ok(Self {
            top,
            right,
            bottom,
            left,
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        <BevyColor as Elicitation>::kani_proof()
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        <BevyColor as Elicitation>::verus_proof()
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        <BevyColor as Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for BevyBorderColor {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "bevy::ui::BorderColor",
            description: Some("Per-edge border colors"),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "top",
                        type_name: "bevy::color::Color",
                        prompt: Some("Top:"),
                    },
                    FieldInfo {
                        name: "right",
                        type_name: "bevy::color::Color",
                        prompt: Some("Right:"),
                    },
                    FieldInfo {
                        name: "bottom",
                        type_name: "bevy::color::Color",
                        prompt: Some("Bottom:"),
                    },
                    FieldInfo {
                        name: "left",
                        type_name: "bevy::color::Color",
                        prompt: Some("Left:"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for BevyBorderColor {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Some("Border color:".to_string()),
            type_name: "bevy::ui::BorderColor".to_string(),
            fields: vec![
                ("top".to_string(), Box::new(BevyColor::prompt_tree())),
                ("right".to_string(), Box::new(BevyColor::prompt_tree())),
                ("bottom".to_string(), Box::new(BevyColor::prompt_tree())),
                ("left".to_string(), Box::new(BevyColor::prompt_tree())),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyBorderColor {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let top = self.top.to_code_literal();
        let right = self.right.to_code_literal();
        let bottom = self.bottom.to_code_literal();
        let left = self.left.to_code_literal();
        quote::quote! {
            bevy::ui::BorderColor { top: #top, right: #right, bottom: #bottom, left: #left }
        }
    }
}

impl crate::emit_code::ToCodeLiteral for BevyNode {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let display = bevy_ui_enum_tok("Display", &self.display.0);
        let box_sizing = bevy_ui_enum_tok("BoxSizing", &self.box_sizing.0);
        let position_type = bevy_ui_enum_tok("PositionType", &self.position_type.0);
        let overflow = self.overflow.to_code_literal();
        let scrollbar_width = self.scrollbar_width;
        let overflow_clip_margin = self.overflow_clip_margin.to_code_literal();
        let left = self.left.to_code_literal();
        let right = self.right.to_code_literal();
        let top = self.top.to_code_literal();
        let bottom = self.bottom.to_code_literal();
        let width = self.width.to_code_literal();
        let height = self.height.to_code_literal();
        let min_width = self.min_width.to_code_literal();
        let min_height = self.min_height.to_code_literal();
        let max_width = self.max_width.to_code_literal();
        let max_height = self.max_height.to_code_literal();
        let aspect_ratio = match self.aspect_ratio {
            Some(v) => quote::quote! { Some(#v) },
            None => quote::quote! { None },
        };
        let align_items = bevy_ui_enum_tok("AlignItems", &self.align_items.0);
        let justify_items = bevy_ui_enum_tok("JustifyItems", &self.justify_items.0);
        let align_self = bevy_ui_enum_tok("AlignSelf", &self.align_self.0);
        let justify_self = bevy_ui_enum_tok("JustifySelf", &self.justify_self.0);
        let align_content = bevy_ui_enum_tok("AlignContent", &self.align_content.0);
        let justify_content = bevy_ui_enum_tok("JustifyContent", &self.justify_content.0);
        let direction = bevy_ui_enum_tok("InlineDirection", &self.direction.0);
        let margin = self.margin.to_code_literal();
        let padding = self.padding.to_code_literal();
        let border = self.border.to_code_literal();
        let border_radius = self.border_radius.to_code_literal();
        let flex_direction = bevy_ui_enum_tok("FlexDirection", &self.flex_direction.0);
        let flex_wrap = bevy_ui_enum_tok("FlexWrap", &self.flex_wrap.0);
        let flex_grow = self.flex_grow;
        let flex_shrink = self.flex_shrink;
        let flex_basis = self.flex_basis.to_code_literal();
        let row_gap = self.row_gap.to_code_literal();
        let column_gap = self.column_gap.to_code_literal();
        let grid_auto_flow = bevy_ui_enum_tok("GridAutoFlow", &self.grid_auto_flow.0);
        let grid_template_rows = self.grid_template_rows.to_code_literal();
        let grid_template_columns = self.grid_template_columns.to_code_literal();
        let grid_auto_rows = self.grid_auto_rows.to_code_literal();
        let grid_auto_columns = self.grid_auto_columns.to_code_literal();
        let grid_row = self.grid_row.to_code_literal();
        let grid_column = self.grid_column.to_code_literal();
        quote::quote! {
            bevy::ui::Node {
                display: #display,
                box_sizing: #box_sizing,
                position_type: #position_type,
                overflow: #overflow,
                scrollbar_width: #scrollbar_width,
                overflow_clip_margin: #overflow_clip_margin,
                left: #left,
                right: #right,
                top: #top,
                bottom: #bottom,
                width: #width,
                height: #height,
                min_width: #min_width,
                min_height: #min_height,
                max_width: #max_width,
                max_height: #max_height,
                aspect_ratio: #aspect_ratio,
                align_items: #align_items,
                justify_items: #justify_items,
                align_self: #align_self,
                justify_self: #justify_self,
                align_content: #align_content,
                justify_content: #justify_content,
                direction: #direction,
                margin: #margin,
                padding: #padding,
                border: #border,
                border_radius: #border_radius,
                flex_direction: #flex_direction,
                flex_wrap: #flex_wrap,
                flex_grow: #flex_grow,
                flex_shrink: #flex_shrink,
                flex_basis: #flex_basis,
                row_gap: #row_gap,
                column_gap: #column_gap,
                grid_auto_flow: #grid_auto_flow,
                grid_template_rows: #grid_template_rows,
                grid_template_columns: #grid_template_columns,
                grid_auto_rows: #grid_auto_rows,
                grid_auto_columns: #grid_auto_columns,
                grid_row: #grid_row,
                grid_column: #grid_column,
            }
        }
    }
}
