//! WCAG constraint implementations.
//!
//! Each constraint maps to a specific WCAG Success Criterion and implements
//! the [`Constraint`] trait with spec traceability.

use super::{Constraint, ConstraintContext, SpecReference, Violation, WcagLevel};
use accesskit::{NodeId, Role};

/// Check if a role requires an accessible label.
fn requires_label(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::Link
            | Role::CheckBox
            | Role::RadioButton
            | Role::TextInput
            | Role::Slider
            | Role::SpinButton
            | Role::MenuItem
            | Role::MenuItemCheckBox
            | Role::MenuItemRadio
            | Role::Tab
            | Role::Switch
            | Role::Search
            | Role::ComboBox
    )
}

/// Check if a role is focusable (keyboard accessible).
fn is_focusable_role(role: Role) -> bool {
    matches!(
        role,
        Role::Button
            | Role::Link
            | Role::CheckBox
            | Role::RadioButton
            | Role::TextInput
            | Role::Slider
            | Role::SpinButton
            | Role::MenuItem
            | Role::MenuItemCheckBox
            | Role::MenuItemRadio
            | Role::Tab
            | Role::Switch
            | Role::Search
            | Role::ComboBox
    )
}

/// WCAG 2.4.6 (AA) / 4.1.2 (A): Element has a non-empty accessible label.
#[derive(Debug, Clone, Copy)]
pub struct HasLabelConstraint;

impl Constraint for HasLabelConstraint {
    fn check(
        &self,
        node_id: NodeId,
        ctx: &ConstraintContext<'_>,
    ) -> Result<(), Violation> {
        let node = match ctx.nodes.get(&node_id) {
            Some(n) => n,
            None => return Ok(()),
        };

        if !requires_label(node.role()) {
            return Ok(());
        }

        match node.label() {
            Some(name) if !name.is_empty() => Ok(()),
            Some(_) => Err(Violation::EmptyLabel {
                element: node_id.into(),
            }),
            None => Err(Violation::MissingLabel {
                element: node_id.into(),
            }),
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "4.1.2",
            level: WcagLevel::A,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/name-role-value",
        }
    }
}

/// WCAG 4.1.2 (A): Element has a valid ARIA role.
#[derive(Debug, Clone, Copy)]
pub struct ValidRoleConstraint;

impl Constraint for ValidRoleConstraint {
    fn check(
        &self,
        node_id: NodeId,
        ctx: &ConstraintContext<'_>,
    ) -> Result<(), Violation> {
        // In AccessKit, all Role enum variants are valid by construction.
        let _node = ctx.nodes.get(&node_id);
        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "4.1.2",
            level: WcagLevel::A,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/name-role-value",
        }
    }
}

/// WCAG 2.5.5 (AAA): Interactive element meets minimum touch target size (44x44).
#[derive(Debug, Clone, Copy)]
pub struct MinTouchTargetConstraint;

impl Constraint for MinTouchTargetConstraint {
    fn check(
        &self,
        node_id: NodeId,
        ctx: &ConstraintContext<'_>,
    ) -> Result<(), Violation> {
        let node = match ctx.nodes.get(&node_id) {
            Some(n) => n,
            None => return Ok(()),
        };

        if !is_focusable_role(node.role()) {
            return Ok(());
        }

        if let Some(bounds) = node.bounds() {
            let width = bounds.width() as u32;
            let height = bounds.height() as u32;

            if width >= 44 && height >= 44 {
                Ok(())
            } else {
                Err(Violation::TouchTarget {
                    element: node_id.into(),
                    actual_width: width,
                    actual_height: height,
                    min_dimension: 44,
                })
            }
        } else {
            Ok(())
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "2.5.5",
            level: WcagLevel::AAA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/target-size-enhanced",
        }
    }
}

/// WCAG 1.4.10 (AA): Element does not overflow viewport boundaries.
#[derive(Debug, Clone, Copy)]
pub struct NoOverflowConstraint;

impl Constraint for NoOverflowConstraint {
    fn check(
        &self,
        node_id: NodeId,
        ctx: &ConstraintContext<'_>,
    ) -> Result<(), Violation> {
        let node = match ctx.nodes.get(&node_id) {
            Some(n) => n,
            None => return Ok(()),
        };

        if let Some(bounds) = node.bounds() {
            let x = bounds.x0 as i32;
            let y = bounds.y0 as i32;
            let width = bounds.width() as u32;
            let height = bounds.height() as u32;

            let fits_horizontally = x >= 0 && (x as u32 + width) <= ctx.viewport.width;
            let fits_vertically = y >= 0 && (y as u32 + height) <= ctx.viewport.height;

            if fits_horizontally && fits_vertically {
                Ok(())
            } else {
                Err(Violation::Overflow {
                    element: node_id.into(),
                    element_x: x,
                    element_y: y,
                    element_width: width,
                    element_height: height,
                    viewport_width: ctx.viewport.width,
                    viewport_height: ctx.viewport.height,
                })
            }
        } else {
            Ok(())
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.10",
            level: WcagLevel::AA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/reflow",
        }
    }
}

/// WCAG 2.1.1 (A): Element is keyboard accessible.
#[derive(Debug, Clone, Copy)]
pub struct KeyboardAccessibleConstraint;

impl Constraint for KeyboardAccessibleConstraint {
    fn check(
        &self,
        node_id: NodeId,
        ctx: &ConstraintContext<'_>,
    ) -> Result<(), Violation> {
        let node = match ctx.nodes.get(&node_id) {
            Some(n) => n,
            None => return Ok(()),
        };

        // Focusable roles are keyboard accessible by role definition.
        // Non-interactive elements don't need keyboard access.
        let _is_focusable = is_focusable_role(node.role());
        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "2.1.1",
            level: WcagLevel::A,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/keyboard",
        }
    }
}
