//! AccessKit bridge for egui — bidirectional conversion between
//! AccessKit trees and egui widgets.
//!
//! Provides:
//! - [`EguiBackend`] — [`UiNodeBridge`] + [`UiRenderBackend`] for egui
//! - [`render_tree`] — render an AccessKit tree directly into an `egui::Ui`
//! - [`bounds_to_size`] — extract pixel size from AccessKit node bounds

use accesskit::{Node, NodeId, Rect, Role, Toggled};
use elicit_ui::{UiNodeBridge, UiRenderBackend};
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
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
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
    }

    fn bridge_generic_container(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_pane(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_window(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_document(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_root_web_area(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_application(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_terminal(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.monospace(&text);
        })
    }

    // ── Interactive widgets ───────────────────────────────────────────────

    fn bridge_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let disabled = node.is_disabled();
        Box::new(move |ui| {
            ui.add_enabled(!disabled, egui::Button::new(&text));
        })
    }

    fn bridge_default_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_button(node, id, children)
    }

    fn bridge_link(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        let url = node.url().unwrap_or("#").to_string();
        Box::new(move |ui| {
            ui.add(egui::Hyperlink::from_label_and_url(&text, &url));
        })
    }

    fn bridge_check_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let disabled = node.is_disabled();
        let mut checked = matches!(node.toggled(), Some(Toggled::True));
        Box::new(move |ui| {
            ui.add_enabled(!disabled, egui::Checkbox::new(&mut checked, &text));
        })
    }

    fn bridge_radio_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let disabled = node.is_disabled();
        let selected = matches!(node.toggled(), Some(Toggled::True));
        Box::new(move |ui| {
            ui.add_enabled(!disabled, egui::RadioButton::new(selected, &text));
        })
    }

    fn bridge_switch(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let disabled = node.is_disabled();
        let mut on = matches!(node.toggled(), Some(Toggled::True));
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                ui.add_enabled(!disabled, egui::Checkbox::new(&mut on, &text));
            });
        })
    }

    fn bridge_color_well(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let disabled = node.is_disabled();
        Box::new(move |ui| {
            ui.add_enabled(!disabled, egui::Button::new(format!("🎨 {text}")));
        })
    }

    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let open = matches!(node.toggled(), Some(Toggled::True));
        let arrow = if open { "▼" } else { "▶" };
        Box::new(move |ui| {
            ui.label(format!("{arrow} {text}"));
        })
    }

    fn bridge_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let val = node.value().unwrap_or("").to_string();
        Box::new(move |ui| {
            egui::ComboBox::from_label(&text)
                .selected_text(&val)
                .show_ui(ui, |_ui| {});
        })
    }

    fn bridge_editable_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_combo_box(node, id, children)
    }

    fn bridge_list_box(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_slider(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let disabled = node.is_disabled();
        let mut val = node.numeric_value().unwrap_or(0.0);
        let min = node.min_numeric_value().unwrap_or(0.0);
        let max = node.max_numeric_value().unwrap_or(100.0);
        Box::new(move |ui| {
            let mut slider = egui::Slider::new(&mut val, min..=max);
            if !text.is_empty() {
                slider = slider.text(&text);
            }
            ui.add_enabled(!disabled, slider);
        })
    }

    fn bridge_spin_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let disabled = node.is_disabled();
        let mut val = node.numeric_value().unwrap_or(0.0);
        let min = node.min_numeric_value().unwrap_or(f64::MIN);
        let max = node.max_numeric_value().unwrap_or(f64::MAX);
        let step = node.numeric_value_step().unwrap_or(1.0);
        Box::new(move |ui| {
            ui.add_enabled(
                !disabled,
                egui::DragValue::new(&mut val).range(min..=max).speed(step),
            );
        })
    }

    fn bridge_progress_indicator(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let val = node.numeric_value().unwrap_or(0.0);
        let max = node.max_numeric_value().unwrap_or(100.0);
        let fraction = if max > 0.0 { (val / max) as f32 } else { 0.0 };
        Box::new(move |ui| {
            let mut pb = egui::ProgressBar::new(fraction.clamp(0.0, 1.0));
            if !text.is_empty() {
                pb = pb.text(&text);
            }
            ui.add(pb);
        })
    }

    fn bridge_scroll_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_progress_indicator(node, id, children)
    }

    fn bridge_scroll_view(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_splitter(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(|ui| {
            ui.separator();
        })
    }

    // ── Text input ───────────────────────────────────────────────────────

    fn bridge_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let mut buf = node.value().unwrap_or("").to_string();
        let hint = node.placeholder().map(|s| s.to_string());
        let readonly = node.is_read_only() || node.is_disabled();
        Box::new(move |ui| {
            let mut te = egui::TextEdit::singleline(&mut buf);
            if let Some(h) = &hint {
                te = te.hint_text(h.as_str());
            }
            if readonly {
                te = te.interactive(false);
            }
            ui.add(te);
        })
    }

    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let mut buf = node.value().unwrap_or("").to_string();
        let hint = node.placeholder().map(|s| s.to_string());
        let readonly = node.is_read_only() || node.is_disabled();
        Box::new(move |ui| {
            let mut te = egui::TextEdit::multiline(&mut buf);
            if let Some(h) = &hint {
                te = te.hint_text(h.as_str());
            }
            if readonly {
                te = te.interactive(false);
            }
            ui.add(te);
        })
    }

    fn bridge_search_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_date_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_date_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_week_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_month_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_email_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_spin_button(node, id, children)
    }

    fn bridge_password_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_phone_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    fn bridge_url_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    // ── Text display ─────────────────────────────────────────────────────

    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_paragraph(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_label(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_heading(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        let size = heading_size(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(&text).strong().size(size),
            ));
        })
    }

    fn bridge_line_break(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(|ui| {
            ui.end_row();
        })
    }

    fn bridge_blockquote(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(format!("│ {text}")).italics(),
            ));
        })
    }

    fn bridge_code(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(egui::RichText::new(&text).monospace()));
        })
    }

    fn bridge_math(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_note(&self, node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        if children.is_empty() {
            let text = node_label(node);
            Box::new(move |ui| {
                ui.label(&text);
            })
        } else {
            Box::new(move |ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        }
    }

    fn bridge_term(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(egui::RichText::new(&text).strong()));
        })
    }

    fn bridge_definition(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    // ── Media ────────────────────────────────────────────────────────────

    fn bridge_image(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let alt = node_label(node);
        let text = if alt.is_empty() {
            "🖼 [image]".to_string()
        } else {
            format!("🖼 {alt}")
        };
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_figure(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_figure_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(egui::RichText::new(&text).italics()));
        })
    }

    fn bridge_canvas(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(format!("[canvas: {text}]"));
        })
    }

    fn bridge_video(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(format!("[video: {text}]"));
        })
    }

    fn bridge_audio(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(format!("[audio: {text}]"));
        })
    }

    // ── Landmark sections ─────────────────────────────────────────────────

    fn bridge_main(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_navigation(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_banner(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_content_info(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_complementary(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_form(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_search(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_region(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_section(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_section_header(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_section_footer(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_article(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_group(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_dialog(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_details(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_tooltip(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_alert(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(
                egui::RichText::new(&text).color(egui::Color32::YELLOW),
            ));
        })
    }

    fn bridge_status(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_timer(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    fn bridge_list(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_list_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(format!("• {text}"));
        })
    }

    fn bridge_description_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    // ── Tables ────────────────────────────────────────────────────────────

    fn bridge_table(&self, node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        let role_str = format!("{:?}", node.role());
        Box::new(move |ui| {
            egui::Grid::new(format!("grid_{role_str}"))
                .striped(true)
                .show(ui, |ui| {
                    for c in children {
                        c(ui);
                    }
                });
        })
    }

    fn bridge_row(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            for c in children {
                c(ui);
            }
            ui.end_row();
        })
    }

    fn bridge_cell(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.label(&text);
        })
    }

    fn bridge_caption(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            ui.add(egui::Label::new(egui::RichText::new(&text).italics()));
        })
    }

    fn bridge_row_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    // ── Trees ─────────────────────────────────────────────────────────────

    fn bridge_tree(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_tree_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        if children.is_empty() {
            let text = node_label(node);
            let selected = node.is_selected().unwrap_or(false);
            Box::new(move |ui| {
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
            Box::new(move |ui| {
                ui.group(|ui| {
                    for c in children {
                        c(ui);
                    }
                });
            })
        }
    }

    // ── Tabs ─────────────────────────────────────────────────────────────

    fn bridge_tab(&self, node: &Node, _id: NodeId, _children: Vec<Self::Widget>) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            let _ = ui.selectable_label(false, &text);
        })
    }

    fn bridge_tab_list(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_tab_panel(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.group(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    // ── Menus ─────────────────────────────────────────────────────────────

    fn bridge_menu(&self, _node: &Node, _id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_menu_item(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<Self::Widget>,
    ) -> Self::Widget {
        let text = node_label(node);
        Box::new(move |ui| {
            let _ = ui.selectable_label(false, &text);
        })
    }

    fn bridge_toolbar(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.horizontal(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }

    fn bridge_radio_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        Box::new(move |ui| {
            ui.vertical(|ui| {
                for c in children {
                    c(ui);
                }
            });
        })
    }
}

// ── Public helpers ────────────────────────────────────────────────────────────

/// Render a verified AccessKit tree into an `egui::Ui`.
///
/// Walks from `root` through the node map, creating egui widgets
/// for each node based on its role. Container nodes create nested
/// layouts; leaf nodes create widgets.
///
/// Returns [`elicit_ui::RenderStats`] for the render pass.
#[tracing::instrument(skip(ui, nodes), fields(root = ?root))]
pub fn render_tree(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    root: NodeId,
) -> elicit_ui::RenderStats {
    let mut stats = elicit_ui::RenderStats::default();
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
            render_container(ui, nodes, node, stats);
        }
        Role::Button | Role::DefaultButton => {
            let text = node_label(node);
            let disabled = node.is_disabled();
            ui.add_enabled(!disabled, egui::Button::new(&text));
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
                    render_node_recursive(ui, nodes, *child_id, stats);
                }
            });
        }
        Role::List | Role::ListBox | Role::Feed | Role::DescriptionList => {
            stats.containers_rendered += 1;
            ui.vertical(|ui| {
                for child_id in node.children() {
                    render_node_recursive(ui, nodes, *child_id, stats);
                }
            });
        }
        Role::Table | Role::Grid | Role::TreeGrid | Role::ListGrid => {
            stats.containers_rendered += 1;
            egui::Grid::new(format!("grid_{:?}", node.role()))
                .striped(true)
                .show(ui, |ui| {
                    for child_id in node.children() {
                        render_node_recursive(ui, nodes, *child_id, stats);
                    }
                });
        }
        Role::TabList => {
            stats.containers_rendered += 1;
            ui.horizontal(|ui| {
                for child_id in node.children() {
                    render_node_recursive(ui, nodes, *child_id, stats);
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
            render_container(ui, nodes, node, stats);
        }
        Role::TreeItem => {
            if !node.children().is_empty() {
                render_container(ui, nodes, node, stats);
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
        Role::Splitter => {
            ui.separator();
            stats.widgets_rendered += 1;
        }
        Role::LineBreak => {
            ui.end_row();
            stats.widgets_rendered += 1;
        }
        _ => {
            if node.children().is_empty() {
                stats.nodes_skipped += 1;
            } else {
                render_container(ui, nodes, node, stats);
            }
        }
    }
}

fn render_container(
    ui: &mut egui::Ui,
    nodes: &HashMap<NodeId, Node>,
    node: &Node,
    stats: &mut elicit_ui::RenderStats,
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
