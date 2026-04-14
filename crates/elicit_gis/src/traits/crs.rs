//! CRS management traits for ISO 19111 operations.
//!
//! Source: ISO 19111:2019 — Spatial referencing by coordinates.

use elicitation::Established;
use futures::future::BoxFuture;

use crate::{
    AuthorityCode, CoordinateMetadata, CoordinateMetadataValid, CoordinatesTransformed, CrsInfo,
    CrsResolved, CrsType, CrsValid, DatumEnsembleInfo, DatumEnsembleResolved, EllipsoidParams,
    EllipsoidValid, GisResult,
};

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

/// Validate CRS structures against ISO 19111 rules.
///
/// Source: ISO 19111:2019 §7–§14 — CRS and operation validity.
pub trait GisCrsValidator: Send + Sync {
    /// Validate a CRS definition referenced by authority code.
    ///
    /// Source: ISO 19111:2019 §6.2 — CRS structural completeness.
    fn validate_crs(&self, code: &AuthorityCode)
    -> BoxFuture<'_, GisResult<Established<CrsValid>>>;

    /// Validate ellipsoid parameters.
    ///
    /// Checks: name non-empty, semi_major_axis > 0, exactly one of
    /// inverse_flattening / semi_minor_axis provided.
    ///
    /// Source: ISO 19111:2019 §7.3 — CD_Ellipsoid.
    fn validate_ellipsoid(
        &self,
        params: &EllipsoidParams,
    ) -> BoxFuture<'_, GisResult<Established<EllipsoidValid>>>;

    /// Validate coordinate metadata — checks CRS validity and, for dynamic
    /// CRS, asserts that a coordinate epoch is present.
    ///
    /// Source: ISO 19111:2019 §7.4 — SC_CoordinateMetadata.
    fn validate_coordinate_metadata(
        &self,
        meta: &CoordinateMetadata,
    ) -> BoxFuture<'_, GisResult<Established<CoordinateMetadataValid>>>;
}

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
