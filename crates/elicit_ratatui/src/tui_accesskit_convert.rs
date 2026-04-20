//! Bidirectional bridge: `TuiNode` ↔ AccessKit `TreeUpdate`.
//!
//! Converts ratatui shadow types (`TuiNode`, `WidgetJson`) into
//! AccessKit trees and vice versa. This enables feeding terminal UI
//! descriptions into the shared AccessKit IR for verification and
//! cross-frontend translation.

use crate::serde_types::{
    BlockJson, BordersJson, ColorJson, DirectionJson, LineJson, ParagraphText, SpanJson, StyleJson,
    TextJson, TuiNode, WidgetJson,
};
use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use elicit_ui::ColorTheme;

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
        TuiNode::StatusBar { chips, theme } => {
            let mut child_ids = Vec::with_capacity(chips.len());
            for (key, action) in chips {
                let cid = NodeId::from(*next_id);
                *next_id += 1;
                let mut chip = Node::new(Role::Group);
                chip.set_label(key.as_str());
                chip.set_description(action.as_str());
                nodes.push((cid, chip));
                child_ids.push(cid);
            }
            let mut bar = Node::new(Role::Status);
            bar.set_class_name(theme.css_class());
            bar.set_children(child_ids);
            nodes.push((my_id, bar));
        }
    }
}

fn widget_to_accesskit(widget: &WidgetJson) -> Node {
    match widget {
        WidgetJson::Paragraph { text, block, .. } => {
            let mut n = Node::new(Role::Label);
            n.set_value(text.to_plain_string().as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::List { items, block, .. } => {
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
        WidgetJson::Table { rows, block, .. } => {
            let mut n = Node::new(Role::Table);
            n.set_value(format!("{} rows", rows.len()).as_str());
            apply_block_label(&mut n, block.as_ref());
            n
        }
        WidgetJson::Gauge {
            ratio,
            label,
            block,
            ..
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
            ratio,
            label,
            block,
            ..
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
    if let Some(b) = block
        && let Some(ref title) = b.title
    {
        node.set_label(title.as_str());
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

    // Role::Status → StatusBar (extract chips from Group children)
    if node.role() == Role::Status {
        let chips: Vec<(String, String)> = children_ids
            .iter()
            .filter_map(|cid| node_map.get(cid))
            .map(|child| {
                (
                    child.label().unwrap_or("").to_string(),
                    child.description().unwrap_or("").to_string(),
                )
            })
            .collect();
        let theme = node
            .class_name()
            .and_then(|cn| cn.parse::<ColorTheme>().ok())
            .unwrap_or_default();
        return TuiNode::StatusBar { chips, theme };
    }

    // Role::Figure with coordinate metadata → ASCII art ERD.
    if node.role() == Role::Figure {
        let desc = node.description().unwrap_or("");
        if desc.contains("w=") && desc.contains("h=") && !children_ids.is_empty() {
            let art = erd_ascii_art(node, children_ids, node_map);
            let label = node.label().unwrap_or("ERD");
            let block = if label.is_empty() {
                None
            } else {
                Some(BlockJson {
                    title: Some(label.to_string()),
                    borders: BordersJson::None,
                    border_type: None,
                    border_style: None,
                    style: None,
                    padding: None,
                })
            };
            return TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: art.into(),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block,
                }),
            };
        }
    }

    if children_ids.is_empty() {
        // Leaf → Widget
        let widget = accesskit_to_widget(node);
        TuiNode::Widget {
            widget: Box::new(widget),
        }
    } else {
        // Container → Layout with children.
        // If the last child is a StatusBar we inject [Min(0), Length(1)] constraints
        // so ratatui reserves exactly one line for it.
        let children: Vec<TuiNode> = children_ids
            .iter()
            .map(|cid| convert_accesskit_node(*cid, node_map))
            .collect();

        let direction = match node.orientation() {
            Some(accesskit::Orientation::Horizontal) => DirectionJson::Horizontal,
            _ => DirectionJson::Vertical,
        };

        let has_status_bar = matches!(children.last(), Some(TuiNode::StatusBar { .. }));
        let constraints = if has_status_bar {
            let mut c: Vec<crate::serde_types::ConstraintJson> = children[..children.len() - 1]
                .iter()
                .map(|_| crate::serde_types::ConstraintJson::Min { value: 0 })
                .collect();
            c.push(crate::serde_types::ConstraintJson::Length { value: 1 });
            c
        } else {
            // Equal-fill distribution: each child gets an equal share of the space.
            // Without explicit constraints, ratatui::Layout returns 0 chunks and
            // nothing renders.
            children
                .iter()
                .map(|_| crate::serde_types::ConstraintJson::Fill { value: 1 })
                .collect()
        };

        TuiNode::Layout {
            direction,
            constraints,
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
        Role::Label | Role::Paragraph | Role::TextRun => WidgetJson::Paragraph {
            text: text_str.into(),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: None,
        },
        Role::Heading | Role::Strong | Role::Emphasis | Role::Code | Role::Mark => {
            WidgetJson::Paragraph {
                text: text_str.into(),
                style: None,
                wrap: true,
                scroll: None,
                alignment: None,
                block: None,
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
                    title: if label.is_empty() { None } else { Some(label) },
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
        Role::MultilineTextInput | Role::TextInput => {
            // SQL editor: syntax-highlighted rich text.
            let is_sql = label.eq_ignore_ascii_case("sql editor");
            let para_text = if is_sql {
                sql_highlight_rich(&text_str, elicit_ui::palettes::mocha())
            } else {
                text_str.into()
            };
            WidgetJson::Paragraph {
                text: para_text,
                style: None,
                wrap: false,
                scroll: None,
                alignment: None,
                block: block_from_label(&label),
            }
        }
        // Fallback: Paragraph with text (label IS the content, so no block title)
        _ => WidgetJson::Paragraph {
            text: text_str.into(),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: None,
        },
    }
}

/// Render an ERD [`Role::Figure`] node as multi-column ASCII art.
///
/// Tables are laid out in a 2-column grid with box-drawing borders.
/// FK edges are listed as text at the bottom.
fn erd_ascii_art(
    _figure: &Node,
    children_ids: &[NodeId],
    node_map: &std::collections::HashMap<NodeId, &Node>,
) -> String {
    const BOX_W: usize = 32;
    const COLS_PER_ROW: usize = 2;

    let mut tables: Vec<Vec<String>> = Vec::new();
    let mut edge_lines: Vec<String> = Vec::new();

    for &child_id in children_ids {
        let Some(child) = node_map.get(&child_id) else {
            continue;
        };
        let child_desc = child.description().unwrap_or("");

        if child_desc.contains("x1=") {
            // FK edge
            edge_lines.push(format!(
                "  {} ({})",
                child.label().unwrap_or("edge"),
                child_desc
            ));
        } else if child_desc.contains("x=") {
            // Table box — collect lines for this table
            let name = child.label().unwrap_or("table");
            let mut box_lines: Vec<String> = Vec::new();
            // header
            let title = truncate_center(name, BOX_W - 2);
            box_lines.push(format!("┌{}┐", "─".repeat(BOX_W - 2)));
            box_lines.push(format!("│{:^width$}│", title, width = BOX_W - 2));
            box_lines.push(format!("├{}┤", "─".repeat(BOX_W - 2)));
            // columns
            for col_id in child.children().iter() {
                let Some(col) = node_map.get(col_id) else {
                    continue;
                };
                let col_text = truncate_pad(col.label().unwrap_or(""), BOX_W - 4);
                box_lines.push(format!("│ {} │", col_text));
            }
            box_lines.push(format!("└{}┘", "─".repeat(BOX_W - 2)));
            tables.push(box_lines);
        }
    }

    let mut out = String::new();
    // Render tables in a COLS_PER_ROW grid.
    for chunk in tables.chunks(COLS_PER_ROW) {
        let max_h = chunk.iter().map(|t| t.len()).max().unwrap_or(0);
        for row in 0..max_h {
            for (i, table) in chunk.iter().enumerate() {
                let line = table.get(row).map(String::as_str).unwrap_or("");
                let pad = " ".repeat(BOX_W.saturating_sub(line.chars().count()));
                if i + 1 < chunk.len() {
                    out.push_str(&format!("{}{}  ", line, pad));
                } else {
                    out.push_str(line);
                }
            }
            out.push('\n');
        }
        out.push('\n');
    }

    if !edge_lines.is_empty() {
        out.push_str("FK Relationships:\n");
        for e in &edge_lines {
            out.push_str(e);
            out.push('\n');
        }
    }

    out
}

fn truncate_pad(s: &str, width: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() >= width {
        chars[..width].iter().collect()
    } else {
        let padding = width - chars.len();
        format!("{}{}", s, " ".repeat(padding))
    }
}

fn truncate_center(s: &str, width: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() >= width {
        chars[..width].iter().collect()
    } else {
        s.to_string()
    }
}

// ── SQL syntax highlighting for ratatui Paragraph ────────────────────────────

/// Build a rich [`ParagraphText`] from a SQL string using palette-driven highlighting.
///
/// Each line in `sql` becomes a [`LineJson`] containing coloured [`SpanJson`] tokens.
fn sql_highlight_rich(sql: &str, palette: &elicit_ui::Palette) -> ParagraphText {
    use elicit_accesskit::sql::{SqlTokenKind, sql_tokens};
    use elicit_ui::SemanticRole;

    fn rgb(c: elicit_ui::SrgbColor) -> ColorJson {
        ColorJson::Rgb {
            r: (c.r * 255.0).round() as u8,
            g: (c.g * 255.0).round() as u8,
            b: (c.b * 255.0).round() as u8,
        }
    }

    let mut all_lines: Vec<LineJson> = Vec::new();
    for line in sql.lines() {
        let spans: Vec<SpanJson> = sql_tokens(line)
            .into_iter()
            .map(|token| {
                let role = match token.kind {
                    SqlTokenKind::Keyword => SemanticRole::Keyword,
                    SqlTokenKind::StringLiteral => SemanticRole::StringLit,
                    SqlTokenKind::Comment => SemanticRole::Comment,
                    SqlTokenKind::Number => SemanticRole::Number,
                    SqlTokenKind::Plain => SemanticRole::Text,
                };
                let color = rgb(palette.color(role));
                SpanJson {
                    content: token.text.to_string(),
                    style: Some(StyleJson {
                        fg: Some(color),
                        ..Default::default()
                    }),
                }
            })
            .collect();
        all_lines.push(LineJson {
            spans,
            style: None,
            alignment: None,
        });
    }
    ParagraphText::Rich(TextJson {
        lines: all_lines,
        style: None,
        alignment: None,
    })
}
