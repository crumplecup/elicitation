//! Arrange widgets in layout containers.

use elicitation::Established;

use crate::{ContainerId, NoOverflow, UiResult, WidgetId};

/// Arrange widgets in layout containers.
pub trait UiLayoutManager: Send + Sync {
    /// Stack children along an axis (`"horizontal"` or `"vertical"`).
    fn container_stack(
        &self,
        axis: &str,
        children: Vec<WidgetId>,
    ) -> UiResult<(ContainerId, Established<NoOverflow>)>;

    /// Arrange children in a grid.
    fn container_grid(
        &self,
        columns: u32,
        children: Vec<WidgetId>,
    ) -> UiResult<(ContainerId, Established<NoOverflow>)>;

    /// Wrap a child in a scroll region.
    fn container_scroll(&self, child: WidgetId)
    -> UiResult<(ContainerId, Established<NoOverflow>)>;

    /// Create a named panel.
    fn container_panel(&self, name: &str, content: Vec<WidgetId>) -> UiResult<ContainerId>;

    /// Add a child widget to an existing container.
    fn add_child(&self, parent: ContainerId, child: WidgetId) -> UiResult<()>;

    /// Remove a widget from the tree.
    fn remove_widget(&self, id: WidgetId) -> UiResult<()>;
}
