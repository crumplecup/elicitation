//! Trait re-exports and the [`GisBackend`] supertrait.

mod crs;

pub use crs::{GisCrsLookup, GisCrsTransformer, GisCrsValidator};

/// Complete geospatial backend — blanket supertrait.
///
/// Any type that implements all three GIS sub-traits automatically implements
/// `GisBackend`. Use `dyn GisCrsLookup`, `dyn GisCrsValidator`, or
/// `dyn GisCrsTransformer` for object-safe dynamic dispatch on individual traits.
pub trait GisBackend: GisCrsLookup + GisCrsValidator + GisCrsTransformer + Send + Sync {}

impl<T> GisBackend for T where T: GisCrsLookup + GisCrsValidator + GisCrsTransformer + Send + Sync {}
