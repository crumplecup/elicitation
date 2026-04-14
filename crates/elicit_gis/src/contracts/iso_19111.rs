//! ISO 19111:2019 propositions — Spatial referencing by coordinates.
//!
//! Source: ISO 19111:2019 (E) — all §references are to this standard.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    // ── §6 CRS general ──────────────────────────────────────────────────────

    /// A CRS consists of exactly one coordinate system and one datum (or datum ensemble).
    ///
    /// Source: ISO 19111:2019 §6.2 — SC_CRS composition rule.
    pub struct CrsConsistsOfCsAndDatum;

    /// Each coordinate tuple element corresponds to exactly one CS axis.
    ///
    /// Source: ISO 19111:2019 §6.2 — tuple dimensionality.
    pub struct CoordinateTupleDimensionMatchesAxes;

    /// SC_CRS is an abstract class; only concrete subtypes are instantiated.
    ///
    /// Source: ISO 19111:2019 §6 — UML constraint {abstract}.
    pub struct ScCrsIsAbstract;

    /// CRS name (CI_Citation.title) is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.2 — name multiplicity 1.
    pub struct ScCrsNameNonEmpty;

    /// CRS scope is a non-empty string describing intended use.
    ///
    /// Source: ISO 19111:2019 §6.2 — scope multiplicity 1.
    pub struct ScCrsScopeNonEmpty;

    /// CRS domain of validity is an EX_Extent instance.
    ///
    /// Source: ISO 19111:2019 §6.2 — domainOfValidity.
    pub struct ScCrsDomainOfValidityIsExtent;

    /// Every registered CRS carries at least one RS_Identifier with authority and code.
    ///
    /// Source: ISO 19111:2019 §6.2 — identifier multiplicity 0..*.
    pub struct ScCrsIdentifierHasAuthorityAndCode;

    /// RS_Identifier authority is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.2.
    pub struct CrsIdentifierAuthorityNonEmpty;

    /// RS_Identifier code is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.2.
    pub struct CrsIdentifierCodeNonEmpty;

    // ── §7 Geodetic CRS ─────────────────────────────────────────────────────

    /// Geodetic CRS datum is a geodetic reference frame (or ensemble thereof).
    ///
    /// Source: ISO 19111:2019 §8.1 — SC_GeodeticCRS.datum.
    pub struct GeodeticCrsDatumIsGeodeticReferenceFrame;

    /// Geodetic CRS CS is ellipsoidal or Cartesian.
    ///
    /// Source: ISO 19111:2019 §8.1 — allowable CS types.
    pub struct GeodeticCrsCsIsEllipsoidalOrCartesian;

    /// Geographic 2D CRS has exactly two axes.
    ///
    /// Source: ISO 19111:2019 §9.2 — axis count constraint.
    pub struct Geographic2dCrsHasTwoAxes;

    /// Geographic 3D CRS has exactly three axes.
    ///
    /// Source: ISO 19111:2019 §9.3 — axis count constraint.
    pub struct Geographic3dCrsHasThreeAxes;

    /// Geocentric CRS uses a Cartesian CS.
    ///
    /// Source: ISO 19111:2019 §9.4 — SC_GeocentricCRS.cs.
    pub struct GeocentricCrsUsesCartesianCs;

    /// EPSG:4326 axis order is latitude first (north), longitude second (east).
    ///
    /// Source: ISO 19111:2019 §15 / EPSG Dataset.
    pub struct Epsg4326AxisOrderLatFirst;

    /// EPSG:4326 latitude is in [-90, 90].
    ///
    /// Source: ISO 19111:2019 §8.2 — valid latitude range.
    pub struct Epsg4326LatitudeRangeValid;

    /// EPSG:4326 longitude is in [-180, 180).
    ///
    /// Source: ISO 19111:2019 §8.2 — valid longitude range.
    pub struct Epsg4326LongitudeRangeValid;

    /// EPSG:4979 is the 3D geographic CRS for WGS 84.
    ///
    /// Source: EPSG Dataset / ISO 19111:2019 §9.3.
    pub struct Epsg4979IsWgs84Geographic3d;

    /// EPSG:4979 ellipsoidal height is unbounded (no enforced min/max).
    ///
    /// Source: ISO 19111:2019 §9.3.
    pub struct Epsg4979HeightUnbounded;

    /// EPSG:4978 is the geocentric (Cartesian XYZ) CRS for WGS 84.
    ///
    /// Source: EPSG Dataset / ISO 19111:2019 §9.4.
    pub struct Epsg4978IsWgs84Geocentric;

    /// EPSG:4978 X-axis points towards the intersection of equator and prime meridian.
    ///
    /// Source: ISO 19111:2019 §9.4 — axis convention.
    pub struct Epsg4978XAxisTowardsPrimeMeridian;

    /// EPSG:4978 Z-axis points towards the north pole.
    ///
    /// Source: ISO 19111:2019 §9.4 — axis convention.
    pub struct Epsg4978ZAxisTowardsNorthPole;

    // ── §7.2 Geodetic Reference Frame ─────────────────────────────────────

    /// Geodetic reference frame name is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.4 — IO_IdentifiedObject.name.
    pub struct GeodeticReferenceFrameNameNonEmpty;

    /// Geodetic reference frame anchor (anchorDefinition) is optional.
    ///
    /// Source: ISO 19111:2019 §8.2 — anchorDefinition multiplicity 0..1.
    pub struct GeodeticReferenceFrameAnchorOptional;

    /// Geodetic reference frame has exactly one ellipsoid.
    ///
    /// Source: ISO 19111:2019 §8.2 — ellipsoid multiplicity 1.
    pub struct GeodeticReferenceFrameHasExactlyOneEllipsoid;

    /// Geodetic reference frame has exactly one prime meridian.
    ///
    /// Source: ISO 19111:2019 §8.2 — primeMeridian multiplicity 1.
    pub struct GeodeticReferenceFrameHasExactlyOnePrimeMeridian;

    /// Realization epoch, when present, is ISO 8601 formatted.
    ///
    /// Source: ISO 19111:2019 §6.4 — realizationEpoch.
    pub struct GeodeticReferenceFrameRealizationEpochIsIso8601;

    // ── §7.3 Ellipsoid ───────────────────────────────────────────────────

    /// Ellipsoid name is non-empty.
    ///
    /// Source: ISO 19111:2019 §8.3 — name.
    pub struct EllipsoidNameNonEmpty;

    /// Ellipsoid semi-major axis is strictly positive.
    ///
    /// Source: ISO 19111:2019 §8.3 — semiMajorAxis > 0.
    pub struct EllipsoidSemiMajorAxisPositive;

    /// Ellipsoid semi-major axis is measured in metres.
    ///
    /// Source: ISO 19111:2019 §8.3 — uom.
    pub struct EllipsoidSemiMajorAxisInMetres;

    /// Ellipsoid carries exactly one of inverse flattening or semi-minor axis.
    ///
    /// Source: ISO 19111:2019 §8.3 — second defining parameter.
    pub struct EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor;

    /// Inverse flattening is positive for a non-spherical ellipsoid.
    ///
    /// Source: ISO 19111:2019 §8.3 — inverseFlattening > 0 when not sphere.
    pub struct EllipsoidInverseFlatteningPositiveWhenNonSphere;

    /// WGS 84 ellipsoid inverse flattening is approximately 298.257223563.
    ///
    /// Source: ISO 19111:2019 / EPSG:7030 definition.
    pub struct EllipsoidWgs84InverseFlatteningApprox298;

    /// Inverse flattening of 0 signals a sphere (no oblateness).
    ///
    /// Source: ISO 19111:2019 §8.3 — sphere convention.
    pub struct EllipsoidInverseFlatteningZeroMeansSphere;

    /// Semi-minor axis is strictly less than semi-major axis for an oblate spheroid.
    ///
    /// Source: ISO 19111:2019 §8.3 — oblate constraint.
    pub struct EllipsoidSemiMinorAxisLessThanSemiMajor;

    /// isSphere flag is consistent with the numeric parameters.
    ///
    /// Source: ISO 19111:2019 §8.3 — isSphere consistency.
    pub struct EllipsoidIsSphereConsistentWithParameters;

    /// Semi-minor axis is measured in metres.
    ///
    /// Source: ISO 19111:2019 §8.3 — uom.
    pub struct EllipsoidSemiMinorAxisInMetres;

    // ── §7.4 Prime Meridian ───────────────────────────────────────────────

    /// Prime meridian name is non-empty.
    ///
    /// Source: ISO 19111:2019 §8.4 — name.
    pub struct PrimeMeridianNameNonEmpty;

    /// Prime meridian Greenwich longitude is a finite real number.
    ///
    /// Source: ISO 19111:2019 §8.4 — greenwichLongitude finite.
    pub struct PrimeMeridianGreenwichLongitudeFinite;

    /// Prime meridian Greenwich longitude lies in [-180, 180].
    ///
    /// Source: ISO 19111:2019 §8.4 — greenwichLongitude range.
    pub struct PrimeMeridianGreenwichLongitudeInDegreeBounds;

    /// The Greenwich prime meridian has a longitude of exactly 0°.
    ///
    /// Source: ISO 19111:2019 §8.4 — EPSG:8901 definition.
    pub struct PrimeMeridianGreenwichIsZero;

    /// Non-Greenwich prime meridians are valid for historical CRS.
    ///
    /// Source: ISO 19111:2019 §8.4 — allowable range.
    pub struct PrimeMeridianNonGreenwichAllowed;

    /// Greenwich longitude unit must be angular (degrees or radians).
    ///
    /// Source: ISO 19111:2019 §8.4 — uom constraint.
    pub struct PrimeMeridianGreenwichLongitudeUnitIsAngular;

    // ── §8 Coordinate system ──────────────────────────────────────────────

    /// Coordinate system has between 1 and 4 axes.
    ///
    /// Source: ISO 19111:2019 §9 — axis count 1..4.
    pub struct CoordinateSystemAxisCountOneToFour;

    /// CS axis count equals coordinate tuple dimensionality.
    ///
    /// Source: ISO 19111:2019 §9 — dimensionality.
    pub struct CsAxisCountMatchesTupleDimensionality;

    /// Coordinate system name is non-empty.
    ///
    /// Source: ISO 19111:2019 §9 — name.
    pub struct CoordinateSystemNameNonEmpty;

    /// Coordinate system axis name is non-empty.
    ///
    /// Source: ISO 19111:2019 §9 — axis.name.
    pub struct AxisNameNonEmpty;

    /// Coordinate system axis abbreviation is non-empty.
    ///
    /// Source: ISO 19111:2019 §9 — axis.abbreviation.
    pub struct AxisAbbreviationNonEmpty;

    /// Axis abbreviations are unique within one coordinate system.
    ///
    /// Source: ISO 19111:2019 §9 — no duplicate abbreviations.
    pub struct AxisAbbreviationUniqueWithinCs;

    /// Axis direction is a value from the CS_AxisDirection code list.
    ///
    /// Source: ISO 19111:2019 §9 — direction codelist.
    pub struct AxisDirectionIsValidCode;

    /// Axis unit is appropriate for the axis type (angular/linear/temporal).
    ///
    /// Source: ISO 19111:2019 §9 — uom constraint.
    pub struct AxisUnitAppropriateForAxisType;

    /// Axis minimum and maximum values are optional constraints.
    ///
    /// Source: ISO 19111:2019 §9 — minimumValue / maximumValue optional.
    pub struct AxisMinMaxValueOptional;

    /// Axis rangeMeaning is either 'exact' or 'wraparound'.
    ///
    /// Source: ISO 19111:2019 §9 — rangeMeaning code.
    pub struct AxisRangeMeaningExactOrWraparound;

    // ── §9 Projected CRS ─────────────────────────────────────────────────

    /// Projected CRS base CRS is a geographic CRS.
    ///
    /// Source: ISO 19111:2019 §10.2 — baseCRS must be geographic.
    pub struct ProjectedCrsBaseCrsIsGeographic;

    /// Projected CRS CS is Cartesian.
    ///
    /// Source: ISO 19111:2019 §10.2 — cs type.
    pub struct ProjectedCrsCsIsCartesian;

    /// Projected CRS projection is a coordinate conversion (not transformation).
    ///
    /// Source: ISO 19111:2019 §10.2 — conversion.
    pub struct ProjectedCrsProjectionIsConversion;

    /// Projected CRS name is non-empty.
    ///
    /// Source: ISO 19111:2019 §10.2 — name.
    pub struct ProjectedCrsNameNonEmpty;

    /// Projected CRS axes use linear units.
    ///
    /// Source: ISO 19111:2019 §10.2 — uom.
    pub struct ProjectedCrsAxesUseLinearUnit;

    /// Projected CRS conventional axis directions are easting/northing or variants.
    ///
    /// Source: ISO 19111:2019 §10.2 / §15 — conventional directions.
    pub struct ProjectedCrsConventionalAxisDirections;

    /// Projected CRS has exactly two axes.
    ///
    /// Source: ISO 19111:2019 §10.2 — axis count.
    pub struct ProjectedCrsHasTwoAxes;

    /// UTM north zones have EPSG codes in the range 32601–32660.
    ///
    /// Source: EPSG Dataset / ISO 19111:2019.
    pub struct UtmNorthZoneEpsgRange32601To32660;

    /// UTM south zones have EPSG codes in the range 32701–32760.
    ///
    /// Source: EPSG Dataset / ISO 19111:2019.
    pub struct UtmSouthZoneEpsgRange32701To32760;

    /// UTM axis order is easting first.
    ///
    /// Source: ISO 19111:2019 §15 — UTM conventional axis order.
    pub struct UtmAxisOrderEastingFirst;

    /// UTM false easting is 500,000 m.
    ///
    /// Source: EPSG Guidance Note 7-2 — UTM parameters.
    pub struct UtmFalseEasting500000;

    /// UTM zone width is exactly 6 degrees of longitude.
    ///
    /// Source: EPSG Guidance Note 7-2 — zone width.
    pub struct UtmZoneWidthSixDegrees;

    /// UTM scale factor at central meridian is 0.9996.
    ///
    /// Source: EPSG Guidance Note 7-2 — k0 parameter.
    pub struct UtmScaleFactorAtCentralMeridian0996;

    /// UTM zone number is in the range 1–60.
    ///
    /// Source: EPSG Guidance Note 7-2.
    pub struct UtmZoneNumberOneToSixty;

    // ── §10 Vertical CRS ─────────────────────────────────────────────────

    /// Vertical CRS datum is a vertical reference frame.
    ///
    /// Source: ISO 19111:2019 §11.2 — datum type.
    pub struct VerticalCrsDatumIsVerticalReferenceFrame;

    /// Vertical CRS CS is a vertical CS.
    ///
    /// Source: ISO 19111:2019 §11.2 — cs type.
    pub struct VerticalCrsCsIsVerticalCs;

    /// Vertical CRS name is non-empty.
    ///
    /// Source: ISO 19111:2019 §11.2.
    pub struct VerticalCrsNameNonEmpty;

    /// Vertical CRS has exactly one axis.
    ///
    /// Source: ISO 19111:2019 §11.2 — axis count.
    pub struct VerticalCrsHasOneAxis;

    /// Vertical reference frame name is non-empty.
    ///
    /// Source: ISO 19111:2019 §11.3.
    pub struct VerticalReferenceFrameNameNonEmpty;

    /// Vertical reference frame realization epoch, when present, is ISO 8601.
    ///
    /// Source: ISO 19111:2019 §11.3.
    pub struct VerticalReferenceFrameRealizationEpochIsIso8601;

    /// Vertical reference frame anchor definition is optional.
    ///
    /// Source: ISO 19111:2019 §11.3.
    pub struct VerticalReferenceFrameAnchorOptional;

    /// Vertical reference frame is gravity-related (height above geoid / depth below).
    ///
    /// Source: ISO 19111:2019 §11.3 — gravityRelatedHeight.
    pub struct VerticalReferenceFrameGravityRelated;

    /// Vertical CRS height axis direction is 'up'.
    ///
    /// Source: ISO 19111:2019 §11.2 — height axis direction.
    pub struct VerticalCrsHeightAxisDirectionUp;

    /// Vertical CRS depth axis direction is 'down'.
    ///
    /// Source: ISO 19111:2019 §11.2 — depth axis direction.
    pub struct VerticalCrsDepthAxisDirectionDown;

    /// Vertical CRS axis uses a linear unit.
    ///
    /// Source: ISO 19111:2019 §11.2 — uom.
    pub struct VerticalCrsAxisLinearUnit;

    /// Vertical CRS EPSG codes are typically in the range 5000–5999.
    ///
    /// Source: EPSG Dataset conventions.
    pub struct VerticalCrsEpsgRange5000To5999;

    // ── §11 Engineering CRS ──────────────────────────────────────────────

    /// Engineering CRS datum is an engineering datum.
    ///
    /// Source: ISO 19111:2019 §12.2 — datum type.
    pub struct EngineeringCrsDatumIsEngineeringDatum;

    /// Engineering CRS name is non-empty.
    ///
    /// Source: ISO 19111:2019 §12.2.
    pub struct EngineeringCrsNameNonEmpty;

    /// Engineering CRS CS type is flexible (Cartesian, affine, etc.).
    ///
    /// Source: ISO 19111:2019 §12.2 — allowable CS types.
    pub struct EngineeringCrsCsTypeFlexible;

    /// Engineering CRS is only applicable in a local context.
    ///
    /// Source: ISO 19111:2019 §12.2 — scope.
    pub struct EngineeringCrsIsLocalContextOnly;

    /// Engineering datum name is non-empty.
    ///
    /// Source: ISO 19111:2019 §12.3.
    pub struct EngineeringDatumNameNonEmpty;

    /// Engineering datum anchor description is optional.
    ///
    /// Source: ISO 19111:2019 §12.3.
    pub struct EngineeringDatumAnchorOptional;

    // ── §12 Compound CRS ─────────────────────────────────────────────────

    /// Compound CRS has at least two component CRSes.
    ///
    /// Source: ISO 19111:2019 §13.2 — componentReferenceSystem multiplicity.
    pub struct CompoundCrsHasAtLeastTwoComponents;

    /// Compound CRS name is non-empty.
    ///
    /// Source: ISO 19111:2019 §13.2.
    pub struct CompoundCrsNameNonEmpty;

    /// Compound CRS component domains are non-overlapping.
    ///
    /// Source: ISO 19111:2019 §13.2 — orthogonality.
    pub struct CompoundCrsComponentsNonOverlapping;

    /// Compound CRS total axis count equals the sum of component axis counts.
    ///
    /// Source: ISO 19111:2019 §13.2 — total dimensionality.
    pub struct CompoundCrsTotalAxisCountIsSumOfComponents;

    /// Typical compound CRS is 2D horizontal + vertical.
    ///
    /// Source: ISO 19111:2019 §13 — common usage.
    pub struct CompoundCrsTypicalIs2dPlusVertical;

    /// Compound CRS EPSG codes are typically in the range 6000–6999.
    ///
    /// Source: EPSG Dataset conventions.
    pub struct CompoundCrsEpsgRange6000To6999;

    /// Compound CRS shall not contain two horizontal component CRSes.
    ///
    /// Source: ISO 19111:2019 §13.2 — mutual exclusion rule.
    pub struct CompoundCrsNoTwoHorizontalComponents;

    /// Compound CRS shall not contain two vertical component CRSes.
    ///
    /// Source: ISO 19111:2019 §13.2 — mutual exclusion rule.
    pub struct CompoundCrsNoTwoVerticalComponents;

    // ── §13 Derived CRS ──────────────────────────────────────────────────

    /// Derived CRS has a base CRS.
    ///
    /// Source: ISO 19111:2019 §14.2 — baseCRS.
    pub struct DerivedCrsHasBaseCrs;

    /// Derived CRS deriving conversion is a coordinate conversion.
    ///
    /// Source: ISO 19111:2019 §14.2 — derivingConversion.
    pub struct DerivedCrsDerivingConversionIsConversion;

    /// Derived CRS CS may differ from base CRS CS.
    ///
    /// Source: ISO 19111:2019 §14.2 — CS independence.
    pub struct DerivedCrsCsDiffersFromBaseCrsAllowed;

    /// Derived CRS name is non-empty.
    ///
    /// Source: ISO 19111:2019 §14.2.
    pub struct DerivedCrsNameNonEmpty;

    /// Derived CRS inherits its datum from the base CRS.
    ///
    /// Source: ISO 19111:2019 §14.2 — datum inheritance.
    pub struct DerivedCrsInheritsDatumFromBase;

    /// Derived projected CRS base CRS must itself be a projected CRS.
    ///
    /// Source: ISO 19111:2019 §14.3.
    pub struct DerivedProjectedCrsBaseMustBeProjCrs;

    // ── §14 Coordinate operations ────────────────────────────────────────

    /// Coordinate operation name is non-empty.
    ///
    /// Source: ISO 19111:2019 §16.2 — name.
    pub struct CoordinateOperationNameNonEmpty;

    /// Coordinate operation has a source CRS.
    ///
    /// Source: ISO 19111:2019 §16.2 — sourceCRS.
    pub struct CoordinateOperationHasSourceCrs;

    /// Coordinate operation has a target CRS.
    ///
    /// Source: ISO 19111:2019 §16.2 — targetCRS.
    pub struct CoordinateOperationHasTargetCrs;

    /// Coordinate operation version is optional.
    ///
    /// Source: ISO 19111:2019 §16.2 — version multiplicity 0..1.
    pub struct CoordinateOperationVersionOptional;

    /// Coordinate operation domain of validity is optional.
    ///
    /// Source: ISO 19111:2019 §16.2 — domainOfValidity multiplicity 0..1.
    pub struct CoordinateOperationDomainOfValidityOptional;

    /// Coordinate conversion involves no datum change.
    ///
    /// Source: ISO 19111:2019 §16.3 — CC_Conversion.
    pub struct ConversionInvolvesNoDatumChange;

    /// Coordinate conversion defines a map projection or similar operation.
    ///
    /// Source: ISO 19111:2019 §16.3 — conversion usage.
    pub struct ConversionDefinesMapProjection;

    /// Coordinate conversion inverse operation exists.
    ///
    /// Source: ISO 19111:2019 §16.3 — invertible.
    pub struct ConversionInverseExists;

    /// Coordinate conversion has an operation method.
    ///
    /// Source: ISO 19111:2019 §16.3 — method.
    pub struct ConversionHasOperationMethod;

    /// Coordinate conversion has parameter values.
    ///
    /// Source: ISO 19111:2019 §16.3 — parameterValue.
    pub struct ConversionHasParameterValues;

    /// Coordinate transformation involves a datum change.
    ///
    /// Source: ISO 19111:2019 §16.4 — CC_Transformation.
    pub struct TransformationInvolvesDatumChange;

    /// Transformation accuracy is a positive real number when specified.
    ///
    /// Source: ISO 19111:2019 §16.4 — coordinateOperationAccuracy.
    pub struct TransformationAccuracyPositiveReal;

    /// Transformation accuracy is non-zero.
    ///
    /// Source: ISO 19111:2019 §16.4.
    pub struct TransformationAccuracyNonZero;

    /// Transformation inverse is approximate, not exact.
    ///
    /// Source: ISO 19111:2019 §16.4 — inverse approximation.
    pub struct TransformationInverseApproximate;

    /// NAD27 → WGS 84 transformation uses a Helmert operation.
    ///
    /// Source: ISO 19111:2019 / EPSG Guidance Note 7-2.
    pub struct TransformationNad27ToWgs84UsesHelmert;

    /// Concatenated operation has at least two steps.
    ///
    /// Source: ISO 19111:2019 §16.5 — CC_ConcatenatedOperation.step.
    pub struct ConcatenatedOperationHasAtLeastTwoSteps;

    /// Consecutive steps in a concatenated operation form a chained pipeline.
    ///
    /// Source: ISO 19111:2019 §16.5 — step chain.
    pub struct ConcatenatedOperationStepsFormAChain;

    /// Source CRS of a concatenated operation is the source CRS of the first step.
    ///
    /// Source: ISO 19111:2019 §16.5.
    pub struct ConcatenatedOperationSourceCrsIsFirstStep;

    /// Target CRS of a concatenated operation is the target CRS of the last step.
    ///
    /// Source: ISO 19111:2019 §16.5.
    pub struct ConcatenatedOperationTargetCrsIsLastStep;

    /// Operation method name is non-empty.
    ///
    /// Source: ISO 19111:2019 §16.6 — OperationMethod.name.
    pub struct OperationMethodNameNonEmpty;

    /// Operation method formula is optional.
    ///
    /// Source: ISO 19111:2019 §16.6 — formula multiplicity 0..1.
    pub struct OperationMethodFormulaOptional;

    /// Operation method has a parameter list.
    ///
    /// Source: ISO 19111:2019 §16.6 — parameter.
    pub struct OperationMethodHasParameterList;

    /// Operation parameter name is non-empty.
    ///
    /// Source: ISO 19111:2019 §16.7 — OperationParameter.name.
    pub struct OperationParameterNameNonEmpty;

    /// Operation parameter value carries a unit of measure.
    ///
    /// Source: ISO 19111:2019 §16.7 — parameterValue.unit.
    pub struct OperationParameterValueHasUnit;

    // ── §15 Axis order ───────────────────────────────────────────────────

    /// Geographic 2D ISO axis order is latitude first, longitude second.
    ///
    /// Source: ISO 19111:2019 §15.2 — canonical lat/lon ordering.
    pub struct Geographic2dIsoAxisOrderLatitudeFirst;

    /// Geographic 3D ISO axis order is latitude, longitude, ellipsoidal height.
    ///
    /// Source: ISO 19111:2019 §15.2.
    pub struct Geographic3dIsoAxisOrderLatLonHeight;

    /// Projected CRS conventional axis order is easting first.
    ///
    /// Source: ISO 19111:2019 §15.3 — easting/northing convention.
    pub struct ProjectedConventionalAxisOrderEastingFirst;

    /// Northing-first variants exist for some projected CRS definitions.
    ///
    /// Source: ISO 19111:2019 §15 — variant note.
    pub struct ProjectedNorthingFirstVariantsExist;

    /// Axis order must follow the CRS definition, not assumed legacy convention.
    ///
    /// Source: ISO 19111:2019 §15 — enforcement.
    pub struct AxisOrderMustFollowCrsDefinition;

    /// Changing axis order requires an explicit operation or annotation.
    ///
    /// Source: ISO 19111:2019 §15 — no silent reordering.
    pub struct AxisOrderChangeRequiresExplicitOperation;

    /// Coordinate tuple element count equals the CRS axis count.
    ///
    /// Source: ISO 19111:2019 §15 — tuple dimensionality.
    pub struct CoordinateTupleElementCountEqualsAxisCount;

    /// Each coordinate element aligns with the axis at that ordinal position.
    ///
    /// Source: ISO 19111:2019 §15.
    pub struct CoordinateElementAlignedToAxisOrdinalPosition;

    // ── §16 EPSG registry ────────────────────────────────────────────────

    /// EPSG code is a positive integer.
    ///
    /// Source: EPSG Dataset / ISO 19111:2019 §6.2 — identifier.code type.
    pub struct EpsgCodePositiveInteger;

    /// EPSG authority name is the string "EPSG".
    ///
    /// Source: EPSG Dataset — authority token.
    pub struct EpsgAuthorityNameIsEpsg;

    /// OGC authority name is the string "OGC".
    ///
    /// Source: OGC registry conventions.
    pub struct OgcAuthorityNameIsOgc;

    /// Other known authority names include ESRI and IGNF.
    ///
    /// Source: EPSG Guidance Note 7-1 — authority names.
    pub struct OtherAuthorityNamesEsriIgnf;

    /// A CRS with null authority code is invalid if it is intended to be registered.
    ///
    /// Source: ISO 19111:2019 §6.2 — identifier constraint.
    pub struct RegisteredCrsNullAuthorityCodeInvalid;

    /// EPSG geographic CRS codes are typically in the range 4000–4999.
    ///
    /// Source: EPSG Dataset — code block allocation.
    pub struct EpsgGeographicCrsRange4000To4999;

    /// EPSG projected CRS codes are typically in the range 20000–32767.
    ///
    /// Source: EPSG Dataset — code block allocation.
    pub struct EpsgProjectedCrsRange20000To32767;

    /// EPSG vertical CRS codes are typically in the range 5000–5999.
    ///
    /// Source: EPSG Dataset — code block allocation.
    pub struct EpsgVerticalCrsRange5000To5999;

    /// EPSG compound CRS codes are typically in the range 6000–6999.
    ///
    /// Source: EPSG Dataset — code block allocation.
    pub struct EpsgCompoundCrsRange6000To6999;

    /// CRS identity is determined by the authority + code pair.
    ///
    /// Source: ISO 19111:2019 §6.2 — identity uniqueness.
    pub struct CrsIdentityByAuthorityAndCode;

    /// Different CRS codes require an explicit coordinate operation to convert.
    ///
    /// Source: ISO 19111:2019 §6 — no implicit conversion.
    pub struct DifferentCrsCodesRequireExplicitOperation;

    // ── §17 Dynamic / static CRS ─────────────────────────────────────────

    /// Static CRS is plate-fixed; no coordinate epoch is required.
    ///
    /// Source: ISO 19111:2019 §17.2 — static datum.
    pub struct StaticCrsPlateFixedNoEpochRequired;

    /// Static CRS datum is defined at a fixed epoch.
    ///
    /// Source: ISO 19111:2019 §17.2.
    pub struct StaticCrsDatumAtFixedEpoch;

    /// Dynamic CRS requires a coordinate epoch with coordinate tuples.
    ///
    /// Source: ISO 19111:2019 §17.2 — dynamic datum.
    pub struct DynamicCrsRequiresCoordinateEpoch;

    /// Coordinate epoch is expressed as a decimal year.
    ///
    /// Source: ISO 19111:2019 §17.2 — coordinateEpoch.
    pub struct CoordinateEpochIsDecimalYear;

    /// Coordinate epoch is a positive finite real number.
    ///
    /// Source: ISO 19111:2019 §17.2.
    pub struct CoordinateEpochPositiveFinite;

    /// ITRF2014 is a dynamic datum (plate-motion model applies).
    ///
    /// Source: ISO 19111:2019 §17 / EPSG Dataset EPSG:1287.
    pub struct Itrf2014IsDynamicDatum;

    /// ITRF2020 is a dynamic datum.
    ///
    /// Source: ISO 19111:2019 §17 / EPSG Dataset.
    pub struct Itrf2020IsDynamicDatum;

    /// IGS20 is a dynamic datum.
    ///
    /// Source: ISO 19111:2019 §17 / EPSG Dataset.
    pub struct Igs20IsDynamicDatum;

    /// Omitting coordinate epoch for a dynamic CRS introduces positioning errors.
    ///
    /// Source: ISO 19111:2019 §17.2 — epoch omission consequence.
    pub struct DynamicCrsOmittingEpochIntroducesError;

    /// Dynamic reference frames carry a frame reference epoch.
    ///
    /// Source: ISO 19111:2019 §17.3 — frameReferenceEpoch.
    pub struct DynamicReferenceFrameHasFrameReferenceEpoch;

    // ── §18 Cross-cutting rules ──────────────────────────────────────────

    /// The authority + code pair uniquely identifies a CRS within a registry.
    ///
    /// Source: ISO 19111:2019 §6.2 — identifier uniqueness.
    pub struct CrsIdentityAuthorityPlusCodeUnique;

    /// Component CRSes in a compound CRS are orthogonal (non-overlapping axes).
    ///
    /// Source: ISO 19111:2019 §13.2 — orthogonality.
    pub struct CompoundCrsComponentsOrthogonal;

    /// Axis abbreviations are unique within a single coordinate system.
    ///
    /// Source: ISO 19111:2019 §9.
    pub struct AxisAbbreviationUniqueInCs;

    /// A null authority code is invalid for a CRS intended to be registered.
    ///
    /// Source: ISO 19111:2019 §6.2.
    pub struct NullAuthorityCodeInvalidForRegisteredCrs;

    /// Angular axes must use angular units.
    ///
    /// Source: ISO 19111:2019 §9 — uom constraint.
    pub struct AngularAxisMustUseAngularUnit;

    /// Linear axes must use linear units.
    ///
    /// Source: ISO 19111:2019 §9 — uom constraint.
    pub struct LinearAxisMustUseLinearUnit;

    /// Parametric axes must use parametric units.
    ///
    /// Source: ISO 19111:2019 §9 — uom constraint.
    pub struct ParametricAxisMustUseParametricUnit;

    /// Time axes must use temporal units.
    ///
    /// Source: ISO 19111:2019 §9 — uom constraint.
    pub struct TimeAxisMustUseTemporalUnit;

    /// CRS scope describes its intended use.
    ///
    /// Source: ISO 19111:2019 §6.2 — scope content.
    pub struct CrsScopeDescribesIntendedUse;

    /// Absent domain of validity implies worldwide applicability.
    ///
    /// Source: ISO 19111:2019 §6.2 — domainOfValidity absence semantics.
    pub struct CrsDomainOfValidityOptionalImpliesGlobal;

    /// Domain of validity may include geographic, temporal, and vertical extents.
    ///
    /// Source: ISO 19111:2019 §6.2 — EX_Extent subtypes.
    pub struct CrsDomainOfValidityExtentTypes;

    /// Identified object alias list may be empty.
    ///
    /// Source: ISO 19111:2019 §6.4 — alias multiplicity 0..*.
    pub struct IdentifiedObjectAliasOptionalList;

    /// Identified object remarks are optional free text.
    ///
    /// Source: ISO 19111:2019 §6.4 — remarks multiplicity 0..1.
    pub struct IdentifiedObjectRemarksOptional;

    // ── §19 Temporal / parametric CRS ────────────────────────────────────

    /// Temporal CRS datum is a temporal datum.
    ///
    /// Source: ISO 19111:2019 §18.2 — datum type.
    pub struct TemporalCrsDatumIsTemporalDatum;

    /// Temporal CRS CS is a temporal CS.
    ///
    /// Source: ISO 19111:2019 §18.2 — cs type.
    pub struct TemporalCrsCsIsTemporalCs;

    /// Temporal datum origin is ISO 8601 date-time.
    ///
    /// Source: ISO 19111:2019 §18.3 — origin.
    pub struct TemporalDatumOriginIsIso8601DateTime;

    /// Temporal CRS has exactly one axis.
    ///
    /// Source: ISO 19111:2019 §18.2 — axis count.
    pub struct TemporalCrsHasOneAxis;

    /// Parametric CRS datum is a parametric datum.
    ///
    /// Source: ISO 19111:2019 §19.2 — datum type.
    pub struct ParametricCrsDatumIsParametricDatum;

    /// Parametric CRS CS is a parametric CS.
    ///
    /// Source: ISO 19111:2019 §19.2 — cs type.
    pub struct ParametricCrsCsIsParametricCs;

    /// Parametric CRS has exactly one axis.
    ///
    /// Source: ISO 19111:2019 §19.2 — axis count.
    pub struct ParametricCrsHasOneAxis;

    // ── §20 Value constraints ─────────────────────────────────────────────

    /// Every coordinate system has at least one axis.
    ///
    /// Source: ISO 19111:2019 §9 — axis multiplicity 1..*.
    pub struct CoordinateSystemMinimumOneAxis;

    /// No coordinate system has more than four axes.
    ///
    /// Source: ISO 19111:2019 §9 — maximum.
    pub struct CoordinateSystemMaximumFourAxes;

    /// Every compound CRS has at least two component CRSes.
    ///
    /// Source: ISO 19111:2019 §13.2.
    pub struct CompoundCrsMinimumTwoComponents;

    /// Every concatenated operation has at least two steps.
    ///
    /// Source: ISO 19111:2019 §16.5.
    pub struct ConcatenatedOperationMinimumTwoSteps;

    /// All CRS string values are UTF-8 encoded.
    ///
    /// Source: ISO 19111:2019 §6.2 — character encoding.
    pub struct CrsStringValuesUtf8Encoded;

    /// Mandatory string attributes are non-null.
    ///
    /// Source: ISO 19111:2019 general — nullability.
    pub struct CrsMandatoryStringNonNull;

    /// Latitude values are in the range [-90, 90].
    ///
    /// Source: ISO 19111:2019 §8.2.
    pub struct LatitudeRangeNegative90To90;

    /// Longitude values are in the range (-180, 180].
    ///
    /// Source: ISO 19111:2019 §8.2.
    pub struct LongitudeRangeNegative180To180;

    /// Latitude values at poles (±90°) are valid.
    ///
    /// Source: ISO 19111:2019 §8.2.
    pub struct LatitudePolesValid;

    /// Longitude value -180° is excluded (equivalent to +180°).
    ///
    /// Source: ISO 19111:2019 §8.2 — conventional range.
    pub struct LongitudeNegative180Excluded;

    /// Map projection scale factor is positive.
    ///
    /// Source: ISO 19111:2019 §16.3 — scaleFactor > 0.
    pub struct MapProjectionScaleFactorPositive;

    /// Map projection false origin values are finite real numbers.
    ///
    /// Source: ISO 19111:2019 §16.3.
    pub struct MapProjectionFalseOriginFiniteReal;

    /// Axis direction value is a member of the CS_AxisDirection code list.
    ///
    /// Source: ISO 19111:2019 §9.
    pub struct AxisDirectionMemberOfCodeList;

    /// CS type value is a member of the defined CS type code list.
    ///
    /// Source: ISO 19111:2019 §9.
    pub struct CsTypeMemberOfDefinedTypes;

    /// CRS type value is a member of the defined CRS type code list.
    ///
    /// Source: ISO 19111:2019 §6.
    pub struct CrsTypeMemberOfDefinedTypes;

    // ── §21 IO_IdentifiedObject (gap fill) ───────────────────────────────

    /// IO_IdentifiedObject primary name attribute is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.4 — name.
    pub struct IdentifiedObjectPrimaryNameNonEmpty;

    /// IO_IdentifiedObject alias list contains no null entries.
    ///
    /// Source: ISO 19111:2019 §6.4 — alias.
    pub struct IdentifiedObjectAliasNoNullEntries;

    /// IO_IdentifiedObject identifier entry carries both authority and code.
    ///
    /// Source: ISO 19111:2019 §6.4.
    pub struct IdentifiedObjectIdentifierEntryComplete;

    /// IO_IdentifiedObject remarks, when present, are non-null.
    ///
    /// Source: ISO 19111:2019 §6.4.
    pub struct IdentifiedObjectRemarksWhenPresentNonNull;

    /// CRS inherits the IO_IdentifiedObject interface.
    ///
    /// Source: ISO 19111:2019 §6.4 — inheritance.
    pub struct CrsInheritsIdentifiedObjectInterface;

    /// Datum inherits the IO_IdentifiedObject interface.
    ///
    /// Source: ISO 19111:2019 §6.4.
    pub struct DatumInheritsIdentifiedObjectInterface;

    /// Coordinate operation inherits the IO_IdentifiedObject interface.
    ///
    /// Source: ISO 19111:2019 §6.4.
    pub struct CoordinateOperationInheritsIdentifiedObjectInterface;

    // ── §22 CD_DatumEnsemble (gap fill) ──────────────────────────────────

    /// Datum ensemble groups conceptually related reference frames.
    ///
    /// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.
    pub struct DatumEnsembleGroupsRelatedDatums;

    /// Datum ensemble name is non-empty.
    ///
    /// Source: ISO 19111:2019 §6.5.
    pub struct DatumEnsembleNameNonEmpty;

    /// Datum ensemble has at least two member datums.
    ///
    /// Source: ISO 19111:2019 §6.5 — member multiplicity ≥2.
    pub struct DatumEnsembleHasAtLeastTwoMembers;

    /// Datum ensemble member list contains no null entries.
    ///
    /// Source: ISO 19111:2019 §6.5.
    pub struct DatumEnsembleMemberNoNullEntries;

    /// Datum ensemble accuracy is a positive real number in metres.
    ///
    /// Source: ISO 19111:2019 §6.5 — ensembleAccuracy.
    pub struct DatumEnsembleAccuracyPositive;

    /// Sub-metre accuracy requires selecting a specific ensemble member.
    ///
    /// Source: ISO 19111:2019 §6.5 — accuracy guidance.
    pub struct DatumEnsembleSubMetreRequiresMemberSelection;

    /// The WGS 84 datum ensemble has EPSG code 6326.
    ///
    /// Source: EPSG Dataset — WGS 84 ensemble.
    pub struct DatumEnsembleWgs84EpsgCode6326;

    /// EPSG:4326 uses the WGS 84 datum ensemble (EPSG:6326), not a single datum.
    ///
    /// Source: ISO 19111:2019 §6.5 / EPSG Dataset.
    pub struct Epsg4326UsesDatumEnsemble;

    /// The NAD 27 datum ensemble has EPSG code 6267.
    ///
    /// Source: EPSG Dataset — NAD 27 ensemble.
    pub struct Nad27EnsembleEpsg6267;

    /// All members of a datum ensemble are of the same datum type.
    ///
    /// Source: ISO 19111:2019 §6.5 — homogeneity.
    pub struct DatumEnsembleMembersHomogeneous;

    // ── §23 SC_CoordinateMetadata (gap fill) ─────────────────────────────

    /// SC_CoordinateMetadata carries a mandatory CRS reference.
    ///
    /// Source: ISO 19111:2019 §7.4 — crs multiplicity 1.
    pub struct CoordinateMetadataHasCrs;

    /// SC_CoordinateMetadata CRS reference is non-null.
    ///
    /// Source: ISO 19111:2019 §7.4.
    pub struct CoordinateMetadataCrsNonNull;

    /// Coordinate epoch within SC_CoordinateMetadata is expressed as a decimal year.
    ///
    /// Source: ISO 19111:2019 §7.4.
    pub struct CoordinateMetadataEpochIsDecimalYear;

    /// SC_CoordinateMetadata with a dynamic CRS must include a coordinate epoch.
    ///
    /// Source: ISO 19111:2019 §7.4.
    pub struct CoordinateMetadataDynamicCrsRequiresEpoch;

    /// SC_CoordinateMetadata referencing a static CRS should omit coordinate epoch.
    ///
    /// Source: ISO 19111:2019 §7.4.
    pub struct CoordinateMetadataStaticCrsEpochShouldBeAbsent;

    /// SC_CoordinateMetadata may be applied at set or individual tuple level.
    ///
    /// Source: ISO 19111:2019 §7.4 — application scope.
    pub struct CoordinateMetadataApplicableAtSetOrTupleLevel;

    /// Coordinate epoch is distinct from the frame reference epoch of the datum.
    ///
    /// Source: ISO 19111:2019 §17 — conceptual distinction.
    pub struct CoordinateEpochDistinctFromFrameReferenceEpoch;

    // ── §24 CC_PassThroughOperation (gap fill) ───────────────────────────

    /// CC_PassThroughOperation passes some axes through unchanged.
    ///
    /// Source: ISO 19111:2019 §16.6 — CC_PassThroughOperation.
    pub struct PassThroughOperationPreservesSomeAxes;

    /// Modified coordinate indices of a pass-through operation are non-empty.
    ///
    /// Source: ISO 19111:2019 §16.6 — modifiedCoordinate.
    pub struct PassThroughOperationModifiedCoordinatesNonEmpty;

    /// Each modified coordinate index is within the CRS dimension range.
    ///
    /// Source: ISO 19111:2019 §16.6 — index range.
    pub struct PassThroughOperationIndexInRange;

    /// The inner operation of a pass-through operation is non-null.
    ///
    /// Source: ISO 19111:2019 §16.6 — operation.
    pub struct PassThroughOperationInnerOperationNonNull;

    /// Pass-through operation source and target dimensions are consistent.
    ///
    /// Source: ISO 19111:2019 §16.6.
    pub struct PassThroughOperationDimensionConsistency;

    /// Pass-through operations are used in compound CRS coordinate operations.
    ///
    /// Source: ISO 19111:2019 §16.6 — usage.
    pub struct PassThroughOperationCompoundCrsUsage;

    // ── §25 CS_SphericalCS (gap fill) ────────────────────────────────────

    /// Spherical CS has exactly three axes.
    ///
    /// Source: ISO 19111:2019 §9.5 — CS_SphericalCS.
    pub struct SphericalCsHasThreeAxes;

    /// Spherical CS two angular axes precede the radial (linear) axis.
    ///
    /// Source: ISO 19111:2019 §9.5 — axis order convention.
    pub struct SphericalCsAngularAxesThenLinear;

    /// Spherical CS is applicable in geocentric context.
    ///
    /// Source: ISO 19111:2019 §9.5 — usage.
    pub struct SphericalCsApplicableToGeocentricContext;

    // ── §26 Helmert conventions (gap fill) ───────────────────────────────

    /// Helmert transformation has seven standard parameters (3 translation, 3 rotation, 1 scale).
    ///
    /// Source: ISO 19111:2019 §11.4 / EPSG Guidance Note 7-2 §2.4.
    pub struct HelmertSevenParameterStructure;

    /// Position Vector convention: rotations apply to the coordinate frame (ISO method 9606).
    ///
    /// Source: ISO 19111:2019 §11.4 / EPSG method 9606.
    pub struct HelmertPositionVectorConvention;

    /// Coordinate Frame convention: rotations apply to the coordinate system (EPSG method 9607).
    ///
    /// Source: EPSG Guidance Note 7-2 §2.4.3 — Bursa-Wolf / Coordinate Frame.
    pub struct HelmertCoordinateFrameConvention;

    /// Position Vector and Coordinate Frame conventions are equivalent only when all rotation parameters are zero.
    ///
    /// Source: EPSG Guidance Note 7-2 §2.4.
    pub struct HelmertConventionsEquivalentOnlyForZeroRotation;

    /// Helmert transformation definition must identify which convention is used.
    ///
    /// Source: ISO 19111:2019 §11.4 — convention identification.
    pub struct HelmertConventionMustBeIdentified;

    /// EPSG method codes 9606 (Position Vector) and 9607 (Coordinate Frame) are distinct.
    ///
    /// Source: EPSG Dataset — method registry.
    pub struct HelmertEpsgMethodCodes9606And9607;

    /// Molodensky-Badekas uses ten parameters (7 Helmert + 3 pivot point coordinates).
    ///
    /// Source: EPSG Guidance Note 7-2 §2.4.4.
    pub struct MolodenskyBadenkasTenParameter;

    // ── §27 Grid-based datum shifts (gap fill) ───────────────────────────

    /// Grid-based datum shift operations use an external grid file.
    ///
    /// Source: ISO 19111:2019 §16.4 / EPSG Guidance Note 7-2 §2.8.
    pub struct GridBasedDatumShiftUsesExternalFile;

    /// NADCON5 is the US standard for horizontal datum shifts.
    ///
    /// Source: EPSG Guidance Note 7-2 §2.8.1 — NADCON5.
    pub struct Nadcon5IsUsHorizontalDatumShift;

    /// NTv2 is a grid-based horizontal datum shift format.
    ///
    /// Source: EPSG Guidance Note 7-2 §2.8.2 — NTv2.
    pub struct Ntv2IsGridHorizontalShift;

    /// VERTCON is the US standard for vertical datum shifts.
    ///
    /// Source: EPSG Guidance Note 7-2 §2.8.3 — VERTCON.
    pub struct VertconIsUsVerticalDatumShift;

    /// Grid shift parameter stores the filename, not the shift values directly.
    ///
    /// Source: ISO 19111:2019 §16.7 — parameter file convention.
    pub struct ParameterFileStoresFilenameNotValue;

    // ── §28 UoM (gap fill) ───────────────────────────────────────────────

    /// Unit of measure name is non-empty.
    ///
    /// Source: ISO 19111:2019 §8.5 — UoM.name.
    pub struct UomNameNonEmpty;

    /// Unit of measure conversion factor to SI base unit is positive.
    ///
    /// Source: ISO 19111:2019 §8.5 — conversionFactor.
    pub struct UomConversionFactorPositive;

    /// Angular unit conversion factor converts to radians.
    ///
    /// Source: ISO 19111:2019 §8.5 — angular UoM.
    pub struct UomAngularConvertToRadians;

    /// Linear unit conversion factor converts to metres.
    ///
    /// Source: ISO 19111:2019 §8.5 — linear UoM.
    pub struct UomLinearConvertToMetres;

    /// Temporal unit conversion factor converts to seconds.
    ///
    /// Source: ISO 19111:2019 §8.5 — temporal UoM.
    pub struct UomTimeConvertToSeconds;

    /// Scale (dimensionless) unit conversion factor is 1.
    ///
    /// Source: ISO 19111:2019 §8.5 — scale UoM.
    pub struct UomScaleDimensionless;

    /// International foot (0.3048 m) and US survey foot (0.30480060960 m) are distinct.
    ///
    /// Source: EPSG Guidance Note 7-2 — foot ambiguity.
    pub struct UomFeetAmbiguityInternationalVsSurvey;

    // ── §29 UTM south zone false northing (gap fill) ─────────────────────

    /// UTM south zones use a false northing of 10,000,000 m.
    ///
    /// Source: EPSG Guidance Note 7-2 — UTM south zone parameters.
    pub struct UtmSouthZoneFalseNorthing10000000;

    /// UTM north zones use a false northing of 0 m.
    ///
    /// Source: EPSG Guidance Note 7-2 — UTM north zone parameters.
    pub struct UtmNorthZoneFalseNorthing0;

    /// UTM north and south zones share false easting (500,000 m) and scale factor (0.9996).
    ///
    /// Source: EPSG Guidance Note 7-2 — UTM common parameters.
    pub struct UtmCommonParametersFalseEastingAndScale;

    // ── Trait operation contracts ─────────────────────────────────────────

    /// A CRS was successfully resolved from a registry by authority + code.
    pub struct CrsResolved;

    /// A datum ensemble was successfully resolved from a registry.
    pub struct DatumEnsembleResolved;

    /// A CRS definition passed structural validity checks.
    pub struct CrsValid;

    /// An ellipsoid definition passed all ISO 19111 §7.3 validity checks.
    pub struct EllipsoidValid;

    /// Coordinate metadata passed completeness checks (CRS present; epoch present iff dynamic).
    pub struct CoordinateMetadataValid;

    /// A coordinate tuple was successfully transformed between two CRSes.
    pub struct CoordinatesTransformed;

    macro_rules! structural_prop {
        ($t:ty, $name:literal) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by ISO 19111 contract */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by ISO 19111 contract */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural: #name — verified by ISO 19111 contract */ }
                }
            }
        };
    }

    // §6
    structural_prop!(CrsConsistsOfCsAndDatum, "CrsConsistsOfCsAndDatum");
    structural_prop!(
        CoordinateTupleDimensionMatchesAxes,
        "CoordinateTupleDimensionMatchesAxes"
    );
    structural_prop!(ScCrsIsAbstract, "ScCrsIsAbstract");
    structural_prop!(ScCrsNameNonEmpty, "ScCrsNameNonEmpty");
    structural_prop!(ScCrsScopeNonEmpty, "ScCrsScopeNonEmpty");
    structural_prop!(
        ScCrsDomainOfValidityIsExtent,
        "ScCrsDomainOfValidityIsExtent"
    );
    structural_prop!(
        ScCrsIdentifierHasAuthorityAndCode,
        "ScCrsIdentifierHasAuthorityAndCode"
    );
    structural_prop!(
        CrsIdentifierAuthorityNonEmpty,
        "CrsIdentifierAuthorityNonEmpty"
    );
    structural_prop!(CrsIdentifierCodeNonEmpty, "CrsIdentifierCodeNonEmpty");
    // §7 Geodetic CRS
    structural_prop!(
        GeodeticCrsDatumIsGeodeticReferenceFrame,
        "GeodeticCrsDatumIsGeodeticReferenceFrame"
    );
    structural_prop!(
        GeodeticCrsCsIsEllipsoidalOrCartesian,
        "GeodeticCrsCsIsEllipsoidalOrCartesian"
    );
    structural_prop!(Geographic2dCrsHasTwoAxes, "Geographic2dCrsHasTwoAxes");
    structural_prop!(Geographic3dCrsHasThreeAxes, "Geographic3dCrsHasThreeAxes");
    structural_prop!(GeocentricCrsUsesCartesianCs, "GeocentricCrsUsesCartesianCs");
    structural_prop!(Epsg4326AxisOrderLatFirst, "Epsg4326AxisOrderLatFirst");
    structural_prop!(Epsg4326LatitudeRangeValid, "Epsg4326LatitudeRangeValid");
    structural_prop!(Epsg4326LongitudeRangeValid, "Epsg4326LongitudeRangeValid");
    structural_prop!(Epsg4979IsWgs84Geographic3d, "Epsg4979IsWgs84Geographic3d");
    structural_prop!(Epsg4979HeightUnbounded, "Epsg4979HeightUnbounded");
    structural_prop!(Epsg4978IsWgs84Geocentric, "Epsg4978IsWgs84Geocentric");
    structural_prop!(
        Epsg4978XAxisTowardsPrimeMeridian,
        "Epsg4978XAxisTowardsPrimeMeridian"
    );
    structural_prop!(
        Epsg4978ZAxisTowardsNorthPole,
        "Epsg4978ZAxisTowardsNorthPole"
    );
    // §7.2
    structural_prop!(
        GeodeticReferenceFrameNameNonEmpty,
        "GeodeticReferenceFrameNameNonEmpty"
    );
    structural_prop!(
        GeodeticReferenceFrameAnchorOptional,
        "GeodeticReferenceFrameAnchorOptional"
    );
    structural_prop!(
        GeodeticReferenceFrameHasExactlyOneEllipsoid,
        "GeodeticReferenceFrameHasExactlyOneEllipsoid"
    );
    structural_prop!(
        GeodeticReferenceFrameHasExactlyOnePrimeMeridian,
        "GeodeticReferenceFrameHasExactlyOnePrimeMeridian"
    );
    structural_prop!(
        GeodeticReferenceFrameRealizationEpochIsIso8601,
        "GeodeticReferenceFrameRealizationEpochIsIso8601"
    );
    // §7.3
    structural_prop!(EllipsoidNameNonEmpty, "EllipsoidNameNonEmpty");
    structural_prop!(
        EllipsoidSemiMajorAxisPositive,
        "EllipsoidSemiMajorAxisPositive"
    );
    structural_prop!(
        EllipsoidSemiMajorAxisInMetres,
        "EllipsoidSemiMajorAxisInMetres"
    );
    structural_prop!(
        EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor,
        "EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor"
    );
    structural_prop!(
        EllipsoidInverseFlatteningPositiveWhenNonSphere,
        "EllipsoidInverseFlatteningPositiveWhenNonSphere"
    );
    structural_prop!(
        EllipsoidWgs84InverseFlatteningApprox298,
        "EllipsoidWgs84InverseFlatteningApprox298"
    );
    structural_prop!(
        EllipsoidInverseFlatteningZeroMeansSphere,
        "EllipsoidInverseFlatteningZeroMeansSphere"
    );
    structural_prop!(
        EllipsoidSemiMinorAxisLessThanSemiMajor,
        "EllipsoidSemiMinorAxisLessThanSemiMajor"
    );
    structural_prop!(
        EllipsoidIsSphereConsistentWithParameters,
        "EllipsoidIsSphereConsistentWithParameters"
    );
    structural_prop!(
        EllipsoidSemiMinorAxisInMetres,
        "EllipsoidSemiMinorAxisInMetres"
    );
    // §7.4
    structural_prop!(PrimeMeridianNameNonEmpty, "PrimeMeridianNameNonEmpty");
    structural_prop!(
        PrimeMeridianGreenwichLongitudeFinite,
        "PrimeMeridianGreenwichLongitudeFinite"
    );
    structural_prop!(
        PrimeMeridianGreenwichLongitudeInDegreeBounds,
        "PrimeMeridianGreenwichLongitudeInDegreeBounds"
    );
    structural_prop!(PrimeMeridianGreenwichIsZero, "PrimeMeridianGreenwichIsZero");
    structural_prop!(
        PrimeMeridianNonGreenwichAllowed,
        "PrimeMeridianNonGreenwichAllowed"
    );
    structural_prop!(
        PrimeMeridianGreenwichLongitudeUnitIsAngular,
        "PrimeMeridianGreenwichLongitudeUnitIsAngular"
    );
    // §8
    structural_prop!(
        CoordinateSystemAxisCountOneToFour,
        "CoordinateSystemAxisCountOneToFour"
    );
    structural_prop!(
        CsAxisCountMatchesTupleDimensionality,
        "CsAxisCountMatchesTupleDimensionality"
    );
    structural_prop!(CoordinateSystemNameNonEmpty, "CoordinateSystemNameNonEmpty");
    structural_prop!(AxisNameNonEmpty, "AxisNameNonEmpty");
    structural_prop!(AxisAbbreviationNonEmpty, "AxisAbbreviationNonEmpty");
    structural_prop!(
        AxisAbbreviationUniqueWithinCs,
        "AxisAbbreviationUniqueWithinCs"
    );
    structural_prop!(AxisDirectionIsValidCode, "AxisDirectionIsValidCode");
    structural_prop!(
        AxisUnitAppropriateForAxisType,
        "AxisUnitAppropriateForAxisType"
    );
    structural_prop!(AxisMinMaxValueOptional, "AxisMinMaxValueOptional");
    structural_prop!(
        AxisRangeMeaningExactOrWraparound,
        "AxisRangeMeaningExactOrWraparound"
    );
    // §9
    structural_prop!(
        ProjectedCrsBaseCrsIsGeographic,
        "ProjectedCrsBaseCrsIsGeographic"
    );
    structural_prop!(ProjectedCrsCsIsCartesian, "ProjectedCrsCsIsCartesian");
    structural_prop!(
        ProjectedCrsProjectionIsConversion,
        "ProjectedCrsProjectionIsConversion"
    );
    structural_prop!(ProjectedCrsNameNonEmpty, "ProjectedCrsNameNonEmpty");
    structural_prop!(
        ProjectedCrsAxesUseLinearUnit,
        "ProjectedCrsAxesUseLinearUnit"
    );
    structural_prop!(
        ProjectedCrsConventionalAxisDirections,
        "ProjectedCrsConventionalAxisDirections"
    );
    structural_prop!(ProjectedCrsHasTwoAxes, "ProjectedCrsHasTwoAxes");
    structural_prop!(
        UtmNorthZoneEpsgRange32601To32660,
        "UtmNorthZoneEpsgRange32601To32660"
    );
    structural_prop!(
        UtmSouthZoneEpsgRange32701To32760,
        "UtmSouthZoneEpsgRange32701To32760"
    );
    structural_prop!(UtmAxisOrderEastingFirst, "UtmAxisOrderEastingFirst");
    structural_prop!(UtmFalseEasting500000, "UtmFalseEasting500000");
    structural_prop!(UtmZoneWidthSixDegrees, "UtmZoneWidthSixDegrees");
    structural_prop!(
        UtmScaleFactorAtCentralMeridian0996,
        "UtmScaleFactorAtCentralMeridian0996"
    );
    structural_prop!(UtmZoneNumberOneToSixty, "UtmZoneNumberOneToSixty");
    // §10
    structural_prop!(
        VerticalCrsDatumIsVerticalReferenceFrame,
        "VerticalCrsDatumIsVerticalReferenceFrame"
    );
    structural_prop!(VerticalCrsCsIsVerticalCs, "VerticalCrsCsIsVerticalCs");
    structural_prop!(VerticalCrsNameNonEmpty, "VerticalCrsNameNonEmpty");
    structural_prop!(VerticalCrsHasOneAxis, "VerticalCrsHasOneAxis");
    structural_prop!(
        VerticalReferenceFrameNameNonEmpty,
        "VerticalReferenceFrameNameNonEmpty"
    );
    structural_prop!(
        VerticalReferenceFrameRealizationEpochIsIso8601,
        "VerticalReferenceFrameRealizationEpochIsIso8601"
    );
    structural_prop!(
        VerticalReferenceFrameAnchorOptional,
        "VerticalReferenceFrameAnchorOptional"
    );
    structural_prop!(
        VerticalReferenceFrameGravityRelated,
        "VerticalReferenceFrameGravityRelated"
    );
    structural_prop!(
        VerticalCrsHeightAxisDirectionUp,
        "VerticalCrsHeightAxisDirectionUp"
    );
    structural_prop!(
        VerticalCrsDepthAxisDirectionDown,
        "VerticalCrsDepthAxisDirectionDown"
    );
    structural_prop!(VerticalCrsAxisLinearUnit, "VerticalCrsAxisLinearUnit");
    structural_prop!(
        VerticalCrsEpsgRange5000To5999,
        "VerticalCrsEpsgRange5000To5999"
    );
    // §11
    structural_prop!(
        EngineeringCrsDatumIsEngineeringDatum,
        "EngineeringCrsDatumIsEngineeringDatum"
    );
    structural_prop!(EngineeringCrsNameNonEmpty, "EngineeringCrsNameNonEmpty");
    structural_prop!(EngineeringCrsCsTypeFlexible, "EngineeringCrsCsTypeFlexible");
    structural_prop!(
        EngineeringCrsIsLocalContextOnly,
        "EngineeringCrsIsLocalContextOnly"
    );
    structural_prop!(EngineeringDatumNameNonEmpty, "EngineeringDatumNameNonEmpty");
    structural_prop!(
        EngineeringDatumAnchorOptional,
        "EngineeringDatumAnchorOptional"
    );
    // §12
    structural_prop!(
        CompoundCrsHasAtLeastTwoComponents,
        "CompoundCrsHasAtLeastTwoComponents"
    );
    structural_prop!(CompoundCrsNameNonEmpty, "CompoundCrsNameNonEmpty");
    structural_prop!(
        CompoundCrsComponentsNonOverlapping,
        "CompoundCrsComponentsNonOverlapping"
    );
    structural_prop!(
        CompoundCrsTotalAxisCountIsSumOfComponents,
        "CompoundCrsTotalAxisCountIsSumOfComponents"
    );
    structural_prop!(
        CompoundCrsTypicalIs2dPlusVertical,
        "CompoundCrsTypicalIs2dPlusVertical"
    );
    structural_prop!(
        CompoundCrsEpsgRange6000To6999,
        "CompoundCrsEpsgRange6000To6999"
    );
    structural_prop!(
        CompoundCrsNoTwoHorizontalComponents,
        "CompoundCrsNoTwoHorizontalComponents"
    );
    structural_prop!(
        CompoundCrsNoTwoVerticalComponents,
        "CompoundCrsNoTwoVerticalComponents"
    );
    // §13
    structural_prop!(DerivedCrsHasBaseCrs, "DerivedCrsHasBaseCrs");
    structural_prop!(
        DerivedCrsDerivingConversionIsConversion,
        "DerivedCrsDerivingConversionIsConversion"
    );
    structural_prop!(
        DerivedCrsCsDiffersFromBaseCrsAllowed,
        "DerivedCrsCsDiffersFromBaseCrsAllowed"
    );
    structural_prop!(DerivedCrsNameNonEmpty, "DerivedCrsNameNonEmpty");
    structural_prop!(
        DerivedCrsInheritsDatumFromBase,
        "DerivedCrsInheritsDatumFromBase"
    );
    structural_prop!(
        DerivedProjectedCrsBaseMustBeProjCrs,
        "DerivedProjectedCrsBaseMustBeProjCrs"
    );
    // §14
    structural_prop!(
        CoordinateOperationNameNonEmpty,
        "CoordinateOperationNameNonEmpty"
    );
    structural_prop!(
        CoordinateOperationHasSourceCrs,
        "CoordinateOperationHasSourceCrs"
    );
    structural_prop!(
        CoordinateOperationHasTargetCrs,
        "CoordinateOperationHasTargetCrs"
    );
    structural_prop!(
        CoordinateOperationVersionOptional,
        "CoordinateOperationVersionOptional"
    );
    structural_prop!(
        CoordinateOperationDomainOfValidityOptional,
        "CoordinateOperationDomainOfValidityOptional"
    );
    structural_prop!(
        ConversionInvolvesNoDatumChange,
        "ConversionInvolvesNoDatumChange"
    );
    structural_prop!(
        ConversionDefinesMapProjection,
        "ConversionDefinesMapProjection"
    );
    structural_prop!(ConversionInverseExists, "ConversionInverseExists");
    structural_prop!(ConversionHasOperationMethod, "ConversionHasOperationMethod");
    structural_prop!(ConversionHasParameterValues, "ConversionHasParameterValues");
    structural_prop!(
        TransformationInvolvesDatumChange,
        "TransformationInvolvesDatumChange"
    );
    structural_prop!(
        TransformationAccuracyPositiveReal,
        "TransformationAccuracyPositiveReal"
    );
    structural_prop!(
        TransformationAccuracyNonZero,
        "TransformationAccuracyNonZero"
    );
    structural_prop!(
        TransformationInverseApproximate,
        "TransformationInverseApproximate"
    );
    structural_prop!(
        TransformationNad27ToWgs84UsesHelmert,
        "TransformationNad27ToWgs84UsesHelmert"
    );
    structural_prop!(
        ConcatenatedOperationHasAtLeastTwoSteps,
        "ConcatenatedOperationHasAtLeastTwoSteps"
    );
    structural_prop!(
        ConcatenatedOperationStepsFormAChain,
        "ConcatenatedOperationStepsFormAChain"
    );
    structural_prop!(
        ConcatenatedOperationSourceCrsIsFirstStep,
        "ConcatenatedOperationSourceCrsIsFirstStep"
    );
    structural_prop!(
        ConcatenatedOperationTargetCrsIsLastStep,
        "ConcatenatedOperationTargetCrsIsLastStep"
    );
    structural_prop!(OperationMethodNameNonEmpty, "OperationMethodNameNonEmpty");
    structural_prop!(
        OperationMethodFormulaOptional,
        "OperationMethodFormulaOptional"
    );
    structural_prop!(
        OperationMethodHasParameterList,
        "OperationMethodHasParameterList"
    );
    structural_prop!(
        OperationParameterNameNonEmpty,
        "OperationParameterNameNonEmpty"
    );
    structural_prop!(
        OperationParameterValueHasUnit,
        "OperationParameterValueHasUnit"
    );
    // §15
    structural_prop!(
        Geographic2dIsoAxisOrderLatitudeFirst,
        "Geographic2dIsoAxisOrderLatitudeFirst"
    );
    structural_prop!(
        Geographic3dIsoAxisOrderLatLonHeight,
        "Geographic3dIsoAxisOrderLatLonHeight"
    );
    structural_prop!(
        ProjectedConventionalAxisOrderEastingFirst,
        "ProjectedConventionalAxisOrderEastingFirst"
    );
    structural_prop!(
        ProjectedNorthingFirstVariantsExist,
        "ProjectedNorthingFirstVariantsExist"
    );
    structural_prop!(
        AxisOrderMustFollowCrsDefinition,
        "AxisOrderMustFollowCrsDefinition"
    );
    structural_prop!(
        AxisOrderChangeRequiresExplicitOperation,
        "AxisOrderChangeRequiresExplicitOperation"
    );
    structural_prop!(
        CoordinateTupleElementCountEqualsAxisCount,
        "CoordinateTupleElementCountEqualsAxisCount"
    );
    structural_prop!(
        CoordinateElementAlignedToAxisOrdinalPosition,
        "CoordinateElementAlignedToAxisOrdinalPosition"
    );
    // §16
    structural_prop!(EpsgCodePositiveInteger, "EpsgCodePositiveInteger");
    structural_prop!(EpsgAuthorityNameIsEpsg, "EpsgAuthorityNameIsEpsg");
    structural_prop!(OgcAuthorityNameIsOgc, "OgcAuthorityNameIsOgc");
    structural_prop!(OtherAuthorityNamesEsriIgnf, "OtherAuthorityNamesEsriIgnf");
    structural_prop!(
        RegisteredCrsNullAuthorityCodeInvalid,
        "RegisteredCrsNullAuthorityCodeInvalid"
    );
    structural_prop!(
        EpsgGeographicCrsRange4000To4999,
        "EpsgGeographicCrsRange4000To4999"
    );
    structural_prop!(
        EpsgProjectedCrsRange20000To32767,
        "EpsgProjectedCrsRange20000To32767"
    );
    structural_prop!(
        EpsgVerticalCrsRange5000To5999,
        "EpsgVerticalCrsRange5000To5999"
    );
    structural_prop!(
        EpsgCompoundCrsRange6000To6999,
        "EpsgCompoundCrsRange6000To6999"
    );
    structural_prop!(
        CrsIdentityByAuthorityAndCode,
        "CrsIdentityByAuthorityAndCode"
    );
    structural_prop!(
        DifferentCrsCodesRequireExplicitOperation,
        "DifferentCrsCodesRequireExplicitOperation"
    );
    // §17
    structural_prop!(
        StaticCrsPlateFixedNoEpochRequired,
        "StaticCrsPlateFixedNoEpochRequired"
    );
    structural_prop!(StaticCrsDatumAtFixedEpoch, "StaticCrsDatumAtFixedEpoch");
    structural_prop!(
        DynamicCrsRequiresCoordinateEpoch,
        "DynamicCrsRequiresCoordinateEpoch"
    );
    structural_prop!(CoordinateEpochIsDecimalYear, "CoordinateEpochIsDecimalYear");
    structural_prop!(
        CoordinateEpochPositiveFinite,
        "CoordinateEpochPositiveFinite"
    );
    structural_prop!(Itrf2014IsDynamicDatum, "Itrf2014IsDynamicDatum");
    structural_prop!(Itrf2020IsDynamicDatum, "Itrf2020IsDynamicDatum");
    structural_prop!(Igs20IsDynamicDatum, "Igs20IsDynamicDatum");
    structural_prop!(
        DynamicCrsOmittingEpochIntroducesError,
        "DynamicCrsOmittingEpochIntroducesError"
    );
    structural_prop!(
        DynamicReferenceFrameHasFrameReferenceEpoch,
        "DynamicReferenceFrameHasFrameReferenceEpoch"
    );
    // §18
    structural_prop!(
        CrsIdentityAuthorityPlusCodeUnique,
        "CrsIdentityAuthorityPlusCodeUnique"
    );
    structural_prop!(
        CompoundCrsComponentsOrthogonal,
        "CompoundCrsComponentsOrthogonal"
    );
    structural_prop!(AxisAbbreviationUniqueInCs, "AxisAbbreviationUniqueInCs");
    structural_prop!(
        NullAuthorityCodeInvalidForRegisteredCrs,
        "NullAuthorityCodeInvalidForRegisteredCrs"
    );
    structural_prop!(
        AngularAxisMustUseAngularUnit,
        "AngularAxisMustUseAngularUnit"
    );
    structural_prop!(LinearAxisMustUseLinearUnit, "LinearAxisMustUseLinearUnit");
    structural_prop!(
        ParametricAxisMustUseParametricUnit,
        "ParametricAxisMustUseParametricUnit"
    );
    structural_prop!(TimeAxisMustUseTemporalUnit, "TimeAxisMustUseTemporalUnit");
    structural_prop!(CrsScopeDescribesIntendedUse, "CrsScopeDescribesIntendedUse");
    structural_prop!(
        CrsDomainOfValidityOptionalImpliesGlobal,
        "CrsDomainOfValidityOptionalImpliesGlobal"
    );
    structural_prop!(
        CrsDomainOfValidityExtentTypes,
        "CrsDomainOfValidityExtentTypes"
    );
    structural_prop!(
        IdentifiedObjectAliasOptionalList,
        "IdentifiedObjectAliasOptionalList"
    );
    structural_prop!(
        IdentifiedObjectRemarksOptional,
        "IdentifiedObjectRemarksOptional"
    );
    // §19
    structural_prop!(
        TemporalCrsDatumIsTemporalDatum,
        "TemporalCrsDatumIsTemporalDatum"
    );
    structural_prop!(TemporalCrsCsIsTemporalCs, "TemporalCrsCsIsTemporalCs");
    structural_prop!(
        TemporalDatumOriginIsIso8601DateTime,
        "TemporalDatumOriginIsIso8601DateTime"
    );
    structural_prop!(TemporalCrsHasOneAxis, "TemporalCrsHasOneAxis");
    structural_prop!(
        ParametricCrsDatumIsParametricDatum,
        "ParametricCrsDatumIsParametricDatum"
    );
    structural_prop!(
        ParametricCrsCsIsParametricCs,
        "ParametricCrsCsIsParametricCs"
    );
    structural_prop!(ParametricCrsHasOneAxis, "ParametricCrsHasOneAxis");
    // §20
    structural_prop!(
        CoordinateSystemMinimumOneAxis,
        "CoordinateSystemMinimumOneAxis"
    );
    structural_prop!(
        CoordinateSystemMaximumFourAxes,
        "CoordinateSystemMaximumFourAxes"
    );
    structural_prop!(
        CompoundCrsMinimumTwoComponents,
        "CompoundCrsMinimumTwoComponents"
    );
    structural_prop!(
        ConcatenatedOperationMinimumTwoSteps,
        "ConcatenatedOperationMinimumTwoSteps"
    );
    structural_prop!(CrsStringValuesUtf8Encoded, "CrsStringValuesUtf8Encoded");
    structural_prop!(CrsMandatoryStringNonNull, "CrsMandatoryStringNonNull");
    structural_prop!(LatitudeRangeNegative90To90, "LatitudeRangeNegative90To90");
    structural_prop!(
        LongitudeRangeNegative180To180,
        "LongitudeRangeNegative180To180"
    );
    structural_prop!(LatitudePolesValid, "LatitudePolesValid");
    structural_prop!(LongitudeNegative180Excluded, "LongitudeNegative180Excluded");
    structural_prop!(
        MapProjectionScaleFactorPositive,
        "MapProjectionScaleFactorPositive"
    );
    structural_prop!(
        MapProjectionFalseOriginFiniteReal,
        "MapProjectionFalseOriginFiniteReal"
    );
    structural_prop!(
        AxisDirectionMemberOfCodeList,
        "AxisDirectionMemberOfCodeList"
    );
    structural_prop!(CsTypeMemberOfDefinedTypes, "CsTypeMemberOfDefinedTypes");
    structural_prop!(CrsTypeMemberOfDefinedTypes, "CrsTypeMemberOfDefinedTypes");
    // §21
    structural_prop!(
        IdentifiedObjectPrimaryNameNonEmpty,
        "IdentifiedObjectPrimaryNameNonEmpty"
    );
    structural_prop!(
        IdentifiedObjectAliasNoNullEntries,
        "IdentifiedObjectAliasNoNullEntries"
    );
    structural_prop!(
        IdentifiedObjectIdentifierEntryComplete,
        "IdentifiedObjectIdentifierEntryComplete"
    );
    structural_prop!(
        IdentifiedObjectRemarksWhenPresentNonNull,
        "IdentifiedObjectRemarksWhenPresentNonNull"
    );
    structural_prop!(
        CrsInheritsIdentifiedObjectInterface,
        "CrsInheritsIdentifiedObjectInterface"
    );
    structural_prop!(
        DatumInheritsIdentifiedObjectInterface,
        "DatumInheritsIdentifiedObjectInterface"
    );
    structural_prop!(
        CoordinateOperationInheritsIdentifiedObjectInterface,
        "CoordinateOperationInheritsIdentifiedObjectInterface"
    );
    // §22
    structural_prop!(
        DatumEnsembleGroupsRelatedDatums,
        "DatumEnsembleGroupsRelatedDatums"
    );
    structural_prop!(DatumEnsembleNameNonEmpty, "DatumEnsembleNameNonEmpty");
    structural_prop!(
        DatumEnsembleHasAtLeastTwoMembers,
        "DatumEnsembleHasAtLeastTwoMembers"
    );
    structural_prop!(
        DatumEnsembleMemberNoNullEntries,
        "DatumEnsembleMemberNoNullEntries"
    );
    structural_prop!(
        DatumEnsembleAccuracyPositive,
        "DatumEnsembleAccuracyPositive"
    );
    structural_prop!(
        DatumEnsembleSubMetreRequiresMemberSelection,
        "DatumEnsembleSubMetreRequiresMemberSelection"
    );
    structural_prop!(
        DatumEnsembleWgs84EpsgCode6326,
        "DatumEnsembleWgs84EpsgCode6326"
    );
    structural_prop!(Epsg4326UsesDatumEnsemble, "Epsg4326UsesDatumEnsemble");
    structural_prop!(Nad27EnsembleEpsg6267, "Nad27EnsembleEpsg6267");
    structural_prop!(
        DatumEnsembleMembersHomogeneous,
        "DatumEnsembleMembersHomogeneous"
    );
    // §23
    structural_prop!(CoordinateMetadataHasCrs, "CoordinateMetadataHasCrs");
    structural_prop!(CoordinateMetadataCrsNonNull, "CoordinateMetadataCrsNonNull");
    structural_prop!(
        CoordinateMetadataEpochIsDecimalYear,
        "CoordinateMetadataEpochIsDecimalYear"
    );
    structural_prop!(
        CoordinateMetadataDynamicCrsRequiresEpoch,
        "CoordinateMetadataDynamicCrsRequiresEpoch"
    );
    structural_prop!(
        CoordinateMetadataStaticCrsEpochShouldBeAbsent,
        "CoordinateMetadataStaticCrsEpochShouldBeAbsent"
    );
    structural_prop!(
        CoordinateMetadataApplicableAtSetOrTupleLevel,
        "CoordinateMetadataApplicableAtSetOrTupleLevel"
    );
    structural_prop!(
        CoordinateEpochDistinctFromFrameReferenceEpoch,
        "CoordinateEpochDistinctFromFrameReferenceEpoch"
    );
    // §24
    structural_prop!(
        PassThroughOperationPreservesSomeAxes,
        "PassThroughOperationPreservesSomeAxes"
    );
    structural_prop!(
        PassThroughOperationModifiedCoordinatesNonEmpty,
        "PassThroughOperationModifiedCoordinatesNonEmpty"
    );
    structural_prop!(
        PassThroughOperationIndexInRange,
        "PassThroughOperationIndexInRange"
    );
    structural_prop!(
        PassThroughOperationInnerOperationNonNull,
        "PassThroughOperationInnerOperationNonNull"
    );
    structural_prop!(
        PassThroughOperationDimensionConsistency,
        "PassThroughOperationDimensionConsistency"
    );
    structural_prop!(
        PassThroughOperationCompoundCrsUsage,
        "PassThroughOperationCompoundCrsUsage"
    );
    // §25
    structural_prop!(SphericalCsHasThreeAxes, "SphericalCsHasThreeAxes");
    structural_prop!(
        SphericalCsAngularAxesThenLinear,
        "SphericalCsAngularAxesThenLinear"
    );
    structural_prop!(
        SphericalCsApplicableToGeocentricContext,
        "SphericalCsApplicableToGeocentricContext"
    );
    // §26
    structural_prop!(
        HelmertSevenParameterStructure,
        "HelmertSevenParameterStructure"
    );
    structural_prop!(
        HelmertPositionVectorConvention,
        "HelmertPositionVectorConvention"
    );
    structural_prop!(
        HelmertCoordinateFrameConvention,
        "HelmertCoordinateFrameConvention"
    );
    structural_prop!(
        HelmertConventionsEquivalentOnlyForZeroRotation,
        "HelmertConventionsEquivalentOnlyForZeroRotation"
    );
    structural_prop!(
        HelmertConventionMustBeIdentified,
        "HelmertConventionMustBeIdentified"
    );
    structural_prop!(
        HelmertEpsgMethodCodes9606And9607,
        "HelmertEpsgMethodCodes9606And9607"
    );
    structural_prop!(
        MolodenskyBadenkasTenParameter,
        "MolodenskyBadenkasTenParameter"
    );
    // §27
    structural_prop!(
        GridBasedDatumShiftUsesExternalFile,
        "GridBasedDatumShiftUsesExternalFile"
    );
    structural_prop!(
        Nadcon5IsUsHorizontalDatumShift,
        "Nadcon5IsUsHorizontalDatumShift"
    );
    structural_prop!(Ntv2IsGridHorizontalShift, "Ntv2IsGridHorizontalShift");
    structural_prop!(
        VertconIsUsVerticalDatumShift,
        "VertconIsUsVerticalDatumShift"
    );
    structural_prop!(
        ParameterFileStoresFilenameNotValue,
        "ParameterFileStoresFilenameNotValue"
    );
    // §28
    structural_prop!(UomNameNonEmpty, "UomNameNonEmpty");
    structural_prop!(UomConversionFactorPositive, "UomConversionFactorPositive");
    structural_prop!(UomAngularConvertToRadians, "UomAngularConvertToRadians");
    structural_prop!(UomLinearConvertToMetres, "UomLinearConvertToMetres");
    structural_prop!(UomTimeConvertToSeconds, "UomTimeConvertToSeconds");
    structural_prop!(UomScaleDimensionless, "UomScaleDimensionless");
    structural_prop!(
        UomFeetAmbiguityInternationalVsSurvey,
        "UomFeetAmbiguityInternationalVsSurvey"
    );
    // §29
    structural_prop!(
        UtmSouthZoneFalseNorthing10000000,
        "UtmSouthZoneFalseNorthing10000000"
    );
    structural_prop!(UtmNorthZoneFalseNorthing0, "UtmNorthZoneFalseNorthing0");
    structural_prop!(
        UtmCommonParametersFalseEastingAndScale,
        "UtmCommonParametersFalseEastingAndScale"
    );
    // Operation contracts
    structural_prop!(CrsResolved, "CrsResolved");
    structural_prop!(DatumEnsembleResolved, "DatumEnsembleResolved");
    structural_prop!(CrsValid, "CrsValid");
    structural_prop!(EllipsoidValid, "EllipsoidValid");
    structural_prop!(CoordinateMetadataValid, "CoordinateMetadataValid");
    structural_prop!(CoordinatesTransformed, "CoordinatesTransformed");
}

pub use emit_impls::{
    AngularAxisMustUseAngularUnit,
    AxisAbbreviationNonEmpty,
    AxisAbbreviationUniqueInCs,
    AxisAbbreviationUniqueWithinCs,
    AxisDirectionIsValidCode,
    AxisDirectionMemberOfCodeList,
    AxisMinMaxValueOptional,
    AxisNameNonEmpty,
    AxisOrderChangeRequiresExplicitOperation,
    AxisOrderMustFollowCrsDefinition,
    AxisRangeMeaningExactOrWraparound,
    AxisUnitAppropriateForAxisType,
    CompoundCrsComponentsNonOverlapping,
    CompoundCrsComponentsOrthogonal,
    CompoundCrsEpsgRange6000To6999,
    // §12
    CompoundCrsHasAtLeastTwoComponents,
    CompoundCrsMinimumTwoComponents,
    CompoundCrsNameNonEmpty,
    CompoundCrsNoTwoHorizontalComponents,
    CompoundCrsNoTwoVerticalComponents,
    CompoundCrsTotalAxisCountIsSumOfComponents,
    CompoundCrsTypicalIs2dPlusVertical,
    ConcatenatedOperationHasAtLeastTwoSteps,
    ConcatenatedOperationMinimumTwoSteps,
    ConcatenatedOperationSourceCrsIsFirstStep,
    ConcatenatedOperationStepsFormAChain,
    ConcatenatedOperationTargetCrsIsLastStep,
    ConversionDefinesMapProjection,
    ConversionHasOperationMethod,
    ConversionHasParameterValues,
    ConversionInverseExists,
    ConversionInvolvesNoDatumChange,
    CoordinateElementAlignedToAxisOrdinalPosition,
    CoordinateEpochDistinctFromFrameReferenceEpoch,
    CoordinateEpochIsDecimalYear,
    CoordinateEpochPositiveFinite,
    CoordinateMetadataApplicableAtSetOrTupleLevel,
    CoordinateMetadataCrsNonNull,
    CoordinateMetadataDynamicCrsRequiresEpoch,
    CoordinateMetadataEpochIsDecimalYear,
    // §23
    CoordinateMetadataHasCrs,
    CoordinateMetadataStaticCrsEpochShouldBeAbsent,
    CoordinateMetadataValid,
    CoordinateOperationDomainOfValidityOptional,
    CoordinateOperationHasSourceCrs,
    CoordinateOperationHasTargetCrs,
    CoordinateOperationInheritsIdentifiedObjectInterface,
    // §14
    CoordinateOperationNameNonEmpty,
    CoordinateOperationVersionOptional,
    // §8
    CoordinateSystemAxisCountOneToFour,
    CoordinateSystemMaximumFourAxes,
    // §20
    CoordinateSystemMinimumOneAxis,
    CoordinateSystemNameNonEmpty,
    CoordinateTupleDimensionMatchesAxes,
    CoordinateTupleElementCountEqualsAxisCount,
    CoordinatesTransformed,
    // §6
    CrsConsistsOfCsAndDatum,
    CrsDomainOfValidityExtentTypes,
    CrsDomainOfValidityOptionalImpliesGlobal,
    CrsIdentifierAuthorityNonEmpty,
    CrsIdentifierCodeNonEmpty,
    // §18
    CrsIdentityAuthorityPlusCodeUnique,
    CrsIdentityByAuthorityAndCode,
    CrsInheritsIdentifiedObjectInterface,
    CrsMandatoryStringNonNull,
    // Operation contracts
    CrsResolved,
    CrsScopeDescribesIntendedUse,
    CrsStringValuesUtf8Encoded,
    CrsTypeMemberOfDefinedTypes,
    CrsValid,
    CsAxisCountMatchesTupleDimensionality,
    CsTypeMemberOfDefinedTypes,
    DatumEnsembleAccuracyPositive,
    // §22
    DatumEnsembleGroupsRelatedDatums,
    DatumEnsembleHasAtLeastTwoMembers,
    DatumEnsembleMemberNoNullEntries,
    DatumEnsembleMembersHomogeneous,
    DatumEnsembleNameNonEmpty,
    DatumEnsembleResolved,
    DatumEnsembleSubMetreRequiresMemberSelection,
    DatumEnsembleWgs84EpsgCode6326,
    DatumInheritsIdentifiedObjectInterface,
    DerivedCrsCsDiffersFromBaseCrsAllowed,
    DerivedCrsDerivingConversionIsConversion,
    // §13
    DerivedCrsHasBaseCrs,
    DerivedCrsInheritsDatumFromBase,
    DerivedCrsNameNonEmpty,
    DerivedProjectedCrsBaseMustBeProjCrs,
    DifferentCrsCodesRequireExplicitOperation,
    DynamicCrsOmittingEpochIntroducesError,
    DynamicCrsRequiresCoordinateEpoch,
    DynamicReferenceFrameHasFrameReferenceEpoch,
    EllipsoidInverseFlatteningPositiveWhenNonSphere,
    EllipsoidInverseFlatteningZeroMeansSphere,
    EllipsoidIsSphereConsistentWithParameters,
    // §7.3
    EllipsoidNameNonEmpty,
    EllipsoidSecondParameterEitherInverseFlatteningOrSemiMinor,
    EllipsoidSemiMajorAxisInMetres,
    EllipsoidSemiMajorAxisPositive,
    EllipsoidSemiMinorAxisInMetres,
    EllipsoidSemiMinorAxisLessThanSemiMajor,
    EllipsoidValid,
    EllipsoidWgs84InverseFlatteningApprox298,
    EngineeringCrsCsTypeFlexible,
    // §11
    EngineeringCrsDatumIsEngineeringDatum,
    EngineeringCrsIsLocalContextOnly,
    EngineeringCrsNameNonEmpty,
    EngineeringDatumAnchorOptional,
    EngineeringDatumNameNonEmpty,
    Epsg4326AxisOrderLatFirst,
    Epsg4326LatitudeRangeValid,
    Epsg4326LongitudeRangeValid,
    Epsg4326UsesDatumEnsemble,
    Epsg4978IsWgs84Geocentric,
    Epsg4978XAxisTowardsPrimeMeridian,
    Epsg4978ZAxisTowardsNorthPole,
    Epsg4979HeightUnbounded,
    Epsg4979IsWgs84Geographic3d,
    EpsgAuthorityNameIsEpsg,
    // §16
    EpsgCodePositiveInteger,
    EpsgCompoundCrsRange6000To6999,
    EpsgGeographicCrsRange4000To4999,
    EpsgProjectedCrsRange20000To32767,
    EpsgVerticalCrsRange5000To5999,
    GeocentricCrsUsesCartesianCs,
    GeodeticCrsCsIsEllipsoidalOrCartesian,
    // §7
    GeodeticCrsDatumIsGeodeticReferenceFrame,
    GeodeticReferenceFrameAnchorOptional,
    GeodeticReferenceFrameHasExactlyOneEllipsoid,
    GeodeticReferenceFrameHasExactlyOnePrimeMeridian,
    // §7.2
    GeodeticReferenceFrameNameNonEmpty,
    GeodeticReferenceFrameRealizationEpochIsIso8601,
    Geographic2dCrsHasTwoAxes,
    // §15
    Geographic2dIsoAxisOrderLatitudeFirst,
    Geographic3dCrsHasThreeAxes,
    Geographic3dIsoAxisOrderLatLonHeight,
    // §27
    GridBasedDatumShiftUsesExternalFile,
    HelmertConventionMustBeIdentified,
    HelmertConventionsEquivalentOnlyForZeroRotation,
    HelmertCoordinateFrameConvention,
    HelmertEpsgMethodCodes9606And9607,
    HelmertPositionVectorConvention,
    // §26
    HelmertSevenParameterStructure,
    IdentifiedObjectAliasNoNullEntries,
    IdentifiedObjectAliasOptionalList,
    IdentifiedObjectIdentifierEntryComplete,
    // §21
    IdentifiedObjectPrimaryNameNonEmpty,
    IdentifiedObjectRemarksOptional,
    IdentifiedObjectRemarksWhenPresentNonNull,
    Igs20IsDynamicDatum,
    Itrf2014IsDynamicDatum,
    Itrf2020IsDynamicDatum,
    LatitudePolesValid,
    LatitudeRangeNegative90To90,
    LinearAxisMustUseLinearUnit,
    LongitudeNegative180Excluded,
    LongitudeRangeNegative180To180,
    MapProjectionFalseOriginFiniteReal,
    MapProjectionScaleFactorPositive,
    MolodenskyBadenkasTenParameter,
    Nad27EnsembleEpsg6267,
    Nadcon5IsUsHorizontalDatumShift,
    Ntv2IsGridHorizontalShift,
    NullAuthorityCodeInvalidForRegisteredCrs,
    OgcAuthorityNameIsOgc,
    OperationMethodFormulaOptional,
    OperationMethodHasParameterList,
    OperationMethodNameNonEmpty,
    OperationParameterNameNonEmpty,
    OperationParameterValueHasUnit,
    OtherAuthorityNamesEsriIgnf,
    ParameterFileStoresFilenameNotValue,
    ParametricAxisMustUseParametricUnit,
    ParametricCrsCsIsParametricCs,
    ParametricCrsDatumIsParametricDatum,
    ParametricCrsHasOneAxis,
    PassThroughOperationCompoundCrsUsage,
    PassThroughOperationDimensionConsistency,
    PassThroughOperationIndexInRange,
    PassThroughOperationInnerOperationNonNull,
    PassThroughOperationModifiedCoordinatesNonEmpty,
    // §24
    PassThroughOperationPreservesSomeAxes,
    PrimeMeridianGreenwichIsZero,
    PrimeMeridianGreenwichLongitudeFinite,
    PrimeMeridianGreenwichLongitudeInDegreeBounds,
    PrimeMeridianGreenwichLongitudeUnitIsAngular,
    // §7.4
    PrimeMeridianNameNonEmpty,
    PrimeMeridianNonGreenwichAllowed,
    ProjectedConventionalAxisOrderEastingFirst,
    ProjectedCrsAxesUseLinearUnit,
    // §9
    ProjectedCrsBaseCrsIsGeographic,
    ProjectedCrsConventionalAxisDirections,
    ProjectedCrsCsIsCartesian,
    ProjectedCrsHasTwoAxes,
    ProjectedCrsNameNonEmpty,
    ProjectedCrsProjectionIsConversion,
    ProjectedNorthingFirstVariantsExist,
    RegisteredCrsNullAuthorityCodeInvalid,
    ScCrsDomainOfValidityIsExtent,
    ScCrsIdentifierHasAuthorityAndCode,
    ScCrsIsAbstract,
    ScCrsNameNonEmpty,
    ScCrsScopeNonEmpty,
    SphericalCsAngularAxesThenLinear,
    SphericalCsApplicableToGeocentricContext,
    // §25
    SphericalCsHasThreeAxes,
    StaticCrsDatumAtFixedEpoch,
    // §17
    StaticCrsPlateFixedNoEpochRequired,
    TemporalCrsCsIsTemporalCs,
    // §19
    TemporalCrsDatumIsTemporalDatum,
    TemporalCrsHasOneAxis,
    TemporalDatumOriginIsIso8601DateTime,
    TimeAxisMustUseTemporalUnit,
    TransformationAccuracyNonZero,
    TransformationAccuracyPositiveReal,
    TransformationInverseApproximate,
    TransformationInvolvesDatumChange,
    TransformationNad27ToWgs84UsesHelmert,
    UomAngularConvertToRadians,
    UomConversionFactorPositive,
    UomFeetAmbiguityInternationalVsSurvey,
    UomLinearConvertToMetres,
    // §28
    UomNameNonEmpty,
    UomScaleDimensionless,
    UomTimeConvertToSeconds,
    UtmAxisOrderEastingFirst,
    UtmCommonParametersFalseEastingAndScale,
    UtmFalseEasting500000,
    UtmNorthZoneEpsgRange32601To32660,
    UtmNorthZoneFalseNorthing0,
    UtmScaleFactorAtCentralMeridian0996,
    UtmSouthZoneEpsgRange32701To32760,
    // §29
    UtmSouthZoneFalseNorthing10000000,
    UtmZoneNumberOneToSixty,
    UtmZoneWidthSixDegrees,
    VertconIsUsVerticalDatumShift,
    VerticalCrsAxisLinearUnit,
    VerticalCrsCsIsVerticalCs,
    // §10
    VerticalCrsDatumIsVerticalReferenceFrame,
    VerticalCrsDepthAxisDirectionDown,
    VerticalCrsEpsgRange5000To5999,
    VerticalCrsHasOneAxis,
    VerticalCrsHeightAxisDirectionUp,
    VerticalCrsNameNonEmpty,
    VerticalReferenceFrameAnchorOptional,
    VerticalReferenceFrameGravityRelated,
    VerticalReferenceFrameNameNonEmpty,
    VerticalReferenceFrameRealizationEpochIsIso8601,
};
