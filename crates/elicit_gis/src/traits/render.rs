//! Renderer-agnostic GIS render traits.
//!
//! These traits define the consumer seam between validated geospatial payloads
//! and concrete rendering backends. The payload side uses upstream georust
//! types directly; the proof side lives entirely in `elicit_gis`.
//!
//! # Three-role taxonomy
//!
//! ## Role 1a — Leaf validators
//!
//! | Trait | Validates | Output proof |
//! |---|---|---|
//! | [`GisRenderFeatureStyleFactory`] | vector symbolization descriptors | `Established<RenderFeatureStyleValid>` |
//! | [`GisRenderRasterStyleFactory`] | raster symbolization descriptors | `Established<RenderRasterStyleValid>` |
//! | [`GisRenderTileStyleFactory`] | tile symbolization descriptors | `Established<RenderTileStyleValid>` |
//! | [`GisRenderTerrainStyleFactory`] | terrain symbolization descriptors | `Established<RenderTerrainStyleValid>` |
//! | [`GisRenderMaterialIntentFactory`] | material-family and texture-binding descriptors | `Established<RenderMaterialIntentValid>` |
//! | [`GisRenderShadowParticipationFactory`] | shadow-participation descriptors | `Established<RenderShadowParticipationValid>` |
//! | [`GisRenderViewOutputFactory`] | camera output and post-process descriptors | `Established<RenderViewOutputValid>` |
//! | [`GisRenderExposureFactory`] | fixed exposure descriptors | `Established<RenderExposureValid>` |
//! | [`GisRenderAutoExposureFactory`] | auto-exposure descriptors | `Established<RenderAutoExposureValid>` |
//! | [`GisRenderColorGradingFactory`] | color-grading descriptors | `Established<RenderColorGradingValid>` |
//! | [`GisRenderBloomFactory`] | bloom descriptors | `Established<RenderBloomValid>` |
//! | [`GisRenderDepthOfFieldFactory`] | depth-of-field descriptors | `Established<RenderDepthOfFieldValid>` |
//! | [`GisRenderMotionBlurFactory`] | motion-blur descriptors | `Established<RenderMotionBlurValid>` |
//! | [`GisRenderChromaticAberrationFactory`] | chromatic-aberration descriptors | `Established<RenderChromaticAberrationValid>` |
//! | [`GisRenderScreenSpaceReflectionsFactory`] | screen-space reflection descriptors | `Established<RenderScreenSpaceReflectionsValid>` |
//! | [`GisRenderScreenSpaceAmbientOcclusionFactory`] | screen-space ambient occlusion descriptors | `Established<RenderScreenSpaceAmbientOcclusionValid>` |
//! | [`GisRenderWorldSpaceFactory`] | world-space mapping descriptors | `Established<RenderWorldSpaceValid>` |
//! | [`GisRenderLightProbeFactory`] | localized light-probe descriptors | `Established<RenderLightProbeValid>` |
//! | [`GisRenderImageBasedLightingFactory`] | image-based lighting descriptors | `Established<RenderImageBasedLightingValid>` |
//! | [`GisRenderSkyboxFactory`] | skybox descriptors | `Established<RenderSkyboxValid>` |
//! | [`GisRenderAmbientLightFactory`] | ambient-light descriptors | `Established<RenderAmbientLightValid>` |
//! | [`GisRenderDirectionalLightFactory`] | directional-light descriptors | `Established<RenderDirectionalLightValid>` |
//! | [`GisRenderAtmosphereFactory`] | atmosphere descriptors | `Established<RenderAtmosphereValid>` |
//! | [`GisRenderFogFactory`] | distance/atmospheric fog descriptors | `Established<RenderFogValid>` |
//! | [`GisRenderVolumetricFogFactory`] | camera-based volumetric fog descriptors | `Established<RenderVolumetricFogValid>` |
//! | [`GisRenderFogVolumeFactory`] | localized fog-volume descriptors | `Established<RenderFogVolumeValid>` |
//! | [`GisRenderCascadeShadowFactory`] | cascade-shadow descriptors | `Established<RenderCascadeShadowConfigValid>` |
//!
//! ## Role 1b — Composers
//!
//! | Trait | Composes | Output proof |
//! |---|---|---|
//! | [`GisRenderEnvironmentFactory`] | scene environment descriptors | `Established<RenderSceneEnvironmentValid>` |
//! | [`GisRenderViewFactory`] | render view descriptors | `Established<RenderViewValid>` |
//!
//! ## Role 1c — Layer and scene factories
//!
//! | Trait | Builds | Output proof |
//! |---|---|---|
//! | [`GisRenderVectorLayerFactory`] | vector render layers from georust vector payloads | `Established<RenderableVectorLayerValid>` |
//! | [`GisRenderRasterLayerFactory`] | raster render layers from georust raster metadata plus an explicit source sidecar | `Established<RenderableRasterLayerValid>` |
//! | [`GisRenderTileLayerFactory`] | tile render layers from explicit tile sources | `Established<RenderableTileLayerValid>` |
//! | [`GisRenderTerrainLayerFactory`] | terrain render layers from elevation rasters | `Established<RenderableTerrainLayerValid>` |
//! | [`GisRenderAnnotationLayerFactory`] | explicit annotation layers from anchored symbolizers | `Established<RenderableAnnotationLayerValid>` |
//! | [`GisRenderSceneFactory`] | full scenes from validated view + validated layers | `Established<RenderableSceneValid>` |
//! | [`GisRenderSceneEnvironmentUpdateFactory`] | scene-environment updates | `Established<RenderableSceneUpdateValid>` |
//! | [`GisRenderSceneViewUpdateFactory`] | scene-view updates | `Established<RenderableSceneUpdateValid>` |
//! | [`GisRenderSceneLayerStructureUpdateFactory`] | layer insert/replace/remove/reorder updates | `Established<RenderableSceneUpdateValid>` |
//! | [`GisRenderSceneLayerStateUpdateFactory`] | layer visibility/opacity/draw-order updates | `Established<RenderableSceneUpdateValid>` |
//! | [`GisRenderSceneUpdateFactory`] | all incremental scene updates | `Established<RenderableSceneUpdateValid>` |
//!
//! ## Role 2 — Orthogonal reporters
//!
//! | Trait | Reports |
//! |---|---|
//! | [`GisRenderLayerMeta`] | identifiers, kind, order, visibility, and opacity for a render layer |
//! | [`GisRenderViewMeta`] | projection, world-space, and output-stack capability queries for a render view |
//! | [`GisRenderAssetDependencyMeta`] | asset, material-family, and pipeline dependency queries for render descriptors |
//! | [`GisRenderSceneUpdateMeta`] | operation kind and payload queries for incremental scene updates |
//! | [`GisRenderSceneMeta`] | identifiers, layer counts, and scene-level label/raster presence |
//!
//! ## Role 3 — Backend supertrait
//!
//! [`RenderBackend`] composes the render seam into a single object-safe
//! interface without forcing the core [`crate::GisBackend`] aggregate to grow
//! before producer/consumer crates implement the new render traits.

use elicitation::Established;
use geo_types::Geometry;
use geojson::FeatureCollection;
use georaster::geotiff::ImageInfo;

use crate::{
    GisResult, LayerDrawOrderAssigned, LayerOpacityUnitInterval, LayerOrderDeterministic,
    RenderAmbientLightDescriptor, RenderAmbientLightValid, RenderAnnotationDescriptor,
    RenderAtmosphereDescriptor, RenderAtmosphereValid, RenderAutoExposureDescriptor,
    RenderAutoExposureValid, RenderBloomDescriptor, RenderBloomValid,
    RenderCascadeShadowConfigDescriptor, RenderCascadeShadowConfigValid,
    RenderChromaticAberrationDescriptor, RenderChromaticAberrationValid,
    RenderColorGradingDescriptor, RenderColorGradingValid,
    RenderDepthOfFieldDescriptor, RenderDepthOfFieldValid,
    RenderDirectionalLightDescriptor, RenderDirectionalLightValid,
    RenderExposureDescriptor, RenderExposureValid, RenderFeatureRecord,
    RenderFeatureStyleDescriptor, RenderFeatureStyleValid, RenderFogDescriptor,
    RenderFogValid, RenderFogVolumeDescriptor, RenderFogVolumeValid,
    RenderAssetReference, RenderMaterialFamily, RenderPipelineDomain, RenderPrepassKind,
    RenderImageBasedLightingDescriptor, RenderImageBasedLightingValid,
    RenderLayerDescriptor, RenderLayerKind, RenderLayerSpec, RenderLightProbeDescriptor,
    RenderLightProbeValid, RenderMotionBlurDescriptor, RenderMotionBlurValid,
    RenderMaterialIntentDescriptor, RenderMaterialIntentValid,
    RenderRasterSourceDescriptor, RenderRasterStyleDescriptor, RenderRasterStyleValid,
    RenderSceneDescriptor, RenderSceneEnvironmentDescriptor, RenderSceneEnvironmentValid,
    RenderSceneSpec, RenderSceneUpdateDescriptor, RenderSceneUpdateKind,
    RenderScreenSpaceAmbientOcclusionDescriptor, RenderScreenSpaceAmbientOcclusionValid,
    RenderScreenSpaceReflectionsDescriptor, RenderScreenSpaceReflectionsValid,
    RenderShadowParticipationDescriptor, RenderShadowParticipationValid,
    RenderSkyboxDescriptor, RenderSkyboxValid, RenderTerrainStyleDescriptor,
    RenderTerrainStyleValid, RenderTileSourceDescriptor, RenderTileStyleDescriptor,
    RenderTileStyleValid, RenderViewDescriptor, RenderViewOutputDescriptor,
    RenderViewOutputValid, RenderViewValid, RenderVolumetricFogDescriptor,
    RenderVolumetricFogValid, RenderWorldSpaceDescriptor, RenderWorldSpaceValid,
    RenderableAnnotationLayerValid, RenderableLayerValid, RenderableRasterLayerValid,
    RenderableSceneUpdateValid, RenderableSceneValid, RenderableTerrainLayerValid,
    RenderableTileLayerValid, RenderableVectorLayerValid,
};

/// Validate renderer-agnostic vector symbolization descriptors.
pub trait GisRenderFeatureStyleFactory: Send + Sync {
    /// Validate a vector feature symbolizer.
    fn build_render_feature_style(
        &self,
        input: RenderFeatureStyleDescriptor,
    ) -> GisResult<Established<RenderFeatureStyleValid>>;
}

/// Validate renderer-agnostic raster symbolization descriptors.
pub trait GisRenderRasterStyleFactory: Send + Sync {
    /// Validate a raster symbolizer.
    fn build_render_raster_style(
        &self,
        input: RenderRasterStyleDescriptor,
    ) -> GisResult<Established<RenderRasterStyleValid>>;
}

/// Validate renderer-agnostic tile symbolization descriptors.
pub trait GisRenderTileStyleFactory: Send + Sync {
    /// Validate a tile-layer symbolizer.
    fn build_render_tile_style(
        &self,
        input: RenderTileStyleDescriptor,
    ) -> GisResult<Established<RenderTileStyleValid>>;
}

/// Validate renderer-agnostic terrain symbolization descriptors.
pub trait GisRenderTerrainStyleFactory: Send + Sync {
    /// Validate a terrain symbolizer.
    fn build_render_terrain_style(
        &self,
        input: RenderTerrainStyleDescriptor,
    ) -> GisResult<Established<RenderTerrainStyleValid>>;
}

/// Validate renderer-agnostic material-family and texture-binding descriptors.
pub trait GisRenderMaterialIntentFactory: Send + Sync {
    /// Validate a material-intent descriptor.
    fn build_render_material_intent(
        &self,
        input: RenderMaterialIntentDescriptor,
    ) -> GisResult<Established<RenderMaterialIntentValid>>;
}

/// Validate shadow-participation descriptors.
pub trait GisRenderShadowParticipationFactory: Send + Sync {
    /// Validate a shadow-participation descriptor.
    fn build_render_shadow_participation(
        &self,
        input: RenderShadowParticipationDescriptor,
    ) -> GisResult<Established<RenderShadowParticipationValid>>;
}

/// Validate renderer-agnostic camera output and post-process descriptors.
pub trait GisRenderViewOutputFactory: Send + Sync {
    /// Validate a render view output descriptor.
    fn build_render_view_output(
        &self,
        input: RenderViewOutputDescriptor,
    ) -> GisResult<Established<RenderViewOutputValid>>;
}

/// Validate fixed-exposure descriptors.
pub trait GisRenderExposureFactory: Send + Sync {
    /// Validate a fixed-exposure descriptor.
    fn build_render_exposure(
        &self,
        input: RenderExposureDescriptor,
    ) -> GisResult<Established<RenderExposureValid>>;
}

/// Validate automatic-exposure descriptors.
pub trait GisRenderAutoExposureFactory: Send + Sync {
    /// Validate an automatic-exposure descriptor.
    fn build_render_auto_exposure(
        &self,
        input: RenderAutoExposureDescriptor,
    ) -> GisResult<Established<RenderAutoExposureValid>>;
}

/// Validate color-grading descriptors.
pub trait GisRenderColorGradingFactory: Send + Sync {
    /// Validate a color-grading descriptor.
    fn build_render_color_grading(
        &self,
        input: RenderColorGradingDescriptor,
    ) -> GisResult<Established<RenderColorGradingValid>>;
}

/// Validate bloom descriptors.
pub trait GisRenderBloomFactory: Send + Sync {
    /// Validate a bloom descriptor.
    fn build_render_bloom(
        &self,
        input: RenderBloomDescriptor,
    ) -> GisResult<Established<RenderBloomValid>>;
}

/// Validate depth-of-field descriptors.
pub trait GisRenderDepthOfFieldFactory: Send + Sync {
    /// Validate a depth-of-field descriptor.
    fn build_render_depth_of_field(
        &self,
        input: RenderDepthOfFieldDescriptor,
    ) -> GisResult<Established<RenderDepthOfFieldValid>>;
}

/// Validate motion-blur descriptors.
pub trait GisRenderMotionBlurFactory: Send + Sync {
    /// Validate a motion-blur descriptor.
    fn build_render_motion_blur(
        &self,
        input: RenderMotionBlurDescriptor,
    ) -> GisResult<Established<RenderMotionBlurValid>>;
}

/// Validate chromatic-aberration descriptors.
pub trait GisRenderChromaticAberrationFactory: Send + Sync {
    /// Validate a chromatic-aberration descriptor.
    fn build_render_chromatic_aberration(
        &self,
        input: RenderChromaticAberrationDescriptor,
    ) -> GisResult<Established<RenderChromaticAberrationValid>>;
}

/// Validate screen-space-reflection descriptors.
pub trait GisRenderScreenSpaceReflectionsFactory: Send + Sync {
    /// Validate a screen-space-reflection descriptor.
    fn build_render_screen_space_reflections(
        &self,
        input: RenderScreenSpaceReflectionsDescriptor,
    ) -> GisResult<Established<RenderScreenSpaceReflectionsValid>>;
}

/// Validate screen-space ambient occlusion descriptors.
pub trait GisRenderScreenSpaceAmbientOcclusionFactory: Send + Sync {
    /// Validate a screen-space ambient occlusion descriptor.
    fn build_render_screen_space_ambient_occlusion(
        &self,
        input: RenderScreenSpaceAmbientOcclusionDescriptor,
    ) -> GisResult<Established<RenderScreenSpaceAmbientOcclusionValid>>;
}

/// Validate renderer-agnostic world-space mapping descriptors.
pub trait GisRenderWorldSpaceFactory: Send + Sync {
    /// Validate a render world-space descriptor.
    fn build_render_world_space(
        &self,
        input: RenderWorldSpaceDescriptor,
    ) -> GisResult<Established<RenderWorldSpaceValid>>;
}

/// Validate localized light-probe descriptors.
pub trait GisRenderLightProbeFactory: Send + Sync {
    /// Validate a localized light-probe descriptor.
    fn build_render_light_probe(
        &self,
        input: RenderLightProbeDescriptor,
    ) -> GisResult<Established<RenderLightProbeValid>>;
}

/// Validate image-based lighting descriptors.
pub trait GisRenderImageBasedLightingFactory: Send + Sync {
    /// Validate an image-based lighting descriptor.
    fn build_render_image_based_lighting(
        &self,
        input: RenderImageBasedLightingDescriptor,
    ) -> GisResult<Established<RenderImageBasedLightingValid>>;
}

/// Validate skybox descriptors.
pub trait GisRenderSkyboxFactory: Send + Sync {
    /// Validate a skybox descriptor.
    fn build_render_skybox(
        &self,
        input: RenderSkyboxDescriptor,
    ) -> GisResult<Established<RenderSkyboxValid>>;
}

/// Validate ambient-light descriptors.
pub trait GisRenderAmbientLightFactory: Send + Sync {
    /// Validate an ambient-light descriptor.
    fn build_render_ambient_light(
        &self,
        input: RenderAmbientLightDescriptor,
    ) -> GisResult<Established<RenderAmbientLightValid>>;
}

/// Validate directional-light descriptors.
pub trait GisRenderDirectionalLightFactory: Send + Sync {
    /// Validate a directional-light descriptor.
    fn build_render_directional_light(
        &self,
        input: RenderDirectionalLightDescriptor,
    ) -> GisResult<Established<RenderDirectionalLightValid>>;
}

/// Validate atmosphere descriptors.
pub trait GisRenderAtmosphereFactory: Send + Sync {
    /// Validate an atmosphere descriptor.
    fn build_render_atmosphere(
        &self,
        input: RenderAtmosphereDescriptor,
    ) -> GisResult<Established<RenderAtmosphereValid>>;
}

/// Validate distance/atmospheric fog descriptors.
pub trait GisRenderFogFactory: Send + Sync {
    /// Validate a fog descriptor.
    fn build_render_fog(
        &self,
        input: RenderFogDescriptor,
    ) -> GisResult<Established<RenderFogValid>>;
}

/// Validate camera-based volumetric fog descriptors.
pub trait GisRenderVolumetricFogFactory: Send + Sync {
    /// Validate a volumetric fog descriptor.
    fn build_render_volumetric_fog(
        &self,
        input: RenderVolumetricFogDescriptor,
    ) -> GisResult<Established<RenderVolumetricFogValid>>;
}

/// Validate localized fog-volume descriptors.
pub trait GisRenderFogVolumeFactory: Send + Sync {
    /// Validate a localized fog-volume descriptor.
    fn build_render_fog_volume(
        &self,
        input: RenderFogVolumeDescriptor,
    ) -> GisResult<Established<RenderFogVolumeValid>>;
}

/// Validate cascade-shadow descriptors.
pub trait GisRenderCascadeShadowFactory: Send + Sync {
    /// Validate a cascade-shadow descriptor.
    fn build_render_cascade_shadows(
        &self,
        input: RenderCascadeShadowConfigDescriptor,
    ) -> GisResult<Established<RenderCascadeShadowConfigValid>>;
}

/// Validate scene-level environment and lighting descriptors.
pub trait GisRenderEnvironmentFactory: Send + Sync {
    /// Validate a scene environment descriptor and establish
    /// `RenderSceneEnvironmentValid`.
    fn build_render_scene_environment(
        &self,
        input: RenderSceneEnvironmentDescriptor,
    ) -> GisResult<Established<RenderSceneEnvironmentValid>>;
}

/// Validate renderer-agnostic map view descriptors.
pub trait GisRenderViewFactory: Send + Sync {
    /// Validate a render view descriptor and establish `RenderViewValid`.
    fn build_render_view(
        &self,
        input: RenderViewDescriptor,
    ) -> GisResult<Established<RenderViewValid>>;
}

/// Build vector render layers from upstream georust vector payloads.
pub trait GisRenderVectorLayerFactory: Send + Sync {
    /// Build a renderable vector layer from concrete `geo-types` geometries.
    fn build_vector_layer_from_geometries(
        &self,
        spec: RenderLayerSpec,
        geometries: Vec<Geometry<f64>>,
        style: RenderFeatureStyleDescriptor,
        style_proof: Established<RenderFeatureStyleValid>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableVectorLayerValid>,
    )>;

    /// Build a renderable vector layer from a GeoJSON feature collection.
    fn build_vector_layer_from_feature_collection(
        &self,
        spec: RenderLayerSpec,
        features: FeatureCollection,
        style: RenderFeatureStyleDescriptor,
        style_proof: Established<RenderFeatureStyleValid>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableVectorLayerValid>,
    )>;

    /// Build a renderable vector layer from explicit feature records carrying
    /// `geo-types` geometry plus arbitrary properties.
    fn build_vector_layer_from_feature_records(
        &self,
        spec: RenderLayerSpec,
        features: Vec<RenderFeatureRecord>,
        style: RenderFeatureStyleDescriptor,
        style_proof: Established<RenderFeatureStyleValid>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableVectorLayerValid>,
    )>;
}

/// Build raster render layers from upstream georust raster metadata.
pub trait GisRenderRasterLayerFactory: Send + Sync {
    /// Build a renderable raster layer from explicit raster-source metadata and
    /// GeoTIFF image metadata.
    fn build_raster_layer_from_image_info(
        &self,
        spec: RenderLayerSpec,
        source: RenderRasterSourceDescriptor,
        image_info: ImageInfo,
        style: RenderRasterStyleDescriptor,
        style_proof: Established<RenderRasterStyleValid>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableRasterLayerValid>,
    )>;
}

/// Build tile render layers from explicit tile sources.
pub trait GisRenderTileLayerFactory: Send + Sync {
    /// Build a renderable tile layer from a tileset source.
    fn build_tile_layer(
        &self,
        spec: RenderLayerSpec,
        source: RenderTileSourceDescriptor,
        style: RenderTileStyleDescriptor,
        style_proof: Established<RenderTileStyleValid>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(RenderLayerDescriptor, Established<RenderableTileLayerValid>)>;
}

/// Build terrain render layers from elevation rasters.
pub trait GisRenderTerrainLayerFactory: Send + Sync {
    /// Build a renderable terrain layer from raster source metadata and
    /// GeoTIFF image metadata.
    fn build_terrain_layer_from_image_info(
        &self,
        spec: RenderLayerSpec,
        source: RenderRasterSourceDescriptor,
        image_info: ImageInfo,
        style: RenderTerrainStyleDescriptor,
        style_proof: Established<RenderTerrainStyleValid>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableTerrainLayerValid>,
    )>;
}

/// Build explicit annotation layers from anchored symbolizers.
pub trait GisRenderAnnotationLayerFactory: Send + Sync {
    /// Build a renderable annotation layer from explicit annotations.
    fn build_annotation_layer(
        &self,
        spec: RenderLayerSpec,
        annotations: Vec<RenderAnnotationDescriptor>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableAnnotationLayerValid>,
    )>;
}

/// Compose validated render layers into a validated scene.
pub trait GisRenderSceneFactory: Send + Sync {
    /// Build a render scene from a validated view and validated layer receipts.
    fn build_render_scene(
        &self,
        spec: RenderSceneSpec,
        environment_proof: Option<Established<RenderSceneEnvironmentValid>>,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
        layers: Vec<(RenderLayerDescriptor, Established<RenderableLayerValid>)>,
    ) -> GisResult<(RenderSceneDescriptor, Established<RenderableSceneValid>)>;
}

/// Build scene-environment updates.
pub trait GisRenderSceneEnvironmentUpdateFactory: Send + Sync {
    /// Replace the active environment for a scene.
    fn build_render_scene_environment_replace(
        &self,
        scene_id: String,
        environment: RenderSceneEnvironmentDescriptor,
        environment_proof: Established<RenderSceneEnvironmentValid>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;

    /// Clear the active environment for a scene.
    fn build_render_scene_environment_clear(
        &self,
        scene_id: String,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;
}

/// Build scene-view updates.
pub trait GisRenderSceneViewUpdateFactory: Send + Sync {
    /// Replace the active view for a scene.
    fn build_render_scene_view_replace(
        &self,
        scene_id: String,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;
}

/// Build scene updates that change layer structure.
pub trait GisRenderSceneLayerStructureUpdateFactory: Send + Sync {
    /// Insert a validated layer into a scene.
    fn build_render_scene_layer_insert(
        &self,
        scene_id: String,
        layer: RenderLayerDescriptor,
        layer_proof: Established<RenderableLayerValid>,
        before_layer_id: Option<String>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;

    /// Replace an existing layer in a scene.
    fn build_render_scene_layer_replace(
        &self,
        scene_id: String,
        target_layer_id: String,
        layer: RenderLayerDescriptor,
        layer_proof: Established<RenderableLayerValid>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;

    /// Remove an existing layer from a scene.
    fn build_render_scene_layer_remove(
        &self,
        scene_id: String,
        target_layer_id: String,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;

    /// Reorder a layer relative to another layer.
    fn build_render_scene_layer_move_before(
        &self,
        scene_id: String,
        target_layer_id: String,
        before_layer_id: Option<String>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;
}

/// Build scene updates that change layer state.
pub trait GisRenderSceneLayerStateUpdateFactory: Send + Sync {
    /// Update a layer's visibility.
    fn build_render_scene_layer_visibility_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        visible: bool,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;

    /// Update a layer's opacity.
    fn build_render_scene_layer_opacity_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        opacity: f32,
        opacity_proof: Established<LayerOpacityUnitInterval>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;

    /// Update a layer's draw order.
    fn build_render_scene_layer_draw_order_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        draw_order: i32,
        draw_order_proof: Established<LayerDrawOrderAssigned>,
    ) -> GisResult<(RenderSceneUpdateDescriptor, Established<RenderableSceneUpdateValid>)>;
}

/// Build all supported incremental scene updates.
pub trait GisRenderSceneUpdateFactory:
    GisRenderSceneEnvironmentUpdateFactory
    + GisRenderSceneViewUpdateFactory
    + GisRenderSceneLayerStructureUpdateFactory
    + GisRenderSceneLayerStateUpdateFactory
    + Send
    + Sync
{
}

impl<T> GisRenderSceneUpdateFactory for T where
    T: GisRenderSceneEnvironmentUpdateFactory
        + GisRenderSceneViewUpdateFactory
        + GisRenderSceneLayerStructureUpdateFactory
        + GisRenderSceneLayerStateUpdateFactory
        + Send
        + Sync
{
}

/// Query orthogonal metadata from a render layer descriptor.
pub trait GisRenderLayerMeta: Send + Sync {
    /// Report the stable layer identifier.
    fn layer_id<'a>(&self, layer: &'a RenderLayerDescriptor) -> &'a str;

    /// Report the user-facing layer name.
    fn layer_name<'a>(&self, layer: &'a RenderLayerDescriptor) -> &'a str;

    /// Report the layer kind.
    fn layer_kind(&self, layer: &RenderLayerDescriptor) -> RenderLayerKind;

    /// Report the assigned draw order.
    fn layer_draw_order(&self, layer: &RenderLayerDescriptor) -> i32;

    /// Report whether the layer is visible.
    fn layer_is_visible(&self, layer: &RenderLayerDescriptor) -> bool;

    /// Re-establish that opacity is in the unit interval for a previously
    /// validated layer descriptor.
    fn layer_opacity(
        &self,
        layer: &RenderLayerDescriptor,
    ) -> GisResult<(f32, Established<LayerOpacityUnitInterval>)>;
}

/// Query orthogonal metadata from a render scene descriptor.
pub trait GisRenderViewMeta: Send + Sync {
    /// Whether the view carries an explicit CRS authority/code pair.
    fn view_has_crs(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view uses a perspective projection.
    fn view_is_perspective(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries explicit world-space policy.
    fn view_has_world_space(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries explicit output policy.
    fn view_has_output(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries an explicit output target override.
    fn view_has_output_target(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries an explicit clear-policy override.
    fn view_has_clear_policy(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view requests HDR output.
    fn view_output_is_hdr(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries an explicit tonemapping operator.
    fn view_has_tonemapping(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries explicit MSAA policy.
    fn view_has_msaa(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries a fixed-exposure policy.
    fn view_has_fixed_exposure(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries an automatic-exposure policy.
    fn view_has_auto_exposure(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries color grading.
    fn view_has_color_grading(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries bloom.
    fn view_has_bloom(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries depth of field.
    fn view_has_depth_of_field(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries motion blur.
    fn view_has_motion_blur(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries chromatic aberration.
    fn view_has_chromatic_aberration(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries screen-space reflections.
    fn view_has_screen_space_reflections(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries screen-space ambient occlusion.
    fn view_has_screen_space_ambient_occlusion(
        &self,
        view: &RenderViewDescriptor,
    ) -> bool;

    /// Whether the view carries an explicit shadow-filtering policy.
    fn view_has_shadow_filtering_method(&self, view: &RenderViewDescriptor) -> bool;

    /// Whether the view carries an explicit screen-space transmission-quality hint.
    fn view_has_screen_space_transmission_quality(
        &self,
        view: &RenderViewDescriptor,
    ) -> bool;
}

/// Query declared asset and material-family dependencies from render descriptors.
pub trait GisRenderAssetDependencyMeta: Send + Sync {
    /// Declared asset dependencies needed to realize a render view.
    fn view_asset_dependencies(&self, view: &RenderViewDescriptor) -> Vec<RenderAssetReference>;

    /// Declared material families needed to realize a render view.
    fn view_material_families(&self, view: &RenderViewDescriptor) -> Vec<RenderMaterialFamily>;

    /// Declared asset dependencies needed to realize a render layer.
    fn layer_asset_dependencies(&self, layer: &RenderLayerDescriptor) -> Vec<RenderAssetReference>;

    /// Declared material families needed to realize a render layer.
    fn layer_material_families(&self, layer: &RenderLayerDescriptor) -> Vec<RenderMaterialFamily>;

    /// Declared pipeline domains needed to realize a render layer.
    fn layer_pipeline_domains(&self, layer: &RenderLayerDescriptor) -> Vec<RenderPipelineDomain>;

    /// Declared prepass requirements needed to realize a render layer.
    fn layer_prepasses(&self, layer: &RenderLayerDescriptor) -> Vec<RenderPrepassKind>;

    /// Declared asset dependencies needed to realize a full render scene.
    fn scene_asset_dependencies(
        &self,
        scene: &RenderSceneDescriptor,
    ) -> Vec<RenderAssetReference>;

    /// Declared material families needed to realize a full render scene.
    fn scene_material_families(
        &self,
        scene: &RenderSceneDescriptor,
    ) -> Vec<RenderMaterialFamily>;

    /// Declared pipeline domains needed to realize a full render scene.
    fn scene_pipeline_domains(
        &self,
        scene: &RenderSceneDescriptor,
    ) -> Vec<RenderPipelineDomain>;

    /// Declared prepass requirements needed to realize a full render scene.
    fn scene_prepasses(&self, scene: &RenderSceneDescriptor) -> Vec<RenderPrepassKind>;
}

/// Query orthogonal metadata from a render scene update descriptor.
pub trait GisRenderSceneUpdateMeta: Send + Sync {
    /// Stable scene identifier targeted by the update.
    fn update_scene_id<'a>(&self, update: &'a RenderSceneUpdateDescriptor) -> &'a str;

    /// Orthogonal operation kind for the update.
    fn update_kind(&self, update: &RenderSceneUpdateDescriptor) -> RenderSceneUpdateKind;

    /// Target layer identifier when the operation addresses one existing layer.
    fn update_target_layer_id<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a str>;

    /// Reference layer identifier when the operation is ordered relative to another layer.
    fn update_reference_layer_id<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a str>;

    /// Replacement environment payload when present.
    fn update_environment<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a RenderSceneEnvironmentDescriptor>;

    /// Replacement view payload when present.
    fn update_view<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a RenderViewDescriptor>;

    /// Replacement or inserted layer payload when present.
    fn update_layer<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a RenderLayerDescriptor>;

    /// Replacement visibility value when present.
    fn update_visibility(&self, update: &RenderSceneUpdateDescriptor) -> Option<bool>;

    /// Replacement opacity value with re-established proof when present.
    fn update_opacity(
        &self,
        update: &RenderSceneUpdateDescriptor,
    ) -> GisResult<Option<(f32, Established<LayerOpacityUnitInterval>)>>;

    /// Replacement draw order with re-established proof when present.
    fn update_draw_order(
        &self,
        update: &RenderSceneUpdateDescriptor,
    ) -> GisResult<Option<(i32, Established<LayerDrawOrderAssigned>)>>;
}

/// Query orthogonal metadata from a render scene descriptor.
pub trait GisRenderSceneMeta: Send + Sync {
    /// Stable scene identifier.
    fn scene_id<'a>(&self, scene: &'a RenderSceneDescriptor) -> &'a str;

    /// Number of layers in the scene.
    fn scene_layer_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Number of vector layers in the scene.
    fn scene_vector_layer_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Number of raster layers in the scene.
    fn scene_raster_layer_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Number of tile layers in the scene.
    fn scene_tile_layer_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Number of terrain layers in the scene.
    fn scene_terrain_layer_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Number of annotation layers in the scene.
    fn scene_annotation_layer_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Whether the scene contains at least one raster layer.
    fn scene_has_raster_layers(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene contains at least one tile layer.
    fn scene_has_tile_layers(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene contains at least one terrain layer.
    fn scene_has_terrain_layers(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene contains at least one annotation layer.
    fn scene_has_annotation_layers(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene contains at least one labeled layer.
    fn scene_has_feature_labels(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene carries an explicit environment descriptor.
    fn scene_has_environment(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene carries explicit distance/atmospheric fog.
    fn scene_has_fog(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene carries camera-based volumetric fog.
    fn scene_has_volumetric_fog(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene carries explicit image-based lighting policy.
    fn scene_has_image_based_lighting(
        &self,
        scene: &RenderSceneDescriptor,
    ) -> bool;

    /// Whether the scene carries an explicit atmosphere descriptor.
    fn scene_has_atmosphere(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Whether the scene carries any localized light probes.
    fn scene_has_light_probes(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Number of localized light probes embedded in the scene environment.
    fn scene_light_probe_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Whether the scene carries a visible solar-disk policy.
    fn scene_has_sun_disk(&self, scene: &RenderSceneDescriptor) -> bool;

    /// Number of localized fog volumes embedded in the scene environment.
    fn scene_fog_volume_count(&self, scene: &RenderSceneDescriptor) -> usize;

    /// Re-establish deterministic ordering for a previously validated scene.
    fn scene_layer_order_is_deterministic(
        &self,
        scene: &RenderSceneDescriptor,
    ) -> GisResult<Established<LayerOrderDeterministic>>;
}

/// Complete render backend — blanket supertrait for the render seam only.
pub trait RenderBackend:
    GisRenderFeatureStyleFactory
    + GisRenderRasterStyleFactory
    + GisRenderTileStyleFactory
    + GisRenderTerrainStyleFactory
    + GisRenderMaterialIntentFactory
    + GisRenderShadowParticipationFactory
    + GisRenderViewOutputFactory
    + GisRenderExposureFactory
    + GisRenderAutoExposureFactory
    + GisRenderColorGradingFactory
    + GisRenderBloomFactory
    + GisRenderDepthOfFieldFactory
    + GisRenderMotionBlurFactory
    + GisRenderChromaticAberrationFactory
    + GisRenderScreenSpaceReflectionsFactory
    + GisRenderScreenSpaceAmbientOcclusionFactory
    + GisRenderWorldSpaceFactory
    + GisRenderLightProbeFactory
    + GisRenderImageBasedLightingFactory
    + GisRenderSkyboxFactory
    + GisRenderAmbientLightFactory
    + GisRenderDirectionalLightFactory
    + GisRenderAtmosphereFactory
    + GisRenderFogFactory
    + GisRenderVolumetricFogFactory
    + GisRenderFogVolumeFactory
    + GisRenderCascadeShadowFactory
    + GisRenderEnvironmentFactory
    + GisRenderViewFactory
    + GisRenderVectorLayerFactory
    + GisRenderRasterLayerFactory
    + GisRenderTileLayerFactory
    + GisRenderTerrainLayerFactory
    + GisRenderAnnotationLayerFactory
    + GisRenderSceneFactory
    + GisRenderSceneEnvironmentUpdateFactory
    + GisRenderSceneViewUpdateFactory
    + GisRenderSceneLayerStructureUpdateFactory
    + GisRenderSceneLayerStateUpdateFactory
    + GisRenderSceneUpdateFactory
    + GisRenderLayerMeta
    + GisRenderViewMeta
    + GisRenderAssetDependencyMeta
    + GisRenderSceneUpdateMeta
    + GisRenderSceneMeta
    + Send
    + Sync
{
}

impl<T> RenderBackend for T where
    T: GisRenderFeatureStyleFactory
        + GisRenderRasterStyleFactory
        + GisRenderTileStyleFactory
        + GisRenderTerrainStyleFactory
        + GisRenderMaterialIntentFactory
        + GisRenderShadowParticipationFactory
        + GisRenderViewOutputFactory
        + GisRenderExposureFactory
        + GisRenderAutoExposureFactory
        + GisRenderColorGradingFactory
        + GisRenderBloomFactory
        + GisRenderDepthOfFieldFactory
        + GisRenderMotionBlurFactory
        + GisRenderChromaticAberrationFactory
        + GisRenderScreenSpaceReflectionsFactory
        + GisRenderScreenSpaceAmbientOcclusionFactory
        + GisRenderWorldSpaceFactory
        + GisRenderLightProbeFactory
        + GisRenderImageBasedLightingFactory
        + GisRenderSkyboxFactory
        + GisRenderAmbientLightFactory
        + GisRenderDirectionalLightFactory
        + GisRenderAtmosphereFactory
        + GisRenderFogFactory
        + GisRenderVolumetricFogFactory
        + GisRenderFogVolumeFactory
        + GisRenderCascadeShadowFactory
        + GisRenderEnvironmentFactory
        + GisRenderViewFactory
        + GisRenderVectorLayerFactory
        + GisRenderRasterLayerFactory
        + GisRenderTileLayerFactory
        + GisRenderTerrainLayerFactory
        + GisRenderAnnotationLayerFactory
        + GisRenderSceneFactory
        + GisRenderSceneEnvironmentUpdateFactory
        + GisRenderSceneViewUpdateFactory
        + GisRenderSceneLayerStructureUpdateFactory
        + GisRenderSceneLayerStateUpdateFactory
        + GisRenderSceneUpdateFactory
        + GisRenderLayerMeta
        + GisRenderViewMeta
        + GisRenderAssetDependencyMeta
        + GisRenderSceneUpdateMeta
        + GisRenderSceneMeta
        + Send
        + Sync
{
}
