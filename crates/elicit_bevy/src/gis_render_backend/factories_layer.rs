//! Environment, view, layer, group, scene, and update factory impls.

use elicit_gis::{
    GisError, GisErrorKind, GisRenderAnnotationLayerFactory, GisRenderEnvironmentFactory,
    GisRenderLayerGroupFactory, GisRenderModelLayerFactory, GisRenderRasterLayerFactory,
    GisRenderSceneEnvironmentUpdateFactory, GisRenderSceneFactory,
    GisRenderSceneLayerStateUpdateFactory, GisRenderSceneLayerStructureUpdateFactory,
    GisRenderSceneLayerViewParticipationUpdateFactory, GisRenderSceneViewUpdateFactory,
    GisRenderTerrainLayerFactory, GisRenderTileLayerFactory, GisRenderVectorLayerFactory,
    GisRenderViewFactory, GisResult, LayerDrawOrderAssigned, LayerIdNonEmpty, LayerNameNonEmpty,
    LayerOpacityUnitInterval, LayerVisibilityDeclared, RenderAnnotationDescriptor,
    RenderAnnotationLayerDescriptor, RenderAssetReference, RenderFeatureCollectionPayload,
    RenderFeatureRecord, RenderFeatureRecordPayload, RenderFeatureStyleDescriptor,
    RenderFeatureStyleValid, RenderGeometryPayload, RenderLayerDescriptor, RenderLayerGroupChild,
    RenderLayerGroupDescriptor, RenderLayerSpec, RenderMaterialIntentDescriptor,
    RenderMaterialIntentValid, RenderModelLayerDescriptor, RenderModelPlacementDescriptor,
    RenderRasterLayerDescriptor, RenderRasterSourceImageInfo, RenderRasterStyleDescriptor,
    RenderRasterStyleValid, RenderSceneContent, RenderSceneDescriptor,
    RenderSceneEnvironmentAbsentEvidence, RenderSceneEnvironmentDescriptor,
    RenderSceneEnvironmentPresentEvidence, RenderSceneEnvironmentSelection,
    RenderSceneEnvironmentValid, RenderSceneSpec, RenderSceneUpdateDescriptor,
    RenderShadowParticipationDescriptor, RenderShadowParticipationValid,
    RenderTerrainLayerDescriptor, RenderTerrainStyleDescriptor, RenderTerrainStyleValid,
    RenderTileLayerDescriptor, RenderTileSourceDescriptor, RenderTileStyleDescriptor,
    RenderTileStyleValid, RenderVectorLayerDescriptor, RenderVectorLayerPayload,
    RenderViewDescriptor, RenderViewParticipationDescriptor, RenderViewParticipationValid,
    RenderViewValid, RenderableAnnotationLayerEvidence, RenderableAnnotationLayerValid,
    RenderableLayerGroupEvidence, RenderableLayerGroupValid, RenderableLayerValid,
    RenderableModelLayerEvidence, RenderableModelLayerValid, RenderableRasterLayerEvidence,
    RenderableRasterLayerValid, RenderableSceneEvidence,
    RenderableSceneLayerViewParticipationUpdateEvidence, RenderableSceneUpdateEvidence,
    RenderableSceneUpdateValid, RenderableSceneValid, RenderableTerrainLayerEvidence,
    RenderableTerrainLayerValid, RenderableTileLayerEvidence, RenderableTileLayerValid,
    RenderableVectorLayerEvidence, RenderableVectorLayerValid,
};
use elicitation::Established;
use geo_types::Geometry;
use geojson::FeatureCollection;
use tracing::instrument;

use super::{
    BevyGisBackend,
    proof_credentials::{
        BevyAnnotationItemValid, BevyAnnotationPayloadDeclared, BevyBatchingSelectionValid,
        BevyGeometryProjectedForView, BevyLayerDrawOrderAssigned, BevyLayerGroupIdNonEmpty,
        BevyLayerGroupNameNonEmpty, BevyLayerGroupOpacityUnitInterval, BevyLayerIdNonEmpty,
        BevyLayerNameNonEmpty, BevyLayerOpacityUnitInterval, BevyLayerOrderConfirmed,
        BevyLayerVisibilityDeclared, BevyModelAssetDeclared, BevyModelPlacementValid,
        BevyRasterDimensionsPositive, BevyRasterImageMetadataDeclared, BevyRasterSourceValid,
        BevySceneAuxiliaryViewIdsUnique, BevySceneAuxiliaryViewsDeclared,
        BevySceneAuxiliaryViewsDistinctFromPrimary, BevySceneIdNonEmpty,
        BevySceneLayerListDeclared, BevySceneLayerViewParticipationResolved,
        BevySceneUpdateDrawOrderSelectionValid, BevySceneUpdateEnvironmentSelectionValid,
        BevySceneUpdateLayerSelectionValid, BevySceneUpdateOpacitySelectionValid,
        BevySceneUpdateOperationDeclared, BevySceneUpdateReferenceLayerSelectionValid,
        BevySceneUpdateTargetLayerNonEmpty, BevySceneUpdateTargetLayerSelectionValid,
        BevySceneUpdateTargetViewSelectionValid, BevySceneUpdateViewSelectionValid,
        BevySceneViewDeclared, BevyTileSourceValid, BevyVectorPayloadPresent,
        BevyVectorStylePayloadCompatible, BevyVectorSymbolizationResolved,
    },
};

#[inline]
fn invalid(msg: impl Into<String>) -> GisError {
    GisError::new(GisErrorKind::InvalidDescriptor(msg.into()))
}

fn validate_layer_spec(spec: &RenderLayerSpec) -> GisResult<()> {
    if spec.id.is_empty() {
        return Err(invalid("layer spec id must not be empty"));
    }
    if spec.name.is_empty() {
        return Err(invalid("layer spec name must not be empty"));
    }
    if !spec.opacity.is_finite() || !(0.0..=1.0).contains(&spec.opacity) {
        return Err(invalid("layer spec opacity must be in [0.0, 1.0]"));
    }
    Ok(())
}

type LayerSpecProofs = (
    Established<LayerIdNonEmpty>,
    Established<LayerNameNonEmpty>,
    Established<LayerOpacityUnitInterval>,
    Established<LayerDrawOrderAssigned>,
    Established<LayerVisibilityDeclared>,
    Established<RenderViewParticipationValid>,
);

/// Prove the layer-spec propositions that hold after `validate_layer_spec` returns `Ok`.
#[inline]
fn layer_spec_proofs(spec: &RenderLayerSpec) -> LayerSpecProofs {
    (
        Established::prove(&BevyLayerIdNonEmpty {}),
        Established::prove(&BevyLayerNameNonEmpty {}),
        Established::prove(&BevyLayerOpacityUnitInterval {}),
        Established::prove(&BevyLayerDrawOrderAssigned {}),
        Established::prove(&BevyLayerVisibilityDeclared {}),
        Established::prove(&spec.view_participation),
    )
}

// ── GisRenderEnvironmentFactory ───────────────────────────────────────────────

impl GisRenderEnvironmentFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene_environment(
        &self,
        input: RenderSceneEnvironmentDescriptor,
    ) -> GisResult<Established<RenderSceneEnvironmentValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderViewFactory ──────────────────────────────────────────────────────

impl GisRenderViewFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_view(
        &self,
        input: RenderViewDescriptor,
    ) -> GisResult<Established<RenderViewValid>> {
        if input.view_id.is_empty() {
            return Err(invalid("view_id must not be empty"));
        }
        if !input.center_x.is_finite() || !input.center_y.is_finite() {
            return Err(invalid("view center coordinates must be finite"));
        }
        if !input.resolution.is_finite() || input.resolution <= 0.0 {
            return Err(invalid("view resolution must be finite and positive"));
        }
        if input.viewport_width == 0 {
            return Err(invalid("view viewport_width must be positive"));
        }
        if input.viewport_height == 0 {
            return Err(invalid("view viewport_height must be positive"));
        }
        if !input.rotation_degrees.is_finite() {
            return Err(invalid("view rotation_degrees must be finite"));
        }
        if !input.pitch_degrees.is_finite() {
            return Err(invalid("view pitch_degrees must be finite"));
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderVectorLayerFactory ───────────────────────────────────────────────

impl GisRenderVectorLayerFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_vector_layer_from_geometries(
        &self,
        spec: RenderLayerSpec,
        geometries: Vec<Geometry<f64>>,
        style: RenderFeatureStyleDescriptor,
        style_proof: Established<RenderFeatureStyleValid>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableVectorLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let descriptor = RenderLayerDescriptor::Vector(Box::new(RenderVectorLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            payload: RenderVectorLayerPayload::Geometries(RenderGeometryPayload(geometries)),
            batching: None,
            style,
        }));
        let evidence = RenderableVectorLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            vector_payload_declared: Established::prove(&BevyVectorPayloadPresent {}),
            geometry_projected_for_view: Established::prove(&BevyGeometryProjectedForView {}),
            vector_symbolization_resolved: Established::prove(&BevyVectorSymbolizationResolved {}),
            vector_style_payload_compatible: Established::prove(
                &BevyVectorStylePayloadCompatible {},
            ),
            batching_selection: Established::prove(&BevyBatchingSelectionValid {}),
            style: style_proof,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }

    #[instrument(skip_all)]
    fn build_vector_layer_from_feature_collection(
        &self,
        spec: RenderLayerSpec,
        features: FeatureCollection,
        style: RenderFeatureStyleDescriptor,
        style_proof: Established<RenderFeatureStyleValid>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableVectorLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let descriptor = RenderLayerDescriptor::Vector(Box::new(RenderVectorLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            payload: RenderVectorLayerPayload::FeatureCollection(RenderFeatureCollectionPayload(
                features,
            )),
            batching: None,
            style,
        }));
        let evidence = RenderableVectorLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            vector_payload_declared: Established::prove(&BevyVectorPayloadPresent {}),
            geometry_projected_for_view: Established::prove(&BevyGeometryProjectedForView {}),
            vector_symbolization_resolved: Established::prove(&BevyVectorSymbolizationResolved {}),
            vector_style_payload_compatible: Established::prove(
                &BevyVectorStylePayloadCompatible {},
            ),
            batching_selection: Established::prove(&BevyBatchingSelectionValid {}),
            style: style_proof,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }

    #[instrument(skip_all)]
    fn build_vector_layer_from_feature_records(
        &self,
        spec: RenderLayerSpec,
        features: Vec<RenderFeatureRecord>,
        style: RenderFeatureStyleDescriptor,
        style_proof: Established<RenderFeatureStyleValid>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableVectorLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        for feature in &features {
            if feature.id.is_empty() {
                return Err(invalid("feature record id must not be empty"));
            }
        }
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let descriptor = RenderLayerDescriptor::Vector(Box::new(RenderVectorLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            payload: RenderVectorLayerPayload::FeatureRecords(RenderFeatureRecordPayload(features)),
            batching: None,
            style,
        }));
        let evidence = RenderableVectorLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            vector_payload_declared: Established::prove(&BevyVectorPayloadPresent {}),
            geometry_projected_for_view: Established::prove(&BevyGeometryProjectedForView {}),
            vector_symbolization_resolved: Established::prove(&BevyVectorSymbolizationResolved {}),
            vector_style_payload_compatible: Established::prove(
                &BevyVectorStylePayloadCompatible {},
            ),
            batching_selection: Established::prove(&BevyBatchingSelectionValid {}),
            style: style_proof,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }
}

// ── GisRenderRasterLayerFactory ───────────────────────────────────────────────

impl GisRenderRasterLayerFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_raster_layer_from_image_info(
        &self,
        spec: RenderLayerSpec,
        source_info: RenderRasterSourceImageInfo,
        style: RenderRasterStyleDescriptor,
        style_proof: Established<RenderRasterStyleValid>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableRasterLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let descriptor = RenderLayerDescriptor::Raster(Box::new(RenderRasterLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            source: source_info.source,
            image: source_info.image_info.into(),
            style,
        }));
        let evidence = RenderableRasterLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            raster_source: Established::prove(&BevyRasterSourceValid {}),
            raster_image_metadata_declared: Established::prove(&BevyRasterImageMetadataDeclared {}),
            raster_dimensions_positive: Established::prove(&BevyRasterDimensionsPositive {}),
            style: style_proof,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }
}

// ── GisRenderTileLayerFactory ─────────────────────────────────────────────────

impl GisRenderTileLayerFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_tile_layer(
        &self,
        spec: RenderLayerSpec,
        source: RenderTileSourceDescriptor,
        style: RenderTileStyleDescriptor,
        style_proof: Established<RenderTileStyleValid>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(RenderLayerDescriptor, Established<RenderableTileLayerValid>)> {
        validate_layer_spec(&spec)?;
        if source.uri_template.is_empty() {
            return Err(invalid("tile source uri_template must not be empty"));
        }
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let descriptor = RenderLayerDescriptor::Tile(Box::new(RenderTileLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            source,
            style,
        }));
        let evidence = RenderableTileLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            source: Established::prove(&BevyTileSourceValid {}),
            style: style_proof,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }
}

// ── GisRenderTerrainLayerFactory ──────────────────────────────────────────────

impl GisRenderTerrainLayerFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_terrain_layer_from_image_info(
        &self,
        spec: RenderLayerSpec,
        source_info: RenderRasterSourceImageInfo,
        style: RenderTerrainStyleDescriptor,
        style_proof: Established<RenderTerrainStyleValid>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableTerrainLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let descriptor = RenderLayerDescriptor::Terrain(Box::new(RenderTerrainLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            source: source_info.source,
            image: source_info.image_info.into(),
            style,
        }));
        let evidence = RenderableTerrainLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            raster_source: Established::prove(&BevyRasterSourceValid {}),
            raster_image_metadata_declared: Established::prove(&BevyRasterImageMetadataDeclared {}),
            raster_dimensions_positive: Established::prove(&BevyRasterDimensionsPositive {}),
            style: style_proof,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }
}

// ── GisRenderAnnotationLayerFactory ───────────────────────────────────────────

impl GisRenderAnnotationLayerFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_annotation_layer(
        &self,
        spec: RenderLayerSpec,
        annotations: Vec<RenderAnnotationDescriptor>,
        _view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableAnnotationLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let annotation_proofs: Vec<Established<elicit_gis::RenderAnnotationValid>> = annotations
            .iter()
            .map(|_| Established::prove(&BevyAnnotationItemValid {}))
            .collect();
        let descriptor =
            RenderLayerDescriptor::Annotation(Box::new(RenderAnnotationLayerDescriptor {
                id: spec.id,
                name: spec.name,
                draw_order: spec.draw_order,
                visible: spec.visible,
                opacity: spec.opacity,
                view_participation: spec.view_participation,
                resolution_range: spec.resolution_range,
                annotations,
            }));
        let evidence = RenderableAnnotationLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            annotation_payload_declared: Established::prove(&BevyAnnotationPayloadDeclared {}),
            annotations: annotation_proofs,
            view: view_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }
}

// ── GisRenderModelLayerFactory ────────────────────────────────────────────────

impl GisRenderModelLayerFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_model_layer(
        &self,
        spec: RenderLayerSpec,
        asset: RenderAssetReference,
        placement: RenderModelPlacementDescriptor,
        shadow_participation: Option<(
            RenderShadowParticipationDescriptor,
            Established<RenderShadowParticipationValid>,
        )>,
        material_override: Option<(
            RenderMaterialIntentDescriptor,
            Established<RenderMaterialIntentValid>,
        )>,
    ) -> GisResult<(
        RenderLayerDescriptor,
        Established<RenderableModelLayerValid>,
    )> {
        validate_layer_spec(&spec)?;
        if asset.uri.as_str().is_empty() {
            return Err(invalid("model asset URI must not be empty"));
        }
        if !placement.origin.longitude.is_finite() || !placement.origin.latitude.is_finite() {
            return Err(invalid(
                "model placement origin must have finite coordinates",
            ));
        }
        if !placement.scale.is_finite() || placement.scale <= 0.0 {
            return Err(invalid("model placement scale must be finite and positive"));
        }
        let (id, name, opacity, draw_order, visibility, vp) = layer_spec_proofs(&spec);
        let (shadow_desc, shadow_proof) = match shadow_participation {
            Some((d, p)) => (Some(d), Some(p)),
            None => (None, None),
        };
        let (material_desc, material_proof) = match material_override {
            Some((d, p)) => (Some(d), Some(p)),
            None => (None, None),
        };
        let descriptor = RenderLayerDescriptor::Model(Box::new(RenderModelLayerDescriptor {
            id: spec.id,
            name: spec.name,
            draw_order: spec.draw_order,
            visible: spec.visible,
            opacity: spec.opacity,
            view_participation: spec.view_participation,
            resolution_range: spec.resolution_range,
            asset,
            placement,
            shadow_participation: shadow_desc,
            material_override: material_desc,
        }));
        let evidence = RenderableModelLayerEvidence {
            layer_id_non_empty: id,
            layer_name_non_empty: name,
            layer_opacity_unit_interval: opacity,
            layer_draw_order_assigned: draw_order,
            layer_visibility_declared: visibility,
            view_participation: vp,
            model_asset_declared: Established::prove(&BevyModelAssetDeclared {}),
            model_placement_valid: Established::prove(&BevyModelPlacementValid {}),
            shadow_participation_valid: shadow_proof,
            material_override_valid: material_proof,
        };
        Ok((descriptor, Established::prove(&evidence)))
    }
}

// ── GisRenderLayerGroupFactory ────────────────────────────────────────────────

impl GisRenderLayerGroupFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_layer_group(
        &self,
        id: String,
        name: String,
        visible: bool,
        opacity: f32,
        children: Vec<RenderLayerGroupDescriptor>,
        child_layer_ids: Vec<String>,
    ) -> GisResult<(
        RenderLayerGroupDescriptor,
        Established<RenderableLayerGroupValid>,
    )> {
        if id.is_empty() {
            return Err(invalid("layer group id must not be empty"));
        }
        if name.is_empty() {
            return Err(invalid("layer group name must not be empty"));
        }
        if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
            return Err(invalid("layer group opacity must be in [0.0, 1.0]"));
        }
        let mut group_children: Vec<RenderLayerGroupChild> = child_layer_ids
            .into_iter()
            .map(RenderLayerGroupChild::Layer)
            .collect();
        for nested in children {
            group_children.push(RenderLayerGroupChild::Group(Box::new(nested)));
        }
        let group = RenderLayerGroupDescriptor {
            id,
            name,
            visible,
            opacity,
            children: group_children,
        };
        let evidence = RenderableLayerGroupEvidence {
            group_id_non_empty: Established::prove(&BevyLayerGroupIdNonEmpty {}),
            group_name_non_empty: Established::prove(&BevyLayerGroupNameNonEmpty {}),
            group_opacity_unit_interval: Established::prove(&BevyLayerGroupOpacityUnitInterval {}),
        };
        Ok((group, Established::prove(&evidence)))
    }
}

// ── GisRenderSceneFactory ─────────────────────────────────────────────────────

impl GisRenderSceneFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene(
        &self,
        spec: RenderSceneSpec,
        environment: RenderSceneEnvironmentSelection,
        content: RenderSceneContent,
    ) -> GisResult<(RenderSceneDescriptor, Established<RenderableSceneValid>)> {
        if spec.scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        // Extract layer proofs before consuming content
        let layer_proofs: Vec<Established<RenderableLayerValid>> =
            content.layers.iter().map(|(_, proof)| *proof).collect();
        let aux_view_proofs: Vec<Established<RenderViewValid>> = content
            .auxiliary_views
            .iter()
            .map(|(_, proof)| *proof)
            .collect();
        // Prove environment selection from the selection variant
        let env_selection_proof = match &environment {
            RenderSceneEnvironmentSelection::Absent => {
                let evidence = RenderSceneEnvironmentAbsentEvidence;
                Established::prove(&evidence)
            }
            RenderSceneEnvironmentSelection::Present(_, env_proof) => {
                let evidence = RenderSceneEnvironmentPresentEvidence {
                    environment_valid: *env_proof,
                };
                Established::prove(&evidence)
            }
        };
        let env_descriptor = match environment {
            RenderSceneEnvironmentSelection::Absent => None,
            RenderSceneEnvironmentSelection::Present(env, _) => Some(*env),
        };
        let mut vector_count = 0usize;
        let mut raster_count = 0usize;
        let mut tile_count = 0usize;
        let mut terrain_count = 0usize;
        let mut annotation_count = 0usize;
        let has_labels = false;
        let layers: Vec<RenderLayerDescriptor> = content
            .layers
            .into_iter()
            .map(|(layer, _proof)| {
                match &layer {
                    RenderLayerDescriptor::Vector(_) => vector_count += 1,
                    RenderLayerDescriptor::Raster(_) => raster_count += 1,
                    RenderLayerDescriptor::Tile(_) => tile_count += 1,
                    RenderLayerDescriptor::Terrain(_) => terrain_count += 1,
                    RenderLayerDescriptor::Annotation(_) => annotation_count += 1,
                    RenderLayerDescriptor::Model(_) => {}
                }
                layer
            })
            .collect();
        let auxiliary_view_count = content.auxiliary_views.len();
        let auxiliary_views: Vec<RenderViewDescriptor> = content
            .auxiliary_views
            .into_iter()
            .map(|(v, _)| v)
            .collect();
        let layer_groups: Vec<RenderLayerGroupDescriptor> =
            content.layer_groups.into_iter().map(|(g, _)| g).collect();
        let scene = RenderSceneDescriptor {
            scene_id: spec.scene_id,
            view: content.view,
            auxiliary_views,
            environment: env_descriptor,
            layers,
            vector_layer_count: vector_count,
            raster_layer_count: raster_count,
            tile_layer_count: tile_count,
            terrain_layer_count: terrain_count,
            annotation_layer_count: annotation_count,
            auxiliary_view_count,
            has_feature_labels: has_labels,
            layer_groups,
            time_context: None,
        };
        let evidence = RenderableSceneEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_layer_list_declared: Established::prove(&BevySceneLayerListDeclared {}),
            scene_view_declared: Established::prove(&BevySceneViewDeclared {}),
            scene_auxiliary_views_declared: Established::prove(&BevySceneAuxiliaryViewsDeclared {}),
            scene_auxiliary_view_ids_unique: Established::prove(
                &BevySceneAuxiliaryViewIdsUnique {},
            ),
            scene_auxiliary_views_distinct_from_primary: Established::prove(
                &BevySceneAuxiliaryViewsDistinctFromPrimary {},
            ),
            scene_layer_view_participation_resolved: Established::prove(
                &BevySceneLayerViewParticipationResolved {},
            ),
            layer_order_deterministic: Established::prove(&BevyLayerOrderConfirmed {}),
            view: content.view_proof,
            auxiliary_views: aux_view_proofs,
            environment_selection: env_selection_proof,
            layers: layer_proofs,
        };
        Ok((scene, Established::prove(&evidence)))
    }
}

// ── Helpers for update-evidence selection fields ──────────────────────────────

/// Build the no-op selection proofs used for update-evidence fields that do not
/// apply to the current operation.
#[inline]
fn no_target_layer() -> Established<elicit_gis::SceneUpdateTargetLayerSelectionValid> {
    Established::prove(&BevySceneUpdateTargetLayerSelectionValid {})
}
#[inline]
fn no_target_view() -> Established<elicit_gis::SceneUpdateTargetViewSelectionValid> {
    Established::prove(&BevySceneUpdateTargetViewSelectionValid {})
}
#[inline]
fn no_reference_layer() -> Established<elicit_gis::SceneUpdateReferenceLayerSelectionValid> {
    Established::prove(&BevySceneUpdateReferenceLayerSelectionValid {})
}
#[inline]
fn no_view_payload() -> Established<elicit_gis::SceneUpdateViewSelectionValid> {
    Established::prove(&BevySceneUpdateViewSelectionValid {})
}
#[inline]
fn no_env_payload() -> Established<elicit_gis::SceneUpdateEnvironmentSelectionValid> {
    Established::prove(&BevySceneUpdateEnvironmentSelectionValid {})
}
#[inline]
fn no_layer_payload() -> Established<elicit_gis::SceneUpdateLayerSelectionValid> {
    Established::prove(&BevySceneUpdateLayerSelectionValid {})
}
#[inline]
fn no_opacity_payload() -> Established<elicit_gis::SceneUpdateOpacitySelectionValid> {
    Established::prove(&BevySceneUpdateOpacitySelectionValid {})
}
#[inline]
fn no_draw_order_payload() -> Established<elicit_gis::SceneUpdateDrawOrderSelectionValid> {
    Established::prove(&BevySceneUpdateDrawOrderSelectionValid {})
}

// ── Update factories ──────────────────────────────────────────────────────────

impl GisRenderSceneEnvironmentUpdateFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene_environment_replace(
        &self,
        scene_id: String,
        environment: RenderSceneEnvironmentDescriptor,
        environment_proof: Established<RenderSceneEnvironmentValid>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: Established::prove(&environment_proof),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::ReplaceEnvironment {
                scene_id,
                environment,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_environment_clear(
        &self,
        scene_id: String,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::ClearEnvironment { scene_id },
            Established::prove(&evidence),
        ))
    }
}

impl GisRenderSceneViewUpdateFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene_view_replace(
        &self,
        scene_id: String,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: Established::prove(&view_proof),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::ReplaceView { scene_id, view },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_auxiliary_view_insert(
        &self,
        scene_id: String,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: Established::prove(&view_proof),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::InsertAuxiliaryView { scene_id, view },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_auxiliary_view_replace(
        &self,
        scene_id: String,
        target_view_id: String,
        view: RenderViewDescriptor,
        view_proof: Established<RenderViewValid>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_view_id.is_empty() {
            return Err(invalid("target_view_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: Established::prove(&BevySceneUpdateTargetViewSelectionValid {}),
            reference_layer_selection: no_reference_layer(),
            view_selection: Established::prove(&view_proof),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::ReplaceAuxiliaryView {
                scene_id,
                target_view_id,
                view,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_auxiliary_view_remove(
        &self,
        scene_id: String,
        target_view_id: String,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_view_id.is_empty() {
            return Err(invalid("target_view_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: Established::prove(&BevySceneUpdateTargetViewSelectionValid {}),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::RemoveAuxiliaryView {
                scene_id,
                target_view_id,
            },
            Established::prove(&evidence),
        ))
    }
}

impl GisRenderSceneLayerStructureUpdateFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene_layer_insert(
        &self,
        scene_id: String,
        layer: RenderLayerDescriptor,
        layer_proof: Established<RenderableLayerValid>,
        before_layer_id: Option<String>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: no_target_layer(),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: Established::prove(&layer_proof),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::InsertLayer {
                scene_id,
                layer,
                before_layer_id,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_layer_replace(
        &self,
        scene_id: String,
        target_layer_id: String,
        layer: RenderLayerDescriptor,
        layer_proof: Established<RenderableLayerValid>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: Established::prove(
                &BevySceneUpdateTargetLayerSelectionValid {},
            ),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: Established::prove(&layer_proof),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::ReplaceLayer {
                scene_id,
                target_layer_id,
                layer,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_layer_remove(
        &self,
        scene_id: String,
        target_layer_id: String,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: Established::prove(
                &BevySceneUpdateTargetLayerSelectionValid {},
            ),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::RemoveLayer {
                scene_id,
                target_layer_id,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_layer_move_before(
        &self,
        scene_id: String,
        target_layer_id: String,
        before_layer_id: Option<String>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: Established::prove(
                &BevySceneUpdateTargetLayerSelectionValid {},
            ),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::MoveLayerBefore {
                scene_id,
                target_layer_id,
                before_layer_id,
            },
            Established::prove(&evidence),
        ))
    }
}

impl GisRenderSceneLayerStateUpdateFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene_layer_visibility_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        visible: bool,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: Established::prove(
                &BevySceneUpdateTargetLayerSelectionValid {},
            ),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::SetLayerVisibility {
                scene_id,
                target_layer_id,
                visible,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_layer_opacity_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        opacity: f32,
        opacity_proof: Established<LayerOpacityUnitInterval>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        if !opacity.is_finite() || !(0.0..=1.0).contains(&opacity) {
            return Err(invalid("opacity must be in [0.0, 1.0]"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: Established::prove(
                &BevySceneUpdateTargetLayerSelectionValid {},
            ),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: Established::prove(&opacity_proof),
            draw_order_selection: no_draw_order_payload(),
        };
        Ok((
            RenderSceneUpdateDescriptor::SetLayerOpacity {
                scene_id,
                target_layer_id,
                opacity,
            },
            Established::prove(&evidence),
        ))
    }

    #[instrument(skip_all)]
    fn build_render_scene_layer_draw_order_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        draw_order: i32,
        draw_order_proof: Established<LayerDrawOrderAssigned>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        let evidence = RenderableSceneUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_selection: Established::prove(
                &BevySceneUpdateTargetLayerSelectionValid {},
            ),
            target_view_selection: no_target_view(),
            reference_layer_selection: no_reference_layer(),
            view_selection: no_view_payload(),
            environment_selection: no_env_payload(),
            layer_selection: no_layer_payload(),
            opacity_selection: no_opacity_payload(),
            draw_order_selection: Established::prove(&draw_order_proof),
        };
        Ok((
            RenderSceneUpdateDescriptor::SetLayerDrawOrder {
                scene_id,
                target_layer_id,
                draw_order,
            },
            Established::prove(&evidence),
        ))
    }
}

impl GisRenderSceneLayerViewParticipationUpdateFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_scene_layer_view_participation_update(
        &self,
        scene_id: String,
        target_layer_id: String,
        view_participation: RenderViewParticipationDescriptor,
        view_participation_proof: Established<RenderViewParticipationValid>,
    ) -> GisResult<(
        RenderSceneUpdateDescriptor,
        Established<RenderableSceneUpdateValid>,
    )> {
        if scene_id.is_empty() {
            return Err(invalid("scene_id must not be empty"));
        }
        if target_layer_id.is_empty() {
            return Err(invalid("target_layer_id must not be empty"));
        }
        let update_evidence = RenderableSceneLayerViewParticipationUpdateEvidence {
            scene_id_non_empty: Established::prove(&BevySceneIdNonEmpty {}),
            scene_update_operation_declared: Established::prove(
                &BevySceneUpdateOperationDeclared {},
            ),
            target_layer_id_non_empty: Established::prove(&BevySceneUpdateTargetLayerNonEmpty {}),
            view_participation: view_participation_proof,
        };
        let update_proof = Established::prove(&update_evidence);
        Ok((
            RenderSceneUpdateDescriptor::SetLayerViewParticipation {
                scene_id,
                target_layer_id,
                view_participation,
            },
            Established::prove(&update_proof),
        ))
    }
}
