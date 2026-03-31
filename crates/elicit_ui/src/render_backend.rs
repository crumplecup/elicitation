//! Render backend trait for frontend-agnostic rendering.
//!
//! The `RenderBackend` trait allows `Layout<Verified>` to render through
//! any frontend (egui, ratatui, leptos, etc.) without coupling `elicit_ui`
//! to a specific rendering library.
//!
//! Frontend crates implement this trait and pass their backend to
//! `Layout<Verified>::render()`.

use crate::RenderStats;
use accesskit::{Node, NodeId};
use std::collections::HashMap;

/// A render backend that can render a verified AccessKit tree.
///
/// Implementations walk the AccessKit tree from root to leaves,
/// mapping each `accesskit::Role` to the appropriate frontend widget.
///
/// # Implementors
///
/// - `elicit_egui::EguiBackend` — renders to egui widgets
/// - `elicit_ratatui::RatatuiBackend` — renders to TuiNode trees
pub trait RenderBackend {
    /// Render the AccessKit tree starting from `root`.
    ///
    /// Returns [`RenderStats`] summarizing what was rendered.
    fn render_tree(
        &self,
        nodes: &HashMap<NodeId, Node>,
        root: NodeId,
    ) -> RenderStats;
}
