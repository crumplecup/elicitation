# OGC SFS Gap Coverage Plan

## Current Coverage: ~8.5/10

## Target: 10/10 Complete Specification Concordance

## Section 5: Spatial Reference Systems - CRITICAL GAPS

### Missing Entire Section Coverage

**Source: OGC 06-103r4 §5 — Spatial reference systems**

**Required Contracts:**

- `SrsDefinitionComplete` - Every geometry SRS has complete definition (datum, projection, units)
- `SrsAuthorityRegistryValid` - SRS codes from recognized authorities (EPSG, OGC, etc.)
- `SrsTransformationPathExists` - Path exists between any two SRS for transformation
- `SrsDimensionalityMatchesGeometry` - SRS dimensionality equals geometry coordinate dimensionality
- `SrsMetadataCompleteness` - SRS includes datum, prime meridian, units, projection method
- `SrsVersionConsistency` - All SRS components from same authority version
- `SrsUnitsCompatibleWithOperations` - SRS units appropriate for metric operations

## Section 6.1.2: Precision Model - COMPLETELY MISSING

### Missing Mathematical Foundation

**Source: OGC 06-103r4 §6.1.2 — Precision model implications**

**Required Contracts:**

- `PrecisionModelDeclared` - Geometry carries explicit precision model declaration
- `CoordinateGridAlignment` - All coordinates align to precision model grid
- `ToleranceBasedEquality` - Geometric equality respects precision tolerance
- `SnappingWithinTolerance` - Coordinate snapping operations bounded by tolerance
- `PrecisionPreservedInOperations` - Set operations maintain input precision characteristics
- `FloatingPointErrorBounded` - Numerical operations error within precision model bounds

## Section 6.1.3: Validity Rules - EDGE CASE GAPS

### Missing Corner Cases

**Source: OGC 06-103r4 §6.1.3 — Detailed validity constraints**

**Required Contracts:**

- `PointNaNCoordinatesInvalid` - Points with NaN coordinates are invalid
- `PointInfiniteCoordinatesInvalid` - Points with infinite coordinates invalid
- `LineStringMinimumTwoDistinctPoints` - Adjacent equal points allowed, but total distinct ≥ 2
- `LineStringCollinearSegmentValid` - Collinear adjacent segments constitute valid LineString
- `PolygonShellOrientationCCW` - Exterior ring orientation explicitly CCW
- `PolygonHoleOrientationCW` - Interior ring orientation explicitly CW
- `PolygonRingOrientationEnforced` - Orientation rules strictly followed
- `PolygonZeroAreaValid` - Zero-area polygons validity semantics
- `PolygonSelfTangencyAtFinitePoints` - Self-tangency allowed only at finite point sets
- `MultiPolygonComponentSrsConsistent` - All components share identical SRS

## Section 6.1.4: Boundary Semantics - MATHEMATICAL RIGOR GAPS

### Missing Topological Foundations

**Source: OGC 06-103r4 §6.1.4 — Boundary definition**

**Required Contracts:**

- `BoundaryMod2CountingExact` - Precise mod-2 counting for GeometryCollection boundaries
- `BoundaryClosureIntersection` - bd(A) = cl(A) ∩ cl(complement(A)) mathematically valid
- `BoundaryDimensionReduction` - dim(bd(geometry)) ≤ dim(geometry) - 1
- `BoundaryOfManifoldValid` - Manifold boundary conditions satisfied
- `BoundaryEmptyForClosedManifolds` - Closed manifolds have empty boundary
- `BoundaryIdempotent` - bd(bd(A)) ⊆ bd(A) for well-behaved geometries

## Section 6.1.5: Per-Type Accessors - COMPLETENESS GAPS

### Missing Edge Case Handling

**Source: OGC 06-103r4 §6.1.5 — Accessor methods**

**Required Contracts:**

- `PointAccessorUndefinedForEmpty` - x(), y(), z(), m() undefined on empty Point
- `LineStringAccessorBoundsChecking` - pointN(n) throws for n ≥ numPoints()
- `PolygonRingAccessorBoundsChecking` - interiorRingN(n) throws for n ≥ numInteriorRings()
- `CollectionGeometryNBoundsChecking` - geometryN(n) throws for n ≥ numGeometries()
- `AccessorSrsPropagation` - All accessors preserve SRS context
- `AccessorPrecisionPreservation` - Accessors maintain precision model characteristics

## Section 6.1.6: Precision Model - SYSTEMATIC MISSING

### Missing Numerical Foundation

**Source: OGC 06-103r4 §6.1.6 — Precision model**

**Required Contracts:**

- `PrecisionModelScaleFactorValid` - Scale factor is positive power of 10 or 0
- `PrecisionModelGridAlignment` - All coordinates multiples of scale factor
- `PrecisionModelFloatingPointMode` - Floating point mode handles coordinates appropriately
- `PrecisionModelFixedModeGrid` - Fixed precision mode enforces grid alignment
- `PrecisionModelOperationsConsistent` - All operations respect current precision model
- `PrecisionModelChangeValid` - Precision model changes preserve geometric validity

## Section 7.2/7.3: Serialization - FORMAT INTEGRITY GAPS

### Missing Parsing and Validation Contracts

**Source: OGC 06-103r4 §7.2, §7.3 — WKT/WKB representation**

**Required Contracts:**

- `WktParseSyntaxTreeValid` - WKT parses to valid abstract syntax tree
- `WktGrammarCompliance` - WKT follows exact grammar production rules
- `WkbByteOrderConsistency` - All numeric fields use same byte order
- `WkbLengthFieldAccurate` - Length fields match actual geometry size
- `WkbTypeCodeConsistency` - Type codes match actual geometry structure
- `SerializationRoundTripExact` - Parse(Format(G)) = G with identical properties
- `WktWkbEquivalence` - WKT and WKB representations describe identical geometries
- `EmptyGeometrySerializationValid` - EMPTY keyword properly serialized/deserialized
- `CoordinateSerializationPrecision` - Serialized coordinates preserve required precision

## Section 8: SQL Geometry Types - IMPLEMENTATION GAP

### Missing SQL Mapping Contracts

**Source: OGC 06-103r4 §8 — SQL geometry types**

**Required Contracts:**

- `SqlGeometryTypeMappingCorrect` - OGC types map to correct SQL geometry types
- `SqlSrsColumnConstraint` - Geometry columns enforce SRS constraints
- `SqlDimensionColumnConstraint` - Geometry columns enforce dimensionality constraints
- `SqlIndexCompatibility` - Geometries compatible with spatial indexing requirements
- `SqlNullSemantics` - NULL geometry handling consistent with SQL standards
- `SqlGeometryCastValid` - Type casting operations preserve geometric validity

## Section 9: Conformance Testing - VERIFICATION GAP

### Missing Test Framework Contracts

**Source: OGC 06-103r4 §9 — Conformance testing**

**Required Contracts:**

- `ConformanceClass0RequirementsMet` - All CC0 mandatory features implemented
- `ConformanceClass1RequirementsMet` - All CC1 mandatory features implemented
- `ConformanceTestSuiteExecution` - Required test cases execute successfully
- `InteropTestPassing` - Cross-implementation compatibility verified
- `PerformanceBenchmarkMet` - Timing and resource usage within acceptable bounds
- `QualityMetricsAchieved` - Accuracy and robustness metrics satisfied

## Cross-Cutting Mathematical Properties - FOUNDATIONAL GAPS

### Missing Topological Invariants

**Source: Various sections — Mathematical consistency**

**Required Contracts:**

- `TopologyPreservedInSetOperations` - Union, intersection, difference preserve topology
- `MetricPropertiesInvariant` - Distance, area, length invariant under rigid motions
- `ContinuityOfSpatialPredicates` - Spatial predicates continuous under small perturbations
- `DimensionConservationLaws` - dim(A ∪ B) = max(dim(A), dim(B)) for well-behaved sets
- `ClosureAxiomsSatisfied` - Topological closure operations behave correctly
- `BoundaryAxiomsValid` - Boundary operations satisfy topological axioms

## Implementation Priority:

### Phase 1 (Weeks 1-3): Critical Foundation

- Section 5 SRS contracts
- Section 6.1.6 Precision model contracts
- Serialization integrity contracts
- Edge case validity contracts

### Phase 2 (Weeks 4-6): Mathematical Rigor

- Boundary semantics mathematical contracts
- Topological invariant contracts
- Metric properties contracts
- Numerical precision contracts

### Phase 3 (Weeks 7-8): Completeness and Verification

- SQL mapping contracts
- Conformance testing contracts
- Cross-cutting mathematical properties
- Final coverage audit and remediation

## Success Criteria:

- Every OGC 06-103r4 section has corresponding contracts
- Edge cases explicitly addressed with dedicated contracts
- Mathematical foundations fully captured
- Implementation guidance clear from contract names
- Ready for formal verification toolchain integration

# Contract Granularity Enhancement for Formal Verification

## Current State: 7/10 Formal Verification Readiness

Boolean predicates suitable for basic checking, but lacking quantitative precision

## Target: 10/10 Formal Verification Expressiveness

Mathematically precise contracts with explicit bounds, tolerances, and logical structure

## Phase 1: Quantitative Metric Contracts

### Current (Too Coarse):

```rust
pub struct DistanceNonNegative;
pub struct AreaNonNegative;
```

### Enhanced with Quantitative Precision:

```rust
pub struct DistanceSatisfiesMetricAxioms {
    symmetry: ∀g1,g2: distance(g1,g2) = distance(g2,g1),
    identity: ∀g1,g2: distance(g1,g2) = 0.0 ↔ intersects(g1,g2),
    triangle: ∀g1,g2,g3: distance(g1,g3) ≤ distance(g1,g2) + distance(g2,g3),
    non_negative: ∀g1,g2: distance(g1,g2) ≥ 0.0
}

pub struct AreaComputationBounded {
    lower_bound: ∀polygon: area(polygon) ≥ 0.0,
    upper_bound: ∀polygon,bbox: area(polygon) ≤ area(bbox),
    precision: ∀polygon: |area_computed - area_true| ≤ ε_area,
    monotonicity: ∀p1,p2: p1 ⊆ p2 → area(p1) ≤ area(p2)
}

pub struct LengthAdditivityProperty {
    segment_addition: ∀segments: length(∪segments) = Σlength(segment),
    precision_bound: ∀linestring: |length_computed - length_true| ≤ ε_length,
    srs_units: ∀linestring: length(linestring).unit = srs.linear_unit
}
```

## Phase 2: Topological Predicate Precision

### Current (Too Abstract):

```rust
pub struct EqualsDe9ImPattern;
```

### Enhanced with First-Order Logic:

```rust
pub struct EqualsTopologicallyEquivalent {
    interior_equality: interior(self) = interior(other),
    boundary_equality: boundary(self) = boundary(other),
    exterior_equality: exterior(self) = exterior(other),
    dimensional_preservation: dimension(self) = dimension(other),
    closure_preservation: closure(self) = closure(other)
}

pub struct IntersectsExistentialQuantification {
    point_existence: ∃p: p ∈ self ∧ p ∈ other,
    closure_intersection: closure(self) ∩ closure(other) ≠ ∅,
    epsilon_neighborhood: ∀ε>0: neighborhood_ε(self) ∩ neighborhood_ε(other) ≠ ∅
}
```

## Phase 3: Dimensional Analysis Contracts

### Current (Too Coarse):

```rust
pub struct GeometryDimension0ForPoint;
```

### Enhanced with Mathematical Rigor:

```rust
pub struct PointDimensionExactlyZero {
    topological_dimension: dim_top(point) = 0,
    hausdorff_dimension: dim_h(point) = 0,
    covering_dimension: dim_cov(point) = 0,
    manifold_dimension: dim_manifold(point) = 0
}

pub struct LineStringDimensionExactlyOne {
    topological_dimension: dim_top(linestring) = 1,
    hausdorff_dimension_for_simple: dim_h(simple_linestring) = 1,
    covering_dimension: dim_cov(linestring) = 1,
    manifold_dimension_where_smooth: dim_manifold(linestring°) = 1
}

pub struct PolygonDimensionExactlyTwo {
    topological_dimension: dim_top(polygon) = 2,
    hausdorff_dimension: dim_h(polygon) = 2,
    lebesgue_measure_positive: μ_lebesgue(polygon°) > 0,
    boundary_dimension: dim_top(boundary(polygon)) = 1
}
```

## Phase 4: Numerical Precision Contracts

### Current (Missing):

No explicit numerical analysis contracts

### New Quantitative Contracts:

```rust
pub struct FloatingPointErrorAnalysis {
    absolute_error_bound: |result_computed - result_exact| ≤ ε_abs,
    relative_error_bound: |result_computed - result_exact|/|result_exact| ≤ ε_rel,
    condition_number_bounded: κ(operation) ≤ κ_max,
    stability_metric: ∀δ_input: |output(perturbed) - output(original)| ≤ σ(δ_input)
}

pub struct CoordinatePrecisionSpecification {
    grid_alignment: ∀coord: coord = round(coord/scale) × scale,
    significant_digits: digits(coord) ≤ precision_limit,
    representation_stability: float64_to_float32(float32_to_float64(coord)) = coord,
    arithmetic_closure: ∀c1,c2: operation(c1,c2) satisfies precision_contract
}

pub struct IterativeAlgorithmConvergence {
    convergence_rate: |x_{n+1} - x_n| ≤ rate × |x_n - x_{n-1}|,
    termination_criterion: |residual| ≤ tolerance,
    maximum_iterations: iterations ≤ iter_max,
    monotonic_convergence: error_sequence.is_monotonic_decreasing()
}
```

## Phase 5: Temporal and Evolution Contracts

### Current (Missing):

No contracts for geometry evolution or temporal behavior

### New Temporal Contracts:

```rust
pub struct GeometryEvolutionContinuity {
    temporal_continuity: ∀t1,t2: |t2-t1| < δ → distance(geometry(t1), geometry(t2)) < ε,
    deformation_bounded: ∀t: |geometry'(t)| ≤ deformation_rate_max,
    topology_preservation: ∀t: is_valid(geometry(t)),
    smoothness_class: geometry(t) ∈ C^k_continuity
}

pub struct DynamicSrsTransformationConsistency {
    epoch_consistency: ∀t1,t2: |transform(epoch=t1) - transform(epoch=t2)| ≤ ε_temporal,
    velocity_model_integration: derivative(transform(epoch)) = velocity_field,
    reference_frame_coherence: transformations_compose_correctly(dynamic_frames)
}
```

## Phase 6: Set Operation Mathematical Contracts

### Current (Too Weak):

```rust
pub struct UnionIsCommutative;
```

### Enhanced with Measure Theory:

```rust
pub struct UnionMeasureTheoreticProperties {
    measure_subadditivity: measure(A ∪ B) ≤ measure(A) + measure(B),
    measure_additivity: A ∩ B = ∅ → measure(A ∪ B) = measure(A) + measure(B),
    monotonicity: A ⊆ B → measure(A) ≤ measure(B),
    continuity_from_below: A₁ ⊆ A₂ ⊆ ... → measure(∪Aᵢ) = lim measure(Aᵢ)
}

pub struct IntersectionGeometricConsistency {
    containment: intersection(A,B) ⊆ A ∧ intersection(A,B) ⊆ B,
    maximality: ∀C: C ⊆ A ∧ C ⊆ B → C ⊆ intersection(A,B),
    measure_monotonicity: measure(intersection(A,B)) ≤ min(measure(A), measure(B)),
    closure_property: closure(intersection(A,B)) ⊆ intersection(closure(A), closure(B))
}
```

## Phase 7: Boundary Operator Mathematical Rigor

### Current (Too Vague):

```rust
pub struct BoundaryOfBoundaryIsEmpty;
```

### Enhanced with Topological Axioms:

```rust
pub struct BoundaryOperatorTopologicalAxioms {
    boundary_of_boundary_empty: boundary(boundary(X)) = ∅,
    boundary_closed: is_closed(boundary(X)),
    boundary_subset: boundary(X) ⊆ X,
    boundary_idempotent: boundary(boundary(X)) = boundary(X) for some spaces,
    boundary_complement_relationship: boundary(X) = boundary(complement(X))
}

pub struct BoundaryDimensionReductionLaw {
    dimensional_drop: dim(boundary(n_manifold)) = n-1,
    hausdorff_dimension_bound: dim_H(boundary(set)) ≤ dim_H(set),
    topological_dimension_reduction: dim_T(boundary(regular_space)) ≤ dim_T(space) - 1
}
```

## Phase 8: Formal Verification Annotation Contracts

### New Meta-Contracts for Tool Integration:

```rust
pub struct VerifiableContractAnnotation {
    preconditions_specified: requires(precondition_predicate),
    postconditions_guaranteed: ensures(postcondition_predicate),
    invariants_maintained: invariant(invariant_predicate),
    loop_variants_defined: decreases(variant_expression),
    assertions_checkable: assert(assertion_predicate)
}

pub struct ProofConstructibility {
    lemma_availability: lemmas_available_for_composition,
    induction_principles_applicable: inductive_structure_present,
    decision_procedure_exists: decidable_in_theory(contract_domain),
    counterexample_generable: falsifiable_if_false
}
```

## Implementation Priority for Formal Verification:

### Phase 1 (Weeks 1-2): Quantitative Foundations

- Metric space contracts with explicit bounds
- Numerical precision and error analysis contracts
- Measure-theoretic properties of set operations

### Phase 2 (Weeks 3-4): Topological Rigor

- Boundary operator mathematical axioms
- Dimension theory contracts
- Topological continuity properties

### Phase 3 (Weeks 5-6): Temporal and Evolution

- Geometry evolution continuity contracts
- Dynamic SRS transformation consistency
- Stability and convergence specifications

### Phase 4 (Weeks 7-8): Verification Annotations

- Pre/post-condition contracts
- Invariant maintenance contracts
- Proof construction facilitators

## Success Metrics for Formal Verification Readiness:

- Contracts expressible in first-order logic with quantifiers
- Quantitative bounds explicit in contract names and parameters
- Mathematical foundations traceable to standard theorems
- Verification tool annotations present for automated proving
- Counterexamples generable for falsifiable contracts
- Composition principles clear for contract combination

# Detailed Critique of Current Geometry Trait System

## Overall Assessment: 6/10 for Contract-Based Correctness

Fundamentally designed as a post-hoc validation system rather than correctness-by-construction

## Major Weaknesses

### 1. **Validation-Oriented Instead of Construction-Oriented**

**Current Design Flaw:**

```rust
fn is_valid(&self) -> GisResult<bool>;  // Asking "is it valid?"
```

**Problem:** Allows invalid geometries to exist; discovers problems late

**Concrete Recommendation:**

```rust
// Replace with constructor-based approach:
fn construct_valid_geometry(
    params: GeometryParameters
) -> Result<(ValidGeometry, Established<GeometryValid>), ValidationError>;

// Or make validity part of the type system:
fn new_point(coordinates: ValidCoordinates) -> Point<Valid>;
```

### 2. **No Contract Preconditions on Method Inputs**

**Current Design Flaw:**

```rust
fn intersection(&self, other: &dyn SfsGeometry) -> GisResult<Box<dyn SfsGeometry>>;
// Accepts ANY geometry, validity unknown
```

**Concrete Recommendation:**

```rust
fn intersection<G1, G2>(
    &self,
    other: &G2
) -> GisResult<(Box<dyn SfsGeometry>, Established<IntersectionPreservesValidity>)>
where
    G1: SfsGeometry + Has<Established<GeometryValid>>,
    G2: SfsGeometry + Has<Established<GeometryValid>>,
    Self: SfsGeometry + Has<Established<GeometryValid>>;
```

### 3. **Contract Information Loss in Returns**

**Current Design Flaw:**

```rust
fn buffer(&self, distance: f64) -> GisResult<Box<dyn SfsGeometry>>;
// Returned geometry has no established properties
```

**Concrete Recommendation:**

```rust
fn buffer(&self, distance: f64) -> GisResult<(
    Box<dyn SfsGeometry>,
    Established<BufferOperationContinuous>,
    Established<BufferPreservesSrs>,
    Established<BufferResultValid>
)>;
```

### 4. **No Mathematical Composition Laws**

**Current Design Flaw:** No way to express that valid inputs produce valid outputs

**Concrete Recommendation:**

```rust
// Add trait expressing mathematical composition:
trait ValidityPreservingOperation {
    type Input: Has<Established<GeometryValid>>;
    type Output: Has<Established<GeometryValid>>;

    fn perform_operation(&self, input: &Self::Input) -> Self::Output;
}

// Implementation example:
impl ValidityPreservingOperation for IntersectionOp {
    type Input = dyn (SfsGeometry + Has<Established<GeometryValid>>);
    type Output = dyn (SfsGeometry + Has<Established<GeometryValid>>);

    fn perform_operation(&self, input: &Self::Input) -> Self::Output {
        // Mathematical guarantee: valid inputs produce valid output
    }
}
```

## Specific Contract System Failures

### 5. **No Prevention of Invalid State Creation**

**Current Design Flaw:**

```rust
// Anyone can implement SfsGeometry with invalid data:
struct BogusGeometry { /* arbitrary invalid data */ }
impl SfsGeometry for BogusGeometry { /* ... */ }
```

**Concrete Recommendation:**

```rust
// Make geometry construction controlled:
pub struct Point<C> where C: CoordinateConstraints {
    coordinates: Coordinates<C>,
    _validity_marker: PhantomData<C>
}

// Only valid coordinates can be used:
pub struct ValidCoordinates;
pub struct InvalidCoordinates;  // Compiler prevents usage

// Factory functions that establish contracts:
impl Point<ValidCoordinates> {
    pub fn new(x: f64, y: f64) -> Result<Self, InvalidCoordinateError> {
        // Establishes ValidCoordinates contract at construction
    }
}
```

### 6. **Trait Segmentation Breaks Contract Flow**

**Current Design Flaw:**

```rust
// Split across multiple traits breaks contract relationships:
SfsGeometry  // Basic properties
SfsSetOps    // Set operations
SfsTopology  // Spatial predicates
```

**Concrete Recommendation:**

```rust
// Unified contract-based interface:
pub trait GeometricObject:
    IntrinsicProperties +
    TopologicalOperations +
    ConstructiveOperations +
    SpatialPredicates
{
    // All properties and operations with explicit contracts
    // Mathematical relationships between them are clear
}
```

### 7. **Weak Type Safety with &dyn SfsGeometry**

**Current Design Flaw:**

```rust
fn boundary(&self) -> GisResult<Option<Box<dyn SfsGeometry>>>;
// Completely erases all type and contract information
```

**Concrete Recommendation:**

```rust
fn boundary(&self) -> GisResult<Option<Box<dyn SfsGeometry<BoundaryContract>>>>;
// Or better yet, preserve specific boundary types:
fn boundary(&self) -> GisResult<BoundaryType<Self>>;
```

## Error Handling Obscures Contracts

### 8. **GisResult Obscures Contract Violations**

**Current Design Flaw:**

```rust
fn is_simple(&self) -> GisResult<bool>;
// Can't distinguish contract violation from system error
```

**Concrete Recommendation:**

```rust
// Separate contract violations from system errors:
fn check_simple(&self) -> Result<bool, ContractViolation>;
fn system_operation(&self) -> Result<(), SystemError>;

// Or use the type system to prevent contract violations:
fn ensure_simple(&self) -> Result<SimpleGeometry<Self>, InvalidGeometryError>;
```

## Concrete Architectural Recommendations

### Phase 1: Contract-First Redesign (Weeks 1-2)

1. **Eliminate post-hoc validation methods** - Remove `is_valid()`, `is_simple()`
2. **Add constructor traits** - `GeometryConstructor`, `ValidGeometryFactory`
3. **Introduce contract markers** - `Established<T>`, `Has<T>` type system

### Phase 2: Mathematical Composition (Weeks 3-4)

1. **Add validity-preserving operation traits**
2. **Implement contract propagation through operations**
3. **Establish mathematical composition laws**

### Phase 3: Type Safety Enhancement (Weeks 5-6)

1. **Replace `&dyn` with constrained generics**
2. **Add phantom types for contract enforcement**
3. **Implement factory patterns for controlled construction**

## Success Metrics for Improved System

### Before Fix:

- Can create invalid geometries
- Must check validity after creation
- Operations may fail due to invalid inputs
- No compile-time contract guarantees

### After Fix:

- Invalid geometries unrepresentable in type system
- Operations compose contracts mathematically
- Compile-time prevention of contract violations
- Explicit contract flow through all operations
- Mathematical relationships between contracts clear

## Specific Code Changes Needed

### In geometry.rs:

```rust
// REMOVE:
fn is_valid(&self) -> GisResult<bool>;
fn is_simple(&self) -> GisResult<bool>;

// ADD:
fn construct_with_validity_guarantee(
    params: ValidatedParameters
) -> Result<(Self, Established<GeometryValid>), ConstructionError>;

fn ensure_contract_compliance<C: GeometryContract>(
    &self
) -> Result<Established<C>, ContractViolation>;
```

### In set_ops.rs:

```rust
// REPLACE:
fn intersection(&self, other: &dyn SfsGeometry) -> GisResult<Box<dyn SfsGeometry>>;

// WITH:
fn intersection_valid_geometries<G>(
    &self,
    other: &G
) -> Result<(
    Box<dyn SfsGeometry>,
    Established<IntersectionPreservesValidity>,
    Established<IntersectionDimensionBounded>
), IntersectionError>
where
    G: SfsGeometry + Has<Established<GeometryValid>>;
```

This approach transforms the system from "check if valid" to "cannot be invalid" - true correctness by construction.
