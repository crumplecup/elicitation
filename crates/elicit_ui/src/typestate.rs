//! Typestate state machine for verified UI construction.

use crate::{validators, VerificationReport, Viewport};
use accesskit::{Node, NodeId, TreeUpdate};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Typestate marker: Layout awaiting verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pending;

/// Typestate marker: Layout verified against WCAG constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verified;

/// Typestate marker: Layout rendered to a specific frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rendered;

/// UI layout with typestate tracking.
///
/// State transitions:
/// - `Layout<Pending>` — awaiting verification
/// - `Layout<Verified>` — verified against WCAG Level AA
/// - `Layout<Rendered>` — rendered to frontend
#[derive(Debug, Clone)]
pub struct Layout<State> {
    nodes: HashMap<NodeId, Node>,
    root: NodeId,
    viewport: Option<Viewport>,
    report: Option<VerificationReport>,
    _state: PhantomData<State>,
}

impl Layout<Pending> {
    /// Create a new pending layout from an AccessKit TreeUpdate.
    pub fn from_update(update: TreeUpdate) -> Self {
        let nodes: HashMap<NodeId, Node> = update.nodes.into_iter().collect();
        let root = update.focus;

        Self {
            nodes,
            root,
            viewport: None,
            report: None,
            _state: PhantomData,
        }
    }

    /// Verify the layout against WCAG Level A constraints.
    ///
    /// Checks:
    /// - HasLabel (WCAG 2.4.6 Level AA, 4.1.2 Level A)
    /// - ValidRole (WCAG 4.1.2 Level A)
    /// - KeyboardAccessible (WCAG 2.1.1 Level A)
    #[tracing::instrument(skip(self), fields(root = ?self.root))]
    pub fn verify_a(
        self,
        viewport: Viewport,
    ) -> Result<Layout<Verified>, VerificationReport> {
        tracing::debug!("Verifying layout against WCAG Level A");
        let mut report = VerificationReport::new();

        // Walk the tree and validate each node
        self.validate_tree_recursive(self.root, &viewport, &mut report, false);

        if report.has_errors() {
            tracing::error!(error_count = report.error_count(), "Verification failed");
            Err(report)
        } else {
            tracing::info!("Verification successful");
            Ok(Layout {
                nodes: self.nodes,
                root: self.root,
                viewport: Some(viewport),
                report: Some(report),
                _state: PhantomData,
            })
        }
    }

    /// Verify the layout against WCAG Level AA constraints.
    ///
    /// Checks all Level A constraints plus:
    /// - NoOverflow (WCAG 1.4.10 Level AA)
    #[tracing::instrument(skip(self), fields(root = ?self.root))]
    pub fn verify_aa(
        self,
        viewport: Viewport,
    ) -> Result<Layout<Verified>, VerificationReport> {
        tracing::debug!("Verifying layout against WCAG Level AA");
        let mut report = VerificationReport::new();

        // Walk the tree and validate each node (includes NoOverflow)
        self.validate_tree_recursive(self.root, &viewport, &mut report, false);

        if report.has_errors() {
            tracing::error!(error_count = report.error_count(), "Verification failed");
            Err(report)
        } else {
            tracing::info!("Verification successful");
            Ok(Layout {
                nodes: self.nodes,
                root: self.root,
                viewport: Some(viewport),
                report: Some(report),
                _state: PhantomData,
            })
        }
    }

    /// Verify the layout against WCAG Level AAA constraints.
    ///
    /// Checks all Level AA constraints plus:
    /// - MinTargetSize (WCAG 2.5.5 Level AAA)
    #[tracing::instrument(skip(self), fields(root = ?self.root))]
    pub fn verify_aaa(
        self,
        viewport: Viewport,
    ) -> Result<Layout<Verified>, VerificationReport> {
        tracing::debug!("Verifying layout against WCAG Level AAA");
        let mut report = VerificationReport::new();

        // Walk the tree and validate each node (includes MinTargetSize)
        self.validate_tree_recursive(self.root, &viewport, &mut report, true);

        if report.has_errors() {
            tracing::error!(error_count = report.error_count(), "Verification failed");
            Err(report)
        } else {
            tracing::info!("Verification successful");
            Ok(Layout {
                nodes: self.nodes,
                root: self.root,
                viewport: Some(viewport),
                report: Some(report),
                _state: PhantomData,
            })
        }
    }

    /// Recursively validate tree nodes.
    fn validate_tree_recursive(
        &self,
        node_id: NodeId,
        viewport: &Viewport,
        report: &mut VerificationReport,
        check_aaa: bool,
    ) {
        // Validate current node
        if let Err(e) = validators::validate_has_label(&self.nodes, node_id) {
            report.add_error(e);
        }
        if let Err(e) = validators::validate_valid_role(&self.nodes, node_id) {
            report.add_error(e);
        }
        if let Err(e) = validators::validate_keyboard_accessible(&self.nodes, node_id) {
            report.add_error(e);
        }
        if let Err(e) = validators::validate_no_overflow(&self.nodes, node_id, *viewport) {
            report.add_error(e);
        }

        // Check AAA-level constraints if requested
        if check_aaa
            && let Err(e) = validators::validate_min_target_size(&self.nodes, node_id)
        {
            report.add_error(e);
        }

        // Recursively validate children
        if let Some(node) = self.nodes.get(&node_id) {
            for child_id in node.children() {
                self.validate_tree_recursive(*child_id, viewport, report, check_aaa);
            }
        }
    }
}

impl Layout<Verified> {
    /// Get the root node ID.
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Get the viewport dimensions.
    pub fn viewport(&self) -> Viewport {
        self.viewport.expect("Verified layout must have viewport")
    }

    /// Get the verification report.
    pub fn report(&self) -> &VerificationReport {
        self.report
            .as_ref()
            .expect("Verified layout must have report")
    }

    /// Render the layout to egui.
    ///
    /// Walks the verified AccessKit tree and renders each node to
    /// the corresponding egui widget.
    ///
    /// Available when `egui-backend` feature is enabled.
    #[cfg(feature = "egui-backend")]
    #[tracing::instrument(skip(self, ctx), fields(root = ?self.root))]
    pub fn render_egui(self, ctx: &egui::Context) -> (Layout<Rendered>, crate::RenderStats) {
        tracing::debug!("Rendering layout to egui");

        let mut stats = crate::RenderStats::default();
        let nodes_ref = &self.nodes;
        let root = self.root;

        let _output = ctx.run_ui(egui::RawInput::default(), |ui| {
            stats = crate::renderer::render_tree(ui, nodes_ref, root);
        });

        let layout = Layout {
            nodes: self.nodes,
            root: self.root,
            viewport: self.viewport,
            report: self.report,
            _state: PhantomData,
        };

        (layout, stats)
    }
}

impl Layout<Rendered> {
    /// Get the root node ID.
    pub fn root(&self) -> NodeId {
        self.root
    }
}
