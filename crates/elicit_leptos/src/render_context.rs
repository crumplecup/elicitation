//! [`RenderContext`] for leptos (server-side rendered HTML).
//!
//! Leptos renders to an HTML string on the server; pixel-level colour
//! inspection is not available without DOM introspection or browser APIs.
//! Instead, this context reads `foreground_color` and `background_color` from
//! the AccessKit node metadata, which are present when the application
//! explicitly sets them on the AccessKit tree.
//!
//! **Limitations:** Colours inherited purely from CSS stylesheets are not
//! visible here; only colours explicitly set on the AccessKit node are
//! checked.  When neither colour is present (the common case for purely
//! CSS-styled elements), `colors_at` returns `None` and the contrast check is
//! silently skipped.

use accesskit::Node;
use elicit_ui::{RenderColors, RenderContext};

// ── LeptosRenderArea ──────────────────────────────────────────────────────────

/// Nominal bounding box for a rendered leptos sub-tree (CSS pixels).
///
/// Used as the `Area` type for [`LeptosRenderContext`].  Widths and heights
/// are in logical CSS pixels and are informational only; the implementation
/// does not perform actual cell lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LeptosRenderArea {
    /// Width in CSS pixels.
    pub width: u16,
    /// Height in CSS pixels.
    pub height: u16,
}

// ── LeptosRenderContext ───────────────────────────────────────────────────────

/// A [`RenderContext`] for leptos that derives colours from AccessKit node metadata.
///
/// Construct via [`LeptosRenderContext::from_node`] to capture any explicitly
/// set `foreground_color`/`background_color` on the AccessKit node.
///
/// When neither colour is present, all inspection methods return `None`,
/// and any contrast proof checks are silently skipped.
pub struct LeptosRenderContext {
    fg: Option<[u8; 3]>,
    bg: Option<[u8; 3]>,
}

impl LeptosRenderContext {
    /// Derive a render context from an AccessKit node's explicit colour metadata.
    ///
    /// Returns `None` for any channel whose alpha is less than 255 (translucent
    /// colours are meaningless for WCAG opaque contrast maths).  When neither
    /// channel is set, `colors_at` will return `None` and checks are skipped.
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

impl RenderContext for LeptosRenderContext {
    type Area = LeptosRenderArea;

    fn symbol_at(&self, _area: &LeptosRenderArea, _col: u16, _row: u16) -> &str {
        ""
    }

    fn area_width(&self, area: &LeptosRenderArea) -> u16 {
        area.width
    }

    fn area_height(&self, area: &LeptosRenderArea) -> u16 {
        area.height
    }

    /// Returns explicitly-set AccessKit node colours, or `None` if absent or translucent.
    ///
    /// `col` and `row` are ignored — leptos colour state is derived from the
    /// node, not from a per-cell buffer; this context stores a single pair.
    fn colors_at(&self, _area: &LeptosRenderArea, _col: u16, _row: u16) -> Option<RenderColors> {
        match (self.fg, self.bg) {
            (Some(fg), Some(bg)) => Some(RenderColors {
                foreground: fg,
                background: bg,
            }),
            _ => None,
        }
    }
}
