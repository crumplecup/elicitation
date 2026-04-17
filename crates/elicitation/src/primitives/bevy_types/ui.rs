//! Bevy UI type elicitation trenchcoats.
//!
//! Covers:
//! - [`BevyVal`] — owned trenchcoat for `bevy::ui::Val`
//! - [`BevyAlignItems`], [`BevyJustifyItems`], [`BevyAlignSelf`], [`BevyJustifySelf`],
//!   [`BevyAlignContent`], [`BevyJustifyContent`], [`BevyDisplay`], [`BevyBoxSizing`],
//!   [`BevyFlexDirection`], [`BevyFlexWrap`], [`BevyPositionType`], [`BevyOverflowAxis`],
//!   [`BevyOverflowClipBox`] — select-trenchcoats for layout enums
//! - [`BevyUiRect`] — survey trenchcoat for `bevy::ui::UiRect`
//! - [`BevyBorderRadius`] — survey trenchcoat for `bevy::ui::BorderRadius`

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, Select, Survey, TypeMetadata,
    VariantMetadata, mcp,
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

// ── OverflowClipBox ───────────────────────────────────────────────────────────

impl_ui_select!(
    type     = bevy::ui::OverflowClipBox,
    style    = BevyOverflowClipBoxStyle,
    prompt   = "Choose the clip boundary box:",
    kani_var = "bevy::ui::OverflowClipBox::PaddingBox",
    variants = [Self::ContentBox, Self::PaddingBox, Self::BorderBox]
);

crate::select_trenchcoat!(bevy::ui::OverflowClipBox, as BevyOverflowClipBox, serde);
crate::select_trenchcoat_traits!(BevyOverflowClipBox, bevy::ui::OverflowClipBox, [copy, eq]);

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
