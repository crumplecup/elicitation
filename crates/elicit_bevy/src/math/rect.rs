//! Rectangle newtypes: `Rect`, `IRect`, `URect`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Rect ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Rect, as Rect);
elicit_newtype_traits!(Rect, bevy::math::Rect, [eq]);

impl serde::Serialize for Rect {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Rect {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Rect::deserialize(d).map(|v| Rect(std::sync::Arc::new(v)))
    }
}
impl From<Rect> for bevy::math::Rect {
    fn from(v: Rect) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Rect {
    /// Min X.
    #[tracing::instrument(skip(self))]
    pub fn rect_min_x(&self) -> f32 {
        self.0.min.x
    }
    /// Min Y.
    #[tracing::instrument(skip(self))]
    pub fn rect_min_y(&self) -> f32 {
        self.0.min.y
    }
    /// Max X.
    #[tracing::instrument(skip(self))]
    pub fn rect_max_x(&self) -> f32 {
        self.0.max.x
    }
    /// Max Y.
    #[tracing::instrument(skip(self))]
    pub fn rect_max_y(&self) -> f32 {
        self.0.max.y
    }
    /// Width.
    #[tracing::instrument(skip(self))]
    pub fn rect_width(&self) -> f32 {
        self.0.width()
    }
    /// Height.
    #[tracing::instrument(skip(self))]
    pub fn rect_height(&self) -> f32 {
        self.0.height()
    }
    /// Returns `true` if the rectangle contains the given point.
    #[tracing::instrument(skip(self))]
    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.0.contains(bevy::math::Vec2::new(x, y))
    }
    /// Constructs from min/max corners (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn rect_new(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Rect {
        Rect::from(bevy::math::Rect::new(min_x, min_y, max_x, max_y))
    }
    /// Constructs from center and half-extents (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_center_half_size(&self, cx: f32, cy: f32, hx: f32, hy: f32) -> Rect {
        Rect::from(bevy::math::Rect::from_center_half_size(
            bevy::math::Vec2::new(cx, cy),
            bevy::math::Vec2::new(hx, hy),
        ))
    }
}

mod emit_impls {
    use super::Rect;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Rect {
        fn to_code_literal(&self) -> TokenStream {
            let (min_x, min_y) = (self.0.min.x, self.0.min.y);
            let (max_x, max_y) = (self.0.max.x, self.0.max.y);
            quote::quote! { ::bevy::math::Rect::new(#min_x, #min_y, #max_x, #max_y) }
        }
    }
}
impl elicitation::ElicitComplete for Rect {}

// ── IRect ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::IRect, as IRect);
elicit_newtype_traits!(IRect, bevy::math::IRect, [eq]);

impl serde::Serialize for IRect {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for IRect {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::IRect::deserialize(d).map(|v| IRect(std::sync::Arc::new(v)))
    }
}
impl From<IRect> for bevy::math::IRect {
    fn from(v: IRect) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl IRect {
    /// Min X.
    #[tracing::instrument(skip(self))]
    pub fn i_rect_min_x(&self) -> i32 {
        self.0.min.x
    }
    /// Min Y.
    #[tracing::instrument(skip(self))]
    pub fn i_rect_min_y(&self) -> i32 {
        self.0.min.y
    }
    /// Max X.
    #[tracing::instrument(skip(self))]
    pub fn i_rect_max_x(&self) -> i32 {
        self.0.max.x
    }
    /// Max Y.
    #[tracing::instrument(skip(self))]
    pub fn i_rect_max_y(&self) -> i32 {
        self.0.max.y
    }
    /// Width.
    #[tracing::instrument(skip(self))]
    pub fn i_rect_width(&self) -> i32 {
        self.0.width()
    }
    /// Height.
    #[tracing::instrument(skip(self))]
    pub fn i_rect_height(&self) -> i32 {
        self.0.height()
    }
    /// Constructs from min/max corners (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn i_rect_new(&self, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> IRect {
        IRect::from(bevy::math::IRect::new(min_x, min_y, max_x, max_y))
    }
}

mod emit_impls_irect {
    use super::IRect;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for IRect {
        fn to_code_literal(&self) -> TokenStream {
            let (min_x, min_y) = (self.0.min.x, self.0.min.y);
            let (max_x, max_y) = (self.0.max.x, self.0.max.y);
            quote::quote! { ::bevy::math::IRect::new(#min_x, #min_y, #max_x, #max_y) }
        }
    }
}
impl elicitation::ElicitComplete for IRect {}

// ── URect ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::URect, as URect);
elicit_newtype_traits!(URect, bevy::math::URect, [eq]);

impl serde::Serialize for URect {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for URect {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::URect::deserialize(d).map(|v| URect(std::sync::Arc::new(v)))
    }
}
impl From<URect> for bevy::math::URect {
    fn from(v: URect) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl URect {
    /// Min X.
    #[tracing::instrument(skip(self))]
    pub fn u_rect_min_x(&self) -> u32 {
        self.0.min.x
    }
    /// Min Y.
    #[tracing::instrument(skip(self))]
    pub fn u_rect_min_y(&self) -> u32 {
        self.0.min.y
    }
    /// Max X.
    #[tracing::instrument(skip(self))]
    pub fn u_rect_max_x(&self) -> u32 {
        self.0.max.x
    }
    /// Max Y.
    #[tracing::instrument(skip(self))]
    pub fn u_rect_max_y(&self) -> u32 {
        self.0.max.y
    }
    /// Width.
    #[tracing::instrument(skip(self))]
    pub fn u_rect_width(&self) -> u32 {
        self.0.width()
    }
    /// Height.
    #[tracing::instrument(skip(self))]
    pub fn u_rect_height(&self) -> u32 {
        self.0.height()
    }
    /// Constructs from min/max corners (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn u_rect_new(&self, min_x: u32, min_y: u32, max_x: u32, max_y: u32) -> URect {
        URect::from(bevy::math::URect::new(min_x, min_y, max_x, max_y))
    }
}

mod emit_impls_urect {
    use super::URect;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for URect {
        fn to_code_literal(&self) -> TokenStream {
            let (min_x, min_y) = (self.0.min.x, self.0.min.y);
            let (max_x, max_y) = (self.0.max.x, self.0.max.y);
            quote::quote! { ::bevy::math::URect::new(#min_x, #min_y, #max_x, #max_y) }
        }
    }
}
impl elicitation::ElicitComplete for URect {}
