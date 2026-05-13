//! Forward bridge: `UiNode` → AccessKit `TreeUpdate`.
//!
//! Converts egui shadow types (`UiNode`, `WidgetJson`, `ContainerJson`)
//! into an AccessKit tree. This enables feeding egui scene descriptions
//! into the shared AccessKit IR for verification and cross-frontend
//! translation.

use crate::serde_types::{ContainerJson, LayoutJson, UiNode, WidgetJson};
use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};

/// Convert a `UiNode` tree into an AccessKit `TreeUpdate`.
///
/// Assigns deterministic `NodeId`s via depth-first index. The root
/// gets `NodeId(0)`.
#[tracing::instrument(skip(root_node))]
pub fn ui_node_to_tree_update(root_node: &UiNode) -> TreeUpdate {
    let mut nodes = Vec::new();
    let mut next_id: u64 = 0;

    let root_id = NodeId::from(next_id);
    convert_node(root_node, &mut nodes, &mut next_id);

    TreeUpdate {
        nodes,
        tree: Some(Tree::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}

/// Convert an AccessKit `TreeUpdate` back into a `UiNode` tree.
///
/// Walks from the root, mapping AccessKit Roles back to the closest
/// egui widget or container variant. Lossy for roles with no egui
/// equivalent (falls back to `WidgetJson::Label`).
#[tracing::instrument(skip(update))]
pub fn tree_update_to_ui_node(update: &TreeUpdate) -> Option<UiNode> {
    let root_id = update.tree.as_ref()?.root;
    let node_map: std::collections::BTreeMap<NodeId, &Node> =
        update.nodes.iter().map(|(id, n)| (*id, n)).collect();

    Some(convert_accesskit_node(root_id, &node_map))
}

// ── Forward: UiNode → AccessKit ─────────────────────────────

fn convert_node(ui_node: &UiNode, nodes: &mut Vec<(NodeId, Node)>, next_id: &mut u64) {
    let my_id = NodeId::from(*next_id);
    *next_id += 1;

    match ui_node {
        UiNode::Widget { widget } => {
            let node = widget_to_accesskit(widget);
            nodes.push((my_id, node));
        }
        UiNode::Container {
            container,
            children,
        } => {
            let mut child_ids = Vec::with_capacity(children.len());
            let first_child_id = *next_id;

            // Reserve IDs for children by pre-counting
            let child_starts: Vec<u64> = children
                .iter()
                .scan(*next_id, |state, _| {
                    let id = *state;
                    // We don't know exact subtree size, so we process sequentially
                    Some(id)
                })
                .collect();
            let _ = child_starts; // not used — we do sequential processing

            // Process children sequentially to get correct IDs
            let _ = first_child_id;
            for child in children {
                let child_id = NodeId::from(*next_id);
                child_ids.push(child_id);
                convert_node(child, nodes, next_id);
            }

            let mut node = container_to_accesskit(container);
            node.set_children(child_ids);
            nodes.push((my_id, node));
        }
        UiNode::Layout {
            layout, children, ..
        } => {
            let mut child_ids = Vec::with_capacity(children.len());

            for child in children {
                let child_id = NodeId::from(*next_id);
                child_ids.push(child_id);
                convert_node(child, nodes, next_id);
            }

            let mut node = layout_to_accesskit(layout);
            node.set_children(child_ids);
            nodes.push((my_id, node));
        }
    }
}

fn widget_to_accesskit(widget: &WidgetJson) -> Node {
    match widget {
        WidgetJson::Label { text, .. } => {
            let mut n = Node::new(Role::Label);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::Button { text, .. } => {
            let mut n = Node::new(Role::Button);
            n.set_label(text.as_str());
            n
        }
        WidgetJson::SmallButton { text } => {
            let mut n = Node::new(Role::Button);
            n.set_label(text.as_str());
            n
        }
        WidgetJson::Checkbox { text, checked, .. } => {
            let mut n = Node::new(Role::CheckBox);
            n.set_label(text.as_str());
            n.set_toggled(if *checked {
                accesskit::Toggled::True
            } else {
                accesskit::Toggled::False
            });
            n
        }
        WidgetJson::RadioValue { text, selected, .. } | WidgetJson::Radio { text, selected } => {
            let mut n = Node::new(Role::RadioButton);
            n.set_label(text.as_str());
            n.set_toggled(if *selected {
                accesskit::Toggled::True
            } else {
                accesskit::Toggled::False
            });
            n
        }
        WidgetJson::SelectableLabel { text, selected, .. }
        | WidgetJson::ToggleValue { text, selected } => {
            let mut n = Node::new(Role::Button);
            n.set_label(text.as_str());
            n.set_toggled(if *selected {
                accesskit::Toggled::True
            } else {
                accesskit::Toggled::False
            });
            n
        }
        WidgetJson::Hyperlink { text, url } => {
            let mut n = Node::new(Role::Link);
            n.set_label(text.as_str());
            n.set_url(url.as_str());
            n
        }
        WidgetJson::Heading { text } => {
            let mut n = Node::new(Role::Heading);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::Monospace { text } | WidgetJson::Code { text } => {
            let mut n = Node::new(Role::Code);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::Small { text } | WidgetJson::Weak { text } => {
            let mut n = Node::new(Role::Label);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::Strong { text } => {
            let mut n = Node::new(Role::Strong);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::ColoredLabel { text, .. } => {
            let mut n = Node::new(Role::Label);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::Separator => Node::new(Role::Splitter),
        WidgetJson::Spinner => {
            let mut n = Node::new(Role::ProgressIndicator);
            n.set_label("Loading");
            n
        }
        WidgetJson::TextEditSingleline { text, hint, .. } => {
            let mut n = Node::new(Role::TextInput);
            n.set_value(text.as_str());
            if let Some(h) = hint {
                n.set_placeholder(h.as_str());
            }
            n
        }
        WidgetJson::TextEditMultiline { text, hint, .. } => {
            let mut n = Node::new(Role::MultilineTextInput);
            n.set_value(text.as_str());
            if let Some(h) = hint {
                n.set_placeholder(h.as_str());
            }
            n
        }
        WidgetJson::CodeEditor { text, .. } => {
            let mut n = Node::new(Role::MultilineTextInput);
            n.set_value(text.as_str());
            n
        }
        WidgetJson::Slider {
            value, range, text, ..
        } => {
            let mut n = Node::new(Role::Slider);
            n.set_numeric_value(*value);
            n.set_min_numeric_value(range.min);
            n.set_max_numeric_value(range.max);
            if let Some(t) = text {
                n.set_label(t.as_str());
            }
            n
        }
        WidgetJson::SliderVertical {
            value, range, text, ..
        } => {
            let mut n = Node::new(Role::Slider);
            n.set_numeric_value(*value);
            n.set_min_numeric_value(range.min);
            n.set_max_numeric_value(range.max);
            n.set_orientation(accesskit::Orientation::Vertical);
            if let Some(t) = text {
                n.set_label(t.as_str());
            }
            n
        }
        WidgetJson::DragValue { value, range, .. } => {
            let mut n = Node::new(Role::SpinButton);
            n.set_numeric_value(*value);
            if let Some(r) = range {
                n.set_min_numeric_value(r.min);
                n.set_max_numeric_value(r.max);
            }
            n
        }
        WidgetJson::DragAngle { radians } | WidgetJson::DragAngleTau { radians } => {
            let mut n = Node::new(Role::SpinButton);
            n.set_numeric_value(*radians);
            n
        }
        WidgetJson::ProgressBar { progress, text, .. } => {
            let mut n = Node::new(Role::ProgressIndicator);
            n.set_numeric_value(*progress as f64 * 100.0);
            n.set_max_numeric_value(100.0);
            if let Some(t) = text {
                n.set_label(t.as_str());
            }
            n
        }
        WidgetJson::Image { uri, .. } => {
            let mut n = Node::new(Role::Image);
            n.set_label(uri.as_str());
            n
        }
        WidgetJson::Link { text } => {
            let mut n = Node::new(Role::Link);
            n.set_label(text.as_str());
            n
        }
        WidgetJson::ColorEditButtonSrgba { .. } | WidgetJson::ColorEditButtonHsva { .. } => {
            let mut n = Node::new(Role::ColorWell);
            n.set_label("Color picker");
            n
        }
    }
}

fn container_to_accesskit(container: &ContainerJson) -> Node {
    match container {
        ContainerJson::Window { title, .. } => {
            let mut n = Node::new(Role::Window);
            n.set_label(title.as_str());
            n
        }
        ContainerJson::LeftPanel { id, .. }
        | ContainerJson::RightPanel { id, .. }
        | ContainerJson::TopPanel { id, .. }
        | ContainerJson::BottomPanel { id, .. } => {
            let mut n = Node::new(Role::Pane);
            n.set_label(id.as_str());
            n
        }
        ContainerJson::CentralPanel => Node::new(Role::Main),
        ContainerJson::ScrollArea { .. } => Node::new(Role::ScrollView),
        ContainerJson::CollapsingHeader { text, .. } => {
            let mut n = Node::new(Role::Section);
            n.set_label(text.as_str());
            n
        }
        ContainerJson::Group => Node::new(Role::Group),
        ContainerJson::Frame { .. } => Node::new(Role::GenericContainer),
        ContainerJson::MenuBar => Node::new(Role::MenuBar),
        ContainerJson::Menu { title } => {
            let mut n = Node::new(Role::Menu);
            n.set_label(title.as_str());
            n
        }
        ContainerJson::Tooltip { text } => {
            let mut n = Node::new(Role::Tooltip);
            n.set_label(text.as_str());
            n
        }
        ContainerJson::Popup { id } => {
            let mut n = Node::new(Role::Dialog);
            n.set_label(id.as_str());
            n
        }
    }
}

fn layout_to_accesskit(layout: &LayoutJson) -> Node {
    match layout {
        LayoutJson::Horizontal { .. }
        | LayoutJson::Vertical { .. }
        | LayoutJson::HorizontalCentered
        | LayoutJson::VerticalCentered
        | LayoutJson::HorizontalJustified
        | LayoutJson::VerticalJustified
        | LayoutJson::HorizontalWrapped
        | LayoutJson::Columns { .. }
        | LayoutJson::Indent { .. }
        | LayoutJson::AddSpace { .. } => Node::new(Role::GenericContainer),
        LayoutJson::Grid { .. } => Node::new(Role::Grid),
    }
}

// ── Reverse: AccessKit → UiNode ─────────────────────────────

fn convert_accesskit_node(
    node_id: NodeId,
    node_map: &std::collections::BTreeMap<NodeId, &Node>,
) -> UiNode {
    let Some(node) = node_map.get(&node_id) else {
        return UiNode::Widget {
            widget: WidgetJson::Label {
                text: String::new(),
                wrap: false,
                color: None,
            },
        };
    };

    let children_ids = node.children();

    if children_ids.is_empty() {
        // Leaf node → Widget
        let widget = accesskit_to_widget(node);
        UiNode::Widget { widget }
    } else {
        // Container node
        let children: Vec<UiNode> = children_ids
            .iter()
            .map(|cid| convert_accesskit_node(*cid, node_map))
            .collect();

        let container = accesskit_to_container(node);
        UiNode::Container {
            container,
            children,
        }
    }
}

fn accesskit_to_widget(node: &Node) -> WidgetJson {
    let role = node.role();
    let label = node.label().unwrap_or("").to_string();
    let value = node.value().unwrap_or("").to_string();
    let text = if !value.is_empty() {
        value
    } else {
        label.clone()
    };

    match role {
        Role::Button | Role::DefaultButton => WidgetJson::Button {
            text,
            wrap: false,
            fill: None,
            stroke: None,
            selected: node
                .toggled()
                .is_some_and(|t| matches!(t, accesskit::Toggled::True)),
            frame: true,
            min_size: None,
        },
        Role::CheckBox | Role::MenuItemCheckBox => WidgetJson::Checkbox {
            text,
            checked: node
                .toggled()
                .is_some_and(|t| matches!(t, accesskit::Toggled::True)),
        },
        Role::RadioButton | Role::MenuItemRadio => WidgetJson::Radio {
            text,
            selected: node
                .toggled()
                .is_some_and(|t| matches!(t, accesskit::Toggled::True)),
        },
        Role::Switch => WidgetJson::ToggleValue {
            text,
            selected: node
                .toggled()
                .is_some_and(|t| matches!(t, accesskit::Toggled::True)),
        },
        Role::TextInput
        | Role::SearchInput
        | Role::EmailInput
        | Role::UrlInput
        | Role::PhoneNumberInput
        | Role::PasswordInput => WidgetJson::TextEditSingleline {
            text,
            hint: node.placeholder().map(|s| s.to_string()),
            interactive: !node.is_disabled(),
        },
        Role::MultilineTextInput => WidgetJson::TextEditMultiline {
            text,
            hint: node.placeholder().map(|s| s.to_string()),
            interactive: !node.is_disabled(),
        },
        Role::Slider => WidgetJson::Slider {
            value: node.numeric_value().unwrap_or(0.0),
            range: crate::serde_types::RangeJson {
                min: node.min_numeric_value().unwrap_or(0.0),
                max: node.max_numeric_value().unwrap_or(100.0),
            },
            step: None,
            text: if label.is_empty() { None } else { Some(label) },
            prefix: None,
            suffix: None,
            logarithmic: false,
            clamping: true,
            show_value: true,
        },
        Role::SpinButton | Role::NumberInput => WidgetJson::DragValue {
            value: node.numeric_value().unwrap_or(0.0),
            range: Some(crate::serde_types::RangeJson {
                min: node.min_numeric_value().unwrap_or(f64::MIN),
                max: node.max_numeric_value().unwrap_or(f64::MAX),
            }),
            speed: None,
            prefix: None,
            suffix: None,
            min_decimals: None,
            max_decimals: None,
        },
        Role::ProgressIndicator | Role::Meter => {
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let fraction = if max > 0.0 { (val / max) as f32 } else { 0.0 };
            WidgetJson::ProgressBar {
                progress: fraction.clamp(0.0, 1.0),
                text: if label.is_empty() { None } else { Some(label) },
                animate: false,
                fill: None,
                desired_width: None,
                corner_radius: None,
            }
        }
        Role::Link => WidgetJson::Hyperlink {
            text,
            url: node.url().unwrap_or("#").to_string(),
        },
        Role::Heading => WidgetJson::Heading { text },
        Role::Strong => WidgetJson::Strong { text },
        Role::Emphasis | Role::Mark => WidgetJson::Weak { text },
        Role::Code => WidgetJson::Code { text },
        Role::Image => WidgetJson::Image {
            uri: label,
            size: None,
            maintain_aspect_ratio: true,
            tint: None,
            corner_radius: None,
        },
        Role::Splitter => WidgetJson::Separator,
        Role::ColorWell => WidgetJson::ColorEditButtonSrgba {
            color: crate::serde_types::ColorJson {
                r: 128,
                g: 128,
                b: 128,
                a: 255,
            },
            alpha: true,
        },
        // Fallback: Label with text
        _ => WidgetJson::Label {
            text,
            wrap: false,
            color: None,
        },
    }
}

fn accesskit_to_container(node: &Node) -> ContainerJson {
    let role = node.role();
    let label = node.label().unwrap_or("").to_string();

    match role {
        Role::Window => ContainerJson::Window {
            title: label,
            default_pos: None,
            default_size: None,
            resizable: true,
            collapsible: true,
            scroll: false,
            title_bar: true,
        },
        Role::Pane | Role::Region => ContainerJson::LeftPanel {
            id: label,
            default_width: None,
            resizable: true,
            min_width: None,
            max_width: None,
        },
        Role::Main => ContainerJson::CentralPanel,
        Role::ScrollView | Role::ScrollBar => ContainerJson::ScrollArea {
            vertical: true,
            horizontal: false,
            max_height: None,
            max_width: None,
            auto_shrink: true,
            always_show_scroll: false,
        },
        Role::Section => ContainerJson::CollapsingHeader {
            text: label,
            default_open: true,
        },
        Role::Group | Role::Form => ContainerJson::Group,
        Role::GenericContainer | Role::Document => ContainerJson::Frame {
            fill: None,
            stroke: None,
            corner_radius: None,
            inner_margin: None,
            outer_margin: None,
        },
        Role::MenuBar => ContainerJson::MenuBar,
        Role::Menu | Role::MenuListPopup => ContainerJson::Menu { title: label },
        Role::Tooltip => ContainerJson::Tooltip { text: label },
        Role::Dialog | Role::AlertDialog => ContainerJson::Popup {
            id: if label.is_empty() {
                "dialog".to_string()
            } else {
                label
            },
        },
        Role::Grid | Role::Table | Role::TreeGrid | Role::ListGrid => ContainerJson::Group,
        // Fallback
        _ => ContainerJson::Group,
    }
}
