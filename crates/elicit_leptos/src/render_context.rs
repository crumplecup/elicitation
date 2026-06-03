//! Stub [`RenderContext`] for leptos (server-side rendered HTML).
//!
//! Leptos renders to an HTML string on the server; pixel-level colour and
//! symbol inspection is not available without DOM introspection or browser
//! APIs.  This stub satisfies the trait bound so leptos crates compile against
//! the same abstractions; all inspection methods return empty / `None`.
//!
//! A full implementation could be layered on top of a headless browser
//! (e.g. Playwright) for integration testing.

use elicit_ui::{RenderColors, RenderContext};

// ── LeptosRenderArea ──────────────────────────────────────────────────────────

/// Nominal bounding box for a rendered leptos sub-tree (CSS pixels).
///
/// Used as the `Area` type for [`LeptosRenderContext`].  Widths and heights
/// are in logical CSS pixels and are informational only; the stub implementation
/// does not perform actual cell lookups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LeptosRenderArea {
    /// Width in CSS pixels.
    pub width: u16,
    /// Height in CSS pixels.
    pub height: u16,
}

// ── LeptosRenderContext ───────────────────────────────────────────────────────

/// A [`RenderContext`] stub for leptos SSR frames.
///
/// All colour and symbol inspection is unavailable from the server render
/// path; methods return empty strings / `None`.
pub struct LeptosRenderContext;

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

    /// Always returns `None` — HTML string rendering does not expose
    /// per-cell colour data.
    fn colors_at(&self, _area: &LeptosRenderArea, _col: u16, _row: u16) -> Option<RenderColors> {
        None
    }
}
