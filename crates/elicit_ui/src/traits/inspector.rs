//! Non-mutating queries over the UI widget tree.

use accesskit::Role;

use crate::{UiResult, WidgetId, WidgetInfo};

/// Non-mutating queries over the UI widget tree.
pub trait UiInspector: Send + Sync {
    /// Get info about a specific widget.
    fn widget_info(&self, id: WidgetId) -> UiResult<WidgetInfo>;

    /// Get the children of a widget.
    fn children(&self, id: WidgetId) -> UiResult<Vec<WidgetId>>;

    /// Get the parent of a widget.
    fn parent(&self, id: WidgetId) -> UiResult<Option<WidgetId>>;

    /// Find all widgets with a given AccessKit role.
    fn find_by_role(&self, role: Role) -> UiResult<Vec<WidgetId>>;

    /// Find widgets whose label contains the given text.
    fn find_by_label(&self, text: &str) -> UiResult<Vec<WidgetId>>;

    /// Total number of widgets in the tree.
    fn widget_count(&self) -> usize;
}
