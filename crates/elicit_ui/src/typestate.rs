//! Typestate state machine for verified UI construction.

use crate::constraints::{
    ConstraintContext, ConstraintSet, ConstraintSetBuilder, ConstraintVerification,
    HasLabelConstraint, KeyboardAccessibleConstraint, MinTouchTargetConstraint,
    NoOverflowConstraint, ValidRoleConstraint,
};
use crate::{VerificationReport, Viewport, validators};
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
    pub fn verify_a(self, viewport: Viewport) -> Result<Layout<Verified>, VerificationReport> {
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
    pub fn verify_aa(self, viewport: Viewport) -> Result<Layout<Verified>, VerificationReport> {
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
    pub fn verify_aaa(self, viewport: Viewport) -> Result<Layout<Verified>, VerificationReport> {
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

    /// Verify the layout using a [`ConstraintProfile`].
    ///
    /// This is the composable API that uses the constraint system directly.
    /// The `verify_a`, `verify_aa`, and `verify_aaa` methods are sugar over
    /// pre-built profiles.
    #[tracing::instrument(skip(self), fields(root = ?self.root, profile = ?profile))]
    pub fn verify_with_profile(
        self,
        viewport: Viewport,
        profile: ConstraintProfile,
    ) -> Result<Layout<Verified>, ConstraintVerification> {
        tracing::debug!(?profile, "Verifying layout with constraint profile");

        let constraint_set = profile.to_constraint_set();
        let ctx = ConstraintContext {
            nodes: &self.nodes,
            viewport,
        };

        let verification = constraint_set.verify(self.root, &ctx);

        if verification.is_valid() {
            tracing::info!("Constraint verification successful");
            Ok(Layout {
                nodes: self.nodes,
                root: self.root,
                viewport: Some(viewport),
                report: None,
                _state: PhantomData,
            })
        } else {
            tracing::error!(
                hard = verification.hard_violations.len(),
                structural = verification.structural_violations.len(),
                "Constraint verification failed"
            );
            Err(verification)
        }
    }

    /// Verify with a custom [`ConstraintSet`].
    ///
    /// For maximum flexibility — bring your own constraints.
    #[tracing::instrument(skip(self, constraint_set), fields(root = ?self.root))]
    pub fn verify_custom(
        self,
        viewport: Viewport,
        constraint_set: &ConstraintSet,
    ) -> Result<Layout<Verified>, ConstraintVerification> {
        tracing::debug!("Verifying layout with custom constraint set");

        let ctx = ConstraintContext {
            nodes: &self.nodes,
            viewport,
        };

        let verification = constraint_set.verify(self.root, &ctx);

        if verification.is_valid() {
            tracing::info!("Custom constraint verification successful");
            Ok(Layout {
                nodes: self.nodes,
                root: self.root,
                viewport: Some(viewport),
                report: None,
                _state: PhantomData,
            })
        } else {
            tracing::error!(
                hard = verification.hard_violations.len(),
                "Custom constraint verification failed"
            );
            Err(verification)
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
        if check_aaa && let Err(e) = validators::validate_min_target_size(&self.nodes, node_id) {
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

    /// Get a reference to the node map.
    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    /// Extract a [`VerifiedTree`] snapshot for rendering.
    ///
    /// Converts the typestate layout into a plain data snapshot that
    /// can be passed to any [`crate::UiRenderer`] backend.
    pub fn into_verified_tree(self) -> crate::VerifiedTree {
        crate::VerifiedTree {
            nodes: self.nodes,
            root: self.root,
            viewport: self.viewport.unwrap_or(Viewport::new(1920, 1080)),
        }
    }

    /// Render the verified layout through a [`crate::UiRenderer`].
    ///
    /// This is the generic render path. Frontend crates provide their
    /// own [`crate::UiRenderer`] implementation (e.g., `EguiBackend`,
    /// `RatatuiBackend`).
    #[tracing::instrument(skip(self, backend), fields(root = ?self.root))]
    pub fn render<B: crate::UiRenderer>(
        self,
        backend: &B,
    ) -> Result<(Layout<Rendered>, crate::RenderStats), crate::UiError> {
        tracing::debug!("Rendering layout via backend");

        let tree = crate::VerifiedTree {
            nodes: self.nodes.clone(),
            root: self.root,
            viewport: self.viewport.unwrap_or(Viewport::new(1920, 1080)),
        };
        let (stats, _proof) = backend.render(&tree)?;

        let layout = Layout {
            nodes: self.nodes,
            root: self.root,
            viewport: self.viewport,
            report: self.report,
            _state: PhantomData,
        };

        Ok((layout, stats))
    }
}

impl Layout<Rendered> {
    /// Get the root node ID.
    pub fn root(&self) -> NodeId {
        self.root
    }
}

/// Pre-built constraint profiles for common verification scenarios.
///
/// Each profile maps to a specific set of constraints from the constraint system.
/// Use `verify_with_profile` on `Layout<Pending>` to apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintProfile {
    /// WCAG Level A: label, role, keyboard access.
    WcagA,
    /// WCAG Level AA: Level A + no overflow.
    WcagAA,
    /// WCAG Level AAA: Level AA + min touch target.
    WcagAAA,
}

impl ConstraintProfile {
    /// Convert this profile to a [`ConstraintSet`].
    pub fn to_constraint_set(&self) -> ConstraintSet {
        match self {
            Self::WcagA => ConstraintSetBuilder::default()
                .hard(HasLabelConstraint)
                .hard(ValidRoleConstraint)
                .hard(KeyboardAccessibleConstraint)
                .build(),
            Self::WcagAA => ConstraintSetBuilder::default()
                .hard(HasLabelConstraint)
                .hard(ValidRoleConstraint)
                .hard(KeyboardAccessibleConstraint)
                .hard(NoOverflowConstraint)
                .build(),
            Self::WcagAAA => ConstraintSetBuilder::default()
                .hard(HasLabelConstraint)
                .hard(ValidRoleConstraint)
                .hard(KeyboardAccessibleConstraint)
                .hard(NoOverflowConstraint)
                .hard(MinTouchTargetConstraint)
                .build(),
        }
    }
}
