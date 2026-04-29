//! Validation functions that establish proofs for UI accessibility propositions.
//!
//! These functions delegate to the [`Constraint`](crate::constraints::Constraint) trait
//! implementations in [`constraints::wcag`](crate::constraints::wcag), preserving backward
//! compatibility with the existing API.

use crate::constraints::{
    Constraint, ConstraintContext, HasLabelConstraint, KeyboardAccessibleConstraint,
    MinTouchTargetConstraint, NoOverflowConstraint, ValidRoleConstraint,
};
use crate::{VerificationError, Viewport};
use accesskit::NodeId;
use std::collections::BTreeMap;

/// Convert a constraint [`Violation`](crate::constraints::Violation) to a legacy
/// [`VerificationError`].
fn violation_to_error(v: crate::constraints::Violation) -> VerificationError {
    match v {
        crate::constraints::Violation::MissingLabel { element } => {
            VerificationError::missing_label(element)
        }
        crate::constraints::Violation::EmptyLabel { element } => {
            VerificationError::empty_label(element)
        }
        crate::constraints::Violation::TouchTarget {
            element,
            actual_width,
            actual_height,
            ..
        } => VerificationError::below_min_target_size(element, actual_width, actual_height),
        crate::constraints::Violation::Overflow {
            element,
            element_x,
            element_y,
            element_width,
            element_height,
            viewport_width,
            viewport_height,
        } => VerificationError::overflows_viewport(
            element,
            element_x,
            element_y,
            element_width,
            element_height,
            viewport_width,
            viewport_height,
        ),
        // Other violation types don't have legacy equivalents
        _ => VerificationError::tree_error("constraint violation"),
    }
}

/// Validate that an element has a non-empty accessible label.
///
/// Delegates to [`HasLabelConstraint`].
pub fn validate_has_label(
    nodes: &BTreeMap<NodeId, accesskit::Node>,
    node_id: NodeId,
) -> Result<(), VerificationError> {
    let ctx = ConstraintContext {
        nodes,
        viewport: Viewport::new(0, 0),
    };
    HasLabelConstraint
        .check(node_id, &ctx)
        .map_err(violation_to_error)
}

/// Validate that an element has a valid ARIA role.
///
/// Delegates to [`ValidRoleConstraint`].
pub fn validate_valid_role(
    nodes: &BTreeMap<NodeId, accesskit::Node>,
    node_id: NodeId,
) -> Result<(), VerificationError> {
    let ctx = ConstraintContext {
        nodes,
        viewport: Viewport::new(0, 0),
    };
    ValidRoleConstraint
        .check(node_id, &ctx)
        .map_err(violation_to_error)
}

/// Validate that an interactive element meets minimum touch target size (44x44).
///
/// Delegates to [`MinTouchTargetConstraint`].
pub fn validate_min_target_size(
    nodes: &BTreeMap<NodeId, accesskit::Node>,
    node_id: NodeId,
) -> Result<(), VerificationError> {
    let ctx = ConstraintContext {
        nodes,
        viewport: Viewport::new(0, 0),
    };
    MinTouchTargetConstraint
        .check(node_id, &ctx)
        .map_err(violation_to_error)
}

/// Validate that an element fits within viewport boundaries.
///
/// Delegates to [`NoOverflowConstraint`].
pub fn validate_no_overflow(
    nodes: &BTreeMap<NodeId, accesskit::Node>,
    node_id: NodeId,
    viewport: Viewport,
) -> Result<(), VerificationError> {
    let ctx = ConstraintContext { nodes, viewport };
    NoOverflowConstraint
        .check(node_id, &ctx)
        .map_err(violation_to_error)
}

/// Validate that an element is keyboard accessible.
///
/// Delegates to [`KeyboardAccessibleConstraint`].
pub fn validate_keyboard_accessible(
    nodes: &BTreeMap<NodeId, accesskit::Node>,
    node_id: NodeId,
) -> Result<(), VerificationError> {
    let ctx = ConstraintContext {
        nodes,
        viewport: Viewport::new(0, 0),
    };
    KeyboardAccessibleConstraint
        .check(node_id, &ctx)
        .map_err(violation_to_error)
}
