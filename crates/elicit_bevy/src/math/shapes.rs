//! Geometric primitive newtypes from `bevy::math::primitives`.
//!
//! Covers common 2D and 3D shapes.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

// ── Circle ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Circle, as Circle);
elicit_newtype_traits!(Circle, bevy::math::primitives::Circle, [eq]);

impl serde::Serialize for Circle {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Circle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Circle::deserialize(d).map(|v| Circle(std::sync::Arc::new(v)))
    }
}
impl From<Circle> for bevy::math::primitives::Circle {
    fn from(v: Circle) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Circle {
    /// Radius.
    #[tracing::instrument(skip(self))]
    pub fn circle_radius(&self) -> f32 {
        self.0.radius
    }
    /// Diameter.
    #[tracing::instrument(skip(self))]
    pub fn diameter(&self) -> f32 {
        self.0.radius * 2.0
    }
    /// Constructs with the given radius (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn circle_new(&self, radius: f32) -> Circle {
        Circle::from(bevy::math::primitives::Circle::new(radius))
    }
}

mod emit_impls {
    use super::Circle;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Circle {
        fn to_code_literal(&self) -> TokenStream {
            let r = self.0.radius;
            quote::quote! { ::bevy::math::primitives::Circle::new(#r) }
        }
    }
}
impl elicitation::ElicitComplete for Circle {}

// ── Sphere ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Sphere, as Sphere);
elicit_newtype_traits!(Sphere, bevy::math::primitives::Sphere, [eq]);

impl serde::Serialize for Sphere {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Sphere {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Sphere::deserialize(d).map(|v| Sphere(std::sync::Arc::new(v)))
    }
}
impl From<Sphere> for bevy::math::primitives::Sphere {
    fn from(v: Sphere) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Sphere {
    /// Radius.
    #[tracing::instrument(skip(self))]
    pub fn sphere_radius(&self) -> f32 {
        self.0.radius
    }
    /// Constructs with the given radius (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn sphere_new(&self, radius: f32) -> Sphere {
        Sphere::from(bevy::math::primitives::Sphere::new(radius))
    }
}

mod emit_impls_sphere {
    use super::Sphere;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Sphere {
        fn to_code_literal(&self) -> TokenStream {
            let r = self.0.radius;
            quote::quote! { ::bevy::math::primitives::Sphere::new(#r) }
        }
    }
}
impl elicitation::ElicitComplete for Sphere {}

// ── Annulus ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Annulus, as Annulus);
elicit_newtype_traits!(Annulus, bevy::math::primitives::Annulus, [eq]);

impl serde::Serialize for Annulus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Annulus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Annulus::deserialize(d).map(|v| Annulus(std::sync::Arc::new(v)))
    }
}
impl From<Annulus> for bevy::math::primitives::Annulus {
    fn from(v: Annulus) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Annulus {
    /// Inner radius.
    #[tracing::instrument(skip(self))]
    pub fn inner_circle_radius(&self) -> f32 {
        self.0.inner_circle.radius
    }
    /// Outer radius.
    #[tracing::instrument(skip(self))]
    pub fn outer_circle_radius(&self) -> f32 {
        self.0.outer_circle.radius
    }
    /// Constructs with inner/outer radius (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn annulus_new(&self, inner: f32, outer: f32) -> Annulus {
        Annulus::from(bevy::math::primitives::Annulus::new(inner, outer))
    }
}

mod emit_impls_annulus {
    use super::Annulus;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Annulus {
        fn to_code_literal(&self) -> TokenStream {
            let inner = self.0.inner_circle.radius;
            let outer = self.0.outer_circle.radius;
            quote::quote! { ::bevy::math::primitives::Annulus::new(#inner, #outer) }
        }
    }
}
impl elicitation::ElicitComplete for Annulus {}

// ── Ellipse ──────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Ellipse, as Ellipse);
elicit_newtype_traits!(Ellipse, bevy::math::primitives::Ellipse, [eq]);

impl serde::Serialize for Ellipse {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Ellipse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Ellipse::deserialize(d).map(|v| Ellipse(std::sync::Arc::new(v)))
    }
}
impl From<Ellipse> for bevy::math::primitives::Ellipse {
    fn from(v: Ellipse) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Ellipse {
    /// Half-width.
    #[tracing::instrument(skip(self))]
    pub fn ellipse_half_size_x(&self) -> f32 {
        self.0.half_size.x
    }
    /// Half-height.
    #[tracing::instrument(skip(self))]
    pub fn ellipse_half_size_y(&self) -> f32 {
        self.0.half_size.y
    }
    /// Constructs with half-width and half-height (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn ellipse_new(&self, half_width: f32, half_height: f32) -> Ellipse {
        Ellipse::from(bevy::math::primitives::Ellipse::new(
            half_width,
            half_height,
        ))
    }
}

mod emit_impls_ellipse {
    use super::Ellipse;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Ellipse {
        fn to_code_literal(&self) -> TokenStream {
            let (hw, hh) = (self.0.half_size.x, self.0.half_size.y);
            quote::quote! { ::bevy::math::primitives::Ellipse::new(#hw, #hh) }
        }
    }
}
impl elicitation::ElicitComplete for Ellipse {}

// ── Rectangle ────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Rectangle, as Rectangle);
elicit_newtype_traits!(Rectangle, bevy::math::primitives::Rectangle, [eq]);

impl serde::Serialize for Rectangle {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Rectangle {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Rectangle::deserialize(d).map(|v| Rectangle(std::sync::Arc::new(v)))
    }
}
impl From<Rectangle> for bevy::math::primitives::Rectangle {
    fn from(v: Rectangle) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Rectangle {
    /// Half-width.
    #[tracing::instrument(skip(self))]
    pub fn rectangle_half_size_x(&self) -> f32 {
        self.0.half_size.x
    }
    /// Half-height.
    #[tracing::instrument(skip(self))]
    pub fn rectangle_half_size_y(&self) -> f32 {
        self.0.half_size.y
    }
    /// Constructs with half-width and half-height (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn rectangle_new(&self, half_width: f32, half_height: f32) -> Rectangle {
        Rectangle::from(bevy::math::primitives::Rectangle::new(
            half_width,
            half_height,
        ))
    }
}

mod emit_impls_rect {
    use super::Rectangle;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Rectangle {
        fn to_code_literal(&self) -> TokenStream {
            let (hw, hh) = (self.0.half_size.x, self.0.half_size.y);
            quote::quote! { ::bevy::math::primitives::Rectangle::new(#hw, #hh) }
        }
    }
}
impl elicitation::ElicitComplete for Rectangle {}

// ── Triangle2d ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Triangle2d, as Triangle2d);
elicit_newtype_traits!(Triangle2d, bevy::math::primitives::Triangle2d, [eq]);

impl serde::Serialize for Triangle2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Triangle2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Triangle2d::deserialize(d)
            .map(|v| Triangle2d(std::sync::Arc::new(v)))
    }
}
impl From<Triangle2d> for bevy::math::primitives::Triangle2d {
    fn from(v: Triangle2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Triangle2d {
    /// First vertex X.
    #[tracing::instrument(skip(self))]
    pub fn triangle2d_v0_x(&self) -> f32 {
        self.0.vertices[0].x
    }
    /// First vertex Y.
    #[tracing::instrument(skip(self))]
    pub fn triangle2d_v0_y(&self) -> f32 {
        self.0.vertices[0].y
    }
    /// Second vertex X.
    #[tracing::instrument(skip(self))]
    pub fn v1_x(&self) -> f32 {
        self.0.vertices[1].x
    }
    /// Second vertex Y.
    #[tracing::instrument(skip(self))]
    pub fn v1_y(&self) -> f32 {
        self.0.vertices[1].y
    }
    /// Third vertex X.
    #[tracing::instrument(skip(self))]
    pub fn v2_x(&self) -> f32 {
        self.0.vertices[2].x
    }
    /// Third vertex Y.
    #[tracing::instrument(skip(self))]
    pub fn v2_y(&self) -> f32 {
        self.0.vertices[2].y
    }
    /// Constructs from three vertices (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn triangle2d_new(
        &self,
        ax: f32,
        ay: f32,
        bx: f32,
        by: f32,
        cx: f32,
        cy: f32,
    ) -> Triangle2d {
        Triangle2d::from(bevy::math::primitives::Triangle2d::new(
            bevy::math::Vec2::new(ax, ay),
            bevy::math::Vec2::new(bx, by),
            bevy::math::Vec2::new(cx, cy),
        ))
    }
}

mod emit_impls_tri2d {
    use super::Triangle2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Triangle2d {
        fn to_code_literal(&self) -> TokenStream {
            let v = &self.0.vertices;
            let (ax, ay) = (v[0].x, v[0].y);
            let (bx, by) = (v[1].x, v[1].y);
            let (cx, cy) = (v[2].x, v[2].y);
            quote::quote! {
                ::bevy::math::primitives::Triangle2d::new(
                    ::bevy::math::Vec2::new(#ax, #ay),
                    ::bevy::math::Vec2::new(#bx, #by),
                    ::bevy::math::Vec2::new(#cx, #cy),
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for Triangle2d {}

// ── Cuboid ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Cuboid, as Cuboid);
elicit_newtype_traits!(Cuboid, bevy::math::primitives::Cuboid, [eq]);

impl serde::Serialize for Cuboid {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Cuboid {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Cuboid::deserialize(d).map(|v| Cuboid(std::sync::Arc::new(v)))
    }
}
impl From<Cuboid> for bevy::math::primitives::Cuboid {
    fn from(v: Cuboid) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Cuboid {
    /// Half-size X.
    #[tracing::instrument(skip(self))]
    pub fn cuboid_half_size_x(&self) -> f32 {
        self.0.half_size.x
    }
    /// Half-size Y.
    #[tracing::instrument(skip(self))]
    pub fn cuboid_half_size_y(&self) -> f32 {
        self.0.half_size.y
    }
    /// Half-size Z.
    #[tracing::instrument(skip(self))]
    pub fn half_size_z(&self) -> f32 {
        self.0.half_size.z
    }
    /// Constructs from half-extents (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn cuboid_new(&self, hx: f32, hy: f32, hz: f32) -> Cuboid {
        Cuboid::from(bevy::math::primitives::Cuboid::new(
            hx * 2.0,
            hy * 2.0,
            hz * 2.0,
        ))
    }
    /// Constructs from full extents (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn from_size(&self, w: f32, h: f32, d: f32) -> Cuboid {
        Cuboid::from(bevy::math::primitives::Cuboid::new(w, h, d))
    }
}

mod emit_impls_cuboid {
    use super::Cuboid;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Cuboid {
        fn to_code_literal(&self) -> TokenStream {
            let (hx, hy, hz) = (
                self.0.half_size.x * 2.0,
                self.0.half_size.y * 2.0,
                self.0.half_size.z * 2.0,
            );
            quote::quote! { ::bevy::math::primitives::Cuboid::new(#hx, #hy, #hz) }
        }
    }
}
impl elicitation::ElicitComplete for Cuboid {}

// ── Cylinder ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Cylinder, as Cylinder);
elicit_newtype_traits!(Cylinder, bevy::math::primitives::Cylinder, [eq]);

impl serde::Serialize for Cylinder {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Cylinder {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Cylinder::deserialize(d).map(|v| Cylinder(std::sync::Arc::new(v)))
    }
}
impl From<Cylinder> for bevy::math::primitives::Cylinder {
    fn from(v: Cylinder) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Cylinder {
    /// Radius.
    #[tracing::instrument(skip(self))]
    pub fn cylinder_radius(&self) -> f32 {
        self.0.radius
    }
    /// Half-height.
    #[tracing::instrument(skip(self))]
    pub fn half_height(&self) -> f32 {
        self.0.half_height
    }
    /// Constructs from radius and half-height (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn cylinder_new(&self, radius: f32, half_height: f32) -> Cylinder {
        Cylinder::from(bevy::math::primitives::Cylinder::new(radius, half_height))
    }
}

mod emit_impls_cyl {
    use super::Cylinder;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Cylinder {
        fn to_code_literal(&self) -> TokenStream {
            let (r, hh) = (self.0.radius, self.0.half_height);
            quote::quote! { ::bevy::math::primitives::Cylinder::new(#r, #hh) }
        }
    }
}
impl elicitation::ElicitComplete for Cylinder {}

// ── Capsule2d ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Capsule2d, as Capsule2d);
elicit_newtype_traits!(Capsule2d, bevy::math::primitives::Capsule2d, [eq]);

impl serde::Serialize for Capsule2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Capsule2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Capsule2d::deserialize(d).map(|v| Capsule2d(std::sync::Arc::new(v)))
    }
}
impl From<Capsule2d> for bevy::math::primitives::Capsule2d {
    fn from(v: Capsule2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Capsule2d {
    /// Radius.
    #[tracing::instrument(skip(self))]
    pub fn capsule2d_radius(&self) -> f32 {
        self.0.radius
    }
    /// Half-length of the cylindrical segment.
    #[tracing::instrument(skip(self))]
    pub fn capsule2d_half_length(&self) -> f32 {
        self.0.half_length
    }
    /// Constructs from radius and half-length (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn capsule2d_new(&self, radius: f32, half_length: f32) -> Capsule2d {
        Capsule2d::from(bevy::math::primitives::Capsule2d::new(radius, half_length))
    }
}

mod emit_impls_cap2d {
    use super::Capsule2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Capsule2d {
        fn to_code_literal(&self) -> TokenStream {
            let (r, hl) = (self.0.radius, self.0.half_length);
            quote::quote! { ::bevy::math::primitives::Capsule2d::new(#r, #hl) }
        }
    }
}
impl elicitation::ElicitComplete for Capsule2d {}

// ── Capsule3d ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Capsule3d, as Capsule3d);
elicit_newtype_traits!(Capsule3d, bevy::math::primitives::Capsule3d, [eq]);

impl serde::Serialize for Capsule3d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Capsule3d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Capsule3d::deserialize(d).map(|v| Capsule3d(std::sync::Arc::new(v)))
    }
}
impl From<Capsule3d> for bevy::math::primitives::Capsule3d {
    fn from(v: Capsule3d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Capsule3d {
    /// Radius.
    #[tracing::instrument(skip(self))]
    pub fn capsule3d_radius(&self) -> f32 {
        self.0.radius
    }
    /// Half-length.
    #[tracing::instrument(skip(self))]
    pub fn capsule3d_half_length(&self) -> f32 {
        self.0.half_length
    }
    /// Constructs from radius and half-length (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn capsule3d_new(&self, radius: f32, half_length: f32) -> Capsule3d {
        Capsule3d::from(bevy::math::primitives::Capsule3d::new(radius, half_length))
    }
}

mod emit_impls_cap3d {
    use super::Capsule3d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Capsule3d {
        fn to_code_literal(&self) -> TokenStream {
            let (r, hl) = (self.0.radius, self.0.half_length);
            quote::quote! { ::bevy::math::primitives::Capsule3d::new(#r, #hl) }
        }
    }
}
impl elicitation::ElicitComplete for Capsule3d {}

// ── Torus ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Torus, as Torus);
elicit_newtype_traits!(Torus, bevy::math::primitives::Torus, [eq]);

impl serde::Serialize for Torus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Torus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Torus::deserialize(d).map(|v| Torus(std::sync::Arc::new(v)))
    }
}
impl From<Torus> for bevy::math::primitives::Torus {
    fn from(v: Torus) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Torus {
    /// Minor (tube) radius.
    #[tracing::instrument(skip(self))]
    pub fn minor_radius(&self) -> f32 {
        self.0.minor_radius
    }
    /// Major radius (distance from center of tube to center of torus).
    #[tracing::instrument(skip(self))]
    pub fn major_radius(&self) -> f32 {
        self.0.major_radius
    }
    /// Constructs from minor and major radius (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn torus_new(&self, minor: f32, major: f32) -> Torus {
        Torus::from(bevy::math::primitives::Torus::new(minor, major))
    }
}

mod emit_impls_torus {
    use super::Torus;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Torus {
        fn to_code_literal(&self) -> TokenStream {
            let (minor, major) = (self.0.minor_radius, self.0.major_radius);
            quote::quote! { ::bevy::math::primitives::Torus::new(#minor, #major) }
        }
    }
}
impl elicitation::ElicitComplete for Torus {}

// ── RegularPolygon ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::RegularPolygon, as RegularPolygon);
elicit_newtype_traits!(RegularPolygon, bevy::math::primitives::RegularPolygon, [eq]);

impl serde::Serialize for RegularPolygon {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for RegularPolygon {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::RegularPolygon::deserialize(d)
            .map(|v| RegularPolygon(std::sync::Arc::new(v)))
    }
}
impl From<RegularPolygon> for bevy::math::primitives::RegularPolygon {
    fn from(v: RegularPolygon) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl RegularPolygon {
    /// Number of sides.
    #[tracing::instrument(skip(self))]
    pub fn sides(&self) -> u32 {
        self.0.sides as u32
    }
    /// Circumradius (radius of circumscribed circle).
    #[tracing::instrument(skip(self))]
    pub fn circumradius(&self) -> f32 {
        self.0.circumcircle.radius
    }
    /// Constructs from sides and circumradius (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn regular_polygon_new(&self, sides: u32, circumradius: f32) -> RegularPolygon {
        RegularPolygon::from(bevy::math::primitives::RegularPolygon::new(
            circumradius,
            sides,
        ))
    }
}

mod emit_impls_rp {
    use super::RegularPolygon;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for RegularPolygon {
        fn to_code_literal(&self) -> TokenStream {
            let r = self.0.circumcircle.radius;
            let s = self.0.sides;
            quote::quote! { ::bevy::math::primitives::RegularPolygon::new(#r, #s) }
        }
    }
}
impl elicitation::ElicitComplete for RegularPolygon {}

// ── Rhombus ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Rhombus, as Rhombus);
elicit_newtype_traits!(Rhombus, bevy::math::primitives::Rhombus, [eq]);

impl serde::Serialize for Rhombus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Rhombus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Rhombus::deserialize(d).map(|v| Rhombus(std::sync::Arc::new(v)))
    }
}
impl From<Rhombus> for bevy::math::primitives::Rhombus {
    fn from(v: Rhombus) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Rhombus {
    /// Half-width diagonal.
    #[tracing::instrument(skip(self))]
    pub fn half_diagonals_x(&self) -> f32 {
        self.0.half_diagonals.x
    }
    /// Half-height diagonal.
    #[tracing::instrument(skip(self))]
    pub fn half_diagonals_y(&self) -> f32 {
        self.0.half_diagonals.y
    }
    /// Constructs from full diagonal widths (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn rhombus_new(&self, dx: f32, dy: f32) -> Rhombus {
        Rhombus::from(bevy::math::primitives::Rhombus::new(dx, dy))
    }
}

mod emit_impls_rhombus {
    use super::Rhombus;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Rhombus {
        fn to_code_literal(&self) -> TokenStream {
            let (dx, dy) = (self.0.half_diagonals.x * 2.0, self.0.half_diagonals.y * 2.0);
            quote::quote! { ::bevy::math::primitives::Rhombus::new(#dx, #dy) }
        }
    }
}
impl elicitation::ElicitComplete for Rhombus {}

// ── Plane2d ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Plane2d, as Plane2d);
elicit_newtype_traits!(Plane2d, bevy::math::primitives::Plane2d, [eq]);

impl serde::Serialize for Plane2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Plane2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Plane2d::deserialize(d).map(|v| Plane2d(std::sync::Arc::new(v)))
    }
}
impl From<Plane2d> for bevy::math::primitives::Plane2d {
    fn from(v: Plane2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Plane2d {
    /// Normal X.
    #[tracing::instrument(skip(self))]
    pub fn plane2d_normal_x(&self) -> f32 {
        self.0.normal.x
    }
    /// Normal Y.
    #[tracing::instrument(skip(self))]
    pub fn plane2d_normal_y(&self) -> f32 {
        self.0.normal.y
    }
    /// Constructs from normal direction (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn plane2d_new(&self, nx: f32, ny: f32) -> Option<Plane2d> {
        bevy::math::Dir2::new(bevy::math::Vec2::new(nx, ny))
            .ok()
            .map(|n| Plane2d::from(bevy::math::primitives::Plane2d { normal: n }))
    }
}

mod emit_impls_plane2 {
    use super::Plane2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Plane2d {
        fn to_code_literal(&self) -> TokenStream {
            let (nx, ny) = (self.0.normal.x, self.0.normal.y);
            quote::quote! {
                ::bevy::math::primitives::Plane2d {
                    normal: ::bevy::math::Dir2::new(
                        ::bevy::math::Vec2::new(#nx, #ny)
                    ).unwrap(),
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Plane2d {}

// ── Plane3d ───────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Plane3d, as Plane3d);
elicit_newtype_traits!(Plane3d, bevy::math::primitives::Plane3d, [eq]);

impl serde::Serialize for Plane3d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Plane3d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Plane3d::deserialize(d).map(|v| Plane3d(std::sync::Arc::new(v)))
    }
}
impl From<Plane3d> for bevy::math::primitives::Plane3d {
    fn from(v: Plane3d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Plane3d {
    /// Normal X.
    #[tracing::instrument(skip(self))]
    pub fn plane3d_normal_x(&self) -> f32 {
        self.0.normal.x
    }
    /// Normal Y.
    #[tracing::instrument(skip(self))]
    pub fn plane3d_normal_y(&self) -> f32 {
        self.0.normal.y
    }
    /// Normal Z.
    #[tracing::instrument(skip(self))]
    pub fn normal_z(&self) -> f32 {
        self.0.normal.z
    }
}

mod emit_impls_plane3 {
    use super::Plane3d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Plane3d {
        fn to_code_literal(&self) -> TokenStream {
            let (nx, ny, nz) = (self.0.normal.x, self.0.normal.y, self.0.normal.z);
            quote::quote! {
                ::bevy::math::primitives::Plane3d {
                    normal: ::bevy::math::Dir3::new(
                        ::bevy::math::Vec3::new(#nx, #ny, #nz)
                    ).unwrap(),
                    ..Default::default()
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Plane3d {}

// ── Cone ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Cone, as Cone);
elicit_newtype_traits!(Cone, bevy::math::primitives::Cone, [eq]);

impl serde::Serialize for Cone {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Cone {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Cone::deserialize(d).map(|v| Cone(std::sync::Arc::new(v)))
    }
}
impl From<Cone> for bevy::math::primitives::Cone {
    fn from(v: Cone) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Cone {
    /// Base radius.
    #[tracing::instrument(skip(self))]
    pub fn cone_radius(&self) -> f32 {
        self.0.radius
    }
    /// Height.
    #[tracing::instrument(skip(self))]
    pub fn cone_height(&self) -> f32 {
        self.0.height
    }
    /// Constructs from radius and height (ignores self).
    #[tracing::instrument(skip(self))]
    pub fn cone_new(&self, radius: f32, height: f32) -> Cone {
        Cone::from(bevy::math::primitives::Cone { radius, height })
    }
}

mod emit_impls_cone {
    use super::Cone;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Cone {
        fn to_code_literal(&self) -> TokenStream {
            let (r, h) = (self.0.radius, self.0.height);
            quote::quote! { ::bevy::math::primitives::Cone { radius: #r, height: #h } }
        }
    }
}
impl elicitation::ElicitComplete for Cone {}

// ── ConicalFrustum ────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::ConicalFrustum, as ConicalFrustum);
elicit_newtype_traits!(ConicalFrustum, bevy::math::primitives::ConicalFrustum, [eq]);

impl serde::Serialize for ConicalFrustum {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for ConicalFrustum {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::ConicalFrustum::deserialize(d)
            .map(|v| ConicalFrustum(std::sync::Arc::new(v)))
    }
}
impl From<ConicalFrustum> for bevy::math::primitives::ConicalFrustum {
    fn from(v: ConicalFrustum) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl ConicalFrustum {
    /// Bottom (base) radius.
    #[tracing::instrument(skip(self))]
    pub fn radius_bottom(&self) -> f32 {
        self.0.radius_bottom
    }
    /// Top radius.
    #[tracing::instrument(skip(self))]
    pub fn radius_top(&self) -> f32 {
        self.0.radius_top
    }
    /// Height.
    #[tracing::instrument(skip(self))]
    pub fn conical_frustum_height(&self) -> f32 {
        self.0.height
    }
}

mod emit_impls_cf {
    use super::ConicalFrustum;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for ConicalFrustum {
        fn to_code_literal(&self) -> TokenStream {
            let (rb, rt, h) = (self.0.radius_bottom, self.0.radius_top, self.0.height);
            quote::quote! {
                ::bevy::math::primitives::ConicalFrustum {
                    radius_bottom: #rb, radius_top: #rt, height: #h,
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for ConicalFrustum {}

// ── Arc2d ─────────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Arc2d, as Arc2d);
elicit_newtype_traits!(Arc2d, bevy::math::primitives::Arc2d, [eq]);

impl serde::Serialize for Arc2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Arc2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Arc2d::deserialize(d).map(|v| Arc2d(std::sync::Arc::new(v)))
    }
}
impl From<Arc2d> for bevy::math::primitives::Arc2d {
    fn from(v: Arc2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Arc2d {
    /// Radius.
    #[tracing::instrument(skip(self))]
    pub fn arc2d_radius(&self) -> f32 {
        self.0.radius
    }
    /// Half-angle in radians.
    #[tracing::instrument(skip(self))]
    pub fn half_angle(&self) -> f32 {
        self.0.half_angle
    }
}

mod emit_impls_arc2d {
    use super::Arc2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Arc2d {
        fn to_code_literal(&self) -> TokenStream {
            let (r, ha) = (self.0.radius, self.0.half_angle);
            quote::quote! {
                ::bevy::math::primitives::Arc2d { radius: #r, half_angle: #ha }
            }
        }
    }
}
impl elicitation::ElicitComplete for Arc2d {}

// ── Segment2d ─────────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Segment2d, as Segment2d);
elicit_newtype_traits!(Segment2d, bevy::math::primitives::Segment2d, [eq]);

impl serde::Serialize for Segment2d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Segment2d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Segment2d::deserialize(d).map(|v| Segment2d(std::sync::Arc::new(v)))
    }
}
impl From<Segment2d> for bevy::math::primitives::Segment2d {
    fn from(v: Segment2d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Segment2d {
    /// X coordinate of the first vertex.
    #[tracing::instrument(skip(self))]
    pub fn segment2d_v0_x(&self) -> f32 {
        self.0.vertices[0].x
    }
    /// Y coordinate of the first vertex.
    #[tracing::instrument(skip(self))]
    pub fn segment2d_v0_y(&self) -> f32 {
        self.0.vertices[0].y
    }
    /// X coordinate of the second vertex.
    #[tracing::instrument(skip(self))]
    pub fn segment2d_v1_x(&self) -> f32 {
        self.0.vertices[1].x
    }
    /// Y coordinate of the second vertex.
    #[tracing::instrument(skip(self))]
    pub fn segment2d_v1_y(&self) -> f32 {
        self.0.vertices[1].y
    }
}

mod emit_impls_seg2d {
    use super::Segment2d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Segment2d {
        fn to_code_literal(&self) -> TokenStream {
            let (v0x, v0y) = (self.0.vertices[0].x, self.0.vertices[0].y);
            let (v1x, v1y) = (self.0.vertices[1].x, self.0.vertices[1].y);
            quote::quote! {
                ::bevy::math::primitives::Segment2d {
                    vertices: [
                        ::bevy::math::Vec2::new(#v0x, #v0y),
                        ::bevy::math::Vec2::new(#v1x, #v1y),
                    ],
                }
            }
        }
    }
}
impl elicitation::ElicitComplete for Segment2d {}

// ── Triangle3d ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Triangle3d, as Triangle3d);
elicit_newtype_traits!(Triangle3d, bevy::math::primitives::Triangle3d, [eq]);

impl serde::Serialize for Triangle3d {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Triangle3d {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Triangle3d::deserialize(d)
            .map(|v| Triangle3d(std::sync::Arc::new(v)))
    }
}
impl From<Triangle3d> for bevy::math::primitives::Triangle3d {
    fn from(v: Triangle3d) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Triangle3d {
    /// First vertex X.
    #[tracing::instrument(skip(self))]
    pub fn triangle3d_v0_x(&self) -> f32 {
        self.0.vertices[0].x
    }
    /// First vertex Y.
    #[tracing::instrument(skip(self))]
    pub fn triangle3d_v0_y(&self) -> f32 {
        self.0.vertices[0].y
    }
    /// First vertex Z.
    #[tracing::instrument(skip(self))]
    pub fn v0_z(&self) -> f32 {
        self.0.vertices[0].z
    }
}

mod emit_impls_tri3d {
    use super::Triangle3d;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Triangle3d {
        fn to_code_literal(&self) -> TokenStream {
            let v = &self.0.vertices;
            let (ax, ay, az) = (v[0].x, v[0].y, v[0].z);
            let (bx, by, bz) = (v[1].x, v[1].y, v[1].z);
            let (cx, cy, cz) = (v[2].x, v[2].y, v[2].z);
            quote::quote! {
                ::bevy::math::primitives::Triangle3d::new(
                    ::bevy::math::Vec3::new(#ax, #ay, #az),
                    ::bevy::math::Vec3::new(#bx, #by, #bz),
                    ::bevy::math::Vec3::new(#cx, #cy, #cz),
                )
            }
        }
    }
}
impl elicitation::ElicitComplete for Triangle3d {}

// ── Tetrahedron ───────────────────────────────────────────────────────────────

elicit_newtype!(bevy::math::primitives::Tetrahedron, as Tetrahedron);
elicit_newtype_traits!(Tetrahedron, bevy::math::primitives::Tetrahedron, [eq]);

impl serde::Serialize for Tetrahedron {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        (*self.0).serialize(s)
    }
}
impl<'de> serde::Deserialize<'de> for Tetrahedron {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        bevy::math::primitives::Tetrahedron::deserialize(d)
            .map(|v| Tetrahedron(std::sync::Arc::new(v)))
    }
}
impl From<Tetrahedron> for bevy::math::primitives::Tetrahedron {
    fn from(v: Tetrahedron) -> Self {
        *v.0
    }
}

#[reflect_methods]
impl Tetrahedron {
    /// First vertex X.
    #[tracing::instrument(skip(self))]
    pub fn tetrahedron_v0_x(&self) -> f32 {
        self.0.vertices[0].x
    }
    /// Number of vertices (always 4).
    #[tracing::instrument(skip(self))]
    pub fn vertex_count(&self) -> usize {
        4
    }
}

mod emit_impls_tet {
    use super::Tetrahedron;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;
    impl ToCodeLiteral for Tetrahedron {
        fn to_code_literal(&self) -> TokenStream {
            let v = &self.0.vertices;
            let verts: Vec<_> = v
                .iter()
                .map(|p| {
                    let (x, y, z) = (p.x, p.y, p.z);
                    quote::quote! { ::bevy::math::Vec3::new(#x, #y, #z) }
                })
                .collect();
            quote::quote! {
                ::bevy::math::primitives::Tetrahedron::new(#(#verts),*)
            }
        }
    }
}
impl elicitation::ElicitComplete for Tetrahedron {}
