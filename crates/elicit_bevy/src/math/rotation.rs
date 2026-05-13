//! 2D rotation newtype: `Rot2`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(bevy::math::Rot2, as Rot2);
elicit_newtype_traits!(Rot2, bevy::math::Rot2, [eq]);

impl serde::Serialize for Rot2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Rot2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Rot2::deserialize(d).map(|v| Rot2(std::sync::Arc::new(v)))
    }
}
impl From<Rot2> for bevy::math::Rot2 {
    fn from(v: Rot2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Rot2 {
    /// Cosine component.
    #[tracing::instrument(skip(self))]
    pub fn cos(&self) -> f32 {
        self.0.cos
    }
    /// Sine component.
    #[tracing::instrument(skip(self))]
    pub fn sin(&self) -> f32 {
        self.0.sin
    }
    /// Rotation angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn as_radians(&self) -> f32 {
        self.0.as_radians()
    }
    /// Rotation angle in degrees.
    #[tracing::instrument(skip(self))]
    pub fn as_degrees(&self) -> f32 {
        self.0.as_degrees()
    }
    /// Returns `true` if the rotation is finite.
    #[tracing::instrument(skip(self))]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Inverse rotation.
    #[tracing::instrument(skip(self))]
    pub fn inverse(&self) -> Rot2 {
        Rot2::from(self.0.inverse())
    }
    /// Identity rotation (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> Rot2 {
        Rot2::from(bevy::math::Rot2::IDENTITY)
    }
    /// Rotation from angle in radians (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_radians(&self, angle: f32) -> Rot2 {
        Rot2::from(bevy::math::Rot2::radians(angle))
    }
    /// Rotation from angle in degrees (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_degrees(&self, angle: f32) -> Rot2 {
        Rot2::from(bevy::math::Rot2::degrees(angle))
    }
}

mod emit_impls {
    use super::Rot2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Rot2 {
        fn to_code_literal(&self) -> TokenStream {
            let angle = self.0.as_radians();
            quote::quote! { ::bevy::math::Rot2::radians(#angle) }
        }
    }
}
impl elicitation::ElicitComplete for Rot2 {}
