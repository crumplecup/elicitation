//! Render a verified UI tree to a specific backend.

use accesskit::Role;
use elicitation::Established;

use crate::{IrSourced, RenderComplete, RenderStats, UiResult, VerifiedTree, WidgetId};

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

    /// Render a tree that was produced by a canonical model's
    /// `to_verified_tree()` call.
    ///
    /// The `_proof` parameter ensures — at the type level — that the tree
    /// passed here was derived from model state via the IR pipeline, not
    /// constructed ad-hoc.  This gives all multi-frontend applications
    /// contractually enforced equivalency: every frontend that calls this
    /// method is guaranteed to have gone through the same IR source.
    ///
    /// The default implementation delegates to [`Self::render`]; backends may
    /// override to take advantage of the stronger guarantee.
    fn render_from_ir(
        &self,
        tree: &VerifiedTree,
        _proof: Established<IrSourced>,
    ) -> UiResult<(RenderStats, Established<RenderComplete>)> {
        self.render(tree)
    }

    /// Render a single subtree rooted at `node_id`.
    fn render_partial(&self, node_id: WidgetId, tree: &VerifiedTree) -> UiResult<RenderStats>;

    /// Whether this backend can render the given AccessKit role.
    fn supports_role(&self, role: Role) -> bool;

    /// Human-readable backend name (e.g. `"egui"`, `"ratatui"`, `"leptos"`).
    fn backend_name(&self) -> &str;
}
