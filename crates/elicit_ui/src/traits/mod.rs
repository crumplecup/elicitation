//! Trait re-exports and the [`UiBackend`] supertrait.

mod auditor;
mod events;
mod inspector;
mod layout_manager;
mod navigation;
mod renderer;
mod style_manager;
mod widget_factory;

pub use auditor::UiAccessibilityAuditor;
pub use events::UiEventDispatcher;
pub use inspector::UiInspector;
pub use layout_manager::UiLayoutManager;
pub use navigation::UiNavigationManager;
pub use renderer::UiRenderer;
pub use style_manager::UiStyleManager;
pub use widget_factory::UiWidgetFactory;

/// Complete UI backend — blanket impl for anything implementing all 8 traits.
///
/// Use `dyn UiBackend` to accept any fully-capable implementation, or constrain
/// generics with `T: UiBackend`.
///
/// # Object safety
///
/// `UiBackend` is not itself object-safe (supertrait of 8 traits), but each
/// individual sub-trait is object-safe and accepts `&dyn SubTrait` directly.
pub trait UiBackend:
    UiWidgetFactory
    + UiLayoutManager
    + UiStyleManager
    + UiNavigationManager
    + UiAccessibilityAuditor
    + UiEventDispatcher
    + UiRenderer
    + UiInspector
    + Send
    + Sync
{
}

impl<T> UiBackend for T where
    T: UiWidgetFactory
        + UiLayoutManager
        + UiStyleManager
        + UiNavigationManager
        + UiAccessibilityAuditor
        + UiEventDispatcher
        + UiRenderer
        + UiInspector
        + Send
        + Sync
{
}
