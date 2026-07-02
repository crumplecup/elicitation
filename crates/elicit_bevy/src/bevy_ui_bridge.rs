//! Bevy `UiNodeBridge` implementation.
//!
//! [`BevyUiBackend`] implements [`UiNodeBridge`] — one method per
//! [`accesskit::Role`] — producing [`BevyUiNode`] descriptors for each
//! tree node.  The blanket [`UiTreeRenderer`](elicit_ui::UiTreeRenderer)
//! assembles the full tree via DFS and returns the root [`BevyUiNode`].
//!
//! Every bridge method threads its pre-rendered `children` into the output
//! node so the full AccessKit IR hierarchy survives into the Bevy ECS
//! descriptor tree without information loss.

use accesskit::{Node, NodeId, Role, Toggled};
use elicit_ui::node_roles::*;
use elicit_ui::{
    NodeRenderedEvidence, NodeRoleProof, RolePreserved, UiNodeBridge, UiRenderBackend,
    WcagNodeProofs, verify_wcag_contrast_proofs,
};
use elicitation::Established;
use tracing::instrument;

use crate::bevy_ui_node::BevyUiNode;
use crate::render_context::{BevyRenderArea, BevyRenderContext};

// ── BevyUiBackend ─────────────────────────────────────────────────────────────

/// Bevy render backend for verified AccessKit trees.
///
/// Implements [`UiNodeBridge`] — one method per [`accesskit::Role`] — so the
/// blanket [`UiTreeRenderer`](elicit_ui::UiTreeRenderer) provides full-tree DFS
/// rendering for free.  Call `.render(tree)` (from `UiTreeRenderer`) to receive
/// the root [`BevyUiNode`] alongside statistics and the render proof.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_bevy::BevyUiBackend;
/// use elicit_ui::UiRenderBackend;
///
/// let backend = BevyUiBackend::new();
/// assert_eq!(backend.backend_name(), "bevy");
/// ```
#[derive(Default)]
pub struct BevyUiBackend;

impl BevyUiBackend {
    /// Create a new Bevy UI render backend.
    pub fn new() -> Self {
        Self
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn node_label(node: &Node) -> String {
    elicit_accesskit::node_label(node).to_string()
}

fn opt_label(node: &Node) -> Option<String> {
    let s = node_label(node);
    if s.is_empty() { None } else { Some(s) }
}

fn unwrap_children(children: Vec<(BevyUiNode, Established<RolePreserved>)>) -> Vec<BevyUiNode> {
    children.into_iter().map(|(w, _)| w).collect()
}

fn prove_render<T: NodeRoleProof>(
    proof: Established<T>,
    proofs: WcagNodeProofs,
) -> Established<RolePreserved> {
    Established::<RolePreserved>::prove(&NodeRenderedEvidence {
        role: proof,
        wcag: proofs,
    })
}

fn container(
    role: &str,
    label: Option<String>,
    children: Vec<BevyUiNode>,
    proofs: WcagNodeProofs,
) -> BevyUiNode {
    BevyUiNode::Container {
        role: role.to_string(),
        label,
        children,
        proofs,
    }
}

// ── UiRenderBackend ───────────────────────────────────────────────────────────

impl UiRenderBackend for BevyUiBackend {
    fn backend_name(&self) -> &'static str {
        "bevy"
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }
}

// ── UiNodeBridge ─────────────────────────────────────────────────────────────

impl UiNodeBridge for BevyUiBackend {
    type Widget = BevyUiNode;

    #[instrument(skip(self, proofs), fields(role = ?node.role()))]
    fn verify_node(&self, node: &Node, proofs: &WcagNodeProofs) {
        let ctx = BevyRenderContext::from_node(node);
        verify_wcag_contrast_proofs(&ctx, &BevyRenderArea::default(), proofs);
    }

    // ── Unknown / generic ─────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_unknown(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<UnknownNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Unknown {
            label: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_generic_container(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<GenericContainerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "generic",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_pane(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<PaneNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("pane", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_window(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<WindowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("window", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_document(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<DocumentNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "document",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_root_web_area(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<RootWebAreaNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "root_web_area",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_application(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ApplicationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "application",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_terminal(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TerminalNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "terminal",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    // ── Interactive controls ──────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_button(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Button {
            label: node_label(node),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_link(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<LinkNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let url = node.url().unwrap_or("");
        let label = if url.is_empty() {
            node_label(node)
        } else {
            format!("{} ({})", node_label(node), url)
        };
        let w = BevyUiNode::Button {
            label,
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_check_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<CheckBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let checked = match node.toggled() {
            Some(Toggled::True) => Some(true),
            Some(Toggled::False) => Some(false),
            Some(Toggled::Mixed) | None => None,
        };
        let w = BevyUiNode::CheckBox {
            label: node_label(node),
            checked,
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_radio_button(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<RadioButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::RadioButton {
            label: node_label(node),
            selected: node.toggled() == Some(Toggled::True),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_switch(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SwitchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Switch {
            label: node_label(node),
            on: node.toggled() == Some(Toggled::True),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_color_well(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ColorWellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::ColorWell {
            label: node_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<DisclosureTriangleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Button {
            label: node_label(node),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ComboBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::ComboBox {
            label: node_label(node),
            value: node.value().unwrap_or("").to_string(),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_list_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ListBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::ListBox {
            label: node_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_slider(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SliderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Slider {
            label: node_label(node),
            value: node.numeric_value(),
            min: node.min_numeric_value(),
            max: node.max_numeric_value(),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_spin_button(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SpinButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::SpinButton {
            label: node_label(node),
            value: node.numeric_value(),
            min: node.min_numeric_value(),
            max: node.max_numeric_value(),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_progress_indicator(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ProgressIndicatorNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Progress {
            label: opt_label(node),
            value: node.numeric_value(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_scroll_view(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ScrollViewNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::ScrollView {
            label: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_splitter(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SplitterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "splitter",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    // ── Text inputs ───────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::TextInput {
            label: node_label(node),
            value: node.value().unwrap_or("").to_string(),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<MultilineTextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::TextArea {
            label: node_label(node),
            value: node.value().unwrap_or("").to_string(),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    // ── Text / inline content ─────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TextRunNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_paragraph(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ParagraphNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_label(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<LabelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_heading(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<HeadingNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: node.level().map(|l| l as u8),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_line_break(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<LineBreakNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: String::new(),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_blockquote(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<BlockquoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "blockquote",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_code(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<CodeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_math(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<MathNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_note(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<NoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("note", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_term(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TermNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_definition(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<DefinitionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    // ── Media / embedded ─────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_image(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ImageNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Image {
            alt: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_figure(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<FigureNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("figure", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_figure_caption(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<FigureCaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_canvas(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<CanvasNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("canvas", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_video(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<VideoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("video", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_audio(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<AudioNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("audio", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    // ── Landmark regions ──────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_main(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<MainNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("main", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_navigation(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<NavigationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "navigation",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_banner(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<BannerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("banner", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_content_info(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ContentInfoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "content_info",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_complementary(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ComplementaryNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "complementary",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_form(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<FormNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("form", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_search(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SearchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("search", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_region(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<RegionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("region", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_section(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SectionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "section",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_section_header(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SectionHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "section_header",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_section_footer(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<SectionFooterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "section_footer",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_article(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ArticleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "article",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_group(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<GroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("group", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    // ── Dialogs / overlays ────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_dialog(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<DialogNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("dialog", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_details(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<DetailsNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "details",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_tooltip(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TooltipNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "tooltip",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    // ── Status / live regions ─────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_alert(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<AlertNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("alert", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_status(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<StatusNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("status", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_timer(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TimerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("timer", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("list", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_list_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ListItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_description_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<DescriptionListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "description_list",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    // ── Tables / grids ────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_table(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TableNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Table {
            label: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_row(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<RowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container("row", opt_label(node), unwrap_children(children), proofs);
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_cell(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<CellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_caption(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<CaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Text {
            content: node_label(node),
            level: None,
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_row_group(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<RowGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "row_group",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    // ── Tree ──────────────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_tree(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TreeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Tree {
            label: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_tree_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TreeItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::TreeItem {
            label: node_label(node),
            expanded: node.is_expanded().unwrap_or(false),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    // ── Tabs ──────────────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_tab(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TabNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Tab {
            label: node_label(node),
            selected: node.is_selected().unwrap_or(false),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_tab_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TabListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "tab_list",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_tab_panel(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<TabPanelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::TabPanel {
            label: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    // ── Menus ─────────────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_menu(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<MenuNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::Menu {
            label: opt_label(node),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_menu_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<MenuItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = BevyUiNode::MenuItem {
            label: node_label(node),
            disabled: node.is_disabled(),
            children: unwrap_children(children),
            proofs,
        };
        (w, prove_render(proof, proofs))
    }

    // ── Toolbar ───────────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_toolbar(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<ToolbarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "toolbar",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }

    // ── Radio group ───────────────────────────────────────────────────────

    #[instrument(skip(self, children, proof, proofs), fields(role = ?node.role()))]
    fn bridge_radio_group(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(BevyUiNode, Established<RolePreserved>)>,
        proof: Established<RadioGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (BevyUiNode, Established<RolePreserved>) {
        let w = container(
            "radio_group",
            opt_label(node),
            unwrap_children(children),
            proofs,
        );
        (w, prove_render(proof, proofs))
    }
}
