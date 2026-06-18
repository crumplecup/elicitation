//! Runtime meta trait impls — tile streaming, picking, projection.

use elicit_gis::{
    GisRenderPickingMeta, GisRenderProjectionMeta, GisRenderTileStreamingMeta, GisResult,
    RenderPickHitDescriptor, RenderSceneDescriptor, RenderScreenPointDescriptor,
    RenderScreenRectDescriptor,
};
use tracing::instrument;

use super::BevyGisBackend;

// ── GisRenderTileStreamingMeta ────────────────────────────────────────────────

impl GisRenderTileStreamingMeta for BevyGisBackend {
    #[instrument(skip_all, fields(layer_id))]
    fn tile_requests_in_flight(
        &self,
        scene: &RenderSceneDescriptor,
        layer_id: &str,
    ) -> GisResult<u32> {
        Ok(self
            .ctx
            .tile_state(&scene.scene_id, layer_id)
            .requests_in_flight)
    }

    #[instrument(skip_all, fields(layer_id))]
    fn tile_cache_utilization(
        &self,
        scene: &RenderSceneDescriptor,
        layer_id: &str,
    ) -> GisResult<u32> {
        Ok(self
            .ctx
            .tile_state(&scene.scene_id, layer_id)
            .cache_utilization)
    }

    #[instrument(skip_all, fields(layer_id))]
    fn tile_layer_is_fully_loaded(
        &self,
        scene: &RenderSceneDescriptor,
        layer_id: &str,
    ) -> GisResult<bool> {
        Ok(self
            .ctx
            .tile_state(&scene.scene_id, layer_id)
            .requests_in_flight
            == 0)
    }

    #[instrument(skip_all, fields(layer_id))]
    fn tile_layer_load_progress(
        &self,
        scene: &RenderSceneDescriptor,
        layer_id: &str,
    ) -> GisResult<f32> {
        let state = self.ctx.tile_state(&scene.scene_id, layer_id);
        if state.requests_in_flight == 0 {
            return Ok(1.0);
        }
        let total = state.requests_in_flight + state.cache_utilization;
        if total == 0 {
            return Ok(1.0);
        }
        Ok(state.cache_utilization as f32 / total as f32)
    }
}

// ── GisRenderProjectionMeta ───────────────────────────────────────────────────

impl GisRenderProjectionMeta for BevyGisBackend {
    #[instrument(skip_all, fields(view_id, longitude, latitude))]
    fn geographic_to_screen(
        &self,
        scene: &RenderSceneDescriptor,
        view_id: &str,
        longitude: f64,
        latitude: f64,
        _altitude_meters: f64,
    ) -> GisResult<Option<RenderScreenPointDescriptor>> {
        let view = if scene.view.view_id == view_id {
            &scene.view
        } else if let Some(v) = scene.auxiliary_views.iter().find(|v| v.view_id == view_id) {
            v
        } else {
            tracing::warn!(view_id, "view not found in scene for geographic_to_screen");
            return Ok(None);
        };

        let dx = longitude - view.center_x;
        let dy = latitude - view.center_y;

        if view.resolution <= 0.0 {
            return Ok(None);
        }

        let rotation_rad = view.rotation_degrees.to_radians();
        let cos_r = rotation_rad.cos();
        let sin_r = rotation_rad.sin();

        let px_x = (dx * cos_r - dy * sin_r) / view.resolution;
        let px_y = (-dx * sin_r - dy * cos_r) / view.resolution;

        let screen_x = (view.viewport_width as f64 / 2.0 + px_x) as f32;
        let screen_y = (view.viewport_height as f64 / 2.0 + px_y) as f32;

        if screen_x < 0.0
            || screen_x > view.viewport_width as f32
            || screen_y < 0.0
            || screen_y > view.viewport_height as f32
        {
            return Ok(None);
        }

        Ok(Some(RenderScreenPointDescriptor {
            x: screen_x,
            y: screen_y,
        }))
    }

    #[instrument(skip_all, fields(view_id))]
    fn screen_to_geographic(
        &self,
        scene: &RenderSceneDescriptor,
        view_id: &str,
        point: RenderScreenPointDescriptor,
    ) -> GisResult<Option<[f64; 3]>> {
        let view = if scene.view.view_id == view_id {
            &scene.view
        } else if let Some(v) = scene.auxiliary_views.iter().find(|v| v.view_id == view_id) {
            v
        } else {
            tracing::warn!(view_id, "view not found in scene for screen_to_geographic");
            return Ok(None);
        };

        if view.resolution <= 0.0 {
            return Ok(None);
        }

        let px_x = point.x as f64 - view.viewport_width as f64 / 2.0;
        let px_y = point.y as f64 - view.viewport_height as f64 / 2.0;

        let rotation_rad = view.rotation_degrees.to_radians();
        let cos_r = rotation_rad.cos();
        let sin_r = rotation_rad.sin();

        let world_x = (px_x * cos_r + px_y * (-sin_r)) * view.resolution;
        let world_y = (px_x * sin_r + px_y * (-cos_r)) * view.resolution;

        let longitude = view.center_x + world_x;
        let latitude = view.center_y - world_y;

        Ok(Some([longitude, latitude, 0.0]))
    }
}

// ── GisRenderPickingMeta ──────────────────────────────────────────────────────

impl GisRenderPickingMeta for BevyGisBackend {
    #[instrument(skip_all)]
    fn features_at_screen_point(
        &self,
        _scene: &RenderSceneDescriptor,
        _view_id: &str,
        _point: RenderScreenPointDescriptor,
        _layer_scope: Option<&[&str]>,
    ) -> GisResult<Vec<RenderPickHitDescriptor>> {
        Ok(Vec::new())
    }

    #[instrument(skip_all)]
    fn features_in_screen_rect(
        &self,
        _scene: &RenderSceneDescriptor,
        _view_id: &str,
        _rect: RenderScreenRectDescriptor,
        _layer_scope: Option<&[&str]>,
    ) -> GisResult<Vec<RenderPickHitDescriptor>> {
        Ok(Vec::new())
    }
}
