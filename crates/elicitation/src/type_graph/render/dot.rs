//! Graphviz DOT renderer.
//!
//! Produces output suitable for piping through `dot -Tsvg` or `dot -Tpng`.
//! Survey nodes are yellow, Select nodes are blue, variant/leaf nodes are white.

use crate::type_graph::builder::{NodeKind, TypeGraph};
use crate::type_graph::render::GraphRenderer;

/// Renders a [`TypeGraph`] as Graphviz DOT syntax.
///
/// # Example output
///
/// ```text
/// digraph {
///     node [shape=record fontname="monospace"]
///     ApplicationConfig [label="{ApplicationConfig|survey|3 fields}" fillcolor=lightyellow style=filled]
///     ApplicationConfig -> NetworkConfig [label="network"]
/// }
/// ```
#[derive(Debug, Clone)]
pub struct DotRenderer {
    /// Include primitive and generic leaf nodes in the output.
    /// Default: `false`.
    pub include_primitives: bool,
    /// Group Survey and Select composite nodes into labelled subgraphs.
    /// Default: `true`.
    pub cluster_by_pattern: bool,
}

impl Default for DotRenderer {
    fn default() -> Self {
        Self {
            include_primitives: false,
            cluster_by_pattern: true,
        }
    }
}

impl DotRenderer {
    /// Create a renderer with default settings.
    pub fn new() -> Self {
        Self::default()
    }
}

impl GraphRenderer for DotRenderer {
    fn render(&self, graph: &TypeGraph) -> String {
        let mut out = String::from("digraph {\n");
        out.push_str("    node [shape=record fontname=\"monospace\"]\n");

        let mut node_names: Vec<&str> = graph.nodes.keys().map(String::as_str).collect();
        node_names.sort_unstable();

        // Node declarations.
        for name in &node_names {
            let node = &graph.nodes[*name];
            let skip = matches!(node.kind, NodeKind::Primitive | NodeKind::Generic)
                && !self.include_primitives;
            if skip {
                continue;
            }

            let (label, color) = node_label_color(name, node, graph);
            out.push_str(&format!(
                "    {} [label=\"{}\" fillcolor={} style=filled]\n",
                dot_id(name),
                label,
                color,
            ));
        }

        // Edges.
        for edge in &graph.edges {
            let target_node = graph.nodes.get(&edge.to);
            let target_is_leaf = target_node
                .is_none_or(|n| matches!(n.kind, NodeKind::Primitive | NodeKind::Generic));
            if target_is_leaf && !self.include_primitives {
                continue;
            }
            let edge_label = match &edge.prompt {
                Some(p) => format!("{}\\n{}", edge.label, dot_escape(p)),
                None => edge.label.clone(),
            };
            out.push_str(&format!(
                "    {} -> {} [label=\"{}\"]\n",
                dot_id(&edge.from),
                dot_id(&edge.to),
                edge_label,
            ));
        }

        out.push('}');
        out
    }
}

fn node_label_color<'a>(
    name: &str,
    node: &crate::type_graph::builder::GraphNode,
    graph: &TypeGraph,
) -> (String, &'a str) {
    match node.kind {
        NodeKind::Survey => {
            let field_count = graph.edges.iter().filter(|e| e.from == name).count();
            let prompt_row = match &node.prompt {
                Some(p) => format!("|{}", dot_escape(p)),
                None => String::new(),
            };
            (
                format!("{{{}|survey|{} fields{}}}", name, field_count, prompt_row),
                "lightyellow",
            )
        }
        NodeKind::Select => {
            let variant_count = graph.edges.iter().filter(|e| e.from == name).count();
            let prompt_row = match &node.prompt {
                Some(p) => format!("|{}", dot_escape(p)),
                None => String::new(),
            };
            (
                format!(
                    "{{{}|select|{} variants{}}}",
                    name, variant_count, prompt_row
                ),
                "lightblue",
            )
        }
        NodeKind::Affirm => {
            let prompt_row = match &node.prompt {
                Some(p) => format!("|{}", dot_escape(p)),
                None => String::new(),
            };
            (format!("{{{}|affirm{}}}", name, prompt_row), "lightgreen")
        }
        NodeKind::Primitive => (name.to_string(), "white"),
        NodeKind::Generic => (format!("(generic:{})", name), "lightyellow"),
    }
}

/// Escape characters that have special meaning inside a DOT record label.
fn dot_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('{', "\\{")
        .replace('}', "\\}")
        .replace('|', "\\|")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

/// Produce a valid DOT identifier. Quotes names containing special characters.
fn dot_id(name: &str) -> String {
    if name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
        format!("\"{}\"", name.replace('"', "\\\""))
    } else {
        name.to_string()
    }
}
