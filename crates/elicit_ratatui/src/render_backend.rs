//! Ratatui `UiNodeBridge` implementation.
//!
//! [`RatatuiBackend`] implements [`UiNodeBridge`] — one method per
//! [`accesskit::Role`] — producing [`TuiNode`] values for each tree node.
//! The blanket [`UiTreeRenderer`](elicit_ui::UiTreeRenderer) assembles the
//! full tree via DFS and returns the root [`TuiNode`] directly.
//!
//! Container roles produce [`TuiNode::Layout`]; leaf roles produce
//! [`TuiNode::Widget`].

use accesskit::{Node, NodeId, Role, Toggled};
use elicit_ui::node_roles::*;
use elicit_ui::text::{
    ParagraphText as UiParagraphText, TextLine, TextModifier, TextSpan, TextStyle, UiColor,
};
use elicit_ui::{RolePreserved, UiNodeBridge, UiRenderBackend};
use elicitation::Established;

use crate::serde_types::{
    AlignmentJson, ConstraintJson, DirectionJson, LineJson, ModifierJson, ParagraphText, RowJson,
    SpanJson, StyleJson, TextJson, TuiNode, WidgetJson,
};

// ── RatatuiBackend ────────────────────────────────────────────────────────────

/// Ratatui render backend for verified AccessKit trees.
///
/// Implements [`UiNodeBridge`] — one method per [`accesskit::Role`] — so the
/// blanket [`UiTreeRenderer`](elicit_ui::UiTreeRenderer) provides full-tree DFS
/// rendering for free.  Call `.render(tree)` (from `UiTreeRenderer`) to receive
/// the root [`TuiNode`] alongside statistics and the render proof.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_ratatui::RatatuiBackend;
/// use elicit_ui::UiRenderBackend;
///
/// let backend = RatatuiBackend::new();
/// assert_eq!(backend.backend_name(), "ratatui");
/// ```
#[derive(Default)]
pub struct RatatuiBackend;

impl RatatuiBackend {
    /// Create a new ratatui render backend.
    pub fn new() -> Self {
        Self
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn titled_block(title: String) -> crate::serde_types::BlockJson {
    crate::serde_types::BlockJson {
        title: Some(title),
        borders: crate::serde_types::BordersJson::All,
        border_type: None,
        style: None,
        border_style: None,
        padding: None,
    }
}

fn node_label(node: &Node) -> String {
    elicit_accesskit::node_label(node).to_string()
}

/// Extracts the `rich_text` serde_json::Value from an accesskit Node's custom
/// properties by checking if the node's NodeJson has the field set.
///
/// Since `accesskit::Node` doesn't carry our `rich_text` sidecar, we encode
/// it as raw JSON stored in the node's `class_name` property with a sentinel
/// prefix.  `display.rs` writes this; we read it back here.
fn node_json_rich_text(node: &Node) -> Option<serde_json::Value> {
    const PREFIX: &str = "__rich_text__:";
    let class = node.class_name()?;
    if !class.starts_with(PREFIX) {
        return None;
    }
    serde_json::from_str(&class[PREFIX.len()..]).ok()
}

/// Convert an `elicit_ui::ParagraphText` JSON value to the ratatui
/// `ParagraphText` used by `WidgetJson::Paragraph`.
fn rich_text_to_paragraph_text(raw: serde_json::Value) -> ParagraphText {
    let Ok(ui_para) = serde_json::from_value::<UiParagraphText>(raw) else {
        return ParagraphText::Plain(String::new());
    };
    match ui_para {
        UiParagraphText::Plain(s) => ParagraphText::Plain(s),
        UiParagraphText::Rich(rich) => {
            let lines = rich.lines.into_iter().map(ui_line_to_json).collect();
            let style = rich.style.as_ref().map(ui_style_to_json);
            let alignment = rich.alignment.map(ui_align_to_json);
            ParagraphText::Rich(TextJson {
                lines,
                style,
                alignment,
            })
        }
    }
}

fn ui_line_to_json(line: TextLine) -> LineJson {
    LineJson {
        spans: line.spans.into_iter().map(ui_span_to_json).collect(),
        style: line.style.as_ref().map(ui_style_to_json),
        alignment: line.alignment.map(ui_align_to_json),
    }
}

fn ui_span_to_json(span: TextSpan) -> SpanJson {
    SpanJson {
        content: span.content,
        style: span.style.as_ref().map(ui_style_to_json),
    }
}

fn ui_style_to_json(style: &TextStyle) -> StyleJson {
    StyleJson {
        fg: style.fg.as_ref().map(ui_color_to_json),
        bg: style.bg.as_ref().map(ui_color_to_json),
        modifiers: style.modifiers.iter().map(ui_modifier_to_json).collect(),
    }
}

fn ui_color_to_json(color: &UiColor) -> crate::serde_types::ColorJson {
    use crate::serde_types::ColorJson;
    match color {
        UiColor::Reset => ColorJson::Reset,
        UiColor::Black => ColorJson::Black,
        UiColor::Red => ColorJson::Red,
        UiColor::Green => ColorJson::Green,
        UiColor::Yellow => ColorJson::Yellow,
        UiColor::Blue => ColorJson::Blue,
        UiColor::Magenta => ColorJson::Magenta,
        UiColor::Cyan => ColorJson::Cyan,
        UiColor::White => ColorJson::White,
        UiColor::DarkGray => ColorJson::DarkGray,
        UiColor::LightRed => ColorJson::LightRed,
        UiColor::LightGreen => ColorJson::LightGreen,
        UiColor::LightYellow => ColorJson::LightYellow,
        UiColor::LightBlue => ColorJson::LightBlue,
        UiColor::LightMagenta => ColorJson::LightMagenta,
        UiColor::LightCyan => ColorJson::LightCyan,
        UiColor::Gray => ColorJson::Gray,
        UiColor::Rgb { r, g, b } => ColorJson::Rgb {
            r: *r,
            g: *g,
            b: *b,
        },
        UiColor::Indexed { index } => ColorJson::Indexed { index: *index },
    }
}

fn ui_modifier_to_json(m: &TextModifier) -> ModifierJson {
    match m {
        TextModifier::Bold => ModifierJson::Bold,
        TextModifier::Dim => ModifierJson::Dim,
        TextModifier::Italic => ModifierJson::Italic,
        TextModifier::Underlined => ModifierJson::Underlined,
        TextModifier::SlowBlink => ModifierJson::SlowBlink,
        TextModifier::RapidBlink => ModifierJson::RapidBlink,
        TextModifier::Reversed => ModifierJson::Reversed,
        TextModifier::Hidden => ModifierJson::Hidden,
        TextModifier::CrossedOut => ModifierJson::CrossedOut,
    }
}

fn ui_align_to_json(a: elicit_ui::text::TextAlign) -> AlignmentJson {
    match a {
        elicit_ui::text::TextAlign::Left => AlignmentJson::Left,
        elicit_ui::text::TextAlign::Center => AlignmentJson::Center,
        elicit_ui::text::TextAlign::Right => AlignmentJson::Right,
    }
}

fn text_widget(node: &Node) -> TuiNode {
    TuiNode::Widget {
        widget: Box::new(WidgetJson::Paragraph {
            text: ParagraphText::Plain(node_label(node)),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: None,
        }),
    }
}

fn vertical_layout(children: Vec<TuiNode>) -> TuiNode {
    let constraints = vec![ConstraintJson::Min { value: 0 }; children.len().max(1)];
    TuiNode::Layout {
        direction: DirectionJson::Vertical,
        constraints,
        children,
        margin: None,
    }
}

fn horizontal_layout(children: Vec<TuiNode>) -> TuiNode {
    let constraints = vec![ConstraintJson::Fill { value: 1 }; children.len().max(1)];
    TuiNode::Layout {
        direction: DirectionJson::Horizontal,
        constraints,
        children,
        margin: None,
    }
}

// ── UiRenderBackend ───────────────────────────────────────────────────────────

impl UiRenderBackend for RatatuiBackend {
    fn backend_name(&self) -> &'static str {
        "ratatui"
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }
}

// ── UiNodeBridge ─────────────────────────────────────────────────────────────

impl UiNodeBridge for RatatuiBackend {
    type Widget = TuiNode;

    // ── Unknown / generic ─────────────────────────────────────────────────

    fn bridge_unknown(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<UnknownNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            if children.is_empty() {
                text_widget(node)
            } else {
                vertical_layout(children)
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_generic_container(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<GenericContainerNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_pane(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<PaneNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_window(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<WindowNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_document(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DocumentNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_root_web_area(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RootWebAreaNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_application(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ApplicationNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_terminal(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TerminalNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Interactive widgets ───────────────────────────────────────────────

    fn bridge_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ButtonNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let text = format!("[ {lbl} ]");
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(text),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: Some("Center".to_string()),
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_link(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<LinkNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let url = node.url().unwrap_or("#");
            let text = format!("{lbl} ({url})");
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(text),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_check_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CheckBoxNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let mark = if node.toggled() == Some(Toggled::True) {
                "[x]"
            } else {
                "[ ]"
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("{mark} {lbl}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_radio_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RadioButtonNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let mark = if node.toggled() == Some(Toggled::True) {
                "(•)"
            } else {
                "( )"
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("{mark} {lbl}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_switch(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SwitchNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let state = if node.toggled() == Some(Toggled::True) {
                "ON"
            } else {
                "OFF"
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("{lbl}: {state}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_color_well(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ColorWellNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let val = node.value().unwrap_or("#000000");
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("Color: {val}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DisclosureTriangleNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let arrow = if node.toggled() == Some(Toggled::True) {
                "▼"
            } else {
                "▶"
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("{arrow} {lbl}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ComboBoxNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let val = node_label(node);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("▼ {val}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_list_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ListBoxNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let items: Vec<String> = children
                .iter()
                .map(|c| match c {
                    TuiNode::Widget { widget } => match widget.as_ref() {
                        WidgetJson::Paragraph { text, .. } => text.to_plain_string(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                })
                .collect();
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::List {
                    items,
                    block: lbl.map(titled_block),
                    style: None,
                    highlight_style: None,
                    highlight_symbol: None,
                    state: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_slider(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SliderNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let ratio = if max > 0.0 {
                (val / max).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Gauge {
                    ratio,
                    label: lbl,
                    block: None,
                    style: None,
                    gauge_style: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_spin_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SpinButtonNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let val = node.numeric_value().unwrap_or(0.0);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("◂ {val} ▸")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: Some("Center".to_string()),
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_progress_indicator(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ProgressIndicatorNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let ratio = if max > 0.0 {
                (val / max).clamp(0.0, 1.0)
            } else {
                0.0
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Gauge {
                    ratio,
                    label: None,
                    block: None,
                    style: None,
                    gauge_style: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_scroll_view(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ScrollViewNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_splitter(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SplitterNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain("─".repeat(40)),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Text input ───────────────────────────────────────────────────────

    fn bridge_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TextInputNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let val = node.value().unwrap_or("");
            let placeholder = node.placeholder().unwrap_or("...");
            let display = if val.is_empty() { placeholder } else { val };
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(display.to_string()),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: lbl.map(titled_block),
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MultilineTextInputNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let (mut result, _) = self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
        );
        if let TuiNode::Widget { widget } = &mut result
            && let WidgetJson::Paragraph { wrap, .. } = widget.as_mut()
        {
            *wrap = true;
        }
        (result, Established::<RolePreserved>::prove(&proof))
    }

    // ── Text display ─────────────────────────────────────────────────────

    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TextRunNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_paragraph(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ParagraphNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            // Prefer the rich-text payload when present (per-span cursor
            // highlighting).  Fall back to the plain-label + whole-widget
            // selected style for compatibility.
            if let Some(rich_json) = node_json_rich_text(node) {
                TuiNode::Widget {
                    widget: Box::new(WidgetJson::Paragraph {
                        text: rich_text_to_paragraph_text(rich_json),
                        style: None,
                        wrap: true,
                        scroll: None,
                        alignment: None,
                        block: None,
                    }),
                }
            } else {
                let style = if node.is_selected().unwrap_or(false) {
                    Some(StyleJson {
                        fg: None,
                        bg: None,
                        modifiers: vec![ModifierJson::Reversed, ModifierJson::Bold],
                    })
                } else {
                    None
                };
                TuiNode::Widget {
                    widget: Box::new(WidgetJson::Paragraph {
                        text: ParagraphText::Plain(node_label(node)),
                        style,
                        wrap: true,
                        scroll: None,
                        alignment: None,
                        block: None,
                    }),
                }
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_label(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<LabelNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_heading(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<HeadingNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let level = node.level().unwrap_or(2);
            let prefix = "#".repeat(level);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("{prefix} {text}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_line_break(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<LineBreakNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(String::new()),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_blockquote(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<BlockquoteNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("│ {text}")),
                    style: None,
                    wrap: true,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_code(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CodeNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_math(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MathNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_note(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<NoteNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            if children.is_empty() {
                text_widget(node)
            } else {
                vertical_layout(children)
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_term(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TermNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_definition(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DefinitionNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Media ────────────────────────────────────────────────────────────

    fn bridge_image(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ImageNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let alt = node_label(node);
            let text = if alt.is_empty() {
                "[image]".to_string()
            } else {
                format!("[image: {alt}]")
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(text),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_figure(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<FigureNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_figure_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<FigureCaptionNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_canvas(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CanvasNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("[canvas: {lbl}]")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_video(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<VideoNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("[video: {lbl}]")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_audio(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<AudioNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("[audio: {lbl}]")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Landmark sections ─────────────────────────────────────────────────

    fn bridge_main(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MainNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_navigation(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<NavigationNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_banner(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<BannerNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_content_info(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ContentInfoNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_complementary(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ComplementaryNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_form(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<FormNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_search(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SearchNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_region(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RegionNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_section(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SectionNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_section_header(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SectionHeaderNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_section_footer(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SectionFooterNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_article(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ArticleNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<GroupNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_dialog(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DialogNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_details(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DetailsNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tooltip(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TooltipNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_alert(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<AlertNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("⚠ {lbl}")),
                    style: None,
                    wrap: true,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_status(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<StatusNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_timer(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TimerNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    fn bridge_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ListNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let items: Vec<String> = children
                .iter()
                .map(|c| match c {
                    TuiNode::Widget { widget } => match widget.as_ref() {
                        WidgetJson::Paragraph { text, .. } => text.to_plain_string(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                })
                .collect();
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::List {
                    items,
                    block: lbl.map(titled_block),
                    style: None,
                    highlight_style: None,
                    highlight_symbol: None,
                    state: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_list_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ListItemNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_description_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DescriptionListNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Tables ────────────────────────────────────────────────────────────

    fn bridge_table(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TableNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let rows: Vec<RowJson> = children
                .into_iter()
                .map(|child| match child {
                    TuiNode::Widget { widget } => match *widget {
                        WidgetJson::Paragraph { text, .. } => RowJson {
                            cells: vec![crate::serde_types::CellJson {
                                content: text.to_plain_string(),
                                style: None,
                            }],
                            style: None,
                            height: None,
                        },
                        _ => RowJson {
                            cells: vec![],
                            style: None,
                            height: None,
                        },
                    },
                    _ => RowJson {
                        cells: vec![],
                        style: None,
                        height: None,
                    },
                })
                .collect();
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Table {
                    header: None,
                    rows,
                    widths: vec![ConstraintJson::Fill { value: 1 }],
                    column_spacing: None,
                    block: lbl.map(titled_block),
                    highlight_style: None,
                    highlight_symbol: None,
                    state: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_row(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RowNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_cell(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CellNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CaptionNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_row_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RowGroupNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Trees ─────────────────────────────────────────────────────────────

    fn bridge_tree(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TreeNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tree_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TreeItemNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            let lbl = node_label(node);
            let prefix = if node.is_selected().unwrap_or(false) {
                "▶ "
            } else {
                "  "
            };
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(format!("{prefix}{lbl}")),
                    style: None,
                    wrap: false,
                    scroll: None,
                    alignment: None,
                    block: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Tabs ─────────────────────────────────────────────────────────────

    fn bridge_tab(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TabNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tab_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TabListNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let titles: Vec<String> = children
                .iter()
                .map(|c| match c {
                    TuiNode::Widget { widget } => match widget.as_ref() {
                        WidgetJson::Paragraph { text, .. } => text.to_plain_string(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                })
                .collect();
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Tabs {
                    titles,
                    selected: None,
                    block: None,
                    style: None,
                    highlight_style: None,
                    divider: None,
                }),
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tab_panel(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TabPanelNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Menus ─────────────────────────────────────────────────────────────

    fn bridge_menu(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MenuNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_menu_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MenuItemNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_toolbar(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ToolbarNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_radio_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RadioGroupNodeValid>,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (__w, Established::<RolePreserved>::prove(&proof))
    }
}
