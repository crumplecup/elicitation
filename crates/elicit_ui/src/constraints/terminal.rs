//! Terminal-specific constraint implementations.
//!
//! Constraints for cell-based terminal UIs where dimensions are measured
//! in character cells (columns × rows) rather than pixels.
//!
//! - [`TerminalNoOverflow`] — WCAG 1.4.10 adapted for cell viewports
//! - [`MinReadableSize`] — minimum pane dimensions for usability
//! - [`TerminalAccessible`] — convenience builder combining terminal + WCAG constraints
//! - [`TerminalBreakpoint`] — named terminal size (cols × rows)
//! - [`TerminalBreakpointSet`] — industry-standard terminal sizes
//! - [`BreakpointReport`] — verification results at each breakpoint

use super::{
    Constraint, ConstraintContext, ConstraintSet, ConstraintSetBuilder, ConstraintVerification,
    SpecReference, Violation, WcagLevel,
};
use super::{HasLabelConstraint, KeyboardAccessibleConstraint, ValidRoleConstraint};
use crate::Viewport;
use accesskit::NodeId;
use accesskit::Role;
use std::collections::HashMap;

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

/// Enforcement tier for a terminal breakpoint.
///
/// Controls how verification failures at this size are reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointTier {
    /// Must pass — layout is invalid if it fails at this size.
    Required,
    /// Warning only — informational, does not block validity.
    Advisory,
    /// Expected to fail — documents known limitations at extreme sizes.
    ExpectedFail,
}

impl std::fmt::Display for BreakpointTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Advisory => write!(f, "advisory"),
            Self::ExpectedFail => write!(f, "expected-fail"),
        }
    }
}

/// A named terminal size breakpoint (cols × rows).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalBreakpoint {
    /// Human-readable name (e.g., "VT100", "small", "ultrawide").
    pub name: String,
    /// Width in columns.
    pub cols: u32,
    /// Height in rows.
    pub rows: u32,
    /// Enforcement tier.
    pub tier: BreakpointTier,
}

impl TerminalBreakpoint {
    /// Create a new terminal breakpoint.
    pub fn new(name: impl Into<String>, cols: u32, rows: u32, tier: BreakpointTier) -> Self {
        Self {
            name: name.into(),
            cols,
            rows,
            tier,
        }
    }

    /// Convert to a [`Viewport`] for constraint verification.
    pub fn to_viewport(&self) -> Viewport {
        Viewport::new(self.cols, self.rows)
    }
}

impl std::fmt::Display for TerminalBreakpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}×{}, {})",
            self.name, self.cols, self.rows, self.tier
        )
    }
}

/// A set of terminal breakpoints for multi-size verification.
///
/// Industry-standard terminal sizes from VT100 to ultrawide monitors.
#[derive(Debug, Clone)]
pub struct TerminalBreakpointSet {
    breakpoints: Vec<TerminalBreakpoint>,
}

impl TerminalBreakpointSet {
    /// Standard terminal breakpoint set covering industry sizes.
    ///
    /// | Name       | Size    | Tier         |
    /// |------------|---------|--------------|
    /// | micro      | 40×12   | ExpectedFail |
    /// | tiny       | 60×20   | Advisory     |
    /// | VT100      | 80×24   | Required     |
    /// | small      | 100×30  | Required     |
    /// | medium     | 120×40  | Required     |
    /// | large      | 160×50  | Required     |
    /// | ultrawide  | 200×60  | Required     |
    pub fn standard() -> Self {
        Self {
            breakpoints: vec![
                TerminalBreakpoint::new("micro", 40, 12, BreakpointTier::ExpectedFail),
                TerminalBreakpoint::new("tiny", 60, 20, BreakpointTier::Advisory),
                TerminalBreakpoint::new("VT100", 80, 24, BreakpointTier::Required),
                TerminalBreakpoint::new("small", 100, 30, BreakpointTier::Required),
                TerminalBreakpoint::new("medium", 120, 40, BreakpointTier::Required),
                TerminalBreakpoint::new("large", 160, 50, BreakpointTier::Required),
                TerminalBreakpoint::new("ultrawide", 200, 60, BreakpointTier::Required),
            ],
        }
    }

    /// Get all breakpoints.
    pub fn breakpoints(&self) -> &[TerminalBreakpoint] {
        &self.breakpoints
    }

    /// Add a custom breakpoint.
    pub fn with_breakpoint(mut self, bp: TerminalBreakpoint) -> Self {
        self.breakpoints.push(bp);
        self
    }

    /// Get only required-tier breakpoints.
    pub fn required(&self) -> Vec<&TerminalBreakpoint> {
        self.breakpoints
            .iter()
            .filter(|bp| bp.tier == BreakpointTier::Required)
            .collect()
    }

    /// Get only advisory-tier breakpoints.
    pub fn advisory(&self) -> Vec<&TerminalBreakpoint> {
        self.breakpoints
            .iter()
            .filter(|bp| bp.tier == BreakpointTier::Advisory)
            .collect()
    }

    /// Verify a tree against all breakpoints, producing a [`BreakpointReport`].
    ///
    /// Runs the given constraint set at each breakpoint's viewport size.
    #[tracing::instrument(skip(self, constraint_set, nodes))]
    pub fn verify_all(
        &self,
        root: NodeId,
        nodes: &HashMap<NodeId, accesskit::Node>,
        constraint_set: &ConstraintSet,
    ) -> BreakpointReport {
        let mut results = Vec::with_capacity(self.breakpoints.len());

        for bp in &self.breakpoints {
            let ctx = ConstraintContext {
                nodes,
                viewport: bp.to_viewport(),
            };
            let verification = constraint_set.verify(root, &ctx);
            let outcome = BreakpointOutcome::from_verification(&verification, bp.tier);

            results.push(BreakpointResult {
                breakpoint: bp.clone(),
                outcome,
                verification,
            });
        }

        BreakpointReport { results }
    }
}

/// Outcome of verifying at a single breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointOutcome {
    /// All constraints passed.
    Pass,
    /// Hard constraints failed at a required breakpoint.
    Fail,
    /// Hard constraints failed at an advisory breakpoint (warning only).
    Warning,
    /// Hard constraints failed at an expected-fail breakpoint (documented).
    ExpectedFailure,
}

impl BreakpointOutcome {
    fn from_verification(v: &ConstraintVerification, tier: BreakpointTier) -> Self {
        if v.is_valid() {
            Self::Pass
        } else {
            match tier {
                BreakpointTier::Required => Self::Fail,
                BreakpointTier::Advisory => Self::Warning,
                BreakpointTier::ExpectedFail => Self::ExpectedFailure,
            }
        }
    }
}

impl std::fmt::Display for BreakpointOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pass => write!(f, "✅ pass"),
            Self::Fail => write!(f, "❌ fail"),
            Self::Warning => write!(f, "⚠️ warning"),
            Self::ExpectedFailure => write!(f, "📝 expected-fail"),
        }
    }
}

/// Result of verifying at a single breakpoint.
#[derive(Debug, Clone)]
pub struct BreakpointResult {
    /// The breakpoint that was tested.
    pub breakpoint: TerminalBreakpoint,
    /// Overall outcome.
    pub outcome: BreakpointOutcome,
    /// Full verification details.
    pub verification: ConstraintVerification,
}

/// Report of verification across all terminal breakpoints.
#[derive(Debug, Clone)]
pub struct BreakpointReport {
    /// Per-breakpoint results.
    pub results: Vec<BreakpointResult>,
}

impl BreakpointReport {
    /// Whether all required breakpoints pass.
    pub fn is_valid(&self) -> bool {
        self.results
            .iter()
            .all(|r| r.outcome != BreakpointOutcome::Fail)
    }

    /// Count of results by outcome.
    pub fn count(&self, outcome: BreakpointOutcome) -> usize {
        self.results.iter().filter(|r| r.outcome == outcome).count()
    }

    /// Get results that failed at required breakpoints.
    pub fn failures(&self) -> Vec<&BreakpointResult> {
        self.results
            .iter()
            .filter(|r| r.outcome == BreakpointOutcome::Fail)
            .collect()
    }

    /// Get results that warned at advisory breakpoints.
    pub fn warnings(&self) -> Vec<&BreakpointResult> {
        self.results
            .iter()
            .filter(|r| r.outcome == BreakpointOutcome::Warning)
            .collect()
    }

    /// Format a summary table of all breakpoint results.
    pub fn summary(&self) -> String {
        let mut out = String::from("Terminal Breakpoint Report\n");
        out.push_str("─────────────────────────────────────────\n");
        for r in &self.results {
            out.push_str(&format!(
                "{:<12} {:>3}×{:<3} [{}] {}\n",
                r.breakpoint.name,
                r.breakpoint.cols,
                r.breakpoint.rows,
                r.breakpoint.tier,
                r.outcome,
            ));
        }
        out.push_str("─────────────────────────────────────────\n");
        out.push_str(&format!(
            "Result: {} ({} pass, {} fail, {} warn, {} expected-fail)\n",
            if self.is_valid() { "PASS" } else { "FAIL" },
            self.count(BreakpointOutcome::Pass),
            self.count(BreakpointOutcome::Fail),
            self.count(BreakpointOutcome::Warning),
            self.count(BreakpointOutcome::ExpectedFailure),
        ));
        out
    }
}
