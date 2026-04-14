//! SFS spatial-predicate and metric-operation traits.
//!
//! Source: OGC 06-103r4 §6.2 — DE-9IM predicates; §6.3 — metric operations.

use crate::{GisResult, traits::SfsGeometry};

/// DE-9IM spatial predicates and metric operations defined by OGC SFS.
///
/// All methods are object-safe: parameters use `&dyn SfsGeometry` rather than
/// `&Self`, and returned geometry values are boxed.
///
/// Source: OGC 06-103r4 §6.2 (predicates) and §6.3 (metrics).
pub trait SfsTopology: SfsGeometry {
    // ── §6.2 DE-9IM Predicates ────────────────────────────────────────────

    /// Returns `true` when `self` and `other` represent the same point set.
    ///
    /// DE-9IM pattern: T*F**FFF*.
    ///
    /// Source: OGC 06-103r4 §6.2.1 — Equals.
    fn equals(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when `self` and `other` share no points.
    ///
    /// DE-9IM pattern: FF*FF****.
    ///
    /// Source: OGC 06-103r4 §6.2.2 — Disjoint.
    fn disjoint(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when `self` and `other` share at least one point.
    ///
    /// Complement of `disjoint`. DE-9IM: T******** | *T******* | ***T***** |
    /// ****T****.
    ///
    /// Source: OGC 06-103r4 §6.2.3 — Intersects.
    fn intersects(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when the geometries share at least one boundary point
    /// and their interiors do not intersect.
    ///
    /// DE-9IM pattern: FT******* | F**T***** | F***T****.
    ///
    /// Source: OGC 06-103r4 §6.2.4 — Touches.
    fn touches(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when the geometries have interiors that intersect and
    /// the dimension of the intersection is less than the maximum of the two
    /// input dimensions.
    ///
    /// Source: OGC 06-103r4 §6.2.5 — Crosses.
    fn crosses(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when `self` lies in the interior or boundary of `other`.
    ///
    /// Converse of `contains`. DE-9IM pattern: T*F**F***.
    ///
    /// Source: OGC 06-103r4 §6.2.6 — Within.
    fn within(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when `other` lies in the interior or boundary of `self`.
    ///
    /// Converse of `within`. DE-9IM pattern: T*****FF*.
    ///
    /// Source: OGC 06-103r4 §6.2.7 — Contains.
    fn contains(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when `self` and `other` share interior points and
    /// neither completely contains the other.
    ///
    /// Source: OGC 06-103r4 §6.2.8 — Overlaps.
    fn overlaps(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when every point of `other` is in the interior or
    /// boundary of `self` (no point of `other` is in the exterior of `self`).
    ///
    /// Source: OGC 06-103r4 §6.2 Annex — Covers.
    fn covers(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Returns `true` when every point of `self` is in the interior or
    /// boundary of `other`.
    ///
    /// Converse of `covers`.
    ///
    /// Source: OGC 06-103r4 §6.2 Annex — CoveredBy.
    fn covered_by(&self, other: &dyn SfsGeometry) -> GisResult<bool>;

    /// Tests whether the actual DE-9IM relationship between `self` and `other`
    /// matches the given 9-character `pattern`.
    ///
    /// Pattern characters: `T`, `F`, `0`, `1`, `2`, `*`.
    ///
    /// Source: OGC 06-103r4 §6.2.9 — relate(g, pattern).
    fn relate(&self, other: &dyn SfsGeometry, pattern: &str) -> GisResult<bool>;

    // ── §6.3 Metric Operations ────────────────────────────────────────────

    /// Returns the area of the geometry in squared SRS units.
    ///
    /// Returns 0.0 for empty, point, and line geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    fn area(&self) -> GisResult<f64>;

    /// Returns the length of the geometry in SRS units.
    ///
    /// Returns 0.0 for empty, point, and surface geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    fn length(&self) -> GisResult<f64>;

    /// Returns the minimum distance between `self` and `other` in SRS units.
    ///
    /// Returns 0.0 when the geometries intersect.
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    fn distance(&self, other: &dyn SfsGeometry) -> GisResult<f64>;

    /// Returns the mathematical centroid as a Point.
    ///
    /// Source: OGC 06-103r4 §6.3 — centroid().
    fn centroid(&self) -> GisResult<Box<dyn SfsGeometry>>;

    /// Returns a Point guaranteed to lie on the geometry.
    ///
    /// Source: OGC 06-103r4 §6.3 — pointOnSurface().
    fn point_on_surface(&self) -> GisResult<Box<dyn SfsGeometry>>;
}
