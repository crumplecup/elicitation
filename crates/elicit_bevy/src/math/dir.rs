//! Direction newtypes: `Dir2`, `Dir3`, `Dir3A`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Dir2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Dir2, as Dir2);
elicit_newtype_traits!(Dir2, bevy::math::Dir2, [eq]);

impl serde::Serialize for Dir2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut m = s.serialize_map(Some(2))?;
        m.serialize_entry("x", &self.0.x)?;
        m.serialize_entry("y", &self.0.y)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dir2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct XY {
            x: f32,
            y: f32,
        }
        let xy = XY::deserialize(d)?;
        let inner = bevy::math::Dir2::new(bevy::math::Vec2::new(xy.x, xy.y))
            .map_err(serde::de::Error::custom)?;
        Ok(Dir2(std::sync::Arc::new(inner)))
    }
}
impl From<Dir2> for bevy::math::Dir2 {
    fn from(v: Dir2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Dir2 {
    /// X component of the underlying unit vector.
    #[tracing::instrument(skip(self))]
    pub fn dir2_x(&self) -> f32 {
        self.0.x
    }
    /// Y component of the underlying unit vector.
    #[tracing::instrument(skip(self))]
    pub fn dir2_y(&self) -> f32 {
        self.0.y
    }
    /// Right direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir2_right(&self) -> Dir2 {
        Dir2::from(bevy::math::Dir2::X)
    }
    /// Up direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir2_up(&self) -> Dir2 {
        Dir2::from(bevy::math::Dir2::Y)
    }
    /// Try to create a `Dir2` from x, y; fails if not normalizable (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir2_try_new(&self, x: f32, y: f32) -> Option<Dir2> {
        bevy::math::Dir2::new(bevy::math::Vec2::new(x, y))
            .ok()
            .map(Dir2::from)
    }
}

mod emit_impls {
    use super::Dir2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Dir2 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            quote::quote! {
                ::bevy::math::Dir2::new(::bevy::math::Vec2::new(#x, #y)).unwrap()
            }
        }
    }
}
impl elicitation::ElicitComplete for Dir2 {}

// ── Dir3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Dir3, as Dir3);
elicit_newtype_traits!(Dir3, bevy::math::Dir3, [eq]);

impl serde::Serialize for Dir3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut m = s.serialize_map(Some(3))?;
        m.serialize_entry("x", &self.0.x)?;
        m.serialize_entry("y", &self.0.y)?;
        m.serialize_entry("z", &self.0.z)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dir3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Xyz {
            x: f32,
            y: f32,
            z: f32,
        }
        let xyz = Xyz::deserialize(d)?;
        let inner = bevy::math::Dir3::new(bevy::math::Vec3::new(xyz.x, xyz.y, xyz.z))
            .map_err(serde::de::Error::custom)?;
        Ok(Dir3(std::sync::Arc::new(inner)))
    }
}
impl From<Dir3> for bevy::math::Dir3 {
    fn from(v: Dir3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Dir3 {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn dir3_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn dir3_y(&self) -> f32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn dir3_z(&self) -> f32 {
        self.0.z
    }
    /// Up (+Y) direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir3_up(&self) -> Dir3 {
        Dir3::from(bevy::math::Dir3::Y)
    }
    /// Forward (-Z) direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn forward(&self) -> Dir3 {
        Dir3::from(bevy::math::Dir3::NEG_Z)
    }
    /// Right (+X) direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir3_right(&self) -> Dir3 {
        Dir3::from(bevy::math::Dir3::X)
    }
    /// Try to create from x, y, z; fails if not normalizable (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir3_try_new(&self, x: f32, y: f32, z: f32) -> Option<Dir3> {
        bevy::math::Dir3::new(bevy::math::Vec3::new(x, y, z))
            .ok()
            .map(Dir3::from)
    }
}

mod emit_impls_dir3 {
    use super::Dir3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Dir3 {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! {
                ::bevy::math::Dir3::new(::bevy::math::Vec3::new(#x, #y, #z)).unwrap()
            }
        }
    }
}
impl elicitation::ElicitComplete for Dir3 {}

// ── Dir3A ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Dir3A, as Dir3A);
elicit_newtype_traits!(Dir3A, bevy::math::Dir3A, [eq]);

impl serde::Serialize for Dir3A {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut m = s.serialize_map(Some(3))?;
        m.serialize_entry("x", &self.0.x)?;
        m.serialize_entry("y", &self.0.y)?;
        m.serialize_entry("z", &self.0.z)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Dir3A {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Xyz {
            x: f32,
            y: f32,
            z: f32,
        }
        let xyz = Xyz::deserialize(d)?;
        let inner = bevy::math::Dir3A::new(bevy::math::Vec3A::new(xyz.x, xyz.y, xyz.z))
            .map_err(serde::de::Error::custom)?;
        Ok(Dir3A(std::sync::Arc::new(inner)))
    }
}
impl From<Dir3A> for bevy::math::Dir3A {
    fn from(v: Dir3A) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Dir3A {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn dir3_a_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn dir3_a_y(&self) -> f32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn dir3_a_z(&self) -> f32 {
        self.0.z
    }
    /// Try to create from x, y, z (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn dir3_a_try_new(&self, x: f32, y: f32, z: f32) -> Option<Dir3A> {
        bevy::math::Dir3A::new(bevy::math::Vec3A::new(x, y, z))
            .ok()
            .map(Dir3A::from)
    }
}

mod emit_impls_dir3a {
    use super::Dir3A;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Dir3A {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            quote::quote! {
                ::bevy::math::Dir3A::new(::bevy::math::Vec3A::new(#x, #y, #z)).unwrap()
            }
        }
    }
}
impl elicitation::ElicitComplete for Dir3A {}
