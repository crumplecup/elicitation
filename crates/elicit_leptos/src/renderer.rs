//! Leptos `UiNodeBridge` implementation.
//!
//! [`LeptosRenderer`] is the concrete backend that walks a verified AccessKit
//! tree and converts it to either semantic HTML5 (for SSR via axum/tower) or
//! Leptos `view!` macro source code (for CSR/WASM or codegen pipelines).
//!
//! Each [`accesskit::Role`] maps to one `bridge_*` method that returns a
//! `String` fragment.  The blanket `UiTreeRenderer` impl assembles the
//! full tree via DFS — parents receive pre-rendered child strings.
//!
//! # Example
//!
//! ```rust,no_run
//! use elicit_leptos::{LeptosRenderer, LeptosRenderMode};
//! use elicit_ui::UiRenderBackend;
//!
//! let renderer = LeptosRenderer::new(LeptosRenderMode::Html);
//! assert_eq!(renderer.backend_name(), "leptos");
//! ```

use accesskit::{Node, NodeId, Role};
use elicit_ui::node_roles::*;
use elicit_ui::{
    NodeRenderedEvidence, RolePreserved, UiNodeBridge, UiRenderBackend, WcagNodeProofs,
    verify_wcag_contrast_proofs,
};
use elicitation::Established;

use crate::leptos_accesskit_convert::LeptosRenderMode;
use crate::render_context::{LeptosRenderArea, LeptosRenderContext};

#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
use {
    crate::render_context::WasmLeptosRenderContext,
    std::sync::{
        Mutex,
        atomic::{AtomicU32, Ordering},
    },
};

// ── LeptosRenderer ────────────────────────────────────────────────────────────

/// Leptos rendering backend for verified AccessKit trees.
///
/// Implements [`UiNodeBridge`] — one method per [`accesskit::Role`] — so the
/// blanket [`UiTreeRenderer`](elicit_ui::UiTreeRenderer) provides full-tree DFS
/// rendering for free.  Call `.render(tree)` (from `UiTreeRenderer`) to receive
/// the root HTML/view-macro string alongside statistics and the render proof.
///
/// # Modes
///
/// | Mode | Output | Use case |
/// |------|--------|----------|
/// | [`LeptosRenderMode::Html`] | Semantic HTML5 string | SSR via axum/tower |
/// | [`LeptosRenderMode::ViewMacro`] | Leptos `view!` body | CSR/WASM or codegen |
///
/// # Runtime proofs (`wasm32` + `debug_assertions` + `runtime-proofs` feature)
///
/// When this feature triple is active, [`wrap_widget`](UiNodeBridge::wrap_widget)
/// injects a `data-wcag-check` attribute on every rendered element and queues the
/// associated [`WcagNodeProofs`].  After the HTML is mounted into the DOM, call
/// `post_render` (available on `wasm32` debug builds with `runtime-proofs`) to
/// query each element via `document.querySelector` and verify colours using
/// `window.getComputedStyle`, which reflects the true CSS cascade rather than
/// only explicitly-set AccessKit metadata.
///
/// # Example
///
/// ```rust,no_run
/// use elicit_leptos::{LeptosRenderer, LeptosRenderMode};
/// use elicit_ui::UiRenderBackend;
///
/// let renderer = LeptosRenderer::new(LeptosRenderMode::Html);
/// assert_eq!(renderer.backend_name(), "leptos");
/// ```
pub struct LeptosRenderer {
    mode: LeptosRenderMode,
    /// Pending WCAG checks queued by `wrap_widget`; drained by `post_render`.
    #[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
    pending: Mutex<Vec<(u32, WcagNodeProofs)>>,
}

/// Document-wide monotonically increasing index for `data-wcag-check` attributes.
///
/// Shared across all `LeptosRenderer` instances to guarantee uniqueness even
/// when multiple renderers are active on the same page.
#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
static WCAG_IDX: AtomicU32 = AtomicU32::new(0);

impl LeptosRenderer {
    /// Create a new renderer with the given output mode.
    pub fn new(mode: LeptosRenderMode) -> Self {
        Self {
            mode,
            #[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
            pending: Mutex::new(Vec::new()),
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

    /// Run queued WCAG contrast checks against live DOM computed styles.
    ///
    /// Only available on `wasm32` debug builds with the `runtime-proofs` feature.
    ///
    /// Call this after the HTML produced by the renderer has been mounted into the
    /// DOM and the browser has computed styles.  For each pending node, the method
    /// queries the element via `[data-wcag-check]` and calls
    /// `window.getComputedStyle` to obtain true CSS cascade colours before
    /// verifying contrast proofs.
    #[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
    pub fn post_render(&self) {
        let pending = {
            let mut guard = self.pending.lock().unwrap_or_else(|e| e.into_inner());
            std::mem::take(&mut *guard)
        };
        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };
        for (idx, proofs) in pending {
            let selector = format!("[data-wcag-check=\"{idx}\"]");
            let Ok(Some(el)) = document.query_selector(&selector) else {
                continue;
            };
            let ctx = WasmLeptosRenderContext::from_element(&el, &window);
            verify_wcag_contrast_proofs(&ctx, &LeptosRenderArea::default(), &proofs);
        }
    }
}

impl Default for LeptosRenderer {
    fn default() -> Self {
        Self::new(LeptosRenderMode::Html)
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn node_label(node: &Node) -> String {
    node.label().or(node.value()).unwrap_or("").to_string()
}

fn join_children(children: Vec<String>) -> String {
    children.join("")
}

/// Build a quoted attribute string for the current render mode.
fn attr(mode: LeptosRenderMode, key: &str, val: &str) -> String {
    match mode {
        LeptosRenderMode::Html => format!(r#" {key}="{val}""#),
        LeptosRenderMode::ViewMacro => format!(r#" {key}="{val}""#),
    }
}

fn aria_label_attr(mode: LeptosRenderMode, node: &Node) -> String {
    let lbl = node.label().unwrap_or("");
    if lbl.is_empty() {
        String::new()
    } else {
        attr(mode, "aria-label", lbl)
    }
}

fn disabled_attr(node: &Node) -> &'static str {
    if node.is_disabled() { " disabled" } else { "" }
}

fn wrap_element(tag: &str, extra: &str, body: &str) -> String {
    format!("<{tag}{extra}>{body}</{tag}>")
}

fn self_closing(tag: &str, extra: &str) -> String {
    format!("<{tag}{extra} />")
}

fn role_attr(mode: LeptosRenderMode, role: &str) -> String {
    attr(mode, "role", role)
}

fn hidden_attr(node: &Node) -> &'static str {
    if node.is_hidden() {
        r#" aria-hidden="true""#
    } else {
        ""
    }
}

// ── UiRenderBackend ───────────────────────────────────────────────────────────

impl UiRenderBackend for LeptosRenderer {
    fn backend_name(&self) -> &'static str {
        "leptos"
    }

    fn supports_role(&self, _role: Role) -> bool {
        true
    }
}

// ── UiNodeBridge ─────────────────────────────────────────────────────────────

impl UiNodeBridge for LeptosRenderer {
    type Widget = String;

    // ── Post-render hooks ─────────────────────────────────────────────────

    /// Inject a `data-wcag-check` attribute and queue the proofs for DOM
    /// verification via [`post_render`](LeptosRenderer::post_render).
    ///
    /// Only active on `wasm32` debug builds with the `runtime-proofs` feature;
    /// on other targets the default no-op is used instead.
    #[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
    fn wrap_widget(&self, widget: String, proofs: &WcagNodeProofs) -> String {
        let n = WCAG_IDX.fetch_add(1, Ordering::Relaxed);
        match inject_data_attr(&widget, n) {
            Some(augmented) => {
                self.pending
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .push((n, proofs.clone()));
                augmented
            }
            None => widget,
        }
    }

    /// Run WCAG contrast checks against AccessKit node colour metadata.
    ///
    /// Disabled on `wasm32` debug builds with the `runtime-proofs` feature —
    /// the `post_render` path uses `window.getComputedStyle` for true CSS
    /// cascade colours instead.
    #[cfg(not(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs")))]
    fn verify_node(&self, node: &Node, proofs: &WcagNodeProofs) {
        let ctx = LeptosRenderContext::from_node(node);
        verify_wcag_contrast_proofs(&ctx, &LeptosRenderArea::default(), proofs);
    }

    // ── Unknown / generic ─────────────────────────────────────────────────

    fn bridge_unknown(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<UnknownNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("div", &role_attr(m, "none"), &join_children(children)) + hidden_attr(node)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_generic_container(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<GenericContainerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("div", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_pane(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<PaneNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!("{}{}", role_attr(m, "region"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_window(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<WindowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!("{}{}", role_attr(m, "dialog"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_document(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DocumentNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "article",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_root_web_area(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<RootWebAreaNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("main", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_application(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ApplicationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!(
                    "{}{}",
                    role_attr(m, "application"),
                    aria_label_attr(m, node)
                ),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_terminal(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TerminalNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "pre",
                &format!("{}{}", role_attr(m, "log"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Interactive widgets ───────────────────────────────────────────────

    fn bridge_button(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let text = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element(
                "button",
                &format!(
                    " type=\"button\"{}{}",
                    aria_label_attr(m, node),
                    disabled_attr(node)
                ),
                &text,
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_default_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DefaultButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let text = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            let _ = id;
            wrap_element(
                "button",
                &format!(
                    " type=\"submit\"{}{}",
                    aria_label_attr(m, node),
                    disabled_attr(node)
                ),
                &text,
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_link(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<LinkNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let text = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            let href = node.url().unwrap_or("#");
            wrap_element(
                "a",
                &format!(" href=\"{href}\"{}", aria_label_attr(m, node)),
                &text,
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_check_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<CheckBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let lbl = node_label(node);
            let checked = if node.toggled() == Some(accesskit::Toggled::True) {
                " checked"
            } else {
                ""
            };
            format!(
                "<label><input type=\"checkbox\"{checked}{dis}{} />{lbl}</label>",
                aria_label_attr(m, node),
                dis = disabled_attr(node),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_radio_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<RadioButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let lbl = node_label(node);
            let checked = if node.toggled() == Some(accesskit::Toggled::True) {
                " checked"
            } else {
                ""
            };
            format!(
                "<label><input type=\"radio\"{checked}{dis}{} />{lbl}</label>",
                aria_label_attr(m, node),
                dis = disabled_attr(node),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_switch(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SwitchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let lbl = node_label(node);
            let checked = if node.toggled() == Some(accesskit::Toggled::True) {
                " checked"
            } else {
                ""
            };
            format!(
                "<button type=\"button\" role=\"switch\"{checked}{dis}{}>{lbl}</button>",
                aria_label_attr(m, node),
                dis = disabled_attr(node),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_color_well(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ColorWellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.value().unwrap_or("#000000");
            format!(
                "<input type=\"color\" value=\"{val}\"{}{} />",
                aria_label_attr(m, node),
                disabled_attr(node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DisclosureTriangleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let lbl = node_label(node);
            let expanded = node.toggled() == Some(accesskit::Toggled::True);
            let aria_expanded = if expanded { "true" } else { "false" };
            format!(
                "<button type=\"button\" aria-expanded=\"{aria_expanded}\"{}>▶ {lbl}</button>",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ComboBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let lbl = node_label(node);
            let opts = join_children(children);
            wrap_element(
                "select",
                &format!("{}{}", aria_label_attr(m, node), disabled_attr(node)),
                &format!("{opts}<option>{lbl}</option>"),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_editable_combo_box(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<EditableComboBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.value().unwrap_or("");
            format!(
                "<input type=\"text\" list=\"\" value=\"{val}\"{}{} />",
                aria_label_attr(m, node),
                disabled_attr(node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_list_box(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ListBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "ul",
                &format!("{}{}", role_attr(m, "listbox"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_slider(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SliderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            format!(
                "<input type=\"range\" min=\"{min}\" max=\"{max}\" value=\"{val}\"{}{} />",
                aria_label_attr(m, node),
                disabled_attr(node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_spin_button(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SpinButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(f64::MIN);
            let max = node.max_numeric_value().unwrap_or(f64::MAX);
            let step = node.numeric_value_step().unwrap_or(1.0);
            format!(
                "<input type=\"number\" min=\"{min}\" max=\"{max}\" step=\"{step}\" value=\"{val}\"{}{} />",
                aria_label_attr(m, node),
                disabled_attr(node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_progress_indicator(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ProgressIndicatorNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            format!(
                "<progress value=\"{val}\" max=\"{max}\"{} />",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_scroll_bar(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ScrollBarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.numeric_value().unwrap_or(0.0);
            let min = node.min_numeric_value().unwrap_or(0.0);
            let max = node.max_numeric_value().unwrap_or(100.0);
            format!(
                "<input type=\"range\" role=\"scrollbar\" min=\"{min}\" max=\"{max}\" value=\"{val}\"{}/>",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_scroll_view(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ScrollViewNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!(" style=\"overflow:auto\"{}", aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_splitter(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SplitterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = join_children(children);
            format!("<hr{} />{body}", aria_label_attr(m, node))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Text input ───────────────────────────────────────────────────────

    fn bridge_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.value().unwrap_or("");
            let placeholder = node.placeholder().unwrap_or("");
            let readonly = if node.is_read_only() { " readonly" } else { "" };
            format!(
                "<input type=\"text\" value=\"{val}\" placeholder=\"{placeholder}\"{}{}{readonly} />",
                aria_label_attr(m, node),
                disabled_attr(node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<MultilineTextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let val = node.value().unwrap_or("");
            let placeholder = node.placeholder().unwrap_or("");
            let readonly = if node.is_read_only() { " readonly" } else { "" };
            let dis = disabled_attr(node);
            format!(
                "<textarea placeholder=\"{placeholder}\"{}{readonly}{dis}>{val}</textarea>",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Text display ─────────────────────────────────────────────────────

    fn bridge_text_run(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TextRunNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("span", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_paragraph(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ParagraphNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("p", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_label(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<LabelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("label", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_heading(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<HeadingNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let level = node.level().unwrap_or(2).clamp(1, 6);
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element(&format!("h{level}"), "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_line_break(
        &self,
        _node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<LineBreakNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = { self_closing("br", "") };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_blockquote(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<BlockquoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("blockquote", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_code(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<CodeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("code", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_math(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<MathNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("span", &role_attr(m, "math"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_note(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<NoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("aside", &role_attr(m, "note"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_term(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TermNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("dt", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_definition(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DefinitionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("dd", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Media ────────────────────────────────────────────────────────────

    fn bridge_image(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ImageNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let alt = node_label(node);
            let src = node.url().unwrap_or("");
            format!(
                "<img src=\"{src}\" alt=\"{alt}\"{}/>",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_figure(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<FigureNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "figure",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_figure_caption(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<FigureCaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("figcaption", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_canvas(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<CanvasNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let lbl = node_label(node);
            wrap_element("canvas", &aria_label_attr(m, node), &lbl)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_video(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<VideoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let src = node.url().unwrap_or("");
            let lbl = node_label(node);
            format!(
                "<video src=\"{src}\" controls{}>{lbl}</video>",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_audio(
        &self,
        node: &Node,
        _id: NodeId,
        _children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<AudioNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let __w = {
            let m = self.mode;
            let src = node.url().unwrap_or("");
            let lbl = node_label(node);
            format!(
                "<audio src=\"{src}\" controls{}>{lbl}</audio>",
                aria_label_attr(m, node)
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Landmark sections ─────────────────────────────────────────────────

    fn bridge_main(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<MainNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("main", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_navigation(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<NavigationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("nav", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_banner(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<BannerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "header",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_content_info(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ContentInfoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "footer",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_complementary(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ComplementaryNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("aside", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_form(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<FormNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("form", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_search(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SearchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "search",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_region(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<RegionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "section",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_section(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SectionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "section",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_section_header(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SectionHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!(
                    "{}{}",
                    role_attr(m, "sectionhead"),
                    aria_label_attr(m, node)
                ),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_section_footer(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<SectionFooterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!(
                    "{}{}",
                    role_attr(m, "contentinfo"),
                    aria_label_attr(m, node)
                ),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_article(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ArticleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "article",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_group(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<GroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            if node.label().is_some() {
                wrap_element(
                    "fieldset",
                    &aria_label_attr(m, node),
                    &join_children(children),
                )
            } else {
                wrap_element("div", &role_attr(m, "group"), &join_children(children))
            }
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_dialog(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DialogNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "dialog",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_details(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DetailsNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "details",
                &aria_label_attr(m, node),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tooltip(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TooltipNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("span", &role_attr(m, "tooltip"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_alert(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<AlertNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("div", &role_attr(m, "alert"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_status(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<StatusNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("div", &role_attr(m, "status"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_timer(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TimerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("div", &role_attr(m, "timer"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    fn bridge_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let orientation = node.orientation();
            let tag = if orientation == Some(accesskit::Orientation::Horizontal) {
                "ol"
            } else {
                "ul"
            };
            wrap_element(tag, &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_list_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ListItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("li", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_description_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<DescriptionListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("dl", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Tables ────────────────────────────────────────────────────────────

    fn bridge_table(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TableNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element("table", &aria_label_attr(m, node), &join_children(children))
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_row(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<RowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { wrap_element("tr", "", &join_children(children)) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_cell(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<CellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("td", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_caption(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<CaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("caption", "", &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_row_group(
        &self,
        _node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<RowGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = { wrap_element("tbody", "", &join_children(children)) };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Trees ─────────────────────────────────────────────────────────────

    fn bridge_tree(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TreeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "ul",
                &format!("{}{}", role_attr(m, "tree"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tree_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TreeItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("li", &role_attr(m, "treeitem"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Tabs ─────────────────────────────────────────────────────────────

    fn bridge_tab(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TabNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            let selected = node.is_selected().unwrap_or(false);
            let aria_sel = if selected {
                " aria-selected=\"true\""
            } else {
                " aria-selected=\"false\""
            };
            wrap_element(
                "button",
                &format!("{}{aria_sel}", role_attr(m, "tab")),
                &body,
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tab_list(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TabListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!("{}{}", role_attr(m, "tablist"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_tab_panel(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<TabPanelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!("{}{}", role_attr(m, "tabpanel"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    // ── Menus ─────────────────────────────────────────────────────────────

    fn bridge_menu(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<MenuNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "ul",
                &format!("{}{}", role_attr(m, "menu"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_menu_item(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<MenuItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            let body = if children.is_empty() {
                node_label(node)
            } else {
                join_children(children)
            };
            wrap_element("li", &role_attr(m, "menuitem"), &body)
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_toolbar(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<ToolbarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "div",
                &format!("{}{}", role_attr(m, "toolbar"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }

    fn bridge_radio_group(
        &self,
        node: &Node,
        _id: NodeId,
        children: Vec<(String, Established<RolePreserved>)>,
        proof: Established<RadioGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (String, Established<RolePreserved>) {
        let children: Vec<String> = children.into_iter().map(|(w, _)| w).collect();
        let __w = {
            let m = self.mode;
            wrap_element(
                "fieldset",
                &format!("{}{}", role_attr(m, "radiogroup"), aria_label_attr(m, node)),
                &join_children(children),
            )
        };
        (
            __w,
            Established::<RolePreserved>::prove(&NodeRenderedEvidence {
                role: proof,
                wcag: proofs,
            }),
        )
    }
}

// ── runtime-proofs helpers ────────────────────────────────────────────────────

/// Inject a `data-wcag-check="{idx}"` attribute into the first HTML tag in `html`.
///
/// Returns `None` when no opening tag is found (e.g. empty string or plain text),
/// in which case the caller should skip queueing the proofs.
#[cfg(all(target_arch = "wasm32", debug_assertions, feature = "runtime-proofs"))]
fn inject_data_attr(html: &str, idx: u32) -> Option<String> {
    let pos = html.find('<')?;
    let rest = &html[pos + 1..];
    let end = rest.find(|c: char| c == ' ' || c == '/' || c == '>')?;
    let insert_pos = pos + 1 + end;
    let attr = format!(" data-wcag-check=\"{idx}\"");
    let mut result = String::with_capacity(html.len() + attr.len());
    result.push_str(&html[..insert_pos]);
    result.push_str(&attr);
    result.push_str(&html[insert_pos..]);
    Some(result)
}
