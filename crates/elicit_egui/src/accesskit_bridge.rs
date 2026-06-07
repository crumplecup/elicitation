//! AccessKit bridge for egui — bidirectional conversion between
//! AccessKit trees and egui widgets.
//!
//! Provides:
//! - [`EguiBackend`] — [`UiNodeBridge`] + [`UiRenderBackend`] for egui
//! - [`render_tree`] — render an AccessKit tree directly into an `egui::Ui`
//! - [`bounds_to_size`] — extract pixel size from AccessKit node bounds

use accesskit::{Node, NodeId, Rect, Role, Toggled};
use elicit_ui::node_roles::*;
use elicit_ui::{
    NodeRenderedEvidence, RolePreserved, UiNodeBridge, UiRenderBackend, WcagNodeProofs,
    verify_wcag_contrast_proofs,
};
use elicitation::Established;
use std::collections::BTreeMap;

#[cfg(not(all(debug_assertions, feature = "runtime-proofs")))]
use crate::render_context::EguiRenderContext;

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
pub struct EguiBackend {
    /// Pending (rect, proofs, ppp) items awaiting GPU pixel readback verification.
    ///
    /// Populated by [`wrap_widget`](UiNodeBridge::wrap_widget) during the egui frame
    /// and drained by [`post_frame`](EguiBackend::post_frame) after GPU readback.
    #[cfg(feature = "runtime-proofs")]
    pending: std::sync::Arc<std::sync::Mutex<Vec<(egui::Rect, WcagNodeProofs, f32)>>>,
}

impl EguiBackend {
    /// Create a new egui render backend.
    pub fn new() -> Self {
        Self::default()
    }

    /// Drain pending contrast checks using GPU pixel data from the current frame.
    ///
    /// Call this once per frame **after** `copy_texture_to_buffer` + `map_async` +
    /// `device.poll(wait_indefinitely())` — i.e. after the pixel data has been
    /// transferred from the GPU into a CPU-accessible staging buffer.
    ///
    /// Each pending `(rect, proofs, ppp)` tuple is verified against the actual
    /// rendered pixels via [`GpuPixelContext`](crate::render_context::GpuPixelContext).
    ///
    /// If `post_frame` is not called every frame the pending queue accumulates,
    /// growing O(N × missed frames).
    ///
    /// # Parameters
    ///
    /// * `pixels` — byte slice from `Buffer::get_mapped_range()` (full frame)
    /// * `width` — surface width in physical pixels
    /// * `bytes_per_row` — padded stride (multiple of 256, as required by wgpu)
    /// * `height` — surface height in physical pixels
    /// * `format` — surface texture format (determines `Bgra` vs `Rgba` byte order)
    #[cfg(all(debug_assertions, feature = "runtime-proofs"))]
    pub fn post_frame(
        &self,
        pixels: &[u8],
        width: u32,
        bytes_per_row: u32,
        height: u32,
        format: wgpu::TextureFormat,
    ) {
        use crate::render_context::GpuPixelContext;
        let items: Vec<(egui::Rect, WcagNodeProofs, f32)> = {
            let mut guard = self.pending.lock().unwrap();
            guard.drain(..).collect()
        };
        for (rect, proofs, ppp) in items {
            let gpu_ctx = GpuPixelContext::new(pixels, width, bytes_per_row, height, format, ppp);
            verify_wcag_contrast_proofs(&gpu_ctx, &rect, &proofs);
        }
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

    // ── Post-render hooks ─────────────────────────────────────────────────

    /// Wrap the widget closure to run WCAG contrast checks after it draws.
    ///
    /// With the `runtime-proofs` feature and debug builds, defers verification
    /// to [`post_frame`](EguiBackend::post_frame) so actual GPU pixel data
    /// (from surface readback) is used instead of theme defaults.
    ///
    /// Without `runtime-proofs`, performs an immediate best-effort check using
    /// `ui.visuals()` theme colours at draw time.
    fn wrap_widget(
        &self,
        widget: Box<dyn FnOnce(&mut egui::Ui)>,
        proofs: &WcagNodeProofs,
    ) -> Box<dyn FnOnce(&mut egui::Ui)> {
        let proofs = *proofs;

        #[cfg(all(debug_assertions, feature = "runtime-proofs"))]
        {
            // Defer: push rect + proofs + ppp to pending queue; post_frame will
            // verify against actual GPU-rendered pixels after surface readback.
            let pending = std::sync::Arc::clone(&self.pending);
            Box::new(move |ui: &mut egui::Ui| {
                widget(ui);
                let ppp = ui.ctx().pixels_per_point();
                let rect = ui.min_rect();
                pending.lock().unwrap().push((rect, proofs, ppp));
            })
        }

        #[cfg(not(all(debug_assertions, feature = "runtime-proofs")))]
        // Fallback: verify immediately using theme colours.
        Box::new(move |ui: &mut egui::Ui| {
            widget(ui);
            let ctx = EguiRenderContext::from_ui(ui);
            verify_wcag_contrast_proofs(&ctx, &ui.min_rect(), &proofs);
        })
    }

    // ── Unknown / generic ─────────────────────────────────────────────────

    fn bridge_unknown(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<UnknownNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GenericContainerNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<PaneNodeValid>,
        proofs: WcagNodeProofs,
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
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_window(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<WindowNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocumentNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RootWebAreaNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ApplicationNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TerminalNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.monospace(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::Button::new(&text));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LinkNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let url = node.url().unwrap_or("#").to_string();
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Hyperlink::from_label_and_url(&text, &url));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CheckBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let mut checked = matches!(node.toggled(), Some(Toggled::True));
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::Checkbox::new(&mut checked, &text));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RadioButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            let selected = matches!(node.toggled(), Some(Toggled::True));
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::RadioButton::new(selected, &text));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SwitchNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ColorWellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let disabled = node.is_disabled();
            Box::new(move |ui: &mut egui::Ui| {
                ui.add_enabled(!disabled, egui::Button::new(format!("🎨 {text}")));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DisclosureTriangleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            let open = matches!(node.toggled(), Some(Toggled::True));
            let arrow = if open { "▼" } else { "▶" };
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("{arrow} {text}"));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ComboBoxNodeValid>,
        proofs: WcagNodeProofs,
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
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListBoxNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SliderNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SpinButtonNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ProgressIndicatorNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ScrollViewNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SplitterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            Box::new(|ui: &mut egui::Ui| {
                ui.separator();
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TextInputNodeValid>,
        proofs: WcagNodeProofs,
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
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MultilineTextInputNodeValid>,
        proofs: WcagNodeProofs,
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
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NumberInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_spin_button(
            node,
            id,
            children,
            Established::<SpinButtonNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Text display ─────────────────────────────────────────────────────

    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TextRunNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ParagraphNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        use elicit_ui::{ParagraphText, TextAlign};
        let rich =
            node_json_rich_text(node).and_then(|v| serde_json::from_value::<ParagraphText>(v).ok());
        let __w: Box<dyn FnOnce(&mut egui::Ui) + Send + Sync> = match rich {
            Some(ParagraphText::Rich(rich)) => {
                let block_style = rich.style.clone();
                let block_align = rich.alignment;
                // Derive LayoutJob halign/justify from block alignment.
                let (halign, justify) = match block_align {
                    Some(TextAlign::Center) => (egui::Align::Center, false),
                    Some(TextAlign::Right) => (egui::Align::RIGHT, false),
                    Some(TextAlign::Justify) => (egui::Align::LEFT, true),
                    _ => (egui::Align::LEFT, false),
                };
                let lines = rich.lines.clone();
                Box::new(move |ui: &mut egui::Ui| {
                    let mut job = egui::text::LayoutJob {
                        halign,
                        justify,
                        ..Default::default()
                    };
                    for (li, line) in lines.iter().enumerate() {
                        // Insert line break between lines.
                        if li > 0 {
                            let line_fmt =
                                build_text_format(None, line.style.as_ref(), block_style.as_ref());
                            job.append("\n", 0.0, line_fmt);
                        }
                        for span in &line.spans {
                            let fmt = build_text_format(
                                span.style.as_ref(),
                                line.style.as_ref(),
                                block_style.as_ref(),
                            );
                            job.append(&span.content, 0.0, fmt);
                        }
                    }
                    ui.label(job);
                })
            }
            Some(ParagraphText::Plain(text)) => Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            }),
            None => {
                let text = node_label(node);
                Box::new(move |ui: &mut egui::Ui| {
                    ui.label(&text);
                })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LabelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<HeadingNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LineBreakNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            Box::new(|ui: &mut egui::Ui| {
                ui.end_row();
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<BlockquoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(format!("│ {text}")).italics(),
                ));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CodeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).monospace()));
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MathNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NoteNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TermNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).strong()));
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DefinitionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ImageNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FigureNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FigureCaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).italics()));
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CanvasNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("[canvas: {text}]"));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<VideoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("[video: {text}]"));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<AudioNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("[audio: {text}]"));
            })
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

    fn bridge_main(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MainNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NavigationNodeValid>,
        proofs: WcagNodeProofs,
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
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_banner(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<BannerNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ContentInfoNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ComplementaryNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FormNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SearchNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RegionNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SectionNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SectionHeaderNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SectionFooterNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ArticleNodeValid>,
        proofs: WcagNodeProofs,
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
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GroupNodeValid>,
        proofs: WcagNodeProofs,
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
        (
            __w,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DialogNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DetailsNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TooltipNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<AlertNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(&text).color(egui::Color32::YELLOW),
                ));
            })
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<StatusNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TimerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _node: &Node,
        _id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(format!("• {text}"));
            })
        };
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DescriptionListNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TableNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RowNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.label(&text);
            })
        };
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                ui.add(egui::Label::new(egui::RichText::new(&text).italics()));
            })
        };
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RowGroupNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TreeNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TreeItemNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TabNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                let _ = ui.selectable_label(false, &text);
            })
        };
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TabListNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TabPanelNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuNodeValid>,
        proofs: WcagNodeProofs,
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
        _children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        let __w = {
            let text = node_label(node);
            Box::new(move |ui: &mut egui::Ui| {
                let _ = ui.selectable_label(false, &text);
            })
        };
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ToolbarNodeValid>,
        proofs: WcagNodeProofs,
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RadioGroupNodeValid>,
        proofs: WcagNodeProofs,
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
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
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
    nodes: &BTreeMap<NodeId, Node>,
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
    elicit_accesskit::node_label(node).to_string()
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
    nodes: &BTreeMap<NodeId, Node>,
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
            // SQL editor: attach syntax-highlighting layouter.
            let is_sql = node
                .label()
                .map(|n: &str| n.eq_ignore_ascii_case("sql editor"))
                .unwrap_or(false);
            if is_sql {
                let mut layouter = |ui: &egui::Ui, s: &dyn egui::TextBuffer, _wrap: f32| {
                    let job = sql_layout_job(s.as_str(), elicit_ui::palettes::mocha());
                    ui.painter().layout_job(job)
                };
                te = te.layouter(&mut layouter);
                ui.add(te);
            } else {
                ui.add(te);
            }
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
        Role::Paragraph => {
            render_paragraph(ui, node);
            stats.widgets_rendered += 1;
        }
        Role::Label
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
                    painter.line_segment([p1, p2], egui::Stroke::new(1.5_f32, edge_col));
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
                        egui::Stroke::new(1.0_f32, border),
                        egui::StrokeKind::Outside,
                    );

                    // Header band.
                    let header_br = tl + egui::Vec2::new(b.w * scale, 24.0 * scale);
                    let header_rect = egui::Rect::from_min_max(tl, header_br);
                    painter.rect_filled(header_rect, 3.0, header_bg);
                    painter.rect_stroke(
                        header_rect,
                        3.0,
                        egui::Stroke::new(1.0_f32, border),
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
    nodes: &BTreeMap<NodeId, Node>,
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

// ── Rich-text paragraph helpers ────────────────────────────────────────────────

/// Render a paragraph node to `ui`, using a [`egui::text::LayoutJob`] when the
/// node carries a `__rich_text__:` sidecar, or falling back to plain text.
fn render_paragraph(ui: &mut egui::Ui, node: &Node) {
    use elicit_ui::{ParagraphText, TextAlign};
    let rich =
        node_json_rich_text(node).and_then(|v| serde_json::from_value::<ParagraphText>(v).ok());
    match rich {
        Some(ParagraphText::Rich(rich)) => {
            let (halign, justify) = match rich.alignment {
                Some(TextAlign::Center) => (egui::Align::Center, false),
                Some(TextAlign::Right) => (egui::Align::RIGHT, false),
                Some(TextAlign::Justify) => (egui::Align::LEFT, true),
                _ => (egui::Align::LEFT, false),
            };
            let mut job = egui::text::LayoutJob {
                halign,
                justify,
                ..Default::default()
            };
            for (li, line) in rich.lines.iter().enumerate() {
                if li > 0 {
                    let fmt = build_text_format(None, line.style.as_ref(), rich.style.as_ref());
                    job.append("\n", 0.0, fmt);
                }
                for span in &line.spans {
                    let fmt = build_text_format(
                        span.style.as_ref(),
                        line.style.as_ref(),
                        rich.style.as_ref(),
                    );
                    job.append(&span.content, 0.0, fmt);
                }
            }
            ui.label(job);
        }
        Some(ParagraphText::Plain(text)) => {
            ui.label(&text);
        }
        None => {
            ui.label(node_label(node));
        }
    }
}

/// Extract the `__rich_text__:` sidecar from the node's `class_name` field,
/// returning the raw JSON value if present.
fn node_json_rich_text(node: &Node) -> Option<serde_json::Value> {
    const PREFIX: &str = "__rich_text__:";
    let class = node.class_name()?;
    if !class.starts_with(PREFIX) {
        return None;
    }
    serde_json::from_str(&class[PREFIX.len()..]).ok()
}

/// Convert a [`elicit_ui::UiColor`] to an [`egui::Color32`].
///
/// Named ANSI colours use the VS Code terminal palette (same reference sRGB
/// values as `elicit_ui::ansi256_to_peniko`).  Alpha is honoured for `Rgba`.
fn ui_color_to_color32(color: &elicit_ui::UiColor) -> egui::Color32 {
    use elicit_ui::UiColor;
    match color {
        UiColor::Reset => egui::Color32::PLACEHOLDER,
        UiColor::Black => egui::Color32::from_rgb(0x0c, 0x0c, 0x0c),
        UiColor::Red => egui::Color32::from_rgb(0xc5, 0x0f, 0x1f),
        UiColor::Green => egui::Color32::from_rgb(0x13, 0xa1, 0x0e),
        UiColor::Yellow => egui::Color32::from_rgb(0xc1, 0x9c, 0x00),
        UiColor::Blue => egui::Color32::from_rgb(0x00, 0x37, 0xda),
        UiColor::Magenta => egui::Color32::from_rgb(0x88, 0x17, 0x98),
        UiColor::Cyan => egui::Color32::from_rgb(0x3a, 0x96, 0xdd),
        UiColor::White => egui::Color32::from_rgb(0xcc, 0xcc, 0xcc),
        UiColor::DarkGray => egui::Color32::from_rgb(0x76, 0x76, 0x76),
        UiColor::LightRed => egui::Color32::from_rgb(0xe7, 0x48, 0x56),
        UiColor::LightGreen => egui::Color32::from_rgb(0x16, 0xc6, 0x0c),
        UiColor::LightYellow => egui::Color32::from_rgb(0xf9, 0xf1, 0xa5),
        UiColor::LightBlue => egui::Color32::from_rgb(0x3b, 0x78, 0xff),
        UiColor::LightMagenta => egui::Color32::from_rgb(0xb4, 0x00, 0x9e),
        UiColor::LightCyan => egui::Color32::from_rgb(0x61, 0xd6, 0xd6),
        UiColor::Gray => egui::Color32::from_rgb(0xf2, 0xf2, 0xf2),
        UiColor::Rgb { r, g, b } => egui::Color32::from_rgb(*r, *g, *b),
        UiColor::Rgba { r, g, b, a } => egui::Color32::from_rgba_unmultiplied(*r, *g, *b, *a),
        UiColor::Indexed { index } => ansi256_to_color32(*index),
    }
}

/// Convert a 256-colour ANSI palette index to [`egui::Color32`].
///
/// Indices 0–15: standard ANSI colours (VS Code palette).
/// Indices 16–231: 6×6×6 colour cube.
/// Indices 232–255: grayscale ramp.
fn ansi256_to_color32(index: u8) -> egui::Color32 {
    let (r, g, b) = match index {
        0 => (0x0c, 0x0c, 0x0c),
        1 => (0xc5, 0x0f, 0x1f),
        2 => (0x13, 0xa1, 0x0e),
        3 => (0xc1, 0x9c, 0x00),
        4 => (0x00, 0x37, 0xda),
        5 => (0x88, 0x17, 0x98),
        6 => (0x3a, 0x96, 0xdd),
        7 => (0xcc, 0xcc, 0xcc),
        8 => (0x76, 0x76, 0x76),
        9 => (0xe7, 0x48, 0x56),
        10 => (0x16, 0xc6, 0x0c),
        11 => (0xf9, 0xf1, 0xa5),
        12 => (0x3b, 0x78, 0xff),
        13 => (0xb4, 0x00, 0x9e),
        14 => (0x61, 0xd6, 0xd6),
        15 => (0xf2, 0xf2, 0xf2),
        16..=231 => {
            let i = index - 16;
            let b_idx = i % 6;
            let g_idx = (i / 6) % 6;
            let r_idx = i / 36;
            let ch = |x: u8| if x == 0 { 0 } else { 55 + x * 40 };
            (ch(r_idx), ch(g_idx), ch(b_idx))
        }
        232..=255 => {
            let v = 8 + (index - 232) * 10;
            (v, v, v)
        }
    };
    egui::Color32::from_rgb(r, g, b)
}

/// Build an [`egui::text::TextFormat`] by cascading span → line → block style.
///
/// Modifiers are applied in this cascade order; span wins over line wins over block.
/// `UiColor::Reset` / absent colour → `Color32::PLACEHOLDER` (theme text colour).
fn build_text_format(
    span_style: Option<&elicit_ui::TextStyle>,
    line_style: Option<&elicit_ui::TextStyle>,
    block_style: Option<&elicit_ui::TextStyle>,
) -> egui::text::TextFormat {
    use elicit_ui::{
        FontFamily, FontStyle, FontWeight, LineHeight, TextDecoration, TextModifier, VerticalAlign,
    };

    // Helper: resolve a field through the three layers.
    macro_rules! cascade {
        ($field:ident) => {
            span_style
                .and_then(|s| s.$field.as_ref())
                .or_else(|| line_style.and_then(|s| s.$field.as_ref()))
                .or_else(|| block_style.and_then(|s| s.$field.as_ref()))
        };
    }
    macro_rules! cascade_vec {
        ($field:ident) => {{
            let sv: &[_] = span_style.map_or(&[], |s| s.$field.as_slice());
            let lv: &[_] = line_style.map_or(&[], |s| s.$field.as_slice());
            let bv: &[_] = block_style.map_or(&[], |s| s.$field.as_slice());
            if !sv.is_empty() {
                sv
            } else if !lv.is_empty() {
                lv
            } else {
                bv
            }
        }};
    }

    let fg_color = cascade!(fg);
    let bg_color = cascade!(bg);
    let modifiers = cascade_vec!(modifiers);
    let decorations = cascade_vec!(decorations);

    // Foreground colour — apply modifier overrides after.
    let mut fg = fg_color
        .map(ui_color_to_color32)
        .unwrap_or(egui::Color32::PLACEHOLDER);

    let mut bg = bg_color
        .map(ui_color_to_color32)
        .unwrap_or(egui::Color32::TRANSPARENT);

    // Process modifiers.
    let mut italic = false;
    let mut underline = egui::Stroke::NONE;
    let mut strikethrough = egui::Stroke::NONE;
    let mut is_dim = false;
    let mut is_hidden = false;
    let mut is_reversed = false;

    for m in modifiers {
        match m {
            TextModifier::Bold => {
                // Bold handled via font_family below.
            }
            TextModifier::Italic => italic = true,
            TextModifier::Underlined => {
                underline = egui::Stroke::new(1.0, fg);
            }
            TextModifier::CrossedOut => {
                strikethrough = egui::Stroke::new(1.0, fg);
            }
            TextModifier::Dim => is_dim = true,
            TextModifier::Hidden => is_hidden = true,
            TextModifier::Reversed => is_reversed = true,
            // SlowBlink / RapidBlink: no native egui blink; content preserved.
            TextModifier::SlowBlink | TextModifier::RapidBlink => {}
        }
    }

    // Decorations from TextStyle::decorations field.
    for d in decorations {
        match d {
            TextDecoration::Underline => {
                underline = egui::Stroke::new(1.0, fg);
            }
            TextDecoration::Strikethrough => {
                strikethrough = egui::Stroke::new(1.0, fg);
            }
            TextDecoration::Overline => {
                // egui has no overline; silently ignored.
            }
        }
    }

    // Apply modifier effects.
    if is_reversed {
        // Swap fg and bg; if no explicit colours, use high-contrast fallback.
        let explicit_fg = fg != egui::Color32::PLACEHOLDER;
        let explicit_bg = bg != egui::Color32::TRANSPARENT;
        if explicit_fg || explicit_bg {
            std::mem::swap(&mut fg, &mut bg);
        } else {
            fg = egui::Color32::from_rgb(0x0c, 0x0c, 0x0c);
            bg = egui::Color32::from_rgb(0xcc, 0xcc, 0xcc);
        }
    }
    if is_dim {
        let [r, g, b, _] = fg.to_array();
        fg = egui::Color32::from_rgba_unmultiplied(r, g, b, 128);
    }
    if is_hidden {
        fg = egui::Color32::TRANSPARENT;
        bg = egui::Color32::TRANSPARENT;
    }

    // Font size.
    let font_size = cascade!(font_size).copied().unwrap_or(14.0);

    // Font family — Bold modifier maps to the "Bold" named family convention.
    let is_bold = modifiers.contains(&TextModifier::Bold)
        || cascade!(font_weight)
            .map(|w| *w == FontWeight::BOLD)
            .unwrap_or(false);

    let font_family = if is_bold {
        // egui bold convention: register a font under this name in your FontDefinitions.
        egui::FontFamily::Name("Bold".into())
    } else {
        match cascade!(font_family) {
            Some(FontFamily::Monospace) => egui::FontFamily::Monospace,
            Some(FontFamily::Named { name }) => egui::FontFamily::Name(name.as_str().into()),
            _ => egui::FontFamily::Proportional,
        }
    };

    // Italic via FontStyle.
    if matches!(
        cascade!(font_style),
        Some(FontStyle::Italic) | Some(FontStyle::Oblique(_))
    ) {
        italic = true;
    }

    // Letter spacing.
    let extra_letter_spacing = cascade!(letter_spacing).copied().unwrap_or(0.0);

    // Line height: convert to absolute points using font_size as base.
    let line_height = cascade!(line_height).map(|lh| match lh {
        LineHeight::Absolute { value } => *value,
        LineHeight::FontSizeRelative { factor } => font_size * factor,
        // MetricsRelative: approximate as font_size × 1.2 × factor.
        LineHeight::MetricsRelative { factor } => font_size * 1.2 * factor,
    });

    // Vertical alignment.
    let valign = match cascade!(vertical_align) {
        Some(VerticalAlign::Top) => egui::Align::TOP,
        Some(VerticalAlign::Bottom) => egui::Align::BOTTOM,
        _ => egui::Align::Center,
    };

    egui::text::TextFormat {
        font_id: egui::FontId::new(font_size, font_family),
        color: fg,
        background: bg,
        italics: italic,
        underline,
        strikethrough,
        extra_letter_spacing,
        line_height,
        valign,
        ..Default::default()
    }
}

// ── SQL syntax highlighting for egui TextEdit ─────────────────────────────────

/// Build an [`egui::text::LayoutJob`] for `sql` with palette-driven syntax highlighting.
fn sql_layout_job(sql: &str, palette: &elicit_ui::Palette) -> egui::text::LayoutJob {
    use elicit_accesskit::sql::{SqlTokenKind, sql_tokens};
    use elicit_ui::SemanticRole;
    let font = egui::FontId::monospace(14.0);
    let mut job = egui::text::LayoutJob::default();
    for token in sql_tokens(sql) {
        let role = match token.kind {
            SqlTokenKind::Keyword => SemanticRole::Keyword,
            SqlTokenKind::StringLiteral => SemanticRole::StringLit,
            SqlTokenKind::Comment => SemanticRole::Comment,
            SqlTokenKind::Number => SemanticRole::Number,
            SqlTokenKind::Plain => SemanticRole::Text,
        };
        let c = palette.color(role);
        job.append(
            token.text,
            0.0,
            egui::text::TextFormat {
                font_id: font.clone(),
                color: egui::Color32::from_rgb(
                    (c.r * 255.0).round() as u8,
                    (c.g * 255.0).round() as u8,
                    (c.b * 255.0).round() as u8,
                ),
                ..Default::default()
            },
        );
    }
    job
}
