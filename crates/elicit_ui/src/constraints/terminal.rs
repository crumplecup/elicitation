//! Terminal-specific constraint implementations.
//!
//! Constraints for cell-based terminal UIs where dimensions are measured
//! in character cells (columns × rows) rather than pixels.
//!
//! - [`TerminalNoOverflow`] — WCAG 1.4.10 adapted for cell viewports
//! - [`MinReadableSize`] — minimum pane dimensions for usability
//! - [`TerminalAccessible`] — convenience builder combining terminal + WCAG constraints

use super::{
    Constraint, ConstraintContext, ConstraintSet, ConstraintSetBuilder, SpecReference, Violation,
    WcagLevel,
};
use super::{HasLabelConstraint, KeyboardAccessibleConstraint, ValidRoleConstraint};
use accesskit::{NodeId, Role};

/// Container-like roles whose children contribute to layout size.
fn is_container_role(role: Role) -> bool {
    matches!(
        role,
        Role::Window
            | Role::Group
            | Role::GenericContainer
            | Role::List
            | Role::Table
            | Role::TabList
            | Role::MenuBar
            | Role::Menu
            | Role::Toolbar
            | Role::Dialog
            | Role::Application
            | Role::Form
            | Role::Grid
            | Role::TreeGrid
            | Role::Tree
    )
}

/// WCAG 1.4.10 (Reflow) adapted for cell-based terminal viewports.
///
/// Checks that each node's bounds fit within the terminal viewport measured
/// in columns × rows. Cell geometry: 1 cell = 1 unit in AccessKit bounds.
///
/// Unlike the pixel-based [`super::NoOverflowConstraint`], this constraint
/// uses the viewport as a cell grid (e.g., 80×24 for VT100).
#[derive(Debug, Clone, Copy)]
pub struct TerminalNoOverflow;

impl Constraint for TerminalNoOverflow {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let node = match ctx.nodes.get(&node_id) {
            Some(n) => n,
            None => return Ok(()),
        };

        if let Some(bounds) = node.bounds() {
            let col = bounds.x0 as i32;
            let row = bounds.y0 as i32;
            let width = bounds.width() as u32;
            let height = bounds.height() as u32;

            // Viewport dimensions are terminal cells (cols × rows)
            let fits_cols = col >= 0 && (col as u32 + width) <= ctx.viewport.width;
            let fits_rows = row >= 0 && (row as u32 + height) <= ctx.viewport.height;

            if fits_cols && fits_rows {
                Ok(())
            } else {
                Err(Violation::TerminalOverflow {
                    element: crate::ElementId::from(node_id),
                    element_col: col,
                    element_row: row,
                    element_cols: width,
                    element_rows: height,
                    viewport_cols: ctx.viewport.width,
                    viewport_rows: ctx.viewport.height,
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

/// Minimum readable size constraint for terminal panes.
///
/// Ensures every container pane has at least `min_rows` × `min_cols` to be
/// usable. Default thresholds: 3 rows × 10 columns (enough for a label,
/// a one-line input, and a status line).
///
/// Anchored to ISO 9241-3 (visual display requirements) and common usability
/// heuristics for terminal applications.
#[derive(Debug, Clone, Copy)]
pub struct MinReadableSize {
    /// Minimum columns required for a pane.
    pub min_cols: u32,
    /// Minimum rows required for a pane.
    pub min_rows: u32,
}

impl Default for MinReadableSize {
    fn default() -> Self {
        Self {
            min_cols: 10,
            min_rows: 3,
        }
    }
}

impl Constraint for MinReadableSize {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let node = match ctx.nodes.get(&node_id) {
            Some(n) => n,
            None => return Ok(()),
        };

        // Only check container-like roles (panes, groups, windows)
        if !is_container_role(node.role()) {
            return Ok(());
        }

        if let Some(bounds) = node.bounds() {
            let cols = bounds.width() as u32;
            let rows = bounds.height() as u32;

            if cols >= self.min_cols && rows >= self.min_rows {
                Ok(())
            } else {
                Err(Violation::BelowMinReadableSize {
                    element: crate::ElementId::from(node_id),
                    actual_cols: cols,
                    actual_rows: rows,
                    min_cols: self.min_cols,
                    min_rows: self.min_rows,
                })
            }
        } else {
            Ok(())
        }
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Iso {
            standard: "ISO 9241-3",
            section: "Visual display requirements — minimum readable area",
        }
    }
}

/// Convenience builder for terminal-accessible constraint sets.
///
/// Combines WCAG accessibility constraints with terminal-specific constraints:
/// - Hard: `HasLabel`, `ValidRole`, `KeyboardAccessible`, `TerminalNoOverflow`
/// - Hard: `MinReadableSize` (default 3×10)
///
/// # Example
///
/// ```rust
/// use elicit_ui::TerminalAccessible;
///
/// let constraint_set = TerminalAccessible::default().to_constraint_set();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TerminalAccessible {
    /// Minimum readable size constraint settings.
    pub min_readable: MinReadableSize,
}

impl Default for TerminalAccessible {
    fn default() -> Self {
        Self {
            min_readable: MinReadableSize::default(),
        }
    }
}

impl TerminalAccessible {
    /// Create with custom minimum readable size thresholds.
    pub fn with_min_readable(min_cols: u32, min_rows: u32) -> Self {
        Self {
            min_readable: MinReadableSize {
                min_cols,
                min_rows,
            },
        }
    }

    /// Convert to a [`ConstraintSet`] with all terminal + WCAG constraints.
    pub fn to_constraint_set(&self) -> ConstraintSet {
        ConstraintSetBuilder::default()
            .hard(HasLabelConstraint)
            .hard(ValidRoleConstraint)
            .hard(KeyboardAccessibleConstraint)
            .hard(TerminalNoOverflow)
            .hard(self.min_readable)
            .build()
    }
}
