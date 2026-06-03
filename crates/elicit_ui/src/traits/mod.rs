//! Trait re-exports and the [`UiBackend`] supertrait.

mod events;
mod inspector;
mod layout_manager;
mod navigation;
mod render_verify;
mod renderer;
mod wcag;

pub use events::UiEventDispatcher;
pub use inspector::UiInspector;
pub use layout_manager::UiLayoutManager;
pub use navigation::UiNavigationManager;
pub use render_verify::{RenderColors, RenderContext, RenderVerifiable, verify_in_debug};
pub use renderer::{UiEventBridge, UiNodeBridge, UiRenderBackend, UiRenderer, UiTreeRenderer};
pub use wcag::{
    WcagBackend, WcagContrastFactory, WcagElementMeta, WcagErrorFactory, WcagFocusFactory,
    WcagKeyboardFactory, WcagLabelFactory, WcagLanguageFactory, WcagMediaFactory,
    WcagOperableFactory, WcagPageMeta, WcagPerceivedFactory, WcagRobustFactory,
    WcagStructureFactory, WcagTargetFactory, WcagTimingFactory, WcagUnderstandableFactory,
};

/// Complete UI backend — blanket impl for anything implementing all required traits.
///
/// Use `dyn UiBackend` to accept any fully-capable implementation, or constrain
/// generics with `T: UiBackend`.
///
/// # Object safety
///
/// `UiBackend` is not itself object-safe (supertrait of many traits), but each
/// individual sub-trait is object-safe and accepts `&dyn SubTrait` directly.
pub trait UiBackend:
    UiLayoutManager + UiNavigationManager + UiEventDispatcher + UiInspector + WcagBackend + Send + Sync
{
}

impl<T> UiBackend for T where
    T: UiLayoutManager
        + UiNavigationManager
        + UiEventDispatcher
        + UiInspector
        + WcagBackend
        + Send
        + Sync
{
}
