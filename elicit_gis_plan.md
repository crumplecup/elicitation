Geospatial Contract Backbone Plan (v0.1)
=======================================

Objective
---------

Establish a minimal, enforceable geospatial contract system aligned with real-world standards,
mirroring existing patterns used for UI (WCAG) and finance (GAAP). The system must ensure that
all geospatial data is provably valid, interpretable, and interoperable.

This plan defines:

- A standards-backed contract stack
- A minimal viable subset for implementation
- A proof-carrying validation model
- Integration points with existing architecture

I. Contract Philosophy
---------------------

All geospatial data MUST:

1. Conform to a well-defined geometry model
2. Declare and respect a coordinate reference system (CRS)
3. Include sufficient metadata for interpretation and lineage
4. Be representable in canonical interchange formats

No geospatial data may enter or exit the system without passing through validation
and producing a proof artifact.

Pattern:
    Raw Input → Validate → Geo IR → Established<GeoValid> → Projection

II. Standards Backbone
----------------------

A. Geometry & Topology (FOUNDATIONAL)
------------------------------------

- OGC Simple Features Specification (SFS)
  - Geometry types: Point, LineString, Polygon, Multi*
  - Validity rules: no self-intersections, closed rings, etc.

- ISO 19107 (Geometry Model)
  - Conceptual alignment for geometry semantics

Contract Guarantees:
    - Geometry is structurally valid
    - Topology rules are satisfied

B. Coordinate Reference Systems (CRS)
-------------------------------------

- ISO 19111 (Spatial Referencing by Coordinates)
- EPSG registry (practical CRS definitions)

Contract Guarantees:
    - CRS is explicitly defined
    - All geometries are associated with a CRS
    - Transformations are explicit and valid

C. Encoding & Interchange
-------------------------

- WKB (Well-Known Binary) — canonical internal format
- WKT (Well-Known Text) — debugging / human-readable
- GeoJSON — external/web interchange

Optional:

- FlatGeobuf — high-performance storage

Contract Guarantees:
    - Lossless round-trip serialization
    - Deterministic encoding/decoding

D. Metadata & Cataloging
------------------------

- ISO 19115 (Geographic Metadata) — primary standard
- FGDC Metadata Standard — compatibility layer

Minimum Required Fields (v0.1):
    - dataset_id
    - title
    - coordinate_system
    - lineage (source/provenance)
    - creation_timestamp

Contract Guarantees:
    - Dataset is interpretable by external systems
    - Provenance is explicitly recorded

E. Services & Access (DEFERRED)
-------------------------------

- OGC API - Features (future)
- WFS/WMS (optional legacy compatibility)

Not required for v0.1, but design must not preclude adoption.

III. Core Intermediate Representation (IR)
------------------------------------------

struct GeoFeature {
    geometry: Geometry,              // OGC SFS compliant
    crs: CoordinateReferenceSystem,  // ISO 19111 compliant
    properties: Map<String, Value>,  // arbitrary attributes
    metadata: Metadata,              // ISO 19115 subset
}

struct Metadata {
    dataset_id: String,
    title: String,
    crs: String,
    lineage: String,
    created_at: Timestamp,
}

IV. Proof-Carrying Validation
-----------------------------

type GeoValid = Established<(
    GeometryValid,
    CrsValid,
    MetadataComplete,
)>;

Validation Pipeline:

fn validate_geo(input: RawGeoData) -> Result<Established<GeoValid>, GeoError> {
    let geom = validate_geometry(input.geometry)?;      // OGC SFS rules
    let crs  = validate_crs(input.crs)?;                // ISO 19111
    let meta = validate_metadata(input.metadata)?;      // ISO 19115 subset

    Ok(Established::new(GeoFeature {
        geometry: geom,
        crs: crs,
        properties: input.properties,
        metadata: meta,
    }))
}

Failure at any stage MUST prevent construction of GeoValid.

V. Invariants (Non-Negotiable)
------------------------------

1. No geometry without CRS
2. No CRS without explicit definition (no implicit defaults)
3. No feature without metadata
4. No serialization without canonical encoding
5. No transformation without explicit CRS conversion

These invariants are enforced at type/validation boundaries.

VI. Storage Strategy
--------------------

Recommended:

- PostgreSQL + PostGIS (leverages OGC SFS compliance)

Mapping:
    Geometry → PostGIS geometry/geography
    CRS      → EPSG SRID
    Metadata → JSONB or structured columns

Contract:
    Database layer MUST NOT bypass validation pipeline.

VII. Interoperability Layer
---------------------------

Inbound:
    - GeoJSON → parse → validate → GeoFeature
    - WKT/WKB → parse → validate → GeoFeature

Outbound:
    - GeoFeature → WKB (canonical)
    - GeoFeature → GeoJSON (external)

Round-trip fidelity MUST be maintained.

VIII. Integration with Existing Systems
---------------------------------------

A. UI (WCAG / IR)
    - GeoFeature projected into AccessKit-compatible structures
    - Map interactions must remain accessible
    - No direct rendering without IR transformation

B. Finance (GAAP / Ledger)
    - Spatial assets reference GeoFeature IDs
    - Asset valuation linked to location
    - Provenance aligns with financial audit trail

C. Database (elicit_db)
    - GeoFeature stored via validated pipeline
    - Queries may return raw or validated forms
    - Optional: enforce GeoValid at query boundary

IX. Minimal Implementation Scope (v0.1)
---------------------------------------

Must implement:

- Geometry validation (basic OGC SFS subset)
- CRS validation (EPSG-backed)
- Metadata enforcement (minimal ISO 19115 subset)
- GeoJSON + WKB support
- PostGIS integration

Explicitly excluded:

- Advanced topology (networks, surfaces)
- Multi-CRS datasets
- OGC service layers
- Raster data

X. Future Extensions
--------------------

- Full ISO 19115 metadata coverage
- CRS transformation pipelines (PROJ integration)
- OGC API - Features server
- Spatial indexing guarantees in proofs
- Temporal geospatial data (time-aware features)
- Cross-domain proofs (Geo + Financial + UI)

XI. Guiding Principle
---------------------

Do not attempt to support all geospatial standards.

Instead:
    Define a minimal, enforceable subset
    Require all data to prove against it
    Expand only when necessary

This preserves:
    - correctness
    - composability
    - long-term maintainability

End State Vision
----------------

A system where:

- Every geospatial object is provably valid
- Every dataset is interpretable and traceable
- Every projection (UI, API, storage) derives from a single verified IR

Equivalent to:
    WCAG → UI correctness
    GAAP → financial correctness
    THIS → spatial correctness
