//! Tests for the LayoutBuilder — ergonomic AccessKit tree construction.

use elicit_ui::{LayoutBuilder, Viewport};

fn viewport() -> Viewport {
    Viewport::new(1920, 1080)
}

// ── Basic widget tests ─────────────────────────────────────────

#[test]
fn build_single_button() {
    let layout = LayoutBuilder::new().button("Submit").size(100, 50).build();

    let verified = layout.verify_a(viewport()).expect("should verify");
    assert!(verified.root().0 == 0, "Root should be NodeId(0)");
}

#[test]
fn build_single_label() {
    let layout = LayoutBuilder::new().label("Hello, world").build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_single_checkbox() {
    let layout = LayoutBuilder::new()
        .checkbox("Accept terms")
        .size(100, 30)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_single_radio() {
    let layout = LayoutBuilder::new().radio("Option A").size(100, 30).build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_slider() {
    let layout = LayoutBuilder::new()
        .slider("Volume", 50.0, 0.0, 100.0)
        .size(200, 30)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_progress_bar() {
    let layout = LayoutBuilder::new()
        .progress("Loading", 75.0, 100.0)
        .size(200, 20)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_heading() {
    let layout = LayoutBuilder::new()
        .heading("Page Title", 1)
        .size(400, 40)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_link() {
    let layout = LayoutBuilder::new()
        .link("Documentation", "https://docs.example.com")
        .size(150, 20)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_text_input_with_placeholder() {
    let layout = LayoutBuilder::new()
        .text_input("Email")
        .placeholder("you@example.com")
        .size(200, 30)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_password_input() {
    let layout = LayoutBuilder::new()
        .password_input("Password")
        .size(200, 30)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

// ── Container tests ────────────────────────────────────────────

#[test]
fn build_form_with_children() {
    let layout = LayoutBuilder::new()
        .form()
        .text_input("Email")
        .size(200, 30)
        .text_input("Password")
        .size(200, 30)
        .button("Submit")
        .size(100, 50)
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_toolbar() {
    let layout = LayoutBuilder::new()
        .toolbar()
        .button("Bold")
        .size(50, 30)
        .button("Italic")
        .size(50, 30)
        .button("Underline")
        .size(80, 30)
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_nested_groups() {
    let layout = LayoutBuilder::new()
        .group()
        .label("Section A")
        .group()
        .button("Nested")
        .size(100, 50)
        .end()
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_list() {
    let layout = LayoutBuilder::new()
        .list()
        .label("Item 1")
        .label("Item 2")
        .label("Item 3")
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_navigation_section() {
    let layout = LayoutBuilder::new()
        .navigation()
        .link("Home", "/")
        .link("About", "/about")
        .link("Contact", "/contact")
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

// ── Property setter tests ──────────────────────────────────────

#[test]
fn disabled_button() {
    let layout = LayoutBuilder::new()
        .button("Cannot click")
        .size(100, 50)
        .disabled()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn checked_checkbox() {
    let layout = LayoutBuilder::new()
        .checkbox("Remember me")
        .checked(true)
        .size(100, 30)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn read_only_input() {
    let layout = LayoutBuilder::new()
        .text_input("ID")
        .value("12345")
        .read_only()
        .size(200, 30)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn custom_bounds() {
    let layout = LayoutBuilder::new()
        .button("Offset")
        .bounds(50.0, 50.0, 200.0, 100.0)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

// ── Composite form tests ──────────────────────────────────────

#[test]
fn build_login_form() {
    let layout = LayoutBuilder::new()
        .heading("Login", 1)
        .size(400, 40)
        .form()
        .text_input("Email")
        .placeholder("you@example.com")
        .size(300, 30)
        .password_input("Password")
        .size(300, 30)
        .checkbox("Remember me")
        .size(150, 30)
        .button("Log in")
        .size(120, 44)
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_settings_page() {
    let layout = LayoutBuilder::new()
        .heading("Settings", 1)
        .size(400, 40)
        .section()
        .heading("Display", 2)
        .size(300, 30)
        .slider("Brightness", 75.0, 0.0, 100.0)
        .size(250, 30)
        .checkbox("Dark mode")
        .size(150, 30)
        .end()
        .section()
        .heading("Audio", 2)
        .size(300, 30)
        .slider("Volume", 50.0, 0.0, 100.0)
        .size(250, 30)
        .checkbox("Mute")
        .size(100, 30)
        .end()
        .button("Save")
        .size(100, 44)
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_survey_form() {
    let layout = LayoutBuilder::new()
        .heading("Survey", 1)
        .size(400, 40)
        .form()
        .label("How satisfied are you?")
        .group()
        .radio("Very satisfied")
        .size(200, 30)
        .radio("Satisfied")
        .size(200, 30)
        .radio("Neutral")
        .size(200, 30)
        .radio("Dissatisfied")
        .size(200, 30)
        .end()
        .separator()
        .multiline_input("Comments")
        .placeholder("Tell us more...")
        .size(400, 100)
        .button("Submit")
        .size(100, 44)
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

// ── Edge cases ─────────────────────────────────────────────────

#[test]
fn build_empty_layout() {
    let layout = LayoutBuilder::new().build();
    layout
        .verify_a(viewport())
        .expect("empty layout should verify");
}

#[test]
fn build_deeply_nested() {
    let layout = LayoutBuilder::new()
        .group()
        .group()
        .group()
        .button("Deep")
        .size(100, 50)
        .end()
        .end()
        .end()
        .build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn build_many_widgets() {
    let mut b = LayoutBuilder::new();
    for i in 0..20 {
        b.button(&format!("Btn {i}")).size(80, 40);
    }
    let layout = b.build();

    layout.verify_a(viewport()).expect("should verify");
}

#[test]
fn unclosed_containers_auto_close() {
    // Deliberately don't call .end() — build() should handle it
    let layout = LayoutBuilder::new()
        .form()
        .group()
        .button("Nested")
        .size(100, 50)
        // No .end() calls
        .build();

    layout.verify_a(viewport()).expect("auto-close should work");
}

#[test]
#[should_panic(expected = "end() called with no open container")]
fn end_on_root_panics() {
    let mut b = LayoutBuilder::new();
    b.end(); // Only root is open — should panic
}

#[test]
fn default_creates_same_as_new() {
    let a = LayoutBuilder::default().build();
    let b = LayoutBuilder::new().build();
    // Both should produce valid empty layouts
    a.verify_a(viewport()).expect("default should verify");
    b.verify_a(viewport()).expect("new should verify");
}

// ── Verification failure with builder ──────────────────────────

#[test]
fn builder_missing_label_fails_verification() {
    // Create a button without label by using the raw Node API
    // The builder always sets labels, so we test that verification
    // catches issues in the AccessKit tree regardless of source
    use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
    use elicit_ui::Layout;

    let root_id = NodeId::from(0u64);
    let btn_id = NodeId::from(1u64);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![btn_id]);

    // Button with NO label — should fail
    let btn = Node::new(Role::Button);

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (btn_id, btn)],
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    let layout = Layout::from_update(update);
    let result = layout.verify_a(viewport());
    assert!(result.is_err(), "Missing label should fail verification");
}
