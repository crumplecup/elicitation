//! Bidirectional conversion between [`BevyUiNode`] and AccessKit [`TreeUpdate`].
//!
//! Two public entry points:
//!
//! - [`bevy_ui_node_to_tree_update`] — forward bridge: `BevyUiNode` → AccessKit IR
//! - [`tree_update_to_bevy_ui_node`] — reverse bridge: AccessKit IR → `BevyUiNode`
//!
//! The IR is authoritative: the reverse direction reads role, label, value and
//! state fields directly from the AccessKit [`Node`], requiring no additional
//! context beyond the `TreeUpdate` itself.

use accesskit::{Node, NodeId, Role, Toggled, Tree, TreeId, TreeUpdate};
use elicit_ui::WcagNodeProofs;
use std::collections::HashMap;
use tracing::instrument;

use crate::BevyUiNode;

// ── Forward: BevyUiNode → TreeUpdate ─────────────────────────────────────────

/// Convert a [`BevyUiNode`] tree to an AccessKit [`TreeUpdate`].
///
/// Assigns sequential [`NodeId`]s starting from 1.  The returned `TreeUpdate`
/// has `tree` set to `Some(Tree { root: NodeId(1) })` and `nodes` populated
/// in DFS pre-order.
#[instrument(skip(root))]
pub fn bevy_ui_node_to_tree_update(root: &BevyUiNode) -> TreeUpdate {
    let mut nodes: Vec<(NodeId, Node)> = Vec::new();
    let mut next_id: u64 = 1;
    let root_id = encode_node(root, &mut nodes, &mut next_id);
    TreeUpdate {
        nodes,
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}

fn encode_node(node: &BevyUiNode, out: &mut Vec<(NodeId, Node)>, next_id: &mut u64) -> NodeId {
    let id = NodeId(*next_id);
    *next_id += 1;

    match node {
        BevyUiNode::Container {
            role,
            label,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let ak_role = container_role_str_to_accesskit(role);
            let mut builder = Node::new(ak_role);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::Text {
            content,
            level,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(if level.is_some() {
                Role::Heading
            } else {
                Role::Label
            });
            builder.set_label(content.clone());
            if let Some(lvl) = level {
                builder.set_level((*lvl).into());
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::Button {
            label,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Button);
            builder.set_label(label.clone());
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::CheckBox {
            label,
            checked,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::CheckBox);
            builder.set_label(label.clone());
            let toggled = match checked {
                Some(true) => Toggled::True,
                Some(false) => Toggled::False,
                None => Toggled::Mixed,
            };
            builder.set_toggled(toggled);
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::RadioButton {
            label,
            selected,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::RadioButton);
            builder.set_label(label.clone());
            if *selected {
                builder.set_toggled(Toggled::True);
            } else {
                builder.set_toggled(Toggled::False);
            }
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::Switch {
            label,
            on,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Switch);
            builder.set_label(label.clone());
            builder.set_toggled(if *on { Toggled::True } else { Toggled::False });
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::Slider {
            label,
            value,
            min,
            max,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Slider);
            builder.set_label(label.clone());
            if let Some(v) = value {
                builder.set_numeric_value(*v);
            }
            if let Some(m) = min {
                builder.set_min_numeric_value(*m);
            }
            if let Some(m) = max {
                builder.set_max_numeric_value(*m);
            }
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::SpinButton {
            label,
            value,
            min,
            max,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::SpinButton);
            builder.set_label(label.clone());
            if let Some(v) = value {
                builder.set_numeric_value(*v);
            }
            if let Some(m) = min {
                builder.set_min_numeric_value(*m);
            }
            if let Some(m) = max {
                builder.set_max_numeric_value(*m);
            }
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::Progress {
            label,
            value,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::ProgressIndicator);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            if let Some(v) = value {
                builder.set_numeric_value(*v);
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::TextInput {
            label,
            value,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::TextInput);
            builder.set_label(label.clone());
            builder.set_value(value.clone());
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::TextArea {
            label,
            value,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::MultilineTextInput);
            builder.set_label(label.clone());
            builder.set_value(value.clone());
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::ColorWell {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::ColorWell);
            builder.set_label(label.clone());
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::ComboBox {
            label,
            value,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::ComboBox);
            builder.set_label(label.clone());
            builder.set_value(value.clone());
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::ListBox {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::ListBox);
            builder.set_label(label.clone());
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::Image { alt, children, .. } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Image);
            if let Some(alt_text) = alt.as_deref() {
                builder.set_label(alt_text.to_string());
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::ScrollView {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::ScrollView);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::Table {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Table);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::Tab {
            label,
            selected,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Tab);
            builder.set_label(label.clone());
            if *selected {
                builder.set_selected(true);
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::TabPanel {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::TabPanel);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::Menu {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Menu);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::MenuItem {
            label,
            disabled,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::MenuItem);
            builder.set_label(label.clone());
            if *disabled {
                builder.set_disabled();
            }
            if !child_ids.is_empty() {
                builder.set_children(child_ids);
            }
            out.push((id, builder));
        }
        BevyUiNode::Tree {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::Tree);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::TreeItem {
            label,
            expanded,
            children,
            ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::TreeItem);
            builder.set_label(label.clone());
            builder.set_expanded(*expanded);
            builder.set_children(child_ids);
            out.push((id, builder));
        }
        BevyUiNode::Unknown {
            label, children, ..
        } => {
            let child_ids: Vec<NodeId> = children
                .iter()
                .map(|c| encode_node(c, out, next_id))
                .collect();
            let mut builder = Node::new(Role::GenericContainer);
            if let Some(lbl) = label.as_deref() {
                builder.set_label(lbl.to_string());
            }
            builder.set_children(child_ids);
            out.push((id, builder));
        }
    }

    id
}

/// Map a role string (as stored in [`BevyUiNode::Container::role`]) back to an
/// AccessKit [`Role`].
fn container_role_str_to_accesskit(role: &str) -> Role {
    match role {
        "window" => Role::Window,
        "pane" => Role::Pane,
        "dialog" => Role::Dialog,
        "application" => Role::Application,
        "document" => Role::Document,
        "root_web_area" => Role::RootWebArea,
        "main" => Role::Main,
        "navigation" => Role::Navigation,
        "banner" => Role::Banner,
        "content_info" => Role::ContentInfo,
        "complementary" => Role::Complementary,
        "form" => Role::Form,
        "search" => Role::Search,
        "region" => Role::Region,
        "section" => Role::Section,
        "article" => Role::Article,
        "group" => Role::Group,
        "details" => Role::Details,
        "toolbar" => Role::Toolbar,
        "radio_group" => Role::RadioGroup,
        "tab_list" => Role::TabList,
        "list" => Role::List,
        "list_item" => Role::ListItem,
        "description_list" => Role::DescriptionList,
        "row" => Role::Row,
        "row_group" => Role::RowGroup,
        "cell" => Role::Cell,
        "column_header" => Role::ColumnHeader,
        "row_header" => Role::RowHeader,
        "figure" => Role::Figure,
        "tooltip" => Role::Tooltip,
        "alert" => Role::Alert,
        "status" => Role::Status,
        "timer" => Role::Timer,
        "terminal" => Role::Terminal,
        "canvas" => Role::Canvas,
        "video" => Role::Video,
        "audio" => Role::Audio,
        "section_header" => Role::SectionHeader,
        "section_footer" => Role::SectionFooter,
        _ => Role::GenericContainer,
    }
}

// ── Reverse: TreeUpdate → BevyUiNode ─────────────────────────────────────────

/// Convert an AccessKit [`TreeUpdate`] to a [`BevyUiNode`] tree.
///
/// Returns `None` if the `TreeUpdate` has no tree root or the root node is
/// absent from the node map.
#[instrument(skip(update), fields(n_nodes = update.nodes.len()))]
pub fn tree_update_to_bevy_ui_node(update: &TreeUpdate) -> Option<BevyUiNode> {
    let root_id = update.tree.as_ref()?.root;
    let map: HashMap<NodeId, &Node> = update.nodes.iter().map(|(id, n)| (*id, n)).collect();
    decode_node(root_id, &map)
}

fn decode_node(id: NodeId, map: &HashMap<NodeId, &Node>) -> Option<BevyUiNode> {
    let node = map.get(&id)?;
    let label = node.label().map(|s| s.to_string());
    let proofs = WcagNodeProofs::default();

    let children_ids: Vec<NodeId> = node.children().to_vec();
    let children: Vec<BevyUiNode> = children_ids
        .iter()
        .filter_map(|cid| decode_node(*cid, map))
        .collect();

    let bevy_node = match node.role() {
        // ── Containers ──
        Role::Window => BevyUiNode::Container {
            role: "window".into(),
            label,
            children,
            proofs,
        },
        Role::Pane => BevyUiNode::Container {
            role: "pane".into(),
            label,
            children,
            proofs,
        },
        Role::Dialog => BevyUiNode::Container {
            role: "dialog".into(),
            label,
            children,
            proofs,
        },
        Role::Application => BevyUiNode::Container {
            role: "application".into(),
            label,
            children,
            proofs,
        },
        Role::Document => BevyUiNode::Container {
            role: "document".into(),
            label,
            children,
            proofs,
        },
        Role::RootWebArea => BevyUiNode::Container {
            role: "root_web_area".into(),
            label,
            children,
            proofs,
        },
        Role::Main => BevyUiNode::Container {
            role: "main".into(),
            label,
            children,
            proofs,
        },
        Role::Navigation => BevyUiNode::Container {
            role: "navigation".into(),
            label,
            children,
            proofs,
        },
        Role::Banner => BevyUiNode::Container {
            role: "banner".into(),
            label,
            children,
            proofs,
        },
        Role::ContentInfo => BevyUiNode::Container {
            role: "content_info".into(),
            label,
            children,
            proofs,
        },
        Role::Complementary => BevyUiNode::Container {
            role: "complementary".into(),
            label,
            children,
            proofs,
        },
        Role::Form => BevyUiNode::Container {
            role: "form".into(),
            label,
            children,
            proofs,
        },
        Role::Search => BevyUiNode::Container {
            role: "search".into(),
            label,
            children,
            proofs,
        },
        Role::Region => BevyUiNode::Container {
            role: "region".into(),
            label,
            children,
            proofs,
        },
        Role::Section => BevyUiNode::Container {
            role: "section".into(),
            label,
            children,
            proofs,
        },
        Role::SectionHeader => BevyUiNode::Container {
            role: "section_header".into(),
            label,
            children,
            proofs,
        },
        Role::SectionFooter => BevyUiNode::Container {
            role: "section_footer".into(),
            label,
            children,
            proofs,
        },
        Role::Article => BevyUiNode::Container {
            role: "article".into(),
            label,
            children,
            proofs,
        },
        Role::Group => BevyUiNode::Container {
            role: "group".into(),
            label,
            children,
            proofs,
        },
        Role::Details => BevyUiNode::Container {
            role: "details".into(),
            label,
            children,
            proofs,
        },
        Role::Toolbar => BevyUiNode::Container {
            role: "toolbar".into(),
            label,
            children,
            proofs,
        },
        Role::RadioGroup => BevyUiNode::Container {
            role: "radio_group".into(),
            label,
            children,
            proofs,
        },
        Role::TabList => BevyUiNode::Container {
            role: "tab_list".into(),
            label,
            children,
            proofs,
        },
        Role::Figure => BevyUiNode::Container {
            role: "figure".into(),
            label,
            children,
            proofs,
        },
        Role::GenericContainer => BevyUiNode::Container {
            role: "generic".into(),
            label,
            children,
            proofs,
        },
        Role::Terminal => BevyUiNode::Container {
            role: "terminal".into(),
            label,
            children,
            proofs,
        },
        Role::Canvas => BevyUiNode::Container {
            role: "canvas".into(),
            label,
            children,
            proofs,
        },
        Role::Video => BevyUiNode::Container {
            role: "video".into(),
            label,
            children,
            proofs,
        },
        Role::Audio => BevyUiNode::Container {
            role: "audio".into(),
            label,
            children,
            proofs,
        },
        Role::Alert => BevyUiNode::Container {
            role: "alert".into(),
            label,
            children,
            proofs,
        },
        Role::Status => BevyUiNode::Container {
            role: "status".into(),
            label,
            children,
            proofs,
        },
        Role::Timer => BevyUiNode::Container {
            role: "timer".into(),
            label,
            children,
            proofs,
        },
        Role::Tooltip => BevyUiNode::Container {
            role: "tooltip".into(),
            label,
            children,
            proofs,
        },
        // ── Interactive controls ──
        Role::Button | Role::DefaultButton => BevyUiNode::Button {
            label: label.unwrap_or_default(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::CheckBox => BevyUiNode::CheckBox {
            label: label.unwrap_or_default(),
            checked: toggled_to_option(node.toggled()),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::RadioButton => BevyUiNode::RadioButton {
            label: label.unwrap_or_default(),
            selected: node.toggled() == Some(Toggled::True),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::Switch => BevyUiNode::Switch {
            label: label.unwrap_or_default(),
            on: node.toggled() == Some(Toggled::True),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::Slider => BevyUiNode::Slider {
            label: label.unwrap_or_default(),
            value: node.numeric_value(),
            min: node.min_numeric_value(),
            max: node.max_numeric_value(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::SpinButton => BevyUiNode::SpinButton {
            label: label.unwrap_or_default(),
            value: node.numeric_value(),
            min: node.min_numeric_value(),
            max: node.max_numeric_value(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::ProgressIndicator | Role::ScrollBar => BevyUiNode::Progress {
            label,
            value: node.numeric_value(),
            children,
            proofs,
        },
        Role::ScrollView => BevyUiNode::ScrollView {
            label,
            children,
            proofs,
        },
        Role::ComboBox | Role::EditableComboBox => BevyUiNode::ComboBox {
            label: label.unwrap_or_default(),
            value: node.value().unwrap_or("").to_string(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::ListBox => BevyUiNode::ListBox {
            label: label.unwrap_or_default(),
            children,
            proofs,
        },
        Role::ColorWell => BevyUiNode::ColorWell {
            label: label.unwrap_or_default(),
            children,
            proofs,
        },
        Role::DisclosureTriangle => BevyUiNode::Button {
            label: label.unwrap_or_default(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::Splitter => BevyUiNode::Container {
            role: "splitter".into(),
            label,
            children,
            proofs,
        },
        // ── Text inputs ──
        Role::TextInput
        | Role::SearchInput
        | Role::EmailInput
        | Role::NumberInput
        | Role::PasswordInput
        | Role::PhoneNumberInput
        | Role::UrlInput
        | Role::DateInput
        | Role::DateTimeInput
        | Role::WeekInput
        | Role::MonthInput
        | Role::TimeInput => BevyUiNode::TextInput {
            label: label.unwrap_or_default(),
            value: node.value().unwrap_or("").to_string(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        Role::MultilineTextInput => BevyUiNode::TextArea {
            label: label.unwrap_or_default(),
            value: node.value().unwrap_or("").to_string(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        // ── Text / inline ──
        Role::Heading => BevyUiNode::Text {
            content: label.unwrap_or_default(),
            level: node.level().map(|l| l as u8),
            children,
            proofs,
        },
        Role::Paragraph
        | Role::Label
        | Role::TextRun
        | Role::LineBreak
        | Role::Blockquote
        | Role::Code
        | Role::Math
        | Role::Note
        | Role::Term
        | Role::Definition
        | Role::Abbr
        | Role::Emphasis
        | Role::Strong
        | Role::Mark
        | Role::Caption
        | Role::FigureCaption => BevyUiNode::Text {
            content: label.unwrap_or_default(),
            level: None,
            children,
            proofs,
        },
        // ── Link ──
        Role::Link => BevyUiNode::Button {
            label: label.unwrap_or_default(),
            disabled: node.is_disabled(),
            children,
            proofs,
        },
        // ── Media ──
        Role::Image => BevyUiNode::Image {
            alt: label,
            children,
            proofs,
        },
        // ── Table ──
        Role::Table
        | Role::Grid
        | Role::TreeGrid
        | Role::Row
        | Role::RowGroup
        | Role::Cell
        | Role::ColumnHeader
        | Role::RowHeader
        | Role::GridCell => BevyUiNode::Table {
            label,
            children,
            proofs,
        },
        // ── Tabs ──
        Role::Tab => BevyUiNode::Tab {
            label: label.unwrap_or_default(),
            selected: node.is_selected().unwrap_or(false),
            children,
            proofs,
        },
        Role::TabPanel => BevyUiNode::TabPanel {
            label,
            children,
            proofs,
        },
        // ── Menu ──
        Role::Menu | Role::MenuBar | Role::MenuListPopup => BevyUiNode::Menu {
            label,
            children,
            proofs,
        },
        Role::MenuItem | Role::MenuItemCheckBox | Role::MenuItemRadio | Role::MenuListOption => {
            BevyUiNode::MenuItem {
                label: label.unwrap_or_default(),
                disabled: node.is_disabled(),
                children,
                proofs,
            }
        }
        // ── Tree ──
        Role::Tree => BevyUiNode::Tree {
            label,
            children,
            proofs,
        },
        Role::TreeItem => BevyUiNode::TreeItem {
            label: label.unwrap_or_default(),
            expanded: node.is_expanded().unwrap_or(false),
            children,
            proofs,
        },
        // ── Lists ──
        Role::List | Role::DescriptionList => BevyUiNode::Container {
            role: "list".into(),
            label,
            children,
            proofs,
        },
        Role::ListItem | Role::ListMarker => BevyUiNode::Text {
            content: label.unwrap_or_default(),
            level: None,
            children,
            proofs,
        },
        // ── Fallback ──
        _ => BevyUiNode::Unknown {
            label,
            children,
            proofs,
        },
    };

    Some(bevy_node)
}

/// Convert an AccessKit [`Toggled`] state to a `checked` boolean option.
fn toggled_to_option(t: Option<Toggled>) -> Option<bool> {
    match t {
        Some(Toggled::True) => Some(true),
        Some(Toggled::False) => Some(false),
        Some(Toggled::Mixed) | None => None,
    }
}
