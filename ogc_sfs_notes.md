# OGC Simple Features Specification — Contract Implementation Notes

**Standard:** OGC 06-103r4 / ISO 19125-1:2004
**Title:** OpenGIS® Implementation Standard for Geographic Information — Simple Feature Access
**Part 1:** Common Architecture
**Published:** 2011-05-28 (OGC 06-103r4); ISO 19125-1:2004 is the corresponding ISO edition.

---

## ⚠ Pattern notice

All prop struct snippets in this file use the **correct** `structural_prop!` pattern.
Do **not** use `#[derive(Prop)]` or `#[spec_reference(...)]` — those do not exist.

```rust
mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Brief description of the proposition.
    ///
    /// Source: OGC 06-103r4 §X.Y — <section title>
    pub struct PropName;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream { quote! { /* structural: #name */ } }
                fn verus_proof() -> TokenStream { quote! { /* structural: #name */ } }
                fn creusot_proof() -> TokenStream { quote! { /* structural: #name */ } }
            }
        };
    }
    structural_prop!(PropName, "PropName");
}
pub use emit_impls::PropName;
```

Use this file as a content reference. All prop struct snippets are ready for
placement in `crates/elicit_gis/src/contracts/ogc_sfs.rs`.

---

## Normative references

- OGC 06-103r4 — OpenGIS Implementation Standard for Geographic Information —
  Simple Feature Access — Part 1: Common Architecture.
  <https://portal.ogc.org/files/?artifact_id=25355>
- ISO 19125-1:2004 — Geographic information — Simple feature access — Part 1:
  Common architecture.
- ISO/IEC 9075 (SQL/MM Spatial) — SQL Multimedia and Application Packages —
  Part 3: Spatial. Referenced by SFS for SQL binding.
- IEEE 754-2008 — Standard for Floating-Point Arithmetic. Referenced for WKB
  coordinate encoding.

---

## §4 Geometry Model Overview

OGC 06-103r4 §4 defines the abstract geometry model that all conforming
implementations must support. Every geometry instance belongs to a concrete
subtype of the abstract `Geometry` class and carries an associated Spatial
Reference System (SRS). The type hierarchy is:

```
Geometry
├── Point
├── Curve
│   └── LineString
│       └── LinearRing
├── Surface
│   └── Polygon
└── GeometryCollection
    ├── MultiPoint
    ├── MultiCurve
    │   └── MultiLineString
    └── MultiSurface
        └── MultiPolygon
```

### §4.1 — SRS assignment

Every geometry instance must carry a non-null spatial reference system
identifier. The SRS is assigned at construction and does not change.

```rust
/// Every geometry instance has an assigned spatial reference system.
///
/// Source: OGC 06-103r4 §4.1 — Geometry model overview
pub struct GeometryHasSrs;
structural_prop!(GeometryHasSrs, "GeometryHasSrs");

/// The SRS on a geometry instance is not null.
///
/// Source: OGC 06-103r4 §4.1 — Geometry model overview
pub struct GeometrySrsNotNull;
structural_prop!(GeometrySrsNotNull, "GeometrySrsNotNull");

/// SRID() returns the integer identifier of the geometry's SRS.
///
/// Source: OGC 06-103r4 §4.1 — Geometry model overview
pub struct GeometrySridReturnsInteger;
structural_prop!(GeometrySridReturnsInteger, "GeometrySridReturnsInteger");
```

### §4.2 — Dimension

The `dimension()` method returns -1 for empty geometries, 0 for point types,
1 for line types, and 2 for surface types.

```rust
/// dimension() returns -1 for any empty geometry instance.
///
/// Source: OGC 06-103r4 §4.2 — Geometry dimension
pub struct GeometryDimensionMinus1WhenEmpty;
structural_prop!(GeometryDimensionMinus1WhenEmpty, "GeometryDimensionMinus1WhenEmpty");

/// dimension() returns 0 for point and multi-point types.
///
/// Source: OGC 06-103r4 §4.2 — Geometry dimension
pub struct GeometryDimension0ForPoint;
structural_prop!(GeometryDimension0ForPoint, "GeometryDimension0ForPoint");

/// dimension() returns 1 for line, curve, and multi-line types.
///
/// Source: OGC 06-103r4 §4.2 — Geometry dimension
pub struct GeometryDimension1ForLine;
structural_prop!(GeometryDimension1ForLine, "GeometryDimension1ForLine");

/// dimension() returns 2 for polygon, surface, and multi-polygon types.
///
/// Source: OGC 06-103r4 §4.2 — Geometry dimension
pub struct GeometryDimension2ForSurface;
structural_prop!(GeometryDimension2ForSurface, "GeometryDimension2ForSurface");
```

### §4.3 — Core predicates

The standard defines three fundamental boolean predicates on `Geometry`:
`isEmpty()`, `isSimple()`, and `isValid()`. These are distinct predicates with
independent semantics.

```rust
/// isEmpty() is defined on every geometry type.
///
/// Source: OGC 06-103r4 §4.3 — Core predicates
pub struct GeometryIsEmptyPredicate;
structural_prop!(GeometryIsEmptyPredicate, "GeometryIsEmptyPredicate");

/// isSimple() is defined on every geometry type.
///
/// Source: OGC 06-103r4 §4.3 — Core predicates
pub struct GeometryIsSimplePredicate;
structural_prop!(GeometryIsSimplePredicate, "GeometryIsSimplePredicate");

/// isValid() is defined on every geometry type.
///
/// Source: OGC 06-103r4 §4.3 — Core predicates
pub struct GeometryIsValidPredicate;
structural_prop!(GeometryIsValidPredicate, "GeometryIsValidPredicate");

/// isSimple() and isValid() are semantically distinct predicates.
///
/// A geometry may be simple but not valid (e.g. a LinearRing with only 3 points),
/// or valid but not simple (depending on interpretation). The standard defines them
/// independently: isSimple tests self-intersection; isValid tests structural soundness.
///
/// Source: OGC 06-103r4 §4.3 — Core predicates
pub struct GeometryIsSimpleAndIsValidDistinct;
structural_prop!(GeometryIsSimpleAndIsValidDistinct, "GeometryIsSimpleAndIsValidDistinct");
```

### §4.4 — Envelope and boundary

The `envelope()` method returns the minimum bounding rectangle (MBR) as a
`Geometry`. The `boundary()` method is defined per geometry subtype.

```rust
/// envelope() returns the minimum bounding rectangle of the geometry.
///
/// Source: OGC 06-103r4 §4.4 — Envelope and boundary
pub struct GeometryEnvelopeReturnsMbr;
structural_prop!(GeometryEnvelopeReturnsMbr, "GeometryEnvelopeReturnsMbr");

/// boundary() is defined for each concrete geometry subtype.
///
/// Source: OGC 06-103r4 §4.4 — Envelope and boundary
pub struct GeometryBoundaryDefinedPerType;
structural_prop!(GeometryBoundaryDefinedPerType, "GeometryBoundaryDefinedPerType");

/// geometryType() returns a character string naming the concrete class.
///
/// Source: OGC 06-103r4 §4.4 — Geometry type name
pub struct GeometryTypeReturnsString;
structural_prop!(GeometryTypeReturnsString, "GeometryTypeReturnsString");
```

---

## §6.1.1 Geometry Methods

OGC 06-103r4 §6.1.1 specifies the interface methods common to all `Geometry`
instances. Each method has defined return type, semantics, and invariants.

### Overview props

```rust
/// geometryType() string equals the concrete class name of the geometry.
///
/// E.g. a Point instance returns "Point", a Polygon returns "Polygon".
///
/// Source: OGC 06-103r4 §6.1.1 — Geometry class interface
pub struct GeometryTypeMatchesConcreteName;
structural_prop!(GeometryTypeMatchesConcreteName, "GeometryTypeMatchesConcreteName");

/// SRID() returns a non-negative integer identifying the spatial reference system.
///
/// Source: OGC 06-103r4 §6.1.1 — Geometry class interface
pub struct SridNonNegative;
structural_prop!(SridNonNegative, "SridNonNegative");

/// SRID() return type is an integer (not a string, not a float).
///
/// Source: OGC 06-103r4 §6.1.1 — Geometry class interface
pub struct SridIsInteger;
structural_prop!(SridIsInteger, "SridIsInteger");
```

### Envelope method

```rust
/// envelope() returns a POLYGON for non-degenerate geometries.
///
/// Source: OGC 06-103r4 §6.1.1 — envelope() method
pub struct EnvelopeIsPolygon;
structural_prop!(EnvelopeIsPolygon, "EnvelopeIsPolygon");

/// envelope() returns a POINT when the geometry degenerates to a single point.
///
/// Source: OGC 06-103r4 §6.1.1 — envelope() method
pub struct EnvelopeIsPointWhenDegenerate;
structural_prop!(EnvelopeIsPointWhenDegenerate, "EnvelopeIsPointWhenDegenerate");

/// envelope() returns LINESTRING EMPTY when the geometry is empty.
///
/// The standard allows returning an empty geometry when the source is empty.
///
/// Source: OGC 06-103r4 §6.1.1 — envelope() method
pub struct EnvelopeEmptyWhenGeometryEmpty;
structural_prop!(EnvelopeEmptyWhenGeometryEmpty, "EnvelopeEmptyWhenGeometryEmpty");
```

### Serialization methods

```rust
/// asText() serialization method is defined on all geometry types.
///
/// Source: OGC 06-103r4 §6.1.1 — asText() / WKT
pub struct AsTextMethodDefined;
structural_prop!(AsTextMethodDefined, "AsTextMethodDefined");

/// asBinary() serialization method is defined on all geometry types.
///
/// Source: OGC 06-103r4 §6.1.1 — asBinary() / WKB
pub struct AsBinaryMethodDefined;
structural_prop!(AsBinaryMethodDefined, "AsBinaryMethodDefined");

/// asText() returns a Well-Known Text representation of the geometry.
///
/// Source: OGC 06-103r4 §6.1.1 — asText() / WKT
pub struct AsTextReturnsWkt;
structural_prop!(AsTextReturnsWkt, "AsTextReturnsWkt");

/// asBinary() returns a Well-Known Binary representation of the geometry.
///
/// Source: OGC 06-103r4 §6.1.1 — asBinary() / WKB
pub struct AsBinaryReturnsWkb;
structural_prop!(AsBinaryReturnsWkb, "AsBinaryReturnsWkb");
```

### isEmpty / isSimple / isValid

```rust
/// isEmpty() returns true for any geometry that contains no points.
///
/// Source: OGC 06-103r4 §6.1.1 — isEmpty()
pub struct IsEmptyTrueForEmpty;
structural_prop!(IsEmptyTrueForEmpty, "IsEmptyTrueForEmpty");

/// isEmpty() returns false for any geometry that contains at least one point.
///
/// Source: OGC 06-103r4 §6.1.1 — isEmpty()
pub struct IsEmptyFalseForNonEmpty;
structural_prop!(IsEmptyFalseForNonEmpty, "IsEmptyFalseForNonEmpty");

/// isSimple() returns true when the geometry has no self-intersections
/// except possibly at its endpoints (for curves).
///
/// Source: OGC 06-103r4 §6.1.1 — isSimple()
pub struct IsSimpleNoSelfIntersection;
structural_prop!(IsSimpleNoSelfIntersection, "IsSimpleNoSelfIntersection");

/// isValid() returns true when the geometry is structurally well-formed
/// per the rules for its concrete type.
///
/// Source: OGC 06-103r4 §6.1.1 — isValid()
pub struct IsValidWellFormed;
structural_prop!(IsValidWellFormed, "IsValidWellFormed");
```

---

## §6.1.2 Coordinate Dimensionality

OGC 06-103r4 §6.1.2 defines four coordinate modes. A geometry uses exactly
one mode uniformly across all its positions.

### Coordinate mode overview props

```rust
/// 2D coordinates carry exactly an x and a y ordinate.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct Coord2DPosition;
structural_prop!(Coord2DPosition, "Coord2DPosition");

/// 3D coordinates carry x, y, and z ordinates, where z is elevation in SRS units.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct Coord3DPosition;
structural_prop!(Coord3DPosition, "Coord3DPosition");

/// In 3D mode, z is the elevation expressed in the SRS vertical units.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct Coord3DZIsElevation;
structural_prop!(Coord3DZIsElevation, "Coord3DZIsElevation");

/// 2DM coordinates carry x, y, and m ordinates, where m is a linear measure.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct Coord2DMPosition;
structural_prop!(Coord2DMPosition, "Coord2DMPosition");

/// In 2DM or 3DM mode, m is a measure value along a linear feature.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordMIsMeasure;
structural_prop!(CoordMIsMeasure, "CoordMIsMeasure");

/// 3DM coordinates carry x, y, z, and m ordinates.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct Coord3DMPosition;
structural_prop!(Coord3DMPosition, "Coord3DMPosition");
```

### Uniformity constraints

```rust
/// All positions within a geometry instance share the same coordinate dimensionality.
///
/// Mixing 2D and 3D positions within a single geometry is not permitted.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordDimensionalityUniform;
structural_prop!(CoordDimensionalityUniform, "CoordDimensionalityUniform");

/// The x ordinate of every position is a finite IEEE 754 double-precision value.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordXIsFinite;
structural_prop!(CoordXIsFinite, "CoordXIsFinite");

/// The y ordinate of every position is a finite IEEE 754 double-precision value.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordYIsFinite;
structural_prop!(CoordYIsFinite, "CoordYIsFinite");

/// The z ordinate, when present, is a finite IEEE 754 double-precision value.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordZIsFiniteWhenPresent;
structural_prop!(CoordZIsFiniteWhenPresent, "CoordZIsFiniteWhenPresent");

/// The m ordinate, when present, is a finite IEEE 754 double-precision value.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordMIsFiniteWhenPresent;
structural_prop!(CoordMIsFiniteWhenPresent, "CoordMIsFiniteWhenPresent");

/// The number of ordinates per position is consistent throughout a geometry.
///
/// Source: OGC 06-103r4 §6.1.2 — Coordinate dimensionality
pub struct CoordDimensionalityIsConsistent;
structural_prop!(CoordDimensionalityIsConsistent, "CoordDimensionalityIsConsistent");
```

---

## §6.1.3 Validity per Geometry Type

OGC 06-103r4 §6.1.3 specifies validity rules for each concrete geometry
subtype. A geometry is valid when it satisfies all type-specific structural
rules. These rules are in addition to the general constraint that
`isValid() == true` only when all sub-components are also valid.

### Point validity

A non-empty Point is always structurally valid according to the standard. The
only invariant is that its coordinate values are finite numbers.

```rust
/// A non-empty Point geometry is always valid (no invalidity conditions for Point).
///
/// Source: OGC 06-103r4 §6.1.3 — Point validity
pub struct PointAlwaysValid;
structural_prop!(PointAlwaysValid, "PointAlwaysValid");

/// An empty Point has no coordinates.
///
/// Source: OGC 06-103r4 §6.1.3 — Point validity
pub struct PointEmptyHasNoCoords;
structural_prop!(PointEmptyHasNoCoords, "PointEmptyHasNoCoords");

/// The x ordinate of a Point is a finite floating-point number.
///
/// Source: OGC 06-103r4 §6.1.3 — Point validity
pub struct PointXIsFinite;
structural_prop!(PointXIsFinite, "PointXIsFinite");

/// The y ordinate of a Point is a finite floating-point number.
///
/// Source: OGC 06-103r4 §6.1.3 — Point validity
pub struct PointYIsFinite;
structural_prop!(PointYIsFinite, "PointYIsFinite");

/// The z ordinate of a Point, when present, is a finite floating-point number.
///
/// Source: OGC 06-103r4 §6.1.3 — Point validity
pub struct PointZIsFiniteWhenPresent;
structural_prop!(PointZIsFiniteWhenPresent, "PointZIsFiniteWhenPresent");

/// isEmpty() returns true for a Point with no coordinates.
///
/// Source: OGC 06-103r4 §6.1.3 — Point validity
pub struct PointEmptyIsEmpty;
structural_prop!(PointEmptyIsEmpty, "PointEmptyIsEmpty");
```

### LineString validity

A valid LineString must have at least 2 distinct points and no consecutive
identical positions.

```rust
/// A valid LineString has at least two distinct positions.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringHasTwoOrMorePoints;
structural_prop!(LineStringHasTwoOrMorePoints, "LineStringHasTwoOrMorePoints");

/// Adjacent positions in a LineString are distinct (no zero-length segments).
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringAdjacentPointsDistinct;
structural_prop!(LineStringAdjacentPointsDistinct, "LineStringAdjacentPointsDistinct");

/// A simple LineString does not self-intersect except possibly at its endpoints.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringSimpleNoSelfIntersection;
structural_prop!(LineStringSimpleNoSelfIntersection, "LineStringSimpleNoSelfIntersection");

/// A LineString whose first position equals its last position is a LinearRing.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringClosedEqualsLinearRing;
structural_prop!(LineStringClosedEqualsLinearRing, "LineStringClosedEqualsLinearRing");

/// The boundary of an open (non-closed) LineString is its two endpoint points.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringOpenBoundaryTwoPoints;
structural_prop!(LineStringOpenBoundaryTwoPoints, "LineStringOpenBoundaryTwoPoints");

/// The boundary of a closed LineString (first == last) is the empty set.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringClosedBoundaryEmpty;
structural_prop!(LineStringClosedBoundaryEmpty, "LineStringClosedBoundaryEmpty");

/// The position count of a LineString is at least 2.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringMinimumTwoPositions;
structural_prop!(LineStringMinimumTwoPositions, "LineStringMinimumTwoPositions");

/// The boundary of a general LineString equals the set of its two endpoints.
///
/// Source: OGC 06-103r4 §6.1.3 — LineString validity
pub struct LineStringBoundaryIsEndpoints;
structural_prop!(LineStringBoundaryIsEndpoints, "LineStringBoundaryIsEndpoints");
```

### LinearRing validity

A LinearRing is a closed, simple LineString forming a topological ring.

```rust
/// A LinearRing is a closed LineString: its first position equals its last.
///
/// Source: OGC 06-103r4 §6.1.3 — LinearRing validity
pub struct LinearRingIsClosedLineString;
structural_prop!(LinearRingIsClosedLineString, "LinearRingIsClosedLineString");

/// A LinearRing has at least 4 positions (including the repeated start/end position).
///
/// Source: OGC 06-103r4 §6.1.3 — LinearRing validity
pub struct LinearRingMinimumFourPositions;
structural_prop!(LinearRingMinimumFourPositions, "LinearRingMinimumFourPositions");

/// A LinearRing is simple: it does not self-intersect at any interior point.
///
/// Source: OGC 06-103r4 §6.1.3 — LinearRing validity
pub struct LinearRingIsSimple;
structural_prop!(LinearRingIsSimple, "LinearRingIsSimple");

/// A LinearRing encloses a non-degenerate area (area > 0).
///
/// Source: OGC 06-103r4 §6.1.3 — LinearRing validity
pub struct LinearRingNonDegenerate;
structural_prop!(LinearRingNonDegenerate, "LinearRingNonDegenerate");

/// The first position of a LinearRing is coordinate-equal to the last position.
///
/// Source: OGC 06-103r4 §6.1.3 — LinearRing validity
pub struct LinearRingFirstPositionEqualsLast;
structural_prop!(LinearRingFirstPositionEqualsLast, "LinearRingFirstPositionEqualsLast");

/// The boundary of a LinearRing is the empty set (it is closed with no endpoints).
///
/// Source: OGC 06-103r4 §6.1.3 — LinearRing validity
pub struct LinearRingBoundaryIsEmpty;
structural_prop!(LinearRingBoundaryIsEmpty, "LinearRingBoundaryIsEmpty");
```

### Polygon validity

A Polygon consists of an exterior ring and zero or more interior rings (holes),
all of which must be valid LinearRings satisfying additional topological rules.

```rust
/// The exterior ring of a valid Polygon is a valid LinearRing.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonExteriorIsLinearRing;
structural_prop!(PolygonExteriorIsLinearRing, "PolygonExteriorIsLinearRing");

/// All interior rings (holes) of a valid Polygon are valid LinearRings.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonInteriorRingsAreLinearRings;
structural_prop!(PolygonInteriorRingsAreLinearRings, "PolygonInteriorRingsAreLinearRings");

/// The exterior ring of a Polygon is oriented counterclockwise (CCW).
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonExteriorIsCCW;
structural_prop!(PolygonExteriorIsCCW, "PolygonExteriorIsCCW");

/// Interior rings (holes) of a Polygon are oriented clockwise (CW).
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonHolesAreCW;
structural_prop!(PolygonHolesAreCW, "PolygonHolesAreCW");

/// No ring of a Polygon self-intersects.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonNoRingSelfIntersects;
structural_prop!(PolygonNoRingSelfIntersects, "PolygonNoRingSelfIntersects");

/// No two rings of a Polygon cross each other.
///
/// Rings may touch at a finite set of points but must not cross.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonRingsDontCross;
structural_prop!(PolygonRingsDontCross, "PolygonRingsDontCross");

/// Each hole of a Polygon lies inside the exterior ring.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonHolesInsideExterior;
structural_prop!(PolygonHolesInsideExterior, "PolygonHolesInsideExterior");

/// Holes of a Polygon do not contain each other.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonHolesDontContainEachOther;
structural_prop!(PolygonHolesDontContainEachOther, "PolygonHolesDontContainEachOther");

/// Ring boundaries of a Polygon touch at most at a finite number of points.
///
/// Boundaries must not share a line segment (touch along a segment is invalid).
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonRingsTouchAtPoints;
structural_prop!(PolygonRingsTouchAtPoints, "PolygonRingsTouchAtPoints");

/// No two rings of a Polygon overlap along a segment.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonRingsDontTouchAlongSegment;
structural_prop!(PolygonRingsDontTouchAlongSegment, "PolygonRingsDontTouchAlongSegment");

/// A valid Polygon has exactly one exterior ring.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonHasExactlyOneExteriorRing;
structural_prop!(PolygonHasExactlyOneExteriorRing, "PolygonHasExactlyOneExteriorRing");

/// The number of interior rings (holes) in a Polygon is non-negative.
///
/// Source: OGC 06-103r4 §6.1.3 — Polygon validity
pub struct PolygonHoleCountNonNegative;
structural_prop!(PolygonHoleCountNonNegative, "PolygonHoleCountNonNegative");
```

### MultiPoint validity

```rust
/// All component Points in a MultiPoint are valid Points.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPoint validity
pub struct MultiPointComponentsArePoints;
structural_prop!(MultiPointComponentsArePoints, "MultiPointComponentsArePoints");

/// A MultiPoint may be empty (contain zero Points).
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPoint validity
pub struct MultiPointMayBeEmpty;
structural_prop!(MultiPointMayBeEmpty, "MultiPointMayBeEmpty");

/// isSimple() on a MultiPoint returns true when no two component Points are equal.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPoint validity
pub struct MultiPointSimpleWhenNoTwoEqual;
structural_prop!(MultiPointSimpleWhenNoTwoEqual, "MultiPointSimpleWhenNoTwoEqual");
```

### MultiLineString validity

```rust
/// All component LineStrings in a MultiLineString are valid LineStrings.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiLineString validity
pub struct MultiLineStringComponentsAreLineStrings;
structural_prop!(MultiLineStringComponentsAreLineStrings, "MultiLineStringComponentsAreLineStrings");

/// A MultiLineString may be empty (contain zero LineStrings).
///
/// Source: OGC 06-103r4 §6.1.3 — MultiLineString validity
pub struct MultiLineStringMayBeEmpty;
structural_prop!(MultiLineStringMayBeEmpty, "MultiLineStringMayBeEmpty");

/// isSimple() on a MultiLineString returns true when the curves intersect only at
/// their endpoints.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiLineString validity
pub struct MultiLineStringSimpleWhenIntersectAtEndpointsOnly;
structural_prop!(MultiLineStringSimpleWhenIntersectAtEndpointsOnly, "MultiLineStringSimpleWhenIntersectAtEndpointsOnly");
```

### MultiPolygon validity

```rust
/// All component Polygons in a MultiPolygon are valid Polygons.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity
pub struct MultiPolygonComponentsArePolygons;
structural_prop!(MultiPolygonComponentsArePolygons, "MultiPolygonComponentsArePolygons");

/// The interiors of component Polygons in a MultiPolygon are pairwise disjoint.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity
pub struct MultiPolygonInteriorsDisjoint;
structural_prop!(MultiPolygonInteriorsDisjoint, "MultiPolygonInteriorsDisjoint");

/// Component Polygon boundaries in a MultiPolygon intersect at most at a finite
/// set of points (not along a segment).
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity
pub struct MultiPolygonBoundariesTouchAtPoints;
structural_prop!(MultiPolygonBoundariesTouchAtPoints, "MultiPolygonBoundariesTouchAtPoints");

/// No two component Polygon boundaries in a MultiPolygon overlap along a segment.
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity
pub struct MultiPolygonBoundariesDontOverlap;
structural_prop!(MultiPolygonBoundariesDontOverlap, "MultiPolygonBoundariesDontOverlap");

/// A MultiPolygon may be empty (contain zero Polygons).
///
/// Source: OGC 06-103r4 §6.1.3 — MultiPolygon validity
pub struct MultiPolygonMayBeEmpty;
structural_prop!(MultiPolygonMayBeEmpty, "MultiPolygonMayBeEmpty");
```

### GeometryCollection validity

```rust
/// All component geometries in a GeometryCollection are themselves valid.
///
/// Source: OGC 06-103r4 §6.1.3 — GeometryCollection validity
pub struct GeometryCollectionAllComponentsValid;
structural_prop!(GeometryCollectionAllComponentsValid, "GeometryCollectionAllComponentsValid");

/// A GeometryCollection may contain geometries of heterogeneous types.
///
/// Source: OGC 06-103r4 §6.1.3 — GeometryCollection validity
pub struct GeometryCollectionHeterogeneousAllowed;
structural_prop!(GeometryCollectionHeterogeneousAllowed, "GeometryCollectionHeterogeneousAllowed");

/// A GeometryCollection may be empty (contain zero components).
///
/// Source: OGC 06-103r4 §6.1.3 — GeometryCollection validity
pub struct GeometryCollectionMayBeEmpty;
structural_prop!(GeometryCollectionMayBeEmpty, "GeometryCollectionMayBeEmpty");
```

---

## §6.2 Spatial Predicates (DE-9IM)

OGC 06-103r4 §6.2 defines the named spatial relationship predicates using the
Dimensionally Extended 9-Intersection Model (DE-9IM). The model represents the
intersection of two geometry sets across their Interior (I), Boundary (B), and
Exterior (E) components:

```
         | Interior(B) | Boundary(B) | Exterior(B)
---------|-------------|-------------|------------
Interior(A) | [0,0]    | [0,1]       | [0,2]
Boundary(A) | [1,0]    | [1,1]       | [1,2]
Exterior(A) | [2,0]    | [2,1]       | [2,2]
```

Each cell value is the topological dimension of the intersection:

- `-1` (or `F`) — empty intersection (no shared points)
- `0` — intersection is a point set (dimensionality 0)
- `1` — intersection is a curve (dimensionality 1)
- `2` — intersection is an area (dimensionality 2)
- `T` — any non-empty intersection (dimension ≥ 0)
- `*` — don't care (any value allowed)

A 9-character pattern string encodes the matrix row-major:
`[II][IB][IE][BI][BB][BE][EI][EB][EE]`.

### DE-9IM matrix overview props

```rust
/// The DE-9IM matrix consists of exactly 9 cells covering I×I through E×E.
///
/// Source: OGC 06-103r4 §6.2 — DE-9IM matrix
pub struct De9ImMatrixDefined;
structural_prop!(De9ImMatrixDefined, "De9ImMatrixDefined");

/// Each DE-9IM cell value is -1, 0, 1, 2, T, F, or *.
///
/// Source: OGC 06-103r4 §6.2 — DE-9IM cell values
pub struct De9ImCellValues;
structural_prop!(De9ImCellValues, "De9ImCellValues");

/// The DE-9IM axes are Interior, Boundary, and Exterior for both geometries.
///
/// Source: OGC 06-103r4 §6.2 — DE-9IM axes
pub struct De9ImInteriorBoundaryExteriorAxes;
structural_prop!(De9ImInteriorBoundaryExteriorAxes, "De9ImInteriorBoundaryExteriorAxes");

/// Relate(B, pattern) evaluates the DE-9IM matrix against a 9-character pattern string.
///
/// Source: OGC 06-103r4 §6.2 — relate() method
pub struct De9ImRelatePatternIsNineChars;
structural_prop!(De9ImRelatePatternIsNineChars, "De9ImRelatePatternIsNineChars");
```

### Equals

- Pattern: `T*F**FFF*`
- Symmetric: Yes — A.equals(B) ↔ B.equals(A)
- Semantics: Same point set; neither contains a point the other lacks.

```rust
/// Equals DE-9IM pattern is T*F**FFF*.
///
/// Both geometries have the same point set. Interior(A)∩Interior(B) is non-empty,
/// neither exterior contains any part of the other.
///
/// Source: OGC 06-103r4 §6.2.1 — Equals predicate
pub struct EqualsDe9ImPattern;
structural_prop!(EqualsDe9ImPattern, "EqualsDe9ImPattern");

/// Equals is symmetric: A.equals(B) if and only if B.equals(A).
///
/// Source: OGC 06-103r4 §6.2.1 — Equals predicate
pub struct EqualsIsSymmetric;
structural_prop!(EqualsIsSymmetric, "EqualsIsSymmetric");

/// A geometry equals itself (reflexivity).
///
/// Source: OGC 06-103r4 §6.2.1 — Equals predicate
pub struct EqualsIsReflexive;
structural_prop!(EqualsIsReflexive, "EqualsIsReflexive");
```

### Disjoint

- Pattern: `FF*FF****`
- Symmetric: Yes
- Semantics: No shared points; interiors and boundaries do not intersect.

```rust
/// Disjoint DE-9IM pattern is FF*FF****.
///
/// Geometries share no points: Interior(A)∩Interior(B) is empty and
/// Interior(A)∩Boundary(B) is empty and Boundary(A)∩Interior(B) is empty
/// and Boundary(A)∩Boundary(B) is empty.
///
/// Source: OGC 06-103r4 §6.2.2 — Disjoint predicate
pub struct DisjointDe9ImPattern;
structural_prop!(DisjointDe9ImPattern, "DisjointDe9ImPattern");

/// Disjoint is symmetric: A.disjoint(B) if and only if B.disjoint(A).
///
/// Source: OGC 06-103r4 §6.2.2 — Disjoint predicate
pub struct DisjointIsSymmetric;
structural_prop!(DisjointIsSymmetric, "DisjointIsSymmetric");

/// Disjoint is the inverse of Intersects: A.disjoint(B) ↔ !A.intersects(B).
///
/// Source: OGC 06-103r4 §6.2.2 — Disjoint predicate
pub struct DisjointIsInverseOfIntersects;
structural_prop!(DisjointIsInverseOfIntersects, "DisjointIsInverseOfIntersects");
```

### Intersects

- Pattern: NOT Disjoint (i.e., `!FF*FF****`)
- Symmetric: Yes
- Semantics: At least one shared point between the two geometries.

```rust
/// Intersects is defined as the logical negation of Disjoint.
///
/// Source: OGC 06-103r4 §6.2.3 — Intersects predicate
pub struct IntersectsIsNotDisjoint;
structural_prop!(IntersectsIsNotDisjoint, "IntersectsIsNotDisjoint");

/// Intersects is symmetric: A.intersects(B) if and only if B.intersects(A).
///
/// Source: OGC 06-103r4 §6.2.3 — Intersects predicate
pub struct IntersectsIsSymmetric;
structural_prop!(IntersectsIsSymmetric, "IntersectsIsSymmetric");

/// Intersects returns true when the two geometries share at least one point.
///
/// Source: OGC 06-103r4 §6.2.3 — Intersects predicate
pub struct IntersectsAtLeastOneSharedPoint;
structural_prop!(IntersectsAtLeastOneSharedPoint, "IntersectsAtLeastOneSharedPoint");
```

### Touches

- Patterns: `FT*******`, `F**T*****`, `F***T****`
- Symmetric: Yes
- Semantics: Geometries share boundary points only; their interiors are disjoint.
- Dimension constraint: Not applicable to Point/Point pairs (points have no boundary).

```rust
/// Touches DE-9IM patterns are FT*******, F**T*****, F***T****.
///
/// The geometries share only boundary points; their interiors do not intersect.
///
/// Source: OGC 06-103r4 §6.2.4 — Touches predicate
pub struct TouchesDe9ImPattern;
structural_prop!(TouchesDe9ImPattern, "TouchesDe9ImPattern");

/// Touches is symmetric: A.touches(B) if and only if B.touches(A).
///
/// Source: OGC 06-103r4 §6.2.4 — Touches predicate
pub struct TouchesIsSymmetric;
structural_prop!(TouchesIsSymmetric, "TouchesIsSymmetric");

/// Touches requires that Interior(A) ∩ Interior(B) is empty.
///
/// Source: OGC 06-103r4 §6.2.4 — Touches predicate
pub struct TouchesInteriorsDisjoint;
structural_prop!(TouchesInteriorsDisjoint, "TouchesInteriorsDisjoint");
```

### Crosses

- Patterns (dim-dependent): `T*T***T**` (curve/curve or curve/surface)
- Symmetric: No — A.crosses(B) and B.crosses(A) may differ.
- Semantics: Interiors intersect; dimension of intersection < max(dim(A), dim(B)).
- Dimension constraint: Valid for 0D/1D, 1D/1D, 1D/2D combinations.

```rust
/// Crosses DE-9IM pattern is T*T***T** (for curve/curve and curve/surface).
///
/// Source: OGC 06-103r4 §6.2.5 — Crosses predicate
pub struct CrossesDe9ImPattern;
structural_prop!(CrossesDe9ImPattern, "CrossesDe9ImPattern");

/// Crosses requires the dimension of the intersection to be less than the maximum
/// dimension of the two input geometries.
///
/// Source: OGC 06-103r4 §6.2.5 — Crosses predicate
pub struct CrossesDimIntersectionLessThanMax;
structural_prop!(CrossesDimIntersectionLessThanMax, "CrossesDimIntersectionLessThanMax");

/// Crosses is not generally symmetric: A.crosses(B) does not imply B.crosses(A).
///
/// Source: OGC 06-103r4 §6.2.5 — Crosses predicate
pub struct CrossesIsNotSymmetric;
structural_prop!(CrossesIsNotSymmetric, "CrossesIsNotSymmetric");

/// Crosses is applicable to 0D/1D, 1D/1D, and 1D/2D geometry pairs.
///
/// Source: OGC 06-103r4 §6.2.5 — Crosses predicate
pub struct CrossesDimensionConstraint;
structural_prop!(CrossesDimensionConstraint, "CrossesDimensionConstraint");
```

### Within

- Pattern: `T*F**F***`
- Symmetric: No — A.within(B) means B.contains(A).
- Semantics: Every point of A lies inside B; A's interior intersects B's interior.

```rust
/// Within DE-9IM pattern is T*F**F***.
///
/// Every point of geometry A is also a point of B, and A's interior intersects
/// B's interior.
///
/// Source: OGC 06-103r4 §6.2.6 — Within predicate
pub struct WithinDe9ImPattern;
structural_prop!(WithinDe9ImPattern, "WithinDe9ImPattern");

/// A.within(B) if and only if B.contains(A) (inverse relationship).
///
/// Source: OGC 06-103r4 §6.2.6 — Within predicate
pub struct WithinIsInverseOfContains;
structural_prop!(WithinIsInverseOfContains, "WithinIsInverseOfContains");

/// Within is not symmetric in general.
///
/// Source: OGC 06-103r4 §6.2.6 — Within predicate
pub struct WithinIsNotSymmetric;
structural_prop!(WithinIsNotSymmetric, "WithinIsNotSymmetric");
```

### Contains

- Pattern: `T*****FF*`
- Symmetric: No
- Semantics: Every point of B is in A; B's interior intersects A's interior.

```rust
/// Contains DE-9IM pattern is T*****FF*.
///
/// Every point of B lies inside A, and B's interior intersects A's interior.
///
/// Source: OGC 06-103r4 §6.2.7 — Contains predicate
pub struct ContainsDe9ImPattern;
structural_prop!(ContainsDe9ImPattern, "ContainsDe9ImPattern");

/// A.contains(B) if and only if B.within(A) (inverse relationship).
///
/// Source: OGC 06-103r4 §6.2.7 — Contains predicate
pub struct ContainsIsInverseOfWithin;
structural_prop!(ContainsIsInverseOfWithin, "ContainsIsInverseOfWithin");

/// Contains is not symmetric in general.
///
/// Source: OGC 06-103r4 §6.2.7 — Contains predicate
pub struct ContainsIsNotSymmetric;
structural_prop!(ContainsIsNotSymmetric, "ContainsIsNotSymmetric");
```

### Overlaps

- Patterns: `T*T***T**` (2D/2D) or `1*T***T**` (0D/0D or 1D/1D)
- Symmetric: Yes
- Semantics: Interiors intersect; neither is within the other; dimension of
  intersection equals dimension of both inputs.
- Dimension constraint: Both geometries must have the same topological dimension.

```rust
/// Overlaps DE-9IM pattern for 2D/2D is T*T***T**.
///
/// Source: OGC 06-103r4 §6.2.8 — Overlaps predicate
pub struct OverlapsDe9ImPattern2D;
structural_prop!(OverlapsDe9ImPattern2D, "OverlapsDe9ImPattern2D");

/// Overlaps DE-9IM pattern for 0D/0D or 1D/1D is 1*T***T**.
///
/// Source: OGC 06-103r4 §6.2.8 — Overlaps predicate
pub struct OverlapsDe9ImPatternLowDim;
structural_prop!(OverlapsDe9ImPatternLowDim, "OverlapsDe9ImPatternLowDim");

/// Overlaps is symmetric: A.overlaps(B) if and only if B.overlaps(A).
///
/// Source: OGC 06-103r4 §6.2.8 — Overlaps predicate
pub struct OverlapsIsSymmetric;
structural_prop!(OverlapsIsSymmetric, "OverlapsIsSymmetric");

/// Overlaps requires both input geometries to have the same topological dimension.
///
/// Source: OGC 06-103r4 §6.2.8 — Overlaps predicate
pub struct OverlapsDimensionConstraint;
structural_prop!(OverlapsDimensionConstraint, "OverlapsDimensionConstraint");
```

### Covers and CoveredBy

These are extensions beyond the original SFS §6.2 but are included in
OGC 06-103r4 Annex A and widely implemented.

```rust
/// Covers is similar to Contains but includes boundary points of B on A's boundary.
///
/// Pattern: T*****FF* or *T****FF* or ***T**FF* or ****T*FF*
///
/// Source: OGC 06-103r4 Annex A — Covers predicate
pub struct CoversDe9ImPattern;
structural_prop!(CoversDe9ImPattern, "CoversDe9ImPattern");

/// CoveredBy is the inverse of Covers: A.coveredBy(B) ↔ B.covers(A).
///
/// Source: OGC 06-103r4 Annex A — CoveredBy predicate
pub struct CoveredByIsInverseOfCovers;
structural_prop!(CoveredByIsInverseOfCovers, "CoveredByIsInverseOfCovers");

/// Covers implies Contains, but Contains does not imply Covers.
///
/// Source: OGC 06-103r4 Annex A — Covers predicate
pub struct CoversImpliesWithin;
structural_prop!(CoversImpliesWithin, "CoversImpliesWithin");
```

### Relate (general DE-9IM test)

```rust
/// Relate(B, pattern) tests the DE-9IM matrix against an explicit 9-character pattern.
///
/// Source: OGC 06-103r4 §6.2.9 — Relate method
pub struct RelatePatternLength9;
structural_prop!(RelatePatternLength9, "RelatePatternLength9");

/// Each character in a Relate pattern must be one of: T, F, *, 0, 1, 2.
///
/// Source: OGC 06-103r4 §6.2.9 — Relate method
pub struct RelatePatternCharacters;
structural_prop!(RelatePatternCharacters, "RelatePatternCharacters");

/// Relate(B, pattern) returns true iff the computed DE-9IM matrix matches the pattern.
///
/// Source: OGC 06-103r4 §6.2.9 — Relate method
pub struct RelateReturnsBooleanMatchingPattern;
structural_prop!(RelateReturnsBooleanMatchingPattern, "RelateReturnsBooleanMatchingPattern");
```

---

## §6.3 Metric Operations

OGC 06-103r4 §6.3 specifies the metric analysis methods: `area()`, `length()`,
`distance()`, `centroid()`, and `pointOnSurface()`. All metric values are in the
units of the geometry's SRS.

### area()

```rust
/// area() returns 0.0 for 0-dimensional (point) geometry types.
///
/// Source: OGC 06-103r4 §6.3 — area() method
pub struct Area0DReturnsZero;
structural_prop!(Area0DReturnsZero, "Area0DReturnsZero");

/// area() returns 0.0 for 1-dimensional (line) geometry types.
///
/// Source: OGC 06-103r4 §6.3 — area() method
pub struct Area1DReturnsZero;
structural_prop!(Area1DReturnsZero, "Area1DReturnsZero");

/// area() returns a positive real for non-empty 2-dimensional geometry types.
///
/// Source: OGC 06-103r4 §6.3 — area() method
pub struct Area2DReturnsPositive;
structural_prop!(Area2DReturnsPositive, "Area2DReturnsPositive");

/// area() is always ≥ 0.
///
/// Source: OGC 06-103r4 §6.3 — area() method
pub struct AreaNonNegative;
structural_prop!(AreaNonNegative, "AreaNonNegative");

/// area() is expressed in squared SRS units.
///
/// Source: OGC 06-103r4 §6.3 — area() method
pub struct AreaUnitsSquaredSrsUnits;
structural_prop!(AreaUnitsSquaredSrsUnits, "AreaUnitsSquaredSrsUnits");

/// area() returns 0.0 for any empty geometry regardless of its declared type.
///
/// Source: OGC 06-103r4 §6.3 — area() method
pub struct AreaEmptyGeometryIsZero;
structural_prop!(AreaEmptyGeometryIsZero, "AreaEmptyGeometryIsZero");
```

### length()

```rust
/// length() returns 0.0 for 0-dimensional (point) geometry types.
///
/// Source: OGC 06-103r4 §6.3 — length() method
pub struct Length0DReturnsZero;
structural_prop!(Length0DReturnsZero, "Length0DReturnsZero");

/// length() returns 0.0 for 2-dimensional (surface) geometry types.
///
/// Source: OGC 06-103r4 §6.3 — length() method
pub struct Length2DReturnsZero;
structural_prop!(Length2DReturnsZero, "Length2DReturnsZero");

/// length() returns a positive real for non-empty 1-dimensional geometry types.
///
/// Source: OGC 06-103r4 §6.3 — length() method
pub struct Length1DReturnsPositive;
structural_prop!(Length1DReturnsPositive, "Length1DReturnsPositive");

/// length() is always ≥ 0.
///
/// Source: OGC 06-103r4 §6.3 — length() method
pub struct LengthNonNegative;
structural_prop!(LengthNonNegative, "LengthNonNegative");

/// length() is expressed in the linear units of the SRS.
///
/// Source: OGC 06-103r4 §6.3 — length() method
pub struct LengthInSrsUnits;
structural_prop!(LengthInSrsUnits, "LengthInSrsUnits");

/// length() returns 0.0 for any empty geometry regardless of its declared type.
///
/// Source: OGC 06-103r4 §6.3 — length() method
pub struct LengthEmptyGeometryIsZero;
structural_prop!(LengthEmptyGeometryIsZero, "LengthEmptyGeometryIsZero");
```

### distance()

```rust
/// distance(B) is always ≥ 0.
///
/// Source: OGC 06-103r4 §6.3 — distance() method
pub struct DistanceNonNegative;
structural_prop!(DistanceNonNegative, "DistanceNonNegative");

/// distance(B) equals 0 if and only if the two geometries intersect.
///
/// Source: OGC 06-103r4 §6.3 — distance() method
pub struct DistanceZeroIffIntersecting;
structural_prop!(DistanceZeroIffIntersecting, "DistanceZeroIffIntersecting");

/// distance(B) is symmetric: distance(A, B) == distance(B, A).
///
/// Source: OGC 06-103r4 §6.3 — distance() method
pub struct DistanceSymmetric;
structural_prop!(DistanceSymmetric, "DistanceSymmetric");

/// distance(B) is expressed in the linear units of the SRS.
///
/// Source: OGC 06-103r4 §6.3 — distance() method
pub struct DistanceInSrsUnits;
structural_prop!(DistanceInSrsUnits, "DistanceInSrsUnits");

/// distance satisfies the triangle inequality: dist(A,C) ≤ dist(A,B) + dist(B,C).
///
/// Source: OGC 06-103r4 §6.3 — distance() method
pub struct DistanceTriangleInequality;
structural_prop!(DistanceTriangleInequality, "DistanceTriangleInequality");
```

### centroid()

```rust
/// centroid() returns a Point for any geometry type.
///
/// Source: OGC 06-103r4 §6.3 — centroid() method
pub struct CentroidReturnsPoint;
structural_prop!(CentroidReturnsPoint, "CentroidReturnsPoint");

/// centroid() is defined for all non-empty geometry types including collections.
///
/// Source: OGC 06-103r4 §6.3 — centroid() method
pub struct CentroidDefinedForAll;
structural_prop!(CentroidDefinedForAll, "CentroidDefinedForAll");

/// The x ordinate of centroid() is a finite floating-point number.
///
/// Source: OGC 06-103r4 §6.3 — centroid() method
pub struct CentroidXIsFinite;
structural_prop!(CentroidXIsFinite, "CentroidXIsFinite");

/// The y ordinate of centroid() is a finite floating-point number.
///
/// Source: OGC 06-103r4 §6.3 — centroid() method
pub struct CentroidYIsFinite;
structural_prop!(CentroidYIsFinite, "CentroidYIsFinite");

/// The centroid of a Polygon lies within the convex hull of that Polygon.
///
/// Source: OGC 06-103r4 §6.3 — centroid() method
pub struct CentroidWithinConvexHull;
structural_prop!(CentroidWithinConvexHull, "CentroidWithinConvexHull");
```

### pointOnSurface()

```rust
/// pointOnSurface() returns a Point guaranteed to be on or inside the geometry.
///
/// Source: OGC 06-103r4 §6.3 — pointOnSurface() method
pub struct PointOnSurfaceReturnsPoint;
structural_prop!(PointOnSurfaceReturnsPoint, "PointOnSurfaceReturnsPoint");

/// The Point returned by pointOnSurface() intersects the geometry (is within it).
///
/// Source: OGC 06-103r4 §6.3 — pointOnSurface() method
pub struct PointOnSurfaceIsOnGeometry;
structural_prop!(PointOnSurfaceIsOnGeometry, "PointOnSurfaceIsOnGeometry");

/// pointOnSurface() is defined for all non-empty geometry types.
///
/// Source: OGC 06-103r4 §6.3 — pointOnSurface() method
pub struct PointOnSurfaceDefinedForAll;
structural_prop!(PointOnSurfaceDefinedForAll, "PointOnSurfaceDefinedForAll");
```

---

## §7.2 Well-Known Text (WKT) Representation

OGC 06-103r4 §7.2 specifies the WKT grammar for serializing and deserializing
geometry instances. WKT is a human-readable text format. The grammar uses BNF
with the following base structure:

```
<WKT representation> ::=
    <point tagged text>
  | <linestring tagged text>
  | <polygon tagged text>
  | <multipoint tagged text>
  | <multilinestring tagged text>
  | <multipolygon tagged text>
  | <geometrycollection tagged text>
```

Each tagged text consists of a keyword, an optional dimension qualifier, and
a coordinate list enclosed in parentheses, or the keyword `EMPTY`.

### WKT keyword rules

Valid base keywords (case-insensitive): `POINT`, `LINESTRING`, `POLYGON`,
`MULTIPOINT`, `MULTILINESTRING`, `MULTIPOLYGON`, `GEOMETRYCOLLECTION`.

Dimension qualifiers (space-separated after keyword): `Z`, `M`, `ZM`.
Alternative forms `POINTZ`, `POINTM`, `POINTZM` are also valid.

```rust
/// The WKT keyword is one of the seven valid base geometry type names.
///
/// Valid values (case-insensitive): POINT, LINESTRING, POLYGON, MULTIPOINT,
/// MULTILINESTRING, MULTIPOLYGON, GEOMETRYCOLLECTION.
///
/// Source: OGC 06-103r4 §7.2 — WKT keywords
pub struct WktKeywordValid;
structural_prop!(WktKeywordValid, "WktKeywordValid");

/// WKT keywords are case-insensitive (POINT, Point, and point are all valid).
///
/// Source: OGC 06-103r4 §7.2 — WKT keywords
pub struct WktKeywordCaseInsensitive;
structural_prop!(WktKeywordCaseInsensitive, "WktKeywordCaseInsensitive");

/// A WKT dimension qualifier (Z, M, or ZM) follows the base keyword with a space.
///
/// Source: OGC 06-103r4 §7.2 — WKT dimension qualifier
pub struct WktDimensionTagPrecedesParens;
structural_prop!(WktDimensionTagPrecedesParens, "WktDimensionTagPrecedesParens");
```

### WKT coordinate formatting

Within a WKT position, ordinates are separated by spaces (not commas).
Between positions in a sequence, a comma is used as separator.

```rust
/// Within a WKT position, x and y (and z and m) are separated by a single space.
///
/// Example: "1.0 2.0" not "1.0,2.0".
///
/// Source: OGC 06-103r4 §7.2 — WKT coordinate format
pub struct WktCoordsSeparatedBySpace;
structural_prop!(WktCoordsSeparatedBySpace, "WktCoordsSeparatedBySpace");

/// Positions in a WKT sequence are separated by commas.
///
/// Example: "1.0 2.0, 3.0 4.0".
///
/// Source: OGC 06-103r4 §7.2 — WKT coordinate format
pub struct WktPositionsSeparatedByComma;
structural_prop!(WktPositionsSeparatedByComma, "WktPositionsSeparatedByComma");

/// The coordinate list in WKT is enclosed in parentheses.
///
/// Source: OGC 06-103r4 §7.2 — WKT coordinate list delimiters
pub struct WktCoordinateListInParens;
structural_prop!(WktCoordinateListInParens, "WktCoordinateListInParens");
```

### WKT per-type productions

```rust
/// WKT POINT production: POINT(x y) or POINT EMPTY.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT point tagged text
pub struct WktPointProduction;
structural_prop!(WktPointProduction, "WktPointProduction");

/// WKT LINESTRING production: LINESTRING(x1 y1, x2 y2, ...).
///
/// Requires at least two positions.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT linestring tagged text
pub struct WktLineStringProduction;
structural_prop!(WktLineStringProduction, "WktLineStringProduction");

/// WKT POLYGON production: POLYGON((exterior ring), (hole1), ...).
///
/// Each ring is a comma-separated sequence of positions enclosed in parentheses.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT polygon tagged text
pub struct WktPolygonProduction;
structural_prop!(WktPolygonProduction, "WktPolygonProduction");

/// Each ring in a WKT POLYGON is enclosed in its own pair of parentheses.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT polygon ring format
pub struct WktRingsParenthesized;
structural_prop!(WktRingsParenthesized, "WktRingsParenthesized");

/// Rings in a WKT POLYGON are separated by commas.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT polygon ring format
pub struct WktRingsSeparatedByComma;
structural_prop!(WktRingsSeparatedByComma, "WktRingsSeparatedByComma");

/// The exterior ring is the first ring listed in a WKT POLYGON.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT polygon tagged text
pub struct WktPolygonExteriorFirstRing;
structural_prop!(WktPolygonExteriorFirstRing, "WktPolygonExteriorFirstRing");

/// WKT MULTIPOINT production: MULTIPOINT((x1 y1), (x2 y2), ...) or MULTIPOINT(x1 y1, x2 y2, ...).
///
/// Both the parenthesized form and the bare coordinate form are accepted.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT multipoint tagged text
pub struct WktMultiPointProduction;
structural_prop!(WktMultiPointProduction, "WktMultiPointProduction");

/// WKT MULTILINESTRING production: MULTILINESTRING((x1 y1, x2 y2), (x3 y3, x4 y4), ...).
///
/// Source: OGC 06-103r4 §7.2.3 — WKT multilinestring tagged text
pub struct WktMultiLineStringProduction;
structural_prop!(WktMultiLineStringProduction, "WktMultiLineStringProduction");

/// WKT MULTIPOLYGON production: MULTIPOLYGON(((ext1), (hole1)), ((ext2)), ...).
///
/// Source: OGC 06-103r4 §7.2.3 — WKT multipolygon tagged text
pub struct WktMultiPolygonProduction;
structural_prop!(WktMultiPolygonProduction, "WktMultiPolygonProduction");

/// WKT GEOMETRYCOLLECTION production: GEOMETRYCOLLECTION(geom1, geom2, ...).
///
/// Each component is a complete tagged-text geometry.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT geometrycollection tagged text
pub struct WktGeometryCollectionProduction;
structural_prop!(WktGeometryCollectionProduction, "WktGeometryCollectionProduction");

/// Nested multi-type WKT wraps each component in parentheses consistent with its type.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT nesting for multi-types
pub struct WktNestingForMultiTypes;
structural_prop!(WktNestingForMultiTypes, "WktNestingForMultiTypes");
```

### WKT EMPTY keyword

```rust
/// An empty geometry is represented by the EMPTY keyword after the type name.
///
/// Examples: POINT EMPTY, LINESTRING EMPTY, POLYGON EMPTY.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT empty geometry
pub struct WktEmptyGeometry;
structural_prop!(WktEmptyGeometry, "WktEmptyGeometry");

/// WKT EMPTY applies to all seven geometry types.
///
/// Source: OGC 06-103r4 §7.2.3 — WKT empty geometry
pub struct WktEmptyApplicableToAllTypes;
structural_prop!(WktEmptyApplicableToAllTypes, "WktEmptyApplicableToAllTypes");
```

### WKT dimension variants

```rust
/// WKT Z variant: POINT Z(x y z) carries an elevation ordinate.
///
/// Source: OGC 06-103r4 §7.2.4 — WKT Z variant
pub struct WktZVariant;
structural_prop!(WktZVariant, "WktZVariant");

/// WKT M variant: POINT M(x y m) carries a measure ordinate.
///
/// Source: OGC 06-103r4 §7.2.4 — WKT M variant
pub struct WktMVariant;
structural_prop!(WktMVariant, "WktMVariant");

/// WKT ZM variant: POINT ZM(x y z m) carries both elevation and measure.
///
/// Source: OGC 06-103r4 §7.2.4 — WKT ZM variant
pub struct WktZMVariant;
structural_prop!(WktZMVariant, "WktZMVariant");
```

### WKT round-trip

```rust
/// Geometry → WKT → geometry produces an equivalent geometry (round-trip).
///
/// Equivalence: the two geometries are equal as point sets (same topological structure
/// and coordinate values within floating-point precision).
///
/// Source: OGC 06-103r4 §7.2 — WKT round-trip
pub struct WktRoundTrip;
structural_prop!(WktRoundTrip, "WktRoundTrip");
```

---

## §7.3 Well-Known Binary (WKB) Representation

OGC 06-103r4 §7.3 specifies the WKB format for compact binary serialization.
WKB is used primarily for storage and network transfer.

### WKB byte layout

The general WKB structure per geometry is:

```
Byte 0           : byte order (0x00 = XDR big-endian, 0x01 = NDR little-endian)
Bytes 1–4        : geometry type code (uint32 in declared byte order)
Bytes 5–end      : type-specific geometry data
```

For a **2D Point** (type code 1):

```
[1 byte order][4 type = 0x00000001][8 x double][8 y double]  = 21 bytes total
```

For a **2D LineString** (type code 2):

```
[1 byte order][4 type = 0x00000002][4 numPoints uint32][numPoints × 16 bytes]
```

For a **2D Polygon** (type code 3):

```
[1 byte order][4 type = 0x00000003][4 numRings uint32]
  [per ring: [4 numPoints uint32][numPoints × 16 bytes]]
```

For **Multi*** types (codes 4–6) and **GeometryCollection** (code 7):

```
[1 byte order][4 type code][4 numGeometries uint32]
  [per sub-geometry: complete WKB blob]
```

### WKB byte order props

```rust
/// WKB byte 0 is the byte order marker: 0x00 (XDR) or 0x01 (NDR).
///
/// Source: OGC 06-103r4 §7.3.1 — WKB byte order
pub struct WkbByteOrderMarkerPresent;
structural_prop!(WkbByteOrderMarkerPresent, "WkbByteOrderMarkerPresent");

/// WKB byte order 0x01 indicates little-endian (NDR) encoding.
///
/// Source: OGC 06-103r4 §7.3.1 — WKB byte order NDR
pub struct WkbByteOrderLittleEndian;
structural_prop!(WkbByteOrderLittleEndian, "WkbByteOrderLittleEndian");

/// WKB byte order 0x00 indicates big-endian (XDR) encoding.
///
/// Source: OGC 06-103r4 §7.3.1 — WKB byte order XDR
pub struct WkbByteOrderBigEndian;
structural_prop!(WkbByteOrderBigEndian, "WkbByteOrderBigEndian");

/// WKB byte order marker must be exactly 0x00 or 0x01; all other values are invalid.
///
/// Source: OGC 06-103r4 §7.3.1 — WKB byte order
pub struct WkbByteOrderMarkerTwoValues;
structural_prop!(WkbByteOrderMarkerTwoValues, "WkbByteOrderMarkerTwoValues");
```

### WKB type code props

```rust
/// The WKB type code is a uint32 encoded in the declared byte order.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodeValid;
structural_prop!(WkbTypeCodeValid, "WkbTypeCodeValid");

/// WKB type code 1 identifies a Point geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodePoint;
structural_prop!(WkbTypeCodePoint, "WkbTypeCodePoint");

/// WKB type code 2 identifies a LineString geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodeLineString;
structural_prop!(WkbTypeCodeLineString, "WkbTypeCodeLineString");

/// WKB type code 3 identifies a Polygon geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodePolygon;
structural_prop!(WkbTypeCodePolygon, "WkbTypeCodePolygon");

/// WKB type code 4 identifies a MultiPoint geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodeMultiPoint;
structural_prop!(WkbTypeCodeMultiPoint, "WkbTypeCodeMultiPoint");

/// WKB type code 5 identifies a MultiLineString geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodeMultiLineString;
structural_prop!(WkbTypeCodeMultiLineString, "WkbTypeCodeMultiLineString");

/// WKB type code 6 identifies a MultiPolygon geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodeMultiPolygon;
structural_prop!(WkbTypeCodeMultiPolygon, "WkbTypeCodeMultiPolygon");

/// WKB type code 7 identifies a GeometryCollection geometry.
///
/// Source: OGC 06-103r4 §7.3.2 — WKB type codes
pub struct WkbTypeCodeGeometryCollection;
structural_prop!(WkbTypeCodeGeometryCollection, "WkbTypeCodeGeometryCollection");
```

### WKB dimension variant type codes

Z variants add 1000 to the base code (ISO SQL/MM convention):

```rust
/// WKB Z variant type codes are in the range 1001–1007 (base + 1000).
///
/// Source: OGC 06-103r4 §7.3.2 — WKB Z variant
pub struct WkbZVariant;
structural_prop!(WkbZVariant, "WkbZVariant");

/// WKB M variant type codes are in the range 2001–2007 (base + 2000).
///
/// Source: OGC 06-103r4 §7.3.2 — WKB M variant
pub struct WkbMVariant;
structural_prop!(WkbMVariant, "WkbMVariant");

/// WKB ZM variant type codes are in the range 3001–3007 (base + 3000).
///
/// Source: OGC 06-103r4 §7.3.2 — WKB ZM variant
pub struct WkbZMVariant;
structural_prop!(WkbZMVariant, "WkbZMVariant");
```

### EWKB SRID extension (PostGIS / ISO SQL/MM)

The SRID-embedded WKB extension (EWKB) is widely implemented (PostGIS, etc.)
and documented in OGC 06-103r4 Annex C. The high bit `0x20000000` of the type
code signals that a 4-byte SRID follows immediately after the type code.

```rust
/// EWKB: when type code bit 0x20000000 is set, a 4-byte SRID follows the type code.
///
/// Source: OGC 06-103r4 Annex C — EWKB SRID extension
pub struct EwkbSridPresent;
structural_prop!(EwkbSridPresent, "EwkbSridPresent");

/// EWKB SRID field is a uint32 encoded in the declared byte order.
///
/// Source: OGC 06-103r4 Annex C — EWKB SRID extension
pub struct EwkbSridIsUint32;
structural_prop!(EwkbSridIsUint32, "EwkbSridIsUint32");

/// In EWKB, the base type code is recovered by masking off the 0x20000000 bit.
///
/// Source: OGC 06-103r4 Annex C — EWKB SRID extension
pub struct EwkbBaseTypeCodeMasked;
structural_prop!(EwkbBaseTypeCodeMasked, "EwkbBaseTypeCodeMasked");
```

### WKB coordinate encoding

```rust
/// Each WKB coordinate ordinate is an IEEE 754 double-precision float (8 bytes).
///
/// Source: OGC 06-103r4 §7.3.3 — WKB coordinate encoding
pub struct WkbCoordinateIsDouble;
structural_prop!(WkbCoordinateIsDouble, "WkbCoordinateIsDouble");

/// WKB coordinates are encoded in the byte order declared by the byte order marker.
///
/// Source: OGC 06-103r4 §7.3.3 — WKB coordinate encoding
pub struct WkbCoordinatesByteOrderMatches;
structural_prop!(WkbCoordinatesByteOrderMatches, "WkbCoordinatesByteOrderMatches");

/// A 2D WKB Point occupies exactly 21 bytes: 1 (byte order) + 4 (type) + 16 (x, y).
///
/// Source: OGC 06-103r4 §7.3 — WKB structure
pub struct WkbPointLength21Bytes2D;
structural_prop!(WkbPointLength21Bytes2D, "WkbPointLength21Bytes2D");

/// WKB coordinate count fields (numPoints, numRings, numGeometries) are uint32.
///
/// Source: OGC 06-103r4 §7.3 — WKB structure
pub struct WkbCountFieldsAreUint32;
structural_prop!(WkbCountFieldsAreUint32, "WkbCountFieldsAreUint32");
```

### WKB structural props

```rust
/// Each ring in a WKB Polygon is preceded by its uint32 point count.
///
/// Source: OGC 06-103r4 §7.3 — WKB polygon structure
pub struct WkbRingHasCount;
structural_prop!(WkbRingHasCount, "WkbRingHasCount");

/// Sub-geometries in a WKB Multi* or GeometryCollection are complete WKB blobs.
///
/// Each sub-geometry has its own byte order marker and type code.
///
/// Source: OGC 06-103r4 §7.3 — WKB collection structure
pub struct WkbSubGeometriesAreCompleteBlobs;
structural_prop!(WkbSubGeometriesAreCompleteBlobs, "WkbSubGeometriesAreCompleteBlobs");

/// WKB byte length is fully determined by the type code and the coordinate counts.
///
/// Source: OGC 06-103r4 §7.3 — WKB structure
pub struct WkbLengthValid;
structural_prop!(WkbLengthValid, "WkbLengthValid");
```

### WKB round-trip

```rust
/// Geometry → WKB → geometry produces an identical geometry (exact round-trip).
///
/// WKB uses binary IEEE 754 encoding so coordinate values are exact (no
/// decimal-conversion loss as in WKT).
///
/// Source: OGC 06-103r4 §7.3 — WKB round-trip
pub struct WkbRoundTrip;
structural_prop!(WkbRoundTrip, "WkbRoundTrip");
```

---

## Cross-Cutting Constraints

The following props apply across all geometry types and both serialization
formats. They capture invariants that emerge from the interaction of multiple
sections.

### SRS consistency

```rust
/// All component geometries in a collection share the same SRID.
///
/// A MultiPolygon where component Polygons carry different SRIDs is invalid.
///
/// Source: OGC 06-103r4 §4.1 and §6.1.3 — SRS consistency in collections
pub struct SrsConsistentInCollection;
structural_prop!(SrsConsistentInCollection, "SrsConsistentInCollection");

/// The SRID of a geometry is inherited from construction and does not change.
///
/// Source: OGC 06-103r4 §4.1 — SRID assignment
pub struct SridAssignedAtConstruction;
structural_prop!(SridAssignedAtConstruction, "SridAssignedAtConstruction");
```

### Coordinate dimensionality consistency

```rust
/// Coordinate dimensionality is uniform across all positions in a geometry,
/// including all positions in all sub-geometries of a collection.
///
/// Source: OGC 06-103r4 §6.1.2 — Dimensionality uniformity
pub struct CoordDimUniformInGeometry;
structural_prop!(CoordDimUniformInGeometry, "CoordDimUniformInGeometry");

/// A GeometryCollection's coordinate dimensionality matches all its components.
///
/// Source: OGC 06-103r4 §6.1.2 — Dimensionality uniformity
pub struct CollectionCoordDimMatchesComponents;
structural_prop!(CollectionCoordDimMatchesComponents, "CollectionCoordDimMatchesComponents");
```

### Empty geometry handling

```rust
/// Empty geometry handling is consistent across all type-specific operations.
///
/// Operations on empty geometries return defined values (0, empty geometry, or false)
/// rather than errors.
///
/// Source: OGC 06-103r4 §6.1 — Empty geometry semantics
pub struct EmptyHandlingConsistent;
structural_prop!(EmptyHandlingConsistent, "EmptyHandlingConsistent");

/// The boundary of an empty geometry is the empty geometry.
///
/// Source: OGC 06-103r4 §6.1 — Empty geometry boundary
pub struct EmptyGeometryBoundaryIsEmpty;
structural_prop!(EmptyGeometryBoundaryIsEmpty, "EmptyGeometryBoundaryIsEmpty");

/// The envelope of an empty geometry is the empty geometry.
///
/// Source: OGC 06-103r4 §6.1.1 — envelope() on empty geometry
pub struct EmptyGeometryEnvelopeIsEmpty;
structural_prop!(EmptyGeometryEnvelopeIsEmpty, "EmptyGeometryEnvelopeIsEmpty");
```

### Validity hierarchy

```rust
/// isValid() returning true for a composite geometry implies all its
/// sub-components also satisfy isValid().
///
/// Source: OGC 06-103r4 §6.1.3 — Validity hierarchy
pub struct IsValidImpliesSubComponentsValid;
structural_prop!(IsValidImpliesSubComponentsValid, "IsValidImpliesSubComponentsValid");

/// The dimension of a geometry collection equals the maximum dimension of its
/// component geometries.
///
/// Source: OGC 06-103r4 §4.2 — Dimension of collections
pub struct DimensionConsistencyInHierarchy;
structural_prop!(DimensionConsistencyInHierarchy, "DimensionConsistencyInHierarchy");
```

---

## §6.1.2 Constructive / Set Operations [GAP FILL]

OGC 06-103r4 §6.1.2 (the Geometry class interface table) specifies six
constructive operations that produce new geometry instances. These were absent
from the earlier §6.1.1 notes.

### buffer()

`buffer(distance : Double) : Geometry`

Expands (or contracts for negative distance) the geometry by `distance` in SRS
units.

```rust
/// buffer(distance) returns a geometry containing all points within distance of self.
///
/// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
pub struct BufferReturnsContainingGeometry;
structural_prop!(BufferReturnsContainingGeometry, "BufferReturnsContainingGeometry");

/// buffer(0) produces a result that contains self.
///
/// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
pub struct BufferZeroContainsSelf;
structural_prop!(BufferZeroContainsSelf, "BufferZeroContainsSelf");

/// buffer(d) with d > 0 produces a result with area >= area of self.
///
/// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
pub struct BufferPositiveDistanceIncreasesArea;
structural_prop!(BufferPositiveDistanceIncreasesArea, "BufferPositiveDistanceIncreasesArea");

/// buffer(d) with d < 0 contracts the geometry inward.
///
/// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
pub struct BufferNegativeDistanceShrinks;
structural_prop!(BufferNegativeDistanceShrinks, "BufferNegativeDistanceShrinks");

/// The result of buffer() is always a valid geometry.
///
/// Source: OGC 06-103r4 §6.1.2 — buffer(distance).
pub struct BufferResultIsValid;
structural_prop!(BufferResultIsValid, "BufferResultIsValid");
```

### convexHull()

`convexHull() : Geometry`

The smallest convex polygon (or lower-dimensional degenerate) that contains all
points of the geometry.

```rust
/// convexHull() returns the smallest convex geometry containing all points of self.
///
/// Source: OGC 06-103r4 §6.1.2 — convexHull().
pub struct ConvexHullReturnsSmallestConvex;
structural_prop!(ConvexHullReturnsSmallestConvex, "ConvexHullReturnsSmallestConvex");

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
structural_prop!(ConvexHullOfConvexPolygonIsSelf, "ConvexHullOfConvexPolygonIsSelf");
```

### intersection()

`intersection(another : Geometry) : Geometry`

Returns only the point set common to both geometries.

```rust
/// intersection(g) returns a geometry containing only points in both self and g.
///
/// Source: OGC 06-103r4 §6.1.2 — intersection(g).
pub struct IntersectionSubsetOfBothInputs;
structural_prop!(IntersectionSubsetOfBothInputs, "IntersectionSubsetOfBothInputs");

/// intersection is commutative: A.intersection(B) == B.intersection(A).
///
/// Source: OGC 06-103r4 §6.1.2 — intersection(g).
pub struct IntersectionIsCommutative;
structural_prop!(IntersectionIsCommutative, "IntersectionIsCommutative");

/// The intersection of two disjoint geometries is empty.
///
/// Source: OGC 06-103r4 §6.1.2 — intersection(g).
pub struct IntersectionOfDisjointIsEmpty;
structural_prop!(IntersectionOfDisjointIsEmpty, "IntersectionOfDisjointIsEmpty");

/// intersection(g) has dimension <= min(dim(self), dim(g)).
///
/// Source: OGC 06-103r4 §6.1.2 — intersection(g).
pub struct IntersectionDimensionAtMostMin;
structural_prop!(IntersectionDimensionAtMostMin, "IntersectionDimensionAtMostMin");
```

### union()

`union(another : Geometry) : Geometry`

```rust
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
```

### difference()

`difference(another : Geometry) : Geometry`

```rust
/// difference(g) is asymmetric: A.difference(B) != B.difference(A) in general.
///
/// Source: OGC 06-103r4 §6.1.2 — difference(g).
pub struct DifferenceAsymmetric;
structural_prop!(DifferenceAsymmetric, "DifferenceAsymmetric");

/// A.difference(B) and B are disjoint.
///
/// Source: OGC 06-103r4 §6.1.2 — difference(g).
pub struct DifferenceDisjointFromSubtracted;
structural_prop!(DifferenceDisjointFromSubtracted, "DifferenceDisjointFromSubtracted");

/// A.difference(B) is a subset of A.
///
/// Source: OGC 06-103r4 §6.1.2 — difference(g).
pub struct DifferenceSubsetOfSelf;
structural_prop!(DifferenceSubsetOfSelf, "DifferenceSubsetOfSelf");

/// difference(g) has dimension <= dim(self).
///
/// Source: OGC 06-103r4 §6.1.2 — difference(g).
pub struct DifferenceDimensionAtMostSelf;
structural_prop!(DifferenceDimensionAtMostSelf, "DifferenceDimensionAtMostSelf");
```

### symDifference()

`symDifference(another : Geometry) : Geometry`

```rust
/// symDifference is commutative: A.symDifference(B) == B.symDifference(A).
///
/// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
pub struct SymDifferenceIsCommutative;
structural_prop!(SymDifferenceIsCommutative, "SymDifferenceIsCommutative");

/// A.symDifference(B) == A.union(B).difference(A.intersection(B)).
///
/// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
pub struct SymDifferenceEqualsUnionMinusIntersection;
structural_prop!(SymDifferenceEqualsUnionMinusIntersection, "SymDifferenceEqualsUnionMinusIntersection");

/// A.union(B) == A.intersection(B).union(A.symDifference(B)).
///
/// Source: OGC 06-103r4 §6.1.2 — symDifference(g).
pub struct UnionEqualsIntersectionPlusSymDifference;
structural_prop!(UnionEqualsIntersectionPlusSymDifference, "UnionEqualsIntersectionPlusSymDifference");
```

---

## §6.1.4 boundary() Per-Type Semantics [GAP FILL]

OGC 06-103r4 §6.1.1 states that `boundary()` is defined per geometry subtype.
The earlier notes stated "defined per type" without listing the actual rules.

```rust
/// The boundary of a Point is the empty set (GeometryCollection EMPTY).
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for Point.
pub struct PointBoundaryIsEmpty;
structural_prop!(PointBoundaryIsEmpty, "PointBoundaryIsEmpty");

/// The boundary of a non-closed LineString is the MultiPoint of its two endpoints
/// (as a MultiPoint, not just a two-element set).
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for LineString.
pub struct LineStringNonClosedBoundaryIsEndpointMultiPoint;
structural_prop!(LineStringNonClosedBoundaryIsEndpointMultiPoint, "LineStringNonClosedBoundaryIsEndpointMultiPoint");

/// The boundary of a closed LinearRing is the empty set.
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for LinearRing.
pub struct LinearRingBoundaryIsEmptySet;
structural_prop!(LinearRingBoundaryIsEmptySet, "LinearRingBoundaryIsEmptySet");

/// The boundary of a Polygon is the MultiLineString of all its rings (exterior + holes).
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for Polygon.
pub struct PolygonBoundaryIsAllRings;
structural_prop!(PolygonBoundaryIsAllRings, "PolygonBoundaryIsAllRings");

/// The boundary of a MultiPolygon is the union of all component Polygon boundaries.
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for MultiPolygon.
pub struct MultiPolygonBoundaryIsAllRings;
structural_prop!(MultiPolygonBoundaryIsAllRings, "MultiPolygonBoundaryIsAllRings");

/// The boundary of a GeometryCollection follows the mod-2 rule for curve components.
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for GeometryCollection.
pub struct GeometryCollectionBoundaryMod2Rule;
structural_prop!(GeometryCollectionBoundaryMod2Rule, "GeometryCollectionBoundaryMod2Rule");

/// The boundary of a boundary is always empty: ∂(∂g) = ∅.
///
/// Source: OGC 06-103r4 §6.1.1 — topological axiom.
pub struct BoundaryOfBoundaryIsEmpty;
structural_prop!(BoundaryOfBoundaryIsEmpty, "BoundaryOfBoundaryIsEmpty");

/// The boundary of any empty geometry is itself an empty geometry.
///
/// Source: OGC 06-103r4 §6.1.1 — boundary() for empty inputs.
pub struct EmptyGeometryBoundaryIsEmptyGeometry;
structural_prop!(EmptyGeometryBoundaryIsEmptyGeometry, "EmptyGeometryBoundaryIsEmptyGeometry");
```

---

## §6.1.5 Per-Type Accessor Invariants [GAP FILL]

OGC 06-103r4 §6.1.1 and the per-type sections define accessor methods with
normative invariants.

### Point accessors

```rust
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
```

### LineString accessors

```rust
/// startPoint() == pointN(0) for a LineString.
///
/// Source: OGC 06-103r4 §6.1.1 — LineString.startPoint().
pub struct LineStringStartPointIsPointNZero;
structural_prop!(LineStringStartPointIsPointNZero, "LineStringStartPointIsPointNZero");

/// endPoint() == pointN(numPoints() - 1) for a LineString.
///
/// Source: OGC 06-103r4 §6.1.1 — LineString.endPoint().
pub struct LineStringEndPointIsPointNLast;
structural_prop!(LineStringEndPointIsPointNLast, "LineStringEndPointIsPointNLast");

/// pointN(i) is defined for i in 0..numPoints()-1.
///
/// Source: OGC 06-103r4 §6.1.1 — LineString.pointN(n).
pub struct LineStringPointNInRange;
structural_prop!(LineStringPointNInRange, "LineStringPointNInRange");

/// numPoints() equals the number of coordinate positions in the LineString.
///
/// Source: OGC 06-103r4 §6.1.1 — LineString.numPoints().
pub struct LineStringNumPointsMatchesCoordCount;
structural_prop!(LineStringNumPointsMatchesCoordCount, "LineStringNumPointsMatchesCoordCount");

/// isClosed() is true iff startPoint() coordinates equal endPoint() coordinates.
///
/// Source: OGC 06-103r4 §6.1.1 — LineString.isClosed().
pub struct LineStringIsClosedStartEqualsEnd;
structural_prop!(LineStringIsClosedStartEqualsEnd, "LineStringIsClosedStartEqualsEnd");

/// isRing() is true iff isClosed() AND isSimple().
///
/// Source: OGC 06-103r4 §6.1.1 — LineString.isRing().
pub struct LineStringIsRingImpliesClosedAndSimple;
structural_prop!(LineStringIsRingImpliesClosedAndSimple, "LineStringIsRingImpliesClosedAndSimple");
```

### Polygon accessors

```rust
/// exteriorRing() is never null for a non-empty Polygon.
///
/// Source: OGC 06-103r4 §6.1.1 — Polygon.exteriorRing().
pub struct PolygonExteriorRingNeverNull;
structural_prop!(PolygonExteriorRingNeverNull, "PolygonExteriorRingNeverNull");

/// interiorRingN(n) returns the nth hole (zero-indexed).
///
/// Source: OGC 06-103r4 §6.1.1 — Polygon.interiorRingN(n).
pub struct PolygonInteriorRingNReturnsHole;
structural_prop!(PolygonInteriorRingNReturnsHole, "PolygonInteriorRingNReturnsHole");

/// numInteriorRings() >= 0.
///
/// Source: OGC 06-103r4 §6.1.1 — Polygon.numInteriorRings().
pub struct PolygonNumInteriorRingsNonNegative;
structural_prop!(PolygonNumInteriorRingsNonNegative, "PolygonNumInteriorRingsNonNegative");

/// interiorRingN(i) is defined for i in 0..numInteriorRings()-1.
///
/// Source: OGC 06-103r4 §6.1.1 — Polygon.interiorRingN(n).
pub struct PolygonInteriorRingNInRange;
structural_prop!(PolygonInteriorRingNInRange, "PolygonInteriorRingNInRange");
```

### GeometryCollection accessors

```rust
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
structural_prop!(CollectionNumGeometriesNonNegative, "CollectionNumGeometriesNonNegative");

/// geometryN(i) is never null for any valid index.
///
/// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.geometryN(n).
pub struct CollectionGeometryNNeverNull;
structural_prop!(CollectionGeometryNNeverNull, "CollectionGeometryNNeverNull");
```

### Derived collection accessors

```rust
/// A MultiLineString isClosed() iff all component LineStrings are closed.
///
/// Source: OGC 06-103r4 §6.1.1 — MultiLineString.isClosed().
pub struct MultiLineStringIsClosedAllClosed2;
structural_prop!(MultiLineStringIsClosedAllClosed2, "MultiLineStringIsClosedAllClosed2");

/// A GeometryCollection isSimple() iff all components are simple.
///
/// Source: OGC 06-103r4 §6.1.1 — GeometryCollection.isSimple().
pub struct GeometryCollectionIsSimpleAllSimple;
structural_prop!(GeometryCollectionIsSimpleAllSimple, "GeometryCollectionIsSimpleAllSimple");
```

---

## §1.4 Conformance Classes [GAP FILL]

OGC 06-103r4 §1.4 defines two conformance classes that partition the
implementation requirements.

**Conformance Class 0 (CC0)** — Minimum: all seven geometry types, SRID
support, WKB and WKT serialization/deserialization.

**Conformance Class 1 (CC1)** — Full: CC0 plus DE-9IM spatial predicates
(§6.2), metric operations (§6.3), and constructive set operations (§6.1.2
interface table).

```rust
/// OGC SFS Conformance Class 0 requires support for all seven geometry types,
/// SRID assignment, and WKB/WKT serialization.
///
/// Source: OGC 06-103r4 §1.4 — Conformance Class 0.
pub struct ConformanceClass0RequiresSevenTypes;
structural_prop!(ConformanceClass0RequiresSevenTypes, "ConformanceClass0RequiresSevenTypes");

/// OGC SFS CC0 mandates WKB and WKT serialization/deserialization for all seven
/// geometry types.
///
/// Source: OGC 06-103r4 §1.4 — Conformance Class 0.
pub struct ConformanceClass0RequiresWkbWkt;
structural_prop!(ConformanceClass0RequiresWkbWkt, "ConformanceClass0RequiresWkbWkt");

/// OGC SFS Conformance Class 1 adds all DE-9IM spatial predicates (§6.2) and
/// metric operations (§6.3) over CC0.
///
/// Source: OGC 06-103r4 §1.4 — Conformance Class 1.
pub struct ConformanceClass1AddsPredicates;
structural_prop!(ConformanceClass1AddsPredicates, "ConformanceClass1AddsPredicates");

/// OGC SFS Conformance Class 1 adds constructive geometry operations: buffer,
/// convexHull, union, intersection, difference, symDifference.
///
/// Source: OGC 06-103r4 §1.4 — Conformance Class 1.
pub struct ConformanceClass1AddsSetOps;
structural_prop!(ConformanceClass1AddsSetOps, "ConformanceClass1AddsSetOps");
```

---

## Summary Table

The table below lists all props defined in this file, grouped by category.
Total: **261 props** (205 original + 56 gap fill).

| # | Prop | Section |
|---|------|---------|
| 1 | `GeometryHasSrs` | §4.1 SRS assignment |
| 2 | `GeometrySrsNotNull` | §4.1 SRS assignment |
| 3 | `GeometrySridReturnsInteger` | §4.1 SRS assignment |
| 4 | `GeometryDimensionMinus1WhenEmpty` | §4.2 Dimension |
| 5 | `GeometryDimension0ForPoint` | §4.2 Dimension |
| 6 | `GeometryDimension1ForLine` | §4.2 Dimension |
| 7 | `GeometryDimension2ForSurface` | §4.2 Dimension |
| 8 | `GeometryIsEmptyPredicate` | §4.3 Core predicates |
| 9 | `GeometryIsSimplePredicate` | §4.3 Core predicates |
| 10 | `GeometryIsValidPredicate` | §4.3 Core predicates |
| 11 | `GeometryIsSimpleAndIsValidDistinct` | §4.3 Core predicates |
| 12 | `GeometryEnvelopeReturnsMbr` | §4.4 Envelope and boundary |
| 13 | `GeometryBoundaryDefinedPerType` | §4.4 Envelope and boundary |
| 14 | `GeometryTypeReturnsString` | §4.4 Geometry type name |
| 15 | `GeometryTypeMatchesConcreteName` | §6.1.1 Geometry methods |
| 16 | `SridNonNegative` | §6.1.1 Geometry methods |
| 17 | `SridIsInteger` | §6.1.1 Geometry methods |
| 18 | `EnvelopeIsPolygon` | §6.1.1 envelope() |
| 19 | `EnvelopeIsPointWhenDegenerate` | §6.1.1 envelope() |
| 20 | `EnvelopeEmptyWhenGeometryEmpty` | §6.1.1 envelope() |
| 21 | `AsTextMethodDefined` | §6.1.1 Serialization |
| 22 | `AsBinaryMethodDefined` | §6.1.1 Serialization |
| 23 | `AsTextReturnsWkt` | §6.1.1 Serialization |
| 24 | `AsBinaryReturnsWkb` | §6.1.1 Serialization |
| 25 | `IsEmptyTrueForEmpty` | §6.1.1 isEmpty() |
| 26 | `IsEmptyFalseForNonEmpty` | §6.1.1 isEmpty() |
| 27 | `IsSimpleNoSelfIntersection` | §6.1.1 isSimple() |
| 28 | `IsValidWellFormed` | §6.1.1 isValid() |
| 29 | `Coord2DPosition` | §6.1.2 Coordinate dimensionality |
| 30 | `Coord3DPosition` | §6.1.2 Coordinate dimensionality |
| 31 | `Coord3DZIsElevation` | §6.1.2 Coordinate dimensionality |
| 32 | `Coord2DMPosition` | §6.1.2 Coordinate dimensionality |
| 33 | `CoordMIsMeasure` | §6.1.2 Coordinate dimensionality |
| 34 | `Coord3DMPosition` | §6.1.2 Coordinate dimensionality |
| 35 | `CoordDimensionalityUniform` | §6.1.2 Uniformity |
| 36 | `CoordXIsFinite` | §6.1.2 Uniformity |
| 37 | `CoordYIsFinite` | §6.1.2 Uniformity |
| 38 | `CoordZIsFiniteWhenPresent` | §6.1.2 Uniformity |
| 39 | `CoordMIsFiniteWhenPresent` | §6.1.2 Uniformity |
| 40 | `CoordDimensionalityIsConsistent` | §6.1.2 Uniformity |
| 41 | `PointAlwaysValid` | §6.1.3 Point validity |
| 42 | `PointEmptyHasNoCoords` | §6.1.3 Point validity |
| 43 | `PointXIsFinite` | §6.1.3 Point validity |
| 44 | `PointYIsFinite` | §6.1.3 Point validity |
| 45 | `PointZIsFiniteWhenPresent` | §6.1.3 Point validity |
| 46 | `PointEmptyIsEmpty` | §6.1.3 Point validity |
| 47 | `LineStringHasTwoOrMorePoints` | §6.1.3 LineString validity |
| 48 | `LineStringAdjacentPointsDistinct` | §6.1.3 LineString validity |
| 49 | `LineStringSimpleNoSelfIntersection` | §6.1.3 LineString validity |
| 50 | `LineStringClosedEqualsLinearRing` | §6.1.3 LineString validity |
| 51 | `LineStringOpenBoundaryTwoPoints` | §6.1.3 LineString validity |
| 52 | `LineStringClosedBoundaryEmpty` | §6.1.3 LineString validity |
| 53 | `LineStringMinimumTwoPositions` | §6.1.3 LineString validity |
| 54 | `LineStringBoundaryIsEndpoints` | §6.1.3 LineString validity |
| 55 | `LinearRingIsClosedLineString` | §6.1.3 LinearRing validity |
| 56 | `LinearRingMinimumFourPositions` | §6.1.3 LinearRing validity |
| 57 | `LinearRingIsSimple` | §6.1.3 LinearRing validity |
| 58 | `LinearRingNonDegenerate` | §6.1.3 LinearRing validity |
| 59 | `LinearRingFirstPositionEqualsLast` | §6.1.3 LinearRing validity |
| 60 | `LinearRingBoundaryIsEmpty` | §6.1.3 LinearRing validity |
| 61 | `PolygonExteriorIsLinearRing` | §6.1.3 Polygon validity |
| 62 | `PolygonInteriorRingsAreLinearRings` | §6.1.3 Polygon validity |
| 63 | `PolygonExteriorIsCCW` | §6.1.3 Polygon validity |
| 64 | `PolygonHolesAreCW` | §6.1.3 Polygon validity |
| 65 | `PolygonNoRingSelfIntersects` | §6.1.3 Polygon validity |
| 66 | `PolygonRingsDontCross` | §6.1.3 Polygon validity |
| 67 | `PolygonHolesInsideExterior` | §6.1.3 Polygon validity |
| 68 | `PolygonHolesDontContainEachOther` | §6.1.3 Polygon validity |
| 69 | `PolygonRingsTouchAtPoints` | §6.1.3 Polygon validity |
| 70 | `PolygonRingsDontTouchAlongSegment` | §6.1.3 Polygon validity |
| 71 | `PolygonHasExactlyOneExteriorRing` | §6.1.3 Polygon validity |
| 72 | `PolygonHoleCountNonNegative` | §6.1.3 Polygon validity |
| 73 | `MultiPointComponentsArePoints` | §6.1.3 MultiPoint validity |
| 74 | `MultiPointMayBeEmpty` | §6.1.3 MultiPoint validity |
| 75 | `MultiPointSimpleWhenNoTwoEqual` | §6.1.3 MultiPoint validity |
| 76 | `MultiLineStringComponentsAreLineStrings` | §6.1.3 MultiLineString validity |
| 77 | `MultiLineStringMayBeEmpty` | §6.1.3 MultiLineString validity |
| 78 | `MultiLineStringSimpleWhenIntersectAtEndpointsOnly` | §6.1.3 MultiLineString validity |
| 79 | `MultiPolygonComponentsArePolygons` | §6.1.3 MultiPolygon validity |
| 80 | `MultiPolygonInteriorsDisjoint` | §6.1.3 MultiPolygon validity |
| 81 | `MultiPolygonBoundariesTouchAtPoints` | §6.1.3 MultiPolygon validity |
| 82 | `MultiPolygonBoundariesDontOverlap` | §6.1.3 MultiPolygon validity |
| 83 | `MultiPolygonMayBeEmpty` | §6.1.3 MultiPolygon validity |
| 84 | `GeometryCollectionAllComponentsValid` | §6.1.3 GeometryCollection validity |
| 85 | `GeometryCollectionHeterogeneousAllowed` | §6.1.3 GeometryCollection validity |
| 86 | `GeometryCollectionMayBeEmpty` | §6.1.3 GeometryCollection validity |
| 87 | `De9ImMatrixDefined` | §6.2 DE-9IM |
| 88 | `De9ImCellValues` | §6.2 DE-9IM |
| 89 | `De9ImInteriorBoundaryExteriorAxes` | §6.2 DE-9IM |
| 90 | `De9ImRelatePatternIsNineChars` | §6.2 DE-9IM |
| 91 | `EqualsDe9ImPattern` | §6.2.1 Equals |
| 92 | `EqualsIsSymmetric` | §6.2.1 Equals |
| 93 | `EqualsIsReflexive` | §6.2.1 Equals |
| 94 | `DisjointDe9ImPattern` | §6.2.2 Disjoint |
| 95 | `DisjointIsSymmetric` | §6.2.2 Disjoint |
| 96 | `DisjointIsInverseOfIntersects` | §6.2.2 Disjoint |
| 97 | `IntersectsIsNotDisjoint` | §6.2.3 Intersects |
| 98 | `IntersectsIsSymmetric` | §6.2.3 Intersects |
| 99 | `IntersectsAtLeastOneSharedPoint` | §6.2.3 Intersects |
| 100 | `TouchesDe9ImPattern` | §6.2.4 Touches |
| 101 | `TouchesIsSymmetric` | §6.2.4 Touches |
| 102 | `TouchesInteriorsDisjoint` | §6.2.4 Touches |
| 103 | `CrossesDe9ImPattern` | §6.2.5 Crosses |
| 104 | `CrossesDimIntersectionLessThanMax` | §6.2.5 Crosses |
| 105 | `CrossesIsNotSymmetric` | §6.2.5 Crosses |
| 106 | `CrossesDimensionConstraint` | §6.2.5 Crosses |
| 107 | `WithinDe9ImPattern` | §6.2.6 Within |
| 108 | `WithinIsInverseOfContains` | §6.2.6 Within |
| 109 | `WithinIsNotSymmetric` | §6.2.6 Within |
| 110 | `ContainsDe9ImPattern` | §6.2.7 Contains |
| 111 | `ContainsIsInverseOfWithin` | §6.2.7 Contains |
| 112 | `ContainsIsNotSymmetric` | §6.2.7 Contains |
| 113 | `OverlapsDe9ImPattern2D` | §6.2.8 Overlaps |
| 114 | `OverlapsDe9ImPatternLowDim` | §6.2.8 Overlaps |
| 115 | `OverlapsIsSymmetric` | §6.2.8 Overlaps |
| 116 | `OverlapsDimensionConstraint` | §6.2.8 Overlaps |
| 117 | `CoversDe9ImPattern` | §6.2 Annex A — Covers |
| 118 | `CoveredByIsInverseOfCovers` | §6.2 Annex A — CoveredBy |
| 119 | `CoversImpliesWithin` | §6.2 Annex A — Covers |
| 120 | `RelatePatternLength9` | §6.2.9 Relate |
| 121 | `RelatePatternCharacters` | §6.2.9 Relate |
| 122 | `RelateReturnsBooleanMatchingPattern` | §6.2.9 Relate |
| 123 | `Area0DReturnsZero` | §6.3 area() |
| 124 | `Area1DReturnsZero` | §6.3 area() |
| 125 | `Area2DReturnsPositive` | §6.3 area() |
| 126 | `AreaNonNegative` | §6.3 area() |
| 127 | `AreaUnitsSquaredSrsUnits` | §6.3 area() |
| 128 | `AreaEmptyGeometryIsZero` | §6.3 area() |
| 129 | `Length0DReturnsZero` | §6.3 length() |
| 130 | `Length2DReturnsZero` | §6.3 length() |
| 131 | `Length1DReturnsPositive` | §6.3 length() |
| 132 | `LengthNonNegative` | §6.3 length() |
| 133 | `LengthInSrsUnits` | §6.3 length() |
| 134 | `LengthEmptyGeometryIsZero` | §6.3 length() |
| 135 | `DistanceNonNegative` | §6.3 distance() |
| 136 | `DistanceZeroIffIntersecting` | §6.3 distance() |
| 137 | `DistanceSymmetric` | §6.3 distance() |
| 138 | `DistanceInSrsUnits` | §6.3 distance() |
| 139 | `DistanceTriangleInequality` | §6.3 distance() |
| 140 | `CentroidReturnsPoint` | §6.3 centroid() |
| 141 | `CentroidDefinedForAll` | §6.3 centroid() |
| 142 | `CentroidXIsFinite` | §6.3 centroid() |
| 143 | `CentroidYIsFinite` | §6.3 centroid() |
| 144 | `CentroidWithinConvexHull` | §6.3 centroid() |
| 145 | `PointOnSurfaceReturnsPoint` | §6.3 pointOnSurface() |
| 146 | `PointOnSurfaceIsOnGeometry` | §6.3 pointOnSurface() |
| 147 | `PointOnSurfaceDefinedForAll` | §6.3 pointOnSurface() |
| 148 | `WktKeywordValid` | §7.2 WKT keywords |
| 149 | `WktKeywordCaseInsensitive` | §7.2 WKT keywords |
| 150 | `WktDimensionTagPrecedesParens` | §7.2 WKT dimension qualifier |
| 151 | `WktCoordsSeparatedBySpace` | §7.2 WKT coordinate format |
| 152 | `WktPositionsSeparatedByComma` | §7.2 WKT coordinate format |
| 153 | `WktCoordinateListInParens` | §7.2 WKT delimiters |
| 154 | `WktPointProduction` | §7.2.3 WKT Point |
| 155 | `WktLineStringProduction` | §7.2.3 WKT LineString |
| 156 | `WktPolygonProduction` | §7.2.3 WKT Polygon |
| 157 | `WktRingsParenthesized` | §7.2.3 WKT Polygon rings |
| 158 | `WktRingsSeparatedByComma` | §7.2.3 WKT Polygon rings |
| 159 | `WktPolygonExteriorFirstRing` | §7.2.3 WKT Polygon |
| 160 | `WktMultiPointProduction` | §7.2.3 WKT MultiPoint |
| 161 | `WktMultiLineStringProduction` | §7.2.3 WKT MultiLineString |
| 162 | `WktMultiPolygonProduction` | §7.2.3 WKT MultiPolygon |
| 163 | `WktGeometryCollectionProduction` | §7.2.3 WKT GeometryCollection |
| 164 | `WktNestingForMultiTypes` | §7.2.3 WKT nesting |
| 165 | `WktEmptyGeometry` | §7.2.3 WKT EMPTY |
| 166 | `WktEmptyApplicableToAllTypes` | §7.2.3 WKT EMPTY |
| 167 | `WktZVariant` | §7.2.4 WKT Z variant |
| 168 | `WktMVariant` | §7.2.4 WKT M variant |
| 169 | `WktZMVariant` | §7.2.4 WKT ZM variant |
| 170 | `WktRoundTrip` | §7.2 WKT round-trip |
| 171 | `WkbByteOrderMarkerPresent` | §7.3.1 WKB byte order |
| 172 | `WkbByteOrderLittleEndian` | §7.3.1 WKB byte order NDR |
| 173 | `WkbByteOrderBigEndian` | §7.3.1 WKB byte order XDR |
| 174 | `WkbByteOrderMarkerTwoValues` | §7.3.1 WKB byte order |
| 175 | `WkbTypeCodeValid` | §7.3.2 WKB type codes |
| 176 | `WkbTypeCodePoint` | §7.3.2 WKB type codes |
| 177 | `WkbTypeCodeLineString` | §7.3.2 WKB type codes |
| 178 | `WkbTypeCodePolygon` | §7.3.2 WKB type codes |
| 179 | `WkbTypeCodeMultiPoint` | §7.3.2 WKB type codes |
| 180 | `WkbTypeCodeMultiLineString` | §7.3.2 WKB type codes |
| 181 | `WkbTypeCodeMultiPolygon` | §7.3.2 WKB type codes |
| 182 | `WkbTypeCodeGeometryCollection` | §7.3.2 WKB type codes |
| 183 | `WkbZVariant` | §7.3.2 WKB Z variant |
| 184 | `WkbMVariant` | §7.3.2 WKB M variant |
| 185 | `WkbZMVariant` | §7.3.2 WKB ZM variant |
| 186 | `EwkbSridPresent` | §7.3 Annex C EWKB |
| 187 | `EwkbSridIsUint32` | §7.3 Annex C EWKB |
| 188 | `EwkbBaseTypeCodeMasked` | §7.3 Annex C EWKB |
| 189 | `WkbCoordinateIsDouble` | §7.3.3 WKB coordinates |
| 190 | `WkbCoordinatesByteOrderMatches` | §7.3.3 WKB coordinates |
| 191 | `WkbPointLength21Bytes2D` | §7.3 WKB structure |
| 192 | `WkbCountFieldsAreUint32` | §7.3 WKB structure |
| 193 | `WkbRingHasCount` | §7.3 WKB Polygon |
| 194 | `WkbSubGeometriesAreCompleteBlobs` | §7.3 WKB collections |
| 195 | `WkbLengthValid` | §7.3 WKB structure |
| 196 | `WkbRoundTrip` | §7.3 WKB round-trip |
| 197 | `SrsConsistentInCollection` | Cross-cutting SRS |
| 198 | `SridAssignedAtConstruction` | Cross-cutting SRS |
| 199 | `CoordDimUniformInGeometry` | Cross-cutting dimensionality |
| 200 | `CollectionCoordDimMatchesComponents` | Cross-cutting dimensionality |
| 201 | `EmptyHandlingConsistent` | Cross-cutting empty |
| 202 | `EmptyGeometryBoundaryIsEmpty` | Cross-cutting empty |
| 203 | `EmptyGeometryEnvelopeIsEmpty` | Cross-cutting empty |
| 204 | `IsValidImpliesSubComponentsValid` | Cross-cutting validity |
| 205 | `DimensionConsistencyInHierarchy` | Cross-cutting dimension |

---

*End of OGC Simple Features Specification Contract Implementation Notes.*
*OGC 06-103r4 / ISO 19125-1:2004 — 205 props across 9 categories.*
