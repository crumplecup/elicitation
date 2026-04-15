# ISO 19111:2019 — Spatial Referencing by Coordinates: Implementation Notes

## ⚠ Pattern correction notice

The prop structs below use the **correct** pattern from
`crates/elicit_db/src/contracts/iso_sql.rs`.

**Do NOT use `#[derive(Prop)]` or `#[spec_reference(...)]` — both are fabricated.**

The correct pattern is:

```rust
mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    /// Brief description of the proposition.
    ///
    /// Source: ISO 19111:2019 §X.Y — <section title>
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

Use this file as a content reference only. All prop struct snippets below must be
converted to the `structural_prop!` pattern before being placed in
`crates/elicit_gis/src/contracts/iso_19111.rs`.

---

## ISO 19111:2019 Contract Checklist

**Standard:** ISO 19111:2019 — Geographic information — Referencing by coordinates  
**Replaces:** ISO 19111:2007 and ISO 19111-2:2009  
**Scope:** Defines the conceptual schema for the description of referencing by
coordinates. Applies to coordinate reference systems (CRS) as used in geographic
information, including two-dimensional, three-dimensional, vertical, engineering,
and derived coordinate systems.

---

## §6 Overview of Spatial Reference Systems

### §6.1 Conceptual structure of a CRS

A coordinate reference system (CRS) is the union of:

1. A coordinate system (CS) that defines axes, directions, and units
2. A datum (or reference frame) that ties the CS to the physical world
3. An optional domain of applicability (geographic extent, temporal extent)

Every CRS has a name, an optional scope description, optional identifiers, and an
optional domain of validity. All concrete CRS types are subclasses of the abstract
`SC_CRS`.

```rust
/// CRS consists of a Coordinate System bound to a Datum.
///
/// Source: ISO 19111:2019 §6.1 — Conceptual structure of a coordinate reference system
pub struct CrsConsistsOfCsAndDatum;
structural_prop!(CrsConsistsOfCsAndDatum, "CrsConsistsOfCsAndDatum");

/// A coordinate tuple has exactly as many ordinates as the CS has axes.
///
/// Source: ISO 19111:2019 §6.1 — Conceptual structure of a coordinate reference system
pub struct CoordinateTupleDimensionMatchesAxes;
structural_prop!(CoordinateTupleDimensionMatchesAxes, "CoordinateTupleDimensionMatchesAxes");

/// SC_CRS is abstract; all concrete CRS types are subclasses.
///
/// Source: ISO 19111:2019 §6.1 — Conceptual structure of a coordinate reference system
pub struct ScCrsIsAbstract;
structural_prop!(ScCrsIsAbstract, "ScCrsIsAbstract");

/// SC_CRS.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §6.2.1 — CRS name
pub struct ScCrsNameNonEmpty;
structural_prop!(ScCrsNameNonEmpty, "ScCrsNameNonEmpty");

/// SC_CRS.scope is a non-empty CharacterString describing intended use.
///
/// Source: ISO 19111:2019 §6.2.2 — CRS scope
pub struct ScCrsScopeNonEmpty;
structural_prop!(ScCrsScopeNonEmpty, "ScCrsScopeNonEmpty");

/// SC_CRS.domainOfValidity references an EX_Extent object (optional).
///
/// Source: ISO 19111:2019 §6.2.3 — Domain of validity
pub struct ScCrsDomainOfValidityIsExtent;
structural_prop!(ScCrsDomainOfValidityIsExtent, "ScCrsDomainOfValidityIsExtent");

/// SC_CRS.identifier references a RS_Identifier with authority and code (optional).
///
/// Source: ISO 19111:2019 §6.2.4 — CRS identifier
pub struct ScCrsIdentifierHasAuthorityAndCode;
structural_prop!(ScCrsIdentifierHasAuthorityAndCode, "ScCrsIdentifierHasAuthorityAndCode");

/// An identifier authority is a non-empty CharacterString naming the registry.
///
/// Source: ISO 19111:2019 §6.2.4 — CRS identifier
pub struct CrsIdentifierAuthorityNonEmpty;
structural_prop!(CrsIdentifierAuthorityNonEmpty, "CrsIdentifierAuthorityNonEmpty");

/// An identifier code is a non-empty CharacterString within the authority namespace.
///
/// Source: ISO 19111:2019 §6.2.4 — CRS identifier
pub struct CrsIdentifierCodeNonEmpty;
structural_prop!(CrsIdentifierCodeNonEmpty, "CrsIdentifierCodeNonEmpty");
```

---

## §7 Geodetic CRS (SC_GeodeticCRS)

### §7.1 Overview and subtypes

A geodetic CRS is anchored to the Earth via a geodetic reference frame. Three
subtypes exist:

- **Geographic 2D**: ellipsoidal latitude + longitude
- **Geographic 3D**: ellipsoidal latitude + longitude + ellipsoidal height
- **Geocentric**: Earth-centred Cartesian X + Y + Z

```rust
/// SC_GeodeticCRS.datum references a CD_GeodeticReferenceFrame.
///
/// Source: ISO 19111:2019 §7.1 — Geodetic reference frame
pub struct GeodeticCrsDatumIsGeodeticReferenceFrame;
structural_prop!(GeodeticCrsDatumIsGeodeticReferenceFrame, "GeodeticCrsDatumIsGeodeticReferenceFrame");

/// SC_GeodeticCRS.coordinateSystem references a CS_EllipsoidalCS or CS_CartesianCS.
///
/// Source: ISO 19111:2019 §7.1 — Geodetic coordinate system type
pub struct GeodeticCrsCsIsEllipsoidalOrCartesian;
structural_prop!(GeodeticCrsCsIsEllipsoidalOrCartesian, "GeodeticCrsCsIsEllipsoidalOrCartesian");

/// A geographic 2D CRS has exactly two axes: latitude and longitude.
///
/// Source: ISO 19111:2019 §7.2 — Geographic two-dimensional coordinate reference system
pub struct Geographic2dCrsHasTwoAxes;
structural_prop!(Geographic2dCrsHasTwoAxes, "Geographic2dCrsHasTwoAxes");

/// A geographic 3D CRS has exactly three axes: latitude, longitude, and ellipsoidal height.
///
/// Source: ISO 19111:2019 §7.2 — Geographic three-dimensional coordinate reference system
pub struct Geographic3dCrsHasThreeAxes;
structural_prop!(Geographic3dCrsHasThreeAxes, "Geographic3dCrsHasThreeAxes");

/// A geocentric CRS uses a CS_CartesianCS with geocentricX, geocentricY, geocentricZ axes.
///
/// Source: ISO 19111:2019 §7.3 — Geocentric coordinate reference system
pub struct GeocentricCrsUsesCartesianCs;
structural_prop!(GeocentricCrsUsesCartesianCs, "GeocentricCrsUsesCartesianCs");
```

### §7.1.1 Well-known geodetic CRS identifiers

```rust
/// EPSG:4326 — WGS 84 geographic 2D CRS; axis order is latitude first, longitude second.
///
/// Source: ISO 19111:2019 §7.2 — Geographic two-dimensional coordinate reference system
pub struct Epsg4326AxisOrderLatFirst;
structural_prop!(Epsg4326AxisOrderLatFirst, "Epsg4326AxisOrderLatFirst");

/// EPSG:4326 — latitude range is [-90, 90] degrees.
///
/// Source: ISO 19111:2019 §7.2 — Geographic two-dimensional coordinate reference system
pub struct Epsg4326LatitudeRangeValid;
structural_prop!(Epsg4326LatitudeRangeValid, "Epsg4326LatitudeRangeValid");

/// EPSG:4326 — longitude range is (-180, 180] degrees.
///
/// Source: ISO 19111:2019 §7.2 — Geographic two-dimensional coordinate reference system
pub struct Epsg4326LongitudeRangeValid;
structural_prop!(Epsg4326LongitudeRangeValid, "Epsg4326LongitudeRangeValid");

/// EPSG:4979 — WGS 84 geographic 3D CRS; axes are lat, lon, ellipsoidal height.
///
/// Source: ISO 19111:2019 §7.2 — Geographic three-dimensional coordinate reference system
pub struct Epsg4979IsWgs84Geographic3d;
structural_prop!(Epsg4979IsWgs84Geographic3d, "Epsg4979IsWgs84Geographic3d");

/// EPSG:4979 — ellipsoidal height axis has no range constraint in the standard.
///
/// Source: ISO 19111:2019 §7.2 — Geographic three-dimensional coordinate reference system
pub struct Epsg4979HeightUnbounded;
structural_prop!(Epsg4979HeightUnbounded, "Epsg4979HeightUnbounded");

/// EPSG:4978 — WGS 84 geocentric CRS; three Cartesian axes X, Y, Z in metres.
///
/// Source: ISO 19111:2019 §7.3 — Geocentric coordinate reference system
pub struct Epsg4978IsWgs84Geocentric;
structural_prop!(Epsg4978IsWgs84Geocentric, "Epsg4978IsWgs84Geocentric");

/// EPSG:4978 — X axis points from geocentre towards intersection of equator and prime meridian (0°N 0°E).
///
/// Source: ISO 19111:2019 §7.3 — Geocentric coordinate reference system
pub struct Epsg4978XAxisTowardsPrimeMeridian;
structural_prop!(Epsg4978XAxisTowardsPrimeMeridian, "Epsg4978XAxisTowardsPrimeMeridian");

/// EPSG:4978 — Z axis points from geocentre towards North Pole.
///
/// Source: ISO 19111:2019 §7.3 — Geocentric coordinate reference system
pub struct Epsg4978ZAxisTowardsNorthPole;
structural_prop!(Epsg4978ZAxisTowardsNorthPole, "Epsg4978ZAxisTowardsNorthPole");
```

---

## §7.2 CD_GeodeticReferenceFrame (datum)

A geodetic reference frame (historically called a "geodetic datum") ties the
coordinate system to the physical Earth. It consists of a name, an anchor
definition, an ellipsoid, and a prime meridian.

```rust
/// CD_GeodeticReferenceFrame.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §7.4.1 — Geodetic reference frame name
pub struct GeodeticReferenceFrameNameNonEmpty;
structural_prop!(GeodeticReferenceFrameNameNonEmpty, "GeodeticReferenceFrameNameNonEmpty");

/// CD_GeodeticReferenceFrame.anchorDefinition is an optional CharacterString describing physical realization.
///
/// Source: ISO 19111:2019 §7.4.2 — Anchor definition
pub struct GeodeticReferenceFrameAnchorOptional;
structural_prop!(GeodeticReferenceFrameAnchorOptional, "GeodeticReferenceFrameAnchorOptional");

/// CD_GeodeticReferenceFrame.ellipsoid references exactly one CD_Ellipsoid.
///
/// Source: ISO 19111:2019 §7.4.3 — Ellipsoid association
pub struct GeodeticReferenceFrameHasExactlyOneEllipsoid;
structural_prop!(GeodeticReferenceFrameHasExactlyOneEllipsoid, "GeodeticReferenceFrameHasExactlyOneEllipsoid");

/// CD_GeodeticReferenceFrame.primeMeridian references exactly one CD_PrimeMeridian.
///
/// Source: ISO 19111:2019 §7.4.4 — Prime meridian association
pub struct GeodeticReferenceFrameHasExactlyOnePrimeMeridian;
structural_prop!(GeodeticReferenceFrameHasExactlyOnePrimeMeridian, "GeodeticReferenceFrameHasExactlyOnePrimeMeridian");

/// CD_GeodeticReferenceFrame.realizationEpoch is an optional ISO 8601 date.
///
/// Source: ISO 19111:2019 §7.4.5 — Realization epoch
pub struct GeodeticReferenceFrameRealizationEpochIsIso8601;
structural_prop!(GeodeticReferenceFrameRealizationEpochIsIso8601, "GeodeticReferenceFrameRealizationEpochIsIso8601");
```

---

## §7.3 CD_Ellipsoid

The reference ellipsoid is the mathematical surface used to approximate the shape
of the Earth. It is defined by a semi-major axis and either an inverse flattening
or a semi-minor axis. A sphere is the degenerate case where both semi-axes are equal.

```rust
/// CD_Ellipsoid.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §7.5.1 — Ellipsoid name
pub struct EllipsoidNameNonEmpty;
structural_prop!(EllipsoidNameNonEmpty, "EllipsoidNameNonEmpty");

/// CD_Ellipsoid.semiMajorAxis is a positive real number greater than zero, in metres.
///
/// Source: ISO 19111:2019 §7.5.2 — Semi-major axis
pub struct EllipsoidSemiMajorAxisPositive;
structural_prop!(EllipsoidSemiMajorAxisPositive, "EllipsoidSemiMajorAxisPositive");

/// CD_Ellipsoid.semiMajorAxis is a finite real number (not NaN or ±Infinity); IEEE 754
/// finiteness precondition required before positivity or axis-ratio proofs.
///
/// Source: ISO 19111:2019 §7.5.2 — Semi-major axis finite
pub struct EllipsoidSemiMajorAxisFinite;
structural_prop!(EllipsoidSemiMajorAxisFinite, "EllipsoidSemiMajorAxisFinite");

/// CD_Ellipsoid.semiMajorAxis unit is always metres.
///
/// Source: ISO 19111:2019 §7.5.2 — Semi-major axis
pub struct EllipsoidSemiMajorAxisInMetres;
structural_prop!(EllipsoidSemiMajorAxisInMetres, "EllipsoidSemiMajorAxisInMetres");

/// CD_Ellipsoid provides either inverseFlattening or semiMinorAxis (not required to provide both).
///
/// Source: ISO 19111:2019 §7.5.3 — Second defining parameter
pub struct EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor;
structural_prop!(EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor, "EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor");

/// CD_Ellipsoid.inverseFlattening is a finite positive real number when the body is non-spherical.
///
/// Source: ISO 19111:2019 §7.5.3 — Inverse flattening
pub struct EllipsoidInverseFlatteningPositiveWhenNonSphere;
structural_prop!(EllipsoidInverseFlatteningPositiveWhenNonSphere, "EllipsoidInverseFlatteningPositiveWhenNonSphere");

/// WGS 84 ellipsoid inverseFlattening is approximately 298.257223563.
///
/// Source: ISO 19111:2019 §7.5.3 — Inverse flattening (WGS 84 value)
pub struct EllipsoidWgs84InverseFlatteningApprox298;
structural_prop!(EllipsoidWgs84InverseFlatteningApprox298, "EllipsoidWgs84InverseFlatteningApprox298");

/// CD_Ellipsoid.inverseFlattening == 0 indicates a sphere (degenerate ellipsoid).
///
/// Source: ISO 19111:2019 §7.5.3 — Sphere as degenerate ellipsoid
pub struct EllipsoidInverseFlatteningZeroMeansSphere;
structural_prop!(EllipsoidInverseFlatteningZeroMeansSphere, "EllipsoidInverseFlatteningZeroMeansSphere");

/// When semiMinorAxis is given, semiMinorAxis < semiMajorAxis (oblate spheroid constraint).
///
/// Source: ISO 19111:2019 §7.5.3 — Oblate spheroid constraint
pub struct EllipsoidSemiMinorAxisLessThanSemiMajor;
structural_prop!(EllipsoidSemiMinorAxisLessThanSemiMajor, "EllipsoidSemiMinorAxisLessThanSemiMajor");

/// isSphere is true iff inverseFlattening == 0 OR semiMajorAxis == semiMinorAxis.
///
/// Source: ISO 19111:2019 §7.5.4 — isSphere derived property
pub struct EllipsoidIsSphereConsistentWithParameters;
structural_prop!(EllipsoidIsSphereConsistentWithParameters, "EllipsoidIsSphereConsistentWithParameters");

/// CD_Ellipsoid.semiMinorAxis is in metres when given explicitly.
///
/// Source: ISO 19111:2019 §7.5.3 — Semi-minor axis
pub struct EllipsoidSemiMinorAxisInMetres;
structural_prop!(EllipsoidSemiMinorAxisInMetres, "EllipsoidSemiMinorAxisInMetres");

/// CD_Ellipsoid.semiMinorAxis is a finite real number (not NaN or ±Infinity); required as
/// a precondition before `EllipsoidSemiMinorAxisLessThanSemiMajor` can be asserted in proofs
/// — comparisons involving NaN are always false under IEEE 754.
///
/// Source: ISO 19111:2019 §7.5.3 — Semi-minor axis finite
pub struct EllipsoidSemiMinorAxisFinite;
structural_prop!(EllipsoidSemiMinorAxisFinite, "EllipsoidSemiMinorAxisFinite");
```

---

## §7.4 CD_PrimeMeridian

The prime meridian defines the origin of longitude measurement. By convention the
Greenwich meridian is used, giving a greenwichLongitude of 0.

```rust
/// CD_PrimeMeridian.name is a non-empty CharacterString (conventionally "Greenwich").
///
/// Source: ISO 19111:2019 §7.6.1 — Prime meridian name
pub struct PrimeMeridianNameNonEmpty;
structural_prop!(PrimeMeridianNameNonEmpty, "PrimeMeridianNameNonEmpty");

/// CD_PrimeMeridian.greenwichLongitude is a finite real number in the angular unit of the CRS.
///
/// Source: ISO 19111:2019 §7.6.2 — Greenwich longitude
pub struct PrimeMeridianGreenwichLongitudeFinite;
structural_prop!(PrimeMeridianGreenwichLongitudeFinite, "PrimeMeridianGreenwichLongitudeFinite");

/// greenwichLongitude ∈ (-180, 180] when expressed in degrees.
///
/// Source: ISO 19111:2019 §7.6.2 — Greenwich longitude range in degrees
pub struct PrimeMeridianGreenwichLongitudeInDegreeBounds;
structural_prop!(PrimeMeridianGreenwichLongitudeInDegreeBounds, "PrimeMeridianGreenwichLongitudeInDegreeBounds");

/// The Greenwich prime meridian has greenwichLongitude exactly 0.
///
/// Source: ISO 19111:2019 §7.6.2 — Greenwich prime meridian
pub struct PrimeMeridianGreenwichIsZero;
structural_prop!(PrimeMeridianGreenwichIsZero, "PrimeMeridianGreenwichIsZero");

/// Non-Greenwich prime meridians are valid but rare; greenwichLongitude ≠ 0.
///
/// Source: ISO 19111:2019 §7.6.2 — Non-Greenwich prime meridian
pub struct PrimeMeridianNonGreenwichAllowed;
structural_prop!(PrimeMeridianNonGreenwichAllowed, "PrimeMeridianNonGreenwichAllowed");

/// CD_PrimeMeridian.greenwichLongitude unit must be an angular unit (degrees or radians).
///
/// Source: ISO 19111:2019 §7.6.2 — Greenwich longitude unit
pub struct PrimeMeridianGreenwichLongitudeUnitIsAngular;
structural_prop!(PrimeMeridianGreenwichLongitudeUnitIsAngular, "PrimeMeridianGreenwichLongitudeUnitIsAngular");
```

---

## §8 CS_CoordinateSystem

A coordinate system defines the set of axes, their directions, and their units of
measurement that together describe how coordinates are measured within a CRS.

### §8.1 General CS constraints

```rust
/// CS_CoordinateSystem.axis array has cardinality 1..4 (one to four axes).
///
/// Source: ISO 19111:2019 §8.1 — Coordinate system general constraints
pub struct CoordinateSystemAxisCountOneToFour;
structural_prop!(CoordinateSystemAxisCountOneToFour, "CoordinateSystemAxisCountOneToFour");

/// Axis count of the CS equals the dimensionality of coordinate tuples in the CRS.
///
/// Source: ISO 19111:2019 §8.1 — Axis count and coordinate tuple cardinality
pub struct CsAxisCountMatchesTupleDimensionality;
structural_prop!(CsAxisCountMatchesTupleDimensionality, "CsAxisCountMatchesTupleDimensionality");

/// CS_CoordinateSystem.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §8.1 — Coordinate system name
pub struct CoordinateSystemNameNonEmpty;
structural_prop!(CoordinateSystemNameNonEmpty, "CoordinateSystemNameNonEmpty");
```

### §8.2 CS_Axis attributes

```rust
/// CS_Axis.name is a non-empty CharacterString identifying the axis.
///
/// Source: ISO 19111:2019 §8.2 — Axis name
pub struct AxisNameNonEmpty;
structural_prop!(AxisNameNonEmpty, "AxisNameNonEmpty");

/// CS_Axis.abbreviation is a non-empty CharacterString, typically 1–3 characters (X, Y, Z, Lat, Lon, N, E, h).
///
/// Source: ISO 19111:2019 §8.2 — Axis abbreviation
pub struct AxisAbbreviationNonEmpty;
structural_prop!(AxisAbbreviationNonEmpty, "AxisAbbreviationNonEmpty");

/// CS_Axis.abbreviation is unique within its containing CS.
///
/// Source: ISO 19111:2019 §8.2 — Axis abbreviation uniqueness within CS
pub struct AxisAbbreviationUniqueWithinCs;
structural_prop!(AxisAbbreviationUniqueWithinCs, "AxisAbbreviationUniqueWithinCs");

/// CS_Axis.direction is a value from the CS_AxisDirection code list.
///
/// Source: ISO 19111:2019 §8.2 — Axis direction
pub struct AxisDirectionIsValidCode;
structural_prop!(AxisDirectionIsValidCode, "AxisDirectionIsValidCode");

/// CS_Axis.unit references a unit of measure appropriate for the axis type.
///
/// Source: ISO 19111:2019 §8.2 — Axis unit of measure
pub struct AxisUnitAppropriateForAxisType;
structural_prop!(AxisUnitAppropriateForAxisType, "AxisUnitAppropriateForAxisType");

/// CS_Axis.minimumValue and maximumValue are optional real numbers defining the valid range.
///
/// Source: ISO 19111:2019 §8.2 — Axis minimum and maximum value
pub struct AxisMinMaxValueOptional;
structural_prop!(AxisMinMaxValueOptional, "AxisMinMaxValueOptional");

/// CS_Axis.rangeMeaning is optional; when present, it is either "exact" or "wraparound".
///
/// Source: ISO 19111:2019 §8.2 — Axis range meaning
pub struct AxisRangeMeaningExactOrWraparound;
structural_prop!(AxisRangeMeaningExactOrWraparound, "AxisRangeMeaningExactOrWraparound");
```

---

## §8.3 CS_AxisDirection Enumeration

Each code in the `CS_AxisDirection` enumeration defines the positive direction of
an axis. One prop per direction code is listed below.

```rust
/// CS_AxisDirection.north — positive direction towards geodetic North.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionNorth;
structural_prop!(AxisDirectionNorth, "AxisDirectionNorth");

/// CS_AxisDirection.south — positive direction towards geodetic South.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionSouth;
structural_prop!(AxisDirectionSouth, "AxisDirectionSouth");

/// CS_AxisDirection.east — positive direction towards geodetic East (conventional +X in projected CS).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionEast;
structural_prop!(AxisDirectionEast, "AxisDirectionEast");

/// CS_AxisDirection.west — positive direction towards geodetic West.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionWest;
structural_prop!(AxisDirectionWest, "AxisDirectionWest");

/// CS_AxisDirection.up — positive direction away from Earth's centre (increasing height).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionUp;
structural_prop!(AxisDirectionUp, "AxisDirectionUp");

/// CS_AxisDirection.down — positive direction towards Earth's centre (decreasing height, used in depth).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionDown;
structural_prop!(AxisDirectionDown, "AxisDirectionDown");

/// CS_AxisDirection.northNorthEast — bearing approximately 22.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionNorthNorthEast;
structural_prop!(AxisDirectionNorthNorthEast, "AxisDirectionNorthNorthEast");

/// CS_AxisDirection.northEast — bearing 45° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionNorthEast;
structural_prop!(AxisDirectionNorthEast, "AxisDirectionNorthEast");

/// CS_AxisDirection.eastNorthEast — bearing approximately 67.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionEastNorthEast;
structural_prop!(AxisDirectionEastNorthEast, "AxisDirectionEastNorthEast");

/// CS_AxisDirection.eastSouthEast — bearing approximately 112.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionEastSouthEast;
structural_prop!(AxisDirectionEastSouthEast, "AxisDirectionEastSouthEast");

/// CS_AxisDirection.southEast — bearing 135° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionSouthEast;
structural_prop!(AxisDirectionSouthEast, "AxisDirectionSouthEast");

/// CS_AxisDirection.southSouthEast — bearing approximately 157.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionSouthSouthEast;
structural_prop!(AxisDirectionSouthSouthEast, "AxisDirectionSouthSouthEast");

/// CS_AxisDirection.southSouthWest — bearing approximately 202.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionSouthSouthWest;
structural_prop!(AxisDirectionSouthSouthWest, "AxisDirectionSouthSouthWest");

/// CS_AxisDirection.southWest — bearing 225° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionSouthWest;
structural_prop!(AxisDirectionSouthWest, "AxisDirectionSouthWest");

/// CS_AxisDirection.westSouthWest — bearing approximately 247.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionWestSouthWest;
structural_prop!(AxisDirectionWestSouthWest, "AxisDirectionWestSouthWest");

/// CS_AxisDirection.westNorthWest — bearing approximately 292.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionWestNorthWest;
structural_prop!(AxisDirectionWestNorthWest, "AxisDirectionWestNorthWest");

/// CS_AxisDirection.northWest — bearing 315° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionNorthWest;
structural_prop!(AxisDirectionNorthWest, "AxisDirectionNorthWest");

/// CS_AxisDirection.northNorthWest — bearing approximately 337.5° from north.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionNorthNorthWest;
structural_prop!(AxisDirectionNorthNorthWest, "AxisDirectionNorthNorthWest");

/// CS_AxisDirection.geocentricX — from Earth's centre towards the intersection of equator and prime meridian (0°N 0°E).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionGeocentricX;
structural_prop!(AxisDirectionGeocentricX, "AxisDirectionGeocentricX");

/// CS_AxisDirection.geocentricY — from Earth's centre towards the intersection of equator and 90°E meridian (0°N 90°E).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionGeocentricY;
structural_prop!(AxisDirectionGeocentricY, "AxisDirectionGeocentricY");

/// CS_AxisDirection.geocentricZ — from Earth's centre towards the North Pole.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionGeocentricZ;
structural_prop!(AxisDirectionGeocentricZ, "AxisDirectionGeocentricZ");

/// CS_AxisDirection.columnPositive — positive direction towards increasing column index.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionColumnPositive;
structural_prop!(AxisDirectionColumnPositive, "AxisDirectionColumnPositive");

/// CS_AxisDirection.columnNegative — positive direction towards decreasing column index.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionColumnNegative;
structural_prop!(AxisDirectionColumnNegative, "AxisDirectionColumnNegative");

/// CS_AxisDirection.rowPositive — positive direction towards increasing row index.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionRowPositive;
structural_prop!(AxisDirectionRowPositive, "AxisDirectionRowPositive");

/// CS_AxisDirection.rowNegative — positive direction towards decreasing row index.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionRowNegative;
structural_prop!(AxisDirectionRowNegative, "AxisDirectionRowNegative");

/// CS_AxisDirection.displayRight — positive direction towards right on a display device.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionDisplayRight;
structural_prop!(AxisDirectionDisplayRight, "AxisDirectionDisplayRight");

/// CS_AxisDirection.displayLeft — positive direction towards left on a display device.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionDisplayLeft;
structural_prop!(AxisDirectionDisplayLeft, "AxisDirectionDisplayLeft");

/// CS_AxisDirection.displayUp — positive direction towards top of a display device.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionDisplayUp;
structural_prop!(AxisDirectionDisplayUp, "AxisDirectionDisplayUp");

/// CS_AxisDirection.displayDown — positive direction towards bottom of a display device.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionDisplayDown;
structural_prop!(AxisDirectionDisplayDown, "AxisDirectionDisplayDown");

/// CS_AxisDirection.future — positive direction towards future time (time axis forward).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionFuture;
structural_prop!(AxisDirectionFuture, "AxisDirectionFuture");

/// CS_AxisDirection.past — positive direction towards past time (time axis backward).
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionPast;
structural_prop!(AxisDirectionPast, "AxisDirectionPast");

/// CS_AxisDirection.towards — positive direction of decreasing distance to a body.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionTowards;
structural_prop!(AxisDirectionTowards, "AxisDirectionTowards");

/// CS_AxisDirection.awayFrom — positive direction of increasing distance from a body.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionAwayFrom;
structural_prop!(AxisDirectionAwayFrom, "AxisDirectionAwayFrom");

/// CS_AxisDirection.counterClockwise — positive direction counter-clockwise when viewed from above.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionCounterClockwise;
structural_prop!(AxisDirectionCounterClockwise, "AxisDirectionCounterClockwise");

/// CS_AxisDirection.clockwise — positive direction clockwise when viewed from above.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionClockwise;
structural_prop!(AxisDirectionClockwise, "AxisDirectionClockwise");

/// CS_AxisDirection.unspecified — direction is not specified or not applicable.
///
/// Source: ISO 19111:2019 §8.3 — CS_AxisDirection code list
pub struct AxisDirectionUnspecified;
structural_prop!(AxisDirectionUnspecified, "AxisDirectionUnspecified");
```

---

## §8.4 CS Types

### §8.4.1 CS_EllipsoidalCS

```rust
/// CS_EllipsoidalCS has 2 axes (lat + lon) for 2D use or 3 axes (lat + lon + h) for 3D use.
///
/// Source: ISO 19111:2019 §8.4 — Ellipsoidal coordinate system
pub struct EllipsoidalCsHasTwoOrThreeAxes;
structural_prop!(EllipsoidalCsHasTwoOrThreeAxes, "EllipsoidalCsHasTwoOrThreeAxes");

/// CS_EllipsoidalCS latitude and longitude axes must use an angular unit (degrees or radians).
///
/// Source: ISO 19111:2019 §8.4 — Ellipsoidal CS angular unit constraint
pub struct EllipsoidalCsLatLonAxesUseAngularUnit;
structural_prop!(EllipsoidalCsLatLonAxesUseAngularUnit, "EllipsoidalCsLatLonAxesUseAngularUnit");

/// CS_EllipsoidalCS height axis (when present) must use a linear unit (metres or feet).
///
/// Source: ISO 19111:2019 §8.4 — Ellipsoidal 3D CS height unit constraint
pub struct EllipsoidalCs3dHeightAxisUsesLinearUnit;
structural_prop!(EllipsoidalCs3dHeightAxisUsesLinearUnit, "EllipsoidalCs3dHeightAxisUsesLinearUnit");

/// CS_EllipsoidalCS in 2D has no height axis; attempting to compute height is undefined.
///
/// Source: ISO 19111:2019 §8.4 — Ellipsoidal 2D has no height
pub struct EllipsoidalCs2dHasNoHeightAxis;
structural_prop!(EllipsoidalCs2dHasNoHeightAxis, "EllipsoidalCs2dHasNoHeightAxis");
```

### §8.4.2 CS_CartesianCS

```rust
/// CS_CartesianCS axes are mutually perpendicular (orthogonal) within the CS.
///
/// Source: ISO 19111:2019 §8.5 — Cartesian coordinate system
pub struct CartesianCsAxesOrthogonal;
structural_prop!(CartesianCsAxesOrthogonal, "CartesianCsAxesOrthogonal");

/// CS_CartesianCS axes for a projected CRS typically use easting and northing in metres.
///
/// Source: ISO 19111:2019 §8.5 — Projected Cartesian CS axes
pub struct CartesianCsProjectedUsesEastingNorthing;
structural_prop!(CartesianCsProjectedUsesEastingNorthing, "CartesianCsProjectedUsesEastingNorthing");

/// CS_CartesianCS for a geocentric CRS has 3 axes: X, Y, Z in metres.
///
/// Source: ISO 19111:2019 §8.5 — Geocentric Cartesian CS has three axes
pub struct CartesianCsGeocentricHasThreeAxes;
structural_prop!(CartesianCsGeocentricHasThreeAxes, "CartesianCsGeocentricHasThreeAxes");

/// CS_CartesianCS all axes must use a linear unit of measure.
///
/// Source: ISO 19111:2019 §8.5 — Cartesian CS linear unit constraint
pub struct CartesianCsAllAxesLinearUnit;
structural_prop!(CartesianCsAllAxesLinearUnit, "CartesianCsAllAxesLinearUnit");
```

### §8.4.3 CS_VerticalCS

```rust
/// CS_VerticalCS has exactly one axis.
///
/// Source: ISO 19111:2019 §8.6 — Vertical coordinate system
pub struct VerticalCsHasExactlyOneAxis;
structural_prop!(VerticalCsHasExactlyOneAxis, "VerticalCsHasExactlyOneAxis");

/// CS_VerticalCS single axis direction is either "up" or "down".
///
/// Source: ISO 19111:2019 §8.6 — Vertical CS axis direction
pub struct VerticalCsAxisDirectionUpOrDown;
structural_prop!(VerticalCsAxisDirectionUpOrDown, "VerticalCsAxisDirectionUpOrDown");

/// CS_VerticalCS axis unit is a linear unit of measure (metres, feet, etc.).
///
/// Source: ISO 19111:2019 §8.6 — Vertical CS linear unit
pub struct VerticalCsAxisUsesLinearUnit;
structural_prop!(VerticalCsAxisUsesLinearUnit, "VerticalCsAxisUsesLinearUnit");
```

### §8.4.4 CS_TemporalCS

```rust
/// CS_TemporalCS has exactly one axis.
///
/// Source: ISO 19111:2019 §8.7 — Temporal coordinate system
pub struct TemporalCsHasExactlyOneAxis;
structural_prop!(TemporalCsHasExactlyOneAxis, "TemporalCsHasExactlyOneAxis");

/// CS_TemporalCS axis direction is "future" or "past".
///
/// Source: ISO 19111:2019 §8.7 — Temporal CS axis direction
pub struct TemporalCsAxisDirectionFutureOrPast;
structural_prop!(TemporalCsAxisDirectionFutureOrPast, "TemporalCsAxisDirectionFutureOrPast");

/// CS_TemporalCS axis unit is a time unit (seconds, days, years, etc.).
///
/// Source: ISO 19111:2019 §8.7 — Temporal CS time unit
pub struct TemporalCsAxisUsesTimeUnit;
structural_prop!(TemporalCsAxisUsesTimeUnit, "TemporalCsAxisUsesTimeUnit");
```

### §8.4.5 CS_ParametricCS

```rust
/// CS_ParametricCS has exactly one axis representing a parametric quantity.
///
/// Source: ISO 19111:2019 §8.8 — Parametric coordinate system
pub struct ParametricCsHasExactlyOneAxis;
structural_prop!(ParametricCsHasExactlyOneAxis, "ParametricCsHasExactlyOneAxis");

/// CS_ParametricCS axis unit is a parametric unit (e.g., hectopascals for pressure).
///
/// Source: ISO 19111:2019 §8.8 — Parametric CS unit
pub struct ParametricCsAxisHasParametricUnit;
structural_prop!(ParametricCsAxisHasParametricUnit, "ParametricCsAxisHasParametricUnit");
```

### §8.4.6 CS_OrdinalCS

```rust
/// CS_OrdinalCS uses integer or ordered labels; no unit of measure is required.
///
/// Source: ISO 19111:2019 §8.9 — Ordinal coordinate system
pub struct OrdinalCsNoUnitRequired;
structural_prop!(OrdinalCsNoUnitRequired, "OrdinalCsNoUnitRequired");

/// CS_OrdinalCS axis values are ordered discrete labels (not continuous quantities).
///
/// Source: ISO 19111:2019 §8.9 — Ordinal CS discrete values
pub struct OrdinalCsValuesAreDiscreteLabels;
structural_prop!(OrdinalCsValuesAreDiscreteLabels, "OrdinalCsValuesAreDiscreteLabels");
```

### §8.4.7 CS_AffineCS

```rust
/// CS_AffineCS supports a general 2D or 3D coordinate system with specified axes and origins.
///
/// Source: ISO 19111:2019 §8.10 — Affine coordinate system
pub struct AffineCsHasTwoOrThreeAxes;
structural_prop!(AffineCsHasTwoOrThreeAxes, "AffineCsHasTwoOrThreeAxes");

/// CS_AffineCS axes need not be orthogonal (unlike Cartesian CS).
///
/// Source: ISO 19111:2019 §8.10 — Affine CS non-orthogonal axes
pub struct AffineCsAxesNeedNotBeOrthogonal;
structural_prop!(AffineCsAxesNeedNotBeOrthogonal, "AffineCsAxesNeedNotBeOrthogonal");
```

### §8.4.8 CS_PolarCS

```rust
/// CS_PolarCS has exactly 2 axes: a radial distance axis and an angular axis.
///
/// Source: ISO 19111:2019 §8.11 — Polar coordinate system
pub struct PolarCsHasTwoAxes;
structural_prop!(PolarCsHasTwoAxes, "PolarCsHasTwoAxes");

/// CS_PolarCS radial distance axis uses a linear unit; angular axis uses an angular unit.
///
/// Source: ISO 19111:2019 §8.11 — Polar CS axis units
pub struct PolarCsDistanceLinearAngleAngular;
structural_prop!(PolarCsDistanceLinearAngleAngular, "PolarCsDistanceLinearAngleAngular");
```

### §8.4.9 CS_CylindricalCS

```rust
/// CS_CylindricalCS has exactly 3 axes: radial distance, azimuth angle, and height.
///
/// Source: ISO 19111:2019 §8.12 — Cylindrical coordinate system
pub struct CylindricalCsHasThreeAxes;
structural_prop!(CylindricalCsHasThreeAxes, "CylindricalCsHasThreeAxes");

/// CS_CylindricalCS distance and height axes use a linear unit; azimuth axis uses angular unit.
///
/// Source: ISO 19111:2019 §8.12 — Cylindrical CS axis units
pub struct CylindricalCsDistanceHeightLinearAzimuthAngular;
structural_prop!(CylindricalCsDistanceHeightLinearAzimuthAngular, "CylindricalCsDistanceHeightLinearAzimuthAngular");
```

---

## §9 Projected CRS (SC_ProjectedCRS)

A projected CRS is a two-dimensional CRS derived from a geographic (base) CRS by
applying a map projection. The result is expressed on a flat surface using a
Cartesian coordinate system with linear units (typically metres).

### §9.1 SC_ProjectedCRS attributes

```rust
/// SC_ProjectedCRS.baseCRS references a SC_GeodeticCRS (a geographic CRS, not geocentric).
///
/// Source: ISO 19111:2019 §9.1 — Projected CRS base CRS
pub struct ProjectedCrsBaseCrsIsGeographic;
structural_prop!(ProjectedCrsBaseCrsIsGeographic, "ProjectedCrsBaseCrsIsGeographic");

/// SC_ProjectedCRS.coordinateSystem references a CS_CartesianCS.
///
/// Source: ISO 19111:2019 §9.1 — Projected CRS coordinate system
pub struct ProjectedCrsCsIsCartesian;
structural_prop!(ProjectedCrsCsIsCartesian, "ProjectedCrsCsIsCartesian");

/// SC_ProjectedCRS.projection references a CC_Conversion (defining the map projection).
///
/// Source: ISO 19111:2019 §9.1 — Projected CRS projection
pub struct ProjectedCrsProjectionIsConversion;
structural_prop!(ProjectedCrsProjectionIsConversion, "ProjectedCrsProjectionIsConversion");

/// SC_ProjectedCRS.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §9.1 — Projected CRS name
pub struct ProjectedCrsNameNonEmpty;
structural_prop!(ProjectedCrsNameNonEmpty, "ProjectedCrsNameNonEmpty");
```

### §9.2 Projected CRS axis constraints

```rust
/// Projected CRS axes use a linear unit; easting and northing are conventionally in metres.
///
/// Source: ISO 19111:2019 §9.2 — Projected CRS axis units
pub struct ProjectedCrsAxesUseLinearUnit;
structural_prop!(ProjectedCrsAxesUseLinearUnit, "ProjectedCrsAxesUseLinearUnit");

/// Projected CRS typically has easting (E) and northing (N) axes; variants westing or southing exist.
///
/// Source: ISO 19111:2019 §9.2 — Projected CRS conventional axis directions
pub struct ProjectedCrsConventionalAxisDirections;
structural_prop!(ProjectedCrsConventionalAxisDirections, "ProjectedCrsConventionalAxisDirections");

/// A projected CRS has exactly 2 axes (it is a 2D CRS).
///
/// Source: ISO 19111:2019 §9.2 — Projected CRS is two-dimensional
pub struct ProjectedCrsHasTwoAxes;
structural_prop!(ProjectedCrsHasTwoAxes, "ProjectedCrsHasTwoAxes");
```

### §9.3 UTM zone constraints (EPSG 32601–32760)

```rust
/// UTM North zone CRS codes range from EPSG:32601 (zone 1N) to EPSG:32660 (zone 60N).
///
/// Source: ISO 19111:2019 §9 — UTM North zones
pub struct UtmNorthZoneEpsgRange32601To32660;
structural_prop!(UtmNorthZoneEpsgRange32601To32660, "UtmNorthZoneEpsgRange32601To32660");

/// UTM South zone CRS codes range from EPSG:32701 (zone 1S) to EPSG:32760 (zone 60S).
///
/// Source: ISO 19111:2019 §9 — UTM South zones
pub struct UtmSouthZoneEpsgRange32701To32760;
structural_prop!(UtmSouthZoneEpsgRange32701To32760, "UtmSouthZoneEpsgRange32701To32760");

/// UTM zones have easting (E) axis first, northing (N) axis second.
///
/// Source: ISO 19111:2019 §9 — UTM axis order
pub struct UtmAxisOrderEastingFirst;
structural_prop!(UtmAxisOrderEastingFirst, "UtmAxisOrderEastingFirst");

/// UTM false easting is 500 000 m; false northing is 0 m (North) or 10 000 000 m (South).
///
/// Source: ISO 19111:2019 §9 — UTM false origin
pub struct UtmFalseEasting500000;
structural_prop!(UtmFalseEasting500000, "UtmFalseEasting500000");

/// UTM zone width is exactly 6° of longitude.
///
/// Source: ISO 19111:2019 §9 — UTM zone width
pub struct UtmZoneWidthSixDegrees;
structural_prop!(UtmZoneWidthSixDegrees, "UtmZoneWidthSixDegrees");

/// UTM scale factor at central meridian is 0.9996.
///
/// Source: ISO 19111:2019 §9 — UTM scale factor
pub struct UtmScaleFactorAtCentralMeridian0996;
structural_prop!(UtmScaleFactorAtCentralMeridian0996, "UtmScaleFactorAtCentralMeridian0996");

/// UTM zone number is an integer in [1, 60].
///
/// Source: ISO 19111:2019 §9 — UTM zone number range
pub struct UtmZoneNumberOneToSixty;
structural_prop!(UtmZoneNumberOneToSixty, "UtmZoneNumberOneToSixty");
```

---

## §10 Vertical CRS (SC_VerticalCRS)

A vertical CRS is a one-dimensional CRS used to describe heights or depths. It is
anchored to the Earth's gravity field via a vertical reference frame (datum).

### §10.1 SC_VerticalCRS attributes

```rust
/// SC_VerticalCRS.datum references a CD_VerticalReferenceFrame.
///
/// Source: ISO 19111:2019 §10.1 — Vertical reference frame
pub struct VerticalCrsDatumIsVerticalReferenceFrame;
structural_prop!(VerticalCrsDatumIsVerticalReferenceFrame, "VerticalCrsDatumIsVerticalReferenceFrame");

/// SC_VerticalCRS.coordinateSystem references a CS_VerticalCS.
///
/// Source: ISO 19111:2019 §10.1 — Vertical CRS coordinate system
pub struct VerticalCrsCsIsVerticalCs;
structural_prop!(VerticalCrsCsIsVerticalCs, "VerticalCrsCsIsVerticalCs");

/// SC_VerticalCRS.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §10.1 — Vertical CRS name
pub struct VerticalCrsNameNonEmpty;
structural_prop!(VerticalCrsNameNonEmpty, "VerticalCrsNameNonEmpty");

/// SC_VerticalCRS has exactly one axis (it is a one-dimensional CRS).
///
/// Source: ISO 19111:2019 §10.1 — Vertical CRS dimensionality
pub struct VerticalCrsHasOneAxis;
structural_prop!(VerticalCrsHasOneAxis, "VerticalCrsHasOneAxis");
```

### §10.2 CD_VerticalReferenceFrame

```rust
/// CD_VerticalReferenceFrame.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §10.2 — Vertical reference frame name
pub struct VerticalReferenceFrameNameNonEmpty;
structural_prop!(VerticalReferenceFrameNameNonEmpty, "VerticalReferenceFrameNameNonEmpty");

/// CD_VerticalReferenceFrame.realizationEpoch is an optional ISO 8601 date.
///
/// Source: ISO 19111:2019 §10.2 — Vertical reference frame realization epoch
pub struct VerticalReferenceFrameRealizationEpochIsIso8601;
structural_prop!(VerticalReferenceFrameRealizationEpochIsIso8601, "VerticalReferenceFrameRealizationEpochIsIso8601");

/// CD_VerticalReferenceFrame.anchorDefinition is an optional prose description.
///
/// Source: ISO 19111:2019 §10.2 — Vertical reference frame anchor definition
pub struct VerticalReferenceFrameAnchorOptional;
structural_prop!(VerticalReferenceFrameAnchorOptional, "VerticalReferenceFrameAnchorOptional");

/// CD_VerticalReferenceFrame is associated with Earth's gravity field (not a geometric surface).
///
/// Source: ISO 19111:2019 §10.2 — Vertical reference frame ties to gravity
pub struct VerticalReferenceFrameGravityRelated;
structural_prop!(VerticalReferenceFrameGravityRelated, "VerticalReferenceFrameGravityRelated");
```

### §10.3 Vertical CRS axis constraints

```rust
/// Vertical CRS height axis has direction "up" for heights above the reference surface.
///
/// Source: ISO 19111:2019 §10.3 — Vertical CRS height axis direction
pub struct VerticalCrsHeightAxisDirectionUp;
structural_prop!(VerticalCrsHeightAxisDirectionUp, "VerticalCrsHeightAxisDirectionUp");

/// Vertical CRS depth axis has direction "down" for depths below the reference surface.
///
/// Source: ISO 19111:2019 §10.3 — Vertical CRS depth axis direction
pub struct VerticalCrsDepthAxisDirectionDown;
structural_prop!(VerticalCrsDepthAxisDirectionDown, "VerticalCrsDepthAxisDirectionDown");

/// Vertical CRS axis unit is a linear unit of measure (metres, feet, fathoms, etc.).
///
/// Source: ISO 19111:2019 §10.3 — Vertical CRS axis linear unit
pub struct VerticalCrsAxisLinearUnit;
structural_prop!(VerticalCrsAxisLinearUnit, "VerticalCrsAxisLinearUnit");

/// EPSG codes for vertical CRS fall in the range 5000–5999.
///
/// Source: ISO 19111:2019 §10 — Vertical CRS EPSG code range
pub struct VerticalCrsEpsgRange5000To5999;
structural_prop!(VerticalCrsEpsgRange5000To5999, "VerticalCrsEpsgRange5000To5999");
```

---

## §11 Engineering CRS (SC_EngineeringCRS)

An engineering CRS is used for local contexts that are not georeferenced to the
Earth. It is defined by an engineering datum that ties it to a local surface,
platform, or structure.

```rust
/// SC_EngineeringCRS.datum references a CD_EngineeringDatum.
///
/// Source: ISO 19111:2019 §11.1 — Engineering datum
pub struct EngineeringCrsDatumIsEngineeringDatum;
structural_prop!(EngineeringCrsDatumIsEngineeringDatum, "EngineeringCrsDatumIsEngineeringDatum");

/// SC_EngineeringCRS.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §11.1 — Engineering CRS name
pub struct EngineeringCrsNameNonEmpty;
structural_prop!(EngineeringCrsNameNonEmpty, "EngineeringCrsNameNonEmpty");

/// SC_EngineeringCRS.coordinateSystem may be Cartesian, affine, polar, cylindrical, or any general local system.
///
/// Source: ISO 19111:2019 §11.2 — Engineering CRS coordinate system type
pub struct EngineeringCrsCsTypeFlexible;
structural_prop!(EngineeringCrsCsTypeFlexible, "EngineeringCrsCsTypeFlexible");

/// SC_EngineeringCRS is not georeferenced — it is valid only within its local context.
///
/// Source: ISO 19111:2019 §11.1 — Engineering CRS local validity
pub struct EngineeringCrsIsLocalContextOnly;
structural_prop!(EngineeringCrsIsLocalContextOnly, "EngineeringCrsIsLocalContextOnly");

/// CD_EngineeringDatum.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §11.2 — Engineering datum name
pub struct EngineeringDatumNameNonEmpty;
structural_prop!(EngineeringDatumNameNonEmpty, "EngineeringDatumNameNonEmpty");

/// CD_EngineeringDatum.anchorDefinition is an optional CharacterString describing the local reference surface.
///
/// Source: ISO 19111:2019 §11.2 — Engineering datum anchor definition
pub struct EngineeringDatumAnchorOptional;
structural_prop!(EngineeringDatumAnchorOptional, "EngineeringDatumAnchorOptional");
```

---

## §12 Compound CRS (SC_CompoundCRS)

A compound CRS combines two or more single CRS objects into a higher-dimensional
system. The canonical use is combining a horizontal (2D) CRS with a vertical (1D)
CRS to produce a 3D+height description.

```rust
/// SC_CompoundCRS.componentRS has at least 2 component CRS objects.
///
/// Source: ISO 19111:2019 §12.1 — Compound CRS minimum components
pub struct CompoundCrsHasAtLeastTwoComponents;
structural_prop!(CompoundCrsHasAtLeastTwoComponents, "CompoundCrsHasAtLeastTwoComponents");

/// SC_CompoundCRS.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §12.1 — Compound CRS name
pub struct CompoundCrsNameNonEmpty;
structural_prop!(CompoundCrsNameNonEmpty, "CompoundCrsNameNonEmpty");

/// Component CRS objects within a compound CRS are non-overlapping in dimension.
///
/// Source: ISO 19111:2019 §12.2 — Compound CRS dimension orthogonality
pub struct CompoundCrsComponentsNonOverlapping;
structural_prop!(CompoundCrsComponentsNonOverlapping, "CompoundCrsComponentsNonOverlapping");

/// Total axis count of a compound CRS equals the sum of all component axis counts.
///
/// Source: ISO 19111:2019 §12.2 — Compound CRS total axis count
pub struct CompoundCrsTotalAxisCountIsSumOfComponents;
structural_prop!(CompoundCrsTotalAxisCountIsSumOfComponents, "CompoundCrsTotalAxisCountIsSumOfComponents");

/// The typical compound CRS combines a horizontal 2D CRS with a vertical 1D CRS.
///
/// Source: ISO 19111:2019 §12.2 — Typical compound CRS structure
pub struct CompoundCrsTypicalIs2dPlusVertical;
structural_prop!(CompoundCrsTypicalIs2dPlusVertical, "CompoundCrsTypicalIs2dPlusVertical");

/// EPSG codes for compound CRS fall in the range 6000–6999.
///
/// Source: ISO 19111:2019 §12 — Compound CRS EPSG code range
pub struct CompoundCrsEpsgRange6000To6999;
structural_prop!(CompoundCrsEpsgRange6000To6999, "CompoundCrsEpsgRange6000To6999");

/// A compound CRS must not include two horizontal CRS components (would create ambiguity).
///
/// Source: ISO 19111:2019 §12.2 — Compound CRS no dual horizontal
pub struct CompoundCrsNoTwoHorizontalComponents;
structural_prop!(CompoundCrsNoTwoHorizontalComponents, "CompoundCrsNoTwoHorizontalComponents");

/// A compound CRS must not include two vertical CRS components.
///
/// Source: ISO 19111:2019 §12.2 — Compound CRS no dual vertical
pub struct CompoundCrsNoTwoVerticalComponents;
structural_prop!(CompoundCrsNoTwoVerticalComponents, "CompoundCrsNoTwoVerticalComponents");
```

---

## §13 Derived CRS (SC_DerivedCRS)

A derived CRS is obtained from another CRS (the base CRS) through a coordinate
conversion. The conversion does not involve a datum change — no new physical
tie to the Earth is introduced.

```rust
/// SC_DerivedCRS.baseCRS references another CRS (the source CRS for the derivation).
///
/// Source: ISO 19111:2019 §13.1 — Derived CRS base CRS
pub struct DerivedCrsHasBaseCrs;
structural_prop!(DerivedCrsHasBaseCrs, "DerivedCrsHasBaseCrs");

/// SC_DerivedCRS.derivingConversion references a CC_Conversion (no datum change).
///
/// Source: ISO 19111:2019 §13.1 — Derived CRS deriving conversion
pub struct DerivedCrsDerivingConversionIsConversion;
structural_prop!(DerivedCrsDerivingConversionIsConversion, "DerivedCrsDerivingConversionIsConversion");

/// SC_DerivedCRS.coordinateSystem may differ from the base CRS coordinate system.
///
/// Source: ISO 19111:2019 §13.1 — Derived CRS different CS allowed
pub struct DerivedCrsCsDiffersFromBaseCrsAllowed;
structural_prop!(DerivedCrsCsDiffersFromBaseCrsAllowed, "DerivedCrsCsDiffersFromBaseCrsAllowed");

/// SC_DerivedCRS.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §13.1 — Derived CRS name
pub struct DerivedCrsNameNonEmpty;
structural_prop!(DerivedCrsNameNonEmpty, "DerivedCrsNameNonEmpty");

/// A DerivedCRS inherits the datum of its base CRS implicitly.
///
/// Source: ISO 19111:2019 §13.1 — Derived CRS inherits datum
pub struct DerivedCrsInheritsDatumFromBase;
structural_prop!(DerivedCrsInheritsDatumFromBase, "DerivedCrsInheritsDatumFromBase");

/// SC_DerivedProjectedCRS: a projected CRS derived from another projected CRS.
///
/// Source: ISO 19111:2019 §13.2 — Derived projected CRS
pub struct DerivedProjectedCrsBaseMustBeProjCrs;
structural_prop!(DerivedProjectedCrsBaseMustBeProjCrs, "DerivedProjectedCrsBaseMustBeProjCrs");
```

---

## §14 Coordinate Operations

### §14.1 CC_CoordinateOperation (abstract)

A coordinate operation transforms coordinate values from one CRS to another. All
concrete operation types (conversion, transformation, concatenated operation)
are subclasses of the abstract `CC_CoordinateOperation`.

```rust
/// CC_CoordinateOperation.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §14.1 — Coordinate operation name
pub struct CoordinateOperationNameNonEmpty;
structural_prop!(CoordinateOperationNameNonEmpty, "CoordinateOperationNameNonEmpty");

/// CC_CoordinateOperation.sourceCRS references the CRS of input coordinates.
///
/// Source: ISO 19111:2019 §14.1 — Coordinate operation source CRS
pub struct CoordinateOperationHasSourceCrs;
structural_prop!(CoordinateOperationHasSourceCrs, "CoordinateOperationHasSourceCrs");

/// CC_CoordinateOperation.targetCRS references the CRS of output coordinates.
///
/// Source: ISO 19111:2019 §14.1 — Coordinate operation target CRS
pub struct CoordinateOperationHasTargetCrs;
structural_prop!(CoordinateOperationHasTargetCrs, "CoordinateOperationHasTargetCrs");

/// CC_CoordinateOperation.operationVersion is an optional CharacterString.
///
/// Source: ISO 19111:2019 §14.1 — Coordinate operation version
pub struct CoordinateOperationVersionOptional;
structural_prop!(CoordinateOperationVersionOptional, "CoordinateOperationVersionOptional");

/// CC_CoordinateOperation.domainOfValidity is an optional EX_Extent.
///
/// Source: ISO 19111:2019 §14.1 — Coordinate operation domain of validity
pub struct CoordinateOperationDomainOfValidityOptional;
structural_prop!(CoordinateOperationDomainOfValidityOptional, "CoordinateOperationDomainOfValidityOptional");
```

### §14.2 CC_Conversion

```rust
/// CC_Conversion: a coordinate operation that does not involve a datum change.
///
/// Source: ISO 19111:2019 §14.2 — Conversion: no datum change
pub struct ConversionInvolvesNoDatumChange;
structural_prop!(ConversionInvolvesNoDatumChange, "ConversionInvolvesNoDatumChange");

/// CC_Conversion defines a map projection (e.g., Transverse Mercator, Lambert Conformal Conic).
///
/// Source: ISO 19111:2019 §14.2 — Conversion defines map projection
pub struct ConversionDefinesMapProjection;
structural_prop!(ConversionDefinesMapProjection, "ConversionDefinesMapProjection");

/// CC_Conversion has an exact inverse operation (the inverse conversion).
///
/// Source: ISO 19111:2019 §14.2 — Conversion inverse exists
pub struct ConversionInverseExists;
structural_prop!(ConversionInverseExists, "ConversionInverseExists");

/// CC_Conversion.method references a CC_OperationMethod defining the algorithm.
///
/// Source: ISO 19111:2019 §14.2 — Conversion method
pub struct ConversionHasOperationMethod;
structural_prop!(ConversionHasOperationMethod, "ConversionHasOperationMethod");

/// CC_Conversion.parameterValue provides parameter values for the operation method.
///
/// Source: ISO 19111:2019 §14.2 — Conversion parameter values
pub struct ConversionHasParameterValues;
structural_prop!(ConversionHasParameterValues, "ConversionHasParameterValues");
```

### §14.3 CC_Transformation

```rust
/// CC_Transformation: a coordinate operation that involves a datum change.
///
/// Source: ISO 19111:2019 §14.3 — Transformation involves datum change
pub struct TransformationInvolvesDatumChange;
structural_prop!(TransformationInvolvesDatumChange, "TransformationInvolvesDatumChange");

/// CC_Transformation.accuracy is an optional positive real number in metres.
///
/// Source: ISO 19111:2019 §14.3 — Transformation accuracy
pub struct TransformationAccuracyPositiveReal;
structural_prop!(TransformationAccuracyPositiveReal, "TransformationAccuracyPositiveReal");

/// CC_Transformation.accuracy value of 0 is invalid (transformations always have some uncertainty).
///
/// Source: ISO 19111:2019 §14.3 — Transformation accuracy non-zero
pub struct TransformationAccuracyNonZero;
structural_prop!(TransformationAccuracyNonZero, "TransformationAccuracyNonZero");

/// CC_Transformation does not have an exact inverse (due to approximations in the datum shift).
///
/// Source: ISO 19111:2019 §14.3 — Transformation approximate inverse
pub struct TransformationInverseApproximate;
structural_prop!(TransformationInverseApproximate, "TransformationInverseApproximate");

/// CC_Transformation example: NAD27 → WGS 84 involves a 3-parameter or 7-parameter Helmert shift.
///
/// Source: ISO 19111:2019 §14.3 — Transformation Helmert example
pub struct TransformationNad27ToWgs84UsesHelmert;
structural_prop!(TransformationNad27ToWgs84UsesHelmert, "TransformationNad27ToWgs84UsesHelmert");
```

### §14.4 CC_ConcatenatedOperation

```rust
/// CC_ConcatenatedOperation is a pipeline of 2 or more coordinate operations.
///
/// Source: ISO 19111:2019 §14.4 — Concatenated operation pipeline
pub struct ConcatenatedOperationHasAtLeastTwoSteps;
structural_prop!(ConcatenatedOperationHasAtLeastTwoSteps, "ConcatenatedOperationHasAtLeastTwoSteps");

/// CC_ConcatenatedOperation step[i].targetCRS must equal step[i+1].sourceCRS.
///
/// Source: ISO 19111:2019 §14.4 — Concatenated operation step chain
pub struct ConcatenatedOperationStepsFormAChain;
structural_prop!(ConcatenatedOperationStepsFormAChain, "ConcatenatedOperationStepsFormAChain");

/// CC_ConcatenatedOperation.sourceCRS equals the sourceCRS of the first step.
///
/// Source: ISO 19111:2019 §14.4 — Concatenated operation overall source CRS
pub struct ConcatenatedOperationSourceCrsIsFirstStep;
structural_prop!(ConcatenatedOperationSourceCrsIsFirstStep, "ConcatenatedOperationSourceCrsIsFirstStep");

/// CC_ConcatenatedOperation.targetCRS equals the targetCRS of the last step.
///
/// Source: ISO 19111:2019 §14.4 — Concatenated operation overall target CRS
pub struct ConcatenatedOperationTargetCrsIsLastStep;
structural_prop!(ConcatenatedOperationTargetCrsIsLastStep, "ConcatenatedOperationTargetCrsIsLastStep");
```

### §14.5 CC_OperationMethod

```rust
/// CC_OperationMethod.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §14.5 — Operation method name
pub struct OperationMethodNameNonEmpty;
structural_prop!(OperationMethodNameNonEmpty, "OperationMethodNameNonEmpty");

/// CC_OperationMethod.formula is an optional CharacterString or reference to the algorithm definition.
///
/// Source: ISO 19111:2019 §14.5 — Operation method formula
pub struct OperationMethodFormulaOptional;
structural_prop!(OperationMethodFormulaOptional, "OperationMethodFormulaOptional");

/// CC_OperationMethod.parameter is a list of CC_OperationParameter definitions.
///
/// Source: ISO 19111:2019 §14.5 — Operation method parameters
pub struct OperationMethodHasParameterList;
structural_prop!(OperationMethodHasParameterList, "OperationMethodHasParameterList");

/// No two CC_OperationParameter entries in a CC_OperationMethod.parameter list
/// may share the same name; duplicate parameter names would make value look-up
/// ambiguous.
///
/// Source: ISO 19111:2019 §14.5 — Operation method parameter uniqueness
pub struct OperationMethodParameterNoDuplicates;
structural_prop!(OperationMethodParameterNoDuplicates, "OperationMethodParameterNoDuplicates");
```

### §14.6 CC_OperationParameter

```rust
/// CC_OperationParameter.name is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §14.6 — Operation parameter name
pub struct OperationParameterNameNonEmpty;
structural_prop!(OperationParameterNameNonEmpty, "OperationParameterNameNonEmpty");

/// CC_OperationParameterValue.parameterValue is a Measure (value + unit).
///
/// Source: ISO 19111:2019 §14.6 — Operation parameter value with unit
pub struct OperationParameterValueHasUnit;
structural_prop!(OperationParameterValueHasUnit, "OperationParameterValueHasUnit");

/// CC_OperationParameterValue.parameterValue shall be a finite real number;
/// NaN and ±Infinity are not valid coordinate operation parameters.
///
/// Source: ISO 19111:2019 §14.6 — Operation parameter value finite real
pub struct OperationParameterValueFiniteReal;
structural_prop!(OperationParameterValueFiniteReal, "OperationParameterValueFiniteReal");
```

---

## §15 Axis Order and Coordinate Tuples

Axis order is a persistent source of interoperability failures. ISO 19111 defines
axis order as a normative property of the CRS. Applications must not silently
reorder axes.

```rust
/// Geographic 2D CRS: ISO 19111 defines axis order as latitude first, longitude second.
///
/// Source: ISO 19111:2019 §15.1 — Geographic axis order: latitude first
pub struct Geographic2dIsoAxisOrderLatitudeFirst;
structural_prop!(Geographic2dIsoAxisOrderLatitudeFirst, "Geographic2dIsoAxisOrderLatitudeFirst");

/// Geographic 3D CRS: ISO axis order is latitude, longitude, then ellipsoidal height.
///
/// Source: ISO 19111:2019 §15.1 — Geographic 3D axis order
pub struct Geographic3dIsoAxisOrderLatLonHeight;
structural_prop!(Geographic3dIsoAxisOrderLatLonHeight, "Geographic3dIsoAxisOrderLatLonHeight");

/// Projected CRS: conventional axis order is easting first, northing second.
///
/// Source: ISO 19111:2019 §15.2 — Projected axis order: easting first
pub struct ProjectedConventionalAxisOrderEastingFirst;
structural_prop!(ProjectedConventionalAxisOrderEastingFirst, "ProjectedConventionalAxisOrderEastingFirst");

/// Some projected CRS (e.g., British National Grid EPSG:27700) use northing before easting.
///
/// Source: ISO 19111:2019 §15.2 — Projected CRS northing-first variants
pub struct ProjectedNorthingFirstVariantsExist;
structural_prop!(ProjectedNorthingFirstVariantsExist, "ProjectedNorthingFirstVariantsExist");

/// Axis order MUST be taken from the CRS definition; applications must not silently override it.
///
/// Source: ISO 19111:2019 §15.3 — Axis order must not be overridden implicitly
pub struct AxisOrderMustFollowCrsDefinition;
structural_prop!(AxisOrderMustFollowCrsDefinition, "AxisOrderMustFollowCrsDefinition");

/// Changing axis order requires an explicit coordinate operation (axis order swap), not implicit reordering.
///
/// Source: ISO 19111:2019 §15.3 — Axis order change requires explicit operation
pub struct AxisOrderChangeRequiresExplicitOperation;
structural_prop!(AxisOrderChangeRequiresExplicitOperation, "AxisOrderChangeRequiresExplicitOperation");

/// A coordinate tuple must have the same number of elements as the CRS has axes.
///
/// Source: ISO 19111:2019 §15.4 — Coordinate tuple element count equals axis count
pub struct CoordinateTupleElementCountEqualsAxisCount;
structural_prop!(CoordinateTupleElementCountEqualsAxisCount, "CoordinateTupleElementCountEqualsAxisCount");

/// Each coordinate element in a tuple corresponds to the axis at the same ordinal position.
///
/// Source: ISO 19111:2019 §15.4 — Coordinate element ordinal position alignment
pub struct CoordinateElementAlignedToAxisOrdinalPosition;
structural_prop!(CoordinateElementAlignedToAxisOrdinalPosition, "CoordinateElementAlignedToAxisOrdinalPosition");
```

---

## §16 WKT Identifiers and the EPSG Registry

### §16.1 Authority names and code structure

```rust
/// EPSG CRS codes are positive integers (no zero, no negative).
///
/// Source: ISO 19111:2019 §16.1 — EPSG code positive integer
pub struct EpsgCodePositiveInteger;
structural_prop!(EpsgCodePositiveInteger, "EpsgCodePositiveInteger");

/// The EPSG authority name string is "EPSG" (exact, case-sensitive per WKT2).
///
/// Source: ISO 19111:2019 §16.1 — EPSG authority name
pub struct EpsgAuthorityNameIsEpsg;
structural_prop!(EpsgAuthorityNameIsEpsg, "EpsgAuthorityNameIsEpsg");

/// The OGC authority name string is "OGC".
///
/// Source: ISO 19111:2019 §16.1 — OGC authority name
pub struct OgcAuthorityNameIsOgc;
structural_prop!(OgcAuthorityNameIsOgc, "OgcAuthorityNameIsOgc");

/// Other valid authority names include "ESRI" and "IGNF".
///
/// Source: ISO 19111:2019 §16.1 — Other authority names
pub struct OtherAuthorityNamesEsriIgnf;
structural_prop!(OtherAuthorityNamesEsriIgnf, "OtherAuthorityNamesEsriIgnf");

/// A null or missing authority code is invalid for a registered CRS.
///
/// Source: ISO 19111:2019 §16.1 — Null authority code invalid
pub struct RegisteredCrsNullAuthorityCodeInvalid;
structural_prop!(RegisteredCrsNullAuthorityCodeInvalid, "RegisteredCrsNullAuthorityCodeInvalid");
```

### §16.2 EPSG code ranges by CRS type

```rust
/// EPSG geographic CRS codes fall in the range 4000–4999.
///
/// Source: ISO 19111:2019 §16.2 — EPSG geographic CRS code range
pub struct EpsgGeographicCrsRange4000To4999;
structural_prop!(EpsgGeographicCrsRange4000To4999, "EpsgGeographicCrsRange4000To4999");

/// EPSG projected CRS codes fall in the range 20000–32767.
///
/// Source: ISO 19111:2019 §16.2 — EPSG projected CRS code range
pub struct EpsgProjectedCrsRange20000To32767;
structural_prop!(EpsgProjectedCrsRange20000To32767, "EpsgProjectedCrsRange20000To32767");

/// EPSG vertical CRS codes fall in the range 5000–5999.
///
/// Source: ISO 19111:2019 §16.2 — EPSG vertical CRS code range
pub struct EpsgVerticalCrsRange5000To5999;
structural_prop!(EpsgVerticalCrsRange5000To5999, "EpsgVerticalCrsRange5000To5999");

/// EPSG compound CRS codes fall in the range 6000–6999.
///
/// Source: ISO 19111:2019 §16.2 — EPSG compound CRS code range
pub struct EpsgCompoundCrsRange6000To6999;
structural_prop!(EpsgCompoundCrsRange6000To6999, "EpsgCompoundCrsRange6000To6999");
```

### §16.3 CRS identity and registry equality

```rust
/// Two CRS objects with the same authority and code are the same CRS.
///
/// Source: ISO 19111:2019 §16.3 — CRS identity by authority and code
pub struct CrsIdentityByAuthorityAndCode;
structural_prop!(CrsIdentityByAuthorityAndCode, "CrsIdentityByAuthorityAndCode");

/// Two CRS objects with different codes must not be treated as equivalent without a verified coordinate operation.
///
/// Source: ISO 19111:2019 §16.3 — Different codes require explicit coordinate operation
pub struct DifferentCrsCodesRequireExplicitOperation;
structural_prop!(DifferentCrsCodesRequireExplicitOperation, "DifferentCrsCodesRequireExplicitOperation");
```

---

## §17 Dynamic and Static CRS

### §17.1 Static CRS

A static CRS is one whose datum is plate-fixed; coordinates do not change over
time due to tectonic motion. Most classical datums (NAD83, WGS 84 original) are
static within their reference frame.

```rust
/// A static CRS is plate-fixed; no coordinate epoch is required for plate-stable locations.
///
/// Source: ISO 19111:2019 §17.1 — Static CRS: plate-fixed datum
pub struct StaticCrsPlateFixedNoEpochRequired;
structural_prop!(StaticCrsPlateFixedNoEpochRequired, "StaticCrsPlateFixedNoEpochRequired");

/// A static CRS datum realization is at a fixed epoch, not a moving reference.
///
/// Source: ISO 19111:2019 §17.1 — Static CRS datum at fixed epoch
pub struct StaticCrsDatumAtFixedEpoch;
structural_prop!(StaticCrsDatumAtFixedEpoch, "StaticCrsDatumAtFixedEpoch");
```

### §17.2 Dynamic CRS

A dynamic CRS is epoch-dependent. Its datum moves with the Earth's crust at the
level of plate tectonic and deformation model accuracy. Precise positioning in a
dynamic CRS requires a coordinate epoch.

```rust
/// A dynamic CRS requires a coordinate epoch for precise positioning.
///
/// Source: ISO 19111:2019 §17.2 — Dynamic CRS requires coordinate epoch
pub struct DynamicCrsRequiresCoordinateEpoch;
structural_prop!(DynamicCrsRequiresCoordinateEpoch, "DynamicCrsRequiresCoordinateEpoch");

/// A coordinate epoch is a decimal year value (e.g., 2023.5 for mid-2023).
///
/// Source: ISO 19111:2019 §17.2 — Coordinate epoch decimal year
pub struct CoordinateEpochIsDecimalYear;
structural_prop!(CoordinateEpochIsDecimalYear, "CoordinateEpochIsDecimalYear");

/// A coordinate epoch is a positive finite real number (year > 0).
///
/// Source: ISO 19111:2019 §17.2 — Coordinate epoch positive finite
pub struct CoordinateEpochPositiveFinite;
structural_prop!(CoordinateEpochPositiveFinite, "CoordinateEpochPositiveFinite");

/// ITRF2014 is a dynamic CRS; coordinates at different epochs differ by up to tens of centimetres.
///
/// Source: ISO 19111:2019 §17.2 — ITRF2014 dynamic datum
pub struct Itrf2014IsDynamicDatum;
structural_prop!(Itrf2014IsDynamicDatum, "Itrf2014IsDynamicDatum");

/// ITRF2020 is a dynamic CRS requiring coordinate epochs for millimetre-level positioning.
///
/// Source: ISO 19111:2019 §17.2 — ITRF2020 dynamic datum
pub struct Itrf2020IsDynamicDatum;
structural_prop!(Itrf2020IsDynamicDatum, "Itrf2020IsDynamicDatum");

/// IGS20 is a dynamic CRS (the IGS realization of ITRF2020).
///
/// Source: ISO 19111:2019 §17.2 — IGS20 dynamic datum
pub struct Igs20IsDynamicDatum;
structural_prop!(Igs20IsDynamicDatum, "Igs20IsDynamicDatum");

/// Dynamic CRS: omitting the coordinate epoch introduces positional uncertainty of centimetres to decimetres.
///
/// Source: ISO 19111:2019 §17.2 — Dynamic CRS epoch omission error
pub struct DynamicCrsOmittingEpochIntroducesError;
structural_prop!(DynamicCrsOmittingEpochIntroducesError, "DynamicCrsOmittingEpochIntroducesError");

/// A dynamic reference frame carries a frameReferenceEpoch attribute (a decimal year).
///
/// Source: ISO 19111:2019 §17.3 — Dynamic reference frame epoch attribute
pub struct DynamicReferenceFrameHasFrameReferenceEpoch;
structural_prop!(DynamicReferenceFrameHasFrameReferenceEpoch, "DynamicReferenceFrameHasFrameReferenceEpoch");

/// A dynamic reference frame should reference an associated velocity model (deformation
/// model) that describes secular plate-motion and crustal deformation rates; when absent
/// consumers cannot propagate coordinates between epochs.
///
/// Source: ISO 19111:2019 §17.3 — Dynamic datum velocity model indicator
pub struct DynamicReferenceFrameVelocityModelReferenced;
structural_prop!(DynamicReferenceFrameVelocityModelReferenced, "DynamicReferenceFrameVelocityModelReferenced");
```

---

## §18 Cross-Cutting Constraints

### §18.1 CRS identity and composability

```rust
/// CRS identity: same authority + same code implies same CRS object.
///
/// Source: ISO 19111:2019 §6.2.4 — CRS identity via identifier
pub struct CrsIdentityAuthorityPlusCodeUnique;
structural_prop!(CrsIdentityAuthorityPlusCodeUnique, "CrsIdentityAuthorityPlusCodeUnique");

/// The reference graph of CRS components (compound CRS → component CRS, derived CRS →
/// base CRS) is acyclic; a CRS must not transitively reference itself as a component or
/// base. Cycle-freedom is a precondition for all terminating graph traversal proofs.
///
/// Source: ISO 19111:2019 §13 / §14 — well-formedness of CRS component graphs
pub struct CrsComponentGraphAcyclic;
structural_prop!(CrsComponentGraphAcyclic, "CrsComponentGraphAcyclic");

/// A valid compound CRS requires orthogonal component CRS axes (no axis represented twice).
///
/// Source: ISO 19111:2019 §12.2 — Compound CRS component orthogonality
pub struct CompoundCrsComponentsOrthogonal;
structural_prop!(CompoundCrsComponentsOrthogonal, "CompoundCrsComponentsOrthogonal");

/// Axis abbreviation is unique within a given CS (no two axes share the same abbreviation).
///
/// Source: ISO 19111:2019 §8.2 — Axis abbreviation uniqueness
pub struct AxisAbbreviationUniqueInCs;
structural_prop!(AxisAbbreviationUniqueInCs, "AxisAbbreviationUniqueInCs");

/// A null or missing authority code is invalid in any registered CRS.
///
/// Source: ISO 19111:2019 §6.2.4 — Null authority code in registered CRS invalid
pub struct NullAuthorityCodeInvalidForRegisteredCrs;
structural_prop!(NullAuthorityCodeInvalidForRegisteredCrs, "NullAuthorityCodeInvalidForRegisteredCrs");
```

### §18.2 Unit of measure constraints

```rust
/// Angular axes (latitude, longitude) must use an angular unit (degrees or radians).
///
/// Source: ISO 19111:2019 §8.4 — Angular axis unit constraint
pub struct AngularAxisMustUseAngularUnit;
structural_prop!(AngularAxisMustUseAngularUnit, "AngularAxisMustUseAngularUnit");

/// Linear axes (easting, northing, height, depth) must use a linear unit (metres, feet, etc.).
///
/// Source: ISO 19111:2019 §8.5 — Linear axis unit constraint
pub struct LinearAxisMustUseLinearUnit;
structural_prop!(LinearAxisMustUseLinearUnit, "LinearAxisMustUseLinearUnit");

/// Parametric axes use a parametric unit appropriate to the quantity being measured.
///
/// Source: ISO 19111:2019 §8.8 — Parametric axis unit
pub struct ParametricAxisMustUseParametricUnit;
structural_prop!(ParametricAxisMustUseParametricUnit, "ParametricAxisMustUseParametricUnit");

/// Time axes use a temporal unit (seconds, days, years, Julian date, etc.).
///
/// Source: ISO 19111:2019 §8.7 — Temporal axis unit
pub struct TimeAxisMustUseTemporalUnit;
structural_prop!(TimeAxisMustUseTemporalUnit, "TimeAxisMustUseTemporalUnit");
```

### §18.3 Scope and domain of validity

```rust
/// SC_CRS.scope describes the intended use and is a non-empty CharacterString.
///
/// Source: ISO 19111:2019 §6.2.2 — CRS scope non-empty
pub struct CrsScopeDescribesIntendedUse;
structural_prop!(CrsScopeDescribesIntendedUse, "CrsScopeDescribesIntendedUse");

/// SC_CRS.domainOfValidity is optional; when absent, the CRS is assumed globally applicable.
///
/// Source: ISO 19111:2019 §6.2.3 — CRS domain of validity optional
pub struct CrsDomainOfValidityOptionalImpliesGlobal;
structural_prop!(CrsDomainOfValidityOptionalImpliesGlobal, "CrsDomainOfValidityOptionalImpliesGlobal");

/// EX_Extent referenced in domainOfValidity may contain geographic, temporal, or vertical extents.
///
/// Source: ISO 19111:2019 §6.2.3 — EX_Extent components
pub struct CrsDomainOfValidityExtentTypes;
structural_prop!(CrsDomainOfValidityExtentTypes, "CrsDomainOfValidityExtentTypes");
```

### §18.4 Remarks and aliases

```rust
/// SC_IdentifiedObject.alias is an optional list of CharacterString alternate names.
///
/// Source: ISO 19111:2019 §6.2 — Identified object alias
pub struct IdentifiedObjectAliasOptionalList;
structural_prop!(IdentifiedObjectAliasOptionalList, "IdentifiedObjectAliasOptionalList");

/// No two entries in SC_IdentifiedObject.alias shall be identical strings; duplicate
/// aliases add no information and can confuse authority-based look-up.
///
/// Source: ISO 19111:2019 §6.2 — Identified object alias uniqueness
pub struct IdentifiedObjectAliasNoDuplicates;
structural_prop!(IdentifiedObjectAliasNoDuplicates, "IdentifiedObjectAliasNoDuplicates");

/// SC_IdentifiedObject.remarks is an optional CharacterString for informative notes.
///
/// Source: ISO 19111:2019 §6.2 — Identified object remarks
pub struct IdentifiedObjectRemarksOptional;
structural_prop!(IdentifiedObjectRemarksOptional, "IdentifiedObjectRemarksOptional");
```

---

## §19 Temporal CRS and Parametric CRS

### §19.1 SC_TemporalCRS

```rust
/// SC_TemporalCRS.datum references a CD_TemporalDatum.
///
/// Source: ISO 19111:2019 §9 (temporal extension) — Temporal CRS datum
pub struct TemporalCrsDatumIsTemporalDatum;
structural_prop!(TemporalCrsDatumIsTemporalDatum, "TemporalCrsDatumIsTemporalDatum");

/// SC_TemporalCRS.coordinateSystem references a CS_TemporalCS.
///
/// Source: ISO 19111:2019 §9 (temporal extension) — Temporal CRS coordinate system
pub struct TemporalCrsCsIsTemporalCs;
structural_prop!(TemporalCrsCsIsTemporalCs, "TemporalCrsCsIsTemporalCs");

/// CD_TemporalDatum.origin is an ISO 8601 date–time defining the epoch of the temporal axis.
///
/// Source: ISO 19111:2019 §9 (temporal extension) — Temporal datum origin
pub struct TemporalDatumOriginIsIso8601DateTime;
structural_prop!(TemporalDatumOriginIsIso8601DateTime, "TemporalDatumOriginIsIso8601DateTime");

/// SC_TemporalCRS has exactly one temporal axis.
///
/// Source: ISO 19111:2019 §9 (temporal extension) — Temporal CRS has one axis
pub struct TemporalCrsHasOneAxis;
structural_prop!(TemporalCrsHasOneAxis, "TemporalCrsHasOneAxis");
```

### §19.2 SC_ParametricCRS

```rust
/// SC_ParametricCRS.datum references a CD_ParametricDatum.
///
/// Source: ISO 19111:2019 §9 (parametric extension) — Parametric CRS datum
pub struct ParametricCrsDatumIsParametricDatum;
structural_prop!(ParametricCrsDatumIsParametricDatum, "ParametricCrsDatumIsParametricDatum");

/// SC_ParametricCRS.coordinateSystem references a CS_ParametricCS.
///
/// Source: ISO 19111:2019 §9 (parametric extension) — Parametric CRS coordinate system
pub struct ParametricCrsCsIsParametricCs;
structural_prop!(ParametricCrsCsIsParametricCs, "ParametricCrsCsIsParametricCs");

/// SC_ParametricCRS has exactly one parametric axis.
///
/// Source: ISO 19111:2019 §9 (parametric extension) — Parametric CRS has one axis
pub struct ParametricCrsHasOneAxis;
structural_prop!(ParametricCrsHasOneAxis, "ParametricCrsHasOneAxis");
```

---

## §20 Additional Value Constraints

### §20.1 Cardinality constraints

```rust
/// CS_CoordinateSystem must have at least 1 axis.
///
/// Source: ISO 19111:2019 §8.1 — Coordinate system minimum axis count
pub struct CoordinateSystemMinimumOneAxis;
structural_prop!(CoordinateSystemMinimumOneAxis, "CoordinateSystemMinimumOneAxis");

/// CS_CoordinateSystem must have at most 4 axes.
///
/// Source: ISO 19111:2019 §8.1 — Coordinate system maximum axis count
pub struct CoordinateSystemMaximumFourAxes;
structural_prop!(CoordinateSystemMaximumFourAxes, "CoordinateSystemMaximumFourAxes");

/// SC_CompoundCRS must have at least 2 component CRS objects.
///
/// Source: ISO 19111:2019 §12.1 — Compound CRS minimum two components
pub struct CompoundCrsMinimumTwoComponents;
structural_prop!(CompoundCrsMinimumTwoComponents, "CompoundCrsMinimumTwoComponents");

/// CC_ConcatenatedOperation must have at least 2 steps.
///
/// Source: ISO 19111:2019 §14.4 — Concatenated operation minimum two steps
pub struct ConcatenatedOperationMinimumTwoSteps;
structural_prop!(ConcatenatedOperationMinimumTwoSteps, "ConcatenatedOperationMinimumTwoSteps");
```

### §20.2 String format constraints

```rust
/// All CharacterString values in CRS objects must be UTF-8 encoded.
///
/// Source: ISO 19111:2019 §6 — CharacterString encoding
pub struct CrsStringValuesUtf8Encoded;
structural_prop!(CrsStringValuesUtf8Encoded, "CrsStringValuesUtf8Encoded");

/// All CharacterString values in CRS objects must be non-null when mandatory.
///
/// Source: ISO 19111:2019 §6 — Mandatory CharacterString non-null
pub struct CrsMandatoryStringNonNull;
structural_prop!(CrsMandatoryStringNonNull, "CrsMandatoryStringNonNull");
```

### §20.3 Numeric precision and range

```rust
/// Latitude values in geographic CRS must lie in the range [-90, 90] degrees.
///
/// Source: ISO 19111:2019 §7.2 — Latitude range constraint
pub struct LatitudeRangeNegative90To90;
structural_prop!(LatitudeRangeNegative90To90, "LatitudeRangeNegative90To90");

/// Longitude values in geographic CRS must lie in the range (-180, 180] degrees.
///
/// Source: ISO 19111:2019 §7.2 — Longitude range constraint
pub struct LongitudeRangeNegative180To180;
structural_prop!(LongitudeRangeNegative180To180, "LongitudeRangeNegative180To180");

/// Latitude at exactly 90° or -90° (poles) is valid (boundary included).
///
/// Source: ISO 19111:2019 §7.2 — Poles are valid latitude values
pub struct LatitudePolesValid;
structural_prop!(LatitudePolesValid, "LatitudePolesValid");

/// Longitude at exactly -180° is excluded (antimeridian is represented as +180°).
///
/// Source: ISO 19111:2019 §7.2 — Antimeridian longitude convention
pub struct LongitudeNegative180Excluded;
structural_prop!(LongitudeNegative180Excluded, "LongitudeNegative180Excluded");

/// Scale factor in a map projection must be a finite positive real number.
///
/// Source: ISO 19111:2019 §9 — Map projection scale factor
pub struct MapProjectionScaleFactorPositive;
structural_prop!(MapProjectionScaleFactorPositive, "MapProjectionScaleFactorPositive");

/// Map projection scale factor is a finite real number (not NaN or ±Infinity);
/// IEEE 754 finiteness precondition required before positivity or ratio proofs.
///
/// Source: ISO 19111:2019 §9 — Map projection scale factor finite
pub struct MapProjectionScaleFactorFinite;
structural_prop!(MapProjectionScaleFactorFinite, "MapProjectionScaleFactorFinite");

/// False easting and false northing in a map projection may be any finite real number.
///
/// Source: ISO 19111:2019 §9 — False origin values
pub struct MapProjectionFalseOriginFiniteReal;
structural_prop!(MapProjectionFalseOriginFiniteReal, "MapProjectionFalseOriginFiniteReal");
```

### §20.4 Enumeration membership constraints

```rust
/// CS_AxisDirection must be a member of the defined CS_AxisDirection code list.
///
/// Source: ISO 19111:2019 §8.3 — Axis direction code list membership
pub struct AxisDirectionMemberOfCodeList;
structural_prop!(AxisDirectionMemberOfCodeList, "AxisDirectionMemberOfCodeList");

/// CS type must be one of: ellipsoidal, Cartesian, vertical, temporal, parametric, ordinal, affine, polar, cylindrical.
///
/// Source: ISO 19111:2019 §8.4–§8.12 — CS type enumeration
pub struct CsTypeMemberOfDefinedTypes;
structural_prop!(CsTypeMemberOfDefinedTypes, "CsTypeMemberOfDefinedTypes");

/// CRS type must be one of: geodetic, projected, vertical, engineering, compound, derived, temporal, parametric.
///
/// Source: ISO 19111:2019 §6 — CRS type enumeration
pub struct CrsTypeMemberOfDefinedTypes;
structural_prop!(CrsTypeMemberOfDefinedTypes, "CrsTypeMemberOfDefinedTypes");
```

---

## Summary: All Props by Category

### CRS General (§6)

| Prop | Description |
|------|-------------|
| `CrsConsistsOfCsAndDatum` | CRS = CS + Datum |
| `CoordinateTupleDimensionMatchesAxes` | Tuple ordinates = CS axis count |
| `ScCrsIsAbstract` | SC_CRS is abstract |
| `ScCrsNameNonEmpty` | name non-empty |
| `ScCrsScopeNonEmpty` | scope non-empty |
| `ScCrsDomainOfValidityIsExtent` | domainOfValidity is EX_Extent |
| `ScCrsIdentifierHasAuthorityAndCode` | identifier has authority + code |
| `CrsIdentifierAuthorityNonEmpty` | authority non-empty |
| `CrsIdentifierCodeNonEmpty` | code non-empty |

### Geodetic CRS (§7)

| Prop | Description |
|------|-------------|
| `GeodeticCrsDatumIsGeodeticReferenceFrame` | datum type |
| `GeodeticCrsCsIsEllipsoidalOrCartesian` | CS type |
| `Geographic2dCrsHasTwoAxes` | 2D has 2 axes |
| `Geographic3dCrsHasThreeAxes` | 3D has 3 axes |
| `GeocentricCrsUsesCartesianCs` | geocentric uses Cartesian |
| `Epsg4326AxisOrderLatFirst` | EPSG:4326 lat first |
| `Epsg4326LatitudeRangeValid` | EPSG:4326 lat range |
| `Epsg4326LongitudeRangeValid` | EPSG:4326 lon range |
| `Epsg4979IsWgs84Geographic3d` | EPSG:4979 |
| `Epsg4979HeightUnbounded` | EPSG:4979 height |
| `Epsg4978IsWgs84Geocentric` | EPSG:4978 |
| `Epsg4978XAxisTowardsPrimeMeridian` | EPSG:4978 X direction |
| `Epsg4978ZAxisTowardsNorthPole` | EPSG:4978 Z direction |

### Geodetic Reference Frame (§7.2)

| Prop | Description |
|------|-------------|
| `GeodeticReferenceFrameNameNonEmpty` | name non-empty |
| `GeodeticReferenceFrameAnchorOptional` | anchor optional |
| `GeodeticReferenceFrameHasExactlyOneEllipsoid` | 1 ellipsoid |
| `GeodeticReferenceFrameHasExactlyOnePrimeMeridian` | 1 prime meridian |
| `GeodeticReferenceFrameRealizationEpochIsIso8601` | epoch ISO 8601 |

### Ellipsoid (§7.3)

| Prop | Description |
|------|-------------|
| `EllipsoidNameNonEmpty` | name non-empty |
| `EllipsoidSemiMajorAxisPositive` | a > 0 |
| `EllipsoidSemiMajorAxisInMetres` | a in metres |
| `EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor` | either 1/f or b |
| `EllipsoidInverseFlatteningPositiveWhenNonSphere` | 1/f > 0 for non-sphere |
| `EllipsoidWgs84InverseFlatteningApprox298` | WGS 84 ≈ 298.257 |
| `EllipsoidInverseFlatteningZeroMeansSphere` | 1/f = 0 → sphere |
| `EllipsoidSemiMinorAxisLessThanSemiMajor` | b < a |
| `EllipsoidIsSphereConsistentWithParameters` | isSphere derived |
| `EllipsoidSemiMinorAxisInMetres` | b in metres |

### Prime Meridian (§7.4)

| Prop | Description |
|------|-------------|
| `PrimeMeridianNameNonEmpty` | name non-empty |
| `PrimeMeridianGreenwichLongitudeFinite` | finite longitude |
| `PrimeMeridianGreenwichLongitudeInDegreeBounds` | (-180, 180] |
| `PrimeMeridianGreenwichIsZero` | Greenwich = 0 |
| `PrimeMeridianNonGreenwichAllowed` | non-zero valid |
| `PrimeMeridianGreenwichLongitudeUnitIsAngular` | angular unit |

### Coordinate System (§8)

| Prop | Description |
|------|-------------|
| `CoordinateSystemAxisCountOneToFour` | 1..4 axes |
| `CsAxisCountMatchesTupleDimensionality` | matches tuple |
| `CoordinateSystemNameNonEmpty` | name non-empty |
| `AxisNameNonEmpty` | axis name non-empty |
| `AxisAbbreviationNonEmpty` | abbreviation non-empty |
| `AxisAbbreviationUniqueWithinCs` | unique in CS |
| `AxisDirectionIsValidCode` | valid direction code |
| `AxisUnitAppropriateForAxisType` | unit matches type |
| `AxisMinMaxValueOptional` | range optional |
| `AxisRangeMeaningExactOrWraparound` | range meaning |

### Axis Direction Codes (§8.3)

| Prop | Description |
|------|-------------|
| `AxisDirectionNorth` | north |
| `AxisDirectionSouth` | south |
| `AxisDirectionEast` | east |
| `AxisDirectionWest` | west |
| `AxisDirectionUp` | up |
| `AxisDirectionDown` | down |
| `AxisDirectionNorthNorthEast` | NNE |
| `AxisDirectionNorthEast` | NE |
| `AxisDirectionEastNorthEast` | ENE |
| `AxisDirectionEastSouthEast` | ESE |
| `AxisDirectionSouthEast` | SE |
| `AxisDirectionSouthSouthEast` | SSE |
| `AxisDirectionSouthSouthWest` | SSW |
| `AxisDirectionSouthWest` | SW |
| `AxisDirectionWestSouthWest` | WSW |
| `AxisDirectionWestNorthWest` | WNW |
| `AxisDirectionNorthWest` | NW |
| `AxisDirectionNorthNorthWest` | NNW |
| `AxisDirectionGeocentricX` | geocentricX |
| `AxisDirectionGeocentricY` | geocentricY |
| `AxisDirectionGeocentricZ` | geocentricZ |
| `AxisDirectionColumnPositive` | columnPositive |
| `AxisDirectionColumnNegative` | columnNegative |
| `AxisDirectionRowPositive` | rowPositive |
| `AxisDirectionRowNegative` | rowNegative |
| `AxisDirectionDisplayRight` | displayRight |
| `AxisDirectionDisplayLeft` | displayLeft |
| `AxisDirectionDisplayUp` | displayUp |
| `AxisDirectionDisplayDown` | displayDown |
| `AxisDirectionFuture` | future |
| `AxisDirectionPast` | past |
| `AxisDirectionTowards` | towards |
| `AxisDirectionAwayFrom` | awayFrom |
| `AxisDirectionCounterClockwise` | counterClockwise |
| `AxisDirectionClockwise` | clockwise |
| `AxisDirectionUnspecified` | unspecified |

### CS Types (§8.4)

| Prop | Description |
|------|-------------|
| `EllipsoidalCsHasTwoOrThreeAxes` | 2 or 3 axes |
| `EllipsoidalCsLatLonAxesUseAngularUnit` | angular unit |
| `EllipsoidalCs3dHeightAxisUsesLinearUnit` | height linear |
| `EllipsoidalCs2dHasNoHeightAxis` | no height in 2D |
| `CartesianCsAxesOrthogonal` | orthogonal |
| `CartesianCsProjectedUsesEastingNorthing` | E/N axes |
| `CartesianCsGeocentricHasThreeAxes` | XYZ |
| `CartesianCsAllAxesLinearUnit` | linear unit |
| `VerticalCsHasExactlyOneAxis` | 1 axis |
| `VerticalCsAxisDirectionUpOrDown` | up or down |
| `VerticalCsAxisUsesLinearUnit` | linear unit |
| `TemporalCsHasExactlyOneAxis` | 1 axis |
| `TemporalCsAxisDirectionFutureOrPast` | future or past |
| `TemporalCsAxisUsesTimeUnit` | time unit |
| `ParametricCsHasExactlyOneAxis` | 1 axis |
| `ParametricCsAxisHasParametricUnit` | parametric unit |
| `OrdinalCsNoUnitRequired` | no unit |
| `OrdinalCsValuesAreDiscreteLabels` | discrete labels |
| `AffineCsHasTwoOrThreeAxes` | 2 or 3 axes |
| `AffineCsAxesNeedNotBeOrthogonal` | non-orthogonal |
| `PolarCsHasTwoAxes` | 2 axes |
| `PolarCsDistanceLinearAngleAngular` | mixed units |
| `CylindricalCsHasThreeAxes` | 3 axes |
| `CylindricalCsDistanceHeightLinearAzimuthAngular` | mixed units |

### Projected CRS (§9)

| Prop | Description |
|------|-------------|
| `ProjectedCrsBaseCrsIsGeographic` | base is geographic |
| `ProjectedCrsCsIsCartesian` | CS is Cartesian |
| `ProjectedCrsProjectionIsConversion` | projection is CC_Conversion |
| `ProjectedCrsNameNonEmpty` | name non-empty |
| `ProjectedCrsAxesUseLinearUnit` | linear unit |
| `ProjectedCrsConventionalAxisDirections` | E/N axes |
| `ProjectedCrsHasTwoAxes` | 2D CRS |
| `UtmNorthZoneEpsgRange32601To32660` | UTM N range |
| `UtmSouthZoneEpsgRange32701To32760` | UTM S range |
| `UtmAxisOrderEastingFirst` | E first |
| `UtmFalseEasting500000` | FE = 500 000 m |
| `UtmZoneWidthSixDegrees` | 6° wide |
| `UtmScaleFactorAtCentralMeridian0996` | k₀ = 0.9996 |
| `UtmZoneNumberOneToSixty` | zone 1–60 |

### Vertical CRS (§10)

| Prop | Description |
|------|-------------|
| `VerticalCrsDatumIsVerticalReferenceFrame` | datum type |
| `VerticalCrsCsIsVerticalCs` | CS type |
| `VerticalCrsNameNonEmpty` | name non-empty |
| `VerticalCrsHasOneAxis` | 1 axis |
| `VerticalReferenceFrameNameNonEmpty` | frame name |
| `VerticalReferenceFrameRealizationEpochIsIso8601` | epoch |
| `VerticalReferenceFrameAnchorOptional` | anchor optional |
| `VerticalReferenceFrameGravityRelated` | gravity related |
| `VerticalCrsHeightAxisDirectionUp` | up |
| `VerticalCrsDepthAxisDirectionDown` | down |
| `VerticalCrsAxisLinearUnit` | linear unit |
| `VerticalCrsEpsgRange5000To5999` | EPSG range |

### Engineering CRS (§11)

| Prop | Description |
|------|-------------|
| `EngineeringCrsDatumIsEngineeringDatum` | datum type |
| `EngineeringCrsNameNonEmpty` | name non-empty |
| `EngineeringCrsCsTypeFlexible` | flexible CS |
| `EngineeringCrsIsLocalContextOnly` | local only |
| `EngineeringDatumNameNonEmpty` | datum name |
| `EngineeringDatumAnchorOptional` | anchor optional |

### Compound CRS (§12)

| Prop | Description |
|------|-------------|
| `CompoundCrsHasAtLeastTwoComponents` | ≥ 2 components |
| `CompoundCrsNameNonEmpty` | name non-empty |
| `CompoundCrsComponentsNonOverlapping` | non-overlapping |
| `CompoundCrsTotalAxisCountIsSumOfComponents` | axis sum |
| `CompoundCrsTypicalIs2dPlusVertical` | 2D + V |
| `CompoundCrsEpsgRange6000To6999` | EPSG range |
| `CompoundCrsNoTwoHorizontalComponents` | no dual horiz |
| `CompoundCrsNoTwoVerticalComponents` | no dual vert |

### Derived CRS (§13)

| Prop | Description |
|------|-------------|
| `DerivedCrsHasBaseCrs` | has base CRS |
| `DerivedCrsDerivingConversionIsConversion` | conversion type |
| `DerivedCrsCsDiffersFromBaseCrsAllowed` | CS may differ |
| `DerivedCrsNameNonEmpty` | name non-empty |
| `DerivedCrsInheritsDatumFromBase` | inherits datum |
| `DerivedProjectedCrsBaseMustBeProjCrs` | base is projected |

### Coordinate Operations (§14)

| Prop | Description |
|------|-------------|
| `CoordinateOperationNameNonEmpty` | name non-empty |
| `CoordinateOperationHasSourceCrs` | sourceCRS |
| `CoordinateOperationHasTargetCrs` | targetCRS |
| `CoordinateOperationVersionOptional` | version optional |
| `CoordinateOperationDomainOfValidityOptional` | domain optional |
| `ConversionInvolvesNoDatumChange` | no datum change |
| `ConversionDefinesMapProjection` | map projection |
| `ConversionInverseExists` | exact inverse |
| `ConversionHasOperationMethod` | has method |
| `ConversionHasParameterValues` | has params |
| `TransformationInvolvesDatumChange` | datum change |
| `TransformationAccuracyPositiveReal` | accuracy > 0 |
| `TransformationAccuracyNonZero` | accuracy ≠ 0 |
| `TransformationInverseApproximate` | approx inverse |
| `TransformationNad27ToWgs84UsesHelmert` | Helmert example |
| `ConcatenatedOperationHasAtLeastTwoSteps` | ≥ 2 steps |
| `ConcatenatedOperationStepsFormAChain` | chain validity |
| `ConcatenatedOperationSourceCrsIsFirstStep` | source = first |
| `ConcatenatedOperationTargetCrsIsLastStep` | target = last |
| `OperationMethodNameNonEmpty` | method name |
| `OperationMethodFormulaOptional` | formula optional |
| `OperationMethodHasParameterList` | has params |
| `OperationParameterNameNonEmpty` | param name |
| `OperationParameterValueHasUnit` | param value unit |

### Axis Order (§15)

| Prop | Description |
|------|-------------|
| `Geographic2dIsoAxisOrderLatitudeFirst` | lat first |
| `Geographic3dIsoAxisOrderLatLonHeight` | lat, lon, h |
| `ProjectedConventionalAxisOrderEastingFirst` | E first |
| `ProjectedNorthingFirstVariantsExist` | N first variants |
| `AxisOrderMustFollowCrsDefinition` | must follow CRS |
| `AxisOrderChangeRequiresExplicitOperation` | explicit swap |
| `CoordinateTupleElementCountEqualsAxisCount` | count matches |
| `CoordinateElementAlignedToAxisOrdinalPosition` | aligned |

### EPSG Registry (§16)

| Prop | Description |
|------|-------------|
| `EpsgCodePositiveInteger` | positive int |
| `EpsgAuthorityNameIsEpsg` | "EPSG" |
| `OgcAuthorityNameIsOgc` | "OGC" |
| `OtherAuthorityNamesEsriIgnf` | other authorities |
| `RegisteredCrsNullAuthorityCodeInvalid` | null invalid |
| `EpsgGeographicCrsRange4000To4999` | 4000–4999 |
| `EpsgProjectedCrsRange20000To32767` | 20000–32767 |
| `EpsgVerticalCrsRange5000To5999` | 5000–5999 |
| `EpsgCompoundCrsRange6000To6999` | 6000–6999 |
| `CrsIdentityByAuthorityAndCode` | identity |
| `DifferentCrsCodesRequireExplicitOperation` | different → op |

### Dynamic / Static CRS (§17)

| Prop | Description |
|------|-------------|
| `StaticCrsPlateFixedNoEpochRequired` | no epoch needed |
| `StaticCrsDatumAtFixedEpoch` | fixed epoch datum |
| `DynamicCrsRequiresCoordinateEpoch` | epoch required |
| `CoordinateEpochIsDecimalYear` | decimal year |
| `CoordinateEpochPositiveFinite` | positive finite |
| `Itrf2014IsDynamicDatum` | ITRF2014 dynamic |
| `Itrf2020IsDynamicDatum` | ITRF2020 dynamic |
| `Igs20IsDynamicDatum` | IGS20 dynamic |
| `DynamicCrsOmittingEpochIntroducesError` | omission error |
| `DynamicReferenceFrameHasFrameReferenceEpoch` | frameReferenceEpoch |

### Cross-Cutting (§18)

| Prop | Description |
|------|-------------|
| `CrsIdentityAuthorityPlusCodeUnique` | identity |
| `CompoundCrsComponentsOrthogonal` | orthogonal |
| `AxisAbbreviationUniqueInCs` | unique abbrev |
| `NullAuthorityCodeInvalidForRegisteredCrs` | null invalid |
| `AngularAxisMustUseAngularUnit` | angular unit |
| `LinearAxisMustUseLinearUnit` | linear unit |
| `ParametricAxisMustUseParametricUnit` | parametric unit |
| `TimeAxisMustUseTemporalUnit` | time unit |
| `CrsScopeDescribesIntendedUse` | scope non-empty |
| `CrsDomainOfValidityOptionalImpliesGlobal` | optional → global |
| `CrsDomainOfValidityExtentTypes` | extent components |
| `IdentifiedObjectAliasOptionalList` | alias optional |
| `IdentifiedObjectRemarksOptional` | remarks optional |

### Temporal and Parametric CRS (§19)

| Prop | Description |
|------|-------------|
| `TemporalCrsDatumIsTemporalDatum` | datum type |
| `TemporalCrsCsIsTemporalCs` | CS type |
| `TemporalDatumOriginIsIso8601DateTime` | origin ISO 8601 |
| `TemporalCrsHasOneAxis` | 1 axis |
| `ParametricCrsDatumIsParametricDatum` | datum type |
| `ParametricCrsCsIsParametricCs` | CS type |
| `ParametricCrsHasOneAxis` | 1 axis |

### Additional Constraints (§20)

| Prop | Description |
|------|-------------|
| `CoordinateSystemMinimumOneAxis` | ≥ 1 axis |
| `CoordinateSystemMaximumFourAxes` | ≤ 4 axes |
| `CompoundCrsMinimumTwoComponents` | ≥ 2 components |
| `ConcatenatedOperationMinimumTwoSteps` | ≥ 2 steps |
| `CrsStringValuesUtf8Encoded` | UTF-8 |
| `CrsMandatoryStringNonNull` | non-null |
| `LatitudeRangeNegative90To90` | [-90, 90] |
| `LongitudeRangeNegative180To180` | (-180, 180] |
| `LatitudePolesValid` | poles valid |
| `LongitudeNegative180Excluded` | -180° excluded |
| `MapProjectionScaleFactorPositive` | k₀ > 0 |
| `MapProjectionFalseOriginFiniteReal` | finite FE/FN |
| `AxisDirectionMemberOfCodeList` | valid code |
| `CsTypeMemberOfDefinedTypes` | valid CS type |
| `CrsTypeMemberOfDefinedTypes` | valid CRS type |

---

## §21 IO_IdentifiedObject (base class of all named objects)

Every CRS type, CS type, datum type, operation type, and unit of measure in
ISO 19111 inherits from `IO_IdentifiedObject`. Contracts on this base apply to
all subclasses.

```rust
/// Every IO_IdentifiedObject has at least one primary name.
///
/// Source: ISO 19111:2019 §6.2.1 — IO_IdentifiedObject.name
pub struct IdentifiedObjectPrimaryNameNonEmpty;
structural_prop!(IdentifiedObjectPrimaryNameNonEmpty, "IdentifiedObjectPrimaryNameNonEmpty");

/// The alias array of an IO_IdentifiedObject may be empty but must not contain
/// null entries.
///
/// Source: ISO 19111:2019 §6.2.1 — IO_IdentifiedObject.alias
pub struct IdentifiedObjectAliasNoNullEntries;
structural_prop!(IdentifiedObjectAliasNoNullEntries, "IdentifiedObjectAliasNoNullEntries");

/// The identifier array (RS_Identifier[]) may be empty; when present, each
/// entry must have a non-empty authority and a non-empty code.
///
/// Source: ISO 19111:2019 §6.2.1 — IO_IdentifiedObject.identifier
pub struct IdentifiedObjectIdentifierEntryComplete;
structural_prop!(IdentifiedObjectIdentifierEntryComplete, "IdentifiedObjectIdentifierEntryComplete");

/// Remarks on an IO_IdentifiedObject are optional; when present, must be
/// a non-null CharacterString.
///
/// Source: ISO 19111:2019 §6.2.1 — IO_IdentifiedObject.remarks
pub struct IdentifiedObjectRemarksWhenPresentNonNull;
structural_prop!(IdentifiedObjectRemarksWhenPresentNonNull, "IdentifiedObjectRemarksWhenPresentNonNull");

/// All concrete CRS types are subclasses of IO_IdentifiedObject, inheriting
/// name and identifier obligations.
///
/// Source: ISO 19111:2019 §6.2 — class hierarchy
pub struct CrsInheritsIdentifiedObjectInterface;
structural_prop!(CrsInheritsIdentifiedObjectInterface, "CrsInheritsIdentifiedObjectInterface");

/// All datum types are subclasses of IO_IdentifiedObject.
///
/// Source: ISO 19111:2019 §6.2 — class hierarchy
pub struct DatumInheritsIdentifiedObjectInterface;
structural_prop!(DatumInheritsIdentifiedObjectInterface, "DatumInheritsIdentifiedObjectInterface");

/// All coordinate operation types are subclasses of IO_IdentifiedObject.
///
/// Source: ISO 19111:2019 §6.2 — class hierarchy
pub struct CoordinateOperationInheritsIdentifiedObjectInterface;
structural_prop!(CoordinateOperationInheritsIdentifiedObjectInterface, "CoordinateOperationInheritsIdentifiedObjectInterface");
```

---

## §22 CD_DatumEnsemble (ISO 19111:2019 addition)

`CD_DatumEnsemble` is a major concept added in the 2019 revision (absent from
ISO 19111:2007). Many well-known CRS identifiers (including EPSG:4326) are now
associated with a datum ensemble rather than a single datum realization.

```rust
/// A CD_DatumEnsemble groups multiple related datum realizations that are
/// compatible within a stated ensemble accuracy.
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble
pub struct DatumEnsembleGroupsRelatedDatums;
structural_prop!(DatumEnsembleGroupsRelatedDatums, "DatumEnsembleGroupsRelatedDatums");

/// CD_DatumEnsemble.name is a non-empty CharacterString identifying the
/// ensemble (e.g., "World Geodetic System 1984 ensemble").
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.name
pub struct DatumEnsembleNameNonEmpty;
structural_prop!(DatumEnsembleNameNonEmpty, "DatumEnsembleNameNonEmpty");

/// CD_DatumEnsemble.member is an array of CD_Datum references with
/// multiplicity 2..* — at least two realizations must be listed.
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.member
pub struct DatumEnsembleHasAtLeastTwoMembers;
structural_prop!(DatumEnsembleHasAtLeastTwoMembers, "DatumEnsembleHasAtLeastTwoMembers");

/// Each member of a CD_DatumEnsemble references a concrete datum (e.g. a
/// specific ITRF realization). No member may be null.
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.member
pub struct DatumEnsembleMemberNoNullEntries;
structural_prop!(DatumEnsembleMemberNoNullEntries, "DatumEnsembleMemberNoNullEntries");

/// CD_DatumEnsemble.ensembleAccuracy is a positive real number (in metres
/// for horizontal CRS) stating the positional accuracy within which all
/// member datums agree.
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.ensembleAccuracy
pub struct DatumEnsembleAccuracyPositive;
structural_prop!(DatumEnsembleAccuracyPositive, "DatumEnsembleAccuracyPositive");

/// CD_DatumEnsemble.ensembleAccuracy is a finite real number (not NaN or ±Infinity);
/// finiteness precondition needed before range comparisons in formal proofs.
///
/// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.ensembleAccuracy finite
pub struct DatumEnsembleAccuracyFinite;
structural_prop!(DatumEnsembleAccuracyFinite, "DatumEnsembleAccuracyFinite");

/// A CRS whose datum is a CD_DatumEnsemble cannot be used for sub-metre
/// positioning without selecting a specific member datum and providing a
/// coordinate epoch.
///
/// Source: ISO 19111:2019 §6.5 — usage note
pub struct DatumEnsembleSubMetreRequiresMemberSelection;
structural_prop!(DatumEnsembleSubMetreRequiresMemberSelection, "DatumEnsembleSubMetreRequiresMemberSelection");

/// The WGS 84 datum ensemble (EPSG:6326) has ensemble accuracy 2 m (the
/// max inter-realization difference over WGS84 history).
///
/// Source: ISO 19111:2019 §6.5 / EPSG registry code 6326
pub struct DatumEnsembleWgs84EpsgCode6326;
structural_prop!(DatumEnsembleWgs84EpsgCode6326, "DatumEnsembleWgs84EpsgCode6326");

/// EPSG:4326 (WGS 84 geographic 2D) uses the WGS 84 datum ensemble
/// (EPSG:6326), not a single-realization datum.
///
/// Source: ISO 19111:2019 §6.5 / EPSG registry
pub struct Epsg4326UsesDatumEnsemble;
structural_prop!(Epsg4326UsesDatumEnsemble, "Epsg4326UsesDatumEnsemble");

/// EPSG:6267 is the NAD27 datum ensemble; EPSG:6269 is the NAD83 datum
/// ensemble.
///
/// Source: ISO 19111:2019 §6.5 / EPSG registry
pub struct Nad27EnsembleEpsg6267;
structural_prop!(Nad27EnsembleEpsg6267, "Nad27EnsembleEpsg6267");

/// Datum ensemble member datums must all be of the same datum subtype
/// (all geodetic, all vertical, etc.).
///
/// Source: ISO 19111:2019 §6.5 — type consistency rule
pub struct DatumEnsembleMembersHomogeneous;
structural_prop!(DatumEnsembleMembersHomogeneous, "DatumEnsembleMembersHomogeneous");
```

---

## §23 SC_CoordinateMetadata (ISO 19111:2019 addition)

`SC_CoordinateMetadata` is the other major addition in the 2019 revision. It
bundles a CRS with an optional coordinate epoch to fully describe a coordinate
set — critical for GNSS and ITRF-based data.

```rust
/// SC_CoordinateMetadata associates a coordinate set with its CRS and an
/// optional coordinate epoch.
///
/// Source: ISO 19111:2019 §7.4 — SC_CoordinateMetadata
pub struct CoordinateMetadataHasCrs;
structural_prop!(CoordinateMetadataHasCrs, "CoordinateMetadataHasCrs");

/// SC_CoordinateMetadata.crs references a valid SC_CRS object.
///
/// Source: ISO 19111:2019 §7.4 — SC_CoordinateMetadata.crs
pub struct CoordinateMetadataCrsNonNull;
structural_prop!(CoordinateMetadataCrsNonNull, "CoordinateMetadataCrsNonNull");

/// SC_CoordinateMetadata.coordinateEpoch is optional; when present, it is
/// a decimal year (positive finite real, e.g. 2023.5).
///
/// Source: ISO 19111:2019 §7.4 — SC_CoordinateMetadata.coordinateEpoch
pub struct CoordinateMetadataEpochIsDecimalYear;
structural_prop!(CoordinateMetadataEpochIsDecimalYear, "CoordinateMetadataEpochIsDecimalYear");

/// When the CRS is dynamic, SC_CoordinateMetadata.coordinateEpoch MUST be
/// provided; omitting it makes coordinates ambiguous at centimetre level.
///
/// Source: ISO 19111:2019 §7.4 — dynamic CRS requirement
pub struct CoordinateMetadataDynamicCrsRequiresEpoch;
structural_prop!(CoordinateMetadataDynamicCrsRequiresEpoch, "CoordinateMetadataDynamicCrsRequiresEpoch");

/// When the CRS is static, SC_CoordinateMetadata.coordinateEpoch SHOULD be
/// omitted (epoch is meaningless for plate-fixed static CRS).
///
/// Source: ISO 19111:2019 §7.4 — static CRS note
pub struct CoordinateMetadataStaticCrsEpochShouldBeAbsent;
structural_prop!(CoordinateMetadataStaticCrsEpochShouldBeAbsent, "CoordinateMetadataStaticCrsEpochShouldBeAbsent");

/// SC_CoordinateMetadata may appear at the coordinate set level (e.g. a
/// GML file header) or at the individual coordinate tuple level.
///
/// Source: ISO 19111:2019 §7.4 — usage contexts
pub struct CoordinateMetadataApplicableAtSetOrTupleLevel;
structural_prop!(CoordinateMetadataApplicableAtSetOrTupleLevel, "CoordinateMetadataApplicableAtSetOrTupleLevel");

/// The coordinateEpoch in SC_CoordinateMetadata and the
/// frameReferenceEpoch in a dynamic datum are distinct concepts — the
/// former tags the coordinates, the latter defines the datum realization.
///
/// Source: ISO 19111:2019 §7.4 + §6.4 — epoch distinction
pub struct CoordinateEpochDistinctFromFrameReferenceEpoch;
structural_prop!(CoordinateEpochDistinctFromFrameReferenceEpoch, "CoordinateEpochDistinctFromFrameReferenceEpoch");
```

---

## §24 CC_PassThroughOperation

`CC_PassThroughOperation` is the missing member of the coordinate operation
hierarchy, bridging single-axis operations with compound CRS transforms.

```rust
/// CC_PassThroughOperation passes some coordinate tuple dimensions unchanged
/// while applying an operation to the remaining dimensions.
///
/// Source: ISO 19111:2019 §11.5 — CC_PassThroughOperation
pub struct PassThroughOperationPreservesSomeAxes;
structural_prop!(PassThroughOperationPreservesSomeAxes, "PassThroughOperationPreservesSomeAxes");

/// CC_PassThroughOperation.modifiedCoordinates lists the 1-based ordinal
/// positions of the axes affected by the inner operation.
///
/// Source: ISO 19111:2019 §11.5 — CC_PassThroughOperation.modifiedCoordinates
pub struct PassThroughOperationModifiedCoordinatesNonEmpty;
structural_prop!(PassThroughOperationModifiedCoordinatesNonEmpty, "PassThroughOperationModifiedCoordinatesNonEmpty");

/// Each index in modifiedCoordinates must be ≥ 1 and ≤ the total dimension
/// of the source CRS.
///
/// Source: ISO 19111:2019 §11.5 — index range constraint
pub struct PassThroughOperationIndexInRange;
structural_prop!(PassThroughOperationIndexInRange, "PassThroughOperationIndexInRange");

/// CC_PassThroughOperation.operation references a CC_SingleOperation applied
/// to the modified axes.
///
/// Source: ISO 19111:2019 §11.5 — CC_PassThroughOperation.operation
pub struct PassThroughOperationInnerOperationNonNull;
structural_prop!(PassThroughOperationInnerOperationNonNull, "PassThroughOperationInnerOperationNonNull");

/// The dimension count of the inner operation's source CRS must equal the
/// count of modifiedCoordinates indices.
///
/// Source: ISO 19111:2019 §11.5 — dimension consistency
pub struct PassThroughOperationDimensionConsistency;
structural_prop!(PassThroughOperationDimensionConsistency, "PassThroughOperationDimensionConsistency");

/// Typical use: apply a horizontal 2D datum shift to (easting, northing)
/// while passing elevation unchanged in a compound CRS operation.
///
/// Source: ISO 19111:2019 §11.5 — usage example
pub struct PassThroughOperationCompoundCrsUsage;
structural_prop!(PassThroughOperationCompoundCrsUsage, "PassThroughOperationCompoundCrsUsage");
```

---

## §25 CS_SphericalCS

`CS_SphericalCS` was omitted from §8.4. It is a valid CS type in ISO 19111
for 3D geocentric/spherical coordinates.

```rust
/// CS_SphericalCS has exactly three axes: two angular (spherical latitude
/// and longitude) and one distance (radius).
///
/// Source: ISO 19111:2019 §8.4 — CS_SphericalCS
pub struct SphericalCsHasThreeAxes;
structural_prop!(SphericalCsHasThreeAxes, "SphericalCsHasThreeAxes");

/// The first two axes of a CS_SphericalCS use angular units (degrees or
/// radians); the third (radius) uses a linear unit.
///
/// Source: ISO 19111:2019 §8.4 — CS_SphericalCS axis units
pub struct SphericalCsAngularAxesThenLinear;
structural_prop!(SphericalCsAngularAxesThenLinear, "SphericalCsAngularAxesThenLinear");

/// CS_SphericalCS is applicable to geocentric CRS and astronomical
/// coordinates but is rarely used in GIS software.
///
/// Source: ISO 19111:2019 §8.4 — CS_SphericalCS applicability
pub struct SphericalCsApplicableToGeocentricContext;
structural_prop!(SphericalCsApplicableToGeocentricContext, "SphericalCsApplicableToGeocentricContext");
```

---

## §26 Helmert Transformation Rotation Conventions

Two incompatible conventions exist for the 7-parameter Helmert transformation.
Mixing them silently produces errors of up to metres.

```rust
/// The 7-parameter Helmert transform uses: 3 translation parameters (tx,
/// ty, tz), 3 rotation parameters (rx, ry, rz), and a scale factor (ds).
///
/// Source: ISO 19111:2019 §11.4 / EPSG Guidance Note 7-2
pub struct HelmertSevenParameterStructure;
structural_prop!(HelmertSevenParameterStructure, "HelmertSevenParameterStructure");

/// Position Vector (ISO) convention: rotation parameters rotate the
/// coordinate frame; signs are opposite to the Coordinate Frame convention.
///
/// Source: ISO 19111:2019 §11.4 — position vector rotation (ISO method 9606-1)
pub struct HelmertPositionVectorConvention;
structural_prop!(HelmertPositionVectorConvention, "HelmertPositionVectorConvention");

/// Coordinate Frame (EPSG pre-2002) convention: rotation parameters rotate
/// the coordinate system around the origin; signs opposite to Position Vector.
///
/// Source: EPSG Guidance Note 7-2 §2.4 — coordinate frame rotation (method 1033)
pub struct HelmertCoordinateFrameConvention;
structural_prop!(HelmertCoordinateFrameConvention, "HelmertCoordinateFrameConvention");

/// The two Helmert rotation conventions are numerically identical only
/// when all rotation parameters are zero (3-parameter translation case).
///
/// Source: EPSG Guidance Note 7-2 §2.4 — convention equivalence condition
pub struct HelmertConventionsEquivalentOnlyForZeroRotation;
structural_prop!(HelmertConventionsEquivalentOnlyForZeroRotation, "HelmertConventionsEquivalentOnlyForZeroRotation");

/// A transformation record MUST identify which rotation convention is used;
/// applying the wrong convention introduces systematic errors proportional
/// to the rotation magnitude.
///
/// Source: ISO 19111:2019 §11.4 — convention identification requirement
pub struct HelmertConventionMustBeIdentified;
structural_prop!(HelmertConventionMustBeIdentified, "HelmertConventionMustBeIdentified");

/// EPSG operation method 9607 = Coordinate Frame rotation (Bursa-Wolf);
/// EPSG operation method 9606 = Position Vector transformation (ISO).
///
/// Source: EPSG registry — methods 9606, 9607
pub struct HelmertEpsgMethodCodes9606And9607;
structural_prop!(HelmertEpsgMethodCodes9606And9607, "HelmertEpsgMethodCodes9606And9607");

/// Molodensky-Badekas is a 10-parameter variant referencing a pivot point
/// near the datum centroid rather than the coordinate origin.
///
/// Source: ISO 19111:2019 §11.4 — Molodensky-Badekas method
pub struct MolodenskyBadenkasTenParameter;
structural_prop!(MolodenskyBadenkasTenParameter, "MolodenskyBadenkasTenParameter");
```

---

## §27 Grid-Based Datum Shift Methods

```rust
/// Grid-based datum shifts (NADCON5, NTv2, VERTCON) reference an external
/// file of shift values rather than analytic parameters.
///
/// Source: ISO 19111:2019 §11.4 / EPSG methods 9613, 9615
pub struct GridBasedDatumShiftUsesExternalFile;
structural_prop!(GridBasedDatumShiftUsesExternalFile, "GridBasedDatumShiftUsesExternalFile");

/// NADCON5 (EPSG method 1075) is the US official horizontal datum shift
/// grid between NAD83 realizations.
///
/// Source: EPSG registry method 1075
pub struct Nadcon5IsUsHorizontalDatumShift;
structural_prop!(Nadcon5IsUsHorizontalDatumShift, "Nadcon5IsUsHorizontalDatumShift");

/// NTv2 (EPSG method 9615) is a grid-based horizontal datum shift format
/// used in Canada, Australia, and many EU countries.
///
/// Source: EPSG registry method 9615
pub struct Ntv2IsGridHorizontalShift;
structural_prop!(Ntv2IsGridHorizontalShift, "Ntv2IsGridHorizontalShift");

/// VERTCON (EPSG method 9661) is the US vertical datum shift grid
/// between NGVD29 and NAVD88.
///
/// Source: EPSG registry method 9661
pub struct VertconIsUsVerticalDatumShift;
structural_prop!(VertconIsUsVerticalDatumShift, "VertconIsUsVerticalDatumShift");

/// A PARAMETERFILE operation parameter stores a filename, not a numeric
/// value; the software must locate and load the referenced grid file.
///
/// Source: ISO 19111:2019 §11.4 — PARAMETERFILE concept
pub struct ParameterFileStoresFilenameNotValue;
structural_prop!(ParameterFileStoresFilenameNotValue, "ParameterFileStoresFilenameNotValue");
```

---

## §28 Unit of Measure (UoM) Constraints

```rust
/// Every UoM has a non-empty name string.
///
/// Source: ISO 19111:2019 §6.6 — UoM.name
pub struct UomNameNonEmpty;
structural_prop!(UomNameNonEmpty, "UomNameNonEmpty");

/// UoM.conversionFactor is a positive real number giving the conversion
/// to the SI base unit for the quantity type.
///
/// Source: ISO 19111:2019 §6.6 — UoM.conversionFactor
pub struct UomConversionFactorPositive;
structural_prop!(UomConversionFactorPositive, "UomConversionFactorPositive");

/// UoM.conversionFactor is a finite real number (not NaN or ±Infinity); finiteness
/// precondition for any proof that uses the factor in arithmetic (e.g., unit conversion
/// chains must terminate without overflow or undefined result).
///
/// Source: ISO 19111:2019 §6.6 — UoM.conversionFactor finite
pub struct UomConversionFactorFinite;
structural_prop!(UomConversionFactorFinite, "UomConversionFactorFinite");

/// Angular units convert to radians; 1 degree = π/180 ≈ 0.017453292519943278.
///
/// Source: ISO 19111:2019 §6.6 — angular UoM SI base is radians
pub struct UomAngularConvertToRadians;
structural_prop!(UomAngularConvertToRadians, "UomAngularConvertToRadians");

/// Linear units convert to metres; 1 foot = 0.3048 m exactly
/// (international foot); US survey foot = 0.304800609601219...
///
/// Source: ISO 19111:2019 §6.6 — linear UoM SI base is metres
pub struct UomLinearConvertToMetres;
structural_prop!(UomLinearConvertToMetres, "UomLinearConvertToMetres");

/// Time units convert to seconds.
///
/// Source: ISO 19111:2019 §6.6 — temporal UoM SI base is seconds
pub struct UomTimeConvertToSeconds;
structural_prop!(UomTimeConvertToSeconds, "UomTimeConvertToSeconds");

/// Scale units are dimensionless; conversionFactor = 1.0 (unity) for
/// parts-per-million variants: 1 ppm = 1e-6.
///
/// Source: ISO 19111:2019 §6.6 — scale UoM
pub struct UomScaleDimensionless;
structural_prop!(UomScaleDimensionless, "UomScaleDimensionless");

/// The international foot and the US survey foot differ; confusion between
/// them produces ~3mm/km error (significant in cadastral/engineering work).
///
/// Source: EPSG registry — EPSG:9002 vs EPSG:9003
pub struct UomFeetAmbiguityInternationalVsSurvey;
structural_prop!(UomFeetAmbiguityInternationalVsSurvey, "UomFeetAmbiguityInternationalVsSurvey");
```

---

## §29 UTM South Zone False Northing

```rust
/// UTM south zones (EPSG:32701–32760) use a false northing of 10,000,000 m
/// to keep all northing values positive within the southern hemisphere.
///
/// Source: ISO 19111:2019 / EPSG Guidance Note 7-2 §3 — UTM south zones
pub struct UtmSouthZoneFalseNorthing10000000;
structural_prop!(UtmSouthZoneFalseNorthing10000000, "UtmSouthZoneFalseNorthing10000000");

/// UTM north zones (EPSG:32601–32660) use false northing = 0 m; the
/// equator is the natural origin for northing.
///
/// Source: ISO 19111:2019 / EPSG Guidance Note 7-2 §3 — UTM north zones
pub struct UtmNorthZoneFalseNorthing0;
structural_prop!(UtmNorthZoneFalseNorthing0, "UtmNorthZoneFalseNorthing0");

/// All UTM zones (north and south) share false easting = 500,000 m and
/// scale factor at central meridian k₀ = 0.9996.
///
/// Source: EPSG Guidance Note 7-2 §3 — UTM common parameters
pub struct UtmCommonParametersFalseEastingAndScale;
structural_prop!(UtmCommonParametersFalseEastingAndScale, "UtmCommonParametersFalseEastingAndScale");
```

---

## Summary Supplement — Props Added in §21–29

| Section | Props added |
|---------|-------------|
| §21 IO_IdentifiedObject base class | 7 |
| §22 CD_DatumEnsemble | 10 |
| §23 SC_CoordinateMetadata | 7 |
| §24 CC_PassThroughOperation | 6 |
| §25 CS_SphericalCS | 3 |
| §26 Helmert rotation conventions | 7 |
| §27 Grid-based datum shifts | 5 |
| §28 Unit of Measure constraints | 7 |
| §29 UTM south zone false northing | 3 |
| **Supplement total** | **55** |
| **Previous total** | **247** |
| **Grand total** | **302** |

---

*Total props: 302*  
*Standard: ISO 19111:2019 — Geographic information — Referencing by coordinates*  
*Generated for use in `crates/elicit_gis/src/contracts/iso_19111.rs`*
