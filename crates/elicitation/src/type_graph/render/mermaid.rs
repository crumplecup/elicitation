//! Mermaid flowchart renderer.
//!
//! Produces output suitable for embedding in Markdown, GitHub READMEs,
//! and agent responses. Renders as a `graph TD` (top-down) or `graph LR`
//! (left-right) flowchart.

use crate::type_graph::builder::{NodeKind, TypeGraph};
use crate::type_graph::render::GraphRenderer;

/// Direction of the Mermaid flowchart layout.
#[derive(Debug, Clone, Copy, Default)]
pub enum MermaidDirection {
    /// Top-down (default). Best for deep hierarchies.
    #[default]
    TopDown,
    /// Left-right. Best for wide/flat structures.
    LeftRight,
}

impl MermaidDirection {
    fn as_str(self) -> &'static str {
        match self {
            Self::TopDown => "TD",
            Self::LeftRight => "LR",
        }
    }
}

/// Renders a [`TypeGraph`] as Mermaid flowchart syntax.
///
/// # Example output
///
/// ````text
/// graph TD
///     ApplicationConfig["ApplicationConfig (survey)"]
///     NetworkConfig["NetworkConfig (survey)"]
///     ApplicationConfig -->|network| NetworkConfig
/// ````
#[derive(Debug, Clone)]
pub struct MermaidRenderer {
    /// Graph layout direction.
    pub direction: MermaidDirection,
    /// Include primitive and generic leaf nodes in the output.
    /// Default: `false` — cleaner graphs for complex workflows.
    pub include_primitives: bool,
}

impl Default for MermaidRenderer {
    fn default() -> Self {
        Self {
            direction: MermaidDirection::TopDown,
            include_primitives: false,
        }
    }
}

impl MermaidRenderer {
    /// Create a renderer with default settings (top-down, no primitives).
    pub fn new() -> Self {
        Self::default()
    }
}

impl GraphRenderer for MermaidRenderer {
    fn render(&self, graph: &TypeGraph) -> String {
        let mut out = format!("graph {}\n", self.direction.as_str());

        // Declare all composite nodes with labels showing pattern.
        let mut node_names: Vec<&str> = graph.nodes.keys().map(String::as_str).collect();
        node_names.sort_unstable();

        for name in &node_names {
            let node = &graph.nodes[*name];
            let skip = matches!(node.kind, NodeKind::Primitive | NodeKind::Generic)
                && !self.include_primitives;
            if skip {
                continue;
            }
            let label = mermaid_node_label(name, node);
            // Mermaid node ids can't contain `::` — sanitise to `__`.
            let id = sanitize_id(name);
            out.push_str(&format!("    {}{}\n", id, label));
        }

        // Edges.
        for edge in &graph.edges {
            let target_node = graph.nodes.get(&edge.to);
            let target_is_leaf = target_node
                .is_none_or(|n| matches!(n.kind, NodeKind::Primitive | NodeKind::Generic));
            if target_is_leaf && !self.include_primitives {
                continue;
            }
            let from_id = sanitize_id(&edge.from);
            let to_id = sanitize_id(&edge.to);
            let edge_label = match &edge.prompt {
                Some(p) => format!("{}: {}", edge.label, p),
                None => edge.label.clone(),
            };
            out.push_str(&format!("    {} -->|{}| {}\n", from_id, edge_label, to_id));
        }

        out
    }
}

/// Build the Mermaid bracket label for a node, including the prompt when present.
fn mermaid_node_label(name: &str, node: &crate::type_graph::builder::GraphNode) -> String {
    let kind_tag = match node.kind {
        NodeKind::Survey => "survey",
        NodeKind::Select => "select",
        NodeKind::Affirm => "affirm",
        NodeKind::Primitive => return format!("[\"{}\"]", name),
        NodeKind::Generic => return format!("(\"(generic:{})\")", name),
    };
    match &node.prompt {
        Some(p) => format!("[\"{} ({})\\n'{}'\"]", name, kind_tag, p),
        None => format!("[\"{} ({})\"]", name, kind_tag),
    }
}

/// Replace characters invalid in Mermaid node identifiers.
fn sanitize_id(name: &str) -> String {
    name.replace("::", "__")
        .replace(['<', '>', ' ', '(', ')'], "_")
}
