//! Renderer-agnostic GIS render propositions.
//!
//! The render seam in `elicit_gis` is an internal contract vocabulary rather
//! than a transcription of one external standard. It names the propositions a
//! downstream renderer needs in order to consume georust payloads
//! mechanically, while keeping the auditable proof-minting surface at trait
//! boundaries in implementation crates.

mod emit_impls {
    use elicitation::contracts::Prop;
    use elicitation::proc_macro2::TokenStream;
    use elicitation::quote::quote;

    macro_rules! structural_prop {
        ($t:ty) => {
            impl Prop for $t {
                fn kani_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn verus_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
                fn creusot_proof() -> TokenStream {
                    quote! { /* structural */ }
                }
            }
        };
    }

    /// The render view center coordinates are finite numbers.
    pub struct RenderViewCenterFinite;
    structural_prop!(RenderViewCenterFinite);

    /// The render view resolution is strictly positive.
    pub struct RenderViewResolutionPositive;
    structural_prop!(RenderViewResolutionPositive);

    /// The render view viewport dimensions are strictly positive.
    pub struct RenderViewViewportPositive;
    structural_prop!(RenderViewViewportPositive);

    /// The render view rotation and pitch values are finite numbers.
    pub struct RenderViewOrientationFinite;
    structural_prop!(RenderViewOrientationFinite);

    /// The render view declares which CRS it is interpreting.
    pub struct RenderViewCrsDeclared;
    structural_prop!(RenderViewCrsDeclared);

    /// The render view explicitly declares its camera projection policy.
    pub struct RenderViewProjectionDeclared;
    structural_prop!(RenderViewProjectionDeclared);

    /// Camera clipping-plane parameters are finite.
    pub struct CameraClipRangeFinite;
    structural_prop!(CameraClipRangeFinite);

    /// Camera clipping planes are strictly ordered.
    pub struct CameraClipRangeOrdered;
    structural_prop!(CameraClipRangeOrdered);

    /// Perspective camera field-of-view values are positive.
    pub struct PerspectiveFieldOfViewPositive;
    structural_prop!(PerspectiveFieldOfViewPositive);

    /// A render view explicitly declares its output policy when one is present.
    pub struct RenderViewOutputDeclared;
    structural_prop!(RenderViewOutputDeclared);

    /// HDR output policy is explicitly declared.
    pub struct HdrOutputDeclared;
    structural_prop!(HdrOutputDeclared);

    /// Tonemapping policy is explicitly declared.
    pub struct TonemappingDeclared;
    structural_prop!(TonemappingDeclared);

    /// MSAA policy is explicitly declared.
    pub struct MsaaDeclared;
    structural_prop!(MsaaDeclared);

    /// Mip-bias values are finite when supplied.
    pub struct MipBiasFinite;
    structural_prop!(MipBiasFinite);

    /// A render scene identifier is non-empty.
    pub struct SceneIdNonEmpty;
    structural_prop!(SceneIdNonEmpty);

    /// A render layer identifier is non-empty.
    pub struct LayerIdNonEmpty;
    structural_prop!(LayerIdNonEmpty);

    /// A render layer name is non-empty.
    pub struct LayerNameNonEmpty;
    structural_prop!(LayerNameNonEmpty);

    /// Layer opacity is within the closed unit interval.
    pub struct LayerOpacityUnitInterval;
    structural_prop!(LayerOpacityUnitInterval);

    /// Layer draw order is explicitly assigned.
    pub struct LayerDrawOrderAssigned;
    structural_prop!(LayerDrawOrderAssigned);

    /// Layer visibility policy is explicitly declared.
    pub struct LayerVisibilityDeclared;
    structural_prop!(LayerVisibilityDeclared);

    /// The render world origin parameters are finite when explicitly supplied.
    pub struct WorldOriginFinite;
    structural_prop!(WorldOriginFinite);

    /// World units-per-meter is strictly positive.
    pub struct WorldUnitsPerMeterPositive;
    structural_prop!(WorldUnitsPerMeterPositive);

    /// World vertical exaggeration is strictly positive.
    pub struct WorldVerticalExaggerationPositive;
    structural_prop!(WorldVerticalExaggerationPositive);

    /// An external render asset declares a stable URI.
    pub struct AssetUriDeclared;
    structural_prop!(AssetUriDeclared);

    /// An external render asset's logical width is positive when supplied.
    pub struct AssetLogicalWidthPositive;
    structural_prop!(AssetLogicalWidthPositive);

    /// An external render asset's logical height is positive when supplied.
    pub struct AssetLogicalHeightPositive;
    structural_prop!(AssetLogicalHeightPositive);

    /// An external render asset's color-space intent is explicitly declared.
    pub struct AssetColorSpaceDeclared;
    structural_prop!(AssetColorSpaceDeclared);

    /// An external render asset's residency intent is explicitly declared.
    pub struct AssetResidencyPolicyDeclared;
    structural_prop!(AssetResidencyPolicyDeclared);

    /// An external render asset's access/update pattern is explicitly declared.
    pub struct AssetAccessPatternDeclared;
    structural_prop!(AssetAccessPatternDeclared);

    /// An external render asset's revision metadata is explicitly declared.
    pub struct AssetRevisionDeclared;
    structural_prop!(AssetRevisionDeclared);

    /// An external render asset's revision metadata is non-empty when supplied.
    pub struct AssetRevisionNonEmpty;
    structural_prop!(AssetRevisionNonEmpty);

    /// An external render asset's integrity metadata is explicitly declared.
    pub struct AssetIntegrityDeclared;
    structural_prop!(AssetIntegrityDeclared);

    /// An external render asset's integrity digest is non-empty when supplied.
    pub struct AssetIntegrityDigestNonEmpty;
    structural_prop!(AssetIntegrityDigestNonEmpty);

    /// A material family is explicitly declared.
    pub struct MaterialFamilyDeclared;
    structural_prop!(MaterialFamilyDeclared);

    /// A material alpha-compositing mode is explicitly declared when supplied.
    pub struct MaterialAlphaModeDeclared;
    structural_prop!(MaterialAlphaModeDeclared);

    /// Material alpha-mask thresholds stay in the closed unit interval.
    pub struct MaterialMaskThresholdUnitInterval;
    structural_prop!(MaterialMaskThresholdUnitInterval);

    /// A material face-culling mode is explicitly declared when supplied.
    pub struct MaterialCullModeDeclared;
    structural_prop!(MaterialCullModeDeclared);

    /// Material depth-bias values are finite when supplied.
    pub struct MaterialDepthBiasFinite;
    structural_prop!(MaterialDepthBiasFinite);

    /// Material specialization keys are non-empty when supplied.
    pub struct MaterialSpecializationKeyNonEmpty;
    structural_prop!(MaterialSpecializationKeyNonEmpty);

    /// A material texture semantic is explicitly declared.
    pub struct MaterialTextureSemanticDeclared;
    structural_prop!(MaterialTextureSemanticDeclared);

    /// A material pipeline domain is explicitly declared when supplied.
    pub struct MaterialPipelineDomainDeclared;
    structural_prop!(MaterialPipelineDomainDeclared);

    /// A material opaque rendering method is explicitly declared when supplied.
    pub struct MaterialOpaqueMethodDeclared;
    structural_prop!(MaterialOpaqueMethodDeclared);

    /// Material prepass entries are unique within one pipeline intent.
    pub struct MaterialPrepassesUnique;
    structural_prop!(MaterialPrepassesUnique);

    /// A material family is compatible with its chosen pipeline domain.
    pub struct MaterialPipelineFamilyCompatible;
    structural_prop!(MaterialPipelineFamilyCompatible);

    /// A chosen opaque method is compatible with its chosen pipeline domain.
    pub struct MaterialOpaqueMethodDomainCompatible;
    structural_prop!(MaterialOpaqueMethodDomainCompatible);

    /// Requested prepasses are compatible with the chosen pipeline domain.
    pub struct MaterialPrepassesDomainCompatible;
    structural_prop!(MaterialPrepassesDomainCompatible);

    /// A render output target is explicitly declared when supplied.
    pub struct RenderOutputTargetDeclared;
    structural_prop!(RenderOutputTargetDeclared);

    /// A symbolic render output target identifier is non-empty when supplied.
    pub struct RenderOutputTargetIdNonEmpty;
    structural_prop!(RenderOutputTargetIdNonEmpty);

    /// A render clear policy is explicitly declared when supplied.
    pub struct RenderClearPolicyDeclared;
    structural_prop!(RenderClearPolicyDeclared);

    /// A per-view depth clear value is finite when supplied.
    pub struct RenderClearDepthFinite;
    structural_prop!(RenderClearDepthFinite);

    /// Stroke width is a finite number.
    pub struct StrokeWidthFinite;
    structural_prop!(StrokeWidthFinite);

    /// Stroke width is non-negative.
    pub struct StrokeWidthNonNegative;
    structural_prop!(StrokeWidthNonNegative);

    /// Stroke floating-point parameters are finite.
    pub struct StrokeParametersFinite;
    structural_prop!(StrokeParametersFinite);

    /// Fill rule is explicitly declared.
    pub struct FillRuleDeclared;
    structural_prop!(FillRuleDeclared);

    /// Point symbol size is strictly positive.
    pub struct PointSymbolSizePositive;
    structural_prop!(PointSymbolSizePositive);

    /// Point symbol content is explicitly declared.
    pub struct PointSymbolSourceDeclared;
    structural_prop!(PointSymbolSourceDeclared);

    /// Point symbol transform parameters are finite.
    pub struct PointSymbolTransformFinite;
    structural_prop!(PointSymbolTransformFinite);

    /// A label text source is explicitly declared.
    pub struct LabelTextSourceDeclared;
    structural_prop!(LabelTextSourceDeclared);

    /// Label priority is a finite number.
    pub struct LabelPriorityFinite;
    structural_prop!(LabelPriorityFinite);

    /// Label transform parameters are finite.
    pub struct LabelTransformFinite;
    structural_prop!(LabelTransformFinite);

    /// Label collision policy is explicitly declared.
    pub struct LabelCollisionPolicyDeclared;
    structural_prop!(LabelCollisionPolicyDeclared);

    /// Symbolizer paint choices are representable by the render IR.
    pub struct SymbolizerUsesRenderablePaint;
    structural_prop!(SymbolizerUsesRenderablePaint);

    /// A feature-style predicate is explicitly declared.
    pub struct FeaturePredicateDeclared;
    structural_prop!(FeaturePredicateDeclared);

    /// A numeric feature expression is explicitly declared.
    pub struct NumericExpressionDeclared;
    structural_prop!(NumericExpressionDeclared);

    /// Any literal parameters embedded in a numeric expression are finite.
    pub struct NumericExpressionParametersFinite;
    structural_prop!(NumericExpressionParametersFinite);

    /// Variadic numeric-expression operators carry at least one operand.
    pub struct NumericExpressionOperandsPresent;
    structural_prop!(NumericExpressionOperandsPresent);

    /// Step-expression thresholds are ordered monotonically.
    pub struct NumericStepStopsOrdered;
    structural_prop!(NumericStepStopsOrdered);

    /// Interpolation-expression stops are ordered monotonically.
    pub struct NumericInterpolationStopsOrdered;
    structural_prop!(NumericInterpolationStopsOrdered);

    /// Exponential interpolation uses a strictly positive base.
    pub struct NumericInterpolationBasePositive;
    structural_prop!(NumericInterpolationBasePositive);

    /// Scale-range bounds are finite when supplied.
    pub struct ScaleRangeFinite;
    structural_prop!(ScaleRangeFinite);

    /// Scale-range bounds are ordered when both bounds are supplied.
    pub struct ScaleRangeOrdered;
    structural_prop!(ScaleRangeOrdered);

    /// Placement altitude policy is explicitly declared.
    pub struct PlacementAltitudeModeDeclared;
    structural_prop!(PlacementAltitudeModeDeclared);

    /// Placement parameters are finite when supplied.
    pub struct PlacementParametersFinite;
    structural_prop!(PlacementParametersFinite);

    /// Extrusion heights are explicitly declared.
    pub struct ExtrusionHeightDeclared;
    structural_prop!(ExtrusionHeightDeclared);

    /// Extrusion heights are non-negative when interpreted by the backend.
    pub struct ExtrusionHeightNonNegative;
    structural_prop!(ExtrusionHeightNonNegative);

    /// Shadow-participation policy is explicitly declared.
    pub struct ShadowParticipationPolicyDeclared;
    structural_prop!(ShadowParticipationPolicyDeclared);

    /// Raster opacity is within the closed unit interval.
    pub struct RasterOpacityUnitInterval;
    structural_prop!(RasterOpacityUnitInterval);

    /// Selected raster bands are valid for the source image.
    pub struct RasterBandSelectionValid;
    structural_prop!(RasterBandSelectionValid);

    /// Raster sampling policy is explicitly declared.
    pub struct RasterSamplingDeclared;
    structural_prop!(RasterSamplingDeclared);

    /// Raster color-ramp domain bounds are finite.
    pub struct RasterColorRampDomainFinite;
    structural_prop!(RasterColorRampDomainFinite);

    /// Raster color-ramp domain bounds are strictly ordered.
    pub struct RasterColorRampDomainOrdered;
    structural_prop!(RasterColorRampDomainOrdered);

    /// Raster value-transform parameters are finite.
    pub struct RasterValueTransformFinite;
    structural_prop!(RasterValueTransformFinite);

    /// Raster normalization domains are ordered.
    pub struct RasterNormalizeDomainOrdered;
    structural_prop!(RasterNormalizeDomainOrdered);

    /// Raster gamma values are strictly positive.
    pub struct RasterGammaPositive;
    structural_prop!(RasterGammaPositive);

    /// Raster derived-product parameters are finite.
    pub struct RasterDerivedProductParametersFinite;
    structural_prop!(RasterDerivedProductParametersFinite);

    /// Raster contour intervals are strictly positive.
    pub struct RasterContourIntervalPositive;
    structural_prop!(RasterContourIntervalPositive);

    /// Raster normal-map strength values are strictly positive.
    pub struct RasterNormalStrengthPositive;
    structural_prop!(RasterNormalStrengthPositive);

    /// Raster source metadata is explicitly provided to the layer factory.
    pub struct RasterSourceDeclared;
    structural_prop!(RasterSourceDeclared);

    /// Raster image metadata is explicitly provided to the layer factory.
    pub struct RasterImageMetadataDeclared;
    structural_prop!(RasterImageMetadataDeclared);

    /// Raster dimensions are strictly positive when declared.
    pub struct RasterDimensionsPositive;
    structural_prop!(RasterDimensionsPositive);

    /// Raster world extent is finite and renderable.
    pub struct RasterExtentFinite;
    structural_prop!(RasterExtentFinite);

    /// Raster world extent bounds are ordered.
    pub struct RasterExtentOrdered;
    structural_prop!(RasterExtentOrdered);

    /// Tile URI template is explicitly declared.
    pub struct TileUriTemplateDeclared;
    structural_prop!(TileUriTemplateDeclared);

    /// Tile zoom bounds are ordered.
    pub struct TileZoomRangeOrdered;
    structural_prop!(TileZoomRangeOrdered);

    /// Tile dimensions are strictly positive.
    pub struct TileDimensionsPositive;
    structural_prop!(TileDimensionsPositive);

    /// Tile streaming/refinement policy is explicitly declared.
    pub struct TileStreamingPolicyDeclared;
    structural_prop!(TileStreamingPolicyDeclared);

    /// Tile refinement priority is explicitly declared.
    pub struct TileRefinementPriorityDeclared;
    structural_prop!(TileRefinementPriorityDeclared);

    /// Tile request concurrency limits are strictly positive.
    pub struct TileConcurrentRequestsPositive;
    structural_prop!(TileConcurrentRequestsPositive);

    /// Tile cache budgets are strictly positive when supplied.
    pub struct TileCacheBudgetPositive;
    structural_prop!(TileCacheBudgetPositive);

    /// Tile retry backoff intervals are strictly positive when retries are enabled.
    pub struct TileRetryBackoffPositive;
    structural_prop!(TileRetryBackoffPositive);

    /// Tile transition fade durations are strictly positive when supplied.
    pub struct TileTransitionFadePositive;
    structural_prop!(TileTransitionFadePositive);

    /// Tile style opacity is within the closed unit interval.
    pub struct TileOpacityUnitInterval;
    structural_prop!(TileOpacityUnitInterval);

    /// Tile style is compatible with the tile payload family.
    pub struct TileStylePayloadCompatible;
    structural_prop!(TileStylePayloadCompatible);

    /// Terrain vertical scale is strictly positive.
    pub struct TerrainVerticalScalePositive;
    structural_prop!(TerrainVerticalScalePositive);

    /// Terrain base-height offsets are finite.
    pub struct TerrainHeightOffsetFinite;
    structural_prop!(TerrainHeightOffsetFinite);

    /// Terrain surface drape policy is explicitly declared when supplied.
    pub struct TerrainSurfaceDrapeModeDeclared;
    structural_prop!(TerrainSurfaceDrapeModeDeclared);

    /// Terrain mesh policy is explicitly declared.
    pub struct TerrainMeshModeDeclared;
    structural_prop!(TerrainMeshModeDeclared);

    /// Terrain shading policy is explicitly declared.
    pub struct TerrainShadingModeDeclared;
    structural_prop!(TerrainShadingModeDeclared);

    /// Terrain LOD screen-space error is strictly positive.
    pub struct TerrainLodScreenSpaceErrorPositive;
    structural_prop!(TerrainLodScreenSpaceErrorPositive);

    /// Terrain LOD skirt heights are non-negative when supplied.
    pub struct TerrainSkirtHeightNonNegative;
    structural_prop!(TerrainSkirtHeightNonNegative);

    /// Terrain overlay opacity is within the closed unit interval.
    pub struct TerrainOverlayOpacityUnitInterval;
    structural_prop!(TerrainOverlayOpacityUnitInterval);

    /// Terrain overlay blend policy is explicitly declared.
    pub struct TerrainOverlayBlendModeDeclared;
    structural_prop!(TerrainOverlayBlendModeDeclared);

    /// Terrain overlay drape policy is explicitly declared.
    pub struct TerrainOverlayDrapeModeDeclared;
    structural_prop!(TerrainOverlayDrapeModeDeclared);

    /// Terrain overlay height-bias values are finite.
    pub struct TerrainOverlayHeightBiasFinite;
    structural_prop!(TerrainOverlayHeightBiasFinite);

    /// Annotation identifier is non-empty.
    pub struct AnnotationIdNonEmpty;
    structural_prop!(AnnotationIdNonEmpty);

    /// Annotation anchor coordinates are finite.
    pub struct AnnotationAnchorFinite;
    structural_prop!(AnnotationAnchorFinite);

    /// Annotation payload is explicitly declared.
    pub struct AnnotationPayloadDeclared;
    structural_prop!(AnnotationPayloadDeclared);

    /// Vector payload is explicitly provided to the layer factory.
    pub struct VectorPayloadDeclared;
    structural_prop!(VectorPayloadDeclared);

    /// Vector geometry has been projected or interpreted for the active view.
    pub struct GeometryProjectedForView;
    structural_prop!(GeometryProjectedForView);

    /// Vector symbolization has been fully resolved before rendering.
    pub struct VectorSymbolizationResolved;
    structural_prop!(VectorSymbolizationResolved);

    /// The chosen vector style is compatible with the payload family.
    pub struct VectorStylePayloadCompatible;
    structural_prop!(VectorStylePayloadCompatible);

    /// Vector batching / instancing policy is explicitly declared.
    pub struct FeatureBatchingPolicyDeclared;
    structural_prop!(FeatureBatchingPolicyDeclared);

    /// Vector batching feature-count limits are strictly positive when supplied.
    pub struct FeatureBatchLimitPositive;
    structural_prop!(FeatureBatchLimitPositive);

    /// Vector batching group-key property names are non-empty.
    pub struct FeatureBatchKeyPropertyNonEmpty;
    structural_prop!(FeatureBatchKeyPropertyNonEmpty);

    /// A scene explicitly declares the layer list it contains.
    pub struct SceneLayerListDeclared;
    structural_prop!(SceneLayerListDeclared);

    /// A scene explicitly declares which validated view it uses.
    pub struct SceneViewDeclared;
    structural_prop!(SceneViewDeclared);

    /// Scene layer ordering is deterministic.
    pub struct LayerOrderDeterministic;
    structural_prop!(LayerOrderDeterministic);

    /// Ambient-light brightness is non-negative.
    pub struct AmbientLightBrightnessNonNegative;
    structural_prop!(AmbientLightBrightnessNonNegative);

    /// Directional-light illuminance is non-negative.
    pub struct DirectionalLightIlluminanceNonNegative;
    structural_prop!(DirectionalLightIlluminanceNonNegative);

    /// Directional-light angles are finite.
    pub struct LightAnglesFinite;
    structural_prop!(LightAnglesFinite);

    /// Explicit cascade-shadow far bounds are strictly positive when supplied.
    pub struct CascadeShadowBoundsPositive;
    structural_prop!(CascadeShadowBoundsPositive);

    /// Explicit cascade-shadow far bounds are strictly ordered when supplied.
    pub struct CascadeShadowBoundsOrdered;
    structural_prop!(CascadeShadowBoundsOrdered);

    /// Cascade-shadow overlap proportions stay in the half-open unit interval.
    pub struct CascadeShadowOverlapHalfOpenUnitInterval;
    structural_prop!(CascadeShadowOverlapHalfOpenUnitInterval);

    /// Minimum cascade-shadow distance is non-negative when supplied.
    pub struct CascadeShadowMinimumDistanceNonNegative;
    structural_prop!(CascadeShadowMinimumDistanceNonNegative);

    /// Maximum cascade-shadow distance is strictly greater than minimum distance when supplied.
    pub struct CascadeShadowMaximumDistanceOrdered;
    structural_prop!(CascadeShadowMaximumDistanceOrdered);

    /// First cascade far bound is strictly positive when supplied.
    pub struct CascadeShadowFirstCascadeFarBoundPositive;
    structural_prop!(CascadeShadowFirstCascadeFarBoundPositive);

    /// First cascade far bound is strictly greater than minimum distance when both are supplied.
    pub struct CascadeShadowFirstCascadeFarBoundGreaterThanMinimum;
    structural_prop!(CascadeShadowFirstCascadeFarBoundGreaterThanMinimum);

    /// Cascade-shadow cascade counts are strictly positive when supplied.
    pub struct CascadeShadowCascadeCountPositive;
    structural_prop!(CascadeShadowCascadeCountPositive);

    /// Fog distances are finite.
    pub struct FogRangeFinite;
    structural_prop!(FogRangeFinite);

    /// Fog distances are ordered.
    pub struct FogRangeOrdered;
    structural_prop!(FogRangeOrdered);

    /// Fog falloff policy is explicitly declared.
    pub struct FogFalloffDeclared;
    structural_prop!(FogFalloffDeclared);

    /// Fog directional-light exponents are non-negative when supplied.
    pub struct FogDirectionalLightExponentNonNegative;
    structural_prop!(FogDirectionalLightExponentNonNegative);

    /// Exponential fog density values are non-negative.
    pub struct FogDensityNonNegative;
    structural_prop!(FogDensityNonNegative);

    /// Atmospheric fog coefficients are finite.
    pub struct AtmosphericFogCoefficientsFinite;
    structural_prop!(AtmosphericFogCoefficientsFinite);

    /// Volumetric fog floating-point parameters are finite when supplied.
    pub struct VolumetricFogParametersFinite;
    structural_prop!(VolumetricFogParametersFinite);

    /// Volumetric fog step counts are strictly positive when supplied.
    pub struct VolumetricFogStepCountPositive;
    structural_prop!(VolumetricFogStepCountPositive);

    /// Fog-volume floating-point parameters are finite when supplied.
    pub struct FogVolumeParametersFinite;
    structural_prop!(FogVolumeParametersFinite);

    /// Fog-volume density factors are non-negative when supplied.
    pub struct FogVolumeDensityFactorNonNegative;
    structural_prop!(FogVolumeDensityFactorNonNegative);

    /// Fog-volume density-texture offsets are finite when supplied.
    pub struct FogVolumeDensityTextureOffsetFinite;
    structural_prop!(FogVolumeDensityTextureOffsetFinite);

    /// Fog-volume absorption coefficients are non-negative when supplied.
    pub struct FogVolumeAbsorptionNonNegative;
    structural_prop!(FogVolumeAbsorptionNonNegative);

    /// Fog-volume scattering coefficients are non-negative when supplied.
    pub struct FogVolumeScatteringNonNegative;
    structural_prop!(FogVolumeScatteringNonNegative);

    /// Fog-volume anisotropy factors are finite when supplied.
    pub struct FogVolumeScatteringAsymmetryFinite;
    structural_prop!(FogVolumeScatteringAsymmetryFinite);

    /// Fog-volume light-intensity multipliers are non-negative when supplied.
    pub struct FogVolumeLightIntensityNonNegative;
    structural_prop!(FogVolumeLightIntensityNonNegative);

    /// Fog-volume density-texture assets are valid when supplied.
    pub struct FogVolumeDensityTextureAssetValid;
    structural_prop!(FogVolumeDensityTextureAssetValid);

    /// Color-grading floating-point parameters are finite.
    pub struct ColorGradingParametersFinite;
    structural_prop!(ColorGradingParametersFinite);

    /// Color-grading midtones ranges are ordered.
    pub struct ColorGradingMidtonesRangeOrdered;
    structural_prop!(ColorGradingMidtonesRangeOrdered);

    /// Auto-exposure EV ranges are ordered.
    pub struct AutoExposureRangeOrdered;
    structural_prop!(AutoExposureRangeOrdered);

    /// Auto-exposure filter windows are ordered.
    pub struct AutoExposureFilterOrdered;
    structural_prop!(AutoExposureFilterOrdered);

    /// Auto-exposure speed parameters are non-negative.
    pub struct AutoExposureSpeedNonNegative;
    structural_prop!(AutoExposureSpeedNonNegative);

    /// Auto-exposure transition distances are non-negative.
    pub struct AutoExposureTransitionDistanceNonNegative;
    structural_prop!(AutoExposureTransitionDistanceNonNegative);

    /// Auto-exposure metering-mask assets are valid when supplied.
    pub struct AutoExposureMeteringMaskAssetValid;
    structural_prop!(AutoExposureMeteringMaskAssetValid);

    /// Auto-exposure compensation-curve assets are valid when supplied.
    pub struct AutoExposureCompensationCurveAssetValid;
    structural_prop!(AutoExposureCompensationCurveAssetValid);

    /// Fixed exposure mode is explicitly declared.
    pub struct ExposureModeDeclared;
    structural_prop!(ExposureModeDeclared);

    /// Explicit EV100 exposure values are finite.
    pub struct ExposureEv100Finite;
    structural_prop!(ExposureEv100Finite);

    /// Fixed exposure and auto-exposure policies do not conflict.
    pub struct ExposurePolicyExclusive;
    structural_prop!(ExposurePolicyExclusive);

    /// Bloom floating-point parameters are finite.
    pub struct BloomParametersFinite;
    structural_prop!(BloomParametersFinite);

    /// Bloom prefilter threshold softness values stay in the unit interval.
    pub struct BloomThresholdSoftnessUnitInterval;
    structural_prop!(BloomThresholdSoftnessUnitInterval);

    /// Bloom scale factors are finite.
    pub struct BloomScaleFinite;
    structural_prop!(BloomScaleFinite);

    /// Bloom composite policy is explicitly declared.
    pub struct BloomCompositeModeDeclared;
    structural_prop!(BloomCompositeModeDeclared);

    /// Depth-of-field mode is explicitly declared.
    pub struct DepthOfFieldModeDeclared;
    structural_prop!(DepthOfFieldModeDeclared);

    /// Depth-of-field floating-point parameters are finite.
    pub struct DepthOfFieldParametersFinite;
    structural_prop!(DepthOfFieldParametersFinite);

    /// Depth-of-field focal distance is non-negative.
    pub struct DepthOfFieldFocalDistanceNonNegative;
    structural_prop!(DepthOfFieldFocalDistanceNonNegative);

    /// Depth-of-field sensor height is strictly positive.
    pub struct DepthOfFieldSensorHeightPositive;
    structural_prop!(DepthOfFieldSensorHeightPositive);

    /// Depth-of-field aperture values are strictly positive.
    pub struct DepthOfFieldApertureFStopsPositive;
    structural_prop!(DepthOfFieldApertureFStopsPositive);

    /// Depth-of-field circle-of-confusion diameters are non-negative.
    pub struct DepthOfFieldCircleOfConfusionNonNegative;
    structural_prop!(DepthOfFieldCircleOfConfusionNonNegative);

    /// Depth-of-field max-depth values are non-negative.
    pub struct DepthOfFieldMaxDepthNonNegative;
    structural_prop!(DepthOfFieldMaxDepthNonNegative);

    /// Motion-blur floating-point parameters are finite.
    pub struct MotionBlurParametersFinite;
    structural_prop!(MotionBlurParametersFinite);

    /// Motion-blur sample counts are strictly positive.
    pub struct MotionBlurSamplesPositive;
    structural_prop!(MotionBlurSamplesPositive);

    /// Chromatic-aberration floating-point parameters are finite.
    pub struct ChromaticAberrationParametersFinite;
    structural_prop!(ChromaticAberrationParametersFinite);

    /// Chromatic-aberration sample counts are strictly positive.
    pub struct ChromaticAberrationSamplesPositive;
    structural_prop!(ChromaticAberrationSamplesPositive);

    /// Chromatic-aberration color-LUT assets are valid when supplied.
    pub struct ChromaticAberrationColorLutAssetValid;
    structural_prop!(ChromaticAberrationColorLutAssetValid);

    /// Screen-space reflection floating-point parameters are finite.
    pub struct ScreenSpaceReflectionsParametersFinite;
    structural_prop!(ScreenSpaceReflectionsParametersFinite);

    /// Screen-space reflection roughness thresholds stay in the unit interval.
    pub struct ScreenSpaceReflectionsRoughnessThresholdUnitInterval;
    structural_prop!(ScreenSpaceReflectionsRoughnessThresholdUnitInterval);

    /// Screen-space reflection thickness values are non-negative.
    pub struct ScreenSpaceReflectionsThicknessNonNegative;
    structural_prop!(ScreenSpaceReflectionsThicknessNonNegative);

    /// Screen-space reflection linear-step counts are strictly positive.
    pub struct ScreenSpaceReflectionsLinearStepsPositive;
    structural_prop!(ScreenSpaceReflectionsLinearStepsPositive);

    /// Screen-space reflection march exponents are non-negative.
    pub struct ScreenSpaceReflectionsLinearMarchExponentNonNegative;
    structural_prop!(ScreenSpaceReflectionsLinearMarchExponentNonNegative);

    /// Screen-space reflection bisection-step counts are strictly positive.
    pub struct ScreenSpaceReflectionsBisectionStepsPositive;
    structural_prop!(ScreenSpaceReflectionsBisectionStepsPositive);

    /// Screen-space ambient-occlusion quality is explicitly declared.
    pub struct ScreenSpaceAmbientOcclusionQualityDeclared;
    structural_prop!(ScreenSpaceAmbientOcclusionQualityDeclared);

    /// Screen-space ambient-occlusion floating-point parameters are finite.
    pub struct ScreenSpaceAmbientOcclusionParametersFinite;
    structural_prop!(ScreenSpaceAmbientOcclusionParametersFinite);

    /// Screen-space ambient-occlusion thickness values are non-negative.
    pub struct ScreenSpaceAmbientOcclusionThicknessNonNegative;
    structural_prop!(ScreenSpaceAmbientOcclusionThicknessNonNegative);

    /// Custom SSAO slice counts are strictly positive when supplied.
    pub struct ScreenSpaceAmbientOcclusionCustomSliceCountPositive;
    structural_prop!(ScreenSpaceAmbientOcclusionCustomSliceCountPositive);

    /// Custom SSAO samples-per-slice-side counts are strictly positive when supplied.
    pub struct ScreenSpaceAmbientOcclusionCustomSamplesPerSlicePositive;
    structural_prop!(ScreenSpaceAmbientOcclusionCustomSamplesPerSlicePositive);

    /// Shadow-filtering method is explicitly declared.
    pub struct ShadowFilteringMethodDeclared;
    structural_prop!(ShadowFilteringMethodDeclared);

    /// Screen-space transmission quality is explicitly declared.
    pub struct ScreenSpaceTransmissionQualityDeclared;
    structural_prop!(ScreenSpaceTransmissionQualityDeclared);

    /// Skybox brightness is non-negative.
    pub struct SkyboxBrightnessNonNegative;
    structural_prop!(SkyboxBrightnessNonNegative);

    /// Skybox rotation values are finite when supplied.
    pub struct SkyboxRotationFinite;
    structural_prop!(SkyboxRotationFinite);

    /// Probe-region center coordinates are finite.
    pub struct ProbeRegionCenterFinite;
    structural_prop!(ProbeRegionCenterFinite);

    /// Probe-region extents are strictly positive.
    pub struct ProbeRegionExtentsPositive;
    structural_prop!(ProbeRegionExtentsPositive);

    /// Irradiance-volume intensity is non-negative when supplied.
    pub struct IrradianceVolumeIntensityNonNegative;
    structural_prop!(IrradianceVolumeIntensityNonNegative);

    /// Irradiance-volume voxel assets are renderable.
    pub struct IrradianceVolumeVoxelsAssetValid;
    structural_prop!(IrradianceVolumeVoxelsAssetValid);

    /// Image-based-lighting source policy is explicitly declared.
    pub struct ImageBasedLightingSourceDeclared;
    structural_prop!(ImageBasedLightingSourceDeclared);

    /// Image-based-lighting intensity is non-negative when supplied.
    pub struct ImageBasedLightingIntensityNonNegative;
    structural_prop!(ImageBasedLightingIntensityNonNegative);

    /// Image-based-lighting rotation values are finite when supplied.
    pub struct ImageBasedLightingRotationFinite;
    structural_prop!(ImageBasedLightingRotationFinite);

    /// Image-based-lighting diffuse cubemap assets are renderable.
    pub struct ImageBasedLightingDiffuseMapAssetValid;
    structural_prop!(ImageBasedLightingDiffuseMapAssetValid);

    /// Image-based-lighting specular cubemap assets are renderable.
    pub struct ImageBasedLightingSpecularMapAssetValid;
    structural_prop!(ImageBasedLightingSpecularMapAssetValid);

    /// Image-based-lighting environment-map assets are renderable.
    pub struct ImageBasedLightingEnvironmentMapAssetValid;
    structural_prop!(ImageBasedLightingEnvironmentMapAssetValid);

    /// Atmosphere-generated cubemap dimensions are strictly positive.
    pub struct ImageBasedLightingAtmosphereCubemapDimensionsPositive;
    structural_prop!(ImageBasedLightingAtmosphereCubemapDimensionsPositive);

    /// Atmosphere shell radii are strictly positive.
    pub struct AtmosphereShellRadiiPositive;
    structural_prop!(AtmosphereShellRadiiPositive);

    /// Atmosphere shell radii are strictly ordered.
    pub struct AtmosphereShellRadiiOrdered;
    structural_prop!(AtmosphereShellRadiiOrdered);

    /// Atmosphere ground-albedo values are finite.
    pub struct AtmosphereGroundAlbedoFinite;
    structural_prop!(AtmosphereGroundAlbedoFinite);

    /// Atmosphere ground-albedo values stay in the unit interval.
    pub struct AtmosphereGroundAlbedoUnitInterval;
    structural_prop!(AtmosphereGroundAlbedoUnitInterval);

    /// Atmosphere precomputation resolutions are strictly positive.
    pub struct AtmosphereScatteringResolutionsPositive;
    structural_prop!(AtmosphereScatteringResolutionsPositive);

    /// Atmosphere term lists are non-empty.
    pub struct AtmosphereScatteringTermsPresent;
    structural_prop!(AtmosphereScatteringTermsPresent);

    /// Atmosphere density multipliers are finite when supplied.
    pub struct AtmosphereDensityMultiplierFinite;
    structural_prop!(AtmosphereDensityMultiplierFinite);

    /// Solar-disk angular size is non-negative when supplied.
    pub struct SunDiskAngularSizeNonNegative;
    structural_prop!(SunDiskAngularSizeNonNegative);

    /// Solar-disk intensity is non-negative when supplied.
    pub struct SunDiskIntensityNonNegative;
    structural_prop!(SunDiskIntensityNonNegative);

    /// Atmosphere scattering coefficients are finite.
    pub struct AtmosphereScatteringCoefficientsFinite;
    structural_prop!(AtmosphereScatteringCoefficientsFinite);

    /// Atmosphere scattering coefficients are non-negative.
    pub struct AtmosphereScatteringCoefficientsNonNegative;
    structural_prop!(AtmosphereScatteringCoefficientsNonNegative);

    /// Atmosphere phase-function policy is explicitly declared.
    pub struct AtmospherePhaseFunctionDeclared;
    structural_prop!(AtmospherePhaseFunctionDeclared);

    /// Atmosphere phase asymmetry values are finite when supplied.
    pub struct AtmospherePhaseFunctionAsymmetryFinite;
    structural_prop!(AtmospherePhaseFunctionAsymmetryFinite);

    /// Atmosphere phase asymmetry values stay in the normalized interval.
    pub struct AtmospherePhaseFunctionAsymmetryNormalized;
    structural_prop!(AtmospherePhaseFunctionAsymmetryNormalized);

    /// Atmosphere falloff policy is explicitly declared.
    pub struct AtmosphereFalloffDeclared;
    structural_prop!(AtmosphereFalloffDeclared);

    /// Atmosphere exponential falloff scales are non-negative.
    pub struct AtmosphereFalloffScaleNonNegative;
    structural_prop!(AtmosphereFalloffScaleNonNegative);

    /// Atmosphere tent-falloff centers stay in the unit interval.
    pub struct AtmosphereFalloffCenterUnitInterval;
    structural_prop!(AtmosphereFalloffCenterUnitInterval);

    /// Atmosphere tent-falloff widths stay in the unit interval.
    pub struct AtmosphereFalloffWidthUnitInterval;
    structural_prop!(AtmosphereFalloffWidthUnitInterval);

    /// An asset reference is valid for downstream rendering.
    pub struct RenderAssetReferenceValid;
    structural_prop!(RenderAssetReferenceValid);

    /// One material texture binding is valid for downstream rendering.
    pub struct RenderMaterialTextureBindingValid;
    structural_prop!(RenderMaterialTextureBindingValid);

    /// One material intent descriptor is valid for downstream rendering.
    pub struct RenderMaterialIntentValid;
    structural_prop!(RenderMaterialIntentValid);

    /// One material pipeline-intent descriptor is valid for downstream rendering.
    pub struct RenderPipelineIntentValid;
    structural_prop!(RenderPipelineIntentValid);

    /// One render output target descriptor is valid for downstream rendering.
    pub struct RenderOutputTargetValid;
    structural_prop!(RenderOutputTargetValid);

    /// One render clear-policy descriptor is valid for downstream rendering.
    pub struct RenderClearPolicyValid;
    structural_prop!(RenderClearPolicyValid);

    /// A world-space policy is valid for downstream rendering.
    pub struct RenderWorldSpaceValid;
    structural_prop!(RenderWorldSpaceValid);

    /// A stroke descriptor satisfies the render seam's stroke invariants.
    pub struct RenderStrokeValid;
    structural_prop!(RenderStrokeValid);

    /// A label descriptor satisfies the render seam's label invariants.
    pub struct RenderLabelStyleValid;
    structural_prop!(RenderLabelStyleValid);

    /// A shadow-participation descriptor is renderable by downstream consumers.
    pub struct RenderShadowParticipationValid;
    structural_prop!(RenderShadowParticipationValid);

    /// A feature symbolizer is renderable by downstream consumers.
    pub struct RenderFeatureSymbolizerValid;
    structural_prop!(RenderFeatureSymbolizerValid);

    /// A feature-style rule is renderable by downstream consumers.
    pub struct RenderFeatureStyleRuleValid;
    structural_prop!(RenderFeatureStyleRuleValid);

    /// A feature style is renderable by downstream consumers.
    pub struct RenderFeatureStyleValid;
    structural_prop!(RenderFeatureStyleValid);

    /// A vector batching / instancing descriptor is renderable by downstream consumers.
    pub struct RenderFeatureBatchingValid;
    structural_prop!(RenderFeatureBatchingValid);

    /// A raster source descriptor is renderable by downstream consumers.
    pub struct RenderRasterSourceValid;
    structural_prop!(RenderRasterSourceValid);

    /// A tile streaming descriptor is renderable by downstream consumers.
    pub struct RenderTileStreamingValid;
    structural_prop!(RenderTileStreamingValid);

    /// A tile source descriptor is renderable by downstream consumers.
    pub struct RenderTileSourceValid;
    structural_prop!(RenderTileSourceValid);

    /// A raster symbolizer is renderable by downstream consumers.
    pub struct RenderRasterStyleValid;
    structural_prop!(RenderRasterStyleValid);

    /// A tile symbolizer is renderable by downstream consumers.
    pub struct RenderTileStyleValid;
    structural_prop!(RenderTileStyleValid);

    /// A terrain symbolizer is renderable by downstream consumers.
    pub struct RenderTerrainStyleValid;
    structural_prop!(RenderTerrainStyleValid);

    /// A terrain overlay descriptor is renderable by downstream consumers.
    pub struct RenderTerrainOverlayValid;
    structural_prop!(RenderTerrainOverlayValid);

    /// A terrain LOD descriptor is renderable by downstream consumers.
    pub struct RenderTerrainLodValid;
    structural_prop!(RenderTerrainLodValid);

    /// A color-grading descriptor is renderable by downstream consumers.
    pub struct RenderColorGradingValid;
    structural_prop!(RenderColorGradingValid);

    /// A fixed-exposure descriptor is renderable by downstream consumers.
    pub struct RenderExposureValid;
    structural_prop!(RenderExposureValid);

    /// An auto-exposure descriptor is renderable by downstream consumers.
    pub struct RenderAutoExposureValid;
    structural_prop!(RenderAutoExposureValid);

    /// A bloom descriptor is renderable by downstream consumers.
    pub struct RenderBloomValid;
    structural_prop!(RenderBloomValid);

    /// A depth-of-field descriptor is renderable by downstream consumers.
    pub struct RenderDepthOfFieldValid;
    structural_prop!(RenderDepthOfFieldValid);

    /// A motion-blur descriptor is renderable by downstream consumers.
    pub struct RenderMotionBlurValid;
    structural_prop!(RenderMotionBlurValid);

    /// A chromatic-aberration descriptor is renderable by downstream consumers.
    pub struct RenderChromaticAberrationValid;
    structural_prop!(RenderChromaticAberrationValid);

    /// A screen-space-reflection descriptor is renderable by downstream consumers.
    pub struct RenderScreenSpaceReflectionsValid;
    structural_prop!(RenderScreenSpaceReflectionsValid);

    /// A screen-space ambient-occlusion descriptor is renderable by downstream consumers.
    pub struct RenderScreenSpaceAmbientOcclusionValid;
    structural_prop!(RenderScreenSpaceAmbientOcclusionValid);

    /// A render-view output descriptor is renderable by downstream consumers.
    pub struct RenderViewOutputValid;
    structural_prop!(RenderViewOutputValid);

    /// An explicit annotation is renderable by downstream consumers.
    pub struct RenderAnnotationValid;
    structural_prop!(RenderAnnotationValid);

    /// A map view descriptor is renderable by downstream consumers.
    pub struct RenderViewValid;
    structural_prop!(RenderViewValid);

    /// A skybox descriptor is renderable by downstream consumers.
    pub struct RenderSkyboxValid;
    structural_prop!(RenderSkyboxValid);

    /// An ambient-light descriptor is renderable by downstream consumers.
    pub struct RenderAmbientLightValid;
    structural_prop!(RenderAmbientLightValid);

    /// A directional-light descriptor is renderable by downstream consumers.
    pub struct RenderDirectionalLightValid;
    structural_prop!(RenderDirectionalLightValid);

    /// A scene environment descriptor is renderable by downstream consumers.
    pub struct RenderSceneEnvironmentValid;
    structural_prop!(RenderSceneEnvironmentValid);

    /// A cascade-shadow descriptor is renderable by downstream consumers.
    pub struct RenderCascadeShadowConfigValid;
    structural_prop!(RenderCascadeShadowConfigValid);

    /// A localized probe region is renderable by downstream consumers.
    pub struct RenderProbeRegionValid;
    structural_prop!(RenderProbeRegionValid);

    /// An irradiance-volume payload is renderable by downstream consumers.
    pub struct RenderIrradianceVolumeValid;
    structural_prop!(RenderIrradianceVolumeValid);

    /// A localized light-probe descriptor is renderable by downstream consumers.
    pub struct RenderLightProbeValid;
    structural_prop!(RenderLightProbeValid);

    /// An image-based-lighting descriptor is renderable by downstream consumers.
    pub struct RenderImageBasedLightingValid;
    structural_prop!(RenderImageBasedLightingValid);

    /// An atmosphere falloff descriptor is renderable by downstream consumers.
    pub struct RenderAtmosphereFalloffValid;
    structural_prop!(RenderAtmosphereFalloffValid);

    /// An atmosphere phase-function descriptor is renderable by downstream consumers.
    pub struct RenderAtmospherePhaseFunctionValid;
    structural_prop!(RenderAtmospherePhaseFunctionValid);

    /// One atmosphere scattering term is renderable by downstream consumers.
    pub struct RenderAtmosphereScatteringTermValid;
    structural_prop!(RenderAtmosphereScatteringTermValid);

    /// An atmosphere descriptor is renderable by downstream consumers.
    pub struct RenderAtmosphereValid;
    structural_prop!(RenderAtmosphereValid);

    /// A visible solar-disk descriptor is renderable by downstream consumers.
    pub struct RenderSunDiskValid;
    structural_prop!(RenderSunDiskValid);

    /// A fog descriptor is renderable by downstream consumers.
    pub struct RenderFogValid;
    structural_prop!(RenderFogValid);

    /// A volumetric-fog descriptor is renderable by downstream consumers.
    pub struct RenderVolumetricFogValid;
    structural_prop!(RenderVolumetricFogValid);

    /// A fog-volume descriptor is renderable by downstream consumers.
    pub struct RenderFogVolumeValid;
    structural_prop!(RenderFogVolumeValid);

    /// A vector layer is fully renderable by downstream consumers.
    pub struct RenderableVectorLayerValid;
    structural_prop!(RenderableVectorLayerValid);

    /// A raster layer is fully renderable by downstream consumers.
    pub struct RenderableRasterLayerValid;
    structural_prop!(RenderableRasterLayerValid);

    /// A tile layer is fully renderable by downstream consumers.
    pub struct RenderableTileLayerValid;
    structural_prop!(RenderableTileLayerValid);

    /// A terrain layer is fully renderable by downstream consumers.
    pub struct RenderableTerrainLayerValid;
    structural_prop!(RenderableTerrainLayerValid);

    /// An annotation layer is fully renderable by downstream consumers.
    pub struct RenderableAnnotationLayerValid;
    structural_prop!(RenderableAnnotationLayerValid);

    /// A render layer is valid regardless of whether it is vector or raster.
    pub struct RenderableLayerValid;
    structural_prop!(RenderableLayerValid);

    /// A full render scene is valid and ready for downstream consumption.
    pub struct RenderableSceneValid;
    structural_prop!(RenderableSceneValid);

    /// Scene update operation is explicitly declared.
    pub struct SceneUpdateOperationDeclared;
    structural_prop!(SceneUpdateOperationDeclared);

    /// Scene update target layer identifier is non-empty.
    pub struct SceneUpdateTargetLayerNonEmpty;
    structural_prop!(SceneUpdateTargetLayerNonEmpty);

    /// Scene update reference layer identifier is non-empty when supplied.
    pub struct SceneUpdateReferenceLayerNonEmpty;
    structural_prop!(SceneUpdateReferenceLayerNonEmpty);

    /// A render scene update is valid and ready for downstream consumption.
    pub struct RenderableSceneUpdateValid;
    structural_prop!(RenderableSceneUpdateValid);
}

pub use emit_impls::{
    AmbientLightBrightnessNonNegative, AnnotationAnchorFinite, AnnotationIdNonEmpty,
    AnnotationPayloadDeclared, AssetAccessPatternDeclared, AssetColorSpaceDeclared,
    AssetIntegrityDeclared, AssetIntegrityDigestNonEmpty,
    AssetLogicalHeightPositive, AssetLogicalWidthPositive,
    AssetResidencyPolicyDeclared, AssetRevisionDeclared, AssetRevisionNonEmpty,
    AssetUriDeclared,
    AtmosphereDensityMultiplierFinite, AtmosphereFalloffCenterUnitInterval,
    AtmosphereFalloffDeclared, AtmosphereFalloffScaleNonNegative,
    AtmosphereFalloffWidthUnitInterval, AtmosphereGroundAlbedoFinite,
    AtmosphereGroundAlbedoUnitInterval, AtmospherePhaseFunctionAsymmetryFinite,
    AtmospherePhaseFunctionAsymmetryNormalized, AtmospherePhaseFunctionDeclared,
    AtmosphereScatteringCoefficientsFinite, AtmosphereScatteringCoefficientsNonNegative,
    AtmosphereScatteringResolutionsPositive, AtmosphereScatteringTermsPresent,
    AtmosphereShellRadiiOrdered, AtmosphereShellRadiiPositive,
    AtmosphericFogCoefficientsFinite, AutoExposureCompensationCurveAssetValid,
    AutoExposureFilterOrdered, AutoExposureMeteringMaskAssetValid, AutoExposureRangeOrdered,
    AutoExposureSpeedNonNegative, AutoExposureTransitionDistanceNonNegative,
    BloomCompositeModeDeclared, BloomParametersFinite, BloomScaleFinite,
    BloomThresholdSoftnessUnitInterval, CameraClipRangeFinite, CameraClipRangeOrdered,
    CascadeShadowBoundsOrdered, CascadeShadowBoundsPositive,
    CascadeShadowCascadeCountPositive,
    CascadeShadowFirstCascadeFarBoundGreaterThanMinimum,
    CascadeShadowFirstCascadeFarBoundPositive,
    CascadeShadowMaximumDistanceOrdered,
    CascadeShadowMinimumDistanceNonNegative,
    CascadeShadowOverlapHalfOpenUnitInterval,
    ChromaticAberrationColorLutAssetValid, ChromaticAberrationParametersFinite,
    ChromaticAberrationSamplesPositive, ColorGradingMidtonesRangeOrdered,
    ColorGradingParametersFinite, DepthOfFieldApertureFStopsPositive,
    DepthOfFieldCircleOfConfusionNonNegative, DepthOfFieldFocalDistanceNonNegative,
    DepthOfFieldMaxDepthNonNegative, DepthOfFieldModeDeclared,
    DepthOfFieldParametersFinite, DepthOfFieldSensorHeightPositive,
    DirectionalLightIlluminanceNonNegative, ExtrusionHeightDeclared,
    ExtrusionHeightNonNegative, ExposureEv100Finite, ExposureModeDeclared,
    ExposurePolicyExclusive, FeatureBatchingPolicyDeclared,
    FeatureBatchKeyPropertyNonEmpty, FeatureBatchLimitPositive,
    FeaturePredicateDeclared, FillRuleDeclared,
    FogDensityNonNegative, FogDirectionalLightExponentNonNegative, FogFalloffDeclared,
    FogVolumeAbsorptionNonNegative, FogVolumeDensityFactorNonNegative,
    FogVolumeDensityTextureAssetValid, FogVolumeDensityTextureOffsetFinite,
    FogVolumeLightIntensityNonNegative, FogVolumeParametersFinite,
    FogRangeFinite, FogRangeOrdered, GeometryProjectedForView, HdrOutputDeclared,
    ImageBasedLightingAtmosphereCubemapDimensionsPositive,
    ImageBasedLightingDiffuseMapAssetValid,
    ImageBasedLightingEnvironmentMapAssetValid,
    ImageBasedLightingIntensityNonNegative, ImageBasedLightingRotationFinite,
    ImageBasedLightingSourceDeclared, ImageBasedLightingSpecularMapAssetValid,
    LabelCollisionPolicyDeclared, LabelPriorityFinite, LabelTextSourceDeclared,
    LabelTransformFinite, LayerDrawOrderAssigned, LayerIdNonEmpty, LayerNameNonEmpty,
    LayerOpacityUnitInterval, LayerOrderDeterministic, LayerVisibilityDeclared,
    LightAnglesFinite, MaterialAlphaModeDeclared, MaterialCullModeDeclared,
    MaterialDepthBiasFinite, MaterialFamilyDeclared,
    MaterialMaskThresholdUnitInterval, MaterialOpaqueMethodDeclared,
    MaterialOpaqueMethodDomainCompatible, MaterialPipelineDomainDeclared,
    MaterialPipelineFamilyCompatible, MaterialPrepassesDomainCompatible,
    MaterialPrepassesUnique, MaterialSpecializationKeyNonEmpty,
    MaterialTextureSemanticDeclared, MipBiasFinite, MotionBlurParametersFinite,
    MotionBlurSamplesPositive, MsaaDeclared, NumericExpressionDeclared,
    NumericExpressionOperandsPresent, NumericExpressionParametersFinite,
    NumericInterpolationBasePositive, NumericInterpolationStopsOrdered,
    NumericStepStopsOrdered, PerspectiveFieldOfViewPositive, ProbeRegionCenterFinite,
    ProbeRegionExtentsPositive,
    PlacementAltitudeModeDeclared, PlacementParametersFinite, PointSymbolSizePositive,
    PointSymbolSourceDeclared, PointSymbolTransformFinite, RasterBandSelectionValid,
    RasterColorRampDomainFinite, RasterColorRampDomainOrdered,
    RasterContourIntervalPositive, RasterDerivedProductParametersFinite,
    RasterDimensionsPositive, RasterExtentFinite, RasterExtentOrdered,
    RasterGammaPositive, RasterImageMetadataDeclared, RasterNormalStrengthPositive,
    RasterNormalizeDomainOrdered, RasterOpacityUnitInterval, RasterSamplingDeclared,
    RasterSourceDeclared, RasterValueTransformFinite, RenderAnnotationValid,
    RenderAssetReferenceValid, RenderAutoExposureValid, RenderBloomValid,
    RenderAtmosphereFalloffValid, RenderAtmospherePhaseFunctionValid,
    RenderAtmosphereScatteringTermValid, RenderAtmosphereValid,
    RenderCascadeShadowConfigValid,
    RenderClearDepthFinite, RenderClearPolicyDeclared, RenderClearPolicyValid,
    RenderChromaticAberrationValid, RenderColorGradingValid, RenderDepthOfFieldValid,
    RenderExposureValid, RenderFogVolumeValid, RenderIrradianceVolumeValid,
    RenderFeatureBatchingValid, RenderFeatureStyleRuleValid,
    RenderFeatureStyleValid, RenderFeatureSymbolizerValid,
    RenderFogValid, RenderImageBasedLightingValid, RenderLabelStyleValid,
    RenderLightProbeValid, RenderMaterialIntentValid,
    RenderMaterialTextureBindingValid, RenderMotionBlurValid, RenderPipelineIntentValid,
    RenderOutputTargetDeclared, RenderOutputTargetIdNonEmpty, RenderOutputTargetValid,
    RenderProbeRegionValid,
    RenderRasterSourceValid, RenderRasterStyleValid, RenderSkyboxValid,
    RenderAmbientLightValid, RenderDirectionalLightValid, RenderSceneEnvironmentValid,
    RenderScreenSpaceAmbientOcclusionValid, RenderScreenSpaceReflectionsValid,
    RenderShadowParticipationValid, RenderStrokeValid, RenderSunDiskValid,
    RenderTerrainLodValid, RenderTerrainOverlayValid, RenderTerrainStyleValid,
    RenderTileSourceValid,
    RenderTileStreamingValid, RenderTileStyleValid,
    RenderViewCenterFinite, RenderViewCrsDeclared, RenderViewOrientationFinite,
    RenderViewOutputDeclared, RenderViewOutputValid, RenderViewProjectionDeclared,
    RenderViewResolutionPositive, RenderViewValid, RenderViewViewportPositive,
    RenderVolumetricFogValid, RenderWorldSpaceValid, RenderableAnnotationLayerValid,
    RenderableLayerValid, RenderableRasterLayerValid, RenderableSceneUpdateValid,
    RenderableSceneValid, RenderableTerrainLayerValid, RenderableTileLayerValid,
    RenderableVectorLayerValid, ScaleRangeFinite, ScaleRangeOrdered, SceneIdNonEmpty,
    SceneLayerListDeclared, SceneUpdateOperationDeclared,
    SceneUpdateReferenceLayerNonEmpty, SceneUpdateTargetLayerNonEmpty,
    ScreenSpaceAmbientOcclusionCustomSamplesPerSlicePositive,
    ScreenSpaceAmbientOcclusionCustomSliceCountPositive,
    ScreenSpaceAmbientOcclusionParametersFinite,
    ScreenSpaceAmbientOcclusionQualityDeclared,
    ScreenSpaceAmbientOcclusionThicknessNonNegative,
    ScreenSpaceReflectionsBisectionStepsPositive,
    ScreenSpaceReflectionsLinearMarchExponentNonNegative,
    ScreenSpaceReflectionsLinearStepsPositive,
    ScreenSpaceReflectionsParametersFinite,
    ScreenSpaceReflectionsRoughnessThresholdUnitInterval,
    ScreenSpaceReflectionsThicknessNonNegative,
    ScreenSpaceTransmissionQualityDeclared,
    ShadowFilteringMethodDeclared, ShadowParticipationPolicyDeclared,
    SceneViewDeclared, SkyboxBrightnessNonNegative, SkyboxRotationFinite,
    StrokeParametersFinite, StrokeWidthFinite, StrokeWidthNonNegative,
    SunDiskAngularSizeNonNegative, SunDiskIntensityNonNegative,
    SymbolizerUsesRenderablePaint, TerrainHeightOffsetFinite,
    TerrainLodScreenSpaceErrorPositive, TerrainMeshModeDeclared,
    TerrainOverlayBlendModeDeclared, TerrainOverlayDrapeModeDeclared,
    TerrainOverlayHeightBiasFinite, TerrainOverlayOpacityUnitInterval,
    TerrainShadingModeDeclared, TerrainSkirtHeightNonNegative,
    TerrainSurfaceDrapeModeDeclared, TerrainVerticalScalePositive,
    TileCacheBudgetPositive,
    TileConcurrentRequestsPositive, TileDimensionsPositive,
    TileOpacityUnitInterval, TileRefinementPriorityDeclared,
    TileRetryBackoffPositive, TileStreamingPolicyDeclared,
    TileStylePayloadCompatible, TileTransitionFadePositive,
    TileUriTemplateDeclared, TileZoomRangeOrdered, TonemappingDeclared,
    VectorPayloadDeclared,
    VectorStylePayloadCompatible, VectorSymbolizationResolved,
    VolumetricFogParametersFinite, VolumetricFogStepCountPositive,
    FogVolumeScatteringAsymmetryFinite, FogVolumeScatteringNonNegative,
    IrradianceVolumeIntensityNonNegative, IrradianceVolumeVoxelsAssetValid,
    WorldOriginFinite,
    WorldUnitsPerMeterPositive, WorldVerticalExaggerationPositive,
};

use elicitation::{Established, contracts::ProvableFrom};

/// Evidence that an asset reference is renderable.
pub struct RenderAssetReferenceEvidence {
    /// Proof that the asset declares a stable URI.
    pub asset_uri_declared: Established<AssetUriDeclared>,
    /// Proof that color-space intent is declared when supplied.
    pub asset_color_space_declared: Option<Established<AssetColorSpaceDeclared>>,
    /// Proof that residency intent is declared when supplied.
    pub asset_residency_policy_declared:
        Option<Established<AssetResidencyPolicyDeclared>>,
    /// Proof that access/update pattern is declared when supplied.
    pub asset_access_pattern_declared:
        Option<Established<AssetAccessPatternDeclared>>,
    /// Proof that revision metadata is declared when supplied.
    pub asset_revision_declared: Option<Established<AssetRevisionDeclared>>,
    /// Proof that revision metadata is non-empty when supplied.
    pub asset_revision_non_empty: Option<Established<AssetRevisionNonEmpty>>,
    /// Proof that integrity metadata is declared when supplied.
    pub asset_integrity_declared: Option<Established<AssetIntegrityDeclared>>,
    /// Proof that integrity digests are non-empty when supplied.
    pub asset_integrity_digest_non_empty:
        Option<Established<AssetIntegrityDigestNonEmpty>>,
    /// Proof that logical width is positive when supplied.
    pub asset_logical_width_positive: Option<Established<AssetLogicalWidthPositive>>,
    /// Proof that logical height is positive when supplied.
    pub asset_logical_height_positive: Option<Established<AssetLogicalHeightPositive>>,
}

impl ProvableFrom<RenderAssetReferenceEvidence> for RenderAssetReferenceValid {}

/// Evidence that one material texture binding is renderable.
pub struct RenderMaterialTextureBindingEvidence {
    /// Proof that the bound texture semantic is explicitly declared.
    pub material_texture_semantic_declared:
        Established<MaterialTextureSemanticDeclared>,
    /// Proof that the bound texture asset is renderable.
    pub asset: Established<RenderAssetReferenceValid>,
}

impl ProvableFrom<RenderMaterialTextureBindingEvidence>
    for RenderMaterialTextureBindingValid
{
}

/// Evidence that one material pipeline-intent descriptor is renderable.
pub struct RenderPipelineIntentEvidence {
    /// Proof that the pipeline domain is explicitly declared.
    pub material_pipeline_domain_declared:
        Established<MaterialPipelineDomainDeclared>,
    /// Optional proof that the opaque method is explicitly declared.
    pub material_opaque_method_declared:
        Option<Established<MaterialOpaqueMethodDeclared>>,
    /// Proof that requested prepasses do not repeat.
    pub material_prepasses_unique: Established<MaterialPrepassesUnique>,
    /// Proof that the material family is compatible with the chosen domain.
    pub material_pipeline_family_compatible:
        Established<MaterialPipelineFamilyCompatible>,
    /// Proof that any chosen opaque method is compatible with the chosen domain.
    pub material_opaque_method_domain_compatible:
        Established<MaterialOpaqueMethodDomainCompatible>,
    /// Proof that requested prepasses are compatible with the chosen domain.
    pub material_prepasses_domain_compatible:
        Established<MaterialPrepassesDomainCompatible>,
}

impl ProvableFrom<RenderPipelineIntentEvidence> for RenderPipelineIntentValid {}

/// Evidence that one render output target descriptor is renderable.
pub struct RenderOutputTargetEvidence {
    /// Proof that the output target is explicitly declared.
    pub render_output_target_declared:
        Established<RenderOutputTargetDeclared>,
    /// Optional proof that a symbolic target identifier is non-empty.
    pub render_output_target_id_non_empty:
        Option<Established<RenderOutputTargetIdNonEmpty>>,
    /// Optional proof that an asset-backed target is renderable.
    pub asset: Option<Established<RenderAssetReferenceValid>>,
}

impl ProvableFrom<RenderOutputTargetEvidence> for RenderOutputTargetValid {}

/// Evidence that one render clear-policy descriptor is renderable.
pub struct RenderClearPolicyEvidence {
    /// Proof that the clear policy is explicitly declared.
    pub render_clear_policy_declared:
        Established<RenderClearPolicyDeclared>,
    /// Optional proof that any depth clear value is finite.
    pub render_clear_depth_finite:
        Option<Established<RenderClearDepthFinite>>,
}

impl ProvableFrom<RenderClearPolicyEvidence> for RenderClearPolicyValid {}

/// Evidence that one material intent descriptor is renderable.
pub struct RenderMaterialIntentEvidence {
    /// Proof that the material family is explicitly declared.
    pub material_family_declared: Established<MaterialFamilyDeclared>,
    /// Optional proof that the alpha mode is explicitly declared.
    pub material_alpha_mode_declared:
        Option<Established<MaterialAlphaModeDeclared>>,
    /// Optional proof that any alpha-mask threshold stays in the unit interval.
    pub material_mask_threshold_unit_interval:
        Option<Established<MaterialMaskThresholdUnitInterval>>,
    /// Optional proof that the cull mode is explicitly declared.
    pub material_cull_mode_declared:
        Option<Established<MaterialCullModeDeclared>>,
    /// Optional proof that any depth bias is finite.
    pub material_depth_bias_finite: Option<Established<MaterialDepthBiasFinite>>,
    /// Optional proof that any specialization key is non-empty.
    pub material_specialization_key_non_empty:
        Option<Established<MaterialSpecializationKeyNonEmpty>>,
    /// Optional proof that explicit pipeline-placement intent is renderable.
    pub pipeline_intent: Option<Established<RenderPipelineIntentValid>>,
    /// Proofs that every explicit material texture binding is renderable.
    pub textures: Vec<Established<RenderMaterialTextureBindingValid>>,
}

impl ProvableFrom<RenderMaterialIntentEvidence> for RenderMaterialIntentValid {}

/// Evidence that a world-space policy is renderable.
pub struct RenderWorldSpaceEvidence {
    /// Proof that an explicit world origin is finite when supplied.
    pub world_origin_finite: Established<WorldOriginFinite>,
    /// Proof that units-per-meter is positive.
    pub world_units_per_meter_positive: Established<WorldUnitsPerMeterPositive>,
    /// Proof that vertical exaggeration is positive.
    pub world_vertical_exaggeration_positive:
        Established<WorldVerticalExaggerationPositive>,
}

impl ProvableFrom<RenderWorldSpaceEvidence> for RenderWorldSpaceValid {}

/// Evidence that a stroke descriptor is renderable.
pub struct RenderStrokeEvidence {
    /// Proof that stroke width is finite.
    pub stroke_width_finite: Established<StrokeWidthFinite>,
    /// Proof that stroke width is non-negative.
    pub stroke_width_non_negative: Established<StrokeWidthNonNegative>,
    /// Proof that stroke floating-point parameters are finite.
    pub stroke_parameters_finite: Established<StrokeParametersFinite>,
}

impl ProvableFrom<RenderStrokeEvidence> for RenderStrokeValid {}

/// Evidence that a label descriptor is renderable.
pub struct RenderLabelStyleEvidence {
    /// Proof that the label source is explicitly declared.
    pub label_text_source_declared: Established<LabelTextSourceDeclared>,
    /// Proof that label priority is finite.
    pub label_priority_finite: Established<LabelPriorityFinite>,
    /// Proof that label transform parameters are finite.
    pub label_transform_finite: Established<LabelTransformFinite>,
    /// Proofs that any numeric expressions used by the label were explicitly
    /// declared.
    pub numeric_expressions_declared: Vec<Established<NumericExpressionDeclared>>,
    /// Proofs that numeric-expression literal parameters are finite.
    pub numeric_expression_parameters_finite:
        Vec<Established<NumericExpressionParametersFinite>>,
    /// Proofs that variadic numeric expressions declare operands.
    pub numeric_expression_operands_present:
        Vec<Established<NumericExpressionOperandsPresent>>,
    /// Proofs that any step-expression thresholds are ordered.
    pub numeric_step_stops_ordered: Vec<Established<NumericStepStopsOrdered>>,
    /// Proofs that any interpolation stops are ordered.
    pub numeric_interpolation_stops_ordered:
        Vec<Established<NumericInterpolationStopsOrdered>>,
    /// Proofs that any exponential interpolation bases are positive.
    pub numeric_interpolation_base_positive:
        Vec<Established<NumericInterpolationBasePositive>>,
}

impl ProvableFrom<RenderLabelStyleEvidence> for RenderLabelStyleValid {}

/// Evidence that a shadow-participation descriptor is renderable.
pub struct RenderShadowParticipationEvidence {
    /// Proof that the shadow-participation policy is explicitly declared.
    pub shadow_participation_policy_declared:
        Established<ShadowParticipationPolicyDeclared>,
}

impl ProvableFrom<RenderShadowParticipationEvidence> for RenderShadowParticipationValid {}

/// Evidence that a vector feature symbolizer is renderable.
pub struct RenderFeatureSymbolizerEvidence {
    /// Proof that the paint choices are renderable.
    pub symbolizer_uses_renderable_paint: Established<SymbolizerUsesRenderablePaint>,
    /// Optional proof for a stroke descriptor.
    pub stroke: Option<Established<RenderStrokeValid>>,
    /// Optional proof that a fill rule was declared.
    pub fill_rule_declared: Option<Established<FillRuleDeclared>>,
    /// Optional proof for point symbol scale.
    pub point_symbol_size_positive: Option<Established<PointSymbolSizePositive>>,
    /// Optional proof that point-symbol content is declared.
    pub point_symbol_source_declared: Option<Established<PointSymbolSourceDeclared>>,
    /// Optional proof for point symbol transforms.
    pub point_symbol_transform_finite: Option<Established<PointSymbolTransformFinite>>,
    /// Optional proof for an asset-backed point symbol.
    pub point_symbol_asset: Option<Established<RenderAssetReferenceValid>>,
    /// Optional proof for label styling.
    pub label: Option<Established<RenderLabelStyleValid>>,
    /// Optional proof that label collision policy is declared.
    pub label_collision_policy_declared:
        Option<Established<LabelCollisionPolicyDeclared>>,
    /// Optional proof that placement altitude mode is declared.
    pub placement_altitude_mode_declared:
        Option<Established<PlacementAltitudeModeDeclared>>,
    /// Optional proof that placement parameters are finite.
    pub placement_parameters_finite: Option<Established<PlacementParametersFinite>>,
    /// Optional proof that shadow-participation policy is valid.
    pub shadow_participation: Option<Established<RenderShadowParticipationValid>>,
    /// Optional proof that material and pipeline intent is valid.
    pub material_intent: Option<Established<RenderMaterialIntentValid>>,
    /// Proofs that any numeric expressions used by placement or extrusion were
    /// explicitly declared.
    pub numeric_expressions_declared: Vec<Established<NumericExpressionDeclared>>,
    /// Proofs that numeric-expression literal parameters are finite.
    pub numeric_expression_parameters_finite:
        Vec<Established<NumericExpressionParametersFinite>>,
    /// Proofs that variadic numeric expressions declare operands.
    pub numeric_expression_operands_present:
        Vec<Established<NumericExpressionOperandsPresent>>,
    /// Proofs that any step-expression thresholds are ordered.
    pub numeric_step_stops_ordered: Vec<Established<NumericStepStopsOrdered>>,
    /// Proofs that any interpolation stops are ordered.
    pub numeric_interpolation_stops_ordered:
        Vec<Established<NumericInterpolationStopsOrdered>>,
    /// Proofs that any exponential interpolation bases are positive.
    pub numeric_interpolation_base_positive:
        Vec<Established<NumericInterpolationBasePositive>>,
    /// Optional proof that extrusion height is declared.
    pub extrusion_height_declared: Option<Established<ExtrusionHeightDeclared>>,
    /// Optional proof that extrusion height is non-negative.
    pub extrusion_height_non_negative:
        Option<Established<ExtrusionHeightNonNegative>>,
}

impl ProvableFrom<RenderFeatureSymbolizerEvidence> for RenderFeatureSymbolizerValid {}

/// Evidence that a rule-based style override is renderable.
pub struct RenderFeatureStyleRuleEvidence {
    /// Proof that the rule predicate is explicitly declared.
    pub feature_predicate_declared: Established<FeaturePredicateDeclared>,
    /// Optional proof that the declared scale range is finite.
    pub scale_range_finite: Option<Established<ScaleRangeFinite>>,
    /// Optional proof that the declared scale range is ordered.
    pub scale_range_ordered: Option<Established<ScaleRangeOrdered>>,
    /// Proof that the rule symbolizer is renderable.
    pub symbolizer: Established<RenderFeatureSymbolizerValid>,
}

impl ProvableFrom<RenderFeatureStyleRuleEvidence> for RenderFeatureStyleRuleValid {}

/// Evidence that a vector feature style is renderable.
pub struct RenderFeatureStyleEvidence {
    /// Proof that the default symbolizer is renderable.
    pub default_symbolizer: Established<RenderFeatureSymbolizerValid>,
    /// Proofs that any declared override rules are renderable.
    pub rules: Vec<Established<RenderFeatureStyleRuleValid>>,
}

impl ProvableFrom<RenderFeatureStyleEvidence> for RenderFeatureStyleValid {}

/// Evidence that a vector batching / instancing descriptor is renderable.
pub struct RenderFeatureBatchingEvidence {
    /// Proof that batching policy was explicitly declared.
    pub feature_batching_policy_declared:
        Established<FeatureBatchingPolicyDeclared>,
    /// Optional proof that max-features-per-batch is strictly positive.
    pub feature_batch_limit_positive: Option<Established<FeatureBatchLimitPositive>>,
    /// Proofs that any grouping-key property names are non-empty.
    pub feature_batch_key_property_non_empty:
        Vec<Established<FeatureBatchKeyPropertyNonEmpty>>,
}

impl ProvableFrom<RenderFeatureBatchingEvidence> for RenderFeatureBatchingValid {}

/// Evidence that a raster source is renderable.
pub struct RenderRasterSourceEvidence {
    /// Proof that the raster source was explicitly declared.
    pub raster_source_declared: Established<RasterSourceDeclared>,
    /// Proof that the underlying asset reference is renderable.
    pub asset: Established<RenderAssetReferenceValid>,
    /// Proof that the raster extent is finite.
    pub raster_extent_finite: Established<RasterExtentFinite>,
    /// Proof that the raster extent bounds are ordered.
    pub raster_extent_ordered: Established<RasterExtentOrdered>,
}

impl ProvableFrom<RenderRasterSourceEvidence> for RenderRasterSourceValid {}

/// Evidence that a tile streaming descriptor is renderable.
pub struct RenderTileStreamingEvidence {
    /// Proof that tile streaming policy was explicitly declared.
    pub tile_streaming_policy_declared: Established<TileStreamingPolicyDeclared>,
    /// Proof that tile refinement priority was explicitly declared.
    pub tile_refinement_priority_declared:
        Established<TileRefinementPriorityDeclared>,
    /// Proof that max concurrent requests is strictly positive.
    pub tile_concurrent_requests_positive:
        Established<TileConcurrentRequestsPositive>,
    /// Optional proof that the tile cache budget is strictly positive.
    pub tile_cache_budget_positive: Option<Established<TileCacheBudgetPositive>>,
    /// Optional proof that retry backoff is strictly positive when retries are enabled.
    pub tile_retry_backoff_positive: Option<Established<TileRetryBackoffPositive>>,
    /// Optional proof that transition fade durations are strictly positive.
    pub tile_transition_fade_positive:
        Option<Established<TileTransitionFadePositive>>,
}

impl ProvableFrom<RenderTileStreamingEvidence> for RenderTileStreamingValid {}

/// Evidence that a raster symbolizer is renderable.
pub struct RenderRasterStyleEvidence {
    /// Proof that raster opacity is in the unit interval.
    pub raster_opacity_unit_interval: Established<RasterOpacityUnitInterval>,
    /// Proof that raster band selection is valid.
    pub raster_band_selection_valid: Established<RasterBandSelectionValid>,
    /// Proof that raster sampling is declared.
    pub raster_sampling_declared: Established<RasterSamplingDeclared>,
    /// Optional proof that the color-ramp domain is finite.
    pub raster_color_ramp_domain_finite: Option<Established<RasterColorRampDomainFinite>>,
    /// Optional proof that the color-ramp domain is ordered.
    pub raster_color_ramp_domain_ordered:
        Option<Established<RasterColorRampDomainOrdered>>,
    /// Proofs that raster value-transform parameters are finite.
    pub raster_value_transforms_finite:
        Vec<Established<RasterValueTransformFinite>>,
    /// Optional proofs that normalization domains are ordered.
    pub raster_normalize_domain_ordered:
        Vec<Established<RasterNormalizeDomainOrdered>>,
    /// Optional proofs that gamma exponents are positive.
    pub raster_gamma_positive: Vec<Established<RasterGammaPositive>>,
    /// Optional proof that derived-product parameters are finite.
    pub raster_derived_product_parameters_finite:
        Option<Established<RasterDerivedProductParametersFinite>>,
    /// Optional proof that contour intervals are strictly positive.
    pub raster_contour_interval_positive:
        Option<Established<RasterContourIntervalPositive>>,
    /// Optional proof that normal-map strength values are strictly positive.
    pub raster_normal_strength_positive:
        Option<Established<RasterNormalStrengthPositive>>,
    /// Optional proof that material and pipeline intent is valid.
    pub material_intent: Option<Established<RenderMaterialIntentValid>>,
}

impl ProvableFrom<RenderRasterStyleEvidence> for RenderRasterStyleValid {}

/// Evidence that a tile source is renderable.
pub struct RenderTileSourceEvidence {
    /// Proof that the tile URI template was declared.
    pub tile_uri_template_declared: Established<TileUriTemplateDeclared>,
    /// Proof that tile zoom bounds are ordered.
    pub tile_zoom_range_ordered: Established<TileZoomRangeOrdered>,
    /// Proof that tile dimensions are positive.
    pub tile_dimensions_positive: Established<TileDimensionsPositive>,
    /// Optional proof that tile streaming policy is renderable.
    pub streaming: Option<Established<RenderTileStreamingValid>>,
}

impl ProvableFrom<RenderTileSourceEvidence> for RenderTileSourceValid {}

/// Evidence that a tile style is renderable.
pub struct RenderTileStyleEvidence {
    /// Proof that tile style opacity is in the unit interval.
    pub tile_opacity_unit_interval: Established<TileOpacityUnitInterval>,
    /// Optional proof that raster tile styling is renderable.
    pub raster_style: Option<Established<RenderRasterStyleValid>>,
    /// Optional proof that vector tile styling is renderable.
    pub vector_style: Option<Established<RenderFeatureStyleValid>>,
    /// Optional proof that material and pipeline intent is valid.
    pub material_intent: Option<Established<RenderMaterialIntentValid>>,
    /// Proof that the chosen style is compatible with the payload family.
    pub tile_style_payload_compatible:
        Established<TileStylePayloadCompatible>,
}

impl ProvableFrom<RenderTileStyleEvidence> for RenderTileStyleValid {}

/// Evidence that a terrain LOD descriptor is renderable.
pub struct RenderTerrainLodEvidence {
    /// Proof that terrain LOD error thresholds are strictly positive.
    pub terrain_lod_screen_space_error_positive:
        Established<TerrainLodScreenSpaceErrorPositive>,
    /// Optional proof that terrain skirt height is non-negative.
    pub terrain_skirt_height_non_negative:
        Option<Established<TerrainSkirtHeightNonNegative>>,
}

impl ProvableFrom<RenderTerrainLodEvidence> for RenderTerrainLodValid {}

/// Evidence that one terrain overlay is renderable.
pub struct RenderTerrainOverlayEvidence {
    /// Proof that overlay opacity is in the unit interval.
    pub terrain_overlay_opacity_unit_interval:
        Established<TerrainOverlayOpacityUnitInterval>,
    /// Proof that the overlay drape mode is declared.
    pub terrain_overlay_drape_mode_declared:
        Established<TerrainOverlayDrapeModeDeclared>,
    /// Proof that the overlay blend mode is declared.
    pub terrain_overlay_blend_mode_declared:
        Established<TerrainOverlayBlendModeDeclared>,
    /// Proof that overlay height bias is finite.
    pub terrain_overlay_height_bias_finite:
        Established<TerrainOverlayHeightBiasFinite>,
    /// Optional proof that a texture-backed overlay asset is renderable.
    pub texture: Option<Established<RenderAssetReferenceValid>>,
    /// Optional proof that a raster-style overlay source is renderable.
    pub raster_style: Option<Established<RenderRasterStyleValid>>,
}

impl ProvableFrom<RenderTerrainOverlayEvidence> for RenderTerrainOverlayValid {}

/// Evidence that a terrain style is renderable.
pub struct RenderTerrainStyleEvidence {
    /// Proof that terrain vertical scale is positive.
    pub terrain_vertical_scale_positive:
        Established<TerrainVerticalScalePositive>,
    /// Proof that base-height offset is finite.
    pub terrain_height_offset_finite: Established<TerrainHeightOffsetFinite>,
    /// Optional proof that the primary surface drape policy is declared.
    pub terrain_surface_drape_mode_declared:
        Option<Established<TerrainSurfaceDrapeModeDeclared>>,
    /// Proof that the mesh policy is declared.
    pub terrain_mesh_mode_declared: Established<TerrainMeshModeDeclared>,
    /// Proof that the shading policy is declared.
    pub terrain_shading_mode_declared:
        Established<TerrainShadingModeDeclared>,
    /// Optional proof that the surface texture asset is renderable.
    pub surface_texture: Option<Established<RenderAssetReferenceValid>>,
    /// Optional proof that the surface raster style is renderable.
    pub surface_raster_style: Option<Established<RenderRasterStyleValid>>,
    /// Proofs that ordered terrain overlays are renderable.
    pub overlays: Vec<Established<RenderTerrainOverlayValid>>,
    /// Optional proof that terrain LOD policy is renderable.
    pub lod: Option<Established<RenderTerrainLodValid>>,
    /// Optional proof that shadow-participation policy is valid.
    pub shadow_participation: Option<Established<RenderShadowParticipationValid>>,
    /// Optional proof that material and pipeline intent is valid.
    pub material_intent: Option<Established<RenderMaterialIntentValid>>,
}

impl ProvableFrom<RenderTerrainStyleEvidence> for RenderTerrainStyleValid {}

/// Evidence that a color-grading descriptor is renderable.
pub struct RenderColorGradingEvidence {
    /// Proof that all color-grading numeric parameters are finite.
    pub color_grading_parameters_finite: Established<ColorGradingParametersFinite>,
    /// Proof that the declared midtones range is ordered.
    pub color_grading_midtones_range_ordered:
        Established<ColorGradingMidtonesRangeOrdered>,
}

impl ProvableFrom<RenderColorGradingEvidence> for RenderColorGradingValid {}

/// Evidence that a fixed-exposure descriptor is renderable.
pub struct RenderExposureEvidence {
    /// Proof that the exposure mode is explicitly declared.
    pub exposure_mode_declared: Established<ExposureModeDeclared>,
    /// Optional proof that an explicit EV100 value is finite.
    pub exposure_ev100_finite: Option<Established<ExposureEv100Finite>>,
}

impl ProvableFrom<RenderExposureEvidence> for RenderExposureValid {}

/// Evidence that an auto-exposure descriptor is renderable.
pub struct RenderAutoExposureEvidence {
    /// Proof that the exposure range is ordered.
    pub auto_exposure_range_ordered: Established<AutoExposureRangeOrdered>,
    /// Proof that the luminance filter window is ordered.
    pub auto_exposure_filter_ordered: Established<AutoExposureFilterOrdered>,
    /// Proof that the brighten/darken adaptation speeds are non-negative.
    pub auto_exposure_speed_non_negative:
        Established<AutoExposureSpeedNonNegative>,
    /// Proof that the transition distance is non-negative.
    pub auto_exposure_transition_distance_non_negative:
        Established<AutoExposureTransitionDistanceNonNegative>,
    /// Optional proof that the metering-mask asset is renderable.
    pub metering_mask_asset:
        Option<Established<AutoExposureMeteringMaskAssetValid>>,
    /// Optional proof that the compensation-curve asset is renderable.
    pub compensation_curve_asset:
        Option<Established<AutoExposureCompensationCurveAssetValid>>,
}

impl ProvableFrom<RenderAutoExposureEvidence> for RenderAutoExposureValid {}

/// Evidence that a bloom descriptor is renderable.
pub struct RenderBloomEvidence {
    /// Proof that bloom numeric parameters are finite.
    pub bloom_parameters_finite: Established<BloomParametersFinite>,
    /// Proof that threshold softness stays in the unit interval.
    pub bloom_threshold_softness_unit_interval:
        Established<BloomThresholdSoftnessUnitInterval>,
    /// Proof that bloom scale factors are finite.
    pub bloom_scale_finite: Established<BloomScaleFinite>,
    /// Proof that the bloom composite mode is declared.
    pub bloom_composite_mode_declared:
        Established<BloomCompositeModeDeclared>,
}

impl ProvableFrom<RenderBloomEvidence> for RenderBloomValid {}

/// Evidence that a depth-of-field descriptor is renderable.
pub struct RenderDepthOfFieldEvidence {
    /// Proof that the depth-of-field mode is explicitly declared.
    pub depth_of_field_mode_declared: Established<DepthOfFieldModeDeclared>,
    /// Proof that depth-of-field floating-point parameters are finite.
    pub depth_of_field_parameters_finite:
        Established<DepthOfFieldParametersFinite>,
    /// Proof that focal distance is non-negative.
    pub depth_of_field_focal_distance_non_negative:
        Established<DepthOfFieldFocalDistanceNonNegative>,
    /// Proof that sensor height is strictly positive.
    pub depth_of_field_sensor_height_positive:
        Established<DepthOfFieldSensorHeightPositive>,
    /// Proof that aperture f-stops are strictly positive.
    pub depth_of_field_aperture_f_stops_positive:
        Established<DepthOfFieldApertureFStopsPositive>,
    /// Proof that the circle-of-confusion diameter is non-negative.
    pub depth_of_field_circle_of_confusion_non_negative:
        Established<DepthOfFieldCircleOfConfusionNonNegative>,
    /// Proof that max depth is non-negative.
    pub depth_of_field_max_depth_non_negative:
        Established<DepthOfFieldMaxDepthNonNegative>,
}

impl ProvableFrom<RenderDepthOfFieldEvidence> for RenderDepthOfFieldValid {}

/// Evidence that a motion-blur descriptor is renderable.
pub struct RenderMotionBlurEvidence {
    /// Proof that motion-blur floating-point parameters are finite.
    pub motion_blur_parameters_finite:
        Established<MotionBlurParametersFinite>,
    /// Proof that motion-blur sample counts are strictly positive.
    pub motion_blur_samples_positive: Established<MotionBlurSamplesPositive>,
}

impl ProvableFrom<RenderMotionBlurEvidence> for RenderMotionBlurValid {}

/// Evidence that a chromatic-aberration descriptor is renderable.
pub struct RenderChromaticAberrationEvidence {
    /// Proof that chromatic-aberration floating-point parameters are finite.
    pub chromatic_aberration_parameters_finite:
        Established<ChromaticAberrationParametersFinite>,
    /// Proof that chromatic-aberration sample counts are strictly positive.
    pub chromatic_aberration_samples_positive:
        Established<ChromaticAberrationSamplesPositive>,
    /// Optional proof that the color-LUT asset is renderable.
    pub color_lut_asset:
        Option<Established<ChromaticAberrationColorLutAssetValid>>,
}

impl ProvableFrom<RenderChromaticAberrationEvidence>
    for RenderChromaticAberrationValid
{
}

/// Evidence that a screen-space-reflection descriptor is renderable.
pub struct RenderScreenSpaceReflectionsEvidence {
    /// Proof that floating-point reflection parameters are finite.
    pub screen_space_reflections_parameters_finite:
        Established<ScreenSpaceReflectionsParametersFinite>,
    /// Optional proof that roughness thresholds stay in the unit interval.
    pub screen_space_reflections_roughness_threshold_unit_interval:
        Option<Established<ScreenSpaceReflectionsRoughnessThresholdUnitInterval>>,
    /// Optional proof that thickness values are non-negative.
    pub screen_space_reflections_thickness_non_negative:
        Option<Established<ScreenSpaceReflectionsThicknessNonNegative>>,
    /// Optional proof that linear-step counts are strictly positive.
    pub screen_space_reflections_linear_steps_positive:
        Option<Established<ScreenSpaceReflectionsLinearStepsPositive>>,
    /// Optional proof that march exponents are non-negative.
    pub screen_space_reflections_linear_march_exponent_non_negative:
        Option<Established<ScreenSpaceReflectionsLinearMarchExponentNonNegative>>,
    /// Optional proof that bisection-step counts are strictly positive.
    pub screen_space_reflections_bisection_steps_positive:
        Option<Established<ScreenSpaceReflectionsBisectionStepsPositive>>,
}

impl ProvableFrom<RenderScreenSpaceReflectionsEvidence>
    for RenderScreenSpaceReflectionsValid
{
}

/// Evidence that a screen-space ambient-occlusion descriptor is renderable.
pub struct RenderScreenSpaceAmbientOcclusionEvidence {
    /// Proof that SSAO quality is explicitly declared.
    pub screen_space_ambient_occlusion_quality_declared:
        Established<ScreenSpaceAmbientOcclusionQualityDeclared>,
    /// Optional proof that floating-point SSAO parameters are finite.
    pub screen_space_ambient_occlusion_parameters_finite:
        Option<Established<ScreenSpaceAmbientOcclusionParametersFinite>>,
    /// Optional proof that thickness values are non-negative.
    pub screen_space_ambient_occlusion_thickness_non_negative:
        Option<Established<ScreenSpaceAmbientOcclusionThicknessNonNegative>>,
    /// Optional proof that custom slice counts are strictly positive.
    pub screen_space_ambient_occlusion_custom_slice_count_positive:
        Option<Established<ScreenSpaceAmbientOcclusionCustomSliceCountPositive>>,
    /// Optional proof that custom samples-per-slice-side values are strictly positive.
    pub screen_space_ambient_occlusion_custom_samples_per_slice_positive:
        Option<Established<ScreenSpaceAmbientOcclusionCustomSamplesPerSlicePositive>>,
}

impl ProvableFrom<RenderScreenSpaceAmbientOcclusionEvidence>
    for RenderScreenSpaceAmbientOcclusionValid
{
}

/// Evidence that a render-view output descriptor is renderable.
pub struct RenderViewOutputEvidence {
    /// Proof that a view-output policy was explicitly declared.
    pub render_view_output_declared: Established<RenderViewOutputDeclared>,
    /// Proof that the HDR toggle is explicitly declared.
    pub hdr_output_declared: Established<HdrOutputDeclared>,
    /// Optional proof that the output target is valid.
    pub target: Option<Established<RenderOutputTargetValid>>,
    /// Optional proof that the clear policy is valid.
    pub clear_policy: Option<Established<RenderClearPolicyValid>>,
    /// Optional proof that tonemapping is explicitly declared.
    pub tonemapping_declared: Option<Established<TonemappingDeclared>>,
    /// Optional proof that MSAA policy is explicitly declared.
    pub msaa_declared: Option<Established<MsaaDeclared>>,
    /// Optional proof that mip-bias values are finite.
    pub mip_bias_finite: Option<Established<MipBiasFinite>>,
    /// Proof that manual and automatic exposure policies do not conflict.
    pub exposure_policy_exclusive: Established<ExposurePolicyExclusive>,
    /// Optional proof that fixed exposure is valid.
    pub exposure: Option<Established<RenderExposureValid>>,
    /// Optional proof that color grading is valid.
    pub color_grading: Option<Established<RenderColorGradingValid>>,
    /// Optional proof that auto exposure is valid.
    pub auto_exposure: Option<Established<RenderAutoExposureValid>>,
    /// Optional proof that bloom is valid.
    pub bloom: Option<Established<RenderBloomValid>>,
    /// Optional proof that depth of field is valid.
    pub depth_of_field: Option<Established<RenderDepthOfFieldValid>>,
    /// Optional proof that motion blur is valid.
    pub motion_blur: Option<Established<RenderMotionBlurValid>>,
    /// Optional proof that chromatic aberration is valid.
    pub chromatic_aberration:
        Option<Established<RenderChromaticAberrationValid>>,
    /// Optional proof that screen-space reflections are valid.
    pub screen_space_reflections:
        Option<Established<RenderScreenSpaceReflectionsValid>>,
    /// Optional proof that screen-space ambient occlusion is valid.
    pub screen_space_ambient_occlusion:
        Option<Established<RenderScreenSpaceAmbientOcclusionValid>>,
    /// Optional proof that shadow-filtering method is declared.
    pub shadow_filtering_method_declared:
        Option<Established<ShadowFilteringMethodDeclared>>,
    /// Optional proof that screen-space transmission quality is declared.
    pub screen_space_transmission_quality_declared:
        Option<Established<ScreenSpaceTransmissionQualityDeclared>>,
}

impl ProvableFrom<RenderViewOutputEvidence> for RenderViewOutputValid {}

/// Evidence that an explicit annotation is renderable.
pub struct RenderAnnotationEvidence {
    /// Proof that the annotation identifier is non-empty.
    pub annotation_id_non_empty: Established<AnnotationIdNonEmpty>,
    /// Proof that the anchor coordinates are finite.
    pub annotation_anchor_finite: Established<AnnotationAnchorFinite>,
    /// Optional proof that the declared scale range is finite.
    pub scale_range_finite: Option<Established<ScaleRangeFinite>>,
    /// Optional proof that the declared scale range is ordered.
    pub scale_range_ordered: Option<Established<ScaleRangeOrdered>>,
    /// Proof that the annotation symbolizer is renderable.
    pub symbolizer: Established<RenderFeatureSymbolizerValid>,
}

impl ProvableFrom<RenderAnnotationEvidence> for RenderAnnotationValid {}

/// Evidence that a render view descriptor is valid.
pub struct RenderViewEvidence {
    /// Proof that the view center is finite.
    pub render_view_center_finite: Established<RenderViewCenterFinite>,
    /// Proof that the view resolution is positive.
    pub render_view_resolution_positive: Established<RenderViewResolutionPositive>,
    /// Proof that the viewport dimensions are positive.
    pub render_view_viewport_positive: Established<RenderViewViewportPositive>,
    /// Proof that view rotation/pitch are finite.
    pub render_view_orientation_finite: Established<RenderViewOrientationFinite>,
    /// Proof that a CRS is declared for the view.
    pub render_view_crs_declared: Established<RenderViewCrsDeclared>,
    /// Proof that the view projection policy is explicitly declared.
    pub render_view_projection_declared: Established<RenderViewProjectionDeclared>,
    /// Proof that camera clipping planes are finite.
    pub camera_clip_range_finite: Established<CameraClipRangeFinite>,
    /// Proof that camera clipping planes are strictly ordered.
    pub camera_clip_range_ordered: Established<CameraClipRangeOrdered>,
    /// Optional proof that a perspective field of view is positive.
    pub perspective_field_of_view_positive:
        Option<Established<PerspectiveFieldOfViewPositive>>,
    /// Optional proof that camera-output policy is valid.
    pub output: Option<Established<RenderViewOutputValid>>,
    /// Optional proof that world-space policy is valid.
    pub world_space: Option<Established<RenderWorldSpaceValid>>,
}

impl ProvableFrom<RenderViewEvidence> for RenderViewValid {}

/// Evidence that a fog descriptor is renderable.
pub struct RenderFogEvidence {
    /// Proof that the fog falloff model is explicitly declared.
    pub fog_falloff_declared: Established<FogFalloffDeclared>,
    /// Optional proof that linear fog distances are finite.
    pub fog_range_finite: Option<Established<FogRangeFinite>>,
    /// Optional proof that linear fog distances are ordered.
    pub fog_range_ordered: Option<Established<FogRangeOrdered>>,
    /// Optional proof that the directional-light exponent is non-negative.
    pub fog_directional_light_exponent_non_negative:
        Option<Established<FogDirectionalLightExponentNonNegative>>,
    /// Optional proof that exponential density values are non-negative.
    pub fog_density_non_negative: Option<Established<FogDensityNonNegative>>,
    /// Optional proof that atmospheric coefficients are finite.
    pub atmospheric_fog_coefficients_finite:
        Option<Established<AtmosphericFogCoefficientsFinite>>,
}

impl ProvableFrom<RenderFogEvidence> for RenderFogValid {}

/// Evidence that a volumetric-fog descriptor is renderable.
pub struct RenderVolumetricFogEvidence {
    /// Optional proof that floating-point volumetric parameters are finite.
    pub volumetric_fog_parameters_finite:
        Option<Established<VolumetricFogParametersFinite>>,
    /// Optional proof that step counts are positive.
    pub volumetric_fog_step_count_positive:
        Option<Established<VolumetricFogStepCountPositive>>,
}

impl ProvableFrom<RenderVolumetricFogEvidence> for RenderVolumetricFogValid {}

/// Evidence that a fog-volume descriptor is renderable.
pub struct RenderFogVolumeEvidence {
    /// Optional proof that fog-volume floating-point parameters are finite.
    pub fog_volume_parameters_finite: Option<Established<FogVolumeParametersFinite>>,
    /// Optional proof that density factors are non-negative.
    pub fog_volume_density_factor_non_negative:
        Option<Established<FogVolumeDensityFactorNonNegative>>,
    /// Optional proof that density-texture offsets are finite.
    pub fog_volume_density_texture_offset_finite:
        Option<Established<FogVolumeDensityTextureOffsetFinite>>,
    /// Optional proof that absorption coefficients are non-negative.
    pub fog_volume_absorption_non_negative:
        Option<Established<FogVolumeAbsorptionNonNegative>>,
    /// Optional proof that scattering coefficients are non-negative.
    pub fog_volume_scattering_non_negative:
        Option<Established<FogVolumeScatteringNonNegative>>,
    /// Optional proof that anisotropy factors are finite.
    pub fog_volume_scattering_asymmetry_finite:
        Option<Established<FogVolumeScatteringAsymmetryFinite>>,
    /// Optional proof that light-intensity multipliers are non-negative.
    pub fog_volume_light_intensity_non_negative:
        Option<Established<FogVolumeLightIntensityNonNegative>>,
    /// Optional proof that density-texture assets are renderable.
    pub fog_volume_density_texture_asset:
        Option<Established<FogVolumeDensityTextureAssetValid>>,
}

impl ProvableFrom<RenderFogVolumeEvidence> for RenderFogVolumeValid {}

/// Evidence that a localized probe region is renderable.
pub struct RenderProbeRegionEvidence {
    /// Proof that probe-region center coordinates are finite.
    pub probe_region_center_finite: Established<ProbeRegionCenterFinite>,
    /// Proof that probe-region extents are strictly positive.
    pub probe_region_extents_positive: Established<ProbeRegionExtentsPositive>,
}

impl ProvableFrom<RenderProbeRegionEvidence> for RenderProbeRegionValid {}

/// Evidence that an irradiance-volume payload is renderable.
pub struct RenderIrradianceVolumeEvidence {
    /// Proof that the irradiance-volume voxel asset is renderable.
    pub irradiance_volume_voxels_asset:
        Established<IrradianceVolumeVoxelsAssetValid>,
    /// Optional proof that intensity values are non-negative.
    pub irradiance_volume_intensity_non_negative:
        Option<Established<IrradianceVolumeIntensityNonNegative>>,
}

impl ProvableFrom<RenderIrradianceVolumeEvidence>
    for RenderIrradianceVolumeValid
{
}

/// Evidence that a localized light probe is renderable.
pub struct RenderLightProbeEvidence {
    /// Proof that the probe region is valid.
    pub region: Established<RenderProbeRegionValid>,
    /// Optional proof that the irradiance-volume payload is valid.
    pub irradiance_volume: Option<Established<RenderIrradianceVolumeValid>>,
}

impl ProvableFrom<RenderLightProbeEvidence> for RenderLightProbeValid {}

/// Evidence that an image-based-lighting descriptor is renderable.
pub struct RenderImageBasedLightingEvidence {
    /// Proof that the image-based-lighting source is explicitly declared.
    pub image_based_lighting_source_declared:
        Established<ImageBasedLightingSourceDeclared>,
    /// Optional proof that intensity values are non-negative.
    pub image_based_lighting_intensity_non_negative:
        Option<Established<ImageBasedLightingIntensityNonNegative>>,
    /// Optional proof that rotation values are finite.
    pub image_based_lighting_rotation_finite:
        Option<Established<ImageBasedLightingRotationFinite>>,
    /// Optional proof that the diffuse cubemap asset is renderable.
    pub image_based_lighting_diffuse_map_asset:
        Option<Established<ImageBasedLightingDiffuseMapAssetValid>>,
    /// Optional proof that the specular cubemap asset is renderable.
    pub image_based_lighting_specular_map_asset:
        Option<Established<ImageBasedLightingSpecularMapAssetValid>>,
    /// Optional proof that the environment-map asset is renderable.
    pub image_based_lighting_environment_map_asset:
        Option<Established<ImageBasedLightingEnvironmentMapAssetValid>>,
    /// Optional proof that atmosphere-generated cubemap dimensions are positive.
    pub image_based_lighting_atmosphere_cubemap_dimensions_positive:
        Option<Established<ImageBasedLightingAtmosphereCubemapDimensionsPositive>>,
}

impl ProvableFrom<RenderImageBasedLightingEvidence>
    for RenderImageBasedLightingValid
{
}

/// Evidence that an atmosphere falloff descriptor is renderable.
pub struct RenderAtmosphereFalloffEvidence {
    /// Proof that the falloff family is explicitly declared.
    pub atmosphere_falloff_declared: Established<AtmosphereFalloffDeclared>,
    /// Optional proof that exponential scales are non-negative.
    pub atmosphere_falloff_scale_non_negative:
        Option<Established<AtmosphereFalloffScaleNonNegative>>,
    /// Optional proof that tent centers stay in the unit interval.
    pub atmosphere_falloff_center_unit_interval:
        Option<Established<AtmosphereFalloffCenterUnitInterval>>,
    /// Optional proof that tent widths stay in the unit interval.
    pub atmosphere_falloff_width_unit_interval:
        Option<Established<AtmosphereFalloffWidthUnitInterval>>,
}

impl ProvableFrom<RenderAtmosphereFalloffEvidence> for RenderAtmosphereFalloffValid {}

/// Evidence that an atmosphere phase-function descriptor is renderable.
pub struct RenderAtmospherePhaseFunctionEvidence {
    /// Proof that the phase-function family is explicitly declared.
    pub atmosphere_phase_function_declared:
        Established<AtmospherePhaseFunctionDeclared>,
    /// Optional proof that asymmetry values are finite.
    pub atmosphere_phase_function_asymmetry_finite:
        Option<Established<AtmospherePhaseFunctionAsymmetryFinite>>,
    /// Optional proof that asymmetry values stay in the normalized interval.
    pub atmosphere_phase_function_asymmetry_normalized:
        Option<Established<AtmospherePhaseFunctionAsymmetryNormalized>>,
}

impl ProvableFrom<RenderAtmospherePhaseFunctionEvidence>
    for RenderAtmospherePhaseFunctionValid
{
}

/// Evidence that one atmosphere scattering term is renderable.
pub struct RenderAtmosphereScatteringTermEvidence {
    /// Proof that absorption/scattering coefficients are finite.
    pub atmosphere_scattering_coefficients_finite:
        Established<AtmosphereScatteringCoefficientsFinite>,
    /// Proof that absorption/scattering coefficients are non-negative.
    pub atmosphere_scattering_coefficients_non_negative:
        Established<AtmosphereScatteringCoefficientsNonNegative>,
    /// Proof that the term falloff descriptor is valid.
    pub falloff: Established<RenderAtmosphereFalloffValid>,
    /// Proof that the term phase-function descriptor is valid.
    pub phase: Established<RenderAtmospherePhaseFunctionValid>,
}

impl ProvableFrom<RenderAtmosphereScatteringTermEvidence>
    for RenderAtmosphereScatteringTermValid
{
}

/// Evidence that a visible solar-disk descriptor is renderable.
pub struct RenderSunDiskEvidence {
    /// Optional proof that angular size stays non-negative.
    pub sun_disk_angular_size_non_negative:
        Option<Established<SunDiskAngularSizeNonNegative>>,
    /// Optional proof that intensity stays non-negative.
    pub sun_disk_intensity_non_negative:
        Option<Established<SunDiskIntensityNonNegative>>,
}

impl ProvableFrom<RenderSunDiskEvidence> for RenderSunDiskValid {}

/// Evidence that a skybox descriptor is renderable.
pub struct RenderSkyboxEvidence {
    /// Proof that the skybox asset is renderable.
    pub asset: Established<RenderAssetReferenceValid>,
    /// Proof that skybox brightness is non-negative.
    pub skybox_brightness_non_negative: Established<SkyboxBrightnessNonNegative>,
    /// Optional proof that skybox rotation values are finite.
    pub skybox_rotation_finite: Option<Established<SkyboxRotationFinite>>,
}

impl ProvableFrom<RenderSkyboxEvidence> for RenderSkyboxValid {}

/// Evidence that an ambient-light descriptor is renderable.
pub struct RenderAmbientLightEvidence {
    /// Proof that ambient-light brightness is non-negative.
    pub ambient_light_brightness_non_negative:
        Established<AmbientLightBrightnessNonNegative>,
}

impl ProvableFrom<RenderAmbientLightEvidence> for RenderAmbientLightValid {}

/// Evidence that a cascade-shadow descriptor is renderable.
pub struct RenderCascadeShadowConfigEvidence {
    /// Optional proof that explicit cascade far bounds are strictly positive.
    pub cascade_shadow_bounds_positive:
        Option<Established<CascadeShadowBoundsPositive>>,
    /// Optional proof that explicit cascade far bounds are strictly ordered.
    pub cascade_shadow_bounds_ordered:
        Option<Established<CascadeShadowBoundsOrdered>>,
    /// Optional proof that overlap proportions stay in the half-open unit interval.
    pub cascade_shadow_overlap_half_open_unit_interval:
        Option<Established<CascadeShadowOverlapHalfOpenUnitInterval>>,
    /// Optional proof that minimum shadow distance is non-negative.
    pub cascade_shadow_minimum_distance_non_negative:
        Option<Established<CascadeShadowMinimumDistanceNonNegative>>,
    /// Optional proof that maximum shadow distance is ordered relative to minimum distance.
    pub cascade_shadow_maximum_distance_ordered:
        Option<Established<CascadeShadowMaximumDistanceOrdered>>,
    /// Optional proof that the first-cascade far bound is strictly positive.
    pub cascade_shadow_first_cascade_far_bound_positive:
        Option<Established<CascadeShadowFirstCascadeFarBoundPositive>>,
    /// Optional proof that the first-cascade far bound is greater than minimum distance.
    pub cascade_shadow_first_cascade_far_bound_greater_than_minimum:
        Option<Established<CascadeShadowFirstCascadeFarBoundGreaterThanMinimum>>,
    /// Optional proof that the cascade count is strictly positive.
    pub cascade_shadow_cascade_count_positive:
        Option<Established<CascadeShadowCascadeCountPositive>>,
}

impl ProvableFrom<RenderCascadeShadowConfigEvidence> for RenderCascadeShadowConfigValid {}

/// Evidence that a directional-light descriptor is renderable.
pub struct RenderDirectionalLightEvidence {
    /// Proof that directional-light illuminance is non-negative.
    pub directional_light_illuminance_non_negative:
        Established<DirectionalLightIlluminanceNonNegative>,
    /// Proof that directional-light angles are finite.
    pub light_angles_finite: Established<LightAnglesFinite>,
    /// Optional proof that directional cascade-shadow policy is valid.
    pub cascade_shadows: Option<Established<RenderCascadeShadowConfigValid>>,
}

impl ProvableFrom<RenderDirectionalLightEvidence> for RenderDirectionalLightValid {}

/// Evidence that an atmosphere descriptor is renderable.
pub struct RenderAtmosphereEvidence {
    /// Proof that the atmosphere shell radii are strictly positive.
    pub atmosphere_shell_radii_positive: Established<AtmosphereShellRadiiPositive>,
    /// Proof that the atmosphere shell radii are strictly ordered.
    pub atmosphere_shell_radii_ordered: Established<AtmosphereShellRadiiOrdered>,
    /// Proof that ground-albedo values are finite.
    pub atmosphere_ground_albedo_finite:
        Established<AtmosphereGroundAlbedoFinite>,
    /// Proof that ground-albedo values stay in the unit interval.
    pub atmosphere_ground_albedo_unit_interval:
        Established<AtmosphereGroundAlbedoUnitInterval>,
    /// Proof that precomputation resolutions are strictly positive.
    pub atmosphere_scattering_resolutions_positive:
        Established<AtmosphereScatteringResolutionsPositive>,
    /// Proof that at least one scattering term is present.
    pub atmosphere_scattering_terms_present:
        Established<AtmosphereScatteringTermsPresent>,
    /// Optional proof that density multipliers are finite.
    pub atmosphere_density_multiplier_finite:
        Option<Established<AtmosphereDensityMultiplierFinite>>,
    /// Optional proof that the visible solar-disk policy is valid.
    pub sun_disk: Option<Established<RenderSunDiskValid>>,
    /// Proofs that each scattering term is renderable.
    pub terms: Vec<Established<RenderAtmosphereScatteringTermValid>>,
}

impl ProvableFrom<RenderAtmosphereEvidence> for RenderAtmosphereValid {}

/// Evidence that a scene environment descriptor is renderable.
pub struct RenderSceneEnvironmentEvidence {
    /// Optional proof that the skybox descriptor is valid.
    pub skybox: Option<Established<RenderSkyboxValid>>,
    /// Optional proof that the ambient-light descriptor is valid.
    pub ambient_light: Option<Established<RenderAmbientLightValid>>,
    /// Optional proof that the directional-light descriptor is valid.
    pub directional_light: Option<Established<RenderDirectionalLightValid>>,
    /// Optional proof that image-based-lighting policy is valid.
    pub image_based_lighting: Option<Established<RenderImageBasedLightingValid>>,
    /// Optional proof that atmosphere policy is valid.
    pub atmosphere: Option<Established<RenderAtmosphereValid>>,
    /// Proofs that localized light probes are renderable.
    pub light_probes: Vec<Established<RenderLightProbeValid>>,
    /// Optional proof that fog policy is valid.
    pub fog: Option<Established<RenderFogValid>>,
    /// Optional proof that volumetric-fog policy is valid.
    pub volumetric_fog: Option<Established<RenderVolumetricFogValid>>,
    /// Proofs that localized fog volumes are renderable.
    pub fog_volumes: Vec<Established<RenderFogVolumeValid>>,
}

impl ProvableFrom<RenderSceneEnvironmentEvidence> for RenderSceneEnvironmentValid {}

/// Evidence that a vector layer is renderable.
pub struct RenderableVectorLayerEvidence {
    /// Proof that the layer identifier is non-empty.
    pub layer_id_non_empty: Established<LayerIdNonEmpty>,
    /// Proof that the layer name is non-empty.
    pub layer_name_non_empty: Established<LayerNameNonEmpty>,
    /// Proof that layer opacity is in the unit interval.
    pub layer_opacity_unit_interval: Established<LayerOpacityUnitInterval>,
    /// Proof that draw order is assigned.
    pub layer_draw_order_assigned: Established<LayerDrawOrderAssigned>,
    /// Proof that visibility is declared.
    pub layer_visibility_declared: Established<LayerVisibilityDeclared>,
    /// Proof that vector payload was provided.
    pub vector_payload_declared: Established<VectorPayloadDeclared>,
    /// Proof that payload geometry has been projected for the active view.
    pub geometry_projected_for_view: Established<GeometryProjectedForView>,
    /// Proof that vector symbolization is fully resolved.
    pub vector_symbolization_resolved: Established<VectorSymbolizationResolved>,
    /// Proof that the chosen style is compatible with the payload family.
    pub vector_style_payload_compatible:
        Established<VectorStylePayloadCompatible>,
    /// Optional proof that batching and instancing hints are valid.
    pub batching: Option<Established<RenderFeatureBatchingValid>>,
    /// Proof that the style descriptor is valid.
    pub style: Established<RenderFeatureStyleValid>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableVectorLayerEvidence> for RenderableVectorLayerValid {}

/// Evidence that a raster layer is renderable.
pub struct RenderableRasterLayerEvidence {
    /// Proof that the layer identifier is non-empty.
    pub layer_id_non_empty: Established<LayerIdNonEmpty>,
    /// Proof that the layer name is non-empty.
    pub layer_name_non_empty: Established<LayerNameNonEmpty>,
    /// Proof that layer opacity is in the unit interval.
    pub layer_opacity_unit_interval: Established<LayerOpacityUnitInterval>,
    /// Proof that draw order is assigned.
    pub layer_draw_order_assigned: Established<LayerDrawOrderAssigned>,
    /// Proof that visibility is declared.
    pub layer_visibility_declared: Established<LayerVisibilityDeclared>,
    /// Proof that raster source metadata was provided.
    pub raster_source: Established<RenderRasterSourceValid>,
    /// Proof that raster image metadata was provided.
    pub raster_image_metadata_declared: Established<RasterImageMetadataDeclared>,
    /// Proof that raster dimensions are positive when declared.
    pub raster_dimensions_positive: Established<RasterDimensionsPositive>,
    /// Proof that the style descriptor is valid.
    pub style: Established<RenderRasterStyleValid>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableRasterLayerEvidence> for RenderableRasterLayerValid {}

/// Evidence that a tile layer is renderable.
pub struct RenderableTileLayerEvidence {
    /// Proof that the layer identifier is non-empty.
    pub layer_id_non_empty: Established<LayerIdNonEmpty>,
    /// Proof that the layer name is non-empty.
    pub layer_name_non_empty: Established<LayerNameNonEmpty>,
    /// Proof that layer opacity is in the unit interval.
    pub layer_opacity_unit_interval: Established<LayerOpacityUnitInterval>,
    /// Proof that draw order is assigned.
    pub layer_draw_order_assigned: Established<LayerDrawOrderAssigned>,
    /// Proof that visibility is declared.
    pub layer_visibility_declared: Established<LayerVisibilityDeclared>,
    /// Proof that the tile source descriptor is valid.
    pub source: Established<RenderTileSourceValid>,
    /// Proof that the tile style descriptor is valid.
    pub style: Established<RenderTileStyleValid>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableTileLayerEvidence> for RenderableTileLayerValid {}

/// Evidence that a terrain layer is renderable.
pub struct RenderableTerrainLayerEvidence {
    /// Proof that the layer identifier is non-empty.
    pub layer_id_non_empty: Established<LayerIdNonEmpty>,
    /// Proof that the layer name is non-empty.
    pub layer_name_non_empty: Established<LayerNameNonEmpty>,
    /// Proof that layer opacity is in the unit interval.
    pub layer_opacity_unit_interval: Established<LayerOpacityUnitInterval>,
    /// Proof that draw order is assigned.
    pub layer_draw_order_assigned: Established<LayerDrawOrderAssigned>,
    /// Proof that visibility is declared.
    pub layer_visibility_declared: Established<LayerVisibilityDeclared>,
    /// Proof that raster source metadata was provided.
    pub raster_source: Established<RenderRasterSourceValid>,
    /// Proof that raster image metadata was provided.
    pub raster_image_metadata_declared: Established<RasterImageMetadataDeclared>,
    /// Proof that raster dimensions are positive when declared.
    pub raster_dimensions_positive: Established<RasterDimensionsPositive>,
    /// Proof that the terrain style descriptor is valid.
    pub style: Established<RenderTerrainStyleValid>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableTerrainLayerEvidence> for RenderableTerrainLayerValid {}

/// Evidence that an annotation layer is renderable.
pub struct RenderableAnnotationLayerEvidence {
    /// Proof that the layer identifier is non-empty.
    pub layer_id_non_empty: Established<LayerIdNonEmpty>,
    /// Proof that the layer name is non-empty.
    pub layer_name_non_empty: Established<LayerNameNonEmpty>,
    /// Proof that layer opacity is in the unit interval.
    pub layer_opacity_unit_interval: Established<LayerOpacityUnitInterval>,
    /// Proof that draw order is assigned.
    pub layer_draw_order_assigned: Established<LayerDrawOrderAssigned>,
    /// Proof that visibility is declared.
    pub layer_visibility_declared: Established<LayerVisibilityDeclared>,
    /// Proof that annotation payload was provided.
    pub annotation_payload_declared: Established<AnnotationPayloadDeclared>,
    /// Proof that each annotation entry is renderable.
    pub annotations: Vec<Established<RenderAnnotationValid>>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableAnnotationLayerEvidence> for RenderableAnnotationLayerValid {}

/// Evidence that a full render scene is valid.
pub struct RenderableSceneEvidence {
    /// Proof that the scene identifier is non-empty.
    pub scene_id_non_empty: Established<SceneIdNonEmpty>,
    /// Proof that the scene declares its layer list.
    pub scene_layer_list_declared: Established<SceneLayerListDeclared>,
    /// Proof that the scene declares its view.
    pub scene_view_declared: Established<SceneViewDeclared>,
    /// Proof that scene layer ordering is deterministic.
    pub layer_order_deterministic: Established<LayerOrderDeterministic>,
    /// Proof that the scene view descriptor is valid.
    pub view: Established<RenderViewValid>,
    /// Optional proof that the scene environment descriptor is valid.
    pub environment: Option<Established<RenderSceneEnvironmentValid>>,
    /// Proof that every layer in the scene is renderable.
    pub layers: Vec<Established<RenderableLayerValid>>,
}

impl ProvableFrom<RenderableSceneEvidence> for RenderableSceneValid {}

/// Evidence that a scene update is renderable.
pub struct RenderableSceneUpdateEvidence {
    /// Proof that the scene identifier is non-empty.
    pub scene_id_non_empty: Established<SceneIdNonEmpty>,
    /// Proof that the update operation is explicitly declared.
    pub scene_update_operation_declared:
        Established<SceneUpdateOperationDeclared>,
    /// Optional proof that the target layer identifier is non-empty.
    pub target_layer_id_non_empty:
        Option<Established<SceneUpdateTargetLayerNonEmpty>>,
    /// Optional proof that the reference layer identifier is non-empty.
    pub reference_layer_id_non_empty:
        Option<Established<SceneUpdateReferenceLayerNonEmpty>>,
    /// Optional proof that the update carries a valid view.
    pub view: Option<Established<RenderViewValid>>,
    /// Optional proof that the update carries a valid environment.
    pub environment: Option<Established<RenderSceneEnvironmentValid>>,
    /// Optional proof that the update carries a valid layer.
    pub layer: Option<Established<RenderableLayerValid>>,
    /// Optional proof that replacement opacity is valid.
    pub layer_opacity_unit_interval:
        Option<Established<LayerOpacityUnitInterval>>,
    /// Optional proof that replacement draw order is assigned.
    pub layer_draw_order_assigned:
        Option<Established<LayerDrawOrderAssigned>>,
}

impl ProvableFrom<RenderableSceneUpdateEvidence> for RenderableSceneUpdateValid {}

impl ProvableFrom<Established<RenderableVectorLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableRasterLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableTileLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableTerrainLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableAnnotationLayerValid>> for RenderableLayerValid {}
