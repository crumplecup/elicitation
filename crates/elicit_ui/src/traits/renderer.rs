//! Render a verified UI tree to a specific backend.

use accesskit::Role;
use elicitation::Established;

use crate::{RenderComplete, RenderStats, UiResult, VerifiedTree, WidgetId};

/// Render a verified UI tree to a specific backend.
///
/// Replaces the previous `RenderBackend` trait with a richer signature
/// that returns proof of successful rendering.
///
/// # Implementors
///
/// - `elicit_egui::EguiBackend` — renders to egui widgets
/// - `elicit_ratatui::RatatuiBackend` — renders to TUI
/// - `elicit_leptos::LeptosRenderer` — renders to leptos components
pub trait UiRenderer: Send + Sync {
    /// Render the full verified tree.
    fn render(&self, tree: &VerifiedTree) -> UiResult<(RenderStats, Established<RenderComplete>)>;

    /// Render a single subtree rooted at `node_id`.
    fn render_partial(&self, node_id: WidgetId, tree: &VerifiedTree) -> UiResult<RenderStats>;

    /// Whether this backend can render the given AccessKit role.
    fn supports_role(&self, role: Role) -> bool;

    /// Human-readable backend name (e.g. `"egui"`, `"ratatui"`, `"leptos"`).
    fn backend_name(&self) -> &str;
}
