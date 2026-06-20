//! Bevy GIS render backend — full [`elicit_gis::RenderBackend`] implementation.

mod factories_layer;
mod factories_leaf;
mod meta_runtime;
mod meta_structural;
mod proof_credentials;

use std::sync::Arc;

use bevy::ecs::resource::Resource;

use crate::BevyGisRenderCtx;

/// Bevy implementation of the renderer-agnostic GIS render backend.
///
/// Wraps a shared [`BevyGisRenderCtx`] so that the stateful runtime reporters
/// (tile streaming, picking, projection) can observe and update scene state.
///
/// Can be inserted into a Bevy [`World`](bevy::ecs::world::World) as a resource
/// via [`BevyGisPlugin`](crate::BevyGisPlugin) and then accessed in systems with
/// `Res<BevyGisBackend>`.
#[derive(Clone, Resource)]
pub struct BevyGisBackend {
    pub(crate) ctx: Arc<BevyGisRenderCtx>,
}

impl BevyGisBackend {
    /// Create a new backend backed by the given runtime context.
    pub fn new(ctx: Arc<BevyGisRenderCtx>) -> Self {
        Self { ctx }
    }
}
