# elicit_gis

Formally verified geospatial data management with compile-time contract
enforcement, built on a proof-carrying trait interface anchored to ISO 19111,
ISO 19115-1, OGC Simple Features, IETF RFC 7946, and FGDC-STD-001.

## Overview

`elicit_gis` models geospatial data construction as a proof-carrying pipeline.
Every geometry, CRS, and metadata object is built through a factory or
validator trait that performs a standards-compliance check and, on success,
returns both the constructed descriptor **and** a typed proof token —
an `Established<P>` — that records which contract was satisfied. Proof tokens
compose upward through evidence bundles into aggregate proofs (`CrsValid`,
`MdMetadataValid`, `FgdcRecordValid`), which are the only legal way to assert
compound geospatial data validity.

The compiler enforces this chain. There is no way to produce
`Established<MdMetadataValid>` without assembling the citation, extent, and
contact proofs from which it is derived.

This is an **interface crate**, not an implementation. Geospatial drivers
(`elicit_geo`, `elicit_proj`, `elicit_geojson`) implement the traits;
consumers depend only on this crate.

---

## Architecture

```text
  ┌─────────────────────────────────────────────────────────────────────────┐
  │                   CRS Traits (ISO 19111)                                 │
  │  GisCrsLookup · GisCrsBuilder · GisCrsTransformer                       │
  │  Iso19111Identified · Iso19111Scoped                                     │
  └────────────────────────────┬─────────────────────────────────────────────┘
                               │ build_crs → (CrsInfo, Established<CrsValid>)
                               ▼
  ┌─────────────────────────────────────────────────────────────────────────┐
  │                Metadata Traits (ISO 19115-1)                             │
  │  Iso19115CitationFactory · Iso19115ExtentFactory                         │
  │  Iso19115LineageFactory  · Iso19115RecordFactory                         │
  │  Iso19115ContactMeta · Iso19115DateMeta · Iso19115QualityMeta            │
  └────────────────────────────┬─────────────────────────────────────────────┘
                               │ build_metadata → (MetadataDescriptor, Established<MdMetadataValid>)
                               ▼
  ┌─────────────────────────────────────────────────────────────────────────┐
  │              Geometry Traits (OGC SFS + RFC 7946)                        │
  │  SfsGeometryFactory · SfsGeometryMeta · SfsGeometryIo                   │
  │  SfsTopology · SfsSetOps                                                 │
  │  GeoJsonGeometryFactory · GeoJsonFeatureFactory                          │
  │  GeoJsonObjectMeta · GeoJsonFeatureMeta                                  │
  └────────────────────────────┬─────────────────────────────────────────────┘
                               │ geometry proofs compose via Evidence bundles
                               ▼
  ┌─────────────────────────────────────────────────────────────────────────┐
  │              FGDC Metadata Traits (FGDC-STD-001-1998)                   │
  │  26 validator + factory + meta traits                                    │
  │  FgdcRecordFactory → (FgdcRecordDescriptor, Established<FgdcRecordValid>)│
  └────────────────────────────┬─────────────────────────────────────────────┘
                               │
               ┌───────────────┼───────────────┐
               ▼               ▼               ▼
         elicit_geo       elicit_proj    elicit_geojson
       (geo + proj4rs)   (PROJ bindings) (geojson crate)
```

### Domain partitioning

| Domain | Traits | Role |
|--------|--------|------|
| **CRS lookup** | `GisCrsLookup`, `Iso19111Identified`, `Iso19111Scoped` | Reporter — queries catalog; no proof tokens produced |
| **CRS construction** | `GisCrsBuilder`, `GisCrsTransformer` | Factory — builds CRS objects; returns `Established<CrsValid>` |
| **ISO 19115 metadata** | `Iso19115CitationFactory`, `Iso19115ExtentFactory`, `Iso19115LineageFactory`, `Iso19115RecordFactory` | Factory — builds metadata objects; returns typed proof tokens |
| **ISO 19115 reporters** | `Iso19115ContactMeta`, `Iso19115DateMeta`, `Iso19115QualityMeta` | Reporter — reads metadata fields; no proof tokens |
| **SFS geometry** | `SfsGeometryFactory` | Factory — builds geometries; returns `Established<{Type}Valid>` |
| **SFS predicates** | `SfsGeometryMeta`, `SfsTopology`, `SfsSetOps` | Predicate — tests geometric relationships; no proof tokens |
| **SFS I/O** | `SfsGeometryIo` | Reporter — serializes geometry to WKT/WKB |
| **GeoJSON** | `GeoJsonGeometryFactory`, `GeoJsonFeatureFactory` | Factory — builds GeoJSON objects; returns proof tokens |
| **GeoJSON reporters** | `GeoJsonObjectMeta`, `GeoJsonFeatureMeta` | Reporter — inspects GeoJSON properties |
| **FGDC validators** | `FgdcBoundingValidator` + 9 more | Validator — checks element constraints; returns `Established<P>` |
| **FGDC factories** | `FgdcCitationFactory` + 10 more | Factory — builds FGDC sections with proof tokens |
| **FGDC reporters** | `FgdcBoundingMeta` + 4 more | Reporter — reads FGDC field values |

---

## Proof Architecture

### Proposition types

Every verifiable geospatial contract has a corresponding Rust type — a
*proposition* — that implements `elicitation::contracts::Prop`. These types
are zero-cost phantoms that exist only at the type level.

```rust
pub struct CrsValid;                 // CRS satisfies ISO 19111:2019 §10
pub struct PolygonValid;             // Polygon satisfies OGC SFS §2.1.4
pub struct PositionValid;            // RFC 7946 §3.1.1 position constraints met
pub struct MdMetadataValid;          // Mandatory ISO 19115-1 elements present
pub struct FgdcRecordValid;          // Mandatory FGDC-STD-001 sections present
```

### Proof tokens

`Established<P>` is the proof that proposition `P` holds. It is a zero-sized
type that carries no runtime data — only type-level evidence.

### The `ProvableFrom<C>` evidence path

The evidence-bundle minting path is `Established::prove`:

```rust
impl Established<P> {
    pub fn prove<C>(_: &C) -> Self  where P: ProvableFrom<C> { … }
}
```

`ProvableFrom<C>` declares "evidence bundle `C` proves proposition `P`". Each
domain's contracts module defines the evidence bundles and their `ProvableFrom`
impls. For example:

```rust
pub struct GeodeticCrsEvidence {
    pub frame: Established<GeodeticReferenceFrameValid>,  // §9.4
    pub cs:    Established<CoordinateSystemValid>,        // §9.9
}
impl ProvableFrom<GeodeticCrsEvidence> for CrsValid {}
```

---

## Proof Composition

Proofs compose bottom-up. The compiler rejects any gap in the chain.

```rust
// 1. Leaf proofs — validator/factory methods return Established<LeafProp>
let (_, ellipsoid_proof)  = backend.build_ellipsoid(ellipsoid_params)?;
let (_, meridian_proof)   = backend.build_prime_meridian(meridian_params)?;
let (_, cs_proof)         = backend.build_coordinate_system(cs_params)?;

// 2. Assemble frame evidence — both fields are required
let frame_evidence = GeodeticFrameEvidence {
    ellipsoid:      ellipsoid_proof,
    prime_meridian: meridian_proof,
};
let frame_proof = Established::prove(&frame_evidence);
// frame_proof: Established<GeodeticReferenceFrameValid>

// 3. Assemble CRS evidence
let crs_evidence = GeodeticCrsEvidence {
    frame: frame_proof,
    cs:    cs_proof,
};
let crs_proof: Established<CrsValid> = Established::prove(&crs_evidence);

// 4. A proven CRS also proves coordinate metadata valid (single-token upcast)
let meta_proof: Established<CoordinateMetadataValid> =
    Established::prove(&crs_proof);
```

The same pattern applies to geometries:

```rust
// OGC SFS polygon: ring validity → polygon validity
let ring_proof: Established<LinearRingValid> = backend.build_linear_ring(coords)?;
let poly_proof: Established<PolygonValid> = Established::prove(&PolygonEvidence {
    exterior: ring_proof,
    holes:    vec![],
});

// RFC 7946 GeoJSON: position validity → geometry validity
let pos_proof: Established<PositionValid> = backend.validate_position(pos)?;
let point_proof: Established<GeoJsonPointValid> = Established::prove(&pos_proof);
```

And to metadata:

```rust
// ISO 19115-1: citation + extent → identification → metadata
let (_, citation_proof) = backend.build_citation(citation_desc)?;
let (_, extent_proof)   = backend.build_extent(extent_desc)?;
let id_proof = Established::prove(&MdIdentificationEvidence {
    citation: citation_proof,
    extents:  vec![extent_proof],
});
let (_, contact_proof) = backend.build_responsibility(contact_desc)?;
let metadata_proof: Established<MdMetadataValid> = Established::prove(&MdMetadataEvidence {
    contact:        contact_proof,
    identification: id_proof,
});
```

---

## Trait Interface

### CRS traits (ISO 19111:2019)

```rust
pub trait GisCrsLookup: Send + Sync {
    fn lookup_crs(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<CrsInfo>>;
    fn list_crs(&self, crs_type: CrsType) -> BoxFuture<'_, GisResult<Vec<CrsInfo>>>;
    fn resolve_datum_ensemble(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<DatumEnsembleInfo>>;
    fn is_dynamic_crs(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<bool>>;
}

pub trait GisCrsBuilder: Send + Sync {
    fn build_ellipsoid(&self, params: EllipsoidParams) -> GisResult<(EllipsoidParams, Established<EllipsoidValid>)>;
    fn build_geodetic_frame(&self, params: GeodeticFrameParams) -> GisResult<(GeodeticFrameParams, Established<GeodeticReferenceFrameValid>)>;
    fn build_crs(&self, evidence: GeodeticCrsEvidence) -> GisResult<(CrsInfo, Established<CrsValid>)>;
    fn build_projected_crs(&self, evidence: ProjectedCrsEvidence) -> GisResult<(CrsInfo, Established<CrsValid>)>;
    fn build_compound_crs(&self, evidence: CompoundCrsEvidence) -> GisResult<(CrsInfo, Established<CrsValid>)>;
    // … build_prime_meridian, build_coordinate_system, build_coordinate_metadata
}

pub trait GisCrsTransformer: Send + Sync {
    fn transform(&self, geom: &dyn SfsGeometryMeta, target: &AuthorityCode) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn transform_with_epoch(&self, geom: &dyn SfsGeometryMeta, target: &AuthorityCode, epoch: DecimalYear) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn normalize_axis_order(&self, geom: &dyn SfsGeometryMeta, code: &AuthorityCode) -> GisResult<Box<dyn SfsGeometryMeta>>;
}
```

### ISO 19111 object model traits

```rust
pub trait Iso19111Identified: Send + Sync {
    fn primary_name(&self) -> BoxFuture<'_, GisResult<String>>;
    fn aliases(&self) -> BoxFuture<'_, GisResult<Vec<Option<String>>>>;
    fn identifiers(&self) -> BoxFuture<'_, GisResult<Vec<AuthorityCode>>>;
    fn identifier_complete(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<bool>>;
    // …
}

pub trait Iso19111Scoped: Send + Sync {
    fn scope(&self) -> BoxFuture<'_, GisResult<Option<String>>>;
    fn domain_of_validity(&self) -> BoxFuture<'_, GisResult<Option<DomainExtent>>>;
    fn domain_extent_types(&self) -> BoxFuture<'_, GisResult<Vec<String>>>;
}
```

### ISO 19115-1 metadata traits

```rust
pub trait Iso19115CitationFactory: Send + Sync {
    fn build_citation(&self, desc: CitationDescriptor) -> GisResult<(CitationDescriptor, Established<CiCitationValid>)>;
    fn build_responsibility(&self, desc: ResponsibilityDescriptor) -> GisResult<(ResponsibilityDescriptor, Established<CiResponsibilityValid>)>;
    fn validate_isbn(&self, isbn: &str) -> GisResult<Established<IsbnValid>>;
    fn validate_issn(&self, issn: &str) -> GisResult<Established<IssnValid>>;
}

pub trait Iso19115ExtentFactory: Send + Sync {
    fn build_geographic_bbox(&self, desc: GeographicBboxDescriptor) -> GisResult<(GeographicBboxDescriptor, Established<ExGeographicBoundingBoxValid>)>;
    fn build_extent(&self, evidence: ExExtentEvidence) -> GisResult<(ExtentDescriptor, Established<ExExtentValid>)>;
}

pub trait Iso19115RecordFactory: Send + Sync {
    fn build_identification(&self, evidence: MdIdentificationEvidence) -> GisResult<(IdentificationDescriptor, Established<MdIdentificationValid>)>;
    fn build_metadata(&self, evidence: MdMetadataEvidence) -> GisResult<(MetadataDescriptor, Established<MdMetadataValid>)>;
}
```

### OGC SFS geometry traits

```rust
pub trait SfsGeometryFactory: Send + Sync {
    fn build_point(&self, x: f64, y: f64, srid: Option<i32>) -> (PointDescriptor, Established<PointValid>);
    fn build_line_string(&self, coords: Vec<SfsCoordinate>, srid: Option<i32>) -> GisResult<(LineStringDescriptor, Established<LineStringValid>)>;
    fn build_linear_ring(&self, coords: Vec<SfsCoordinate>, srid: Option<i32>) -> GisResult<(LinearRingDescriptor, Established<LinearRingValid>)>;
    fn build_polygon(&self, evidence: PolygonEvidence, srid: Option<i32>) -> GisResult<(PolygonDescriptor, Established<PolygonValid>)>;
    fn build_multi_polygon(&self, evidence: MultiPolygonEvidence, srid: Option<i32>) -> GisResult<(MultiGeometryDescriptor, Established<MultiPolygonValid>)>;
    fn geometry_from_wkt(&self, wkt: &str) -> GisResult<Box<dyn SfsGeometryMeta>>;
    // … build_point_3d, build_multi_point, build_multi_line_string, build_geometry_collection, geometry_from_wkb
}

pub trait SfsGeometryMeta: Send + Sync {
    fn geometry_type(&self) -> &str;
    fn srid(&self) -> i32;
    fn dimension(&self) -> i32;
    fn is_empty(&self) -> bool;
    fn is_simple(&self) -> GisResult<bool>;
    fn is_valid(&self) -> GisResult<bool>;
    fn envelope(&self) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn boundary(&self) -> GisResult<Option<Box<dyn SfsGeometryMeta>>>;
    // … coord_dimension, is_3d, is_measured
}

pub trait SfsTopology: SfsGeometryMeta {
    fn equals(&self, other: &dyn SfsGeometryMeta) -> GisResult<bool>;
    fn disjoint(&self, other: &dyn SfsGeometryMeta) -> GisResult<bool>;
    fn intersects(&self, other: &dyn SfsGeometryMeta) -> GisResult<bool>;
    fn within(&self, other: &dyn SfsGeometryMeta) -> GisResult<bool>;
    fn contains(&self, other: &dyn SfsGeometryMeta) -> GisResult<bool>;
    fn overlaps(&self, other: &dyn SfsGeometryMeta) -> GisResult<bool>;
    fn relate(&self, other: &dyn SfsGeometryMeta, pattern: &str) -> GisResult<bool>;
    fn area(&self) -> GisResult<f64>;
    fn distance(&self, other: &dyn SfsGeometryMeta) -> GisResult<f64>;
    fn centroid(&self) -> GisResult<Box<dyn SfsGeometryMeta>>;
    // … touches, crosses, covers, covered_by, length, point_on_surface
}

pub trait SfsSetOps: SfsGeometryMeta {
    fn buffer(&self, distance: f64) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn convex_hull(&self) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn intersection(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn union(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn difference(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;
    fn sym_difference(&self, other: &dyn SfsGeometryMeta) -> GisResult<Box<dyn SfsGeometryMeta>>;
}
```

### RFC 7946 GeoJSON traits

```rust
pub trait GeoJsonGeometryFactory: Send + Sync {
    fn validate_position(&self, pos: GeoJsonPosition) -> GisResult<(GeoJsonPosition, Established<PositionValid>)>;
    fn build_geojson_point(&self, pos: Established<PositionValid>) -> (GeoJsonGeometryDescriptor, Established<GeoJsonPointValid>);
    fn build_geojson_polygon(&self, evidence: GeoJsonPolygonEvidence) -> GisResult<(GeoJsonGeometryDescriptor, Established<GeoJsonPolygonValid>)>;
    fn build_geojson_geometry_collection(&self, evidence: GeoJsonGeometryCollectionEvidence) -> GisResult<(GeoJsonGeometryDescriptor, Established<GeoJsonGeometryCollectionValid>)>;
    fn geometry_from_geojson_str(&self, json: &str) -> GisResult<Box<dyn GeoJsonObjectMeta>>;
    // … build_geojson_multi_point, build_geojson_line_string, build_geojson_multi_line_string,
    //   build_geojson_multi_polygon
}

pub trait GeoJsonFeatureFactory: Send + Sync {
    fn build_geojson_feature(&self, desc: GeoJsonFeatureDescriptor) -> GisResult<(GeoJsonFeatureDescriptor, Established<GeoJsonFeatureValid>)>;
    fn build_geojson_feature_collection(&self, evidence: GeoJsonFeatureCollectionEvidence) -> GisResult<(GeoJsonFeatureCollectionDescriptor, Established<GeoJsonFeatureCollectionValid>)>;
    fn document_from_geojson_str(&self, json: &str) -> GisResult<GeoJsonDocumentDescriptor>;
}
```

### `GisBackend` supertrait

```rust
pub trait GisBackend:
    GisCrsLookup + GisCrsBuilder + GisCrsTransformer
    + Iso19111Identified + Iso19111Scoped
    + Iso19115CitationFactory + Iso19115ContactMeta + Iso19115DateMeta
    + Iso19115ExtentFactory + Iso19115LineageFactory + Iso19115QualityMeta
    + Iso19115RecordFactory
    + GeoJsonGeometryFactory + GeoJsonFeatureFactory
    + GeoJsonObjectMeta + GeoJsonFeatureMeta
    + SfsGeometryFactory + SfsGeometryMeta + SfsGeometryIo
    + SfsTopology + SfsSetOps
    + FgdcBackend
    + Send + Sync
{}
```

`GisBackend` is not itself object-safe (it is a supertrait of 30+ traits), but
each sub-trait is individually object-safe. Use `dyn SfsGeometryFactory`,
`dyn GisCrsLookup`, etc. for dynamic dispatch at architectural boundaries.

---

## Contract Module Reference (1,354 propositions)

### `contracts::iso_19111` — ISO 19111:2019 (265 propositions)

Covers the complete CRS object model: identified objects, coordinate systems,
reference frames, datum ensembles, and coordinate operations.

| Group | Representative propositions |
|-------|----------------------------|
| Core object model | `CrsConsistsOfCsAndDatum`, `CoordinateTupleDimensionMatchesAxes`, `ScCrsNameNonEmpty`, `ScCrsScopeNonEmpty` |
| Authority / identifiers | `ScCrsIdentifierHasAuthorityAndCode`, `CrsIdentifierAuthorityNonEmpty`, `CrsIdentifierCodeNonEmpty` |
| Geodetic CRS | `GeodeticCrsDatumIsGeodeticReferenceFrame`, `GeodeticCrsCsIsEllipsoidalOrCartesian`, `Geographic2dCrsHasTwoAxes`, `Geographic3dCrsHasThreeAxes` |
| Well-known EPSG codes | `Epsg4326AxisOrderLatFirst`, `Epsg4326LatitudeRangeValid`, `Epsg4979IsWgs84Geographic3d`, `Epsg4978IsWgs84Geocentric` |
| Reference frame | `GeodeticReferenceFrameNameNonEmpty`, `GeodeticReferenceFrameHasExactlyOneEllipsoid`, `GeodeticReferenceFrameRealizationEpochIsIso8601` |
| Ellipsoid / meridian | `EllipsoidSemiMajorAxisPositive`, `EllipsoidInverseFlatteringPositive`, `PrimeMeridianGreenwichIsZero` |
| Coordinate system | `CoordinateSystemHasAtLeastOneAxis`, `AxisDirectionMemberOfCodeList`, `CsTypeMemberOfDefinedTypes` |
| Datum ensemble | `DatumEnsembleHasAtLeastTwoMembers`, `DatumEnsembleAccuracyPositive`, `DatumEnsembleWgs84EpsgCode6326` |
| Coordinate metadata | `CoordinateMetadataHasCrs`, `CoordinateMetadataDynamicCrsRequiresEpoch`, `CoordinateEpochDistinctFromFrameReferenceEpoch` |
| Coordinate operations | `PassThroughOperationPreservesSomeAxes`, `PassThroughOperationIndexInRange`, `MapProjectionScaleFactorPositive` |
| Aggregate proofs | `EllipsoidValid`, `PrimeMeridianValid`, `CoordinateSystemValid`, `GeodeticReferenceFrameValid`, `CrsValid`, `CoordinateMetadataValid` |

### `contracts::iso_19115` — ISO 19115-1:2014 (440 propositions)

Covers the complete metadata object model: mandatory elements, conditional
obligations, cardinality constraints, code list values, and domain rules.

| Group | Representative propositions |
|-------|----------------------------|
| MD_Metadata root | `MdMetadataContactMandatory`, `MdMetadataDateInfoMandatory`, `MdMetadataIdentificationInfoMandatory`, `MdMetadataHierarchyLevelScopeCode` |
| CI_Citation | `CiCitationTitleMandatory`, `CiCitationTitleNonEmpty`, `CiCitationDateMandatory`, `CiCitationIdentifierOptional` |
| CI_Responsibility | `CiResponsibilityRoleMandatory`, `CiResponsibilityRoleCodeValid`, `CiResponsibilityPartyMandatory` |
| CI_Date | `CiDateDateMandatory`, `CiDateDateTypeCodeMandatory`, `CiDateDateTypeCodeValid`, `CiDateValueConformsToIso8601` |
| EX_Extent | `ExGeographicBoundingBoxWestBoundLongitudeInRange`, `ExGeographicBoundingBoxNorthGeqSouth`, `ExVerticalExtentMinimumValueValid` |
| MD_Identification | `MdDataIdentificationLanguageMandatory`, `MdDataIdentificationTopicCategoryMandatory`, `MdDataIdentificationTopicCategoryCodeValid` |
| MD_Distribution | `MdDistributionDistributorOptional`, `MdFormatNameMandatory`, `MdDigitalTransferOptionsMediumNameCodeValid` |
| MD_DataQuality | `DqDataQualityScopeHierarchyLevelMandatory`, `LiLineageStatementOptional`, `LiSourceScaleDenominatorPositive` |
| MD_Constraints | `MdConstraintsUseLimitationOptional`, `MdSecurityConstraintsClassificationMandatory` |
| Spatial representation | `MdVectorSpatialRepresentationTopologyLevelCodeValid`, `MdGridSpatialRepresentationNumberOfDimensionsMandatory` |
| Aggregate proofs | `CiCitationValid`, `CiResponsibilityValid`, `ExGeographicBoundingBoxValid`, `ExExtentValid`, `MdIdentificationValid`, `MdMetadataValid` |

### `contracts::ogc_sfs` — OGC Simple Features Access Part 1 (278 propositions)

Covers the geometry object model: type invariants, dimension rules, boundary
definitions, WKT/WKB requirements, and the DE-9IM topological relations.

| Group | Representative propositions |
|-------|----------------------------|
| Geometry core | `GeometryHasSrs`, `GeometrySridReturnsInteger`, `GeometryIsEmptyPredicate`, `GeometryIsSimplePredicate`, `GeometryIsValidPredicate` |
| Dimension | `GeometryDimension0ForPoint`, `GeometryDimension1ForLine`, `GeometryDimension2ForSurface` |
| Envelope / boundary | `EnvelopeIsPolygon`, `EnvelopeIsPointWhenDegenerate`, `GeometryBoundaryDefinedPerType` |
| Point | `PointAlwaysValid`, `PointXIsFinite`, `PointYIsFinite`, `PointZIsFiniteWhenPresent`, `PointEmptyIsEmpty` |
| LineString | `LineStringHasTwoOrMorePoints`, `LineStringAdjacentPointsDistinct`, `LineStringSimpleNoSelfIntersection`, `LineStringClosedEqualsLinearRing` |
| LinearRing | `LinearRingIsClosedLineString`, `LinearRingMinimumFourPositions`, `LinearRingIsSimple`, `LinearRingFirstPositionEqualsLast` |
| Polygon | `PolygonExteriorIsLinearRing`, `PolygonExteriorIsCCW`, `PolygonHolesAreCW`, `PolygonHolesInsideExterior` |
| Multi-geometries | `MultiPointPointsHaveSameDimension`, `MultiLineStringAllComponentsLineStrings`, `MultiPolygonComponentsDontOverlap` |
| DE-9IM topology | `EqualGeometriesSameType`, `DisjointIntersectionEmpty`, `ContainsInteriorSharesInterior`, `OverlapsCompatibleDimension` |
| WKT/WKB I/O | `AsTextReturnsWkt`, `AsBinaryReturnsWkb`, `WktRoundtrip`, `WkbRoundtrip` |
| Aggregate proofs | `PointValid`, `LineStringValid`, `LinearRingValid`, `PolygonValid`, `MultiPointValid`, `MultiLineStringValid`, `MultiPolygonValid`, `GeometryCollectionValid`, `SfsGeometryValid` |

### `contracts::rfc7946` — IETF RFC 7946 (GeoJSON) (211 propositions)

Covers the full GeoJSON specification: type rules, position constraints,
geometry validity, feature structure, bounding box, and coordinate reference.

| Group | Representative propositions |
|-------|----------------------------|
| Object structure | `GeoJsonRootIsObject`, `GeoJsonObjectHasTypeMember`, `GeoJsonTypeIsCaseSensitive`, `GeoJsonTypeIsOneOfNineValues` |
| Position | `PositionHasAtLeastTwoElements`, `PositionElementZeroIsLongitude`, `PositionElementOneIsLatitude`, `PositionLongitudeInRange`, `PositionLatitudeInRange` |
| Geometry types | `PointPositionExactlyOne`, `LineStringMinimumTwoPositions`, `PolygonRingsClosedAndCounterClockwise`, `LinearRingMinimumFourPositions` |
| Polygon rules | `PolygonExteriorCounterClockwise`, `PolygonHolesClockwise`, `PolygonHoleInsideExterior`, `PolygonNoSelfIntersection` |
| Multi-geometries | `MultiPointPositionsNotEmpty`, `MultiLineStringLinesNotEmpty`, `MultiPolygonNonOverlapping` |
| GeometryCollection | `GeometryCollectionMembersNotEmpty`, `GeometryCollectionNoNesting` |
| Feature | `GeoJsonFeatureGeometryNullAllowed`, `GeoJsonFeaturePropertiesNullAllowed`, `GeoJsonFeatureIdStringOrNumber` |
| FeatureCollection | `GeoJsonFeatureCollectionFeaturesArray`, `GeoJsonFeatureCollectionNoNesting` |
| Bounding box | `GeoJsonBboxMemberIsOptional`, `GeoJsonBboxWhenPresentIsArray`, `GeoJsonBboxMinimumFourElements` |
| CRS / coordinate rule | `GeoJsonCrsIsWgs84`, `GeoJsonAltitudeUnspecifiedUnit`, `GeoJsonForeignMembersShouldBeIgnored` |
| Aggregate proofs | `PositionValid`, `GeoJsonPointValid`, `GeoJsonLineStringValid`, `GeoJsonPolygonValid`, `GeoJsonMultiPolygonValid`, `GeoJsonGeometryValid`, `GeoJsonFeatureValid`, `GeoJsonFeatureCollectionValid` |

### `contracts::fgdc` — FGDC-STD-001-1998 (160 propositions)

Covers the FGDC Content Standard for Digital Geospatial Metadata: section
structure, field constraints, code value validation, and coordinate rules.

| Group | Representative propositions |
|-------|----------------------------|
| Record structure | `FgdcMetadataHasIdentificationSection`, `FgdcMetadataHasMetadataReferenceSection` |
| Citation | `FgdcCitationHasAtLeastOneOriginator`, `FgdcCitationPublicationDateIsYyyymmddOrToken`, `FgdcCitationTitleNonEmpty` |
| Description | `FgdcDescriptionAbstractPresent`, `FgdcDescriptionPurposePresent` |
| Time period | `FgdcTimeOfContentTimePeriodPresent`, `FgdcTimeOfContentCurrentnessReferenceValid` |
| Status | `FgdcStatusProgressCodeValid`, `FgdcStatusUpdateFrequencyCodeValid` |
| Bounding box | `FgdcBoundingWestCoordInRange`, `FgdcBoundingNorthCoordInRange`, `FgdcBoundingNorthGeqSouth` |
| G-ring / polygon | `FgdcGPolygonOuterRingHasAtLeastFourPoints`, `FgdcGRingLatitudeInRange`, `FgdcGRingLongitudeInRange` |
| Keywords | `FgdcKeywordsHasAtLeastOneTheme`, `FgdcThemeHasKeywordThesaurus`, `FgdcThemeHasAtLeastOneKeyword` |
| Map projection | `FgdcMapProjNameIsValidCode`, `FgdcMapProjScaleFactorPositive`, `FgdcMapProjFalseOriginIsReal` |
| Security | `FgdcSecurityClassificationCodeValid`, `FgdcSecurityHandlingDescriptionPresent` |
| Contact | `FgdcContactOrganizationOrPersonRequired`, `FgdcContactVoiceTelephonePresent` |
| Distribution | `FgdcDistributorContactPresent`, `FgdcNetworkResourceNameIsUrl` |
| Aggregate proofs | `FgdcCitationInfoValid`, `FgdcTimePeriodInfoValid`, `FgdcContactInfoValid`, `FgdcIdentificationSectionValid`, `FgdcDistributionSectionValid`, `FgdcMetadataReferenceSectionValid`, `FgdcRecordValid` |

---

## Proof Composition Reference

Evidence bundle types and their `ProvableFrom` implications:

### ISO 19111 CRS chains

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `GeodeticFrameEvidence` | `ellipsoid: Established<EllipsoidValid>`, `prime_meridian: Established<PrimeMeridianValid>` | `GeodeticReferenceFrameValid` |
| `GeodeticCrsEvidence` | `frame: Established<GeodeticReferenceFrameValid>`, `cs: Established<CoordinateSystemValid>` | `CrsValid` |
| `ProjectedCrsEvidence` | `base: Established<CrsValid>`, `cs: Established<CoordinateSystemValid>` | `CrsValid` |
| `CompoundCrsEvidence` | `components: Vec<Established<CrsValid>>` | `CrsValid` |
| `Established<CrsValid>` | (single-token upcast) | `CoordinateMetadataValid` |

### OGC SFS geometry chains

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `PolygonEvidence` | `exterior: Established<LinearRingValid>`, `holes: Vec<Established<LinearRingValid>>` | `PolygonValid` |
| `MultiPointEvidence` | `points: Vec<Established<PointValid>>` | `MultiPointValid` |
| `MultiLineStringEvidence` | `lines: Vec<Established<LineStringValid>>` | `MultiLineStringValid` |
| `MultiPolygonEvidence` | `polygons: Vec<Established<PolygonValid>>` | `MultiPolygonValid` |
| `GeometryCollectionEvidence` | `geoms: Vec<Established<SfsGeometryValid>>` | `GeometryCollectionValid` |

### RFC 7946 GeoJSON chains

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `Established<PositionValid>` | (single-token upcast) | `GeoJsonPointValid` |
| `GeoJsonMultiPointEvidence` | `positions: Vec<Established<PositionValid>>` | `GeoJsonMultiPointValid` |
| `GeoJsonLineStringEvidence` | `positions: Vec<Established<PositionValid>>` (≥ 2) | `GeoJsonLineStringValid` |
| `GeoJsonMultiLineStringEvidence` | `lines: Vec<Established<GeoJsonLineStringValid>>` | `GeoJsonMultiLineStringValid` |
| `GeoJsonPolygonEvidence` | `exterior: Vec<Established<PositionValid>>`, `holes: Vec<Vec<…>>` | `GeoJsonPolygonValid` |
| `GeoJsonMultiPolygonEvidence` | `polygons: Vec<Established<GeoJsonPolygonValid>>` | `GeoJsonMultiPolygonValid` |
| `GeoJsonGeometryCollectionEvidence` | `geometries: Vec<Established<GeoJsonGeometryValid>>` | `GeoJsonGeometryCollectionValid` |
| `GeoJsonFeatureCollectionEvidence` | `features: Vec<Established<GeoJsonFeatureValid>>` | `GeoJsonFeatureCollectionValid` |

### ISO 19115-1 metadata chains

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `ExExtentEvidence` | `geographic: Vec<Established<ExGeographicBoundingBoxValid>>` | `ExExtentValid` |
| `MdIdentificationEvidence` | `citation: Established<CiCitationValid>`, `extents: Vec<Established<ExExtentValid>>` | `MdIdentificationValid` |
| `MdMetadataEvidence` | `contact: Established<CiResponsibilityValid>`, `identification: Established<MdIdentificationValid>` | `MdMetadataValid` |

### FGDC metadata chains

| Evidence bundle | Required fields | Proposition proved |
|---|---|---|
| `FgdcIdentificationEvidence` | `citation: Established<FgdcCitationInfoValid>`, `time_period: Established<FgdcTimePeriodInfoValid>` | `FgdcIdentificationSectionValid` |
| `FgdcDistributionEvidence` | `contact: Established<FgdcContactInfoValid>` | `FgdcDistributionSectionValid` |
| `FgdcMetadataRefEvidence` | `contact: Established<FgdcContactInfoValid>` | `FgdcMetadataReferenceSectionValid` |
| `FgdcRecordEvidence` | `identification`, `metadata_ref` (required) + `data_quality`, `spatial_org`, `spatial_ref`, `entity_attr`, `distribution` (optional) | `FgdcRecordValid` |

---

## Descriptor Types

`elicit_gis::types` provides the data-carrying companions to proof tokens:

### ISO 19111 / CRS types

| Type | Purpose |
|------|---------|
| `AuthorityCode` | Authority + code pair (e.g. `EPSG:4326`) |
| `EpsgCode` | Newtype over `u32` for EPSG numeric codes |
| `CrsInfo` | CRS name, type, authority code, scope |
| `DatumEnsembleInfo` | Ensemble name, member list, accuracy |
| `EllipsoidParams` | Semi-major axis, inverse flattening, unit |
| `PrimeMeridianParams` | Name, Greenwich longitude |
| `CoordinateAxisInfo` | Name, abbreviation, direction, unit |
| `CoordinateSystemParams` | CS type, axes list |
| `GeodeticFrameParams` | Frame name, ellipsoid, prime meridian, realization epoch |
| `GeographicBoundingBox` | West/east/south/north in decimal degrees |
| `DomainExtent` | Geographic bounding boxes, vertical extent, temporal |
| `CoordinateMetadata` | CRS + optional coordinate epoch (`DecimalYear`) |
| `CrsType` | `Geographic2d`, `Geographic3d`, `Geocentric`, `Projected`, `Compound`, `Vertical`, `Engineering`, `Temporal` |
| `AxisDirection` | `North`, `East`, `South`, `West`, `Up`, `Down`, `Future`, `Past`, … |

### ISO 19115-1 / metadata types

| Type | Purpose |
|------|---------|
| `CitationDescriptor` | Title, alternate titles, dates, identifier, edition |
| `ResponsibilityDescriptor` | Party name, role code, contact info |
| `GeographicBboxDescriptor` | West/east/south/north bounding coordinates |
| `ExtentDescriptor` | Geographic, vertical, and temporal extents |
| `IdentificationDescriptor` | Citation, abstract, purpose, extents, topic categories |
| `MetadataDescriptor` | Identification, contact, dates, lineage, distribution |
| `LineageDescriptor` | Statement, process steps, sources |
| `DataQualityDescriptor` | Scope, reports |
| `Iso19115Date` | Date string + date-type code |
| `LineageProcessStep` | Description, date-time, sources |

### OGC SFS geometry types

| Type | Purpose |
|------|---------|
| `SfsCoordinate` | `x`, `y` coordinate pair |
| `SfsCoordinate3D` | `x`, `y`, `z` coordinate triple |
| `PointDescriptor` | Coordinate + optional SRID |
| `LineStringDescriptor` | Point list + optional SRID |
| `LinearRingDescriptor` | Closed line string + optional SRID |
| `PolygonDescriptor` | Exterior ring + interior holes + optional SRID |
| `MultiGeometryDescriptor` | Geometry list + optional SRID |
| `GeometryCollectionDescriptor` | Heterogeneous geometry list + optional SRID |

### RFC 7946 / GeoJSON types

| Type | Purpose |
|------|---------|
| `GeoJsonPosition` | `[longitude, latitude]` or `[longitude, latitude, altitude]` |
| `GeoJsonGeometryKind` | `Point`, `MultiPoint`, `LineString`, `MultiLineString`, `Polygon`, `MultiPolygon`, `GeometryCollection` |
| `GeoJsonGeometryDescriptor` | Geometry kind + positions/coordinates |
| `GeoJsonFeatureDescriptor` | Optional geometry + optional properties |
| `GeoJsonFeatureCollectionDescriptor` | Feature list + optional bbox |
| `GeoJsonDocumentDescriptor` | Root GeoJSON object |
| `GeoJsonFeatureId` | `String(String)` or `Number(f64)` |

---

## Usage

```toml
[dependencies]
elicit_gis = { workspace = true }
```

### SFS geometry construction

```rust
use elicit_gis::{SfsGeometryFactory, PolygonEvidence, PolygonValid};
use elicitation::Established;

fn build_square(factory: &dyn SfsGeometryFactory) -> GisResult<Established<PolygonValid>> {
    let ring_coords = vec![
        SfsCoordinate { x: 0.0, y: 0.0 },
        SfsCoordinate { x: 1.0, y: 0.0 },
        SfsCoordinate { x: 1.0, y: 1.0 },
        SfsCoordinate { x: 0.0, y: 1.0 },
        SfsCoordinate { x: 0.0, y: 0.0 }, // close the ring
    ];
    let (_, ring_proof) = factory.build_linear_ring(ring_coords, Some(4326))?;
    let (_, poly_proof) = factory.build_polygon(
        PolygonEvidence { exterior: ring_proof, holes: vec![] }, Some(4326)
    )?;
    Ok(poly_proof)
}
```

### GeoJSON feature with proof chain

```rust
use elicit_gis::{GeoJsonGeometryFactory, GeoJsonFeatureFactory, GeoJsonPolygonEvidence};

async fn build_feature(
    factory: &dyn GeoJsonGeometryFactory,
    feat_factory: &dyn GeoJsonFeatureFactory,
) -> GisResult<Established<GeoJsonFeatureValid>> {
    // Validate positions (RFC 7946 §3.1.1)
    let positions: Vec<_> = ring_positions
        .iter()
        .map(|p| factory.validate_position(*p))
        .collect::<GisResult<_>>()?;
    let exterior: Vec<_> = positions.into_iter().map(|(_, p)| p).collect();

    // Polygon evidence: exterior positions + no holes
    let (_, poly_proof) = factory.build_geojson_polygon(
        GeoJsonPolygonEvidence { exterior, holes: vec![] }
    )?;

    // Feature wraps the geometry
    let (_, feature_proof) = feat_factory.build_geojson_feature(
        GeoJsonFeatureDescriptor { geometry: Some(poly_desc), properties: None, id: None }
    )?;
    Ok(feature_proof)
}
```

### CRS lookup and transformation

```rust
use elicit_gis::{GisCrsLookup, GisCrsTransformer};

async fn reproject(
    lookup: &dyn GisCrsLookup,
    transformer: &dyn GisCrsTransformer,
    geom: &dyn SfsGeometryMeta,
) -> GisResult<Box<dyn SfsGeometryMeta>> {
    let crs_info = lookup.lookup_crs(&AuthorityCode::epsg(32632)).await?;
    transformer.transform(geom, &crs_info.code)
}
```

---

## Compile-Time Guarantee Summary

| What is guaranteed | Mechanism |
|---|---|
| Polygon has a valid exterior ring | `PolygonEvidence.exterior` field is `Established<LinearRingValid>` — not optional |
| GeoJSON positions are in range | `validate_position()` checks `[-180,180]` longitude + `[-90,90]` latitude before minting `PositionValid` |
| CRS proof requires ellipsoid + meridian | `GeodeticFrameEvidence` struct fields are both required `Established<_>` tokens |
| ISO 19115 metadata has mandatory contact | `MdMetadataEvidence.contact` field is `Established<CiResponsibilityValid>` — not `Option` |
| FGDC record has both mandatory sections | `FgdcRecordEvidence` requires `identification` + `metadata_ref`; optional sections use `Option<Established<_>>` |
| `assert()` bypasses are audit-visible | Any `Established::assert()` on a GIS proposition stands out immediately in review |

---

## Implementing a Custom Backend

To implement `GisBackend` for a new geospatial driver:

1. Implement the factory traits (`SfsGeometryFactory`, `GeoJsonGeometryFactory`,
   `GisCrsBuilder`, `Iso19115CitationFactory`, `Iso19115RecordFactory`, etc.).
   Each method must validate/construct the object, then call
   `Established::assert()` after success.

2. Implement the reporter traits (`SfsGeometryMeta`, `GisCrsLookup`,
   `Iso19115ContactMeta`, etc.). These return plain data; no proof tokens.

3. Implement `FgdcBackend` (supertrait of all 26 FGDC sub-traits). The
   blanket impl for `GisBackend` is satisfied automatically once all sub-traits
   are implemented.

> **Note:** `Established::assert()` is the correct constructor for backend
> implementations — the factory *is* the authority that the operation succeeded.
> The credential-gated `Established::prove()` path is reserved for
> evidence-bundle composition, not leaf-level construction.

---

## Standards Grounding

| Standard | Coverage |
|----------|----------|
| ISO 19111:2019 (CRS) | CRS object model — identified objects, datums, coordinate systems, operations, metadata |
| ISO 19115-1:2014 (Metadata) | Metadata object model — mandatory/optional elements, code lists, extent, lineage, quality |
| OGC Simple Features Access Part 1 (SFA-CA) | Geometry object model — type invariants, dimension, boundary, WKT/WKB, DE-9IM topology |
| IETF RFC 7946 (GeoJSON) | GeoJSON encoding — object types, position rules, geometry validity, feature structure, CRS |
| FGDC-STD-001-1998 | CSDGM — section structure, citation, time period, bounding box, distribution, contact |

---

## Crate Layout

```text
src/
├── lib.rs                       pub use surface + GisBackend supertrait
├── error.rs                     GisError / GisResult
├── types/
│   ├── mod.rs
│   ├── authority.rs             AuthorityCode, EpsgCode, CrsInfo, DatumEnsembleInfo, EllipsoidParams
│   ├── axis.rs                  AxisDirection, CsType
│   ├── crs.rs                   CrsType, DecimalYear, HelmertConvention, CoordinateMetadata
│   ├── iso_19111.rs             PrimeMeridianParams, CoordinateAxisInfo, GeodeticFrameParams, DomainExtent
│   ├── iso_19115.rs             CitationDescriptor, MetadataDescriptor, ExtentDescriptor, Iso19115Date
│   ├── ogc_sfs.rs               SfsCoordinate, PointDescriptor, PolygonDescriptor, MultiGeometryDescriptor
│   ├── rfc7946.rs               GeoJsonPosition, GeoJsonGeometryKind, GeoJsonFeatureDescriptor
│   └── fgdc.rs                  FgdcKeywordGroup, FgdcAttributeDescriptor, FgdcRecordDescriptor, …
├── contracts/
│   ├── mod.rs                   Re-exports + all Evidence bundle types listed
│   ├── iso_19111.rs             265 props — CRS object model + ProvableFrom chains
│   ├── iso_19115.rs             440 props — metadata object model + ProvableFrom chains
│   ├── ogc_sfs.rs               278 props — SFS geometry invariants + ProvableFrom chains
│   ├── rfc7946.rs               211 props — GeoJSON spec + ProvableFrom chains
│   └── fgdc.rs                  160 props — FGDC section rules + ProvableFrom chains
└── traits/
    ├── mod.rs                   Re-exports + GisBackend supertrait
    ├── crs.rs                   GisCrsLookup, GisCrsBuilder, GisCrsTransformer
    ├── iso_19111.rs             Iso19111Identified, Iso19111Scoped
    ├── iso_19115.rs             Iso19115CitationFactory, Iso19115ExtentFactory, Iso19115LineageFactory,
    │                            Iso19115RecordFactory, Iso19115ContactMeta, Iso19115DateMeta,
    │                            Iso19115QualityMeta
    ├── sfs.rs                   SfsGeometryFactory, SfsGeometryMeta, SfsGeometryIo
    ├── topology.rs              SfsTopology
    ├── set_ops.rs               SfsSetOps
    ├── rfc7946.rs               GeoJsonGeometryFactory, GeoJsonFeatureFactory,
    │                            GeoJsonObjectMeta, GeoJsonFeatureMeta, GeoJsonBackend
    └── fgdc.rs                  26 FGDC sub-traits + FgdcBackend supertrait
```

---

## Formal Verification

The proof architecture is designed for downstream formal verification.
Each proposition type implements `elicitation::contracts::Prop`, which
exposes a `kani_proof()` method for generating verification harnesses.

- **Kani** — bounded model checking on coordinate range validation paths
- **Creusot** — deductive verification that factory methods check invariants
  before calling `Established::assert()`
- **Verus** — SMT-based proofs of evidence bundle composition totality

---

## License

Apache-2.0 OR MIT
