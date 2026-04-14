# RFC 7946 GeoJSON Contract Implementation Notes

## Standard Reference

**Standard:** RFC 7946 — The GeoJSON Format  
**Published:** August 2016  
**Authors:** H. Butler, M. Daly, A. Doyle, S. Gillies, S. Hagen, T. Schaub  
**URL:** <https://www.rfc-editor.org/rfc/rfc7946>  
**Obsoletes:** GeoJSON 2008 specification (geojson.org)

## ⚠ Correct prop pattern

Do **not** use `#[derive(Prop)]` or `#[spec_reference(...)]` — those attributes do not exist.

The correct pattern (from `crates/elicit_db/src/contracts/iso_sql.rs`):

```rust
mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Brief description of the proposition.
    ///
    /// Source: RFC 7946 §X.Y — <exact section title>
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

Use this file as a content reference. All snippets below are ready to paste into
`crates/elicit_gis/src/contracts/rfc7946.rs`.

---

## §2 — GeoJSON Text

A GeoJSON text is a single serialised JSON value. The root value MUST be a JSON
object (not an array, string, number, boolean, or null). This section establishes
the outermost envelope contract before any type dispatch.

```rust
/// A GeoJSON text consists of exactly one JSON value.
///
/// Source: RFC 7946 §2 — GeoJSON Text
pub struct GeoJsonTextIsSingleJsonValue;
structural_prop!(GeoJsonTextIsSingleJsonValue, "GeoJsonTextIsSingleJsonValue");

/// The root JSON value of a GeoJSON text is a JSON object.
///
/// Source: RFC 7946 §2 — GeoJSON Text
pub struct GeoJsonRootIsObject;
structural_prop!(GeoJsonRootIsObject, "GeoJsonRootIsObject");
```

---

## §3 — GeoJSON Object

Every GeoJSON object must carry a `type` member whose value discriminates among the
nine permitted types. The `bbox` member is optional. Foreign (unknown) members at any
level should be silently ignored by conformant parsers.

### §3 — `type` member constraints

```rust
/// Every GeoJSON object has a member named "type".
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonObjectHasTypeMember;
structural_prop!(GeoJsonObjectHasTypeMember, "GeoJsonObjectHasTypeMember");

/// The value of the "type" member is a JSON string (not number, boolean, array, or object).
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonTypeMemberIsString;
structural_prop!(GeoJsonTypeMemberIsString, "GeoJsonTypeMemberIsString");

/// The value of the "type" member is not JSON null.
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonTypeMemberIsNotNull;
structural_prop!(GeoJsonTypeMemberIsNotNull, "GeoJsonTypeMemberIsNotNull");

/// The "type" string is case-sensitive and must be matched exactly as defined.
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonTypeIsCaseSensitive;
structural_prop!(GeoJsonTypeIsCaseSensitive, "GeoJsonTypeIsCaseSensitive");

/// The "type" value is one of the nine defined strings: Point, MultiPoint,
/// LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection,
/// Feature, or FeatureCollection.
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonTypeIsOneOfNineValues;
structural_prop!(GeoJsonTypeIsOneOfNineValues, "GeoJsonTypeIsOneOfNineValues");

/// The "type" value may not be extended with values not defined in RFC 7946.
///
/// Source: RFC 7946 §3 — GeoJSON Object / §7 — Extending GeoJSON
pub struct GeoJsonTypeIsNotExtensible;
structural_prop!(GeoJsonTypeIsNotExtensible, "GeoJsonTypeIsNotExtensible");
```

### §3 — `bbox` and foreign-member constraints

```rust
/// The "bbox" member is optional; its absence is valid at every GeoJSON level.
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonBboxMemberIsOptional;
structural_prop!(GeoJsonBboxMemberIsOptional, "GeoJsonBboxMemberIsOptional");

/// When the "bbox" member is present its value is a JSON array.
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonBboxWhenPresentIsArray;
structural_prop!(GeoJsonBboxWhenPresentIsArray, "GeoJsonBboxWhenPresentIsArray");

/// Foreign members (members with names not defined in RFC 7946) SHOULD be ignored
/// by parsers that do not understand them.
///
/// Source: RFC 7946 §3 — GeoJSON Object
pub struct GeoJsonForeignMembersShouldBeIgnored;
structural_prop!(GeoJsonForeignMembersShouldBeIgnored, "GeoJsonForeignMembersShouldBeIgnored");
```

---

## §3.1.1 — Position

A position is the fundamental building block of every geometry type.  It is
represented as a JSON array (never an object or scalar).  The array MUST contain at
least two elements (longitude, latitude) and MAY contain a third (altitude).  Extra
elements beyond index 2 are explicitly permitted but ignored; implementations SHOULD
NOT produce them.  All values must be JSON numbers (IEEE 754 doubles).

RFC 7946 mandates the **right-hand Cartesian** interpolation model: the straight line
connecting two positions is a straight line in Cartesian space, not a geodesic arc.

### §3.1.1 — Array type constraints

```rust
/// A position is a JSON array, not an object, number, string, boolean, or null.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionIsJsonArray;
structural_prop!(PositionIsJsonArray, "PositionIsJsonArray");

/// A position is not a JSON object.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionIsNotJsonObject;
structural_prop!(PositionIsNotJsonObject, "PositionIsNotJsonObject");

/// A position is not JSON null.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionIsNotJsonNull;
structural_prop!(PositionIsNotJsonNull, "PositionIsNotJsonNull");

/// A position is not a JSON string.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionIsNotJsonString;
structural_prop!(PositionIsNotJsonString, "PositionIsNotJsonString");

/// A position is not a bare JSON number (it is an array that contains numbers).
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionIsNotJsonNumber;
structural_prop!(PositionIsNotJsonNumber, "PositionIsNotJsonNumber");
```

### §3.1.1 — Element count and type constraints

```rust
/// A position array has at least two elements; a single-element or empty array is invalid.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionHasAtLeastTwoElements;
structural_prop!(PositionHasAtLeastTwoElements, "PositionHasAtLeastTwoElements");

/// Every element within a position array is a JSON number.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionElementsAreJsonNumbers;
structural_prop!(PositionElementsAreJsonNumbers, "PositionElementsAreJsonNumbers");
```

### §3.1.1 — Coordinate semantics and ranges

```rust
/// The first element (index 0) of a position is longitude in decimal degrees.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionElementZeroIsLongitude;
structural_prop!(PositionElementZeroIsLongitude, "PositionElementZeroIsLongitude");

/// The second element (index 1) of a position is latitude in decimal degrees.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionElementOneIsLatitude;
structural_prop!(PositionElementOneIsLatitude, "PositionElementOneIsLatitude");

/// Longitude is in the range [-180, 180] inclusive.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionLongitudeInRange;
structural_prop!(PositionLongitudeInRange, "PositionLongitudeInRange");

/// Latitude is in the range [-90, 90] inclusive.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionLatitudeInRange;
structural_prop!(PositionLatitudeInRange, "PositionLatitudeInRange");

/// Longitude is a finite number (not NaN or +/-Infinity).
///
/// Source: RFC 7946 §3.1.1 — Position / §11.1 — I-JSON
pub struct PositionLongitudeIsFinite;
structural_prop!(PositionLongitudeIsFinite, "PositionLongitudeIsFinite");

/// Latitude is a finite number (not NaN or +/-Infinity).
///
/// Source: RFC 7946 §3.1.1 — Position / §11.1 — I-JSON
pub struct PositionLatitudeIsFinite;
structural_prop!(PositionLatitudeIsFinite, "PositionLatitudeIsFinite");
```

### §3.1.1 — Altitude (third element)

```rust
/// The third element (index 2), when present, is altitude above the WGS 84 reference
/// ellipsoid expressed in metres.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionElementTwoIsAltitudeWhenPresent;
structural_prop!(PositionElementTwoIsAltitudeWhenPresent, "PositionElementTwoIsAltitudeWhenPresent");

/// Altitude references the WGS 84 ellipsoid, not the geoid or mean sea level.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionAltitudeReferencesWgs84Ellipsoid;
structural_prop!(PositionAltitudeReferencesWgs84Ellipsoid, "PositionAltitudeReferencesWgs84Ellipsoid");

/// A position SHOULD NOT have more than three elements.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionShouldNotExceedThreeElements;
structural_prop!(PositionShouldNotExceedThreeElements, "PositionShouldNotExceedThreeElements");

/// Elements beyond index 2 are ignored by conformant implementations.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionExtraElementsBeyondThreeIgnored;
structural_prop!(PositionExtraElementsBeyondThreeIgnored, "PositionExtraElementsBeyondThreeIgnored");
```

### §3.1.1 — Interpolation model

```rust
/// The line segment between two positions is a straight Cartesian line, not a geodesic arc.
///
/// Source: RFC 7946 §3.1.1 — Position
pub struct PositionInterpolationIsCartesian;
structural_prop!(PositionInterpolationIsCartesian, "PositionInterpolationIsCartesian");
```

---

## §3.1.2 — Point

A Point geometry carries a single position in `coordinates`.  The coordinates value
is a flat position array (longitude, latitude[, altitude]) — not an array of
positions.

```rust
/// The "type" member of a Point geometry equals the string "Point".
///
/// Source: RFC 7946 §3.1.2 — Point
pub struct PointTypeEqualsPoint;
structural_prop!(PointTypeEqualsPoint, "PointTypeEqualsPoint");

/// A Point geometry has a "coordinates" member.
///
/// Source: RFC 7946 §3.1.2 — Point
pub struct PointHasCoordinatesMember;
structural_prop!(PointHasCoordinatesMember, "PointHasCoordinatesMember");

/// The "coordinates" value of a Point is a single position array (not an array of positions).
///
/// Source: RFC 7946 §3.1.2 — Point
pub struct PointCoordinatesIsSinglePosition;
structural_prop!(PointCoordinatesIsSinglePosition, "PointCoordinatesIsSinglePosition");

/// The "coordinates" of a Point is not a nested (doubly-wrapped) array.
///
/// Source: RFC 7946 §3.1.2 — Point
pub struct PointCoordinatesIsNotNestedArray;
structural_prop!(PointCoordinatesIsNotNestedArray, "PointCoordinatesIsNotNestedArray");

/// The "coordinates" of a Point has at least two numeric elements.
///
/// Source: RFC 7946 §3.1.2 — Point
pub struct PointCoordinatesHasMinTwoElements;
structural_prop!(PointCoordinatesHasMinTwoElements, "PointCoordinatesHasMinTwoElements");

/// The "coordinates" value of a Point is not null.
///
/// Source: RFC 7946 §3.1.2 — Point
pub struct PointCoordinatesIsNotNull;
structural_prop!(PointCoordinatesIsNotNull, "PointCoordinatesIsNotNull");
```

---

## §3.1.3 — MultiPoint

A MultiPoint is an array of zero or more positions.  Each element must be a valid
position.  An empty MultiPoint is permitted.

```rust
/// The "type" member of a MultiPoint geometry equals the string "MultiPoint".
///
/// Source: RFC 7946 §3.1.3 — MultiPoint
pub struct MultiPointTypeEqualsMultiPoint;
structural_prop!(MultiPointTypeEqualsMultiPoint, "MultiPointTypeEqualsMultiPoint");

/// A MultiPoint geometry has a "coordinates" member.
///
/// Source: RFC 7946 §3.1.3 — MultiPoint
pub struct MultiPointHasCoordinatesMember;
structural_prop!(MultiPointHasCoordinatesMember, "MultiPointHasCoordinatesMember");

/// The "coordinates" of a MultiPoint is an array of position arrays.
///
/// Source: RFC 7946 §3.1.3 — MultiPoint
pub struct MultiPointCoordinatesIsArrayOfPositions;
structural_prop!(MultiPointCoordinatesIsArrayOfPositions, "MultiPointCoordinatesIsArrayOfPositions");

/// Each element in a MultiPoint "coordinates" array is a valid position.
///
/// Source: RFC 7946 §3.1.3 — MultiPoint
pub struct MultiPointEachElementIsValidPosition;
structural_prop!(MultiPointEachElementIsValidPosition, "MultiPointEachElementIsValidPosition");

/// A MultiPoint "coordinates" array may be empty.
///
/// Source: RFC 7946 §3.1.3 — MultiPoint
pub struct MultiPointCoordinatesMayBeEmpty;
structural_prop!(MultiPointCoordinatesMayBeEmpty, "MultiPointCoordinatesMayBeEmpty");

/// The "coordinates" value of a MultiPoint is not null.
///
/// Source: RFC 7946 §3.1.3 — MultiPoint
pub struct MultiPointCoordinatesIsNotNull;
structural_prop!(MultiPointCoordinatesIsNotNull, "MultiPointCoordinatesIsNotNull");
```

---

## §3.1.4 — LineString

A LineString connects two or more positions with straight Cartesian line segments.
The `coordinates` array must contain at least two positions; a one-element or empty
array is invalid.

```rust
/// The "type" member of a LineString geometry equals the string "LineString".
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringTypeEqualsLineString;
structural_prop!(LineStringTypeEqualsLineString, "LineStringTypeEqualsLineString");

/// A LineString geometry has a "coordinates" member.
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringHasCoordinatesMember;
structural_prop!(LineStringHasCoordinatesMember, "LineStringHasCoordinatesMember");

/// The "coordinates" value of a LineString is not null.
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringCoordinatesIsNotNull;
structural_prop!(LineStringCoordinatesIsNotNull, "LineStringCoordinatesIsNotNull");

/// The "coordinates" value of a LineString is a JSON array.
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringCoordinatesIsArray;
structural_prop!(LineStringCoordinatesIsArray, "LineStringCoordinatesIsArray");

/// A LineString "coordinates" array has at least two positions.
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringCoordinatesHasMinTwoPositions;
structural_prop!(LineStringCoordinatesHasMinTwoPositions, "LineStringCoordinatesHasMinTwoPositions");

/// Each element in a LineString "coordinates" array is a valid position.
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringEachElementIsValidPosition;
structural_prop!(LineStringEachElementIsValidPosition, "LineStringEachElementIsValidPosition");

/// A LineString "coordinates" array is never empty (minimum two positions required).
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringCoordinatesIsNotEmpty;
structural_prop!(LineStringCoordinatesIsNotEmpty, "LineStringCoordinatesIsNotEmpty");

/// The minimum two positions of a LineString form a directed path with at least one segment.
///
/// Source: RFC 7946 §3.1.4 — LineString
pub struct LineStringMinTwoPositionsFormPath;
structural_prop!(LineStringMinTwoPositionsFormPath, "LineStringMinTwoPositionsFormPath");
```

---

## §3.1.5 — MultiLineString

A MultiLineString is an array of LineString coordinate arrays.  Each constituent
LineString must independently satisfy the two-position minimum.  The outer array may
be empty.

```rust
/// The "type" member of a MultiLineString geometry equals the string "MultiLineString".
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringTypeEqualsMultiLineString;
structural_prop!(MultiLineStringTypeEqualsMultiLineString, "MultiLineStringTypeEqualsMultiLineString");

/// A MultiLineString geometry has a "coordinates" member.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringHasCoordinatesMember;
structural_prop!(MultiLineStringHasCoordinatesMember, "MultiLineStringHasCoordinatesMember");

/// The "coordinates" value of a MultiLineString is not null.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringCoordinatesIsNotNull;
structural_prop!(MultiLineStringCoordinatesIsNotNull, "MultiLineStringCoordinatesIsNotNull");

/// The "coordinates" value of a MultiLineString is a JSON array.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringCoordinatesIsArray;
structural_prop!(MultiLineStringCoordinatesIsArray, "MultiLineStringCoordinatesIsArray");

/// Each element in a MultiLineString "coordinates" array is itself an array.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringEachElementIsArray;
structural_prop!(MultiLineStringEachElementIsArray, "MultiLineStringEachElementIsArray");

/// Each LineString within a MultiLineString has at least two positions.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringEachLineStringHasMinTwoPositions;
structural_prop!(MultiLineStringEachLineStringHasMinTwoPositions, "MultiLineStringEachLineStringHasMinTwoPositions");

/// Each position element within a MultiLineString LineString component is a valid position.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringEachPositionIsValid;
structural_prop!(MultiLineStringEachPositionIsValid, "MultiLineStringEachPositionIsValid");

/// The outer "coordinates" array of a MultiLineString may be empty.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString
pub struct MultiLineStringMayBeEmpty;
structural_prop!(MultiLineStringMayBeEmpty, "MultiLineStringMayBeEmpty");
```

---

## §3.1.6 — Polygon

Polygon is the richest geometry type in RFC 7946.  Its `coordinates` is an array of
linear rings.  Each linear ring is a closed LineString with at least 4 positions
where the first and last are identical.  Winding order is mandated by the right-hand
rule: exterior ring counter-clockwise, interior (hole) rings clockwise.

### §3.1.6 — Basic Polygon structure

```rust
/// The "type" member of a Polygon geometry equals the string "Polygon".
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonTypeEqualsPolygon;
structural_prop!(PolygonTypeEqualsPolygon, "PolygonTypeEqualsPolygon");

/// A Polygon geometry has a "coordinates" member.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonHasCoordinatesMember;
structural_prop!(PolygonHasCoordinatesMember, "PolygonHasCoordinatesMember");

/// The "coordinates" value of a Polygon is an array of linear ring arrays.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonCoordinatesIsArrayOfRings;
structural_prop!(PolygonCoordinatesIsArrayOfRings, "PolygonCoordinatesIsArrayOfRings");
```

### §3.1.6 — Linear ring constraints

```rust
/// A linear ring is a closed LineString: the first and last positions are identical.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonLinearRingIsClosedLineString;
structural_prop!(PolygonLinearRingIsClosedLineString, "PolygonLinearRingIsClosedLineString");

/// A linear ring has at least four positions (three distinct vertices plus the repeated first).
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonLinearRingHasMinFourPositions;
structural_prop!(PolygonLinearRingHasMinFourPositions, "PolygonLinearRingHasMinFourPositions");

/// The first and last positions of a linear ring MUST contain identical coordinate values.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonLinearRingFirstAndLastAreIdentical;
structural_prop!(PolygonLinearRingFirstAndLastAreIdentical, "PolygonLinearRingFirstAndLastAreIdentical");

/// The first and last positions of a linear ring SHOULD be represented by the exact same
/// JSON array (not merely numerically equal values in distinct arrays).
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonLinearRingRepresentationShouldBeIdentical;
structural_prop!(PolygonLinearRingRepresentationShouldBeIdentical, "PolygonLinearRingRepresentationShouldBeIdentical");

/// A linear ring does not self-intersect; its edges may touch only at the shared endpoint.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonRingDoesNotSelfIntersect;
structural_prop!(PolygonRingDoesNotSelfIntersect, "PolygonRingDoesNotSelfIntersect");
```

### §3.1.6 — Winding order (right-hand rule)

```rust
/// The exterior ring (index 0) follows counter-clockwise winding order (right-hand rule).
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonExteriorRingIsCounterclockwise;
structural_prop!(PolygonExteriorRingIsCounterclockwise, "PolygonExteriorRingIsCounterclockwise");

/// Hole rings (index >= 1) follow clockwise winding order (right-hand rule).
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonHoleRingsAreClockwise;
structural_prop!(PolygonHoleRingsAreClockwise, "PolygonHoleRingsAreClockwise");

/// By the right-hand rule, the enclosed area lies to the left of the direction of travel.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonRightHandRuleAreaToLeft;
structural_prop!(PolygonRightHandRuleAreaToLeft, "PolygonRightHandRuleAreaToLeft");

/// The exterior ring (index 0) has positive signed area by the shoelace formula.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonExteriorRingHasPositiveSignedArea;
structural_prop!(PolygonExteriorRingHasPositiveSignedArea, "PolygonExteriorRingHasPositiveSignedArea");

/// Hole rings (index >= 1) have negative signed area by the shoelace formula.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonHoleRingHasNegativeSignedArea;
structural_prop!(PolygonHoleRingHasNegativeSignedArea, "PolygonHoleRingHasNegativeSignedArea");
```

### §3.1.6 — Ring role semantics

```rust
/// When a Polygon has more than one ring, the ring at index 0 is the exterior boundary.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonFirstRingIsExteriorBoundary;
structural_prop!(PolygonFirstRingIsExteriorBoundary, "PolygonFirstRingIsExteriorBoundary");

/// When a Polygon has more than one ring, rings at index >= 1 are interior holes.
///
/// Source: RFC 7946 §3.1.6 — Polygon
pub struct PolygonSubsequentRingsAreInteriorHoles;
structural_prop!(PolygonSubsequentRingsAreInteriorHoles, "PolygonSubsequentRingsAreInteriorHoles");
```

---

## §3.1.7 — MultiPolygon

A MultiPolygon is an array of Polygon coordinate arrays.  Every element must
independently satisfy all Polygon constraints (winding, ring closure, minimum
positions).  The outer array may be empty.

```rust
/// The "type" member of a MultiPolygon geometry equals the string "MultiPolygon".
///
/// Source: RFC 7946 §3.1.7 — MultiPolygon
pub struct MultiPolygonTypeEqualsMultiPolygon;
structural_prop!(MultiPolygonTypeEqualsMultiPolygon, "MultiPolygonTypeEqualsMultiPolygon");

/// A MultiPolygon geometry has a "coordinates" member.
///
/// Source: RFC 7946 §3.1.7 — MultiPolygon
pub struct MultiPolygonHasCoordinatesMember;
structural_prop!(MultiPolygonHasCoordinatesMember, "MultiPolygonHasCoordinatesMember");

/// The "coordinates" value of a MultiPolygon is a JSON array.
///
/// Source: RFC 7946 §3.1.7 — MultiPolygon
pub struct MultiPolygonCoordinatesIsArray;
structural_prop!(MultiPolygonCoordinatesIsArray, "MultiPolygonCoordinatesIsArray");

/// Each element in a MultiPolygon "coordinates" array is a valid Polygon coordinates array.
///
/// Source: RFC 7946 §3.1.7 — MultiPolygon
pub struct MultiPolygonEachElementIsPolygonCoordinates;
structural_prop!(MultiPolygonEachElementIsPolygonCoordinates, "MultiPolygonEachElementIsPolygonCoordinates");

/// Each polygon element within a MultiPolygon satisfies all Polygon ring constraints.
///
/// Source: RFC 7946 §3.1.7 — MultiPolygon
pub struct MultiPolygonEachPolygonObeysPolygonRules;
structural_prop!(MultiPolygonEachPolygonObeysPolygonRules, "MultiPolygonEachPolygonObeysPolygonRules");
```

---

## §3.1.8 — GeometryCollection

A GeometryCollection contains a heterogeneous array of geometry objects under a
`geometries` member (not `coordinates`).  Implementations SHOULD avoid nesting
GeometryCollections and SHOULD prefer MultiX types when all members share a type.

### §3.1.8 — Required members

```rust
/// The "type" member of a GeometryCollection equals the string "GeometryCollection".
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionTypeEqualsGeometryCollection;
structural_prop!(GeometryCollectionTypeEqualsGeometryCollection, "GeometryCollectionTypeEqualsGeometryCollection");

/// A GeometryCollection has a "geometries" member.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionHasGeometriesMember;
structural_prop!(GeometryCollectionHasGeometriesMember, "GeometryCollectionHasGeometriesMember");

/// A GeometryCollection does not have a "coordinates" member.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionMustNotHaveCoordinatesMember;
structural_prop!(GeometryCollectionMustNotHaveCoordinatesMember, "GeometryCollectionMustNotHaveCoordinatesMember");
```

### §3.1.8 — `geometries` array constraints

```rust
/// The "geometries" value of a GeometryCollection is a JSON array.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionGeometriesIsJsonArray;
structural_prop!(GeometryCollectionGeometriesIsJsonArray, "GeometryCollectionGeometriesIsJsonArray");

/// Each element in the "geometries" array is a valid GeoJSON Geometry object.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionEachElementIsValidGeometry;
structural_prop!(GeometryCollectionEachElementIsValidGeometry, "GeometryCollectionEachElementIsValidGeometry");

/// Each geometry object within "geometries" carries its own "type" member.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionEachGeometryHasTypeMember;
structural_prop!(GeometryCollectionEachGeometryHasTypeMember, "GeometryCollectionEachGeometryHasTypeMember");

/// The "geometries" array of a GeometryCollection may be empty.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionMayBeEmpty;
structural_prop!(GeometryCollectionMayBeEmpty, "GeometryCollectionMayBeEmpty");
```

### §3.1.8 — Anti-nesting and anti-aggregation recommendations

```rust
/// A GeometryCollection SHOULD NOT contain another GeometryCollection.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionShouldNotBeNested;
structural_prop!(GeometryCollectionShouldNotBeNested, "GeometryCollectionShouldNotBeNested");

/// A GeometryCollection SHOULD NOT be used when a MultiPoint, MultiLineString, or
/// MultiPolygon could represent the same data more concisely.
///
/// Source: RFC 7946 §3.1.8 — Geometry Collection
pub struct GeometryCollectionShouldNotReplaceMultiXType;
structural_prop!(GeometryCollectionShouldNotReplaceMultiXType, "GeometryCollectionShouldNotReplaceMultiXType");
```

---

## §3.1.9 — Antimeridian Cutting

RFC 7946 introduces a normative recommendation to cut geometries at the antimeridian
(180 degrees E/W longitude) rather than representing a geometry whose bounding box
wraps around the globe.  Cutting converts a LineString to a MultiLineString and a
Polygon to a MultiPolygon; neither resulting part crosses the antimeridian.

```rust
/// Geometries that cross the antimeridian (180 degrees) SHOULD be split along it.
///
/// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
pub struct AntimeridianCrossingGeometriesShouldBeSplit;
structural_prop!(AntimeridianCrossingGeometriesShouldBeSplit, "AntimeridianCrossingGeometriesShouldBeSplit");

/// A LineString crossing the antimeridian is split into a MultiLineString.
///
/// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
pub struct AntimeridianLineStringSplitsToMultiLineString;
structural_prop!(AntimeridianLineStringSplitsToMultiLineString, "AntimeridianLineStringSplitsToMultiLineString");

/// A Polygon crossing the antimeridian is split into a MultiPolygon.
///
/// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
pub struct AntimeridianPolygonSplitsToMultiPolygon;
structural_prop!(AntimeridianPolygonSplitsToMultiPolygon, "AntimeridianPolygonSplitsToMultiPolygon");

/// Neither part of a split geometry crosses the antimeridian after cutting.
///
/// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
pub struct AntimeridianSplitPartDoesNotCrossLine;
structural_prop!(AntimeridianSplitPartDoesNotCrossLine, "AntimeridianSplitPartDoesNotCrossLine");

/// No coordinate in a cut geometry has a longitude value outside the range [-180, 180].
///
/// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
pub struct AntimeridianNoLongitudeOutsideRange;
structural_prop!(AntimeridianNoLongitudeOutsideRange, "AntimeridianNoLongitudeOutsideRange");

/// Antimeridian cutting preserves the topological connectivity of the original geometry.
///
/// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
pub struct AntimeridianCutPreservesTopology;
structural_prop!(AntimeridianCutPreservesTopology, "AntimeridianCutPreservesTopology");
```

---

## §3.1.10 — Coordinate Precision and Uncertainty

RFC 7946 cautions that the number of decimal digits in a coordinate value carries no
normative implication about data accuracy.  Consumers must not infer uncertainty from
digit count.

```rust
/// The number of decimal digits in a coordinate value MUST NOT imply a precision level
/// or uncertainty claim.
///
/// Source: RFC 7946 §3.1.10 — Uncertainty and Precision of Coordinates
pub struct CoordinatePrecisionMustNotImplyUncertainty;
structural_prop!(CoordinatePrecisionMustNotImplyUncertainty, "CoordinatePrecisionMustNotImplyUncertainty");

/// The digit count of a coordinate is a serialisation choice, not a data-accuracy indicator.
///
/// Source: RFC 7946 §3.1.10 — Uncertainty and Precision of Coordinates
pub struct DigitCountIsNotDataAccuracyIndicator;
structural_prop!(DigitCountIsNotDataAccuracyIndicator, "DigitCountIsNotDataAccuracyIndicator");
```

---

## §3.2 — Feature Object

A Feature is a spatially bounded entity.  It MUST carry both a `geometry` member and
a `properties` member.  `geometry` may be null (unlocated feature); `properties` may
be null (no attributes).  The optional `id` member, when present, must be a string
or number — never null, boolean, array, or object.

### §3.2 — Required members

```rust
/// The "type" member of a Feature object equals the string "Feature".
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureTypeEqualsFeature;
structural_prop!(FeatureTypeEqualsFeature, "FeatureTypeEqualsFeature");

/// A Feature object has a "geometry" member.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureHasGeometryMember;
structural_prop!(FeatureHasGeometryMember, "FeatureHasGeometryMember");

/// A Feature object has a "properties" member.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureHasPropertiesMember;
structural_prop!(FeatureHasPropertiesMember, "FeatureHasPropertiesMember");
```

### §3.2 — `geometry` member value constraints

```rust
/// The "geometry" value is either a GeoJSON Geometry object or JSON null.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureGeometryIsGeometryObjectOrNull;
structural_prop!(FeatureGeometryIsGeometryObjectOrNull, "FeatureGeometryIsGeometryObjectOrNull");

/// When "geometry" is a non-null value it is a valid GeoJSON Geometry object.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureGeometryWhenPresentIsValidGeometry;
structural_prop!(FeatureGeometryWhenPresentIsValidGeometry, "FeatureGeometryWhenPresentIsValidGeometry");

/// A null "geometry" value means the feature is unlocated (spatially indeterminate).
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureNullGeometryMeansUnlocated;
structural_prop!(FeatureNullGeometryMeansUnlocated, "FeatureNullGeometryMeansUnlocated");
```

### §3.2 — `properties` member value constraints

```rust
/// The "properties" value is either a JSON object or JSON null.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeaturePropertiesIsJsonObjectOrNull;
structural_prop!(FeaturePropertiesIsJsonObjectOrNull, "FeaturePropertiesIsJsonObjectOrNull");
```

### §3.2 — `id` member constraints

```rust
/// The "id" member of a Feature is optional; its absence is valid.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdMemberIsOptional;
structural_prop!(FeatureIdMemberIsOptional, "FeatureIdMemberIsOptional");

/// When the "id" member is present its value is a JSON string or JSON number.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdWhenPresentIsStringOrNumber;
structural_prop!(FeatureIdWhenPresentIsStringOrNumber, "FeatureIdWhenPresentIsStringOrNumber");

/// A string value is a permitted type for the Feature "id".
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdStringIsAllowed;
structural_prop!(FeatureIdStringIsAllowed, "FeatureIdStringIsAllowed");

/// A number value is a permitted type for the Feature "id".
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdNumberIsAllowed;
structural_prop!(FeatureIdNumberIsAllowed, "FeatureIdNumberIsAllowed");

/// The "id" value MUST NOT be JSON null.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdMustNotBeNull;
structural_prop!(FeatureIdMustNotBeNull, "FeatureIdMustNotBeNull");

/// The "id" value MUST NOT be a JSON boolean.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdMustNotBeBoolean;
structural_prop!(FeatureIdMustNotBeBoolean, "FeatureIdMustNotBeBoolean");

/// The "id" value MUST NOT be a JSON array.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdMustNotBeArray;
structural_prop!(FeatureIdMustNotBeArray, "FeatureIdMustNotBeArray");

/// The "id" value MUST NOT be a JSON object.
///
/// Source: RFC 7946 §3.2 — Feature Object
pub struct FeatureIdMustNotBeObject;
structural_prop!(FeatureIdMustNotBeObject, "FeatureIdMustNotBeObject");
```

---

## §3.3 — FeatureCollection Object

A FeatureCollection aggregates zero or more Feature objects.  Each element of
`features` must be a Feature (not a raw Geometry or an arbitrary JSON value).

```rust
/// The "type" member of a FeatureCollection equals the string "FeatureCollection".
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionTypeEqualsFeatureCollection;
structural_prop!(FeatureCollectionTypeEqualsFeatureCollection, "FeatureCollectionTypeEqualsFeatureCollection");

/// A FeatureCollection has a "features" member.
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionHasFeaturesMember;
structural_prop!(FeatureCollectionHasFeaturesMember, "FeatureCollectionHasFeaturesMember");

/// The "features" value is a JSON array.
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionFeaturesIsJsonArray;
structural_prop!(FeatureCollectionFeaturesIsJsonArray, "FeatureCollectionFeaturesIsJsonArray");

/// The "features" value is not JSON null.
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionFeaturesIsNotNull;
structural_prop!(FeatureCollectionFeaturesIsNotNull, "FeatureCollectionFeaturesIsNotNull");

/// Each element of the "features" array is a GeoJSON Feature object.
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionEachElementIsFeatureObject;
structural_prop!(FeatureCollectionEachElementIsFeatureObject, "FeatureCollectionEachElementIsFeatureObject");

/// No element of the "features" array is JSON null.
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionEachElementIsNotNull;
structural_prop!(FeatureCollectionEachElementIsNotNull, "FeatureCollectionEachElementIsNotNull");

/// A FeatureCollection "features" array may be empty.
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionMayBeEmpty;
structural_prop!(FeatureCollectionMayBeEmpty, "FeatureCollectionMayBeEmpty");

/// The "features" value is not a single Feature object (it is always wrapped in an array).
///
/// Source: RFC 7946 §3.3 — FeatureCollection Object
pub struct FeatureCollectionFeaturesIsNotSingleFeature;
structural_prop!(FeatureCollectionFeaturesIsNotSingleFeature, "FeatureCollectionFeaturesIsNotSingleFeature");
```

---

## §4 — Coordinate Reference System

RFC 7946 mandates a single, fixed CRS: WGS 84 (EPSG 4326) with longitude/latitude
axis order.  The pre-2016 `crs` member is explicitly banned.  Alternative CRS is
forbidden unless the producer and consumer have made a prior arrangement outside the
scope of the format.

### §4 — CRS identity and axis order

```rust
/// Only the WGS 84 geographic coordinate reference system is permitted.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsOnlyWgs84IsPermitted;
structural_prop!(CrsOnlyWgs84IsPermitted, "CrsOnlyWgs84IsPermitted");

/// The coordinate reference system is equivalent to OGC URN urn:ogc:def:crs:OGC::CRS84.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsEquivalentToOgcUrn;
structural_prop!(CrsEquivalentToOgcUrn, "CrsEquivalentToOgcUrn");

/// The axis order is longitude first (easting), latitude second (northing).
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsAxisOrderLongitudeFirst;
structural_prop!(CrsAxisOrderLongitudeFirst, "CrsAxisOrderLongitudeFirst");

/// The axis order is latitude second; lat-first EPSG 4326 axis order is not used.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsAxisOrderLatitudeSecond;
structural_prop!(CrsAxisOrderLatitudeSecond, "CrsAxisOrderLatitudeSecond");
```

### §4 — Altitude and absent-altitude semantics

```rust
/// When an altitude is provided it is measured in metres above the WGS 84 ellipsoid.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsAltitudeIsAboveWgs84Ellipsoid;
structural_prop!(CrsAltitudeIsAboveWgs84Ellipsoid, "CrsAltitudeIsAboveWgs84Ellipsoid");

/// When altitude is absent the position SHOULD be treated as local ground or sea level.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsAbsentAltitudeTreatedAsSurface;
structural_prop!(CrsAbsentAltitudeTreatedAsSurface, "CrsAbsentAltitudeTreatedAsSurface");
```

### §4 — Forbidden CRS mechanisms

```rust
/// The "crs" member MUST NOT appear in any RFC 7946 GeoJSON document.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsMemberMustNotAppearInDocument;
structural_prop!(CrsMemberMustNotAppearInDocument, "CrsMemberMustNotAppearInDocument");

/// Alternative coordinate reference systems are forbidden without prior arrangement.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsAlternativeForbiddenWithoutArrangement;
structural_prop!(CrsAlternativeForbiddenWithoutArrangement, "CrsAlternativeForbiddenWithoutArrangement");

/// Numeric EPSG codes MUST NOT be used to identify the CRS in a GeoJSON document.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsNoEpsgCodesPermitted;
structural_prop!(CrsNoEpsgCodesPermitted, "CrsNoEpsgCodesPermitted");

/// A named-CRS JSON object (as used in the pre-2016 GeoJSON spec) MUST NOT appear.
///
/// Source: RFC 7946 §4 — Coordinate Reference System
pub struct CrsNoNamedCrsObjectPermitted;
structural_prop!(CrsNoNamedCrsObjectPermitted, "CrsNoNamedCrsObjectPermitted");
```

---

## §5 — Bounding Box

The optional `bbox` member communicates the coordinate range of a GeoJSON object.
Its length is 2 times n where n equals coordinate dimensionality (2D = 4 elements,
3D = 6 elements).  Elements appear in south-west-then-north-east order.  For objects
straddling the antimeridian the west value may exceed the east value.

### §5 — Optional presence and array form

```rust
/// The "bbox" member is optional at every level of a GeoJSON document.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxMemberIsOptional;
structural_prop!(BboxMemberIsOptional, "BboxMemberIsOptional");

/// When present, "bbox" is a JSON array (not an object or scalar).
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxWhenPresentIsJsonArray;
structural_prop!(BboxWhenPresentIsJsonArray, "BboxWhenPresentIsJsonArray");

/// The length of the "bbox" array is 2 times n where n is coordinate dimensionality.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxLengthIsTwiceN;
structural_prop!(BboxLengthIsTwiceN, "BboxLengthIsTwiceN");
```

### §5 — 2D and 3D element counts

```rust
/// A 2D bounding box has exactly four elements.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct Bbox2dHasExactlyFourElements;
structural_prop!(Bbox2dHasExactlyFourElements, "Bbox2dHasExactlyFourElements");

/// A 3D bounding box has exactly six elements.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct Bbox3dHasExactlySixElements;
structural_prop!(Bbox3dHasExactlySixElements, "Bbox3dHasExactlySixElements");
```

### §5 — Element ordering

```rust
/// A 2D bbox has the order [min_lon, min_lat, max_lon, max_lat].
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct Bbox2dOrderIsMinLonMinLatMaxLonMaxLat;
structural_prop!(Bbox2dOrderIsMinLonMinLatMaxLonMaxLat, "Bbox2dOrderIsMinLonMinLatMaxLonMaxLat");

/// A 3D bbox has the order [min_lon, min_lat, min_alt, max_lon, max_lat, max_alt].
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt;
structural_prop!(Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt, "Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt");

/// All elements of the "bbox" array are JSON numbers.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxAllElementsAreJsonNumbers;
structural_prop!(BboxAllElementsAreJsonNumbers, "BboxAllElementsAreJsonNumbers");

/// Bbox edges follow lines of constant longitude, latitude, and altitude.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxEdgesFollowConstantCoordinateLines;
structural_prop!(BboxEdgesFollowConstantCoordinateLines, "BboxEdgesFollowConstantCoordinateLines");

/// The south-west corner appears before the north-east corner in the bbox array.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxSouthwestCornerPrecedesNortheast;
structural_prop!(BboxSouthwestCornerPrecedesNortheast, "BboxSouthwestCornerPrecedesNortheast");
```

### §5 — Coordinate range validity

```rust
/// The minimum latitude value in a bbox does not exceed the maximum latitude value.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxMinLatLeMaxLat;
structural_prop!(BboxMinLatLeMaxLat, "BboxMinLatLeMaxLat");

/// When the bbox is 3D, the minimum altitude does not exceed the maximum altitude.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxMinAltLeMaxAlt;
structural_prop!(BboxMinAltLeMaxAlt, "BboxMinAltLeMaxAlt");
```

### §5.2 — Antimeridian bbox

```rust
/// For a bbox crossing the antimeridian the west longitude value MAY be greater than
/// the east longitude value.
///
/// Source: RFC 7946 §5.2 — The Antimeridian
pub struct BboxAntimeridianWestValueMayExceedEast;
structural_prop!(BboxAntimeridianWestValueMayExceedEast, "BboxAntimeridianWestValueMayExceedEast");
```

### §5.3 — Pole handling

```rust
/// Bbox latitude values MUST NOT be greater than 90.
///
/// Source: RFC 7946 §5.3 — The Poles
pub struct BboxLatitudeMustNotExceed90;
structural_prop!(BboxLatitudeMustNotExceed90, "BboxLatitudeMustNotExceed90");

/// Bbox latitude values MUST NOT be less than -90.
///
/// Source: RFC 7946 §5.3 — The Poles
pub struct BboxLatitudeMustNotBelowNeg90;
structural_prop!(BboxLatitudeMustNotBelowNeg90, "BboxLatitudeMustNotBelowNeg90");

/// A bbox MUST NOT use an out-of-range latitude to imply coverage of a spherical cap.
///
/// Source: RFC 7946 §5.3 — The Poles
pub struct BboxMustNotUseOutOfRangeLatForSphericalCap;
structural_prop!(BboxMustNotUseOutOfRangeLatForSphericalCap, "BboxMustNotUseOutOfRangeLatForSphericalCap");
```

### §5 — Coverage relationship

```rust
/// When present, "bbox" SHOULD contain all geometry positions of the associated object.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxShouldContainAllGeometryPositions;
structural_prop!(BboxShouldContainAllGeometryPositions, "BboxShouldContainAllGeometryPositions");

/// The dimensionality of "bbox" matches the dimensionality of the geometry's coordinates.
///
/// Source: RFC 7946 §5 — Bounding Box
pub struct BboxDimensionMatchesGeometryDimension;
structural_prop!(BboxDimensionMatchesGeometryDimension, "BboxDimensionMatchesGeometryDimension");
```

---

## §6.1 — Foreign Members

GeoJSON is extensible through foreign members at any object level.  However, foreign
members do not alter the normative semantics of GeoJSON objects and SHOULD be ignored
by parsers that do not recognise them.  Application-specific data belongs in
`properties`, not as top-level foreign members of a Feature or FeatureCollection.

```rust
/// Additional members at any GeoJSON object level SHOULD be ignored by parsers.
///
/// Source: RFC 7946 §6.1 — Foreign Members
pub struct ForeignMembersAtAnyLevelShouldBeIgnored;
structural_prop!(ForeignMembersAtAnyLevelShouldBeIgnored, "ForeignMembersAtAnyLevelShouldBeIgnored");

/// Foreign members MUST NOT alter the normative semantics of the GeoJSON object.
///
/// Source: RFC 7946 §6.1 — Foreign Members
pub struct ForeignMembersDoNotAlterSemantics;
structural_prop!(ForeignMembersDoNotAlterSemantics, "ForeignMembersDoNotAlterSemantics");

/// A foreign member MUST NOT override or shadow the required "type" member.
///
/// Source: RFC 7946 §6.1 — Foreign Members
pub struct ForeignMembersCannotOverrideTypeMember;
structural_prop!(ForeignMembersCannotOverrideTypeMember, "ForeignMembersCannotOverrideTypeMember");

/// Application-specific data SHOULD be placed in the Feature "properties" member,
/// not as foreign members of the Feature root object.
///
/// Source: RFC 7946 §6.1 — Foreign Members
pub struct PropertiesIsPreferredLocationForAppData;
structural_prop!(PropertiesIsPreferredLocationForAppData, "PropertiesIsPreferredLocationForAppData");
```

---

## §7 — Extending GeoJSON (Non-Extensibility)

RFC 7946 explicitly prohibits new geometry types and forbids altering the semantics
of the nine existing types.  This replaces the pre-2016 specification which allowed
extension geometry types.

```rust
/// No new geometry type values may be defined beyond the seven in RFC 7946.
///
/// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
pub struct NoNewGeometryTypesMayBeDefined;
structural_prop!(NoNewGeometryTypesMayBeDefined, "NoNewGeometryTypesMayBeDefined");

/// The semantics of the seven existing geometry types MUST NOT be changed.
///
/// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
pub struct ExistingGeometrySemanticsMustNotChange;
structural_prop!(ExistingGeometrySemanticsMustNotChange, "ExistingGeometrySemanticsMustNotChange");

/// The semantics of the Feature object MUST NOT be changed.
///
/// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
pub struct FeatureSemanticsMustNotChange;
structural_prop!(FeatureSemanticsMustNotChange, "FeatureSemanticsMustNotChange");

/// The semantics of the FeatureCollection object MUST NOT be changed.
///
/// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
pub struct FeatureCollectionSemanticsMustNotChange;
structural_prop!(FeatureCollectionSemanticsMustNotChange, "FeatureCollectionSemanticsMustNotChange");
```

---

## §9 — Mapping to and from geo URIs

A GeoJSON Point geometry can be losslessly round-tripped to and from a `geo` URI as
defined in RFC 5870.  The longitude/latitude order in GeoJSON coordinates maps to
the lat/lon order in the geo URI path after suitable axis swapping.

```rust
/// A GeoJSON Point geometry may be mapped to a geo URI (RFC 5870).
///
/// Source: RFC 7946 §9 — Mapping 'geo' URIs
pub struct GeoJsonPointMappableToGeoUri;
structural_prop!(GeoJsonPointMappableToGeoUri, "GeoJsonPointMappableToGeoUri");

/// A geo URI (RFC 5870) may be mapped to a GeoJSON Point geometry.
///
/// Source: RFC 7946 §9 — Mapping 'geo' URIs
pub struct GeoUriMappableToGeoJsonPoint;
structural_prop!(GeoUriMappableToGeoJsonPoint, "GeoUriMappableToGeoJsonPoint");
```

---

## §11.1 — I-JSON Constraints

GeoJSON documents MUST be valid I-JSON (Interoperable JSON, RFC 7493).  I-JSON
restricts JSON with three key constraints: UTF-8 encoding, no duplicate object keys,
and numbers representable as IEEE 754 double-precision values.  NaN and +/-Infinity
are not valid JSON number values; RFC 7946 inherits that restriction.

```rust
/// GeoJSON documents MUST be encoded in UTF-8.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonUtf8EncodingRequired;
structural_prop!(IJsonUtf8EncodingRequired, "IJsonUtf8EncodingRequired");

/// No GeoJSON object may have duplicate member names (keys).
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonNoDuplicateKeysAllowed;
structural_prop!(IJsonNoDuplicateKeysAllowed, "IJsonNoDuplicateKeysAllowed");

/// All numbers in a GeoJSON document must be representable as IEEE 754
/// double-precision floating-point values.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonNumbersMustBeIeee754Representable;
structural_prop!(IJsonNumbersMustBeIeee754Representable, "IJsonNumbersMustBeIeee754Representable");

/// GeoJSON documents MUST NOT contain NaN as a number value.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonNoNaNValues;
structural_prop!(IJsonNoNaNValues, "IJsonNoNaNValues");

/// GeoJSON documents MUST NOT contain positive or negative Infinity as a number value.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonNoInfinityValues;
structural_prop!(IJsonNoInfinityValues, "IJsonNoInfinityValues");

/// A UTF-8 byte-order mark (BOM) SHOULD NOT be prepended to a GeoJSON document.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonNoBomShouldBeAdded;
structural_prop!(IJsonNoBomShouldBeAdded, "IJsonNoBomShouldBeAdded");

/// All string values in a GeoJSON document are valid Unicode sequences.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonStringsAreUnicode;
structural_prop!(IJsonStringsAreUnicode, "IJsonStringsAreUnicode");

/// String values in a GeoJSON document MUST NOT contain unescaped control characters.
///
/// Source: RFC 7946 §11.1 — I-JSON
pub struct IJsonNoUnescapedControlCharacters;
structural_prop!(IJsonNoUnescapedControlCharacters, "IJsonNoUnescapedControlCharacters");
```

---

## §11.2 — Coordinate Precision Guidance

RFC 7946 §11.2 notes that six decimal places in a degree coordinate gives
approximately 10 cm of precision at the equator, which exceeds most data-collection
accuracy.  Producers should not pad coordinates with false precision.

```rust
/// Six decimal degree places give ~10 cm precision; more digits do not improve accuracy.
///
/// Source: RFC 7946 §11.2 — Coordinate Precision
pub struct CoordPrecisionSixDecimalPlacesIsSufficient;
structural_prop!(CoordPrecisionSixDecimalPlacesIsSufficient, "CoordPrecisionSixDecimalPlacesIsSufficient");

/// Coordinate precision MUST NOT be used to communicate a data accuracy or uncertainty level.
///
/// Source: RFC 7946 §11.2 — Coordinate Precision
pub struct CoordPrecisionMustNotImplyAccuracyLevel;
structural_prop!(CoordPrecisionMustNotImplyAccuracyLevel, "CoordPrecisionMustNotImplyAccuracyLevel");

/// Coordinates are expressed as decimal degrees, not degrees-minutes-seconds (DMS) format.
///
/// Source: RFC 7946 §11.2 — Coordinate Precision
pub struct CoordPrecisionNotInDmsFormat;
structural_prop!(CoordPrecisionNotInDmsFormat, "CoordPrecisionNotInDmsFormat");

/// Coordinates are not expressed in sexagesimal (DDMMSS) notation.
///
/// Source: RFC 7946 §11.2 — Coordinate Precision
pub struct CoordPrecisionNotInSexagesimalFormat;
structural_prop!(CoordPrecisionNotInSexagesimalFormat, "CoordPrecisionNotInSexagesimalFormat");
```

---

## §12 — Media Type

The IANA-registered media type for GeoJSON is `application/geo+json`.  The widely
used file extension is `.geojson`.  The charset parameter SHOULD be omitted because
UTF-8 is implied, but `charset=UTF-8` is also acceptable.

```rust
/// The media type for GeoJSON content is "application/geo+json".
///
/// Source: RFC 7946 §12 — IANA Considerations
pub struct MediaTypeIsApplicationGeoJson;
structural_prop!(MediaTypeIsApplicationGeoJson, "MediaTypeIsApplicationGeoJson");

/// The widely used file extension for GeoJSON files is ".geojson".
///
/// Source: RFC 7946 §12 — IANA Considerations
pub struct FileExtensionIsGeojson;
structural_prop!(FileExtensionIsGeojson, "FileExtensionIsGeojson");

/// When present, the charset parameter of the media type SHOULD be "UTF-8".
///
/// Source: RFC 7946 §12 — IANA Considerations
pub struct MediaTypeCharsetIsUtf8;
structural_prop!(MediaTypeCharsetIsUtf8, "MediaTypeCharsetIsUtf8");

/// GeoJSON MUST NOT be served with the generic "application/json" type when the
/// more specific "application/geo+json" is available.
///
/// Source: RFC 7946 §12 — IANA Considerations
pub struct MediaTypeNotApplicationJson;
structural_prop!(MediaTypeNotApplicationJson, "MediaTypeNotApplicationJson");
```

---

## Cross-cutting Contracts

The following propositions span multiple RFC 7946 sections and capture invariants
that cannot be stated purely within one section but are required for full conformance.

### Coordinate dimensionality consistency

```rust
/// All positions within a single geometry object have the same coordinate dimension
/// (all 2D or all 3D; mixing 2D and 3D positions within one geometry is invalid).
///
/// Source: RFC 7946 §3.1.1 — Position (cross-cutting)
pub struct AllPositionsInGeometryHaveConsistentDimension;
structural_prop!(AllPositionsInGeometryHaveConsistentDimension, "AllPositionsInGeometryHaveConsistentDimension");

/// All Feature geometries in a FeatureCollection SHOULD use a consistent coordinate
/// dimension to aid interoperability.
///
/// Source: RFC 7946 §3.1.1 — Position (cross-cutting)
pub struct FeatureCollectionConsistentCoordinateDimension;
structural_prop!(FeatureCollectionConsistentCoordinateDimension, "FeatureCollectionConsistentCoordinateDimension");
```

### CRS enforcement

```rust
/// GeoJSON MUST NOT contain projected (metre-based) coordinates; only WGS 84
/// decimal-degree coordinates are permitted.
///
/// Source: RFC 7946 §4 — Coordinate Reference System (cross-cutting)
pub struct NoProjectedCoordinatesPermitted;
structural_prop!(NoProjectedCoordinatesPermitted, "NoProjectedCoordinatesPermitted");

/// All coordinates are expressed in decimal degrees, never in degrees-minutes-seconds
/// or any other angular notation.
///
/// Source: RFC 7946 §4 — Coordinate Reference System (cross-cutting)
pub struct CoordinatesAreInDecimalDegreesOnly;
structural_prop!(CoordinatesAreInDecimalDegreesOnly, "CoordinatesAreInDecimalDegreesOnly");
```

### Type-coordinates pairing invariant

```rust
/// The "type" value of a geometry object determines the expected structure of its
/// "coordinates" member; a mismatch is invalid.
///
/// Source: RFC 7946 §3 — GeoJSON Object (cross-cutting)
pub struct GeometryTypeValueMatchesCoordinateStructure;
structural_prop!(GeometryTypeValueMatchesCoordinateStructure, "GeometryTypeValueMatchesCoordinateStructure");
```

### Null geometry vs absent geometry

```rust
/// A Feature with an explicitly null "geometry" is semantically distinct from a Feature
/// whose "geometry" member is absent (the latter is invalid per RFC 7946).
///
/// Source: RFC 7946 §3.2 — Feature Object (cross-cutting)
pub struct NullGeometryDistinctFromAbsentGeometry;
structural_prop!(NullGeometryDistinctFromAbsentGeometry, "NullGeometryDistinctFromAbsentGeometry");
```

### Feature id uniqueness

```rust
/// Feature "id" values SHOULD be unique within a FeatureCollection to enable reliable
/// feature lookup by id.
///
/// Source: RFC 7946 §3.2 — Feature Object (cross-cutting)
pub struct FeatureIdUniquenessWithinCollectionRecommended;
structural_prop!(FeatureIdUniquenessWithinCollectionRecommended, "FeatureIdUniquenessWithinCollectionRecommended");

/// A FeatureCollection does not require its Feature members to carry "id" values.
///
/// Source: RFC 7946 §3.2 / §3.3 — Feature Object / FeatureCollection Object
pub struct FeatureCollectionIdIsNotRequired;
structural_prop!(FeatureCollectionIdIsNotRequired, "FeatureCollectionIdIsNotRequired");
```

### Object well-formedness

```rust
/// A GeoJSON object MUST NOT use a null value for its "type" member under any
/// circumstances.
///
/// Source: RFC 7946 §3 — GeoJSON Object (cross-cutting)
pub struct GeoJsonObjectMustNotUseNullType;
structural_prop!(GeoJsonObjectMustNotUseNullType, "GeoJsonObjectMustNotUseNullType");

/// Every GeoJSON object must have its "type" member; its absence makes the object
/// invalid regardless of any other members present.
///
/// Source: RFC 7946 §3 — GeoJSON Object (cross-cutting)
pub struct GeoJsonObjectTypeMemberNeverMissing;
structural_prop!(GeoJsonObjectTypeMemberNeverMissing, "GeoJsonObjectTypeMemberNeverMissing");
```

### Full compliance composite

```rust
/// A GeoJSON document satisfies every normative MUST and MUST NOT requirement in
/// RFC 7946 (full compliance).
///
/// Source: RFC 7946 — complete standard
pub struct FullRfc7946Compliance;
structural_prop!(FullRfc7946Compliance, "FullRfc7946Compliance");
```

---

## Summary — All Prop Names by Category

### §2 GeoJSON Text (2)

- `GeoJsonTextIsSingleJsonValue`
- `GeoJsonRootIsObject`

### §3 GeoJSON Object (9)

- `GeoJsonObjectHasTypeMember`
- `GeoJsonTypeMemberIsString`
- `GeoJsonTypeMemberIsNotNull`
- `GeoJsonTypeIsCaseSensitive`
- `GeoJsonTypeIsOneOfNineValues`
- `GeoJsonTypeIsNotExtensible`
- `GeoJsonBboxMemberIsOptional`
- `GeoJsonBboxWhenPresentIsArray`
- `GeoJsonForeignMembersShouldBeIgnored`

### §3.1.1 Position (18)

- `PositionIsJsonArray`
- `PositionIsNotJsonObject`
- `PositionIsNotJsonNull`
- `PositionIsNotJsonString`
- `PositionIsNotJsonNumber`
- `PositionHasAtLeastTwoElements`
- `PositionElementsAreJsonNumbers`
- `PositionElementZeroIsLongitude`
- `PositionElementOneIsLatitude`
- `PositionLongitudeInRange`
- `PositionLatitudeInRange`
- `PositionLongitudeIsFinite`
- `PositionLatitudeIsFinite`
- `PositionElementTwoIsAltitudeWhenPresent`
- `PositionAltitudeReferencesWgs84Ellipsoid`
- `PositionShouldNotExceedThreeElements`
- `PositionExtraElementsBeyondThreeIgnored`
- `PositionInterpolationIsCartesian`

### §3.1.2 Point (6)

- `PointTypeEqualsPoint`
- `PointHasCoordinatesMember`
- `PointCoordinatesIsSinglePosition`
- `PointCoordinatesIsNotNestedArray`
- `PointCoordinatesHasMinTwoElements`
- `PointCoordinatesIsNotNull`

### §3.1.3 MultiPoint (6)

- `MultiPointTypeEqualsMultiPoint`
- `MultiPointHasCoordinatesMember`
- `MultiPointCoordinatesIsArrayOfPositions`
- `MultiPointEachElementIsValidPosition`
- `MultiPointCoordinatesMayBeEmpty`
- `MultiPointCoordinatesIsNotNull`

### §3.1.4 LineString (8)

- `LineStringTypeEqualsLineString`
- `LineStringHasCoordinatesMember`
- `LineStringCoordinatesIsNotNull`
- `LineStringCoordinatesIsArray`
- `LineStringCoordinatesHasMinTwoPositions`
- `LineStringEachElementIsValidPosition`
- `LineStringCoordinatesIsNotEmpty`
- `LineStringMinTwoPositionsFormPath`

### §3.1.5 MultiLineString (8)

- `MultiLineStringTypeEqualsMultiLineString`
- `MultiLineStringHasCoordinatesMember`
- `MultiLineStringCoordinatesIsNotNull`
- `MultiLineStringCoordinatesIsArray`
- `MultiLineStringEachElementIsArray`
- `MultiLineStringEachLineStringHasMinTwoPositions`
- `MultiLineStringEachPositionIsValid`
- `MultiLineStringMayBeEmpty`

### §3.1.6 Polygon (15)

- `PolygonTypeEqualsPolygon`
- `PolygonHasCoordinatesMember`
- `PolygonCoordinatesIsArrayOfRings`
- `PolygonLinearRingIsClosedLineString`
- `PolygonLinearRingHasMinFourPositions`
- `PolygonLinearRingFirstAndLastAreIdentical`
- `PolygonLinearRingRepresentationShouldBeIdentical`
- `PolygonRingDoesNotSelfIntersect`
- `PolygonExteriorRingIsCounterclockwise`
- `PolygonHoleRingsAreClockwise`
- `PolygonRightHandRuleAreaToLeft`
- `PolygonExteriorRingHasPositiveSignedArea`
- `PolygonHoleRingHasNegativeSignedArea`
- `PolygonFirstRingIsExteriorBoundary`
- `PolygonSubsequentRingsAreInteriorHoles`

### §3.1.7 MultiPolygon (5)

- `MultiPolygonTypeEqualsMultiPolygon`
- `MultiPolygonHasCoordinatesMember`
- `MultiPolygonCoordinatesIsArray`
- `MultiPolygonEachElementIsPolygonCoordinates`
- `MultiPolygonEachPolygonObeysPolygonRules`

### §3.1.8 GeometryCollection (9)

- `GeometryCollectionTypeEqualsGeometryCollection`
- `GeometryCollectionHasGeometriesMember`
- `GeometryCollectionMustNotHaveCoordinatesMember`
- `GeometryCollectionGeometriesIsJsonArray`
- `GeometryCollectionEachElementIsValidGeometry`
- `GeometryCollectionEachGeometryHasTypeMember`
- `GeometryCollectionMayBeEmpty`
- `GeometryCollectionShouldNotBeNested`
- `GeometryCollectionShouldNotReplaceMultiXType`

### §3.1.9 Antimeridian Cutting (6)

- `AntimeridianCrossingGeometriesShouldBeSplit`
- `AntimeridianLineStringSplitsToMultiLineString`
- `AntimeridianPolygonSplitsToMultiPolygon`
- `AntimeridianSplitPartDoesNotCrossLine`
- `AntimeridianNoLongitudeOutsideRange`
- `AntimeridianCutPreservesTopology`

### §3.1.10 Coordinate Precision and Uncertainty (2)

- `CoordinatePrecisionMustNotImplyUncertainty`
- `DigitCountIsNotDataAccuracyIndicator`

### §3.2 Feature Object (15)

- `FeatureTypeEqualsFeature`
- `FeatureHasGeometryMember`
- `FeatureHasPropertiesMember`
- `FeatureGeometryIsGeometryObjectOrNull`
- `FeatureGeometryWhenPresentIsValidGeometry`
- `FeatureNullGeometryMeansUnlocated`
- `FeaturePropertiesIsJsonObjectOrNull`
- `FeatureIdMemberIsOptional`
- `FeatureIdWhenPresentIsStringOrNumber`
- `FeatureIdStringIsAllowed`
- `FeatureIdNumberIsAllowed`
- `FeatureIdMustNotBeNull`
- `FeatureIdMustNotBeBoolean`
- `FeatureIdMustNotBeArray`
- `FeatureIdMustNotBeObject`

### §3.3 FeatureCollection Object (8)

- `FeatureCollectionTypeEqualsFeatureCollection`
- `FeatureCollectionHasFeaturesMember`
- `FeatureCollectionFeaturesIsJsonArray`
- `FeatureCollectionFeaturesIsNotNull`
- `FeatureCollectionEachElementIsFeatureObject`
- `FeatureCollectionEachElementIsNotNull`
- `FeatureCollectionMayBeEmpty`
- `FeatureCollectionFeaturesIsNotSingleFeature`

### §4 Coordinate Reference System (10)

- `CrsOnlyWgs84IsPermitted`
- `CrsEquivalentToOgcUrn`
- `CrsAxisOrderLongitudeFirst`
- `CrsAxisOrderLatitudeSecond`
- `CrsAltitudeIsAboveWgs84Ellipsoid`
- `CrsAbsentAltitudeTreatedAsSurface`
- `CrsMemberMustNotAppearInDocument`
- `CrsAlternativeForbiddenWithoutArrangement`
- `CrsNoEpsgCodesPermitted`
- `CrsNoNamedCrsObjectPermitted`

### §5 Bounding Box (18)

- `BboxMemberIsOptional`
- `BboxWhenPresentIsJsonArray`
- `BboxLengthIsTwiceN`
- `Bbox2dHasExactlyFourElements`
- `Bbox3dHasExactlySixElements`
- `Bbox2dOrderIsMinLonMinLatMaxLonMaxLat`
- `Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt`
- `BboxAllElementsAreJsonNumbers`
- `BboxEdgesFollowConstantCoordinateLines`
- `BboxSouthwestCornerPrecedesNortheast`
- `BboxMinLatLeMaxLat`
- `BboxMinAltLeMaxAlt`
- `BboxAntimeridianWestValueMayExceedEast`
- `BboxLatitudeMustNotExceed90`
- `BboxLatitudeMustNotBelowNeg90`
- `BboxMustNotUseOutOfRangeLatForSphericalCap`
- `BboxShouldContainAllGeometryPositions`
- `BboxDimensionMatchesGeometryDimension`

### §6.1 Foreign Members (4)

- `ForeignMembersAtAnyLevelShouldBeIgnored`
- `ForeignMembersDoNotAlterSemantics`
- `ForeignMembersCannotOverrideTypeMember`
- `PropertiesIsPreferredLocationForAppData`

### §7 Non-Extensibility (4)

- `NoNewGeometryTypesMayBeDefined`
- `ExistingGeometrySemanticsMustNotChange`
- `FeatureSemanticsMustNotChange`
- `FeatureCollectionSemanticsMustNotChange`

### §9 geo URI Mapping (2)

- `GeoJsonPointMappableToGeoUri`
- `GeoUriMappableToGeoJsonPoint`

### §11.1 I-JSON (8)

- `IJsonUtf8EncodingRequired`
- `IJsonNoDuplicateKeysAllowed`
- `IJsonNumbersMustBeIeee754Representable`
- `IJsonNoNaNValues`
- `IJsonNoInfinityValues`
- `IJsonNoBomShouldBeAdded`
- `IJsonStringsAreUnicode`
- `IJsonNoUnescapedControlCharacters`

### §11.2 Coordinate Precision (4)

- `CoordPrecisionSixDecimalPlacesIsSufficient`
- `CoordPrecisionMustNotImplyAccuracyLevel`
- `CoordPrecisionNotInDmsFormat`
- `CoordPrecisionNotInSexagesimalFormat`

### §12 Media Type (4)

- `MediaTypeIsApplicationGeoJson`
- `FileExtensionIsGeojson`
- `MediaTypeCharsetIsUtf8`
- `MediaTypeNotApplicationJson`

### Cross-cutting Contracts (11)

- `AllPositionsInGeometryHaveConsistentDimension`
- `FeatureCollectionConsistentCoordinateDimension`
- `NoProjectedCoordinatesPermitted`
- `CoordinatesAreInDecimalDegreesOnly`
- `GeometryTypeValueMatchesCoordinateStructure`
- `NullGeometryDistinctFromAbsentGeometry`
- `FeatureIdUniquenessWithinCollectionRecommended`
- `FeatureCollectionIdIsNotRequired`
- `GeoJsonObjectMustNotUseNullType`
- `GeoJsonObjectTypeMemberNeverMissing`
- `FullRfc7946Compliance`

---

**Total: 189 prop structs across 22 categories.**
