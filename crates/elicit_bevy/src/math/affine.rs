//! Affine transform newtypes: `Affine2`, `Affine3A`, `DAffine2`, `DAffine3`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Affine2 ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Affine2, as Affine2);
elicit_newtype_traits!(Affine2, bevy::math::Affine2, [eq]);

impl serde::Serialize for Affine2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Affine2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Affine2::deserialize(d).map(|v| Affine2(std::sync::Arc::new(v)))
    }
}
impl From<Affine2> for bevy::math::Affine2 {
    fn from(v: Affine2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Affine2 {
    /// Returns `true` if all components are finite.
    #[tracing::instrument(skip(self))]
    pub fn affine2_is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Returns the inverse of this affine transform.
    #[tracing::instrument(skip(self))]
    pub fn affine2_inverse(&self) -> Affine2 {
        Affine2::from(self.0.inverse())
    }
    /// Identity transform (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn affine2_identity(&self) -> Affine2 {
        Affine2::from(bevy::math::Affine2::IDENTITY)
    }
    /// Translation-only affine transform (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn affine2_from_translation(&self, x: f32, y: f32) -> Affine2 {
        Affine2::from(bevy::math::Affine2::from_translation(
            bevy::math::Vec2::new(x, y),
        ))
    }
    /// Uniform scale (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn affine2_from_scale(&self, x: f32, y: f32) -> Affine2 {
        Affine2::from(bevy::math::Affine2::from_scale(bevy::math::Vec2::new(x, y)))
    }
    /// Rotation in radians (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_angle(&self, angle: f32) -> Affine2 {
        Affine2::from(bevy::math::Affine2::from_angle(angle))
    }
}

mod emit_impls {
    use super::Affine2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Affine2 {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::math::Affine2::IDENTITY }
        }
    }
}
impl elicitation::ElicitComplete for Affine2 {}

// ── Affine3A ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Affine3A, as Affine3A);
elicit_newtype_traits!(Affine3A, bevy::math::Affine3A, [eq]);

impl serde::Serialize for Affine3A {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Affine3A {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Affine3A::deserialize(d).map(|v| Affine3A(std::sync::Arc::new(v)))
    }
}
impl From<Affine3A> for bevy::math::Affine3A {
    fn from(v: Affine3A) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Affine3A {
    /// Returns `true` if all components are finite.
    #[tracing::instrument(skip(self))]
    pub fn affine3_a_is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Returns the inverse.
    #[tracing::instrument(skip(self))]
    pub fn affine3_a_inverse(&self) -> Affine3A {
        Affine3A::from(self.0.inverse())
    }
    /// Identity (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn affine3_a_identity(&self) -> Affine3A {
        Affine3A::from(bevy::math::Affine3A::IDENTITY)
    }
    /// Translation-only (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn affine3_a_from_translation(&self, x: f32, y: f32, z: f32) -> Affine3A {
        Affine3A::from(bevy::math::Affine3A::from_translation(
            bevy::math::Vec3::new(x, y, z),
        ))
    }
    /// Uniform scale (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn affine3_a_from_scale(&self, x: f32, y: f32, z: f32) -> Affine3A {
        Affine3A::from(bevy::math::Affine3A::from_scale(bevy::math::Vec3::new(
            x, y, z,
        )))
    }
}

mod emit_impls_affine3a {
    use super::Affine3A;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Affine3A {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::math::Affine3A::IDENTITY }
        }
    }
}
impl elicitation::ElicitComplete for Affine3A {}

// ── DAffine2 ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DAffine2, as DAffine2);
elicit_newtype_traits!(DAffine2, bevy::math::DAffine2, [eq]);

impl serde::Serialize for DAffine2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DAffine2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DAffine2::deserialize(d).map(|v| DAffine2(std::sync::Arc::new(v)))
    }
}
impl From<DAffine2> for bevy::math::DAffine2 {
    fn from(v: DAffine2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DAffine2 {
    /// Returns `true` if all components are finite.
    #[tracing::instrument(skip(self))]
    pub fn d_affine2_is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Identity (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_affine2_identity(&self) -> DAffine2 {
        DAffine2::from(bevy::math::DAffine2::IDENTITY)
    }
}

mod emit_impls_daffine2 {
    use super::DAffine2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DAffine2 {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::math::DAffine2::IDENTITY }
        }
    }
}
impl elicitation::ElicitComplete for DAffine2 {}

// ── DAffine3 ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DAffine3, as DAffine3);
elicit_newtype_traits!(DAffine3, bevy::math::DAffine3, [eq]);

impl serde::Serialize for DAffine3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DAffine3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DAffine3::deserialize(d).map(|v| DAffine3(std::sync::Arc::new(v)))
    }
}
impl From<DAffine3> for bevy::math::DAffine3 {
    fn from(v: DAffine3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DAffine3 {
    /// Returns `true` if all components are finite.
    #[tracing::instrument(skip(self))]
    pub fn d_affine3_is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Identity (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_affine3_identity(&self) -> DAffine3 {
        DAffine3::from(bevy::math::DAffine3::IDENTITY)
    }
}

mod emit_impls_daffine3 {
    use super::DAffine3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DAffine3 {
        fn to_code_literal(&self) -> TokenStream {
            quote::quote! { ::bevy::math::DAffine3::IDENTITY }
        }
    }
}
impl elicitation::ElicitComplete for DAffine3 {}
