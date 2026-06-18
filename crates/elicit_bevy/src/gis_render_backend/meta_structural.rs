//! Pure field-read meta trait impls.

use elicit_gis::{
    GisRenderAssetDependencyMeta, GisRenderLayerMeta, GisRenderSceneMeta, GisRenderSceneUpdateMeta,
    GisRenderTimeMeta, GisRenderViewMeta, GisResult, LayerDrawOrderAssigned,
    LayerOpacityUnitInterval, LayerOrderDeterministic, RenderAssetReference,
    RenderCameraProjectionDescriptor, RenderLayerDescriptor, RenderLayerKind,
    RenderLayerTimeFilter, RenderMaterialFamily, RenderOutputTargetDescriptor,
    RenderPipelineDomain, RenderPrepassKind, RenderScaleRangeDescriptor, RenderSceneDescriptor,
    RenderSceneEnvironmentDescriptor, RenderSceneEnvironmentValid, RenderSceneUpdateDescriptor,
    RenderSceneUpdateKind, RenderTimeContext, RenderViewDescriptor,
    RenderViewParticipationDescriptor, RenderViewValid, RenderableLayerValid,
    RenderableSceneUpdateValid, RenderableSceneValid,
};
use elicitation::Established;
use tracing::instrument;

use super::BevyGisBackend;

fn layer_id_str(layer: &RenderLayerDescriptor) -> &str {
    match layer {
        RenderLayerDescriptor::Vector(l) => &l.id,
        RenderLayerDescriptor::Raster(l) => &l.id,
        RenderLayerDescriptor::Tile(l) => &l.id,
        RenderLayerDescriptor::Terrain(l) => &l.id,
        RenderLayerDescriptor::Annotation(l) => &l.id,
        RenderLayerDescriptor::Model(l) => &l.id,
    }
}

fn layer_name_str(layer: &RenderLayerDescriptor) -> &str {
    match layer {
        RenderLayerDescriptor::Vector(l) => &l.name,
        RenderLayerDescriptor::Raster(l) => &l.name,
        RenderLayerDescriptor::Tile(l) => &l.name,
        RenderLayerDescriptor::Terrain(l) => &l.name,
        RenderLayerDescriptor::Annotation(l) => &l.name,
        RenderLayerDescriptor::Model(l) => &l.name,
    }
}

fn layer_draw_order_val(layer: &RenderLayerDescriptor) -> i32 {
    match layer {
        RenderLayerDescriptor::Vector(l) => l.draw_order,
        RenderLayerDescriptor::Raster(l) => l.draw_order,
        RenderLayerDescriptor::Tile(l) => l.draw_order,
        RenderLayerDescriptor::Terrain(l) => l.draw_order,
        RenderLayerDescriptor::Annotation(l) => l.draw_order,
        RenderLayerDescriptor::Model(l) => l.draw_order,
    }
}

fn layer_visible_val(layer: &RenderLayerDescriptor) -> bool {
    match layer {
        RenderLayerDescriptor::Vector(l) => l.visible,
        RenderLayerDescriptor::Raster(l) => l.visible,
        RenderLayerDescriptor::Tile(l) => l.visible,
        RenderLayerDescriptor::Terrain(l) => l.visible,
        RenderLayerDescriptor::Annotation(l) => l.visible,
        RenderLayerDescriptor::Model(l) => l.visible,
    }
}

fn layer_opacity_val(layer: &RenderLayerDescriptor) -> f32 {
    match layer {
        RenderLayerDescriptor::Vector(l) => l.opacity,
        RenderLayerDescriptor::Raster(l) => l.opacity,
        RenderLayerDescriptor::Tile(l) => l.opacity,
        RenderLayerDescriptor::Terrain(l) => l.opacity,
        RenderLayerDescriptor::Annotation(l) => l.opacity,
        RenderLayerDescriptor::Model(l) => l.opacity,
    }
}

fn layer_view_participation_ref(
    layer: &RenderLayerDescriptor,
) -> &RenderViewParticipationDescriptor {
    match layer {
        RenderLayerDescriptor::Vector(l) => &l.view_participation,
        RenderLayerDescriptor::Raster(l) => &l.view_participation,
        RenderLayerDescriptor::Tile(l) => &l.view_participation,
        RenderLayerDescriptor::Terrain(l) => &l.view_participation,
        RenderLayerDescriptor::Annotation(l) => &l.view_participation,
        RenderLayerDescriptor::Model(l) => &l.view_participation,
    }
}

fn layer_resolution_range(layer: &RenderLayerDescriptor) -> Option<&RenderScaleRangeDescriptor> {
    match layer {
        RenderLayerDescriptor::Vector(l) => l.resolution_range.as_ref(),
        RenderLayerDescriptor::Raster(l) => l.resolution_range.as_ref(),
        RenderLayerDescriptor::Tile(l) => l.resolution_range.as_ref(),
        RenderLayerDescriptor::Terrain(l) => l.resolution_range.as_ref(),
        RenderLayerDescriptor::Annotation(l) => l.resolution_range.as_ref(),
        RenderLayerDescriptor::Model(l) => l.resolution_range.as_ref(),
    }
}

// ── GisRenderLayerMeta ────────────────────────────────────────────────────────

impl GisRenderLayerMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn layer_id<'a>(&self, layer: &'a RenderLayerDescriptor) -> &'a str {
        layer_id_str(layer)
    }

    #[instrument(skip_all)]
    fn layer_name<'a>(&self, layer: &'a RenderLayerDescriptor) -> &'a str {
        layer_name_str(layer)
    }

    #[instrument(skip_all)]
    fn layer_kind(&self, layer: &RenderLayerDescriptor) -> RenderLayerKind {
        match layer {
            RenderLayerDescriptor::Vector(_) => RenderLayerKind::Vector,
            RenderLayerDescriptor::Raster(_) => RenderLayerKind::Raster,
            RenderLayerDescriptor::Tile(_) => RenderLayerKind::Tile,
            RenderLayerDescriptor::Terrain(_) => RenderLayerKind::Terrain,
            RenderLayerDescriptor::Annotation(_) => RenderLayerKind::Annotation,
            RenderLayerDescriptor::Model(_) => RenderLayerKind::Model,
        }
    }

    #[instrument(skip_all)]
    fn layer_draw_order(&self, layer: &RenderLayerDescriptor) -> i32 {
        layer_draw_order_val(layer)
    }

    #[instrument(skip_all)]
    fn layer_is_visible(&self, layer: &RenderLayerDescriptor) -> bool {
        layer_visible_val(layer)
    }

    #[instrument(skip_all)]
    fn layer_view_participation<'a>(
        &self,
        layer: &'a RenderLayerDescriptor,
    ) -> &'a RenderViewParticipationDescriptor {
        layer_view_participation_ref(layer)
    }

    #[instrument(skip_all)]
    fn layer_opacity(
        &self,
        layer: &RenderLayerDescriptor,
        layer_proof: Established<RenderableLayerValid>,
    ) -> GisResult<(f32, Established<LayerOpacityUnitInterval>)> {
        Ok((layer_opacity_val(layer), Established::prove(&layer_proof)))
    }

    #[instrument(skip_all)]
    fn layer_scale_range<'a>(
        &self,
        layer: &'a RenderLayerDescriptor,
    ) -> Option<&'a RenderScaleRangeDescriptor> {
        layer_resolution_range(layer)
    }

    #[instrument(skip_all)]
    fn layer_is_visible_at_resolution(
        &self,
        layer: &RenderLayerDescriptor,
        resolution: f64,
    ) -> bool {
        match layer_resolution_range(layer) {
            None => true,
            Some(range) => {
                let above_min = range.min_resolution.is_none_or(|min| resolution >= min);
                let below_max = range.max_resolution.is_none_or(|max| resolution <= max);
                above_min && below_max
            }
        }
    }
}

// ── GisRenderViewMeta ─────────────────────────────────────────────────────────

impl GisRenderViewMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn view_id<'a>(&self, view: &'a RenderViewDescriptor) -> &'a str {
        &view.view_id
    }

    #[instrument(skip_all)]
    fn view_has_crs(&self, view: &RenderViewDescriptor) -> bool {
        view.crs_authority.is_some() && view.crs_code.is_some()
    }

    #[instrument(skip_all)]
    fn view_is_perspective(&self, view: &RenderViewDescriptor) -> bool {
        matches!(
            view.projection,
            RenderCameraProjectionDescriptor::Perspective { .. }
        )
    }

    #[instrument(skip_all)]
    fn view_has_world_space(&self, view: &RenderViewDescriptor) -> bool {
        view.world_space.is_some()
    }

    #[instrument(skip_all)]
    fn view_has_output(&self, view: &RenderViewDescriptor) -> bool {
        view.output.is_some()
    }

    #[instrument(skip_all)]
    fn view_has_output_target(&self, view: &RenderViewDescriptor) -> bool {
        view.output.as_ref().is_some_and(|o| o.target.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_clear_policy(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.clear_policy.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_viewport(&self, view: &RenderViewDescriptor) -> bool {
        view.output.as_ref().is_some_and(|o| o.viewport.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_resolution_override(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.resolution_override.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_subview_layout(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.subview_layout.is_some())
    }

    #[instrument(skip_all)]
    fn view_output_is_hdr(&self, view: &RenderViewDescriptor) -> bool {
        view.output.as_ref().is_some_and(|o| o.hdr)
    }

    #[instrument(skip_all)]
    fn view_has_tonemapping(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.tonemapping.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_msaa(&self, view: &RenderViewDescriptor) -> bool {
        view.output.as_ref().is_some_and(|o| o.msaa.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_fixed_exposure(&self, view: &RenderViewDescriptor) -> bool {
        view.output.as_ref().is_some_and(|o| o.exposure.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_auto_exposure(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.auto_exposure.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_color_grading(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.color_grading.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_bloom(&self, view: &RenderViewDescriptor) -> bool {
        view.output.as_ref().is_some_and(|o| o.bloom.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_depth_of_field(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.depth_of_field.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_motion_blur(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.motion_blur.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_chromatic_aberration(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.chromatic_aberration.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_screen_space_reflections(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.screen_space_reflections.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_screen_space_ambient_occlusion(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.screen_space_ambient_occlusion.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_shadow_filtering_method(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.shadow_filtering_method.is_some())
    }

    #[instrument(skip_all)]
    fn view_has_screen_space_transmission_quality(&self, view: &RenderViewDescriptor) -> bool {
        view.output
            .as_ref()
            .is_some_and(|o| o.screen_space_transmission_quality.is_some())
    }
}

// ── GisRenderAssetDependencyMeta ──────────────────────────────────────────────

impl GisRenderAssetDependencyMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn view_asset_dependencies(&self, view: &RenderViewDescriptor) -> Vec<RenderAssetReference> {
        let mut assets = Vec::new();
        if let Some(output) = &view.output {
            match &output.target {
                Some(RenderOutputTargetDescriptor::ImageAsset { asset }) => {
                    assets.push(asset.clone());
                }
                Some(RenderOutputTargetDescriptor::TextureViewAsset { asset }) => {
                    assets.push(asset.clone());
                }
                _ => {}
            }
            if let Some(ca) = &output.chromatic_aberration
                && let Some(lut) = &ca.color_lut
            {
                assets.push(lut.clone());
            }
            if let Some(ae) = &output.auto_exposure {
                if let Some(mask) = &ae.metering_mask {
                    assets.push(mask.clone());
                }
                if let Some(curve) = &ae.compensation_curve {
                    assets.push(curve.clone());
                }
            }
        }
        assets
    }

    #[instrument(skip_all)]
    fn view_material_families(&self, _view: &RenderViewDescriptor) -> Vec<RenderMaterialFamily> {
        Vec::new()
    }

    #[instrument(skip_all)]
    fn layer_asset_dependencies(&self, layer: &RenderLayerDescriptor) -> Vec<RenderAssetReference> {
        match layer {
            RenderLayerDescriptor::Vector(_) => Vec::new(),
            RenderLayerDescriptor::Raster(l) => vec![l.source.asset.clone()],
            RenderLayerDescriptor::Tile(_) => Vec::new(),
            RenderLayerDescriptor::Terrain(l) => vec![l.source.asset.clone()],
            RenderLayerDescriptor::Annotation(_) => Vec::new(),
            RenderLayerDescriptor::Model(l) => {
                vec![l.asset.clone()]
            }
        }
    }

    #[instrument(skip_all)]
    fn layer_material_families(&self, layer: &RenderLayerDescriptor) -> Vec<RenderMaterialFamily> {
        match layer {
            RenderLayerDescriptor::Model(l) => l
                .material_override
                .as_ref()
                .map(|m| vec![m.family.clone()])
                .unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    #[instrument(skip_all)]
    fn layer_pipeline_domains(&self, layer: &RenderLayerDescriptor) -> Vec<RenderPipelineDomain> {
        match layer {
            RenderLayerDescriptor::Model(l) => l
                .material_override
                .as_ref()
                .and_then(|m| m.pipeline_intent.as_ref())
                .map(|p| vec![p.domain.clone()])
                .unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    #[instrument(skip_all)]
    fn layer_prepasses(&self, layer: &RenderLayerDescriptor) -> Vec<RenderPrepassKind> {
        match layer {
            RenderLayerDescriptor::Model(l) => l
                .material_override
                .as_ref()
                .and_then(|m| m.pipeline_intent.as_ref())
                .map(|p| p.prepasses.clone())
                .unwrap_or_default(),
            _ => Vec::new(),
        }
    }

    #[instrument(skip_all)]
    fn scene_asset_dependencies(&self, scene: &RenderSceneDescriptor) -> Vec<RenderAssetReference> {
        let mut assets: Vec<RenderAssetReference> = scene
            .layers
            .iter()
            .flat_map(|l| self.layer_asset_dependencies(l))
            .collect();
        assets.extend(self.view_asset_dependencies(&scene.view));
        for av in &scene.auxiliary_views {
            assets.extend(self.view_asset_dependencies(av));
        }
        assets
    }

    #[instrument(skip_all)]
    fn scene_material_families(&self, scene: &RenderSceneDescriptor) -> Vec<RenderMaterialFamily> {
        scene
            .layers
            .iter()
            .flat_map(|l| self.layer_material_families(l))
            .collect()
    }

    #[instrument(skip_all)]
    fn scene_pipeline_domains(&self, scene: &RenderSceneDescriptor) -> Vec<RenderPipelineDomain> {
        scene
            .layers
            .iter()
            .flat_map(|l| self.layer_pipeline_domains(l))
            .collect()
    }

    #[instrument(skip_all)]
    fn scene_prepasses(&self, scene: &RenderSceneDescriptor) -> Vec<RenderPrepassKind> {
        scene
            .layers
            .iter()
            .flat_map(|l| self.layer_prepasses(l))
            .collect()
    }
}

// ── GisRenderSceneUpdateMeta ──────────────────────────────────────────────────

impl GisRenderSceneUpdateMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn update_scene_id<'a>(&self, update: &'a RenderSceneUpdateDescriptor) -> &'a str {
        match update {
            RenderSceneUpdateDescriptor::ReplaceEnvironment { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::ClearEnvironment { scene_id } => scene_id,
            RenderSceneUpdateDescriptor::ReplaceView { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::InsertAuxiliaryView { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::ReplaceAuxiliaryView { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::RemoveAuxiliaryView { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::InsertLayer { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::ReplaceLayer { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::RemoveLayer { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::MoveLayerBefore { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::SetLayerVisibility { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::SetLayerOpacity { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::SetLayerDrawOrder { scene_id, .. } => scene_id,
            RenderSceneUpdateDescriptor::SetLayerViewParticipation { scene_id, .. } => scene_id,
        }
    }

    #[instrument(skip_all)]
    fn update_kind(&self, update: &RenderSceneUpdateDescriptor) -> RenderSceneUpdateKind {
        match update {
            RenderSceneUpdateDescriptor::ReplaceEnvironment { .. } => {
                RenderSceneUpdateKind::ReplaceEnvironment
            }
            RenderSceneUpdateDescriptor::ClearEnvironment { .. } => {
                RenderSceneUpdateKind::ClearEnvironment
            }
            RenderSceneUpdateDescriptor::ReplaceView { .. } => RenderSceneUpdateKind::ReplaceView,
            RenderSceneUpdateDescriptor::InsertAuxiliaryView { .. } => {
                RenderSceneUpdateKind::InsertAuxiliaryView
            }
            RenderSceneUpdateDescriptor::ReplaceAuxiliaryView { .. } => {
                RenderSceneUpdateKind::ReplaceAuxiliaryView
            }
            RenderSceneUpdateDescriptor::RemoveAuxiliaryView { .. } => {
                RenderSceneUpdateKind::RemoveAuxiliaryView
            }
            RenderSceneUpdateDescriptor::InsertLayer { .. } => RenderSceneUpdateKind::InsertLayer,
            RenderSceneUpdateDescriptor::ReplaceLayer { .. } => RenderSceneUpdateKind::ReplaceLayer,
            RenderSceneUpdateDescriptor::RemoveLayer { .. } => RenderSceneUpdateKind::RemoveLayer,
            RenderSceneUpdateDescriptor::MoveLayerBefore { .. } => {
                RenderSceneUpdateKind::MoveLayerBefore
            }
            RenderSceneUpdateDescriptor::SetLayerVisibility { .. } => {
                RenderSceneUpdateKind::SetLayerVisibility
            }
            RenderSceneUpdateDescriptor::SetLayerOpacity { .. } => {
                RenderSceneUpdateKind::SetLayerOpacity
            }
            RenderSceneUpdateDescriptor::SetLayerDrawOrder { .. } => {
                RenderSceneUpdateKind::SetLayerDrawOrder
            }
            RenderSceneUpdateDescriptor::SetLayerViewParticipation { .. } => {
                RenderSceneUpdateKind::SetLayerViewParticipation
            }
        }
    }

    #[instrument(skip_all)]
    fn update_target_layer_id<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a str> {
        match update {
            RenderSceneUpdateDescriptor::ReplaceLayer {
                target_layer_id, ..
            } => Some(target_layer_id),
            RenderSceneUpdateDescriptor::RemoveLayer {
                target_layer_id, ..
            } => Some(target_layer_id),
            RenderSceneUpdateDescriptor::MoveLayerBefore {
                target_layer_id, ..
            } => Some(target_layer_id),
            RenderSceneUpdateDescriptor::SetLayerVisibility {
                target_layer_id, ..
            } => Some(target_layer_id),
            RenderSceneUpdateDescriptor::SetLayerOpacity {
                target_layer_id, ..
            } => Some(target_layer_id),
            RenderSceneUpdateDescriptor::SetLayerDrawOrder {
                target_layer_id, ..
            } => Some(target_layer_id),
            RenderSceneUpdateDescriptor::SetLayerViewParticipation {
                target_layer_id, ..
            } => Some(target_layer_id),
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_target_view_id<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a str> {
        match update {
            RenderSceneUpdateDescriptor::ReplaceAuxiliaryView { target_view_id, .. } => {
                Some(target_view_id)
            }
            RenderSceneUpdateDescriptor::RemoveAuxiliaryView { target_view_id, .. } => {
                Some(target_view_id)
            }
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_reference_layer_id<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a str> {
        match update {
            RenderSceneUpdateDescriptor::InsertLayer {
                before_layer_id, ..
            } => before_layer_id.as_deref(),
            RenderSceneUpdateDescriptor::MoveLayerBefore {
                before_layer_id, ..
            } => before_layer_id.as_deref(),
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_environment<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
        update_proof: Established<RenderableSceneUpdateValid>,
    ) -> Option<(
        &'a RenderSceneEnvironmentDescriptor,
        Established<RenderSceneEnvironmentValid>,
    )> {
        match update {
            RenderSceneUpdateDescriptor::ReplaceEnvironment { environment, .. } => {
                Some((environment, Established::prove(&update_proof)))
            }
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_view<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
        update_proof: Established<RenderableSceneUpdateValid>,
    ) -> Option<(&'a RenderViewDescriptor, Established<RenderViewValid>)> {
        match update {
            RenderSceneUpdateDescriptor::ReplaceView { view, .. } => {
                Some((view, Established::prove(&update_proof)))
            }
            RenderSceneUpdateDescriptor::InsertAuxiliaryView { view, .. } => {
                Some((view, Established::prove(&update_proof)))
            }
            RenderSceneUpdateDescriptor::ReplaceAuxiliaryView { view, .. } => {
                Some((view, Established::prove(&update_proof)))
            }
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_layer<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
        update_proof: Established<RenderableSceneUpdateValid>,
    ) -> Option<(&'a RenderLayerDescriptor, Established<RenderableLayerValid>)> {
        match update {
            RenderSceneUpdateDescriptor::InsertLayer { layer, .. } => {
                Some((layer, Established::prove(&update_proof)))
            }
            RenderSceneUpdateDescriptor::ReplaceLayer { layer, .. } => {
                Some((layer, Established::prove(&update_proof)))
            }
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_visibility(&self, update: &RenderSceneUpdateDescriptor) -> Option<bool> {
        match update {
            RenderSceneUpdateDescriptor::SetLayerVisibility { visible, .. } => Some(*visible),
            _ => None,
        }
    }

    #[instrument(skip_all)]
    fn update_opacity(
        &self,
        update: &RenderSceneUpdateDescriptor,
        update_proof: Established<RenderableSceneUpdateValid>,
    ) -> GisResult<Option<(f32, Established<LayerOpacityUnitInterval>)>> {
        match update {
            RenderSceneUpdateDescriptor::SetLayerOpacity { opacity, .. } => {
                Ok(Some((*opacity, Established::prove(&update_proof))))
            }
            _ => Ok(None),
        }
    }

    #[instrument(skip_all)]
    fn update_draw_order(
        &self,
        update: &RenderSceneUpdateDescriptor,
        update_proof: Established<RenderableSceneUpdateValid>,
    ) -> GisResult<Option<(i32, Established<LayerDrawOrderAssigned>)>> {
        match update {
            RenderSceneUpdateDescriptor::SetLayerDrawOrder { draw_order, .. } => {
                Ok(Some((*draw_order, Established::prove(&update_proof))))
            }
            _ => Ok(None),
        }
    }

    #[instrument(skip_all)]
    fn update_view_participation<'a>(
        &self,
        update: &'a RenderSceneUpdateDescriptor,
    ) -> Option<&'a RenderViewParticipationDescriptor> {
        match update {
            RenderSceneUpdateDescriptor::SetLayerViewParticipation {
                view_participation, ..
            } => Some(view_participation),
            _ => None,
        }
    }
}

// ── GisRenderSceneMeta ────────────────────────────────────────────────────────

impl GisRenderSceneMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn scene_id<'a>(&self, scene: &'a RenderSceneDescriptor) -> &'a str {
        &scene.scene_id
    }

    #[instrument(skip_all)]
    fn scene_layer_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.layers.len()
    }

    #[instrument(skip_all)]
    fn scene_vector_layer_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.vector_layer_count
    }

    #[instrument(skip_all)]
    fn scene_raster_layer_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.raster_layer_count
    }

    #[instrument(skip_all)]
    fn scene_tile_layer_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.tile_layer_count
    }

    #[instrument(skip_all)]
    fn scene_terrain_layer_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.terrain_layer_count
    }

    #[instrument(skip_all)]
    fn scene_annotation_layer_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.annotation_layer_count
    }

    #[instrument(skip_all)]
    fn scene_auxiliary_view_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene.auxiliary_view_count
    }

    #[instrument(skip_all)]
    fn scene_has_raster_layers(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.raster_layer_count > 0
    }

    #[instrument(skip_all)]
    fn scene_has_tile_layers(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.tile_layer_count > 0
    }

    #[instrument(skip_all)]
    fn scene_has_terrain_layers(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.terrain_layer_count > 0
    }

    #[instrument(skip_all)]
    fn scene_has_annotation_layers(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.annotation_layer_count > 0
    }

    #[instrument(skip_all)]
    fn scene_has_auxiliary_views(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.auxiliary_view_count > 0
    }

    #[instrument(skip_all)]
    fn scene_has_feature_labels(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.has_feature_labels
    }

    #[instrument(skip_all)]
    fn scene_has_environment(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.environment.is_some()
    }

    #[instrument(skip_all)]
    fn scene_has_fog(&self, scene: &RenderSceneDescriptor) -> bool {
        scene.environment.as_ref().is_some_and(|e| e.fog.is_some())
    }

    #[instrument(skip_all)]
    fn scene_has_volumetric_fog(&self, scene: &RenderSceneDescriptor) -> bool {
        scene
            .environment
            .as_ref()
            .is_some_and(|e| e.volumetric_fog.is_some())
    }

    #[instrument(skip_all)]
    fn scene_has_image_based_lighting(&self, scene: &RenderSceneDescriptor) -> bool {
        scene
            .environment
            .as_ref()
            .is_some_and(|e| e.image_based_lighting.is_some())
    }

    #[instrument(skip_all)]
    fn scene_has_atmosphere(&self, scene: &RenderSceneDescriptor) -> bool {
        scene
            .environment
            .as_ref()
            .is_some_and(|e| e.atmosphere.is_some())
    }

    #[instrument(skip_all)]
    fn scene_has_light_probes(&self, scene: &RenderSceneDescriptor) -> bool {
        scene
            .environment
            .as_ref()
            .is_some_and(|e| !e.light_probes.is_empty())
    }

    #[instrument(skip_all)]
    fn scene_light_probe_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene
            .environment
            .as_ref()
            .map_or(0, |e| e.light_probes.len())
    }

    #[instrument(skip_all)]
    fn scene_has_sun_disk(&self, scene: &RenderSceneDescriptor) -> bool {
        scene
            .environment
            .as_ref()
            .and_then(|e| e.atmosphere.as_ref())
            .is_some_and(|a| a.sun_disk.is_some())
    }

    #[instrument(skip_all)]
    fn scene_fog_volume_count(&self, scene: &RenderSceneDescriptor) -> usize {
        scene
            .environment
            .as_ref()
            .map_or(0, |e| e.fog_volumes.len())
    }

    #[instrument(skip_all)]
    fn scene_layer_order_is_deterministic(
        &self,
        scene: &RenderSceneDescriptor,
        scene_proof: Established<RenderableSceneValid>,
    ) -> GisResult<Established<LayerOrderDeterministic>> {
        tracing::debug!(
            layer_count = scene.layers.len(),
            "confirming layer order determinism"
        );
        Ok(Established::prove(&scene_proof))
    }
}

// ── GisRenderTimeMeta ─────────────────────────────────────────────────────────

impl GisRenderTimeMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn scene_time_context<'a>(
        &self,
        scene: &'a RenderSceneDescriptor,
    ) -> Option<&'a RenderTimeContext> {
        scene.time_context.as_ref()
    }

    #[instrument(skip_all)]
    fn layer_time_filter<'a>(
        &self,
        _layer: &'a RenderLayerDescriptor,
    ) -> Option<&'a RenderLayerTimeFilter> {
        None
    }

    #[instrument(skip_all)]
    fn layer_is_visible_at_time(
        &self,
        layer: &RenderLayerDescriptor,
        context: &RenderTimeContext,
    ) -> bool {
        match self.layer_time_filter(layer) {
            None => true,
            Some(filter) => {
                let after_start = filter
                    .valid_from_ms
                    .is_none_or(|from| context.end_ms >= from);
                let before_end = filter
                    .valid_until_ms
                    .is_none_or(|until| context.start_ms <= until);
                after_start && before_end
            }
        }
    }
}
