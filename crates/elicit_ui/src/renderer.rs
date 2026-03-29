//! egui renderer for WCAG-verified AccessKit trees.
//!
//! Walks the AccessKit tree from root to leaves, mapping each
//! `accesskit::Role` to the corresponding egui widget. Because
//! the tree has already passed verification, every node is
//! guaranteed to satisfy its WCAG constraints before rendering.

use crate::RenderStats;
use accesskit::{Node, NodeId, Rect, Role, Toggled};
use std::collections::HashMap;

/// Render a verified AccessKit tree into an egui `Ui`.
///
/// Walks from `root` through the node map, creating egui widgets
/// for each node based on its role. Container nodes create nested
/// layouts; leaf nodes create widgets.
///
/// Returns [`RenderStats`] summarizing what was rendered.
#[tracing::instrument(skip(ui, nodes), fields(root = ?root))]
pub fn render_tree(ui: &mut egui::Ui, nodes: &HashMap<NodeId, Node>, root: NodeId) -> RenderStats {
    let mut stats = RenderStats::default();
    render_node_recursive(ui, nodes, root, &mut stats);
    tracing::debug!(
        visited = stats.nodes_visited,
        widgets = stats.widgets_rendered,
        containers = stats.containers_rendered,
        skipped = stats.nodes_skipped,
        "Render pass complete"
    );
    stats
}

/// Recursively render a single node and its children.
fn render_node_recursive(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node_id: NodeId,
    stats: &mut RenderStats,
) {
    let Some(node) = nodes.get(&node_id) else {
        stats.nodes_skipped += 1;
        return;
    };
    stats.nodes_visited += 1;

    if node.is_hidden() {
        stats.nodes_skipped += 1;
        return;
    }

    let role = node.role();
    match role {
        // ── Containers ──────────────────────────────────────
        Role::Window
        | Role::Pane
        | Role::Form
        | Role::Group
        | Role::Section
        | Role::Region
        | Role::Main
        | Role::GenericContainer
        | Role::Document => {
            render_container(ui, nodes, node, stats);
        }

        // ── Interactive widgets ─────────────────────────────
        Role::Button | Role::DefaultButton => {
            render_button(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::CheckBox | Role::MenuItemCheckBox => {
            render_checkbox(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::RadioButton | Role::MenuItemRadio => {
            render_radio(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::Switch => {
            render_switch(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::TextInput
        | Role::SearchInput
        | Role::EmailInput
        | Role::UrlInput
        | Role::PhoneNumberInput
        | Role::PasswordInput => {
            render_text_input(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::MultilineTextInput => {
            render_multiline_input(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::NumberInput | Role::SpinButton => {
            render_number_input(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::Slider => {
            render_slider(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::ProgressIndicator | Role::Meter => {
            render_progress(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::ComboBox | Role::EditableComboBox => {
            render_combobox(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::Link => {
            render_link(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::ColorWell => {
            render_color_well(ui, node);
            stats.widgets_rendered += 1;
        }

        // ── Text / semantic ─────────────────────────────────
        Role::Label
        | Role::Paragraph
        | Role::TextRun
        | Role::Heading
        | Role::Legend
        | Role::Caption
        | Role::Blockquote
        | Role::Code
        | Role::Strong
        | Role::Emphasis
        | Role::Mark
        | Role::Abbr
        | Role::Term
        | Role::Definition
        | Role::Note
        | Role::Status
        | Role::Alert
        | Role::Log
        | Role::Time
        | Role::Timer => {
            render_label(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::Image => {
            render_image_placeholder(ui, node);
            stats.widgets_rendered += 1;
        }

        // ── Structural containers that recurse ──────────────
        Role::Toolbar => {
            render_toolbar(ui, nodes, node, stats);
        }
        Role::List | Role::ListBox | Role::Feed | Role::DescriptionList => {
            render_list(ui, nodes, node, stats);
        }
        Role::Table | Role::Grid | Role::TreeGrid | Role::ListGrid => {
            render_table(ui, nodes, node, stats);
        }
        Role::TabList => {
            render_tab_list(ui, nodes, node, stats);
        }
        Role::Tab
        | Role::TabPanel
        | Role::ListItem
        | Role::Row
        | Role::Cell
        | Role::GridCell
        | Role::RowHeader
        | Role::ColumnHeader
        | Role::RowGroup
        | Role::TreeItem
        | Role::ListBoxOption
        | Role::MenuItem
        | Role::MenuListOption => {
            render_container(ui, nodes, node, stats);
        }
        Role::Dialog | Role::AlertDialog => {
            render_container(ui, nodes, node, stats);
        }
        Role::Menu | Role::MenuBar | Role::MenuListPopup => {
            render_container(ui, nodes, node, stats);
        }
        Role::Navigation
        | Role::Banner
        | Role::Complementary
        | Role::ContentInfo
        | Role::Header
        | Role::Footer
        | Role::SectionHeader
        | Role::SectionFooter
        | Role::Search
        | Role::Article => {
            render_container(ui, nodes, node, stats);
        }
        Role::ScrollView | Role::ScrollBar => {
            render_container(ui, nodes, node, stats);
        }

        // ── Separators / breaks ─────────────────────────────
        Role::Splitter => {
            ui.separator();
            stats.widgets_rendered += 1;
        }
        Role::LineBreak => {
            ui.end_row();
            stats.widgets_rendered += 1;
        }

        // ── Unsupported / unknown ───────────────────────────
        _ => {
            // For unknown roles, render children if any, or skip leaf
            if node.children().is_empty() {
                stats.nodes_skipped += 1;
            } else {
                render_container(ui, nodes, node, stats);
            }
        }
    }
}

// ── Container rendering ──────────────────────────────────────

fn render_container(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut RenderStats,
) {
    stats.containers_rendered += 1;
    let children = node.children();
    if children.is_empty() {
        return;
    }

    ui.group(|ui| {
        for child_id in children {
            render_node_recursive(ui, nodes, *child_id, stats);
        }
    });
}

fn render_toolbar(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut RenderStats,
) {
    stats.containers_rendered += 1;
    ui.horizontal(|ui| {
        for child_id in node.children() {
            render_node_recursive(ui, nodes, *child_id, stats);
        }
    });
}

fn render_list(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut RenderStats,
) {
    stats.containers_rendered += 1;
    ui.vertical(|ui| {
        for child_id in node.children() {
            render_node_recursive(ui, nodes, *child_id, stats);
        }
    });
}

fn render_table(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut RenderStats,
) {
    stats.containers_rendered += 1;
    egui::Grid::new(format!("grid_{:?}", node.role()))
        .striped(true)
        .show(ui, |ui| {
            for child_id in node.children() {
                render_node_recursive(ui, nodes, *child_id, stats);
            }
        });
}

fn render_tab_list(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut RenderStats,
) {
    stats.containers_rendered += 1;
    ui.horizontal(|ui| {
        for child_id in node.children() {
            render_node_recursive(ui, nodes, *child_id, stats);
        }
    });
}

// ── Widget rendering ─────────────────────────────────────────

fn node_label(node: &Node) -> String {
    node.label().or(node.value()).unwrap_or("").to_string()
}

fn render_button(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    let mut btn = egui::Button::new(&text);
    if node.is_disabled() {
        btn = btn.sense(egui::Sense::hover());
    }
    ui.add(btn);
}

fn render_checkbox(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    let mut checked = matches!(node.toggled(), Some(Toggled::True));
    ui.add_enabled(
        !node.is_disabled(),
        egui::Checkbox::new(&mut checked, &text),
    );
}

fn render_radio(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    let selected = matches!(node.toggled(), Some(Toggled::True));
    ui.add_enabled(!node.is_disabled(), egui::RadioButton::new(selected, &text));
}

fn render_switch(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    let mut on = matches!(node.toggled(), Some(Toggled::True));
    ui.horizontal(|ui| {
        ui.add_enabled(!node.is_disabled(), egui::Checkbox::new(&mut on, &text));
    });
}

fn render_text_input(ui: &mut egui::Ui, node: &Node) {
    let mut buf = node.value().unwrap_or("").to_string();
    let mut te = egui::TextEdit::singleline(&mut buf);
    if let Some(hint) = node.placeholder() {
        te = te.hint_text(hint);
    }
    if node.is_read_only() || node.is_disabled() {
        te = te.interactive(false);
    }
    ui.add(te);
}

fn render_multiline_input(ui: &mut egui::Ui, node: &Node) {
    let mut buf = node.value().unwrap_or("").to_string();
    let mut te = egui::TextEdit::multiline(&mut buf);
    if let Some(hint) = node.placeholder() {
        te = te.hint_text(hint);
    }
    if node.is_read_only() || node.is_disabled() {
        te = te.interactive(false);
    }
    ui.add(te);
}

fn render_number_input(ui: &mut egui::Ui, node: &Node) {
    let mut val = node.numeric_value().unwrap_or(0.0);
    let min = node.min_numeric_value().unwrap_or(f64::MIN);
    let max = node.max_numeric_value().unwrap_or(f64::MAX);
    let step = node.numeric_value_step().unwrap_or(1.0);
    ui.add_enabled(
        !node.is_disabled(),
        egui::DragValue::new(&mut val).range(min..=max).speed(step),
    );
}

fn render_slider(ui: &mut egui::Ui, node: &Node) {
    let mut val = node.numeric_value().unwrap_or(0.0);
    let min = node.min_numeric_value().unwrap_or(0.0);
    let max = node.max_numeric_value().unwrap_or(100.0);
    let text = node_label(node);
    let mut slider = egui::Slider::new(&mut val, min..=max);
    if !text.is_empty() {
        slider = slider.text(&text);
    }
    ui.add_enabled(!node.is_disabled(), slider);
}

fn render_progress(ui: &mut egui::Ui, node: &Node) {
    let val = node.numeric_value().unwrap_or(0.0);
    let max = node.max_numeric_value().unwrap_or(100.0);
    let fraction = if max > 0.0 { (val / max) as f32 } else { 0.0 };
    let text = node_label(node);
    let mut pb = egui::ProgressBar::new(fraction.clamp(0.0, 1.0));
    if !text.is_empty() {
        pb = pb.text(&text);
    }
    ui.add(pb);
}

fn render_combobox(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    let selected = node.value().unwrap_or("").to_string();
    egui::ComboBox::from_label(&text)
        .selected_text(&selected)
        .show_ui(ui, |_ui| {
            // Children would provide options — rendered from tree children
        });
}

fn render_link(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    let url = node.url().unwrap_or("#");
    ui.add(egui::Hyperlink::from_label_and_url(&text, url));
}

fn render_color_well(ui: &mut egui::Ui, node: &Node) {
    let text = node_label(node);
    // No native egui color picker widget for inline use — show a button
    let btn = egui::Button::new(format!("🎨 {text}"));
    ui.add_enabled(!node.is_disabled(), btn);
}

fn render_label(ui: &mut egui::Ui, node: &Node) {
    let text = node.value().or(node.label()).unwrap_or("");
    let role = node.role();

    let rt = match role {
        Role::Heading => {
            let size = heading_size(node);
            egui::RichText::new(text).strong().size(size)
        }
        Role::Strong => egui::RichText::new(text).strong(),
        Role::Emphasis | Role::Mark => egui::RichText::new(text).italics(),
        Role::Code => egui::RichText::new(text).monospace(),
        Role::Alert | Role::Status => egui::RichText::new(text).color(egui::Color32::YELLOW),
        _ => egui::RichText::new(text),
    };

    ui.add(egui::Label::new(rt));
}

fn render_image_placeholder(ui: &mut egui::Ui, node: &Node) {
    let alt = node_label(node);
    let text = if alt.is_empty() {
        "🖼 [image]".to_string()
    } else {
        format!("🖼 {alt}")
    };
    ui.label(text);
}

/// Determine heading font size from the hierarchical level property.
fn heading_size(node: &Node) -> f32 {
    // AccessKit stores heading level via level()
    match node.level() {
        Some(1) => 28.0,
        Some(2) => 22.0,
        Some(3) => 18.0,
        Some(4) => 16.0,
        Some(5) => 14.0,
        _ => 12.0,
    }
}

/// Compute minimum desired size from AccessKit bounds.
///
/// Returns `(width, height)` if the node has bounds set, otherwise `None`.
pub fn bounds_to_size(node: &Node) -> Option<(f32, f32)> {
    let Rect { x0, y0, x1, y1 } = node.bounds()?;
    let w = (x1 - x0).abs() as f32;
    let h = (y1 - y0).abs() as f32;
    Some((w, h))
}
