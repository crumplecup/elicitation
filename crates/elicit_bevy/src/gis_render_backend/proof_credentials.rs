//! `pub(crate)` mint credentials for propositions the Bevy GIS backend establishes.
//!
//! Each entry names a credential type and the proposition it proves.  Credentials are
//! `pub(crate)` zero-sized types — external code cannot construct them, so the only
//! way to produce an `Established<P>` is to go through the factory or meta method
//! that holds the corresponding credential.  The method body is the bounded audit surface.

use elicitation::proof_credential;

use elicit_gis::{
    AnnotationPayloadDeclared, GeometryProjectedForView, LayerDrawOrderAssigned,
    LayerGroupIdNonEmpty, LayerGroupNameNonEmpty, LayerGroupOpacityUnitInterval, LayerIdNonEmpty,
    LayerNameNonEmpty, LayerOpacityUnitInterval, LayerOrderDeterministic, LayerVisibilityDeclared,
    ModelAssetDeclared, ModelPlacementValid, RasterDimensionsPositive, RasterImageMetadataDeclared,
    RenderAnnotationValid, RenderFeatureBatchingSelectionValid, RenderRasterSourceValid,
    RenderSceneEnvironmentSelectionValid, RenderTileSourceValid, SceneAuxiliaryViewIdsUnique,
    SceneAuxiliaryViewsDeclared, SceneAuxiliaryViewsDistinctFromPrimary, SceneIdNonEmpty,
    SceneLayerListDeclared, SceneLayerViewParticipationResolved,
    SceneUpdateDrawOrderSelectionValid, SceneUpdateEnvironmentSelectionValid,
    SceneUpdateLayerSelectionValid, SceneUpdateOpacitySelectionValid, SceneUpdateOperationDeclared,
    SceneUpdateReferenceLayerSelectionValid, SceneUpdateTargetLayerNonEmpty,
    SceneUpdateTargetLayerSelectionValid, SceneUpdateTargetViewSelectionValid,
    SceneUpdateViewSelectionValid, SceneViewDeclared, VectorPayloadDeclared,
    VectorStylePayloadCompatible, VectorSymbolizationResolved,
};

// ── Layer spec structural proofs ──────────────────────────────────────────────
//
// Proven when `validate_layer_spec` returns `Ok(())`.

proof_credential! {
    /// Bevy confirmed the layer identifier field is non-empty.
    pub(crate) BevyLayerIdNonEmpty => LayerIdNonEmpty;

    /// Bevy confirmed the layer name field is non-empty.
    pub(crate) BevyLayerNameNonEmpty => LayerNameNonEmpty;

    /// Bevy confirmed the layer opacity field is in [0.0, 1.0].
    pub(crate) BevyLayerOpacityUnitInterval => LayerOpacityUnitInterval;

    /// Bevy confirmed a concrete draw-order value is present on the layer spec.
    pub(crate) BevyLayerDrawOrderAssigned => LayerDrawOrderAssigned;

    /// Bevy confirmed a concrete visibility flag is present on the layer spec.
    pub(crate) BevyLayerVisibilityDeclared => LayerVisibilityDeclared;
}

// ── Layer group structural proofs ─────────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed the layer group identifier field is non-empty.
    pub(crate) BevyLayerGroupIdNonEmpty => LayerGroupIdNonEmpty;

    /// Bevy confirmed the layer group name field is non-empty.
    pub(crate) BevyLayerGroupNameNonEmpty => LayerGroupNameNonEmpty;

    /// Bevy confirmed the layer group opacity field is in [0.0, 1.0].
    pub(crate) BevyLayerGroupOpacityUnitInterval => LayerGroupOpacityUnitInterval;
}

// ── Vector layer behavioral proofs ────────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed a vector payload (geometries, features, or records) was provided.
    pub(crate) BevyVectorPayloadPresent => VectorPayloadDeclared;

    /// Bevy confirmed geometry coordinates are in the view's projection space.
    pub(crate) BevyGeometryProjectedForView => GeometryProjectedForView;

    /// Bevy confirmed the vector symbolizer is fully resolved for this payload.
    pub(crate) BevyVectorSymbolizationResolved => VectorSymbolizationResolved;

    /// Bevy confirmed the feature style is compatible with the vector payload family.
    pub(crate) BevyVectorStylePayloadCompatible => VectorStylePayloadCompatible;

    /// Bevy confirmed the batching selection is valid for this layer.
    pub(crate) BevyBatchingSelectionValid => RenderFeatureBatchingSelectionValid;
}

// ── Raster layer behavioral proofs ────────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed the raster source descriptor is valid.
    pub(crate) BevyRasterSourceValid => RenderRasterSourceValid;

    /// Bevy confirmed raster image metadata was provided.
    pub(crate) BevyRasterImageMetadataDeclared => RasterImageMetadataDeclared;

    /// Bevy confirmed raster dimensions are positive when declared.
    pub(crate) BevyRasterDimensionsPositive => RasterDimensionsPositive;
}

// ── Tile layer behavioral proofs ──────────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed the tile source descriptor carries a non-empty URI template.
    pub(crate) BevyTileSourceValid => RenderTileSourceValid;
}

// ── Annotation layer behavioral proofs ───────────────────────────────────────

proof_credential! {
    /// Bevy confirmed at least one annotation was provided in the payload.
    pub(crate) BevyAnnotationPayloadDeclared => AnnotationPayloadDeclared;

    /// Bevy confirmed an individual annotation entry is renderable.
    pub(crate) BevyAnnotationItemValid => RenderAnnotationValid;
}

// ── Model layer behavioral proofs ─────────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed the model asset URI is non-empty.
    pub(crate) BevyModelAssetDeclared => ModelAssetDeclared;

    /// Bevy confirmed the model placement descriptor has finite, positive values.
    pub(crate) BevyModelPlacementValid => ModelPlacementValid;
}

// ── Scene factory structural proofs ──────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed the scene identifier is non-empty.
    pub(crate) BevySceneIdNonEmpty => SceneIdNonEmpty;

    /// Bevy confirmed the scene layer list is explicitly declared.
    pub(crate) BevySceneLayerListDeclared => SceneLayerListDeclared;

    /// Bevy confirmed the scene primary view is declared.
    pub(crate) BevySceneViewDeclared => SceneViewDeclared;

    /// Bevy confirmed the auxiliary view list is explicitly declared.
    pub(crate) BevySceneAuxiliaryViewsDeclared => SceneAuxiliaryViewsDeclared;

    /// Bevy confirmed auxiliary view identifiers are unique.
    pub(crate) BevySceneAuxiliaryViewIdsUnique => SceneAuxiliaryViewIdsUnique;

    /// Bevy confirmed auxiliary view identifiers differ from the primary view identifier.
    pub(crate) BevySceneAuxiliaryViewsDistinctFromPrimary => SceneAuxiliaryViewsDistinctFromPrimary;

    /// Bevy confirmed every layer's view-routing references resolve against scene views.
    pub(crate) BevySceneLayerViewParticipationResolved => SceneLayerViewParticipationResolved;

    /// Bevy confirmed the scene environment selection (absent or validated present) is valid.
    pub(crate) BevySceneEnvironmentSelectionValid => RenderSceneEnvironmentSelectionValid;
}

// ── Scene update structural proofs ────────────────────────────────────────────
//
// Used for update-evidence fields where the factory operation does not involve
// a particular selection (e.g., a view-replace has no layer payload).

proof_credential! {
    /// Bevy confirmed the update operation kind was explicitly declared.
    pub(crate) BevySceneUpdateOperationDeclared => SceneUpdateOperationDeclared;

    /// Bevy confirmed the target-layer selection is valid for this update operation.
    pub(crate) BevySceneUpdateTargetLayerSelectionValid => SceneUpdateTargetLayerSelectionValid;

    /// Bevy confirmed the target-view selection is valid for this update operation.
    pub(crate) BevySceneUpdateTargetViewSelectionValid => SceneUpdateTargetViewSelectionValid;

    /// Bevy confirmed the reference-layer selection is valid for this update operation.
    pub(crate) BevySceneUpdateReferenceLayerSelectionValid => SceneUpdateReferenceLayerSelectionValid;

    /// Bevy confirmed the view-payload selection is valid for this update operation.
    pub(crate) BevySceneUpdateViewSelectionValid => SceneUpdateViewSelectionValid;

    /// Bevy confirmed the environment-payload selection is valid for this update operation.
    pub(crate) BevySceneUpdateEnvironmentSelectionValid => SceneUpdateEnvironmentSelectionValid;

    /// Bevy confirmed the layer-payload selection is valid for this update operation.
    pub(crate) BevySceneUpdateLayerSelectionValid => SceneUpdateLayerSelectionValid;

    /// Bevy confirmed the opacity-payload selection is valid for this update operation.
    pub(crate) BevySceneUpdateOpacitySelectionValid => SceneUpdateOpacitySelectionValid;

    /// Bevy confirmed the draw-order-payload selection is valid for this update operation.
    pub(crate) BevySceneUpdateDrawOrderSelectionValid => SceneUpdateDrawOrderSelectionValid;

    /// Bevy confirmed the target-layer identifier is non-empty for this update operation.
    pub(crate) BevySceneUpdateTargetLayerNonEmpty => SceneUpdateTargetLayerNonEmpty;
}

// ── Meta: layer observability proofs ─────────────────────────────────────────

proof_credential! {
    /// Bevy confirmed deterministic layer ordering by inspecting a factory-built
    /// scene descriptor.
    pub(crate) BevyLayerOrderConfirmed => LayerOrderDeterministic;
}
