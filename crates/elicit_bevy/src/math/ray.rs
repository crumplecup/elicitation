//! Ray newtypes: `Ray2d` and `Ray3d`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Ray2d ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Ray2d, as Ray2d);
elicit_newtype_traits!(Ray2d, bevy::math::Ray2d, [eq]);

impl serde::Serialize for Ray2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut m = s.serialize_map(Some(2))?;
        m.serialize_entry("origin_x", &self.0.origin.x)?;
        m.serialize_entry("origin_y", &self.0.origin.y)?;
        m.serialize_entry("dir_x", &self.0.direction.x)?;
        m.serialize_entry("dir_y", &self.0.direction.y)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ray2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Fields {
            origin_x: f32,
            origin_y: f32,
            dir_x: f32,
            dir_y: f32,
        }
        let f = Fields::deserialize(d)?;
        let dir = bevy::math::Dir2::new(bevy::math::Vec2::new(f.dir_x, f.dir_y))
            .map_err(serde::de::Error::custom)?;
        Ok(Ray2d(std::sync::Arc::new(bevy::math::Ray2d {
            origin: bevy::math::Vec2::new(f.origin_x, f.origin_y),
            direction: dir,
        })))
    }
}
impl From<Ray2d> for bevy::math::Ray2d {
    fn from(v: Ray2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Ray2d {
    /// Origin X.
    #[tracing::instrument(skip(self))]
    pub fn ray2d_origin_x(&self) -> f32 {
        self.0.origin.x
    }
    /// Origin Y.
    #[tracing::instrument(skip(self))]
    pub fn ray2d_origin_y(&self) -> f32 {
        self.0.origin.y
    }
    /// Direction X.
    #[tracing::instrument(skip(self))]
    pub fn ray2d_dir_x(&self) -> f32 {
        self.0.direction.x
    }
    /// Direction Y.
    #[tracing::instrument(skip(self))]
    pub fn ray2d_dir_y(&self) -> f32 {
        self.0.direction.y
    }
    /// Point at parameter `t` along the ray.
    #[tracing::instrument(skip(self))]
    pub fn ray2d_get_point(&self, t: f32) -> (f32, f32) {
        let p = self.0.get_point(t);
        (p.x, p.y)
    }
    /// Constructs a Ray2d from origin and direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn ray2d_new(&self, origin_x: f32, origin_y: f32, dir_x: f32, dir_y: f32) -> Option<Ray2d> {
        bevy::math::Dir2::new(bevy::math::Vec2::new(dir_x, dir_y))
            .ok()
            .map(|dir| {
                Ray2d(std::sync::Arc::new(bevy::math::Ray2d {
                    origin: bevy::math::Vec2::new(origin_x, origin_y),
                    direction: dir,
                }))
            })
    }
}

mod emit_impls {
    use super::Ray2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Ray2d {
        fn to_code_literal(&self) -> TokenStream {
            let ox = self.0.origin.x;
            let oy = self.0.origin.y;
            let dx = self.0.direction.x;
            let dy = self.0.direction.y;
            quote::quote! {
                ::bevy::math::Ray2d {
                    origin: ::bevy::math::Vec2::new(#ox, #oy),
                    direction: ::bevy::math::Dir2::new(
                        ::bevy::math::Vec2::new(#dx, #dy)
                    ).unwrap(),
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Ray2d {}

// ── Ray3d ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Ray3d, as Ray3d);
elicit_newtype_traits!(Ray3d, bevy::math::Ray3d, [eq]);

impl serde::Serialize for Ray3d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut m = s.serialize_map(Some(6))?;
        m.serialize_entry("origin_x", &self.0.origin.x)?;
        m.serialize_entry("origin_y", &self.0.origin.y)?;
        m.serialize_entry("origin_z", &self.0.origin.z)?;
        m.serialize_entry("dir_x", &self.0.direction.x)?;
        m.serialize_entry("dir_y", &self.0.direction.y)?;
        m.serialize_entry("dir_z", &self.0.direction.z)?;
        m.end()
    }
}
impl<'de> serde::Deserialize<'de> for Ray3d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        struct Fields {
            origin_x: f32,
            origin_y: f32,
            origin_z: f32,
            dir_x: f32,
            dir_y: f32,
            dir_z: f32,
        }
        let f = Fields::deserialize(d)?;
        let dir = bevy::math::Dir3::new(bevy::math::Vec3::new(f.dir_x, f.dir_y, f.dir_z))
            .map_err(serde::de::Error::custom)?;
        Ok(Ray3d(std::sync::Arc::new(bevy::math::Ray3d {
            origin: bevy::math::Vec3::new(f.origin_x, f.origin_y, f.origin_z),
            direction: dir,
        })))
    }
}
impl From<Ray3d> for bevy::math::Ray3d {
    fn from(v: Ray3d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Ray3d {
    /// Origin X.
    #[tracing::instrument(skip(self))]
    pub fn ray3d_origin_x(&self) -> f32 {
        self.0.origin.x
    }
    /// Origin Y.
    #[tracing::instrument(skip(self))]
    pub fn ray3d_origin_y(&self) -> f32 {
        self.0.origin.y
    }
    /// Origin Z.
    #[tracing::instrument(skip(self))]
    pub fn origin_z(&self) -> f32 {
        self.0.origin.z
    }
    /// Direction X.
    #[tracing::instrument(skip(self))]
    pub fn ray3d_dir_x(&self) -> f32 {
        self.0.direction.x
    }
    /// Direction Y.
    #[tracing::instrument(skip(self))]
    pub fn ray3d_dir_y(&self) -> f32 {
        self.0.direction.y
    }
    /// Direction Z.
    #[tracing::instrument(skip(self))]
    pub fn dir_z(&self) -> f32 {
        self.0.direction.z
    }
    /// Point at parameter `t`.
    #[tracing::instrument(skip(self))]
    pub fn ray3d_get_point(&self, t: f32) -> (f32, f32, f32) {
        let p = self.0.get_point(t);
        (p.x, p.y, p.z)
    }
    /// Construct a Ray3d (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn ray3d_new(&self, ox: f32, oy: f32, oz: f32, dx: f32, dy: f32, dz: f32) -> Option<Ray3d> {
        bevy::math::Dir3::new(bevy::math::Vec3::new(dx, dy, dz))
            .ok()
            .map(|dir| {
                Ray3d(std::sync::Arc::new(bevy::math::Ray3d {
                    origin: bevy::math::Vec3::new(ox, oy, oz),
                    direction: dir,
                }))
            })
    }
}

mod emit_impls_ray3d {
    use super::Ray3d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Ray3d {
        fn to_code_literal(&self) -> TokenStream {
            let ox = self.0.origin.x;
            let oy = self.0.origin.y;
            let oz = self.0.origin.z;
            let dx = self.0.direction.x;
            let dy = self.0.direction.y;
            let dz = self.0.direction.z;
            quote::quote! {
                ::bevy::math::Ray3d {
                    origin: ::bevy::math::Vec3::new(#ox, #oy, #oz),
                    direction: ::bevy::math::Dir3::new(
                        ::bevy::math::Vec3::new(#dx, #dy, #dz)
                    ).unwrap(),
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Ray3d {}
