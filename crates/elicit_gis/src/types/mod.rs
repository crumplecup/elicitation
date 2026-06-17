//! Types re-exports.

mod authority;
mod axis;
mod crs;
mod fgdc;
mod iso_19111;
mod iso_19115;
mod ogc_sfs;
mod render;
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
pub use render::{
    RenderAltitudeMode, RenderAmbientLightDescriptor, RenderAnnotationAnchorDescriptor,
    RenderAnnotationDescriptor, RenderAnnotationGeometryAnchor, RenderAnnotationLayerDescriptor,
    RenderAssetAccessPattern, RenderAssetColorSpace, RenderAssetIntegrityAlgorithm,
    RenderAssetIntegrityDescriptor, RenderAssetReference, RenderAssetResidencyPolicy,
    RenderAtmosphereDescriptor, RenderAtmosphereFalloffDescriptor,
    RenderAtmospherePhaseFunctionDescriptor, RenderAtmosphereScatteringTermDescriptor,
    RenderAutoExposureDescriptor, RenderBillboardMode, RenderBloomCompositeMode,
    RenderBloomDescriptor, RenderBloomPrefilterDescriptor, RenderBrush,
    RenderCameraProjectionDescriptor, RenderCascadeShadowConfigDescriptor,
    RenderChromaticAberrationDescriptor, RenderClearColorPolicy, RenderClearPolicyDescriptor,
    RenderColorGradingDescriptor, RenderColorGradingGlobalDescriptor,
    RenderColorGradingSectionDescriptor, RenderDepthOfFieldDescriptor, RenderDepthOfFieldMode,
    RenderDirectionalLightDescriptor, RenderExposureDescriptor, RenderExposurePreset,
    RenderExtrusionDescriptor, RenderFaceCulling, RenderFeatureBatchingDescriptor,
    RenderFeatureBatchingStrategy, RenderFeatureCollectionPayload, RenderFeatureGeometry,
    RenderFeaturePlacementDescriptor, RenderFeaturePredicate, RenderFeatureRecord,
    RenderFeatureRecordPayload, RenderFeatureStyleDescriptor, RenderFeatureStyleRuleDescriptor,
    RenderFeatureSymbolizerDescriptor, RenderFillDescriptor, RenderFillRule, RenderFogDescriptor,
    RenderFogFalloffDescriptor, RenderFogVolumeDescriptor, RenderFogVolumeOffsetDescriptor,
    RenderGeometryPayload, RenderGradient, RenderIblCubemapsDescriptor,
    RenderIblEnvironmentMapDescriptor, RenderImageBasedLightingDescriptor,
    RenderImageBasedLightingSourceDescriptor, RenderIrradianceVolumeDescriptor, RenderLabelAnchor,
    RenderLabelCollisionPolicy, RenderLabelDescriptor, RenderLabelPlacement, RenderLabelTextSource,
    RenderLayerDescriptor, RenderLayerGroupChild, RenderLayerGroupDescriptor, RenderLayerKind,
    RenderLayerSpec, RenderLayerTimeFilter, RenderLightProbeDescriptor, RenderMaterialAlphaMode,
    RenderMaterialFamily, RenderMaterialIntentDescriptor, RenderMaterialTextureBindingDescriptor,
    RenderMaterialTextureSemantic, RenderModelLayerDescriptor, RenderModelOrientationDescriptor,
    RenderModelOriginDescriptor, RenderModelPlacementDescriptor, RenderMotionBlurDescriptor,
    RenderMsaaDescriptor, RenderNumericExpression, RenderNumericInterpolationMode,
    RenderNumericInterpolationStopDescriptor, RenderNumericStepStopDescriptor, RenderOpaqueMethod,
    RenderOutputTargetDescriptor, RenderPickHitDescriptor, RenderPipelineDomain,
    RenderPipelineIntentDescriptor, RenderPointShape, RenderPointSymbolContent,
    RenderPointSymbolDescriptor, RenderPrepassKind, RenderProbeRegionDescriptor,
    RenderRasterBandSelection, RenderRasterColorRampDescriptor, RenderRasterDerivedProduct,
    RenderRasterGeoreferenceDescriptor, RenderRasterImageMetadataDescriptor,
    RenderRasterLayerDescriptor, RenderRasterSampling, RenderRasterSourceDescriptor,
    RenderRasterSourceImageInfo, RenderRasterStyleDescriptor, RenderRasterValueTransform,
    RenderResolutionOverrideDescriptor, RenderScaleRangeDescriptor, RenderSceneDescriptor,
    RenderSceneEnvironmentDescriptor, RenderSceneSpec, RenderSceneUpdateDescriptor,
    RenderSceneUpdateKind, RenderScreenPointDescriptor, RenderScreenRectDescriptor,
    RenderScreenSpaceAmbientOcclusionDescriptor, RenderScreenSpaceAmbientOcclusionQuality,
    RenderScreenSpaceReflectionsDescriptor, RenderScreenSpaceTransmissionQuality,
    RenderShadowFilteringMethod, RenderShadowParticipationDescriptor, RenderSkyboxDescriptor,
    RenderStrokeDescriptor, RenderSubViewLayoutDescriptor, RenderSunDiskDescriptor,
    RenderTerrainDrapeMode, RenderTerrainLayerDescriptor, RenderTerrainLodDescriptor,
    RenderTerrainMeshMode, RenderTerrainOverlayBlendMode, RenderTerrainOverlayDescriptor,
    RenderTerrainOverlaySource, RenderTerrainShadingMode, RenderTerrainStyleDescriptor,
    RenderTileAddressingScheme, RenderTileLayerDescriptor, RenderTileMissingTilePolicy,
    RenderTilePayloadKind, RenderTileRefinementPriority, RenderTileRetryPolicyDescriptor,
    RenderTileSourceDescriptor, RenderTileStreamingDescriptor, RenderTileStyleDescriptor,
    RenderTimeContext, RenderTonemappingDescriptor, RenderUpAxis, RenderVectorLayerDescriptor,
    RenderVectorLayerPayload, RenderViewDescriptor, RenderViewOutputDescriptor,
    RenderViewParticipationDescriptor, RenderViewportDescriptor, RenderVolumetricFogDescriptor,
    RenderWorldOrigin, RenderWorldSpaceDescriptor,
};
pub use rfc7946::{
    GeoJsonDocumentDescriptor, GeoJsonFeatureCollectionDescriptor, GeoJsonFeatureDescriptor,
    GeoJsonFeatureId, GeoJsonGeometryDescriptor, GeoJsonGeometryKind, GeoJsonPosition,
};
