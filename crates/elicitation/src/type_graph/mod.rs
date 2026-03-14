//! Type graph visualization for elicitation workflows.
//!
//! Provides a runtime-queryable structural registry ([`TypeGraphKey`]),
//! a graph builder ([`TypeGraph`]), and renderers ([`MermaidRenderer`],
//! [`DotRenderer`]) that together enable visualization of any
//! `#[derive(Elicit)]` type hierarchy.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use elicitation::type_graph::{TypeGraph, MermaidRenderer, GraphRenderer};
//!
//! let graph = TypeGraph::from_root("ApplicationConfig")?;
//! let mermaid = MermaidRenderer::new().render(&graph);
//! println!("{mermaid}");
//! ```
//!
//! # Registry
//!
//! Types register themselves automatically via `#[derive(Elicit)]`. To query
//! the registry directly:
//!
//! ```rust,ignore
//! use elicitation::type_graph::{lookup_type_graph, all_graphable_types};
//!
//! for name in all_graphable_types() {
//!     println!("{name}");
//! }
//! ```

pub mod registry;

#[cfg(feature = "graph")]
pub mod builder;
#[cfg(feature = "graph")]
pub mod plugin;
#[cfg(feature = "graph")]
pub mod render;

pub use registry::{TypeGraphKey, all_graphable_types, lookup_type_graph};

#[cfg(feature = "graph")]
pub use builder::{GraphEdge, GraphNode, NodeKind, TypeGraph, TypeGraphError};
#[cfg(feature = "graph")]
pub use plugin::TypeGraphPlugin;
#[cfg(feature = "graph")]
pub use render::{DotRenderer, GraphRenderer, MermaidDirection, MermaidRenderer};
