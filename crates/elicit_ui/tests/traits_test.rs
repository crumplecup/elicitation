//! Object-safety compilation tests for all UI traits.

use elicit_ui::{
    UiAccessibilityAuditor, UiBackend, UiEventDispatcher, UiInspector, UiLayoutManager,
    UiNavigationManager, UiRenderer, UiStyleManager, UiWidgetFactory,
};

// Object safety: if these compile, all traits are dyn-safe.
fn _dyn_widget_factory(_: &dyn UiWidgetFactory) {}
fn _dyn_layout_manager(_: &dyn UiLayoutManager) {}
fn _dyn_style_manager(_: &dyn UiStyleManager) {}
fn _dyn_navigation(_: &dyn UiNavigationManager) {}
fn _dyn_auditor(_: &dyn UiAccessibilityAuditor) {}
fn _dyn_events(_: &dyn UiEventDispatcher) {}
fn _dyn_renderer(_: &dyn UiRenderer) {}
fn _dyn_inspector(_: &dyn UiInspector) {}

#[test]
fn object_safety_compiles() {}
