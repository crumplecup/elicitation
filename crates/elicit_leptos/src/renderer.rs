//! Leptos `UiRenderer` implementation.
//!
//! [`LeptosRenderer`] is the concrete backend that walks a verified AccessKit
//! tree and converts it to either semantic HTML5 (for SSR via axum/tower) or
//! Leptos `view!` macro source code (for CSR/WASM or codegen pipelines).
//!
//! # Example
//!
//! ```rust
//! use elicit_leptos::{LeptosRenderer, LeptosRenderMode};
//! use elicit_ui::UiRenderer;
//!
//! let renderer = LeptosRenderer::new(LeptosRenderMode::Html);
//! assert_eq!(renderer.backend_name(), "leptos");
//! ```

use accesskit::Role;
use elicit_ui::{RenderComplete, RenderStats, UiRenderer, UiResult, VerifiedTree, WidgetId};
use elicitation::Established;
use std::sync::Mutex;
use tracing::instrument;

use crate::leptos_accesskit_convert::{self, LeptosRenderMode};

// ── LeptosRenderer ────────────────────────────────────────────────────────────

/// Leptos rendering backend for verified AccessKit trees.
///
/// Implements [`UiRenderer`] by converting the AccessKit node tree to the
/// selected output format on each [`render`](UiRenderer::render) call.
/// The last rendered output is retained and can be retrieved via
/// [`last_html`](LeptosRenderer::last_html) or
/// [`last_view_code`](LeptosRenderer::last_view_code).
///
/// # Modes
///
/// | Mode | Output | Use case |
/// |------|--------|----------|
/// | [`LeptosRenderMode::Html`] | Semantic HTML5 string | SSR via axum/tower |
/// | [`LeptosRenderMode::ViewMacro`] | Leptos `view!` body | CSR/WASM or codegen |
///
/// # Example
///
/// ```rust
/// use elicit_leptos::{LeptosRenderer, LeptosRenderMode};
/// use elicit_ui::UiRenderer;
///
/// let renderer = LeptosRenderer::new(LeptosRenderMode::Html);
/// assert_eq!(renderer.backend_name(), "leptos");
/// ```
pub struct LeptosRenderer {
    mode: LeptosRenderMode,
    last_output: Mutex<String>,
}

impl LeptosRenderer {
    /// Create a new renderer with the given output mode.
    pub fn new(mode: LeptosRenderMode) -> Self {
        Self {
            mode,
            last_output: Mutex::new(String::new()),
        }
    }

    /// Shorthand: renderer targeting SSR HTML output.
    pub fn html() -> Self {
        Self::new(LeptosRenderMode::Html)
    }

    /// Shorthand: renderer targeting Leptos `view!` macro code output.
    pub fn view_macro() -> Self {
        Self::new(LeptosRenderMode::ViewMacro)
    }

    /// Return the output mode.
    pub fn mode(&self) -> LeptosRenderMode {
        self.mode
    }

    /// Return the last rendered HTML string.
    ///
    /// Returns an empty string if [`render`](UiRenderer::render) has not been
    /// called yet, or if the renderer is in [`LeptosRenderMode::ViewMacro`] mode.
    pub fn last_html(&self) -> String {
        if self.mode == LeptosRenderMode::Html {
            self.last_output.lock().unwrap().clone()
        } else {
            String::new()
        }
    }

    /// Return the last rendered Leptos `view!` macro body.
    ///
    /// Returns an empty string if [`render`](UiRenderer::render) has not been
    /// called yet, or if the renderer is in [`LeptosRenderMode::Html`] mode.
    pub fn last_view_code(&self) -> String {
        if self.mode == LeptosRenderMode::ViewMacro {
            self.last_output.lock().unwrap().clone()
        } else {
            String::new()
        }
    }

    /// Return the last output regardless of mode.
    pub fn last_output(&self) -> String {
        self.last_output.lock().unwrap().clone()
    }
}

impl Default for LeptosRenderer {
    fn default() -> Self {
        Self::new(LeptosRenderMode::Html)
    }
}

impl UiRenderer for LeptosRenderer {
    #[instrument(skip(self, tree), fields(mode = ?self.mode))]
    fn render(&self, tree: &VerifiedTree) -> UiResult<(RenderStats, Established<RenderComplete>)> {
        let (output, stats) =
            leptos_accesskit_convert::render_tree_with_stats(tree.nodes(), tree.root(), self.mode);

        tracing::debug!(
            visited = stats.nodes_visited,
            widgets = stats.widgets_rendered,
            containers = stats.containers_rendered,
            skipped = stats.nodes_skipped,
            mode = ?self.mode,
            "Leptos render pass complete"
        );

        *self.last_output.lock().unwrap() = output;
        Ok((stats, Established::assert()))
    }

    #[instrument(skip(self, tree), fields(mode = ?self.mode))]
    fn render_partial(&self, _node_id: WidgetId, tree: &VerifiedTree) -> UiResult<RenderStats> {
        let (output, stats) =
            leptos_accesskit_convert::render_tree_with_stats(tree.nodes(), tree.root(), self.mode);
        *self.last_output.lock().unwrap() = output;
        Ok(stats)
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }

    fn backend_name(&self) -> &str {
        "leptos"
    }
}
