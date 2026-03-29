//! Core domain types for UI verification.

use derive_more::{AsRef, Deref, Display};
use derive_new::new;
use serde::{Deserialize, Serialize};

/// Accessible label for UI elements.
///
/// Guaranteed non-empty at construction time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, AsRef, Display, Serialize, Deserialize)]
#[display("{}", _0)]
pub struct Label(String);

impl Label {
    /// Create a new label from a non-empty string.
    ///
    /// Returns `None` if the input is empty.
    pub fn new(s: impl Into<String>) -> Option<Self> {
        let s = s.into();
        if s.is_empty() {
            None
        } else {
            Some(Self(s))
        }
    }

    /// Get the label text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Element size in pixels (width, height).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, Serialize, Deserialize)]
pub struct Size {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

impl Size {
    /// Check if size meets minimum touch target requirements (44x44).
    ///
    /// WCAG 2.5.5 Level AAA: Target Size (Enhanced)
    pub fn meets_min_target_size(&self) -> bool {
        self.width >= 44 && self.height >= 44
    }
}

/// Viewport dimensions in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, Serialize, Deserialize)]
pub struct Viewport {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

/// Unique identifier for UI elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, Serialize, Deserialize)]
pub struct ElementId(accesskit::NodeId);

impl ElementId {
    /// Create a new element ID.
    pub fn new(id: u64) -> Self {
        Self(accesskit::NodeId(id))
    }

    /// Get the underlying NodeId.
    pub fn node_id(&self) -> accesskit::NodeId {
        self.0
    }
}

impl From<ElementId> for accesskit::NodeId {
    fn from(id: ElementId) -> Self {
        id.0
    }
}

impl From<accesskit::NodeId> for ElementId {
    fn from(node_id: accesskit::NodeId) -> Self {
        Self(node_id)
    }
}

impl std::fmt::Display for ElementId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ElementId({})", self.0 .0)
    }
}

/// Statistics collected during a render pass.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RenderStats {
    /// Total nodes visited.
    pub nodes_visited: usize,
    /// Number of interactive widgets rendered.
    pub widgets_rendered: usize,
    /// Number of container nodes rendered.
    pub containers_rendered: usize,
    /// Number of nodes skipped (hidden or unsupported role).
    pub nodes_skipped: usize,
}
