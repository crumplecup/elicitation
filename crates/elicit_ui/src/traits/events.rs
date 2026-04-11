//! Register UI event handlers by ID.

use crate::{UiResult, WidgetId};

/// Register UI event handlers by ID.
///
/// Handler IDs are opaque strings — the caller maps them to closures
/// in their own dispatcher. This keeps the trait object-safe (no generics,
/// no function pointers in the trait).
pub trait UiEventDispatcher: Send + Sync {
    /// Register a click handler.
    fn on_click(&self, widget: WidgetId, handler_id: &str) -> UiResult<()>;

    /// Register a focus handler.
    fn on_focus(&self, widget: WidgetId, handler_id: &str) -> UiResult<()>;

    /// Register a blur (focus-lost) handler.
    fn on_blur(&self, widget: WidgetId, handler_id: &str) -> UiResult<()>;

    /// Register a key handler.
    fn on_key(&self, widget: WidgetId, key: &str, handler_id: &str) -> UiResult<()>;
}
