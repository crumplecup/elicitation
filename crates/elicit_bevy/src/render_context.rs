//! [`RenderContext`] for Bevy UI (AccessKit-backed colour sampling).
//!
//! Bevy renders to the GPU; pixel-level colour inspection via the CPU is
//! impractical at bridge time.  Instead this context reads `foreground_color`
//! and `background_color` from the AccessKit node metadata, which the IR
//! populates when colours are explicitly set.
//!
//! When neither colour is present (the common case for GPU-styled elements),
//! `colors_at` returns `None` and the contrast check is silently skipped.

use accesskit::Node;
use elicit_ui::{RenderColors, RenderContext};

// ── BevyRenderArea ────────────────────────────────────────────────────────────

/// Nominal bounding box for a Bevy UI sub-tree (logical pixels).
///
/// Used as the `Area` type for [`BevyRenderContext`].  Widths and heights are
/// in logical pixels and are informational only; this implementation does not
/// perform actual per-cell lookups.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BevyRenderArea {
    /// Width in logical pixels.
    pub width: f32,
    /// Height in logical pixels.
    pub height: f32,
}

// ── BevyRenderContext ─────────────────────────────────────────────────────────

/// A [`RenderContext`] for Bevy UI that derives colours from AccessKit node metadata.
///
/// Construct via [`BevyRenderContext::from_node`] to capture any explicitly
/// set `foreground_color`/`background_color` on the AccessKit node.
///
/// When neither colour is present, all inspection methods return `None`,
/// and any contrast proof checks are silently skipped.
pub struct BevyRenderContext {
    fg: Option<[u8; 3]>,
    bg: Option<[u8; 3]>,
}

impl BevyRenderContext {
    /// Derive a render context from an AccessKit node's explicit colour metadata.
    ///
    /// Returns `None` for any channel whose alpha is less than 255 (translucent
    /// colours are meaningless for WCAG opaque contrast maths).
    pub fn from_node(node: &Node) -> Self {
        Self {
            fg: node.foreground_color().and_then(accesskit_color_to_rgb),
            bg: node.background_color().and_then(accesskit_color_to_rgb),
        }
    }
}

/// Extract `[r, g, b]` from an [`accesskit::Color`], returning `None` when alpha < 255.
fn accesskit_color_to_rgb(c: accesskit::Color) -> Option<[u8; 3]> {
    if c.alpha < 255 {
        None
    } else {
        Some([c.red, c.green, c.blue])
    }
}

impl RenderContext for BevyRenderContext {
    type Area = BevyRenderArea;

    fn symbol_at(&self, _area: &BevyRenderArea, _col: u16, _row: u16) -> &str {
        ""
    }

    fn area_width(&self, area: &BevyRenderArea) -> u16 {
        area.width as u16
    }

    fn area_height(&self, area: &BevyRenderArea) -> u16 {
        area.height as u16
    }

    /// Returns explicitly-set AccessKit node colours, or `None` if absent or translucent.
    ///
    /// `col` and `row` are ignored — Bevy colour state is derived from the node,
    /// not from a per-cell buffer; this context stores a single pair.
    fn colors_at(&self, _area: &BevyRenderArea, _col: u16, _row: u16) -> Option<RenderColors> {
        match (self.fg, self.bg) {
            (Some(fg), Some(bg)) => Some(RenderColors {
                foreground: fg,
                background: bg,
            }),
            _ => None,
        }
    }
}
