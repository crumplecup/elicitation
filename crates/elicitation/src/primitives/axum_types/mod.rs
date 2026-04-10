//! Axum descriptor primitives.
//!
//! Serializable descriptor types for axum router, handler, response, and
//! serve configurations. Available with the `axum-types` feature.

mod descriptors;
mod enums;

pub use descriptors::{
    AxumExtractorEntry, AxumHandlerDescriptor, AxumResponseDescriptor, AxumRouteEntry,
    AxumRouterDescriptor, AxumServeDescriptor,
};
pub use enums::{AxumExtractorKind, AxumHttpMethod, AxumResponseKind};
