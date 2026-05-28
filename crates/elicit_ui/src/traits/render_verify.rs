//! Runtime proof mode — post-hoc validation of UI invariants against the
//! actual rendered output.
//!
//! # Motivation
//!
//! Compile-time contracts (`Prop` / `Established<P>`) can prove that the *data*
//! fed into the renderer satisfies an invariant, but they cannot see what the
//! renderer actually drew on screen.  `RenderVerifiable` closes that gap: after
//! each frame is painted, the framework inspects the live render buffer and
//! asserts that the invariant still holds.
//!
//! # Architecture
//!
//! ```text
//! elicit_ui        — RenderContext (abstract buffer inspection)
//!                    RenderVerifiable<Ctx> (per-Prop impl hook)
//!
//! elicit_ratatui   — impl RenderContext for ratatui::buffer::Buffer
//! elicit_egui      — impl RenderContext for egui::Context        (future)
//! elicit_leptos    — impl RenderContext for leptos DOM snapshot   (future)
//!
//! user crate       — impl RenderVerifiable<RatatuiRenderContext>
//!                        for BoardColumnsAligned { ... }
//! ```
//!
//! # Usage
//!
//! After rendering a node, call [`verify_in_debug`] with the render context
//! and the area that was painted.  In debug builds this runs all registered
//! checks via `debug_assert!`.  In release builds it compiles to nothing.
//!
//! ```rust,ignore
//! // In render_node, after drawing the board Article:
//! verify_in_debug::<BoardColumnsAligned, _>(&ctx, &area);
//! ```

use elicitation::contracts::Prop;

// ── RenderContext ─────────────────────────────────────────────────────────────

/// Abstract view into a rendered frame buffer.
///
/// Each frontend crate implements this for its own native buffer type
/// (e.g. `ratatui::buffer::Buffer`, `egui::Context`).  The methods are
/// intentionally minimal — just enough to inspect rendered symbol positions
/// without exposing frontend-specific types to `elicit_ui`.
pub trait RenderContext {
    /// Opaque area / bounding-box type for this frontend.
    ///
    /// For ratatui this is `ratatui::layout::Rect`; for egui it would be
    /// `egui::Rect`.
    type Area: Copy;

    /// Returns the symbol (grapheme cluster or character) rendered at
    /// `(col, row)` within `area`.
    ///
    /// `col` and `row` are zero-based offsets relative to the top-left of
    /// `area`.  Returns an empty string if the position is out of bounds.
    fn symbol_at(&self, area: &Self::Area, col: u16, row: u16) -> &str;

    /// Width of `area` in terminal columns / pixels.
    fn area_width(&self, area: &Self::Area) -> u16;

    /// Height of `area` in terminal rows / pixels.
    fn area_height(&self, area: &Self::Area) -> u16;
}

// ── RenderVerifiable ──────────────────────────────────────────────────────────

/// Post-render invariant check for a [`Prop`].
///
/// Implement this on a `Prop` type to declare how to verify — by inspecting
/// the live render buffer — that the invariant is actually preserved in what
/// was drawn on screen.
///
/// Implementations should:
/// - use `debug_assert!` (or [`verify_in_debug`]) so checks are zero-cost in
///   release builds
/// - emit `tracing::error!` before asserting, so the log captures context even
///   if the assert fires
/// - be as specific as possible about *which* cells to inspect, to avoid false
///   positives from unrelated content
pub trait RenderVerifiable<Ctx: RenderContext>: Prop {
    /// Inspect `ctx` over `area` and assert the invariant holds.
    ///
    /// Called automatically by [`verify_in_debug`].  Do not call this directly
    /// unless you are certain you are in a debug build.
    fn verify_rendered(ctx: &Ctx, area: &Ctx::Area);
}

// ── verify_in_debug ───────────────────────────────────────────────────────────

/// Run `P::verify_rendered` in debug builds; compile to nothing in release.
///
/// Call this immediately after rendering the node that `P` governs.
#[inline]
pub fn verify_in_debug<P, Ctx>(ctx: &Ctx, area: &Ctx::Area)
where
    P: RenderVerifiable<Ctx>,
    Ctx: RenderContext,
{
    #[cfg(debug_assertions)]
    P::verify_rendered(ctx, area);

    #[cfg(not(debug_assertions))]
    {
        let _ = (ctx, area);
    }
}
