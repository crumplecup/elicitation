//! RFC 7946 GeoJSON propositions.
//!
//! Source: RFC 7946 — The GeoJSON Format (August 2016).
//! All §references are to RFC 7946 unless stated otherwise.

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

    // -- §2 GeoJSON Text --

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

    // -- §3 GeoJSON Object — type member --

    /// Every GeoJSON object has a member named "type".
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object
    pub struct GeoJsonObjectHasTypeMember;
    structural_prop!(GeoJsonObjectHasTypeMember, "GeoJsonObjectHasTypeMember");

    /// The value of the "type" member is a JSON string.
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

    /// The "type" value is one of the nine defined strings.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object
    pub struct GeoJsonTypeIsOneOfNineValues;
    structural_prop!(GeoJsonTypeIsOneOfNineValues, "GeoJsonTypeIsOneOfNineValues");

    /// The "type" value may not be extended with values not defined in RFC 7946.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object / §7
    pub struct GeoJsonTypeIsNotExtensible;
    structural_prop!(GeoJsonTypeIsNotExtensible, "GeoJsonTypeIsNotExtensible");

    // -- §3 GeoJSON Object — bbox and foreign members --

    /// The "bbox" member is optional; its absence is valid at every GeoJSON level.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object
    pub struct GeoJsonBboxMemberIsOptional;
    structural_prop!(GeoJsonBboxMemberIsOptional, "GeoJsonBboxMemberIsOptional");

    /// When the "bbox" member is present its value is a JSON array.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object
    pub struct GeoJsonBboxWhenPresentIsArray;
    structural_prop!(
        GeoJsonBboxWhenPresentIsArray,
        "GeoJsonBboxWhenPresentIsArray"
    );

    /// Foreign members SHOULD be ignored by parsers that do not understand them.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object
    pub struct GeoJsonForeignMembersShouldBeIgnored;
    structural_prop!(
        GeoJsonForeignMembersShouldBeIgnored,
        "GeoJsonForeignMembersShouldBeIgnored"
    );

    // -- §3.1.1 Position — array type --

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

    /// A position is not a bare JSON number.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionIsNotJsonNumber;
    structural_prop!(PositionIsNotJsonNumber, "PositionIsNotJsonNumber");

    // -- §3.1.1 Position — element count and type --

    /// A position array has at least two elements.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionHasAtLeastTwoElements;
    structural_prop!(
        PositionHasAtLeastTwoElements,
        "PositionHasAtLeastTwoElements"
    );

    /// Every element within a position array is a JSON number.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionElementsAreJsonNumbers;
    structural_prop!(
        PositionElementsAreJsonNumbers,
        "PositionElementsAreJsonNumbers"
    );

    // -- §3.1.1 Position — coordinate semantics and ranges --

    /// The first element (index 0) of a position is longitude in decimal degrees.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionElementZeroIsLongitude;
    structural_prop!(
        PositionElementZeroIsLongitude,
        "PositionElementZeroIsLongitude"
    );

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

    /// Altitude, when present, is a finite number (not NaN or +/-Infinity).
    ///
    /// Source: RFC 7946 §3.1.1 — Position / §11.1 — I-JSON
    pub struct PositionAltitudeIsFinite;
    structural_prop!(PositionAltitudeIsFinite, "PositionAltitudeIsFinite");

    // -- §3.1.1 Position — altitude (third element) --

    /// The third element (index 2), when present, is altitude above the WGS 84 ellipsoid.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionElementTwoIsAltitudeWhenPresent;
    structural_prop!(
        PositionElementTwoIsAltitudeWhenPresent,
        "PositionElementTwoIsAltitudeWhenPresent"
    );

    /// Altitude references the WGS 84 ellipsoid, not the geoid or mean sea level.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionAltitudeReferencesWgs84Ellipsoid;
    structural_prop!(
        PositionAltitudeReferencesWgs84Ellipsoid,
        "PositionAltitudeReferencesWgs84Ellipsoid"
    );

    /// A position SHOULD NOT have more than three elements.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionShouldNotExceedThreeElements;
    structural_prop!(
        PositionShouldNotExceedThreeElements,
        "PositionShouldNotExceedThreeElements"
    );

    /// Elements beyond index 2 are ignored by conformant implementations.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionExtraElementsBeyondThreeIgnored;
    structural_prop!(
        PositionExtraElementsBeyondThreeIgnored,
        "PositionExtraElementsBeyondThreeIgnored"
    );

    // -- §3.1.1 Position — interpolation model --

    /// The line segment between two positions is a straight Cartesian line, not a geodesic arc.
    ///
    /// Source: RFC 7946 §3.1.1 — Position
    pub struct PositionInterpolationIsCartesian;
    structural_prop!(
        PositionInterpolationIsCartesian,
        "PositionInterpolationIsCartesian"
    );

    // -- §3.1.2 Point --

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

    /// The "coordinates" value of a Point is a single position array.
    ///
    /// Source: RFC 7946 §3.1.2 — Point
    pub struct PointCoordinatesIsSinglePosition;
    structural_prop!(
        PointCoordinatesIsSinglePosition,
        "PointCoordinatesIsSinglePosition"
    );

    /// The "coordinates" of a Point is not a nested (doubly-wrapped) array.
    ///
    /// Source: RFC 7946 §3.1.2 — Point
    pub struct PointCoordinatesIsNotNestedArray;
    structural_prop!(
        PointCoordinatesIsNotNestedArray,
        "PointCoordinatesIsNotNestedArray"
    );

    /// The "coordinates" of a Point has at least two numeric elements.
    ///
    /// Source: RFC 7946 §3.1.2 — Point
    pub struct PointCoordinatesHasMinTwoElements;
    structural_prop!(
        PointCoordinatesHasMinTwoElements,
        "PointCoordinatesHasMinTwoElements"
    );

    /// The "coordinates" value of a Point is not null.
    ///
    /// Source: RFC 7946 §3.1.2 — Point
    pub struct PointCoordinatesIsNotNull;
    structural_prop!(PointCoordinatesIsNotNull, "PointCoordinatesIsNotNull");

    // -- §3.1.3 MultiPoint --

    /// The "type" member of a MultiPoint geometry equals the string "MultiPoint".
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint
    pub struct MultiPointTypeEqualsMultiPoint;
    structural_prop!(
        MultiPointTypeEqualsMultiPoint,
        "MultiPointTypeEqualsMultiPoint"
    );

    /// A MultiPoint geometry has a "coordinates" member.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint
    pub struct MultiPointHasCoordinatesMember;
    structural_prop!(
        MultiPointHasCoordinatesMember,
        "MultiPointHasCoordinatesMember"
    );

    /// The "coordinates" of a MultiPoint is an array of position arrays.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint
    pub struct MultiPointCoordinatesIsArrayOfPositions;
    structural_prop!(
        MultiPointCoordinatesIsArrayOfPositions,
        "MultiPointCoordinatesIsArrayOfPositions"
    );

    /// Each element in a MultiPoint "coordinates" array is a valid position.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint
    pub struct MultiPointEachElementIsValidPosition;
    structural_prop!(
        MultiPointEachElementIsValidPosition,
        "MultiPointEachElementIsValidPosition"
    );

    /// A MultiPoint "coordinates" array may be empty.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint
    pub struct MultiPointCoordinatesMayBeEmpty;
    structural_prop!(
        MultiPointCoordinatesMayBeEmpty,
        "MultiPointCoordinatesMayBeEmpty"
    );

    /// The "coordinates" value of a MultiPoint is not null.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint
    pub struct MultiPointCoordinatesIsNotNull;
    structural_prop!(
        MultiPointCoordinatesIsNotNull,
        "MultiPointCoordinatesIsNotNull"
    );

    // -- §3.1.4 LineString --

    /// The "type" member of a LineString geometry equals the string "LineString".
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringTypeEqualsLineString;
    structural_prop!(
        LineStringTypeEqualsLineString,
        "LineStringTypeEqualsLineString"
    );

    /// A LineString geometry has a "coordinates" member.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringHasCoordinatesMember;
    structural_prop!(
        LineStringHasCoordinatesMember,
        "LineStringHasCoordinatesMember"
    );

    /// The "coordinates" value of a LineString is not null.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringCoordinatesIsNotNull;
    structural_prop!(
        LineStringCoordinatesIsNotNull,
        "LineStringCoordinatesIsNotNull"
    );

    /// The "coordinates" value of a LineString is a JSON array.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringCoordinatesIsArray;
    structural_prop!(LineStringCoordinatesIsArray, "LineStringCoordinatesIsArray");

    /// A LineString "coordinates" array has at least two positions.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringCoordinatesHasMinTwoPositions;
    structural_prop!(
        LineStringCoordinatesHasMinTwoPositions,
        "LineStringCoordinatesHasMinTwoPositions"
    );

    /// Each element in a LineString "coordinates" array is a valid position.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringEachElementIsValidPosition;
    structural_prop!(
        LineStringEachElementIsValidPosition,
        "LineStringEachElementIsValidPosition"
    );

    /// A LineString "coordinates" array is never empty (minimum two positions required).
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringCoordinatesIsNotEmpty;
    structural_prop!(
        LineStringCoordinatesIsNotEmpty,
        "LineStringCoordinatesIsNotEmpty"
    );

    /// The minimum two positions of a LineString form a directed path with at least one segment.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString
    pub struct LineStringMinTwoPositionsFormPath;
    structural_prop!(
        LineStringMinTwoPositionsFormPath,
        "LineStringMinTwoPositionsFormPath"
    );

    // -- §3.1.5 MultiLineString --

    /// The "type" member of a MultiLineString geometry equals the string "MultiLineString".
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringTypeEqualsMultiLineString;
    structural_prop!(
        MultiLineStringTypeEqualsMultiLineString,
        "MultiLineStringTypeEqualsMultiLineString"
    );

    /// A MultiLineString geometry has a "coordinates" member.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringHasCoordinatesMember;
    structural_prop!(
        MultiLineStringHasCoordinatesMember,
        "MultiLineStringHasCoordinatesMember"
    );

    /// The "coordinates" value of a MultiLineString is not null.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringCoordinatesIsNotNull;
    structural_prop!(
        MultiLineStringCoordinatesIsNotNull,
        "MultiLineStringCoordinatesIsNotNull"
    );

    /// The "coordinates" value of a MultiLineString is a JSON array.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringCoordinatesIsArray;
    structural_prop!(
        MultiLineStringCoordinatesIsArray,
        "MultiLineStringCoordinatesIsArray"
    );

    /// Each element in a MultiLineString "coordinates" array is itself an array.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringEachElementIsArray;
    structural_prop!(
        MultiLineStringEachElementIsArray,
        "MultiLineStringEachElementIsArray"
    );

    /// Each LineString within a MultiLineString has at least two positions.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringEachLineStringHasMinTwoPositions;
    structural_prop!(
        MultiLineStringEachLineStringHasMinTwoPositions,
        "MultiLineStringEachLineStringHasMinTwoPositions"
    );

    /// Each position element within a MultiLineString component is a valid position.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct MultiLineStringEachPositionIsValid;
    structural_prop!(
        MultiLineStringEachPositionIsValid,
        "MultiLineStringEachPositionIsValid"
    );

    /// The outer "coordinates" array of a MultiLineString may be empty.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString
    pub struct GeoJsonMultiLineStringMayBeEmpty;
    structural_prop!(
        GeoJsonMultiLineStringMayBeEmpty,
        "GeoJsonMultiLineStringMayBeEmpty"
    );

    // -- §3.1.6 Polygon — basic structure --

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
    structural_prop!(
        PolygonCoordinatesIsArrayOfRings,
        "PolygonCoordinatesIsArrayOfRings"
    );

    /// The "coordinates" value of a Polygon is not null.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonCoordinatesIsNotNull;
    structural_prop!(PolygonCoordinatesIsNotNull, "PolygonCoordinatesIsNotNull");

    /// A Polygon ring array has at least one element (the exterior boundary).
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonCoordinatesHasAtLeastOneRing;
    structural_prop!(
        PolygonCoordinatesHasAtLeastOneRing,
        "PolygonCoordinatesHasAtLeastOneRing"
    );

    // -- §3.1.6 Polygon — linear ring constraints --

    /// A linear ring is a closed LineString: the first and last positions are identical.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonLinearRingIsClosedLineString;
    structural_prop!(
        PolygonLinearRingIsClosedLineString,
        "PolygonLinearRingIsClosedLineString"
    );

    /// A linear ring has at least four positions (three distinct vertices plus the repeated first).
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonLinearRingHasMinFourPositions;
    structural_prop!(
        PolygonLinearRingHasMinFourPositions,
        "PolygonLinearRingHasMinFourPositions"
    );

    /// The first and last positions of a linear ring MUST contain identical coordinate values.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonLinearRingFirstAndLastAreIdentical;
    structural_prop!(
        PolygonLinearRingFirstAndLastAreIdentical,
        "PolygonLinearRingFirstAndLastAreIdentical"
    );

    /// The first and last positions of a linear ring SHOULD be represented by the exact same
    /// JSON array (not merely numerically equal values in distinct arrays).
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonLinearRingRepresentationShouldBeIdentical;
    structural_prop!(
        PolygonLinearRingRepresentationShouldBeIdentical,
        "PolygonLinearRingRepresentationShouldBeIdentical"
    );

    /// A linear ring does not self-intersect; its edges may touch only at the shared endpoint.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonRingDoesNotSelfIntersect;
    structural_prop!(
        PolygonRingDoesNotSelfIntersect,
        "PolygonRingDoesNotSelfIntersect"
    );

    // -- §3.1.6 Polygon — winding order --

    /// The exterior ring (index 0) follows counter-clockwise winding order (right-hand rule).
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonExteriorRingIsCounterclockwise;
    structural_prop!(
        PolygonExteriorRingIsCounterclockwise,
        "PolygonExteriorRingIsCounterclockwise"
    );

    /// Hole rings (index >= 1) follow clockwise winding order (right-hand rule).
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonHoleRingsAreClockwise;
    structural_prop!(PolygonHoleRingsAreClockwise, "PolygonHoleRingsAreClockwise");

    /// By the right-hand rule, the enclosed area lies to the left of the direction of travel.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonRightHandRuleAreaToLeft;
    structural_prop!(
        PolygonRightHandRuleAreaToLeft,
        "PolygonRightHandRuleAreaToLeft"
    );

    /// The exterior ring (index 0) has positive signed area by the shoelace formula.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonExteriorRingHasPositiveSignedArea;
    structural_prop!(
        PolygonExteriorRingHasPositiveSignedArea,
        "PolygonExteriorRingHasPositiveSignedArea"
    );

    /// Hole rings (index >= 1) have negative signed area by the shoelace formula.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonHoleRingHasNegativeSignedArea;
    structural_prop!(
        PolygonHoleRingHasNegativeSignedArea,
        "PolygonHoleRingHasNegativeSignedArea"
    );

    // -- §3.1.6 Polygon — ring role semantics --

    /// When a Polygon has more than one ring, the ring at index 0 is the exterior boundary.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonFirstRingIsExteriorBoundary;
    structural_prop!(
        PolygonFirstRingIsExteriorBoundary,
        "PolygonFirstRingIsExteriorBoundary"
    );

    /// When a Polygon has more than one ring, rings at index >= 1 are interior holes.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonSubsequentRingsAreInteriorHoles;
    structural_prop!(
        PolygonSubsequentRingsAreInteriorHoles,
        "PolygonSubsequentRingsAreInteriorHoles"
    );

    // -- §3.1.6 Polygon — hole topology --

    /// Each interior hole ring lies geometrically within the exterior ring.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonHolesAreInteriorToExteriorRing;
    structural_prop!(
        PolygonHolesAreInteriorToExteriorRing,
        "PolygonHolesAreInteriorToExteriorRing"
    );

    /// Interior hole rings are mutually non-overlapping (they may touch at points).
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon
    pub struct PolygonHolesDoNotOverlap;
    structural_prop!(PolygonHolesDoNotOverlap, "PolygonHolesDoNotOverlap");

    // -- §3.1.7 MultiPolygon --

    /// The "type" member of a MultiPolygon geometry equals the string "MultiPolygon".
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct MultiPolygonTypeEqualsMultiPolygon;
    structural_prop!(
        MultiPolygonTypeEqualsMultiPolygon,
        "MultiPolygonTypeEqualsMultiPolygon"
    );

    /// A MultiPolygon geometry has a "coordinates" member.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct MultiPolygonHasCoordinatesMember;
    structural_prop!(
        MultiPolygonHasCoordinatesMember,
        "MultiPolygonHasCoordinatesMember"
    );

    /// The "coordinates" value of a MultiPolygon is a JSON array.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct MultiPolygonCoordinatesIsArray;
    structural_prop!(
        MultiPolygonCoordinatesIsArray,
        "MultiPolygonCoordinatesIsArray"
    );

    /// The "coordinates" value of a MultiPolygon is not null.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct MultiPolygonCoordinatesIsNotNull;
    structural_prop!(
        MultiPolygonCoordinatesIsNotNull,
        "MultiPolygonCoordinatesIsNotNull"
    );

    /// The outer "coordinates" array of a MultiPolygon may be empty.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct GeoJsonMultiPolygonMayBeEmpty;
    structural_prop!(
        GeoJsonMultiPolygonMayBeEmpty,
        "GeoJsonMultiPolygonMayBeEmpty"
    );

    /// Each element in a MultiPolygon "coordinates" array is a valid Polygon coordinates array.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct MultiPolygonEachElementIsPolygonCoordinates;
    structural_prop!(
        MultiPolygonEachElementIsPolygonCoordinates,
        "MultiPolygonEachElementIsPolygonCoordinates"
    );

    /// Each polygon element within a MultiPolygon satisfies all Polygon ring constraints.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon
    pub struct MultiPolygonEachPolygonObeysPolygonRules;
    structural_prop!(
        MultiPolygonEachPolygonObeysPolygonRules,
        "MultiPolygonEachPolygonObeysPolygonRules"
    );

    // -- §3.1.8 GeometryCollection --

    /// The "type" member of a GeometryCollection equals the string "GeometryCollection".
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionTypeEqualsGeometryCollection;
    structural_prop!(
        GeometryCollectionTypeEqualsGeometryCollection,
        "GeometryCollectionTypeEqualsGeometryCollection"
    );

    /// A GeometryCollection has a "geometries" member.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionHasGeometriesMember;
    structural_prop!(
        GeometryCollectionHasGeometriesMember,
        "GeometryCollectionHasGeometriesMember"
    );

    /// A GeometryCollection does not have a "coordinates" member.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionMustNotHaveCoordinatesMember;
    structural_prop!(
        GeometryCollectionMustNotHaveCoordinatesMember,
        "GeometryCollectionMustNotHaveCoordinatesMember"
    );

    /// The "geometries" value of a GeometryCollection is a JSON array.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionGeometriesIsJsonArray;
    structural_prop!(
        GeometryCollectionGeometriesIsJsonArray,
        "GeometryCollectionGeometriesIsJsonArray"
    );

    /// The "geometries" value of a GeometryCollection is not null.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionGeometriesIsNotNull;
    structural_prop!(
        GeometryCollectionGeometriesIsNotNull,
        "GeometryCollectionGeometriesIsNotNull"
    );

    /// Each element in the "geometries" array is a valid GeoJSON Geometry object.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionEachElementIsValidGeometry;
    structural_prop!(
        GeometryCollectionEachElementIsValidGeometry,
        "GeometryCollectionEachElementIsValidGeometry"
    );

    /// Each geometry object within "geometries" carries its own "type" member.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionEachGeometryHasTypeMember;
    structural_prop!(
        GeometryCollectionEachGeometryHasTypeMember,
        "GeometryCollectionEachGeometryHasTypeMember"
    );

    /// The "geometries" array of a GeometryCollection may be empty.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeoJsonGeometryCollectionMayBeEmpty;
    structural_prop!(
        GeoJsonGeometryCollectionMayBeEmpty,
        "GeoJsonGeometryCollectionMayBeEmpty"
    );

    /// A GeometryCollection SHOULD NOT contain another GeometryCollection.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionShouldNotBeNested;
    structural_prop!(
        GeometryCollectionShouldNotBeNested,
        "GeometryCollectionShouldNotBeNested"
    );

    /// A GeometryCollection SHOULD NOT be used when a MultiX type could suffice.
    ///
    /// Source: RFC 7946 §3.1.8 — Geometry Collection
    pub struct GeometryCollectionShouldNotReplaceMultiXType;
    structural_prop!(
        GeometryCollectionShouldNotReplaceMultiXType,
        "GeometryCollectionShouldNotReplaceMultiXType"
    );

    // -- §3.1.9 Antimeridian Cutting --

    /// Geometries that cross the antimeridian (180 degrees) SHOULD be split along it.
    ///
    /// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
    pub struct AntimeridianCrossingGeometriesShouldBeSplit;
    structural_prop!(
        AntimeridianCrossingGeometriesShouldBeSplit,
        "AntimeridianCrossingGeometriesShouldBeSplit"
    );

    /// A LineString crossing the antimeridian is split into a MultiLineString.
    ///
    /// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
    pub struct AntimeridianLineStringSplitsToMultiLineString;
    structural_prop!(
        AntimeridianLineStringSplitsToMultiLineString,
        "AntimeridianLineStringSplitsToMultiLineString"
    );

    /// A Polygon crossing the antimeridian is split into a MultiPolygon.
    ///
    /// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
    pub struct AntimeridianPolygonSplitsToMultiPolygon;
    structural_prop!(
        AntimeridianPolygonSplitsToMultiPolygon,
        "AntimeridianPolygonSplitsToMultiPolygon"
    );

    /// Neither part of a split geometry crosses the antimeridian after cutting.
    ///
    /// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
    pub struct AntimeridianSplitPartDoesNotCrossLine;
    structural_prop!(
        AntimeridianSplitPartDoesNotCrossLine,
        "AntimeridianSplitPartDoesNotCrossLine"
    );

    /// No coordinate in a cut geometry has a longitude value outside the range [-180, 180].
    ///
    /// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
    pub struct AntimeridianNoLongitudeOutsideRange;
    structural_prop!(
        AntimeridianNoLongitudeOutsideRange,
        "AntimeridianNoLongitudeOutsideRange"
    );

    /// Antimeridian cutting preserves the topological connectivity of the original geometry.
    ///
    /// Source: RFC 7946 §3.1.9 — Antimeridian Cutting
    pub struct AntimeridianCutPreservesTopology;
    structural_prop!(
        AntimeridianCutPreservesTopology,
        "AntimeridianCutPreservesTopology"
    );

    // -- §3.1.10 Coordinate Precision and Uncertainty --

    /// The number of decimal digits in a coordinate value MUST NOT imply a precision level.
    ///
    /// Source: RFC 7946 §3.1.10 — Uncertainty and Precision of Coordinates
    pub struct CoordinatePrecisionMustNotImplyUncertainty;
    structural_prop!(
        CoordinatePrecisionMustNotImplyUncertainty,
        "CoordinatePrecisionMustNotImplyUncertainty"
    );

    /// The digit count of a coordinate is a serialisation choice, not a data-accuracy indicator.
    ///
    /// Source: RFC 7946 §3.1.10 — Uncertainty and Precision of Coordinates
    pub struct DigitCountIsNotDataAccuracyIndicator;
    structural_prop!(
        DigitCountIsNotDataAccuracyIndicator,
        "DigitCountIsNotDataAccuracyIndicator"
    );

    // -- §3.2 Feature Object --

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

    /// The "geometry" value is either a GeoJSON Geometry object or JSON null.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object
    pub struct FeatureGeometryIsGeometryObjectOrNull;
    structural_prop!(
        FeatureGeometryIsGeometryObjectOrNull,
        "FeatureGeometryIsGeometryObjectOrNull"
    );

    /// When "geometry" is a non-null value it is a valid GeoJSON Geometry object.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object
    pub struct FeatureGeometryWhenPresentIsValidGeometry;
    structural_prop!(
        FeatureGeometryWhenPresentIsValidGeometry,
        "FeatureGeometryWhenPresentIsValidGeometry"
    );

    /// A null "geometry" value means the feature is unlocated (spatially indeterminate).
    ///
    /// Source: RFC 7946 §3.2 — Feature Object
    pub struct FeatureNullGeometryMeansUnlocated;
    structural_prop!(
        FeatureNullGeometryMeansUnlocated,
        "FeatureNullGeometryMeansUnlocated"
    );

    /// The "properties" value is either a JSON object or JSON null.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object
    pub struct FeaturePropertiesIsJsonObjectOrNull;
    structural_prop!(
        FeaturePropertiesIsJsonObjectOrNull,
        "FeaturePropertiesIsJsonObjectOrNull"
    );

    /// The "id" member of a Feature is optional; its absence is valid.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object
    pub struct FeatureIdMemberIsOptional;
    structural_prop!(FeatureIdMemberIsOptional, "FeatureIdMemberIsOptional");

    /// When the "id" member is present its value is a JSON string or JSON number.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object
    pub struct FeatureIdWhenPresentIsStringOrNumber;
    structural_prop!(
        FeatureIdWhenPresentIsStringOrNumber,
        "FeatureIdWhenPresentIsStringOrNumber"
    );

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

    // -- §3.3 FeatureCollection --

    /// The "type" member of a FeatureCollection equals the string "FeatureCollection".
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionTypeEqualsFeatureCollection;
    structural_prop!(
        FeatureCollectionTypeEqualsFeatureCollection,
        "FeatureCollectionTypeEqualsFeatureCollection"
    );

    /// A FeatureCollection has a "features" member.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionHasFeaturesMember;
    structural_prop!(
        FeatureCollectionHasFeaturesMember,
        "FeatureCollectionHasFeaturesMember"
    );

    /// The "features" value is a JSON array.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionFeaturesIsJsonArray;
    structural_prop!(
        FeatureCollectionFeaturesIsJsonArray,
        "FeatureCollectionFeaturesIsJsonArray"
    );

    /// The "features" value is not JSON null.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionFeaturesIsNotNull;
    structural_prop!(
        FeatureCollectionFeaturesIsNotNull,
        "FeatureCollectionFeaturesIsNotNull"
    );

    /// Each element of the "features" array is a GeoJSON Feature object.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionEachElementIsFeatureObject;
    structural_prop!(
        FeatureCollectionEachElementIsFeatureObject,
        "FeatureCollectionEachElementIsFeatureObject"
    );

    /// No element of the "features" array is JSON null.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionEachElementIsNotNull;
    structural_prop!(
        FeatureCollectionEachElementIsNotNull,
        "FeatureCollectionEachElementIsNotNull"
    );

    /// A FeatureCollection "features" array may be empty.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionMayBeEmpty;
    structural_prop!(FeatureCollectionMayBeEmpty, "FeatureCollectionMayBeEmpty");

    /// The "features" value is always wrapped in an array, never a single Feature object.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object
    pub struct FeatureCollectionFeaturesIsNotSingleFeature;
    structural_prop!(
        FeatureCollectionFeaturesIsNotSingleFeature,
        "FeatureCollectionFeaturesIsNotSingleFeature"
    );

    // -- §4 Coordinate Reference System --

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

    /// When an altitude is provided it is measured in metres above the WGS 84 ellipsoid.
    ///
    /// Source: RFC 7946 §4 — Coordinate Reference System
    pub struct CrsAltitudeIsAboveWgs84Ellipsoid;
    structural_prop!(
        CrsAltitudeIsAboveWgs84Ellipsoid,
        "CrsAltitudeIsAboveWgs84Ellipsoid"
    );

    /// When altitude is absent the position SHOULD be treated as local ground or sea level.
    ///
    /// Source: RFC 7946 §4 — Coordinate Reference System
    pub struct CrsAbsentAltitudeTreatedAsSurface;
    structural_prop!(
        CrsAbsentAltitudeTreatedAsSurface,
        "CrsAbsentAltitudeTreatedAsSurface"
    );

    /// The "crs" member MUST NOT appear in any RFC 7946 GeoJSON document.
    ///
    /// Source: RFC 7946 §4 — Coordinate Reference System
    pub struct CrsMemberMustNotAppearInDocument;
    structural_prop!(
        CrsMemberMustNotAppearInDocument,
        "CrsMemberMustNotAppearInDocument"
    );

    /// Alternative coordinate reference systems are forbidden without prior arrangement.
    ///
    /// Source: RFC 7946 §4 — Coordinate Reference System
    pub struct CrsAlternativeForbiddenWithoutArrangement;
    structural_prop!(
        CrsAlternativeForbiddenWithoutArrangement,
        "CrsAlternativeForbiddenWithoutArrangement"
    );

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

    // -- §5 Bounding Box --

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

    /// A 2D bbox has the order [min_lon, min_lat, max_lon, max_lat].
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct Bbox2dOrderIsMinLonMinLatMaxLonMaxLat;
    structural_prop!(
        Bbox2dOrderIsMinLonMinLatMaxLonMaxLat,
        "Bbox2dOrderIsMinLonMinLatMaxLonMaxLat"
    );

    /// A 3D bbox has the order [min_lon, min_lat, min_alt, max_lon, max_lat, max_alt].
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt;
    structural_prop!(
        Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt,
        "Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt"
    );

    /// All elements of the "bbox" array are JSON numbers.
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct BboxAllElementsAreJsonNumbers;
    structural_prop!(
        BboxAllElementsAreJsonNumbers,
        "BboxAllElementsAreJsonNumbers"
    );

    /// All elements of the "bbox" array are finite (not NaN or +/-Infinity).
    ///
    /// Source: RFC 7946 §5 — Bounding Box / §11.1 — I-JSON
    pub struct BboxElementsAreFinite;
    structural_prop!(BboxElementsAreFinite, "BboxElementsAreFinite");

    /// Bbox edges follow lines of constant longitude, latitude, and altitude.
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct BboxEdgesFollowConstantCoordinateLines;
    structural_prop!(
        BboxEdgesFollowConstantCoordinateLines,
        "BboxEdgesFollowConstantCoordinateLines"
    );

    /// The south-west corner appears before the north-east corner in the bbox array.
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct BboxSouthwestCornerPrecedesNortheast;
    structural_prop!(
        BboxSouthwestCornerPrecedesNortheast,
        "BboxSouthwestCornerPrecedesNortheast"
    );

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

    /// For a bbox crossing the antimeridian the west longitude value MAY be greater than east.
    ///
    /// Source: RFC 7946 §5.2 — The Antimeridian
    pub struct BboxAntimeridianWestValueMayExceedEast;
    structural_prop!(
        BboxAntimeridianWestValueMayExceedEast,
        "BboxAntimeridianWestValueMayExceedEast"
    );

    /// Bbox latitude values MUST NOT be greater than 90.
    ///
    /// Source: RFC 7946 §5.3 — The Poles
    pub struct BboxLatitudeMustNotExceed90;
    structural_prop!(BboxLatitudeMustNotExceed90, "BboxLatitudeMustNotExceed90");

    /// Bbox latitude values MUST NOT be less than -90.
    ///
    /// Source: RFC 7946 §5.3 — The Poles
    pub struct BboxLatitudeMustNotBelowNeg90;
    structural_prop!(
        BboxLatitudeMustNotBelowNeg90,
        "BboxLatitudeMustNotBelowNeg90"
    );

    /// A bbox MUST NOT use an out-of-range latitude to imply coverage of a spherical cap.
    ///
    /// Source: RFC 7946 §5.3 — The Poles
    pub struct BboxMustNotUseOutOfRangeLatForSphericalCap;
    structural_prop!(
        BboxMustNotUseOutOfRangeLatForSphericalCap,
        "BboxMustNotUseOutOfRangeLatForSphericalCap"
    );

    /// When present, "bbox" SHOULD contain all geometry positions of the associated object.
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct BboxShouldContainAllGeometryPositions;
    structural_prop!(
        BboxShouldContainAllGeometryPositions,
        "BboxShouldContainAllGeometryPositions"
    );

    /// The dimensionality of "bbox" matches the dimensionality of the geometry coordinates.
    ///
    /// Source: RFC 7946 §5 — Bounding Box
    pub struct BboxDimensionMatchesGeometryDimension;
    structural_prop!(
        BboxDimensionMatchesGeometryDimension,
        "BboxDimensionMatchesGeometryDimension"
    );

    // -- §6.1 Foreign Members --

    /// Additional members at any GeoJSON object level SHOULD be ignored by parsers.
    ///
    /// Source: RFC 7946 §6.1 — Foreign Members
    pub struct ForeignMembersAtAnyLevelShouldBeIgnored;
    structural_prop!(
        ForeignMembersAtAnyLevelShouldBeIgnored,
        "ForeignMembersAtAnyLevelShouldBeIgnored"
    );

    /// Foreign members MUST NOT alter the normative semantics of the GeoJSON object.
    ///
    /// Source: RFC 7946 §6.1 — Foreign Members
    pub struct ForeignMembersDoNotAlterSemantics;
    structural_prop!(
        ForeignMembersDoNotAlterSemantics,
        "ForeignMembersDoNotAlterSemantics"
    );

    /// A foreign member MUST NOT override or shadow the required "type" member.
    ///
    /// Source: RFC 7946 §6.1 — Foreign Members
    pub struct ForeignMembersCannotOverrideTypeMember;
    structural_prop!(
        ForeignMembersCannotOverrideTypeMember,
        "ForeignMembersCannotOverrideTypeMember"
    );

    /// Application-specific data SHOULD be placed in the Feature "properties" member.
    ///
    /// Source: RFC 7946 §6.1 — Foreign Members
    pub struct PropertiesIsPreferredLocationForAppData;
    structural_prop!(
        PropertiesIsPreferredLocationForAppData,
        "PropertiesIsPreferredLocationForAppData"
    );

    // -- §7 GeoJSON Types Are Not Extensible --

    /// No new geometry type values may be defined beyond the seven in RFC 7946.
    ///
    /// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
    pub struct NoNewGeometryTypesMayBeDefined;
    structural_prop!(
        NoNewGeometryTypesMayBeDefined,
        "NoNewGeometryTypesMayBeDefined"
    );

    /// The semantics of the seven existing geometry types MUST NOT be changed.
    ///
    /// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
    pub struct ExistingGeometrySemanticsMustNotChange;
    structural_prop!(
        ExistingGeometrySemanticsMustNotChange,
        "ExistingGeometrySemanticsMustNotChange"
    );

    /// The semantics of the Feature object MUST NOT be changed.
    ///
    /// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
    pub struct FeatureSemanticsMustNotChange;
    structural_prop!(
        FeatureSemanticsMustNotChange,
        "FeatureSemanticsMustNotChange"
    );

    /// The semantics of the FeatureCollection object MUST NOT be changed.
    ///
    /// Source: RFC 7946 §7 — GeoJSON Types Are Not Extensible
    pub struct FeatureCollectionSemanticsMustNotChange;
    structural_prop!(
        FeatureCollectionSemanticsMustNotChange,
        "FeatureCollectionSemanticsMustNotChange"
    );

    // -- §9 Mapping geo URIs --

    /// A GeoJSON Point geometry may be mapped to a geo URI (RFC 5870).
    ///
    /// Source: RFC 7946 §9 — Mapping geo URIs
    pub struct GeoJsonPointMappableToGeoUri;
    structural_prop!(GeoJsonPointMappableToGeoUri, "GeoJsonPointMappableToGeoUri");

    /// A geo URI (RFC 5870) may be mapped to a GeoJSON Point geometry.
    ///
    /// Source: RFC 7946 §9 — Mapping geo URIs
    pub struct GeoUriMappableToGeoJsonPoint;
    structural_prop!(GeoUriMappableToGeoJsonPoint, "GeoUriMappableToGeoJsonPoint");

    // -- §11.1 I-JSON Constraints --

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

    /// All numbers in a GeoJSON document must be representable as IEEE 754 double-precision.
    ///
    /// Source: RFC 7946 §11.1 — I-JSON
    pub struct IJsonNumbersMustBeIeee754Representable;
    structural_prop!(
        IJsonNumbersMustBeIeee754Representable,
        "IJsonNumbersMustBeIeee754Representable"
    );

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
    structural_prop!(
        IJsonNoUnescapedControlCharacters,
        "IJsonNoUnescapedControlCharacters"
    );

    // -- §11.2 Coordinate Precision Guidance --

    /// Six decimal degree places give ~10 cm precision; more digits do not improve accuracy.
    ///
    /// Source: RFC 7946 §11.2 — Coordinate Precision
    pub struct CoordPrecisionSixDecimalPlacesIsSufficient;
    structural_prop!(
        CoordPrecisionSixDecimalPlacesIsSufficient,
        "CoordPrecisionSixDecimalPlacesIsSufficient"
    );

    /// Coordinate precision MUST NOT be used to communicate a data accuracy level.
    ///
    /// Source: RFC 7946 §11.2 — Coordinate Precision
    pub struct CoordPrecisionMustNotImplyAccuracyLevel;
    structural_prop!(
        CoordPrecisionMustNotImplyAccuracyLevel,
        "CoordPrecisionMustNotImplyAccuracyLevel"
    );

    /// Coordinates are expressed as decimal degrees, not degrees-minutes-seconds format.
    ///
    /// Source: RFC 7946 §11.2 — Coordinate Precision
    pub struct CoordPrecisionNotInDmsFormat;
    structural_prop!(CoordPrecisionNotInDmsFormat, "CoordPrecisionNotInDmsFormat");

    /// Coordinates are not expressed in sexagesimal (DDMMSS) notation.
    ///
    /// Source: RFC 7946 §11.2 — Coordinate Precision
    pub struct CoordPrecisionNotInSexagesimalFormat;
    structural_prop!(
        CoordPrecisionNotInSexagesimalFormat,
        "CoordPrecisionNotInSexagesimalFormat"
    );

    // -- §12 Media Type --

    /// The media type for GeoJSON content is "application/geo+json".
    ///
    /// Source: RFC 7946 §12 — IANA Considerations
    pub struct MediaTypeIsApplicationGeoJson;
    structural_prop!(
        MediaTypeIsApplicationGeoJson,
        "MediaTypeIsApplicationGeoJson"
    );

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

    /// GeoJSON MUST NOT be served with the generic "application/json" type when
    /// the more specific "application/geo+json" is available.
    ///
    /// Source: RFC 7946 §12 — IANA Considerations
    pub struct MediaTypeNotApplicationJson;
    structural_prop!(MediaTypeNotApplicationJson, "MediaTypeNotApplicationJson");

    // -- Cross-cutting — coordinate consistency --

    /// All positions within a single geometry object have the same coordinate dimension.
    ///
    /// Source: RFC 7946 §3.1.1 — Position (cross-cutting)
    pub struct AllPositionsInGeometryHaveConsistentDimension;
    structural_prop!(
        AllPositionsInGeometryHaveConsistentDimension,
        "AllPositionsInGeometryHaveConsistentDimension"
    );

    /// All Feature geometries in a FeatureCollection SHOULD use a consistent coordinate dimension.
    ///
    /// Source: RFC 7946 §3.1.1 — Position (cross-cutting)
    pub struct FeatureCollectionConsistentCoordinateDimension;
    structural_prop!(
        FeatureCollectionConsistentCoordinateDimension,
        "FeatureCollectionConsistentCoordinateDimension"
    );

    // -- Cross-cutting — CRS enforcement --

    /// GeoJSON MUST NOT contain projected (metre-based) coordinates.
    ///
    /// Source: RFC 7946 §4 — Coordinate Reference System (cross-cutting)
    pub struct NoProjectedCoordinatesPermitted;
    structural_prop!(
        NoProjectedCoordinatesPermitted,
        "NoProjectedCoordinatesPermitted"
    );

    /// All coordinates are expressed in decimal degrees, never in any other angular notation.
    ///
    /// Source: RFC 7946 §4 — Coordinate Reference System (cross-cutting)
    pub struct CoordinatesAreInDecimalDegreesOnly;
    structural_prop!(
        CoordinatesAreInDecimalDegreesOnly,
        "CoordinatesAreInDecimalDegreesOnly"
    );

    // -- Cross-cutting — type-coordinates pairing --

    /// The "type" value of a geometry determines the expected structure of "coordinates".
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object (cross-cutting)
    pub struct GeometryTypeValueMatchesCoordinateStructure;
    structural_prop!(
        GeometryTypeValueMatchesCoordinateStructure,
        "GeometryTypeValueMatchesCoordinateStructure"
    );

    // -- Cross-cutting — null geometry vs absent geometry --

    /// A Feature with an explicitly null "geometry" is semantically distinct from a Feature
    /// whose "geometry" member is absent (the latter is invalid per RFC 7946).
    ///
    /// Source: RFC 7946 §3.2 — Feature Object (cross-cutting)
    pub struct NullGeometryDistinctFromAbsentGeometry;
    structural_prop!(
        NullGeometryDistinctFromAbsentGeometry,
        "NullGeometryDistinctFromAbsentGeometry"
    );

    // -- Cross-cutting — Feature id uniqueness --

    /// Feature "id" values SHOULD be unique within a FeatureCollection.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object (cross-cutting)
    pub struct FeatureIdUniquenessWithinCollectionRecommended;
    structural_prop!(
        FeatureIdUniquenessWithinCollectionRecommended,
        "FeatureIdUniquenessWithinCollectionRecommended"
    );

    /// A FeatureCollection does not require its Feature members to carry "id" values.
    ///
    /// Source: RFC 7946 §3.2 / §3.3 (cross-cutting)
    pub struct FeatureCollectionIdIsNotRequired;
    structural_prop!(
        FeatureCollectionIdIsNotRequired,
        "FeatureCollectionIdIsNotRequired"
    );

    // -- Cross-cutting — object well-formedness --

    /// A GeoJSON object MUST NOT use a null value for its "type" member.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object (cross-cutting)
    pub struct GeoJsonObjectMustNotUseNullType;
    structural_prop!(
        GeoJsonObjectMustNotUseNullType,
        "GeoJsonObjectMustNotUseNullType"
    );

    /// Every GeoJSON object must have its "type" member; its absence makes the object invalid.
    ///
    /// Source: RFC 7946 §3 — GeoJSON Object (cross-cutting)
    pub struct GeoJsonObjectTypeMemberNeverMissing;
    structural_prop!(
        GeoJsonObjectTypeMemberNeverMissing,
        "GeoJsonObjectTypeMemberNeverMissing"
    );

    // -- Cross-cutting — aggregate validity seams --

    /// A position satisfies all RFC 7946 §3.1.1 coordinate and type constraints.
    ///
    /// Source: RFC 7946 §3.1.1 — Position (aggregate seam)
    pub struct PositionValid;
    structural_prop!(PositionValid, "PositionValid");

    /// A Point geometry satisfies all RFC 7946 §3.1.2 constraints.
    ///
    /// Source: RFC 7946 §3.1.2 — Point (aggregate seam)
    pub struct GeoJsonPointValid;
    structural_prop!(GeoJsonPointValid, "GeoJsonPointValid");

    /// A MultiPoint geometry satisfies all RFC 7946 §3.1.3 constraints.
    ///
    /// Source: RFC 7946 §3.1.3 — MultiPoint (aggregate seam)
    pub struct GeoJsonMultiPointValid;
    structural_prop!(GeoJsonMultiPointValid, "GeoJsonMultiPointValid");

    /// A LineString geometry satisfies all RFC 7946 §3.1.4 constraints.
    ///
    /// Source: RFC 7946 §3.1.4 — LineString (aggregate seam)
    pub struct GeoJsonLineStringValid;
    structural_prop!(GeoJsonLineStringValid, "GeoJsonLineStringValid");

    /// A MultiLineString geometry satisfies all RFC 7946 §3.1.5 constraints.
    ///
    /// Source: RFC 7946 §3.1.5 — MultiLineString (aggregate seam)
    pub struct GeoJsonMultiLineStringValid;
    structural_prop!(GeoJsonMultiLineStringValid, "GeoJsonMultiLineStringValid");

    /// A Polygon geometry satisfies all RFC 7946 §3.1.6 constraints.
    ///
    /// Source: RFC 7946 §3.1.6 — Polygon (aggregate seam)
    pub struct GeoJsonPolygonValid;
    structural_prop!(GeoJsonPolygonValid, "GeoJsonPolygonValid");

    /// A MultiPolygon geometry satisfies all RFC 7946 §3.1.7 constraints.
    ///
    /// Source: RFC 7946 §3.1.7 — MultiPolygon (aggregate seam)
    pub struct GeoJsonMultiPolygonValid;
    structural_prop!(GeoJsonMultiPolygonValid, "GeoJsonMultiPolygonValid");

    /// A GeometryCollection satisfies all RFC 7946 §3.1.8 constraints.
    ///
    /// Source: RFC 7946 §3.1.8 — GeometryCollection (aggregate seam)
    pub struct GeoJsonGeometryCollectionValid;
    structural_prop!(
        GeoJsonGeometryCollectionValid,
        "GeoJsonGeometryCollectionValid"
    );

    /// A GeoJSON Geometry object is a valid instance of one of the seven geometry types.
    ///
    /// Source: RFC 7946 §3.1 — Geometry Object (aggregate seam)
    pub struct GeoJsonGeometryValid;
    structural_prop!(GeoJsonGeometryValid, "GeoJsonGeometryValid");

    /// A Feature object satisfies all RFC 7946 §3.2 constraints and its geometry,
    /// when non-null, satisfies GeoJsonGeometryValid.
    ///
    /// Source: RFC 7946 §3.2 — Feature Object (aggregate seam)
    pub struct FeatureValid;
    structural_prop!(FeatureValid, "FeatureValid");

    /// A FeatureCollection satisfies all RFC 7946 §3.3 constraints and each member
    /// Feature satisfies FeatureValid.
    ///
    /// Source: RFC 7946 §3.3 — FeatureCollection Object (aggregate seam)
    pub struct FeatureCollectionValid;
    structural_prop!(FeatureCollectionValid, "FeatureCollectionValid");

    /// A GeoJSON document satisfies every normative MUST and MUST NOT in RFC 7946.
    ///
    /// Source: RFC 7946 — complete standard (top composite)
    pub struct GeoJsonDocumentValid;
    structural_prop!(GeoJsonDocumentValid, "GeoJsonDocumentValid");

    /// Full RFC 7946 compliance alias; prefer GeoJsonDocumentValid for new proofs.
    ///
    /// Source: RFC 7946 — complete standard
    pub struct FullRfc7946Compliance;
    structural_prop!(FullRfc7946Compliance, "FullRfc7946Compliance");
}

pub use emit_impls::{
    AllPositionsInGeometryHaveConsistentDimension, AntimeridianCrossingGeometriesShouldBeSplit,
    AntimeridianCutPreservesTopology, AntimeridianLineStringSplitsToMultiLineString,
    AntimeridianNoLongitudeOutsideRange, AntimeridianPolygonSplitsToMultiPolygon,
    AntimeridianSplitPartDoesNotCrossLine, Bbox2dHasExactlyFourElements,
    Bbox2dOrderIsMinLonMinLatMaxLonMaxLat, Bbox3dHasExactlySixElements,
    Bbox3dOrderIsMinLonMinLatMinAltMaxLonMaxLatMaxAlt, BboxAllElementsAreJsonNumbers,
    BboxAntimeridianWestValueMayExceedEast, BboxDimensionMatchesGeometryDimension,
    BboxEdgesFollowConstantCoordinateLines, BboxElementsAreFinite, BboxLatitudeMustNotBelowNeg90,
    BboxLatitudeMustNotExceed90, BboxLengthIsTwiceN, BboxMemberIsOptional, BboxMinAltLeMaxAlt,
    BboxMinLatLeMaxLat, BboxMustNotUseOutOfRangeLatForSphericalCap,
    BboxShouldContainAllGeometryPositions, BboxSouthwestCornerPrecedesNortheast,
    BboxWhenPresentIsJsonArray, CoordPrecisionMustNotImplyAccuracyLevel,
    CoordPrecisionNotInDmsFormat, CoordPrecisionNotInSexagesimalFormat,
    CoordPrecisionSixDecimalPlacesIsSufficient, CoordinatePrecisionMustNotImplyUncertainty,
    CoordinatesAreInDecimalDegreesOnly, CrsAbsentAltitudeTreatedAsSurface,
    CrsAlternativeForbiddenWithoutArrangement, CrsAltitudeIsAboveWgs84Ellipsoid,
    CrsAxisOrderLatitudeSecond, CrsAxisOrderLongitudeFirst, CrsEquivalentToOgcUrn,
    CrsMemberMustNotAppearInDocument, CrsNoEpsgCodesPermitted, CrsNoNamedCrsObjectPermitted,
    CrsOnlyWgs84IsPermitted, DigitCountIsNotDataAccuracyIndicator,
    ExistingGeometrySemanticsMustNotChange, FeatureCollectionConsistentCoordinateDimension,
    FeatureCollectionEachElementIsFeatureObject, FeatureCollectionEachElementIsNotNull,
    FeatureCollectionFeaturesIsJsonArray, FeatureCollectionFeaturesIsNotNull,
    FeatureCollectionFeaturesIsNotSingleFeature, FeatureCollectionHasFeaturesMember,
    FeatureCollectionIdIsNotRequired, FeatureCollectionMayBeEmpty,
    FeatureCollectionSemanticsMustNotChange, FeatureCollectionTypeEqualsFeatureCollection,
    FeatureCollectionValid, FeatureGeometryIsGeometryObjectOrNull,
    FeatureGeometryWhenPresentIsValidGeometry, FeatureHasGeometryMember,
    FeatureHasPropertiesMember, FeatureIdMemberIsOptional, FeatureIdMustNotBeArray,
    FeatureIdMustNotBeBoolean, FeatureIdMustNotBeNull, FeatureIdMustNotBeObject,
    FeatureIdNumberIsAllowed, FeatureIdStringIsAllowed,
    FeatureIdUniquenessWithinCollectionRecommended, FeatureIdWhenPresentIsStringOrNumber,
    FeatureNullGeometryMeansUnlocated, FeaturePropertiesIsJsonObjectOrNull,
    FeatureSemanticsMustNotChange, FeatureTypeEqualsFeature, FeatureValid, FileExtensionIsGeojson,
    ForeignMembersAtAnyLevelShouldBeIgnored, ForeignMembersCannotOverrideTypeMember,
    ForeignMembersDoNotAlterSemantics, FullRfc7946Compliance, GeoJsonBboxMemberIsOptional,
    GeoJsonBboxWhenPresentIsArray, GeoJsonDocumentValid, GeoJsonForeignMembersShouldBeIgnored,
    GeoJsonGeometryCollectionMayBeEmpty, GeoJsonGeometryCollectionValid, GeoJsonGeometryValid,
    GeoJsonLineStringValid, GeoJsonMultiLineStringMayBeEmpty, GeoJsonMultiLineStringValid,
    GeoJsonMultiPointValid, GeoJsonMultiPolygonMayBeEmpty, GeoJsonMultiPolygonValid,
    GeoJsonObjectHasTypeMember, GeoJsonObjectMustNotUseNullType,
    GeoJsonObjectTypeMemberNeverMissing, GeoJsonPointMappableToGeoUri, GeoJsonPointValid,
    GeoJsonPolygonValid, GeoJsonRootIsObject, GeoJsonTextIsSingleJsonValue,
    GeoJsonTypeIsCaseSensitive, GeoJsonTypeIsNotExtensible, GeoJsonTypeIsOneOfNineValues,
    GeoJsonTypeMemberIsNotNull, GeoJsonTypeMemberIsString, GeoUriMappableToGeoJsonPoint,
    GeometryCollectionEachElementIsValidGeometry, GeometryCollectionEachGeometryHasTypeMember,
    GeometryCollectionGeometriesIsJsonArray, GeometryCollectionGeometriesIsNotNull,
    GeometryCollectionHasGeometriesMember, GeometryCollectionMustNotHaveCoordinatesMember,
    GeometryCollectionShouldNotBeNested, GeometryCollectionShouldNotReplaceMultiXType,
    GeometryCollectionTypeEqualsGeometryCollection, GeometryTypeValueMatchesCoordinateStructure,
    IJsonNoBomShouldBeAdded, IJsonNoDuplicateKeysAllowed, IJsonNoInfinityValues, IJsonNoNaNValues,
    IJsonNoUnescapedControlCharacters, IJsonNumbersMustBeIeee754Representable,
    IJsonStringsAreUnicode, IJsonUtf8EncodingRequired, LineStringCoordinatesHasMinTwoPositions,
    LineStringCoordinatesIsArray, LineStringCoordinatesIsNotEmpty, LineStringCoordinatesIsNotNull,
    LineStringEachElementIsValidPosition, LineStringHasCoordinatesMember,
    LineStringMinTwoPositionsFormPath, LineStringTypeEqualsLineString, MediaTypeCharsetIsUtf8,
    MediaTypeIsApplicationGeoJson, MediaTypeNotApplicationJson, MultiLineStringCoordinatesIsArray,
    MultiLineStringCoordinatesIsNotNull, MultiLineStringEachElementIsArray,
    MultiLineStringEachLineStringHasMinTwoPositions, MultiLineStringEachPositionIsValid,
    MultiLineStringHasCoordinatesMember, MultiLineStringTypeEqualsMultiLineString,
    MultiPointCoordinatesIsArrayOfPositions, MultiPointCoordinatesIsNotNull,
    MultiPointCoordinatesMayBeEmpty, MultiPointEachElementIsValidPosition,
    MultiPointHasCoordinatesMember, MultiPointTypeEqualsMultiPoint, MultiPolygonCoordinatesIsArray,
    MultiPolygonCoordinatesIsNotNull, MultiPolygonEachElementIsPolygonCoordinates,
    MultiPolygonEachPolygonObeysPolygonRules, MultiPolygonHasCoordinatesMember,
    MultiPolygonTypeEqualsMultiPolygon, NoNewGeometryTypesMayBeDefined,
    NoProjectedCoordinatesPermitted, NullGeometryDistinctFromAbsentGeometry,
    PointCoordinatesHasMinTwoElements, PointCoordinatesIsNotNestedArray, PointCoordinatesIsNotNull,
    PointCoordinatesIsSinglePosition, PointHasCoordinatesMember, PointTypeEqualsPoint,
    PolygonCoordinatesHasAtLeastOneRing, PolygonCoordinatesIsArrayOfRings,
    PolygonCoordinatesIsNotNull, PolygonExteriorRingHasPositiveSignedArea,
    PolygonExteriorRingIsCounterclockwise, PolygonFirstRingIsExteriorBoundary,
    PolygonHasCoordinatesMember, PolygonHoleRingHasNegativeSignedArea,
    PolygonHoleRingsAreClockwise, PolygonHolesAreInteriorToExteriorRing, PolygonHolesDoNotOverlap,
    PolygonLinearRingFirstAndLastAreIdentical, PolygonLinearRingHasMinFourPositions,
    PolygonLinearRingIsClosedLineString, PolygonLinearRingRepresentationShouldBeIdentical,
    PolygonRightHandRuleAreaToLeft, PolygonRingDoesNotSelfIntersect,
    PolygonSubsequentRingsAreInteriorHoles, PolygonTypeEqualsPolygon, PositionAltitudeIsFinite,
    PositionAltitudeReferencesWgs84Ellipsoid, PositionElementOneIsLatitude,
    PositionElementTwoIsAltitudeWhenPresent, PositionElementZeroIsLongitude,
    PositionElementsAreJsonNumbers, PositionExtraElementsBeyondThreeIgnored,
    PositionHasAtLeastTwoElements, PositionInterpolationIsCartesian, PositionIsJsonArray,
    PositionIsNotJsonNull, PositionIsNotJsonNumber, PositionIsNotJsonObject,
    PositionIsNotJsonString, PositionLatitudeInRange, PositionLatitudeIsFinite,
    PositionLongitudeInRange, PositionLongitudeIsFinite, PositionShouldNotExceedThreeElements,
    PositionValid, PropertiesIsPreferredLocationForAppData,
};

// ── Proof composition: RFC 7946 GeoJSON validity chain ───────────────────────

use elicitation::{Established, contracts::ProvableFrom};

// Single-dependency: a valid Position proves a GeoJSON Point valid.
//
// Source: RFC 7946 §3.1.2 — Point.
impl ProvableFrom<Established<PositionValid>> for GeoJsonPointValid {}

/// Evidence that a GeoJSON MultiPoint is valid.
///
/// Requires proven position validity for each element.
///
/// Source: RFC 7946 §3.1.5 — MultiPoint.
pub struct GeoJsonMultiPointEvidence {
    /// Proof for each position.
    pub positions: Vec<Established<PositionValid>>,
}

impl ProvableFrom<GeoJsonMultiPointEvidence> for GeoJsonMultiPointValid {}

/// Evidence that a GeoJSON LineString is valid.
///
/// Requires at least two proven valid positions.
///
/// Source: RFC 7946 §3.1.4 — LineString.
pub struct GeoJsonLineStringEvidence {
    /// Proof for each position (≥ 2).
    pub positions: Vec<Established<PositionValid>>,
}

impl ProvableFrom<GeoJsonLineStringEvidence> for GeoJsonLineStringValid {}

/// Evidence that a GeoJSON MultiLineString is valid.
///
/// Requires proven line-string validity for each member.
///
/// Source: RFC 7946 §3.1.5 — MultiLineString.
pub struct GeoJsonMultiLineStringEvidence {
    /// Proof for each line string.
    pub lines: Vec<Established<GeoJsonLineStringValid>>,
}

impl ProvableFrom<GeoJsonMultiLineStringEvidence> for GeoJsonMultiLineStringValid {}

/// Evidence that a GeoJSON Polygon is valid.
///
/// Requires proven position validity for the exterior ring positions and for
/// each interior ring.
///
/// Source: RFC 7946 §3.1.6 — Polygon.
pub struct GeoJsonPolygonEvidence {
    /// Proofs for the exterior ring positions (≥ 4, first == last).
    pub exterior: Vec<Established<PositionValid>>,
    /// Proofs for each hole's positions (≥ 4, first == last).
    pub holes: Vec<Vec<Established<PositionValid>>>,
}

impl ProvableFrom<GeoJsonPolygonEvidence> for GeoJsonPolygonValid {}

/// Evidence that a GeoJSON MultiPolygon is valid.
///
/// Requires proven polygon validity for each member.
///
/// Source: RFC 7946 §3.1.7 — MultiPolygon.
pub struct GeoJsonMultiPolygonEvidence {
    /// Proof for each polygon.
    pub polygons: Vec<Established<GeoJsonPolygonValid>>,
}

impl ProvableFrom<GeoJsonMultiPolygonEvidence> for GeoJsonMultiPolygonValid {}

/// Evidence that a GeoJSON GeometryCollection is valid.
///
/// Requires proven geometry validity for each member.
///
/// Source: RFC 7946 §3.1.8 — GeometryCollection.
pub struct GeoJsonGeometryCollectionEvidence {
    /// Proof for each member geometry.
    pub geometries: Vec<Established<GeoJsonGeometryValid>>,
}

impl ProvableFrom<GeoJsonGeometryCollectionEvidence> for GeoJsonGeometryCollectionValid {}

/// Evidence that a GeoJSON Feature is valid.
///
/// The `geometry` field is optional; `None` represents a null geometry.
///
/// Source: RFC 7946 §3.2 — Feature.
pub struct GeoJsonFeatureEvidence {
    /// Proof that the geometry object is valid, if present.
    pub geometry: Option<Established<GeoJsonGeometryValid>>,
}

impl ProvableFrom<GeoJsonFeatureEvidence> for FeatureValid {}

/// Evidence that a GeoJSON FeatureCollection is valid.
///
/// Requires proven feature validity for each member.
///
/// Source: RFC 7946 §3.3 — FeatureCollection.
pub struct GeoJsonFeatureCollectionEvidence {
    /// Proof for each feature.
    pub features: Vec<Established<FeatureValid>>,
}

impl ProvableFrom<GeoJsonFeatureCollectionEvidence> for FeatureCollectionValid {}

// ── Upcasts: concrete geometry proofs → GeoJsonGeometryValid ─────────────────
//
// Any proven concrete GeoJSON geometry type also proves the abstract
// GeoJsonGeometryValid superprop.

impl ProvableFrom<Established<GeoJsonPointValid>> for GeoJsonGeometryValid {}
impl ProvableFrom<Established<GeoJsonMultiPointValid>> for GeoJsonGeometryValid {}
impl ProvableFrom<Established<GeoJsonLineStringValid>> for GeoJsonGeometryValid {}
impl ProvableFrom<Established<GeoJsonMultiLineStringValid>> for GeoJsonGeometryValid {}
impl ProvableFrom<Established<GeoJsonPolygonValid>> for GeoJsonGeometryValid {}
impl ProvableFrom<Established<GeoJsonMultiPolygonValid>> for GeoJsonGeometryValid {}
impl ProvableFrom<Established<GeoJsonGeometryCollectionValid>> for GeoJsonGeometryValid {}

// ── Upcasts: root GeoJSON object types → GeoJsonDocumentValid ────────────────
//
// A valid geometry object, feature, or feature collection each proves the
// document-level superprop.

impl ProvableFrom<Established<GeoJsonGeometryValid>> for GeoJsonDocumentValid {}
impl ProvableFrom<Established<FeatureValid>> for GeoJsonDocumentValid {}
impl ProvableFrom<Established<FeatureCollectionValid>> for GeoJsonDocumentValid {}
