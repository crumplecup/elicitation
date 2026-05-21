//! New types for the UI trait interface layer.

use accesskit::NodeId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Opaque identifier for a widget in the UI tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct WidgetId(pub u64);

impl WidgetId {
    /// Create a `WidgetId` from an AccessKit `NodeId`.
    pub fn from_node(id: NodeId) -> Self {
        Self(id.0)
    }

    /// Convert back to an AccessKit `NodeId`.
    pub fn to_node_id(self) -> NodeId {
        NodeId(self.0)
    }
}

/// Opaque identifier for a layout container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ContainerId(pub u64);

impl ContainerId {
    /// Create a `ContainerId` from an AccessKit `NodeId`.
    pub fn from_node(id: NodeId) -> Self {
        Self(id.0)
    }

    /// Convert back to an AccessKit `NodeId`.
    pub fn to_node_id(self) -> NodeId {
        NodeId(self.0)
    }
}

/// Summary info about a single widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WidgetInfo {
    /// Widget identifier.
    pub id: WidgetId,
    /// AccessKit role as a string.
    pub role: String,
    /// Accessible label if present.
    pub label: Option<String>,
    /// Whether this widget can receive focus.
    pub is_focusable: bool,
    /// Child widget IDs.
    pub children: Vec<WidgetId>,
}

/// Accessibility status for a widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WidgetA11y {
    /// Widget identifier.
    pub id: WidgetId,
    /// Descriptions of any accessibility violations.
    pub violations: Vec<String>,
    /// WCAG conformance level: `"A"`, `"AA"`, `"AAA"`, or `None` if violations exist.
    pub level: Option<String>,
}

/// A contrast violation for a widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ContrastViolation {
    /// Widget with insufficient contrast.
    pub widget_id: WidgetId,
    /// Measured contrast ratio.
    pub actual_ratio: f64,
    /// Required contrast ratio.
    pub required_ratio: f64,
    /// Human-readable description of the violation context.
    pub context: String,
}

/// A verified, owned snapshot of the UI tree ready for rendering.
///
/// Produced from `Layout<Verified>` via [`Layout::into_verified_tree`](crate::Layout::into_verified_tree).
/// Carries the AccessKit nodes without the typestate machinery.
#[derive(Debug, Clone)]
pub struct VerifiedTree {
    pub(crate) nodes: BTreeMap<NodeId, accesskit::Node>,
    pub(crate) root: NodeId,
    pub(crate) viewport: crate::Viewport,
}

impl VerifiedTree {
    /// Root node ID of the tree.
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Viewport dimensions the tree was verified against.
    pub fn viewport(&self) -> crate::Viewport {
        self.viewport
    }

    /// All nodes in the tree, keyed by `NodeId`.
    pub fn nodes(&self) -> &BTreeMap<NodeId, accesskit::Node> {
        &self.nodes
    }

    /// Construct a [`VerifiedTree`] directly from raw parts.
    ///
    /// This bypasses the typestate verification pipeline and is
    /// intended for use in renderer tests that need a known-good tree
    /// without going through the full WCAG verification path.
    pub fn from_parts(
        nodes: BTreeMap<NodeId, accesskit::Node>,
        root: NodeId,
        viewport: crate::Viewport,
    ) -> Self {
        Self {
            nodes,
            root,
            viewport,
        }
    }
}
