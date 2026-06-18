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

    /// A render view identifier is non-empty.
    pub struct RenderViewIdNonEmpty;
    structural_prop!(RenderViewIdNonEmpty);

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

    /// A render view explicitly establishes its perspective-projection policy.
    pub struct PerspectiveProjectionPolicyDeclared;
    structural_prop!(PerspectiveProjectionPolicyDeclared);

    /// A render view explicitly declares its output policy when one is present.
    pub struct RenderViewOutputDeclared;
    structural_prop!(RenderViewOutputDeclared);

    /// A render view explicitly establishes its output selection.
    pub struct RenderViewOutputSelectionDeclared;
    structural_prop!(RenderViewOutputSelectionDeclared);

    /// A render viewport is explicitly declared.
    pub struct RenderViewportDeclared;
    structural_prop!(RenderViewportDeclared);

    /// A render view-output descriptor explicitly establishes its viewport policy.
    pub struct RenderViewportPolicyDeclared;
    structural_prop!(RenderViewportPolicyDeclared);

    /// A render viewport's physical size is strictly positive.
    pub struct RenderViewportSizePositive;
    structural_prop!(RenderViewportSizePositive);

    /// A render resolution override is explicitly declared.
    pub struct RenderResolutionOverrideDeclared;
    structural_prop!(RenderResolutionOverrideDeclared);

    /// A render view-output descriptor explicitly establishes its resolution-override policy.
    pub struct RenderResolutionOverridePolicyDeclared;
    structural_prop!(RenderResolutionOverridePolicyDeclared);

    /// A render resolution override is strictly positive.
    pub struct RenderResolutionOverridePositive;
    structural_prop!(RenderResolutionOverridePositive);

    /// A render subview layout is explicitly declared.
    pub struct RenderSubViewLayoutDeclared;
    structural_prop!(RenderSubViewLayoutDeclared);

    /// A render view-output descriptor explicitly establishes its subview-layout policy.
    pub struct RenderSubViewLayoutPolicyDeclared;
    structural_prop!(RenderSubViewLayoutPolicyDeclared);

    /// A render subview layout declares strictly positive full extents.
    pub struct RenderSubViewLayoutFullExtentPositive;
    structural_prop!(RenderSubViewLayoutFullExtentPositive);

    /// A render subview layout declares strictly positive subview extents.
    pub struct RenderSubViewLayoutSubviewExtentPositive;
    structural_prop!(RenderSubViewLayoutSubviewExtentPositive);

    /// A render subview layout carries finite offsets.
    pub struct RenderSubViewLayoutOffsetFinite;
    structural_prop!(RenderSubViewLayoutOffsetFinite);

    /// A render subview layout stays within the declared full extent.
    pub struct RenderSubViewLayoutWithinFullExtent;
    structural_prop!(RenderSubViewLayoutWithinFullExtent);

    /// A layer's view-routing mode is explicitly declared.
    pub struct ViewParticipationModeDeclared;
    structural_prop!(ViewParticipationModeDeclared);

    /// Declared layer-routing view identifiers are non-empty.
    pub struct ViewParticipationViewIdsNonEmpty;
    structural_prop!(ViewParticipationViewIdsNonEmpty);

    /// Declared layer-routing view identifiers are unique.
    pub struct ViewParticipationViewIdsUnique;
    structural_prop!(ViewParticipationViewIdsUnique);

    /// The declared view list is structurally compatible with the routing mode.
    pub struct ViewParticipationListMatchesMode;
    structural_prop!(ViewParticipationListMatchesMode);

    /// Tonemapping selection is explicitly established for one view-output descriptor.
    pub struct TonemappingPolicyDeclared;
    structural_prop!(TonemappingPolicyDeclared);

    /// HDR output policy is explicitly declared.
    pub struct HdrOutputDeclared;
    structural_prop!(HdrOutputDeclared);

    /// Tonemapping policy is explicitly declared.
    pub struct TonemappingDeclared;
    structural_prop!(TonemappingDeclared);

    /// MSAA policy is explicitly declared.
    pub struct MsaaDeclared;
    structural_prop!(MsaaDeclared);

    /// MSAA selection is explicitly established for one view-output descriptor.
    pub struct MsaaPolicyDeclared;
    structural_prop!(MsaaPolicyDeclared);

    /// Mip-bias values are finite when supplied.
    pub struct MipBiasFinite;
    structural_prop!(MipBiasFinite);

    /// Mip-bias selection is explicitly established for one view-output descriptor.
    pub struct MipBiasPolicyDeclared;
    structural_prop!(MipBiasPolicyDeclared);

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

    /// A render view explicitly establishes its world-space selection.
    pub struct RenderWorldSpaceSelectionDeclared;
    structural_prop!(RenderWorldSpaceSelectionDeclared);

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

    /// Asset color-space selection is explicitly established for one asset reference.
    pub struct AssetColorSpaceSelectionDeclared;
    structural_prop!(AssetColorSpaceSelectionDeclared);

    /// An external render asset's residency intent is explicitly declared.
    pub struct AssetResidencyPolicyDeclared;
    structural_prop!(AssetResidencyPolicyDeclared);

    /// Asset residency selection is explicitly established for one asset reference.
    pub struct AssetResidencyPolicySelectionDeclared;
    structural_prop!(AssetResidencyPolicySelectionDeclared);

    /// An external render asset's access/update pattern is explicitly declared.
    pub struct AssetAccessPatternDeclared;
    structural_prop!(AssetAccessPatternDeclared);

    /// Asset access-pattern selection is explicitly established for one asset reference.
    pub struct AssetAccessPatternSelectionDeclared;
    structural_prop!(AssetAccessPatternSelectionDeclared);

    /// An external render asset's revision metadata is explicitly declared.
    pub struct AssetRevisionDeclared;
    structural_prop!(AssetRevisionDeclared);

    /// An external render asset's revision metadata is non-empty when supplied.
    pub struct AssetRevisionNonEmpty;
    structural_prop!(AssetRevisionNonEmpty);

    /// Asset revision selection is explicitly established for one asset reference.
    pub struct AssetRevisionSelectionDeclared;
    structural_prop!(AssetRevisionSelectionDeclared);

    /// An external render asset's integrity metadata is explicitly declared.
    pub struct AssetIntegrityDeclared;
    structural_prop!(AssetIntegrityDeclared);

    /// An external render asset's integrity digest is non-empty when supplied.
    pub struct AssetIntegrityDigestNonEmpty;
    structural_prop!(AssetIntegrityDigestNonEmpty);

    /// Asset integrity selection is explicitly established for one asset reference.
    pub struct AssetIntegritySelectionDeclared;
    structural_prop!(AssetIntegritySelectionDeclared);

    /// Asset logical-size selection is explicitly established for one asset reference.
    pub struct AssetLogicalSizeSelectionDeclared;
    structural_prop!(AssetLogicalSizeSelectionDeclared);

    /// A material family is explicitly declared.
    pub struct MaterialFamilyDeclared;
    structural_prop!(MaterialFamilyDeclared);

    /// A material alpha-compositing mode is explicitly declared when supplied.
    pub struct MaterialAlphaModeDeclared;
    structural_prop!(MaterialAlphaModeDeclared);

    /// Material alpha-mode selection is explicitly established for one material intent.
    pub struct MaterialAlphaModeSelectionDeclared;
    structural_prop!(MaterialAlphaModeSelectionDeclared);

    /// Material alpha-mask thresholds stay in the closed unit interval.
    pub struct MaterialMaskThresholdUnitInterval;
    structural_prop!(MaterialMaskThresholdUnitInterval);

    /// A material face-culling mode is explicitly declared when supplied.
    pub struct MaterialCullModeDeclared;
    structural_prop!(MaterialCullModeDeclared);

    /// Material cull-mode selection is explicitly established for one material intent.
    pub struct MaterialCullModeSelectionDeclared;
    structural_prop!(MaterialCullModeSelectionDeclared);

    /// Material unlit selection is explicitly established for one material intent.
    pub struct MaterialUnlitSelectionDeclared;
    structural_prop!(MaterialUnlitSelectionDeclared);

    /// Material depth-write selection is explicitly established for one material intent.
    pub struct MaterialDepthWriteSelectionDeclared;
    structural_prop!(MaterialDepthWriteSelectionDeclared);

    /// Material depth-bias values are finite when supplied.
    pub struct MaterialDepthBiasFinite;
    structural_prop!(MaterialDepthBiasFinite);

    /// Material depth-bias selection is explicitly established for one material intent.
    pub struct MaterialDepthBiasSelectionDeclared;
    structural_prop!(MaterialDepthBiasSelectionDeclared);

    /// Material specialization keys are non-empty when supplied.
    pub struct MaterialSpecializationKeyNonEmpty;
    structural_prop!(MaterialSpecializationKeyNonEmpty);

    /// Material specialization-key selection is explicitly established for one material intent.
    pub struct MaterialSpecializationKeySelectionDeclared;
    structural_prop!(MaterialSpecializationKeySelectionDeclared);

    /// A material texture semantic is explicitly declared.
    pub struct MaterialTextureSemanticDeclared;
    structural_prop!(MaterialTextureSemanticDeclared);

    /// A material pipeline domain is explicitly declared when supplied.
    pub struct MaterialPipelineDomainDeclared;
    structural_prop!(MaterialPipelineDomainDeclared);

    /// Material pipeline-intent selection is explicitly established for one material intent.
    pub struct RenderPipelineIntentSelectionDeclared;
    structural_prop!(RenderPipelineIntentSelectionDeclared);

    /// A material opaque rendering method is explicitly declared when supplied.
    pub struct MaterialOpaqueMethodDeclared;
    structural_prop!(MaterialOpaqueMethodDeclared);

    /// Material opaque-method selection is explicitly established for one pipeline intent.
    pub struct MaterialOpaqueMethodSelectionDeclared;
    structural_prop!(MaterialOpaqueMethodSelectionDeclared);

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

    /// A render view-output descriptor explicitly establishes its output-target policy.
    pub struct RenderOutputTargetPolicyDeclared;
    structural_prop!(RenderOutputTargetPolicyDeclared);

    /// A symbolic render output target identifier is non-empty when supplied.
    pub struct RenderOutputTargetIdNonEmpty;
    structural_prop!(RenderOutputTargetIdNonEmpty);

    /// Render output-target identifier selection is explicitly established for one target.
    pub struct RenderOutputTargetIdSelectionDeclared;
    structural_prop!(RenderOutputTargetIdSelectionDeclared);

    /// Render output-target asset selection is explicitly established for one target.
    pub struct RenderOutputTargetAssetSelectionDeclared;
    structural_prop!(RenderOutputTargetAssetSelectionDeclared);

    /// A render clear policy is explicitly declared when supplied.
    pub struct RenderClearPolicyDeclared;
    structural_prop!(RenderClearPolicyDeclared);

    /// A render view-output descriptor explicitly establishes its clear-policy selection.
    pub struct RenderClearPolicySelectionDeclared;
    structural_prop!(RenderClearPolicySelectionDeclared);

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

    /// Stroke selection is explicitly established for one feature symbolizer.
    pub struct RenderStrokeSelectionDeclared;
    structural_prop!(RenderStrokeSelectionDeclared);

    /// Fill selection is explicitly established for one feature symbolizer.
    pub struct RenderFillSelectionDeclared;
    structural_prop!(RenderFillSelectionDeclared);

    /// Point-symbol selection is explicitly established for one feature symbolizer.
    pub struct RenderPointSymbolSelectionDeclared;
    structural_prop!(RenderPointSymbolSelectionDeclared);

    /// Label selection is explicitly established for one feature symbolizer.
    pub struct RenderLabelSelectionDeclared;
    structural_prop!(RenderLabelSelectionDeclared);

    /// Placement selection is explicitly established for one feature symbolizer.
    pub struct RenderFeaturePlacementSelectionDeclared;
    structural_prop!(RenderFeaturePlacementSelectionDeclared);

    /// Extrusion selection is explicitly established for one feature symbolizer.
    pub struct RenderExtrusionSelectionDeclared;
    structural_prop!(RenderExtrusionSelectionDeclared);

    /// Shadow-participation selection is explicitly established for one feature symbolizer.
    pub struct RenderFeatureShadowParticipationSelectionDeclared;
    structural_prop!(RenderFeatureShadowParticipationSelectionDeclared);

    /// Material-intent selection is explicitly established for one feature symbolizer.
    pub struct RenderFeatureMaterialIntentSelectionDeclared;
    structural_prop!(RenderFeatureMaterialIntentSelectionDeclared);

    /// Scale-range selection is explicitly established for one rule or annotation.
    pub struct RenderScaleRangeSelectionDeclared;
    structural_prop!(RenderScaleRangeSelectionDeclared);

    /// Feature-batch limit selection is explicitly established for one batching descriptor.
    pub struct FeatureBatchLimitSelectionDeclared;
    structural_prop!(FeatureBatchLimitSelectionDeclared);

    /// Feature-batching selection is explicitly established for one vector layer.
    pub struct RenderFeatureBatchingSelectionDeclared;
    structural_prop!(RenderFeatureBatchingSelectionDeclared);

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

    /// A 3D model layer carries a non-empty asset reference.
    pub struct ModelAssetDeclared;
    structural_prop!(ModelAssetDeclared);

    /// A 3D model placement descriptor carries finite, valid coordinates.
    pub struct ModelPlacementValid;
    structural_prop!(ModelPlacementValid);

    /// A 3D model layer is fully specified and renderable.
    pub struct RenderableModelLayerValid;
    structural_prop!(RenderableModelLayerValid);

    /// A layer group has a non-empty stable identifier.
    pub struct LayerGroupIdNonEmpty;
    structural_prop!(LayerGroupIdNonEmpty);

    /// A layer group has a non-empty human-readable name.
    pub struct LayerGroupNameNonEmpty;
    structural_prop!(LayerGroupNameNonEmpty);

    /// A layer group's opacity is within the unit interval [0.0, 1.0].
    pub struct LayerGroupOpacityUnitInterval;
    structural_prop!(LayerGroupOpacityUnitInterval);

    /// A layer group descriptor is fully validated and safe to embed in a scene.
    pub struct RenderableLayerGroupValid;
    structural_prop!(RenderableLayerGroupValid);

    /// A scene explicitly declares the layer list it contains.
    pub struct SceneLayerListDeclared;
    structural_prop!(SceneLayerListDeclared);

    /// A scene explicitly declares which validated view it uses.
    pub struct SceneViewDeclared;
    structural_prop!(SceneViewDeclared);

    /// A scene explicitly declares its auxiliary views when present.
    pub struct SceneAuxiliaryViewsDeclared;
    structural_prop!(SceneAuxiliaryViewsDeclared);

    /// Auxiliary scene-view identifiers are unique.
    pub struct SceneAuxiliaryViewIdsUnique;
    structural_prop!(SceneAuxiliaryViewIdsUnique);

    /// Auxiliary scene-view identifiers are distinct from the primary view identifier.
    pub struct SceneAuxiliaryViewsDistinctFromPrimary;
    structural_prop!(SceneAuxiliaryViewsDistinctFromPrimary);

    /// Every layer view-routing reference resolves against the scene's views.
    pub struct SceneLayerViewParticipationResolved;
    structural_prop!(SceneLayerViewParticipationResolved);

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

    /// Fog selection is explicitly established for one scene-environment descriptor.
    pub struct RenderFogSelectionDeclared;
    structural_prop!(RenderFogSelectionDeclared);

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

    /// Volumetric-fog selection is explicitly established for one scene-environment descriptor.
    pub struct RenderVolumetricFogSelectionDeclared;
    structural_prop!(RenderVolumetricFogSelectionDeclared);

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

    /// Fixed-exposure selection is explicitly established for one view-output descriptor.
    pub struct RenderExposureSelectionDeclared;
    structural_prop!(RenderExposureSelectionDeclared);

    /// Color-grading selection is explicitly established for one view-output descriptor.
    pub struct RenderColorGradingSelectionDeclared;
    structural_prop!(RenderColorGradingSelectionDeclared);

    /// Auto-exposure selection is explicitly established for one view-output descriptor.
    pub struct RenderAutoExposureSelectionDeclared;
    structural_prop!(RenderAutoExposureSelectionDeclared);

    /// Bloom floating-point parameters are finite.
    pub struct BloomParametersFinite;
    structural_prop!(BloomParametersFinite);

    /// Bloom selection is explicitly established for one view-output descriptor.
    pub struct RenderBloomSelectionDeclared;
    structural_prop!(RenderBloomSelectionDeclared);

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

    /// Depth-of-field selection is explicitly established for one view-output descriptor.
    pub struct RenderDepthOfFieldSelectionDeclared;
    structural_prop!(RenderDepthOfFieldSelectionDeclared);

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

    /// Motion-blur selection is explicitly established for one view-output descriptor.
    pub struct RenderMotionBlurSelectionDeclared;
    structural_prop!(RenderMotionBlurSelectionDeclared);

    /// Motion-blur sample counts are strictly positive.
    pub struct MotionBlurSamplesPositive;
    structural_prop!(MotionBlurSamplesPositive);

    /// Chromatic-aberration floating-point parameters are finite.
    pub struct ChromaticAberrationParametersFinite;
    structural_prop!(ChromaticAberrationParametersFinite);

    /// Chromatic-aberration selection is explicitly established for one view-output descriptor.
    pub struct RenderChromaticAberrationSelectionDeclared;
    structural_prop!(RenderChromaticAberrationSelectionDeclared);

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

    /// Screen-space reflections selection is explicitly established for one view-output descriptor.
    pub struct RenderScreenSpaceReflectionsSelectionDeclared;
    structural_prop!(RenderScreenSpaceReflectionsSelectionDeclared);

    /// Screen-space ambient-occlusion quality is explicitly declared.
    pub struct ScreenSpaceAmbientOcclusionQualityDeclared;
    structural_prop!(ScreenSpaceAmbientOcclusionQualityDeclared);

    /// Screen-space ambient-occlusion selection is explicitly established for one view-output descriptor.
    pub struct RenderScreenSpaceAmbientOcclusionSelectionDeclared;
    structural_prop!(RenderScreenSpaceAmbientOcclusionSelectionDeclared);

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

    /// Shadow-filtering-method selection is explicitly established for one view-output descriptor.
    pub struct ShadowFilteringMethodSelectionDeclared;
    structural_prop!(ShadowFilteringMethodSelectionDeclared);

    /// Screen-space transmission quality is explicitly declared.
    pub struct ScreenSpaceTransmissionQualityDeclared;
    structural_prop!(ScreenSpaceTransmissionQualityDeclared);

    /// Screen-space transmission-quality selection is explicitly established for one view-output descriptor.
    pub struct ScreenSpaceTransmissionQualitySelectionDeclared;
    structural_prop!(ScreenSpaceTransmissionQualitySelectionDeclared);

    /// Skybox brightness is non-negative.
    pub struct SkyboxBrightnessNonNegative;
    structural_prop!(SkyboxBrightnessNonNegative);

    /// Skybox selection is explicitly established for one scene-environment descriptor.
    pub struct RenderSkyboxSelectionDeclared;
    structural_prop!(RenderSkyboxSelectionDeclared);

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

    /// Ambient-light selection is explicitly established for one scene-environment descriptor.
    pub struct RenderAmbientLightSelectionDeclared;
    structural_prop!(RenderAmbientLightSelectionDeclared);

    /// Directional-light selection is explicitly established for one scene-environment descriptor.
    pub struct RenderDirectionalLightSelectionDeclared;
    structural_prop!(RenderDirectionalLightSelectionDeclared);

    /// Irradiance-volume voxel assets are renderable.
    pub struct IrradianceVolumeVoxelsAssetValid;
    structural_prop!(IrradianceVolumeVoxelsAssetValid);

    /// Image-based-lighting source policy is explicitly declared.
    pub struct ImageBasedLightingSourceDeclared;
    structural_prop!(ImageBasedLightingSourceDeclared);

    /// Image-based-lighting selection is explicitly established for one scene-environment descriptor.
    pub struct RenderImageBasedLightingSelectionDeclared;
    structural_prop!(RenderImageBasedLightingSelectionDeclared);

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

    /// Atmosphere selection is explicitly established for one scene-environment descriptor.
    pub struct RenderAtmosphereSelectionDeclared;
    structural_prop!(RenderAtmosphereSelectionDeclared);

    /// Sun-disk selection is explicitly established for one atmosphere descriptor.
    pub struct RenderSunDiskSelectionDeclared;
    structural_prop!(RenderSunDiskSelectionDeclared);

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

    /// Asset color-space selection for one asset reference is valid.
    pub struct AssetColorSpaceSelectionValid;
    structural_prop!(AssetColorSpaceSelectionValid);

    /// Asset residency selection for one asset reference is valid.
    pub struct AssetResidencyPolicySelectionValid;
    structural_prop!(AssetResidencyPolicySelectionValid);

    /// Asset access-pattern selection for one asset reference is valid.
    pub struct AssetAccessPatternSelectionValid;
    structural_prop!(AssetAccessPatternSelectionValid);

    /// Asset revision selection for one asset reference is valid.
    pub struct AssetRevisionSelectionValid;
    structural_prop!(AssetRevisionSelectionValid);

    /// Asset integrity selection for one asset reference is valid.
    pub struct AssetIntegritySelectionValid;
    structural_prop!(AssetIntegritySelectionValid);

    /// Asset logical-size selection for one asset reference is valid.
    pub struct AssetLogicalSizeSelectionValid;
    structural_prop!(AssetLogicalSizeSelectionValid);

    /// One material texture binding is valid for downstream rendering.
    pub struct RenderMaterialTextureBindingValid;
    structural_prop!(RenderMaterialTextureBindingValid);

    /// One material intent descriptor is valid for downstream rendering.
    pub struct RenderMaterialIntentValid;
    structural_prop!(RenderMaterialIntentValid);

    /// Material alpha-mode selection for one material intent is valid.
    pub struct MaterialAlphaModeSelectionValid;
    structural_prop!(MaterialAlphaModeSelectionValid);

    /// Material cull-mode selection for one material intent is valid.
    pub struct MaterialCullModeSelectionValid;
    structural_prop!(MaterialCullModeSelectionValid);

    /// Material unlit selection for one material intent is valid.
    pub struct MaterialUnlitSelectionValid;
    structural_prop!(MaterialUnlitSelectionValid);

    /// Material depth-write selection for one material intent is valid.
    pub struct MaterialDepthWriteSelectionValid;
    structural_prop!(MaterialDepthWriteSelectionValid);

    /// Material depth-bias selection for one material intent is valid.
    pub struct MaterialDepthBiasSelectionValid;
    structural_prop!(MaterialDepthBiasSelectionValid);

    /// Material specialization-key selection for one material intent is valid.
    pub struct MaterialSpecializationKeySelectionValid;
    structural_prop!(MaterialSpecializationKeySelectionValid);

    /// Material pipeline-intent selection for one material intent is valid.
    pub struct RenderPipelineIntentSelectionValid;
    structural_prop!(RenderPipelineIntentSelectionValid);

    /// One material pipeline-intent descriptor is valid for downstream rendering.
    pub struct RenderPipelineIntentValid;
    structural_prop!(RenderPipelineIntentValid);

    /// Material opaque-method selection for one pipeline intent is valid.
    pub struct MaterialOpaqueMethodSelectionValid;
    structural_prop!(MaterialOpaqueMethodSelectionValid);

    /// One render output target descriptor is valid for downstream rendering.
    pub struct RenderOutputTargetValid;
    structural_prop!(RenderOutputTargetValid);

    /// Render output-target identifier selection for one target is valid.
    pub struct RenderOutputTargetIdSelectionValid;
    structural_prop!(RenderOutputTargetIdSelectionValid);

    /// Render output-target asset selection for one target is valid.
    pub struct RenderOutputTargetAssetSelectionValid;
    structural_prop!(RenderOutputTargetAssetSelectionValid);

    /// Output-target policy for one render view-output descriptor is valid.
    pub struct RenderOutputTargetPolicyValid;
    structural_prop!(RenderOutputTargetPolicyValid);

    /// One render clear-policy descriptor is valid for downstream rendering.
    pub struct RenderClearPolicyValid;
    structural_prop!(RenderClearPolicyValid);

    /// Clear-policy selection for one render view-output descriptor is valid.
    pub struct RenderClearPolicySelectionValid;
    structural_prop!(RenderClearPolicySelectionValid);

    /// One render viewport descriptor is valid for downstream rendering.
    pub struct RenderViewportValid;
    structural_prop!(RenderViewportValid);

    /// Stroke selection for one feature symbolizer is valid.
    pub struct RenderStrokeSelectionValid;
    structural_prop!(RenderStrokeSelectionValid);

    /// Fill selection for one feature symbolizer is valid.
    pub struct RenderFillSelectionValid;
    structural_prop!(RenderFillSelectionValid);

    /// Point-symbol selection for one feature symbolizer is valid.
    pub struct RenderPointSymbolSelectionValid;
    structural_prop!(RenderPointSymbolSelectionValid);

    /// Label selection for one feature symbolizer is valid.
    pub struct RenderLabelSelectionValid;
    structural_prop!(RenderLabelSelectionValid);

    /// Placement selection for one feature symbolizer is valid.
    pub struct RenderFeaturePlacementSelectionValid;
    structural_prop!(RenderFeaturePlacementSelectionValid);

    /// Extrusion selection for one feature symbolizer is valid.
    pub struct RenderExtrusionSelectionValid;
    structural_prop!(RenderExtrusionSelectionValid);

    /// Shadow-participation selection for one feature symbolizer is valid.
    pub struct RenderFeatureShadowParticipationSelectionValid;
    structural_prop!(RenderFeatureShadowParticipationSelectionValid);

    /// Material-intent selection for one feature symbolizer is valid.
    pub struct RenderFeatureMaterialIntentSelectionValid;
    structural_prop!(RenderFeatureMaterialIntentSelectionValid);

    /// Scale-range selection for one rule or annotation is valid.
    pub struct RenderScaleRangeSelectionValid;
    structural_prop!(RenderScaleRangeSelectionValid);

    /// Feature-batch limit selection for one batching descriptor is valid.
    pub struct FeatureBatchLimitSelectionValid;
    structural_prop!(FeatureBatchLimitSelectionValid);

    /// Feature-batching selection for one vector layer is valid.
    pub struct RenderFeatureBatchingSelectionValid;
    structural_prop!(RenderFeatureBatchingSelectionValid);

    /// Viewport policy for one render view-output descriptor is valid.
    pub struct RenderViewportPolicyValid;
    structural_prop!(RenderViewportPolicyValid);

    /// One render resolution-override descriptor is valid for downstream rendering.
    pub struct RenderResolutionOverrideValid;
    structural_prop!(RenderResolutionOverrideValid);

    /// Resolution-override policy for one render view-output descriptor is valid.
    pub struct RenderResolutionOverridePolicyValid;
    structural_prop!(RenderResolutionOverridePolicyValid);

    /// One render subview-layout descriptor is valid for downstream rendering.
    pub struct RenderSubViewLayoutValid;
    structural_prop!(RenderSubViewLayoutValid);

    /// Subview-layout policy for one render view-output descriptor is valid.
    pub struct RenderSubViewLayoutPolicyValid;
    structural_prop!(RenderSubViewLayoutPolicyValid);

    /// Tonemapping selection for one render view-output descriptor is valid.
    pub struct TonemappingPolicyValid;
    structural_prop!(TonemappingPolicyValid);

    /// MSAA selection for one render view-output descriptor is valid.
    pub struct MsaaPolicyValid;
    structural_prop!(MsaaPolicyValid);

    /// Mip-bias selection for one render view-output descriptor is valid.
    pub struct MipBiasPolicyValid;
    structural_prop!(MipBiasPolicyValid);

    /// Fixed-exposure selection for one render view-output descriptor is valid.
    pub struct RenderExposureSelectionValid;
    structural_prop!(RenderExposureSelectionValid);

    /// Color-grading selection for one render view-output descriptor is valid.
    pub struct RenderColorGradingSelectionValid;
    structural_prop!(RenderColorGradingSelectionValid);

    /// Auto-exposure selection for one render view-output descriptor is valid.
    pub struct RenderAutoExposureSelectionValid;
    structural_prop!(RenderAutoExposureSelectionValid);

    /// Bloom selection for one render view-output descriptor is valid.
    pub struct RenderBloomSelectionValid;
    structural_prop!(RenderBloomSelectionValid);

    /// Depth-of-field selection for one render view-output descriptor is valid.
    pub struct RenderDepthOfFieldSelectionValid;
    structural_prop!(RenderDepthOfFieldSelectionValid);

    /// Motion-blur selection for one render view-output descriptor is valid.
    pub struct RenderMotionBlurSelectionValid;
    structural_prop!(RenderMotionBlurSelectionValid);

    /// Chromatic-aberration selection for one render view-output descriptor is valid.
    pub struct RenderChromaticAberrationSelectionValid;
    structural_prop!(RenderChromaticAberrationSelectionValid);

    /// Screen-space reflections selection for one render view-output descriptor is valid.
    pub struct RenderScreenSpaceReflectionsSelectionValid;
    structural_prop!(RenderScreenSpaceReflectionsSelectionValid);

    /// Screen-space ambient-occlusion selection for one render view-output descriptor is valid.
    pub struct RenderScreenSpaceAmbientOcclusionSelectionValid;
    structural_prop!(RenderScreenSpaceAmbientOcclusionSelectionValid);

    /// Shadow-filtering-method selection for one render view-output descriptor is valid.
    pub struct ShadowFilteringMethodSelectionValid;
    structural_prop!(ShadowFilteringMethodSelectionValid);

    /// Screen-space transmission-quality selection for one render view-output descriptor is valid.
    pub struct ScreenSpaceTransmissionQualitySelectionValid;
    structural_prop!(ScreenSpaceTransmissionQualitySelectionValid);

    /// One layer view-routing descriptor is valid for downstream rendering.
    pub struct RenderViewParticipationValid;
    structural_prop!(RenderViewParticipationValid);

    /// Perspective-projection policy for one render view descriptor is valid.
    pub struct PerspectiveProjectionPolicyValid;
    structural_prop!(PerspectiveProjectionPolicyValid);

    /// View-output selection for one render view descriptor is valid.
    pub struct RenderViewOutputSelectionValid;
    structural_prop!(RenderViewOutputSelectionValid);

    /// A world-space policy is valid for downstream rendering.
    pub struct RenderWorldSpaceValid;
    structural_prop!(RenderWorldSpaceValid);

    /// World-space selection for one render view descriptor is valid.
    pub struct RenderWorldSpaceSelectionValid;
    structural_prop!(RenderWorldSpaceSelectionValid);

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

    /// Skybox selection for one scene-environment descriptor is valid.
    pub struct RenderSkyboxSelectionValid;
    structural_prop!(RenderSkyboxSelectionValid);

    /// Ambient-light selection for one scene-environment descriptor is valid.
    pub struct RenderAmbientLightSelectionValid;
    structural_prop!(RenderAmbientLightSelectionValid);

    /// Directional-light selection for one scene-environment descriptor is valid.
    pub struct RenderDirectionalLightSelectionValid;
    structural_prop!(RenderDirectionalLightSelectionValid);

    /// Image-based-lighting selection for one scene-environment descriptor is valid.
    pub struct RenderImageBasedLightingSelectionValid;
    structural_prop!(RenderImageBasedLightingSelectionValid);

    /// Atmosphere selection for one scene-environment descriptor is valid.
    pub struct RenderAtmosphereSelectionValid;
    structural_prop!(RenderAtmosphereSelectionValid);

    /// Fog selection for one scene-environment descriptor is valid.
    pub struct RenderFogSelectionValid;
    structural_prop!(RenderFogSelectionValid);

    /// Volumetric-fog selection for one scene-environment descriptor is valid.
    pub struct RenderVolumetricFogSelectionValid;
    structural_prop!(RenderVolumetricFogSelectionValid);

    /// Scene-environment selection for one render scene descriptor is valid.
    pub struct RenderSceneEnvironmentSelectionValid;
    structural_prop!(RenderSceneEnvironmentSelectionValid);

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

    /// Sun-disk selection for one atmosphere descriptor is valid.
    pub struct RenderSunDiskSelectionValid;
    structural_prop!(RenderSunDiskSelectionValid);

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

    /// Scene-update target-layer selection is explicitly established.
    pub struct SceneUpdateTargetLayerSelectionDeclared;
    structural_prop!(SceneUpdateTargetLayerSelectionDeclared);

    /// Scene-update target-view selection is explicitly established.
    pub struct SceneUpdateTargetViewSelectionDeclared;
    structural_prop!(SceneUpdateTargetViewSelectionDeclared);

    /// Scene-update reference-layer selection is explicitly established.
    pub struct SceneUpdateReferenceLayerSelectionDeclared;
    structural_prop!(SceneUpdateReferenceLayerSelectionDeclared);

    /// Scene-update view payload selection is explicitly established.
    pub struct SceneUpdateViewSelectionDeclared;
    structural_prop!(SceneUpdateViewSelectionDeclared);

    /// Scene-update environment payload selection is explicitly established.
    pub struct SceneUpdateEnvironmentSelectionDeclared;
    structural_prop!(SceneUpdateEnvironmentSelectionDeclared);

    /// Scene-update layer payload selection is explicitly established.
    pub struct SceneUpdateLayerSelectionDeclared;
    structural_prop!(SceneUpdateLayerSelectionDeclared);

    /// Scene-update opacity payload selection is explicitly established.
    pub struct SceneUpdateOpacitySelectionDeclared;
    structural_prop!(SceneUpdateOpacitySelectionDeclared);

    /// Scene-update draw-order payload selection is explicitly established.
    pub struct SceneUpdateDrawOrderSelectionDeclared;
    structural_prop!(SceneUpdateDrawOrderSelectionDeclared);

    /// A render scene update is valid and ready for downstream consumption.
    pub struct RenderableSceneUpdateValid;
    structural_prop!(RenderableSceneUpdateValid);

    /// One layer-routing scene update is valid and ready for downstream consumption.
    pub struct RenderableSceneLayerViewParticipationUpdateValid;
    structural_prop!(RenderableSceneLayerViewParticipationUpdateValid);

    /// Scene-update target-layer selection is valid.
    pub struct SceneUpdateTargetLayerSelectionValid;
    structural_prop!(SceneUpdateTargetLayerSelectionValid);

    /// Scene-update target-view selection is valid.
    pub struct SceneUpdateTargetViewSelectionValid;
    structural_prop!(SceneUpdateTargetViewSelectionValid);

    /// Scene-update reference-layer selection is valid.
    pub struct SceneUpdateReferenceLayerSelectionValid;
    structural_prop!(SceneUpdateReferenceLayerSelectionValid);

    /// Scene-update view payload selection is valid.
    pub struct SceneUpdateViewSelectionValid;
    structural_prop!(SceneUpdateViewSelectionValid);

    /// Scene-update environment payload selection is valid.
    pub struct SceneUpdateEnvironmentSelectionValid;
    structural_prop!(SceneUpdateEnvironmentSelectionValid);

    /// Scene-update layer payload selection is valid.
    pub struct SceneUpdateLayerSelectionValid;
    structural_prop!(SceneUpdateLayerSelectionValid);

    /// Scene-update opacity payload selection is valid.
    pub struct SceneUpdateOpacitySelectionValid;
    structural_prop!(SceneUpdateOpacitySelectionValid);

    /// Scene-update draw-order payload selection is valid.
    pub struct SceneUpdateDrawOrderSelectionValid;
    structural_prop!(SceneUpdateDrawOrderSelectionValid);

    /// Clear depth value selection is explicitly established for one clear-policy descriptor.
    pub struct ClearDepthSelectionDeclared;
    structural_prop!(ClearDepthSelectionDeclared);

    /// Clear depth selection for one clear-policy descriptor is valid.
    pub struct ClearDepthSelectionValid;
    structural_prop!(ClearDepthSelectionValid);

    /// Viewport depth-range selection is explicitly established for one viewport descriptor.
    pub struct RenderViewportDepthRangeSelectionDeclared;
    structural_prop!(RenderViewportDepthRangeSelectionDeclared);

    /// Viewport depth-range selection for one viewport descriptor is valid.
    pub struct RenderViewportDepthRangeSelectionValid;
    structural_prop!(RenderViewportDepthRangeSelectionValid);

    /// Tile cache-budget selection is explicitly established for one streaming descriptor.
    pub struct TileCacheBudgetSelectionDeclared;
    structural_prop!(TileCacheBudgetSelectionDeclared);

    /// Tile cache-budget selection for one streaming descriptor is valid.
    pub struct TileCacheBudgetSelectionValid;
    structural_prop!(TileCacheBudgetSelectionValid);

    /// Tile retry-backoff selection is explicitly established for one streaming descriptor.
    pub struct TileRetryBackoffSelectionDeclared;
    structural_prop!(TileRetryBackoffSelectionDeclared);

    /// Tile retry-backoff selection for one streaming descriptor is valid.
    pub struct TileRetryBackoffSelectionValid;
    structural_prop!(TileRetryBackoffSelectionValid);

    /// Tile transition-fade selection is explicitly established for one streaming descriptor.
    pub struct TileTransitionFadeSelectionDeclared;
    structural_prop!(TileTransitionFadeSelectionDeclared);

    /// Tile transition-fade selection for one streaming descriptor is valid.
    pub struct TileTransitionFadeSelectionValid;
    structural_prop!(TileTransitionFadeSelectionValid);

    /// Raster color-ramp domain selection is explicitly established for one raster-style descriptor.
    pub struct RasterColorRampDomainSelectionDeclared;
    structural_prop!(RasterColorRampDomainSelectionDeclared);

    /// Raster color-ramp domain selection for one raster-style descriptor is valid.
    pub struct RasterColorRampDomainSelectionValid;
    structural_prop!(RasterColorRampDomainSelectionValid);

    /// Raster derived-product selection is explicitly established for one raster-style descriptor.
    pub struct RasterDerivedProductSelectionDeclared;
    structural_prop!(RasterDerivedProductSelectionDeclared);

    /// Raster derived-product selection for one raster-style descriptor is valid.
    pub struct RasterDerivedProductSelectionValid;
    structural_prop!(RasterDerivedProductSelectionValid);

    /// Raster contour-interval selection is explicitly established for one raster-style descriptor.
    pub struct RasterContourIntervalSelectionDeclared;
    structural_prop!(RasterContourIntervalSelectionDeclared);

    /// Raster contour-interval selection for one raster-style descriptor is valid.
    pub struct RasterContourIntervalSelectionValid;
    structural_prop!(RasterContourIntervalSelectionValid);

    /// Raster normal-strength selection is explicitly established for one raster-style descriptor.
    pub struct RasterNormalStrengthSelectionDeclared;
    structural_prop!(RasterNormalStrengthSelectionDeclared);

    /// Raster normal-strength selection for one raster-style descriptor is valid.
    pub struct RasterNormalStrengthSelectionValid;
    structural_prop!(RasterNormalStrengthSelectionValid);

    /// Raster-style material-intent selection is explicitly established for one raster-style descriptor.
    pub struct RenderRasterStyleMaterialIntentSelectionDeclared;
    structural_prop!(RenderRasterStyleMaterialIntentSelectionDeclared);

    /// Raster-style material-intent selection for one raster-style descriptor is valid.
    pub struct RenderRasterStyleMaterialIntentSelectionValid;
    structural_prop!(RenderRasterStyleMaterialIntentSelectionValid);

    /// Tile-source streaming selection is explicitly established for one tile-source descriptor.
    pub struct TileSourceStreamingSelectionDeclared;
    structural_prop!(TileSourceStreamingSelectionDeclared);

    /// Tile-source streaming selection for one tile-source descriptor is valid.
    pub struct TileSourceStreamingSelectionValid;
    structural_prop!(TileSourceStreamingSelectionValid);

    /// Tile-style raster selection is explicitly established for one tile-style descriptor.
    pub struct RenderTileStyleRasterSelectionDeclared;
    structural_prop!(RenderTileStyleRasterSelectionDeclared);

    /// Tile-style raster selection for one tile-style descriptor is valid.
    pub struct RenderTileStyleRasterSelectionValid;
    structural_prop!(RenderTileStyleRasterSelectionValid);

    /// Tile-style vector selection is explicitly established for one tile-style descriptor.
    pub struct RenderTileStyleVectorSelectionDeclared;
    structural_prop!(RenderTileStyleVectorSelectionDeclared);

    /// Tile-style vector selection for one tile-style descriptor is valid.
    pub struct RenderTileStyleVectorSelectionValid;
    structural_prop!(RenderTileStyleVectorSelectionValid);

    /// Tile-style material-intent selection is explicitly established for one tile-style descriptor.
    pub struct RenderTileStyleMaterialIntentSelectionDeclared;
    structural_prop!(RenderTileStyleMaterialIntentSelectionDeclared);

    /// Tile-style material-intent selection for one tile-style descriptor is valid.
    pub struct RenderTileStyleMaterialIntentSelectionValid;
    structural_prop!(RenderTileStyleMaterialIntentSelectionValid);

    /// Terrain skirt-height selection is explicitly established for one terrain-LOD descriptor.
    pub struct TerrainSkirtHeightSelectionDeclared;
    structural_prop!(TerrainSkirtHeightSelectionDeclared);

    /// Terrain skirt-height selection for one terrain-LOD descriptor is valid.
    pub struct TerrainSkirtHeightSelectionValid;
    structural_prop!(TerrainSkirtHeightSelectionValid);

    /// Terrain overlay texture selection is explicitly established for one terrain-overlay descriptor.
    pub struct TerrainOverlayTextureSelectionDeclared;
    structural_prop!(TerrainOverlayTextureSelectionDeclared);

    /// Terrain overlay texture selection for one terrain-overlay descriptor is valid.
    pub struct TerrainOverlayTextureSelectionValid;
    structural_prop!(TerrainOverlayTextureSelectionValid);

    /// Terrain overlay raster-style selection is explicitly established for one terrain-overlay descriptor.
    pub struct TerrainOverlayRasterStyleSelectionDeclared;
    structural_prop!(TerrainOverlayRasterStyleSelectionDeclared);

    /// Terrain overlay raster-style selection for one terrain-overlay descriptor is valid.
    pub struct TerrainOverlayRasterStyleSelectionValid;
    structural_prop!(TerrainOverlayRasterStyleSelectionValid);

    /// Terrain surface drape-mode selection is explicitly established for one terrain-style descriptor.
    pub struct TerrainSurfaceDrapeSelectionDeclared;
    structural_prop!(TerrainSurfaceDrapeSelectionDeclared);

    /// Terrain surface drape-mode selection for one terrain-style descriptor is valid.
    pub struct TerrainSurfaceDrapeSelectionValid;
    structural_prop!(TerrainSurfaceDrapeSelectionValid);

    /// Terrain surface texture selection is explicitly established for one terrain-style descriptor.
    pub struct TerrainSurfaceTextureSelectionDeclared;
    structural_prop!(TerrainSurfaceTextureSelectionDeclared);

    /// Terrain surface texture selection for one terrain-style descriptor is valid.
    pub struct TerrainSurfaceTextureSelectionValid;
    structural_prop!(TerrainSurfaceTextureSelectionValid);

    /// Terrain surface raster-style selection is explicitly established for one terrain-style descriptor.
    pub struct TerrainSurfaceRasterStyleSelectionDeclared;
    structural_prop!(TerrainSurfaceRasterStyleSelectionDeclared);

    /// Terrain surface raster-style selection for one terrain-style descriptor is valid.
    pub struct TerrainSurfaceRasterStyleSelectionValid;
    structural_prop!(TerrainSurfaceRasterStyleSelectionValid);

    /// Terrain LOD selection is explicitly established for one terrain-style descriptor.
    pub struct TerrainLodSelectionDeclared;
    structural_prop!(TerrainLodSelectionDeclared);

    /// Terrain LOD selection for one terrain-style descriptor is valid.
    pub struct TerrainLodSelectionValid;
    structural_prop!(TerrainLodSelectionValid);

    /// Terrain shadow-participation selection is explicitly established for one terrain-style descriptor.
    pub struct TerrainShadowParticipationSelectionDeclared;
    structural_prop!(TerrainShadowParticipationSelectionDeclared);

    /// Terrain shadow-participation selection for one terrain-style descriptor is valid.
    pub struct TerrainShadowParticipationSelectionValid;
    structural_prop!(TerrainShadowParticipationSelectionValid);

    /// Terrain material-intent selection is explicitly established for one terrain-style descriptor.
    pub struct TerrainMaterialIntentSelectionDeclared;
    structural_prop!(TerrainMaterialIntentSelectionDeclared);

    /// Terrain material-intent selection for one terrain-style descriptor is valid.
    pub struct TerrainMaterialIntentSelectionValid;
    structural_prop!(TerrainMaterialIntentSelectionValid);

    /// Exposure EV100 selection is explicitly established for one exposure descriptor.
    pub struct ExposureEv100SelectionDeclared;
    structural_prop!(ExposureEv100SelectionDeclared);

    /// Exposure EV100 selection for one exposure descriptor is valid.
    pub struct ExposureEv100SelectionValid;
    structural_prop!(ExposureEv100SelectionValid);

    /// Auto-exposure metering-mask selection is explicitly established for one auto-exposure descriptor.
    pub struct AutoExposureMeteringMaskSelectionDeclared;
    structural_prop!(AutoExposureMeteringMaskSelectionDeclared);

    /// Auto-exposure metering-mask selection for one auto-exposure descriptor is valid.
    pub struct AutoExposureMeteringMaskSelectionValid;
    structural_prop!(AutoExposureMeteringMaskSelectionValid);

    /// Auto-exposure compensation-curve selection is explicitly established for one auto-exposure descriptor.
    pub struct AutoExposureCompensationCurveSelectionDeclared;
    structural_prop!(AutoExposureCompensationCurveSelectionDeclared);

    /// Auto-exposure compensation-curve selection for one auto-exposure descriptor is valid.
    pub struct AutoExposureCompensationCurveSelectionValid;
    structural_prop!(AutoExposureCompensationCurveSelectionValid);

    /// Chromatic-aberration color-LUT selection is explicitly established for one CA descriptor.
    pub struct ChromaticAberrationColorLutSelectionDeclared;
    structural_prop!(ChromaticAberrationColorLutSelectionDeclared);

    /// Chromatic-aberration color-LUT selection for one CA descriptor is valid.
    pub struct ChromaticAberrationColorLutSelectionValid;
    structural_prop!(ChromaticAberrationColorLutSelectionValid);

    /// SSR roughness-threshold selection is explicitly established for one SSR descriptor.
    pub struct ScreenSpaceReflectionsRoughnessThresholdSelectionDeclared;
    structural_prop!(ScreenSpaceReflectionsRoughnessThresholdSelectionDeclared);

    /// SSR roughness-threshold selection for one SSR descriptor is valid.
    pub struct ScreenSpaceReflectionsRoughnessThresholdSelectionValid;
    structural_prop!(ScreenSpaceReflectionsRoughnessThresholdSelectionValid);

    /// SSR thickness selection is explicitly established for one SSR descriptor.
    pub struct ScreenSpaceReflectionsThicknessSelectionDeclared;
    structural_prop!(ScreenSpaceReflectionsThicknessSelectionDeclared);

    /// SSR thickness selection for one SSR descriptor is valid.
    pub struct ScreenSpaceReflectionsThicknessSelectionValid;
    structural_prop!(ScreenSpaceReflectionsThicknessSelectionValid);

    /// SSR linear-steps selection is explicitly established for one SSR descriptor.
    pub struct ScreenSpaceReflectionsLinearStepsSelectionDeclared;
    structural_prop!(ScreenSpaceReflectionsLinearStepsSelectionDeclared);

    /// SSR linear-steps selection for one SSR descriptor is valid.
    pub struct ScreenSpaceReflectionsLinearStepsSelectionValid;
    structural_prop!(ScreenSpaceReflectionsLinearStepsSelectionValid);

    /// SSR linear-march-exponent selection is explicitly established for one SSR descriptor.
    pub struct ScreenSpaceReflectionsLinearMarchExponentSelectionDeclared;
    structural_prop!(ScreenSpaceReflectionsLinearMarchExponentSelectionDeclared);

    /// SSR linear-march-exponent selection for one SSR descriptor is valid.
    pub struct ScreenSpaceReflectionsLinearMarchExponentSelectionValid;
    structural_prop!(ScreenSpaceReflectionsLinearMarchExponentSelectionValid);

    /// SSR bisection-steps selection is explicitly established for one SSR descriptor.
    pub struct ScreenSpaceReflectionsBisectionStepsSelectionDeclared;
    structural_prop!(ScreenSpaceReflectionsBisectionStepsSelectionDeclared);

    /// SSR bisection-steps selection for one SSR descriptor is valid.
    pub struct ScreenSpaceReflectionsBisectionStepsSelectionValid;
    structural_prop!(ScreenSpaceReflectionsBisectionStepsSelectionValid);

    /// SSAO floating-point parameters selection is explicitly established for one SSAO descriptor.
    pub struct ScreenSpaceAmbientOcclusionParametersSelectionDeclared;
    structural_prop!(ScreenSpaceAmbientOcclusionParametersSelectionDeclared);

    /// SSAO floating-point parameters selection for one SSAO descriptor is valid.
    pub struct ScreenSpaceAmbientOcclusionParametersSelectionValid;
    structural_prop!(ScreenSpaceAmbientOcclusionParametersSelectionValid);

    /// SSAO thickness selection is explicitly established for one SSAO descriptor.
    pub struct ScreenSpaceAmbientOcclusionThicknessSelectionDeclared;
    structural_prop!(ScreenSpaceAmbientOcclusionThicknessSelectionDeclared);

    /// SSAO thickness selection for one SSAO descriptor is valid.
    pub struct ScreenSpaceAmbientOcclusionThicknessSelectionValid;
    structural_prop!(ScreenSpaceAmbientOcclusionThicknessSelectionValid);

    /// SSAO custom slice-count selection is explicitly established for one SSAO descriptor.
    pub struct ScreenSpaceAmbientOcclusionCustomSliceCountSelectionDeclared;
    structural_prop!(ScreenSpaceAmbientOcclusionCustomSliceCountSelectionDeclared);

    /// SSAO custom slice-count selection for one SSAO descriptor is valid.
    pub struct ScreenSpaceAmbientOcclusionCustomSliceCountSelectionValid;
    structural_prop!(ScreenSpaceAmbientOcclusionCustomSliceCountSelectionValid);

    /// SSAO custom samples-per-slice selection is explicitly established for one SSAO descriptor.
    pub struct ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionDeclared;
    structural_prop!(ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionDeclared);

    /// SSAO custom samples-per-slice selection for one SSAO descriptor is valid.
    pub struct ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionValid;
    structural_prop!(ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionValid);

    /// Annotation scale-range selection is explicitly established for one annotation descriptor.
    pub struct AnnotationScaleRangeSelectionDeclared;
    structural_prop!(AnnotationScaleRangeSelectionDeclared);

    /// Annotation scale-range selection for one annotation descriptor is valid.
    pub struct AnnotationScaleRangeSelectionValid;
    structural_prop!(AnnotationScaleRangeSelectionValid);

    /// Fog range selection is explicitly established for one fog descriptor.
    pub struct FogRangeSelectionDeclared;
    structural_prop!(FogRangeSelectionDeclared);

    /// Fog range selection for one fog descriptor is valid.
    pub struct FogRangeSelectionValid;
    structural_prop!(FogRangeSelectionValid);

    /// Fog directional-light exponent selection is explicitly established for one fog descriptor.
    pub struct FogDirectionalLightExponentSelectionDeclared;
    structural_prop!(FogDirectionalLightExponentSelectionDeclared);

    /// Fog directional-light exponent selection for one fog descriptor is valid.
    pub struct FogDirectionalLightExponentSelectionValid;
    structural_prop!(FogDirectionalLightExponentSelectionValid);

    /// Fog density selection is explicitly established for one fog descriptor.
    pub struct FogDensitySelectionDeclared;
    structural_prop!(FogDensitySelectionDeclared);

    /// Fog density selection for one fog descriptor is valid.
    pub struct FogDensitySelectionValid;
    structural_prop!(FogDensitySelectionValid);

    /// Atmospheric fog coefficients selection is explicitly established for one fog descriptor.
    pub struct AtmosphericFogCoefficientsSelectionDeclared;
    structural_prop!(AtmosphericFogCoefficientsSelectionDeclared);

    /// Atmospheric fog coefficients selection for one fog descriptor is valid.
    pub struct AtmosphericFogCoefficientsSelectionValid;
    structural_prop!(AtmosphericFogCoefficientsSelectionValid);

    /// Volumetric fog floating-point parameters selection is explicitly established.
    pub struct VolumetricFogParametersSelectionDeclared;
    structural_prop!(VolumetricFogParametersSelectionDeclared);

    /// Volumetric fog floating-point parameters selection is valid.
    pub struct VolumetricFogParametersSelectionValid;
    structural_prop!(VolumetricFogParametersSelectionValid);

    /// Volumetric fog step-count selection is explicitly established.
    pub struct VolumetricFogStepCountSelectionDeclared;
    structural_prop!(VolumetricFogStepCountSelectionDeclared);

    /// Volumetric fog step-count selection is valid.
    pub struct VolumetricFogStepCountSelectionValid;
    structural_prop!(VolumetricFogStepCountSelectionValid);

    /// Fog-volume floating-point parameters selection is explicitly established.
    pub struct FogVolumeParametersSelectionDeclared;
    structural_prop!(FogVolumeParametersSelectionDeclared);

    /// Fog-volume floating-point parameters selection is valid.
    pub struct FogVolumeParametersSelectionValid;
    structural_prop!(FogVolumeParametersSelectionValid);

    /// Fog-volume density-factor selection is explicitly established.
    pub struct FogVolumeDensityFactorSelectionDeclared;
    structural_prop!(FogVolumeDensityFactorSelectionDeclared);

    /// Fog-volume density-factor selection is valid.
    pub struct FogVolumeDensityFactorSelectionValid;
    structural_prop!(FogVolumeDensityFactorSelectionValid);

    /// Fog-volume density-texture offset selection is explicitly established.
    pub struct FogVolumeDensityTextureOffsetSelectionDeclared;
    structural_prop!(FogVolumeDensityTextureOffsetSelectionDeclared);

    /// Fog-volume density-texture offset selection is valid.
    pub struct FogVolumeDensityTextureOffsetSelectionValid;
    structural_prop!(FogVolumeDensityTextureOffsetSelectionValid);

    /// Fog-volume absorption selection is explicitly established.
    pub struct FogVolumeAbsorptionSelectionDeclared;
    structural_prop!(FogVolumeAbsorptionSelectionDeclared);

    /// Fog-volume absorption selection is valid.
    pub struct FogVolumeAbsorptionSelectionValid;
    structural_prop!(FogVolumeAbsorptionSelectionValid);

    /// Fog-volume scattering selection is explicitly established.
    pub struct FogVolumeScatteringSelectionDeclared;
    structural_prop!(FogVolumeScatteringSelectionDeclared);

    /// Fog-volume scattering selection is valid.
    pub struct FogVolumeScatteringSelectionValid;
    structural_prop!(FogVolumeScatteringSelectionValid);

    /// Fog-volume scattering asymmetry selection is explicitly established.
    pub struct FogVolumeScatteringAsymmetrySelectionDeclared;
    structural_prop!(FogVolumeScatteringAsymmetrySelectionDeclared);

    /// Fog-volume scattering asymmetry selection is valid.
    pub struct FogVolumeScatteringAsymmetrySelectionValid;
    structural_prop!(FogVolumeScatteringAsymmetrySelectionValid);

    /// Fog-volume light-intensity selection is explicitly established.
    pub struct FogVolumeLightIntensitySelectionDeclared;
    structural_prop!(FogVolumeLightIntensitySelectionDeclared);

    /// Fog-volume light-intensity selection is valid.
    pub struct FogVolumeLightIntensitySelectionValid;
    structural_prop!(FogVolumeLightIntensitySelectionValid);

    /// Fog-volume density-texture asset selection is explicitly established.
    pub struct FogVolumeDensityTextureSelectionDeclared;
    structural_prop!(FogVolumeDensityTextureSelectionDeclared);

    /// Fog-volume density-texture asset selection is valid.
    pub struct FogVolumeDensityTextureSelectionValid;
    structural_prop!(FogVolumeDensityTextureSelectionValid);

    /// Irradiance-volume intensity selection is explicitly established.
    pub struct IrradianceVolumeIntensitySelectionDeclared;
    structural_prop!(IrradianceVolumeIntensitySelectionDeclared);

    /// Irradiance-volume intensity selection is valid.
    pub struct IrradianceVolumeIntensitySelectionValid;
    structural_prop!(IrradianceVolumeIntensitySelectionValid);

    /// Light-probe irradiance-volume selection is explicitly established.
    pub struct RenderLightProbeIrradianceVolumeSelectionDeclared;
    structural_prop!(RenderLightProbeIrradianceVolumeSelectionDeclared);

    /// Light-probe irradiance-volume selection is valid.
    pub struct RenderLightProbeIrradianceVolumeSelectionValid;
    structural_prop!(RenderLightProbeIrradianceVolumeSelectionValid);

    /// IBL intensity selection is explicitly established.
    pub struct ImageBasedLightingIntensitySelectionDeclared;
    structural_prop!(ImageBasedLightingIntensitySelectionDeclared);

    /// IBL intensity selection is valid.
    pub struct ImageBasedLightingIntensitySelectionValid;
    structural_prop!(ImageBasedLightingIntensitySelectionValid);

    /// IBL rotation selection is explicitly established.
    pub struct ImageBasedLightingRotationSelectionDeclared;
    structural_prop!(ImageBasedLightingRotationSelectionDeclared);

    /// IBL rotation selection is valid.
    pub struct ImageBasedLightingRotationSelectionValid;
    structural_prop!(ImageBasedLightingRotationSelectionValid);

    /// IBL diffuse-map asset selection is explicitly established.
    pub struct ImageBasedLightingDiffuseMapSelectionDeclared;
    structural_prop!(ImageBasedLightingDiffuseMapSelectionDeclared);

    /// IBL diffuse-map asset selection is valid.
    pub struct ImageBasedLightingDiffuseMapSelectionValid;
    structural_prop!(ImageBasedLightingDiffuseMapSelectionValid);

    /// IBL specular-map asset selection is explicitly established.
    pub struct ImageBasedLightingSpecularMapSelectionDeclared;
    structural_prop!(ImageBasedLightingSpecularMapSelectionDeclared);

    /// IBL specular-map asset selection is valid.
    pub struct ImageBasedLightingSpecularMapSelectionValid;
    structural_prop!(ImageBasedLightingSpecularMapSelectionValid);

    /// IBL environment-map asset selection is explicitly established.
    pub struct ImageBasedLightingEnvironmentMapSelectionDeclared;
    structural_prop!(ImageBasedLightingEnvironmentMapSelectionDeclared);

    /// IBL environment-map asset selection is valid.
    pub struct ImageBasedLightingEnvironmentMapSelectionValid;
    structural_prop!(ImageBasedLightingEnvironmentMapSelectionValid);

    /// IBL atmosphere-cubemap dimensions selection is explicitly established.
    pub struct ImageBasedLightingAtmosphereCubemapDimensionsSelectionDeclared;
    structural_prop!(ImageBasedLightingAtmosphereCubemapDimensionsSelectionDeclared);

    /// IBL atmosphere-cubemap dimensions selection is valid.
    pub struct ImageBasedLightingAtmosphereCubemapDimensionsSelectionValid;
    structural_prop!(ImageBasedLightingAtmosphereCubemapDimensionsSelectionValid);

    /// Atmosphere falloff scale selection is explicitly established.
    pub struct AtmosphereFalloffScaleSelectionDeclared;
    structural_prop!(AtmosphereFalloffScaleSelectionDeclared);

    /// Atmosphere falloff scale selection is valid.
    pub struct AtmosphereFalloffScaleSelectionValid;
    structural_prop!(AtmosphereFalloffScaleSelectionValid);

    /// Atmosphere falloff center selection is explicitly established.
    pub struct AtmosphereFalloffCenterSelectionDeclared;
    structural_prop!(AtmosphereFalloffCenterSelectionDeclared);

    /// Atmosphere falloff center selection is valid.
    pub struct AtmosphereFalloffCenterSelectionValid;
    structural_prop!(AtmosphereFalloffCenterSelectionValid);

    /// Atmosphere falloff width selection is explicitly established.
    pub struct AtmosphereFalloffWidthSelectionDeclared;
    structural_prop!(AtmosphereFalloffWidthSelectionDeclared);

    /// Atmosphere falloff width selection is valid.
    pub struct AtmosphereFalloffWidthSelectionValid;
    structural_prop!(AtmosphereFalloffWidthSelectionValid);

    /// Atmosphere phase-function asymmetry selection is explicitly established.
    pub struct AtmospherePhaseFunctionAsymmetrySelectionDeclared;
    structural_prop!(AtmospherePhaseFunctionAsymmetrySelectionDeclared);

    /// Atmosphere phase-function asymmetry selection is valid.
    pub struct AtmospherePhaseFunctionAsymmetrySelectionValid;
    structural_prop!(AtmospherePhaseFunctionAsymmetrySelectionValid);

    /// Sun disk angular-size selection is explicitly established.
    pub struct SunDiskAngularSizeSelectionDeclared;
    structural_prop!(SunDiskAngularSizeSelectionDeclared);

    /// Sun disk angular-size selection is valid.
    pub struct SunDiskAngularSizeSelectionValid;
    structural_prop!(SunDiskAngularSizeSelectionValid);

    /// Sun disk intensity selection is explicitly established.
    pub struct SunDiskIntensitySelectionDeclared;
    structural_prop!(SunDiskIntensitySelectionDeclared);

    /// Sun disk intensity selection is valid.
    pub struct SunDiskIntensitySelectionValid;
    structural_prop!(SunDiskIntensitySelectionValid);

    /// Skybox rotation selection is explicitly established.
    pub struct SkyboxRotationSelectionDeclared;
    structural_prop!(SkyboxRotationSelectionDeclared);

    /// Skybox rotation selection is valid.
    pub struct SkyboxRotationSelectionValid;
    structural_prop!(SkyboxRotationSelectionValid);

    /// Cascade shadow bounds selection is explicitly established.
    pub struct CascadeShadowBoundsSelectionDeclared;
    structural_prop!(CascadeShadowBoundsSelectionDeclared);

    /// Cascade shadow bounds selection is valid.
    pub struct CascadeShadowBoundsSelectionValid;
    structural_prop!(CascadeShadowBoundsSelectionValid);

    /// Cascade shadow overlap selection is explicitly established.
    pub struct CascadeShadowOverlapSelectionDeclared;
    structural_prop!(CascadeShadowOverlapSelectionDeclared);

    /// Cascade shadow overlap selection is valid.
    pub struct CascadeShadowOverlapSelectionValid;
    structural_prop!(CascadeShadowOverlapSelectionValid);

    /// Cascade shadow minimum-distance selection is explicitly established.
    pub struct CascadeShadowMinimumDistanceSelectionDeclared;
    structural_prop!(CascadeShadowMinimumDistanceSelectionDeclared);

    /// Cascade shadow minimum-distance selection is valid.
    pub struct CascadeShadowMinimumDistanceSelectionValid;
    structural_prop!(CascadeShadowMinimumDistanceSelectionValid);

    /// Cascade shadow maximum-distance selection is explicitly established.
    pub struct CascadeShadowMaximumDistanceSelectionDeclared;
    structural_prop!(CascadeShadowMaximumDistanceSelectionDeclared);

    /// Cascade shadow maximum-distance selection is valid.
    pub struct CascadeShadowMaximumDistanceSelectionValid;
    structural_prop!(CascadeShadowMaximumDistanceSelectionValid);

    /// Cascade shadow first-cascade far-bound selection is explicitly established.
    pub struct CascadeShadowFirstFarBoundSelectionDeclared;
    structural_prop!(CascadeShadowFirstFarBoundSelectionDeclared);

    /// Cascade shadow first-cascade far-bound selection is valid.
    pub struct CascadeShadowFirstFarBoundSelectionValid;
    structural_prop!(CascadeShadowFirstFarBoundSelectionValid);

    /// Cascade shadow cascade-count selection is explicitly established.
    pub struct CascadeShadowCascadeCountSelectionDeclared;
    structural_prop!(CascadeShadowCascadeCountSelectionDeclared);

    /// Cascade shadow cascade-count selection is valid.
    pub struct CascadeShadowCascadeCountSelectionValid;
    structural_prop!(CascadeShadowCascadeCountSelectionValid);

    /// Directional-light cascade-shadows selection is explicitly established.
    pub struct DirectionalLightCascadeShadowsSelectionDeclared;
    structural_prop!(DirectionalLightCascadeShadowsSelectionDeclared);

    /// Directional-light cascade-shadows selection is valid.
    pub struct DirectionalLightCascadeShadowsSelectionValid;
    structural_prop!(DirectionalLightCascadeShadowsSelectionValid);

    /// Atmosphere density-multiplier selection is explicitly established.
    pub struct AtmosphereDensityMultiplierSelectionDeclared;
    structural_prop!(AtmosphereDensityMultiplierSelectionDeclared);

    /// Atmosphere density-multiplier selection is valid.
    pub struct AtmosphereDensityMultiplierSelectionValid;
    structural_prop!(AtmosphereDensityMultiplierSelectionValid);
}

pub use emit_impls::{
    AmbientLightBrightnessNonNegative, AnnotationAnchorFinite, AnnotationIdNonEmpty,
    AnnotationPayloadDeclared, AnnotationScaleRangeSelectionDeclared,
    AnnotationScaleRangeSelectionValid, AssetAccessPatternDeclared,
    AssetAccessPatternSelectionDeclared, AssetAccessPatternSelectionValid, AssetColorSpaceDeclared,
    AssetColorSpaceSelectionDeclared, AssetColorSpaceSelectionValid, AssetIntegrityDeclared,
    AssetIntegrityDigestNonEmpty, AssetIntegritySelectionDeclared, AssetIntegritySelectionValid,
    AssetLogicalHeightPositive, AssetLogicalSizeSelectionDeclared, AssetLogicalSizeSelectionValid,
    AssetLogicalWidthPositive, AssetResidencyPolicyDeclared, AssetResidencyPolicySelectionDeclared,
    AssetResidencyPolicySelectionValid, AssetRevisionDeclared, AssetRevisionNonEmpty,
    AssetRevisionSelectionDeclared, AssetRevisionSelectionValid, AssetUriDeclared,
    AtmosphereDensityMultiplierFinite, AtmosphereDensityMultiplierSelectionDeclared,
    AtmosphereDensityMultiplierSelectionValid, AtmosphereFalloffCenterSelectionDeclared,
    AtmosphereFalloffCenterSelectionValid, AtmosphereFalloffCenterUnitInterval,
    AtmosphereFalloffDeclared, AtmosphereFalloffScaleNonNegative,
    AtmosphereFalloffScaleSelectionDeclared, AtmosphereFalloffScaleSelectionValid,
    AtmosphereFalloffWidthSelectionDeclared, AtmosphereFalloffWidthSelectionValid,
    AtmosphereFalloffWidthUnitInterval, AtmosphereGroundAlbedoFinite,
    AtmosphereGroundAlbedoUnitInterval, AtmospherePhaseFunctionAsymmetryFinite,
    AtmospherePhaseFunctionAsymmetryNormalized, AtmospherePhaseFunctionAsymmetrySelectionDeclared,
    AtmospherePhaseFunctionAsymmetrySelectionValid, AtmospherePhaseFunctionDeclared,
    AtmosphereScatteringCoefficientsFinite, AtmosphereScatteringCoefficientsNonNegative,
    AtmosphereScatteringResolutionsPositive, AtmosphereScatteringTermsPresent,
    AtmosphereShellRadiiOrdered, AtmosphereShellRadiiPositive, AtmosphericFogCoefficientsFinite,
    AtmosphericFogCoefficientsSelectionDeclared, AtmosphericFogCoefficientsSelectionValid,
    AutoExposureCompensationCurveAssetValid, AutoExposureCompensationCurveSelectionDeclared,
    AutoExposureCompensationCurveSelectionValid, AutoExposureFilterOrdered,
    AutoExposureMeteringMaskAssetValid, AutoExposureMeteringMaskSelectionDeclared,
    AutoExposureMeteringMaskSelectionValid, AutoExposureRangeOrdered, AutoExposureSpeedNonNegative,
    AutoExposureTransitionDistanceNonNegative, BloomCompositeModeDeclared, BloomParametersFinite,
    BloomScaleFinite, BloomThresholdSoftnessUnitInterval, CameraClipRangeFinite,
    CameraClipRangeOrdered, CascadeShadowBoundsOrdered, CascadeShadowBoundsPositive,
    CascadeShadowBoundsSelectionDeclared, CascadeShadowBoundsSelectionValid,
    CascadeShadowCascadeCountPositive, CascadeShadowCascadeCountSelectionDeclared,
    CascadeShadowCascadeCountSelectionValid, CascadeShadowFirstCascadeFarBoundGreaterThanMinimum,
    CascadeShadowFirstCascadeFarBoundPositive, CascadeShadowFirstFarBoundSelectionDeclared,
    CascadeShadowFirstFarBoundSelectionValid, CascadeShadowMaximumDistanceOrdered,
    CascadeShadowMaximumDistanceSelectionDeclared, CascadeShadowMaximumDistanceSelectionValid,
    CascadeShadowMinimumDistanceNonNegative, CascadeShadowMinimumDistanceSelectionDeclared,
    CascadeShadowMinimumDistanceSelectionValid, CascadeShadowOverlapHalfOpenUnitInterval,
    CascadeShadowOverlapSelectionDeclared, CascadeShadowOverlapSelectionValid,
    ChromaticAberrationColorLutAssetValid, ChromaticAberrationColorLutSelectionDeclared,
    ChromaticAberrationColorLutSelectionValid, ChromaticAberrationParametersFinite,
    ChromaticAberrationSamplesPositive, ClearDepthSelectionDeclared, ClearDepthSelectionValid,
    ColorGradingMidtonesRangeOrdered, ColorGradingParametersFinite,
    DepthOfFieldApertureFStopsPositive, DepthOfFieldCircleOfConfusionNonNegative,
    DepthOfFieldFocalDistanceNonNegative, DepthOfFieldMaxDepthNonNegative,
    DepthOfFieldModeDeclared, DepthOfFieldParametersFinite, DepthOfFieldSensorHeightPositive,
    DirectionalLightCascadeShadowsSelectionDeclared, DirectionalLightCascadeShadowsSelectionValid,
    DirectionalLightIlluminanceNonNegative, ExposureEv100Finite, ExposureEv100SelectionDeclared,
    ExposureEv100SelectionValid, ExposureModeDeclared, ExposurePolicyExclusive,
    ExtrusionHeightDeclared, ExtrusionHeightNonNegative, FeatureBatchKeyPropertyNonEmpty,
    FeatureBatchLimitPositive, FeatureBatchLimitSelectionDeclared, FeatureBatchLimitSelectionValid,
    FeatureBatchingPolicyDeclared, FeaturePredicateDeclared, FillRuleDeclared,
    FogDensityNonNegative, FogDensitySelectionDeclared, FogDensitySelectionValid,
    FogDirectionalLightExponentNonNegative, FogDirectionalLightExponentSelectionDeclared,
    FogDirectionalLightExponentSelectionValid, FogFalloffDeclared, FogRangeFinite, FogRangeOrdered,
    FogRangeSelectionDeclared, FogRangeSelectionValid, FogVolumeAbsorptionNonNegative,
    FogVolumeAbsorptionSelectionDeclared, FogVolumeAbsorptionSelectionValid,
    FogVolumeDensityFactorNonNegative, FogVolumeDensityFactorSelectionDeclared,
    FogVolumeDensityFactorSelectionValid, FogVolumeDensityTextureAssetValid,
    FogVolumeDensityTextureOffsetFinite, FogVolumeDensityTextureOffsetSelectionDeclared,
    FogVolumeDensityTextureOffsetSelectionValid, FogVolumeDensityTextureSelectionDeclared,
    FogVolumeDensityTextureSelectionValid, FogVolumeLightIntensityNonNegative,
    FogVolumeLightIntensitySelectionDeclared, FogVolumeLightIntensitySelectionValid,
    FogVolumeParametersFinite, FogVolumeParametersSelectionDeclared,
    FogVolumeParametersSelectionValid, FogVolumeScatteringAsymmetryFinite,
    FogVolumeScatteringAsymmetrySelectionDeclared, FogVolumeScatteringAsymmetrySelectionValid,
    FogVolumeScatteringNonNegative, FogVolumeScatteringSelectionDeclared,
    FogVolumeScatteringSelectionValid, GeometryProjectedForView, HdrOutputDeclared,
    ImageBasedLightingAtmosphereCubemapDimensionsPositive,
    ImageBasedLightingAtmosphereCubemapDimensionsSelectionDeclared,
    ImageBasedLightingAtmosphereCubemapDimensionsSelectionValid,
    ImageBasedLightingDiffuseMapAssetValid, ImageBasedLightingDiffuseMapSelectionDeclared,
    ImageBasedLightingDiffuseMapSelectionValid, ImageBasedLightingEnvironmentMapAssetValid,
    ImageBasedLightingEnvironmentMapSelectionDeclared,
    ImageBasedLightingEnvironmentMapSelectionValid, ImageBasedLightingIntensityNonNegative,
    ImageBasedLightingIntensitySelectionDeclared, ImageBasedLightingIntensitySelectionValid,
    ImageBasedLightingRotationFinite, ImageBasedLightingRotationSelectionDeclared,
    ImageBasedLightingRotationSelectionValid, ImageBasedLightingSourceDeclared,
    ImageBasedLightingSpecularMapAssetValid, ImageBasedLightingSpecularMapSelectionDeclared,
    ImageBasedLightingSpecularMapSelectionValid, IrradianceVolumeIntensityNonNegative,
    IrradianceVolumeIntensitySelectionDeclared, IrradianceVolumeIntensitySelectionValid,
    IrradianceVolumeVoxelsAssetValid, LabelCollisionPolicyDeclared, LabelPriorityFinite,
    LabelTextSourceDeclared, LabelTransformFinite, LayerDrawOrderAssigned, LayerGroupIdNonEmpty,
    LayerGroupNameNonEmpty, LayerGroupOpacityUnitInterval, LayerIdNonEmpty, LayerNameNonEmpty,
    LayerOpacityUnitInterval, LayerOrderDeterministic, LayerVisibilityDeclared, LightAnglesFinite,
    MaterialAlphaModeDeclared, MaterialAlphaModeSelectionDeclared, MaterialAlphaModeSelectionValid,
    MaterialCullModeDeclared, MaterialCullModeSelectionDeclared, MaterialCullModeSelectionValid,
    MaterialDepthBiasFinite, MaterialDepthBiasSelectionDeclared, MaterialDepthBiasSelectionValid,
    MaterialDepthWriteSelectionDeclared, MaterialDepthWriteSelectionValid, MaterialFamilyDeclared,
    MaterialMaskThresholdUnitInterval, MaterialOpaqueMethodDeclared,
    MaterialOpaqueMethodDomainCompatible, MaterialOpaqueMethodSelectionDeclared,
    MaterialOpaqueMethodSelectionValid, MaterialPipelineDomainDeclared,
    MaterialPipelineFamilyCompatible, MaterialPrepassesDomainCompatible, MaterialPrepassesUnique,
    MaterialSpecializationKeyNonEmpty, MaterialSpecializationKeySelectionDeclared,
    MaterialSpecializationKeySelectionValid, MaterialTextureSemanticDeclared,
    MaterialUnlitSelectionDeclared, MaterialUnlitSelectionValid, MipBiasFinite,
    MipBiasPolicyDeclared, MipBiasPolicyValid, ModelAssetDeclared, ModelPlacementValid,
    MotionBlurParametersFinite, MotionBlurSamplesPositive, MsaaDeclared, MsaaPolicyDeclared,
    MsaaPolicyValid, NumericExpressionDeclared, NumericExpressionOperandsPresent,
    NumericExpressionParametersFinite, NumericInterpolationBasePositive,
    NumericInterpolationStopsOrdered, NumericStepStopsOrdered, PerspectiveFieldOfViewPositive,
    PerspectiveProjectionPolicyDeclared, PerspectiveProjectionPolicyValid,
    PlacementAltitudeModeDeclared, PlacementParametersFinite, PointSymbolSizePositive,
    PointSymbolSourceDeclared, PointSymbolTransformFinite, ProbeRegionCenterFinite,
    ProbeRegionExtentsPositive, RasterBandSelectionValid, RasterColorRampDomainFinite,
    RasterColorRampDomainOrdered, RasterColorRampDomainSelectionDeclared,
    RasterColorRampDomainSelectionValid, RasterContourIntervalPositive,
    RasterContourIntervalSelectionDeclared, RasterContourIntervalSelectionValid,
    RasterDerivedProductParametersFinite, RasterDerivedProductSelectionDeclared,
    RasterDerivedProductSelectionValid, RasterDimensionsPositive, RasterExtentFinite,
    RasterExtentOrdered, RasterGammaPositive, RasterImageMetadataDeclared,
    RasterNormalStrengthPositive, RasterNormalStrengthSelectionDeclared,
    RasterNormalStrengthSelectionValid, RasterNormalizeDomainOrdered, RasterOpacityUnitInterval,
    RasterSamplingDeclared, RasterSourceDeclared, RasterValueTransformFinite,
    RenderAmbientLightSelectionDeclared, RenderAmbientLightSelectionValid, RenderAmbientLightValid,
    RenderAnnotationValid, RenderAssetReferenceValid, RenderAtmosphereFalloffValid,
    RenderAtmospherePhaseFunctionValid, RenderAtmosphereScatteringTermValid,
    RenderAtmosphereSelectionDeclared, RenderAtmosphereSelectionValid, RenderAtmosphereValid,
    RenderAutoExposureSelectionDeclared, RenderAutoExposureSelectionValid, RenderAutoExposureValid,
    RenderBloomSelectionDeclared, RenderBloomSelectionValid, RenderBloomValid,
    RenderCascadeShadowConfigValid, RenderChromaticAberrationSelectionDeclared,
    RenderChromaticAberrationSelectionValid, RenderChromaticAberrationValid,
    RenderClearDepthFinite, RenderClearPolicyDeclared, RenderClearPolicySelectionDeclared,
    RenderClearPolicySelectionValid, RenderClearPolicyValid, RenderColorGradingSelectionDeclared,
    RenderColorGradingSelectionValid, RenderColorGradingValid, RenderDepthOfFieldSelectionDeclared,
    RenderDepthOfFieldSelectionValid, RenderDepthOfFieldValid,
    RenderDirectionalLightSelectionDeclared, RenderDirectionalLightSelectionValid,
    RenderDirectionalLightValid, RenderExposureSelectionDeclared, RenderExposureSelectionValid,
    RenderExposureValid, RenderExtrusionSelectionDeclared, RenderExtrusionSelectionValid,
    RenderFeatureBatchingSelectionDeclared, RenderFeatureBatchingSelectionValid,
    RenderFeatureBatchingValid, RenderFeatureMaterialIntentSelectionDeclared,
    RenderFeatureMaterialIntentSelectionValid, RenderFeaturePlacementSelectionDeclared,
    RenderFeaturePlacementSelectionValid, RenderFeatureShadowParticipationSelectionDeclared,
    RenderFeatureShadowParticipationSelectionValid, RenderFeatureStyleRuleValid,
    RenderFeatureStyleValid, RenderFeatureSymbolizerValid, RenderFillSelectionDeclared,
    RenderFillSelectionValid, RenderFogSelectionDeclared, RenderFogSelectionValid, RenderFogValid,
    RenderFogVolumeValid, RenderImageBasedLightingSelectionDeclared,
    RenderImageBasedLightingSelectionValid, RenderImageBasedLightingValid,
    RenderIrradianceVolumeValid, RenderLabelSelectionDeclared, RenderLabelSelectionValid,
    RenderLabelStyleValid, RenderLightProbeIrradianceVolumeSelectionDeclared,
    RenderLightProbeIrradianceVolumeSelectionValid, RenderLightProbeValid,
    RenderMaterialIntentValid, RenderMaterialTextureBindingValid,
    RenderMotionBlurSelectionDeclared, RenderMotionBlurSelectionValid, RenderMotionBlurValid,
    RenderOutputTargetAssetSelectionDeclared, RenderOutputTargetAssetSelectionValid,
    RenderOutputTargetDeclared, RenderOutputTargetIdNonEmpty,
    RenderOutputTargetIdSelectionDeclared, RenderOutputTargetIdSelectionValid,
    RenderOutputTargetPolicyDeclared, RenderOutputTargetPolicyValid, RenderOutputTargetValid,
    RenderPipelineIntentSelectionDeclared, RenderPipelineIntentSelectionValid,
    RenderPipelineIntentValid, RenderPointSymbolSelectionDeclared, RenderPointSymbolSelectionValid,
    RenderProbeRegionValid, RenderRasterSourceValid,
    RenderRasterStyleMaterialIntentSelectionDeclared,
    RenderRasterStyleMaterialIntentSelectionValid, RenderRasterStyleValid,
    RenderResolutionOverrideDeclared, RenderResolutionOverridePolicyDeclared,
    RenderResolutionOverridePolicyValid, RenderResolutionOverridePositive,
    RenderResolutionOverrideValid, RenderScaleRangeSelectionDeclared,
    RenderScaleRangeSelectionValid, RenderSceneEnvironmentSelectionValid,
    RenderSceneEnvironmentValid, RenderScreenSpaceAmbientOcclusionSelectionDeclared,
    RenderScreenSpaceAmbientOcclusionSelectionValid, RenderScreenSpaceAmbientOcclusionValid,
    RenderScreenSpaceReflectionsSelectionDeclared, RenderScreenSpaceReflectionsSelectionValid,
    RenderScreenSpaceReflectionsValid, RenderShadowParticipationValid,
    RenderSkyboxSelectionDeclared, RenderSkyboxSelectionValid, RenderSkyboxValid,
    RenderStrokeSelectionDeclared, RenderStrokeSelectionValid, RenderStrokeValid,
    RenderSubViewLayoutDeclared, RenderSubViewLayoutFullExtentPositive,
    RenderSubViewLayoutOffsetFinite, RenderSubViewLayoutPolicyDeclared,
    RenderSubViewLayoutPolicyValid, RenderSubViewLayoutSubviewExtentPositive,
    RenderSubViewLayoutValid, RenderSubViewLayoutWithinFullExtent, RenderSunDiskSelectionDeclared,
    RenderSunDiskSelectionValid, RenderSunDiskValid, RenderTerrainLodValid,
    RenderTerrainOverlayValid, RenderTerrainStyleValid, RenderTileSourceValid,
    RenderTileStreamingValid, RenderTileStyleMaterialIntentSelectionDeclared,
    RenderTileStyleMaterialIntentSelectionValid, RenderTileStyleRasterSelectionDeclared,
    RenderTileStyleRasterSelectionValid, RenderTileStyleValid,
    RenderTileStyleVectorSelectionDeclared, RenderTileStyleVectorSelectionValid,
    RenderViewCenterFinite, RenderViewCrsDeclared, RenderViewIdNonEmpty,
    RenderViewOrientationFinite, RenderViewOutputDeclared, RenderViewOutputSelectionDeclared,
    RenderViewOutputSelectionValid, RenderViewOutputValid, RenderViewParticipationValid,
    RenderViewProjectionDeclared, RenderViewResolutionPositive, RenderViewValid,
    RenderViewViewportPositive, RenderViewportDeclared, RenderViewportDepthRangeSelectionDeclared,
    RenderViewportDepthRangeSelectionValid, RenderViewportPolicyDeclared,
    RenderViewportPolicyValid, RenderViewportSizePositive, RenderViewportValid,
    RenderVolumetricFogSelectionDeclared, RenderVolumetricFogSelectionValid,
    RenderVolumetricFogValid, RenderWorldSpaceSelectionDeclared, RenderWorldSpaceSelectionValid,
    RenderWorldSpaceValid, RenderableAnnotationLayerValid, RenderableLayerGroupValid,
    RenderableLayerValid, RenderableModelLayerValid, RenderableRasterLayerValid,
    RenderableSceneLayerViewParticipationUpdateValid, RenderableSceneUpdateValid,
    RenderableSceneValid, RenderableTerrainLayerValid, RenderableTileLayerValid,
    RenderableVectorLayerValid, ScaleRangeFinite, ScaleRangeOrdered, SceneAuxiliaryViewIdsUnique,
    SceneAuxiliaryViewsDeclared, SceneAuxiliaryViewsDistinctFromPrimary, SceneIdNonEmpty,
    SceneLayerListDeclared, SceneLayerViewParticipationResolved,
    SceneUpdateDrawOrderSelectionDeclared, SceneUpdateDrawOrderSelectionValid,
    SceneUpdateEnvironmentSelectionDeclared, SceneUpdateEnvironmentSelectionValid,
    SceneUpdateLayerSelectionDeclared, SceneUpdateLayerSelectionValid,
    SceneUpdateOpacitySelectionDeclared, SceneUpdateOpacitySelectionValid,
    SceneUpdateOperationDeclared, SceneUpdateReferenceLayerNonEmpty,
    SceneUpdateReferenceLayerSelectionDeclared, SceneUpdateReferenceLayerSelectionValid,
    SceneUpdateTargetLayerNonEmpty, SceneUpdateTargetLayerSelectionDeclared,
    SceneUpdateTargetLayerSelectionValid, SceneUpdateTargetViewSelectionDeclared,
    SceneUpdateTargetViewSelectionValid, SceneUpdateViewSelectionDeclared,
    SceneUpdateViewSelectionValid, SceneViewDeclared,
    ScreenSpaceAmbientOcclusionCustomSamplesPerSlicePositive,
    ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionDeclared,
    ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionValid,
    ScreenSpaceAmbientOcclusionCustomSliceCountPositive,
    ScreenSpaceAmbientOcclusionCustomSliceCountSelectionDeclared,
    ScreenSpaceAmbientOcclusionCustomSliceCountSelectionValid,
    ScreenSpaceAmbientOcclusionParametersFinite,
    ScreenSpaceAmbientOcclusionParametersSelectionDeclared,
    ScreenSpaceAmbientOcclusionParametersSelectionValid,
    ScreenSpaceAmbientOcclusionQualityDeclared, ScreenSpaceAmbientOcclusionThicknessNonNegative,
    ScreenSpaceAmbientOcclusionThicknessSelectionDeclared,
    ScreenSpaceAmbientOcclusionThicknessSelectionValid,
    ScreenSpaceReflectionsBisectionStepsPositive,
    ScreenSpaceReflectionsBisectionStepsSelectionDeclared,
    ScreenSpaceReflectionsBisectionStepsSelectionValid,
    ScreenSpaceReflectionsLinearMarchExponentNonNegative,
    ScreenSpaceReflectionsLinearMarchExponentSelectionDeclared,
    ScreenSpaceReflectionsLinearMarchExponentSelectionValid,
    ScreenSpaceReflectionsLinearStepsPositive, ScreenSpaceReflectionsLinearStepsSelectionDeclared,
    ScreenSpaceReflectionsLinearStepsSelectionValid, ScreenSpaceReflectionsParametersFinite,
    ScreenSpaceReflectionsRoughnessThresholdSelectionDeclared,
    ScreenSpaceReflectionsRoughnessThresholdSelectionValid,
    ScreenSpaceReflectionsRoughnessThresholdUnitInterval,
    ScreenSpaceReflectionsThicknessNonNegative, ScreenSpaceReflectionsThicknessSelectionDeclared,
    ScreenSpaceReflectionsThicknessSelectionValid, ScreenSpaceTransmissionQualityDeclared,
    ScreenSpaceTransmissionQualitySelectionDeclared, ScreenSpaceTransmissionQualitySelectionValid,
    ShadowFilteringMethodDeclared, ShadowFilteringMethodSelectionDeclared,
    ShadowFilteringMethodSelectionValid, ShadowParticipationPolicyDeclared,
    SkyboxBrightnessNonNegative, SkyboxRotationFinite, SkyboxRotationSelectionDeclared,
    SkyboxRotationSelectionValid, StrokeParametersFinite, StrokeWidthFinite,
    StrokeWidthNonNegative, SunDiskAngularSizeNonNegative, SunDiskAngularSizeSelectionDeclared,
    SunDiskAngularSizeSelectionValid, SunDiskIntensityNonNegative,
    SunDiskIntensitySelectionDeclared, SunDiskIntensitySelectionValid,
    SymbolizerUsesRenderablePaint, TerrainHeightOffsetFinite, TerrainLodScreenSpaceErrorPositive,
    TerrainLodSelectionDeclared, TerrainLodSelectionValid, TerrainMaterialIntentSelectionDeclared,
    TerrainMaterialIntentSelectionValid, TerrainMeshModeDeclared, TerrainOverlayBlendModeDeclared,
    TerrainOverlayDrapeModeDeclared, TerrainOverlayHeightBiasFinite,
    TerrainOverlayOpacityUnitInterval, TerrainOverlayRasterStyleSelectionDeclared,
    TerrainOverlayRasterStyleSelectionValid, TerrainOverlayTextureSelectionDeclared,
    TerrainOverlayTextureSelectionValid, TerrainShadingModeDeclared,
    TerrainShadowParticipationSelectionDeclared, TerrainShadowParticipationSelectionValid,
    TerrainSkirtHeightNonNegative, TerrainSkirtHeightSelectionDeclared,
    TerrainSkirtHeightSelectionValid, TerrainSurfaceDrapeModeDeclared,
    TerrainSurfaceDrapeSelectionDeclared, TerrainSurfaceDrapeSelectionValid,
    TerrainSurfaceRasterStyleSelectionDeclared, TerrainSurfaceRasterStyleSelectionValid,
    TerrainSurfaceTextureSelectionDeclared, TerrainSurfaceTextureSelectionValid,
    TerrainVerticalScalePositive, TileCacheBudgetPositive, TileCacheBudgetSelectionDeclared,
    TileCacheBudgetSelectionValid, TileConcurrentRequestsPositive, TileDimensionsPositive,
    TileOpacityUnitInterval, TileRefinementPriorityDeclared, TileRetryBackoffPositive,
    TileRetryBackoffSelectionDeclared, TileRetryBackoffSelectionValid,
    TileSourceStreamingSelectionDeclared, TileSourceStreamingSelectionValid,
    TileStreamingPolicyDeclared, TileStylePayloadCompatible, TileTransitionFadePositive,
    TileTransitionFadeSelectionDeclared, TileTransitionFadeSelectionValid, TileUriTemplateDeclared,
    TileZoomRangeOrdered, TonemappingDeclared, TonemappingPolicyDeclared, TonemappingPolicyValid,
    VectorPayloadDeclared, VectorStylePayloadCompatible, VectorSymbolizationResolved,
    ViewParticipationListMatchesMode, ViewParticipationModeDeclared,
    ViewParticipationViewIdsNonEmpty, ViewParticipationViewIdsUnique,
    VolumetricFogParametersFinite, VolumetricFogParametersSelectionDeclared,
    VolumetricFogParametersSelectionValid, VolumetricFogStepCountPositive,
    VolumetricFogStepCountSelectionDeclared, VolumetricFogStepCountSelectionValid,
    WorldOriginFinite, WorldUnitsPerMeterPositive, WorldVerticalExaggerationPositive,
};

use crate::{
    RenderAmbientLightDescriptor, RenderAtmosphereDescriptor, RenderAutoExposureDescriptor,
    RenderBloomDescriptor, RenderCascadeShadowConfigDescriptor,
    RenderChromaticAberrationDescriptor, RenderClearPolicyDescriptor, RenderColorGradingDescriptor,
    RenderDepthOfFieldDescriptor, RenderDirectionalLightDescriptor, RenderExposureDescriptor,
    RenderFeatureStyleDescriptor, RenderFogDescriptor, RenderFogVolumeDescriptor,
    RenderImageBasedLightingDescriptor, RenderLightProbeDescriptor, RenderMaterialIntentDescriptor,
    RenderMotionBlurDescriptor, RenderOutputTargetDescriptor, RenderRasterStyleDescriptor,
    RenderResolutionOverrideDescriptor, RenderSceneEnvironmentDescriptor,
    RenderScreenSpaceAmbientOcclusionDescriptor, RenderScreenSpaceReflectionsDescriptor,
    RenderShadowParticipationDescriptor, RenderSkyboxDescriptor, RenderSubViewLayoutDescriptor,
    RenderTerrainStyleDescriptor, RenderTileStyleDescriptor, RenderViewDescriptor,
    RenderViewOutputDescriptor, RenderViewParticipationDescriptor, RenderViewportDescriptor,
    RenderVolumetricFogDescriptor, RenderWorldSpaceDescriptor,
};
use elicitation::{Established, contracts::ProvableFrom};

/// Evidence that an asset reference is renderable.
pub struct RenderAssetReferenceEvidence {
    /// Proof that the asset declares a stable URI.
    pub asset_uri_declared: Established<AssetUriDeclared>,
    /// Proof that color-space selection is valid for the full descriptor shape.
    pub asset_color_space_selection: Established<AssetColorSpaceSelectionValid>,
    /// Proof that residency selection is valid for the full descriptor shape.
    pub asset_residency_policy_selection: Established<AssetResidencyPolicySelectionValid>,
    /// Proof that access-pattern selection is valid for the full descriptor shape.
    pub asset_access_pattern_selection: Established<AssetAccessPatternSelectionValid>,
    /// Proof that revision selection is valid for the full descriptor shape.
    pub asset_revision_selection: Established<AssetRevisionSelectionValid>,
    /// Proof that integrity selection is valid for the full descriptor shape.
    pub asset_integrity_selection: Established<AssetIntegritySelectionValid>,
    /// Proof that logical-size selection is valid for the full descriptor shape.
    pub asset_logical_size_selection: Established<AssetLogicalSizeSelectionValid>,
}

impl ProvableFrom<RenderAssetReferenceEvidence> for RenderAssetReferenceValid {}

/// Evidence that one asset reference carries a valid color-space selection.
pub struct AssetColorSpaceSelectionEvidence {
    /// Proof that color-space selection has been explicitly established.
    pub asset_color_space_selection_declared: Established<AssetColorSpaceSelectionDeclared>,
}

impl ProvableFrom<AssetColorSpaceSelectionEvidence> for AssetColorSpaceSelectionValid {}

/// Evidence that one asset reference carries a valid residency selection.
pub struct AssetResidencyPolicySelectionEvidence {
    /// Proof that residency selection has been explicitly established.
    pub asset_residency_policy_selection_declared:
        Established<AssetResidencyPolicySelectionDeclared>,
}

impl ProvableFrom<AssetResidencyPolicySelectionEvidence> for AssetResidencyPolicySelectionValid {}

/// Evidence that one asset reference carries a valid access-pattern selection.
pub struct AssetAccessPatternSelectionEvidence {
    /// Proof that access-pattern selection has been explicitly established.
    pub asset_access_pattern_selection_declared: Established<AssetAccessPatternSelectionDeclared>,
}

impl ProvableFrom<AssetAccessPatternSelectionEvidence> for AssetAccessPatternSelectionValid {}

/// Evidence that one asset reference carries a valid revision selection.
pub struct AssetRevisionSelectionEvidence {
    /// Proof that revision selection has been explicitly established.
    pub asset_revision_selection_declared: Established<AssetRevisionSelectionDeclared>,
}

impl ProvableFrom<AssetRevisionSelectionEvidence> for AssetRevisionSelectionValid {}

/// Evidence that one asset reference carries a valid integrity selection.
pub struct AssetIntegritySelectionEvidence {
    /// Proof that integrity selection has been explicitly established.
    pub asset_integrity_selection_declared: Established<AssetIntegritySelectionDeclared>,
}

impl ProvableFrom<AssetIntegritySelectionEvidence> for AssetIntegritySelectionValid {}

/// Evidence that one asset reference carries a valid logical-size selection.
pub struct AssetLogicalSizeSelectionEvidence {
    /// Proof that logical-size selection has been explicitly established.
    pub asset_logical_size_selection_declared: Established<AssetLogicalSizeSelectionDeclared>,
}

impl ProvableFrom<AssetLogicalSizeSelectionEvidence> for AssetLogicalSizeSelectionValid {}

/// Evidence that one material texture binding is renderable.
pub struct RenderMaterialTextureBindingEvidence {
    /// Proof that the bound texture semantic is explicitly declared.
    pub material_texture_semantic_declared: Established<MaterialTextureSemanticDeclared>,
    /// Proof that the bound texture asset is renderable.
    pub asset: Established<RenderAssetReferenceValid>,
}

impl ProvableFrom<RenderMaterialTextureBindingEvidence> for RenderMaterialTextureBindingValid {}

/// Evidence that one material pipeline-intent descriptor is renderable.
pub struct RenderPipelineIntentEvidence {
    /// Proof that the pipeline domain is explicitly declared.
    pub material_pipeline_domain_declared: Established<MaterialPipelineDomainDeclared>,
    /// Proof that opaque-method selection is valid for the full descriptor shape.
    pub material_opaque_method_selection: Established<MaterialOpaqueMethodSelectionValid>,
    /// Proof that requested prepasses do not repeat.
    pub material_prepasses_unique: Established<MaterialPrepassesUnique>,
    /// Proof that the material family is compatible with the chosen domain.
    pub material_pipeline_family_compatible: Established<MaterialPipelineFamilyCompatible>,
    /// Proof that any chosen opaque method is compatible with the chosen domain.
    pub material_opaque_method_domain_compatible: Established<MaterialOpaqueMethodDomainCompatible>,
    /// Proof that requested prepasses are compatible with the chosen domain.
    pub material_prepasses_domain_compatible: Established<MaterialPrepassesDomainCompatible>,
}

impl ProvableFrom<RenderPipelineIntentEvidence> for RenderPipelineIntentValid {}

/// Evidence that one pipeline intent carries a valid opaque-method selection.
pub struct MaterialOpaqueMethodSelectionEvidence {
    /// Proof that opaque-method selection has been explicitly established.
    pub material_opaque_method_selection_declared:
        Established<MaterialOpaqueMethodSelectionDeclared>,
}

impl ProvableFrom<MaterialOpaqueMethodSelectionEvidence> for MaterialOpaqueMethodSelectionValid {}

/// Evidence that one render output target descriptor is renderable.
pub struct RenderOutputTargetEvidence {
    /// Proof that the output target is explicitly declared.
    pub render_output_target_declared: Established<RenderOutputTargetDeclared>,
    /// Proof that target-id selection is valid for the full descriptor shape.
    pub render_output_target_id_selection: Established<RenderOutputTargetIdSelectionValid>,
    /// Proof that asset selection is valid for the full descriptor shape.
    pub asset_selection: Established<RenderOutputTargetAssetSelectionValid>,
}

impl ProvableFrom<RenderOutputTargetEvidence> for RenderOutputTargetValid {}

/// Evidence that one output target carries a valid identifier selection.
pub struct RenderOutputTargetIdSelectionEvidence {
    /// Proof that target-id selection has been explicitly established.
    pub render_output_target_id_selection_declared:
        Established<RenderOutputTargetIdSelectionDeclared>,
}

impl ProvableFrom<RenderOutputTargetIdSelectionEvidence> for RenderOutputTargetIdSelectionValid {}

/// Evidence that one output target carries a valid asset selection.
pub struct RenderOutputTargetAssetSelectionEvidence {
    /// Proof that asset selection has been explicitly established.
    pub render_output_target_asset_selection_declared:
        Established<RenderOutputTargetAssetSelectionDeclared>,
}

impl ProvableFrom<RenderOutputTargetAssetSelectionEvidence>
    for RenderOutputTargetAssetSelectionValid
{
}

/// Evidence that one render view-output descriptor carries a valid
/// output-target policy.
pub struct RenderOutputTargetPolicyEvidence {
    /// Proof that output-target policy has been explicitly established.
    pub render_output_target_policy_declared: Established<RenderOutputTargetPolicyDeclared>,
}

impl ProvableFrom<RenderOutputTargetPolicyEvidence> for RenderOutputTargetPolicyValid {}

/// Evidence that one render clear-policy descriptor is renderable.
pub struct RenderClearPolicyEvidence {
    /// Proof that the clear policy is explicitly declared.
    pub render_clear_policy_declared: Established<RenderClearPolicyDeclared>,
    /// Proof that the depth-clear selection has been explicitly established.
    pub clear_depth_selection: Established<ClearDepthSelectionValid>,
}

impl ProvableFrom<RenderClearPolicyEvidence> for RenderClearPolicyValid {}

/// Evidence that one clear-policy descriptor carries a valid depth-clear selection.
pub struct ClearDepthSelectionEvidence {
    /// Proof that depth-clear selection has been explicitly established.
    pub clear_depth_selection_declared: Established<ClearDepthSelectionDeclared>,
}

impl ProvableFrom<ClearDepthSelectionEvidence> for ClearDepthSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid
/// clear-policy selection.
pub struct RenderClearPolicySelectionEvidence {
    /// Proof that clear-policy selection has been explicitly established.
    pub render_clear_policy_selection_declared: Established<RenderClearPolicySelectionDeclared>,
}

impl ProvableFrom<RenderClearPolicySelectionEvidence> for RenderClearPolicySelectionValid {}

/// Evidence that one render viewport descriptor is renderable.
pub struct RenderViewportEvidence {
    /// Proof that the viewport is explicitly declared.
    pub render_viewport_declared: Established<RenderViewportDeclared>,
    /// Proof that the viewport's physical size is strictly positive.
    pub render_viewport_size_positive: Established<RenderViewportSizePositive>,
    /// Proof that the depth-range selection has been explicitly established.
    pub render_viewport_depth_range_selection: Established<RenderViewportDepthRangeSelectionValid>,
}

impl ProvableFrom<RenderViewportEvidence> for RenderViewportValid {}

/// Evidence that one viewport descriptor carries a valid depth-range selection.
pub struct RenderViewportDepthRangeSelectionEvidence {
    /// Proof that viewport depth-range selection has been explicitly established.
    pub render_viewport_depth_range_selection_declared:
        Established<RenderViewportDepthRangeSelectionDeclared>,
}

impl ProvableFrom<RenderViewportDepthRangeSelectionEvidence>
    for RenderViewportDepthRangeSelectionValid
{
}

/// Evidence that one render view-output descriptor carries a valid viewport
/// policy.
pub struct RenderViewportPolicyEvidence {
    /// Proof that viewport policy has been explicitly established.
    pub render_viewport_policy_declared: Established<RenderViewportPolicyDeclared>,
}

impl ProvableFrom<RenderViewportPolicyEvidence> for RenderViewportPolicyValid {}

/// Evidence that one render resolution-override descriptor is renderable.
pub struct RenderResolutionOverrideEvidence {
    /// Proof that the resolution override is explicitly declared.
    pub render_resolution_override_declared: Established<RenderResolutionOverrideDeclared>,
    /// Proof that the resolution override is strictly positive.
    pub render_resolution_override_positive: Established<RenderResolutionOverridePositive>,
}

impl ProvableFrom<RenderResolutionOverrideEvidence> for RenderResolutionOverrideValid {}

/// Evidence that one render view-output descriptor carries a valid
/// resolution-override policy.
pub struct RenderResolutionOverridePolicyEvidence {
    /// Proof that resolution-override policy has been explicitly established.
    pub render_resolution_override_policy_declared:
        Established<RenderResolutionOverridePolicyDeclared>,
}

impl ProvableFrom<RenderResolutionOverridePolicyEvidence> for RenderResolutionOverridePolicyValid {}

/// Evidence that one render subview-layout descriptor is renderable.
pub struct RenderSubViewLayoutEvidence {
    /// Proof that the subview layout is explicitly declared.
    pub render_subview_layout_declared: Established<RenderSubViewLayoutDeclared>,
    /// Proof that the full layout extents are strictly positive.
    pub render_subview_layout_full_extent_positive:
        Established<RenderSubViewLayoutFullExtentPositive>,
    /// Proof that the subview extents are strictly positive.
    pub render_subview_layout_subview_extent_positive:
        Established<RenderSubViewLayoutSubviewExtentPositive>,
    /// Proof that the subview offsets are finite.
    pub render_subview_layout_offset_finite: Established<RenderSubViewLayoutOffsetFinite>,
    /// Proof that the subview stays within the declared full extent.
    pub render_subview_layout_within_full_extent: Established<RenderSubViewLayoutWithinFullExtent>,
}

impl ProvableFrom<RenderSubViewLayoutEvidence> for RenderSubViewLayoutValid {}

/// Evidence that one render view-output descriptor carries a valid
/// subview-layout policy.
pub struct RenderSubViewLayoutPolicyEvidence {
    /// Proof that subview-layout policy has been explicitly established.
    pub render_subview_layout_policy_declared: Established<RenderSubViewLayoutPolicyDeclared>,
}

impl ProvableFrom<RenderSubViewLayoutPolicyEvidence> for RenderSubViewLayoutPolicyValid {}

/// Evidence that one render view-output descriptor carries a valid
/// tonemapping selection.
pub struct TonemappingPolicyEvidence {
    /// Proof that tonemapping selection has been explicitly established.
    pub tonemapping_policy_declared: Established<TonemappingPolicyDeclared>,
}

impl ProvableFrom<TonemappingPolicyEvidence> for TonemappingPolicyValid {}

/// Evidence that one render view-output descriptor carries a valid MSAA
/// selection.
pub struct MsaaPolicyEvidence {
    /// Proof that MSAA selection has been explicitly established.
    pub msaa_policy_declared: Established<MsaaPolicyDeclared>,
}

impl ProvableFrom<MsaaPolicyEvidence> for MsaaPolicyValid {}

/// Evidence that one render view-output descriptor carries a valid mip-bias
/// selection.
pub struct MipBiasPolicyEvidence {
    /// Proof that mip-bias selection has been explicitly established.
    pub mip_bias_policy_declared: Established<MipBiasPolicyDeclared>,
}

impl ProvableFrom<MipBiasPolicyEvidence> for MipBiasPolicyValid {}

/// Evidence that one render view-output descriptor carries a valid
/// fixed-exposure selection.
pub struct RenderExposureSelectionEvidence {
    /// Proof that fixed-exposure selection has been explicitly established.
    pub render_exposure_selection_declared: Established<RenderExposureSelectionDeclared>,
}

impl ProvableFrom<RenderExposureSelectionEvidence> for RenderExposureSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid
/// color-grading selection.
pub struct RenderColorGradingSelectionEvidence {
    /// Proof that color-grading selection has been explicitly established.
    pub render_color_grading_selection_declared: Established<RenderColorGradingSelectionDeclared>,
}

impl ProvableFrom<RenderColorGradingSelectionEvidence> for RenderColorGradingSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid
/// auto-exposure selection.
pub struct RenderAutoExposureSelectionEvidence {
    /// Proof that auto-exposure selection has been explicitly established.
    pub render_auto_exposure_selection_declared: Established<RenderAutoExposureSelectionDeclared>,
}

impl ProvableFrom<RenderAutoExposureSelectionEvidence> for RenderAutoExposureSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid bloom
/// selection.
pub struct RenderBloomSelectionEvidence {
    /// Proof that bloom selection has been explicitly established.
    pub render_bloom_selection_declared: Established<RenderBloomSelectionDeclared>,
}

impl ProvableFrom<RenderBloomSelectionEvidence> for RenderBloomSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid
/// depth-of-field selection.
pub struct RenderDepthOfFieldSelectionEvidence {
    /// Proof that depth-of-field selection has been explicitly established.
    pub render_depth_of_field_selection_declared: Established<RenderDepthOfFieldSelectionDeclared>,
}

impl ProvableFrom<RenderDepthOfFieldSelectionEvidence> for RenderDepthOfFieldSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid motion-blur
/// selection.
pub struct RenderMotionBlurSelectionEvidence {
    /// Proof that motion-blur selection has been explicitly established.
    pub render_motion_blur_selection_declared: Established<RenderMotionBlurSelectionDeclared>,
}

impl ProvableFrom<RenderMotionBlurSelectionEvidence> for RenderMotionBlurSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid
/// chromatic-aberration selection.
pub struct RenderChromaticAberrationSelectionEvidence {
    /// Proof that chromatic-aberration selection has been explicitly established.
    pub render_chromatic_aberration_selection_declared:
        Established<RenderChromaticAberrationSelectionDeclared>,
}

impl ProvableFrom<RenderChromaticAberrationSelectionEvidence>
    for RenderChromaticAberrationSelectionValid
{
}

/// Evidence that one render view-output descriptor carries a valid
/// screen-space-reflections selection.
pub struct RenderScreenSpaceReflectionsSelectionEvidence {
    /// Proof that screen-space-reflections selection has been explicitly established.
    pub render_screen_space_reflections_selection_declared:
        Established<RenderScreenSpaceReflectionsSelectionDeclared>,
}

impl ProvableFrom<RenderScreenSpaceReflectionsSelectionEvidence>
    for RenderScreenSpaceReflectionsSelectionValid
{
}

/// Evidence that one render view-output descriptor carries a valid
/// screen-space ambient-occlusion selection.
pub struct RenderScreenSpaceAmbientOcclusionSelectionEvidence {
    /// Proof that screen-space ambient-occlusion selection has been explicitly established.
    pub render_screen_space_ambient_occlusion_selection_declared:
        Established<RenderScreenSpaceAmbientOcclusionSelectionDeclared>,
}

impl ProvableFrom<RenderScreenSpaceAmbientOcclusionSelectionEvidence>
    for RenderScreenSpaceAmbientOcclusionSelectionValid
{
}

/// Evidence that one render view-output descriptor carries a valid
/// shadow-filtering-method selection.
pub struct ShadowFilteringMethodSelectionEvidence {
    /// Proof that shadow-filtering-method selection has been explicitly established.
    pub shadow_filtering_method_selection_declared:
        Established<ShadowFilteringMethodSelectionDeclared>,
}

impl ProvableFrom<ShadowFilteringMethodSelectionEvidence> for ShadowFilteringMethodSelectionValid {}

/// Evidence that one render view-output descriptor carries a valid
/// screen-space transmission-quality selection.
pub struct ScreenSpaceTransmissionQualitySelectionEvidence {
    /// Proof that screen-space transmission-quality selection has been explicitly established.
    pub screen_space_transmission_quality_selection_declared:
        Established<ScreenSpaceTransmissionQualitySelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceTransmissionQualitySelectionEvidence>
    for ScreenSpaceTransmissionQualitySelectionValid
{
}

/// Evidence that one layer view-routing descriptor is renderable.
pub struct RenderViewParticipationEvidence {
    /// Proof that the routing mode is explicitly declared.
    pub view_participation_mode_declared: Established<ViewParticipationModeDeclared>,
    /// Proof that declared view identifiers are non-empty.
    pub view_participation_view_ids_non_empty: Established<ViewParticipationViewIdsNonEmpty>,
    /// Proof that declared view identifiers are unique.
    pub view_participation_view_ids_unique: Established<ViewParticipationViewIdsUnique>,
    /// Proof that the declared view list matches the routing mode.
    pub view_participation_list_matches_mode: Established<ViewParticipationListMatchesMode>,
}

impl ProvableFrom<RenderViewParticipationEvidence> for RenderViewParticipationValid {}

/// Evidence that one material intent descriptor is renderable.
pub struct RenderMaterialIntentEvidence {
    /// Proof that the material family is explicitly declared.
    pub material_family_declared: Established<MaterialFamilyDeclared>,
    /// Proof that alpha-mode selection is valid for the full descriptor shape.
    pub material_alpha_mode_selection: Established<MaterialAlphaModeSelectionValid>,
    /// Proof that cull-mode selection is valid for the full descriptor shape.
    pub material_cull_mode_selection: Established<MaterialCullModeSelectionValid>,
    /// Proof that unlit selection is valid for the full descriptor shape.
    pub material_unlit_selection: Established<MaterialUnlitSelectionValid>,
    /// Proof that depth-write selection is valid for the full descriptor shape.
    pub material_depth_write_selection: Established<MaterialDepthWriteSelectionValid>,
    /// Proof that depth-bias selection is valid for the full descriptor shape.
    pub material_depth_bias_selection: Established<MaterialDepthBiasSelectionValid>,
    /// Proof that specialization-key selection is valid for the full descriptor shape.
    pub material_specialization_key_selection: Established<MaterialSpecializationKeySelectionValid>,
    /// Proof that pipeline-intent selection is valid for the full descriptor shape.
    pub pipeline_intent_selection: Established<RenderPipelineIntentSelectionValid>,
    /// Proofs that every explicit material texture binding is renderable.
    pub textures: Vec<Established<RenderMaterialTextureBindingValid>>,
}

impl ProvableFrom<RenderMaterialIntentEvidence> for RenderMaterialIntentValid {}

/// Evidence that one material intent carries a valid alpha-mode selection.
pub struct MaterialAlphaModeSelectionEvidence {
    /// Proof that alpha-mode selection has been explicitly established.
    pub material_alpha_mode_selection_declared: Established<MaterialAlphaModeSelectionDeclared>,
}

impl ProvableFrom<MaterialAlphaModeSelectionEvidence> for MaterialAlphaModeSelectionValid {}

/// Evidence that one material intent carries a valid cull-mode selection.
pub struct MaterialCullModeSelectionEvidence {
    /// Proof that cull-mode selection has been explicitly established.
    pub material_cull_mode_selection_declared: Established<MaterialCullModeSelectionDeclared>,
}

impl ProvableFrom<MaterialCullModeSelectionEvidence> for MaterialCullModeSelectionValid {}

/// Evidence that one material intent carries a valid unlit selection.
pub struct MaterialUnlitSelectionEvidence {
    /// Proof that unlit selection has been explicitly established.
    pub material_unlit_selection_declared: Established<MaterialUnlitSelectionDeclared>,
}

impl ProvableFrom<MaterialUnlitSelectionEvidence> for MaterialUnlitSelectionValid {}

/// Evidence that one material intent carries a valid depth-write selection.
pub struct MaterialDepthWriteSelectionEvidence {
    /// Proof that depth-write selection has been explicitly established.
    pub material_depth_write_selection_declared: Established<MaterialDepthWriteSelectionDeclared>,
}

impl ProvableFrom<MaterialDepthWriteSelectionEvidence> for MaterialDepthWriteSelectionValid {}

/// Evidence that one material intent carries a valid depth-bias selection.
pub struct MaterialDepthBiasSelectionEvidence {
    /// Proof that depth-bias selection has been explicitly established.
    pub material_depth_bias_selection_declared: Established<MaterialDepthBiasSelectionDeclared>,
}

impl ProvableFrom<MaterialDepthBiasSelectionEvidence> for MaterialDepthBiasSelectionValid {}

/// Evidence that one material intent carries a valid specialization-key selection.
pub struct MaterialSpecializationKeySelectionEvidence {
    /// Proof that specialization-key selection has been explicitly established.
    pub material_specialization_key_selection_declared:
        Established<MaterialSpecializationKeySelectionDeclared>,
}

impl ProvableFrom<MaterialSpecializationKeySelectionEvidence>
    for MaterialSpecializationKeySelectionValid
{
}

/// Evidence that one material intent carries a valid pipeline-intent selection.
pub struct RenderPipelineIntentSelectionEvidence {
    /// Proof that pipeline-intent selection has been explicitly established.
    pub render_pipeline_intent_selection_declared:
        Established<RenderPipelineIntentSelectionDeclared>,
}

impl ProvableFrom<RenderPipelineIntentSelectionEvidence> for RenderPipelineIntentSelectionValid {}

/// Evidence that a world-space policy is renderable.
pub struct RenderWorldSpaceEvidence {
    /// Proof that an explicit world origin is finite when supplied.
    pub world_origin_finite: Established<WorldOriginFinite>,
    /// Proof that units-per-meter is positive.
    pub world_units_per_meter_positive: Established<WorldUnitsPerMeterPositive>,
    /// Proof that vertical exaggeration is positive.
    pub world_vertical_exaggeration_positive: Established<WorldVerticalExaggerationPositive>,
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
    pub numeric_expression_parameters_finite: Vec<Established<NumericExpressionParametersFinite>>,
    /// Proofs that variadic numeric expressions declare operands.
    pub numeric_expression_operands_present: Vec<Established<NumericExpressionOperandsPresent>>,
    /// Proofs that any step-expression thresholds are ordered.
    pub numeric_step_stops_ordered: Vec<Established<NumericStepStopsOrdered>>,
    /// Proofs that any interpolation stops are ordered.
    pub numeric_interpolation_stops_ordered: Vec<Established<NumericInterpolationStopsOrdered>>,
    /// Proofs that any exponential interpolation bases are positive.
    pub numeric_interpolation_base_positive: Vec<Established<NumericInterpolationBasePositive>>,
}

impl ProvableFrom<RenderLabelStyleEvidence> for RenderLabelStyleValid {}

/// Evidence that a shadow-participation descriptor is renderable.
pub struct RenderShadowParticipationEvidence {
    /// Proof that the shadow-participation policy is explicitly declared.
    pub shadow_participation_policy_declared: Established<ShadowParticipationPolicyDeclared>,
}

impl ProvableFrom<RenderShadowParticipationEvidence> for RenderShadowParticipationValid {}

/// Evidence that a vector feature symbolizer is renderable.
pub struct RenderFeatureSymbolizerEvidence {
    /// Proof that the paint choices are renderable.
    pub symbolizer_uses_renderable_paint: Established<SymbolizerUsesRenderablePaint>,
    /// Proof that stroke selection is valid for the full symbolizer shape.
    pub stroke_selection: Established<RenderStrokeSelectionValid>,
    /// Proof that fill selection is valid for the full symbolizer shape.
    pub fill_selection: Established<RenderFillSelectionValid>,
    /// Proof that point-symbol selection is valid for the full symbolizer shape.
    pub point_symbol_selection: Established<RenderPointSymbolSelectionValid>,
    /// Proof that label selection is valid for the full symbolizer shape.
    pub label_selection: Established<RenderLabelSelectionValid>,
    /// Proof that placement selection is valid for the full symbolizer shape.
    pub placement_selection: Established<RenderFeaturePlacementSelectionValid>,
    /// Proof that shadow-participation selection is valid for the full symbolizer shape.
    pub shadow_participation_selection: Established<RenderFeatureShadowParticipationSelectionValid>,
    /// Proof that material-intent selection is valid for the full symbolizer shape.
    pub material_intent_selection: Established<RenderFeatureMaterialIntentSelectionValid>,
    /// Proofs that any numeric expressions used by placement or extrusion were
    /// explicitly declared.
    pub numeric_expressions_declared: Vec<Established<NumericExpressionDeclared>>,
    /// Proofs that numeric-expression literal parameters are finite.
    pub numeric_expression_parameters_finite: Vec<Established<NumericExpressionParametersFinite>>,
    /// Proofs that variadic numeric expressions declare operands.
    pub numeric_expression_operands_present: Vec<Established<NumericExpressionOperandsPresent>>,
    /// Proofs that any step-expression thresholds are ordered.
    pub numeric_step_stops_ordered: Vec<Established<NumericStepStopsOrdered>>,
    /// Proofs that any interpolation stops are ordered.
    pub numeric_interpolation_stops_ordered: Vec<Established<NumericInterpolationStopsOrdered>>,
    /// Proofs that any exponential interpolation bases are positive.
    pub numeric_interpolation_base_positive: Vec<Established<NumericInterpolationBasePositive>>,
    /// Proof that extrusion selection is valid for the full symbolizer shape.
    pub extrusion_selection: Established<RenderExtrusionSelectionValid>,
}

impl ProvableFrom<RenderFeatureSymbolizerEvidence> for RenderFeatureSymbolizerValid {}

/// Evidence that one feature symbolizer carries a valid stroke selection.
pub struct RenderStrokeSelectionEvidence {
    /// Proof that stroke selection has been explicitly established.
    pub render_stroke_selection_declared: Established<RenderStrokeSelectionDeclared>,
}

impl ProvableFrom<RenderStrokeSelectionEvidence> for RenderStrokeSelectionValid {}

/// Evidence that one feature symbolizer carries a valid fill selection.
pub struct RenderFillSelectionEvidence {
    /// Proof that fill selection has been explicitly established.
    pub render_fill_selection_declared: Established<RenderFillSelectionDeclared>,
}

impl ProvableFrom<RenderFillSelectionEvidence> for RenderFillSelectionValid {}

/// Evidence that one feature symbolizer carries a valid point-symbol selection.
pub struct RenderPointSymbolSelectionEvidence {
    /// Proof that point-symbol selection has been explicitly established.
    pub render_point_symbol_selection_declared: Established<RenderPointSymbolSelectionDeclared>,
}

impl ProvableFrom<RenderPointSymbolSelectionEvidence> for RenderPointSymbolSelectionValid {}

/// Evidence that one feature symbolizer carries a valid label selection.
pub struct RenderLabelSelectionEvidence {
    /// Proof that label selection has been explicitly established.
    pub render_label_selection_declared: Established<RenderLabelSelectionDeclared>,
}

impl ProvableFrom<RenderLabelSelectionEvidence> for RenderLabelSelectionValid {}

/// Evidence that one feature symbolizer carries a valid placement selection.
pub struct RenderFeaturePlacementSelectionEvidence {
    /// Proof that placement selection has been explicitly established.
    pub render_feature_placement_selection_declared:
        Established<RenderFeaturePlacementSelectionDeclared>,
}

impl ProvableFrom<RenderFeaturePlacementSelectionEvidence>
    for RenderFeaturePlacementSelectionValid
{
}

/// Evidence that one feature symbolizer carries a valid extrusion selection.
pub struct RenderExtrusionSelectionEvidence {
    /// Proof that extrusion selection has been explicitly established.
    pub render_extrusion_selection_declared: Established<RenderExtrusionSelectionDeclared>,
}

impl ProvableFrom<RenderExtrusionSelectionEvidence> for RenderExtrusionSelectionValid {}

/// Evidence that one feature symbolizer carries a valid shadow-participation selection.
pub struct RenderFeatureShadowParticipationSelectionEvidence {
    /// Proof that shadow-participation selection has been explicitly established.
    pub render_feature_shadow_participation_selection_declared:
        Established<RenderFeatureShadowParticipationSelectionDeclared>,
}

impl ProvableFrom<RenderFeatureShadowParticipationSelectionEvidence>
    for RenderFeatureShadowParticipationSelectionValid
{
}

/// Evidence that one feature symbolizer carries a valid material-intent selection.
pub struct RenderFeatureMaterialIntentSelectionEvidence {
    /// Proof that material-intent selection has been explicitly established.
    pub render_feature_material_intent_selection_declared:
        Established<RenderFeatureMaterialIntentSelectionDeclared>,
}

impl ProvableFrom<RenderFeatureMaterialIntentSelectionEvidence>
    for RenderFeatureMaterialIntentSelectionValid
{
}

/// Evidence that a rule-based style override is renderable.
pub struct RenderFeatureStyleRuleEvidence {
    /// Proof that the rule predicate is explicitly declared.
    pub feature_predicate_declared: Established<FeaturePredicateDeclared>,
    /// Proof that scale-range selection is valid for the full rule shape.
    pub scale_range_selection: Established<RenderScaleRangeSelectionValid>,
    /// Proof that the rule symbolizer is renderable.
    pub symbolizer: Established<RenderFeatureSymbolizerValid>,
}

impl ProvableFrom<RenderFeatureStyleRuleEvidence> for RenderFeatureStyleRuleValid {}

/// Evidence that one rule or annotation carries a valid scale-range selection.
pub struct RenderScaleRangeSelectionEvidence {
    /// Proof that scale-range selection has been explicitly established.
    pub render_scale_range_selection_declared: Established<RenderScaleRangeSelectionDeclared>,
}

impl ProvableFrom<RenderScaleRangeSelectionEvidence> for RenderScaleRangeSelectionValid {}

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
    pub feature_batching_policy_declared: Established<FeatureBatchingPolicyDeclared>,
    /// Proof that max-features-per-batch selection is valid for the full descriptor shape.
    pub feature_batch_limit_selection: Established<FeatureBatchLimitSelectionValid>,
    /// Proofs that any grouping-key property names are non-empty.
    pub feature_batch_key_property_non_empty: Vec<Established<FeatureBatchKeyPropertyNonEmpty>>,
}

impl ProvableFrom<RenderFeatureBatchingEvidence> for RenderFeatureBatchingValid {}

/// Evidence that one batching descriptor carries a valid max-features selection.
pub struct FeatureBatchLimitSelectionEvidence {
    /// Proof that max-features selection has been explicitly established.
    pub feature_batch_limit_selection_declared: Established<FeatureBatchLimitSelectionDeclared>,
}

impl ProvableFrom<FeatureBatchLimitSelectionEvidence> for FeatureBatchLimitSelectionValid {}

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
    pub tile_refinement_priority_declared: Established<TileRefinementPriorityDeclared>,
    /// Proof that max concurrent requests is strictly positive.
    pub tile_concurrent_requests_positive: Established<TileConcurrentRequestsPositive>,
    /// Proof that the cache-budget selection has been explicitly established.
    pub tile_cache_budget_selection: Established<TileCacheBudgetSelectionValid>,
    /// Proof that the retry-backoff selection has been explicitly established.
    pub tile_retry_backoff_selection: Established<TileRetryBackoffSelectionValid>,
    /// Proof that the transition-fade selection has been explicitly established.
    pub tile_transition_fade_selection: Established<TileTransitionFadeSelectionValid>,
}

impl ProvableFrom<RenderTileStreamingEvidence> for RenderTileStreamingValid {}

/// Evidence that one streaming descriptor carries a valid cache-budget selection.
pub struct TileCacheBudgetSelectionEvidence {
    /// Proof that tile cache-budget selection has been explicitly established.
    pub tile_cache_budget_selection_declared: Established<TileCacheBudgetSelectionDeclared>,
}

impl ProvableFrom<TileCacheBudgetSelectionEvidence> for TileCacheBudgetSelectionValid {}

/// Evidence that one streaming descriptor carries a valid retry-backoff selection.
pub struct TileRetryBackoffSelectionEvidence {
    /// Proof that tile retry-backoff selection has been explicitly established.
    pub tile_retry_backoff_selection_declared: Established<TileRetryBackoffSelectionDeclared>,
}

impl ProvableFrom<TileRetryBackoffSelectionEvidence> for TileRetryBackoffSelectionValid {}

/// Evidence that one streaming descriptor carries a valid transition-fade selection.
pub struct TileTransitionFadeSelectionEvidence {
    /// Proof that tile transition-fade selection has been explicitly established.
    pub tile_transition_fade_selection_declared: Established<TileTransitionFadeSelectionDeclared>,
}

impl ProvableFrom<TileTransitionFadeSelectionEvidence> for TileTransitionFadeSelectionValid {}

/// Evidence that a raster symbolizer is renderable.
pub struct RenderRasterStyleEvidence {
    /// Proof that raster opacity is in the unit interval.
    pub raster_opacity_unit_interval: Established<RasterOpacityUnitInterval>,
    /// Proof that raster band selection is valid.
    pub raster_band_selection_valid: Established<RasterBandSelectionValid>,
    /// Proof that raster sampling is declared.
    pub raster_sampling_declared: Established<RasterSamplingDeclared>,
    /// Proof that the color-ramp domain selection has been explicitly established.
    pub raster_color_ramp_domain_selection: Established<RasterColorRampDomainSelectionValid>,
    /// Proofs that raster value-transform parameters are finite.
    pub raster_value_transforms_finite: Vec<Established<RasterValueTransformFinite>>,
    /// Optional proofs that normalization domains are ordered.
    pub raster_normalize_domain_ordered: Vec<Established<RasterNormalizeDomainOrdered>>,
    /// Optional proofs that gamma exponents are positive.
    pub raster_gamma_positive: Vec<Established<RasterGammaPositive>>,
    /// Proof that the derived-product selection has been explicitly established.
    pub raster_derived_product_selection: Established<RasterDerivedProductSelectionValid>,
    /// Proof that the contour-interval selection has been explicitly established.
    pub raster_contour_interval_selection: Established<RasterContourIntervalSelectionValid>,
    /// Proof that the normal-strength selection has been explicitly established.
    pub raster_normal_strength_selection: Established<RasterNormalStrengthSelectionValid>,
    /// Proof that the material-intent selection has been explicitly established.
    pub material_intent_selection: Established<RenderRasterStyleMaterialIntentSelectionValid>,
}

impl ProvableFrom<RenderRasterStyleEvidence> for RenderRasterStyleValid {}

/// Evidence that one raster-style descriptor carries a valid color-ramp domain selection.
pub struct RasterColorRampDomainSelectionEvidence {
    /// Proof that the color-ramp domain selection has been explicitly established.
    pub raster_color_ramp_domain_selection_declared:
        Established<RasterColorRampDomainSelectionDeclared>,
}

impl ProvableFrom<RasterColorRampDomainSelectionEvidence> for RasterColorRampDomainSelectionValid {}

/// Evidence that one raster-style descriptor carries a valid derived-product selection.
pub struct RasterDerivedProductSelectionEvidence {
    /// Proof that the derived-product selection has been explicitly established.
    pub raster_derived_product_selection_declared:
        Established<RasterDerivedProductSelectionDeclared>,
}

impl ProvableFrom<RasterDerivedProductSelectionEvidence> for RasterDerivedProductSelectionValid {}

/// Evidence that one raster-style descriptor carries a valid contour-interval selection.
pub struct RasterContourIntervalSelectionEvidence {
    /// Proof that the contour-interval selection has been explicitly established.
    pub raster_contour_interval_selection_declared:
        Established<RasterContourIntervalSelectionDeclared>,
}

impl ProvableFrom<RasterContourIntervalSelectionEvidence> for RasterContourIntervalSelectionValid {}

/// Evidence that one raster-style descriptor carries a valid normal-strength selection.
pub struct RasterNormalStrengthSelectionEvidence {
    /// Proof that the normal-strength selection has been explicitly established.
    pub raster_normal_strength_selection_declared:
        Established<RasterNormalStrengthSelectionDeclared>,
}

impl ProvableFrom<RasterNormalStrengthSelectionEvidence> for RasterNormalStrengthSelectionValid {}

/// Evidence that one raster-style descriptor carries a valid material-intent selection.
pub struct RenderRasterStyleMaterialIntentSelectionEvidence {
    /// Proof that the raster-style material-intent selection has been explicitly established.
    pub render_raster_style_material_intent_selection_declared:
        Established<RenderRasterStyleMaterialIntentSelectionDeclared>,
}

impl ProvableFrom<RenderRasterStyleMaterialIntentSelectionEvidence>
    for RenderRasterStyleMaterialIntentSelectionValid
{
}

/// Evidence that a tile source is renderable.
pub struct RenderTileSourceEvidence {
    /// Proof that the tile URI template was declared.
    pub tile_uri_template_declared: Established<TileUriTemplateDeclared>,
    /// Proof that tile zoom bounds are ordered.
    pub tile_zoom_range_ordered: Established<TileZoomRangeOrdered>,
    /// Proof that tile dimensions are positive.
    pub tile_dimensions_positive: Established<TileDimensionsPositive>,
    /// Proof that the streaming selection has been explicitly established.
    pub streaming_selection: Established<TileSourceStreamingSelectionValid>,
}

impl ProvableFrom<RenderTileSourceEvidence> for RenderTileSourceValid {}

/// Evidence that one tile-source descriptor carries a valid streaming selection.
pub struct TileSourceStreamingSelectionEvidence {
    /// Proof that tile-source streaming selection has been explicitly established.
    pub tile_source_streaming_selection_declared: Established<TileSourceStreamingSelectionDeclared>,
}

impl ProvableFrom<TileSourceStreamingSelectionEvidence> for TileSourceStreamingSelectionValid {}

/// Evidence that a tile style is renderable.
pub struct RenderTileStyleEvidence {
    /// Proof that tile style opacity is in the unit interval.
    pub tile_opacity_unit_interval: Established<TileOpacityUnitInterval>,
    /// Proof that the raster-style selection has been explicitly established.
    pub raster_style_selection: Established<RenderTileStyleRasterSelectionValid>,
    /// Proof that the vector-style selection has been explicitly established.
    pub vector_style_selection: Established<RenderTileStyleVectorSelectionValid>,
    /// Proof that the material-intent selection has been explicitly established.
    pub material_intent_selection: Established<RenderTileStyleMaterialIntentSelectionValid>,
    /// Proof that the chosen style is compatible with the payload family.
    pub tile_style_payload_compatible: Established<TileStylePayloadCompatible>,
}

impl ProvableFrom<RenderTileStyleEvidence> for RenderTileStyleValid {}

/// Evidence that one tile-style descriptor carries a valid raster-style selection.
pub struct RenderTileStyleRasterSelectionEvidence {
    /// Proof that tile-style raster selection has been explicitly established.
    pub render_tile_style_raster_selection_declared:
        Established<RenderTileStyleRasterSelectionDeclared>,
}

impl ProvableFrom<RenderTileStyleRasterSelectionEvidence> for RenderTileStyleRasterSelectionValid {}

/// Evidence that one tile-style descriptor carries a valid vector-style selection.
pub struct RenderTileStyleVectorSelectionEvidence {
    /// Proof that tile-style vector selection has been explicitly established.
    pub render_tile_style_vector_selection_declared:
        Established<RenderTileStyleVectorSelectionDeclared>,
}

impl ProvableFrom<RenderTileStyleVectorSelectionEvidence> for RenderTileStyleVectorSelectionValid {}

/// Evidence that one tile-style descriptor carries a valid material-intent selection.
pub struct RenderTileStyleMaterialIntentSelectionEvidence {
    /// Proof that tile-style material-intent selection has been explicitly established.
    pub render_tile_style_material_intent_selection_declared:
        Established<RenderTileStyleMaterialIntentSelectionDeclared>,
}

impl ProvableFrom<RenderTileStyleMaterialIntentSelectionEvidence>
    for RenderTileStyleMaterialIntentSelectionValid
{
}

/// Evidence that a terrain LOD descriptor is renderable.
pub struct RenderTerrainLodEvidence {
    /// Proof that terrain LOD error thresholds are strictly positive.
    pub terrain_lod_screen_space_error_positive: Established<TerrainLodScreenSpaceErrorPositive>,
    /// Proof that the skirt-height selection has been explicitly established.
    pub terrain_skirt_height_selection: Established<TerrainSkirtHeightSelectionValid>,
}

impl ProvableFrom<RenderTerrainLodEvidence> for RenderTerrainLodValid {}

/// Evidence that one terrain-LOD descriptor carries a valid skirt-height selection.
pub struct TerrainSkirtHeightSelectionEvidence {
    /// Proof that terrain skirt-height selection has been explicitly established.
    pub terrain_skirt_height_selection_declared: Established<TerrainSkirtHeightSelectionDeclared>,
}

impl ProvableFrom<TerrainSkirtHeightSelectionEvidence> for TerrainSkirtHeightSelectionValid {}

/// Evidence that one terrain overlay is renderable.
pub struct RenderTerrainOverlayEvidence {
    /// Proof that overlay opacity is in the unit interval.
    pub terrain_overlay_opacity_unit_interval: Established<TerrainOverlayOpacityUnitInterval>,
    /// Proof that the overlay drape mode is declared.
    pub terrain_overlay_drape_mode_declared: Established<TerrainOverlayDrapeModeDeclared>,
    /// Proof that the overlay blend mode is declared.
    pub terrain_overlay_blend_mode_declared: Established<TerrainOverlayBlendModeDeclared>,
    /// Proof that overlay height bias is finite.
    pub terrain_overlay_height_bias_finite: Established<TerrainOverlayHeightBiasFinite>,
    /// Proof that the texture selection has been explicitly established.
    pub texture_selection: Established<TerrainOverlayTextureSelectionValid>,
    /// Proof that the raster-style selection has been explicitly established.
    pub raster_style_selection: Established<TerrainOverlayRasterStyleSelectionValid>,
}

impl ProvableFrom<RenderTerrainOverlayEvidence> for RenderTerrainOverlayValid {}

/// Evidence that one terrain-overlay descriptor carries a valid texture selection.
pub struct TerrainOverlayTextureSelectionEvidence {
    /// Proof that terrain overlay texture selection has been explicitly established.
    pub terrain_overlay_texture_selection_declared:
        Established<TerrainOverlayTextureSelectionDeclared>,
}

impl ProvableFrom<TerrainOverlayTextureSelectionEvidence> for TerrainOverlayTextureSelectionValid {}

/// Evidence that one terrain-overlay descriptor carries a valid raster-style selection.
pub struct TerrainOverlayRasterStyleSelectionEvidence {
    /// Proof that terrain overlay raster-style selection has been explicitly established.
    pub terrain_overlay_raster_style_selection_declared:
        Established<TerrainOverlayRasterStyleSelectionDeclared>,
}

impl ProvableFrom<TerrainOverlayRasterStyleSelectionEvidence>
    for TerrainOverlayRasterStyleSelectionValid
{
}

/// Evidence that a terrain style is renderable.
pub struct RenderTerrainStyleEvidence {
    /// Proof that terrain vertical scale is positive.
    pub terrain_vertical_scale_positive: Established<TerrainVerticalScalePositive>,
    /// Proof that base-height offset is finite.
    pub terrain_height_offset_finite: Established<TerrainHeightOffsetFinite>,
    /// Proof that the surface drape-mode selection has been explicitly established.
    pub terrain_surface_drape_selection: Established<TerrainSurfaceDrapeSelectionValid>,
    /// Proof that the mesh policy is declared.
    pub terrain_mesh_mode_declared: Established<TerrainMeshModeDeclared>,
    /// Proof that the shading policy is declared.
    pub terrain_shading_mode_declared: Established<TerrainShadingModeDeclared>,
    /// Proof that the surface texture selection has been explicitly established.
    pub surface_texture_selection: Established<TerrainSurfaceTextureSelectionValid>,
    /// Proof that the surface raster-style selection has been explicitly established.
    pub surface_raster_style_selection: Established<TerrainSurfaceRasterStyleSelectionValid>,
    /// Proofs that ordered terrain overlays are renderable.
    pub overlays: Vec<Established<RenderTerrainOverlayValid>>,
    /// Proof that the LOD selection has been explicitly established.
    pub lod_selection: Established<TerrainLodSelectionValid>,
    /// Proof that the shadow-participation selection has been explicitly established.
    pub shadow_participation_selection: Established<TerrainShadowParticipationSelectionValid>,
    /// Proof that the material-intent selection has been explicitly established.
    pub material_intent_selection: Established<TerrainMaterialIntentSelectionValid>,
}

impl ProvableFrom<RenderTerrainStyleEvidence> for RenderTerrainStyleValid {}

/// Evidence that one terrain-style descriptor carries a valid surface drape-mode selection.
pub struct TerrainSurfaceDrapeSelectionEvidence {
    /// Proof that terrain surface drape-mode selection has been explicitly established.
    pub terrain_surface_drape_selection_declared: Established<TerrainSurfaceDrapeSelectionDeclared>,
}

impl ProvableFrom<TerrainSurfaceDrapeSelectionEvidence> for TerrainSurfaceDrapeSelectionValid {}

/// Evidence that one terrain-style descriptor carries a valid surface texture selection.
pub struct TerrainSurfaceTextureSelectionEvidence {
    /// Proof that terrain surface texture selection has been explicitly established.
    pub terrain_surface_texture_selection_declared:
        Established<TerrainSurfaceTextureSelectionDeclared>,
}

impl ProvableFrom<TerrainSurfaceTextureSelectionEvidence> for TerrainSurfaceTextureSelectionValid {}

/// Evidence that one terrain-style descriptor carries a valid surface raster-style selection.
pub struct TerrainSurfaceRasterStyleSelectionEvidence {
    /// Proof that terrain surface raster-style selection has been explicitly established.
    pub terrain_surface_raster_style_selection_declared:
        Established<TerrainSurfaceRasterStyleSelectionDeclared>,
}

impl ProvableFrom<TerrainSurfaceRasterStyleSelectionEvidence>
    for TerrainSurfaceRasterStyleSelectionValid
{
}

/// Evidence that one terrain-style descriptor carries a valid LOD selection.
pub struct TerrainLodSelectionEvidence {
    /// Proof that terrain LOD selection has been explicitly established.
    pub terrain_lod_selection_declared: Established<TerrainLodSelectionDeclared>,
}

impl ProvableFrom<TerrainLodSelectionEvidence> for TerrainLodSelectionValid {}

/// Evidence that one terrain-style descriptor carries a valid shadow-participation selection.
pub struct TerrainShadowParticipationSelectionEvidence {
    /// Proof that terrain shadow-participation selection has been explicitly established.
    pub terrain_shadow_participation_selection_declared:
        Established<TerrainShadowParticipationSelectionDeclared>,
}

impl ProvableFrom<TerrainShadowParticipationSelectionEvidence>
    for TerrainShadowParticipationSelectionValid
{
}

/// Evidence that one terrain-style descriptor carries a valid material-intent selection.
pub struct TerrainMaterialIntentSelectionEvidence {
    /// Proof that terrain material-intent selection has been explicitly established.
    pub terrain_material_intent_selection_declared:
        Established<TerrainMaterialIntentSelectionDeclared>,
}

impl ProvableFrom<TerrainMaterialIntentSelectionEvidence> for TerrainMaterialIntentSelectionValid {}

/// Evidence that a color-grading descriptor is renderable.
pub struct RenderColorGradingEvidence {
    /// Proof that all color-grading numeric parameters are finite.
    pub color_grading_parameters_finite: Established<ColorGradingParametersFinite>,
    /// Proof that the declared midtones range is ordered.
    pub color_grading_midtones_range_ordered: Established<ColorGradingMidtonesRangeOrdered>,
}

impl ProvableFrom<RenderColorGradingEvidence> for RenderColorGradingValid {}

/// Evidence that a fixed-exposure descriptor is renderable.
pub struct RenderExposureEvidence {
    /// Proof that the exposure mode is explicitly declared.
    pub exposure_mode_declared: Established<ExposureModeDeclared>,
    /// Proof that the EV100 selection has been explicitly established.
    pub exposure_ev100_selection: Established<ExposureEv100SelectionValid>,
}

impl ProvableFrom<RenderExposureEvidence> for RenderExposureValid {}

/// Evidence that one exposure descriptor carries a valid EV100 selection.
pub struct ExposureEv100SelectionEvidence {
    /// Proof that exposure EV100 selection has been explicitly established.
    pub exposure_ev100_selection_declared: Established<ExposureEv100SelectionDeclared>,
}

impl ProvableFrom<ExposureEv100SelectionEvidence> for ExposureEv100SelectionValid {}

/// Evidence that an auto-exposure descriptor is renderable.
pub struct RenderAutoExposureEvidence {
    /// Proof that the exposure range is ordered.
    pub auto_exposure_range_ordered: Established<AutoExposureRangeOrdered>,
    /// Proof that the luminance filter window is ordered.
    pub auto_exposure_filter_ordered: Established<AutoExposureFilterOrdered>,
    /// Proof that the brighten/darken adaptation speeds are non-negative.
    pub auto_exposure_speed_non_negative: Established<AutoExposureSpeedNonNegative>,
    /// Proof that the transition distance is non-negative.
    pub auto_exposure_transition_distance_non_negative:
        Established<AutoExposureTransitionDistanceNonNegative>,
    /// Proof that the metering-mask selection has been explicitly established.
    pub metering_mask_selection: Established<AutoExposureMeteringMaskSelectionValid>,
    /// Proof that the compensation-curve selection has been explicitly established.
    pub compensation_curve_selection: Established<AutoExposureCompensationCurveSelectionValid>,
}

impl ProvableFrom<RenderAutoExposureEvidence> for RenderAutoExposureValid {}

/// Evidence that one auto-exposure descriptor carries a valid metering-mask selection.
pub struct AutoExposureMeteringMaskSelectionEvidence {
    /// Proof that auto-exposure metering-mask selection has been explicitly established.
    pub auto_exposure_metering_mask_selection_declared:
        Established<AutoExposureMeteringMaskSelectionDeclared>,
}

impl ProvableFrom<AutoExposureMeteringMaskSelectionEvidence>
    for AutoExposureMeteringMaskSelectionValid
{
}

/// Evidence that one auto-exposure descriptor carries a valid compensation-curve selection.
pub struct AutoExposureCompensationCurveSelectionEvidence {
    /// Proof that auto-exposure compensation-curve selection has been explicitly established.
    pub auto_exposure_compensation_curve_selection_declared:
        Established<AutoExposureCompensationCurveSelectionDeclared>,
}

impl ProvableFrom<AutoExposureCompensationCurveSelectionEvidence>
    for AutoExposureCompensationCurveSelectionValid
{
}

/// Evidence that a bloom descriptor is renderable.
pub struct RenderBloomEvidence {
    /// Proof that bloom numeric parameters are finite.
    pub bloom_parameters_finite: Established<BloomParametersFinite>,
    /// Proof that threshold softness stays in the unit interval.
    pub bloom_threshold_softness_unit_interval: Established<BloomThresholdSoftnessUnitInterval>,
    /// Proof that bloom scale factors are finite.
    pub bloom_scale_finite: Established<BloomScaleFinite>,
    /// Proof that the bloom composite mode is declared.
    pub bloom_composite_mode_declared: Established<BloomCompositeModeDeclared>,
}

impl ProvableFrom<RenderBloomEvidence> for RenderBloomValid {}

/// Evidence that a depth-of-field descriptor is renderable.
pub struct RenderDepthOfFieldEvidence {
    /// Proof that the depth-of-field mode is explicitly declared.
    pub depth_of_field_mode_declared: Established<DepthOfFieldModeDeclared>,
    /// Proof that depth-of-field floating-point parameters are finite.
    pub depth_of_field_parameters_finite: Established<DepthOfFieldParametersFinite>,
    /// Proof that focal distance is non-negative.
    pub depth_of_field_focal_distance_non_negative:
        Established<DepthOfFieldFocalDistanceNonNegative>,
    /// Proof that sensor height is strictly positive.
    pub depth_of_field_sensor_height_positive: Established<DepthOfFieldSensorHeightPositive>,
    /// Proof that aperture f-stops are strictly positive.
    pub depth_of_field_aperture_f_stops_positive: Established<DepthOfFieldApertureFStopsPositive>,
    /// Proof that the circle-of-confusion diameter is non-negative.
    pub depth_of_field_circle_of_confusion_non_negative:
        Established<DepthOfFieldCircleOfConfusionNonNegative>,
    /// Proof that max depth is non-negative.
    pub depth_of_field_max_depth_non_negative: Established<DepthOfFieldMaxDepthNonNegative>,
}

impl ProvableFrom<RenderDepthOfFieldEvidence> for RenderDepthOfFieldValid {}

/// Evidence that a motion-blur descriptor is renderable.
pub struct RenderMotionBlurEvidence {
    /// Proof that motion-blur floating-point parameters are finite.
    pub motion_blur_parameters_finite: Established<MotionBlurParametersFinite>,
    /// Proof that motion-blur sample counts are strictly positive.
    pub motion_blur_samples_positive: Established<MotionBlurSamplesPositive>,
}

impl ProvableFrom<RenderMotionBlurEvidence> for RenderMotionBlurValid {}

/// Evidence that a chromatic-aberration descriptor is renderable.
pub struct RenderChromaticAberrationEvidence {
    /// Proof that chromatic-aberration floating-point parameters are finite.
    pub chromatic_aberration_parameters_finite: Established<ChromaticAberrationParametersFinite>,
    /// Proof that chromatic-aberration sample counts are strictly positive.
    pub chromatic_aberration_samples_positive: Established<ChromaticAberrationSamplesPositive>,
    /// Proof that the color-LUT selection has been explicitly established.
    pub color_lut_selection: Established<ChromaticAberrationColorLutSelectionValid>,
}

impl ProvableFrom<RenderChromaticAberrationEvidence> for RenderChromaticAberrationValid {}

/// Evidence that one CA descriptor carries a valid color-LUT selection.
pub struct ChromaticAberrationColorLutSelectionEvidence {
    /// Proof that CA color-LUT selection has been explicitly established.
    pub chromatic_aberration_color_lut_selection_declared:
        Established<ChromaticAberrationColorLutSelectionDeclared>,
}

impl ProvableFrom<ChromaticAberrationColorLutSelectionEvidence>
    for ChromaticAberrationColorLutSelectionValid
{
}

/// Evidence that a screen-space-reflection descriptor is renderable.
pub struct RenderScreenSpaceReflectionsEvidence {
    /// Proof that floating-point reflection parameters are finite.
    pub screen_space_reflections_parameters_finite:
        Established<ScreenSpaceReflectionsParametersFinite>,
    /// Proof that the roughness-threshold selection has been explicitly established.
    pub screen_space_reflections_roughness_threshold_selection:
        Established<ScreenSpaceReflectionsRoughnessThresholdSelectionValid>,
    /// Proof that the thickness selection has been explicitly established.
    pub screen_space_reflections_thickness_selection:
        Established<ScreenSpaceReflectionsThicknessSelectionValid>,
    /// Proof that the linear-steps selection has been explicitly established.
    pub screen_space_reflections_linear_steps_selection:
        Established<ScreenSpaceReflectionsLinearStepsSelectionValid>,
    /// Proof that the linear-march-exponent selection has been explicitly established.
    pub screen_space_reflections_linear_march_exponent_selection:
        Established<ScreenSpaceReflectionsLinearMarchExponentSelectionValid>,
    /// Proof that the bisection-steps selection has been explicitly established.
    pub screen_space_reflections_bisection_steps_selection:
        Established<ScreenSpaceReflectionsBisectionStepsSelectionValid>,
}

impl ProvableFrom<RenderScreenSpaceReflectionsEvidence> for RenderScreenSpaceReflectionsValid {}

/// Evidence that one SSR descriptor carries a valid roughness-threshold selection.
pub struct ScreenSpaceReflectionsRoughnessThresholdSelectionEvidence {
    /// Proof that SSR roughness-threshold selection has been explicitly established.
    pub screen_space_reflections_roughness_threshold_selection_declared:
        Established<ScreenSpaceReflectionsRoughnessThresholdSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceReflectionsRoughnessThresholdSelectionEvidence>
    for ScreenSpaceReflectionsRoughnessThresholdSelectionValid
{
}

/// Evidence that one SSR descriptor carries a valid thickness selection.
pub struct ScreenSpaceReflectionsThicknessSelectionEvidence {
    /// Proof that SSR thickness selection has been explicitly established.
    pub screen_space_reflections_thickness_selection_declared:
        Established<ScreenSpaceReflectionsThicknessSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceReflectionsThicknessSelectionEvidence>
    for ScreenSpaceReflectionsThicknessSelectionValid
{
}

/// Evidence that one SSR descriptor carries a valid linear-steps selection.
pub struct ScreenSpaceReflectionsLinearStepsSelectionEvidence {
    /// Proof that SSR linear-steps selection has been explicitly established.
    pub screen_space_reflections_linear_steps_selection_declared:
        Established<ScreenSpaceReflectionsLinearStepsSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceReflectionsLinearStepsSelectionEvidence>
    for ScreenSpaceReflectionsLinearStepsSelectionValid
{
}

/// Evidence that one SSR descriptor carries a valid linear-march-exponent selection.
pub struct ScreenSpaceReflectionsLinearMarchExponentSelectionEvidence {
    /// Proof that SSR linear-march-exponent selection has been explicitly established.
    pub screen_space_reflections_linear_march_exponent_selection_declared:
        Established<ScreenSpaceReflectionsLinearMarchExponentSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceReflectionsLinearMarchExponentSelectionEvidence>
    for ScreenSpaceReflectionsLinearMarchExponentSelectionValid
{
}

/// Evidence that one SSR descriptor carries a valid bisection-steps selection.
pub struct ScreenSpaceReflectionsBisectionStepsSelectionEvidence {
    /// Proof that SSR bisection-steps selection has been explicitly established.
    pub screen_space_reflections_bisection_steps_selection_declared:
        Established<ScreenSpaceReflectionsBisectionStepsSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceReflectionsBisectionStepsSelectionEvidence>
    for ScreenSpaceReflectionsBisectionStepsSelectionValid
{
}

/// Evidence that a screen-space ambient-occlusion descriptor is renderable.
pub struct RenderScreenSpaceAmbientOcclusionEvidence {
    /// Proof that SSAO quality is explicitly declared.
    pub screen_space_ambient_occlusion_quality_declared:
        Established<ScreenSpaceAmbientOcclusionQualityDeclared>,
    /// Proof that the SSAO-parameters selection has been explicitly established.
    pub screen_space_ambient_occlusion_parameters_selection:
        Established<ScreenSpaceAmbientOcclusionParametersSelectionValid>,
    /// Proof that the SSAO-thickness selection has been explicitly established.
    pub screen_space_ambient_occlusion_thickness_selection:
        Established<ScreenSpaceAmbientOcclusionThicknessSelectionValid>,
    /// Proof that the SSAO custom-slice-count selection has been explicitly established.
    pub screen_space_ambient_occlusion_custom_slice_count_selection:
        Established<ScreenSpaceAmbientOcclusionCustomSliceCountSelectionValid>,
    /// Proof that the SSAO custom-samples-per-slice selection has been explicitly established.
    pub screen_space_ambient_occlusion_custom_samples_per_slice_selection:
        Established<ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionValid>,
}

impl ProvableFrom<RenderScreenSpaceAmbientOcclusionEvidence>
    for RenderScreenSpaceAmbientOcclusionValid
{
}

/// Evidence that one SSAO descriptor carries a valid parameters selection.
pub struct ScreenSpaceAmbientOcclusionParametersSelectionEvidence {
    /// Proof that SSAO parameters selection has been explicitly established.
    pub screen_space_ambient_occlusion_parameters_selection_declared:
        Established<ScreenSpaceAmbientOcclusionParametersSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceAmbientOcclusionParametersSelectionEvidence>
    for ScreenSpaceAmbientOcclusionParametersSelectionValid
{
}

/// Evidence that one SSAO descriptor carries a valid thickness selection.
pub struct ScreenSpaceAmbientOcclusionThicknessSelectionEvidence {
    /// Proof that SSAO thickness selection has been explicitly established.
    pub screen_space_ambient_occlusion_thickness_selection_declared:
        Established<ScreenSpaceAmbientOcclusionThicknessSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceAmbientOcclusionThicknessSelectionEvidence>
    for ScreenSpaceAmbientOcclusionThicknessSelectionValid
{
}

/// Evidence that one SSAO descriptor carries a valid custom-slice-count selection.
pub struct ScreenSpaceAmbientOcclusionCustomSliceCountSelectionEvidence {
    /// Proof that SSAO custom-slice-count selection has been explicitly established.
    pub screen_space_ambient_occlusion_custom_slice_count_selection_declared:
        Established<ScreenSpaceAmbientOcclusionCustomSliceCountSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceAmbientOcclusionCustomSliceCountSelectionEvidence>
    for ScreenSpaceAmbientOcclusionCustomSliceCountSelectionValid
{
}

/// Evidence that one SSAO descriptor carries a valid custom-samples-per-slice selection.
pub struct ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionEvidence {
    /// Proof that SSAO custom-samples-per-slice selection has been explicitly established.
    pub screen_space_ambient_occlusion_custom_samples_per_slice_selection_declared:
        Established<ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionDeclared>,
}

impl ProvableFrom<ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionEvidence>
    for ScreenSpaceAmbientOcclusionCustomSamplesPerSliceSelectionValid
{
}

/// Evidence that a render-view output descriptor is renderable.
pub struct RenderViewOutputEvidence {
    /// Proof that a view-output policy was explicitly declared.
    pub render_view_output_declared: Established<RenderViewOutputDeclared>,
    /// Proof that the HDR toggle is explicitly declared.
    pub hdr_output_declared: Established<HdrOutputDeclared>,
    /// Proof that output-target policy has been validated.
    pub target_policy: Established<RenderOutputTargetPolicyValid>,
    /// Proof that clear-policy selection has been validated.
    pub clear_policy_selection: Established<RenderClearPolicySelectionValid>,
    /// Proof that viewport policy has been validated.
    pub viewport_policy: Established<RenderViewportPolicyValid>,
    /// Proof that resolution-override policy has been validated.
    pub resolution_override_policy: Established<RenderResolutionOverridePolicyValid>,
    /// Proof that subview-layout policy has been validated.
    pub subview_layout_policy: Established<RenderSubViewLayoutPolicyValid>,
    /// Proof that tonemapping selection has been validated.
    pub tonemapping_policy: Established<TonemappingPolicyValid>,
    /// Proof that MSAA selection has been validated.
    pub msaa_policy: Established<MsaaPolicyValid>,
    /// Proof that mip-bias selection has been validated.
    pub mip_bias_policy: Established<MipBiasPolicyValid>,
    /// Proof that manual and automatic exposure policies do not conflict.
    pub exposure_policy_exclusive: Established<ExposurePolicyExclusive>,
    /// Proof that fixed-exposure selection has been validated.
    pub exposure_selection: Established<RenderExposureSelectionValid>,
    /// Proof that color-grading selection has been validated.
    pub color_grading_selection: Established<RenderColorGradingSelectionValid>,
    /// Proof that auto-exposure selection has been validated.
    pub auto_exposure_selection: Established<RenderAutoExposureSelectionValid>,
    /// Proof that bloom selection has been validated.
    pub bloom_selection: Established<RenderBloomSelectionValid>,
    /// Proof that depth-of-field selection has been validated.
    pub depth_of_field_selection: Established<RenderDepthOfFieldSelectionValid>,
    /// Proof that motion-blur selection has been validated.
    pub motion_blur_selection: Established<RenderMotionBlurSelectionValid>,
    /// Proof that chromatic-aberration selection has been validated.
    pub chromatic_aberration_selection: Established<RenderChromaticAberrationSelectionValid>,
    /// Proof that screen-space-reflections selection has been validated.
    pub screen_space_reflections_selection: Established<RenderScreenSpaceReflectionsSelectionValid>,
    /// Proof that screen-space ambient-occlusion selection has been validated.
    pub screen_space_ambient_occlusion_selection:
        Established<RenderScreenSpaceAmbientOcclusionSelectionValid>,
    /// Proof that shadow-filtering-method selection has been validated.
    pub shadow_filtering_method_selection: Established<ShadowFilteringMethodSelectionValid>,
    /// Proof that screen-space transmission-quality selection has been validated.
    pub screen_space_transmission_quality_selection:
        Established<ScreenSpaceTransmissionQualitySelectionValid>,
}

impl ProvableFrom<RenderViewOutputEvidence> for RenderViewOutputValid {}

/// Evidence that one render view descriptor carries a valid perspective
/// projection policy.
pub struct PerspectiveProjectionPolicyEvidence {
    /// Proof that perspective-projection policy has been explicitly established.
    pub perspective_projection_policy_declared: Established<PerspectiveProjectionPolicyDeclared>,
}

impl ProvableFrom<PerspectiveProjectionPolicyEvidence> for PerspectiveProjectionPolicyValid {}

/// Evidence that one render view descriptor carries a valid output selection.
pub struct RenderViewOutputSelectionEvidence {
    /// Proof that output selection has been explicitly established.
    pub render_view_output_selection_declared: Established<RenderViewOutputSelectionDeclared>,
}

impl ProvableFrom<RenderViewOutputSelectionEvidence> for RenderViewOutputSelectionValid {}

/// Evidence that one render view descriptor carries a valid world-space
/// selection.
pub struct RenderWorldSpaceSelectionEvidence {
    /// Proof that world-space selection has been explicitly established.
    pub render_world_space_selection_declared: Established<RenderWorldSpaceSelectionDeclared>,
}

impl ProvableFrom<RenderWorldSpaceSelectionEvidence> for RenderWorldSpaceSelectionValid {}

/// Evidence that one visible-solar-disk selection is valid.
pub struct RenderSunDiskSelectionEvidence {
    /// Proof that the sun-disk selection has been explicitly established.
    pub render_sun_disk_selection_declared: Established<RenderSunDiskSelectionDeclared>,
}

impl ProvableFrom<RenderSunDiskSelectionEvidence> for RenderSunDiskSelectionValid {}

/// Evidence that one scene-environment skybox selection is valid.
pub struct RenderSkyboxSelectionEvidence {
    /// Proof that the skybox selection has been explicitly established.
    pub render_skybox_selection_declared: Established<RenderSkyboxSelectionDeclared>,
}

impl ProvableFrom<RenderSkyboxSelectionEvidence> for RenderSkyboxSelectionValid {}

/// Evidence that one scene-environment ambient-light selection is valid.
pub struct RenderAmbientLightSelectionEvidence {
    /// Proof that the ambient-light selection has been explicitly established.
    pub render_ambient_light_selection_declared: Established<RenderAmbientLightSelectionDeclared>,
}

impl ProvableFrom<RenderAmbientLightSelectionEvidence> for RenderAmbientLightSelectionValid {}

/// Evidence that one scene-environment directional-light selection is valid.
pub struct RenderDirectionalLightSelectionEvidence {
    /// Proof that the directional-light selection has been explicitly established.
    pub render_directional_light_selection_declared:
        Established<RenderDirectionalLightSelectionDeclared>,
}

impl ProvableFrom<RenderDirectionalLightSelectionEvidence>
    for RenderDirectionalLightSelectionValid
{
}

/// Evidence that one scene-environment IBL selection is valid.
pub struct RenderImageBasedLightingSelectionEvidence {
    /// Proof that the image-based-lighting selection has been explicitly established.
    pub render_image_based_lighting_selection_declared:
        Established<RenderImageBasedLightingSelectionDeclared>,
}

impl ProvableFrom<RenderImageBasedLightingSelectionEvidence>
    for RenderImageBasedLightingSelectionValid
{
}

/// Evidence that one scene-environment atmosphere selection is valid.
pub struct RenderAtmosphereSelectionEvidence {
    /// Proof that the atmosphere selection has been explicitly established.
    pub render_atmosphere_selection_declared: Established<RenderAtmosphereSelectionDeclared>,
}

impl ProvableFrom<RenderAtmosphereSelectionEvidence> for RenderAtmosphereSelectionValid {}

/// Evidence that one scene-environment fog selection is valid.
pub struct RenderFogSelectionEvidence {
    /// Proof that the fog selection has been explicitly established.
    pub render_fog_selection_declared: Established<RenderFogSelectionDeclared>,
}

impl ProvableFrom<RenderFogSelectionEvidence> for RenderFogSelectionValid {}

/// Evidence that one scene-environment volumetric-fog selection is valid.
pub struct RenderVolumetricFogSelectionEvidence {
    /// Proof that the volumetric-fog selection has been explicitly established.
    pub render_volumetric_fog_selection_declared: Established<RenderVolumetricFogSelectionDeclared>,
}

impl ProvableFrom<RenderVolumetricFogSelectionEvidence> for RenderVolumetricFogSelectionValid {}

/// Explicit environment choice carried into [`crate::GisRenderSceneFactory::build_render_scene`].
///
/// Every call must choose a branch — there is no implicit default.  The
/// `Present` variant bundles the descriptor together with the proof token
/// returned by
/// [`crate::GisRenderEnvironmentFactory::build_render_scene_environment`],
/// keeping validated data and its proof inseparable at the call site.
#[derive(Debug, Clone)]
pub enum RenderSceneEnvironmentSelection {
    /// Explicitly no environment for this scene.
    Absent,
    /// A validated environment descriptor and its associated proof.
    Present(
        Box<RenderSceneEnvironmentDescriptor>,
        Established<RenderSceneEnvironmentValid>,
    ),
}

/// Evidence that the scene environment was explicitly declared absent.
///
/// This is a structural proof — no data is required; the implementor's
/// choice of the `Absent` branch is itself the evidence.
pub struct RenderSceneEnvironmentAbsentEvidence;

impl ProvableFrom<RenderSceneEnvironmentAbsentEvidence> for RenderSceneEnvironmentSelectionValid {}

/// Evidence that a validated environment was explicitly selected for the scene.
pub struct RenderSceneEnvironmentPresentEvidence {
    /// Proof that the scene environment descriptor has been validated.
    pub environment_valid: Established<RenderSceneEnvironmentValid>,
}

impl ProvableFrom<RenderSceneEnvironmentPresentEvidence> for RenderSceneEnvironmentSelectionValid {}

/// Evidence that an explicit annotation is renderable.
pub struct RenderAnnotationEvidence {
    /// Proof that the annotation identifier is non-empty.
    pub annotation_id_non_empty: Established<AnnotationIdNonEmpty>,
    /// Proof that the anchor coordinates are finite.
    pub annotation_anchor_finite: Established<AnnotationAnchorFinite>,
    /// Proof that the scale-range selection has been explicitly established.
    pub annotation_scale_range_selection: Established<AnnotationScaleRangeSelectionValid>,
    /// Proof that the annotation symbolizer is renderable.
    pub symbolizer: Established<RenderFeatureSymbolizerValid>,
}

impl ProvableFrom<RenderAnnotationEvidence> for RenderAnnotationValid {}

/// Evidence that one annotation carries a valid scale-range selection.
pub struct AnnotationScaleRangeSelectionEvidence {
    /// Proof that annotation scale-range selection has been explicitly established.
    pub annotation_scale_range_selection_declared:
        Established<AnnotationScaleRangeSelectionDeclared>,
}

impl ProvableFrom<AnnotationScaleRangeSelectionEvidence> for AnnotationScaleRangeSelectionValid {}

/// Evidence that a render view descriptor is valid.
pub struct RenderViewEvidence {
    /// Proof that the view identifier is non-empty.
    pub render_view_id_non_empty: Established<RenderViewIdNonEmpty>,
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
    /// Proof that perspective-projection policy has been validated.
    pub perspective_projection_policy: Established<PerspectiveProjectionPolicyValid>,
    /// Proof that camera-output selection has been validated.
    pub output_selection: Established<RenderViewOutputSelectionValid>,
    /// Proof that world-space selection has been validated.
    pub world_space_selection: Established<RenderWorldSpaceSelectionValid>,
}

impl ProvableFrom<RenderViewEvidence> for RenderViewValid {}

/// Evidence that a fog descriptor is renderable.
pub struct RenderFogEvidence {
    /// Proof that the fog falloff model is explicitly declared.
    pub fog_falloff_declared: Established<FogFalloffDeclared>,
    /// Proof that the fog linear-range selection has been explicitly established.
    pub fog_range_selection: Established<FogRangeSelectionValid>,
    /// Proof that the directional-light-exponent selection has been explicitly established.
    pub fog_directional_light_exponent_selection:
        Established<FogDirectionalLightExponentSelectionValid>,
    /// Proof that the fog-density selection has been explicitly established.
    pub fog_density_selection: Established<FogDensitySelectionValid>,
    /// Proof that the atmospheric-fog-coefficients selection has been explicitly established.
    pub atmospheric_fog_coefficients_selection:
        Established<AtmosphericFogCoefficientsSelectionValid>,
}

impl ProvableFrom<RenderFogEvidence> for RenderFogValid {}

/// Evidence that one fog descriptor carries a valid linear-range selection.
pub struct FogRangeSelectionEvidence {
    /// Proof that fog linear-range selection has been explicitly established.
    pub fog_range_selection_declared: Established<FogRangeSelectionDeclared>,
}

impl ProvableFrom<FogRangeSelectionEvidence> for FogRangeSelectionValid {}

/// Evidence that one fog descriptor carries a valid directional-light-exponent selection.
pub struct FogDirectionalLightExponentSelectionEvidence {
    /// Proof that the fog directional-light-exponent selection has been explicitly established.
    pub fog_directional_light_exponent_selection_declared:
        Established<FogDirectionalLightExponentSelectionDeclared>,
}

impl ProvableFrom<FogDirectionalLightExponentSelectionEvidence>
    for FogDirectionalLightExponentSelectionValid
{
}

/// Evidence that one fog descriptor carries a valid density selection.
pub struct FogDensitySelectionEvidence {
    /// Proof that the fog density selection has been explicitly established.
    pub fog_density_selection_declared: Established<FogDensitySelectionDeclared>,
}

impl ProvableFrom<FogDensitySelectionEvidence> for FogDensitySelectionValid {}

/// Evidence that one fog descriptor carries valid atmospheric-fog-coefficients selection.
pub struct AtmosphericFogCoefficientsSelectionEvidence {
    /// Proof that the atmospheric-fog-coefficients selection has been explicitly established.
    pub atmospheric_fog_coefficients_selection_declared:
        Established<AtmosphericFogCoefficientsSelectionDeclared>,
}

impl ProvableFrom<AtmosphericFogCoefficientsSelectionEvidence>
    for AtmosphericFogCoefficientsSelectionValid
{
}

/// Evidence that a volumetric-fog descriptor is renderable.
pub struct RenderVolumetricFogEvidence {
    /// Proof that the volumetric-fog-parameters selection has been explicitly established.
    pub volumetric_fog_parameters_selection: Established<VolumetricFogParametersSelectionValid>,
    /// Proof that the volumetric-fog step-count selection has been explicitly established.
    pub volumetric_fog_step_count_selection: Established<VolumetricFogStepCountSelectionValid>,
}

impl ProvableFrom<RenderVolumetricFogEvidence> for RenderVolumetricFogValid {}

/// Evidence that one volumetric-fog descriptor carries a valid parameters selection.
pub struct VolumetricFogParametersSelectionEvidence {
    /// Proof that volumetric-fog parameters selection has been explicitly established.
    pub volumetric_fog_parameters_selection_declared:
        Established<VolumetricFogParametersSelectionDeclared>,
}

impl ProvableFrom<VolumetricFogParametersSelectionEvidence>
    for VolumetricFogParametersSelectionValid
{
}

/// Evidence that one volumetric-fog descriptor carries a valid step-count selection.
pub struct VolumetricFogStepCountSelectionEvidence {
    /// Proof that volumetric-fog step-count selection has been explicitly established.
    pub volumetric_fog_step_count_selection_declared:
        Established<VolumetricFogStepCountSelectionDeclared>,
}

impl ProvableFrom<VolumetricFogStepCountSelectionEvidence>
    for VolumetricFogStepCountSelectionValid
{
}

/// Evidence that a fog-volume descriptor is renderable.
pub struct RenderFogVolumeEvidence {
    /// Proof that the fog-volume-parameters selection has been explicitly established.
    pub fog_volume_parameters_selection: Established<FogVolumeParametersSelectionValid>,
    /// Proof that the fog-volume density-factor selection has been explicitly established.
    pub fog_volume_density_factor_selection: Established<FogVolumeDensityFactorSelectionValid>,
    /// Proof that the fog-volume density-texture-offset selection has been explicitly established.
    pub fog_volume_density_texture_offset_selection:
        Established<FogVolumeDensityTextureOffsetSelectionValid>,
    /// Proof that the fog-volume absorption selection has been explicitly established.
    pub fog_volume_absorption_selection: Established<FogVolumeAbsorptionSelectionValid>,
    /// Proof that the fog-volume scattering selection has been explicitly established.
    pub fog_volume_scattering_selection: Established<FogVolumeScatteringSelectionValid>,
    /// Proof that the fog-volume scattering-asymmetry selection has been explicitly established.
    pub fog_volume_scattering_asymmetry_selection:
        Established<FogVolumeScatteringAsymmetrySelectionValid>,
    /// Proof that the fog-volume light-intensity selection has been explicitly established.
    pub fog_volume_light_intensity_selection: Established<FogVolumeLightIntensitySelectionValid>,
    /// Proof that the fog-volume density-texture-asset selection has been explicitly established.
    pub fog_volume_density_texture_asset_selection:
        Established<FogVolumeDensityTextureSelectionValid>,
}

impl ProvableFrom<RenderFogVolumeEvidence> for RenderFogVolumeValid {}

/// Evidence that one fog-volume descriptor carries a valid parameters selection.
pub struct FogVolumeParametersSelectionEvidence {
    /// Proof that fog-volume parameters selection has been explicitly established.
    pub fog_volume_parameters_selection_declared: Established<FogVolumeParametersSelectionDeclared>,
}

impl ProvableFrom<FogVolumeParametersSelectionEvidence> for FogVolumeParametersSelectionValid {}

/// Evidence that one fog-volume descriptor carries a valid density-factor selection.
pub struct FogVolumeDensityFactorSelectionEvidence {
    /// Proof that fog-volume density-factor selection has been explicitly established.
    pub fog_volume_density_factor_selection_declared:
        Established<FogVolumeDensityFactorSelectionDeclared>,
}

impl ProvableFrom<FogVolumeDensityFactorSelectionEvidence>
    for FogVolumeDensityFactorSelectionValid
{
}

/// Evidence that one fog-volume descriptor carries a valid density-texture-offset selection.
pub struct FogVolumeDensityTextureOffsetSelectionEvidence {
    /// Proof that fog-volume density-texture-offset selection has been explicitly established.
    pub fog_volume_density_texture_offset_selection_declared:
        Established<FogVolumeDensityTextureOffsetSelectionDeclared>,
}

impl ProvableFrom<FogVolumeDensityTextureOffsetSelectionEvidence>
    for FogVolumeDensityTextureOffsetSelectionValid
{
}

/// Evidence that one fog-volume descriptor carries a valid absorption selection.
pub struct FogVolumeAbsorptionSelectionEvidence {
    /// Proof that fog-volume absorption selection has been explicitly established.
    pub fog_volume_absorption_selection_declared: Established<FogVolumeAbsorptionSelectionDeclared>,
}

impl ProvableFrom<FogVolumeAbsorptionSelectionEvidence> for FogVolumeAbsorptionSelectionValid {}

/// Evidence that one fog-volume descriptor carries a valid scattering selection.
pub struct FogVolumeScatteringSelectionEvidence {
    /// Proof that fog-volume scattering selection has been explicitly established.
    pub fog_volume_scattering_selection_declared: Established<FogVolumeScatteringSelectionDeclared>,
}

impl ProvableFrom<FogVolumeScatteringSelectionEvidence> for FogVolumeScatteringSelectionValid {}

/// Evidence that one fog-volume descriptor carries a valid scattering-asymmetry selection.
pub struct FogVolumeScatteringAsymmetrySelectionEvidence {
    /// Proof that fog-volume scattering-asymmetry selection has been explicitly established.
    pub fog_volume_scattering_asymmetry_selection_declared:
        Established<FogVolumeScatteringAsymmetrySelectionDeclared>,
}

impl ProvableFrom<FogVolumeScatteringAsymmetrySelectionEvidence>
    for FogVolumeScatteringAsymmetrySelectionValid
{
}

/// Evidence that one fog-volume descriptor carries a valid light-intensity selection.
pub struct FogVolumeLightIntensitySelectionEvidence {
    /// Proof that fog-volume light-intensity selection has been explicitly established.
    pub fog_volume_light_intensity_selection_declared:
        Established<FogVolumeLightIntensitySelectionDeclared>,
}

impl ProvableFrom<FogVolumeLightIntensitySelectionEvidence>
    for FogVolumeLightIntensitySelectionValid
{
}

/// Evidence that one fog-volume descriptor carries a valid density-texture-asset selection.
pub struct FogVolumeDensityTextureAssetSelectionEvidence {
    /// Proof that fog-volume density-texture-asset selection has been explicitly established.
    pub fog_volume_density_texture_asset_selection_declared:
        Established<FogVolumeDensityTextureSelectionDeclared>,
}

impl ProvableFrom<FogVolumeDensityTextureAssetSelectionEvidence>
    for FogVolumeDensityTextureSelectionValid
{
}

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
    pub irradiance_volume_voxels_asset: Established<IrradianceVolumeVoxelsAssetValid>,
    /// Proof that the irradiance-volume intensity selection has been explicitly established.
    pub irradiance_volume_intensity_selection: Established<IrradianceVolumeIntensitySelectionValid>,
}

impl ProvableFrom<RenderIrradianceVolumeEvidence> for RenderIrradianceVolumeValid {}

/// Evidence that one irradiance-volume descriptor carries a valid intensity selection.
pub struct IrradianceVolumeIntensitySelectionEvidence {
    /// Proof that irradiance-volume intensity selection has been explicitly established.
    pub irradiance_volume_intensity_selection_declared:
        Established<IrradianceVolumeIntensitySelectionDeclared>,
}

impl ProvableFrom<IrradianceVolumeIntensitySelectionEvidence>
    for IrradianceVolumeIntensitySelectionValid
{
}

/// Evidence that a localized light probe is renderable.
pub struct RenderLightProbeEvidence {
    /// Proof that the probe region is valid.
    pub region: Established<RenderProbeRegionValid>,
    /// Proof that the irradiance-volume selection has been explicitly established.
    pub irradiance_volume_selection: Established<RenderLightProbeIrradianceVolumeSelectionValid>,
}

impl ProvableFrom<RenderLightProbeEvidence> for RenderLightProbeValid {}

/// Evidence that one light-probe descriptor carries a valid irradiance-volume selection.
pub struct LightProbeIrradianceVolumeSelectionEvidence {
    /// Proof that light-probe irradiance-volume selection has been explicitly established.
    pub light_probe_irradiance_volume_selection_declared:
        Established<RenderLightProbeIrradianceVolumeSelectionDeclared>,
}

impl ProvableFrom<LightProbeIrradianceVolumeSelectionEvidence>
    for RenderLightProbeIrradianceVolumeSelectionValid
{
}

/// Evidence that an image-based-lighting descriptor is renderable.
pub struct RenderImageBasedLightingEvidence {
    /// Proof that the image-based-lighting source is explicitly declared.
    pub image_based_lighting_source_declared: Established<ImageBasedLightingSourceDeclared>,
    /// Proof that the IBL intensity selection has been explicitly established.
    pub image_based_lighting_intensity_selection:
        Established<ImageBasedLightingIntensitySelectionValid>,
    /// Proof that the IBL rotation selection has been explicitly established.
    pub image_based_lighting_rotation_selection:
        Established<ImageBasedLightingRotationSelectionValid>,
    /// Proof that the IBL diffuse-map selection has been explicitly established.
    pub image_based_lighting_diffuse_map_selection:
        Established<ImageBasedLightingDiffuseMapSelectionValid>,
    /// Proof that the IBL specular-map selection has been explicitly established.
    pub image_based_lighting_specular_map_selection:
        Established<ImageBasedLightingSpecularMapSelectionValid>,
    /// Proof that the IBL environment-map selection has been explicitly established.
    pub image_based_lighting_environment_map_selection:
        Established<ImageBasedLightingEnvironmentMapSelectionValid>,
    /// Proof that the IBL atmosphere-cubemap-dimensions selection has been explicitly established.
    pub image_based_lighting_atmosphere_cubemap_dimensions_selection:
        Established<ImageBasedLightingAtmosphereCubemapDimensionsSelectionValid>,
}

impl ProvableFrom<RenderImageBasedLightingEvidence> for RenderImageBasedLightingValid {}

/// Evidence that one IBL descriptor carries a valid intensity selection.
pub struct ImageBasedLightingIntensitySelectionEvidence {
    /// Proof that IBL intensity selection has been explicitly established.
    pub image_based_lighting_intensity_selection_declared:
        Established<ImageBasedLightingIntensitySelectionDeclared>,
}

impl ProvableFrom<ImageBasedLightingIntensitySelectionEvidence>
    for ImageBasedLightingIntensitySelectionValid
{
}

/// Evidence that one IBL descriptor carries a valid rotation selection.
pub struct ImageBasedLightingRotationSelectionEvidence {
    /// Proof that IBL rotation selection has been explicitly established.
    pub image_based_lighting_rotation_selection_declared:
        Established<ImageBasedLightingRotationSelectionDeclared>,
}

impl ProvableFrom<ImageBasedLightingRotationSelectionEvidence>
    for ImageBasedLightingRotationSelectionValid
{
}

/// Evidence that one IBL descriptor carries a valid diffuse-map selection.
pub struct ImageBasedLightingDiffuseMapSelectionEvidence {
    /// Proof that IBL diffuse-map selection has been explicitly established.
    pub image_based_lighting_diffuse_map_selection_declared:
        Established<ImageBasedLightingDiffuseMapSelectionDeclared>,
}

impl ProvableFrom<ImageBasedLightingDiffuseMapSelectionEvidence>
    for ImageBasedLightingDiffuseMapSelectionValid
{
}

/// Evidence that one IBL descriptor carries a valid specular-map selection.
pub struct ImageBasedLightingSpecularMapSelectionEvidence {
    /// Proof that IBL specular-map selection has been explicitly established.
    pub image_based_lighting_specular_map_selection_declared:
        Established<ImageBasedLightingSpecularMapSelectionDeclared>,
}

impl ProvableFrom<ImageBasedLightingSpecularMapSelectionEvidence>
    for ImageBasedLightingSpecularMapSelectionValid
{
}

/// Evidence that one IBL descriptor carries a valid environment-map selection.
pub struct ImageBasedLightingEnvironmentMapSelectionEvidence {
    /// Proof that IBL environment-map selection has been explicitly established.
    pub image_based_lighting_environment_map_selection_declared:
        Established<ImageBasedLightingEnvironmentMapSelectionDeclared>,
}

impl ProvableFrom<ImageBasedLightingEnvironmentMapSelectionEvidence>
    for ImageBasedLightingEnvironmentMapSelectionValid
{
}

/// Evidence that one IBL descriptor carries a valid atmosphere-cubemap-dimensions selection.
pub struct ImageBasedLightingAtmosphereCubemapDimensionsSelectionEvidence {
    /// Proof that IBL atmosphere-cubemap-dimensions selection has been explicitly established.
    pub image_based_lighting_atmosphere_cubemap_dimensions_selection_declared:
        Established<ImageBasedLightingAtmosphereCubemapDimensionsSelectionDeclared>,
}

impl ProvableFrom<ImageBasedLightingAtmosphereCubemapDimensionsSelectionEvidence>
    for ImageBasedLightingAtmosphereCubemapDimensionsSelectionValid
{
}

/// Evidence that an atmosphere falloff descriptor is renderable.
pub struct RenderAtmosphereFalloffEvidence {
    /// Proof that the falloff family is explicitly declared.
    pub atmosphere_falloff_declared: Established<AtmosphereFalloffDeclared>,
    /// Proof that the atmosphere-falloff scale selection has been explicitly established.
    pub atmosphere_falloff_scale_selection: Established<AtmosphereFalloffScaleSelectionValid>,
    /// Proof that the atmosphere-falloff center selection has been explicitly established.
    pub atmosphere_falloff_center_selection: Established<AtmosphereFalloffCenterSelectionValid>,
    /// Proof that the atmosphere-falloff width selection has been explicitly established.
    pub atmosphere_falloff_width_selection: Established<AtmosphereFalloffWidthSelectionValid>,
}

impl ProvableFrom<RenderAtmosphereFalloffEvidence> for RenderAtmosphereFalloffValid {}

/// Evidence that one atmosphere-falloff descriptor carries a valid scale selection.
pub struct AtmosphereFalloffScaleSelectionEvidence {
    /// Proof that atmosphere-falloff scale selection has been explicitly established.
    pub atmosphere_falloff_scale_selection_declared:
        Established<AtmosphereFalloffScaleSelectionDeclared>,
}

impl ProvableFrom<AtmosphereFalloffScaleSelectionEvidence>
    for AtmosphereFalloffScaleSelectionValid
{
}

/// Evidence that one atmosphere-falloff descriptor carries a valid center selection.
pub struct AtmosphereFalloffCenterSelectionEvidence {
    /// Proof that atmosphere-falloff center selection has been explicitly established.
    pub atmosphere_falloff_center_selection_declared:
        Established<AtmosphereFalloffCenterSelectionDeclared>,
}

impl ProvableFrom<AtmosphereFalloffCenterSelectionEvidence>
    for AtmosphereFalloffCenterSelectionValid
{
}

/// Evidence that one atmosphere-falloff descriptor carries a valid width selection.
pub struct AtmosphereFalloffWidthSelectionEvidence {
    /// Proof that atmosphere-falloff width selection has been explicitly established.
    pub atmosphere_falloff_width_selection_declared:
        Established<AtmosphereFalloffWidthSelectionDeclared>,
}

impl ProvableFrom<AtmosphereFalloffWidthSelectionEvidence>
    for AtmosphereFalloffWidthSelectionValid
{
}

/// Evidence that an atmosphere phase-function descriptor is renderable.
pub struct RenderAtmospherePhaseFunctionEvidence {
    /// Proof that the phase-function family is explicitly declared.
    pub atmosphere_phase_function_declared: Established<AtmospherePhaseFunctionDeclared>,
    /// Proof that the atmosphere-phase-function asymmetry selection has been explicitly established.
    pub atmosphere_phase_function_asymmetry_selection:
        Established<AtmospherePhaseFunctionAsymmetrySelectionValid>,
}

impl ProvableFrom<RenderAtmospherePhaseFunctionEvidence> for RenderAtmospherePhaseFunctionValid {}

/// Evidence that one atmosphere-phase-function descriptor carries a valid asymmetry selection.
pub struct AtmospherePhaseFunctionAsymmetrySelectionEvidence {
    /// Proof that atmosphere-phase-function asymmetry selection has been explicitly established.
    pub atmosphere_phase_function_asymmetry_selection_declared:
        Established<AtmospherePhaseFunctionAsymmetrySelectionDeclared>,
}

impl ProvableFrom<AtmospherePhaseFunctionAsymmetrySelectionEvidence>
    for AtmospherePhaseFunctionAsymmetrySelectionValid
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

impl ProvableFrom<RenderAtmosphereScatteringTermEvidence> for RenderAtmosphereScatteringTermValid {}

/// Evidence that a visible solar-disk descriptor is renderable.
pub struct RenderSunDiskEvidence {
    /// Proof that the sun-disk angular-size selection has been explicitly established.
    pub sun_disk_angular_size_selection: Established<SunDiskAngularSizeSelectionValid>,
    /// Proof that the sun-disk intensity selection has been explicitly established.
    pub sun_disk_intensity_selection: Established<SunDiskIntensitySelectionValid>,
}

impl ProvableFrom<RenderSunDiskEvidence> for RenderSunDiskValid {}

/// Evidence that one sun-disk descriptor carries a valid angular-size selection.
pub struct SunDiskAngularSizeSelectionEvidence {
    /// Proof that sun-disk angular-size selection has been explicitly established.
    pub sun_disk_angular_size_selection_declared: Established<SunDiskAngularSizeSelectionDeclared>,
}

impl ProvableFrom<SunDiskAngularSizeSelectionEvidence> for SunDiskAngularSizeSelectionValid {}

/// Evidence that one sun-disk descriptor carries a valid intensity selection.
pub struct SunDiskIntensitySelectionEvidence {
    /// Proof that sun-disk intensity selection has been explicitly established.
    pub sun_disk_intensity_selection_declared: Established<SunDiskIntensitySelectionDeclared>,
}

impl ProvableFrom<SunDiskIntensitySelectionEvidence> for SunDiskIntensitySelectionValid {}

/// Evidence that a skybox descriptor is renderable.
pub struct RenderSkyboxEvidence {
    /// Proof that the skybox asset is renderable.
    pub asset: Established<RenderAssetReferenceValid>,
    /// Proof that skybox brightness is non-negative.
    pub skybox_brightness_non_negative: Established<SkyboxBrightnessNonNegative>,
    /// Proof that the skybox rotation selection has been explicitly established.
    pub skybox_rotation_selection: Established<SkyboxRotationSelectionValid>,
}

impl ProvableFrom<RenderSkyboxEvidence> for RenderSkyboxValid {}

/// Evidence that one skybox descriptor carries a valid rotation selection.
pub struct SkyboxRotationSelectionEvidence {
    /// Proof that skybox rotation selection has been explicitly established.
    pub skybox_rotation_selection_declared: Established<SkyboxRotationSelectionDeclared>,
}

impl ProvableFrom<SkyboxRotationSelectionEvidence> for SkyboxRotationSelectionValid {}

/// Evidence that an ambient-light descriptor is renderable.
pub struct RenderAmbientLightEvidence {
    /// Proof that ambient-light brightness is non-negative.
    pub ambient_light_brightness_non_negative: Established<AmbientLightBrightnessNonNegative>,
}

impl ProvableFrom<RenderAmbientLightEvidence> for RenderAmbientLightValid {}

/// Evidence that a cascade-shadow descriptor is renderable.
pub struct RenderCascadeShadowConfigEvidence {
    /// Proof that the cascade-shadow bounds selection has been explicitly established.
    pub cascade_shadow_bounds_selection: Established<CascadeShadowBoundsSelectionValid>,
    /// Proof that the cascade-shadow overlap selection has been explicitly established.
    pub cascade_shadow_overlap_selection: Established<CascadeShadowOverlapSelectionValid>,
    /// Proof that the cascade-shadow minimum-distance selection has been explicitly established.
    pub cascade_shadow_minimum_distance_selection:
        Established<CascadeShadowMinimumDistanceSelectionValid>,
    /// Proof that the cascade-shadow maximum-distance selection has been explicitly established.
    pub cascade_shadow_maximum_distance_selection:
        Established<CascadeShadowMaximumDistanceSelectionValid>,
    /// Proof that the first-cascade-far-bound selection has been explicitly established.
    pub cascade_shadow_first_cascade_far_bound_selection:
        Established<CascadeShadowFirstFarBoundSelectionValid>,
    /// Proof that the cascade-count selection has been explicitly established.
    pub cascade_shadow_cascade_count_selection:
        Established<CascadeShadowCascadeCountSelectionValid>,
}

impl ProvableFrom<RenderCascadeShadowConfigEvidence> for RenderCascadeShadowConfigValid {}

/// Evidence that one cascade-shadow descriptor carries a valid bounds selection.
pub struct CascadeShadowBoundsSelectionEvidence {
    /// Proof that cascade-shadow bounds selection has been explicitly established.
    pub cascade_shadow_bounds_selection_declared: Established<CascadeShadowBoundsSelectionDeclared>,
}

impl ProvableFrom<CascadeShadowBoundsSelectionEvidence> for CascadeShadowBoundsSelectionValid {}

/// Evidence that one cascade-shadow descriptor carries a valid overlap selection.
pub struct CascadeShadowOverlapSelectionEvidence {
    /// Proof that cascade-shadow overlap selection has been explicitly established.
    pub cascade_shadow_overlap_selection_declared:
        Established<CascadeShadowOverlapSelectionDeclared>,
}

impl ProvableFrom<CascadeShadowOverlapSelectionEvidence> for CascadeShadowOverlapSelectionValid {}

/// Evidence that one cascade-shadow descriptor carries a valid minimum-distance selection.
pub struct CascadeShadowMinimumDistanceSelectionEvidence {
    /// Proof that cascade-shadow minimum-distance selection has been explicitly established.
    pub cascade_shadow_minimum_distance_selection_declared:
        Established<CascadeShadowMinimumDistanceSelectionDeclared>,
}

impl ProvableFrom<CascadeShadowMinimumDistanceSelectionEvidence>
    for CascadeShadowMinimumDistanceSelectionValid
{
}

/// Evidence that one cascade-shadow descriptor carries a valid maximum-distance selection.
pub struct CascadeShadowMaximumDistanceSelectionEvidence {
    /// Proof that cascade-shadow maximum-distance selection has been explicitly established.
    pub cascade_shadow_maximum_distance_selection_declared:
        Established<CascadeShadowMaximumDistanceSelectionDeclared>,
}

impl ProvableFrom<CascadeShadowMaximumDistanceSelectionEvidence>
    for CascadeShadowMaximumDistanceSelectionValid
{
}

/// Evidence that one cascade-shadow descriptor carries a valid first-cascade-far-bound selection.
pub struct CascadeShadowFirstCascadeFarBoundSelectionEvidence {
    /// Proof that cascade-shadow first-cascade-far-bound selection has been explicitly established.
    pub cascade_shadow_first_cascade_far_bound_selection_declared:
        Established<CascadeShadowFirstFarBoundSelectionDeclared>,
}

impl ProvableFrom<CascadeShadowFirstCascadeFarBoundSelectionEvidence>
    for CascadeShadowFirstFarBoundSelectionValid
{
}

/// Evidence that one cascade-shadow descriptor carries a valid cascade-count selection.
pub struct CascadeShadowCascadeCountSelectionEvidence {
    /// Proof that cascade-shadow cascade-count selection has been explicitly established.
    pub cascade_shadow_cascade_count_selection_declared:
        Established<CascadeShadowCascadeCountSelectionDeclared>,
}

impl ProvableFrom<CascadeShadowCascadeCountSelectionEvidence>
    for CascadeShadowCascadeCountSelectionValid
{
}

/// Evidence that a directional-light descriptor is renderable.
pub struct RenderDirectionalLightEvidence {
    /// Proof that directional-light illuminance is non-negative.
    pub directional_light_illuminance_non_negative:
        Established<DirectionalLightIlluminanceNonNegative>,
    /// Proof that directional-light angles are finite.
    pub light_angles_finite: Established<LightAnglesFinite>,
    /// Proof that the directional-light cascade-shadows selection has been explicitly established.
    pub cascade_shadows_selection: Established<DirectionalLightCascadeShadowsSelectionValid>,
}

impl ProvableFrom<RenderDirectionalLightEvidence> for RenderDirectionalLightValid {}

/// Evidence that one directional-light descriptor carries a valid cascade-shadows selection.
pub struct DirectionalLightCascadeShadowsSelectionEvidence {
    /// Proof that directional-light cascade-shadows selection has been explicitly established.
    pub directional_light_cascade_shadows_selection_declared:
        Established<DirectionalLightCascadeShadowsSelectionDeclared>,
}

impl ProvableFrom<DirectionalLightCascadeShadowsSelectionEvidence>
    for DirectionalLightCascadeShadowsSelectionValid
{
}

/// Evidence that an atmosphere descriptor is renderable.
pub struct RenderAtmosphereEvidence {
    /// Proof that the atmosphere shell radii are strictly positive.
    pub atmosphere_shell_radii_positive: Established<AtmosphereShellRadiiPositive>,
    /// Proof that the atmosphere shell radii are strictly ordered.
    pub atmosphere_shell_radii_ordered: Established<AtmosphereShellRadiiOrdered>,
    /// Proof that ground-albedo values are finite.
    pub atmosphere_ground_albedo_finite: Established<AtmosphereGroundAlbedoFinite>,
    /// Proof that ground-albedo values stay in the unit interval.
    pub atmosphere_ground_albedo_unit_interval: Established<AtmosphereGroundAlbedoUnitInterval>,
    /// Proof that precomputation resolutions are strictly positive.
    pub atmosphere_scattering_resolutions_positive:
        Established<AtmosphereScatteringResolutionsPositive>,
    /// Proof that at least one scattering term is present.
    pub atmosphere_scattering_terms_present: Established<AtmosphereScatteringTermsPresent>,
    /// Proof that the atmosphere density-multiplier selection has been explicitly established.
    pub atmosphere_density_multiplier_selection:
        Established<AtmosphereDensityMultiplierSelectionValid>,
    /// Proof that the visible solar-disk selection is valid.
    pub sun_disk_selection: Established<RenderSunDiskSelectionValid>,
    /// Proofs that each scattering term is renderable.
    pub terms: Vec<Established<RenderAtmosphereScatteringTermValid>>,
}

impl ProvableFrom<RenderAtmosphereEvidence> for RenderAtmosphereValid {}

/// Evidence that one atmosphere descriptor carries a valid density-multiplier selection.
pub struct AtmosphereDensityMultiplierSelectionEvidence {
    /// Proof that atmosphere density-multiplier selection has been explicitly established.
    pub atmosphere_density_multiplier_selection_declared:
        Established<AtmosphereDensityMultiplierSelectionDeclared>,
}

impl ProvableFrom<AtmosphereDensityMultiplierSelectionEvidence>
    for AtmosphereDensityMultiplierSelectionValid
{
}

/// Evidence that a scene environment descriptor is renderable.
pub struct RenderSceneEnvironmentEvidence {
    /// Proof that the skybox selection is valid.
    pub skybox_selection: Established<RenderSkyboxSelectionValid>,
    /// Proof that the ambient-light selection is valid.
    pub ambient_light_selection: Established<RenderAmbientLightSelectionValid>,
    /// Proof that the directional-light selection is valid.
    pub directional_light_selection: Established<RenderDirectionalLightSelectionValid>,
    /// Proof that image-based-lighting selection is valid.
    pub image_based_lighting_selection: Established<RenderImageBasedLightingSelectionValid>,
    /// Proof that atmosphere selection is valid.
    pub atmosphere_selection: Established<RenderAtmosphereSelectionValid>,
    /// Proofs that localized light probes are renderable.
    pub light_probes: Vec<Established<RenderLightProbeValid>>,
    /// Proof that fog selection is valid.
    pub fog_selection: Established<RenderFogSelectionValid>,
    /// Proof that volumetric-fog selection is valid.
    pub volumetric_fog_selection: Established<RenderVolumetricFogSelectionValid>,
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
    /// Proof that per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
    /// Proof that vector payload was provided.
    pub vector_payload_declared: Established<VectorPayloadDeclared>,
    /// Proof that payload geometry has been projected for the active view.
    pub geometry_projected_for_view: Established<GeometryProjectedForView>,
    /// Proof that vector symbolization is fully resolved.
    pub vector_symbolization_resolved: Established<VectorSymbolizationResolved>,
    /// Proof that the chosen style is compatible with the payload family.
    pub vector_style_payload_compatible: Established<VectorStylePayloadCompatible>,
    /// Proof that batching selection is valid for the full layer shape.
    pub batching_selection: Established<RenderFeatureBatchingSelectionValid>,
    /// Proof that the style descriptor is valid.
    pub style: Established<RenderFeatureStyleValid>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableVectorLayerEvidence> for RenderableVectorLayerValid {}

/// Evidence that one vector layer carries a valid batching selection.
pub struct RenderFeatureBatchingSelectionEvidence {
    /// Proof that batching selection has been explicitly established.
    pub render_feature_batching_selection_declared:
        Established<RenderFeatureBatchingSelectionDeclared>,
}

impl ProvableFrom<RenderFeatureBatchingSelectionEvidence> for RenderFeatureBatchingSelectionValid {}

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
    /// Proof that per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
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
    /// Proof that per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
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
    /// Proof that per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
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
    /// Proof that per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
    /// Proof that annotation payload was provided.
    pub annotation_payload_declared: Established<AnnotationPayloadDeclared>,
    /// Proof that each annotation entry is renderable.
    pub annotations: Vec<Established<RenderAnnotationValid>>,
    /// Proof that the view descriptor is valid.
    pub view: Established<RenderViewValid>,
}

impl ProvableFrom<RenderableAnnotationLayerEvidence> for RenderableAnnotationLayerValid {}

/// Evidence that a 3D model layer is renderable.
pub struct RenderableModelLayerEvidence {
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
    /// Proof that per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
    /// Proof that the model asset reference is non-empty.
    pub model_asset_declared: Established<ModelAssetDeclared>,
    /// Proof that the model placement descriptor carries valid coordinates.
    pub model_placement_valid: Established<ModelPlacementValid>,
    /// Sidecar proof for the optional shadow-participation descriptor.
    pub shadow_participation_valid: Option<Established<RenderShadowParticipationValid>>,
    /// Sidecar proof for the optional material-intent override descriptor.
    pub material_override_valid: Option<Established<RenderMaterialIntentValid>>,
}

impl ProvableFrom<RenderableModelLayerEvidence> for RenderableModelLayerValid {}

/// Evidence that a layer group descriptor is valid.
pub struct RenderableLayerGroupEvidence {
    /// Proof that the group identifier is non-empty.
    pub group_id_non_empty: Established<LayerGroupIdNonEmpty>,
    /// Proof that the group name is non-empty.
    pub group_name_non_empty: Established<LayerGroupNameNonEmpty>,
    /// Proof that group opacity is in the unit interval.
    pub group_opacity_unit_interval: Established<LayerGroupOpacityUnitInterval>,
}

impl ProvableFrom<RenderableLayerGroupEvidence> for RenderableLayerGroupValid {}

/// Evidence that a full render scene is valid.
pub struct RenderableSceneEvidence {
    /// Proof that the scene identifier is non-empty.
    pub scene_id_non_empty: Established<SceneIdNonEmpty>,
    /// Proof that the scene declares its layer list.
    pub scene_layer_list_declared: Established<SceneLayerListDeclared>,
    /// Proof that the scene declares its view.
    pub scene_view_declared: Established<SceneViewDeclared>,
    /// Proof that the scene declares its auxiliary views.
    pub scene_auxiliary_views_declared: Established<SceneAuxiliaryViewsDeclared>,
    /// Proof that auxiliary view identifiers are unique.
    pub scene_auxiliary_view_ids_unique: Established<SceneAuxiliaryViewIdsUnique>,
    /// Proof that auxiliary view identifiers differ from the primary view identifier.
    pub scene_auxiliary_views_distinct_from_primary:
        Established<SceneAuxiliaryViewsDistinctFromPrimary>,
    /// Proof that every layer view-routing reference resolves against scene views.
    pub scene_layer_view_participation_resolved: Established<SceneLayerViewParticipationResolved>,
    /// Proof that scene layer ordering is deterministic.
    pub layer_order_deterministic: Established<LayerOrderDeterministic>,
    /// Proof that the scene view descriptor is valid.
    pub view: Established<RenderViewValid>,
    /// Proof that every auxiliary scene view descriptor is valid.
    pub auxiliary_views: Vec<Established<RenderViewValid>>,
    /// Proof that scene-level environment selection is valid.
    pub environment_selection: Established<RenderSceneEnvironmentSelectionValid>,
    /// Proof that every layer in the scene is renderable.
    pub layers: Vec<Established<RenderableLayerValid>>,
}

impl ProvableFrom<RenderableSceneEvidence> for RenderableSceneValid {}

/// Evidence that one scene-update target-layer selection is valid.
pub struct SceneUpdateTargetLayerSelectionEvidence {
    /// Proof that the target-layer selection has been explicitly established.
    pub scene_update_target_layer_selection_declared:
        Established<SceneUpdateTargetLayerSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateTargetLayerSelectionEvidence>
    for SceneUpdateTargetLayerSelectionValid
{
}

/// Evidence that one scene-update target-view selection is valid.
pub struct SceneUpdateTargetViewSelectionEvidence {
    /// Proof that the target-view selection has been explicitly established.
    pub scene_update_target_view_selection_declared:
        Established<SceneUpdateTargetViewSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateTargetViewSelectionEvidence> for SceneUpdateTargetViewSelectionValid {}

/// Evidence that one scene-update reference-layer selection is valid.
pub struct SceneUpdateReferenceLayerSelectionEvidence {
    /// Proof that the reference-layer selection has been explicitly established.
    pub scene_update_reference_layer_selection_declared:
        Established<SceneUpdateReferenceLayerSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateReferenceLayerSelectionEvidence>
    for SceneUpdateReferenceLayerSelectionValid
{
}

/// Evidence that one scene-update view-payload selection is valid.
pub struct SceneUpdateViewSelectionEvidence {
    /// Proof that the view-payload selection has been explicitly established.
    pub scene_update_view_selection_declared: Established<SceneUpdateViewSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateViewSelectionEvidence> for SceneUpdateViewSelectionValid {}

/// Evidence that one scene-update environment-payload selection is valid.
pub struct SceneUpdateEnvironmentSelectionEvidence {
    /// Proof that the environment-payload selection has been explicitly established.
    pub scene_update_environment_selection_declared:
        Established<SceneUpdateEnvironmentSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateEnvironmentSelectionEvidence>
    for SceneUpdateEnvironmentSelectionValid
{
}

/// Evidence that one scene-update layer-payload selection is valid.
pub struct SceneUpdateLayerSelectionEvidence {
    /// Proof that the layer-payload selection has been explicitly established.
    pub scene_update_layer_selection_declared: Established<SceneUpdateLayerSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateLayerSelectionEvidence> for SceneUpdateLayerSelectionValid {}

/// Evidence that one scene-update opacity selection is valid.
pub struct SceneUpdateOpacitySelectionEvidence {
    /// Proof that the opacity selection has been explicitly established.
    pub scene_update_opacity_selection_declared: Established<SceneUpdateOpacitySelectionDeclared>,
}

impl ProvableFrom<SceneUpdateOpacitySelectionEvidence> for SceneUpdateOpacitySelectionValid {}

/// Evidence that one scene-update draw-order selection is valid.
pub struct SceneUpdateDrawOrderSelectionEvidence {
    /// Proof that the draw-order selection has been explicitly established.
    pub scene_update_draw_order_selection_declared:
        Established<SceneUpdateDrawOrderSelectionDeclared>,
}

impl ProvableFrom<SceneUpdateDrawOrderSelectionEvidence> for SceneUpdateDrawOrderSelectionValid {}

/// Evidence that a scene update is renderable.
pub struct RenderableSceneUpdateEvidence {
    /// Proof that the scene identifier is non-empty.
    pub scene_id_non_empty: Established<SceneIdNonEmpty>,
    /// Proof that the update operation is explicitly declared.
    pub scene_update_operation_declared: Established<SceneUpdateOperationDeclared>,
    /// Proof that target-layer selection is valid.
    pub target_layer_selection: Established<SceneUpdateTargetLayerSelectionValid>,
    /// Proof that target-view selection is valid.
    pub target_view_selection: Established<SceneUpdateTargetViewSelectionValid>,
    /// Proof that reference-layer selection is valid.
    pub reference_layer_selection: Established<SceneUpdateReferenceLayerSelectionValid>,
    /// Proof that view-payload selection is valid.
    pub view_selection: Established<SceneUpdateViewSelectionValid>,
    /// Proof that environment-payload selection is valid.
    pub environment_selection: Established<SceneUpdateEnvironmentSelectionValid>,
    /// Proof that layer-payload selection is valid.
    pub layer_selection: Established<SceneUpdateLayerSelectionValid>,
    /// Proof that opacity-payload selection is valid.
    pub opacity_selection: Established<SceneUpdateOpacitySelectionValid>,
    /// Proof that draw-order-payload selection is valid.
    pub draw_order_selection: Established<SceneUpdateDrawOrderSelectionValid>,
}

impl ProvableFrom<RenderableSceneUpdateEvidence> for RenderableSceneUpdateValid {}

/// Evidence that one layer-routing scene update is renderable.
pub struct RenderableSceneLayerViewParticipationUpdateEvidence {
    /// Proof that the scene identifier is non-empty.
    pub scene_id_non_empty: Established<SceneIdNonEmpty>,
    /// Proof that the update operation is explicitly declared.
    pub scene_update_operation_declared: Established<SceneUpdateOperationDeclared>,
    /// Proof that the target layer identifier is non-empty.
    pub target_layer_id_non_empty: Established<SceneUpdateTargetLayerNonEmpty>,
    /// Proof that replacement per-view routing is valid.
    pub view_participation: Established<RenderViewParticipationValid>,
}

impl ProvableFrom<RenderableSceneLayerViewParticipationUpdateEvidence>
    for RenderableSceneLayerViewParticipationUpdateValid
{
}

impl ProvableFrom<Established<RenderableSceneLayerViewParticipationUpdateValid>>
    for RenderableSceneUpdateValid
{
}

impl ProvableFrom<Established<RenderableVectorLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableRasterLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableTileLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableTerrainLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableAnnotationLayerValid>> for RenderableLayerValid {}
impl ProvableFrom<Established<RenderableModelLayerValid>> for RenderableLayerValid {}

// ── Descriptor → Valid: leaf factory credentials ──────────────────────────────
//
// Each descriptor type IS the proof that the corresponding proposition holds.
// Backend factory methods validate the descriptor, then call
// `Established::prove(&input)` to mint the proof token.  Having the descriptor
// in scope IS the credential; any validation beyond type construction is the
// factory method's audit surface.

impl ProvableFrom<RenderFeatureStyleDescriptor> for RenderFeatureStyleValid {}
impl ProvableFrom<RenderRasterStyleDescriptor> for RenderRasterStyleValid {}
impl ProvableFrom<RenderTileStyleDescriptor> for RenderTileStyleValid {}
impl ProvableFrom<RenderTerrainStyleDescriptor> for RenderTerrainStyleValid {}
impl ProvableFrom<RenderMaterialIntentDescriptor> for RenderMaterialIntentValid {}
impl ProvableFrom<RenderShadowParticipationDescriptor> for RenderShadowParticipationValid {}
impl ProvableFrom<RenderOutputTargetDescriptor> for RenderOutputTargetValid {}
impl ProvableFrom<RenderClearPolicyDescriptor> for RenderClearPolicyValid {}
impl ProvableFrom<RenderViewportDescriptor> for RenderViewportValid {}
impl ProvableFrom<RenderResolutionOverrideDescriptor> for RenderResolutionOverrideValid {}
impl ProvableFrom<RenderSubViewLayoutDescriptor> for RenderSubViewLayoutValid {}
impl ProvableFrom<RenderViewParticipationDescriptor> for RenderViewParticipationValid {}
impl ProvableFrom<RenderViewOutputDescriptor> for RenderViewOutputValid {}
impl ProvableFrom<RenderExposureDescriptor> for RenderExposureValid {}
impl ProvableFrom<RenderAutoExposureDescriptor> for RenderAutoExposureValid {}
impl ProvableFrom<RenderColorGradingDescriptor> for RenderColorGradingValid {}
impl ProvableFrom<RenderBloomDescriptor> for RenderBloomValid {}
impl ProvableFrom<RenderDepthOfFieldDescriptor> for RenderDepthOfFieldValid {}
impl ProvableFrom<RenderMotionBlurDescriptor> for RenderMotionBlurValid {}
impl ProvableFrom<RenderChromaticAberrationDescriptor> for RenderChromaticAberrationValid {}
impl ProvableFrom<RenderScreenSpaceReflectionsDescriptor> for RenderScreenSpaceReflectionsValid {}
impl ProvableFrom<RenderScreenSpaceAmbientOcclusionDescriptor>
    for RenderScreenSpaceAmbientOcclusionValid
{
}
impl ProvableFrom<RenderWorldSpaceDescriptor> for RenderWorldSpaceValid {}
impl ProvableFrom<RenderLightProbeDescriptor> for RenderLightProbeValid {}
impl ProvableFrom<RenderImageBasedLightingDescriptor> for RenderImageBasedLightingValid {}
impl ProvableFrom<RenderSkyboxDescriptor> for RenderSkyboxValid {}
impl ProvableFrom<RenderAmbientLightDescriptor> for RenderAmbientLightValid {}
impl ProvableFrom<RenderDirectionalLightDescriptor> for RenderDirectionalLightValid {}
impl ProvableFrom<RenderAtmosphereDescriptor> for RenderAtmosphereValid {}
impl ProvableFrom<RenderFogDescriptor> for RenderFogValid {}
impl ProvableFrom<RenderVolumetricFogDescriptor> for RenderVolumetricFogValid {}
impl ProvableFrom<RenderFogVolumeDescriptor> for RenderFogVolumeValid {}
impl ProvableFrom<RenderCascadeShadowConfigDescriptor> for RenderCascadeShadowConfigValid {}
impl ProvableFrom<RenderSceneEnvironmentDescriptor> for RenderSceneEnvironmentValid {}
impl ProvableFrom<RenderViewDescriptor> for RenderViewValid {}

// ── Payload-proof → selection-valid: update factory proof chain ───────────────
//
// When an update factory receives a validated payload proof from the caller, that
// proof is itself the evidence that the corresponding selection is valid.  These
// impls let the factory call `Established::prove(&received_proof_variable)` to
// derive the selection-validity token directly from the received variable.

impl ProvableFrom<Established<RenderSceneEnvironmentValid>>
    for SceneUpdateEnvironmentSelectionValid
{
}
impl ProvableFrom<Established<RenderViewValid>> for SceneUpdateViewSelectionValid {}
impl ProvableFrom<Established<RenderableLayerValid>> for SceneUpdateLayerSelectionValid {}
impl ProvableFrom<Established<LayerOpacityUnitInterval>> for SceneUpdateOpacitySelectionValid {}
impl ProvableFrom<Established<LayerDrawOrderAssigned>> for SceneUpdateDrawOrderSelectionValid {}
impl ProvableFrom<Established<RenderViewParticipationValid>>
    for RenderableSceneLayerViewParticipationUpdateValid
{
}

// ── Sidecar proof exchange: validated descriptor → sub-proposition ────────────
//
// Factory-built descriptors travel with a sidecar `Established<ValidType>` proof.
// Meta methods accept the sidecar and re-derive narrower sub-propositions via
// `prove(&sidecar)` instead of re-performing validation or minting from scratch.
//
// Each impl here is semantically justified by the evidence bundle that produced
// the parent valid-proof: the sub-proposition was a required field in that bundle.

// RenderableLayerValid evidence includes LayerOpacityUnitInterval in the spec proofs.
impl ProvableFrom<Established<RenderableLayerValid>> for LayerOpacityUnitInterval {}

// RenderableSceneUpdateValid evidence includes SceneUpdateOpacitySelectionValid,
// which was derived from LayerOpacityUnitInterval via the update factory.
impl ProvableFrom<Established<RenderableSceneUpdateValid>> for LayerOpacityUnitInterval {}

// RenderableSceneUpdateValid evidence includes SceneUpdateDrawOrderSelectionValid,
// which was derived from LayerDrawOrderAssigned via the update factory.
impl ProvableFrom<Established<RenderableSceneUpdateValid>> for LayerDrawOrderAssigned {}

// Update operations that carry a payload (environment/view/layer) had that
// payload's proof required by the factory.  The update valid proof therefore
// transitively certifies the sub-payload.  Meta methods use these impls to
// carry the sub-proof forward via `prove(&update_proof)`.
impl ProvableFrom<Established<RenderableSceneUpdateValid>> for RenderSceneEnvironmentValid {}
impl ProvableFrom<Established<RenderableSceneUpdateValid>> for RenderViewValid {}
impl ProvableFrom<Established<RenderableSceneUpdateValid>> for RenderableLayerValid {}

// RenderableSceneValid evidence contains layer_order_deterministic, so a valid
// scene implies deterministic order.  Meta methods accept the sidecar and
// re-derive the sub-proposition via `prove`.
impl ProvableFrom<Established<RenderableSceneValid>> for LayerOrderDeterministic {}
