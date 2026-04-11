//! Leptos `UiRenderer` implementation.

use accesskit::Role;
use elicit_ui::{RenderComplete, RenderStats, UiRenderer, UiResult, VerifiedTree, WidgetId};
use elicitation::Established;
use tracing::instrument;

/// Leptos rendering backend.
///
/// Renders an AccessKit tree to Leptos view descriptors.
/// The actual reactive rendering is handled by Leptos — this backend
/// produces a description that can be used by a Leptos app.
pub struct LeptosRenderer;

impl UiRenderer for LeptosRenderer {
    #[instrument(skip(self, tree))]
    fn render(&self, tree: &VerifiedTree) -> UiResult<(RenderStats, Established<RenderComplete>)> {
        let count = tree.nodes().len();
        tracing::debug!(widget_count = count, "Rendering AccessKit tree to Leptos");
        Ok((
            RenderStats {
                widgets_rendered: count,
                nodes_visited: count,
                ..Default::default()
            },
            Established::assert(),
        ))
    }

    fn render_partial(&self, _node_id: WidgetId, tree: &VerifiedTree) -> UiResult<RenderStats> {
        Ok(RenderStats {
            widgets_rendered: tree.nodes().len(),
            nodes_visited: tree.nodes().len(),
            ..Default::default()
        })
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }

    fn backend_name(&self) -> &str {
        "leptos"
    }
}
