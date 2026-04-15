//! Front-end renderer trait family: IR → Frontend.
//!
//! The chain is:
//!
//! ```text
//! WCAG factory traits  →  AccessKit IR  →  UiNodeBridge impl  →  Frontend output
//! ```
//!
//! # Trait hierarchy
//!
//! ```text
//! UiRenderBackend                 ← identity + capabilities
//!     └── UiNodeBridge            ← one method per accesskit::Role (full mapping)
//!             └── UiTreeRenderer  ← blanket DFS traversal (auto-derived)
//! UiRenderBackend
//!     └── UiEventBridge           ← optional: frontend → AccessKit events
//! UiTreeRenderer → UiRenderer     ← blanket supertrait alias
//! ```
//!
//! # Design rationale
//!
//! A complete one-to-one role mapping is the only way to guarantee consistent,
//! predictable UI output across multiple frontends.  Every `accesskit::Role`
//! variant has a declared translation; nothing silently falls through.
//!
//! Core interactive roles are **required** — every frontend must provide an
//! explicit implementation.  Specialised roles (DPub, PDF, ARIA Graphics,
//! browser-specific) have **default** implementations that delegate to their
//! semantic equivalent and can be overridden.
//!
//! The `Widget` associated type is **not** object-safe by design: frontend
//! selection is a compile-time decision, so zero-cost generics are the right
//! tool.  For egui (immediate-mode), `Widget = Box<dyn FnOnce(&mut egui::Ui)>`
//! lets the DFS pre-build the render closure tree before execution.

use std::collections::HashMap;

use accesskit::{ActionRequest, Node, NodeId, Role};
use elicitation::Established;
use tracing::instrument;

use crate::{RenderComplete, RenderStats, UiResult, VerifiedTree, WidgetId};

// ── UiRenderBackend ──────────────────────────────────────────────────────────

/// Identity and capability declaration for a frontend rendering backend.
///
/// Every frontend bridge must declare its name and which AccessKit roles it
/// actively supports (as opposed to having defaulted implementations).
pub trait UiRenderBackend {
    /// Short stable identifier for this backend, e.g. `"egui"`, `"leptos"`, `"ratatui"`.
    fn backend_name(&self) -> &'static str;

    /// Return `true` if this backend provides a non-default implementation for
    /// the given role.  Used for capability detection and test tooling.
    fn supports_role(&self, role: Role) -> bool;
}

// ── UiNodeBridge ─────────────────────────────────────────────────────────────

/// Full per-role translation from AccessKit nodes to frontend widgets.
///
/// One method exists per [`accesskit::Role`] variant.  The [`DFS`](UiTreeRenderer)
/// traversal calls the right method automatically — implementors never write a
/// `match node.role()` block.
///
/// ## Required methods
///
/// All core interactive, structural, and media roles are **required** — they
/// have no default implementation and must be implemented explicitly.
///
/// ## Methods with defaults
///
/// Specialised roles (DPub publishing, PDF, ARIA Graphics, browser-specific
/// wrappers, input variants) delegate to a semantically equivalent required
/// method.  Override them when the frontend has a more precise rendering.
///
/// ## Widget
///
/// `Widget` is the frontend's output primitive.  For retained-mode frontends
/// it is usually a value (`String`, `TuiNode`).  For immediate-mode (egui) it
/// should be a deferred closure `Box<dyn FnOnce(&mut egui::Ui)>` so that
/// children are built before the containing layout is created but executed
/// inside it.
pub trait UiNodeBridge: UiRenderBackend {
    /// The frontend's native element or render unit.
    type Widget;

    // ── Unknown / fallback ────────────────────────────────────────────────

    /// Unrecognised or unsupported role — required safety net.
    ///
    /// All roles that have no default delegation call this when their default
    /// is missing.  Frontends may render a debug placeholder or empty element.
    fn bridge_unknown(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    // ── Structural containers ─────────────────────────────────────────────

    /// Invisible wrapper (`aria-none` / `presentation`).  Frontends typically
    /// render children directly without any wrapper element.
    fn bridge_generic_container(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Split-pane or panel — a distinct visible region within a window.
    fn bridge_pane(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Top-level application window frame.
    fn bridge_window(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Document root (non-web contexts, e.g. an office document).
    fn bridge_document(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>)
    -> Self::Widget;

    /// Web-page root (`<html>` element).
    fn bridge_root_web_area(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Application container (ARIA `application` landmark).
    fn bridge_application(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// VT-100-style terminal widget.
    fn bridge_terminal(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>)
    -> Self::Widget;

    // ── Interactive controls ──────────────────────────────────────────────

    /// Standard push button.
    fn bridge_button(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Default-action button (activated by Enter in a form).
    fn bridge_default_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Hyperlink — navigates or triggers an action.
    fn bridge_link(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Two-state checkbox (checked / unchecked / mixed).
    fn bridge_check_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// One-of-N radio button.
    fn bridge_radio_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Binary toggle switch (on / off).
    fn bridge_switch(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Native color-picker control.
    fn bridge_color_well(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Expand/collapse disclosure triangle (summary/details-style).
    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Dropdown combo-box (read-only input + popup list).
    fn bridge_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Editable combo-box (free-text input + popup list).
    fn bridge_editable_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Standalone listbox (always visible list of selectable options).
    fn bridge_list_box(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>)
    -> Self::Widget;

    /// Continuous range slider.
    fn bridge_slider(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Numeric spin button (increment / decrement).
    fn bridge_spin_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Linear progress indicator (determinate or indeterminate).
    fn bridge_progress_indicator(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Scroll bar (horizontal or vertical).
    fn bridge_scroll_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Scrollable viewport container.
    fn bridge_scroll_view(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Resize handle / pane splitter.
    fn bridge_splitter(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>)
    -> Self::Widget;

    // ── Text inputs ───────────────────────────────────────────────────────

    /// Single-line plain text input.
    fn bridge_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Multi-line text area.
    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Search-specialised text input (`type="search"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_search_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Date input (`type="date"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_date_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Date-and-time input (`type="datetime-local"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_date_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Week input (`type="week"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_week_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Month input (`type="month"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_month_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Time input (`type="time"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Email address input (`type="email"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_email_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Numeric input (`type="number"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Password input (`type="password"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_password_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// Phone-number input (`type="tel"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_phone_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    /// URL input (`type="url"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_url_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_text_input(node, id, children)
    }

    // ── Text / inline content ─────────────────────────────────────────────

    /// Inline text run (the leaf-level text node).
    fn bridge_text_run(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>)
    -> Self::Widget;

    /// Paragraph of text.
    fn bridge_paragraph(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Accessible label / static caption text.
    fn bridge_label(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Heading (level is in `node.level()`).
    fn bridge_heading(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Explicit line break.
    fn bridge_line_break(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Block quotation.
    fn bridge_blockquote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Code block or inline code.
    fn bridge_code(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Mathematical expression.
    fn bridge_math(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Short note or advisory text (ARIA `note`).
    fn bridge_note(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Description-list term (`<dt>`).
    fn bridge_term(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Description-list definition (`<dd>`).
    fn bridge_definition(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Abbreviated text (`<abbr>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_abbr(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Emphasised inline text (`<em>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_emphasis(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Strong importance inline text (`<strong>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_strong(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Highlighted / marked text (`<mark>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_mark(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Machine-readable time or date annotation (`<time>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_time(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Ruby annotation container (`<ruby>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_ruby(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Ruby annotation text (`<rt>` / `<rp>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_ruby_annotation(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Suggested replacement text (e.g. spelling correction).
    ///
    /// Default: delegates to [`Self::bridge_paragraph`].
    fn bridge_suggestion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_paragraph(node, id, children)
    }

    /// Editorial comment (not rendered to end-users in most contexts).
    ///
    /// Default: delegates to [`Self::bridge_paragraph`].
    fn bridge_comment(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_paragraph(node, id, children)
    }

    /// Deleted/struck content (`<del>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_content_deletion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Inserted content (`<ins>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_content_insertion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Legend for a fieldset.
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_legend(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    // ── Media / embedded ─────────────────────────────────────────────────

    /// Image / raster graphic.
    fn bridge_image(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Figure container (`<figure>`).
    fn bridge_figure(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Caption for a figure or table.
    fn bridge_figure_caption(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// 2-D drawing canvas.
    fn bridge_canvas(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Video player.
    fn bridge_video(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Audio player.
    fn bridge_audio(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// SVG document root.
    ///
    /// Default: delegates to [`Self::bridge_image`].
    fn bridge_svg_root(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_image(node, id, children)
    }

    /// Embedded object (Flash, ActiveX, `<object>`).
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_embedded_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    /// Browser plug-in object.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_plugin_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    /// Embedded web view.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_web_view(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    /// Inline frame (`<iframe>`).
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_iframe(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    /// Presentational iframe (no accessible content).
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_iframe_presentational(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_generic_container(node, id, children)
    }

    // ── Landmark regions ──────────────────────────────────────────────────

    /// `<main>` landmark — primary content area.
    fn bridge_main(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// `<nav>` landmark — navigation links.
    fn bridge_navigation(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// `<header>` / `<banner>` landmark — page header.
    fn bridge_banner(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// `<footer>` / `contentinfo` landmark — page footer.
    fn bridge_content_info(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// `<aside>` / `complementary` landmark.
    fn bridge_complementary(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// `<form>` landmark / form region.
    fn bridge_form(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// `<search>` landmark.
    fn bridge_search(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Generic named region (`<section>` with accessible label).
    fn bridge_region(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Unnamed section (no accessible label).
    fn bridge_section(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Section-level heading container (distinct from [`bridge_heading`]).
    fn bridge_section_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Section-level footer container.
    fn bridge_section_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// `<header>` within a section (not the page banner).
    ///
    /// Default: delegates to [`Self::bridge_section_header`].
    fn bridge_header(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_section_header(node, id, children)
    }

    /// `<footer>` within a section (not the page footer).
    ///
    /// Default: delegates to [`Self::bridge_section_footer`].
    fn bridge_footer(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_section_footer(node, id, children)
    }

    /// `<article>` — self-contained content unit.
    fn bridge_article(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Generic logical grouping (`<div>` / `role="group"`).
    fn bridge_group(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    // ── Dialogs / overlays ────────────────────────────────────────────────

    /// Modal or non-modal dialog.
    fn bridge_dialog(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Alert dialog — requires immediate user response.
    ///
    /// Default: delegates to [`Self::bridge_dialog`].
    fn bridge_alert_dialog(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_dialog(node, id, children)
    }

    /// Expand/collapse `<details>` container.
    fn bridge_details(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Transient tooltip.
    fn bridge_tooltip(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    // ── Status / live regions ─────────────────────────────────────────────

    /// Live alert / notification region.
    fn bridge_alert(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Status bar or status region (non-urgent live region).
    fn bridge_status(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Log — append-only live region.
    ///
    /// Default: delegates to [`Self::bridge_status`].
    fn bridge_log(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_status(node, id, children)
    }

    /// Marquee — scrolling live region.
    ///
    /// Default: delegates to [`Self::bridge_status`].
    fn bridge_marquee(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_status(node, id, children)
    }

    /// Countdown timer.
    fn bridge_timer(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    // ── Lists ─────────────────────────────────────────────────────────────

    /// Ordered or unordered list.
    fn bridge_list(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Item within a list.
    fn bridge_list_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Bullet / number marker for a list item.
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_list_marker(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_label(node, id, children)
    }

    /// Description list (`<dl>`).
    fn bridge_description_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Continuous stream of content (`role="feed"`).
    ///
    /// Default: delegates to [`Self::bridge_list`].
    fn bridge_feed(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_list(node, id, children)
    }

    /// Option within a [`bridge_list_box`].
    ///
    /// Default: delegates to [`Self::bridge_list_item`].
    fn bridge_list_box_option(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_list_item(node, id, children)
    }

    // ── Tables / grids ────────────────────────────────────────────────────

    /// Data table (`<table>`).
    fn bridge_table(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Table row (`<tr>`).
    fn bridge_row(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Table data cell (`<td>`).
    fn bridge_cell(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Table caption (`<caption>`).
    fn bridge_caption(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Row group — `<thead>`, `<tbody>`, or `<tfoot>`.
    fn bridge_row_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Row header cell (`<th scope="row">`).
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_row_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_cell(node, id, children)
    }

    /// Column header cell (`<th scope="col">`).
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_column_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_cell(node, id, children)
    }

    /// Interactive ARIA grid (keyboard-navigable table).
    ///
    /// Default: delegates to [`Self::bridge_table`].
    fn bridge_grid(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_table(node, id, children)
    }

    /// Cell within a grid.
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_grid_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_cell(node, id, children)
    }

    /// Tree-grid (hierarchical interactive grid).
    ///
    /// Default: delegates to [`Self::bridge_tree`].
    fn bridge_tree_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_tree(node, id, children)
    }

    /// Chromium-style list grid.
    ///
    /// Default: delegates to [`Self::bridge_grid`].
    fn bridge_list_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_grid(node, id, children)
    }

    /// Layout table (presentational, not data).
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_layout_table(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_generic_container(node, id, children)
    }

    /// Row within a layout table.
    ///
    /// Default: delegates to [`Self::bridge_row`].
    fn bridge_layout_table_row(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_row(node, id, children)
    }

    /// Cell within a layout table.
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_layout_table_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_generic_container(node, id, children)
    }

    // ── Tree ──────────────────────────────────────────────────────────────

    /// Hierarchical tree widget.
    fn bridge_tree(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Node within a tree widget.
    fn bridge_tree_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    // ── Tabs ──────────────────────────────────────────────────────────────

    /// Individual tab button.
    fn bridge_tab(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Tab strip containing tab buttons.
    fn bridge_tab_list(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>)
    -> Self::Widget;

    /// Content panel associated with a tab.
    fn bridge_tab_panel(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    // ── Menus ─────────────────────────────────────────────────────────────

    /// Context or popup menu container.
    fn bridge_menu(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Horizontal menu bar.
    ///
    /// Default: delegates to [`Self::bridge_menu`].
    fn bridge_menu_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_menu(node, id, children)
    }

    /// Item within a menu.
    fn bridge_menu_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Option in a menu-style list popup.
    ///
    /// Default: delegates to [`Self::bridge_menu_item`].
    fn bridge_menu_list_option(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_menu_item(node, id, children)
    }

    /// Popup list for a combo-box or select.
    ///
    /// Default: delegates to [`Self::bridge_menu`].
    fn bridge_menu_list_popup(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_menu(node, id, children)
    }

    /// Checkbox-style menu item.
    ///
    /// Default: delegates to [`Self::bridge_check_box`].
    fn bridge_menu_item_check_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_check_box(node, id, children)
    }

    /// Radio-button-style menu item.
    ///
    /// Default: delegates to [`Self::bridge_radio_button`].
    fn bridge_menu_item_radio(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_radio_button(node, id, children)
    }

    // ── Toolbar / navigation aids ─────────────────────────────────────────

    /// Toolbar container.
    fn bridge_toolbar(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget;

    /// Title bar (window chrome header).
    ///
    /// Default: delegates to [`Self::bridge_toolbar`].
    fn bridge_title_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_toolbar(node, id, children)
    }

    /// Radio group container.
    fn bridge_radio_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget;

    /// Gauge / scalar measurement indicator.
    ///
    /// Default: delegates to [`Self::bridge_progress_indicator`].
    fn bridge_meter(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_progress_indicator(node, id, children)
    }

    // ── Browser / input system internals ──────────────────────────────────

    /// Virtual on-screen keyboard.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_keyboard(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    /// Text insertion caret.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_caret(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    /// IME composition candidate.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_ime_candidate(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_unknown(node, id, children)
    }

    // ── PDF ───────────────────────────────────────────────────────────────

    /// PDF document root.
    ///
    /// Default: delegates to [`Self::bridge_document`].
    fn bridge_pdf_root(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_document(node, id, children)
    }

    /// PDF interactive highlight / link.
    ///
    /// Default: delegates to [`Self::bridge_link`].
    fn bridge_pdf_actionable_highlight(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_link(node, id, children)
    }

    // ── ARIA Graphics ─────────────────────────────────────────────────────

    /// ARIA graphics document.
    ///
    /// Default: delegates to [`Self::bridge_document`].
    fn bridge_graphics_document(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_document(node, id, children)
    }

    /// ARIA graphics object (sub-graphic with accessible children).
    ///
    /// Default: delegates to [`Self::bridge_group`].
    fn bridge_graphics_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_group(node, id, children)
    }

    /// ARIA graphics symbol (standalone meaningful graphic).
    ///
    /// Default: delegates to [`Self::bridge_image`].
    fn bridge_graphics_symbol(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_image(node, id, children)
    }

    // ── DPub publishing roles ─────────────────────────────────────────────
    // https://www.w3.org/TR/dpub-aam-1.0/#mapping_role_table
    //
    // All 41 DPub roles have defaults that delegate to their structural
    // equivalent.  Override in frontends that serve EPUB / document content.

    /// Abstract / summary section.  Default → [`Self::bridge_section`].
    fn bridge_doc_abstract(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Acknowledgements section.  Default → [`Self::bridge_section`].
    fn bridge_doc_acknowledgements(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Afterword section.  Default → [`Self::bridge_section`].
    fn bridge_doc_afterword(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Appendix section.  Default → [`Self::bridge_section`].
    fn bridge_doc_appendix(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Back-matter link.  Default → [`Self::bridge_link`].
    fn bridge_doc_back_link(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_link(node, id, children)
    }

    /// Single bibliographic entry.  Default → [`Self::bridge_list_item`].
    fn bridge_doc_biblio_entry(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_list_item(node, id, children)
    }

    /// Bibliography section.  Default → [`Self::bridge_list`].
    fn bridge_doc_bibliography(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_list(node, id, children)
    }

    /// Inline bibliography reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_biblio_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_link(node, id, children)
    }

    /// Chapter section.  Default → [`Self::bridge_section`].
    fn bridge_doc_chapter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Colophon (production notes).  Default → [`Self::bridge_section`].
    fn bridge_doc_colophon(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Conclusion section.  Default → [`Self::bridge_section`].
    fn bridge_doc_conclusion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Cover image or section.  Default → [`Self::bridge_figure`].
    fn bridge_doc_cover(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_figure(node, id, children)
    }

    /// Individual credit line.  Default → [`Self::bridge_paragraph`].
    fn bridge_doc_credit(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_paragraph(node, id, children)
    }

    /// Credits section.  Default → [`Self::bridge_section`].
    fn bridge_doc_credits(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Dedication section.  Default → [`Self::bridge_section`].
    fn bridge_doc_dedication(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Single endnote.  Default → [`Self::bridge_note`].
    fn bridge_doc_endnote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_note(node, id, children)
    }

    /// Endnotes section.  Default → [`Self::bridge_list`].
    fn bridge_doc_endnotes(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_list(node, id, children)
    }

    /// Epigraph (introductory quotation).  Default → [`Self::bridge_blockquote`].
    fn bridge_doc_epigraph(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_blockquote(node, id, children)
    }

    /// Epilogue section.  Default → [`Self::bridge_section`].
    fn bridge_doc_epilogue(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Errata section.  Default → [`Self::bridge_section`].
    fn bridge_doc_errata(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Example / code sample section.  Default → [`Self::bridge_section`].
    fn bridge_doc_example(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Single footnote.  Default → [`Self::bridge_note`].
    fn bridge_doc_footnote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_note(node, id, children)
    }

    /// Foreword section.  Default → [`Self::bridge_section`].
    fn bridge_doc_foreword(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Glossary section.  Default → [`Self::bridge_description_list`].
    fn bridge_doc_glossary(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_description_list(node, id, children)
    }

    /// Inline glossary term reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_gloss_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_link(node, id, children)
    }

    /// Index section.  Default → [`Self::bridge_section`].
    fn bridge_doc_index(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Introduction section.  Default → [`Self::bridge_section`].
    fn bridge_doc_introduction(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Inline note reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_note_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_link(node, id, children)
    }

    /// Notice / warning box.  Default → [`Self::bridge_alert`].
    fn bridge_doc_notice(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_alert(node, id, children)
    }

    /// Page break marker.  Default → [`Self::bridge_line_break`].
    fn bridge_doc_page_break(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_line_break(node, id, children)
    }

    /// Page-level footer.  Default → [`Self::bridge_section_footer`].
    fn bridge_doc_page_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section_footer(node, id, children)
    }

    /// Page-level header.  Default → [`Self::bridge_section_header`].
    fn bridge_doc_page_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section_header(node, id, children)
    }

    /// Page list (TOC of page numbers).  Default → [`Self::bridge_list`].
    fn bridge_doc_page_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_list(node, id, children)
    }

    /// Part / volume division.  Default → [`Self::bridge_section`].
    fn bridge_doc_part(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Preface section.  Default → [`Self::bridge_section`].
    fn bridge_doc_preface(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Prologue section.  Default → [`Self::bridge_section`].
    fn bridge_doc_prologue(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_section(node, id, children)
    }

    /// Pull-quote.  Default → [`Self::bridge_blockquote`].
    fn bridge_doc_pullquote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_blockquote(node, id, children)
    }

    /// Q&A block.  Default → [`Self::bridge_group`].
    fn bridge_doc_qna(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_group(node, id, children)
    }

    /// Subtitle / sub-heading.  Default → [`Self::bridge_heading`].
    fn bridge_doc_subtitle(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<Self::Widget>,
    ) -> Self::Widget {
        self.bridge_heading(node, id, children)
    }

    /// Tip / hint box.  Default → [`Self::bridge_note`].
    fn bridge_doc_tip(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_note(node, id, children)
    }

    /// Table of contents.  Default → [`Self::bridge_navigation`].
    fn bridge_doc_toc(&self, node: &Node, id: NodeId, children: Vec<Self::Widget>) -> Self::Widget {
        self.bridge_navigation(node, id, children)
    }
}

// ── Internal: DFS traversal + role dispatch ───────────────────────────────────

fn render_dfs<T: UiNodeBridge>(
    bridge: &T,
    nodes: &HashMap<NodeId, Node>,
    id: NodeId,
    stats: &mut RenderStats,
) -> T::Widget {
    let node = match nodes.get(&id) {
        Some(n) => n,
        None => {
            let placeholder = Node::new(Role::Unknown);
            return bridge.bridge_unknown(&placeholder, id, vec![]);
        }
    };

    if node.is_hidden() {
        stats.nodes_skipped += 1;
        let placeholder = Node::new(Role::Unknown);
        return bridge.bridge_unknown(&placeholder, id, vec![]);
    }

    stats.nodes_visited += 1;

    let is_container = is_container_role(node.role());
    let children: Vec<T::Widget> = node
        .children()
        .iter()
        .map(|child_id| render_dfs(bridge, nodes, *child_id, stats))
        .collect();

    if is_container {
        stats.containers_rendered += 1;
    } else {
        stats.widgets_rendered += 1;
    }

    dispatch_role(bridge, node, id, children)
}

/// Returns `true` for roles that structurally contain other nodes.
fn is_container_role(role: Role) -> bool {
    matches!(
        role,
        Role::GenericContainer
            | Role::Pane
            | Role::Window
            | Role::Document
            | Role::RootWebArea
            | Role::Application
            | Role::Main
            | Role::Navigation
            | Role::Banner
            | Role::ContentInfo
            | Role::Complementary
            | Role::Form
            | Role::Search
            | Role::Region
            | Role::Section
            | Role::SectionHeader
            | Role::SectionFooter
            | Role::Header
            | Role::Footer
            | Role::Article
            | Role::Group
            | Role::Dialog
            | Role::AlertDialog
            | Role::Details
            | Role::List
            | Role::Feed
            | Role::DescriptionList
            | Role::Table
            | Role::Grid
            | Role::TreeGrid
            | Role::ListGrid
            | Role::LayoutTable
            | Role::RowGroup
            | Role::Row
            | Role::Tree
            | Role::TabList
            | Role::TabPanel
            | Role::Menu
            | Role::MenuBar
            | Role::MenuListPopup
            | Role::Toolbar
            | Role::RadioGroup
            | Role::ScrollView
            | Role::Figure
            | Role::ListBox
            | Role::ComboBox
            | Role::EditableComboBox
            | Role::Terminal
    )
}

/// Dispatch a single node to the correct [`UiNodeBridge`] method based on its role.
///
/// This is the exhaustive `match` that enforces full role coverage at compile
/// time via the trait's required methods.
fn dispatch_role<T: UiNodeBridge>(
    bridge: &T,
    node: &Node,
    id: NodeId,
    children: Vec<T::Widget>,
) -> T::Widget {
    match node.role() {
        Role::Unknown => bridge.bridge_unknown(node, id, children),
        Role::TextRun => bridge.bridge_text_run(node, id, children),
        Role::Cell => bridge.bridge_cell(node, id, children),
        Role::Label => bridge.bridge_label(node, id, children),
        Role::Image => bridge.bridge_image(node, id, children),
        Role::Link => bridge.bridge_link(node, id, children),
        Role::Row => bridge.bridge_row(node, id, children),
        Role::ListItem => bridge.bridge_list_item(node, id, children),
        Role::ListMarker => bridge.bridge_list_marker(node, id, children),
        Role::TreeItem => bridge.bridge_tree_item(node, id, children),
        Role::ListBoxOption => bridge.bridge_list_box_option(node, id, children),
        Role::MenuItem => bridge.bridge_menu_item(node, id, children),
        Role::MenuListOption => bridge.bridge_menu_list_option(node, id, children),
        Role::Paragraph => bridge.bridge_paragraph(node, id, children),
        Role::GenericContainer => bridge.bridge_generic_container(node, id, children),
        Role::CheckBox => bridge.bridge_check_box(node, id, children),
        Role::RadioButton => bridge.bridge_radio_button(node, id, children),
        Role::TextInput => bridge.bridge_text_input(node, id, children),
        Role::Button => bridge.bridge_button(node, id, children),
        Role::DefaultButton => bridge.bridge_default_button(node, id, children),
        Role::Pane => bridge.bridge_pane(node, id, children),
        Role::RowHeader => bridge.bridge_row_header(node, id, children),
        Role::ColumnHeader => bridge.bridge_column_header(node, id, children),
        Role::RowGroup => bridge.bridge_row_group(node, id, children),
        Role::List => bridge.bridge_list(node, id, children),
        Role::Table => bridge.bridge_table(node, id, children),
        Role::LayoutTableCell => bridge.bridge_layout_table_cell(node, id, children),
        Role::LayoutTableRow => bridge.bridge_layout_table_row(node, id, children),
        Role::LayoutTable => bridge.bridge_layout_table(node, id, children),
        Role::Switch => bridge.bridge_switch(node, id, children),
        Role::Menu => bridge.bridge_menu(node, id, children),
        Role::MultilineTextInput => bridge.bridge_multiline_text_input(node, id, children),
        Role::SearchInput => bridge.bridge_search_input(node, id, children),
        Role::DateInput => bridge.bridge_date_input(node, id, children),
        Role::DateTimeInput => bridge.bridge_date_time_input(node, id, children),
        Role::WeekInput => bridge.bridge_week_input(node, id, children),
        Role::MonthInput => bridge.bridge_month_input(node, id, children),
        Role::TimeInput => bridge.bridge_time_input(node, id, children),
        Role::EmailInput => bridge.bridge_email_input(node, id, children),
        Role::NumberInput => bridge.bridge_number_input(node, id, children),
        Role::PasswordInput => bridge.bridge_password_input(node, id, children),
        Role::PhoneNumberInput => bridge.bridge_phone_number_input(node, id, children),
        Role::UrlInput => bridge.bridge_url_input(node, id, children),
        Role::Abbr => bridge.bridge_abbr(node, id, children),
        Role::Alert => bridge.bridge_alert(node, id, children),
        Role::AlertDialog => bridge.bridge_alert_dialog(node, id, children),
        Role::Application => bridge.bridge_application(node, id, children),
        Role::Article => bridge.bridge_article(node, id, children),
        Role::Audio => bridge.bridge_audio(node, id, children),
        Role::Banner => bridge.bridge_banner(node, id, children),
        Role::Blockquote => bridge.bridge_blockquote(node, id, children),
        Role::Canvas => bridge.bridge_canvas(node, id, children),
        Role::Caption => bridge.bridge_caption(node, id, children),
        Role::Caret => bridge.bridge_caret(node, id, children),
        Role::Code => bridge.bridge_code(node, id, children),
        Role::ColorWell => bridge.bridge_color_well(node, id, children),
        Role::ComboBox => bridge.bridge_combo_box(node, id, children),
        Role::EditableComboBox => bridge.bridge_editable_combo_box(node, id, children),
        Role::Complementary => bridge.bridge_complementary(node, id, children),
        Role::Comment => bridge.bridge_comment(node, id, children),
        Role::ContentDeletion => bridge.bridge_content_deletion(node, id, children),
        Role::ContentInsertion => bridge.bridge_content_insertion(node, id, children),
        Role::ContentInfo => bridge.bridge_content_info(node, id, children),
        Role::Definition => bridge.bridge_definition(node, id, children),
        Role::DescriptionList => bridge.bridge_description_list(node, id, children),
        Role::Details => bridge.bridge_details(node, id, children),
        Role::Dialog => bridge.bridge_dialog(node, id, children),
        Role::DisclosureTriangle => bridge.bridge_disclosure_triangle(node, id, children),
        Role::Document => bridge.bridge_document(node, id, children),
        Role::EmbeddedObject => bridge.bridge_embedded_object(node, id, children),
        Role::Emphasis => bridge.bridge_emphasis(node, id, children),
        Role::Feed => bridge.bridge_feed(node, id, children),
        Role::FigureCaption => bridge.bridge_figure_caption(node, id, children),
        Role::Figure => bridge.bridge_figure(node, id, children),
        Role::Footer => bridge.bridge_footer(node, id, children),
        Role::Form => bridge.bridge_form(node, id, children),
        Role::Grid => bridge.bridge_grid(node, id, children),
        Role::GridCell => bridge.bridge_grid_cell(node, id, children),
        Role::Group => bridge.bridge_group(node, id, children),
        Role::Header => bridge.bridge_header(node, id, children),
        Role::Heading => bridge.bridge_heading(node, id, children),
        Role::Iframe => bridge.bridge_iframe(node, id, children),
        Role::IframePresentational => bridge.bridge_iframe_presentational(node, id, children),
        Role::ImeCandidate => bridge.bridge_ime_candidate(node, id, children),
        Role::Keyboard => bridge.bridge_keyboard(node, id, children),
        Role::Legend => bridge.bridge_legend(node, id, children),
        Role::LineBreak => bridge.bridge_line_break(node, id, children),
        Role::ListBox => bridge.bridge_list_box(node, id, children),
        Role::Log => bridge.bridge_log(node, id, children),
        Role::Main => bridge.bridge_main(node, id, children),
        Role::Mark => bridge.bridge_mark(node, id, children),
        Role::Marquee => bridge.bridge_marquee(node, id, children),
        Role::Math => bridge.bridge_math(node, id, children),
        Role::MenuBar => bridge.bridge_menu_bar(node, id, children),
        Role::MenuItemCheckBox => bridge.bridge_menu_item_check_box(node, id, children),
        Role::MenuItemRadio => bridge.bridge_menu_item_radio(node, id, children),
        Role::MenuListPopup => bridge.bridge_menu_list_popup(node, id, children),
        Role::Meter => bridge.bridge_meter(node, id, children),
        Role::Navigation => bridge.bridge_navigation(node, id, children),
        Role::Note => bridge.bridge_note(node, id, children),
        Role::PluginObject => bridge.bridge_plugin_object(node, id, children),
        Role::ProgressIndicator => bridge.bridge_progress_indicator(node, id, children),
        Role::RadioGroup => bridge.bridge_radio_group(node, id, children),
        Role::Region => bridge.bridge_region(node, id, children),
        Role::RootWebArea => bridge.bridge_root_web_area(node, id, children),
        Role::Ruby => bridge.bridge_ruby(node, id, children),
        Role::RubyAnnotation => bridge.bridge_ruby_annotation(node, id, children),
        Role::ScrollBar => bridge.bridge_scroll_bar(node, id, children),
        Role::ScrollView => bridge.bridge_scroll_view(node, id, children),
        Role::Search => bridge.bridge_search(node, id, children),
        Role::Section => bridge.bridge_section(node, id, children),
        Role::SectionFooter => bridge.bridge_section_footer(node, id, children),
        Role::SectionHeader => bridge.bridge_section_header(node, id, children),
        Role::Slider => bridge.bridge_slider(node, id, children),
        Role::SpinButton => bridge.bridge_spin_button(node, id, children),
        Role::Splitter => bridge.bridge_splitter(node, id, children),
        Role::Status => bridge.bridge_status(node, id, children),
        Role::Strong => bridge.bridge_strong(node, id, children),
        Role::Suggestion => bridge.bridge_suggestion(node, id, children),
        Role::SvgRoot => bridge.bridge_svg_root(node, id, children),
        Role::Tab => bridge.bridge_tab(node, id, children),
        Role::TabList => bridge.bridge_tab_list(node, id, children),
        Role::TabPanel => bridge.bridge_tab_panel(node, id, children),
        Role::Term => bridge.bridge_term(node, id, children),
        Role::Time => bridge.bridge_time(node, id, children),
        Role::Timer => bridge.bridge_timer(node, id, children),
        Role::TitleBar => bridge.bridge_title_bar(node, id, children),
        Role::Toolbar => bridge.bridge_toolbar(node, id, children),
        Role::Tooltip => bridge.bridge_tooltip(node, id, children),
        Role::Tree => bridge.bridge_tree(node, id, children),
        Role::TreeGrid => bridge.bridge_tree_grid(node, id, children),
        Role::Video => bridge.bridge_video(node, id, children),
        Role::WebView => bridge.bridge_web_view(node, id, children),
        Role::Window => bridge.bridge_window(node, id, children),
        Role::PdfActionableHighlight => bridge.bridge_pdf_actionable_highlight(node, id, children),
        Role::PdfRoot => bridge.bridge_pdf_root(node, id, children),
        Role::GraphicsDocument => bridge.bridge_graphics_document(node, id, children),
        Role::GraphicsObject => bridge.bridge_graphics_object(node, id, children),
        Role::GraphicsSymbol => bridge.bridge_graphics_symbol(node, id, children),
        Role::DocAbstract => bridge.bridge_doc_abstract(node, id, children),
        Role::DocAcknowledgements => bridge.bridge_doc_acknowledgements(node, id, children),
        Role::DocAfterword => bridge.bridge_doc_afterword(node, id, children),
        Role::DocAppendix => bridge.bridge_doc_appendix(node, id, children),
        Role::DocBackLink => bridge.bridge_doc_back_link(node, id, children),
        Role::DocBiblioEntry => bridge.bridge_doc_biblio_entry(node, id, children),
        Role::DocBibliography => bridge.bridge_doc_bibliography(node, id, children),
        Role::DocBiblioRef => bridge.bridge_doc_biblio_ref(node, id, children),
        Role::DocChapter => bridge.bridge_doc_chapter(node, id, children),
        Role::DocColophon => bridge.bridge_doc_colophon(node, id, children),
        Role::DocConclusion => bridge.bridge_doc_conclusion(node, id, children),
        Role::DocCover => bridge.bridge_doc_cover(node, id, children),
        Role::DocCredit => bridge.bridge_doc_credit(node, id, children),
        Role::DocCredits => bridge.bridge_doc_credits(node, id, children),
        Role::DocDedication => bridge.bridge_doc_dedication(node, id, children),
        Role::DocEndnote => bridge.bridge_doc_endnote(node, id, children),
        Role::DocEndnotes => bridge.bridge_doc_endnotes(node, id, children),
        Role::DocEpigraph => bridge.bridge_doc_epigraph(node, id, children),
        Role::DocEpilogue => bridge.bridge_doc_epilogue(node, id, children),
        Role::DocErrata => bridge.bridge_doc_errata(node, id, children),
        Role::DocExample => bridge.bridge_doc_example(node, id, children),
        Role::DocFootnote => bridge.bridge_doc_footnote(node, id, children),
        Role::DocForeword => bridge.bridge_doc_foreword(node, id, children),
        Role::DocGlossary => bridge.bridge_doc_glossary(node, id, children),
        Role::DocGlossRef => bridge.bridge_doc_gloss_ref(node, id, children),
        Role::DocIndex => bridge.bridge_doc_index(node, id, children),
        Role::DocIntroduction => bridge.bridge_doc_introduction(node, id, children),
        Role::DocNoteRef => bridge.bridge_doc_note_ref(node, id, children),
        Role::DocNotice => bridge.bridge_doc_notice(node, id, children),
        Role::DocPageBreak => bridge.bridge_doc_page_break(node, id, children),
        Role::DocPageFooter => bridge.bridge_doc_page_footer(node, id, children),
        Role::DocPageHeader => bridge.bridge_doc_page_header(node, id, children),
        Role::DocPageList => bridge.bridge_doc_page_list(node, id, children),
        Role::DocPart => bridge.bridge_doc_part(node, id, children),
        Role::DocPreface => bridge.bridge_doc_preface(node, id, children),
        Role::DocPrologue => bridge.bridge_doc_prologue(node, id, children),
        Role::DocPullquote => bridge.bridge_doc_pullquote(node, id, children),
        Role::DocQna => bridge.bridge_doc_qna(node, id, children),
        Role::DocSubtitle => bridge.bridge_doc_subtitle(node, id, children),
        Role::DocTip => bridge.bridge_doc_tip(node, id, children),
        Role::DocToc => bridge.bridge_doc_toc(node, id, children),
        Role::ListGrid => bridge.bridge_list_grid(node, id, children),
        Role::Terminal => bridge.bridge_terminal(node, id, children),
    }
}

// ── UiTreeRenderer ────────────────────────────────────────────────────────────

/// Full-tree rendering via DFS.  Blanket-implemented for any [`UiNodeBridge`].
///
/// Implementors only need to provide the per-role bridge methods; the traversal
/// algorithm, statistics tracking, and proof issuance are automatic.
pub trait UiTreeRenderer: UiNodeBridge {
    /// Render the complete tree from root, returning the root widget.
    ///
    /// The root widget is the composed output for the entire tree.  Frontends
    /// that buffer their output (e.g. a string renderer) receive it here and
    /// can store or return it as needed.
    fn render(
        &self,
        tree: &VerifiedTree,
    ) -> UiResult<(Self::Widget, RenderStats, Established<RenderComplete>)>;

    /// Render a sub-tree rooted at `subtree_root`, returning the sub-root widget.
    fn render_partial(
        &self,
        subtree_root: WidgetId,
        tree: &VerifiedTree,
    ) -> UiResult<(Self::Widget, RenderStats)>;
}

impl<T: UiNodeBridge> UiTreeRenderer for T {
    #[instrument(skip(self, tree), fields(backend = self.backend_name()))]
    fn render(
        &self,
        tree: &VerifiedTree,
    ) -> UiResult<(Self::Widget, RenderStats, Established<RenderComplete>)> {
        let mut stats = RenderStats::default();
        let widget = render_dfs(self, tree.nodes(), tree.root(), &mut stats);
        tracing::debug!(
            visited = stats.nodes_visited,
            widgets = stats.widgets_rendered,
            containers = stats.containers_rendered,
            skipped = stats.nodes_skipped,
            "render complete"
        );
        Ok((widget, stats, Established::assert()))
    }

    #[instrument(skip(self, tree), fields(backend = self.backend_name(), root = ?subtree_root))]
    fn render_partial(
        &self,
        subtree_root: WidgetId,
        tree: &VerifiedTree,
    ) -> UiResult<(Self::Widget, RenderStats)> {
        let mut stats = RenderStats::default();
        let widget = render_dfs(self, tree.nodes(), subtree_root.to_node_id(), &mut stats);
        Ok((widget, stats))
    }
}

// ── UiEventBridge ────────────────────────────────────────────────────────────

/// Translate frontend input events into AccessKit action requests.
///
/// Implemented by frontends that own the event loop (egui, ratatui).
/// Frontends with reactive event systems (leptos) may implement this
/// differently or rely on AccessKit's platform adapter directly.
pub trait UiEventBridge: UiRenderBackend {
    /// The frontend's native event type.
    ///
    /// - egui: `egui::Event`
    /// - ratatui: `crossterm::event::Event`
    /// - leptos: `web_sys::Event`
    type FrontendEvent;

    /// Convert a frontend event to an AccessKit [`ActionRequest`], if applicable.
    ///
    /// Return `None` if the event does not map to any AccessKit action
    /// (e.g. window resize, system events unrelated to UI nodes).
    fn bridge_event(&self, event: &Self::FrontendEvent) -> Option<ActionRequest>;
}

// ── UiRenderer ───────────────────────────────────────────────────────────────

/// Complete frontend rendering capability — blanket alias for [`UiTreeRenderer`].
///
/// Any type implementing [`UiNodeBridge`] (and therefore [`UiRenderBackend`])
/// automatically satisfies this bound via the blanket [`UiTreeRenderer`] impl.
/// Use `.render(tree)` to traverse the AccessKit tree and receive the root
/// widget together with statistics and the [`RenderComplete`] proof.
///
/// [`UiEventBridge`] is kept separate: not all rendering contexts own the
/// event loop.
pub trait UiRenderer: UiTreeRenderer {}

impl<T: UiTreeRenderer> UiRenderer for T {}
