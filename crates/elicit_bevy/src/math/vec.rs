//! Glam vector newtypes re-exported from `bevy::math`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Vec2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Vec2, as Vec2);
elicit_newtype_traits!(Vec2, bevy::math::Vec2, [eq]);

impl serde::Serialize for Vec2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Vec2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Vec2::deserialize(d).map(|v| Vec2(std::sync::Arc::new(v)))
    }
}
impl From<Vec2> for bevy::math::Vec2 {
    fn from(v: Vec2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Vec2 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn vec2_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn vec2_y(&self) -> f32 {
        self.0.y
    }
    /// Computes the length.
    #[tracing::instrument(skip(self))]
    pub fn vec2_length(&self) -> f32 {
        self.0.length()
    }
    /// Returns the normalized vector, or zero if length is zero.
    #[tracing::instrument(skip(self))]
    pub fn vec2_normalize_or_zero(&self) -> Vec2 {
        Vec2::from(self.0.normalize_or_zero())
    }
    /// Dot product with another vector.
    #[tracing::instrument(skip(self, other))]
    pub fn vec2_dot(&self, other: Vec2) -> f32 {
        self.0.dot(*other.0)
    }
    /// Returns a new vector with each component clamped.
    #[tracing::instrument(skip(self, min, max))]
    pub fn clamp(&self, min: Vec2, max: Vec2) -> Vec2 {
        Vec2::from(self.0.clamp(*min.0, *max.0))
    }
    /// Distance to another vector.
    #[tracing::instrument(skip(self, other))]
    pub fn vec2_distance(&self, other: Vec2) -> f32 {
        self.0.distance(*other.0)
    }
    /// Constructs from x and y (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vec2_new(&self, x: f32, y: f32) -> Vec2 {
        Vec2::from(bevy::math::Vec2::new(x, y))
    }
    /// Zero vector.
    #[tracing::instrument(skip(self))]
    pub fn vec2_zero(&self) -> Vec2 {
        Vec2::from(bevy::math::Vec2::ZERO)
    }
    /// One vector.
    #[tracing::instrument(skip(self))]
    pub fn vec2_one(&self) -> Vec2 {
        Vec2::from(bevy::math::Vec2::ONE)
    }
}

mod emit_impls {
    use super::Vec2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Vec2 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            quote::quote! { ::bevy::math::Vec2::new(#x, #y) }
        }
    }
}
impl elicitation::ElicitComplete for Vec2 {}

// ── Vec3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Vec3, as Vec3);
elicit_newtype_traits!(Vec3, bevy::math::Vec3, [eq]);

impl serde::Serialize for Vec3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Vec3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Vec3::deserialize(d).map(|v| Vec3(std::sync::Arc::new(v)))
    }
}
impl From<Vec3> for bevy::math::Vec3 {
    fn from(v: Vec3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Vec3 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn vec3_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn vec3_y(&self) -> f32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn vec3_z(&self) -> f32 {
        self.0.z
    }
    /// Computes the length.
    #[tracing::instrument(skip(self))]
    pub fn vec3_length(&self) -> f32 {
        self.0.length()
    }
    /// Returns the normalized vector, or zero if length is zero.
    #[tracing::instrument(skip(self))]
    pub fn vec3_normalize_or_zero(&self) -> Vec3 {
        Vec3::from(self.0.normalize_or_zero())
    }
    /// Dot product with another vector.
    #[tracing::instrument(skip(self, other))]
    pub fn vec3_dot(&self, other: Vec3) -> f32 {
        self.0.dot(*other.0)
    }
    /// Cross product.
    #[tracing::instrument(skip(self, other))]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::from(self.0.cross(*other.0))
    }
    /// Distance to another vector.
    #[tracing::instrument(skip(self, other))]
    pub fn vec3_distance(&self, other: Vec3) -> f32 {
        self.0.distance(*other.0)
    }
    /// Constructs from x, y, z (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vec3_new(&self, x: f32, y: f32, z: f32) -> Vec3 {
        Vec3::from(bevy::math::Vec3::new(x, y, z))
    }
    /// Zero vector.
    #[tracing::instrument(skip(self))]
    pub fn vec3_zero(&self) -> Vec3 {
        Vec3::from(bevy::math::Vec3::ZERO)
    }
    /// One vector.
    #[tracing::instrument(skip(self))]
    pub fn vec3_one(&self) -> Vec3 {
        Vec3::from(bevy::math::Vec3::ONE)
    }
    /// Up direction (0, 1, 0).
    #[tracing::instrument(skip(self))]
    pub fn up(&self) -> Vec3 {
        Vec3::from(bevy::math::Vec3::Y)
    }
    /// Forward direction (0, 0, -1) in right-hand coordinates.
    #[tracing::instrument(skip(self))]
    pub fn forward(&self) -> Vec3 {
        Vec3::from(bevy::math::Vec3::NEG_Z)
    }
}

mod emit_impls_vec3 {
    use super::Vec3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Vec3 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! { ::bevy::math::Vec3::new(#x, #y, #z) }
        }
    }
}
impl elicitation::ElicitComplete for Vec3 {}

// ── Vec3A ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Vec3A, as Vec3A);
elicit_newtype_traits!(Vec3A, bevy::math::Vec3A, [eq]);

impl serde::Serialize for Vec3A {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Vec3A {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Vec3A::deserialize(d).map(|v| Vec3A(std::sync::Arc::new(v)))
    }
}
impl From<Vec3A> for bevy::math::Vec3A {
    fn from(v: Vec3A) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Vec3A {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn vec3_a_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn vec3_a_y(&self) -> f32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn vec3_a_z(&self) -> f32 {
        self.0.z
    }
    /// Length.
    #[tracing::instrument(skip(self))]
    pub fn vec3_a_length(&self) -> f32 {
        self.0.length()
    }
    /// Constructs from x, y, z (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vec3_a_new(&self, x: f32, y: f32, z: f32) -> Vec3A {
        Vec3A::from(bevy::math::Vec3A::new(x, y, z))
    }
}

mod emit_impls_vec3a {
    use super::Vec3A;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Vec3A {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! { ::bevy::math::Vec3A::new(#x, #y, #z) }
        }
    }
}
impl elicitation::ElicitComplete for Vec3A {}

// ── Vec4 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Vec4, as Vec4);
elicit_newtype_traits!(Vec4, bevy::math::Vec4, [eq]);

impl serde::Serialize for Vec4 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Vec4 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Vec4::deserialize(d).map(|v| Vec4(std::sync::Arc::new(v)))
    }
}
impl From<Vec4> for bevy::math::Vec4 {
    fn from(v: Vec4) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Vec4 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn vec4_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn vec4_y(&self) -> f32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn vec4_z(&self) -> f32 {
        self.0.z
    }
    /// W component.
    #[tracing::instrument(skip(self))]
    pub fn vec4_w(&self) -> f32 {
        self.0.w
    }
    /// Length.
    #[tracing::instrument(skip(self))]
    pub fn vec4_length(&self) -> f32 {
        self.0.length()
    }
    /// Constructs from x, y, z, w (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn vec4_new(&self, x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4::from(bevy::math::Vec4::new(x, y, z, w))
    }
}

mod emit_impls_vec4 {
    use super::Vec4;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Vec4 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            let w = self.0.w;
            quote::quote! { ::bevy::math::Vec4::new(#x, #y, #z, #w) }
        }
    }
}
impl elicitation::ElicitComplete for Vec4 {}

// ── DVec2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DVec2, as DVec2);
elicit_newtype_traits!(DVec2, bevy::math::DVec2, [eq]);

impl serde::Serialize for DVec2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DVec2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DVec2::deserialize(d).map(|v| DVec2(std::sync::Arc::new(v)))
    }
}
impl From<DVec2> for bevy::math::DVec2 {
    fn from(v: DVec2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DVec2 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec2_x(&self) -> f64 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec2_y(&self) -> f64 {
        self.0.y
    }
    /// Constructs from x, y (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_vec2_new(&self, x: f64, y: f64) -> DVec2 {
        DVec2::from(bevy::math::DVec2::new(x, y))
    }
}

mod emit_impls_dvec2 {
    use super::DVec2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DVec2 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            quote::quote! { ::bevy::math::DVec2::new(#x, #y) }
        }
    }
}
impl elicitation::ElicitComplete for DVec2 {}

// ── DVec3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DVec3, as DVec3);
elicit_newtype_traits!(DVec3, bevy::math::DVec3, [eq]);

impl serde::Serialize for DVec3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DVec3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DVec3::deserialize(d).map(|v| DVec3(std::sync::Arc::new(v)))
    }
}
impl From<DVec3> for bevy::math::DVec3 {
    fn from(v: DVec3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DVec3 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec3_x(&self) -> f64 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec3_y(&self) -> f64 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec3_z(&self) -> f64 {
        self.0.z
    }
    /// Constructs from x, y, z (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_vec3_new(&self, x: f64, y: f64, z: f64) -> DVec3 {
        DVec3::from(bevy::math::DVec3::new(x, y, z))
    }
}

mod emit_impls_dvec3 {
    use super::DVec3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DVec3 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! { ::bevy::math::DVec3::new(#x, #y, #z) }
        }
    }
}
impl elicitation::ElicitComplete for DVec3 {}

// ── DVec4 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DVec4, as DVec4);
elicit_newtype_traits!(DVec4, bevy::math::DVec4, [eq]);

impl serde::Serialize for DVec4 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DVec4 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DVec4::deserialize(d).map(|v| DVec4(std::sync::Arc::new(v)))
    }
}
impl From<DVec4> for bevy::math::DVec4 {
    fn from(v: DVec4) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DVec4 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec4_x(&self) -> f64 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec4_y(&self) -> f64 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec4_z(&self) -> f64 {
        self.0.z
    }
    /// W component.
    #[tracing::instrument(skip(self))]
    pub fn d_vec4_w(&self) -> f64 {
        self.0.w
    }
    /// Constructs from x, y, z, w (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_vec4_new(&self, x: f64, y: f64, z: f64, w: f64) -> DVec4 {
        DVec4::from(bevy::math::DVec4::new(x, y, z, w))
    }
}

mod emit_impls_dvec4 {
    use super::DVec4;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DVec4 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            let w = self.0.w;
            quote::quote! { ::bevy::math::DVec4::new(#x, #y, #z, #w) }
        }
    }
}
impl elicitation::ElicitComplete for DVec4 {}

// ── IVec2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::IVec2, as IVec2);
elicit_newtype_traits!(IVec2, bevy::math::IVec2, [eq]);

impl serde::Serialize for IVec2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for IVec2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::IVec2::deserialize(d).map(|v| IVec2(std::sync::Arc::new(v)))
    }
}
impl From<IVec2> for bevy::math::IVec2 {
    fn from(v: IVec2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl IVec2 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec2_x(&self) -> i32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec2_y(&self) -> i32 {
        self.0.y
    }
    /// Constructs from x, y (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn i_vec2_new(&self, x: i32, y: i32) -> IVec2 {
        IVec2::from(bevy::math::IVec2::new(x, y))
    }
}

mod emit_impls_ivec2 {
    use super::IVec2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for IVec2 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            quote::quote! { ::bevy::math::IVec2::new(#x, #y) }
        }
    }
}
impl elicitation::ElicitComplete for IVec2 {}

// ── IVec3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::IVec3, as IVec3);
elicit_newtype_traits!(IVec3, bevy::math::IVec3, [eq]);

impl serde::Serialize for IVec3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for IVec3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::IVec3::deserialize(d).map(|v| IVec3(std::sync::Arc::new(v)))
    }
}
impl From<IVec3> for bevy::math::IVec3 {
    fn from(v: IVec3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl IVec3 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec3_x(&self) -> i32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec3_y(&self) -> i32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec3_z(&self) -> i32 {
        self.0.z
    }
    /// Constructs from x, y, z (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn i_vec3_new(&self, x: i32, y: i32, z: i32) -> IVec3 {
        IVec3::from(bevy::math::IVec3::new(x, y, z))
    }
}

mod emit_impls_ivec3 {
    use super::IVec3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for IVec3 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! { ::bevy::math::IVec3::new(#x, #y, #z) }
        }
    }
}
impl elicitation::ElicitComplete for IVec3 {}

// ── IVec4 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::IVec4, as IVec4);
elicit_newtype_traits!(IVec4, bevy::math::IVec4, [eq]);

impl serde::Serialize for IVec4 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for IVec4 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::IVec4::deserialize(d).map(|v| IVec4(std::sync::Arc::new(v)))
    }
}
impl From<IVec4> for bevy::math::IVec4 {
    fn from(v: IVec4) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl IVec4 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec4_x(&self) -> i32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec4_y(&self) -> i32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec4_z(&self) -> i32 {
        self.0.z
    }
    /// W component.
    #[tracing::instrument(skip(self))]
    pub fn i_vec4_w(&self) -> i32 {
        self.0.w
    }
    /// Constructs from x, y, z, w (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn i_vec4_new(&self, x: i32, y: i32, z: i32, w: i32) -> IVec4 {
        IVec4::from(bevy::math::IVec4::new(x, y, z, w))
    }
}

mod emit_impls_ivec4 {
    use super::IVec4;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for IVec4 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            let w = self.0.w;
            quote::quote! { ::bevy::math::IVec4::new(#x, #y, #z, #w) }
        }
    }
}
impl elicitation::ElicitComplete for IVec4 {}

// ── UVec2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::UVec2, as UVec2);
elicit_newtype_traits!(UVec2, bevy::math::UVec2, [eq]);

impl serde::Serialize for UVec2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for UVec2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::UVec2::deserialize(d).map(|v| UVec2(std::sync::Arc::new(v)))
    }
}
impl From<UVec2> for bevy::math::UVec2 {
    fn from(v: UVec2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl UVec2 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec2_x(&self) -> u32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec2_y(&self) -> u32 {
        self.0.y
    }
    /// Constructs from x, y (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn u_vec2_new(&self, x: u32, y: u32) -> UVec2 {
        UVec2::from(bevy::math::UVec2::new(x, y))
    }
}

mod emit_impls_uvec2 {
    use super::UVec2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for UVec2 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            quote::quote! { ::bevy::math::UVec2::new(#x, #y) }
        }
    }
}
impl elicitation::ElicitComplete for UVec2 {}

// ── UVec3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::UVec3, as UVec3);
elicit_newtype_traits!(UVec3, bevy::math::UVec3, [eq]);

impl serde::Serialize for UVec3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for UVec3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::UVec3::deserialize(d).map(|v| UVec3(std::sync::Arc::new(v)))
    }
}
impl From<UVec3> for bevy::math::UVec3 {
    fn from(v: UVec3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl UVec3 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec3_x(&self) -> u32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec3_y(&self) -> u32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec3_z(&self) -> u32 {
        self.0.z
    }
    /// Constructs from x, y, z (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn u_vec3_new(&self, x: u32, y: u32, z: u32) -> UVec3 {
        UVec3::from(bevy::math::UVec3::new(x, y, z))
    }
}

mod emit_impls_uvec3 {
    use super::UVec3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for UVec3 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! { ::bevy::math::UVec3::new(#x, #y, #z) }
        }
    }
}
impl elicitation::ElicitComplete for UVec3 {}

// ── UVec4 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::UVec4, as UVec4);
elicit_newtype_traits!(UVec4, bevy::math::UVec4, [eq]);

impl serde::Serialize for UVec4 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for UVec4 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::UVec4::deserialize(d).map(|v| UVec4(std::sync::Arc::new(v)))
    }
}
impl From<UVec4> for bevy::math::UVec4 {
    fn from(v: UVec4) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl UVec4 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec4_x(&self) -> u32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec4_y(&self) -> u32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec4_z(&self) -> u32 {
        self.0.z
    }
    /// W component.
    #[tracing::instrument(skip(self))]
    pub fn u_vec4_w(&self) -> u32 {
        self.0.w
    }
    /// Constructs from x, y, z, w (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn u_vec4_new(&self, x: u32, y: u32, z: u32, w: u32) -> UVec4 {
        UVec4::from(bevy::math::UVec4::new(x, y, z, w))
    }
}

mod emit_impls_uvec4 {
    use super::UVec4;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for UVec4 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            let w = self.0.w;
            quote::quote! { ::bevy::math::UVec4::new(#x, #y, #z, #w) }
        }
    }
}
impl elicitation::ElicitComplete for UVec4 {}
