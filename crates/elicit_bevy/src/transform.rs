//! Bevy transform shadow types.
//!
//! [`Transform`] is a component holding translation, rotation, and scale.
//! [`GlobalTransform`] is computed by the transform propagation system.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;
use std::sync::Arc;

// ── Transform ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::transform::components::Transform, as Transform);
elicit_newtype_traits!(Transform, bevy::transform::components::Transform, [eq]);

impl serde::Serialize for Transform {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Transform {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::transform::components::Transform::deserialize(d).map(|v| Transform(Arc::new(v)))
    }
}
impl From<Transform> for bevy::transform::components::Transform {
    /// `Transform` is `Copy`, so unwrap via deref.
    fn from(v: Transform) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Transform {
    /// Returns the translation as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn transform_translation(&self) -> [f32; 3] {
        [
            self.0.translation.x,
            self.0.translation.y,
            self.0.translation.z,
        ]
    }

    /// Translation X component.
    #[tracing::instrument(skip(self))]
    pub fn translation_x(&self) -> f32 {
        self.0.translation.x
    }

    /// Translation Y component.
    #[tracing::instrument(skip(self))]
    pub fn translation_y(&self) -> f32 {
        self.0.translation.y
    }

    /// Translation Z component.
    #[tracing::instrument(skip(self))]
    pub fn translation_z(&self) -> f32 {
        self.0.translation.z
    }

    /// Returns the rotation as `[x, y, z, w]`.
    #[tracing::instrument(skip(self))]
    pub fn rotation(&self) -> [f32; 4] {
        [
            self.0.rotation.x,
            self.0.rotation.y,
            self.0.rotation.z,
            self.0.rotation.w,
        ]
    }

    /// Returns the scale as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn scale(&self) -> [f32; 3] {
        [self.0.scale.x, self.0.scale.y, self.0.scale.z]
    }

    /// Constructs the identity transform (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> Transform {
        Transform::from(bevy::transform::components::Transform::IDENTITY)
    }

    /// Constructs a transform at the given translation with default rotation and scale (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_xyz_constructor(&self, x: f32, y: f32, z: f32) -> Transform {
        Transform::from(bevy::transform::components::Transform::from_xyz(x, y, z))
    }

    /// Constructs a transform with the given uniform scale (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_scale_constructor(&self, scale: f32) -> Transform {
        Transform::from(bevy::transform::components::Transform::from_scale(
            bevy::math::Vec3::splat(scale),
        ))
    }

    /// Returns the right (positive X) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn right(&self) -> [f32; 3] {
        let v = self.0.right();
        [v.x, v.y, v.z]
    }

    /// Returns the left (negative X) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn left(&self) -> [f32; 3] {
        let v = self.0.left();
        [v.x, v.y, v.z]
    }

    /// Returns the up (positive Y) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn up(&self) -> [f32; 3] {
        let v = self.0.up();
        [v.x, v.y, v.z]
    }

    /// Returns the down (negative Y) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn down(&self) -> [f32; 3] {
        let v = self.0.down();
        [v.x, v.y, v.z]
    }

    /// Returns the forward (negative Z) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn forward(&self) -> [f32; 3] {
        let v = self.0.forward();
        [v.x, v.y, v.z]
    }

    /// Returns the back (positive Z) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn back(&self) -> [f32; 3] {
        let v = self.0.back();
        [v.x, v.y, v.z]
    }

    /// Returns a copy of this transform with translation set to `(x, y, z)`.
    #[tracing::instrument(skip(self))]
    pub fn with_translation(&self, x: f32, y: f32, z: f32) -> Transform {
        Transform::from((*self.0).with_translation(bevy::math::Vec3::new(x, y, z)))
    }

    /// Returns a copy of this transform with rotation set to a quaternion `(x, y, z, w)`.
    #[tracing::instrument(skip(self))]
    pub fn with_rotation(&self, x: f32, y: f32, z: f32, w: f32) -> Transform {
        Transform::from((*self.0).with_rotation(bevy::math::Quat::from_xyzw(x, y, z, w)))
    }

    /// Returns a copy of this transform with scale set to `(x, y, z)`.
    #[tracing::instrument(skip(self))]
    pub fn with_scale(&self, x: f32, y: f32, z: f32) -> Transform {
        Transform::from((*self.0).with_scale(bevy::math::Vec3::new(x, y, z)))
    }

    /// Returns a copy rotated by the given angle (radians) around the X axis.
    #[tracing::instrument(skip(self))]
    pub fn rotate_x(&self, angle: f32) -> Transform {
        let mut t = *self.0;
        t.rotate_x(angle);
        Transform::from(t)
    }

    /// Returns a copy rotated by the given angle (radians) around the Y axis.
    #[tracing::instrument(skip(self))]
    pub fn rotate_y(&self, angle: f32) -> Transform {
        let mut t = *self.0;
        t.rotate_y(angle);
        Transform::from(t)
    }

    /// Returns a copy rotated by the given angle (radians) around the Z axis.
    #[tracing::instrument(skip(self))]
    pub fn rotate_z(&self, angle: f32) -> Transform {
        let mut t = *self.0;
        t.rotate_z(angle);
        Transform::from(t)
    }

    /// Returns a copy rotated by `angle` radians around the local X axis.
    #[tracing::instrument(skip(self))]
    pub fn rotate_local_x(&self, angle: f32) -> Transform {
        let mut t = *self.0;
        t.rotate_local_x(angle);
        Transform::from(t)
    }

    /// Returns a copy rotated by `angle` radians around the local Y axis.
    #[tracing::instrument(skip(self))]
    pub fn rotate_local_y(&self, angle: f32) -> Transform {
        let mut t = *self.0;
        t.rotate_local_y(angle);
        Transform::from(t)
    }

    /// Returns a copy rotated by `angle` radians around the local Z axis.
    #[tracing::instrument(skip(self))]
    pub fn rotate_local_z(&self, angle: f32) -> Transform {
        let mut t = *self.0;
        t.rotate_local_z(angle);
        Transform::from(t)
    }

    /// Multiplies this transform by another transform.
    #[tracing::instrument(skip(self))]
    pub fn mul_transform(&self, other: Transform) -> Transform {
        Transform::from(self.0.mul_transform(*other.0))
    }

    /// Transforms a point from local space to world space, returning `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn transform_point(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
        let p = self.0.transform_point(bevy::math::Vec3::new(x, y, z));
        [p.x, p.y, p.z]
    }

    /// Returns `true` if all components are finite (not NaN or infinity).
    #[tracing::instrument(skip(self))]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }

    /// Computes the transformation matrix.
    #[tracing::instrument(skip(self))]
    pub fn compute_matrix(&self) -> [[f32; 4]; 4] {
        let m = self.0.to_matrix();
        m.to_cols_array_2d()
    }
}

mod emit_impls_transform {
    use super::Transform;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Transform {
        fn to_code_literal(&self) -> TokenStream {
            let [tx, ty, tz] = [
                self.0.translation.x,
                self.0.translation.y,
                self.0.translation.z,
            ];
            let [rx, ry, rz, rw] = [
                self.0.rotation.x,
                self.0.rotation.y,
                self.0.rotation.z,
                self.0.rotation.w,
            ];
            let [sx, sy, sz] = [self.0.scale.x, self.0.scale.y, self.0.scale.z];
            quote::quote! {
                ::bevy::transform::components::Transform {
                    translation: ::bevy::math::Vec3::new(#tx, #ty, #tz),
                    rotation: ::bevy::math::Quat::from_xyzw(#rx, #ry, #rz, #rw),
                    scale: ::bevy::math::Vec3::new(#sx, #sy, #sz),
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Transform {}

// ── GlobalTransform ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::transform::components::GlobalTransform, as GlobalTransform);
elicit_newtype_traits!(
    GlobalTransform,
    bevy::transform::components::GlobalTransform,
    []
);

impl serde::Serialize for GlobalTransform {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let cols = self.0.affine().to_cols_array();
        let mut seq = s.serialize_seq(Some(cols.len()))?;
        for c in &cols {
            seq.serialize_element(c)?;
        }
        seq.end()
    }
}
impl<'de> serde::Deserialize<'de> for GlobalTransform {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let cols: Vec<f32> = serde::Deserialize::deserialize(d)?;
        if cols.len() != 12 {
            return Err(serde::de::Error::custom("expected 12 f32 values"));
        }
        let arr: [f32; 12] = cols.try_into().unwrap();
        let affine = bevy::math::Affine3A::from_cols_array(&arr);
        Ok(GlobalTransform(Arc::new(
            bevy::transform::components::GlobalTransform::from(affine),
        )))
    }
}
impl From<GlobalTransform> for bevy::transform::components::GlobalTransform {
    fn from(v: GlobalTransform) -> Self {
        Arc::try_unwrap(v.0).unwrap_or_else(|arc| (*arc).clone())
    }
}

#[reflect_methods]
impl GlobalTransform {
    /// Returns translation as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_transform_translation(&self) -> [f32; 3] {
        let t = self.0.translation();
        [t.x, t.y, t.z]
    }

    /// Computes the local `Transform` from this global transform.
    #[tracing::instrument(skip(self))]
    pub fn compute_transform(&self) -> Transform {
        Transform::from(self.0.compute_transform())
    }

    /// Returns the forward (negative Z) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_forward(&self) -> [f32; 3] {
        let v = self.0.forward();
        [v.x, v.y, v.z]
    }

    /// Returns the back (positive Z) direction in world space as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_back(&self) -> [f32; 3] {
        let v = self.0.back();
        [v.x, v.y, v.z]
    }

    /// Returns the up (positive Y) direction as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_up(&self) -> [f32; 3] {
        let v = self.0.up();
        [v.x, v.y, v.z]
    }

    /// Returns the down (negative Y) direction as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_down(&self) -> [f32; 3] {
        let v = self.0.down();
        [v.x, v.y, v.z]
    }

    /// Returns the right (positive X) direction as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_right(&self) -> [f32; 3] {
        let v = self.0.right();
        [v.x, v.y, v.z]
    }

    /// Returns the left (negative X) direction as `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_left(&self) -> [f32; 3] {
        let v = self.0.left();
        [v.x, v.y, v.z]
    }

    /// Transforms a point from local space to world space, returning `[x, y, z]`.
    #[tracing::instrument(skip(self))]
    pub fn global_transform_point(&self, x: f32, y: f32, z: f32) -> [f32; 3] {
        let p = self.0.transform_point(bevy::math::Vec3::new(x, y, z));
        [p.x, p.y, p.z]
    }

    /// Returns the 4×4 matrix as a flat array (column-major).
    #[tracing::instrument(skip(self))]
    pub fn matrix_cols(&self) -> [[f32; 4]; 4] {
        self.0.to_matrix().to_cols_array_2d()
    }
}

mod emit_impls_global {
    use super::GlobalTransform;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for GlobalTransform {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.affine().to_cols_array();
            quote::quote! {
                ::bevy::transform::components::GlobalTransform::from(
                    ::bevy::math::Affine3A::from_cols_array(&[#(#cols),*])
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for GlobalTransform {}
