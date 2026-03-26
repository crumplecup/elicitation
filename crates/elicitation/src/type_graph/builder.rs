//! Type graph builder — walks the [`TypeGraphKey`] registry from a root type.
//!
//! Produces a [`TypeGraph`] (nodes + edges) that renderers consume. The walk
//! is depth-first with a visited set; cycle safety is guaranteed by marking
//! nodes visited **before** expanding their edges.
//!
//! # Leaf node classification
//!
//! - Not found in registry + looks like a type parameter (single uppercase
//!   letter, or no `::` and title-case single word ≤ 2 chars) →
//!   rendered as `(generic:T)`.
//! - Not found in registry, otherwise → rendered as a plain `Primitive` leaf.

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    ElicitationPattern, PatternDetails,
    type_graph::registry::{all_graphable_types, lookup_type_graph},
};

/// A node in the type graph.
#[derive(Debug, Clone)]
pub struct GraphNode {
    /// The type name (or variant qualified name like `Mode::Fast`).
    pub name: String,
    /// How this node was classified.
    pub kind: NodeKind,
}

/// Classification of a graph node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// Struct — elicited field-by-field.
    Survey,
    /// Enum — variant selected, then variant fields elicited.
    Select,
    /// Boolean yes/no.
    Affirm,
    /// Primitive value or unregistered concrete type.
    Primitive,
    /// Unregistered type that looks like a generic parameter (e.g. `T`, `K`).
    Generic,
}

/// A directed edge in the type graph.
#[derive(Debug, Clone)]
pub struct GraphEdge {
    /// Source node name.
    pub from: String,
    /// Edge label (field name or variant label).
    pub label: String,
    /// Target node name.
    pub to: String,
}

/// Error returned when graph construction fails.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
pub enum TypeGraphError {
    /// The requested root type is not in the registry.
    #[display("root type '{}' is not registered in the TypeGraph inventory", _0)]
    UnknownRoot(#[error(not(source))] String),
}

/// A fully-traversed type graph ready for rendering.
///
/// Built by [`TypeGraph::from_root`] or [`TypeGraph::from_roots`].
/// Nodes are stored in traversal order (breadth-first from root).
#[derive(Debug, Default)]
pub struct TypeGraph {
    /// All nodes encountered during traversal, keyed by node name.
    pub nodes: HashMap<String, GraphNode>,
    /// Directed edges in traversal order.
    pub edges: Vec<GraphEdge>,
    /// Root type names (in the order they were added).
    pub roots: Vec<String>,
}

impl TypeGraph {
    /// Walk from a single root type name.
    ///
    /// Returns `Err` if the root type is not registered.
    pub fn from_root(root: &str) -> Result<Self, TypeGraphError> {
        Self::from_roots(&[root])
    }

    /// Walk from multiple root type names.
    ///
    /// Returns `Err` if any root is not registered.
    pub fn from_roots(roots: &[&str]) -> Result<Self, TypeGraphError> {
        let mut graph = Self::default();
        let mut visited: HashSet<String> = HashSet::new();
        let mut queue: VecDeque<String> = VecDeque::new();

        for &root in roots {
            if lookup_type_graph(root).is_none() {
                return Err(TypeGraphError::UnknownRoot(root.to_string()));
            }
            graph.roots.push(root.to_string());
            queue.push_back(root.to_string());
        }

        while let Some(name) = queue.pop_front() {
            // Mark visited BEFORE expanding — prevents infinite loops on recursive types.
            if !visited.insert(name.clone()) {
                continue;
            }

            match lookup_type_graph(&name) {
                None => {
                    // Leaf node — classify as generic placeholder or primitive.
                    let kind = classify_leaf(&name);
                    graph.nodes.insert(name.clone(), GraphNode { name, kind });
                }
                Some(meta) => {
                    let kind = match meta.pattern() {
                        ElicitationPattern::Survey => NodeKind::Survey,
                        ElicitationPattern::Select => NodeKind::Select,
                        ElicitationPattern::Affirm => NodeKind::Affirm,
                        ElicitationPattern::Primitive => NodeKind::Primitive,
                    };
                    graph.nodes.insert(
                        name.clone(),
                        GraphNode {
                            name: name.clone(),
                            kind,
                        },
                    );

                    match meta.details {
                        PatternDetails::Survey { fields } => {
                            for field in fields {
                                graph.edges.push(GraphEdge {
                                    from: name.clone(),
                                    label: field.name.to_string(),
                                    to: field.type_name.to_string(),
                                });
                                if !visited.contains(field.type_name) {
                                    queue.push_back(field.type_name.to_string());
                                }
                            }
                        }
                        PatternDetails::Select { variants } => {
                            for variant in variants {
                                // Fully-qualified variant node id prevents collisions
                                // between enums that share a variant name.
                                let variant_node = format!("{}::{}", name, variant.label);

                                if variant.fields.is_empty() {
                                    // Unit variant — leaf node, edge directly from enum.
                                    graph.edges.push(GraphEdge {
                                        from: name.clone(),
                                        label: variant.label.clone(),
                                        to: variant_node.clone(),
                                    });
                                    if !visited.contains(&variant_node) {
                                        visited.insert(variant_node.clone());
                                        graph.nodes.insert(
                                            variant_node.clone(),
                                            GraphNode {
                                                name: variant_node,
                                                kind: NodeKind::Primitive,
                                            },
                                        );
                                    }
                                } else {
                                    // Data variant — intermediate node with its own edges.
                                    graph.edges.push(GraphEdge {
                                        from: name.clone(),
                                        label: variant.label.clone(),
                                        to: variant_node.clone(),
                                    });
                                    if !visited.contains(&variant_node) {
                                        visited.insert(variant_node.clone());
                                        graph.nodes.insert(
                                            variant_node.clone(),
                                            GraphNode {
                                                name: variant_node.clone(),
                                                kind: NodeKind::Select,
                                            },
                                        );
                                        for field in &variant.fields {
                                            graph.edges.push(GraphEdge {
                                                from: variant_node.clone(),
                                                label: field.name.to_string(),
                                                to: field.type_name.to_string(),
                                            });
                                            if !visited.contains(field.type_name) {
                                                queue.push_back(field.type_name.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        PatternDetails::Affirm | PatternDetails::Primitive => {
                            // Already added as leaf node above.
                        }
                    }
                }
            }
        }

        Ok(graph)
    }

    /// All registered graphable type names (convenience pass-through).
    pub fn registered_types() -> Vec<&'static str> {
        all_graphable_types()
    }
}

/// Classify an unregistered type name as Generic or Primitive.
///
/// Heuristic: a single uppercase ASCII letter, or a short (≤2 char)
/// all-uppercase name with no `::` is likely a type parameter.
fn classify_leaf(name: &str) -> NodeKind {
    let trimmed = name.trim();
    let is_generic = !trimmed.contains("::")
        && !trimmed.contains('<')
        && trimmed.len() <= 2
        && trimmed.chars().all(|c| c.is_ascii_uppercase());
    if is_generic {
        NodeKind::Generic
    } else {
        NodeKind::Primitive
    }
}
