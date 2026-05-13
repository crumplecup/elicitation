//! Tests for the AccessKit → Leptos HTML5 / view! bridge.

use accesskit::{Node, NodeId, Role};
use elicit_leptos::{LeptosRenderMode, render_tree};
use std::collections::HashMap;

fn nid(n: u64) -> NodeId {
    NodeId::from(n)
}

fn singleton(role: Role) -> (HashMap<NodeId, Node>, NodeId) {
    let root_id = nid(0);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, Node::new(role));
    (nodes, root_id)
}

fn with_label(role: Role, label: &str) -> (HashMap<NodeId, Node>, NodeId) {
    let root_id = nid(0);
    let mut node = Node::new(role);
    node.set_label(label);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    (nodes, root_id)
}

fn with_value(role: Role, value: &str) -> (HashMap<NodeId, Node>, NodeId) {
    let root_id = nid(0);
    let mut node = Node::new(role);
    node.set_value(value);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    (nodes, root_id)
}

// ── Container roles ────────────────────────────────────────────────────────────

#[test]
fn window_renders_transparent() {
    // Window is a transparent container in the legacy renderer: no wrapper
    // element is emitted. Children are rendered at the same depth directly.
    let (nodes, root) = singleton(Role::Window);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.is_empty(), "got: {html}");
}

#[test]
fn main_renders_main_element() {
    let (nodes, root) = singleton(Role::Main);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<main"), "got: {html}");
}

#[test]
fn navigation_renders_nav() {
    let (nodes, root) = singleton(Role::Navigation);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<nav"), "got: {html}");
}

#[test]
fn section_renders_section() {
    let (nodes, root) = singleton(Role::Section);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<section"), "got: {html}");
}

#[test]
fn dialog_renders_dialog() {
    let (nodes, root) = singleton(Role::Dialog);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<dialog"), "got: {html}");
}

#[test]
fn banner_renders_header() {
    let (nodes, root) = singleton(Role::Banner);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<header"), "got: {html}");
}

#[test]
fn content_info_renders_footer() {
    let (nodes, root) = singleton(Role::ContentInfo);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<footer"), "got: {html}");
}

#[test]
fn form_renders_form() {
    let (nodes, root) = singleton(Role::Form);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<form"), "got: {html}");
}

// ── Group with/without label ───────────────────────────────────────────────────

#[test]
fn group_with_label_renders_fieldset() {
    let (nodes, root) = with_label(Role::Group, "My group");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<fieldset"), "got: {html}");
    assert!(html.contains("<legend"), "got: {html}");
    assert!(html.contains("My group"), "got: {html}");
}

#[test]
fn group_without_label_renders_div_with_role() {
    let (nodes, root) = singleton(Role::Group);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains(r#"role="group""#), "got: {html}");
}

// ── List roles ─────────────────────────────────────────────────────────────────

#[test]
fn list_renders_ul() {
    let (nodes, root) = singleton(Role::List);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<ul"), "got: {html}");
}

#[test]
fn description_list_renders_dl() {
    let (nodes, root) = singleton(Role::DescriptionList);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<dl"), "got: {html}");
}

#[test]
fn list_item_renders_li() {
    let (nodes, root) = with_value(Role::ListItem, "item one");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<li"), "got: {html}");
    assert!(html.contains("item one"), "got: {html}");
}

// ── Table roles ────────────────────────────────────────────────────────────────

#[test]
fn table_renders_table_element() {
    let (nodes, root) = singleton(Role::Table);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<table"), "got: {html}");
}

#[test]
fn grid_renders_table_with_grid_role() {
    let (nodes, root) = singleton(Role::Grid);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<table"), "got: {html}");
    assert!(html.contains(r#"role="grid""#), "got: {html}");
}

#[test]
fn row_renders_tr() {
    let (nodes, root) = singleton(Role::Row);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<tr"), "got: {html}");
}

#[test]
fn cell_renders_td() {
    let (nodes, root) = with_value(Role::Cell, "42");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<td"), "got: {html}");
    assert!(html.contains("42"), "got: {html}");
}

#[test]
fn column_header_has_scope_col() {
    let (nodes, root) = with_value(Role::ColumnHeader, "Name");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<th"), "got: {html}");
    assert!(html.contains(r#"scope="col""#), "got: {html}");
}

#[test]
fn row_header_has_scope_row() {
    let (nodes, root) = with_value(Role::RowHeader, "Row 1");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<th"), "got: {html}");
    assert!(html.contains(r#"scope="row""#), "got: {html}");
}

// ── Tab roles ──────────────────────────────────────────────────────────────────

#[test]
fn tablist_has_role_tablist() {
    let (nodes, root) = singleton(Role::TabList);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains(r#"role="tablist""#), "got: {html}");
}

#[test]
fn tab_renders_button_with_role_tab() {
    let (nodes, root) = with_value(Role::Tab, "Overview");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<button"), "got: {html}");
    assert!(html.contains(r#"role="tab""#), "got: {html}");
    assert!(html.contains("Overview"), "got: {html}");
}

// ── Menu roles ─────────────────────────────────────────────────────────────────

#[test]
fn menubar_renders_nav_with_menubar_role() {
    let (nodes, root) = singleton(Role::MenuBar);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<nav"), "got: {html}");
    assert!(html.contains(r#"role="menubar""#), "got: {html}");
}

#[test]
fn menu_item_renders_li_with_role() {
    let (nodes, root) = with_value(Role::MenuItem, "Open File");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<li"), "got: {html}");
    assert!(html.contains(r#"role="menuitem""#), "got: {html}");
}

// ── Interactive controls ───────────────────────────────────────────────────────

#[test]
fn button_renders_button_element() {
    let (nodes, root) = with_value(Role::Button, "Submit");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<button"), "got: {html}");
    assert!(html.contains("Submit"), "got: {html}");
}

#[test]
fn checkbox_renders_input_type_checkbox() {
    let (nodes, root) = with_label(Role::CheckBox, "Agree");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains(r#"type="checkbox""#), "got: {html}");
    assert!(html.contains("Agree"), "got: {html}");
}

#[test]
fn radio_renders_input_type_radio() {
    let (nodes, root) = with_label(Role::RadioButton, "Option A");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains(r#"type="radio""#), "got: {html}");
}

#[test]
fn text_input_renders_input_type_text() {
    let root_id = nid(0);
    let mut node = Node::new(Role::TextInput);
    node.set_value("hello");
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains(r#"type="text""#), "got: {html}");
    assert!(html.contains(r#"value="hello""#), "got: {html}");
}

#[test]
fn slider_renders_input_type_range() {
    let root_id = nid(0);
    let mut node = Node::new(Role::Slider);
    node.set_numeric_value(50.0);
    node.set_min_numeric_value(0.0);
    node.set_max_numeric_value(100.0);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains(r#"type="range""#), "got: {html}");
    assert!(html.contains(r#"value="50""#), "got: {html}");
}

#[test]
fn link_renders_anchor_with_href() {
    let root_id = nid(0);
    let mut node = Node::new(Role::Link);
    node.set_value("Click here");
    node.set_url("https://example.com");
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains("<a "), "got: {html}");
    assert!(html.contains("https://example.com"), "got: {html}");
    assert!(html.contains("Click here"), "got: {html}");
}

// ── Text and semantic content ──────────────────────────────────────────────────

#[test]
fn heading_level_2_renders_h2() {
    let root_id = nid(0);
    let mut node = Node::new(Role::Heading);
    node.set_value("Section Title");
    node.set_level(2);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains("<h2>"), "got: {html}");
    assert!(html.contains("Section Title"), "got: {html}");
    assert!(html.contains("</h2>"), "got: {html}");
}

#[test]
fn heading_level_1_renders_h1() {
    let root_id = nid(0);
    let mut node = Node::new(Role::Heading);
    node.set_value("Page Title");
    node.set_level(1);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains("<h1>"), "got: {html}");
}

#[test]
fn heading_default_level_renders_h2() {
    // no explicit level set → default 2
    let (nodes, root) = with_value(Role::Heading, "My Heading");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<h2>"), "got: {html}");
}

#[test]
fn paragraph_renders_p() {
    let (nodes, root) = with_value(Role::Paragraph, "Some text");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<p>"), "got: {html}");
    assert!(html.contains("Some text"), "got: {html}");
}

#[test]
fn label_renders_label_element() {
    let (nodes, root) = with_value(Role::Label, "Name");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<label>"), "got: {html}");
    assert!(html.contains("Name"), "got: {html}");
}

#[test]
fn code_renders_code_element() {
    let (nodes, root) = with_value(Role::Code, "let x = 1;");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<code>"), "got: {html}");
}

#[test]
fn strong_renders_strong_element() {
    let (nodes, root) = with_value(Role::Strong, "Important");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<strong>"), "got: {html}");
}

#[test]
fn emphasis_renders_em_element() {
    let (nodes, root) = with_value(Role::Emphasis, "Italics");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<em>"), "got: {html}");
}

// ── Structural primitives ──────────────────────────────────────────────────────

#[test]
fn splitter_renders_hr() {
    let (nodes, root) = singleton(Role::Splitter);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<hr"), "got: {html}");
}

#[test]
fn line_break_renders_br() {
    let (nodes, root) = singleton(Role::LineBreak);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<br"), "got: {html}");
}

// ── Tree roles ─────────────────────────────────────────────────────────────────

#[test]
fn tree_renders_ul_with_tree_role() {
    let (nodes, root) = singleton(Role::Tree);
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<ul"), "got: {html}");
    assert!(html.contains(r#"role="tree""#), "got: {html}");
}

#[test]
fn tree_item_renders_li_with_treeitem_role() {
    let (nodes, root) = with_value(Role::TreeItem, "Leaf node");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<li"), "got: {html}");
    assert!(html.contains(r#"role="treeitem""#), "got: {html}");
}

// ── HTML escaping ──────────────────────────────────────────────────────────────

#[test]
fn html_special_chars_are_escaped() {
    let (nodes, root) = with_value(Role::Paragraph, "<script>alert('xss')</script>");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(!html.contains("<script>"), "XSS unescaped: {html}");
    assert!(html.contains("&lt;script&gt;"), "got: {html}");
}

// ── ViewMacro mode ─────────────────────────────────────────────────────────────

#[test]
fn view_macro_heading_uses_quoted_text() {
    let root_id = nid(0);
    let mut node = Node::new(Role::Heading);
    node.set_value("Hello World");
    node.set_level(3);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let code = render_tree(&nodes, root_id, LeptosRenderMode::ViewMacro);
    assert!(code.contains("<h3>"), "got: {code}");
    assert!(code.contains("\"Hello World\""), "got: {code}");
}

#[test]
fn view_macro_paragraph_uses_quoted_text() {
    let (nodes, root) = with_value(Role::Paragraph, "body text");
    let code = render_tree(&nodes, root, LeptosRenderMode::ViewMacro);
    assert!(code.contains("<p>"), "got: {code}");
    assert!(code.contains("\"body text\""), "got: {code}");
}

#[test]
fn view_macro_button_uses_quoted_text() {
    let (nodes, root) = with_value(Role::Button, "Click me");
    let code = render_tree(&nodes, root, LeptosRenderMode::ViewMacro);
    assert!(code.contains("<button>"), "got: {code}");
    assert!(code.contains("\"Click me\""), "got: {code}");
}

// ── Hidden nodes are skipped ───────────────────────────────────────────────────

#[test]
fn hidden_node_produces_no_output() {
    let root_id = nid(0);
    let mut node = Node::new(Role::Paragraph);
    node.set_value("invisible");
    node.set_hidden();
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(
        html.is_empty(),
        "hidden node should produce empty output, got: {html}"
    );
}

// ── Nested tree (parent + children) ───────────────────────────────────────────

#[test]
fn nav_with_link_children_renders_correctly() {
    let nav_id = nid(0);
    let link1_id = nid(1);
    let link2_id = nid(2);

    let mut nav = Node::new(Role::Navigation);
    nav.set_children(vec![link1_id, link2_id]);

    let mut link1 = Node::new(Role::Link);
    link1.set_value("Home");
    link1.set_url("/");

    let mut link2 = Node::new(Role::Link);
    link2.set_value("About");
    link2.set_url("/about");

    let mut nodes = HashMap::new();
    nodes.insert(nav_id, nav);
    nodes.insert(link1_id, link1);
    nodes.insert(link2_id, link2);

    let html = render_tree(&nodes, nav_id, LeptosRenderMode::Html);
    assert!(html.contains("<nav"), "got: {html}");
    assert!(html.contains("href=\"/\""), "got: {html}");
    assert!(html.contains("href=\"/about\""), "got: {html}");
    assert!(html.contains("Home"), "got: {html}");
    assert!(html.contains("About"), "got: {html}");
}

#[test]
fn table_with_header_and_row_renders_full_structure() {
    let table_id = nid(0);
    let hdr_id = nid(1);
    let row_id = nid(2);
    let cell_id = nid(3);

    let mut table = Node::new(Role::Table);
    table.set_children(vec![hdr_id, row_id]);

    let mut hdr = Node::new(Role::ColumnHeader);
    hdr.set_value("ID");

    let mut row = Node::new(Role::Row);
    row.set_children(vec![cell_id]);

    let mut cell = Node::new(Role::Cell);
    cell.set_value("42");

    let mut nodes = HashMap::new();
    nodes.insert(table_id, table);
    nodes.insert(hdr_id, hdr);
    nodes.insert(row_id, row);
    nodes.insert(cell_id, cell);

    let html = render_tree(&nodes, table_id, LeptosRenderMode::Html);
    assert!(html.contains("<table"), "got: {html}");
    assert!(html.contains(r#"scope="col""#), "got: {html}");
    assert!(html.contains("<tr"), "got: {html}");
    assert!(html.contains("<td"), "got: {html}");
    assert!(html.contains("42"), "got: {html}");
}

// ── LeptosRenderer via UiRenderer ─────────────────────────────────────────────

#[test]
fn renderer_html_mode_stores_last_output() {
    use accesskit::NodeId;
    use elicit_leptos::LeptosRenderer;
    use elicit_ui::{UiTreeRenderer, VerifiedTree, Viewport};

    let root_id = NodeId::from(0u64);
    let mut root = Node::new(Role::Main);
    root.set_label("Test App");

    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(root_id, root);

    let viewport = Viewport::new(800u32, 600u32);
    let tree = VerifiedTree::from_parts(nodes, root_id, viewport);

    let renderer = LeptosRenderer::html();
    let (html, stats, _established) = renderer.render(&tree).expect("render ok");

    assert!(stats.nodes_visited > 0);
    assert!(html.contains("<main"), "got: {html}");
}

#[test]
fn renderer_view_macro_mode_uses_quoted_text() {
    use accesskit::NodeId;
    use elicit_leptos::LeptosRenderer;
    use elicit_ui::{UiTreeRenderer, VerifiedTree, Viewport};

    let root_id = NodeId::from(0u64);
    let mut root = Node::new(Role::Paragraph);
    root.set_value("hello");

    let mut nodes = std::collections::BTreeMap::new();
    nodes.insert(root_id, root);

    let viewport = Viewport::new(800u32, 600u32);
    let tree = VerifiedTree::from_parts(nodes, root_id, viewport);

    let renderer = LeptosRenderer::view_macro();
    let (code, _stats, _established) = renderer.render(&tree).expect("render ok");

    // ViewMacro mode produces the same HTML-like syntax as Html mode;
    // text content is not wrapped in Rust string quotes.
    assert!(code.contains("<p>"), "got: {code}");
    assert!(code.contains("hello"), "got: {code}");
}

#[test]
fn renderer_backend_name_is_leptos() {
    use elicit_leptos::LeptosRenderer;
    use elicit_ui::UiRenderBackend;
    let r = LeptosRenderer::html();
    assert_eq!(r.backend_name(), "leptos");
}

// ── TreeItem: selection + expansion ──────────────────────────────────────────

#[test]
fn tree_item_leaf_selected_has_aria_selected() {
    let root_id = nid(0);
    let mut node = Node::new(Role::TreeItem);
    node.set_value("users");
    node.set_selected(true);
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains(r#"aria-selected="true""#), "got: {html}");
    assert!(html.contains(r#"class="selected""#), "got: {html}");
}

#[test]
fn tree_item_leaf_not_selected_has_no_aria_selected() {
    let root_id = nid(0);
    let mut node = Node::new(Role::TreeItem);
    node.set_value("users");
    let mut nodes = HashMap::new();
    nodes.insert(root_id, node);
    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(!html.contains("aria-selected"), "got: {html}");
}

#[test]
fn tree_item_group_expanded_has_open_attribute() {
    use accesskit::Toggled;
    // Parent TreeItem (expandable) with one child leaf
    let parent_id = nid(0);
    let child_id = nid(1);
    let mut parent = Node::new(Role::TreeItem);
    parent.set_value("public");
    parent.set_toggled(Toggled::True);
    parent.set_children(vec![child_id]);
    let mut child = Node::new(Role::TreeItem);
    child.set_value("users");
    let mut nodes = HashMap::new();
    nodes.insert(parent_id, parent);
    nodes.insert(child_id, child);
    let html = render_tree(&nodes, parent_id, LeptosRenderMode::Html);
    assert!(html.contains("<details"), "got: {html}");
    assert!(
        html.contains(" open"),
        "expected open attribute, got: {html}"
    );
}

#[test]
fn tree_item_group_collapsed_has_no_open_attribute() {
    let parent_id = nid(0);
    let child_id = nid(1);
    let mut parent = Node::new(Role::TreeItem);
    parent.set_value("public");
    parent.set_children(vec![child_id]);
    let mut child = Node::new(Role::TreeItem);
    child.set_value("users");
    let mut nodes = HashMap::new();
    nodes.insert(parent_id, parent);
    nodes.insert(child_id, child);
    let html = render_tree(&nodes, parent_id, LeptosRenderMode::Html);
    assert!(html.contains("<details"), "got: {html}");
    assert!(
        !html.contains(" open"),
        "should not have open attribute, got: {html}"
    );
}

// ── Table: thead / tbody split ────────────────────────────────────────────────

#[test]
fn grid_with_header_and_body_rows_gets_thead_tbody() {
    // root: Grid
    //   hdr: Row → [ColumnHeader("id"), ColumnHeader("name")]
    //   r1:  Row → [Cell("1"), Cell("Alice")]
    let root_id = nid(0);
    let hdr_id = nid(1);
    let ch1_id = nid(2);
    let ch2_id = nid(3);
    let r1_id = nid(4);
    let c1_id = nid(5);
    let c2_id = nid(6);

    let mut root = Node::new(Role::Grid);
    root.set_children(vec![hdr_id, r1_id]);

    let mut hdr = Node::new(Role::Row);
    hdr.set_children(vec![ch1_id, ch2_id]);

    let mut ch1 = Node::new(Role::ColumnHeader);
    ch1.set_value("id");
    let mut ch2 = Node::new(Role::ColumnHeader);
    ch2.set_value("name");

    let mut r1 = Node::new(Role::Row);
    r1.set_children(vec![c1_id, c2_id]);

    let mut c1 = Node::new(Role::Cell);
    c1.set_value("1");
    let mut c2 = Node::new(Role::Cell);
    c2.set_value("Alice");

    let mut nodes = HashMap::new();
    nodes.insert(root_id, root);
    nodes.insert(hdr_id, hdr);
    nodes.insert(ch1_id, ch1);
    nodes.insert(ch2_id, ch2);
    nodes.insert(r1_id, r1);
    nodes.insert(c1_id, c1);
    nodes.insert(c2_id, c2);

    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(html.contains("<thead>"), "missing <thead>, got: {html}");
    assert!(html.contains("</thead>"), "missing </thead>, got: {html}");
    assert!(html.contains("<tbody>"), "missing <tbody>, got: {html}");
    assert!(html.contains("</tbody>"), "missing </tbody>, got: {html}");
    // Header must appear before body
    let thead_pos = html.find("<thead>").unwrap();
    let tbody_pos = html.find("<tbody>").unwrap();
    assert!(thead_pos < tbody_pos, "thead should come before tbody");
}

#[test]
fn table_without_column_headers_has_no_thead() {
    let root_id = nid(0);
    let r1_id = nid(1);
    let c1_id = nid(2);

    let mut root = Node::new(Role::Table);
    root.set_children(vec![r1_id]);
    let mut r1 = Node::new(Role::Row);
    r1.set_children(vec![c1_id]);
    let mut c1 = Node::new(Role::Cell);
    c1.set_value("foo");

    let mut nodes = HashMap::new();
    nodes.insert(root_id, root);
    nodes.insert(r1_id, r1);
    nodes.insert(c1_id, c1);

    let html = render_tree(&nodes, root_id, LeptosRenderMode::Html);
    assert!(!html.contains("<thead>"), "unexpected <thead>, got: {html}");
    assert!(html.contains("<tbody>"), "missing <tbody>, got: {html}");
}

// ── MultilineTextInput: value is rendered ─────────────────────────────────────

#[test]
fn textarea_renders_value_as_content() {
    let (nodes, root) = with_value(Role::MultilineTextInput, "SELECT * FROM users;");
    let html = render_tree(&nodes, root, LeptosRenderMode::Html);
    assert!(html.contains("<textarea"), "got: {html}");
    assert!(
        html.contains("SELECT * FROM users;"),
        "textarea value missing, got: {html}"
    );
    // Must not contain the literal two-character sequence backslash-n
    assert!(
        !html.contains(r"\n"),
        "literal \\n escape found in output, got: {html}"
    );
}
