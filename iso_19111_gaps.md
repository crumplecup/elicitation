# ISO 19111:2019 Gap Coverage Plan

## Current Status: ~7/10 Coverage

## Phase 1: Critical Missing Sections (High Priority)

### §7.4 Coordinate Metadata Completeness

- [ ] `CoordinateMetadataCompletenessVerified` - All required metadata elements present
- [ ] `CoordinateMetadataCrsNonNull` - CRS reference mandatory
- [ ] `CoordinateMetadataEpochFormatValid` - Decimal year format compliance (§7.4)
- [ ] `CoordinateMetadataEpochRequiredForDynamic` - Epoch mandatory for dynamic CRS (§7.4)
- [ ] `CoordinateMetadataEpochAbsentForStatic` - Epoch absent for static CRS (§7.4)
- [ ] `CoordinateMetadataApplicationScopeValid` - Metadata application level specified (§7.4)

### §16 Operation Methods & Parameters

- [ ] `OperationMethodFormulaPresent` - Mathematical formula provided when required (§16.6)
- [ ] `OperationMethodFormulaUtf8` - Formula encoded in UTF-8 (§16.6)
- [ ] `OperationMethodParameterSetComplete` - All required parameters present (§16.6)
- [ ] `OperationMethodParameterNoDuplicates` - No duplicate parameter names (§16.6)
- [ ] `ParameterValueUnitConsistent` - Parameter value units match definition (§16.7)
- [ ] `ParameterValueFiniteReal` - Parameter values are finite real numbers (§16.7)
- [ ] `ParameterValueWithinBounds` - Parameter values within method-defined bounds (§16.7)

### §8.2 Geodetic Reference Frame Details

- [ ] `GeodeticReferenceFrameAnchorDefinitionValid` - Anchor definition content UTF-8 (§8.2)
- [ ] `GeodeticReferenceFrameAnchorDefinitionMeaningful` - Anchor definition non-trivial (§8.2)
- [ ] `RealizationEpochFormatISO8601` - Epoch in ISO 8601 format (§8.2)
- [ ] `RealizationEpochInRange` - Epoch within reasonable temporal bounds (§8.2)
- [ ] `RealizationEpochPrecisionSufficient` - Epoch precision adequate for application (§8.2)

### §17 Dynamic Datum Completeness

- [ ] `DynamicDatumFrameReferenceEpochPresent` - Frame reference epoch mandatory (§17.3)
- [ ] `FrameReferenceEpochFormatValid` - Frame epoch in decimal year format (§17.3)
- [ ] `FrameReferenceEpochConsistent` - Frame epoch reasonable for datum (§17.3)
- [ ] `DynamicDatumVelocityModelIndicator` - Velocity model presence indicated (§17.3)
- [ ] `CoordinateEpochFrameEpochDistinction` - Coordinate vs frame epoch distinction clear (§17.2)

## Phase 2: Completeness Validation (Medium Priority)

### UML Diagram Constraint Enforcement

- [ ] `UmlMultiplicityConstraintsMet` - All UML multiplicity constraints satisfied
- [ ] `UmlCompositionRulesFollowed` - Composition/aggregation rules respected
- [ ] `UmlInheritanceHierarchyValid` - Inheritance relationships correct
- [ ] `UmlAssociationCardinalitiesValid` - Association cardinalities observed

### §6.2 CRS Identification Completeness

- [ ] `CrsIdentificationComplete` - All identification elements present (§6.2)
- [ ] `CrsAliasListValid` - Alias list properly formed and encoded (§6.2)
- [ ] `CrsAliasNoDuplicates` - No duplicate aliases in list (§6.2)
- [ ] `CrsRemarksUtf8Encoded` - Remarks in UTF-8 encoding (§6.2)
- [ ] `CrsRemarksMeaningful` - Remarks contain substantive content (§6.2)
- [ ] `CrsScopeContentValid` - Scope describes intended use adequately (§6.2)

### §9 Coordinate System Completeness

- [ ] `CsAxisDirectionCodeListCompliant` - Axis directions from official code list (§9)
- [ ] `CsTypeCodeListValid` - CS types from defined enumeration (§9)
- [ ] `AxisAbbreviationUtf8` - Axis abbreviations UTF-8 encoded (§9)
- [ ] `AxisNameUtf8` - Axis names UTF-8 encoded (§9)
- [ ] `AxisUnitMeasureValid` - Units from recognized measurement systems (§9)

### Cross-Standard References (ISO 19115)

- [ ] `MetadataCitationComplete` - CI_Citation elements present and valid
- [ ] `MetadataExtentGeographicValid` - EX_Extent geographic elements valid
- [ ] `MetadataExtentTemporalValid` - EX_Extent temporal elements valid
- [ ] `MetadataExtentVerticalValid` - EX_Extent vertical elements valid

## Phase 3: Edge Case Coverage (Low Priority)

### §12 Engineering CRS Details

- [ ] `EngineeringDatumAnchorDescriptionValid` - Anchor description UTF-8 (§12.3)
- [ ] `EngineeringDatumAnchorMeaningful` - Anchor description substantive (§12.3)
- [ ] `EngineeringCrsScopeLocalContext` - Scope indicates local applicability (§12.2)

### §19 Parametric CRS Completeness

- [ ] `ParametricDatumOriginFormatValid` - Origin in ISO 8601 datetime format (§19.3)
- [ ] `ParametricDatumOriginInRange` - Origin within reasonable temporal bounds (§19.3)
- [ ] `ParametricCsAxisTypeValid` - Axis type appropriate for parametric CS (§19.2)
- [ ] `ParametricAxisUnitValid` - Unit appropriate for parametric quantity (§19.2)

### Encoding Format Validation

- [ ] `GmlEncodingValid` - GML representation conforms to standard
- [ ] `XmlElementCardinalityCorrect` - XML element occurrences match UML multiplicities
- [ ] `XmlAttributePresenceValid` - Required XML attributes present
- [ ] `XmlNamespaceDeclarationsValid` - Namespace declarations correct

## Success Criteria

- [ ] All propositions above implemented with ISO 19111:2019 citations
- [ ] Each proposition includes clear validation logic
- [ ] Cross-references to dependent standards included
- [ ] Coverage extends to UML diagram constraints
- [ ] Result: 10/10 ISO 19111:2019 compliance score

# Contract Type Granularity Enhancement Plan

## Current State: 8/10 Granularity

Basic structural and categorical contracts in place

## Goal: 10/10 Granularity

Mathematically precise, computationally verifiable contract types

## Phase 1: Ellipsoid Parameter Refinement

### Replace Broad Types:

```rust
// Current - too general
pub struct EllipsoidSemiMajorAxisPositive;
pub struct EllipsoidInverseFlatteningPositiveWhenNonSphere;
```

### With Precise Types:

```rust
pub struct EllipsoidSemiMajorAxisExactly6378137ForWgs84;
pub struct EllipsoidInverseFlatteningExactly298257223563ForWgs84;
pub struct EllipsoidSemiMinorAxisComputedFromSemiMajorAndFlattening;
pub struct EllipsoidFlatteningParameterConsistencyCheck;
pub struct EllipsoidEccentricitySquaredDerivationValid;
pub struct EllipsoidPolarRadiusOfCurvatureCorrect;
pub struct EllipsoidNormalRadiusOfCurvatureFormulaValid;
pub struct EllipsoidMeanRadiusComputationAccurate;
pub struct EllipsoidSurfaceAreaFormulaValid;
pub struct EllipsoidVolumeFormulaValid;
```

## Phase 2: Prime Meridian Precision

### Replace:

```rust
pub struct PrimeMeridianGreenwichLongitudeFinite;
```

### With:

```rust
pub struct PrimeMeridianGreenwichLongitudeExactlyZeroDegrees;
pub struct PrimeMeridianParisLongitudeExactly2d20m14d025e;
pub struct PrimeMeridianConversionToRadiansCorrect;
pub struct PrimeMeridianOffsetAppliedToCoordinates;
pub struct PrimeMeridianReferenceEpochSpecified;
pub struct PrimeMeridianAngularVelocityConsidered;
```

## Phase 3: Projection Formula Specificity

### Replace:

```rust
pub struct MapProjectionScaleFactorPositive;
```

### With Projection-Specific Types:

```rust
pub struct TransverseMercatorScaleFactorAtCentralMeridian09996;
pub struct TransverseMercatorFalseEasting500000Metres;
pub struct TransverseMercatorFalseNorthingZeroForNorthernHemisphere;
pub struct TransverseMercatorFalseNorthing10000000ForSouthernHemisphere;
pub struct LambertConformalConicStandardParallel1Valid;
pub struct LambertConformalConicStandardParallel2Valid;
pub struct LambertConformalConicCentralMeridianValid;
pub struct AlbersEqualAreaLatitudeOfOriginValid;
pub struct AlbersEqualAreaStandardParallelsDistinct;
pub struct StereographicProjectionCentralLatitudeValid;
pub struct MercatorProjectionScaleFactorAtEquatorValid;
pub struct OrthographicProjectionCenterLatitudeValid;
pub struct ObliqueStereographicProjectionAzimuthValid;
```

## Phase 4: Coordinate Transformation Decomposition

### Replace:

```rust
pub struct CoordinatesTransformed;
```

### With Chained Types:

```rust
pub struct GeodeticToGeocentricConversionFormulaApplied;
pub struct GeocentricToGeodeticIterationConverged;
pub struct HelmertTransformationSevenParametersValid;
pub struct HelmertRotationMatrixOrthogonal;
pub struct HelmertTranslationAppliedCorrectly;
pub struct HelmertScaleFactorApplied;
pub struct MolodenskyTransformationParametersComplete;
pub struct MolodenskyDatumShiftComputationValid;
pub struct GridShiftFileFormatValid;
pub struct GridShiftInterpolationWithinCell;
pub struct GridShiftCoordinateContinuityMaintained;
pub struct VerticalDatumShiftAppliedToHeightComponent;
pub struct CompoundTransformationChainedCorrectly;
pub struct ConcatenatedOperationErrorPropagated;
```

## Phase 5: Temporal Calculation Precision

### Replace:

```rust
pub struct CoordinateEpochIsDecimalYear;
```

### With Detailed Types:

```rust
pub struct CoordinateEpochDecimalYearComputationFromGregorianDate;
pub struct CoordinateEpochReferenceFrameEpochDifferenceCalculated;
pub struct VelocityModelTimeDeltaComputationCorrect;
pub struct PlateMotionDisplacementIntegratedOverTime;
pub struct SecularVariationRateAppliedToCoordinates;
pub struct CoordinateEpochPrecisionSufficientForAccuracy;
pub struct EpochDifferenceWithinTransformationValidityPeriod;
pub struct DynamicDatumCoordinateEpochMandatory;
pub struct StaticDatumCoordinateEpochAbsent;
```

## Phase 6: Numerical Method Contracts

### New Types for Computational Accuracy:

```rust
pub struct IterativeSolutionConvergenceWithin1eMinus12;
pub struct MatrixInversionConditionNumberAcceptable;
pub struct TrigonometricFunctionArgumentInRange;
pub struct LogarithmicFunctionArgumentPositive;
pub struct SquareRootArgumentNonNegative;
pub struct DivisionByZeroPreventedInFormula;
pub struct FloatingPointPrecisionMaintainedInChain;
pub struct RoundingErrorBoundedByDatumAccuracy;
pub struct InterpolationPolynomialDegreeAppropriate;
pub struct NumericalIntegrationStepSizeAdequate;
pub struct SeriesExpansionConvergedToTolerance;
```

## Phase 7: Formula Implementation Verification

### Mathematical Formula Contracts:

```rust
pub struct MeridianArcLengthIntegralFormulaCorrect;
pub struct ParallelArcLengthFormulaTrigonometricValid;
pub struct GeocentricLatitudeConversionFormulaValid;
pub struct ReducedLatitudeConversionFormulaValid;
pub struct AuthalicLatitudeConversionFormulaValid;
pub struct RectifyingLatitudeConversionFormulaValid;
pub struct ConformalLatitudeConversionFormulaValid;
pub struct IsometricLatitudeConversionFormulaValid;
pub struct GreatCircleDistanceFormulaHaversine;
pub struct ForwardAzimuthComputationOnSphereValid;
pub struct SphericalExcessComputationForTriangles;
```

## Phase 8: Unit and Measurement Precision

### Replace:

```rust
pub struct EllipsoidSemiMajorAxisInMetres;
```

### With:

```rust
pub struct EllipsoidSemiMajorAxisInSiMetresExactly;
pub struct EllipsoidSemiMajorAxisUncertaintyQuantified;
pub struct AngularMeasurementConvertedToRadians;
pub struct LinearMeasurementConvertedToMetres;
pub struct TemporalMeasurementConvertedToSeconds;
pub struct ScaleFactorDimensionlessQuantity;
pub struct UnitConversionFactorAppliedCorrectly;
pub struct UnitConsistencyMaintainedInFormula;
pub struct MeasurementPrecisionMatchesInstrumentCapability;
```

## Implementation Priority Order:

1. **Ellipsoid Parameter Contracts** - Foundation for all geodetic calculations
2. **Projection Formula Contracts** - Core mapping functionality
3. **Transformation Chain Contracts** - Coordinate interoperability
4. **Temporal Calculation Contracts** - Dynamic reference systems
5. **Numerical Method Contracts** - Computational reliability
6. **Unit Measurement Contracts** - Dimensional consistency

## Success Metrics:

- Each contract type represents a single, testable mathematical assertion
- Contracts compose to form complete validation chains
- Mathematical formulas can be extracted from contract definitions
- Numerical precision bounds are explicit in contract names
- Edge cases are handled by specific contract types rather than generic ones

# Targeted Critique of Current CRS Traits

## GisCrsLookup Trait

### Method: lookup_crs

**Current Signature:**

```rust
fn lookup_crs(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<(CrsInfo, Established<CrsResolved>)>>
```

**Critique:**

- No precondition contract on `AuthorityCode` input (should be `Established<AuthorityCodeFormatValid>`)
- Return type `CrsInfo` lacks mathematical coherence proofs
- Missing `Established<CrsComponentsMathematicallyConsistent>` in return
- No precision or completeness guarantees on returned CRS data

**Improvement Recommendation:**

```rust
fn resolve_crs_properties(
    &self,
    code: &AuthorityCode,
) -> BoxFuture<'_, GisResult<(
    CrsDefinition,  // Richer return type
    Established<AuthorityCodeValid>,
    Established<CrsStructurallyValid>,
    Established<CrsMathematicalComponentsConsistent>,
    Established<CrsParametersComplete>
)>>
```

### Method: resolve_datum_ensemble

**Current Signature:**

```rust
fn resolve_datum_ensemble(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<(DatumEnsembleInfo, Established<DatumEnsembleResolved>)>>
```

**Critique:**

- No member consistency checking in return
- Missing ensemble accuracy mathematical validation
- No verification of member datum homogeneity

**Improvement Recommendation:**

```rust
fn construct_datum_ensemble_properties(
    &self,
    code: &AuthorityCode,
) -> BoxFuture<'_, GisResult<(
    DatumEnsembleDefinition,
    Established<DatumEnsembleResolved>,
    Established<DatumEnsembleMembersHomogeneous>,
    Established<DatumEnsembleAccuracyMathematicallyValid>,
    Established<DatumEnsembleMemberRelationshipsConsistent>
)>>
```

## GisCrsValidator Trait

### Method: validate_crs

**Current Signature:**

```rust
fn validate_crs(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<Established<CrsValid>>>
```

**Critique:**

- Post-hoc "validation" mindset instead of construction
- Single boolean contract insufficient for mathematical rigor
- No decomposition into component validity
- Missing numerical precision contracts

**Improvement Recommendation:**

```rust
fn construct_crs_validity_properties(
    &self,
    code: &AuthorityCode,
) -> BoxFuture<'_, GisResult<(
    Established<CrsStructurallyValid>,
    Established<CrsMathematicallyCoherent>,
    Established<CrsComponentsConsistentlyDefined>,
    Established<CrsNumericalPropertiesWellFormed>
)>>
```

### Method: validate_ellipsoid

**Current Signature:**

```rust
fn validate_ellipsoid(&self, params: &EllipsoidParams) -> BoxFuture<'_, GisResult<Established<EllipsoidValid>>>
```

**Critique:**

- Comment describes checks but method signature hides them
- No mathematical relationship verification between parameters
- Missing computational precision contracts
- No curvature or derived property validation

**Improvement Recommendation:**

```rust
fn construct_ellipsoid_mathematical_properties(
    &self,
    params: &EllipsoidParams,
) -> BoxFuture<'_, GisResult<(
    EllipsoidMathematicalDefinition,
    Established<EllipsoidParametersConsistent>,
    Established<EllipsoidFormulasComputable>,
    Established<EllipsoidCurvaturePropertiesValid>,
    Established<EllipsoidNumericalPrecisionAdequate>
)>>
```

## GisCrsTransformer Trait

### Method: transform

**Current Signature:**

```rust
fn transform(&self, coords: &[f64], from: &AuthorityCode, to: &AuthorityCode) -> BoxFuture<'_, GisResult<(Vec<f64>, Established<CoordinatesTransformed>)>>
```

**Critique:**

- No precondition contracts on input CRS being mathematically valid
- Single `CoordinatesTransformed` contract too weak
- Missing numerical stability and error bound contracts
- No precision preservation guarantees

**Improvement Recommendation:**

```rust
fn construct_transformation_result(
    &self,
    coords: &[f64],
    from_crs: &(impl CrsDefinition + ?Sized),
    to_crs: &(impl CrsDefinition + ?Sized),
) -> BoxFuture<'_, GisResult<(
    Vec<f64>,
    Established<TransformationPathExists>,
    Established<TransformationNumericallyStable>,
    Established<TransformationErrorBounded>,
    Established<TransformationPrecisionPreserved>,
    Established<CoordinateRelationshipsMaintained>
)>>
```

## Overall Trait Architecture Issues

### Missing Contract Flow

- No explicit connection showing how `CrsValid` depends on `EllipsoidValid` + `DatumValid` + etc.
- Transformation methods don't require mathematically coherent inputs
- No composition laws expressed in trait bounds

### Insufficient Mathematical Rigor

- Methods don't establish quantitative properties (error bounds, convergence)
- No computational complexity or precision contracts
- Missing temporal coherence requirements for dynamic CRS

### Weak Return Type Design

- Simple `Established<T>` returns hide rich mathematical structure
- No way to compose transformation results with further operations
- Missing metadata about numerical precision of results
