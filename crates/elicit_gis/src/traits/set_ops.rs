//! SFS constructive / set-operation trait.
//!
//! Source: OGC 06-103r4 §6.1.2 — Geometry class interface (set operations).

use crate::{GisResult, SfsGeometryMeta};

/// Constructive geometry operations defined by the OGC Simple Features
/// Specification geometry class interface.
///
/// All methods are object-safe: `other` is `&dyn SfsGeometryMeta` and returned
/// geometry values are boxed.
///
/// Source: OGC 06-103r4 §6.1.2 — buffer, convexHull, intersection, union,
/// difference, symDifference.
pub trait SfsSetOps: SfsGeometryMeta {
    /// Returns a geometry that contains all points within `distance` of `self`.
    ///
    /// A distance of 0.0 returns a geometry containing `self`.
    /// A positive distance expands; a negative distance contracts inward.
    /// The result is always a valid geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
    fn buffer(&self, distance: f64) -> GisResult<Box<dyn SfsGeometryMeta>>;

    /// Returns the smallest convex geometry that contains all points of `self`.
    ///
    /// Idempotent: convexHull(convexHull(g)) == convexHull(g).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — convexHull().
    fn convex_hull(&self) -> GisResult<Box<dyn SfsGeometryMeta>>;

    /// Returns a geometry containing only points in both `self` and `other`.
    ///
    /// Commutative. The result has dimension ≤ min(dim(self), dim(other)).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — intersection(g).
    fn intersection(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;

    /// Returns a geometry containing all points in `self` or `other`.
    ///
    /// Commutative and associative. The result has dimension ==
    /// max(dim(self), dim(other)) for non-empty inputs.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — union(g).
    fn union(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;

    /// Returns a geometry containing points in `self` that are not in `other`.
    ///
    /// Asymmetric: A.difference(B) ≠ B.difference(A) in general.
    /// The result is a subset of `self` and is disjoint from `other`.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — difference(g).
    fn difference(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;

    /// Returns a geometry containing points in `self` or `other` but not both.
    ///
    /// Commutative. Equivalent to union(a, b).difference(intersection(a, b)).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
    fn sym_difference(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;
}
