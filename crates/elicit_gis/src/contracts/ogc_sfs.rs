//! OGC Simple Features Specification propositions.
//!
//! Source: OGC 06-103r4 / ISO 19125-1:2004 — Simple Feature Access Part 1.
//! All §references are to OGC 06-103r4 unless stated otherwise.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    // -- Section 4.1 SRS assignment --

    /// Every geometry instance has an assigned spatial reference system.
    ///
    /// Source: OGC 06-103r4 §4.1 — Geometry model overview.
    pub struct GeometryHasSrs;
    structural_prop!(GeometryHasSrs, "GeometryHasSrs");

    /// The SRS on a geometry instance is not null.
    ///
    /// Source: OGC 06-103r4 §4.1 — Geometry model overview.
    pub struct GeometrySrsNotNull;
    structural_prop!(GeometrySrsNotNull, "GeometrySrsNotNull");

    /// SRID() returns the integer identifier of the geometry's SRS.
    ///
    /// Source: OGC 06-103r4 §4.1 — Geometry model overview.
    pub struct GeometrySridReturnsInteger;
    structural_prop!(GeometrySridReturnsInteger, "GeometrySridReturnsInteger");

    // -- Section 4.2 Dimension --

    /// dimension() returns -1 for any empty geometry instance.
    ///
    /// Source: OGC 06-103r4 §4.2 — Geometry dimension.
    pub struct GeometryDimensionMinus1WhenEmpty;
    structural_prop!(
        GeometryDimensionMinus1WhenEmpty,
        "GeometryDimensionMinus1WhenEmpty"
    );

    /// dimension() returns 0 for point and multi-point types.
    ///
    /// Source: OGC 06-103r4 §4.2 — Geometry dimension.
    pub struct GeometryDimension0ForPoint;
    structural_prop!(GeometryDimension0ForPoint, "GeometryDimension0ForPoint");

    /// dimension() returns 1 for line and multi-line types.
    ///
    /// Source: OGC 06-103r4 §4.2 — Geometry dimension.
    pub struct GeometryDimension1ForLine;
    structural_prop!(GeometryDimension1ForLine, "GeometryDimension1ForLine");

    /// dimension() returns 2 for surface and multi-surface types.
    ///
    /// Source: OGC 06-103r4 §4.2 — Geometry dimension.
    pub struct GeometryDimension2ForSurface;
    structural_prop!(GeometryDimension2ForSurface, "GeometryDimension2ForSurface");

    // -- Section 4.3 Core predicates --

    /// isEmpty() is a total predicate: defined for every geometry instance.
    ///
    /// Source: OGC 06-103r4 §4.3 — Core predicates.
    pub struct GeometryIsEmptyPredicate;
    structural_prop!(GeometryIsEmptyPredicate, "GeometryIsEmptyPredicate");

    /// isSimple() is a total predicate: defined for every geometry instance.
    ///
    /// Source: OGC 06-103r4 §4.3 — Core predicates.
    pub struct GeometryIsSimplePredicate;
    structural_prop!(GeometryIsSimplePredicate, "GeometryIsSimplePredicate");

    /// isValid() is a total predicate: defined for every geometry instance.
    ///
    /// Source: OGC 06-103r4 §4.3 — Core predicates.
    pub struct GeometryIsValidPredicate;
    structural_prop!(GeometryIsValidPredicate, "GeometryIsValidPredicate");

    /// isSimple() and isValid() are distinct predicates with independent truth values.
    ///
    /// Source: OGC 06-103r4 §4.3 — Core predicates.
    pub struct GeometryIsSimpleAndIsValidDistinct;
    structural_prop!(
        GeometryIsSimpleAndIsValidDistinct,
        "GeometryIsSimpleAndIsValidDistinct"
    );

    // -- Section 4.4 Envelope and boundary --

    /// envelope() returns the minimum bounding rectangle (MBR) of the geometry.
    ///
    /// Source: OGC 06-103r4 §4.4 — Envelope.
    pub struct GeometryEnvelopeReturnsMbr;
    structural_prop!(GeometryEnvelopeReturnsMbr, "GeometryEnvelopeReturnsMbr");

    /// boundary() is defined per geometry subtype; its semantics vary by type.
    ///
    /// Source: OGC 06-103r4 §4.4 — Boundary definition.
    pub struct GeometryBoundaryDefinedPerType;
    structural_prop!(
        GeometryBoundaryDefinedPerType,
        "GeometryBoundaryDefinedPerType"
    );

    // -- Section 6.1.1 Geometry class interface --

    /// geometryType() returns a non-empty string.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — geometryType().
    pub struct GeometryTypeReturnsString;
    structural_prop!(GeometryTypeReturnsString, "GeometryTypeReturnsString");

    /// geometryType() returns the name of the concrete instantiated subtype.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — geometryType().
    pub struct GeometryTypeMatchesConcreteName;
    structural_prop!(
        GeometryTypeMatchesConcreteName,
        "GeometryTypeMatchesConcreteName"
    );

    /// SRID() returns a non-negative integer.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — SRID().
    pub struct SridNonNegative;
    structural_prop!(SridNonNegative, "SridNonNegative");

    /// SRID() returns a value that is an integer type.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — SRID().
    pub struct SridIsInteger;
    structural_prop!(SridIsInteger, "SridIsInteger");

    /// envelope() returns a POLYGON for non-degenerate, non-empty geometries.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — envelope().
    pub struct EnvelopeIsPolygon;
    structural_prop!(EnvelopeIsPolygon, "EnvelopeIsPolygon");

    /// envelope() returns a POINT for a geometry that degenerates to a single point.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — envelope() degenerate case.
    pub struct EnvelopeIsPointWhenDegenerate;
    structural_prop!(
        EnvelopeIsPointWhenDegenerate,
        "EnvelopeIsPointWhenDegenerate"
    );

    /// envelope() returns an empty geometry when applied to an empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — envelope() on empty.
    pub struct EnvelopeEmptyWhenGeometryEmpty;
    structural_prop!(
        EnvelopeEmptyWhenGeometryEmpty,
        "EnvelopeEmptyWhenGeometryEmpty"
    );

    /// asText() method is defined for all geometry subtypes.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — asText().
    pub struct AsTextMethodDefined;
    structural_prop!(AsTextMethodDefined, "AsTextMethodDefined");

    /// asBinary() method is defined for all geometry subtypes.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — asBinary().
    pub struct AsBinaryMethodDefined;
    structural_prop!(AsBinaryMethodDefined, "AsBinaryMethodDefined");

    /// asText() returns a valid WKT representation of the geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.1 / §7.2 — asText().
    pub struct AsTextReturnsWkt;
    structural_prop!(AsTextReturnsWkt, "AsTextReturnsWkt");

    /// asBinary() returns a valid WKB representation of the geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.1 / §7.3 — asBinary().
    pub struct AsBinaryReturnsWkb;
    structural_prop!(AsBinaryReturnsWkb, "AsBinaryReturnsWkb");

    /// isEmpty() returns true when the geometry set is the empty set.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — isEmpty().
    pub struct IsEmptyTrueForEmpty;
    structural_prop!(IsEmptyTrueForEmpty, "IsEmptyTrueForEmpty");

    /// isEmpty() returns false when the geometry contains at least one point.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — isEmpty().
    pub struct IsEmptyFalseForNonEmpty;
    structural_prop!(IsEmptyFalseForNonEmpty, "IsEmptyFalseForNonEmpty");

    /// isSimple() returns false when the geometry has self-intersections.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — isSimple().
    pub struct IsSimpleNoSelfIntersection;
    structural_prop!(IsSimpleNoSelfIntersection, "IsSimpleNoSelfIntersection");

    /// isValid() returns true only when the geometry satisfies all validity rules.
    ///
    /// Source: OGC 06-103r4 §6.1.1 / §6.1.3 — isValid().
    pub struct IsValidWellFormed;
    structural_prop!(IsValidWellFormed, "IsValidWellFormed");

    // -- Section 6.1.2 Coordinate dimensionality --

    /// A 2D coordinate position consists of exactly two ordinates: X and Y.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    pub struct Coord2DPosition;
    structural_prop!(Coord2DPosition, "Coord2DPosition");

    /// A 3D coordinate position (XYZ) consists of exactly three ordinates.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    pub struct Coord3DPosition;
    structural_prop!(Coord3DPosition, "Coord3DPosition");

    /// The Z ordinate in a 3D coordinate represents elevation or height.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    pub struct Coord3DZIsElevation;
    structural_prop!(Coord3DZIsElevation, "Coord3DZIsElevation");

    /// A 2D+M coordinate position (XYM) consists of X, Y, and a measure ordinate.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    pub struct Coord2DMPosition;
    structural_prop!(Coord2DMPosition, "Coord2DMPosition");

    /// The M ordinate in an XYM or XYZM geometry represents a user-defined measure.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    pub struct CoordMIsMeasure;
    structural_prop!(CoordMIsMeasure, "CoordMIsMeasure");

    /// A 4D coordinate position (XYZM) consists of exactly four ordinates.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate dimensionality.
    pub struct Coord3DMPosition;
    structural_prop!(Coord3DMPosition, "Coord3DMPosition");

    /// Coordinate dimensionality is uniform across all positions of a geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — dimensionality uniformity.
    pub struct CoordDimensionalityUniform;
    structural_prop!(CoordDimensionalityUniform, "CoordDimensionalityUniform");

    /// The X ordinate of every coordinate position is a finite value.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate constraints.
    pub struct CoordXIsFinite;
    structural_prop!(CoordXIsFinite, "CoordXIsFinite");

    /// The Y ordinate of every coordinate position is a finite value.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate constraints.
    pub struct CoordYIsFinite;
    structural_prop!(CoordYIsFinite, "CoordYIsFinite");

    /// The Z ordinate is finite when the geometry carries Z coordinates.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate constraints.
    pub struct CoordZIsFiniteWhenPresent;
    structural_prop!(CoordZIsFiniteWhenPresent, "CoordZIsFiniteWhenPresent");

    /// The M ordinate is finite when the geometry carries M coordinates.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — coordinate constraints.
    pub struct CoordMIsFiniteWhenPresent;
    structural_prop!(CoordMIsFiniteWhenPresent, "CoordMIsFiniteWhenPresent");

    /// The coordinate dimensionality is consistent with the actual ordinate count.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — dimensionality uniformity.
    pub struct CoordDimensionalityIsConsistent;
    structural_prop!(
        CoordDimensionalityIsConsistent,
        "CoordDimensionalityIsConsistent"
    );

    // -- Section 6.1.3 Point validity --

    /// A Point geometry is always valid unless it has NaN coordinates.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Point validity.
    pub struct PointAlwaysValid;
    structural_prop!(PointAlwaysValid, "PointAlwaysValid");

    /// An empty Point has no coordinate values.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Point validity.
    pub struct PointEmptyHasNoCoords;
    structural_prop!(PointEmptyHasNoCoords, "PointEmptyHasNoCoords");

    /// The X ordinate of a non-empty Point is a finite value.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Point validity.
    pub struct PointXIsFinite;
    structural_prop!(PointXIsFinite, "PointXIsFinite");

    /// The Y ordinate of a non-empty Point is a finite value.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Point validity.
    pub struct PointYIsFinite;
    structural_prop!(PointYIsFinite, "PointYIsFinite");

    /// The Z ordinate of a 3D Point is finite when present.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Point validity.
    pub struct PointZIsFiniteWhenPresent;
    structural_prop!(PointZIsFiniteWhenPresent, "PointZIsFiniteWhenPresent");

    /// An empty Point satisfies isEmpty().
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Point validity.
    pub struct PointEmptyIsEmpty;
    structural_prop!(PointEmptyIsEmpty, "PointEmptyIsEmpty");

    // -- Section 6.1.3 LineString validity --

    /// A valid LineString has at least two distinct point positions.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringHasTwoOrMorePoints;
    structural_prop!(LineStringHasTwoOrMorePoints, "LineStringHasTwoOrMorePoints");

    /// Adjacent coordinate positions in a LineString are distinct.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringAdjacentPointsDistinct;
    structural_prop!(
        LineStringAdjacentPointsDistinct,
        "LineStringAdjacentPointsDistinct"
    );

    /// A simple LineString has no self-intersections except possibly at its endpoints.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringSimpleNoSelfIntersection;
    structural_prop!(
        LineStringSimpleNoSelfIntersection,
        "LineStringSimpleNoSelfIntersection"
    );

    /// A closed, simple LineString is equivalent to a LinearRing.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringClosedEqualsLinearRing;
    structural_prop!(
        LineStringClosedEqualsLinearRing,
        "LineStringClosedEqualsLinearRing"
    );

    /// An open (non-closed) LineString has exactly two boundary points.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringOpenBoundaryTwoPoints;
    structural_prop!(
        LineStringOpenBoundaryTwoPoints,
        "LineStringOpenBoundaryTwoPoints"
    );

    /// A closed LineString (LinearRing) has an empty boundary.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringClosedBoundaryEmpty;
    structural_prop!(
        LineStringClosedBoundaryEmpty,
        "LineStringClosedBoundaryEmpty"
    );

    /// A LineString requires at least two coordinate positions.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringMinimumTwoPositions;
    structural_prop!(
        LineStringMinimumTwoPositions,
        "LineStringMinimumTwoPositions"
    );

    /// The boundary of a general LineString equals the set of its two endpoints.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LineString validity.
    pub struct LineStringBoundaryIsEndpoints;
    structural_prop!(
        LineStringBoundaryIsEndpoints,
        "LineStringBoundaryIsEndpoints"
    );

    // -- Section 6.1.3 LinearRing validity --

    /// A LinearRing is a closed LineString: its first position equals its last.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LinearRing validity.
    pub struct LinearRingIsClosedLineString;
    structural_prop!(LinearRingIsClosedLineString, "LinearRingIsClosedLineString");

    /// A LinearRing has at least 4 positions (including the repeated start/end).
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LinearRing validity.
    pub struct LinearRingMinimumFourPositions;
    structural_prop!(
        LinearRingMinimumFourPositions,
        "LinearRingMinimumFourPositions"
    );

    /// A LinearRing is simple: it does not self-intersect.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LinearRing validity.
    pub struct LinearRingIsSimple;
    structural_prop!(LinearRingIsSimple, "LinearRingIsSimple");

    /// A LinearRing is non-degenerate: it encloses a non-zero area.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LinearRing validity.
    pub struct LinearRingNonDegenerate;
    structural_prop!(LinearRingNonDegenerate, "LinearRingNonDegenerate");

    /// The first and last positions of a LinearRing are identical.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LinearRing validity.
    pub struct LinearRingFirstPositionEqualsLast;
    structural_prop!(
        LinearRingFirstPositionEqualsLast,
        "LinearRingFirstPositionEqualsLast"
    );

    /// The boundary of a LinearRing is the empty set (it is closed).
    ///
    /// Source: OGC 06-103r4 §6.1.3 — LinearRing validity.
    pub struct LinearRingBoundaryIsEmpty;
    structural_prop!(LinearRingBoundaryIsEmpty, "LinearRingBoundaryIsEmpty");

    // -- Section 6.1.3 Polygon validity --

    /// The exterior boundary of a Polygon is a LinearRing.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonExteriorIsLinearRing;
    structural_prop!(PolygonExteriorIsLinearRing, "PolygonExteriorIsLinearRing");

    /// All interior boundaries (holes) of a Polygon are LinearRings.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonInteriorRingsAreLinearRings;
    structural_prop!(
        PolygonInteriorRingsAreLinearRings,
        "PolygonInteriorRingsAreLinearRings"
    );

    /// The exterior ring of a valid Polygon is oriented counter-clockwise.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonExteriorIsCCW;
    structural_prop!(PolygonExteriorIsCCW, "PolygonExteriorIsCCW");

    /// Interior rings (holes) of a valid Polygon are oriented clockwise.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonHolesAreCW;
    structural_prop!(PolygonHolesAreCW, "PolygonHolesAreCW");

    /// No ring of a Polygon self-intersects.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonNoRingSelfIntersects;
    structural_prop!(PolygonNoRingSelfIntersects, "PolygonNoRingSelfIntersects");

    /// Rings of a Polygon do not cross each other.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonRingsDontCross;
    structural_prop!(PolygonRingsDontCross, "PolygonRingsDontCross");

    /// Interior rings of a Polygon lie inside the exterior ring.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonHolesInsideExterior;
    structural_prop!(PolygonHolesInsideExterior, "PolygonHolesInsideExterior");

    /// Interior rings of a Polygon do not contain each other.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonHolesDontContainEachOther;
    structural_prop!(
        PolygonHolesDontContainEachOther,
        "PolygonHolesDontContainEachOther"
    );

    /// Rings of a Polygon may touch at a finite set of points but not along a segment.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonRingsTouchAtPoints;
    structural_prop!(PolygonRingsTouchAtPoints, "PolygonRingsTouchAtPoints");

    /// Rings of a Polygon do not touch along a line segment.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonRingsDontTouchAlongSegment;
    structural_prop!(
        PolygonRingsDontTouchAlongSegment,
        "PolygonRingsDontTouchAlongSegment"
    );

    /// A Polygon has exactly one exterior ring.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonHasExactlyOneExteriorRing;
    structural_prop!(
        PolygonHasExactlyOneExteriorRing,
        "PolygonHasExactlyOneExteriorRing"
    );

    /// The number of interior rings (holes) of a Polygon is non-negative.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — Polygon validity.
    pub struct PolygonHoleCountNonNegative;
    structural_prop!(PolygonHoleCountNonNegative, "PolygonHoleCountNonNegative");

    // -- Section 6.1.3 MultiPoint validity --

    /// All components of a MultiPoint are Point instances.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPoint validity.
    pub struct MultiPointComponentsArePoints;
    structural_prop!(
        MultiPointComponentsArePoints,
        "MultiPointComponentsArePoints"
    );

    /// A MultiPoint may be empty (zero component Points).
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPoint validity.
    pub struct MultiPointMayBeEmpty;
    structural_prop!(MultiPointMayBeEmpty, "MultiPointMayBeEmpty");

    /// A MultiPoint is simple when no two component Points are at the same position.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPoint validity.
    pub struct MultiPointSimpleWhenNoTwoEqual;
    structural_prop!(
        MultiPointSimpleWhenNoTwoEqual,
        "MultiPointSimpleWhenNoTwoEqual"
    );

    // -- Section 6.1.3 MultiLineString validity --

    /// All components of a MultiLineString are LineString or LinearRing instances.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiLineString validity.
    pub struct MultiLineStringComponentsAreLineStrings;
    structural_prop!(
        MultiLineStringComponentsAreLineStrings,
        "MultiLineStringComponentsAreLineStrings"
    );

    /// A MultiLineString may be empty (zero component LineStrings).
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiLineString validity.
    pub struct MultiLineStringMayBeEmpty;
    structural_prop!(MultiLineStringMayBeEmpty, "MultiLineStringMayBeEmpty");

    /// A MultiLineString is simple when component curves intersect only at endpoints.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiLineString validity.
    pub struct MultiLineStringSimpleWhenIntersectAtEndpointsOnly;
    structural_prop!(
        MultiLineStringSimpleWhenIntersectAtEndpointsOnly,
        "MultiLineStringSimpleWhenIntersectAtEndpointsOnly"
    );

    // -- Section 6.1.3 MultiPolygon validity --

    /// All components of a MultiPolygon are Polygon instances.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity.
    pub struct MultiPolygonComponentsArePolygons;
    structural_prop!(
        MultiPolygonComponentsArePolygons,
        "MultiPolygonComponentsArePolygons"
    );

    /// Interior sets of component Polygons in a MultiPolygon are pairwise disjoint.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity.
    pub struct MultiPolygonInteriorsDisjoint;
    structural_prop!(
        MultiPolygonInteriorsDisjoint,
        "MultiPolygonInteriorsDisjoint"
    );

    /// Boundaries of component Polygons in a MultiPolygon touch at most at points.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity.
    pub struct MultiPolygonBoundariesTouchAtPoints;
    structural_prop!(
        MultiPolygonBoundariesTouchAtPoints,
        "MultiPolygonBoundariesTouchAtPoints"
    );

    /// Boundaries of component Polygons in a MultiPolygon do not overlap.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity.
    pub struct MultiPolygonBoundariesDontOverlap;
    structural_prop!(
        MultiPolygonBoundariesDontOverlap,
        "MultiPolygonBoundariesDontOverlap"
    );

    /// A MultiPolygon may be empty (zero component Polygons).
    ///
    /// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity.
    pub struct MultiPolygonMayBeEmpty;
    structural_prop!(MultiPolygonMayBeEmpty, "MultiPolygonMayBeEmpty");

    // -- Section 6.1.3 GeometryCollection validity --

    /// All component geometries of a GeometryCollection must themselves be valid.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — GeometryCollection validity.
    pub struct GeometryCollectionAllComponentsValid;
    structural_prop!(
        GeometryCollectionAllComponentsValid,
        "GeometryCollectionAllComponentsValid"
    );

    /// A GeometryCollection may contain components of mixed geometry types.
    ///
    /// Source: OGC 06-103r4 §6.1.3 — GeometryCollection validity.
    pub struct GeometryCollectionHeterogeneousAllowed;
    structural_prop!(
        GeometryCollectionHeterogeneousAllowed,
        "GeometryCollectionHeterogeneousAllowed"
    );

    /// A GeometryCollection may be empty (zero components).
    ///
    /// Source: OGC 06-103r4 §6.1.3 — GeometryCollection validity.
    pub struct GeometryCollectionMayBeEmpty;
    structural_prop!(GeometryCollectionMayBeEmpty, "GeometryCollectionMayBeEmpty");

    // -- Section 6.2 DE-9IM matrix definition --

    /// The DE-9IM defines a 3x3 matrix of topological intersections.
    ///
    /// Source: OGC 06-103r4 §6.2 — DE-9IM.
    pub struct De9ImMatrixDefined;
    structural_prop!(De9ImMatrixDefined, "De9ImMatrixDefined");

    /// Each DE-9IM cell value is one of: -1 (empty), 0, 1, or 2 (dimension).
    ///
    /// Source: OGC 06-103r4 §6.2 — DE-9IM cell values.
    pub struct De9ImCellValues;
    structural_prop!(De9ImCellValues, "De9ImCellValues");

    /// The DE-9IM matrix rows and columns are ordered Interior, Boundary, Exterior.
    ///
    /// Source: OGC 06-103r4 §6.2 — DE-9IM axes.
    pub struct De9ImInteriorBoundaryExteriorAxes;
    structural_prop!(
        De9ImInteriorBoundaryExteriorAxes,
        "De9ImInteriorBoundaryExteriorAxes"
    );

    /// A DE-9IM relate pattern string consists of exactly 9 characters.
    ///
    /// Source: OGC 06-103r4 §6.2 — relate pattern.
    pub struct De9ImRelatePatternIsNineChars;
    structural_prop!(
        De9ImRelatePatternIsNineChars,
        "De9ImRelatePatternIsNineChars"
    );

    // -- Section 6.2 Equals --

    /// Equals corresponds to DE-9IM pattern T*F**FFF*.
    ///
    /// Source: OGC 06-103r4 §6.2 — Equals.
    pub struct EqualsDe9ImPattern;
    structural_prop!(EqualsDe9ImPattern, "EqualsDe9ImPattern");

    /// Equals is symmetric: A.Equals(B) iff B.Equals(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Equals.
    pub struct EqualsIsSymmetric;
    structural_prop!(EqualsIsSymmetric, "EqualsIsSymmetric");

    /// Equals is reflexive: A.Equals(A) is true for every non-empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.2 — Equals.
    pub struct EqualsIsReflexive;
    structural_prop!(EqualsIsReflexive, "EqualsIsReflexive");

    // -- Section 6.2 Disjoint --

    /// Disjoint corresponds to DE-9IM pattern FF*FF****.
    ///
    /// Source: OGC 06-103r4 §6.2 — Disjoint.
    pub struct DisjointDe9ImPattern;
    structural_prop!(DisjointDe9ImPattern, "DisjointDe9ImPattern");

    /// Disjoint is symmetric: A.Disjoint(B) iff B.Disjoint(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Disjoint.
    pub struct DisjointIsSymmetric;
    structural_prop!(DisjointIsSymmetric, "DisjointIsSymmetric");

    /// Disjoint is the complement of Intersects.
    ///
    /// Source: OGC 06-103r4 §6.2 — Disjoint.
    pub struct DisjointIsInverseOfIntersects;
    structural_prop!(
        DisjointIsInverseOfIntersects,
        "DisjointIsInverseOfIntersects"
    );

    // -- Section 6.2 Intersects --

    /// Intersects is the complement of Disjoint.
    ///
    /// Source: OGC 06-103r4 §6.2 — Intersects.
    pub struct IntersectsIsNotDisjoint;
    structural_prop!(IntersectsIsNotDisjoint, "IntersectsIsNotDisjoint");

    /// Intersects is symmetric: A.Intersects(B) iff B.Intersects(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Intersects.
    pub struct IntersectsIsSymmetric;
    structural_prop!(IntersectsIsSymmetric, "IntersectsIsSymmetric");

    /// Intersects implies at least one shared point between the two geometries.
    ///
    /// Source: OGC 06-103r4 §6.2 — Intersects.
    pub struct IntersectsAtLeastOneSharedPoint;
    structural_prop!(
        IntersectsAtLeastOneSharedPoint,
        "IntersectsAtLeastOneSharedPoint"
    );

    // -- Section 6.2 Touches --

    /// Touches corresponds to DE-9IM pattern FT*******|F**T*****|F***T****.
    ///
    /// Source: OGC 06-103r4 §6.2 — Touches.
    pub struct TouchesDe9ImPattern;
    structural_prop!(TouchesDe9ImPattern, "TouchesDe9ImPattern");

    /// Touches is symmetric: A.Touches(B) iff B.Touches(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Touches.
    pub struct TouchesIsSymmetric;
    structural_prop!(TouchesIsSymmetric, "TouchesIsSymmetric");

    /// Touches implies that the interiors of the two geometries are disjoint.
    ///
    /// Source: OGC 06-103r4 §6.2 — Touches.
    pub struct TouchesInteriorsDisjoint;
    structural_prop!(TouchesInteriorsDisjoint, "TouchesInteriorsDisjoint");

    // -- Section 6.2 Crosses --

    /// Crosses corresponds to the applicable DE-9IM pattern.
    ///
    /// Source: OGC 06-103r4 §6.2 — Crosses.
    pub struct CrossesDe9ImPattern;
    structural_prop!(CrossesDe9ImPattern, "CrossesDe9ImPattern");

    /// For Crosses, the dimension of the intersection is less than max(dim(self), dim(g)).
    ///
    /// Source: OGC 06-103r4 §6.2 — Crosses.
    pub struct CrossesDimIntersectionLessThanMax;
    structural_prop!(
        CrossesDimIntersectionLessThanMax,
        "CrossesDimIntersectionLessThanMax"
    );

    /// Crosses is not symmetric in general.
    ///
    /// Source: OGC 06-103r4 §6.2 — Crosses.
    pub struct CrossesIsNotSymmetric;
    structural_prop!(CrossesIsNotSymmetric, "CrossesIsNotSymmetric");

    /// Crosses applies only when the two geometries have specific dimensional relationships.
    ///
    /// Source: OGC 06-103r4 §6.2 — Crosses.
    pub struct CrossesDimensionConstraint;
    structural_prop!(CrossesDimensionConstraint, "CrossesDimensionConstraint");

    // -- Section 6.2 Within --

    /// Within corresponds to DE-9IM pattern T*F**F***.
    ///
    /// Source: OGC 06-103r4 §6.2 — Within.
    pub struct WithinDe9ImPattern;
    structural_prop!(WithinDe9ImPattern, "WithinDe9ImPattern");

    /// Within is the converse of Contains: A.Within(B) iff B.Contains(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Within.
    pub struct WithinIsInverseOfContains;
    structural_prop!(WithinIsInverseOfContains, "WithinIsInverseOfContains");

    /// Within is not symmetric.
    ///
    /// Source: OGC 06-103r4 §6.2 — Within.
    pub struct WithinIsNotSymmetric;
    structural_prop!(WithinIsNotSymmetric, "WithinIsNotSymmetric");

    // -- Section 6.2 Contains --

    /// Contains corresponds to DE-9IM pattern T*****FF*.
    ///
    /// Source: OGC 06-103r4 §6.2 — Contains.
    pub struct ContainsDe9ImPattern;
    structural_prop!(ContainsDe9ImPattern, "ContainsDe9ImPattern");

    /// Contains is the converse of Within: A.Contains(B) iff B.Within(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Contains.
    pub struct ContainsIsInverseOfWithin;
    structural_prop!(ContainsIsInverseOfWithin, "ContainsIsInverseOfWithin");

    /// Contains is not symmetric.
    ///
    /// Source: OGC 06-103r4 §6.2 — Contains.
    pub struct ContainsIsNotSymmetric;
    structural_prop!(ContainsIsNotSymmetric, "ContainsIsNotSymmetric");

    // -- Section 6.2 Overlaps --

    /// Overlaps for 2D geometries corresponds to DE-9IM pattern T*T***T**.
    ///
    /// Source: OGC 06-103r4 §6.2 — Overlaps.
    pub struct OverlapsDe9ImPattern2D;
    structural_prop!(OverlapsDe9ImPattern2D, "OverlapsDe9ImPattern2D");

    /// Overlaps for lower-dimensional geometries corresponds to T*T***T**.
    ///
    /// Source: OGC 06-103r4 §6.2 — Overlaps.
    pub struct OverlapsDe9ImPatternLowDim;
    structural_prop!(OverlapsDe9ImPatternLowDim, "OverlapsDe9ImPatternLowDim");

    /// Overlaps is symmetric: A.Overlaps(B) iff B.Overlaps(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Overlaps.
    pub struct OverlapsIsSymmetric;
    structural_prop!(OverlapsIsSymmetric, "OverlapsIsSymmetric");

    /// Overlaps applies only to geometry pairs of equal dimension.
    ///
    /// Source: OGC 06-103r4 §6.2 — Overlaps.
    pub struct OverlapsDimensionConstraint;
    structural_prop!(OverlapsDimensionConstraint, "OverlapsDimensionConstraint");

    // -- Section 6.2 Covers / CoveredBy --

    /// Covers corresponds to DE-9IM pattern T*****FF*|*T****FF*|***T**FF*|****T*FF*.
    ///
    /// Source: OGC 06-103r4 §6.2 — Covers.
    pub struct CoversDe9ImPattern;
    structural_prop!(CoversDe9ImPattern, "CoversDe9ImPattern");

    /// CoveredBy is the converse of Covers: A.CoveredBy(B) iff B.Covers(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — CoveredBy.
    pub struct CoveredByIsInverseOfCovers;
    structural_prop!(CoveredByIsInverseOfCovers, "CoveredByIsInverseOfCovers");

    /// Covers implies Within: if A.Covers(B) then B.Within(A).
    ///
    /// Source: OGC 06-103r4 §6.2 — Covers.
    pub struct CoversImpliesWithin;
    structural_prop!(CoversImpliesWithin, "CoversImpliesWithin");

    // -- Section 6.2 Relate --

    /// A relate pattern string must be exactly 9 characters long.
    ///
    /// Source: OGC 06-103r4 §6.2 — relate(g, pattern).
    pub struct RelatePatternLength9;
    structural_prop!(RelatePatternLength9, "RelatePatternLength9");

    /// A relate pattern may use characters T, F, 0, 1, 2, or * only.
    ///
    /// Source: OGC 06-103r4 §6.2 — relate(g, pattern).
    pub struct RelatePatternCharacters;
    structural_prop!(RelatePatternCharacters, "RelatePatternCharacters");

    /// relate(g, pattern) returns true iff the actual DE-9IM values match the pattern.
    ///
    /// Source: OGC 06-103r4 §6.2 — relate(g, pattern).
    pub struct RelateReturnsBooleanMatchingPattern;
    structural_prop!(
        RelateReturnsBooleanMatchingPattern,
        "RelateReturnsBooleanMatchingPattern"
    );

    // -- Section 6.3 Area --

    /// area() returns 0.0 for point (0D) geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct Area0DReturnsZero;
    structural_prop!(Area0DReturnsZero, "Area0DReturnsZero");

    /// area() returns 0.0 for curve (1D) geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct Area1DReturnsZero;
    structural_prop!(Area1DReturnsZero, "Area1DReturnsZero");

    /// area() returns a positive value for non-degenerate surface (2D) geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct Area2DReturnsPositive;
    structural_prop!(Area2DReturnsPositive, "Area2DReturnsPositive");

    /// area() is always non-negative.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct AreaNonNegative;
    structural_prop!(AreaNonNegative, "AreaNonNegative");

    /// area() returns a finite (non-NaN, non-infinite) value for any non-empty geometry.
    ///
    /// IEEE 754 precondition required by AreaNonNegative and Area2DReturnsPositive in
    /// formal verification: ordering comparisons (`>= 0`, `> 0`) are only provable
    /// when the operand is finite.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct AreaIsFinite;
    structural_prop!(AreaIsFinite, "AreaIsFinite");

    /// area() is expressed in squared SRS units.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct AreaUnitsSquaredSrsUnits;
    structural_prop!(AreaUnitsSquaredSrsUnits, "AreaUnitsSquaredSrsUnits");

    /// area() returns 0.0 for an empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.3 — area().
    pub struct AreaEmptyGeometryIsZero;
    structural_prop!(AreaEmptyGeometryIsZero, "AreaEmptyGeometryIsZero");

    // -- Section 6.3 Length --

    /// length() returns 0.0 for point (0D) geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct Length0DReturnsZero;
    structural_prop!(Length0DReturnsZero, "Length0DReturnsZero");

    /// length() returns 0.0 for surface (2D) geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct Length2DReturnsZero;
    structural_prop!(Length2DReturnsZero, "Length2DReturnsZero");

    /// length() returns a positive value for non-degenerate curve (1D) geometries.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct Length1DReturnsPositive;
    structural_prop!(Length1DReturnsPositive, "Length1DReturnsPositive");

    /// length() is always non-negative.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct LengthNonNegative;
    structural_prop!(LengthNonNegative, "LengthNonNegative");

    /// length() returns a finite (non-NaN, non-infinite) value for any non-empty geometry.
    ///
    /// IEEE 754 precondition required by LengthNonNegative and Length1DReturnsPositive in
    /// formal verification: ordering comparisons are only provable when the operand is finite.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct LengthIsFinite;
    structural_prop!(LengthIsFinite, "LengthIsFinite");

    /// length() is expressed in SRS linear units.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct LengthInSrsUnits;
    structural_prop!(LengthInSrsUnits, "LengthInSrsUnits");

    /// length() returns 0.0 for an empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.3 — length().
    pub struct LengthEmptyGeometryIsZero;
    structural_prop!(LengthEmptyGeometryIsZero, "LengthEmptyGeometryIsZero");

    // -- Section 6.3 Distance --

    /// distance(g) is always non-negative.
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    pub struct DistanceNonNegative;
    structural_prop!(DistanceNonNegative, "DistanceNonNegative");

    /// distance(g) returns a finite (non-NaN, non-infinite) value for any geometry pair.
    ///
    /// IEEE 754 precondition required by DistanceNonNegative and DistanceTriangleInequality
    /// in formal verification: ordering comparisons and arithmetic are only provable when
    /// both operands are finite.
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    pub struct DistanceIsFinite;
    structural_prop!(DistanceIsFinite, "DistanceIsFinite");

    /// distance(g) == 0.0 iff the geometries intersect.
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    pub struct DistanceZeroIffIntersecting;
    structural_prop!(DistanceZeroIffIntersecting, "DistanceZeroIffIntersecting");

    /// distance(g) is symmetric: A.distance(B) == B.distance(A).
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    pub struct DistanceSymmetric;
    structural_prop!(DistanceSymmetric, "DistanceSymmetric");

    /// distance(g) is expressed in SRS linear units.
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    pub struct DistanceInSrsUnits;
    structural_prop!(DistanceInSrsUnits, "DistanceInSrsUnits");

    /// distance(g) satisfies the triangle inequality: d(A,C) <= d(A,B) + d(B,C).
    ///
    /// Source: OGC 06-103r4 §6.3 — distance(g).
    pub struct DistanceTriangleInequality;
    structural_prop!(DistanceTriangleInequality, "DistanceTriangleInequality");

    // -- Section 6.3 Centroid --

    /// centroid() always returns a Point instance.
    ///
    /// Source: OGC 06-103r4 §6.3 — centroid().
    pub struct CentroidReturnsPoint;
    structural_prop!(CentroidReturnsPoint, "CentroidReturnsPoint");

    /// centroid() is defined for all geometry instances (never throws).
    ///
    /// Source: OGC 06-103r4 §6.3 — centroid().
    pub struct CentroidDefinedForAll;
    structural_prop!(CentroidDefinedForAll, "CentroidDefinedForAll");

    /// The X coordinate of the centroid is finite.
    ///
    /// Source: OGC 06-103r4 §6.3 — centroid().
    pub struct CentroidXIsFinite;
    structural_prop!(CentroidXIsFinite, "CentroidXIsFinite");

    /// The Y coordinate of the centroid is finite.
    ///
    /// Source: OGC 06-103r4 §6.3 — centroid().
    pub struct CentroidYIsFinite;
    structural_prop!(CentroidYIsFinite, "CentroidYIsFinite");

    /// The centroid of a convex geometry lies within or on that geometry.
    ///
    /// Source: OGC 06-103r4 §6.3 — centroid().
    pub struct CentroidWithinConvexHull;
    structural_prop!(CentroidWithinConvexHull, "CentroidWithinConvexHull");

    // -- Section 6.3 PointOnSurface --

    /// pointOnSurface() always returns a Point instance.
    ///
    /// Source: OGC 06-103r4 §6.3 — pointOnSurface().
    pub struct PointOnSurfaceReturnsPoint;
    structural_prop!(PointOnSurfaceReturnsPoint, "PointOnSurfaceReturnsPoint");

    /// The point returned by pointOnSurface() lies on the geometry.
    ///
    /// Source: OGC 06-103r4 §6.3 — pointOnSurface().
    pub struct PointOnSurfaceIsOnGeometry;
    structural_prop!(PointOnSurfaceIsOnGeometry, "PointOnSurfaceIsOnGeometry");

    /// pointOnSurface() is defined for all geometry instances (never throws).
    ///
    /// Source: OGC 06-103r4 §6.3 — pointOnSurface().
    pub struct PointOnSurfaceDefinedForAll;
    structural_prop!(PointOnSurfaceDefinedForAll, "PointOnSurfaceDefinedForAll");

    // -- Section 7.2 WKT representation --

    /// The WKT keyword at the head of a geometry literal is a valid type name.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT production rules.
    pub struct WktKeywordValid;
    structural_prop!(WktKeywordValid, "WktKeywordValid");

    /// WKT type keywords are case-insensitive.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT production rules.
    pub struct WktKeywordCaseInsensitive;
    structural_prop!(WktKeywordCaseInsensitive, "WktKeywordCaseInsensitive");

    /// WKT dimension tags (Z, M, ZM) precede the opening parenthesis.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT dimension tags.
    pub struct WktDimensionTagPrecedesParens;
    structural_prop!(
        WktDimensionTagPrecedesParens,
        "WktDimensionTagPrecedesParens"
    );

    /// WKT coordinate ordinates within a position are separated by a single space.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT coordinate syntax.
    pub struct WktCoordsSeparatedBySpace;
    structural_prop!(WktCoordsSeparatedBySpace, "WktCoordsSeparatedBySpace");

    /// WKT coordinate positions within a sequence are separated by commas.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT coordinate syntax.
    pub struct WktPositionsSeparatedByComma;
    structural_prop!(WktPositionsSeparatedByComma, "WktPositionsSeparatedByComma");

    /// WKT coordinate lists are enclosed in parentheses.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT production rules.
    pub struct WktCoordinateListInParens;
    structural_prop!(WktCoordinateListInParens, "WktCoordinateListInParens");

    /// A WKT Point literal contains exactly the coordinate ordinates for the point.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT Point production.
    pub struct WktPointProduction;
    structural_prop!(WktPointProduction, "WktPointProduction");

    /// A WKT LineString literal contains a comma-separated list of positions.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT LineString production.
    pub struct WktLineStringProduction;
    structural_prop!(WktLineStringProduction, "WktLineStringProduction");

    /// A WKT Polygon literal contains one or more parenthesized ring sequences.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT Polygon production.
    pub struct WktPolygonProduction;
    structural_prop!(WktPolygonProduction, "WktPolygonProduction");

    /// WKT Polygon rings are each individually enclosed in parentheses.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT Polygon production.
    pub struct WktRingsParenthesized;
    structural_prop!(WktRingsParenthesized, "WktRingsParenthesized");

    /// WKT Polygon rings are separated by commas.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT Polygon production.
    pub struct WktRingsSeparatedByComma;
    structural_prop!(WktRingsSeparatedByComma, "WktRingsSeparatedByComma");

    /// The exterior ring of a WKT Polygon is the first ring in the list.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT Polygon production.
    pub struct WktPolygonExteriorFirstRing;
    structural_prop!(WktPolygonExteriorFirstRing, "WktPolygonExteriorFirstRing");

    /// A WKT MultiPoint literal contains a list of Point productions.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT MultiPoint production.
    pub struct WktMultiPointProduction;
    structural_prop!(WktMultiPointProduction, "WktMultiPointProduction");

    /// A WKT MultiLineString literal contains a list of LineString productions.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT MultiLineString production.
    pub struct WktMultiLineStringProduction;
    structural_prop!(WktMultiLineStringProduction, "WktMultiLineStringProduction");

    /// A WKT MultiPolygon literal contains a list of Polygon productions.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT MultiPolygon production.
    pub struct WktMultiPolygonProduction;
    structural_prop!(WktMultiPolygonProduction, "WktMultiPolygonProduction");

    /// A WKT GeometryCollection literal contains a list of any geometry productions.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT GeometryCollection production.
    pub struct WktGeometryCollectionProduction;
    structural_prop!(
        WktGeometryCollectionProduction,
        "WktGeometryCollectionProduction"
    );

    /// WKT multi-type literals nest component productions inside the outer parentheses.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT nesting.
    pub struct WktNestingForMultiTypes;
    structural_prop!(WktNestingForMultiTypes, "WktNestingForMultiTypes");

    /// An empty WKT geometry is represented with the EMPTY keyword.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT empty geometry.
    pub struct WktEmptyGeometry;
    structural_prop!(WktEmptyGeometry, "WktEmptyGeometry");

    /// The EMPTY keyword may appear in WKT for all seven geometry types.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT empty geometry.
    pub struct WktEmptyApplicableToAllTypes;
    structural_prop!(WktEmptyApplicableToAllTypes, "WktEmptyApplicableToAllTypes");

    /// A WKT 3D (XYZ) geometry uses the Z tag between the type keyword and the opening paren.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT Z variant.
    pub struct WktZVariant;
    structural_prop!(WktZVariant, "WktZVariant");

    /// A WKT measured (XYM) geometry uses the M tag between the type keyword and the opening paren.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT M variant.
    pub struct WktMVariant;
    structural_prop!(WktMVariant, "WktMVariant");

    /// A WKT 4D (XYZM) geometry uses the ZM tag between the type keyword and the opening paren.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT ZM variant.
    pub struct WktZMVariant;
    structural_prop!(WktZMVariant, "WktZMVariant");

    /// WKT serialization/deserialization preserves geometry type and coordinate values.
    ///
    /// Source: OGC 06-103r4 §7.2 — WKT round-trip.
    pub struct WktRoundTrip;
    structural_prop!(WktRoundTrip, "WktRoundTrip");

    // -- Section 7.3 WKB representation --

    /// Each WKB blob begins with a byte-order marker byte.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB byte-order marker.
    pub struct WkbByteOrderMarkerPresent;
    structural_prop!(WkbByteOrderMarkerPresent, "WkbByteOrderMarkerPresent");

    /// WKB byte-order 0x01 (NDR) indicates little-endian encoding.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB byte-order.
    pub struct WkbByteOrderLittleEndian;
    structural_prop!(WkbByteOrderLittleEndian, "WkbByteOrderLittleEndian");

    /// WKB byte-order 0x00 (XDR) indicates big-endian encoding.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB byte-order.
    pub struct WkbByteOrderBigEndian;
    structural_prop!(WkbByteOrderBigEndian, "WkbByteOrderBigEndian");

    /// The WKB byte-order marker takes exactly two distinct values: 0x00 and 0x01.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB byte-order.
    pub struct WkbByteOrderMarkerTwoValues;
    structural_prop!(WkbByteOrderMarkerTwoValues, "WkbByteOrderMarkerTwoValues");

    /// The WKB geometry type field is a 4-byte unsigned integer.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB geometry type field.
    pub struct WkbTypeCodeValid;
    structural_prop!(WkbTypeCodeValid, "WkbTypeCodeValid");

    /// WKB type code 1 encodes a Point geometry.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodePoint;
    structural_prop!(WkbTypeCodePoint, "WkbTypeCodePoint");

    /// WKB type code 2 encodes a LineString geometry.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodeLineString;
    structural_prop!(WkbTypeCodeLineString, "WkbTypeCodeLineString");

    /// WKB type code 3 encodes a Polygon geometry.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodePolygon;
    structural_prop!(WkbTypeCodePolygon, "WkbTypeCodePolygon");

    /// WKB type code 4 encodes a MultiPoint geometry.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodeMultiPoint;
    structural_prop!(WkbTypeCodeMultiPoint, "WkbTypeCodeMultiPoint");

    /// WKB type code 5 encodes a MultiLineString geometry.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodeMultiLineString;
    structural_prop!(WkbTypeCodeMultiLineString, "WkbTypeCodeMultiLineString");

    /// WKB type code 6 encodes a MultiPolygon geometry.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodeMultiPolygon;
    structural_prop!(WkbTypeCodeMultiPolygon, "WkbTypeCodeMultiPolygon");

    /// WKB type code 7 encodes a GeometryCollection.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB type codes.
    pub struct WkbTypeCodeGeometryCollection;
    structural_prop!(
        WkbTypeCodeGeometryCollection,
        "WkbTypeCodeGeometryCollection"
    );

    /// WKB encodes Z coordinates by adding 1000 to the base type code (ISO variant).
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB Z variant.
    pub struct WkbZVariant;
    structural_prop!(WkbZVariant, "WkbZVariant");

    /// WKB encodes M coordinates by adding 2000 to the base type code (ISO variant).
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB M variant.
    pub struct WkbMVariant;
    structural_prop!(WkbMVariant, "WkbMVariant");

    /// WKB encodes ZM coordinates by adding 3000 to the base type code (ISO variant).
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB ZM variant.
    pub struct WkbZMVariant;
    structural_prop!(WkbZMVariant, "WkbZMVariant");

    /// EWKB (extended WKB) may include an SRID field after the type code.
    ///
    /// Source: OGC 06-103r4 §7.3 / ISO 13249-3 — SRID extension.
    pub struct EwkbSridPresent;
    structural_prop!(EwkbSridPresent, "EwkbSridPresent");

    /// The EWKB SRID field is a 4-byte unsigned integer.
    ///
    /// Source: OGC 06-103r4 §7.3 / ISO 13249-3 — SRID field.
    pub struct EwkbSridIsUint32;
    structural_prop!(EwkbSridIsUint32, "EwkbSridIsUint32");

    /// The EWKB base geometry type code is obtained by masking the SRID flag bits.
    ///
    /// Source: OGC 06-103r4 §7.3 / ISO 13249-3 — SRID masking.
    pub struct EwkbBaseTypeCodeMasked;
    structural_prop!(EwkbBaseTypeCodeMasked, "EwkbBaseTypeCodeMasked");

    /// Each WKB coordinate ordinate is an IEEE 754 double-precision value (8 bytes).
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB coordinate encoding.
    pub struct WkbCoordinateIsDouble;
    structural_prop!(WkbCoordinateIsDouble, "WkbCoordinateIsDouble");

    /// Coordinate bytes in WKB are ordered according to the declared byte-order marker.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB coordinate encoding.
    pub struct WkbCoordinatesByteOrderMatches;
    structural_prop!(
        WkbCoordinatesByteOrderMatches,
        "WkbCoordinatesByteOrderMatches"
    );

    /// A 2D WKB Point blob is exactly 21 bytes: 1 (byte order) + 4 (type) + 16 (XY).
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB Point encoding.
    pub struct WkbPointLength21Bytes2D;
    structural_prop!(WkbPointLength21Bytes2D, "WkbPointLength21Bytes2D");

    /// Geometry count fields in multi-type WKB are 4-byte unsigned integers.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB multi-type encoding.
    pub struct WkbCountFieldsAreUint32;
    structural_prop!(WkbCountFieldsAreUint32, "WkbCountFieldsAreUint32");

    /// Each ring in a WKB Polygon is preceded by a 4-byte point count field.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB Polygon ring encoding.
    pub struct WkbRingHasCount;
    structural_prop!(WkbRingHasCount, "WkbRingHasCount");

    /// Sub-geometries in WKB collections are complete self-describing WKB blobs.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB collection encoding.
    pub struct WkbSubGeometriesAreCompleteBlobs;
    structural_prop!(
        WkbSubGeometriesAreCompleteBlobs,
        "WkbSubGeometriesAreCompleteBlobs"
    );

    /// The total byte length of a WKB blob is derivable from the type and coordinate count.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB length.
    pub struct WkbLengthValid;
    structural_prop!(WkbLengthValid, "WkbLengthValid");

    /// WKB serialization/deserialization preserves geometry type and coordinate values.
    ///
    /// Source: OGC 06-103r4 §7.3 — WKB round-trip.
    pub struct WkbRoundTrip;
    structural_prop!(WkbRoundTrip, "WkbRoundTrip");

    // -- Cross-cutting constraints --

    /// All component geometries in a collection share the same SRID.
    ///
    /// Source: OGC 06-103r4 §4.1 and §6.1.3 — SRS consistency in collections.
    pub struct SrsConsistentInCollection;
    structural_prop!(SrsConsistentInCollection, "SrsConsistentInCollection");

    /// The SRID of a geometry is inherited from construction and does not change.
    ///
    /// Source: OGC 06-103r4 §4.1 — SRID assignment.
    pub struct SridAssignedAtConstruction;
    structural_prop!(SridAssignedAtConstruction, "SridAssignedAtConstruction");

    /// Coordinate dimensionality is uniform across all positions in a geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — dimensionality uniformity.
    pub struct CoordDimUniformInGeometry;
    structural_prop!(CoordDimUniformInGeometry, "CoordDimUniformInGeometry");

    /// A GeometryCollection coordinate dimensionality matches all its components.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — dimensionality uniformity.
    pub struct CollectionCoordDimMatchesComponents;
    structural_prop!(
        CollectionCoordDimMatchesComponents,
        "CollectionCoordDimMatchesComponents"
    );

    /// Operations on empty geometries return defined values rather than errors.
    ///
    /// Source: OGC 06-103r4 §6.1 — empty geometry semantics.
    pub struct EmptyHandlingConsistent;
    structural_prop!(EmptyHandlingConsistent, "EmptyHandlingConsistent");

    /// The boundary of an empty geometry is the empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.1 — empty geometry boundary.
    pub struct EmptyGeometryBoundaryIsEmpty;
    structural_prop!(EmptyGeometryBoundaryIsEmpty, "EmptyGeometryBoundaryIsEmpty");

    /// The envelope of an empty geometry is the empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — envelope() on empty geometry.
    pub struct EmptyGeometryEnvelopeIsEmpty;
    structural_prop!(EmptyGeometryEnvelopeIsEmpty, "EmptyGeometryEnvelopeIsEmpty");

    /// isValid() true for a composite implies all sub-components also satisfy isValid().
    ///
    /// Source: OGC 06-103r4 §6.1.3 — validity hierarchy.
    pub struct IsValidImpliesSubComponentsValid;
    structural_prop!(
        IsValidImpliesSubComponentsValid,
        "IsValidImpliesSubComponentsValid"
    );

    /// The dimension of a geometry collection equals the maximum dimension of its components.
    ///
    /// Source: OGC 06-103r4 §4.2 — dimension of collections.
    pub struct DimensionConsistencyInHierarchy;
    structural_prop!(
        DimensionConsistencyInHierarchy,
        "DimensionConsistencyInHierarchy"
    );

    // -- Section 6.1.2 Constructive / Set operations [GAP FILL] --

    /// buffer(distance) returns a geometry containing all points within distance of self.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
    pub struct BufferReturnsContainingGeometry;
    structural_prop!(
        BufferReturnsContainingGeometry,
        "BufferReturnsContainingGeometry"
    );

    /// buffer(0) produces a result that contains self.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
    pub struct BufferZeroContainsSelf;
    structural_prop!(BufferZeroContainsSelf, "BufferZeroContainsSelf");

    /// buffer(d) with d > 0 produces a result with area >= area of self.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
    pub struct BufferPositiveDistanceIncreasesArea;
    structural_prop!(
        BufferPositiveDistanceIncreasesArea,
        "BufferPositiveDistanceIncreasesArea"
    );

    /// buffer(d) with d < 0 contracts the geometry inward.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
    pub struct BufferNegativeDistanceShrinks;
    structural_prop!(
        BufferNegativeDistanceShrinks,
        "BufferNegativeDistanceShrinks"
    );

    /// The result of buffer() is always a valid geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
    pub struct BufferResultIsValid;
    structural_prop!(BufferResultIsValid, "BufferResultIsValid");

    /// convexHull() returns the smallest convex geometry containing all points of self.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — convexHull().
    pub struct ConvexHullReturnsSmallestConvex;
    structural_prop!(
        ConvexHullReturnsSmallestConvex,
        "ConvexHullReturnsSmallestConvex"
    );

    /// The result of convexHull() contains all points of self.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — convexHull().
    pub struct ConvexHullContainsSelf;
    structural_prop!(ConvexHullContainsSelf, "ConvexHullContainsSelf");

    /// convexHull is idempotent: convexHull(convexHull(g)) == convexHull(g).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — convexHull().
    pub struct ConvexHullIdempotent;
    structural_prop!(ConvexHullIdempotent, "ConvexHullIdempotent");

    /// The convexHull of a convex polygon is that polygon itself.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — convexHull().
    pub struct ConvexHullOfConvexPolygonIsSelf;
    structural_prop!(
        ConvexHullOfConvexPolygonIsSelf,
        "ConvexHullOfConvexPolygonIsSelf"
    );

    /// intersection(g) returns a geometry containing only points in both self and g.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — intersection(g).
    pub struct IntersectionSubsetOfBothInputs;
    structural_prop!(
        IntersectionSubsetOfBothInputs,
        "IntersectionSubsetOfBothInputs"
    );

    /// intersection is commutative: A.intersection(B) == B.intersection(A).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — intersection(g).
    pub struct IntersectionIsCommutative;
    structural_prop!(IntersectionIsCommutative, "IntersectionIsCommutative");

    /// The intersection of two disjoint geometries is empty.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — intersection(g).
    pub struct IntersectionOfDisjointIsEmpty;
    structural_prop!(
        IntersectionOfDisjointIsEmpty,
        "IntersectionOfDisjointIsEmpty"
    );

    /// intersection(g) has dimension <= min(dim(self), dim(g)).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — intersection(g).
    pub struct IntersectionDimensionAtMostMin;
    structural_prop!(
        IntersectionDimensionAtMostMin,
        "IntersectionDimensionAtMostMin"
    );

    /// union(g) returns a geometry containing all points of self or g.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — union(g).
    pub struct UnionSupersetOfBothInputs;
    structural_prop!(UnionSupersetOfBothInputs, "UnionSupersetOfBothInputs");

    /// union is commutative: A.union(B) == B.union(A).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — union(g).
    pub struct UnionIsCommutative;
    structural_prop!(UnionIsCommutative, "UnionIsCommutative");

    /// union is associative: A.union(B.union(C)) == (A.union(B)).union(C).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — union(g).
    pub struct UnionIsAssociative;
    structural_prop!(UnionIsAssociative, "UnionIsAssociative");

    /// union has dimension == max(dim(self), dim(g)) when both inputs are non-empty.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — union(g).
    pub struct UnionDimensionIsMax;
    structural_prop!(UnionDimensionIsMax, "UnionDimensionIsMax");

    /// difference(g) is asymmetric: A.difference(B) != B.difference(A) in general.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — difference(g).
    pub struct DifferenceAsymmetric;
    structural_prop!(DifferenceAsymmetric, "DifferenceAsymmetric");

    /// A.difference(B) and B are disjoint.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — difference(g).
    pub struct DifferenceDisjointFromSubtracted;
    structural_prop!(
        DifferenceDisjointFromSubtracted,
        "DifferenceDisjointFromSubtracted"
    );

    /// A.difference(B) is a subset of A.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — difference(g).
    pub struct DifferenceSubsetOfSelf;
    structural_prop!(DifferenceSubsetOfSelf, "DifferenceSubsetOfSelf");

    /// difference(g) has dimension <= dim(self).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — difference(g).
    pub struct DifferenceDimensionAtMostSelf;
    structural_prop!(
        DifferenceDimensionAtMostSelf,
        "DifferenceDimensionAtMostSelf"
    );

    /// symDifference is commutative: A.symDifference(B) == B.symDifference(A).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
    pub struct SymDifferenceIsCommutative;
    structural_prop!(SymDifferenceIsCommutative, "SymDifferenceIsCommutative");

    /// A.symDifference(B) == A.union(B).difference(A.intersection(B)).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
    pub struct SymDifferenceEqualsUnionMinusIntersection;
    structural_prop!(
        SymDifferenceEqualsUnionMinusIntersection,
        "SymDifferenceEqualsUnionMinusIntersection"
    );

    /// A.union(B) == A.intersection(B).union(A.symDifference(B)).
    ///
    /// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
    pub struct UnionEqualsIntersectionPlusSymDifference;
    structural_prop!(
        UnionEqualsIntersectionPlusSymDifference,
        "UnionEqualsIntersectionPlusSymDifference"
    );

    // -- Section 6.1.4 boundary() Per-type semantics [GAP FILL] --

    /// The boundary of a Point is the empty set (GeometryCollection EMPTY).
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for Point.
    pub struct PointBoundaryIsEmpty;
    structural_prop!(PointBoundaryIsEmpty, "PointBoundaryIsEmpty");

    /// The boundary of a non-closed LineString is the MultiPoint of its two endpoints.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for LineString.
    pub struct LineStringNonClosedBoundaryIsEndpointMultiPoint;
    structural_prop!(
        LineStringNonClosedBoundaryIsEndpointMultiPoint,
        "LineStringNonClosedBoundaryIsEndpointMultiPoint"
    );

    /// The boundary of a closed LinearRing is the empty set.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for LinearRing.
    pub struct LinearRingBoundaryIsEmptySet;
    structural_prop!(LinearRingBoundaryIsEmptySet, "LinearRingBoundaryIsEmptySet");

    /// The boundary of a Polygon is the MultiLineString of all its rings.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for Polygon.
    pub struct PolygonBoundaryIsAllRings;
    structural_prop!(PolygonBoundaryIsAllRings, "PolygonBoundaryIsAllRings");

    /// The boundary of a MultiPolygon is the union of all component Polygon boundaries.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for MultiPolygon.
    pub struct MultiPolygonBoundaryIsAllRings;
    structural_prop!(
        MultiPolygonBoundaryIsAllRings,
        "MultiPolygonBoundaryIsAllRings"
    );

    /// The boundary of a GeometryCollection follows the mod-2 rule for curve components.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for GeometryCollection.
    pub struct GeometryCollectionBoundaryMod2Rule;
    structural_prop!(
        GeometryCollectionBoundaryMod2Rule,
        "GeometryCollectionBoundaryMod2Rule"
    );

    /// The boundary of a boundary is always empty: boundary(boundary(g)) = empty.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — topological axiom.
    pub struct BoundaryOfBoundaryIsEmpty;
    structural_prop!(BoundaryOfBoundaryIsEmpty, "BoundaryOfBoundaryIsEmpty");

    /// The boundary of any empty geometry is itself an empty geometry.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — boundary() for empty inputs.
    pub struct EmptyGeometryBoundaryIsEmptyGeometry;
    structural_prop!(
        EmptyGeometryBoundaryIsEmptyGeometry,
        "EmptyGeometryBoundaryIsEmptyGeometry"
    );

    // -- Section 6.1.5 Per-type accessor invariants [GAP FILL] --

    /// x() returns the x ordinate of a Point.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — Point.x().
    pub struct PointXReturnsXOrdinate;
    structural_prop!(PointXReturnsXOrdinate, "PointXReturnsXOrdinate");

    /// y() returns the y ordinate of a Point.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — Point.y().
    pub struct PointYReturnsYOrdinate;
    structural_prop!(PointYReturnsYOrdinate, "PointYReturnsYOrdinate");

    /// z() is defined and returns the z ordinate when coord dimension >= 3.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — Point.z().
    pub struct PointZDefinedWhen3D;
    structural_prop!(PointZDefinedWhen3D, "PointZDefinedWhen3D");

    /// m() is defined and returns the m ordinate when the geometry has M.
    ///
    /// Source: OGC 06-103r4 §6.1.2 — Point.m().
    pub struct PointMDefinedWhenMPresent;
    structural_prop!(PointMDefinedWhenMPresent, "PointMDefinedWhenMPresent");

    /// startPoint() == pointN(0) for a LineString.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — LineString.startPoint().
    pub struct LineStringStartPointIsPointNZero;
    structural_prop!(
        LineStringStartPointIsPointNZero,
        "LineStringStartPointIsPointNZero"
    );

    /// endPoint() == pointN(numPoints() - 1) for a LineString.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — LineString.endPoint().
    pub struct LineStringEndPointIsPointNLast;
    structural_prop!(
        LineStringEndPointIsPointNLast,
        "LineStringEndPointIsPointNLast"
    );

    /// pointN(i) is defined for i in 0..numPoints()-1.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — LineString.pointN(n).
    pub struct LineStringPointNInRange;
    structural_prop!(LineStringPointNInRange, "LineStringPointNInRange");

    /// numPoints() equals the number of coordinate positions in the LineString.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — LineString.numPoints().
    pub struct LineStringNumPointsMatchesCoordCount;
    structural_prop!(
        LineStringNumPointsMatchesCoordCount,
        "LineStringNumPointsMatchesCoordCount"
    );

    /// isClosed() is true iff startPoint() coordinates equal endPoint() coordinates.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — LineString.isClosed().
    pub struct LineStringIsClosedStartEqualsEnd;
    structural_prop!(
        LineStringIsClosedStartEqualsEnd,
        "LineStringIsClosedStartEqualsEnd"
    );

    /// isRing() is true iff isClosed() AND isSimple().
    ///
    /// Source: OGC 06-103r4 §6.1.1 — LineString.isRing().
    pub struct LineStringIsRingImpliesClosedAndSimple;
    structural_prop!(
        LineStringIsRingImpliesClosedAndSimple,
        "LineStringIsRingImpliesClosedAndSimple"
    );

    /// exteriorRing() is never null for a non-empty Polygon.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — Polygon.exteriorRing().
    pub struct PolygonExteriorRingNeverNull;
    structural_prop!(PolygonExteriorRingNeverNull, "PolygonExteriorRingNeverNull");

    /// interiorRingN(n) returns the nth hole (zero-indexed).
    ///
    /// Source: OGC 06-103r4 §6.1.1 — Polygon.interiorRingN(n).
    pub struct PolygonInteriorRingNReturnsHole;
    structural_prop!(
        PolygonInteriorRingNReturnsHole,
        "PolygonInteriorRingNReturnsHole"
    );

    /// numInteriorRings() >= 0.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — Polygon.numInteriorRings().
    pub struct PolygonNumInteriorRingsNonNegative;
    structural_prop!(
        PolygonNumInteriorRingsNonNegative,
        "PolygonNumInteriorRingsNonNegative"
    );

    /// interiorRingN(i) is defined for i in 0..numInteriorRings()-1.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — Polygon.interiorRingN(n).
    pub struct PolygonInteriorRingNInRange;
    structural_prop!(PolygonInteriorRingNInRange, "PolygonInteriorRingNInRange");

    /// numGeometries() returns the number of component geometries.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.numGeometries().
    pub struct CollectionNumGeometriesCount;
    structural_prop!(CollectionNumGeometriesCount, "CollectionNumGeometriesCount");

    /// geometryN(i) is defined for i in 0..numGeometries()-1.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.geometryN(n).
    pub struct CollectionGeometryNIndexed;
    structural_prop!(CollectionGeometryNIndexed, "CollectionGeometryNIndexed");

    /// numGeometries() >= 0.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.numGeometries().
    pub struct CollectionNumGeometriesNonNegative;
    structural_prop!(
        CollectionNumGeometriesNonNegative,
        "CollectionNumGeometriesNonNegative"
    );

    /// geometryN(i) is never null for any valid index.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.geometryN(n).
    pub struct CollectionGeometryNNeverNull;
    structural_prop!(CollectionGeometryNNeverNull, "CollectionGeometryNNeverNull");

    /// A MultiLineString isClosed() iff all component LineStrings are closed.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — MultiLineString.isClosed().
    pub struct MultiLineStringIsClosedAllClosed2;
    structural_prop!(
        MultiLineStringIsClosedAllClosed2,
        "MultiLineStringIsClosedAllClosed2"
    );

    /// A GeometryCollection isSimple() iff all components are simple.
    ///
    /// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.isSimple().
    pub struct GeometryCollectionIsSimpleAllSimple;
    structural_prop!(
        GeometryCollectionIsSimpleAllSimple,
        "GeometryCollectionIsSimpleAllSimple"
    );

    // -- Section 1.4 Conformance classes [GAP FILL] --

    /// OGC SFS Conformance Class 0 requires all seven geometry types, SRID, WKB/WKT.
    ///
    /// Source: OGC 06-103r4 §1.4 — Conformance Class 0.
    pub struct ConformanceClass0RequiresSevenTypes;
    structural_prop!(
        ConformanceClass0RequiresSevenTypes,
        "ConformanceClass0RequiresSevenTypes"
    );

    /// OGC SFS CC0 mandates WKB and WKT serialization for all seven types.
    ///
    /// Source: OGC 06-103r4 §1.4 — Conformance Class 0.
    pub struct ConformanceClass0RequiresWkbWkt;
    structural_prop!(
        ConformanceClass0RequiresWkbWkt,
        "ConformanceClass0RequiresWkbWkt"
    );

    /// OGC SFS Conformance Class 1 adds DE-9IM predicates and metric operations.
    ///
    /// Source: OGC 06-103r4 §1.4 — Conformance Class 1.
    pub struct ConformanceClass1AddsPredicates;
    structural_prop!(
        ConformanceClass1AddsPredicates,
        "ConformanceClass1AddsPredicates"
    );

    /// OGC SFS Conformance Class 1 adds constructive geometry operations.
    ///
    /// Source: OGC 06-103r4 §1.4 — Conformance Class 1.
    pub struct ConformanceClass1AddsSetOps;
    structural_prop!(ConformanceClass1AddsSetOps, "ConformanceClass1AddsSetOps");

    // -- Aggregate validity props (formal-verification seams) --

    /// A Point geometry is valid: its coordinates are finite (or it is the empty point).
    ///
    /// Composes PointXIsFinite + PointYIsFinite + PointZIsFiniteWhenPresent.
    /// Used as a precondition seam in Kani/Verus proofs of any computation that
    /// receives a point operand.
    ///
    /// Source: OGC 06-103r4 §6.1.4 — Point validity.
    pub struct PointValid;
    structural_prop!(PointValid, "PointValid");

    /// A LineString geometry is valid: two or more distinct points, finite coordinates,
    /// no self-intersection beyond common boundary points.
    ///
    /// Composes LineStringHasTwoOrMorePoints + PointXIsFinite + PointYIsFinite
    /// + LineStringSimpleNoSelfIntersection.
    ///
    /// Used as a precondition seam in formal verification of curve operations.
    ///
    /// Source: OGC 06-103r4 §6.1.6 — LineString validity.
    pub struct LineStringValid;
    structural_prop!(LineStringValid, "LineStringValid");

    /// A LinearRing geometry is valid: closed, simple, four or more positions, finite
    /// coordinates, non-degenerate area.
    ///
    /// Composes LinearRingIsClosedLineString + LinearRingIsSimple
    /// + LinearRingMinimumFourPositions + LinearRingNonDegenerate.
    ///
    /// Used as a precondition seam in formal verification of ring and polygon operations.
    ///
    /// Source: OGC 06-103r4 §6.1.7 — LinearRing validity.
    pub struct LinearRingValid;
    structural_prop!(LinearRingValid, "LinearRingValid");

    /// A Polygon geometry is valid: valid rings, CCW exterior, CW holes, no crossing
    /// rings, holes strictly inside the exterior.
    ///
    /// Composes PolygonExteriorIsCCW + PolygonHolesAreCW + PolygonHolesInsideExterior
    /// + PolygonHolesDontContainEachOther + PolygonNoRingSelfIntersects
    /// + PolygonRingsDontCross + LinearRingValid (for each ring).
    ///
    /// Used as a precondition seam in formal verification of area and overlay operations.
    ///
    /// Source: OGC 06-103r4 §6.1.11 — Polygon validity.
    pub struct PolygonValid;
    structural_prop!(PolygonValid, "PolygonValid");

    /// A MultiPoint geometry is valid: all component Points satisfy PointValid.
    ///
    /// Composes MultiPointComponentsArePoints + PointValid for each element.
    /// Used as a precondition seam in formal verification of multi-geometry operations.
    ///
    /// Source: OGC 06-103r4 §6.1.8 — MultiPoint validity.
    pub struct MultiPointValid;
    structural_prop!(MultiPointValid, "MultiPointValid");

    /// A MultiLineString geometry is valid: all component LineStrings satisfy
    /// LineStringValid.
    ///
    /// Composes MultiLineStringComponentsAreLineStrings + LineStringValid for each element.
    /// Used as a precondition seam in formal verification of multi-curve operations.
    ///
    /// Source: OGC 06-103r4 §6.1.9 — MultiLineString validity.
    pub struct MultiLineStringValid;
    structural_prop!(MultiLineStringValid, "MultiLineStringValid");

    /// A MultiPolygon geometry is valid: all component Polygons satisfy PolygonValid,
    /// interiors are disjoint, boundaries touch only at finite points.
    ///
    /// Composes MultiPolygonComponentsArePolygons + PolygonValid for each element
    /// + MultiPolygonInteriorsDisjoint + MultiPolygonBoundariesTouchAtPoints.
    ///
    /// Used as a precondition seam in formal verification of multi-surface operations.
    ///
    /// Source: OGC 06-103r4 §6.1.13 — MultiPolygon validity.
    pub struct MultiPolygonValid;
    structural_prop!(MultiPolygonValid, "MultiPolygonValid");

    /// A GeometryCollection is valid: every component geometry satisfies its
    /// type-specific validity aggregate prop.
    ///
    /// Composes GeometryCollectionAllComponentsValid paired with the appropriate
    /// per-type validity prop for each element.
    /// Used as a precondition seam in formal verification of collection operations.
    ///
    /// Source: OGC 06-103r4 §6.1.14 — GeometryCollection validity.
    pub struct GeometryCollectionValid;
    structural_prop!(GeometryCollectionValid, "GeometryCollectionValid");

    /// Any SFS geometry instance is valid per the OGC SFS §6 rules for its concrete type.
    ///
    /// Top-level aggregate: the union of all type-specific validity props.
    /// Used as the primary precondition in formal verification of any geometry operation
    /// when the concrete type is not known statically.
    ///
    /// Source: OGC 06-103r4 §6.1 — geometry validity.
    pub struct SfsGeometryValid;
    structural_prop!(SfsGeometryValid, "SfsGeometryValid");
}

pub use emit_impls::{
    Area0DReturnsZero, Area1DReturnsZero, Area2DReturnsPositive, AreaEmptyGeometryIsZero,
    AreaIsFinite, AreaNonNegative, AreaUnitsSquaredSrsUnits, AsBinaryMethodDefined,
    AsBinaryReturnsWkb, AsTextMethodDefined, AsTextReturnsWkt, BoundaryOfBoundaryIsEmpty,
    BufferNegativeDistanceShrinks, BufferPositiveDistanceIncreasesArea, BufferResultIsValid,
    BufferReturnsContainingGeometry, BufferZeroContainsSelf, CentroidDefinedForAll,
    CentroidReturnsPoint, CentroidWithinConvexHull, CentroidXIsFinite, CentroidYIsFinite,
    CollectionCoordDimMatchesComponents, CollectionGeometryNIndexed, CollectionGeometryNNeverNull,
    CollectionNumGeometriesCount, CollectionNumGeometriesNonNegative,
    ConformanceClass0RequiresSevenTypes, ConformanceClass0RequiresWkbWkt,
    ConformanceClass1AddsPredicates, ConformanceClass1AddsSetOps, ContainsDe9ImPattern,
    ContainsIsInverseOfWithin, ContainsIsNotSymmetric, ConvexHullContainsSelf,
    ConvexHullIdempotent, ConvexHullOfConvexPolygonIsSelf, ConvexHullReturnsSmallestConvex,
    Coord2DMPosition, Coord2DPosition, Coord3DMPosition, Coord3DPosition, Coord3DZIsElevation,
    CoordDimUniformInGeometry, CoordDimensionalityIsConsistent, CoordDimensionalityUniform,
    CoordMIsFiniteWhenPresent, CoordMIsMeasure, CoordXIsFinite, CoordYIsFinite,
    CoordZIsFiniteWhenPresent, CoveredByIsInverseOfCovers, CoversDe9ImPattern, CoversImpliesWithin,
    CrossesDe9ImPattern, CrossesDimIntersectionLessThanMax, CrossesDimensionConstraint,
    CrossesIsNotSymmetric, De9ImCellValues, De9ImInteriorBoundaryExteriorAxes, De9ImMatrixDefined,
    De9ImRelatePatternIsNineChars, DifferenceAsymmetric, DifferenceDimensionAtMostSelf,
    DifferenceDisjointFromSubtracted, DifferenceSubsetOfSelf, DimensionConsistencyInHierarchy,
    DisjointDe9ImPattern, DisjointIsInverseOfIntersects, DisjointIsSymmetric, DistanceInSrsUnits,
    DistanceIsFinite, DistanceNonNegative, DistanceSymmetric, DistanceTriangleInequality,
    DistanceZeroIffIntersecting, EmptyGeometryBoundaryIsEmpty,
    EmptyGeometryBoundaryIsEmptyGeometry, EmptyGeometryEnvelopeIsEmpty, EmptyHandlingConsistent,
    EnvelopeEmptyWhenGeometryEmpty, EnvelopeIsPointWhenDegenerate, EnvelopeIsPolygon,
    EqualsDe9ImPattern, EqualsIsReflexive, EqualsIsSymmetric, EwkbBaseTypeCodeMasked,
    EwkbSridIsUint32, EwkbSridPresent, GeometryBoundaryDefinedPerType,
    GeometryCollectionAllComponentsValid, GeometryCollectionBoundaryMod2Rule,
    GeometryCollectionHeterogeneousAllowed, GeometryCollectionIsSimpleAllSimple,
    GeometryCollectionMayBeEmpty, GeometryCollectionValid, GeometryDimension0ForPoint,
    GeometryDimension1ForLine, GeometryDimension2ForSurface, GeometryDimensionMinus1WhenEmpty,
    GeometryEnvelopeReturnsMbr, GeometryHasSrs, GeometryIsEmptyPredicate,
    GeometryIsSimpleAndIsValidDistinct, GeometryIsSimplePredicate, GeometryIsValidPredicate,
    GeometrySridReturnsInteger, GeometrySrsNotNull, GeometryTypeMatchesConcreteName,
    GeometryTypeReturnsString, IntersectionDimensionAtMostMin, IntersectionIsCommutative,
    IntersectionOfDisjointIsEmpty, IntersectionSubsetOfBothInputs, IntersectsAtLeastOneSharedPoint,
    IntersectsIsNotDisjoint, IntersectsIsSymmetric, IsEmptyFalseForNonEmpty, IsEmptyTrueForEmpty,
    IsSimpleNoSelfIntersection, IsValidImpliesSubComponentsValid, IsValidWellFormed,
    Length0DReturnsZero, Length1DReturnsPositive, Length2DReturnsZero, LengthEmptyGeometryIsZero,
    LengthInSrsUnits, LengthIsFinite, LengthNonNegative, LineStringAdjacentPointsDistinct,
    LineStringBoundaryIsEndpoints, LineStringClosedBoundaryEmpty, LineStringClosedEqualsLinearRing,
    LineStringEndPointIsPointNLast, LineStringHasTwoOrMorePoints, LineStringIsClosedStartEqualsEnd,
    LineStringIsRingImpliesClosedAndSimple, LineStringMinimumTwoPositions,
    LineStringNonClosedBoundaryIsEndpointMultiPoint, LineStringNumPointsMatchesCoordCount,
    LineStringOpenBoundaryTwoPoints, LineStringPointNInRange, LineStringSimpleNoSelfIntersection,
    LineStringStartPointIsPointNZero, LineStringValid, LinearRingBoundaryIsEmpty,
    LinearRingBoundaryIsEmptySet, LinearRingFirstPositionEqualsLast, LinearRingIsClosedLineString,
    LinearRingIsSimple, LinearRingMinimumFourPositions, LinearRingNonDegenerate, LinearRingValid,
    MultiLineStringComponentsAreLineStrings, MultiLineStringIsClosedAllClosed2,
    MultiLineStringMayBeEmpty, MultiLineStringSimpleWhenIntersectAtEndpointsOnly,
    MultiLineStringValid, MultiPointComponentsArePoints, MultiPointMayBeEmpty,
    MultiPointSimpleWhenNoTwoEqual, MultiPointValid, MultiPolygonBoundariesDontOverlap,
    MultiPolygonBoundariesTouchAtPoints, MultiPolygonBoundaryIsAllRings,
    MultiPolygonComponentsArePolygons, MultiPolygonInteriorsDisjoint, MultiPolygonMayBeEmpty,
    MultiPolygonValid, OverlapsDe9ImPattern2D, OverlapsDe9ImPatternLowDim,
    OverlapsDimensionConstraint, OverlapsIsSymmetric, PointAlwaysValid, PointBoundaryIsEmpty,
    PointEmptyHasNoCoords, PointEmptyIsEmpty, PointMDefinedWhenMPresent,
    PointOnSurfaceDefinedForAll, PointOnSurfaceIsOnGeometry, PointOnSurfaceReturnsPoint,
    PointValid, PointXIsFinite, PointXReturnsXOrdinate, PointYIsFinite, PointYReturnsYOrdinate,
    PointZDefinedWhen3D, PointZIsFiniteWhenPresent, PolygonBoundaryIsAllRings,
    PolygonExteriorIsCCW, PolygonExteriorIsLinearRing, PolygonExteriorRingNeverNull,
    PolygonHasExactlyOneExteriorRing, PolygonHoleCountNonNegative, PolygonHolesAreCW,
    PolygonHolesDontContainEachOther, PolygonHolesInsideExterior, PolygonInteriorRingNInRange,
    PolygonInteriorRingNReturnsHole, PolygonInteriorRingsAreLinearRings,
    PolygonNoRingSelfIntersects, PolygonNumInteriorRingsNonNegative, PolygonRingsDontCross,
    PolygonRingsDontTouchAlongSegment, PolygonRingsTouchAtPoints, PolygonValid,
    RelatePatternCharacters, RelatePatternLength9, RelateReturnsBooleanMatchingPattern,
    SfsGeometryValid, SridAssignedAtConstruction, SridIsInteger, SridNonNegative,
    SrsConsistentInCollection, SymDifferenceEqualsUnionMinusIntersection,
    SymDifferenceIsCommutative, TouchesDe9ImPattern, TouchesInteriorsDisjoint, TouchesIsSymmetric,
    UnionDimensionIsMax, UnionEqualsIntersectionPlusSymDifference, UnionIsAssociative,
    UnionIsCommutative, UnionSupersetOfBothInputs, WithinDe9ImPattern, WithinIsInverseOfContains,
    WithinIsNotSymmetric, WkbByteOrderBigEndian, WkbByteOrderLittleEndian,
    WkbByteOrderMarkerPresent, WkbByteOrderMarkerTwoValues, WkbCoordinateIsDouble,
    WkbCoordinatesByteOrderMatches, WkbCountFieldsAreUint32, WkbLengthValid, WkbMVariant,
    WkbPointLength21Bytes2D, WkbRingHasCount, WkbRoundTrip, WkbSubGeometriesAreCompleteBlobs,
    WkbTypeCodeGeometryCollection, WkbTypeCodeLineString, WkbTypeCodeMultiLineString,
    WkbTypeCodeMultiPoint, WkbTypeCodeMultiPolygon, WkbTypeCodePoint, WkbTypeCodePolygon,
    WkbTypeCodeValid, WkbZMVariant, WkbZVariant, WktCoordinateListInParens,
    WktCoordsSeparatedBySpace, WktDimensionTagPrecedesParens, WktEmptyApplicableToAllTypes,
    WktEmptyGeometry, WktGeometryCollectionProduction, WktKeywordCaseInsensitive, WktKeywordValid,
    WktLineStringProduction, WktMVariant, WktMultiLineStringProduction, WktMultiPointProduction,
    WktMultiPolygonProduction, WktNestingForMultiTypes, WktPointProduction,
    WktPolygonExteriorFirstRing, WktPolygonProduction, WktPositionsSeparatedByComma,
    WktRingsParenthesized, WktRingsSeparatedByComma, WktRoundTrip, WktZMVariant, WktZVariant,
};

// ── Proof composition: OGC SFS geometry validity chain ───────────────────────
//
// Each evidence bundle declares what must be proven before a composite
// proposition can be asserted.  Backends call `Established::prove(&bundle)`
// for composites; `Established::assert()` remains available for leaf
// propositions verified at runtime.

use elicitation::{Established, contracts::ProvableFrom};

/// Evidence that a polygon is topologically valid.
///
/// Requires proven linear-ring validity for the exterior ring and for
/// each interior hole.
///
/// Source: OGC 06-103r4 §6.1.11 — Polygon.
pub struct PolygonEvidence {
    /// Proof that the exterior ring is a valid closed simple linear ring.
    pub exterior: Established<LinearRingValid>,
    /// Proof for each interior ring (hole), if any.
    pub holes: Vec<Established<LinearRingValid>>,
}

impl ProvableFrom<PolygonEvidence> for PolygonValid {}

/// Evidence that a MultiPoint is valid.
///
/// Requires proven point validity for each component point.
///
/// Source: OGC 06-103r4 §6.1.14 — MultiPoint.
pub struct MultiPointEvidence {
    /// Proof for each component point.
    pub points: Vec<Established<PointValid>>,
}

impl ProvableFrom<MultiPointEvidence> for MultiPointValid {}

/// Evidence that a MultiLineString is valid.
///
/// Requires proven line-string validity for each component.
///
/// Source: OGC 06-103r4 §6.1.15 — MultiLineString.
pub struct MultiLineStringEvidence {
    /// Proof for each component line string.
    pub lines: Vec<Established<LineStringValid>>,
}

impl ProvableFrom<MultiLineStringEvidence> for MultiLineStringValid {}

/// Evidence that a MultiPolygon is valid.
///
/// Requires proven polygon validity for each component.
///
/// Source: OGC 06-103r4 §6.1.16 — MultiPolygon.
pub struct MultiPolygonEvidence {
    /// Proof for each component polygon.
    pub polygons: Vec<Established<PolygonValid>>,
}

impl ProvableFrom<MultiPolygonEvidence> for MultiPolygonValid {}

/// Evidence that a GeometryCollection is valid.
///
/// Requires proven SFS geometry validity for each component.
///
/// Source: OGC 06-103r4 §6.1.13 — GeometryCollection.
pub struct GeometryCollectionEvidence {
    /// Proof for each component geometry.
    pub geoms: Vec<Established<SfsGeometryValid>>,
}

impl ProvableFrom<GeometryCollectionEvidence> for GeometryCollectionValid {}

// ── Upcasts: concrete geometry proofs → SfsGeometryValid ─────────────────────
//
// Any proven concrete geometry type also proves the abstract SfsGeometryValid
// superprop.  These single-dependency impls let backends upcast without an
// explicit bundle struct.

impl ProvableFrom<Established<PointValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<LineStringValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<LinearRingValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<PolygonValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<MultiPointValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<MultiLineStringValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<MultiPolygonValid>> for SfsGeometryValid {}
impl ProvableFrom<Established<GeometryCollectionValid>> for SfsGeometryValid {}
