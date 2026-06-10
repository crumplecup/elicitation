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
use elicit_ui::{
    NodeRenderedEvidence, RolePreserved, UiNodeBridge, UiRenderBackend, WcagNodeProofs,
};
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
        // Ratatui has no alpha channel; drop alpha and use RGB (best-effort).
        UiColor::Rgba { r, g, b, a: _ } => ColorJson::Rgb {
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
        // Ratatui has no justify mode; fall back to left alignment.
        elicit_ui::text::TextAlign::Justify => AlignmentJson::Left,
    }
}

fn text_widget(node: &Node, proofs: WcagNodeProofs) -> TuiNode {
    TuiNode::Widget {
        widget: Box::new(WidgetJson::Paragraph {
            text: ParagraphText::Plain(node_label(node)),
            style: None,
            wrap: true,
            scroll: None,
            alignment: None,
            block: None,
        }),
        proofs,
    }
}

/// Build a vertical layout.
///
/// Constraint selection per child:
/// - `TuiNode::StatusBar` → `Length{1}` (always exactly one row)
/// - `TuiNode::Layout` with `size_hint` set → `Min{size_hint}` (content-driven minimum)
/// - everything else → `Min{0}` (takes equal share of remaining space)
#[tracing::instrument(skip(children), fields(n_children = children.len()))]
fn vertical_layout(children: Vec<TuiNode>) -> TuiNode {
    let constraints = children
        .iter()
        .map(|c| match c {
            TuiNode::StatusBar { .. } => ConstraintJson::Length { value: 1 },
            TuiNode::Layout {
                size_hint: Some(h), ..
            } => ConstraintJson::Min { value: *h },
            _ => ConstraintJson::Min { value: 0 },
        })
        .collect::<Vec<_>>();
    tracing::trace!(?constraints, "vertical_layout constraints");
    TuiNode::Layout {
        direction: DirectionJson::Vertical,
        constraints,
        children,
        margin: None,
        size_hint: None,
    }
}

/// Build a horizontal layout distributing space equally among children.
#[tracing::instrument(skip(children), fields(n_children = children.len()))]
fn horizontal_layout(children: Vec<TuiNode>) -> TuiNode {
    let constraints = vec![ConstraintJson::Fill { value: 1 }; children.len().max(1)];
    TuiNode::Layout {
        direction: DirectionJson::Horizontal,
        constraints,
        children,
        margin: None,
        size_hint: None,
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

    fn verify_node(&self, node: &Node, proofs: &WcagNodeProofs) {
        #[cfg(debug_assertions)]
        {
            // SC 1.1.1 / 4.1.2: name_present — the AccessKit label must survive
            // to render time; catching it here means a regression between factory
            // validation and rendering is visible immediately.
            if proofs.name_present.is_some() {
                tracing::error!(
                    role = ?node.role(),
                    sc = "1.1.1/4.1.2",
                    "name_present proof held but node has no accessible label",
                );
                debug_assert!(
                    node.label().is_some(),
                    "name_present proof held but node {:?} has no accessible label (SC 1.1.1/4.1.2)",
                    node.role(),
                );
            }

            // SC 1.3.1: heading nodes must carry a heading level in the AccessKit tree.
            if proofs.heading_structure.is_some()
                && node.role() == Role::Heading
                && node.level().is_none()
            {
                tracing::error!(
                    sc = "1.3.1",
                    "heading_structure proof held but heading node has no level"
                );
                debug_assert!(
                    false,
                    "heading_structure proof held but heading node has no level (SC 1.3.1)"
                );
            }

            // SC 4.1.3: alert/status nodes that carry an error_identified proof must
            // have label text — the error description must reach the rendered node.
            if proofs.error_identified.is_some() {
                let role = node.role();
                if role == Role::Alert || role == Role::Status {
                    tracing::error!(
                        role = ?role,
                        sc = "4.1.3",
                        "error_identified proof held but node has no label",
                    );
                    debug_assert!(
                        node.label().is_some(),
                        "error_identified proof held but {:?} node has no label (SC 4.1.3)",
                        role,
                    );
                }
            }
        }
    }

    // ── Unknown / generic ─────────────────────────────────────────────────

    fn bridge_unknown(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<UnknownNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            if children.is_empty() {
                text_widget(node, proofs)
            } else {
                vertical_layout(children)
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_generic_container(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<GenericContainerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_pane(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<PaneNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    #[tracing::instrument(skip(self, children, proof, proofs), fields(n_children = children.len()))]
    fn bridge_window(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<WindowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        // Window always contains [Banner, content..., Status].
        // Banner and Status are single-row chrome; the content rows fill the rest.
        // Using Length{1} (not Min{1}) prevents ratatui from giving chrome an
        // equal share of the total height.
        let n = children.len();
        let constraints: Vec<ConstraintJson> = children
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    // Banner: always 1 row
                    ConstraintJson::Length { value: 1 }
                } else if i == n - 1
                    && matches!(c, TuiNode::StatusBar { .. } | TuiNode::Widget { .. })
                {
                    // Trailing status/widget: 1 row
                    ConstraintJson::Length { value: 1 }
                } else {
                    // Content: fill remaining space
                    ConstraintJson::Fill { value: 1 }
                }
            })
            .collect();
        tracing::debug!(?constraints, "bridge_window layout");
        let __w = TuiNode::Layout {
            direction: DirectionJson::Vertical,
            constraints,
            children,
            margin: None,
            size_hint: None,
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_document(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DocumentNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_root_web_area(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RootWebAreaNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_application(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ApplicationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_terminal(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TerminalNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Interactive widgets ───────────────────────────────────────────────

    fn bridge_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ButtonNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_link(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<LinkNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_check_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CheckBoxNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_radio_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RadioButtonNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_switch(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SwitchNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_color_well(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ColorWellNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DisclosureTriangleNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ComboBoxNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_list_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ListBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            // Render as a Paragraph with wrap: true instead of a List widget.
            // Ratatui's List truncates items wider than the column; Paragraph
            // wraps them so every character remains visible.
            let text = children
                .iter()
                .map(|c| match c {
                    TuiNode::Widget { widget, .. } => match widget.as_ref() {
                        WidgetJson::Paragraph { text, .. } => text.to_plain_string(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join("\n");
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(text),
                    style: None,
                    wrap: true,
                    scroll: None,
                    alignment: None,
                    block: lbl.map(titled_block),
                }),
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_slider(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SliderNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_spin_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SpinButtonNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_progress_indicator(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ProgressIndicatorNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_scroll_view(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ScrollViewNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_splitter(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SplitterNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Text input ───────────────────────────────────────────────────────

    fn bridge_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TextInputNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MultilineTextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let (mut result, _) = self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        );
        if let TuiNode::Widget { widget, .. } = &mut result
            && let WidgetJson::Paragraph { wrap, .. } = widget.as_mut()
        {
            *wrap = true;
        }
        (
            result,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Text display ─────────────────────────────────────────────────────

    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TextRunNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_paragraph(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ParagraphNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = {
            // Prefer the rich-text payload when present (per-span cursor
            // highlighting).  Fall back to the plain-label + whole-widget
            // selected style for compatibility.
            if let Some(rich_json) = node_json_rich_text(node) {
                let rich_pt = rich_text_to_paragraph_text(rich_json);
                // Propagate the text-level alignment to the paragraph wrapper so
                // that Paragraph::alignment() is applied in terminal_tools —
                // Text::alignment() alone does not center short lines within the
                // widget area.
                let outer_alignment = if let ParagraphText::Rich(ref t) = rich_pt {
                    t.alignment.as_ref().map(|a| format!("{a:?}"))
                } else {
                    None
                };
                TuiNode::Widget {
                    widget: Box::new(WidgetJson::Paragraph {
                        text: rich_pt,
                        style: None,
                        wrap: true,
                        scroll: None,
                        alignment: outer_alignment,
                        block: None,
                    }),
                    proofs,
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
                    proofs,
                }
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_label(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<LabelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_heading(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<HeadingNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_line_break(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<LineBreakNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_blockquote(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<BlockquoteNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_code(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CodeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_math(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MathNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_note(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<NoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            if children.is_empty() {
                text_widget(node, proofs)
            } else {
                vertical_layout(children)
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_term(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TermNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_definition(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DefinitionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Media ────────────────────────────────────────────────────────────

    fn bridge_image(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ImageNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_figure(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<FigureNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_figure_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<FigureCaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_canvas(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CanvasNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_video(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<VideoNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_audio(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<AudioNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Landmark sections ─────────────────────────────────────────────────

    #[tracing::instrument(skip(self, children, proof, proofs), fields(n_children = children.len()))]
    fn bridge_main(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MainNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_navigation(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<NavigationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    #[tracing::instrument(skip(self, children, proof, proofs), fields(n_children = children.len()))]
    fn bridge_banner(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<BannerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let mut __w = horizontal_layout(children);
        // Banner is always a single-row title bar; carry size_hint=1 so the
        // parent Window layout allocates Min{1} instead of an equal share.
        if let TuiNode::Layout {
            ref mut size_hint, ..
        } = __w
        {
            *size_hint = Some(1);
        }
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_content_info(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ContentInfoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_complementary(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ComplementaryNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_form(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<FormNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_search(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SearchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_region(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RegionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_section(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SectionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_section_header(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SectionHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_section_footer(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<SectionFooterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_article(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ArticleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    #[tracing::instrument(skip(self, children, proof, proofs), fields(
        orientation = ?_node.orientation(),
        numeric_value = ?_node.numeric_value(),
        n_children = children.len()
    ))]
    fn bridge_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<GroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let size_hint = _node.numeric_value().map(|v| v as u16);
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let mut layout = match _node.orientation() {
            Some(accesskit::Orientation::Horizontal) => horizontal_layout(children),
            _ => vertical_layout(children),
        };
        // Propagate size hint so the parent layout can allocate adequate space.
        if let TuiNode::Layout {
            size_hint: ref mut sh,
            ..
        } = layout
        {
            *sh = size_hint;
        }
        tracing::debug!(
            direction = ?match &layout { TuiNode::Layout { direction, .. } => direction, _ => &DirectionJson::Vertical },
            size_hint = ?size_hint,
            "bridge_group produced layout"
        );
        (
            layout,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_dialog(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DialogNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_details(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DetailsNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tooltip(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TooltipNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_alert(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<AlertNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_status(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<StatusNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_timer(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TimerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    fn bridge_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            // Render as a Paragraph with wrap: true instead of a List widget.
            // Ratatui's List truncates items wider than the column; Paragraph
            // wraps them so every character remains visible.
            let text = children
                .iter()
                .map(|c| match c {
                    TuiNode::Widget { widget, .. } => match widget.as_ref() {
                        WidgetJson::Paragraph { text, .. } => text.to_plain_string(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join("\n");
            let lbl = node.label().map(|s| s.to_string());
            TuiNode::Widget {
                widget: Box::new(WidgetJson::Paragraph {
                    text: ParagraphText::Plain(text),
                    style: None,
                    wrap: true,
                    scroll: None,
                    alignment: None,
                    block: lbl.map(titled_block),
                }),
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_list_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ListItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_description_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<DescriptionListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Tables ────────────────────────────────────────────────────────────

    fn bridge_table(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TableNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let rows: Vec<RowJson> = children
                .into_iter()
                .map(|child| match child {
                    TuiNode::Widget { widget, .. } => match *widget {
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_row(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_cell(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<CaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_row_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RowGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Trees ─────────────────────────────────────────────────────────────

    fn bridge_tree(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TreeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tree_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TreeItemNodeValid>,
        proofs: WcagNodeProofs,
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Tabs ─────────────────────────────────────────────────────────────

    fn bridge_tab(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TabNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tab_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TabListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let titles: Vec<String> = children
                .iter()
                .map(|c| match c {
                    TuiNode::Widget { widget, .. } => match widget.as_ref() {
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
                proofs,
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tab_panel(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<TabPanelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Menus ─────────────────────────────────────────────────────────────

    fn bridge_menu(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MenuNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_menu_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<MenuItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let __w = { text_widget(node, proofs) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_toolbar(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<ToolbarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { horizontal_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_radio_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(TuiNode, Established<RolePreserved>)>,
        proof: Established<RadioGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (TuiNode, Established<RolePreserved>) {
        let children: Vec<TuiNode> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { vertical_layout(children) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }
}
