//! Trait re-exports and the [`GisBackend`] supertrait.

mod crs;
mod fgdc;
mod iso_19111;
mod iso_19115;
mod rfc7946;
mod set_ops;
mod sfs;
mod topology;

pub use crs::{GisCrsBuilder, GisCrsLookup, GisCrsTransformer};
pub use fgdc::{
    FgdcBackend, FgdcBoundingMeta, FgdcBoundingValidator, FgdcCitationFactory, FgdcContactFactory,
    FgdcContactMeta, FgdcContactValidator, FgdcDataQualityFactory, FgdcDateValidator,
    FgdcDistributionFactory, FgdcDistributionMeta, FgdcDistributionParamValidator,
    FgdcEntityAttrFactory, FgdcEntityAttrValidator, FgdcIdentificationFactory, FgdcKeywordMeta,
    FgdcMapProjValidator, FgdcMetadataMeta, FgdcMetadataRefFactory, FgdcRecordFactory,
    FgdcSecurityValidator, FgdcSpatialOrgFactory, FgdcSpatialParamValidator, FgdcSpatialRefFactory,
    FgdcStatusValidator, FgdcTimePeriodFactory, FgdcVertRefValidator,
};
pub use iso_19111::{Iso19111Identified, Iso19111Scoped};
pub use iso_19115::{
    Iso19115CitationFactory, Iso19115ContactMeta, Iso19115DateMeta, Iso19115ExtentFactory,
    Iso19115LineageFactory, Iso19115QualityMeta, Iso19115RecordFactory,
};
pub use rfc7946::{
    GeoJsonBackend, GeoJsonFeatureFactory, GeoJsonFeatureMeta, GeoJsonGeometryFactory,
    GeoJsonObjectMeta,
};
pub use set_ops::SfsSetOps;
pub use sfs::{SfsGeometryFactory, SfsGeometryIo, SfsGeometryMeta};
pub use topology::SfsTopology;

/// Complete geospatial backend — blanket supertrait.
///
/// Any type that implements all GIS sub-traits automatically implements
/// `GisBackend`.  Use the individual object-safe sub-traits
/// (`dyn GisCrsLookup`, `dyn SfsGeometryFactory`, etc.) for dynamic dispatch
/// at architectural boundaries.
pub trait GisBackend:
    GisCrsLookup
    + GisCrsBuilder
    + GisCrsTransformer
    + Iso19111Identified
    + Iso19111Scoped
    + Iso19115CitationFactory
    + Iso19115ContactMeta
    + Iso19115DateMeta
    + Iso19115ExtentFactory
    + Iso19115LineageFactory
    + Iso19115QualityMeta
    + Iso19115RecordFactory
    + GeoJsonGeometryFactory
    + GeoJsonFeatureFactory
    + GeoJsonObjectMeta
    + GeoJsonFeatureMeta
    + SfsGeometryFactory
    + SfsGeometryMeta
    + SfsGeometryIo
    + SfsTopology
    + SfsSetOps
    + FgdcBackend
    + Send
    + Sync
{
}

impl<T> GisBackend for T where
    T: GisCrsLookup
        + GisCrsBuilder
        + GisCrsTransformer
        + Iso19111Identified
        + Iso19111Scoped
        + Iso19115CitationFactory
        + Iso19115ContactMeta
        + Iso19115DateMeta
        + Iso19115ExtentFactory
        + Iso19115LineageFactory
        + Iso19115QualityMeta
        + Iso19115RecordFactory
        + GeoJsonGeometryFactory
        + GeoJsonFeatureFactory
        + GeoJsonObjectMeta
        + GeoJsonFeatureMeta
        + SfsGeometryFactory
        + SfsGeometryMeta
        + SfsGeometryIo
        + SfsTopology
        + SfsSetOps
        + FgdcBackend
        + Send
        + Sync
{
}
