//! OGC SFS geometry construction, identity, and I/O traits.
//!
//! Three-role taxonomy applied to OGC Simple Features:
//!
//! * **[`SfsGeometryFactory`]** (factory / builder) — raw coordinate sequences
//!   plus precondition proof tokens flow in; a geometry descriptor plus a
//!   postcondition [`Established<P>`] token flows out.  Leaf builders
//!   ([`build_point`], [`build_line_string`], [`build_linear_ring`]) require
//!   only raw coordinates.  Composite builders ([`build_polygon`],
//!   [`build_multi_point`], etc.) demand `Established<P>` tokens from their
//!   already-validated components as typed preconditions, encoding the SFS
//!   validity dependency graph in the type system.
//!
//! * **[`SfsGeometryMeta`]** (orthogonal concern — identity) — intrinsic
//!   geometry metadata: type name, SRID, dimensionality, emptiness, simplicity,
//!   validity reporting, envelope, and boundary.  Works on any geometry
//!   regardless of structural correctness; even a malformed geometry has a
//!   type name and an SRID.
//!
//! * **[`SfsGeometryIo`]** (orthogonal concern — serialization) — WKT and WKB
//!   encoding.  Orthogonal to geometry type and structural correctness; any
//!   geometry, even a malformed one, can be serialized.  WKT/WKB *parsing*
//!   lives in [`SfsGeometryFactory`] because parsing is construction and emits
//!   validity proof tokens.
//!
//! Source: OGC 06-103r4.
//!
//! [`build_point`]: SfsGeometryFactory::build_point
//! [`build_line_string`]: SfsGeometryFactory::build_line_string
//! [`build_linear_ring`]: SfsGeometryFactory::build_linear_ring
//! [`build_polygon`]: SfsGeometryFactory::build_polygon
//! [`build_multi_point`]: SfsGeometryFactory::build_multi_point
//! [`Established<P>`]: elicitation::Established

use elicitation::Established;

use crate::{
    GeometryCollectionDescriptor, GeometryCollectionValid, GisResult, LineStringDescriptor,
    LineStringValid, LinearRingDescriptor, LinearRingValid, MultiGeometryDescriptor,
    MultiLineStringValid, MultiPointValid, MultiPolygonValid, PointDescriptor, PointValid,
    PolygonDescriptor, PolygonValid, SfsCoordinate, SfsCoordinate3D, SfsGeometryValid,
};

// ── Factory (Role 1) ──────────────────────────────────────────────────────────

/// Build and validate SFS geometry objects against the OGC SFS §6.1 rules.
///
/// Each method is a **factory**: it accepts raw coordinate data (and, for
/// composite types, proof tokens from already-validated components) and returns
/// either an error or a geometry descriptor together with an
/// [`Established<P>`] token certifying the validity proposition.
///
/// # Leaf builders
///
/// [`build_point`], [`build_point_3d`], [`build_point_empty`],
/// [`build_line_string`], and [`build_linear_ring`] stand at the bottom of
/// the geometry dependency graph.  They require only raw coordinate data.
///
/// # Composite builders
///
/// [`build_polygon`], [`build_multi_point`], [`build_multi_line_string`],
/// [`build_multi_polygon`], and [`build_geometry_collection`] demand
/// `Established<P>` tokens from their components as typed preconditions,
/// encoding the SFS validity dependency graph in the type system.  A Polygon
/// cannot be assembled without handing in `Established<LinearRingValid>` tokens
/// for each of its rings; a MultiPolygon requires `Established<PolygonValid>`
/// tokens for each component.
///
/// # Parse factories
///
/// [`geometry_from_wkt`] and [`geometry_from_wkb`] construct a geometry from
/// its serialized form.  On success they emit `Established<SfsGeometryValid>`,
/// placing WKT/WKB parsing in the same validity-by-construction pattern as the
/// typed builders.
///
/// # Object safety
///
/// All method signatures use only concrete parameter and return types.  The
/// trait is `dyn`-compatible.
///
/// Source: OGC 06-103r4 §6.1 — geometry validity rules.
///
/// [`build_point`]: SfsGeometryFactory::build_point
/// [`build_point_3d`]: SfsGeometryFactory::build_point_3d
/// [`build_point_empty`]: SfsGeometryFactory::build_point_empty
/// [`build_line_string`]: SfsGeometryFactory::build_line_string
/// [`build_linear_ring`]: SfsGeometryFactory::build_linear_ring
/// [`build_polygon`]: SfsGeometryFactory::build_polygon
/// [`build_multi_point`]: SfsGeometryFactory::build_multi_point
/// [`build_multi_line_string`]: SfsGeometryFactory::build_multi_line_string
/// [`build_multi_polygon`]: SfsGeometryFactory::build_multi_polygon
/// [`build_geometry_collection`]: SfsGeometryFactory::build_geometry_collection
/// [`geometry_from_wkt`]: SfsGeometryFactory::geometry_from_wkt
/// [`geometry_from_wkb`]: SfsGeometryFactory::geometry_from_wkb
/// [`Established<P>`]: elicitation::Established
pub trait SfsGeometryFactory: Send + Sync {
    // ── Leaf builders ─────────────────────────────────────────────────────

    /// Build a 2D Point from finite XY coordinates.
    ///
    /// Validates: PointXIsFinite, PointYIsFinite, PointValid.
    ///
    /// Source: OGC 06-103r4 §6.1.4 — Point.
    fn build_point(
        &self,
        coord: SfsCoordinate,
        srid: Option<i32>,
    ) -> GisResult<(PointDescriptor, Established<PointValid>)>;

    /// Build a 3D Point from finite XYZ coordinates.
    ///
    /// Validates: PointXIsFinite, PointYIsFinite, PointZIsFiniteWhenPresent,
    /// PointValid.
    ///
    /// Source: OGC 06-103r4 §6.1.4 — Point with Z.
    fn build_point_3d(
        &self,
        coord: SfsCoordinate3D,
        srid: Option<i32>,
    ) -> GisResult<(PointDescriptor, Established<PointValid>)>;

    /// Build an empty Point.
    ///
    /// An empty Point is always valid (PointAlwaysValid, PointEmptyIsEmpty,
    /// PointEmptyHasNoCoords).  Returns a proof token without fallibility.
    ///
    /// Source: OGC 06-103r4 §6.1.4 — empty Point.
    fn build_point_empty(&self, srid: Option<i32>) -> (PointDescriptor, Established<PointValid>);

    /// Build a LineString from two or more coordinate positions.
    ///
    /// Validates: LineStringHasTwoOrMorePoints, CoordXIsFinite, CoordYIsFinite,
    /// LineStringAdjacentPointsDistinct, LineStringSimpleNoSelfIntersection,
    /// LineStringValid.
    ///
    /// Source: OGC 06-103r4 §6.1.6 — LineString.
    fn build_line_string(
        &self,
        coords: Vec<SfsCoordinate>,
        srid: Option<i32>,
    ) -> GisResult<(LineStringDescriptor, Established<LineStringValid>)>;

    /// Build a LinearRing from four or more coordinate positions.
    ///
    /// Validates: LinearRingMinimumFourPositions, LinearRingFirstPositionEqualsLast,
    /// LinearRingIsClosedLineString, LinearRingIsSimple, LinearRingNonDegenerate,
    /// CoordXIsFinite, CoordYIsFinite, LinearRingValid.
    ///
    /// Source: OGC 06-103r4 §6.1.7 — LinearRing.
    fn build_linear_ring(
        &self,
        coords: Vec<SfsCoordinate>,
        srid: Option<i32>,
    ) -> GisResult<(LinearRingDescriptor, Established<LinearRingValid>)>;

    // ── Composite builders ────────────────────────────────────────────────

    /// Build a Polygon from a validated exterior ring and zero or more
    /// validated interior (hole) rings.
    ///
    /// Preconditions: `Established<LinearRingValid>` for the exterior ring and
    /// each hole.
    ///
    /// Validates: PolygonExteriorIsCCW, PolygonHolesAreCW,
    /// PolygonHolesInsideExterior, PolygonHolesDontContainEachOther,
    /// PolygonNoRingSelfIntersects, PolygonRingsDontCross, PolygonValid.
    ///
    /// Source: OGC 06-103r4 §6.1.11 — Polygon.
    fn build_polygon(
        &self,
        exterior: Established<LinearRingValid>,
        holes: Vec<Established<LinearRingValid>>,
    ) -> GisResult<(PolygonDescriptor, Established<PolygonValid>)>;

    /// Build a MultiPoint from validated component Points.
    ///
    /// Preconditions: `Established<PointValid>` for each component.
    ///
    /// Validates: MultiPointComponentsArePoints, MultiPointValid.
    ///
    /// Source: OGC 06-103r4 §6.1.8 — MultiPoint.
    fn build_multi_point(
        &self,
        points: Vec<Established<PointValid>>,
    ) -> GisResult<(MultiGeometryDescriptor, Established<MultiPointValid>)>;

    /// Build a MultiLineString from validated component LineStrings.
    ///
    /// Preconditions: `Established<LineStringValid>` for each component.
    ///
    /// Validates: MultiLineStringComponentsAreLineStrings, MultiLineStringValid.
    ///
    /// Source: OGC 06-103r4 §6.1.9 — MultiLineString.
    fn build_multi_line_string(
        &self,
        lines: Vec<Established<LineStringValid>>,
    ) -> GisResult<(MultiGeometryDescriptor, Established<MultiLineStringValid>)>;

    /// Build a MultiPolygon from validated component Polygons.
    ///
    /// Preconditions: `Established<PolygonValid>` for each component.
    ///
    /// Validates: MultiPolygonComponentsArePolygons, MultiPolygonInteriorsDisjoint,
    /// MultiPolygonBoundariesTouchAtPoints, MultiPolygonValid.
    ///
    /// Source: OGC 06-103r4 §6.1.13 — MultiPolygon.
    fn build_multi_polygon(
        &self,
        polygons: Vec<Established<PolygonValid>>,
    ) -> GisResult<(MultiGeometryDescriptor, Established<MultiPolygonValid>)>;

    /// Build a GeometryCollection from validated component geometries of any
    /// type.
    ///
    /// Preconditions: `Established<SfsGeometryValid>` for each component.
    ///
    /// Validates: GeometryCollectionAllComponentsValid, GeometryCollectionValid.
    ///
    /// Source: OGC 06-103r4 §6.1.14 — GeometryCollection.
    fn build_geometry_collection(
        &self,
        geoms: Vec<Established<SfsGeometryValid>>,
    ) -> GisResult<(
        GeometryCollectionDescriptor,
        Established<GeometryCollectionValid>,
    )>;

    // ── Parse factories ───────────────────────────────────────────────────

    /// Parse a geometry from its WKT representation and validate it.
    ///
    /// On success emits `Established<SfsGeometryValid>` certifying that the
    /// parsed geometry satisfies all §6.1 validity rules for its concrete type.
    ///
    /// Exercises: WktRoundTrip, WktKeywordValid, WktCoordinateListInParens,
    /// SfsGeometryValid.
    ///
    /// Source: OGC 06-103r4 §7.2 — Well-Known Text.
    fn geometry_from_wkt(
        &self,
        wkt: &str,
    ) -> GisResult<(Box<dyn SfsGeometryMeta>, Established<SfsGeometryValid>)>;

    /// Parse a geometry from its WKB representation and validate it.
    ///
    /// On success emits `Established<SfsGeometryValid>` certifying that the
    /// parsed geometry satisfies all §6.1 validity rules for its concrete type.
    ///
    /// Exercises: WkbRoundTrip, WkbByteOrderMarkerPresent, WkbCoordinateIsDouble,
    /// SfsGeometryValid.
    ///
    /// Source: OGC 06-103r4 §7.3 — Well-Known Binary.
    fn geometry_from_wkb(
        &self,
        wkb: &[u8],
    ) -> GisResult<(Box<dyn SfsGeometryMeta>, Established<SfsGeometryValid>)>;
}

// ── Orthogonal concern: Identity / Metadata (Role 2a) ─────────────────────────

/// Intrinsic metadata of any OGC SFS geometry.
///
/// This trait is an **orthogonal concern**: all methods report descriptive
/// facts about the geometry instance and work regardless of structural
/// correctness.  Even a malformed or empty geometry has a type name, an SRID,
/// and coordinate dimensionality.
///
/// Spatial predicates live in [`SfsTopology`][crate::SfsTopology];
/// constructive set operations live in [`SfsSetOps`][crate::SfsSetOps];
/// validity-by-construction lives in [`SfsGeometryFactory`].
///
/// The trait is `dyn`-compatible: all methods return concrete types or
/// `GisResult<T>` for concrete `T`.
///
/// Source: OGC 06-103r4 §4–§6.1.3 — Geometry class interface.
pub trait SfsGeometryMeta: Send + Sync {
    /// Returns the name of the concrete geometry type.
    ///
    /// E.g. `"Point"`, `"Polygon"`, `"MultiLineString"`.
    ///
    /// Exercises: GeometryTypeReturnsString, GeometryTypeMatchesConcreteName.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — geometryType().
    fn geometry_type(&self) -> &str;

    /// Returns the integer SRID of the associated spatial reference system.
    ///
    /// Exercises: GeometrySridReturnsInteger, SridNonNegative,
    /// GeometryHasSrs, GeometrySrsNotNull.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — SRID().
    fn srid(&self) -> i32;

    /// Returns the topological dimension.
    ///
    /// Returns -1 for empty geometries, 0 for Point types, 1 for curve types,
    /// 2 for surface types.
    ///
    /// Exercises: GeometryDimension0ForPoint, GeometryDimension1ForLine,
    /// GeometryDimension2ForSurface, GeometryDimensionMinus1WhenEmpty.
    ///
    /// Source: OGC 06-103r4 §4.2 / §6.1.1 — dimension().
    fn dimension(&self) -> i32;

    /// Returns the coordinate dimensionality.
    ///
    /// 2 for XY, 3 for XYZ or XYM, 4 for XYZM.
    ///
    /// Exercises: CoordDimensionalityIsConsistent, CoordDimUniformInGeometry,
    /// CoordDimensionalityUniform.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    fn coord_dimension(&self) -> u32;

    /// Returns `true` when the geometry contains no points.
    ///
    /// Exercises: IsEmptyTrueForEmpty, IsEmptyFalseForNonEmpty,
    /// EmptyHandlingConsistent.
    ///
    /// Source: OGC 06-103r4 §4.3 / §6.1.1 — isEmpty().
    fn is_empty(&self) -> bool;

    /// Returns `true` when the coordinate sequence carries a Z ordinate.
    ///
    /// Exercises: Coord3DZIsElevation, CoordDimensionalityUniform.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — Z coordinate presence.
    fn is_3d(&self) -> bool;

    /// Returns `true` when the coordinate sequence carries an M ordinate.
    ///
    /// Exercises: CoordMIsMeasure, Coord2DMPosition, Coord3DMPosition.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — M coordinate presence.
    fn is_measured(&self) -> bool;

    /// Returns `true` when the geometry has no anomalous points such as
    /// self-intersections or self-tangencies.
    ///
    /// Exercises: IsSimpleNoSelfIntersection, GeometryIsSimplePredicate,
    /// GeometryIsSimpleAndIsValidDistinct.
    ///
    /// Source: OGC 06-103r4 §4.3 / §6.1.1 — isSimple().
    fn is_simple(&self) -> GisResult<bool>;

    /// Returns `true` when the geometry satisfies all §6.1.3 validity rules.
    ///
    /// Exercises: IsValidWellFormed, GeometryIsValidPredicate,
    /// IsValidImpliesSubComponentsValid.
    ///
    /// Source: OGC 06-103r4 §4.3 / §6.1.3 — isValid().
    fn is_valid(&self) -> GisResult<bool>;

    /// Returns the minimum bounding rectangle as a boxed geometry.
    ///
    /// Returns a `POLYGON` for non-degenerate cases, a `POINT` for a
    /// single-point geometry, and an empty geometry when `self` is empty.
    ///
    /// Exercises: GeometryEnvelopeReturnsMbr, EnvelopeIsPolygon,
    /// EnvelopeIsPointWhenDegenerate, EnvelopeEmptyWhenGeometryEmpty,
    /// EmptyGeometryEnvelopeIsEmpty.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — envelope().
    fn envelope(&self) -> GisResult<Box<dyn SfsGeometryMeta>>;

    /// Returns the boundary of this geometry, or `None` for a
    /// `GeometryCollection` (boundary defined by the mod-2 rule, handled
    /// separately).
    ///
    /// Exercises: GeometryBoundaryDefinedPerType, LinearRingBoundaryIsEmpty,
    /// BoundaryOfBoundaryIsEmpty, PointBoundaryIsEmpty,
    /// LineStringBoundaryIsEndpoints.
    ///
    /// Source: OGC 06-103r4 §6.1.4 — boundary() per-type semantics.
    fn boundary(&self) -> GisResult<Option<Box<dyn SfsGeometryMeta>>>;
}

// ── Orthogonal concern: Serialization I/O (Role 2b) ───────────────────────────

/// WKT and WKB serialization for any OGC SFS geometry.
///
/// This trait is an **orthogonal concern**: it encodes the geometry as text
/// or binary independently of the geometry's structural validity or concrete
/// type.  Even a malformed geometry can be serialized to WKT or WKB.
///
/// WKT/WKB *parsing* belongs to [`SfsGeometryFactory`] because parsing is
/// construction — it validates the input and emits a proof token.
///
/// The trait is `dyn`-compatible.
///
/// Source: OGC 06-103r4 §7 — WKT and WKB representations.
pub trait SfsGeometryIo: Send + Sync {
    /// Returns the WKT representation of this geometry.
    ///
    /// Exercises: AsTextMethodDefined, AsTextReturnsWkt, WktRoundTrip,
    /// WktKeywordValid, WktKeywordCaseInsensitive, WktCoordinateListInParens,
    /// WktCoordsSeparatedBySpace, WktPositionsSeparatedByComma.
    ///
    /// Source: OGC 06-103r4 §7.2 — asText().
    fn as_wkt(&self) -> GisResult<String>;

    /// Returns the WKB representation in NDR (little-endian) byte order.
    ///
    /// Exercises: AsBinaryMethodDefined, AsBinaryReturnsWkb, WkbRoundTrip,
    /// WkbByteOrderMarkerPresent, WkbByteOrderLittleEndian,
    /// WkbCoordinateIsDouble, WkbTypeCodeValid.
    ///
    /// Source: OGC 06-103r4 §7.3 — asBinary().
    fn as_wkb(&self) -> GisResult<Vec<u8>>;
}
