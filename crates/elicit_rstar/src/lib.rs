//! `elicit_rstar` — factory-driven dynamic tool support for `rstar::RTree<T>`.
//!
//! This crate keeps the runtime API generic over the stored item type `T`, while
//! using elicitable trenchcoat wrappers from `elicitation` for the built-in
//! 2D `rstar` primitives.
//!
//! # Dynamic factories
//!
//! Two capability-based factories are provided:
//!
//! | Factory | Item bound | Tools |
//! |---|---|---|
//! | [`RTreeObjectFactory`] | `T: ElicitComplete + RTreeObject<Envelope = AABB<[f64; 2]>>` | `new`, `bulk_load`, `size`, `items`, `insert`, `locate_in_envelope`, `locate_in_envelope_intersecting` |
//! | [`PointDistanceFactory`] | `T: ElicitComplete + RTreeObject<Envelope = AABB<[f64; 2]>> + PointDistance` | all `RTreeObjectFactory` tools plus `nearest_neighbor`, `nearest_neighbors`, `locate_all_at_point`, `locate_within_distance` |
//!
//! Register a tree snapshot type with the dynamic registry, prime the matching
//! factory, then instantiate the factory meta-tool:
//!
//! ```rust,no_run
//! use elicit_rstar::{RstarTree, prime_point_distance_tree, prime_rtree_object_tree};
//! use elicitation::{DynamicToolRegistry, RstarRectangle};
//!
//! prime_rtree_object_tree::<RstarRectangle>();
//! prime_point_distance_tree::<RstarRectangle>();
//!
//! let registry = DynamicToolRegistry::new()
//!     .register_type::<RstarTree<RstarRectangle>>("rect_tree");
//!
//! # let registry = registry;
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod factory;
mod tree;

pub use factory::{
    PointDistanceFactory, RTreeObjectFactory, prime_point_distance_tree, prime_rtree_object_tree,
};
pub use tree::{LineTree, RectangleTree, RstarTree};
