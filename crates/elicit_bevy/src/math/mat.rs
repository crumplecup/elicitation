//! Matrix newtypes: `Mat2`, `Mat3`, `Mat3A`, `Mat4`, `DMat2`, `DMat3`, `DMat4`.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Mat2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Mat2, as Mat2);
elicit_newtype_traits!(Mat2, bevy::math::Mat2, [eq]);

impl serde::Serialize for Mat2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Mat2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Mat2::deserialize(d).map(|v| Mat2(std::sync::Arc::new(v)))
    }
}
impl From<Mat2> for bevy::math::Mat2 {
    fn from(v: Mat2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Mat2 {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f32 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> Mat2 {
        Mat2::from(self.0.transpose())
    }
    /// Inverse (panics if singular).
    #[tracing::instrument(skip(self))]
    pub fn inverse(&self) -> Mat2 {
        Mat2::from(self.0.inverse())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> Mat2 {
        Mat2::from(bevy::math::Mat2::IDENTITY)
    }
}

mod emit_impls {
    use super::Mat2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Mat2 {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let (c00, c01, c10, c11) = (cols[0], cols[1], cols[2], cols[3]);
            quote::quote! {
                ::bevy::math::Mat2::from_cols_array(&[#c00, #c01, #c10, #c11])
            }
        }
    }
}
impl elicitation::ElicitComplete for Mat2 {}

// ── Mat3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Mat3, as Mat3);
elicit_newtype_traits!(Mat3, bevy::math::Mat3, [eq]);

impl serde::Serialize for Mat3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Mat3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Mat3::deserialize(d).map(|v| Mat3(std::sync::Arc::new(v)))
    }
}
impl From<Mat3> for bevy::math::Mat3 {
    fn from(v: Mat3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Mat3 {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f32 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> Mat3 {
        Mat3::from(self.0.transpose())
    }
    /// Inverse.
    #[tracing::instrument(skip(self))]
    pub fn inverse(&self) -> Mat3 {
        Mat3::from(self.0.inverse())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> Mat3 {
        Mat3::from(bevy::math::Mat3::IDENTITY)
    }
}

mod emit_impls_mat3 {
    use super::Mat3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Mat3 {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let c: Vec<_> = cols.to_vec();
            quote::quote! {
                ::bevy::math::Mat3::from_cols_array(&[#(#c),*])
            }
        }
    }
}
impl elicitation::ElicitComplete for Mat3 {}

// ── Mat3A ────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Mat3A, as Mat3A);
elicit_newtype_traits!(Mat3A, bevy::math::Mat3A, [eq]);

impl serde::Serialize for Mat3A {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Mat3A {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Mat3A::deserialize(d).map(|v| Mat3A(std::sync::Arc::new(v)))
    }
}
impl From<Mat3A> for bevy::math::Mat3A {
    fn from(v: Mat3A) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Mat3A {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f32 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> Mat3A {
        Mat3A::from(self.0.transpose())
    }
    /// Inverse.
    #[tracing::instrument(skip(self))]
    pub fn inverse(&self) -> Mat3A {
        Mat3A::from(self.0.inverse())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> Mat3A {
        Mat3A::from(bevy::math::Mat3A::IDENTITY)
    }
}

mod emit_impls_mat3a {
    use super::Mat3A;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Mat3A {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let c: Vec<_> = cols.to_vec();
            quote::quote! {
                ::bevy::math::Mat3A::from_cols_array(&[#(#c),*])
            }
        }
    }
}
impl elicitation::ElicitComplete for Mat3A {}

// ── Mat4 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::Mat4, as Mat4);
elicit_newtype_traits!(Mat4, bevy::math::Mat4, [eq]);

impl serde::Serialize for Mat4 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Mat4 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::Mat4::deserialize(d).map(|v| Mat4(std::sync::Arc::new(v)))
    }
}
impl From<Mat4> for bevy::math::Mat4 {
    fn from(v: Mat4) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Mat4 {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f32 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> Mat4 {
        Mat4::from(self.0.transpose())
    }
    /// Inverse.
    #[tracing::instrument(skip(self))]
    pub fn inverse(&self) -> Mat4 {
        Mat4::from(self.0.inverse())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> Mat4 {
        Mat4::from(bevy::math::Mat4::IDENTITY)
    }
    /// Perspective projection (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn perspective_rh(
        &self,
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Mat4 {
        Mat4::from(bevy::math::Mat4::perspective_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }
    /// Orthographic projection (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn orthographic_rh(
        &self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Mat4 {
        Mat4::from(bevy::math::Mat4::orthographic_rh(
            left, right, bottom, top, near, far,
        ))
    }
}

mod emit_impls_mat4 {
    use super::Mat4;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Mat4 {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let c: Vec<_> = cols.to_vec();
            quote::quote! {
                ::bevy::math::Mat4::from_cols_array(&[#(#c),*])
            }
        }
    }
}
impl elicitation::ElicitComplete for Mat4 {}

// ── DMat2 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DMat2, as DMat2);
elicit_newtype_traits!(DMat2, bevy::math::DMat2, [eq]);

impl serde::Serialize for DMat2 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DMat2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DMat2::deserialize(d).map(|v| DMat2(std::sync::Arc::new(v)))
    }
}
impl From<DMat2> for bevy::math::DMat2 {
    fn from(v: DMat2) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DMat2 {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> DMat2 {
        DMat2::from(self.0.transpose())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> DMat2 {
        DMat2::from(bevy::math::DMat2::IDENTITY)
    }
}

mod emit_impls_dmat2 {
    use super::DMat2;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DMat2 {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let (c00, c01, c10, c11) = (cols[0], cols[1], cols[2], cols[3]);
            quote::quote! {
                ::bevy::math::DMat2::from_cols_array(&[#c00, #c01, #c10, #c11])
            }
        }
    }
}
impl elicitation::ElicitComplete for DMat2 {}

// ── DMat3 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DMat3, as DMat3);
elicit_newtype_traits!(DMat3, bevy::math::DMat3, [eq]);

impl serde::Serialize for DMat3 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DMat3 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DMat3::deserialize(d).map(|v| DMat3(std::sync::Arc::new(v)))
    }
}
impl From<DMat3> for bevy::math::DMat3 {
    fn from(v: DMat3) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DMat3 {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> DMat3 {
        DMat3::from(self.0.transpose())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> DMat3 {
        DMat3::from(bevy::math::DMat3::IDENTITY)
    }
}

mod emit_impls_dmat3 {
    use super::DMat3;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DMat3 {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let c: Vec<_> = cols.to_vec();
            quote::quote! { ::bevy::math::DMat3::from_cols_array(&[#(#c),*]) }
        }
    }
}
impl elicitation::ElicitComplete for DMat3 {}

// ── DMat4 ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::DMat4, as DMat4);
elicit_newtype_traits!(DMat4, bevy::math::DMat4, [eq]);

impl serde::Serialize for DMat4 {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for DMat4 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::DMat4::deserialize(d).map(|v| DMat4(std::sync::Arc::new(v)))
    }
}
impl From<DMat4> for bevy::math::DMat4 {
    fn from(v: DMat4) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl DMat4 {
    /// Determinant.
    #[tracing::instrument(skip(self))]
    pub fn determinant(&self) -> f64 {
        self.0.determinant()
    }
    /// Transpose.
    #[tracing::instrument(skip(self))]
    pub fn transpose(&self) -> DMat4 {
        DMat4::from(self.0.transpose())
    }
    /// Inverse.
    #[tracing::instrument(skip(self))]
    pub fn inverse(&self) -> DMat4 {
        DMat4::from(self.0.inverse())
    }
    /// Identity matrix (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn identity(&self) -> DMat4 {
        DMat4::from(bevy::math::DMat4::IDENTITY)
    }
}

mod emit_impls_dmat4 {
    use super::DMat4;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for DMat4 {
        fn to_code_literal(&self) -> TokenStream {
            let cols = self.0.to_cols_array();
            let c: Vec<_> = cols.to_vec();
            quote::quote! { ::bevy::math::DMat4::from_cols_array(&[#(#c),*]) }
        }
    }
}
impl elicitation::ElicitComplete for DMat4 {}
