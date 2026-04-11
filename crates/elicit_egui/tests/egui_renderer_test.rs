//! Tests for the egui renderer — renders WCAG-verified AccessKit trees to egui widgets.
//!
//! Covers EguiBackend (via Layout::render), render_tree, and bounds_to_size.

use accesskit::{Node, NodeId, Rect, Role, Toggled, Tree, TreeId, TreeUpdate};
use elicit_egui::{EguiBackend, bounds_to_size, render_tree};
use elicit_ui::{Layout, LayoutBuilder, RenderStats, Viewport};
use std::collections::HashMap;

// ── Helpers ────────────────────────────────────────────────────

fn node_id(n: u64) -> NodeId {
    NodeId::from(n)
}

fn make_update(root_id: NodeId, nodes: Vec<(NodeId, Node)>, focus: NodeId) -> TreeUpdate {
    TreeUpdate {
        nodes,
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus,
    }
}

fn viewport() -> Viewport {
    Viewport::new(1920, 1080)
}

fn egui_ctx() -> egui::Context {
    egui::Context::default()
}

/// Build a root window node containing the given child IDs.
fn window_root(children: Vec<NodeId>) -> Node {
    let mut root = Node::new(Role::Window);
    root.set_children(children);
    root
}

/// Build a button node with label and bounds.
fn make_button(label: &str) -> Node {
    let mut node = Node::new(Role::Button);
    node.set_label(label);
    node.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 50.0,
    });
    node
}

/// Build a label node.
fn make_label(text: &str) -> Node {
    let mut node = Node::new(Role::Label);
    node.set_value(text);
    node
}

// ── EguiBackend via Layout::render ─────────────────────────────

#[test]
fn render_via_backend_single_button() {
    let root_id = node_id(0);
    let btn_id = node_id(1);

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![btn_id])),
            (btn_id, make_button("Click me")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (rendered, stats) = verified.render(&backend).unwrap();

    assert!(rendered.root() == root_id);
    assert_eq!(stats.widgets_rendered, 1, "Should render 1 button widget");
    assert_eq!(stats.containers_rendered, 1, "Root window is a container");
    assert_eq!(stats.nodes_visited, 2, "Root + button = 2 nodes");
}

// ── Single widget tests (via render_tree) ──────────────────────

#[test]
fn render_single_button() {
    let root_id = node_id(0);
    let btn_id = node_id(1);

    let mut nodes = HashMap::new();
    nodes.insert(root_id, window_root(vec![btn_id]));
    nodes.insert(btn_id, make_button("Click me"));

    let ctx = egui_ctx();
    let mut stats = RenderStats::default();
    let _output = ctx.run_ui(egui::RawInput::default(), |ui| {
        stats = render_tree(ui, &nodes, root_id);
    });

    assert_eq!(stats.widgets_rendered, 1);
    assert_eq!(stats.containers_rendered, 1);
    assert_eq!(stats.nodes_visited, 2);
}

#[test]
fn render_single_label() {
    let root_id = node_id(0);
    let lbl_id = node_id(1);

    let mut nodes = HashMap::new();
    nodes.insert(root_id, window_root(vec![lbl_id]));
    nodes.insert(lbl_id, make_label("Hello, world"));

    let ctx = egui_ctx();
    let mut stats = RenderStats::default();
    let _output = ctx.run_ui(egui::RawInput::default(), |ui| {
        stats = render_tree(ui, &nodes, root_id);
    });

    assert_eq!(stats.widgets_rendered, 1);
    assert_eq!(stats.containers_rendered, 1);
}

#[test]
fn render_checkbox() {
    let root_id = node_id(0);
    let cb_id = node_id(1);

    let mut cb = Node::new(Role::CheckBox);
    cb.set_label("Accept terms");
    cb.set_toggled(Toggled::False);
    cb.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 50.0,
        y1: 50.0,
    });

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![cb_id])), (cb_id, cb)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

#[test]
fn render_text_input() {
    let root_id = node_id(0);
    let input_id = node_id(1);

    let mut input = Node::new(Role::TextInput);
    input.set_label("Email");
    input.set_placeholder("user@example.com");
    input.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 200.0,
        y1: 30.0,
    });

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![input_id])), (input_id, input)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

#[test]
fn render_slider() {
    let root_id = node_id(0);
    let slider_id = node_id(1);

    let mut slider = Node::new(Role::Slider);
    slider.set_label("Volume");
    slider.set_numeric_value(50.0);
    slider.set_min_numeric_value(0.0);
    slider.set_max_numeric_value(100.0);
    slider.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 200.0,
        y1: 30.0,
    });

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![slider_id])), (slider_id, slider)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

#[test]
fn render_progress_bar() {
    let root_id = node_id(0);
    let pb_id = node_id(1);

    let mut pb = Node::new(Role::ProgressIndicator);
    pb.set_label("Loading...");
    pb.set_numeric_value(75.0);
    pb.set_max_numeric_value(100.0);
    pb.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 200.0,
        y1: 20.0,
    });

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![pb_id])), (pb_id, pb)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

#[test]
fn render_heading() {
    let root_id = node_id(0);
    let h_id = node_id(1);

    let mut heading = Node::new(Role::Heading);
    heading.set_value("Page Title");
    heading.set_level(1);
    heading.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 400.0,
        y1: 40.0,
    });

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![h_id])), (h_id, heading)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

// ── Composite form tests ──────────────────────────────────────

#[test]
fn render_login_form() {
    let root_id = node_id(0);
    let form_id = node_id(1);
    let email_id = node_id(2);
    let pass_id = node_id(3);
    let submit_id = node_id(4);
    let remember_id = node_id(5);

    let mut form = Node::new(Role::Form);
    form.set_children(vec![email_id, pass_id, remember_id, submit_id]);

    let mut email = Node::new(Role::EmailInput);
    email.set_label("Email");
    email.set_placeholder("you@example.com");
    email.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 200.0,
        y1: 30.0,
    });

    let mut pass = Node::new(Role::PasswordInput);
    pass.set_label("Password");
    pass.set_bounds(Rect {
        x0: 0.0,
        y0: 40.0,
        x1: 200.0,
        y1: 70.0,
    });

    let mut remember = Node::new(Role::CheckBox);
    remember.set_label("Remember me");
    remember.set_toggled(Toggled::False);
    remember.set_bounds(Rect {
        x0: 0.0,
        y0: 80.0,
        x1: 150.0,
        y1: 110.0,
    });

    let submit = make_button("Log in");

    let mut root = window_root(vec![form_id]);
    root.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 1920.0,
        y1: 1080.0,
    });

    let update = make_update(
        root_id,
        vec![
            (root_id, root),
            (form_id, form),
            (email_id, email),
            (pass_id, pass),
            (remember_id, remember),
            (submit_id, submit),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(rendered.root(), root_id);
    assert_eq!(
        stats.widgets_rendered, 4,
        "email + password + checkbox + button"
    );
    assert_eq!(stats.containers_rendered, 2, "window + form");
    assert_eq!(stats.nodes_visited, 6, "root + form + 4 children");
}

#[test]
fn render_toolbar_with_buttons() {
    let root_id = node_id(0);
    let toolbar_id = node_id(1);
    let btn1_id = node_id(2);
    let btn2_id = node_id(3);
    let btn3_id = node_id(4);

    let mut toolbar = Node::new(Role::Toolbar);
    toolbar.set_children(vec![btn1_id, btn2_id, btn3_id]);

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![toolbar_id])),
            (toolbar_id, toolbar),
            (btn1_id, make_button("Bold")),
            (btn2_id, make_button("Italic")),
            (btn3_id, make_button("Underline")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 3, "3 buttons in toolbar");
    assert_eq!(stats.containers_rendered, 2, "window + toolbar");
}

// ── Edge cases ─────────────────────────────────────────────────

#[test]
fn render_disabled_button() {
    let root_id = node_id(0);
    let btn_id = node_id(1);

    let mut btn = make_button("Cannot click");
    btn.set_disabled();

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![btn_id])), (btn_id, btn)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

#[test]
fn render_hidden_node_skipped() {
    let root_id = node_id(0);
    let hidden_id = node_id(1);
    let visible_id = node_id(2);

    let mut hidden = make_button("Hidden");
    hidden.set_hidden();

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![hidden_id, visible_id])),
            (hidden_id, hidden),
            (visible_id, make_button("Visible")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1, "Only visible button renders");
    assert_eq!(stats.nodes_skipped, 1, "Hidden button skipped");
}

#[test]
fn render_empty_tree() {
    let root_id = node_id(0);

    let update = make_update(root_id, vec![(root_id, window_root(vec![]))], root_id);

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 0);
    assert_eq!(stats.containers_rendered, 1, "Root is still a container");
    assert_eq!(stats.nodes_visited, 1);
}

#[test]
fn render_deeply_nested() {
    let root_id = node_id(0);
    let group1_id = node_id(1);
    let group2_id = node_id(2);
    let btn_id = node_id(3);

    let mut group1 = Node::new(Role::Group);
    group1.set_children(vec![group2_id]);

    let mut group2 = Node::new(Role::Group);
    group2.set_children(vec![btn_id]);

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![group1_id])),
            (group1_id, group1),
            (group2_id, group2),
            (btn_id, make_button("Deep")),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1, "1 deep button");
    assert_eq!(stats.containers_rendered, 3, "window + group1 + group2");
    assert_eq!(stats.nodes_visited, 4);
}

#[test]
fn render_radio_group() {
    let root_id = node_id(0);
    let rg_id = node_id(1);
    let r1_id = node_id(2);
    let r2_id = node_id(3);
    let r3_id = node_id(4);

    let mut rg = Node::new(Role::Group);
    rg.set_children(vec![r1_id, r2_id, r3_id]);

    let mut r1 = Node::new(Role::RadioButton);
    r1.set_label("Option A");
    r1.set_toggled(Toggled::True);
    r1.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 100.0,
        y1: 30.0,
    });

    let mut r2 = Node::new(Role::RadioButton);
    r2.set_label("Option B");
    r2.set_toggled(Toggled::False);
    r2.set_bounds(Rect {
        x0: 0.0,
        y0: 40.0,
        x1: 100.0,
        y1: 70.0,
    });

    let mut r3 = Node::new(Role::RadioButton);
    r3.set_label("Option C");
    r3.set_toggled(Toggled::False);
    r3.set_bounds(Rect {
        x0: 0.0,
        y0: 80.0,
        x1: 100.0,
        y1: 110.0,
    });

    let update = make_update(
        root_id,
        vec![
            (root_id, window_root(vec![rg_id])),
            (rg_id, rg),
            (r1_id, r1),
            (r2_id, r2),
            (r3_id, r3),
        ],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 3, "3 radio buttons");
    assert_eq!(stats.containers_rendered, 2, "window + group");
}

#[test]
fn render_link() {
    let root_id = node_id(0);
    let link_id = node_id(1);

    let mut link = Node::new(Role::Link);
    link.set_label("Visit docs");
    link.set_url("https://docs.example.com");
    link.set_bounds(Rect {
        x0: 0.0,
        y0: 0.0,
        x1: 150.0,
        y1: 20.0,
    });

    let update = make_update(
        root_id,
        vec![(root_id, window_root(vec![link_id])), (link_id, link)],
        root_id,
    );

    let layout = Layout::from_update(update);
    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 1);
}

// ── bounds_to_size ────────────────────────────────────────────

#[test]
fn bounds_to_size_none_without_bounds() {
    let node = Node::new(Role::Button);
    assert_eq!(bounds_to_size(&node), None);
}

#[test]
fn bounds_to_size_computes_correctly() {
    let mut node = Node::new(Role::Button);
    node.set_bounds(Rect {
        x0: 10.0,
        y0: 20.0,
        x1: 110.0,
        y1: 70.0,
    });
    let (w, h) = bounds_to_size(&node).expect("Should have bounds");
    assert!((w - 100.0).abs() < f32::EPSILON);
    assert!((h - 50.0).abs() < f32::EPSILON);
}

// ── Builder → verify → render round-trip ──────────────────────

#[test]
fn builder_verify_render_roundtrip() {
    let layout = LayoutBuilder::new()
        .button("Click me")
        .size(100, 50)
        .checkbox("Check me")
        .size(100, 30)
        .text_input("Type here")
        .size(200, 30)
        .build();

    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (rendered, stats) = verified.render(&backend).unwrap();

    assert!(rendered.root().0 == 0);
    assert_eq!(stats.widgets_rendered, 3, "button + checkbox + text input");
    assert_eq!(stats.containers_rendered, 1, "root window");
}

#[test]
fn builder_form_verify_render() {
    let layout = LayoutBuilder::new()
        .form()
        .text_input("Name")
        .size(200, 30)
        .text_input("Email")
        .size(200, 30)
        .button("Send")
        .size(100, 44)
        .end()
        .build();

    let verified = layout.verify_a(viewport()).expect("should verify");

    let ctx = egui_ctx();
    let backend = EguiBackend::new(&ctx);
    let (_rendered, stats) = verified.render(&backend).unwrap();

    assert_eq!(stats.widgets_rendered, 3, "2 inputs + 1 button");
    assert_eq!(stats.containers_rendered, 2, "window + form");
    assert_eq!(stats.nodes_visited, 5, "window + form + 3 children");
}
