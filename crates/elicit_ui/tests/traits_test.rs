//! Object-safety compilation tests for all UI traits.

use elicit_ui::{
    UiBackend, UiEventDispatcher, UiInspector, UiLayoutManager, UiNavigationManager,
    WcagContrastFactory, WcagElementMeta, WcagErrorFactory, WcagFocusFactory, WcagKeyboardFactory,
    WcagLabelFactory, WcagLanguageFactory, WcagMediaFactory, WcagOperableFactory, WcagPageMeta,
    WcagPerceivedFactory, WcagRobustFactory, WcagStructureFactory, WcagTargetFactory,
    WcagTimingFactory, WcagUnderstandableFactory,
};

// Object safety: if these compile, all traits are dyn-safe.
fn _dyn_layout_manager(_: &dyn UiLayoutManager) {}
fn _dyn_navigation(_: &dyn UiNavigationManager) {}
fn _dyn_events(_: &dyn UiEventDispatcher) {}
fn _dyn_inspector(_: &dyn UiInspector) {}
fn _dyn_contrast(_: &dyn WcagContrastFactory) {}
fn _dyn_label(_: &dyn WcagLabelFactory) {}
fn _dyn_focus(_: &dyn WcagFocusFactory) {}
fn _dyn_keyboard(_: &dyn WcagKeyboardFactory) {}
fn _dyn_timing(_: &dyn WcagTimingFactory) {}
fn _dyn_target(_: &dyn WcagTargetFactory) {}
fn _dyn_structure(_: &dyn WcagStructureFactory) {}
fn _dyn_media(_: &dyn WcagMediaFactory) {}
fn _dyn_language(_: &dyn WcagLanguageFactory) {}
fn _dyn_error(_: &dyn WcagErrorFactory) {}
fn _dyn_perceived(_: &dyn WcagPerceivedFactory) {}
fn _dyn_operable(_: &dyn WcagOperableFactory) {}
fn _dyn_understandable(_: &dyn WcagUnderstandableFactory) {}
fn _dyn_robust(_: &dyn WcagRobustFactory) {}
fn _dyn_element_meta(_: &dyn WcagElementMeta) {}
fn _dyn_page_meta(_: &dyn WcagPageMeta) {}
fn _dyn_backend(_: &dyn UiBackend) {}

#[test]
fn object_safety_compiles() {}
