//! Bevy color shadow types.
//!
//! Covers `Color` (enum) and all inner color-space structs.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

/// Shared boilerplate for all plain color-space newtypes (all Copy, PartialEq, no Hash).
macro_rules! color_space_newtype {
    ($name:ident, $upstream:path) => {
        elicit_newtype!($upstream, as $name);
        // f32 fields → PartialEq only; no Hash
        elicit_newtype_traits!($name, $upstream, [eq]);

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                (*self.0).serialize(s)
            }
        }
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                <$upstream>::deserialize(d).map(|v| $name(Arc::new(v)))
            }
        }
        /// Unwrap to bevy type; all color-space structs are `Copy`.
        impl From<$name> for $upstream {
            fn from(v: $name) -> Self {
                *v.0
            }
        }
        impl elicitation::ElicitComplete for $name {}
    };
}

// ── Color spaces ──────────────────────────────────────────────────────────────

color_space_newtype!(Srgba, bevy::color::Srgba);

#[reflect_methods]
impl Srgba {
    /// Red channel `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_red(&self) -> f32 {
        self.0.red
    }

    /// Green channel `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_green(&self) -> f32 {
        self.0.green
    }

    /// Blue channel `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_blue(&self) -> f32 {
        self.0.blue
    }

    /// Alpha channel `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with the red channel set to `red`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_with_red(&self, red: f32) -> Srgba {
        Srgba::from((*self.0).with_red(red))
    }

    /// Returns a copy with the green channel set to `green`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_with_green(&self, green: f32) -> Srgba {
        Srgba::from((*self.0).with_green(green))
    }

    /// Returns a copy with the blue channel set to `blue`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_with_blue(&self, blue: f32) -> Srgba {
        Srgba::from((*self.0).with_blue(blue))
    }

    /// Returns a copy with the alpha channel set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn srgba_with_alpha(&self, alpha: f32) -> Srgba {
        use bevy::color::Alpha;
        Srgba::from(self.0.with_alpha(alpha))
    }

    /// Returns the RGBA components as `[R, G, B, A]` bytes.
    #[tracing::instrument(skip(self))]
    pub fn to_u8_array(&self) -> Vec<u8> {
        use bevy::color::ColorToPacked;
        (*self.0).to_u8_array().to_vec()
    }

    /// Returns the RGBA components as `[R, G, B, A]` floats.
    #[tracing::instrument(skip(self))]
    pub fn to_f32_array(&self) -> Vec<f32> {
        use bevy::color::ColorToComponents;
        (*self.0).to_f32_array().to_vec()
    }

    /// Returns the CSS hex representation, e.g. `"#FF8800"`.
    #[tracing::instrument(skip(self))]
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    /// Parses an `Srgba` from a CSS hex string. The `&self` receiver is unused.
    #[tracing::instrument(skip(self))]
    pub fn from_hex(&self, hex: &str) -> Option<Srgba> {
        bevy::color::Srgba::hex(hex).ok().map(Srgba::from)
    }

    /// Constructs from r, g, b, a (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn srgba_new(&self, r: f32, g: f32, b: f32, a: f32) -> Srgba {
        Srgba::from(bevy::color::Srgba {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        })
    }

    /// Returns `true` if the color is fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn srgba_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if the color is fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn srgba_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor` in sRGB space.
    #[tracing::instrument(skip(self))]
    pub fn srgba_lerp(&self, other: Srgba, factor: f32) -> Srgba {
        use bevy::color::Mix;
        Srgba::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_srgba {
    use super::Srgba;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Srgba {
        fn to_code_literal(&self) -> TokenStream {
            let (r, g, b, a) = (self.0.red, self.0.green, self.0.blue, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Srgba::from(::bevy::color::Srgba {
                    red: #r, green: #g, blue: #b, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(LinearRgba, bevy::color::LinearRgba);

#[reflect_methods]
impl LinearRgba {
    /// Red channel.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_red(&self) -> f32 {
        self.0.red
    }

    /// Green channel.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_green(&self) -> f32 {
        self.0.green
    }

    /// Blue channel.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_blue(&self) -> f32 {
        self.0.blue
    }

    /// Alpha channel.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with the red channel set to `red`.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_with_red(&self, red: f32) -> LinearRgba {
        LinearRgba::from((*self.0).with_red(red))
    }

    /// Returns a copy with the green channel set to `green`.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_with_green(&self, green: f32) -> LinearRgba {
        LinearRgba::from((*self.0).with_green(green))
    }

    /// Returns a copy with the blue channel set to `blue`.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_with_blue(&self, blue: f32) -> LinearRgba {
        LinearRgba::from((*self.0).with_blue(blue))
    }

    /// Returns a copy with the alpha channel set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_with_alpha(&self, alpha: f32) -> LinearRgba {
        use bevy::color::Alpha;
        LinearRgba::from(self.0.with_alpha(alpha))
    }

    /// Perceptual luminance (weighted sum of linear RGB).
    #[tracing::instrument(skip(self))]
    pub fn luminance(&self) -> f32 {
        use bevy::color::Luminance;
        self.0.luminance()
    }

    /// Constructs from r, g, b, a (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_new(&self, r: f32, g: f32, b: f32, a: f32) -> LinearRgba {
        LinearRgba::from(bevy::color::LinearRgba {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        })
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn linear_rgba_lerp(&self, other: LinearRgba, factor: f32) -> LinearRgba {
        use bevy::color::Mix;
        LinearRgba::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_lrgba {
    use super::LinearRgba;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for LinearRgba {
        fn to_code_literal(&self) -> TokenStream {
            let (r, g, b, a) = (self.0.red, self.0.green, self.0.blue, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::LinearRgba::from(::bevy::color::LinearRgba {
                    red: #r, green: #g, blue: #b, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(Hsla, bevy::color::Hsla);

#[reflect_methods]
impl Hsla {
    /// Hue `[0, 360)`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_hue(&self) -> f32 {
        self.0.hue
    }

    /// Saturation `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_saturation(&self) -> f32 {
        self.0.saturation
    }

    /// Lightness `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_lightness(&self) -> f32 {
        self.0.lightness
    }

    /// Alpha `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with hue set to `hue` degrees.
    #[tracing::instrument(skip(self))]
    pub fn hsla_with_hue(&self, hue: f32) -> Hsla {
        use bevy::color::Hue;
        Hsla::from(self.0.with_hue(hue))
    }

    /// Returns a copy with saturation set to `saturation`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_with_saturation(&self, saturation: f32) -> Hsla {
        Hsla::from((*self.0).with_saturation(saturation))
    }

    /// Returns a copy with lightness set to `lightness`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_with_lightness(&self, lightness: f32) -> Hsla {
        Hsla::from((*self.0).with_lightness(lightness))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_with_alpha(&self, alpha: f32) -> Hsla {
        use bevy::color::Alpha;
        Hsla::from(self.0.with_alpha(alpha))
    }

    /// Rotates the hue by `degrees`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_rotate_hue(&self, degrees: f32) -> Hsla {
        use bevy::color::Hue;
        Hsla::from((*self.0).rotate_hue(degrees))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn hsla_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn hsla_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn hsla_lerp(&self, other: Hsla, factor: f32) -> Hsla {
        use bevy::color::Mix;
        Hsla::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_hsla {
    use super::Hsla;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Hsla {
        fn to_code_literal(&self) -> TokenStream {
            let (h, s, l, a) = (
                self.0.hue,
                self.0.saturation,
                self.0.lightness,
                self.0.alpha,
            );
            quote::quote! {
                ::elicit_bevy::Hsla::from(::bevy::color::Hsla {
                    hue: #h, saturation: #s, lightness: #l, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(Hsva, bevy::color::Hsva);

#[reflect_methods]
impl Hsva {
    /// Hue.
    #[tracing::instrument(skip(self))]
    pub fn hsva_hue(&self) -> f32 {
        self.0.hue
    }

    /// Saturation.
    #[tracing::instrument(skip(self))]
    pub fn hsva_saturation(&self) -> f32 {
        self.0.saturation
    }

    /// Value (brightness) `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn value(&self) -> f32 {
        self.0.value
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn hsva_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with hue set to `hue` degrees.
    #[tracing::instrument(skip(self))]
    pub fn hsva_with_hue(&self, hue: f32) -> Hsva {
        use bevy::color::Hue;
        Hsva::from(self.0.with_hue(hue))
    }

    /// Returns a copy with saturation set to `saturation`.
    #[tracing::instrument(skip(self))]
    pub fn hsva_with_saturation(&self, saturation: f32) -> Hsva {
        Hsva::from((*self.0).with_saturation(saturation))
    }

    /// Returns a copy with value set to `value`.
    #[tracing::instrument(skip(self))]
    pub fn with_value(&self, value: f32) -> Hsva {
        Hsva::from((*self.0).with_value(value))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn hsva_with_alpha(&self, alpha: f32) -> Hsva {
        use bevy::color::Alpha;
        Hsva::from(self.0.with_alpha(alpha))
    }

    /// Rotates the hue by `degrees`.
    #[tracing::instrument(skip(self))]
    pub fn hsva_rotate_hue(&self, degrees: f32) -> Hsva {
        use bevy::color::Hue;
        Hsva::from((*self.0).rotate_hue(degrees))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn hsva_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn hsva_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn hsva_lerp(&self, other: Hsva, factor: f32) -> Hsva {
        use bevy::color::Mix;
        Hsva::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_hsva {
    use super::Hsva;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Hsva {
        fn to_code_literal(&self) -> TokenStream {
            let (h, s, v, a) = (self.0.hue, self.0.saturation, self.0.value, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Hsva::from(::bevy::color::Hsva {
                    hue: #h, saturation: #s, value: #v, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(Hwba, bevy::color::Hwba);

#[reflect_methods]
impl Hwba {
    /// Hue.
    #[tracing::instrument(skip(self))]
    pub fn hwba_hue(&self) -> f32 {
        self.0.hue
    }

    /// Whiteness `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn whiteness(&self) -> f32 {
        self.0.whiteness
    }

    /// Blackness `[0, 1]`.
    #[tracing::instrument(skip(self))]
    pub fn blackness(&self) -> f32 {
        self.0.blackness
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn hwba_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with hue set to `hue` degrees.
    #[tracing::instrument(skip(self))]
    pub fn hwba_with_hue(&self, hue: f32) -> Hwba {
        use bevy::color::Hue;
        Hwba::from(self.0.with_hue(hue))
    }

    /// Returns a copy with whiteness set to `whiteness`.
    #[tracing::instrument(skip(self))]
    pub fn with_whiteness(&self, whiteness: f32) -> Hwba {
        Hwba::from((*self.0).with_whiteness(whiteness))
    }

    /// Returns a copy with blackness set to `blackness`.
    #[tracing::instrument(skip(self))]
    pub fn with_blackness(&self, blackness: f32) -> Hwba {
        Hwba::from((*self.0).with_blackness(blackness))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn hwba_with_alpha(&self, alpha: f32) -> Hwba {
        use bevy::color::Alpha;
        Hwba::from(self.0.with_alpha(alpha))
    }

    /// Rotates the hue by `degrees`.
    #[tracing::instrument(skip(self))]
    pub fn hwba_rotate_hue(&self, degrees: f32) -> Hwba {
        use bevy::color::Hue;
        Hwba::from((*self.0).rotate_hue(degrees))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn hwba_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn hwba_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn hwba_lerp(&self, other: Hwba, factor: f32) -> Hwba {
        use bevy::color::Mix;
        Hwba::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_hwba {
    use super::Hwba;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Hwba {
        fn to_code_literal(&self) -> TokenStream {
            let (h, w, b, a) = (self.0.hue, self.0.whiteness, self.0.blackness, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Hwba::from(::bevy::color::Hwba {
                    hue: #h, whiteness: #w, blackness: #b, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(Laba, bevy::color::Laba);

#[reflect_methods]
impl Laba {
    /// Lightness L*.
    #[tracing::instrument(skip(self))]
    pub fn laba_lightness(&self) -> f32 {
        self.0.lightness
    }

    /// a* axis (green-red).
    #[tracing::instrument(skip(self))]
    pub fn laba_a(&self) -> f32 {
        self.0.a
    }

    /// b* axis (blue-yellow).
    #[tracing::instrument(skip(self))]
    pub fn laba_b(&self) -> f32 {
        self.0.b
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn laba_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with lightness set to `lightness`.
    #[tracing::instrument(skip(self))]
    pub fn laba_with_lightness(&self, lightness: f32) -> Laba {
        Laba::from((*self.0).with_lightness(lightness))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn laba_with_alpha(&self, alpha: f32) -> Laba {
        use bevy::color::Alpha;
        Laba::from(self.0.with_alpha(alpha))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn laba_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn laba_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn laba_lerp(&self, other: Laba, factor: f32) -> Laba {
        use bevy::color::Mix;
        Laba::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_laba {
    use super::Laba;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Laba {
        fn to_code_literal(&self) -> TokenStream {
            let (l, a, b, al) = (self.0.lightness, self.0.a, self.0.b, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Laba::from(::bevy::color::Laba {
                    lightness: #l, a: #a, b: #b, alpha: #al,
                })
            }
        }
    }
}

color_space_newtype!(Lcha, bevy::color::Lcha);

#[reflect_methods]
impl Lcha {
    /// Lightness.
    #[tracing::instrument(skip(self))]
    pub fn lcha_lightness(&self) -> f32 {
        self.0.lightness
    }

    /// Chroma.
    #[tracing::instrument(skip(self))]
    pub fn lcha_chroma(&self) -> f32 {
        self.0.chroma
    }

    /// Hue in degrees.
    #[tracing::instrument(skip(self))]
    pub fn lcha_hue(&self) -> f32 {
        self.0.hue
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn lcha_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with lightness set to `lightness`.
    #[tracing::instrument(skip(self))]
    pub fn lcha_with_lightness(&self, lightness: f32) -> Lcha {
        Lcha::from((*self.0).with_lightness(lightness))
    }

    /// Returns a copy with chroma set to `chroma`.
    #[tracing::instrument(skip(self))]
    pub fn lcha_with_chroma(&self, chroma: f32) -> Lcha {
        Lcha::from((*self.0).with_chroma(chroma))
    }

    /// Returns a copy with hue set to `hue` degrees.
    #[tracing::instrument(skip(self))]
    pub fn lcha_with_hue(&self, hue: f32) -> Lcha {
        use bevy::color::Hue;
        Lcha::from(self.0.with_hue(hue))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn lcha_with_alpha(&self, alpha: f32) -> Lcha {
        use bevy::color::Alpha;
        Lcha::from(self.0.with_alpha(alpha))
    }

    /// Rotates the hue by `degrees`.
    #[tracing::instrument(skip(self))]
    pub fn lcha_rotate_hue(&self, degrees: f32) -> Lcha {
        use bevy::color::Hue;
        Lcha::from((*self.0).rotate_hue(degrees))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn lcha_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn lcha_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn lcha_lerp(&self, other: Lcha, factor: f32) -> Lcha {
        use bevy::color::Mix;
        Lcha::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_lcha {
    use super::Lcha;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Lcha {
        fn to_code_literal(&self) -> TokenStream {
            let (l, c, h, a) = (self.0.lightness, self.0.chroma, self.0.hue, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Lcha::from(::bevy::color::Lcha {
                    lightness: #l, chroma: #c, hue: #h, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(Oklaba, bevy::color::Oklaba);

#[reflect_methods]
impl Oklaba {
    /// Lightness L.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_lightness(&self) -> f32 {
        self.0.lightness
    }

    /// a channel.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_a(&self) -> f32 {
        self.0.a
    }

    /// b channel.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_b(&self) -> f32 {
        self.0.b
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with lightness set to `lightness`.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_with_lightness(&self, lightness: f32) -> Oklaba {
        Oklaba::from((*self.0).with_lightness(lightness))
    }

    /// Returns a copy with the a channel set to `a`.
    #[tracing::instrument(skip(self))]
    pub fn with_a(&self, a: f32) -> Oklaba {
        Oklaba::from((*self.0).with_a(a))
    }

    /// Returns a copy with the b channel set to `b`.
    #[tracing::instrument(skip(self))]
    pub fn with_b(&self, b: f32) -> Oklaba {
        Oklaba::from((*self.0).with_b(b))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_with_alpha(&self, alpha: f32) -> Oklaba {
        use bevy::color::Alpha;
        Oklaba::from(self.0.with_alpha(alpha))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn oklaba_lerp(&self, other: Oklaba, factor: f32) -> Oklaba {
        use bevy::color::Mix;
        Oklaba::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_oklaba {
    use super::Oklaba;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Oklaba {
        fn to_code_literal(&self) -> TokenStream {
            let (l, a, b, al) = (self.0.lightness, self.0.a, self.0.b, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Oklaba::from(::bevy::color::Oklaba {
                    lightness: #l, a: #a, b: #b, alpha: #al,
                })
            }
        }
    }
}

color_space_newtype!(Oklcha, bevy::color::Oklcha);

#[reflect_methods]
impl Oklcha {
    /// Lightness.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_lightness(&self) -> f32 {
        self.0.lightness
    }

    /// Chroma.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_chroma(&self) -> f32 {
        self.0.chroma
    }

    /// Hue in degrees.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_hue(&self) -> f32 {
        self.0.hue
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with lightness set to `lightness`.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_with_lightness(&self, lightness: f32) -> Oklcha {
        Oklcha::from((*self.0).with_lightness(lightness))
    }

    /// Returns a copy with chroma set to `chroma`.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_with_chroma(&self, chroma: f32) -> Oklcha {
        Oklcha::from((*self.0).with_chroma(chroma))
    }

    /// Returns a copy with hue set to `hue` degrees.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_with_hue(&self, hue: f32) -> Oklcha {
        use bevy::color::Hue;
        Oklcha::from(self.0.with_hue(hue))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_with_alpha(&self, alpha: f32) -> Oklcha {
        use bevy::color::Alpha;
        Oklcha::from(self.0.with_alpha(alpha))
    }

    /// Rotates the hue by `degrees`.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_rotate_hue(&self, degrees: f32) -> Oklcha {
        use bevy::color::Hue;
        Oklcha::from((*self.0).rotate_hue(degrees))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn oklcha_lerp(&self, other: Oklcha, factor: f32) -> Oklcha {
        use bevy::color::Mix;
        Oklcha::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_oklcha {
    use super::Oklcha;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Oklcha {
        fn to_code_literal(&self) -> TokenStream {
            let (l, c, h, a) = (self.0.lightness, self.0.chroma, self.0.hue, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Oklcha::from(::bevy::color::Oklcha {
                    lightness: #l, chroma: #c, hue: #h, alpha: #a,
                })
            }
        }
    }
}

color_space_newtype!(Xyza, bevy::color::Xyza);

#[reflect_methods]
impl Xyza {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn x(&self) -> f32 {
        self.0.x
    }

    /// Y (luminance).
    #[tracing::instrument(skip(self))]
    pub fn y(&self) -> f32 {
        self.0.y
    }

    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn z(&self) -> f32 {
        self.0.z
    }

    /// Alpha.
    #[tracing::instrument(skip(self))]
    pub fn xyza_alpha(&self) -> f32 {
        self.0.alpha
    }

    /// Returns a copy with x set to `x`.
    #[tracing::instrument(skip(self))]
    pub fn with_x(&self, x: f32) -> Xyza {
        Xyza::from((*self.0).with_x(x))
    }

    /// Returns a copy with y set to `y`.
    #[tracing::instrument(skip(self))]
    pub fn with_y(&self, y: f32) -> Xyza {
        Xyza::from((*self.0).with_y(y))
    }

    /// Returns a copy with z set to `z`.
    #[tracing::instrument(skip(self))]
    pub fn with_z(&self, z: f32) -> Xyza {
        Xyza::from((*self.0).with_z(z))
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn xyza_with_alpha(&self, alpha: f32) -> Xyza {
        use bevy::color::Alpha;
        Xyza::from(self.0.with_alpha(alpha))
    }

    /// Returns `true` if fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn xyza_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn xyza_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates towards `other` by `factor`.
    #[tracing::instrument(skip(self))]
    pub fn xyza_lerp(&self, other: Xyza, factor: f32) -> Xyza {
        use bevy::color::Mix;
        Xyza::from((*self.0).mix(&*other.0, factor))
    }
}

mod emit_xyza {
    use super::Xyza;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Xyza {
        fn to_code_literal(&self) -> TokenStream {
            let (x, y, z, a) = (self.0.x, self.0.y, self.0.z, self.0.alpha);
            quote::quote! {
                ::elicit_bevy::Xyza::from(::bevy::color::Xyza {
                    x: #x, y: #y, z: #z, alpha: #a,
                })
            }
        }
    }
}

// ── Color (enum) ──────────────────────────────────────────────────────────────

elicit_newtype!(bevy::color::Color, as Color);
// Color is Copy and has PartialEq but no Hash.
elicit_newtype_traits!(Color, bevy::color::Color, [eq]);

impl serde::Serialize for Color {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::color::Color::deserialize(d).map(|v| Color(Arc::new(v)))
    }
}

/// `Color` is `Copy`, so unwrap via deref.
impl From<Color> for bevy::color::Color {
    fn from(v: Color) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Color {
    /// Converts to sRGBA color space.
    #[tracing::instrument(skip(self))]
    pub fn to_srgba(&self) -> Srgba {
        Srgba::from(self.0.to_srgba())
    }

    /// Converts to linear RGB color space.
    #[tracing::instrument(skip(self))]
    pub fn to_linear(&self) -> LinearRgba {
        LinearRgba::from(self.0.to_linear())
    }

    /// Converts to HSL color space.
    #[tracing::instrument(skip(self))]
    pub fn to_hsla(&self) -> Hsla {
        Hsla::from(bevy::color::Hsla::from(*self.0))
    }

    /// Converts to Oklab color space.
    #[tracing::instrument(skip(self))]
    pub fn to_oklaba(&self) -> Oklaba {
        Oklaba::from(bevy::color::Oklaba::from(*self.0))
    }

    /// Alpha component.
    #[tracing::instrument(skip(self))]
    pub fn color_alpha(&self) -> f32 {
        use bevy::color::Alpha;
        self.0.alpha()
    }

    /// Returns a copy with alpha set to `alpha`.
    #[tracing::instrument(skip(self))]
    pub fn color_with_alpha(&self, alpha: f32) -> Color {
        use bevy::color::Alpha;
        Color::from(self.0.with_alpha(alpha))
    }

    /// Returns `true` if the color is fully transparent.
    #[tracing::instrument(skip(self))]
    pub fn color_is_fully_transparent(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_transparent()
    }

    /// Returns `true` if the color is fully opaque.
    #[tracing::instrument(skip(self))]
    pub fn color_is_fully_opaque(&self) -> bool {
        use bevy::color::Alpha;
        self.0.is_fully_opaque()
    }

    /// Linearly interpolates between `self` and `end` in linear RGB space.
    #[tracing::instrument(skip(self))]
    pub fn linear_slow_transition(&self, end: Color, f: f32) -> Color {
        use bevy::color::Mix;
        Color::from((*self.0).mix(&*end.0, f))
    }

    /// Variant name, e.g. `"Srgba"`, `"LinearRgba"`.
    #[tracing::instrument(skip(self))]
    pub fn variant_name(&self) -> &'static str {
        match *self.0 {
            bevy::color::Color::Srgba(_) => "Srgba",
            bevy::color::Color::LinearRgba(_) => "LinearRgba",
            bevy::color::Color::Hsla(_) => "Hsla",
            bevy::color::Color::Hsva(_) => "Hsva",
            bevy::color::Color::Hwba(_) => "Hwba",
            bevy::color::Color::Laba(_) => "Laba",
            bevy::color::Color::Lcha(_) => "Lcha",
            bevy::color::Color::Oklaba(_) => "Oklaba",
            bevy::color::Color::Oklcha(_) => "Oklcha",
            bevy::color::Color::Xyza(_) => "Xyza",
        }
    }

    /// Returns `true` if this color is stored in the `Srgba` variant.
    #[tracing::instrument(skip(self))]
    pub fn is_srgba(&self) -> bool {
        matches!(*self.0, bevy::color::Color::Srgba(_))
    }

    /// Returns `true` if this color is stored in the `LinearRgba` variant.
    #[tracing::instrument(skip(self))]
    pub fn is_linear_rgba(&self) -> bool {
        matches!(*self.0, bevy::color::Color::LinearRgba(_))
    }

    /// Returns `true` if this color is stored in the `Hsla` variant.
    #[tracing::instrument(skip(self))]
    pub fn is_hsla(&self) -> bool {
        matches!(*self.0, bevy::color::Color::Hsla(_))
    }

    /// Constructs `Color::WHITE` (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn white(&self) -> Color {
        Color::from(bevy::color::Color::WHITE)
    }

    /// Constructs `Color::BLACK` (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn black(&self) -> Color {
        Color::from(bevy::color::Color::BLACK)
    }

    /// Constructs fully transparent `Color::NONE` (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn none(&self) -> Color {
        Color::from(bevy::color::Color::NONE)
    }

    /// Constructs from sRGB r, g, b with alpha = 1.0 (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn srgb(&self, r: f32, g: f32, b: f32) -> Color {
        Color::from(bevy::color::Color::srgb(r, g, b))
    }

    /// Constructs from a CSS hex string (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn srgb_from_hex(&self, hex: &str) -> Option<Color> {
        bevy::color::Srgba::hex(hex)
            .ok()
            .map(|c| Color::from(bevy::color::Color::Srgba(c)))
    }
}

mod emit_color {
    use super::Color;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Color {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(&**self).unwrap_or_default();
            quote::quote! {
                ::elicit_bevy::Color::from(
                    ::serde_json::from_str::<::bevy::color::Color>(#json).unwrap()
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Color {}
