//! AccessKit display for [`ErdDiagram`], [`ErdNode`], [`ErdEdge`], and [`ErdColumn`].

use accesskit::Role as AkRole;
use elicit_accesskit::{NodeId, NodeJson, Role};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::archive::{ErdColumn, ErdDiagram, ErdEdge, ErdNode};

use super::ArchiveDisplay;

// ── ErdColumn ─────────────────────────────────────────────────────────────────

/// Display mode for an [`ErdColumn`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum ErdColumnMode {
    /// A compact row.
    #[default]
    Row,
}

impl ArchiveDisplay for ErdColumn {
    type Mode = ErdColumnMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Row)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut flags = Vec::new();
        if self.is_pk {
            flags.push("PK");
        }
        if self.is_fk {
            flags.push("FK");
        }
        let suffix = if flags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", flags.join(", "))
        };
        let label = format!("{}: {}{}", self.name, self.sql_type, suffix);
        (
            root_id,
            vec![(root_id, NodeJson::new(Role(AkRole::Row)).with_label(label))],
        )
    }
}

// ── ErdNode ───────────────────────────────────────────────────────────────────

/// Display mode for an [`ErdNode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum ErdNodeMode {
    /// A table box listing columns as children.
    #[default]
    TableBox,
}

impl ArchiveDisplay for ErdNode {
    type Mode = ErdNodeMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Group)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();

        let col_ids: Vec<NodeId> = (0..self.columns.len())
            .map(|i| NodeId::from(id_base + 1 + i as u64))
            .collect();

        for (i, col) in self.columns.iter().enumerate() {
            let (_, col_nodes) = col.to_ak_nodes(&ErdColumnMode::Row, id_base + 1 + i as u64);
            nodes.extend(col_nodes);
        }

        let root = NodeJson::new(Role(AkRole::Group))
            .with_label(format!("{}.{}", self.schema, self.table))
            .with_children(col_ids);
        nodes.insert(0, (root_id, root));

        (root_id, nodes)
    }
}

// ── ErdEdge ───────────────────────────────────────────────────────────────────

/// Display mode for an [`ErdEdge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum ErdEdgeMode {
    /// A compact row describing the FK relationship.
    #[default]
    Row,
}

impl ArchiveDisplay for ErdEdge {
    type Mode = ErdEdgeMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::Row)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let label = format!(
            "{}: {}.{}.{} → {}.{}.{}",
            self.constraint_name,
            self.from_schema,
            self.from_table,
            self.from_column,
            self.to_schema,
            self.to_table,
            self.to_column,
        );
        (
            root_id,
            vec![(root_id, NodeJson::new(Role(AkRole::Row)).with_label(label))],
        )
    }
}

// ── ErdDiagram ────────────────────────────────────────────────────────────────

/// Display strategies for an [`ErdDiagram`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize, JsonSchema)]
pub enum ErdDiagramMode {
    /// List of table nodes.
    #[default]
    NodeList,
    /// List of FK edges.
    EdgeList,
    /// Visual layout placeholder (Phase 10 will replace with coordinate data).
    Visual,
}

impl ArchiveDisplay for ErdDiagram {
    type Mode = ErdDiagramMode;

    fn root_role(mode: &Self::Mode) -> Role {
        match mode {
            ErdDiagramMode::NodeList | ErdDiagramMode::EdgeList => Role(AkRole::List),
            ErdDiagramMode::Visual => Role(AkRole::Figure),
        }
    }

    fn to_ak_nodes(&self, mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId::from(id_base);
        let mut nodes = Vec::new();
        let mut next = id_base + 1;

        match mode {
            ErdDiagramMode::NodeList => {
                let mut child_roots = Vec::new();
                for node in &self.nodes {
                    let (child_root, child_nodes) = node.to_ak_nodes(&ErdNodeMode::TableBox, next);
                    let used = child_nodes.len() as u64;
                    child_roots.push(child_root);
                    nodes.extend(child_nodes);
                    next += used;
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!(
                        "schema {} — {} tables",
                        self.schema,
                        self.nodes.len()
                    ))
                    .with_children(child_roots);
                nodes.insert(0, (root_id, root));
            }
            ErdDiagramMode::EdgeList => {
                let mut child_roots = Vec::new();
                for edge in &self.edges {
                    let (child_root, child_nodes) = edge.to_ak_nodes(&ErdEdgeMode::Row, next);
                    next += child_nodes.len() as u64;
                    child_roots.push(child_root);
                    nodes.extend(child_nodes);
                }
                let root = NodeJson::new(Role(AkRole::List))
                    .with_label(format!("{} FK relationships", self.edges.len()))
                    .with_children(child_roots);
                nodes.insert(0, (root_id, root));
            }
            ErdDiagramMode::Visual => {
                // Phase 10 placeholder — emit a text summary in a Figure container.
                let summary = format!(
                    "ERD: {} — {} tables, {} FK edges (visual layout pending Phase 10)",
                    self.schema,
                    self.nodes.len(),
                    self.edges.len(),
                );
                let root = NodeJson::new(Role(AkRole::Figure)).with_label(summary);
                nodes.push((root_id, root));
            }
        }

        (root_id, nodes)
    }
}
