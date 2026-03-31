//! Spatial WCAG constraints powered by the constraint system.
//!
//! These constraints require layout computation (bounding boxes)
//! and verify spatial properties like reflow, text spacing,
//! and resize behavior.

use accesskit::NodeId;

use crate::ElementId;
use crate::constraints::{Constraint, ConstraintContext, SpecReference, Violation, WcagLevel};

/// WCAG 1.4.10 — Reflow (Level AA).
///
/// Content must reflow to fit within 320px width without requiring
/// horizontal scrolling. This constraint checks that no element
/// exceeds a given width threshold.
#[derive(Debug, Clone, Copy)]
pub struct Reflow320;

impl Constraint for Reflow320 {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let Some(node) = ctx.nodes.get(&node_id) else {
            return Ok(());
        };

        if let Some(bounds) = node.bounds() {
            let width = bounds.x1 - bounds.x0;
            let max_width = 320.0;
            if width > max_width {
                return Err(Violation::Reflow {
                    element: ElementId::from(node_id),
                    actual_width: width,
                    max_width,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.10",
            level: WcagLevel::AA,
            url: "https://www.w3.org/WAI/WCAG21/Understanding/reflow",
        }
    }
}

/// WCAG 1.4.12 — Text Spacing (Level AA).
///
/// Content must not be clipped or overlapped when:
/// - Line height is at least 1.5× font size
/// - Spacing following paragraphs is at least 2× font size
/// - Letter spacing is at least 0.12× font size
/// - Word spacing is at least 0.16× font size
///
/// This constraint checks for overlap between sibling text elements
/// after spacing adjustments (approximated by checking bounds overlap).
#[derive(Debug, Clone, Copy)]
pub struct TextSpacing;

impl Constraint for TextSpacing {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let Some(node) = ctx.nodes.get(&node_id) else {
            return Ok(());
        };

        let children: Vec<NodeId> = node.children().to_vec();

        // Check pairwise overlap between sibling bounds
        for i in 0..children.len() {
            for j in (i + 1)..children.len() {
                let child_a = children[i];
                let child_b = children[j];

                if let (Some(a_node), Some(b_node)) =
                    (ctx.nodes.get(&child_a), ctx.nodes.get(&child_b))
                {
                    if let (Some(a_bounds), Some(b_bounds)) = (a_node.bounds(), b_node.bounds()) {
                        // Check for vertical overlap (text spacing issue)
                        let a_bottom = a_bounds.y1;
                        let b_top = b_bounds.y0;

                        if a_bottom > b_top && a_bounds.y0 < b_bounds.y1 {
                            return Err(Violation::TextSpacing {
                                element1: ElementId::from(child_a),
                                element2: ElementId::from(child_b),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.12",
            level: WcagLevel::AA,
            url: "https://www.w3.org/WAI/WCAG21/Understanding/text-spacing",
        }
    }
}

/// WCAG 1.4.4 — Resize Text (Level AA).
///
/// Text can be resized up to 200% without loss of content or functionality.
/// This constraint checks that elements at the given viewport remain
/// within bounds after a 2x scaling factor.
#[derive(Debug, Clone, Copy)]
pub struct ResizeText200;

impl Constraint for ResizeText200 {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let Some(node) = ctx.nodes.get(&node_id) else {
            return Ok(());
        };

        if let Some(bounds) = node.bounds() {
            // At 200% zoom, element dimensions double
            let scaled_right = bounds.x0 + (bounds.x1 - bounds.x0) * 2.0;
            let scaled_bottom = bounds.y0 + (bounds.y1 - bounds.y0) * 2.0;

            let vp_width = f64::from(ctx.viewport.width);
            let vp_height = f64::from(ctx.viewport.height);

            if scaled_right > vp_width || scaled_bottom > vp_height {
                return Err(Violation::Overflow {
                    element: ElementId::from(node_id),
                    element_x: bounds.x0 as i32,
                    element_y: bounds.y0 as i32,
                    element_width: ((bounds.x1 - bounds.x0) * 2.0) as u32,
                    element_height: ((bounds.y1 - bounds.y0) * 2.0) as u32,
                    viewport_width: ctx.viewport.width,
                    viewport_height: ctx.viewport.height,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.4",
            level: WcagLevel::AA,
            url: "https://www.w3.org/WAI/WCAG21/Understanding/resize-text",
        }
    }
}

/// Design system: minimum spacing between interactive elements.
///
/// Not a WCAG criterion, but a common design system heuristic.
/// Checks that interactive sibling elements have at least `min_gap`
/// pixels between their bounding boxes.
#[derive(Debug, Clone, Copy)]
pub struct MinSpacing {
    /// Minimum gap in pixels between interactive siblings.
    pub min_gap: f64,
}

impl Constraint for MinSpacing {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let Some(node) = ctx.nodes.get(&node_id) else {
            return Ok(());
        };

        let children: Vec<NodeId> = node.children().to_vec();

        for i in 0..children.len() {
            for j in (i + 1)..children.len() {
                if let (Some(a_node), Some(b_node)) =
                    (ctx.nodes.get(&children[i]), ctx.nodes.get(&children[j]))
                {
                    if let (Some(a_bounds), Some(b_bounds)) = (a_node.bounds(), b_node.bounds()) {
                        // Vertical gap
                        let v_gap = (b_bounds.y0 - a_bounds.y1).max(a_bounds.y0 - b_bounds.y1);
                        // Horizontal gap
                        let h_gap = (b_bounds.x0 - a_bounds.x1).max(a_bounds.x0 - b_bounds.x1);

                        let gap = v_gap.max(h_gap);

                        if gap < self.min_gap && gap >= 0.0 {
                            return Err(Violation::GridAlignment {
                                element: ElementId::from(children[j]),
                                position: (b_bounds.x0, b_bounds.y0),
                                grid_step: self.min_gap,
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::DesignSystem {
            name: "Material Design 3",
            section: "Spacing",
            url: "https://m3.material.io/foundations/layout/applying-layout",
        }
    }
}

/// Design system: grid alignment constraint.
///
/// Checks that element positions snap to a regular grid step.
#[derive(Debug, Clone, Copy)]
pub struct GridAlignment {
    /// Grid step size in pixels (e.g., 4.0 or 8.0).
    pub step: f64,
}

impl Constraint for GridAlignment {
    #[tracing::instrument(level = "debug", skip(self, ctx))]
    fn check(&self, node_id: NodeId, ctx: &ConstraintContext<'_>) -> Result<(), Violation> {
        let Some(node) = ctx.nodes.get(&node_id) else {
            return Ok(());
        };

        if let Some(bounds) = node.bounds() {
            let x_aligned = (bounds.x0 % self.step).abs() < f64::EPSILON;
            let y_aligned = (bounds.y0 % self.step).abs() < f64::EPSILON;

            if !x_aligned || !y_aligned {
                return Err(Violation::GridAlignment {
                    element: ElementId::from(node_id),
                    position: (bounds.x0, bounds.y0),
                    grid_step: self.step,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::DesignSystem {
            name: "Material Design 3",
            section: "Layout grid",
            url: "https://m3.material.io/foundations/layout/applying-layout",
        }
    }
}
