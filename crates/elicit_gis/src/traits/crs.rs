//! CRS management traits for ISO 19111 operations.
//!
//! # Design
//!
//! These traits follow a **validity-by-construction** pattern.  Raw parameter
//! structs flow *in*; a validated descriptor plus an [`Established<P>`] proof
//! token flow *out*.  Composite builders demand proof tokens for their
//! components as preconditions, encoding the CRS dependency graph in the type
//! system.
//!
//! Leaf builders (ellipsoid, prime meridian, coordinate system) accept only
//! raw parameters.  Composite builders (geodetic frame, CRS subtypes, compound
//! CRS, coordinate metadata) require `Established<P>` tokens from the
//! components that must already be valid before the composite can be assembled.
//!
//! Because every `Established<P>` is a concrete type, all method signatures
//! are generic-free and the traits are `dyn`-compatible.
//!
//! Source: ISO 19111:2019 — Spatial referencing by coordinates.
//!
//! [`Established<P>`]: elicitation::Established

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuthorityCode, CoordinateMetadata, CoordinateMetadataValid, CoordinateSystemParams,
    CoordinateSystemValid, CoordinatesTransformed, CrsInfo, CrsResolved, CrsType, CrsValid,
    DatumEnsembleInfo, DatumEnsembleResolved, EllipsoidParams, EllipsoidValid, GeodeticFrameParams,
    GeodeticReferenceFrameValid, GisResult, PrimeMeridianParams, PrimeMeridianValid,
};

// ── Lookup ────────────────────────────────────────────────────────────────────

/// Look up and introspect CRS definitions from a registry.
///
/// Source: ISO 19111:2019 §6.2 / §16 — CRS identification and registry.
pub trait GisCrsLookup: Send + Sync {
    /// Resolve a CRS by its authority and code.
    ///
    /// Source: ISO 19111:2019 §6.2 — RS_Identifier lookup.
    fn lookup_crs(
        &self,
        code: &AuthorityCode,
    ) -> BoxFuture<'_, GisResult<(CrsInfo, Established<CrsResolved>)>>;

    /// List all known CRS entries of a given type.
    ///
    /// Source: ISO 19111:2019 §6 — CRS type hierarchy.
    fn list_crs(&self, crs_type: CrsType) -> BoxFuture<'_, GisResult<Vec<CrsInfo>>>;

    /// Resolve a datum ensemble by its authority code and return its members
    /// and ensemble accuracy.
    ///
    /// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.
    fn resolve_datum_ensemble(
        &self,
        code: &AuthorityCode,
    ) -> BoxFuture<'_, GisResult<(DatumEnsembleInfo, Established<DatumEnsembleResolved>)>>;

    /// Determine whether a CRS is dynamic (requires a coordinate epoch).
    ///
    /// Source: ISO 19111:2019 §17.2 — dynamic CRS definition.
    fn is_dynamic_crs(&self, code: &AuthorityCode) -> BoxFuture<'_, GisResult<bool>>;
}

// ── Builder ───────────────────────────────────────────────────────────────────

/// Build and validate CRS component structures against ISO 19111 rules.
///
/// Each method is a **factory**: it accepts raw parameters (and, for
/// composite types, proof tokens from already-validated components) and
/// returns either an error or a validated descriptor together with an
/// [`Established<P>`] proof token certifying the validity proposition.
///
/// # Leaf builders
///
/// [`build_ellipsoid`], [`build_prime_meridian`], [`build_coordinate_system`],
/// and [`build_datum_ensemble`] require only raw parameters — they stand at the
/// bottom of the dependency graph.
///
/// # Composite builders
///
/// [`build_geodetic_frame`], [`build_crs`], [`build_projected_crs`],
/// [`build_compound_crs`], and [`build_coordinate_metadata`] each require one
/// or more proof tokens from earlier leaf or composite builds, encoding the CRS
/// component graph as type-checked preconditions.
///
/// Source: ISO 19111:2019 §7–§17.
///
/// [`Established<P>`]: elicitation::Established
/// [`build_ellipsoid`]: Self::build_ellipsoid
/// [`build_prime_meridian`]: Self::build_prime_meridian
/// [`build_coordinate_system`]: Self::build_coordinate_system
/// [`build_datum_ensemble`]: Self::build_datum_ensemble
/// [`build_geodetic_frame`]: Self::build_geodetic_frame
/// [`build_crs`]: Self::build_crs
/// [`build_projected_crs`]: Self::build_projected_crs
/// [`build_compound_crs`]: Self::build_compound_crs
/// [`build_coordinate_metadata`]: Self::build_coordinate_metadata
pub trait GisCrsBuilder: Send + Sync {
    // ── Leaf builders ─────────────────────────────────────────────────────────

    /// Validate ellipsoid parameters and return the validated parameters
    /// alongside an `EllipsoidValid` proof token.
    ///
    /// Checks: name non-empty, semi_major_axis > 0 and finite, exactly one
    /// second parameter (inverse_flattening or semi_minor_axis) provided, and
    /// — when non-sphere — semi_minor_axis < semi_major_axis.
    ///
    /// Source: ISO 19111:2019 §7.3 — CD_Ellipsoid.
    fn build_ellipsoid(
        &self,
        params: &EllipsoidParams,
    ) -> BoxFuture<'_, GisResult<(EllipsoidParams, Established<EllipsoidValid>)>>;

    /// Validate prime meridian parameters and return the validated parameters
    /// alongside a `PrimeMeridianValid` proof token.
    ///
    /// Checks: name non-empty, greenwich_longitude finite, unit is angular,
    /// and — for the Greenwich meridian — longitude is exactly zero.
    ///
    /// Source: ISO 19111:2019 §7.4 — CD_PrimeMeridian.
    fn build_prime_meridian(
        &self,
        params: &PrimeMeridianParams,
    ) -> BoxFuture<'_, GisResult<(PrimeMeridianParams, Established<PrimeMeridianValid>)>>;

    /// Validate coordinate system parameters and return the validated
    /// parameters alongside a `CoordinateSystemValid` proof token.
    ///
    /// Checks: at least one axis, axis abbreviations unique, axis directions
    /// from the code list, unit types match axis type (angular/linear/temporal).
    ///
    /// Source: ISO 19111:2019 §8 — CS_CoordinateSystem.
    fn build_coordinate_system(
        &self,
        params: &CoordinateSystemParams,
    ) -> BoxFuture<'_, GisResult<(CoordinateSystemParams, Established<CoordinateSystemValid>)>>;

    /// Validate datum ensemble metadata and return the validated info alongside
    /// a `DatumEnsembleResolved` proof token.
    ///
    /// Checks: at least two members (no nulls), homogeneous datum type,
    /// ensemble_accuracy > 0 and finite.
    ///
    /// Source: ISO 19111:2019 §6.5 — CD_DatumEnsemble.
    fn build_datum_ensemble(
        &self,
        info: &DatumEnsembleInfo,
    ) -> BoxFuture<'_, GisResult<(DatumEnsembleInfo, Established<DatumEnsembleResolved>)>>;

    // ── Composite builders ────────────────────────────────────────────────────

    /// Build a geodetic reference frame, requiring proof that its ellipsoid
    /// and prime meridian are already valid.
    ///
    /// The `ellipsoid_valid` and `pm_valid` tokens are **preconditions** — the
    /// caller must have obtained them from [`build_ellipsoid`] and
    /// [`build_prime_meridian`] respectively.  This makes it impossible to
    /// assemble a frame from unvalidated components.
    ///
    /// Checks: name non-empty, exactly one ellipsoid, exactly one prime
    /// meridian, optional realization epoch is ISO 8601 when present.
    ///
    /// Source: ISO 19111:2019 §7.2 — CD_GeodeticReferenceFrame.
    ///
    /// [`build_ellipsoid`]: Self::build_ellipsoid
    /// [`build_prime_meridian`]: Self::build_prime_meridian
    fn build_geodetic_frame(
        &self,
        params: &GeodeticFrameParams,
        ellipsoid_valid: Established<EllipsoidValid>,
        pm_valid: Established<PrimeMeridianValid>,
    ) -> BoxFuture<'_, GisResult<(CrsInfo, Established<GeodeticReferenceFrameValid>)>>;

    /// Build a single-component geodetic, vertical, engineering, temporal, or
    /// parametric CRS, requiring proof that its reference frame and coordinate
    /// system are already valid.
    ///
    /// The `frame_valid` and `cs_valid` tokens are **preconditions**.  For
    /// datum-ensemble CRS (e.g. EPSG:4326 using WGS 84 ensemble) pass the
    /// `DatumEnsembleResolved` token via `build_compound_crs` instead, or
    /// consult the implementation documentation.
    ///
    /// Checks: CRS name non-empty, CS type compatible with CRS subtype, axis
    /// count matches subtype expectations (geographic 2D → 2, vertical → 1,
    /// etc.).
    ///
    /// Source: ISO 19111:2019 §7–§11 — single-component CRS subtypes.
    fn build_crs(
        &self,
        code: &AuthorityCode,
        frame_valid: Established<GeodeticReferenceFrameValid>,
        cs_valid: Established<CoordinateSystemValid>,
    ) -> BoxFuture<'_, GisResult<(CrsInfo, Established<CrsValid>)>>;

    /// Build a projected CRS, requiring proof that its base geographic CRS and
    /// Cartesian coordinate system are already valid.
    ///
    /// Projected CRS is a derived CRS: it requires a validated base geographic
    /// CRS (`base_crs_valid`) plus a 2-axis Cartesian coordinate system
    /// (`cs_valid`).
    ///
    /// Checks: base is geographic, CS is Cartesian with two axes, scale factor
    /// positive and finite, false easting/northing finite, axis order follows
    /// convention.
    ///
    /// Source: ISO 19111:2019 §9 — SC_ProjectedCRS.
    fn build_projected_crs(
        &self,
        code: &AuthorityCode,
        base_crs_valid: Established<CrsValid>,
        cs_valid: Established<CoordinateSystemValid>,
    ) -> BoxFuture<'_, GisResult<(CrsInfo, Established<CrsValid>)>>;

    /// Build a compound CRS from two or more already-validated component CRSes.
    ///
    /// `component_proofs` is an ordered list of `CrsValid` tokens for each
    /// component; the acyclicity token proves the component graph contains no
    /// cycles.
    ///
    /// Checks: at least two components, components are orthogonal (no shared
    /// axes), no two horizontal or two vertical components, total axis count
    /// equals sum of component axis counts.
    ///
    /// Source: ISO 19111:2019 §12 — SC_CompoundCRS.
    fn build_compound_crs(
        &self,
        code: &AuthorityCode,
        component_proofs: Vec<Established<CrsValid>>,
    ) -> BoxFuture<'_, GisResult<(CrsInfo, Established<CrsValid>)>>;

    /// Build validated coordinate metadata for a CRS reference plus an
    /// optional coordinate epoch.
    ///
    /// Requires `crs_valid` proving the referenced CRS is structurally correct.
    /// For dynamic CRS a coordinate epoch must be present; for static CRS it
    /// must be absent.
    ///
    /// Checks: CRS non-null, dynamic CRS requires epoch, static CRS epoch
    /// absent, epoch is a positive finite decimal year.
    ///
    /// Source: ISO 19111:2019 §17 — SC_CoordinateMetadata.
    fn build_coordinate_metadata(
        &self,
        meta: &CoordinateMetadata,
        crs_valid: Established<CrsValid>,
    ) -> BoxFuture<'_, GisResult<(CoordinateMetadata, Established<CoordinateMetadataValid>)>>;
}

// ── Transformer ───────────────────────────────────────────────────────────────

/// Transform coordinate tuples between coordinate reference systems.
///
/// Source: ISO 19111:2019 §14 — Coordinate operations.
pub trait GisCrsTransformer: Send + Sync {
    /// Transform a coordinate tuple from one CRS to another.
    ///
    /// `coords` must have the same element count as the source CRS axis count.
    ///
    /// Source: ISO 19111:2019 §14.2 — CC_Conversion / §14.3 — CC_Transformation.
    fn transform(
        &self,
        coords: &[f64],
        from: &AuthorityCode,
        to: &AuthorityCode,
    ) -> BoxFuture<'_, GisResult<(Vec<f64>, Established<CoordinatesTransformed>)>>;

    /// Transform a coordinate tuple between dynamic CRS, accounting for
    /// coordinate epochs on both sides.
    ///
    /// Source: ISO 19111:2019 §14 / §17 — epoch-aware transformation.
    fn transform_with_epoch(
        &self,
        coords: &[f64],
        from: &CoordinateMetadata,
        to: &CoordinateMetadata,
    ) -> BoxFuture<'_, GisResult<(Vec<f64>, Established<CoordinatesTransformed>)>>;

    /// Reorder coordinate values to match the canonical ISO axis order for the
    /// given CRS (e.g. swap lon/lat to lat/lon for EPSG:4326).
    ///
    /// Source: ISO 19111:2019 §15 — axis order.
    fn normalize_axis_order(
        &self,
        coords: &[f64],
        crs: &AuthorityCode,
    ) -> BoxFuture<'_, GisResult<Vec<f64>>>;
}
