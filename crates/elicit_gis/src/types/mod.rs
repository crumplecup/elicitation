//! Types re-exports.

mod authority;
mod axis;
mod crs;
mod fgdc;
mod iso_19111;
mod iso_19115;
mod ogc_sfs;
mod rfc7946;

pub use authority::{AuthorityCode, CrsInfo, DatumEnsembleInfo, EllipsoidParams, EpsgCode};
pub use axis::{AxisDirection, CsType};
pub use crs::{CoordinateMetadata, CrsType, DecimalYear, HelmertConvention};
pub use fgdc::{
    FgdcAttributeDescriptor, FgdcCitationDescriptor, FgdcContactDescriptor,
    FgdcDataQualityDescriptor, FgdcDistributionDescriptor, FgdcDomainKind,
    FgdcEntityAttrDescriptor, FgdcEnumeratedValue, FgdcHorizCrsKind, FgdcIdentificationDescriptor,
    FgdcKeywordGroup, FgdcMetadataRefDescriptor, FgdcProcessStepInfo, FgdcPvectKind,
    FgdcRangeDomainInfo, FgdcRecordDescriptor, FgdcSourceInfo, FgdcSpatialOrgDescriptor,
    FgdcSpatialRefDescriptor, FgdcTimePeriodDescriptor, FgdcTimePeriodKind,
};
pub use iso_19111::{
    CoordinateAxisInfo, CoordinateSystemParams, DomainExtent, GeodeticFrameParams,
    GeographicBoundingBox, PrimeMeridianParams,
};
pub use iso_19115::{
    CitationDescriptor, DataQualityDescriptor, DataQualityReport, ExtentDescriptor,
    GeographicBboxDescriptor, IdentificationDescriptor, Iso19115Date, LineageDescriptor,
    LineageProcessStep, MetadataDescriptor, ResponsibilityDescriptor, TemporalExtentDescriptor,
    VerticalExtentDescriptor,
};
pub use ogc_sfs::{
    GeometryCollectionDescriptor, LineStringDescriptor, LinearRingDescriptor,
    MultiGeometryDescriptor, PointDescriptor, PolygonDescriptor, SfsCoordinate, SfsCoordinate3D,
};
pub use rfc7946::{
    GeoJsonDocumentDescriptor, GeoJsonFeatureCollectionDescriptor, GeoJsonFeatureDescriptor,
    GeoJsonFeatureId, GeoJsonGeometryDescriptor, GeoJsonGeometryKind, GeoJsonPosition,
};
