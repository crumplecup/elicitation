//! Composable, spec-backed constraint system for UI layout verification.
//!
//! Constraints are organized into enforcement tiers:
//! - **Hard**: Must pass for layout validity (e.g., WCAG Level AA)
//! - **Structural**: Compile-time or type-level guarantees (e.g., CSS unit semantics)
//! - **Advisory**: Warnings, not errors (e.g., design system heuristics)

mod spatial;
mod wcag;

pub use spatial::{GridAlignment, MinSpacing, Reflow320, ResizeText200, TextSpacing};
pub use wcag::{
    HasLabelConstraint, KeyboardAccessibleConstraint, MinTouchTargetConstraint,
    NoOverflowConstraint, ValidRoleConstraint,
};

use crate::{ElementId, Viewport};
use accesskit::NodeId;
use std::collections::HashMap;

/// External specification reference for constraint traceability.
///
/// Every constraint is anchored to a recognized standard — no arbitrary decisions.
#[derive(Debug, Clone)]
pub enum SpecReference {
    /// Web Content Accessibility Guidelines criterion.
    Wcag {
        /// WCAG success criterion number (e.g., "1.4.3").
        criterion: &'static str,
        /// Conformance level.
        level: WcagLevel,
        /// URL to the understanding document.
        url: &'static str,
    },
    /// CSS specification module.
    Css {
        /// Module name (e.g., "CSS Values and Units Module Level 3").
        module: &'static str,
        /// Section reference.
        section: &'static str,
        /// URL to the specification.
        url: &'static str,
    },
    /// Design system guideline.
    DesignSystem {
        /// Design system name (e.g., "Material Design 3").
        name: &'static str,
        /// Section reference.
        section: &'static str,
        /// URL to the guideline.
        url: &'static str,
    },
    /// ISO standard.
    Iso {
        /// Standard identifier (e.g., "ISO 9241-3").
        standard: &'static str,
        /// Section reference.
        section: &'static str,
    },
}

/// WCAG conformance level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WcagLevel {
    /// Level A (minimum).
    A,
    /// Level AA (recommended).
    AA,
    /// Level AAA (enhanced).
    AAA,
}

impl std::fmt::Display for WcagLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::AA => write!(f, "AA"),
            Self::AAA => write!(f, "AAA"),
        }
    }
}

/// Spec-traceable constraint violation.
#[derive(Debug, Clone)]
pub enum Violation {
    /// Element missing required accessible label.
    MissingLabel {
        /// Element that lacks a label.
        element: ElementId,
    },
    /// Element label is empty.
    EmptyLabel {
        /// Element with empty label.
        element: ElementId,
    },
    /// Interactive element below minimum touch target size.
    TouchTarget {
        /// Element that is too small.
        element: ElementId,
        /// Actual width in pixels.
        actual_width: u32,
        /// Actual height in pixels.
        actual_height: u32,
        /// Minimum dimension required.
        min_dimension: u32,
    },
    /// Element overflows viewport boundaries.
    Overflow {
        /// Element that overflows.
        element: ElementId,
        /// Element position and size.
        element_x: i32,
        /// Element Y position.
        element_y: i32,
        /// Element width.
        element_width: u32,
        /// Element height.
        element_height: u32,
        /// Viewport width.
        viewport_width: u32,
        /// Viewport height.
        viewport_height: u32,
    },
    /// Insufficient color contrast ratio.
    Contrast {
        /// Element with insufficient contrast.
        element: ElementId,
        /// Actual contrast ratio.
        actual_ratio: f64,
        /// Minimum required ratio.
        min_ratio: f64,
    },
    /// Color contrast insufficient (from color_contrast module).
    ContrastInsufficient {
        /// Actual contrast ratio achieved.
        actual: f32,
        /// Minimum ratio required by the spec.
        required: f32,
        /// Foreground color as hex string.
        foreground: String,
        /// Background color as hex string.
        background: String,
    },
    /// Text spacing causes overlap after WCAG adjustments.
    TextSpacing {
        /// First overlapping element.
        element1: ElementId,
        /// Second overlapping element.
        element2: ElementId,
    },
    /// Content does not reflow at 320px width.
    Reflow {
        /// Element that does not reflow.
        element: ElementId,
        /// Actual width.
        actual_width: f64,
        /// Maximum allowed width.
        max_width: f64,
    },
    /// Element not aligned to design grid.
    GridAlignment {
        /// Misaligned element.
        element: ElementId,
        /// Actual position.
        position: (f64, f64),
        /// Grid step size.
        grid_step: f64,
    },
}

/// Layout context passed to constraints during verification.
#[derive(Debug, Clone)]
pub struct ConstraintContext<'a> {
    /// All nodes in the AccessKit tree.
    pub nodes: &'a HashMap<NodeId, accesskit::Node>,
    /// Viewport dimensions.
    pub viewport: Viewport,
}

/// A composable, spec-backed constraint on UI layout.
///
/// Every constraint is traceable to a recognized external standard.
pub trait Constraint: Send + Sync {
    /// Check this constraint against a single node.
    ///
    /// Returns `Ok(())` if the constraint is satisfied, or a spec-traceable
    /// `Violation` describing the failure.
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation>;

    /// External specification reference for this constraint.
    fn spec_ref(&self) -> SpecReference;
}

/// Composable constraint set with enforcement tiers.
pub struct ConstraintSet {
    /// Hard constraints: must pass for layout validity.
    hard: Vec<Box<dyn Constraint>>,
    /// Structural constraints: type-level or compile-time guarantees.
    structural: Vec<Box<dyn Constraint>>,
    /// Advisory constraints: warnings, not errors.
    advisory: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    /// Create a new builder for constructing a constraint set.
    pub fn builder() -> ConstraintSetBuilder {
        ConstraintSetBuilder::default()
    }

    /// Verify all constraints against the given context, walking the tree recursively.
    ///
    /// Returns hard violations as errors. Advisory violations are collected as warnings.
    #[tracing::instrument(skip(self, ctx))]
    pub fn verify(&self, root: NodeId, ctx: &ConstraintContext<'_>) -> ConstraintVerification {
        let mut hard_violations = Vec::new();
        let mut structural_violations = Vec::new();
        let mut warnings = Vec::new();

        self.verify_recursive(
            root,
            ctx,
            &mut hard_violations,
            &mut structural_violations,
            &mut warnings,
        );

        ConstraintVerification {
            hard_violations,
            structural_violations,
            warnings,
        }
    }

    fn verify_recursive(
        &self,
        node_id: NodeId,
        ctx: &ConstraintContext<'_>,
        hard_violations: &mut Vec<Violation>,
        structural_violations: &mut Vec<Violation>,
        warnings: &mut Vec<Violation>,
    ) {
        for constraint in &self.hard {
            if let Err(v) = constraint.check(node_id, ctx) {
                hard_violations.push(v);
            }
        }
        for constraint in &self.structural {
            if let Err(v) = constraint.check(node_id, ctx) {
                structural_violations.push(v);
            }
        }
        for constraint in &self.advisory {
            if let Err(v) = constraint.check(node_id, ctx) {
                warnings.push(v);
            }
        }

        if let Some(node) = ctx.nodes.get(&node_id) {
            for child_id in node.children() {
                self.verify_recursive(
                    *child_id,
                    ctx,
                    hard_violations,
                    structural_violations,
                    warnings,
                );
            }
        }
    }

    /// Get the hard constraints.
    pub fn hard_constraints(&self) -> &[Box<dyn Constraint>] {
        &self.hard
    }

    /// Get the structural constraints.
    pub fn structural_constraints(&self) -> &[Box<dyn Constraint>] {
        &self.structural
    }

    /// Get the advisory constraints.
    pub fn advisory_constraints(&self) -> &[Box<dyn Constraint>] {
        &self.advisory
    }
}

/// Result of constraint verification.
#[derive(Debug, Clone, Default)]
pub struct ConstraintVerification {
    /// Hard constraint violations (layout is invalid).
    pub hard_violations: Vec<Violation>,
    /// Structural constraint violations.
    pub structural_violations: Vec<Violation>,
    /// Advisory warnings (informational).
    pub warnings: Vec<Violation>,
}

impl ConstraintVerification {
    /// Whether the layout passes all hard constraints.
    pub fn is_valid(&self) -> bool {
        self.hard_violations.is_empty() && self.structural_violations.is_empty()
    }

    /// Total number of violations across all tiers.
    pub fn violation_count(&self) -> usize {
        self.hard_violations.len() + self.structural_violations.len()
    }

    /// Total number of advisory warnings.
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

/// Builder for constructing a [`ConstraintSet`] with enforcement tiers.
#[derive(Default)]
pub struct ConstraintSetBuilder {
    hard: Vec<Box<dyn Constraint>>,
    structural: Vec<Box<dyn Constraint>>,
    advisory: Vec<Box<dyn Constraint>>,
}

impl ConstraintSetBuilder {
    /// Add a hard constraint (must pass for validity).
    pub fn hard(mut self, constraint: impl Constraint + 'static) -> Self {
        self.hard.push(Box::new(constraint));
        self
    }

    /// Add a structural constraint (compile-time guarantee).
    pub fn structural(mut self, constraint: impl Constraint + 'static) -> Self {
        self.structural.push(Box::new(constraint));
        self
    }

    /// Add an advisory constraint (warning, not error).
    pub fn advisory(mut self, constraint: impl Constraint + 'static) -> Self {
        self.advisory.push(Box::new(constraint));
        self
    }

    /// Build the constraint set.
    pub fn build(self) -> ConstraintSet {
        ConstraintSet {
            hard: self.hard,
            structural: self.structural,
            advisory: self.advisory,
        }
    }
}
