//! AccessKit bridge for egui — bidirectional conversion between
//! AccessKit trees and egui widgets.
//!
//! Provides:
//! - [`EguiBackend`] — [`UiNodeBridge`] + [`UiRenderBackend`] for egui
//! - [`render_tree`] — render an AccessKit tree directly into an `egui::Ui`
//! - [`bounds_to_size`] — extract pixel size from AccessKit node bounds

use accesskit::{Node, NodeId, Rect, Role, Toggled};
use elicit_ui::node_roles::*;
use elicit_ui::{RolePreserved, UiNodeBridge, UiRenderBackend};
use elicitation::Established;
use std::collections::HashMap;

// ── EguiBackend ───────────────────────────────────────────────────────────────

/// egui render backend for verified AccessKit trees.
///
/// Implements [`UiNodeBridge`] — one method per [`accesskit::Role`] —
/// producing `Box<dyn FnOnce(&mut egui::Ui)>` closures assembled by the
/// blanket [`UiTreeRenderer`](elicit_ui::UiTreeRenderer) DFS.
///
/// The returned root closure can be called inside any egui frame to execute
/// the full widget tree.  Use [`render_tree`] for direct immediate-mode
/// rendering when you already have a `&mut egui::Ui`.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_egui::EguiBackend;
/// use elicit_ui::UiRenderBackend;
/// let backend = EguiBackend::new();
/// assert_eq!(backend.backend_name(), "egui");
/// ```
#[derive(Default)]
pub struct EguiBackend;

impl EguiBackend {
    /// Create a new egui render backend.
    pub fn new() -> Self {
        Self
    }
}

// ── UiRenderBackend ───────────────────────────────────────────────────────────

impl UiRenderBackend for EguiBackend {
    fn backend_name(&self) -> &'static str {
        "egui"
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }
}

// ── UiNodeBridge ─────────────────────────────────────────────────────────────

impl UiNodeBridge for EguiBackend {
    type Widget = Box<dyn FnOnce(&mut egui::Ui)>;

    // ── Unknown / generic ─────────────────────────────────────────────────

    fn bridge_unknown(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<UnknownNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                if children.is_empty() {
                    ui.label(&text);
                } else {
                    ui.group(|ui| {
                        for c in children {
                            c(ui);
                        }
                    });
                }
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_generic_container(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GenericContainerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_pane(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<PaneNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_window(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<WindowNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_document(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocumentNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_root_web_area(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RootWebAreaNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_application(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ApplicationNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_terminal(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TerminalNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.monospace(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Interactive widgets ───────────────────────────────────────────────

    fn bridge_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::Button::new(&text));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_link(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LinkNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let url = node.url().unwrap_or("#").to_string();
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Hyperlink::from_label_and_url(&text, &url));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_check_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CheckBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut checked = matches!(node.toggled(), Some(Toggled::True));
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::Checkbox::new(&mut checked, &text));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_radio_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RadioButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let selected = matches!(node.toggled(), Some(Toggled::True));
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::RadioButton::new(selected, &text));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_switch(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SwitchNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut on = matches!(node.toggled(), Some(Toggled::True));
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    ui.add_enabled(!disabled, egui::Checkbox::new(&mut on, &text));
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_color_well(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ColorWellNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::Button::new(format!("🎨 {text}")));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DisclosureTriangleNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let open = matches!(node.toggled(), Some(Toggled::True));
            let arrow = if open { "▼" } else { "▶" };
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("{arrow} {text}"));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ComboBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let val = node.value().unwrap_or("").to_string();
            Box::new(move |ui: &mut egui::Ui| {
                egui::ComboBox::from_label(&text)
                    .selected_text(&val)
                    .show_ui(ui, |_ui| {});
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_list_box(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_slider(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SliderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            Box::new(move |ui: &mut egui::Ui| {
                let mut slider = egui::Slider::new(&mut val, min..=max);
                if !text.is_empty() {
                    slider = slider.text(&text);
                }
                ui.add_enabled(!disabled, slider);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_spin_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SpinButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let disabled = node.is_disabled();
            let mut val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(f64::MIN);
            let max = node.max_numeric_value().unwrap_or(f64::MAX);
            let step = node.numeric_value_step().unwrap_or(1.0);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(
                    !disabled,
                    egui::DragValue::new(&mut val).range(min..=max).speed(step),
                );
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_progress_indicator(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ProgressIndicatorNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let fraction = if max > 0.0 { (val / max) as f32 } else { 0.0 };
            Box::new(move |ui: &mut egui::Ui| {
                let mut pb = egui::ProgressBar::new(fraction.clamp(0.0, 1.0));
                if !text.is_empty() {
                    pb = pb.text(&text);
                }
                ui.add(pb);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_scroll_view(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ScrollViewNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_splitter(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SplitterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            Box::new(|ui: &mut egui::Ui| {
                ui.separator();
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Text input ───────────────────────────────────────────────────────

    fn bridge_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TextInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let mut buf = node.value().unwrap_or("").to_string();
            let hint = node.placeholder().map(|s| s.to_string());
            let readonly = node.is_read_only() || node.is_disabled();
            Box::new(move |ui: &mut egui::Ui| {
                let mut te = egui::TextEdit::singleline(&mut buf);
                if let Some(h) = &hint {
                    te = te.hint_text(h.as_str());
                }
                if readonly {
                    te = te.interactive(false);
                }
                ui.add(te);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MultilineTextInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let mut buf = node.value().unwrap_or("").to_string();
            let hint = node.placeholder().map(|s| s.to_string());
            let readonly = node.is_read_only() || node.is_disabled();
            Box::new(move |ui: &mut egui::Ui| {
                let mut te = egui::TextEdit::multiline(&mut buf);
                if let Some(h) = &hint {
                    te = te.hint_text(h.as_str());
                }
                if readonly {
                    te = te.interactive(false);
                }
                ui.add(te);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NumberInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_spin_button(
            node,
            id,
            children,
            Established::<SpinButtonNodeValid>::prove(&proof),
        )
    }

    // ── Text display ─────────────────────────────────────────────────────

    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TextRunNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_paragraph(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ParagraphNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_label(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LabelNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_heading(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<HeadingNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let size = heading_size(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(&text).strong().size(size),
                ));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_line_break(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LineBreakNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            Box::new(|ui: &mut egui::Ui| {
                ui.end_row();
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_blockquote(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<BlockquoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(format!("│ {text}")).italics(),
                ));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_code(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CodeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).monospace()));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_math(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MathNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_note(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w: Self::Widget = {
            if children.is_empty() {
                let text = node_label(node);
                Box::new(move |ui: &mut egui::Ui| {
                    ui.label(&text);
                })
            } else {
                Box::new(move |ui: &mut egui::Ui| {
                    ui.group(|ui| {
                        for c in children {
                            c(ui);
                        }
                    });
                })
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_term(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TermNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).strong()));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_definition(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DefinitionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Media ────────────────────────────────────────────────────────────

    fn bridge_image(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ImageNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let alt = node_label(node);
            let text = if alt.is_empty() {
                "🖼 [image]".to_string()
            } else {
                format!("🖼 {alt}")
            };
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_figure(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FigureNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_figure_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FigureCaptionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).italics()));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_canvas(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CanvasNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("[canvas: {text}]"));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_video(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<VideoNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("[video: {text}]"));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_audio(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<AudioNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("[audio: {text}]"));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Landmark sections ─────────────────────────────────────────────────

    fn bridge_main(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MainNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_navigation(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NavigationNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_banner(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<BannerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_content_info(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ContentInfoNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_complementary(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ComplementaryNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_form(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FormNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_search(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SearchNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_region(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RegionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_section(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SectionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_section_header(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SectionHeaderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_section_footer(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SectionFooterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_article(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ArticleNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GroupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_dialog(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DialogNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_details(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DetailsNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tooltip(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TooltipNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_alert(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<AlertNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(&text).color(egui::Color32::YELLOW),
                ));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_status(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<StatusNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_timer(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TimerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    fn bridge_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_list_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListItemNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("• {text}"));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_description_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DescriptionListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Tables ────────────────────────────────────────────────────────────

    fn bridge_table(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TableNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let role_str = format!("{:?}", node.role());
            Box::new(move |ui: &mut egui::Ui| {
                egui::Grid::new(format!("grid_{role_str}"))
                    .striped(true)
                    .show(ui, |ui| {
                        for c in children {
                            c(ui);
                        }
                    });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_row(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RowNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                for c in children {
                    c(ui);
                }
                ui.end_row();
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_cell(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CellNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CaptionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).italics()));
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_row_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RowGroupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Trees ─────────────────────────────────────────────────────────────

    fn bridge_tree(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TreeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tree_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TreeItemNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w: Self::Widget = {
            if children.is_empty() {
                let text = node_label(node);
                let selected = node.is_selected().unwrap_or(false);
                Box::new(move |ui: &mut egui::Ui| {
                    if selected {
                        let hl = ui.visuals().selection.bg_fill;
                        let fg = ui.visuals().selection.stroke.color;
                        ui.painter()
                            .rect_filled(ui.available_rect_before_wrap(), 0.0, hl);
                        let _ = ui.colored_label(fg, &text);
                    } else {
                        ui.label(&text);
                    }
                })
            } else {
                Box::new(move |ui: &mut egui::Ui| {
                    ui.group(|ui| {
                        for c in children {
                            c(ui);
                        }
                    });
                })
            }
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Tabs ─────────────────────────────────────────────────────────────

    fn bridge_tab(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TabNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                let _ = ui.selectable_label(false, &text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tab_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TabListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_tab_panel(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TabPanelNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    // ── Menus ─────────────────────────────────────────────────────────────

    fn bridge_menu(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_menu_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuItemNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                let _ = ui.selectable_label(false, &text);
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_toolbar(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ToolbarNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.horizontal(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }

    fn bridge_radio_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RadioGroupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let children: Vec<Self::Widget> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            Box::new(move |ui: &mut egui::Ui| {
                ui.vertical(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        };
        (__w, Established::<RolePreserved>::prove(&proof))
    }
}

// ── Public helpers ────────────────────────────────────────────────────────────

/// Render a verified AccessKit tree into an `egui::Ui`.
///
/// Walks from `root` through the node map, creating egui widgets
/// for each node based on its role. Container nodes create nested
/// layouts; leaf nodes create widgets.
///
/// Returns [`elicit_ui::RenderStats`] and the [`NodeId`]s of any
/// [`Role::Button`] / [`Role::DefaultButton`] nodes clicked this frame.
#[tracing::instrument(skip(ui, nodes), fields(root = ?root))]
pub fn render_tree(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    root: NodeId,
) -> (elicit_ui::RenderStats, Vec<NodeId>) {
    let mut stats = elicit_ui::RenderStats::default();
    let mut clicked_nodes: Vec<NodeId> = Vec::new();
    render_node_recursive(ui, nodes, root, &mut stats, &mut clicked_nodes);
    tracing::debug!(
        visited = stats.nodes_visited,
        widgets = stats.widgets_rendered,
        containers = stats.containers_rendered,
        skipped = stats.nodes_skipped,
        "Render pass complete"
    );
    (stats, clicked_nodes)
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

// ── Internal rendering helpers ────────────────────────────────────────────────

fn node_label(node: &Node) -> String {
    node.label().or(node.value()).unwrap_or("").to_string()
}

fn heading_size(node: &Node) -> f32 {
    match node.level() {
        Some(1) => 28.0,
        Some(2) => 22.0,
        Some(3) => 18.0,
        Some(4) => 16.0,
        Some(5) => 14.0,
        _ => 12.0,
    }
}

fn render_node_recursive(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node_id: NodeId,
    stats: &mut elicit_ui::RenderStats,
    clicked_nodes: &mut Vec<NodeId>,
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
        Role::Window
        | Role::Pane
        | Role::Form
        | Role::Group
        | Role::Section
        | Role::Region
        | Role::Main
        | Role::GenericContainer
        | Role::Document => {
            render_container(ui, nodes, node, stats, clicked_nodes);
        }
        Role::Button | Role::DefaultButton => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let resp = ui.add_enabled(!disabled, egui::Button::new(&text));
            if resp.clicked() {
                clicked_nodes.push(node_id);
            }
            stats.widgets_rendered += 1;
        }
        Role::CheckBox | Role::MenuItemCheckBox => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut checked = matches!(node.toggled(), Some(Toggled::True));
            ui.add_enabled(!disabled, egui::Checkbox::new(&mut checked, &text));
            stats.widgets_rendered += 1;
        }
        Role::RadioButton | Role::MenuItemRadio => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let selected = matches!(node.toggled(), Some(Toggled::True));
            ui.add_enabled(!disabled, egui::RadioButton::new(selected, &text));
            stats.widgets_rendered += 1;
        }
        Role::Switch => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut on = matches!(node.toggled(), Some(Toggled::True));
            ui.horizontal(|ui| {
                ui.add_enabled(!disabled, egui::Checkbox::new(&mut on, &text));
            });
            stats.widgets_rendered += 1;
        }
        Role::TextInput
        | Role::SearchInput
        | Role::EmailInput
        | Role::UrlInput
        | Role::PhoneNumberInput
        | Role::PasswordInput => {
            let mut buf = node.value().unwrap_or("").to_string();
            let mut te = egui::TextEdit::singleline(&mut buf);
            if let Some(hint) = node.placeholder() {
                te = te.hint_text(hint);
            }
            if node.is_read_only() || node.is_disabled() {
                te = te.interactive(false);
            }
            ui.add(te);
            stats.widgets_rendered += 1;
        }
        Role::MultilineTextInput => {
            let mut buf = node.value().unwrap_or("").to_string();
            let mut te = egui::TextEdit::multiline(&mut buf);
            if let Some(hint) = node.placeholder() {
                te = te.hint_text(hint);
            }
            if node.is_read_only() || node.is_disabled() {
                te = te.interactive(false);
            }
            ui.add(te);
            stats.widgets_rendered += 1;
        }
        Role::NumberInput | Role::SpinButton => {
            let mut val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(f64::MIN);
            let max = node.max_numeric_value().unwrap_or(f64::MAX);
            let step = node.numeric_value_step().unwrap_or(1.0);
            ui.add_enabled(
                !node.is_disabled(),
                egui::DragValue::new(&mut val).range(min..=max).speed(step),
            );
            stats.widgets_rendered += 1;
        }
        Role::Slider => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let mut slider = egui::Slider::new(&mut val, min..=max);
            if !text.is_empty() {
                slider = slider.text(&text);
            }
            ui.add_enabled(!disabled, slider);
            stats.widgets_rendered += 1;
        }
        Role::ProgressIndicator | Role::Meter => {
            let text = node_label(node);
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            let fraction = if max > 0.0 { (val / max) as f32 } else { 0.0 };
            let mut pb = egui::ProgressBar::new(fraction.clamp(0.0, 1.0));
            if !text.is_empty() {
                pb = pb.text(&text);
            }
            ui.add(pb);
            stats.widgets_rendered += 1;
        }
        Role::ComboBox | Role::EditableComboBox => {
            let text = node_label(node);
            let val = node.value().unwrap_or("").to_string();
            egui::ComboBox::from_label(&text)
                .selected_text(&val)
                .show_ui(ui, |_ui| {});
            stats.widgets_rendered += 1;
        }
        Role::Link => {
            let text = node_label(node);
            let url = node.url().unwrap_or("#");
            ui.add(egui::Hyperlink::from_label_and_url(&text, url));
            stats.widgets_rendered += 1;
        }
        Role::ColorWell => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            ui.add_enabled(!disabled, egui::Button::new(format!("🎨 {text}")));
            stats.widgets_rendered += 1;
        }
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
            let alt = node_label(node);
            let text = if alt.is_empty() {
                "🖼 [image]".to_string()
            } else {
                format!("🖼 {alt}")
            };
            ui.label(text);
            stats.widgets_rendered += 1;
        }
        Role::Toolbar => {
            stats.containers_rendered += 1;
            ui.horizontal(|ui| {
                for child_id in node.children() {
                    render_node_recursive(ui, nodes, *child_id, stats, clicked_nodes);
                }
            });
        }
        Role::List | Role::ListBox | Role::Feed | Role::DescriptionList => {
            stats.containers_rendered += 1;
            ui.vertical(|ui| {
                for child_id in node.children() {
                    render_node_recursive(ui, nodes, *child_id, stats, clicked_nodes);
                }
            });
        }
        Role::Table | Role::Grid | Role::TreeGrid | Role::ListGrid => {
            stats.containers_rendered += 1;
            egui::Grid::new(format!("grid_{:?}", node.role()))
                .striped(true)
                .show(ui, |ui| {
                    for child_id in node.children() {
                        render_node_recursive(ui, nodes, *child_id, stats, clicked_nodes);
                    }
                });
        }
        Role::TabList => {
            stats.containers_rendered += 1;
            ui.horizontal(|ui| {
                for child_id in node.children() {
                    render_node_recursive(ui, nodes, *child_id, stats, clicked_nodes);
                }
            });
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
        | Role::ListBoxOption
        | Role::MenuItem
        | Role::MenuListOption => {
            render_container(ui, nodes, node, stats, clicked_nodes);
        }
        Role::TreeItem => {
            if !node.children().is_empty() {
                render_container(ui, nodes, node, stats, clicked_nodes);
            } else {
                let text = node_label(node);
                let selected = node.is_selected().unwrap_or(false);
                if selected {
                    let hl = ui.visuals().selection.bg_fill;
                    let fg = ui.visuals().selection.stroke.color;
                    ui.painter()
                        .rect_filled(ui.available_rect_before_wrap(), 0.0, hl);
                    let _ = ui.colored_label(fg, &text);
                } else {
                    ui.label(&text);
                }
                stats.widgets_rendered += 1;
            }
        }
        Role::Dialog | Role::AlertDialog => {
            render_container(ui, nodes, node, stats, clicked_nodes);
        }
        Role::Menu | Role::MenuBar | Role::MenuListPopup => {
            render_container(ui, nodes, node, stats, clicked_nodes);
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
            render_container(ui, nodes, node, stats, clicked_nodes);
        }
        Role::ScrollView | Role::ScrollBar => {
            render_container(ui, nodes, node, stats, clicked_nodes);
        }
        Role::Splitter => {
            ui.separator();
            stats.widgets_rendered += 1;
        }
        Role::LineBreak => {
            ui.end_row();
            stats.widgets_rendered += 1;
        }
        Role::Figure => {
            let desc = node.description().unwrap_or("");
            let children = node.children();
            if desc.contains("w=") && desc.contains("h=") && !children.is_empty() {
                // Spatial ERD diagram: render with Painter.
                let coords = parse_kv_coords_egui(desc);
                let cw = coords.get("w").copied().unwrap_or(800.0);
                let ch = coords.get("h").copied().unwrap_or(600.0);

                // Collect box and edge data before borrowing `ui`.
                struct ErdBox {
                    x: f32,
                    y: f32,
                    w: f32,
                    h: f32,
                    name: String,
                    cols: Vec<String>,
                }
                struct ErdEdge {
                    x1: f32,
                    y1: f32,
                    x2: f32,
                    y2: f32,
                    label: String,
                }

                let mut boxes: Vec<ErdBox> = Vec::new();
                let mut edges: Vec<ErdEdge> = Vec::new();

                for child_id in children.iter() {
                    let Some(child) = nodes.get(child_id) else {
                        continue;
                    };
                    let child_desc = child.description().unwrap_or("");
                    let c = parse_kv_coords_egui(child_desc);
                    if child_desc.contains("x1=") {
                        edges.push(ErdEdge {
                            x1: c.get("x1").copied().unwrap_or(0.0),
                            y1: c.get("y1").copied().unwrap_or(0.0),
                            x2: c.get("x2").copied().unwrap_or(0.0),
                            y2: c.get("y2").copied().unwrap_or(0.0),
                            label: child.label().unwrap_or("").to_string(),
                        });
                    } else if child_desc.contains("x=") {
                        let cols: Vec<String> = child
                            .children()
                            .iter()
                            .filter_map(|&col_id| nodes.get(&col_id))
                            .map(|n| n.label().unwrap_or("").to_string())
                            .collect();
                        boxes.push(ErdBox {
                            x: c.get("x").copied().unwrap_or(0.0),
                            y: c.get("y").copied().unwrap_or(0.0),
                            w: c.get("w").copied().unwrap_or(200.0),
                            h: c.get("h").copied().unwrap_or(80.0),
                            name: child.label().unwrap_or("").to_string(),
                            cols,
                        });
                    }
                }

                let desired = egui::Vec2::new(cw.min(2000.0), ch.min(1200.0));
                let (resp, painter) = ui.allocate_painter(desired, egui::Sense::hover());
                let origin = resp.rect.min;
                let scale = (resp.rect.width() / cw).min(1.0);

                let col_fg = egui::Color32::from_rgb(205, 214, 244);
                let title_fg = egui::Color32::from_rgb(137, 180, 250);
                let box_bg = egui::Color32::from_rgb(49, 50, 68);
                let header_bg = egui::Color32::from_rgb(30, 30, 46);
                let border = egui::Color32::from_rgb(69, 71, 90);
                let edge_col = egui::Color32::from_rgb(108, 112, 134);

                // Edges.
                for e in &edges {
                    let p1 = origin + egui::Vec2::new(e.x1 * scale, e.y1 * scale);
                    let p2 = origin + egui::Vec2::new(e.x2 * scale, e.y2 * scale);
                    painter.line_segment([p1, p2], egui::Stroke::new(1.5, edge_col));
                    let _ = &e.label; // used for accessibility / future tooltips
                }

                // Boxes.
                let font_id = egui::FontId::monospace(10.0 * scale);
                let title_font = egui::FontId::monospace(11.0 * scale);
                for b in &boxes {
                    let tl = origin + egui::Vec2::new(b.x * scale, b.y * scale);
                    let br = tl + egui::Vec2::new(b.w * scale, b.h * scale);
                    let rect = egui::Rect::from_min_max(tl, br);

                    // Box background.
                    painter.rect_filled(rect, 3.0, box_bg);
                    painter.rect_stroke(
                        rect,
                        3.0,
                        egui::Stroke::new(1.0, border),
                        egui::StrokeKind::Outside,
                    );

                    // Header band.
                    let header_br = tl + egui::Vec2::new(b.w * scale, 24.0 * scale);
                    let header_rect = egui::Rect::from_min_max(tl, header_br);
                    painter.rect_filled(header_rect, 3.0, header_bg);
                    painter.rect_stroke(
                        header_rect,
                        3.0,
                        egui::Stroke::new(1.0, border),
                        egui::StrokeKind::Outside,
                    );

                    // Table name.
                    painter.text(
                        header_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        &b.name,
                        title_font.clone(),
                        title_fg,
                    );

                    // Column rows.
                    for (i, col) in b.cols.iter().enumerate() {
                        let ty = tl.y + (24.0 + (i as f32 + 0.5) * 20.0 + 4.0) * scale;
                        let tx = tl.x + 6.0 * scale;
                        painter.text(
                            egui::Pos2::new(tx, ty),
                            egui::Align2::LEFT_CENTER,
                            col,
                            font_id.clone(),
                            col_fg,
                        );
                    }
                }

                stats.containers_rendered += 1;
            } else {
                render_container(ui, nodes, node, stats, clicked_nodes);
            }
        }
        _ => {
            if node.children().is_empty() {
                stats.nodes_skipped += 1;
            } else {
                render_container(ui, nodes, node, stats, clicked_nodes);
            }
        }
    }
}

fn render_container(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut elicit_ui::RenderStats,
    clicked_nodes: &mut Vec<NodeId>,
) {
    stats.containers_rendered += 1;
    let children = node.children();
    if children.is_empty() {
        return;
    }
    ui.group(|ui| {
        for child_id in children {
            render_node_recursive(ui, nodes, *child_id, stats, clicked_nodes);
        }
    });
}

fn render_label(ui: &mut egui::Ui, node: &Node) {
    let text = node.value().or(node.label()).unwrap_or("");
    let rt = match node.role() {
        Role::Heading => egui::RichText::new(text).strong().size(heading_size(node)),
        Role::Strong => egui::RichText::new(text).strong(),
        Role::Emphasis | Role::Mark => egui::RichText::new(text).italics(),
        Role::Code => egui::RichText::new(text).monospace(),
        Role::Alert | Role::Status => egui::RichText::new(text).color(egui::Color32::YELLOW),
        _ => egui::RichText::new(text),
    };
    ui.add(egui::Label::new(rt));
}

/// Parse a comma-separated `key=value` coordinate string (e.g. `"x=10,y=20,w=200,h=80"`)
/// into a map of string keys to `f32` values.
///
/// Used to decode spatial metadata from [`accesskit::Role::Figure`] ERD nodes.
fn parse_kv_coords_egui(desc: &str) -> std::collections::HashMap<&str, f32> {
    desc.split(',')
        .filter_map(|part| {
            let (k, v) = part.split_once('=')?;
            let v = v.trim().parse::<f32>().ok()?;
            Some((k.trim(), v))
        })
        .collect()
}
