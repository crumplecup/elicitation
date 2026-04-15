//! Trait re-exports and the [`GisBackend`] supertrait.

mod crs;
mod geometry;
mod iso_19111;
mod set_ops;
mod topology;

pub use crs::{GisCrsBuilder, GisCrsLookup, GisCrsTransformer};
pub use geometry::SfsGeometry;
pub use iso_19111::{Iso19111Identified, Iso19111Scoped};
pub use set_ops::SfsSetOps;
pub use topology::SfsTopology;

/// Complete geospatial backend — blanket supertrait.
///
/// Any type that implements all GIS sub-traits automatically implements
/// `GisBackend`.  Use the individual object-safe sub-traits
/// (`dyn GisCrsLookup`, `dyn GisCrsBuilder`, etc.) for dynamic dispatch
/// at architectural boundaries.
pub trait GisBackend:
    GisCrsLookup + GisCrsBuilder + GisCrsTransformer + Iso19111Identified + Iso19111Scoped + Send + Sync
{
}

impl<T> GisBackend for T where
    T: GisCrsLookup
        + GisCrsBuilder
        + GisCrsTransformer
        + Iso19111Identified
        + Iso19111Scoped
        + Send
        + Sync
{
}
