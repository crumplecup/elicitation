//! Layout engine bridge between AccessKit trees and Taffy.
//!
//! Converts AccessKit node trees into Taffy layout trees, computes
//! CSS-based layout (Block/Flexbox/Grid), and extracts computed
//! bounding boxes for constraint verification.

#[cfg(feature = "layout-engine")]
use std::collections::BTreeMap;

#[cfg(feature = "layout-engine")]
use accesskit::NodeId as AkNodeId;

#[cfg(feature = "layout-engine")]
use taffy::prelude::*;

#[cfg(feature = "layout-engine")]
use crate::{BoundingBox, LayoutContext, Viewport};

/// Error from the layout engine.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("Layout engine error: {} at {}:{}", message, file, line)]
pub struct LayoutEngineError {
    /// Error description.
    pub message: String,
    /// Source file.
    pub file: &'static str,
    /// Source line.
    pub line: u32,
}

impl LayoutEngineError {
    /// Create a new layout engine error with caller location.
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: message.into(),
            file: loc.file(),
            line: loc.line(),
        }
    }
}

/// Layout mode for a container node.
///
/// Maps to Taffy's display modes.
#[cfg(feature = "layout-engine")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LayoutMode {
    /// CSS Block layout (default).
    #[default]
    Block,
    /// CSS Flexbox layout.
    Flex,
    /// CSS Grid layout.
    Grid,
}

/// Ordered wrapper for `taffy::NodeId` enabling `BTreeMap` use.
///
/// `taffy::NodeId` is internally a `u64` and converts via `From` — this
/// wrapper adds the `Ord` impl so it can be used as a `BTreeMap` key.
#[cfg(feature = "layout-engine")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrdTaffyId(pub taffy::NodeId);

#[cfg(feature = "layout-engine")]
impl PartialOrd for OrdTaffyId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "layout-engine")]
impl Ord for OrdTaffyId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        u64::from(self.0).cmp(&u64::from(other.0))
    }
}

/// Bridge between AccessKit tree and Taffy layout engine.
///
/// Walks an AccessKit tree, constructs a parallel Taffy tree,
/// computes layout, and extracts bounding boxes.
#[cfg(feature = "layout-engine")]
pub struct TaffyBridge {
    tree: TaffyTree,
    /// Maps AccessKit NodeId → Taffy NodeId.
    ak_to_taffy: BTreeMap<AkNodeId, taffy::NodeId>,
    /// Maps Taffy NodeId → AccessKit NodeId.
    taffy_to_ak: BTreeMap<OrdTaffyId, AkNodeId>,
}

#[cfg(feature = "layout-engine")]
impl TaffyBridge {
    /// Create a new empty bridge.
    #[tracing::instrument(level = "debug")]
    pub fn new() -> Self {
        Self {
            tree: TaffyTree::new(),
            ak_to_taffy: BTreeMap::new(),
            taffy_to_ak: BTreeMap::new(),
        }
    }

    /// Build the Taffy tree from AccessKit nodes.
    ///
    /// Walks the tree starting from `root_id`, creating Taffy nodes
    /// for each AccessKit node with appropriate styles.
    #[tracing::instrument(level = "debug", skip(self, nodes))]
    pub fn build_from_accesskit(
        &mut self,
        root_id: AkNodeId,
        nodes: &BTreeMap<AkNodeId, accesskit::Node>,
    ) -> Result<taffy::NodeId, LayoutEngineError> {
        self.build_node_recursive(root_id, nodes)
    }

    fn build_node_recursive(
        &mut self,
        ak_id: AkNodeId,
        nodes: &BTreeMap<AkNodeId, accesskit::Node>,
    ) -> Result<taffy::NodeId, LayoutEngineError> {
        let node = nodes
            .get(&ak_id)
            .ok_or_else(|| LayoutEngineError::new(format!("Node not found: {ak_id:?}")))?;

        let style = self.accesskit_to_taffy_style(node);

        // Build children first
        let child_ids: Vec<taffy::NodeId> = node
            .children()
            .iter()
            .filter_map(|child_ak_id| self.build_node_recursive(*child_ak_id, nodes).ok())
            .collect();

        let taffy_id = if child_ids.is_empty() {
            self.tree
                .new_leaf(style)
                .map_err(|e| LayoutEngineError::new(format!("Failed to create leaf: {e}")))?
        } else {
            self.tree
                .new_with_children(style, &child_ids)
                .map_err(|e| LayoutEngineError::new(format!("Failed to create node: {e}")))?
        };

        self.ak_to_taffy.insert(ak_id, taffy_id);
        self.taffy_to_ak.insert(OrdTaffyId(taffy_id), ak_id);

        Ok(taffy_id)
    }

    /// Convert AccessKit node properties to a Taffy style.
    fn accesskit_to_taffy_style(&self, node: &accesskit::Node) -> Style {
        let mut style = Style::default();

        // Extract size from AccessKit bounds if available
        if let Some(bounds) = node.bounds() {
            let width = bounds.x1 - bounds.x0;
            let height = bounds.y1 - bounds.y0;

            if width > 0.0 {
                style.size.width = Dimension::length(width as f32);
            }
            if height > 0.0 {
                style.size.height = Dimension::length(height as f32);
            }
        }

        style
    }

    /// Compute layout for the tree at a given viewport size.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn compute_layout(
        &mut self,
        root: taffy::NodeId,
        viewport: &Viewport,
    ) -> Result<(), LayoutEngineError> {
        let available = Size {
            width: AvailableSpace::Definite(viewport.width as f32),
            height: AvailableSpace::Definite(viewport.height as f32),
        };

        self.tree
            .compute_layout(root, available)
            .map_err(|e| LayoutEngineError::new(format!("Layout computation failed: {e}")))
    }

    /// Extract computed bounding boxes for all nodes.
    ///
    /// Returns a `LayoutContext` with bounding boxes populated from
    /// Taffy's computed layout.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn extract_layout_context(
        &self,
        viewport: Viewport,
    ) -> Result<LayoutContext, LayoutEngineError> {
        let mut bounds = BTreeMap::new();

        for (taffy_id, ak_id) in &self.taffy_to_ak {
            let layout = self
                .tree
                .layout(taffy_id.0)
                .map_err(|e| LayoutEngineError::new(format!("Failed to get layout: {e}")))?;

            let bb = BoundingBox::new(
                f64::from(layout.location.x),
                f64::from(layout.location.y),
                f64::from(layout.size.width),
                f64::from(layout.size.height),
            );

            bounds.insert(*ak_id, bb);
        }

        Ok(LayoutContext::new(viewport, bounds))
    }

    /// Compute layout at a specific width for reflow testing.
    ///
    /// WCAG 1.4.10 requires content to reflow at 320px width.
    #[tracing::instrument(level = "debug", skip(self))]
    pub fn compute_at_width(
        &mut self,
        root: taffy::NodeId,
        width: u32,
        height: u32,
    ) -> Result<LayoutContext, LayoutEngineError> {
        let viewport = Viewport::new(width, height);
        self.compute_layout(root, &viewport)?;
        self.extract_layout_context(viewport)
    }
}

#[cfg(feature = "layout-engine")]
impl Default for TaffyBridge {
    fn default() -> Self {
        Self::new()
    }
}
