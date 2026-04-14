//! `elicit_gis` — Geospatial contract interface crate.
//!
//! Provides a standards-anchored vocabulary of geospatial propositions
//! ([`contracts`]), domain types ([`types`]), and object-safe async traits
//! ([`traits`]) for geospatial framework boundaries.
//!
//! # Design
//!
//! This is an **interface crate**, not an implementation. Geospatial drivers
//! (`elicit_geo`, `elicit_proj`, `elicit_geojson`) implement the traits;
//! consumers depend on this crate only.
//!
//! Traits use [`Established<P>`] contract return types instead of associated
//! types, giving object safety (`dyn GisCrsLookup`) and a common proof
//! language at call sites.
//!
//! # Standards
//!
//! - ISO 19111:2019 — Spatial referencing by coordinates
//! - OGC Simple Features Specification Part 1 (OGC 06-103r4)
//! - ISO 19115-1:2014 — Geographic metadata
//! - FGDC CSDGM — Content Standard for Digital Geospatial Metadata
//! - OGC WKT-CRS (OGC 18-010r7)
//! - RFC 7946 — GeoJSON
//!
//! [`Established<P>`]: elicitation::Established

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod contracts;
mod error;
mod traits;
mod types;

pub use contracts::{
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

pub use error::{GisError, GisErrorKind, GisResult};
pub use traits::{GisBackend, GisCrsLookup, GisCrsTransformer, GisCrsValidator};
pub use types::{
    AuthorityCode, AxisDirection, CoordinateMetadata, CrsInfo, CrsType, CsType, DatumEnsembleInfo,
    DecimalYear, EllipsoidParams, EpsgCode, HelmertConvention,
};
