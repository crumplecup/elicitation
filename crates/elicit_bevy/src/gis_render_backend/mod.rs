//! Bevy GIS render backend — full [`elicit_gis::RenderBackend`] implementation.

mod factories_layer;
mod factories_leaf;
mod meta_runtime;
mod meta_structural;
mod proof_credentials;

use std::sync::Arc;

use crate::BevyGisRenderCtx;

/// Bevy implementation of the renderer-agnostic GIS render backend.
///
/// Wraps a shared [`BevyGisRenderCtx`] so that the stateful runtime reporters
/// (tile streaming, picking, projection) can observe and update scene state.
#[derive(Clone)]
pub struct BevyGisBackend {
    ctx: Arc<BevyGisRenderCtx>,
}

impl BevyGisBackend {
    /// Create a new backend backed by the given runtime context.
    pub fn new(ctx: Arc<BevyGisRenderCtx>) -> Self {
        Self { ctx }
    }
}
