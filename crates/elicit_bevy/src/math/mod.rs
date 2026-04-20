//! Bevy math shadow types (glam re-exports).
//!
//! Each type is wrapped in `Arc<T>` via `elicit_newtype!`, exposing all
//! key methods as MCP tools through `#[reflect_methods]`.

mod affine;
mod dir;
mod isometry;
mod mat;
mod quat;
mod ray;
mod rect;
mod rotation;
mod shapes;
mod vec;

pub use affine::{Affine2, Affine3A, DAffine2, DAffine3};
pub use dir::{Dir2, Dir3, Dir3A};
pub use isometry::{Isometry2d, Isometry3d};
pub use mat::{DMat2, DMat3, DMat4, Mat2, Mat3, Mat3A, Mat4};
pub use quat::{DQuat, Quat};
pub use ray::{Ray2d, Ray3d};
pub use rect::{IRect, Rect, URect};
pub use rotation::Rot2;
pub use shapes::{
    Annulus, Arc2d, Capsule2d, Capsule3d, Circle, Cone, ConicalFrustum, Cuboid, Cylinder, Ellipse,
    Plane2d, Plane3d, Rectangle, RegularPolygon, Rhombus, Segment2d, Sphere, Tetrahedron, Torus,
    Triangle2d, Triangle3d,
};
pub use vec::{
    DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A, Vec4,
};
