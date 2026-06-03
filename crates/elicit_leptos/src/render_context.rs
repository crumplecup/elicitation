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

#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
use web_sys;

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

// ── WasmLeptosRenderContext ───────────────────────────────────────────────────

/// A [`RenderContext`] for leptos that derives colours from `window.getComputedStyle()`.
///
/// Only available on `wasm32` debug builds with the `runtime-proofs` feature.
/// Construct via [`WasmLeptosRenderContext::from_element`]; the foreground
/// colour comes from the element's computed `color` property and the background
/// is resolved by walking ancestor elements until an opaque background is found.
///
/// When no opaque colour pair can be determined, `colors_at` returns `None` and
/// the contrast check is silently skipped.
#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
pub struct WasmLeptosRenderContext {
    fg: Option<[u8; 3]>,
    bg: Option<[u8; 3]>,
}

#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
impl WasmLeptosRenderContext {
    /// Build a render context from a live DOM element.
    ///
    /// Calls `window.getComputedStyle(el)` for the foreground colour and walks
    /// ancestor elements to find the nearest opaque background colour.
    pub fn from_element(el: &web_sys::Element, window: &web_sys::Window) -> Self {
        let style = window.get_computed_style(el).ok().flatten();
        let fg = style
            .as_ref()
            .and_then(|s| s.get_property_value("color").ok())
            .as_deref()
            .and_then(parse_rgb);
        let bg = resolve_opaque_bg(el, window);
        Self { fg, bg }
    }
}

#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
impl RenderContext for WasmLeptosRenderContext {
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

/// Walk ancestor elements until an opaque `background-color` is found.
#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
fn resolve_opaque_bg(el: &web_sys::Element, window: &web_sys::Window) -> Option<[u8; 3]> {
    let mut current = Some(el.clone());
    while let Some(elem) = current {
        if let Ok(Some(style)) = window.get_computed_style(&elem) {
            if let Ok(val) = style.get_property_value("background-color") {
                if let Some(rgb) = parse_rgb(&val) {
                    return Some(rgb);
                }
            }
        }
        current = elem.parent_element();
    }
    None
}

/// Parse a CSS `rgb(r, g, b)` or `rgba(r, g, b, a)` string into `[r, g, b]`.
///
/// Returns `None` for transparent or semi-transparent colours (alpha < 0.999)
/// and for any colour format that cannot be parsed.
#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
pub(crate) fn parse_rgb(s: &str) -> Option<[u8; 3]> {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix("rgb(").and_then(|t| t.strip_suffix(')')) {
        parse_three_channels(inner)
    } else if let Some(inner) = s.strip_prefix("rgba(").and_then(|t| t.strip_suffix(')')) {
        let mut parts = inner.splitn(4, ',');
        let r = channel_u8(parts.next())?;
        let g = channel_u8(parts.next())?;
        let b = channel_u8(parts.next())?;
        let a: f32 = parts.next()?.trim().parse().ok()?;
        if a < 0.999 {
            return None;
        }
        Some([r, g, b])
    } else {
        None
    }
}

#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
fn parse_three_channels(s: &str) -> Option<[u8; 3]> {
    let mut parts = s.splitn(3, ',');
    let r = channel_u8(parts.next())?;
    let g = channel_u8(parts.next())?;
    let b = channel_u8(parts.next())?;
    Some([r, g, b])
}

#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
fn channel_u8(s: Option<&str>) -> Option<u8> {
    Some(s?.trim().parse::<f32>().ok()?.round() as u8)
}
