//! Isometry newtypes: `Isometry2d` and `Isometry3d`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Isometry2d ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Isometry2d, as Isometry2d);
elicit_newtype_traits!(Isometry2d, bevy::math::Isometry2d, [eq]);

impl serde::Serialize for Isometry2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut m = s.serialize_map(Some(4))?;
        m.serialize_entry("tx", &self.0.translation.x)?;
        m.serialize_entry("ty", &self.0.translation.y)?;
        m.serialize_entry("cos", &self.0.rotation.cos)?;
        m.serialize_entry("sin", &self.0.rotation.sin)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Isometry2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Fields {
            tx: f32,
            ty: f32,
            cos: f32,
            sin: f32,
        }
        let f = Fields::deserialize(d)?;
        let rot = bevy::math::Rot2 {
            cos: f.cos,
            sin: f.sin,
        };
        Ok(Isometry2d(std::sync::Arc::new(bevy::math::Isometry2d {
            translation: bevy::math::Vec2::new(f.tx, f.ty),
            rotation: rot,
        })))
    }
}
impl From<Isometry2d> for bevy::math::Isometry2d {
    fn from(v: Isometry2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Isometry2d {
    /// Translation X.
    #[tracing::instrument(skip(self))]
    pub fn isometry2d_tx(&self) -> f32 {
        self.0.translation.x
    }
    /// Translation Y.
    #[tracing::instrument(skip(self))]
    pub fn isometry2d_ty(&self) -> f32 {
        self.0.translation.y
    }
    /// Rotation angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn rotation_radians(&self) -> f32 {
        self.0.rotation.as_radians()
    }
    /// Inverse isometry.
    #[tracing::instrument(skip(self))]
    pub fn isometry2d_inverse(&self) -> Isometry2d {
        Isometry2d::from(self.0.inverse())
    }
    /// Identity (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn isometry2d_identity(&self) -> Isometry2d {
        Isometry2d::from(bevy::math::Isometry2d::IDENTITY)
    }
    /// Constructs from translation and rotation (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn isometry2d_new(&self, tx: f32, ty: f32, angle_radians: f32) -> Isometry2d {
        Isometry2d::from(bevy::math::Isometry2d::new(
            bevy::math::Vec2::new(tx, ty),
            bevy::math::Rot2::radians(angle_radians),
        ))
    }
}

mod emit_impls {
    use super::Isometry2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Isometry2d {
        fn to_code_literal(&self) -> TokenStream {
            let tx = self.0.translation.x;
            let ty = self.0.translation.y;
            let angle = self.0.rotation.as_radians();
            quote::quote! {
                ::bevy::math::Isometry2d::new(
                    ::bevy::math::Vec2::new(#tx, #ty),
                    ::bevy::math::Rot2::radians(#angle),
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for Isometry2d {}

// ── Isometry3d ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Isometry3d, as Isometry3d);
elicit_newtype_traits!(Isometry3d, bevy::math::Isometry3d, [eq]);

impl serde::Serialize for Isometry3d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let t = self.0.translation;
        let r = self.0.rotation;
        let mut m = s.serialize_map(Some(7))?;
        m.serialize_entry("tx", &t.x)?;
        m.serialize_entry("ty", &t.y)?;
        m.serialize_entry("tz", &t.z)?;
        m.serialize_entry("rx", &r.x)?;
        m.serialize_entry("ry", &r.y)?;
        m.serialize_entry("rz", &r.z)?;
        m.serialize_entry("rw", &r.w)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Isometry3d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Fields {
            tx: f32,
            ty: f32,
            tz: f32,
            rx: f32,
            ry: f32,
            rz: f32,
            rw: f32,
        }
        let f = Fields::deserialize(d)?;
        Ok(Isometry3d(std::sync::Arc::new(
            bevy::math::Isometry3d::new(
                bevy::math::Vec3::new(f.tx, f.ty, f.tz),
                bevy::math::Quat::from_xyzw(f.rx, f.ry, f.rz, f.rw),
            ),
        )))
    }
}
impl From<Isometry3d> for bevy::math::Isometry3d {
    fn from(v: Isometry3d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Isometry3d {
    /// Translation X.
    #[tracing::instrument(skip(self))]
    pub fn isometry3d_tx(&self) -> f32 {
        self.0.translation.x
    }
    /// Translation Y.
    #[tracing::instrument(skip(self))]
    pub fn isometry3d_ty(&self) -> f32 {
        self.0.translation.y
    }
    /// Translation Z.
    #[tracing::instrument(skip(self))]
    pub fn tz(&self) -> f32 {
        self.0.translation.z
    }
    /// Rotation X.
    #[tracing::instrument(skip(self))]
    pub fn rx(&self) -> f32 {
        self.0.rotation.x
    }
    /// Rotation Y.
    #[tracing::instrument(skip(self))]
    pub fn ry(&self) -> f32 {
        self.0.rotation.y
    }
    /// Rotation Z.
    #[tracing::instrument(skip(self))]
    pub fn rz(&self) -> f32 {
        self.0.rotation.z
    }
    /// Rotation W.
    #[tracing::instrument(skip(self))]
    pub fn rw(&self) -> f32 {
        self.0.rotation.w
    }
    /// Inverse isometry.
    #[tracing::instrument(skip(self))]
    pub fn isometry3d_inverse(&self) -> Isometry3d {
        Isometry3d::from(self.0.inverse())
    }
    /// Identity (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn isometry3d_identity(&self) -> Isometry3d {
        Isometry3d::from(bevy::math::Isometry3d::IDENTITY)
    }
    /// Constructs from translation and quaternion rotation (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn isometry3d_new(
        &self,
        tx: f32,
        ty: f32,
        tz: f32,
        rx: f32,
        ry: f32,
        rz: f32,
        rw: f32,
    ) -> Isometry3d {
        Isometry3d::from(bevy::math::Isometry3d::new(
            bevy::math::Vec3::new(tx, ty, tz),
            bevy::math::Quat::from_xyzw(rx, ry, rz, rw),
        ))
    }
}

mod emit_impls_iso3d {
    use super::Isometry3d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Isometry3d {
        fn to_code_literal(&self) -> TokenStream {
            let t = self.0.translation;
            let r = self.0.rotation;
            let (tx, ty, tz) = (t.x, t.y, t.z);
            let (rx, ry, rz, rw) = (r.x, r.y, r.z, r.w);
            quote::quote! {
                ::bevy::math::Isometry3d::new(
                    ::bevy::math::Vec3::new(#tx, #ty, #tz),
                    ::bevy::math::Quat::from_xyzw(#rx, #ry, #rz, #rw),
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for Isometry3d {}
