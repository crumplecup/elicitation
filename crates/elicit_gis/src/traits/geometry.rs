//! SFS geometry intrinsic-property trait.
//!
//! Source: OGC 06-103r4 §6.1.1 — Geometry class interface.

use crate::GisResult;

/// Intrinsic properties of any OGC Simple Features geometry.
///
/// This trait is object-safe: all methods return concrete types.
/// Spatial predicates live in [`SfsTopology`][crate::traits::SfsTopology];
/// constructive operations live in [`SfsSetOps`][crate::traits::SfsSetOps].
///
/// Source: OGC 06-103r4 §4–§6.1.2 — Geometry class interface.
pub trait SfsGeometry: Send + Sync {
    /// Returns the name of the concrete geometry type.
    ///
    /// E.g. `"Point"`, `"Polygon"`, `"MultiLineString"`.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — geometryType().
    fn geometry_type(&self) -> &str;

    /// Returns the integer SRID of the associated spatial reference system.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — SRID().
    fn srid(&self) -> i32;

    /// Returns the topological dimension.
    ///
    /// Returns -1 for empty, 0 for point types, 1 for line types, 2 for
    /// surface types.
    ///
    /// Source: OGC 06-103r4 §4.2 / §6.1.1 — dimension().
    fn dimension(&self) -> i32;

    /// Returns the coordinate dimensionality.
    ///
    /// 2 for XY, 3 for XYZ or XYM, 4 for XYZM.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    fn coord_dimension(&self) -> u32;

    /// Returns `true` when the geometry contains no points.
    ///
    /// Source: OGC 06-103r4 §4.3 / §6.1.1 — isEmpty().
    fn is_empty(&self) -> bool;

    /// Returns `true` when the geometry has no anomalous geometric points such
    /// as self-intersections or self-tangencies.
    ///
    /// Source: OGC 06-103r4 §4.3 / §6.1.1 — isSimple().
    fn is_simple(&self) -> GisResult<bool>;

    /// Returns `true` when the geometry satisfies all §6.1.3 validity rules.
    ///
    /// Source: OGC 06-103r4 §4.3 / §6.1.3 — isValid().
    fn is_valid(&self) -> GisResult<bool>;

    /// Returns the WKT representation (§7.2).
    ///
    /// Source: OGC 06-103r4 §7.2 — asText().
    fn as_text(&self) -> GisResult<String>;

    /// Returns the WKB representation in NDR (little-endian) byte order (§7.3).
    ///
    /// Source: OGC 06-103r4 §7.3 — asBinary().
    fn as_binary(&self) -> GisResult<Vec<u8>>;

    /// Returns the minimum bounding rectangle as a boxed geometry.
    ///
    /// Returns a POLYGON for non-degenerate cases, a POINT for a single-point
    /// geometry, and an empty geometry when `self` is empty.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — envelope().
    fn envelope(&self) -> GisResult<Box<dyn SfsGeometry>>;

    /// Returns the boundary as a boxed geometry, or `None` for a
    /// GeometryCollection where the boundary is defined by the mod-2 rule.
    ///
    /// Source: OGC 06-103r4 §6.1.4 — boundary() per-type semantics.
    fn boundary(&self) -> GisResult<Option<Box<dyn SfsGeometry>>>;
}
