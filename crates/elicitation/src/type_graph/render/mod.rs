//! Graph rendering backends.
//!
//! [`GraphRenderer`] is the common trait. Each renderer takes a [`TypeGraph`]
//! reference and returns a `String` in its target format.
//!
//! Available renderers:
//! - [`MermaidRenderer`] — Mermaid flowchart syntax (renders in GitHub, docs)
//! - [`DotRenderer`] — Graphviz DOT syntax (pipe through `dot -Tsvg`)

mod dot;
mod mermaid;

pub use dot::DotRenderer;
pub use mermaid::{MermaidDirection, MermaidRenderer};

use crate::type_graph::builder::TypeGraph;

/// Common interface for all graph renderers.
///
/// Each renderer owns its configuration (direction, include_primitives, etc.)
/// and produces a `String` from a [`TypeGraph`] reference.
pub trait GraphRenderer {
    /// Render `graph` to a string in this renderer's output format.
    fn render(&self, graph: &TypeGraph) -> String;
}
