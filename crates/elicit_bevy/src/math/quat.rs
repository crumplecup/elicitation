//! Quaternion newtypes: `Quat` and `DQuat`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Quat ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Quat, as Quat);
elicit_newtype_traits!(Quat, bevy::math::Quat, [eq]);

impl serde::Serialize for Quat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Quat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Quat::deserialize(d).map(|v| Quat(std::sync::Arc::new(v)))
    }
}
impl From<Quat> for bevy::math::Quat {
    fn from(v: Quat) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Quat {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn quat_x(&self) -> f32 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn quat_y(&self) -> f32 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn quat_z(&self) -> f32 {
        self.0.z
    }
    /// W component.
    #[tracing::instrument(skip(self))]
    pub fn quat_w(&self) -> f32 {
        self.0.w
    }
    /// Returns `true` if this quaternion is finite.
    #[tracing::instrument(skip(self))]
    pub fn is_finite(&self) -> bool {
        self.0.is_finite()
    }
    /// Returns `true` if this quaternion is approximately normalized.
    #[tracing::instrument(skip(self))]
    pub fn is_normalized(&self) -> bool {
        self.0.is_normalized()
    }
    /// Returns the normalized quaternion.
    #[tracing::instrument(skip(self))]
    pub fn normalize(&self) -> Quat {
        Quat::from(self.0.normalize())
    }
    /// Returns the conjugate / inverse for unit quaternions.
    #[tracing::instrument(skip(self))]
    pub fn conjugate(&self) -> Quat {
        Quat::from(self.0.conjugate())
    }
    /// Converts to axis-angle representation. Returns (axis_x, axis_y, axis_z, angle).
    #[tracing::instrument(skip(self))]
    pub fn to_axis_angle(&self) -> (f32, f32, f32, f32) {
        let (axis, angle) = self.0.to_axis_angle();
        (axis.x, axis.y, axis.z, angle)
    }
    /// Euler angles in radians (x=roll, y=pitch, z=yaw).
    #[tracing::instrument(skip(self))]
    pub fn to_euler_xyz(&self) -> (f32, f32, f32) {
        let (x, y, z) = self.0.to_euler(bevy::math::EulerRot::XYZ);
        (x, y, z)
    }
    /// Identity quaternion (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn quat_identity(&self) -> Quat {
        Quat::from(bevy::math::Quat::IDENTITY)
    }
    /// Rotation around Y axis by `angle` radians (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn quat_from_rotation_y(&self, angle: f32) -> Quat {
        Quat::from(bevy::math::Quat::from_rotation_y(angle))
    }
    /// Rotation around X axis by `angle` radians (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_rotation_x(&self, angle: f32) -> Quat {
        Quat::from(bevy::math::Quat::from_rotation_x(angle))
    }
    /// Rotation around Z axis by `angle` radians (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_rotation_z(&self, angle: f32) -> Quat {
        Quat::from(bevy::math::Quat::from_rotation_z(angle))
    }
    /// Spherical linear interpolation toward `end` by `t` in [0, 1].
    #[tracing::instrument(skip(self, end))]
    pub fn slerp(&self, end: Quat, t: f32) -> Quat {
        Quat::from(self.0.slerp(*end.0, t))
    }
}

mod emit_impls {
    use super::Quat;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Quat {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            let w = self.0.w;
            quote::quote! { ::bevy::math::Quat::from_xyzw(#x, #y, #z, #w) }
        }
    }
}
impl elicitation::ElicitComplete for Quat {}

// ── DQuat ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DQuat, as DQuat);
elicit_newtype_traits!(DQuat, bevy::math::DQuat, [eq]);

impl serde::Serialize for DQuat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DQuat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DQuat::deserialize(d).map(|v| DQuat(std::sync::Arc::new(v)))
    }
}
impl From<DQuat> for bevy::math::DQuat {
    fn from(v: DQuat) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DQuat {
    /// X component.
    #[tracing::instrument(skip(self))]
    pub fn d_quat_x(&self) -> f64 {
        self.0.x
    }
    /// Y component.
    #[tracing::instrument(skip(self))]
    pub fn d_quat_y(&self) -> f64 {
        self.0.y
    }
    /// Z component.
    #[tracing::instrument(skip(self))]
    pub fn d_quat_z(&self) -> f64 {
        self.0.z
    }
    /// W component.
    #[tracing::instrument(skip(self))]
    pub fn d_quat_w(&self) -> f64 {
        self.0.w
    }
    /// Identity quaternion (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_quat_identity(&self) -> DQuat {
        DQuat::from(bevy::math::DQuat::IDENTITY)
    }
    /// Rotation around Y axis by `angle` radians (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn d_quat_from_rotation_y(&self, angle: f64) -> DQuat {
        DQuat::from(bevy::math::DQuat::from_rotation_y(angle))
    }
}

mod emit_impls_dquat {
    use super::DQuat;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DQuat {
        fn to_code_literal(&self) -> TokenStream {
            let x = self.0.x;
            let y = self.0.y;
            let z = self.0.z;
            let w = self.0.w;
            quote::quote! { ::bevy::math::DQuat::from_xyzw(#x, #y, #z, #w) }
        }
    }
}
impl elicitation::ElicitComplete for DQuat {}
