//! Bidirectional bridge: `TuiNode` ↔ AccessKit `TreeUpdate`.
//!
//! Converts ratatui shadow types (`TuiNode`, `WidgetJson`) into
//! AccessKit trees and vice versa. This enables feeding terminal UI
//! descriptions into the shared AccessKit IR for verification and
//! cross-frontend translation.

use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use crate::serde_types::{
    BlockJson, BordersJson, DirectionJson, TuiNode, WidgetJson,
};

/// Convert a `TuiNode` tree into an AccessKit `TreeUpdate`.
///
/// Assigns deterministic `NodeId`s via depth-first index. The root
/// gets `NodeId(0)`. Cell-based layout: 1 cell = 1 unit in AccessKit bounds.
#[tracing::instrument(skip(root_node))]
pub fn tui_node_to_tree_update(root_node: &TuiNode) -> TreeUpdate {
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

/// Convert an AccessKit `TreeUpdate` back into a `TuiNode` tree.
///
/// Walks from the root, mapping AccessKit Roles back to the closest
/// ratatui widget variant. Lossy for roles with no TuiNode equivalent
/// (falls back to `WidgetJson::Paragraph`).
#[tracing::instrument(skip(update))]
pub fn tree_update_to_tui_node(update: &TreeUpdate) -> Option<TuiNode> {
    let root_id = update.tree.as_ref()?.root;
    let node_map: std::collections::HashMap<NodeId, &Node> =
        update.nodes.iter().map(|(id, n)| (*id, n)).collect();

    Some(convert_accesskit_node(root_id, &node_map))
}

// ── Forward: TuiNode → AccessKit ────────────────────────────

fn convert_node(tui_node: &TuiNode, nodes: &mut Vec<(NodeId, Node)>, next_id: &mut u64) {
    let my_id = NodeId::from(*next_id);
    *next_id += 1;

    match tui_node {
        TuiNode::Widget { widget } => {
            let node = widget_to_accesskit(widget);
            nodes.push((my_id, node));
        }
        TuiNode::Layout {
            direction,
            children,
            ..
        } => {
            let mut child_ids = Vec::with_capacity(children.len());

            for child in children {
                let child_id = NodeId::from(*next_id);
                child_ids.push(child_id);
                convert_node(child, nodes, next_id);
            }

            let mut node = Node::new(Role::GenericContainer);
            // Encode direction as orientation
            match direction {
                DirectionJson::Horizontal => {
                    node.set_orientation(accesskit::Orientation::Horizontal);
                }
                DirectionJson::Vertical => {
                    node.set_orientation(accesskit::Orientation::Vertical);
                }
            }
            node.set_children(child_ids);
            nodes.push((my_id, node));
        }
    }
}

fn widget_to_accesskit(widget: &WidgetJson) -> Node {
    match widget {
        WidgetJson::Paragraph {
            text,
            block,
            ..
        } => {
            let mut n = Node::new(Role::Label);
            n.set_value(text.to_plain_string().as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::List {
            items, block, ..
        } => {
            let mut n = Node::new(Role::List);
            // Encode items as the value (comma-separated for round-trip)
            if !items.is_empty() {
                n.set_value(items.join("\n").as_str());
            }
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Block { block } => {
            let mut n = Node::new(Role::Group);
            apply_block_label(&mut n, Some(block));
            n
        }
        WidgetJson::Table {
            rows, block, ..
        } => {
            let mut n = Node::new(Role::Table);
            n.set_value(format!("{} rows", rows.len()).as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Gauge {
            ratio, label, block, ..
        } => {
            let mut n = Node::new(Role::ProgressIndicator);
            n.set_numeric_value(*ratio * 100.0);
            n.set_max_numeric_value(100.0);
            if let Some(l) = label {
                n.set_label(l.as_str());
            }
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::LineGauge {
            ratio, label, block, ..
        } => {
            let mut n = Node::new(Role::ProgressIndicator);
            n.set_numeric_value(*ratio * 100.0);
            n.set_max_numeric_value(100.0);
            if let Some(l) = label {
                n.set_label(l.as_str());
            }
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Tabs {
            titles,
            selected,
            block,
            ..
        } => {
            let mut n = Node::new(Role::TabList);
            if !titles.is_empty() {
                n.set_value(titles.join(", ").as_str());
            }
            if let Some(sel) = selected {
                n.set_numeric_value(*sel as f64);
            }
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Sparkline { data, block, .. } => {
            let mut n = Node::new(Role::Figure);
            n.set_value(format!("{} points", data.len()).as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::BarChart { data, block, .. } => {
            let mut n = Node::new(Role::Figure);
            n.set_value(format!("{} groups", data.len()).as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Chart {
            datasets, block, ..
        } => {
            let mut n = Node::new(Role::Figure);
            n.set_value(format!("{} datasets", datasets.len()).as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Scrollbar { orientation, .. } => {
            let mut n = Node::new(Role::ScrollBar);
            match orientation {
                crate::serde_types::ScrollbarOrientationJson::VerticalRight
                | crate::serde_types::ScrollbarOrientationJson::VerticalLeft => {
                    n.set_orientation(accesskit::Orientation::Vertical);
                }
                crate::serde_types::ScrollbarOrientationJson::HorizontalBottom
                | crate::serde_types::ScrollbarOrientationJson::HorizontalTop => {
                    n.set_orientation(accesskit::Orientation::Horizontal);
                }
            }
            n
        }
        WidgetJson::Clear => Node::new(Role::GenericContainer),
    }
}

fn apply_block_label(node: &mut Node, block: Option<&BlockJson>) {
    if let Some(b) = block {
        if let Some(ref title) = b.title {
            node.set_label(title.as_str());
        }
    }
}

// ── Reverse: AccessKit → TuiNode ────────────────────────────

fn convert_accesskit_node(
    node_id: NodeId,
    node_map: &std::collections::HashMap<NodeId, &Node>,
) -> TuiNode {
    let Some(node) = node_map.get(&node_id) else {
        return TuiNode::Widget {
            widget: Box::new(WidgetJson::Paragraph {
                text: String::new().into(),
                style: None,
                wrap: true,
                scroll: None,
                alignment: None,
                block: None,
            }),
        };
    };

    let children_ids = node.children();

    if children_ids.is_empty() {
        // Leaf → Widget
        let widget = accesskit_to_widget(node);
        TuiNode::Widget {
            widget: Box::new(widget),
        }
    } else {
        // Container → Layout with children
        let children: Vec<TuiNode> = children_ids
            .iter()
            .map(|cid| convert_accesskit_node(*cid, node_map))
            .collect();

        let direction = match node.orientation() {
            Some(accesskit::Orientation::Horizontal) => DirectionJson::Horizontal,
            _ => DirectionJson::Vertical,
        };

        TuiNode::Layout {
            direction,
            constraints: Vec::new(),
            children,
            margin: None,
        }
    }
}

fn accesskit_to_widget(node: &Node) -> WidgetJson {
    let role = node.role();
    let label = node.label().unwrap_or("").to_string();
    let value = node.value().unwrap_or("").to_string();
    let text_str = if !value.is_empty() {
        value.clone()
    } else {
        label.clone()
    };

    let block_from_label = |l: &str| -> Option<BlockJson> {
        if l.is_empty() {
            None
        } else {
            Some(BlockJson {
                title: Some(l.to_string()),
                borders: BordersJson::None,
                border_type: None,
                border_style: None,
                style: None,
                padding: None,
            })
        }
    };

    match role {
        Role::Label | Role::Paragraph | Role::TextRun => {
            WidgetJson::Paragraph {
                text: text_str.into(),
                style: None,
                wrap: true,
                scroll: None,
                alignment: None,
                block: block_from_label(&label),
            }
        }
        Role::Heading | Role::Strong | Role::Emphasis | Role::Code | Role::Mark => {
            WidgetJson::Paragraph {
                text: text_str.into(),
                style: None,
                wrap: true,
                scroll: None,
                alignment: None,
                block: block_from_label(&label),
            }
        }
        Role::List | Role::ListBox | Role::Feed | Role::DescriptionList => {
            let items = if value.is_empty() {
                vec![]
            } else {
                value.split('\n').map(|s| s.to_string()).collect()
            };
            WidgetJson::List {
                items,
                block: block_from_label(&label),
                style: None,
                highlight_style: None,
                highlight_symbol: None,
                state: None,
            }
        }
        Role::Group | Role::Section | Role::Region | Role::GenericContainer | Role::Form => {
            WidgetJson::Block {
                block: BlockJson {
                    title: if label.is_empty() {
                        None
                    } else {
                        Some(label)
                    },
                    borders: BordersJson::None,
                    border_type: None,
                    border_style: None,
                    style: None,
                    padding: None,
                },
            }
        }
        Role::Table | Role::Grid | Role::TreeGrid | Role::ListGrid => WidgetJson::Table {
            header: None,
            rows: Vec::new(),
            widths: Vec::new(),
            column_spacing: None,
            block: block_from_label(&label),
            highlight_style: None,
            highlight_symbol: None,
            state: None,
        },
        Role::ProgressIndicator | Role::Meter => {
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let ratio = if max > 0.0 { val / max } else { 0.0 };
            WidgetJson::Gauge {
                ratio: ratio.clamp(0.0, 1.0),
                label: if label.is_empty() { None } else { Some(label) },
                block: None,
                style: None,
                gauge_style: None,
            }
        }
        Role::TabList | Role::Tab => {
            let titles = if value.is_empty() {
                vec![]
            } else {
                value.split(", ").map(|s| s.to_string()).collect()
            };
            WidgetJson::Tabs {
                titles,
                selected: node.numeric_value().map(|v| v as usize),
                block: block_from_label(&label),
                style: None,
                highlight_style: None,
                divider: None,
            }
        }
        Role::Figure | Role::Image => WidgetJson::Sparkline {
            data: Vec::new(),
            block: block_from_label(&label),
            style: None,
            max: None,
            direction: None,
        },
        Role::ScrollBar => WidgetJson::Scrollbar {
            orientation: match node.orientation() {
                Some(accesskit::Orientation::Horizontal) => {
                    crate::serde_types::ScrollbarOrientationJson::HorizontalBottom
                }
                _ => crate::serde_types::ScrollbarOrientationJson::VerticalRight,
            },
            thumb_symbol: None,
            track_symbol: None,
            begin_symbol: None,
            end_symbol: None,
            style: None,
            thumb_style: None,
            track_style: None,
            state: None,
        },
        // Fallback: Paragraph with text
        _ => WidgetJson::Paragraph {
            text: text_str.into(),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: block_from_label(&label),
        },
    }
}
