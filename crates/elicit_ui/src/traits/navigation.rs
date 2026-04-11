//! Manage focus order and keyboard navigation.

use elicitation::Established;

use crate::{FocusVisible, KeyboardAccessible, UiResult, WidgetId};

/// Manage focus order and keyboard navigation.
pub trait UiNavigationManager: Send + Sync {
    /// Set explicit tab/focus order for widgets.
    fn set_focus_order(&self, ids: Vec<WidgetId>) -> UiResult<Established<KeyboardAccessible>>;

    /// Move focus to a specific widget.
    fn set_focus(&self, id: WidgetId) -> UiResult<Established<FocusVisible>>;

    /// Register a keyboard shortcut with accessible label.
    fn register_shortcut(
        &self,
        key: &str,
        action_id: &str,
        label: &str,
    ) -> UiResult<Established<KeyboardAccessible>>;

    /// Add a skip navigation link.
    fn skip_link(&self, target_id: WidgetId) -> UiResult<Established<KeyboardAccessible>>;

    /// Get the current focus order.
    fn focus_order(&self) -> UiResult<Vec<WidgetId>>;
}
