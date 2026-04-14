//! Trait re-exports and the [`GisBackend`] supertrait.

mod crs;
mod geometry;
mod set_ops;
mod topology;

pub use crs::{GisCrsLookup, GisCrsTransformer, GisCrsValidator};
pub use geometry::SfsGeometry;
pub use set_ops::SfsSetOps;
pub use topology::SfsTopology;

/// Complete geospatial backend — blanket supertrait.
///
/// Any type that implements all three GIS CRS sub-traits automatically
/// implements `GisBackend`. Use `dyn GisCrsLookup`, `dyn GisCrsValidator`,
/// or `dyn GisCrsTransformer` for object-safe dynamic dispatch.
pub trait GisBackend: GisCrsLookup + GisCrsValidator + GisCrsTransformer + Send + Sync {}

impl<T> GisBackend for T where T: GisCrsLookup + GisCrsValidator + GisCrsTransformer + Send + Sync {}
