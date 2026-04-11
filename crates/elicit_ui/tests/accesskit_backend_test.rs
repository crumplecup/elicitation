//! Integration tests for `AccessKitUiBackend`.

use elicit_ui::{
    AccessKitUiBackend, SrgbColor, UiInspector, UiLayoutManager, UiStyleManager, UiWidgetFactory,
};

#[test]
fn create_button_produces_proof_tokens() {
    let backend = AccessKitUiBackend::new();
    let result = backend.create_button("Submit", 100, 50);
    assert!(result.is_ok(), "button creation should succeed");
    let (id, _has_label, _min_size, _keyboard) = result.unwrap();
    let _ = id;
}

#[test]
fn create_button_fails_empty_label() {
    let backend = AccessKitUiBackend::new();
    let result = backend.create_button("", 100, 50);
    assert!(result.is_err(), "empty label should fail");
}

#[test]
fn create_button_fails_small_target() {
    let backend = AccessKitUiBackend::new();
    let result = backend.create_button("Submit", 30, 20);
    assert!(result.is_err(), "small target should fail");
}

#[test]
fn contrast_enforcement() {
    let backend = AccessKitUiBackend::new();
    let white = SrgbColor {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    let light_gray = SrgbColor {
        r: 0.9,
        g: 0.9,
        b: 0.9,
    };
    let (widget_id, ..) = backend.create_label("test", "text").unwrap();
    // Light gray on white has very low contrast — should fail.
    let result = backend.set_colors(widget_id, light_gray, white);
    assert!(result.is_err(), "low contrast pair should be rejected");
}

#[test]
fn contrast_passes_high_contrast() {
    let backend = AccessKitUiBackend::new();
    let black = SrgbColor {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    let white = SrgbColor {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    let (widget_id, ..) = backend.create_label("test", "text").unwrap();
    let result = backend.set_colors(widget_id, black, white);
    assert!(result.is_ok(), "black on white should pass");
}

#[test]
fn widget_count_tracks_additions() {
    let backend = AccessKitUiBackend::new();
    let initial = backend.widget_count();
    backend.create_button("A", 50, 50).unwrap();
    backend.create_button("B", 50, 50).unwrap();
    backend.create_label("C", "text").unwrap();
    assert_eq!(backend.widget_count(), initial + 3);
}

#[test]
fn container_stack_returns_proof() {
    let backend = AccessKitUiBackend::new();
    let (id1, ..) = backend.create_button("X", 50, 50).unwrap();
    let (id2, ..) = backend.create_button("Y", 50, 50).unwrap();
    let result = backend.container_stack("horizontal", vec![id1, id2]);
    assert!(result.is_ok());
    let (container_id, _no_overflow) = result.unwrap();
    let _ = container_id;
}
