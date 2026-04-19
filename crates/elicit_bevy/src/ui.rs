//! UI layout and style wrappers.
//!
//! Covers [`Val`], [`UiRect`], [`BorderRadius`], and all CSS-like layout enums,
//! plus focus, overflow clip, grid track repetition, scroll, gradients, shadow,
//! and marker components.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── Val ───────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::Val, as Val, forward_serde);
elicit_newtype_traits!(Val, bevy::ui::Val, [eq]);

impl From<Val> for bevy::ui::Val {
    fn from(v: Val) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Val {
    /// Returns a debug string representation of this value.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        format!("{:?}", *self.0)
    }

    /// Returns `true` if this is [`Val::Auto`].
    #[tracing::instrument(skip(self))]
    pub fn is_auto(&self) -> bool {
        matches!(*self.0, bevy::ui::Val::Auto)
    }

    /// Returns `true` if this is [`Val::Px`].
    #[tracing::instrument(skip(self))]
    pub fn is_px(&self) -> bool {
        matches!(*self.0, bevy::ui::Val::Px(_))
    }

    /// Returns the pixel value if this is [`Val::Px`].
    #[tracing::instrument(skip(self))]
    pub fn get_px(&self) -> Option<f32> {
        if let bevy::ui::Val::Px(v) = *self.0 {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the percent value if this is [`Val::Percent`].
    #[tracing::instrument(skip(self))]
    pub fn get_percent(&self) -> Option<f32> {
        if let bevy::ui::Val::Percent(v) = *self.0 {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the vw value if this is [`Val::Vw`].
    #[tracing::instrument(skip(self))]
    pub fn get_vw(&self) -> Option<f32> {
        if let bevy::ui::Val::Vw(v) = *self.0 {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the vh value if this is [`Val::Vh`].
    #[tracing::instrument(skip(self))]
    pub fn get_vh(&self) -> Option<f32> {
        if let bevy::ui::Val::Vh(v) = *self.0 {
            Some(v)
        } else {
            None
        }
    }

    /// Constructs [`Val::Px`] (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn px(&self, v: f32) -> Val {
        Val::from(bevy::ui::Val::Px(v))
    }

    /// Constructs [`Val::Percent`] (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn percent(&self, v: f32) -> Val {
        Val::from(bevy::ui::Val::Percent(v))
    }

    /// Constructs [`Val::Vw`] (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vw(&self, v: f32) -> Val {
        Val::from(bevy::ui::Val::Vw(v))
    }

    /// Constructs [`Val::Vh`] (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vh(&self, v: f32) -> Val {
        Val::from(bevy::ui::Val::Vh(v))
    }

    /// Constructs [`Val::Auto`] (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn auto_val(&self) -> Val {
        Val::from(bevy::ui::Val::Auto)
    }
}

mod emit_impls_val {
    use super::Val;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Val {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::ui::Val::Auto => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::Auto)
                },
                bevy::ui::Val::Px(v) => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::Px(#v))
                },
                bevy::ui::Val::Percent(v) => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::Percent(#v))
                },
                bevy::ui::Val::Vw(v) => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::Vw(#v))
                },
                bevy::ui::Val::Vh(v) => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::Vh(#v))
                },
                bevy::ui::Val::VMin(v) => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::VMin(#v))
                },
                bevy::ui::Val::VMax(v) => quote::quote! {
                    ::elicit_bevy::Val::from(::bevy::ui::Val::VMax(#v))
                },
            }
        }
    }
}

impl elicitation::ElicitComplete for Val {}

// ── UiRect ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::UiRect, as UiRect, forward_serde);
elicit_newtype_traits!(UiRect, bevy::ui::UiRect, [eq]);

impl From<UiRect> for bevy::ui::UiRect {
    fn from(v: UiRect) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl UiRect {
    /// Returns the left value.
    #[tracing::instrument(skip(self))]
    pub fn left(&self) -> Val {
        Val::from(self.0.left)
    }

    /// Returns the right value.
    #[tracing::instrument(skip(self))]
    pub fn right(&self) -> Val {
        Val::from(self.0.right)
    }

    /// Returns the top value.
    #[tracing::instrument(skip(self))]
    pub fn top(&self) -> Val {
        Val::from(self.0.top)
    }

    /// Returns the bottom value.
    #[tracing::instrument(skip(self))]
    pub fn bottom(&self) -> Val {
        Val::from(self.0.bottom)
    }

    /// Constructs a [`UiRect`] with all sides set to the same value (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn ui_rect_all(&self, v: Val) -> UiRect {
        UiRect::from(bevy::ui::UiRect::all(*v.0))
    }

    /// Constructs a [`UiRect`] with left and right set, top and bottom zero (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn horizontal(&self, h: Val) -> UiRect {
        UiRect::from(bevy::ui::UiRect::horizontal(*h.0))
    }

    /// Constructs a [`UiRect`] with top and bottom set, left and right zero (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vertical(&self, v: Val) -> UiRect {
        UiRect::from(bevy::ui::UiRect::vertical(*v.0))
    }

    /// Returns a copy with the given left value.
    #[tracing::instrument(skip(self))]
    pub fn with_left(&self, v: Val) -> UiRect {
        let mut r = *self.0;
        r.left = *v.0;
        UiRect::from(r)
    }

    /// Returns a copy with the given right value.
    #[tracing::instrument(skip(self))]
    pub fn with_right(&self, v: Val) -> UiRect {
        let mut r = *self.0;
        r.right = *v.0;
        UiRect::from(r)
    }

    /// Returns a copy with the given top value.
    #[tracing::instrument(skip(self))]
    pub fn with_top(&self, v: Val) -> UiRect {
        let mut r = *self.0;
        r.top = *v.0;
        UiRect::from(r)
    }

    /// Returns a copy with the given bottom value.
    #[tracing::instrument(skip(self))]
    pub fn with_bottom(&self, v: Val) -> UiRect {
        let mut r = *self.0;
        r.bottom = *v.0;
        UiRect::from(r)
    }
}

mod emit_impls_ui_rect {
    use super::{UiRect, Val};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for UiRect {
        fn to_code_literal(&self) -> TokenStream {
            let left = Val::from(self.0.left).to_code_literal();
            let right = Val::from(self.0.right).to_code_literal();
            let top = Val::from(self.0.top).to_code_literal();
            let bottom = Val::from(self.0.bottom).to_code_literal();
            quote::quote! {
                ::elicit_bevy::UiRect::from(::bevy::ui::UiRect {
                    left: ::bevy::ui::Val::from(#left),
                    right: ::bevy::ui::Val::from(#right),
                    top: ::bevy::ui::Val::from(#top),
                    bottom: ::bevy::ui::Val::from(#bottom),
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for UiRect {}

// ── BorderRadius ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BorderRadius, as BorderRadius, forward_serde);
elicit_newtype_traits!(BorderRadius, bevy::ui::BorderRadius, [eq]);

impl From<BorderRadius> for bevy::ui::BorderRadius {
    fn from(v: BorderRadius) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl BorderRadius {
    /// Returns the top-left radius.
    #[tracing::instrument(skip(self))]
    pub fn top_left(&self) -> Val {
        Val::from(self.0.top_left)
    }

    /// Returns the top-right radius.
    #[tracing::instrument(skip(self))]
    pub fn top_right(&self) -> Val {
        Val::from(self.0.top_right)
    }

    /// Returns the bottom-right radius.
    #[tracing::instrument(skip(self))]
    pub fn bottom_right(&self) -> Val {
        Val::from(self.0.bottom_right)
    }

    /// Returns the bottom-left radius.
    #[tracing::instrument(skip(self))]
    pub fn bottom_left(&self) -> Val {
        Val::from(self.0.bottom_left)
    }

    /// Constructs a [`BorderRadius`] with all corners set to the same value (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn border_radius_all(&self, v: Val) -> BorderRadius {
        BorderRadius::from(bevy::ui::BorderRadius::all(*v.0))
    }

    /// Constructs a zero (sharp) border radius (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn zero(&self) -> BorderRadius {
        BorderRadius::from(bevy::ui::BorderRadius::ZERO)
    }
}

mod emit_impls_border_radius {
    use super::{BorderRadius, Val};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BorderRadius {
        fn to_code_literal(&self) -> TokenStream {
            let tl = Val::from(self.0.top_left).to_code_literal();
            let tr = Val::from(self.0.top_right).to_code_literal();
            let br = Val::from(self.0.bottom_right).to_code_literal();
            let bl = Val::from(self.0.bottom_left).to_code_literal();
            quote::quote! {
                ::elicit_bevy::BorderRadius::from(::bevy::ui::BorderRadius {
                    top_left: ::bevy::ui::Val::from(#tl),
                    top_right: ::bevy::ui::Val::from(#tr),
                    bottom_right: ::bevy::ui::Val::from(#br),
                    bottom_left: ::bevy::ui::Val::from(#bl),
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for BorderRadius {}

// ── Display ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::Display, as Display);
elicit_newtype_traits!(Display, bevy::ui::Display, [eq]);

impl Display {
    fn as_str(&self) -> &'static str {
        self.display_as_str()
    }
}

impl From<Display> for bevy::ui::Display {
    fn from(v: Display) -> Self {
        *v.0
    }
}

impl serde::Serialize for Display {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Display {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Flex" => bevy::ui::Display::Flex,
            "Grid" => bevy::ui::Display::Grid,
            "Block" => bevy::ui::Display::Block,
            "None" => bevy::ui::Display::None,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Flex", "Grid", "Block", "None"],
                ));
            }
        };
        Ok(Display(Arc::new(inner)))
    }
}

#[reflect_methods]
impl Display {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn display_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::Display::Flex => "Flex",
            bevy::ui::Display::Grid => "Grid",
            bevy::ui::Display::Block => "Block",
            bevy::ui::Display::None => "None",
        }
    }

    /// Returns `true` if this is [`Display::Flex`].
    #[tracing::instrument(skip(self))]
    pub fn is_flex(&self) -> bool {
        matches!(*self.0, bevy::ui::Display::Flex)
    }

    /// Returns `true` if this is [`Display::Grid`].
    #[tracing::instrument(skip(self))]
    pub fn is_grid(&self) -> bool {
        matches!(*self.0, bevy::ui::Display::Grid)
    }

    /// Returns `true` if this is [`Display::None`].
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        matches!(*self.0, bevy::ui::Display::None)
    }
}

mod emit_impls_display {
    use super::Display;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Display {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::Display::from(::bevy::ui::Display::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for Display {}

// ── PositionType ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::PositionType, as PositionType);
elicit_newtype_traits!(PositionType, bevy::ui::PositionType, [eq]);

impl PositionType {
    fn as_str(&self) -> &'static str {
        self.position_type_as_str()
    }
}

impl From<PositionType> for bevy::ui::PositionType {
    fn from(v: PositionType) -> Self {
        *v.0
    }
}

impl serde::Serialize for PositionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for PositionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Relative" => bevy::ui::PositionType::Relative,
            "Absolute" => bevy::ui::PositionType::Absolute,
            _ => return Err(D::Error::unknown_variant(&s, &["Relative", "Absolute"])),
        };
        Ok(PositionType(Arc::new(inner)))
    }
}

#[reflect_methods]
impl PositionType {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn position_type_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::PositionType::Relative => "Relative",
            bevy::ui::PositionType::Absolute => "Absolute",
        }
    }

    /// Returns `true` if this is [`PositionType::Relative`].
    #[tracing::instrument(skip(self))]
    pub fn is_relative(&self) -> bool {
        matches!(*self.0, bevy::ui::PositionType::Relative)
    }

    /// Returns `true` if this is [`PositionType::Absolute`].
    #[tracing::instrument(skip(self))]
    pub fn is_absolute(&self) -> bool {
        matches!(*self.0, bevy::ui::PositionType::Absolute)
    }
}

mod emit_impls_position_type {
    use super::PositionType;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for PositionType {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::PositionType::from(::bevy::ui::PositionType::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for PositionType {}

// ── FlexDirection ─────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::FlexDirection, as FlexDirection);
elicit_newtype_traits!(FlexDirection, bevy::ui::FlexDirection, [eq]);

impl FlexDirection {
    fn as_str(&self) -> &'static str {
        self.flex_direction_as_str()
    }
}

impl From<FlexDirection> for bevy::ui::FlexDirection {
    fn from(v: FlexDirection) -> Self {
        *v.0
    }
}

impl serde::Serialize for FlexDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FlexDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Row" => bevy::ui::FlexDirection::Row,
            "Column" => bevy::ui::FlexDirection::Column,
            "RowReverse" => bevy::ui::FlexDirection::RowReverse,
            "ColumnReverse" => bevy::ui::FlexDirection::ColumnReverse,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Row", "Column", "RowReverse", "ColumnReverse"],
                ));
            }
        };
        Ok(FlexDirection(Arc::new(inner)))
    }
}

#[reflect_methods]
impl FlexDirection {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn flex_direction_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::FlexDirection::Row => "Row",
            bevy::ui::FlexDirection::Column => "Column",
            bevy::ui::FlexDirection::RowReverse => "RowReverse",
            bevy::ui::FlexDirection::ColumnReverse => "ColumnReverse",
        }
    }

    /// Returns `true` if this is [`FlexDirection::Row`].
    #[tracing::instrument(skip(self))]
    pub fn is_row(&self) -> bool {
        matches!(*self.0, bevy::ui::FlexDirection::Row)
    }

    /// Returns `true` if this is [`FlexDirection::Column`].
    #[tracing::instrument(skip(self))]
    pub fn is_column(&self) -> bool {
        matches!(*self.0, bevy::ui::FlexDirection::Column)
    }
}

mod emit_impls_flex_dir {
    use super::FlexDirection;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for FlexDirection {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::FlexDirection::from(::bevy::ui::FlexDirection::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for FlexDirection {}

// ── FlexWrap ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::FlexWrap, as FlexWrap);
elicit_newtype_traits!(FlexWrap, bevy::ui::FlexWrap, [eq]);

impl FlexWrap {
    fn as_str(&self) -> &'static str {
        self.flex_wrap_as_str()
    }
}

impl From<FlexWrap> for bevy::ui::FlexWrap {
    fn from(v: FlexWrap) -> Self {
        *v.0
    }
}

impl serde::Serialize for FlexWrap {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for FlexWrap {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "NoWrap" => bevy::ui::FlexWrap::NoWrap,
            "Wrap" => bevy::ui::FlexWrap::Wrap,
            "WrapReverse" => bevy::ui::FlexWrap::WrapReverse,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["NoWrap", "Wrap", "WrapReverse"],
                ));
            }
        };
        Ok(FlexWrap(Arc::new(inner)))
    }
}

#[reflect_methods]
impl FlexWrap {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn flex_wrap_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::FlexWrap::NoWrap => "NoWrap",
            bevy::ui::FlexWrap::Wrap => "Wrap",
            bevy::ui::FlexWrap::WrapReverse => "WrapReverse",
        }
    }

    /// Returns `true` if this is [`FlexWrap::Wrap`] or [`FlexWrap::WrapReverse`].
    #[tracing::instrument(skip(self))]
    pub fn is_wrap(&self) -> bool {
        !matches!(*self.0, bevy::ui::FlexWrap::NoWrap)
    }
}

mod emit_impls_flex_wrap {
    use super::FlexWrap;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for FlexWrap {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::FlexWrap::from(::bevy::ui::FlexWrap::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for FlexWrap {}

// ── JustifyContent ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::JustifyContent, as JustifyContent);
elicit_newtype_traits!(JustifyContent, bevy::ui::JustifyContent, [eq]);

impl JustifyContent {
    fn as_str(&self) -> &'static str {
        self.justify_content_as_str()
    }
}

impl From<JustifyContent> for bevy::ui::JustifyContent {
    fn from(v: JustifyContent) -> Self {
        *v.0
    }
}

impl serde::Serialize for JustifyContent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for JustifyContent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Default" => bevy::ui::JustifyContent::Default,
            "Start" => bevy::ui::JustifyContent::Start,
            "End" => bevy::ui::JustifyContent::End,
            "FlexStart" => bevy::ui::JustifyContent::FlexStart,
            "FlexEnd" => bevy::ui::JustifyContent::FlexEnd,
            "Center" => bevy::ui::JustifyContent::Center,
            "Stretch" => bevy::ui::JustifyContent::Stretch,
            "SpaceBetween" => bevy::ui::JustifyContent::SpaceBetween,
            "SpaceEvenly" => bevy::ui::JustifyContent::SpaceEvenly,
            "SpaceAround" => bevy::ui::JustifyContent::SpaceAround,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &[
                        "Default",
                        "Start",
                        "End",
                        "FlexStart",
                        "FlexEnd",
                        "Center",
                        "Stretch",
                        "SpaceBetween",
                        "SpaceEvenly",
                        "SpaceAround",
                    ],
                ));
            }
        };
        Ok(JustifyContent(Arc::new(inner)))
    }
}

#[reflect_methods]
impl JustifyContent {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn justify_content_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::JustifyContent::Default => "Default",
            bevy::ui::JustifyContent::Start => "Start",
            bevy::ui::JustifyContent::End => "End",
            bevy::ui::JustifyContent::FlexStart => "FlexStart",
            bevy::ui::JustifyContent::FlexEnd => "FlexEnd",
            bevy::ui::JustifyContent::Center => "Center",
            bevy::ui::JustifyContent::Stretch => "Stretch",
            bevy::ui::JustifyContent::SpaceBetween => "SpaceBetween",
            bevy::ui::JustifyContent::SpaceEvenly => "SpaceEvenly",
            bevy::ui::JustifyContent::SpaceAround => "SpaceAround",
        }
    }

    /// Returns `true` if this is [`JustifyContent::Center`].
    #[tracing::instrument(skip(self))]
    pub fn justify_content_is_center(&self) -> bool {
        matches!(*self.0, bevy::ui::JustifyContent::Center)
    }
}

mod emit_impls_justify_content {
    use super::JustifyContent;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for JustifyContent {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::JustifyContent::from(::bevy::ui::JustifyContent::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for JustifyContent {}

// ── AlignItems ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::AlignItems, as AlignItems);
elicit_newtype_traits!(AlignItems, bevy::ui::AlignItems, [eq]);

impl AlignItems {
    fn as_str(&self) -> &'static str {
        self.align_items_as_str()
    }
}

impl From<AlignItems> for bevy::ui::AlignItems {
    fn from(v: AlignItems) -> Self {
        *v.0
    }
}

impl serde::Serialize for AlignItems {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AlignItems {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Default" => bevy::ui::AlignItems::Default,
            "Start" => bevy::ui::AlignItems::Start,
            "End" => bevy::ui::AlignItems::End,
            "FlexStart" => bevy::ui::AlignItems::FlexStart,
            "FlexEnd" => bevy::ui::AlignItems::FlexEnd,
            "Center" => bevy::ui::AlignItems::Center,
            "Baseline" => bevy::ui::AlignItems::Baseline,
            "Stretch" => bevy::ui::AlignItems::Stretch,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &[
                        "Default",
                        "Start",
                        "End",
                        "FlexStart",
                        "FlexEnd",
                        "Center",
                        "Baseline",
                        "Stretch",
                    ],
                ));
            }
        };
        Ok(AlignItems(Arc::new(inner)))
    }
}

#[reflect_methods]
impl AlignItems {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn align_items_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::AlignItems::Default => "Default",
            bevy::ui::AlignItems::Start => "Start",
            bevy::ui::AlignItems::End => "End",
            bevy::ui::AlignItems::FlexStart => "FlexStart",
            bevy::ui::AlignItems::FlexEnd => "FlexEnd",
            bevy::ui::AlignItems::Center => "Center",
            bevy::ui::AlignItems::Baseline => "Baseline",
            bevy::ui::AlignItems::Stretch => "Stretch",
        }
    }

    /// Returns `true` if this is [`AlignItems::Center`].
    #[tracing::instrument(skip(self))]
    pub fn align_items_is_center(&self) -> bool {
        matches!(*self.0, bevy::ui::AlignItems::Center)
    }

    /// Returns `true` if this is [`AlignItems::Stretch`].
    #[tracing::instrument(skip(self))]
    pub fn is_stretch(&self) -> bool {
        matches!(*self.0, bevy::ui::AlignItems::Stretch)
    }
}

mod emit_impls_align_items {
    use super::AlignItems;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AlignItems {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::AlignItems::from(::bevy::ui::AlignItems::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for AlignItems {}

// ── AlignContent ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::AlignContent, as AlignContent);
elicit_newtype_traits!(AlignContent, bevy::ui::AlignContent, [eq]);

impl AlignContent {
    fn as_str(&self) -> &'static str {
        self.align_content_as_str()
    }
}

impl From<AlignContent> for bevy::ui::AlignContent {
    fn from(v: AlignContent) -> Self {
        *v.0
    }
}

impl serde::Serialize for AlignContent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AlignContent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Default" => bevy::ui::AlignContent::Default,
            "Start" => bevy::ui::AlignContent::Start,
            "End" => bevy::ui::AlignContent::End,
            "FlexStart" => bevy::ui::AlignContent::FlexStart,
            "FlexEnd" => bevy::ui::AlignContent::FlexEnd,
            "Center" => bevy::ui::AlignContent::Center,
            "Stretch" => bevy::ui::AlignContent::Stretch,
            "SpaceBetween" => bevy::ui::AlignContent::SpaceBetween,
            "SpaceEvenly" => bevy::ui::AlignContent::SpaceEvenly,
            "SpaceAround" => bevy::ui::AlignContent::SpaceAround,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &[
                        "Default",
                        "Start",
                        "End",
                        "FlexStart",
                        "FlexEnd",
                        "Center",
                        "Stretch",
                        "SpaceBetween",
                        "SpaceEvenly",
                        "SpaceAround",
                    ],
                ));
            }
        };
        Ok(AlignContent(Arc::new(inner)))
    }
}

#[reflect_methods]
impl AlignContent {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn align_content_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::AlignContent::Default => "Default",
            bevy::ui::AlignContent::Start => "Start",
            bevy::ui::AlignContent::End => "End",
            bevy::ui::AlignContent::FlexStart => "FlexStart",
            bevy::ui::AlignContent::FlexEnd => "FlexEnd",
            bevy::ui::AlignContent::Center => "Center",
            bevy::ui::AlignContent::Stretch => "Stretch",
            bevy::ui::AlignContent::SpaceBetween => "SpaceBetween",
            bevy::ui::AlignContent::SpaceEvenly => "SpaceEvenly",
            bevy::ui::AlignContent::SpaceAround => "SpaceAround",
        }
    }
}

mod emit_impls_align_content {
    use super::AlignContent;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AlignContent {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::AlignContent::from(::bevy::ui::AlignContent::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for AlignContent {}

// ── AlignSelf ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::AlignSelf, as AlignSelf);
elicit_newtype_traits!(AlignSelf, bevy::ui::AlignSelf, [eq]);

impl AlignSelf {
    fn as_str(&self) -> &'static str {
        self.align_self_as_str()
    }
}

impl From<AlignSelf> for bevy::ui::AlignSelf {
    fn from(v: AlignSelf) -> Self {
        *v.0
    }
}

impl serde::Serialize for AlignSelf {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AlignSelf {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Auto" => bevy::ui::AlignSelf::Auto,
            "Start" => bevy::ui::AlignSelf::Start,
            "End" => bevy::ui::AlignSelf::End,
            "FlexStart" => bevy::ui::AlignSelf::FlexStart,
            "FlexEnd" => bevy::ui::AlignSelf::FlexEnd,
            "Center" => bevy::ui::AlignSelf::Center,
            "Baseline" => bevy::ui::AlignSelf::Baseline,
            "Stretch" => bevy::ui::AlignSelf::Stretch,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &[
                        "Auto",
                        "Start",
                        "End",
                        "FlexStart",
                        "FlexEnd",
                        "Center",
                        "Baseline",
                        "Stretch",
                    ],
                ));
            }
        };
        Ok(AlignSelf(Arc::new(inner)))
    }
}

#[reflect_methods]
impl AlignSelf {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn align_self_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::AlignSelf::Auto => "Auto",
            bevy::ui::AlignSelf::Start => "Start",
            bevy::ui::AlignSelf::End => "End",
            bevy::ui::AlignSelf::FlexStart => "FlexStart",
            bevy::ui::AlignSelf::FlexEnd => "FlexEnd",
            bevy::ui::AlignSelf::Center => "Center",
            bevy::ui::AlignSelf::Baseline => "Baseline",
            bevy::ui::AlignSelf::Stretch => "Stretch",
        }
    }
}

mod emit_impls_align_self {
    use super::AlignSelf;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AlignSelf {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::AlignSelf::from(::bevy::ui::AlignSelf::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for AlignSelf {}

// ── JustifyItems ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::JustifyItems, as JustifyItems);
elicit_newtype_traits!(JustifyItems, bevy::ui::JustifyItems, [eq]);

impl JustifyItems {
    fn as_str(&self) -> &'static str {
        self.justify_items_as_str()
    }
}

impl From<JustifyItems> for bevy::ui::JustifyItems {
    fn from(v: JustifyItems) -> Self {
        *v.0
    }
}

impl serde::Serialize for JustifyItems {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for JustifyItems {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Default" => bevy::ui::JustifyItems::Default,
            "Start" => bevy::ui::JustifyItems::Start,
            "End" => bevy::ui::JustifyItems::End,
            "Center" => bevy::ui::JustifyItems::Center,
            "Baseline" => bevy::ui::JustifyItems::Baseline,
            "Stretch" => bevy::ui::JustifyItems::Stretch,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Default", "Start", "End", "Center", "Baseline", "Stretch"],
                ));
            }
        };
        Ok(JustifyItems(Arc::new(inner)))
    }
}

#[reflect_methods]
impl JustifyItems {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn justify_items_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::JustifyItems::Default => "Default",
            bevy::ui::JustifyItems::Start => "Start",
            bevy::ui::JustifyItems::End => "End",
            bevy::ui::JustifyItems::Center => "Center",
            bevy::ui::JustifyItems::Baseline => "Baseline",
            bevy::ui::JustifyItems::Stretch => "Stretch",
        }
    }
}

mod emit_impls_justify_items {
    use super::JustifyItems;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for JustifyItems {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::JustifyItems::from(::bevy::ui::JustifyItems::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for JustifyItems {}

// ── JustifySelf ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::JustifySelf, as JustifySelf);
elicit_newtype_traits!(JustifySelf, bevy::ui::JustifySelf, [eq]);

impl JustifySelf {
    fn as_str(&self) -> &'static str {
        self.justify_self_as_str()
    }
}

impl From<JustifySelf> for bevy::ui::JustifySelf {
    fn from(v: JustifySelf) -> Self {
        *v.0
    }
}

impl serde::Serialize for JustifySelf {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for JustifySelf {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Auto" => bevy::ui::JustifySelf::Auto,
            "Start" => bevy::ui::JustifySelf::Start,
            "End" => bevy::ui::JustifySelf::End,
            "Center" => bevy::ui::JustifySelf::Center,
            "Baseline" => bevy::ui::JustifySelf::Baseline,
            "Stretch" => bevy::ui::JustifySelf::Stretch,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Auto", "Start", "End", "Center", "Baseline", "Stretch"],
                ));
            }
        };
        Ok(JustifySelf(Arc::new(inner)))
    }
}

#[reflect_methods]
impl JustifySelf {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn justify_self_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::JustifySelf::Auto => "Auto",
            bevy::ui::JustifySelf::Start => "Start",
            bevy::ui::JustifySelf::End => "End",
            bevy::ui::JustifySelf::Center => "Center",
            bevy::ui::JustifySelf::Baseline => "Baseline",
            bevy::ui::JustifySelf::Stretch => "Stretch",
        }
    }
}

mod emit_impls_justify_self {
    use super::JustifySelf;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for JustifySelf {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::JustifySelf::from(::bevy::ui::JustifySelf::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for JustifySelf {}

// ── OverflowAxis ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::OverflowAxis, as OverflowAxis);
elicit_newtype_traits!(OverflowAxis, bevy::ui::OverflowAxis, [eq]);

impl OverflowAxis {
    fn as_str(&self) -> &'static str {
        self.overflow_axis_as_str()
    }
}

impl From<OverflowAxis> for bevy::ui::OverflowAxis {
    fn from(v: OverflowAxis) -> Self {
        *v.0
    }
}

impl serde::Serialize for OverflowAxis {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for OverflowAxis {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let s = String::deserialize(deserializer)?;
        let inner = match s.as_str() {
            "Visible" => bevy::ui::OverflowAxis::Visible,
            "Clip" => bevy::ui::OverflowAxis::Clip,
            "Hidden" => bevy::ui::OverflowAxis::Hidden,
            "Scroll" => bevy::ui::OverflowAxis::Scroll,
            _ => {
                return Err(D::Error::unknown_variant(
                    &s,
                    &["Visible", "Clip", "Hidden", "Scroll"],
                ));
            }
        };
        Ok(OverflowAxis(Arc::new(inner)))
    }
}

#[reflect_methods]
impl OverflowAxis {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn overflow_axis_as_str(&self) -> &'static str {
        match *self.0 {
            bevy::ui::OverflowAxis::Visible => "Visible",
            bevy::ui::OverflowAxis::Clip => "Clip",
            bevy::ui::OverflowAxis::Hidden => "Hidden",
            bevy::ui::OverflowAxis::Scroll => "Scroll",
        }
    }

    /// Returns `true` if this is [`OverflowAxis::Visible`].
    #[tracing::instrument(skip(self))]
    pub fn is_visible(&self) -> bool {
        matches!(*self.0, bevy::ui::OverflowAxis::Visible)
    }

    /// Returns `true` if this clips overflow.
    #[tracing::instrument(skip(self))]
    pub fn is_clipping(&self) -> bool {
        matches!(
            *self.0,
            bevy::ui::OverflowAxis::Clip
                | bevy::ui::OverflowAxis::Hidden
                | bevy::ui::OverflowAxis::Scroll
        )
    }
}

mod emit_impls_overflow_axis {
    use super::OverflowAxis;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OverflowAxis {
        fn to_code_literal(&self) -> TokenStream {
            let variant = quote::format_ident!("{}", self.as_str());
            quote::quote! {
                ::elicit_bevy::OverflowAxis::from(::bevy::ui::OverflowAxis::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for OverflowAxis {}

// ── Overflow ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::Overflow, as Overflow);
elicit_newtype_traits!(Overflow, bevy::ui::Overflow, [eq]);

impl From<Overflow> for bevy::ui::Overflow {
    fn from(v: Overflow) -> Self {
        *v.0
    }
}

impl serde::Serialize for Overflow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("x", &OverflowAxis::from(self.0.x))?;
        map.serialize_entry("y", &OverflowAxis::from(self.0.y))?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Overflow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Overflow;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"an Overflow object with "x" and "y" OverflowAxis fields"#
                )
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Overflow, A::Error> {
                let mut x: Option<OverflowAxis> = None;
                let mut y: Option<OverflowAxis> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let ox = x.map(|v| *v.0).unwrap_or(bevy::ui::OverflowAxis::Visible);
                let oy = y.map(|v| *v.0).unwrap_or(bevy::ui::OverflowAxis::Visible);
                Ok(Overflow(Arc::new(bevy::ui::Overflow { x: ox, y: oy })))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl Overflow {
    /// Returns the x-axis overflow.
    #[tracing::instrument(skip(self))]
    pub fn x(&self) -> OverflowAxis {
        OverflowAxis::from(self.0.x)
    }

    /// Returns the y-axis overflow.
    #[tracing::instrument(skip(self))]
    pub fn y(&self) -> OverflowAxis {
        OverflowAxis::from(self.0.y)
    }

    /// Constructs [`Overflow`] where both axes are visible (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn visible(&self) -> Overflow {
        Overflow::from(bevy::ui::Overflow::visible())
    }

    /// Constructs [`Overflow`] where both axes clip (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn clip(&self) -> Overflow {
        Overflow::from(bevy::ui::Overflow::clip())
    }

    /// Constructs [`Overflow`] where both axes are hidden (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn hidden(&self) -> Overflow {
        Overflow::from(bevy::ui::Overflow::hidden())
    }

    /// Constructs [`Overflow`] where both axes scroll (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn scroll(&self) -> Overflow {
        Overflow::from(bevy::ui::Overflow::scroll())
    }
}

mod emit_impls_overflow {
    use super::{Overflow, OverflowAxis};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Overflow {
        fn to_code_literal(&self) -> TokenStream {
            let x = OverflowAxis::from(self.0.x).to_code_literal();
            let y = OverflowAxis::from(self.0.y).to_code_literal();
            quote::quote! {
                ::elicit_bevy::Overflow::from(::bevy::ui::Overflow {
                    x: ::bevy::ui::OverflowAxis::from(#x),
                    y: ::bevy::ui::OverflowAxis::from(#y),
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for Overflow {}

// ── FocusPolicy ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::FocusPolicy, as FocusPolicy, forward_serde);
elicit_newtype_traits!(FocusPolicy, bevy::ui::FocusPolicy, [eq]);

impl From<FocusPolicy> for bevy::ui::FocusPolicy {
    fn from(v: FocusPolicy) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl FocusPolicy {
    /// Returns `true` if this is [`FocusPolicy::Block`].
    #[tracing::instrument(skip(self))]
    pub fn is_block(&self) -> bool {
        matches!(*self.0, bevy::ui::FocusPolicy::Block)
    }

    /// Returns `true` if this is [`FocusPolicy::Pass`].
    #[tracing::instrument(skip(self))]
    pub fn is_pass(&self) -> bool {
        matches!(*self.0, bevy::ui::FocusPolicy::Pass)
    }
}

mod emit_impls_focus_policy {
    use super::FocusPolicy;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for FocusPolicy {
        fn to_code_literal(&self) -> TokenStream {
            let variant = match *self.0 {
                bevy::ui::FocusPolicy::Block => quote::format_ident!("Block"),
                bevy::ui::FocusPolicy::Pass => quote::format_ident!("Pass"),
            };
            quote::quote! {
                ::elicit_bevy::FocusPolicy::from(::bevy::ui::FocusPolicy::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for FocusPolicy {}

// ── BoxSizing ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BoxSizing, as BoxSizing, forward_serde);
elicit_newtype_traits!(BoxSizing, bevy::ui::BoxSizing, [eq]);

impl From<BoxSizing> for bevy::ui::BoxSizing {
    fn from(v: BoxSizing) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl BoxSizing {
    /// Returns `true` if this is [`BoxSizing::BorderBox`].
    #[tracing::instrument(skip(self))]
    pub fn is_border_box(&self) -> bool {
        matches!(*self.0, bevy::ui::BoxSizing::BorderBox)
    }

    /// Returns `true` if this is [`BoxSizing::ContentBox`].
    #[tracing::instrument(skip(self))]
    pub fn is_content_box(&self) -> bool {
        matches!(*self.0, bevy::ui::BoxSizing::ContentBox)
    }
}

mod emit_impls_box_sizing {
    use super::BoxSizing;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BoxSizing {
        fn to_code_literal(&self) -> TokenStream {
            let variant = match *self.0 {
                bevy::ui::BoxSizing::BorderBox => quote::format_ident!("BorderBox"),
                bevy::ui::BoxSizing::ContentBox => quote::format_ident!("ContentBox"),
            };
            quote::quote! {
                ::elicit_bevy::BoxSizing::from(::bevy::ui::BoxSizing::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for BoxSizing {}

// ── GridAutoFlow ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::GridAutoFlow, as GridAutoFlow, forward_serde);
elicit_newtype_traits!(GridAutoFlow, bevy::ui::GridAutoFlow, [eq]);

impl From<GridAutoFlow> for bevy::ui::GridAutoFlow {
    fn from(v: GridAutoFlow) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl GridAutoFlow {
    /// Returns a string name for this variant.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        match *self.0 {
            bevy::ui::GridAutoFlow::Row => "Row".to_string(),
            bevy::ui::GridAutoFlow::Column => "Column".to_string(),
            bevy::ui::GridAutoFlow::RowDense => "RowDense".to_string(),
            bevy::ui::GridAutoFlow::ColumnDense => "ColumnDense".to_string(),
        }
    }
}

mod emit_impls_grid_auto_flow {
    use super::GridAutoFlow;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GridAutoFlow {
        fn to_code_literal(&self) -> TokenStream {
            let variant = match *self.0 {
                bevy::ui::GridAutoFlow::Row => quote::format_ident!("Row"),
                bevy::ui::GridAutoFlow::Column => quote::format_ident!("Column"),
                bevy::ui::GridAutoFlow::RowDense => quote::format_ident!("RowDense"),
                bevy::ui::GridAutoFlow::ColumnDense => quote::format_ident!("ColumnDense"),
            };
            quote::quote! {
                ::elicit_bevy::GridAutoFlow::from(::bevy::ui::GridAutoFlow::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for GridAutoFlow {}

// ── ZIndex ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::ZIndex, as ZIndex);
elicit_newtype_traits!(ZIndex, bevy::ui::ZIndex, [eq]);

impl From<ZIndex> for bevy::ui::ZIndex {
    fn from(v: ZIndex) -> Self {
        *v.0
    }
}

impl serde::Serialize for ZIndex {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for ZIndex {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = i32::deserialize(d)?;
        Ok(ZIndex(Arc::new(bevy::ui::ZIndex(v))))
    }
}

#[reflect_methods]
impl ZIndex {
    /// Returns the z-index value.
    #[tracing::instrument(skip(self))]
    pub fn get(&self) -> i32 {
        self.0.0
    }
}

mod emit_impls_z_index {
    use super::ZIndex;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ZIndex {
        fn to_code_literal(&self) -> TokenStream {
            let v = self.0.0;
            quote::quote! {
                ::elicit_bevy::ZIndex::from(::bevy::ui::ZIndex(#v))
            }
        }
    }
}

impl elicitation::ElicitComplete for ZIndex {}

// ── GlobalZIndex ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::GlobalZIndex, as GlobalZIndex);
elicit_newtype_traits!(GlobalZIndex, bevy::ui::GlobalZIndex, [eq]);

impl From<GlobalZIndex> for bevy::ui::GlobalZIndex {
    fn from(v: GlobalZIndex) -> Self {
        *v.0
    }
}

impl serde::Serialize for GlobalZIndex {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for GlobalZIndex {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = i32::deserialize(d)?;
        Ok(GlobalZIndex(Arc::new(bevy::ui::GlobalZIndex(v))))
    }
}

#[reflect_methods]
impl GlobalZIndex {
    /// Returns the global z-index value.
    #[tracing::instrument(skip(self))]
    pub fn get(&self) -> i32 {
        self.0.0
    }
}

mod emit_impls_global_z_index {
    use super::GlobalZIndex;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GlobalZIndex {
        fn to_code_literal(&self) -> TokenStream {
            let v = self.0.0;
            quote::quote! {
                ::elicit_bevy::GlobalZIndex::from(::bevy::ui::GlobalZIndex(#v))
            }
        }
    }
}

impl elicitation::ElicitComplete for GlobalZIndex {}

// ── BackgroundColor ───────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BackgroundColor, as BackgroundColor, forward_serde);
elicit_newtype_traits!(BackgroundColor, bevy::ui::BackgroundColor, [eq]);

impl From<BackgroundColor> for bevy::ui::BackgroundColor {
    fn from(v: BackgroundColor) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl BackgroundColor {
    /// Returns the color as a [`crate::Color`] wrapper.
    #[tracing::instrument(skip(self))]
    pub fn get_color(&self) -> crate::Color {
        crate::Color::from((*self.0).0)
    }
}

mod emit_impls_background_color {
    use super::BackgroundColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BackgroundColor {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::BackgroundColor::from(
                    ::serde_json::from_str::<::bevy::ui::BackgroundColor>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for BackgroundColor {}

// ── BorderColor ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BorderColor, as BorderColor, forward_serde);
elicit_newtype_traits!(BorderColor, bevy::ui::BorderColor, [eq]);

impl From<BorderColor> for bevy::ui::BorderColor {
    fn from(v: BorderColor) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl BorderColor {
    /// Returns the top border color.
    #[tracing::instrument(skip(self))]
    pub fn top(&self) -> crate::Color {
        crate::Color::from((*self.0).top)
    }

    /// Returns the right border color.
    #[tracing::instrument(skip(self))]
    pub fn right(&self) -> crate::Color {
        crate::Color::from((*self.0).right)
    }

    /// Returns the bottom border color.
    #[tracing::instrument(skip(self))]
    pub fn bottom(&self) -> crate::Color {
        crate::Color::from((*self.0).bottom)
    }

    /// Returns the left border color.
    #[tracing::instrument(skip(self))]
    pub fn left(&self) -> crate::Color {
        crate::Color::from((*self.0).left)
    }
}

mod emit_impls_border_color {
    use super::BorderColor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BorderColor {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::BorderColor::from(
                    ::serde_json::from_str::<::bevy::ui::BorderColor>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for BorderColor {}

// ── Outline ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::Outline, as Outline, forward_serde);
elicit_newtype_traits!(Outline, bevy::ui::Outline, [eq]);

impl From<Outline> for bevy::ui::Outline {
    fn from(v: Outline) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl Outline {
    /// Returns the outline width.
    #[tracing::instrument(skip(self))]
    pub fn get_width(&self) -> Val {
        Val::from((*self.0).width)
    }

    /// Returns the outline offset.
    #[tracing::instrument(skip(self))]
    pub fn get_offset(&self) -> Val {
        Val::from((*self.0).offset)
    }

    /// Returns the outline color.
    #[tracing::instrument(skip(self))]
    pub fn get_color(&self) -> crate::Color {
        crate::Color::from((*self.0).color)
    }
}

mod emit_impls_outline {
    use super::Outline;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Outline {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::Outline::from(
                    ::serde_json::from_str::<::bevy::ui::Outline>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Outline {}

// ── Interaction ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::Interaction, as Interaction, forward_serde);
elicit_newtype_traits!(Interaction, bevy::ui::Interaction, [eq]);

impl From<Interaction> for bevy::ui::Interaction {
    fn from(v: Interaction) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Interaction {
    /// Returns `true` if the node is currently pressed.
    #[tracing::instrument(skip(self))]
    pub fn is_pressed(&self) -> bool {
        matches!(*self.0, bevy::ui::Interaction::Pressed)
    }

    /// Returns `true` if the node is currently hovered.
    #[tracing::instrument(skip(self))]
    pub fn is_hovered(&self) -> bool {
        matches!(*self.0, bevy::ui::Interaction::Hovered)
    }

    /// Returns `true` if the node has no interaction.
    #[tracing::instrument(skip(self))]
    pub fn is_none(&self) -> bool {
        matches!(*self.0, bevy::ui::Interaction::None)
    }
}

mod emit_impls_interaction {
    use super::Interaction;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Interaction {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::Interaction::from(
                    ::serde_json::from_str::<::bevy::ui::Interaction>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Interaction {}

// ── OverflowClipBox ───────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::OverflowClipBox, as OverflowClipBox, forward_serde);
elicit_newtype_traits!(OverflowClipBox, bevy::ui::OverflowClipBox, [eq]);

impl From<OverflowClipBox> for bevy::ui::OverflowClipBox {
    fn from(v: OverflowClipBox) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl OverflowClipBox {
    /// Returns `true` if this is the content box.
    #[tracing::instrument(skip(self))]
    pub fn is_content_box(&self) -> bool {
        matches!(*self.0, bevy::ui::OverflowClipBox::ContentBox)
    }

    /// Returns `true` if this is the padding box.
    #[tracing::instrument(skip(self))]
    pub fn is_padding_box(&self) -> bool {
        matches!(*self.0, bevy::ui::OverflowClipBox::PaddingBox)
    }

    /// Returns `true` if this is the border box.
    #[tracing::instrument(skip(self))]
    pub fn is_border_box(&self) -> bool {
        matches!(*self.0, bevy::ui::OverflowClipBox::BorderBox)
    }
}

mod emit_impls_overflow_clip_box {
    use super::OverflowClipBox;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OverflowClipBox {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::OverflowClipBox::from(
                    ::serde_json::from_str::<::bevy::ui::OverflowClipBox>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for OverflowClipBox {}

// ── OverflowClipMargin ────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::OverflowClipMargin, as OverflowClipMargin, forward_serde);
elicit_newtype_traits!(OverflowClipMargin, bevy::ui::OverflowClipMargin, [eq]);

impl From<OverflowClipMargin> for bevy::ui::OverflowClipMargin {
    fn from(v: OverflowClipMargin) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl OverflowClipMargin {
    /// Returns the visual box setting.
    #[tracing::instrument(skip(self))]
    pub fn get_visual_box(&self) -> OverflowClipBox {
        OverflowClipBox::from((*self.0).visual_box)
    }

    /// Returns the margin in logical pixels.
    #[tracing::instrument(skip(self))]
    pub fn get_margin(&self) -> f32 {
        (*self.0).margin
    }
}

mod emit_impls_overflow_clip_margin {
    use super::OverflowClipMargin;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OverflowClipMargin {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::OverflowClipMargin::from(
                    ::serde_json::from_str::<::bevy::ui::OverflowClipMargin>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for OverflowClipMargin {}

// ── GridTrackRepetition ───────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::GridTrackRepetition, as GridTrackRepetition, forward_serde);
elicit_newtype_traits!(GridTrackRepetition, bevy::ui::GridTrackRepetition, [eq]);

impl From<GridTrackRepetition> for bevy::ui::GridTrackRepetition {
    fn from(v: GridTrackRepetition) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl GridTrackRepetition {
    /// Returns `true` if this is an auto-fill repetition.
    #[tracing::instrument(skip(self))]
    pub fn is_auto_fill(&self) -> bool {
        matches!(*self.0, bevy::ui::GridTrackRepetition::AutoFill)
    }

    /// Returns `true` if this is an auto-fit repetition.
    #[tracing::instrument(skip(self))]
    pub fn is_auto_fit(&self) -> bool {
        matches!(*self.0, bevy::ui::GridTrackRepetition::AutoFit)
    }

    /// Returns the explicit count, if any.
    #[tracing::instrument(skip(self))]
    pub fn get_count(&self) -> Option<u16> {
        match *self.0 {
            bevy::ui::GridTrackRepetition::Count(n) => Some(n),
            _ => None,
        }
    }
}

mod emit_impls_grid_track_repetition {
    use super::GridTrackRepetition;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for GridTrackRepetition {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::GridTrackRepetition::from(
                    ::serde_json::from_str::<::bevy::ui::GridTrackRepetition>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for GridTrackRepetition {}

// ── MinTrackSizingFunction ────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::MinTrackSizingFunction, as MinTrackSizingFunction, forward_serde);
elicit_newtype_traits!(
    MinTrackSizingFunction,
    bevy::ui::MinTrackSizingFunction,
    [eq]
);

impl From<MinTrackSizingFunction> for bevy::ui::MinTrackSizingFunction {
    fn from(v: MinTrackSizingFunction) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl MinTrackSizingFunction {
    /// Returns `true` if this is the `Auto` variant.
    #[tracing::instrument(skip(self))]
    pub fn is_auto(&self) -> bool {
        matches!(*self.0, bevy::ui::MinTrackSizingFunction::Auto)
    }
}

mod emit_impls_min_track_sizing_function {
    use super::MinTrackSizingFunction;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for MinTrackSizingFunction {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::ui::MinTrackSizingFunction as M;
            match *self.0 {
                M::Px(v) => quote::quote! { ::bevy::ui::MinTrackSizingFunction::Px(#v) },
                M::Percent(v) => {
                    quote::quote! { ::bevy::ui::MinTrackSizingFunction::Percent(#v) }
                }
                M::MinContent => {
                    quote::quote! { ::bevy::ui::MinTrackSizingFunction::MinContent }
                }
                M::MaxContent => {
                    quote::quote! { ::bevy::ui::MinTrackSizingFunction::MaxContent }
                }
                M::Auto => quote::quote! { ::bevy::ui::MinTrackSizingFunction::Auto },
                M::VMin(v) => quote::quote! { ::bevy::ui::MinTrackSizingFunction::VMin(#v) },
                M::VMax(v) => quote::quote! { ::bevy::ui::MinTrackSizingFunction::VMax(#v) },
                M::Vh(v) => quote::quote! { ::bevy::ui::MinTrackSizingFunction::Vh(#v) },
                M::Vw(v) => quote::quote! { ::bevy::ui::MinTrackSizingFunction::Vw(#v) },
            }
        }
    }
}

impl elicitation::ElicitComplete for MinTrackSizingFunction {}

// ── MaxTrackSizingFunction ────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::MaxTrackSizingFunction, as MaxTrackSizingFunction, forward_serde);
elicit_newtype_traits!(
    MaxTrackSizingFunction,
    bevy::ui::MaxTrackSizingFunction,
    [eq]
);

impl From<MaxTrackSizingFunction> for bevy::ui::MaxTrackSizingFunction {
    fn from(v: MaxTrackSizingFunction) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl MaxTrackSizingFunction {
    /// Returns `true` if this is the `Auto` variant.
    #[tracing::instrument(skip(self))]
    pub fn is_auto(&self) -> bool {
        matches!(*self.0, bevy::ui::MaxTrackSizingFunction::Auto)
    }

    /// Returns `true` if this is a fractional (`fr`) sizing.
    #[tracing::instrument(skip(self))]
    pub fn is_fraction(&self) -> bool {
        matches!(*self.0, bevy::ui::MaxTrackSizingFunction::Fraction(_))
    }
}

mod emit_impls_max_track_sizing_function {
    use super::MaxTrackSizingFunction;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for MaxTrackSizingFunction {
        fn to_code_literal(&self) -> TokenStream {
            use bevy::ui::MaxTrackSizingFunction as M;
            match *self.0 {
                M::Px(v) => quote::quote! { ::bevy::ui::MaxTrackSizingFunction::Px(#v) },
                M::Percent(v) => {
                    quote::quote! { ::bevy::ui::MaxTrackSizingFunction::Percent(#v) }
                }
                M::MinContent => {
                    quote::quote! { ::bevy::ui::MaxTrackSizingFunction::MinContent }
                }
                M::MaxContent => {
                    quote::quote! { ::bevy::ui::MaxTrackSizingFunction::MaxContent }
                }
                M::FitContentPx(v) => {
                    quote::quote! { ::bevy::ui::MaxTrackSizingFunction::FitContentPx(#v) }
                }
                M::FitContentPercent(v) => {
                    quote::quote! { ::bevy::ui::MaxTrackSizingFunction::FitContentPercent(#v) }
                }
                M::Auto => quote::quote! { ::bevy::ui::MaxTrackSizingFunction::Auto },
                M::Fraction(v) => {
                    quote::quote! { ::bevy::ui::MaxTrackSizingFunction::Fraction(#v) }
                }
                M::VMin(v) => quote::quote! { ::bevy::ui::MaxTrackSizingFunction::VMin(#v) },
                M::VMax(v) => quote::quote! { ::bevy::ui::MaxTrackSizingFunction::VMax(#v) },
                M::Vh(v) => quote::quote! { ::bevy::ui::MaxTrackSizingFunction::Vh(#v) },
                M::Vw(v) => quote::quote! { ::bevy::ui::MaxTrackSizingFunction::Vw(#v) },
            }
        }
    }
}

impl elicitation::ElicitComplete for MaxTrackSizingFunction {}

// ── GridTrack ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::GridTrack, as GridTrack, forward_serde);
elicit_newtype_traits!(GridTrack, bevy::ui::GridTrack, [eq]);

impl From<GridTrack> for bevy::ui::GridTrack {
    fn from(v: GridTrack) -> Self {
        *v.0
    }
}

mod emit_impls_grid_track {
    use super::{GridTrack, MaxTrackSizingFunction, MinTrackSizingFunction};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    use std::sync::Arc;

    impl ToCodeLiteral for GridTrack {
        fn to_code_literal(&self) -> TokenStream {
            // Deserialize min/max via serde since the fields are pub(crate) in bevy_ui.
            let val = serde_json::to_value(&*self.0).unwrap_or_default();
            let min: bevy::ui::MinTrackSizingFunction =
                serde_json::from_value(val["min_sizing_function"].clone()).unwrap_or_default();
            let max: bevy::ui::MaxTrackSizingFunction =
                serde_json::from_value(val["max_sizing_function"].clone()).unwrap_or_default();
            let min_lit = MinTrackSizingFunction(Arc::new(min)).to_code_literal();
            let max_lit = MaxTrackSizingFunction(Arc::new(max)).to_code_literal();
            quote::quote! {
                ::elicit_bevy::GridTrack::from(
                    ::bevy::ui::GridTrack::minmax(#min_lit, #max_lit)
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for GridTrack {}

// ── RepeatedGridTrack ─────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::RepeatedGridTrack, as RepeatedGridTrack, forward_serde);
elicit_newtype_traits!(RepeatedGridTrack, bevy::ui::RepeatedGridTrack, [eq]);

impl From<RepeatedGridTrack> for bevy::ui::RepeatedGridTrack {
    fn from(v: RepeatedGridTrack) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl RepeatedGridTrack {
    /// Returns the repetition count if explicit, otherwise `None`.
    #[tracing::instrument(skip(self))]
    pub fn get_count(&self) -> Option<u16> {
        let val = serde_json::to_value(&*self.0).unwrap_or_default();
        let rep = val.get("repetition")?;
        rep.get("Count").and_then(|v| v.as_u64()).map(|n| n as u16)
    }
}

mod emit_impls_repeated_grid_track {
    use super::RepeatedGridTrack;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RepeatedGridTrack {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::RepeatedGridTrack::from(
                    ::serde_json::from_str::<::bevy::ui::RepeatedGridTrack>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for RepeatedGridTrack {}

// ── UiScale ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::UiScale, as UiScale);
elicit_newtype_traits!(UiScale, bevy::ui::UiScale, []);

impl From<UiScale> for bevy::ui::UiScale {
    fn from(v: UiScale) -> Self {
        bevy::ui::UiScale((*v.0).0)
    }
}

impl serde::Serialize for UiScale {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).0.serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for UiScale {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        f32::deserialize(d).map(|v| UiScale(Arc::new(bevy::ui::UiScale(v))))
    }
}

#[reflect_methods]
impl UiScale {
    /// Returns the scale factor.
    #[tracing::instrument(skip(self))]
    pub fn get(&self) -> f32 {
        (*self.0).0
    }
}

mod emit_impls_ui_scale {
    use super::UiScale;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for UiScale {
        fn to_code_literal(&self) -> TokenStream {
            let v = (*self.0).0;
            quote::quote! {
                ::elicit_bevy::UiScale::from(::bevy::ui::UiScale(#v))
            }
        }
    }
}

impl elicitation::ElicitComplete for UiScale {}

// ── ScrollPosition ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::ScrollPosition, as ScrollPosition);
elicit_newtype_traits!(ScrollPosition, bevy::ui::ScrollPosition, []);

impl From<ScrollPosition> for bevy::ui::ScrollPosition {
    fn from(v: ScrollPosition) -> Self {
        (*v.0).clone()
    }
}

impl serde::Serialize for ScrollPosition {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("ScrollPosition", 2)?;
        st.serialize_field("x", &(*self.0).0.x)?;
        st.serialize_field("y", &(*self.0).0.y)?;
        st.end()
    }
}

impl<'de> serde::Deserialize<'de> for ScrollPosition {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ScrollPosition;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a ScrollPosition {{x, y}}")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut x: Option<f32> = None;
                let mut y: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                let x = x.unwrap_or(0.0);
                let y = y.unwrap_or(0.0);
                Ok(ScrollPosition(Arc::new(bevy::ui::ScrollPosition(
                    bevy::math::Vec2::new(x, y),
                ))))
            }
        }
        d.deserialize_map(V)
    }
}

#[reflect_methods]
impl ScrollPosition {
    /// Returns the horizontal scroll position.
    #[tracing::instrument(skip(self))]
    pub fn get_x(&self) -> f32 {
        (*self.0).0.x
    }

    /// Returns the vertical scroll position.
    #[tracing::instrument(skip(self))]
    pub fn get_y(&self) -> f32 {
        (*self.0).0.y
    }
}

mod emit_impls_scroll_position {
    use super::ScrollPosition;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ScrollPosition {
        fn to_code_literal(&self) -> TokenStream {
            let x = (*self.0).0.x;
            let y = (*self.0).0.y;
            quote::quote! {
                ::elicit_bevy::ScrollPosition::from(
                    ::bevy::ui::ScrollPosition(::bevy::math::Vec2::new(#x, #y))
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for ScrollPosition {}

// ── IgnoreScroll ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::IgnoreScroll, as IgnoreScroll);
elicit_newtype_traits!(IgnoreScroll, bevy::ui::IgnoreScroll, []);

impl From<IgnoreScroll> for bevy::ui::IgnoreScroll {
    fn from(v: IgnoreScroll) -> Self {
        (*v.0).clone()
    }
}

impl serde::Serialize for IgnoreScroll {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut st = s.serialize_struct("IgnoreScroll", 2)?;
        st.serialize_field("x", &(*self.0).0.x)?;
        st.serialize_field("y", &(*self.0).0.y)?;
        st.end()
    }
}

impl<'de> serde::Deserialize<'de> for IgnoreScroll {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = IgnoreScroll;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "an IgnoreScroll {{x, y}}")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut x: Option<bool> = None;
                let mut y: Option<bool> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                Ok(IgnoreScroll(Arc::new(bevy::ui::IgnoreScroll(
                    bevy::math::BVec2::new(x.unwrap_or(false), y.unwrap_or(false)),
                ))))
            }
        }
        d.deserialize_map(V)
    }
}

#[reflect_methods]
impl IgnoreScroll {
    /// Returns whether horizontal scrolling is ignored.
    #[tracing::instrument(skip(self))]
    pub fn get_x(&self) -> bool {
        (*self.0).0.x
    }

    /// Returns whether vertical scrolling is ignored.
    #[tracing::instrument(skip(self))]
    pub fn get_y(&self) -> bool {
        (*self.0).0.y
    }
}

mod emit_impls_ignore_scroll {
    use super::IgnoreScroll;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for IgnoreScroll {
        fn to_code_literal(&self) -> TokenStream {
            let x = (*self.0).0.x;
            let y = (*self.0).0.y;
            quote::quote! {
                ::elicit_bevy::IgnoreScroll::from(
                    ::bevy::ui::IgnoreScroll(::bevy::math::BVec2::new(#x, #y))
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for IgnoreScroll {}

// ── UiPosition ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::UiPosition, as UiPosition, forward_serde);
elicit_newtype_traits!(UiPosition, bevy::ui::UiPosition, [eq]);

impl From<UiPosition> for bevy::ui::UiPosition {
    fn from(v: UiPosition) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl UiPosition {
    /// Returns the horizontal offset relative to the anchor.
    #[tracing::instrument(skip(self))]
    pub fn get_x(&self) -> Val {
        Val::from((*self.0).x)
    }

    /// Returns the vertical offset relative to the anchor.
    #[tracing::instrument(skip(self))]
    pub fn get_y(&self) -> Val {
        Val::from((*self.0).y)
    }
}

mod emit_impls_ui_position {
    use super::UiPosition;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for UiPosition {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::UiPosition::from(
                    ::serde_json::from_str::<::bevy::ui::UiPosition>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for UiPosition {}

// ── InterpolationColorSpace ───────────────────────────────────────────────────

elicit_newtype!(bevy::ui::InterpolationColorSpace, as InterpolationColorSpace, forward_serde);
elicit_newtype_traits!(
    InterpolationColorSpace,
    bevy::ui::InterpolationColorSpace,
    [eq_hash]
);

impl From<InterpolationColorSpace> for bevy::ui::InterpolationColorSpace {
    fn from(v: InterpolationColorSpace) -> Self {
        *v.0
    }
}

mod emit_impls_interpolation_color_space {
    use super::InterpolationColorSpace;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for InterpolationColorSpace {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::InterpolationColorSpace::from(
                    ::serde_json::from_str::<::bevy::ui::InterpolationColorSpace>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for InterpolationColorSpace {}

// ── ColorStop ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::ColorStop, as ColorStop, forward_serde);
elicit_newtype_traits!(ColorStop, bevy::ui::ColorStop, [eq]);

impl From<ColorStop> for bevy::ui::ColorStop {
    fn from(v: ColorStop) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl ColorStop {
    /// Returns the stop color.
    #[tracing::instrument(skip(self))]
    pub fn get_color(&self) -> crate::Color {
        crate::Color::from((*self.0).color)
    }

    /// Returns the stop position along the gradient line.
    #[tracing::instrument(skip(self))]
    pub fn get_point(&self) -> Val {
        Val::from((*self.0).point)
    }

    /// Returns the interpolation midpoint hint (0–1).
    #[tracing::instrument(skip(self))]
    pub fn get_hint(&self) -> f32 {
        (*self.0).hint
    }
}

mod emit_impls_color_stop {
    use super::ColorStop;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ColorStop {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::ColorStop::from(
                    ::serde_json::from_str::<::bevy::ui::ColorStop>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for ColorStop {}

// ── AngularColorStop ──────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::AngularColorStop, as AngularColorStop, forward_serde);
elicit_newtype_traits!(AngularColorStop, bevy::ui::AngularColorStop, [eq]);

impl From<AngularColorStop> for bevy::ui::AngularColorStop {
    fn from(v: AngularColorStop) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl AngularColorStop {
    /// Returns the stop color.
    #[tracing::instrument(skip(self))]
    pub fn get_color(&self) -> crate::Color {
        crate::Color::from((*self.0).color)
    }

    /// Returns the explicit angle in radians, if set.
    #[tracing::instrument(skip(self))]
    pub fn get_angle(&self) -> Option<f32> {
        (*self.0).angle
    }

    /// Returns the interpolation midpoint hint (0–1).
    #[tracing::instrument(skip(self))]
    pub fn get_hint(&self) -> f32 {
        (*self.0).hint
    }
}

mod emit_impls_angular_color_stop {
    use super::AngularColorStop;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AngularColorStop {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::AngularColorStop::from(
                    ::serde_json::from_str::<::bevy::ui::AngularColorStop>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for AngularColorStop {}

// ── RadialGradientShape ───────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::RadialGradientShape, as RadialGradientShape, forward_serde);
elicit_newtype_traits!(RadialGradientShape, bevy::ui::RadialGradientShape, [eq]);

impl From<RadialGradientShape> for bevy::ui::RadialGradientShape {
    fn from(v: RadialGradientShape) -> Self {
        *v.0
    }
}

mod emit_impls_radial_gradient_shape {
    use super::RadialGradientShape;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RadialGradientShape {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::RadialGradientShape::from(
                    ::serde_json::from_str::<::bevy::ui::RadialGradientShape>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for RadialGradientShape {}

// ── LinearGradient ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::LinearGradient, as LinearGradient, forward_serde);
elicit_newtype_traits!(LinearGradient, bevy::ui::LinearGradient, [eq]);

impl From<LinearGradient> for bevy::ui::LinearGradient {
    fn from(v: LinearGradient) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl LinearGradient {
    /// Returns the gradient angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn get_angle(&self) -> f32 {
        (*self.0).angle
    }

    /// Returns the number of color stops.
    #[tracing::instrument(skip(self))]
    pub fn stop_count(&self) -> usize {
        (*self.0).stops.len()
    }

    /// Returns the interpolation color space.
    #[tracing::instrument(skip(self))]
    pub fn get_color_space(&self) -> InterpolationColorSpace {
        InterpolationColorSpace::from((*self.0).color_space)
    }
}

mod emit_impls_linear_gradient {
    use super::LinearGradient;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for LinearGradient {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::LinearGradient::from(
                    ::serde_json::from_str::<::bevy::ui::LinearGradient>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for LinearGradient {}

// ── RadialGradient ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::RadialGradient, as RadialGradient, forward_serde);
elicit_newtype_traits!(RadialGradient, bevy::ui::RadialGradient, [eq]);

impl From<RadialGradient> for bevy::ui::RadialGradient {
    fn from(v: RadialGradient) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl RadialGradient {
    /// Returns the number of color stops.
    #[tracing::instrument(skip(self))]
    pub fn stop_count(&self) -> usize {
        (*self.0).stops.len()
    }

    /// Returns the gradient center position.
    #[tracing::instrument(skip(self))]
    pub fn get_position(&self) -> UiPosition {
        UiPosition::from((*self.0).position)
    }

    /// Returns the interpolation color space.
    #[tracing::instrument(skip(self))]
    pub fn get_color_space(&self) -> InterpolationColorSpace {
        InterpolationColorSpace::from((*self.0).color_space)
    }
}

mod emit_impls_radial_gradient {
    use super::RadialGradient;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for RadialGradient {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::RadialGradient::from(
                    ::serde_json::from_str::<::bevy::ui::RadialGradient>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for RadialGradient {}

// ── ConicGradient ─────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::ConicGradient, as ConicGradient, forward_serde);
elicit_newtype_traits!(ConicGradient, bevy::ui::ConicGradient, [eq]);

impl From<ConicGradient> for bevy::ui::ConicGradient {
    fn from(v: ConicGradient) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl ConicGradient {
    /// Returns the starting angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn get_start(&self) -> f32 {
        (*self.0).start
    }

    /// Returns the number of angular color stops.
    #[tracing::instrument(skip(self))]
    pub fn stop_count(&self) -> usize {
        (*self.0).stops.len()
    }

    /// Returns the gradient center position.
    #[tracing::instrument(skip(self))]
    pub fn get_position(&self) -> UiPosition {
        UiPosition::from((*self.0).position)
    }
}

mod emit_impls_conic_gradient {
    use super::ConicGradient;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ConicGradient {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::ConicGradient::from(
                    ::serde_json::from_str::<::bevy::ui::ConicGradient>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for ConicGradient {}

// ── Gradient ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::Gradient, as Gradient, forward_serde);
elicit_newtype_traits!(Gradient, bevy::ui::Gradient, [eq]);

impl From<Gradient> for bevy::ui::Gradient {
    fn from(v: Gradient) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl Gradient {
    /// Returns `true` if the gradient has no color stops.
    #[tracing::instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        (*self.0).is_empty()
    }

    /// Returns `true` if this is a linear gradient.
    #[tracing::instrument(skip(self))]
    pub fn is_linear(&self) -> bool {
        matches!(&*self.0, bevy::ui::Gradient::Linear(_))
    }

    /// Returns `true` if this is a radial gradient.
    #[tracing::instrument(skip(self))]
    pub fn is_radial(&self) -> bool {
        matches!(&*self.0, bevy::ui::Gradient::Radial(_))
    }

    /// Returns `true` if this is a conic gradient.
    #[tracing::instrument(skip(self))]
    pub fn is_conic(&self) -> bool {
        matches!(&*self.0, bevy::ui::Gradient::Conic(_))
    }
}

mod emit_impls_gradient {
    use super::Gradient;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Gradient {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::Gradient::from(
                    ::serde_json::from_str::<::bevy::ui::Gradient>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Gradient {}

// ── BackgroundGradient ────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BackgroundGradient, as BackgroundGradient, forward_serde);
elicit_newtype_traits!(BackgroundGradient, bevy::ui::BackgroundGradient, [eq]);

impl From<BackgroundGradient> for bevy::ui::BackgroundGradient {
    fn from(v: BackgroundGradient) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl BackgroundGradient {
    /// Returns the number of gradients.
    #[tracing::instrument(skip(self))]
    pub fn get_len(&self) -> usize {
        (*self.0).0.len()
    }
}

mod emit_impls_background_gradient {
    use super::BackgroundGradient;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BackgroundGradient {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::BackgroundGradient::from(
                    ::serde_json::from_str::<::bevy::ui::BackgroundGradient>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for BackgroundGradient {}

// ── BorderGradient ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BorderGradient, as BorderGradient, forward_serde);
elicit_newtype_traits!(BorderGradient, bevy::ui::BorderGradient, [eq]);

impl From<BorderGradient> for bevy::ui::BorderGradient {
    fn from(v: BorderGradient) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl BorderGradient {
    /// Returns the number of gradients.
    #[tracing::instrument(skip(self))]
    pub fn get_len(&self) -> usize {
        (*self.0).0.len()
    }
}

mod emit_impls_border_gradient {
    use super::BorderGradient;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BorderGradient {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::BorderGradient::from(
                    ::serde_json::from_str::<::bevy::ui::BorderGradient>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for BorderGradient {}

// ── ShadowStyle ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::ShadowStyle, as ShadowStyle, forward_serde);
elicit_newtype_traits!(ShadowStyle, bevy::ui::ShadowStyle, [eq]);

impl From<ShadowStyle> for bevy::ui::ShadowStyle {
    fn from(v: ShadowStyle) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl ShadowStyle {
    /// Returns the shadow color.
    #[tracing::instrument(skip(self))]
    pub fn get_color(&self) -> crate::Color {
        crate::Color::from((*self.0).color)
    }

    /// Returns the horizontal offset.
    #[tracing::instrument(skip(self))]
    pub fn get_x_offset(&self) -> Val {
        Val::from((*self.0).x_offset)
    }

    /// Returns the vertical offset.
    #[tracing::instrument(skip(self))]
    pub fn get_y_offset(&self) -> Val {
        Val::from((*self.0).y_offset)
    }

    /// Returns the spread radius.
    #[tracing::instrument(skip(self))]
    pub fn get_spread_radius(&self) -> Val {
        Val::from((*self.0).spread_radius)
    }

    /// Returns the blur radius.
    #[tracing::instrument(skip(self))]
    pub fn get_blur_radius(&self) -> Val {
        Val::from((*self.0).blur_radius)
    }
}

mod emit_impls_shadow_style {
    use super::ShadowStyle;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for ShadowStyle {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::ShadowStyle::from(
                    ::serde_json::from_str::<::bevy::ui::ShadowStyle>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for ShadowStyle {}

// ── BoxShadow ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::ui::BoxShadow, as BoxShadow, forward_serde);
elicit_newtype_traits!(BoxShadow, bevy::ui::BoxShadow, [eq]);

impl From<BoxShadow> for bevy::ui::BoxShadow {
    fn from(v: BoxShadow) -> Self {
        (*v.0).clone()
    }
}

#[reflect_methods]
impl BoxShadow {
    /// Returns the number of shadow layers.
    #[tracing::instrument(skip(self))]
    pub fn get_len(&self) -> usize {
        (*self.0).0.len()
    }
}

mod emit_impls_box_shadow {
    use super::BoxShadow;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BoxShadow {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&*self.0).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::BoxShadow::from(
                    ::serde_json::from_str::<::bevy::ui::BoxShadow>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for BoxShadow {}

// ── Marker components ─────────────────────────────────────────────────────────
//
// These are Bevy unit-struct marker components.  They carry no fields, so the
// standard `elicit_newtype!` Arc-wrapper pattern would be wasteful.  Instead,
// we define plain shadow structs and use a local macro to wire up all the
// required `Elicitation` trait impls uniformly.

/// Generates the full `Elicitation` trait chain for a shadow struct with fields
/// that mirrors a Bevy component.  The shadow struct must already derive
/// `Debug + Clone + Serialize + Deserialize + JsonSchema` and provide a
/// `ToCodeLiteral` impl.
macro_rules! shadow_elicitation {
    ($name:ident) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for $name {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                let response = communicator
                    .send_prompt(concat!("Enter value for ", stringify!($name)))
                    .await?;
                serde_json::from_str(&response)
                    .or_else(|_| serde_json::from_str::<Self>(&format!("\"{}\"", response)))
                    .map_err(|e| {
                        elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                            format!("Invalid {}: {}", stringify!($name), e),
                        ))
                    })
            }

            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }

            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }

            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }

        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }

        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }

        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(concat!("Shadow type for `", stringify!($name), "`.").to_string())
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $name {}
    };
}

/// Generates the full `Elicitation` trait chain for a shadow unit-struct that
/// mirrors a Bevy marker component.  The shadow struct must already derive
/// `Debug + Clone + Default + Serialize + Deserialize + JsonSchema` and provide
/// a `ToCodeLiteral` impl.
macro_rules! unit_elicitation {
    ($name:ident, $inner_path:path) => {
        impl elicitation::Prompt for $name {
            fn prompt() -> Option<&'static str> {
                None
            }
        }

        impl elicitation::Elicitation for $name {
            type Style = ();

            async fn elicit<C: elicitation::ElicitCommunicator>(
                _communicator: &C,
            ) -> elicitation::ElicitResult<Self> {
                Ok(Self)
            }

            fn kani_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::kani_trusted_opaque(stringify!($name))
            }

            fn verus_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::verus_trusted_opaque(stringify!($name))
            }

            fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
                elicitation::verification::proof_helpers::creusot_trusted_opaque(stringify!($name))
            }
        }

        impl elicitation::ElicitIntrospect for $name {
            fn pattern() -> elicitation::ElicitationPattern {
                elicitation::ElicitationPattern::Primitive
            }

            fn metadata() -> elicitation::TypeMetadata {
                elicitation::TypeMetadata {
                    type_name: stringify!($name),
                    description: None,
                    details: elicitation::PatternDetails::Primitive,
                }
            }
        }

        impl elicitation::ElicitPromptTree for $name {
            fn prompt_tree() -> elicitation::PromptTree {
                elicitation::PromptTree::Leaf {
                    prompt: stringify!($name).to_string(),
                    type_name: stringify!($name).to_string(),
                }
            }
        }

        impl elicitation::ElicitSpec for $name {
            fn type_spec() -> elicitation::TypeSpec {
                elicitation::TypeSpecBuilder::default()
                    .type_name(stringify!($name).to_string())
                    .summary(
                        concat!(
                            "Marker component shadow for `",
                            stringify!($inner_path),
                            "`."
                        )
                        .to_string(),
                    )
                    .build()
                    .expect("valid TypeSpec")
            }
        }

        impl elicitation::ElicitComplete for $name {}
    };
}

// ── InteractionDisabled ───────────────────────────────────────────────────────

/// Shadow of [`bevy::ui::InteractionDisabled`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct InteractionDisabled;

impl From<InteractionDisabled> for bevy::ui::InteractionDisabled {
    fn from(_: InteractionDisabled) -> Self {
        bevy::ui::InteractionDisabled
    }
}

mod emit_impls_interaction_disabled {
    use super::InteractionDisabled;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for InteractionDisabled {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::InteractionDisabled }
        }
    }
}

unit_elicitation!(InteractionDisabled, bevy::ui::InteractionDisabled);

// ── Pressed ───────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::ui::Pressed`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Pressed;

impl From<Pressed> for bevy::ui::Pressed {
    fn from(_: Pressed) -> Self {
        bevy::ui::Pressed
    }
}

mod emit_impls_pressed {
    use super::Pressed;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Pressed {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::Pressed }
        }
    }
}

unit_elicitation!(Pressed, bevy::ui::Pressed);

// ── Checkable ─────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::ui::Checkable`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Checkable;

impl From<Checkable> for bevy::ui::Checkable {
    fn from(_: Checkable) -> Self {
        bevy::ui::Checkable
    }
}

mod emit_impls_checkable {
    use super::Checkable;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Checkable {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::Checkable }
        }
    }
}

unit_elicitation!(Checkable, bevy::ui::Checkable);

// ── Checked ───────────────────────────────────────────────────────────────────

/// Shadow of [`bevy::ui::Checked`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct Checked;

impl From<Checked> for bevy::ui::Checked {
    fn from(_: Checked) -> Self {
        bevy::ui::Checked
    }
}

mod emit_impls_checked {
    use super::Checked;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Checked {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::Checked }
        }
    }
}

unit_elicitation!(Checked, bevy::ui::Checked);

// ── OverrideClip ──────────────────────────────────────────────────────────────

/// Shadow of [`bevy::ui::OverrideClip`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct OverrideClip;

impl From<OverrideClip> for bevy::ui::OverrideClip {
    fn from(_: OverrideClip) -> Self {
        bevy::ui::OverrideClip
    }
}

mod emit_impls_override_clip {
    use super::OverrideClip;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for OverrideClip {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::OverrideClip }
        }
    }
}

unit_elicitation!(OverrideClip, bevy::ui::OverrideClip);

// ── IsDefaultUiCamera ─────────────────────────────────────────────────────────

/// Shadow of [`bevy::ui::IsDefaultUiCamera`].
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct IsDefaultUiCamera;

impl From<IsDefaultUiCamera> for bevy::ui::IsDefaultUiCamera {
    fn from(_: IsDefaultUiCamera) -> Self {
        bevy::ui::IsDefaultUiCamera
    }
}

mod emit_impls_is_default_ui_camera {
    use super::IsDefaultUiCamera;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for IsDefaultUiCamera {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::IsDefaultUiCamera }
        }
    }
}

unit_elicitation!(IsDefaultUiCamera, bevy::ui::IsDefaultUiCamera);

// ── AutoDirectionalNavigation ─────────────────────────────────────────────────

/// Shadow of [`bevy::ui::auto_directional_navigation::AutoDirectionalNavigation`].
///
/// Component that enables automatic directional navigation for a focusable UI entity.
/// When attached, the system automatically computes navigation targets in each
/// cardinal direction based on spatial layout without requiring manual `TabIndex` links.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct AutoDirectionalNavigation {
    /// Whether to also consider `TabIndex` ordering hints during navigation.
    /// Currently reserved for future functionality.
    pub respect_tab_order: bool,
}

impl From<AutoDirectionalNavigation>
    for bevy::ui::auto_directional_navigation::AutoDirectionalNavigation
{
    fn from(v: AutoDirectionalNavigation) -> Self {
        bevy::ui::auto_directional_navigation::AutoDirectionalNavigation {
            respect_tab_order: v.respect_tab_order,
        }
    }
}

impl From<bevy::ui::auto_directional_navigation::AutoDirectionalNavigation>
    for AutoDirectionalNavigation
{
    fn from(v: bevy::ui::auto_directional_navigation::AutoDirectionalNavigation) -> Self {
        AutoDirectionalNavigation {
            respect_tab_order: v.respect_tab_order,
        }
    }
}

mod emit_impls_auto_directional_navigation {
    use super::AutoDirectionalNavigation;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for AutoDirectionalNavigation {
        fn to_code_literal(&self) -> TokenStream {
            let rto = self.respect_tab_order;
            quote::quote! {
                ::elicit_bevy::AutoDirectionalNavigation {
                    respect_tab_order: #rto,
                }
            }
        }
    }
}

shadow_elicitation!(AutoDirectionalNavigation);
