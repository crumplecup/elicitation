//! Stateful plugin providing runtime GIS render context.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tracing::instrument;

use crate::BevyGisBackend;

/// Runtime tile-load state for one layer within a scene.
#[derive(Debug, Clone, Default)]
pub(crate) struct BevyGisTileLayerState {
    /// Number of tile requests currently in flight.
    pub requests_in_flight: u32,
    /// Number of tiles currently held in the backend tile cache.
    pub cache_utilization: u32,
}

/// Runtime state for one active GIS render scene.
#[derive(Debug, Default)]
pub(crate) struct BevyGisSceneState {
    /// Per tile-layer streaming state, keyed by layer id.
    pub tile_layers: HashMap<String, BevyGisTileLayerState>,
}

/// Shared runtime context for the Bevy GIS render backend.
///
/// Stores per-scene mutable state (tile streaming, picking, projection) that
/// cannot be derived from the immutable render IR descriptors alone.
#[derive(Debug, Default)]
pub struct BevyGisRenderCtx {
    pub(crate) scenes: Mutex<HashMap<String, BevyGisSceneState>>,
}

impl BevyGisRenderCtx {
    /// Create an empty context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a scene so tile-streaming state can be tracked for it.
    #[instrument(skip(self))]
    pub fn register_scene(&self, scene_id: &str) {
        let mut scenes = self.scenes.lock().unwrap_or_else(|p| {
            tracing::error!("GIS render context mutex poisoned");
            p.into_inner()
        });
        scenes.entry(scene_id.to_owned()).or_default();
    }

    /// Remove a scene from the context.
    #[instrument(skip(self))]
    pub fn unregister_scene(&self, scene_id: &str) {
        let mut scenes = self.scenes.lock().unwrap_or_else(|p| {
            tracing::error!("GIS render context mutex poisoned");
            p.into_inner()
        });
        scenes.remove(scene_id);
    }

    /// Report that tiles are in flight for a layer.
    #[instrument(skip(self))]
    pub fn set_tile_requests_in_flight(&self, scene_id: &str, layer_id: &str, count: u32) {
        let mut scenes = self.scenes.lock().unwrap_or_else(|p| {
            tracing::error!("GIS render context mutex poisoned");
            p.into_inner()
        });
        let scene = scenes.entry(scene_id.to_owned()).or_default();
        scene
            .tile_layers
            .entry(layer_id.to_owned())
            .or_default()
            .requests_in_flight = count;
    }

    /// Report current tile cache utilization for a layer.
    #[instrument(skip(self))]
    pub fn set_tile_cache_utilization(&self, scene_id: &str, layer_id: &str, count: u32) {
        let mut scenes = self.scenes.lock().unwrap_or_else(|p| {
            tracing::error!("GIS render context mutex poisoned");
            p.into_inner()
        });
        let scene = scenes.entry(scene_id.to_owned()).or_default();
        scene
            .tile_layers
            .entry(layer_id.to_owned())
            .or_default()
            .cache_utilization = count;
    }

    pub(crate) fn tile_state(&self, scene_id: &str, layer_id: &str) -> BevyGisTileLayerState {
        let scenes = self.scenes.lock().unwrap_or_else(|p| {
            tracing::error!("GIS render context mutex poisoned");
            p.into_inner()
        });
        scenes
            .get(scene_id)
            .and_then(|s| s.tile_layers.get(layer_id))
            .cloned()
            .unwrap_or_default()
    }
}

// ── BevyGisPlugin ─────────────────────────────────────────────────────────────

/// Bevy plugin that registers the GIS render backend as an ECS resource.
///
/// Add to your app with `app.add_plugins(BevyGisPlugin::new(backend))`.
/// Bevy systems can then access the backend via `Res<BevyGisBackend>`.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_bevy::{App, BevyGisBackend, BevyGisPlugin, BevyGisRenderCtx, DefaultPlugins};
/// use std::sync::Arc;
///
/// fn main() {
///     let ctx = Arc::new(BevyGisRenderCtx::new());
///     let backend = BevyGisBackend::new(ctx);
///
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(BevyGisPlugin::new(backend))
///         .run();
/// }
/// ```
pub struct BevyGisPlugin {
    backend: BevyGisBackend,
}

impl BevyGisPlugin {
    /// Create a plugin wrapping the given GIS render backend.
    pub fn new(backend: BevyGisBackend) -> Self {
        Self { backend }
    }

    /// Create a plugin with a fresh [`BevyGisRenderCtx`] backed by an
    /// [`Arc`] so callers can retain a handle to the same
    /// context that is registered in the ECS world.
    pub fn with_shared_ctx(ctx: Arc<BevyGisRenderCtx>) -> (Self, BevyGisBackend) {
        let backend = BevyGisBackend::new(ctx);
        (Self::new(backend.clone()), backend)
    }
}

impl crate::Plugin for BevyGisPlugin {
    #[tracing::instrument(skip(self, app))]
    fn build(&self, app: &mut crate::App) {
        app.0.insert_resource(self.backend.clone());
    }
}
