//! Sprite wrappers.
//!
//! Covers [`Anchor`], [`Sprite`], [`SpriteScalingMode`], [`SpriteImageMode`],
//! [`SliceScaleMode`], [`BorderRect`], [`TextureSlicer`], [`Text2d`], [`Text2dShadow`],
//! [`SpritePickingCamera`], [`SpritePickingMode`], and [`SpritePickingSettings`].
//!
//! In Bevy 0.18, [`bevy::sprite::Anchor`] is a newtype wrapping [`bevy::math::Vec2`]
//! with named constants for common positions.

use crate::{Color, Vec2};
use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── Anchor ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::Anchor, as Anchor);
elicit_newtype_traits!(Anchor, bevy::sprite::Anchor, [eq]);

impl From<Anchor> for bevy::sprite::Anchor {
    fn from(v: Anchor) -> Self {
        *v.0
    }
}

impl serde::Serialize for Anchor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let v = self.0.as_vec();
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("x", &v.x)?;
        map.serialize_entry("y", &v.y)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Anchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Anchor;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "x" and "y" fields"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Anchor, A::Error> {
                let mut x: Option<f32> = None;
                let mut y: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = Some(map.next_value()?),
                        "y" => y = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let xv = x.ok_or_else(|| de::Error::missing_field("x"))?;
                let yv = y.ok_or_else(|| de::Error::missing_field("y"))?;
                Ok(Anchor(Arc::new(bevy::sprite::Anchor(
                    bevy::math::Vec2::new(xv, yv),
                ))))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl Anchor {
    /// Returns the underlying position as a [`Vec2`].
    #[tracing::instrument(skip(self))]
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::from(self.0.as_vec())
    }

    /// Returns the center anchor (0, 0).
    #[tracing::instrument(skip(self))]
    pub fn center(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::CENTER)
    }

    /// Returns the bottom-left anchor (-0.5, -0.5).
    #[tracing::instrument(skip(self))]
    pub fn bottom_left(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::BOTTOM_LEFT)
    }

    /// Returns the bottom-center anchor (0, -0.5).
    #[tracing::instrument(skip(self))]
    pub fn bottom_center(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::BOTTOM_CENTER)
    }

    /// Returns the bottom-right anchor (0.5, -0.5).
    #[tracing::instrument(skip(self))]
    pub fn bottom_right(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::BOTTOM_RIGHT)
    }

    /// Returns the center-left anchor (-0.5, 0).
    #[tracing::instrument(skip(self))]
    pub fn center_left(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::CENTER_LEFT)
    }

    /// Returns the center-right anchor (0.5, 0).
    #[tracing::instrument(skip(self))]
    pub fn center_right(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::CENTER_RIGHT)
    }

    /// Returns the top-left anchor (-0.5, 0.5).
    #[tracing::instrument(skip(self))]
    pub fn top_left(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::TOP_LEFT)
    }

    /// Returns the top-center anchor (0, 0.5).
    #[tracing::instrument(skip(self))]
    pub fn top_center(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::TOP_CENTER)
    }

    /// Returns the top-right anchor (0.5, 0.5).
    #[tracing::instrument(skip(self))]
    pub fn top_right(&self) -> Anchor {
        Anchor::from(bevy::sprite::Anchor::TOP_RIGHT)
    }

    /// Constructs an anchor at a custom (x, y) position.
    #[tracing::instrument(skip(self))]
    pub fn custom(&self, x: f32, y: f32) -> Anchor {
        Anchor(Arc::new(bevy::sprite::Anchor(bevy::math::Vec2::new(x, y))))
    }
}

mod emit_impls_anchor {
    use super::Anchor;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Anchor {
        fn to_code_literal(&self) -> TokenStream {
            let v = self.0.as_vec();
            let x = v.x;
            let y = v.y;
            quote::quote! {
                ::elicit_bevy::Anchor::from(
                    ::bevy::sprite::Anchor(::bevy::math::Vec2::new(#x, #y))
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Anchor {}

// ── Sprite ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::Sprite, as Sprite);
elicit_newtype_traits!(Sprite, bevy::sprite::Sprite, []);

impl From<Sprite> for bevy::sprite::Sprite {
    fn from(v: Sprite) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

impl serde::Serialize for Sprite {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let s = &*self.0;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("color", &Color::from(s.color))?;
        map.serialize_entry("flip_x", &s.flip_x)?;
        map.serialize_entry("flip_y", &s.flip_y)?;
        if let Some(sz) = s.custom_size {
            map.serialize_entry("custom_size_x", &sz.x)?;
            map.serialize_entry("custom_size_y", &sz.y)?;
        }
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Sprite {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Sprite;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "a Sprite JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Sprite, A::Error> {
                let mut flip_x: Option<bool> = None;
                let mut flip_y: Option<bool> = None;
                let mut custom_size_x: Option<f32> = None;
                let mut custom_size_y: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "flip_x" => flip_x = Some(map.next_value()?),
                        "flip_y" => flip_y = Some(map.next_value()?),
                        "custom_size_x" => custom_size_x = Some(map.next_value()?),
                        "custom_size_y" => custom_size_y = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let mut s = bevy::sprite::Sprite::default();
                if let Some(v) = flip_x {
                    s.flip_x = v;
                }
                if let Some(v) = flip_y {
                    s.flip_y = v;
                }
                if let (Some(x), Some(y)) = (custom_size_x, custom_size_y) {
                    s.custom_size = Some(bevy::math::Vec2::new(x, y));
                }
                Ok(Sprite(Arc::new(s)))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl Sprite {
    /// Returns `true` if the sprite is flipped horizontally.
    #[tracing::instrument(skip(self))]
    pub fn flip_x(&self) -> bool {
        self.0.flip_x
    }

    /// Returns `true` if the sprite is flipped vertically.
    #[tracing::instrument(skip(self))]
    pub fn flip_y(&self) -> bool {
        self.0.flip_y
    }

    /// Returns the tint color.
    #[tracing::instrument(skip(self))]
    pub fn color(&self) -> Color {
        Color::from(self.0.color)
    }

    /// Returns the custom size if set.
    #[tracing::instrument(skip(self))]
    pub fn custom_size(&self) -> Option<Vec2> {
        self.0.custom_size.map(Vec2::from)
    }

    /// Returns a copy with the given tint color.
    #[tracing::instrument(skip(self))]
    pub fn with_color(&self, c: Color) -> Sprite {
        let mut s = (*self.0).clone();
        s.color = *c.0;
        Sprite::from(s)
    }

    /// Returns a copy with horizontal flip set.
    #[tracing::instrument(skip(self))]
    pub fn with_flip_x(&self, v: bool) -> Sprite {
        let mut s = (*self.0).clone();
        s.flip_x = v;
        Sprite::from(s)
    }

    /// Returns a copy with vertical flip set.
    #[tracing::instrument(skip(self))]
    pub fn with_flip_y(&self, v: bool) -> Sprite {
        let mut s = (*self.0).clone();
        s.flip_y = v;
        Sprite::from(s)
    }
}

mod emit_impls_sprite {
    use super::Sprite;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Sprite {
        fn to_code_literal(&self) -> TokenStream {
            let flip_x = self.0.flip_x;
            let flip_y = self.0.flip_y;
            quote::quote! {
                ::elicit_bevy::Sprite::from({
                    let mut s = ::bevy::sprite::Sprite::default();
                    s.flip_x = #flip_x;
                    s.flip_y = #flip_y;
                    s
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for Sprite {}

// ── SpriteScalingMode ─────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::SpriteScalingMode, as SpriteScalingMode);
elicit_newtype_traits!(SpriteScalingMode, bevy::sprite::SpriteScalingMode, [eq]);

impl From<SpriteScalingMode> for bevy::sprite::SpriteScalingMode {
    fn from(v: SpriteScalingMode) -> Self {
        *v.0
    }
}

impl serde::Serialize for SpriteScalingMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = s.serialize_map(Some(1))?;
        let name = match *self.0 {
            bevy::sprite::SpriteScalingMode::FillCenter => "FillCenter",
            bevy::sprite::SpriteScalingMode::FillStart => "FillStart",
            bevy::sprite::SpriteScalingMode::FillEnd => "FillEnd",
            bevy::sprite::SpriteScalingMode::FitCenter => "FitCenter",
            bevy::sprite::SpriteScalingMode::FitStart => "FitStart",
            bevy::sprite::SpriteScalingMode::FitEnd => "FitEnd",
        };
        map.serialize_entry("variant", name)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for SpriteScalingMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SpriteScalingMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    r#"an object with "variant" field (FillCenter, FillStart, FillEnd)"#
                )
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut map: A,
            ) -> Result<SpriteScalingMode, A::Error> {
                let mut variant: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    if key == "variant" {
                        variant = Some(map.next_value()?);
                    } else {
                        map.next_value::<serde::de::IgnoredAny>()?;
                    }
                }
                let variant = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let inner = match variant.as_str() {
                    "FillCenter" => bevy::sprite::SpriteScalingMode::FillCenter,
                    "FillStart" => bevy::sprite::SpriteScalingMode::FillStart,
                    "FillEnd" => bevy::sprite::SpriteScalingMode::FillEnd,
                    "FitCenter" => bevy::sprite::SpriteScalingMode::FitCenter,
                    "FitStart" => bevy::sprite::SpriteScalingMode::FitStart,
                    "FitEnd" => bevy::sprite::SpriteScalingMode::FitEnd,
                    other => {
                        return Err(de::Error::unknown_variant(
                            other,
                            &[
                                "FillCenter",
                                "FillStart",
                                "FillEnd",
                                "FitCenter",
                                "FitStart",
                                "FitEnd",
                            ],
                        ));
                    }
                };
                Ok(SpriteScalingMode(Arc::new(inner)))
            }
        }
        d.deserialize_map(V)
    }
}

#[reflect_methods]
impl SpriteScalingMode {
    /// Returns a string name for this variant.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        match *self.0 {
            bevy::sprite::SpriteScalingMode::FillCenter => "FillCenter".to_string(),
            bevy::sprite::SpriteScalingMode::FillStart => "FillStart".to_string(),
            bevy::sprite::SpriteScalingMode::FillEnd => "FillEnd".to_string(),
            bevy::sprite::SpriteScalingMode::FitCenter => "FitCenter".to_string(),
            bevy::sprite::SpriteScalingMode::FitStart => "FitStart".to_string(),
            bevy::sprite::SpriteScalingMode::FitEnd => "FitEnd".to_string(),
        }
    }
}

mod emit_impls_sprite_scaling_mode {
    use super::SpriteScalingMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpriteScalingMode {
        fn to_code_literal(&self) -> TokenStream {
            let variant = match *self.0 {
                bevy::sprite::SpriteScalingMode::FillCenter => {
                    quote::format_ident!("FillCenter")
                }
                bevy::sprite::SpriteScalingMode::FillStart => {
                    quote::format_ident!("FillStart")
                }
                bevy::sprite::SpriteScalingMode::FillEnd => {
                    quote::format_ident!("FillEnd")
                }
                bevy::sprite::SpriteScalingMode::FitCenter => {
                    quote::format_ident!("FitCenter")
                }
                bevy::sprite::SpriteScalingMode::FitStart => {
                    quote::format_ident!("FitStart")
                }
                bevy::sprite::SpriteScalingMode::FitEnd => {
                    quote::format_ident!("FitEnd")
                }
            };
            quote::quote! {
                ::elicit_bevy::SpriteScalingMode::from(::bevy::sprite::SpriteScalingMode::#variant)
            }
        }
    }
}

impl elicitation::ElicitComplete for SpriteScalingMode {}

// ── SpriteImageMode ───────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::SpriteImageMode, as SpriteImageMode);
elicit_newtype_traits!(SpriteImageMode, bevy::sprite::SpriteImageMode, [eq]);

impl From<SpriteImageMode> for bevy::sprite::SpriteImageMode {
    fn from(v: SpriteImageMode) -> Self {
        (*v.0).clone()
    }
}

impl serde::Serialize for SpriteImageMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let inner = &*self.0;
        match inner {
            bevy::sprite::SpriteImageMode::Auto => {
                let mut map = s.serialize_map(Some(1))?;
                map.serialize_entry("variant", "Auto")?;
                map.end()
            }
            bevy::sprite::SpriteImageMode::Scale(mode) => {
                let mut map = s.serialize_map(Some(2))?;
                map.serialize_entry("variant", "Scale")?;
                map.serialize_entry("mode", &SpriteScalingMode::from(*mode))?;
                map.end()
            }
            bevy::sprite::SpriteImageMode::Tiled {
                tile_x,
                tile_y,
                stretch_value,
            } => {
                let mut map = s.serialize_map(Some(4))?;
                map.serialize_entry("variant", "Tiled")?;
                map.serialize_entry("tile_x", tile_x)?;
                map.serialize_entry("tile_y", tile_y)?;
                map.serialize_entry("stretch_value", stretch_value)?;
                map.end()
            }
            bevy::sprite::SpriteImageMode::Sliced(_) => {
                let mut map = s.serialize_map(Some(1))?;
                map.serialize_entry("variant", "Sliced")?;
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for SpriteImageMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        use serde::de::{self, MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = SpriteImageMode;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, r#"an object with "variant" field (Auto, Scale, Tiled)"#)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<SpriteImageMode, A::Error> {
                let mut variant: Option<String> = None;
                let mut mode: Option<SpriteScalingMode> = None;
                let mut tile_x: Option<bool> = None;
                let mut tile_y: Option<bool> = None;
                let mut stretch_value: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variant" => variant = Some(map.next_value()?),
                        "mode" => mode = Some(map.next_value()?),
                        "tile_x" => tile_x = Some(map.next_value()?),
                        "tile_y" => tile_y = Some(map.next_value()?),
                        "stretch_value" => stretch_value = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let variant = variant.ok_or_else(|| de::Error::missing_field("variant"))?;
                let inner = match variant.as_str() {
                    "Auto" => bevy::sprite::SpriteImageMode::Auto,
                    "Scale" => {
                        let m = mode.ok_or_else(|| de::Error::missing_field("mode"))?;
                        bevy::sprite::SpriteImageMode::Scale(bevy::sprite::SpriteScalingMode::from(
                            m,
                        ))
                    }
                    "Tiled" => bevy::sprite::SpriteImageMode::Tiled {
                        tile_x: tile_x.ok_or_else(|| de::Error::missing_field("tile_x"))?,
                        tile_y: tile_y.ok_or_else(|| de::Error::missing_field("tile_y"))?,
                        stretch_value: stretch_value
                            .ok_or_else(|| de::Error::missing_field("stretch_value"))?,
                    },
                    other => {
                        return Err(de::Error::unknown_variant(
                            other,
                            &["Auto", "Scale", "Tiled"],
                        ));
                    }
                };
                Ok(SpriteImageMode(Arc::new(inner)))
            }
        }
        d.deserialize_map(V)
    }
}

#[reflect_methods]
impl SpriteImageMode {
    /// Returns a string name for this variant.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> String {
        match &*self.0 {
            bevy::sprite::SpriteImageMode::Auto => "Auto".to_string(),
            bevy::sprite::SpriteImageMode::Scale(_) => "Scale".to_string(),
            bevy::sprite::SpriteImageMode::Sliced(_) => "Sliced".to_string(),
            bevy::sprite::SpriteImageMode::Tiled { .. } => "Tiled".to_string(),
        }
    }

    /// Returns the scaling mode if this is [`SpriteImageMode::Scale`].
    #[tracing::instrument(skip(self))]
    pub fn get_scale_mode(&self) -> Option<SpriteScalingMode> {
        if let bevy::sprite::SpriteImageMode::Scale(m) = *self.0 {
            Some(SpriteScalingMode::from(m))
        } else {
            None
        }
    }
}

mod emit_impls_sprite_image_mode {
    use super::{SpriteImageMode, SpriteScalingMode};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpriteImageMode {
        fn to_code_literal(&self) -> TokenStream {
            match &*self.0 {
                bevy::sprite::SpriteImageMode::Auto => quote::quote! {
                    ::elicit_bevy::SpriteImageMode::from(::bevy::sprite::SpriteImageMode::Auto)
                },
                bevy::sprite::SpriteImageMode::Scale(m) => {
                    let mode = SpriteScalingMode::from(*m).to_code_literal();
                    quote::quote! {
                        ::elicit_bevy::SpriteImageMode::from(
                            ::bevy::sprite::SpriteImageMode::Scale(
                                ::bevy::sprite::SpriteScalingMode::from(#mode)
                            )
                        )
                    }
                }
                bevy::sprite::SpriteImageMode::Tiled {
                    tile_x,
                    tile_y,
                    stretch_value,
                } => {
                    let tx = *tile_x;
                    let ty = *tile_y;
                    let sv = *stretch_value;
                    quote::quote! {
                        ::elicit_bevy::SpriteImageMode::from(
                            ::bevy::sprite::SpriteImageMode::Tiled {
                                tile_x: #tx,
                                tile_y: #ty,
                                stretch_value: #sv,
                            }
                        )
                    }
                }
                bevy::sprite::SpriteImageMode::Sliced(_) => quote::quote! {
                    ::elicit_bevy::SpriteImageMode::from(::bevy::sprite::SpriteImageMode::Auto)
                },
            }
        }
    }
}

impl elicitation::ElicitComplete for SpriteImageMode {}

// ── SliceScaleMode ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::SliceScaleMode, as SliceScaleMode);
elicit_newtype_traits!(SliceScaleMode, bevy::sprite::SliceScaleMode, [eq]);

impl From<SliceScaleMode> for bevy::sprite::SliceScaleMode {
    fn from(v: SliceScaleMode) -> Self {
        *v.0
    }
}

impl serde::Serialize for SliceScaleMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        match *self.0 {
            bevy::sprite::SliceScaleMode::Stretch => serializer.serialize_str("Stretch"),
            bevy::sprite::SliceScaleMode::Tile { stretch_value } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("variant", "Tile")?;
                map.serialize_entry("stretch_value", &stretch_value)?;
                map.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for SliceScaleMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error as _;
        let v = serde_json::Value::deserialize(deserializer)
            .map_err(|e| D::Error::custom(e.to_string()))?;
        let inner = if let Some(s) = v.as_str() {
            match s {
                "Stretch" => bevy::sprite::SliceScaleMode::Stretch,
                other => return Err(D::Error::unknown_variant(other, &["Stretch", "Tile"])),
            }
        } else if let Some(obj) = v.as_object() {
            let variant = obj
                .get("variant")
                .and_then(|v| v.as_str())
                .unwrap_or("Stretch");
            match variant {
                "Tile" => {
                    let stretch_value = obj
                        .get("stretch_value")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(1.0) as f32;
                    bevy::sprite::SliceScaleMode::Tile { stretch_value }
                }
                "Stretch" => bevy::sprite::SliceScaleMode::Stretch,
                other => return Err(D::Error::unknown_variant(other, &["Stretch", "Tile"])),
            }
        } else {
            bevy::sprite::SliceScaleMode::Stretch
        };
        Ok(SliceScaleMode(Arc::new(inner)))
    }
}

#[reflect_methods]
impl SliceScaleMode {
    /// Returns the variant name as a static string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &'static str {
        match *self.0 {
            bevy::sprite::SliceScaleMode::Stretch => "Stretch",
            bevy::sprite::SliceScaleMode::Tile { .. } => "Tile",
        }
    }
}

mod emit_impls_slice_scale_mode {
    use super::SliceScaleMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SliceScaleMode {
        fn to_code_literal(&self) -> TokenStream {
            match *self.0 {
                bevy::sprite::SliceScaleMode::Stretch => quote::quote! {
                    ::elicit_bevy::SliceScaleMode::from(::bevy::sprite::SliceScaleMode::Stretch)
                },
                bevy::sprite::SliceScaleMode::Tile { stretch_value } => quote::quote! {
                    ::elicit_bevy::SliceScaleMode::from(::bevy::sprite::SliceScaleMode::Tile {
                        stretch_value: #stretch_value,
                    })
                },
            }
        }
    }
}

impl elicitation::ElicitComplete for SliceScaleMode {}

// ── BorderRect ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::BorderRect, as BorderRect);
elicit_newtype_traits!(BorderRect, bevy::sprite::BorderRect, [eq]);

impl From<BorderRect> for bevy::sprite::BorderRect {
    fn from(v: BorderRect) -> Self {
        *v.0
    }
}

impl serde::Serialize for BorderRect {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("min_inset_x", &self.0.min_inset.x)?;
        map.serialize_entry("min_inset_y", &self.0.min_inset.y)?;
        map.serialize_entry("max_inset_x", &self.0.max_inset.x)?;
        map.serialize_entry("max_inset_y", &self.0.max_inset.y)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for BorderRect {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = BorderRect;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "an object with min_inset_x, min_inset_y, max_inset_x, max_inset_y"
                )
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<BorderRect, A::Error> {
                let mut mix: Option<f32> = None;
                let mut miy: Option<f32> = None;
                let mut max_x: Option<f32> = None;
                let mut max_y: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "min_inset_x" => mix = Some(map.next_value()?),
                        "min_inset_y" => miy = Some(map.next_value()?),
                        "max_inset_x" => max_x = Some(map.next_value()?),
                        "max_inset_y" => max_y = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(BorderRect(Arc::new(bevy::sprite::BorderRect {
                    min_inset: bevy::math::Vec2::new(mix.unwrap_or(0.0), miy.unwrap_or(0.0)),
                    max_inset: bevy::math::Vec2::new(max_x.unwrap_or(0.0), max_y.unwrap_or(0.0)),
                })))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl BorderRect {
    /// Returns the min inset x value.
    #[tracing::instrument(skip(self))]
    pub fn min_inset_x(&self) -> f32 {
        self.0.min_inset.x
    }

    /// Returns the min inset y value.
    #[tracing::instrument(skip(self))]
    pub fn min_inset_y(&self) -> f32 {
        self.0.min_inset.y
    }

    /// Returns the max inset x value.
    #[tracing::instrument(skip(self))]
    pub fn max_inset_x(&self) -> f32 {
        self.0.max_inset.x
    }

    /// Returns the max inset y value.
    #[tracing::instrument(skip(self))]
    pub fn max_inset_y(&self) -> f32 {
        self.0.max_inset.y
    }
}

mod emit_impls_border_rect {
    use super::BorderRect;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for BorderRect {
        fn to_code_literal(&self) -> TokenStream {
            let mix = self.0.min_inset.x;
            let miy = self.0.min_inset.y;
            let max_x = self.0.max_inset.x;
            let max_y = self.0.max_inset.y;
            quote::quote! {
                ::elicit_bevy::BorderRect::from(::bevy::sprite::BorderRect {
                    min_inset: ::bevy::math::Vec2::new(#mix, #miy),
                    max_inset: ::bevy::math::Vec2::new(#max_x, #max_y),
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for BorderRect {}

// ── TextureSlicer ─────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::TextureSlicer, as TextureSlicer);
elicit_newtype_traits!(TextureSlicer, bevy::sprite::TextureSlicer, [eq]);

impl From<TextureSlicer> for bevy::sprite::TextureSlicer {
    fn from(v: TextureSlicer) -> Self {
        (*v.0).clone()
    }
}

impl serde::Serialize for TextureSlicer {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("border", &BorderRect::from(self.0.border))?;
        map.serialize_entry(
            "center_scale_mode",
            &SliceScaleMode::from(self.0.center_scale_mode),
        )?;
        map.serialize_entry(
            "sides_scale_mode",
            &SliceScaleMode::from(self.0.sides_scale_mode),
        )?;
        map.serialize_entry("max_corner_scale", &self.0.max_corner_scale)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for TextureSlicer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = TextureSlicer;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "TextureSlicer object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<TextureSlicer, A::Error> {
                let mut border: Option<BorderRect> = None;
                let mut center: Option<SliceScaleMode> = None;
                let mut sides: Option<SliceScaleMode> = None;
                let mut max_corner: Option<f32> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "border" => border = Some(map.next_value()?),
                        "center_scale_mode" => center = Some(map.next_value()?),
                        "sides_scale_mode" => sides = Some(map.next_value()?),
                        "max_corner_scale" => max_corner = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let br = border
                    .map(bevy::sprite::BorderRect::from)
                    .unwrap_or_default();
                let csm = center
                    .map(bevy::sprite::SliceScaleMode::from)
                    .unwrap_or(bevy::sprite::SliceScaleMode::Stretch);
                let ssm = sides
                    .map(bevy::sprite::SliceScaleMode::from)
                    .unwrap_or(bevy::sprite::SliceScaleMode::Stretch);
                Ok(TextureSlicer(Arc::new(bevy::sprite::TextureSlicer {
                    border: br,
                    center_scale_mode: csm,
                    sides_scale_mode: ssm,
                    max_corner_scale: max_corner.unwrap_or(1.0),
                })))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl TextureSlicer {
    /// Returns the max corner scale.
    #[tracing::instrument(skip(self))]
    pub fn max_corner_scale(&self) -> f32 {
        self.0.max_corner_scale
    }
}

mod emit_impls_texture_slicer {
    use super::{BorderRect, SliceScaleMode, TextureSlicer};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for TextureSlicer {
        fn to_code_literal(&self) -> TokenStream {
            let border = BorderRect::from(self.0.border).to_code_literal();
            let center = SliceScaleMode::from(self.0.center_scale_mode).to_code_literal();
            let sides = SliceScaleMode::from(self.0.sides_scale_mode).to_code_literal();
            let max_c = self.0.max_corner_scale;
            quote::quote! {
                ::elicit_bevy::TextureSlicer::from(::bevy::sprite::TextureSlicer {
                    border: ::bevy::sprite::BorderRect::from(#border),
                    center_scale_mode: ::bevy::sprite::SliceScaleMode::from(#center),
                    sides_scale_mode: ::bevy::sprite::SliceScaleMode::from(#sides),
                    max_corner_scale: #max_c,
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for TextureSlicer {}

// ── Text2d ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::Text2d, as Text2d);
elicit_newtype_traits!(Text2d, bevy::sprite::Text2d, []);

impl From<Text2d> for bevy::sprite::Text2d {
    fn from(v: Text2d) -> Self {
        bevy::sprite::Text2d(v.0.0.clone())
    }
}

impl serde::Serialize for Text2d {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.0.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Text2d {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Text2d(Arc::new(bevy::sprite::Text2d(s))))
    }
}

#[reflect_methods]
impl Text2d {
    /// Returns the text string.
    #[tracing::instrument(skip(self))]
    pub fn as_str(&self) -> &str {
        &self.0.0
    }
}

mod emit_impls_text2d {
    use super::Text2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Text2d {
        fn to_code_literal(&self) -> TokenStream {
            let s = &self.0.0;
            quote::quote! {
                ::elicit_bevy::Text2d::from(::bevy::sprite::Text2d(#s.to_string()))
            }
        }
    }
}

impl elicitation::ElicitComplete for Text2d {}

// ── Text2dShadow ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::sprite::Text2dShadow, as Text2dShadow);
elicit_newtype_traits!(Text2dShadow, bevy::sprite::Text2dShadow, [eq]);

impl From<Text2dShadow> for bevy::sprite::Text2dShadow {
    fn from(v: Text2dShadow) -> Self {
        *v.0
    }
}

impl serde::Serialize for Text2dShadow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(3))?;
        map.serialize_entry("offset_x", &self.0.offset.x)?;
        map.serialize_entry("offset_y", &self.0.offset.y)?;
        map.serialize_entry("color", &Color::from(self.0.color))?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Text2dShadow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Text2dShadow;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Text2dShadow object with offset_x, offset_y, color")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Text2dShadow, A::Error> {
                let mut ox: Option<f32> = None;
                let mut oy: Option<f32> = None;
                let mut color: Option<Color> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "offset_x" => ox = Some(map.next_value()?),
                        "offset_y" => oy = Some(map.next_value()?),
                        "color" => color = Some(map.next_value()?),
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Text2dShadow(Arc::new(bevy::sprite::Text2dShadow {
                    offset: bevy::math::Vec2::new(ox.unwrap_or(0.0), oy.unwrap_or(0.0)),
                    color: color
                        .map(bevy::color::Color::from)
                        .unwrap_or(bevy::color::Color::BLACK),
                })))
            }
        }
        deserializer.deserialize_map(V)
    }
}

#[reflect_methods]
impl Text2dShadow {
    /// Returns the x offset.
    #[tracing::instrument(skip(self))]
    pub fn offset_x(&self) -> f32 {
        self.0.offset.x
    }

    /// Returns the y offset.
    #[tracing::instrument(skip(self))]
    pub fn offset_y(&self) -> f32 {
        self.0.offset.y
    }
}

mod emit_impls_text2d_shadow {
    use super::{Color, Text2dShadow};
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Text2dShadow {
        fn to_code_literal(&self) -> TokenStream {
            let ox = self.0.offset.x;
            let oy = self.0.offset.y;
            let color = Color::from(self.0.color).to_code_literal();
            quote::quote! {
                ::elicit_bevy::Text2dShadow::from(::bevy::sprite::Text2dShadow {
                    offset: ::bevy::math::Vec2::new(#ox, #oy),
                    color: ::bevy::render::color::Color::from(#color),
                })
            }
        }
    }
}

impl elicitation::ElicitComplete for Text2dShadow {}

// ── shadow_elicitation + unit_elicitation macros ──────────────────────────────

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

// ── SpritePickingCamera ───────────────────────────────────────────────────────

/// Shadow of [`bevy::sprite::SpritePickingCamera`].
///
/// Marker component. When [`SpritePickingSettings::require_markers`] is `true`,
/// only cameras tagged with this component participate in sprite picking.
#[derive(
    Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub struct SpritePickingCamera;

impl From<SpritePickingCamera> for bevy::sprite::SpritePickingCamera {
    fn from(_: SpritePickingCamera) -> Self {
        bevy::sprite::SpritePickingCamera
    }
}

mod emit_impls_sprite_picking_camera {
    use super::SpritePickingCamera;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpritePickingCamera {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::elicit_bevy::SpritePickingCamera }
        }
    }
}

unit_elicitation!(SpritePickingCamera, bevy::sprite::SpritePickingCamera);

// ── SpritePickingMode ─────────────────────────────────────────────────────────

/// Shadow of [`bevy::sprite::SpritePickingMode`].
///
/// Controls how transparent pixels are treated during sprite picking.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub enum SpritePickingMode {
    /// Use only the bounding box; transparent pixels still count as hits.
    BoundingBox,
    /// Ignore pixels with alpha below the given threshold (inclusive).
    AlphaThreshold(f32),
}

impl Default for SpritePickingMode {
    fn default() -> Self {
        Self::AlphaThreshold(0.1)
    }
}

impl From<SpritePickingMode> for bevy::sprite::SpritePickingMode {
    fn from(v: SpritePickingMode) -> Self {
        match v {
            SpritePickingMode::BoundingBox => bevy::sprite::SpritePickingMode::BoundingBox,
            SpritePickingMode::AlphaThreshold(t) => {
                bevy::sprite::SpritePickingMode::AlphaThreshold(t)
            }
        }
    }
}

impl From<bevy::sprite::SpritePickingMode> for SpritePickingMode {
    fn from(v: bevy::sprite::SpritePickingMode) -> Self {
        match v {
            bevy::sprite::SpritePickingMode::BoundingBox => SpritePickingMode::BoundingBox,
            bevy::sprite::SpritePickingMode::AlphaThreshold(t) => {
                SpritePickingMode::AlphaThreshold(t)
            }
        }
    }
}

mod emit_impls_sprite_picking_mode {
    use super::SpritePickingMode;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpritePickingMode {
        fn to_code_literal(&self) -> TokenStream {
            match self {
                SpritePickingMode::BoundingBox => {
                    quote::quote! { ::elicit_bevy::SpritePickingMode::BoundingBox }
                }
                SpritePickingMode::AlphaThreshold(t) => {
                    quote::quote! { ::elicit_bevy::SpritePickingMode::AlphaThreshold(#t) }
                }
            }
        }
    }
}

shadow_elicitation!(SpritePickingMode);

// ── SpritePickingSettings ─────────────────────────────────────────────────────

/// Shadow of [`bevy::sprite::SpritePickingSettings`].
///
/// Runtime resource controlling sprite picking behavior.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SpritePickingSettings {
    /// When `true`, only cameras marked with [`SpritePickingCamera`] participate in picking.
    pub require_markers: bool,
    /// How to handle transparent pixels during picking.
    pub picking_mode: SpritePickingMode,
}

impl Default for SpritePickingSettings {
    fn default() -> Self {
        Self {
            require_markers: false,
            picking_mode: SpritePickingMode::AlphaThreshold(0.1),
        }
    }
}

impl From<SpritePickingSettings> for bevy::sprite::SpritePickingSettings {
    fn from(v: SpritePickingSettings) -> Self {
        bevy::sprite::SpritePickingSettings {
            require_markers: v.require_markers,
            picking_mode: v.picking_mode.into(),
        }
    }
}

mod emit_impls_sprite_picking_settings {
    use super::SpritePickingSettings;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for SpritePickingSettings {
        fn to_code_literal(&self) -> TokenStream {
            let require_markers = self.require_markers;
            let mode = self.picking_mode.to_code_literal();
            quote::quote! {
                ::elicit_bevy::SpritePickingSettings {
                    require_markers: #require_markers,
                    picking_mode: #mode,
                }
            }
        }
    }
}

shadow_elicitation!(SpritePickingSettings);

// ── TextureSliceGenerator ─────────────────────────────────────────────────────

/// Generator for [`bevy::sprite::TextureSlice`] values.
///
/// `TextureSlice` describes a region of a texture atlas to draw, produced at
/// runtime by the 9-slicing system. This generator captures the slice geometry
/// explicitly so that a specific slice can be authored directly.
///
/// Coordinates are in texture-space pixels.
///
/// # Example
///
/// ```rust,ignore
/// use elicit_bevy::TextureSliceGenerator;
/// use elicitation::Generator;
///
/// let gen = TextureSliceGenerator {
///     rect_min: [0.0, 0.0],
///     rect_max: [32.0, 32.0],
///     draw_size: [64.0, 64.0],
///     offset: [0.0, 0.0],
/// };
/// let slice = gen.generate();
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation::Elicit,
)]
pub struct TextureSliceGenerator {
    /// Top-left corner of the texture rectangle `[x, y]`.
    pub rect_min: [f32; 2],
    /// Bottom-right corner of the texture rectangle `[x, y]`.
    pub rect_max: [f32; 2],
    /// Size at which to draw the slice `[width, height]`.
    pub draw_size: [f32; 2],
    /// Draw offset from the entity origin `[x, y]`.
    pub offset: [f32; 2],
}

impl elicitation::Generator for TextureSliceGenerator {
    type Target = bevy::sprite::TextureSlice;

    #[tracing::instrument(skip(self))]
    fn generate(&self) -> bevy::sprite::TextureSlice {
        bevy::sprite::TextureSlice {
            texture_rect: bevy::math::Rect {
                min: bevy::math::Vec2::new(self.rect_min[0], self.rect_min[1]),
                max: bevy::math::Vec2::new(self.rect_max[0], self.rect_max[1]),
            },
            draw_size: bevy::math::Vec2::new(self.draw_size[0], self.draw_size[1]),
            offset: bevy::math::Vec2::new(self.offset[0], self.offset[1]),
        }
    }
}

