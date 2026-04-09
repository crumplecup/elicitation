//! Re-exports of geo algorithm traits for use with elicitation geo wrappers.
//!
//! Enabled by the `geo` feature.

pub use geo::line_measures::{
    Bearing, Destination, Distance, FrechetDistance, InterpolateLine, InterpolatePoint, Length,
};
pub use geo::{
    Area, BooleanOps, BoundingRect, Centroid, ClosestPoint, ConcaveHull, Contains,
    ContainsProperly, ConvexHull, CoordsIter, Covers, HasDimensions, HausdorffDistance,
    InteriorPoint, Intersects, IsConvex, LineLocatePoint, RemoveRepeatedPoints, Rotate, Scale,
    Simplify, SimplifyVw, Translate, Within,
};
pub use geo::{Euclidean, Geodesic, Haversine};
