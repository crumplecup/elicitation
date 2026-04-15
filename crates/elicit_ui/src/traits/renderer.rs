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

use crate::node_roles::*;
use crate::{
    RenderComplete, RenderStats, RolePreserved, UiResult, VerifiedTree, WcagVerified, WidgetId,
};

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
    fn bridge_unknown(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<UnknownNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Structural containers ─────────────────────────────────────────────

    /// Invisible wrapper (`aria-none` / `presentation`).  Frontends typically
    /// render children directly without any wrapper element.
    fn bridge_generic_container(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GenericContainerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Split-pane or panel — a distinct visible region within a window.
    fn bridge_pane(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PaneNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Top-level application window frame.
    fn bridge_window(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<WindowNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Document root (non-web contexts, e.g. an office document).
    fn bridge_document(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocumentNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Web-page root (`<html>` element).
    fn bridge_root_web_area(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RootWebAreaNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Application container (ARIA `application` landmark).
    fn bridge_application(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ApplicationNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// VT-100-style terminal widget.
    fn bridge_terminal(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TerminalNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Interactive controls ──────────────────────────────────────────────

    /// Standard push button.
    fn bridge_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Default-action button (activated by Enter in a form).
    fn bridge_default_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DefaultButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Hyperlink — navigates or triggers an action.
    fn bridge_link(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LinkNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Two-state checkbox (checked / unchecked / mixed).
    fn bridge_check_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CheckBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// One-of-N radio button.
    fn bridge_radio_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RadioButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Binary toggle switch (on / off).
    fn bridge_switch(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SwitchNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Native color-picker control.
    fn bridge_color_well(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ColorWellNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Expand/collapse disclosure triangle (summary/details-style).
    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DisclosureTriangleNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Dropdown combo-box (read-only input + popup list).
    fn bridge_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ComboBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Editable combo-box (free-text input + popup list).
    fn bridge_editable_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<EditableComboBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Standalone listbox (always visible list of selectable options).
    fn bridge_list_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Continuous range slider.
    fn bridge_slider(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SliderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Numeric spin button (increment / decrement).
    fn bridge_spin_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SpinButtonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Linear progress indicator (determinate or indeterminate).
    fn bridge_progress_indicator(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ProgressIndicatorNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Scroll bar (horizontal or vertical).
    fn bridge_scroll_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ScrollBarNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Scrollable viewport container.
    fn bridge_scroll_view(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ScrollViewNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Resize handle / pane splitter.
    fn bridge_splitter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SplitterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Text inputs ───────────────────────────────────────────────────────

    /// Single-line plain text input.
    fn bridge_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TextInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Multi-line text area.
    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MultilineTextInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Search-specialised text input (`type="search"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_search_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SearchInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Date input (`type="date"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_date_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DateInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Date-and-time input (`type="datetime-local"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_date_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DateTimeInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Week input (`type="week"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_week_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<WeekInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Month input (`type="month"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_month_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MonthInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Time input (`type="time"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TimeInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Email address input (`type="email"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_email_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<EmailInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Numeric input (`type="number"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<NumberInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Password input (`type="password"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_password_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PasswordInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// Phone-number input (`type="tel"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_phone_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PhoneNumberInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    /// URL input (`type="url"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_url_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<UrlInputNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(node, id, children, Established::assert())
    }

    // ── Text / inline content ─────────────────────────────────────────────

    /// Inline text run (the leaf-level text node).
    fn bridge_text_run(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TextRunNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Paragraph of text.
    fn bridge_paragraph(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ParagraphNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Accessible label / static caption text.
    fn bridge_label(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LabelNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Heading (level is in `node.level()`).
    fn bridge_heading(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<HeadingNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Explicit line break.
    fn bridge_line_break(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LineBreakNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Block quotation.
    fn bridge_blockquote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<BlockquoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Code block or inline code.
    fn bridge_code(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CodeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Mathematical expression.
    fn bridge_math(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MathNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Short note or advisory text (ARIA `note`).
    fn bridge_note(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<NoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Description-list term (`<dt>`).
    fn bridge_term(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TermNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Description-list definition (`<dd>`).
    fn bridge_definition(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DefinitionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Abbreviated text (`<abbr>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_abbr(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<AbbrNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Emphasised inline text (`<em>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_emphasis(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<EmphasisNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Strong importance inline text (`<strong>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_strong(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<StrongNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Highlighted / marked text (`<mark>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_mark(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MarkNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Machine-readable time or date annotation (`<time>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_time(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TimeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Ruby annotation container (`<ruby>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_ruby(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RubyNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Ruby annotation text (`<rt>` / `<rp>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_ruby_annotation(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RubyAnnotationNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Suggested replacement text (e.g. spelling correction).
    ///
    /// Default: delegates to [`Self::bridge_paragraph`].
    fn bridge_suggestion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SuggestionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_paragraph(node, id, children, Established::assert())
    }

    /// Editorial comment (not rendered to end-users in most contexts).
    ///
    /// Default: delegates to [`Self::bridge_paragraph`].
    fn bridge_comment(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CommentNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_paragraph(node, id, children, Established::assert())
    }

    /// Deleted/struck content (`<del>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_content_deletion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ContentDeletionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Inserted content (`<ins>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_content_insertion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ContentInsertionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Legend for a fieldset.
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_legend(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LegendNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    // ── Media / embedded ─────────────────────────────────────────────────

    /// Image / raster graphic.
    fn bridge_image(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ImageNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Figure container (`<figure>`).
    fn bridge_figure(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FigureNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Caption for a figure or table.
    fn bridge_figure_caption(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FigureCaptionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// 2-D drawing canvas.
    fn bridge_canvas(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CanvasNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Video player.
    fn bridge_video(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<VideoNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Audio player.
    fn bridge_audio(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<AudioNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// SVG document root.
    ///
    /// Default: delegates to [`Self::bridge_image`].
    fn bridge_svg_root(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SvgRootNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_image(node, id, children, Established::assert())
    }

    /// Embedded object (Flash, ActiveX, `<object>`).
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_embedded_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<EmbeddedObjectNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    /// Browser plug-in object.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_plugin_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PluginObjectNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    /// Embedded web view.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_web_view(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<WebViewNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    /// Inline frame (`<iframe>`).
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_iframe(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<IframeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    /// Presentational iframe (no accessible content).
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_iframe_presentational(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<IframePresentationalNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_generic_container(node, id, children, Established::assert())
    }

    // ── Landmark regions ──────────────────────────────────────────────────

    /// `<main>` landmark — primary content area.
    fn bridge_main(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MainNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<nav>` landmark — navigation links.
    fn bridge_navigation(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<NavigationNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<header>` / `<banner>` landmark — page header.
    fn bridge_banner(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<BannerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<footer>` / `contentinfo` landmark — page footer.
    fn bridge_content_info(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ContentInfoNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<aside>` / `complementary` landmark.
    fn bridge_complementary(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ComplementaryNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<form>` landmark / form region.
    fn bridge_form(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FormNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<search>` landmark.
    fn bridge_search(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SearchNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Generic named region (`<section>` with accessible label).
    fn bridge_region(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RegionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Unnamed section (no accessible label).
    fn bridge_section(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SectionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Section-level heading container (distinct from [`bridge_heading`]).
    fn bridge_section_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SectionHeaderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Section-level footer container.
    fn bridge_section_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SectionFooterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<header>` within a section (not the page banner).
    ///
    /// Default: delegates to [`Self::bridge_section_header`].
    fn bridge_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<HeaderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_header(node, id, children, Established::assert())
    }

    /// `<footer>` within a section (not the page footer).
    ///
    /// Default: delegates to [`Self::bridge_section_footer`].
    fn bridge_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FooterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_footer(node, id, children, Established::assert())
    }

    /// `<article>` — self-contained content unit.
    fn bridge_article(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ArticleNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Generic logical grouping (`<div>` / `role="group"`).
    fn bridge_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GroupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Dialogs / overlays ────────────────────────────────────────────────

    /// Modal or non-modal dialog.
    fn bridge_dialog(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DialogNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Alert dialog — requires immediate user response.
    ///
    /// Default: delegates to [`Self::bridge_dialog`].
    fn bridge_alert_dialog(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<AlertDialogNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_dialog(node, id, children, Established::assert())
    }

    /// Expand/collapse `<details>` container.
    fn bridge_details(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DetailsNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Transient tooltip.
    fn bridge_tooltip(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TooltipNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Status / live regions ─────────────────────────────────────────────

    /// Live alert / notification region.
    fn bridge_alert(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<AlertNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Status bar or status region (non-urgent live region).
    fn bridge_status(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<StatusNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Log — append-only live region.
    ///
    /// Default: delegates to [`Self::bridge_status`].
    fn bridge_log(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LogNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_status(node, id, children, Established::assert())
    }

    /// Marquee — scrolling live region.
    ///
    /// Default: delegates to [`Self::bridge_status`].
    fn bridge_marquee(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MarqueeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_status(node, id, children, Established::assert())
    }

    /// Countdown timer.
    fn bridge_timer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TimerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Lists ─────────────────────────────────────────────────────────────

    /// Ordered or unordered list.
    fn bridge_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Item within a list.
    fn bridge_list_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListItemNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Bullet / number marker for a list item.
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_list_marker(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListMarkerNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(node, id, children, Established::assert())
    }

    /// Description list (`<dl>`).
    fn bridge_description_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DescriptionListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Continuous stream of content (`role="feed"`).
    ///
    /// Default: delegates to [`Self::bridge_list`].
    fn bridge_feed(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FeedNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(node, id, children, Established::assert())
    }

    /// Option within a [`bridge_list_box`].
    ///
    /// Default: delegates to [`Self::bridge_list_item`].
    fn bridge_list_box_option(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListBoxOptionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list_item(node, id, children, Established::assert())
    }

    // ── Tables / grids ────────────────────────────────────────────────────

    /// Data table (`<table>`).
    fn bridge_table(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TableNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Table row (`<tr>`).
    fn bridge_row(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RowNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Table data cell (`<td>`).
    fn bridge_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CellNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Table caption (`<caption>`).
    fn bridge_caption(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CaptionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Row group — `<thead>`, `<tbody>`, or `<tfoot>`.
    fn bridge_row_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RowGroupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Row header cell (`<th scope="row">`).
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_row_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RowHeaderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_cell(node, id, children, Established::assert())
    }

    /// Column header cell (`<th scope="col">`).
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_column_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ColumnHeaderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_cell(node, id, children, Established::assert())
    }

    /// Interactive ARIA grid (keyboard-navigable table).
    ///
    /// Default: delegates to [`Self::bridge_table`].
    fn bridge_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GridNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_table(node, id, children, Established::assert())
    }

    /// Cell within a grid.
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_grid_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GridCellNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_cell(node, id, children, Established::assert())
    }

    /// Tree-grid (hierarchical interactive grid).
    ///
    /// Default: delegates to [`Self::bridge_tree`].
    fn bridge_tree_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TreeGridNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_tree(node, id, children, Established::assert())
    }

    /// Chromium-style list grid.
    ///
    /// Default: delegates to [`Self::bridge_grid`].
    fn bridge_list_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListGridNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_grid(node, id, children, Established::assert())
    }

    /// Layout table (presentational, not data).
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_layout_table(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LayoutTableNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_generic_container(node, id, children, Established::assert())
    }

    /// Row within a layout table.
    ///
    /// Default: delegates to [`Self::bridge_row`].
    fn bridge_layout_table_row(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LayoutTableRowNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_row(node, id, children, Established::assert())
    }

    /// Cell within a layout table.
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_layout_table_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LayoutTableCellNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_generic_container(node, id, children, Established::assert())
    }

    // ── Tree ──────────────────────────────────────────────────────────────

    /// Hierarchical tree widget.
    fn bridge_tree(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TreeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Node within a tree widget.
    fn bridge_tree_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TreeItemNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Tabs ──────────────────────────────────────────────────────────────

    /// Individual tab button.
    fn bridge_tab(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TabNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Tab strip containing tab buttons.
    fn bridge_tab_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TabListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Content panel associated with a tab.
    fn bridge_tab_panel(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TabPanelNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Menus ─────────────────────────────────────────────────────────────

    /// Context or popup menu container.
    fn bridge_menu(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Horizontal menu bar.
    ///
    /// Default: delegates to [`Self::bridge_menu`].
    fn bridge_menu_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuBarNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_menu(node, id, children, Established::assert())
    }

    /// Item within a menu.
    fn bridge_menu_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuItemNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Option in a menu-style list popup.
    ///
    /// Default: delegates to [`Self::bridge_menu_item`].
    fn bridge_menu_list_option(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuListOptionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_menu_item(node, id, children, Established::assert())
    }

    /// Popup list for a combo-box or select.
    ///
    /// Default: delegates to [`Self::bridge_menu`].
    fn bridge_menu_list_popup(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuListPopupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_menu(node, id, children, Established::assert())
    }

    /// Checkbox-style menu item.
    ///
    /// Default: delegates to [`Self::bridge_check_box`].
    fn bridge_menu_item_check_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuItemCheckBoxNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_check_box(node, id, children, Established::assert())
    }

    /// Radio-button-style menu item.
    ///
    /// Default: delegates to [`Self::bridge_radio_button`].
    fn bridge_menu_item_radio(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuItemRadioNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_radio_button(node, id, children, Established::assert())
    }

    // ── Toolbar / navigation aids ─────────────────────────────────────────

    /// Toolbar container.
    fn bridge_toolbar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ToolbarNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Title bar (window chrome header).
    ///
    /// Default: delegates to [`Self::bridge_toolbar`].
    fn bridge_title_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TitleBarNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_toolbar(node, id, children, Established::assert())
    }

    /// Radio group container.
    fn bridge_radio_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RadioGroupNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Gauge / scalar measurement indicator.
    ///
    /// Default: delegates to [`Self::bridge_progress_indicator`].
    fn bridge_meter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MeterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_progress_indicator(node, id, children, Established::assert())
    }

    // ── Browser / input system internals ──────────────────────────────────

    /// Virtual on-screen keyboard.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_keyboard(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<KeyboardNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    /// Text insertion caret.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_caret(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CaretNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    /// IME composition candidate.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_ime_candidate(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ImeCandidateNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(node, id, children, Established::assert())
    }

    // ── PDF ───────────────────────────────────────────────────────────────

    /// PDF document root.
    ///
    /// Default: delegates to [`Self::bridge_document`].
    fn bridge_pdf_root(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PdfRootNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_document(node, id, children, Established::assert())
    }

    /// PDF interactive highlight / link.
    ///
    /// Default: delegates to [`Self::bridge_link`].
    fn bridge_pdf_actionable_highlight(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PdfActionableHighlightNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(node, id, children, Established::assert())
    }

    // ── ARIA Graphics ─────────────────────────────────────────────────────

    /// ARIA graphics document.
    ///
    /// Default: delegates to [`Self::bridge_document`].
    fn bridge_graphics_document(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GraphicsDocumentNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_document(node, id, children, Established::assert())
    }

    /// ARIA graphics object (sub-graphic with accessible children).
    ///
    /// Default: delegates to [`Self::bridge_group`].
    fn bridge_graphics_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GraphicsObjectNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_group(node, id, children, Established::assert())
    }

    /// ARIA graphics symbol (standalone meaningful graphic).
    ///
    /// Default: delegates to [`Self::bridge_image`].
    fn bridge_graphics_symbol(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GraphicsSymbolNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_image(node, id, children, Established::assert())
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
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocAbstractNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Acknowledgements section.  Default → [`Self::bridge_section`].
    fn bridge_doc_acknowledgements(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocAcknowledgementsNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Afterword section.  Default → [`Self::bridge_section`].
    fn bridge_doc_afterword(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocAfterwordNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Appendix section.  Default → [`Self::bridge_section`].
    fn bridge_doc_appendix(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocAppendixNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Back-matter link.  Default → [`Self::bridge_link`].
    fn bridge_doc_back_link(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocBackLinkNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(node, id, children, Established::assert())
    }

    /// Single bibliographic entry.  Default → [`Self::bridge_list_item`].
    fn bridge_doc_biblio_entry(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocBiblioEntryNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list_item(node, id, children, Established::assert())
    }

    /// Bibliography section.  Default → [`Self::bridge_list`].
    fn bridge_doc_bibliography(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocBibliographyNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(node, id, children, Established::assert())
    }

    /// Inline bibliography reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_biblio_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocBiblioRefNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(node, id, children, Established::assert())
    }

    /// Chapter section.  Default → [`Self::bridge_section`].
    fn bridge_doc_chapter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocChapterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Colophon (production notes).  Default → [`Self::bridge_section`].
    fn bridge_doc_colophon(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocColophonNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Conclusion section.  Default → [`Self::bridge_section`].
    fn bridge_doc_conclusion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocConclusionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Cover image or section.  Default → [`Self::bridge_figure`].
    fn bridge_doc_cover(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocCoverNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_figure(node, id, children, Established::assert())
    }

    /// Individual credit line.  Default → [`Self::bridge_paragraph`].
    fn bridge_doc_credit(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocCreditNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_paragraph(node, id, children, Established::assert())
    }

    /// Credits section.  Default → [`Self::bridge_section`].
    fn bridge_doc_credits(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocCreditsNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Dedication section.  Default → [`Self::bridge_section`].
    fn bridge_doc_dedication(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocDedicationNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Single endnote.  Default → [`Self::bridge_note`].
    fn bridge_doc_endnote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocEndnoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_note(node, id, children, Established::assert())
    }

    /// Endnotes section.  Default → [`Self::bridge_list`].
    fn bridge_doc_endnotes(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocEndnotesNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(node, id, children, Established::assert())
    }

    /// Epigraph (introductory quotation).  Default → [`Self::bridge_blockquote`].
    fn bridge_doc_epigraph(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocEpigraphNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_blockquote(node, id, children, Established::assert())
    }

    /// Epilogue section.  Default → [`Self::bridge_section`].
    fn bridge_doc_epilogue(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocEpilogueNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Errata section.  Default → [`Self::bridge_section`].
    fn bridge_doc_errata(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocErrataNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Example / code sample section.  Default → [`Self::bridge_section`].
    fn bridge_doc_example(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocExampleNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Single footnote.  Default → [`Self::bridge_note`].
    fn bridge_doc_footnote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocFootnoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_note(node, id, children, Established::assert())
    }

    /// Foreword section.  Default → [`Self::bridge_section`].
    fn bridge_doc_foreword(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocForewordNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Glossary section.  Default → [`Self::bridge_description_list`].
    fn bridge_doc_glossary(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocGlossaryNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_description_list(node, id, children, Established::assert())
    }

    /// Inline glossary term reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_gloss_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocGlossRefNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(node, id, children, Established::assert())
    }

    /// Index section.  Default → [`Self::bridge_section`].
    fn bridge_doc_index(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocIndexNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Introduction section.  Default → [`Self::bridge_section`].
    fn bridge_doc_introduction(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocIntroductionNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Inline note reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_note_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocNoteRefNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(node, id, children, Established::assert())
    }

    /// Notice / warning box.  Default → [`Self::bridge_alert`].
    fn bridge_doc_notice(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocNoticeNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_alert(node, id, children, Established::assert())
    }

    /// Page break marker.  Default → [`Self::bridge_line_break`].
    fn bridge_doc_page_break(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPageBreakNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_line_break(node, id, children, Established::assert())
    }

    /// Page-level footer.  Default → [`Self::bridge_section_footer`].
    fn bridge_doc_page_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPageFooterNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_footer(node, id, children, Established::assert())
    }

    /// Page-level header.  Default → [`Self::bridge_section_header`].
    fn bridge_doc_page_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPageHeaderNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_header(node, id, children, Established::assert())
    }

    /// Page list (TOC of page numbers).  Default → [`Self::bridge_list`].
    fn bridge_doc_page_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPageListNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(node, id, children, Established::assert())
    }

    /// Part / volume division.  Default → [`Self::bridge_section`].
    fn bridge_doc_part(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPartNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Preface section.  Default → [`Self::bridge_section`].
    fn bridge_doc_preface(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPrefaceNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Prologue section.  Default → [`Self::bridge_section`].
    fn bridge_doc_prologue(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPrologueNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(node, id, children, Established::assert())
    }

    /// Pull-quote.  Default → [`Self::bridge_blockquote`].
    fn bridge_doc_pullquote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocPullquoteNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_blockquote(node, id, children, Established::assert())
    }

    /// Q&A block.  Default → [`Self::bridge_group`].
    fn bridge_doc_qna(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocQnaNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_group(node, id, children, Established::assert())
    }

    /// Subtitle / sub-heading.  Default → [`Self::bridge_heading`].
    fn bridge_doc_subtitle(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocSubtitleNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_heading(node, id, children, Established::assert())
    }

    /// Tip / hint box.  Default → [`Self::bridge_note`].
    fn bridge_doc_tip(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocTipNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_note(node, id, children, Established::assert())
    }

    /// Table of contents.  Default → [`Self::bridge_navigation`].
    fn bridge_doc_toc(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocTocNodeValid>,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_navigation(node, id, children, Established::assert())
    }
}

// ── Internal: DFS traversal + role dispatch ───────────────────────────────────

fn render_dfs<T: UiNodeBridge>(
    bridge: &T,
    nodes: &HashMap<NodeId, Node>,
    id: NodeId,
    stats: &mut RenderStats,
    wcag: &Established<WcagVerified>,
) -> (T::Widget, Established<RolePreserved>) {
    let node = match nodes.get(&id) {
        Some(n) => n,
        None => {
            let placeholder = Node::new(Role::Unknown);
            return bridge.bridge_unknown(
                &placeholder,
                id,
                vec![],
                Established::<UnknownNodeValid>::assert(),
            );
        }
    };

    if node.is_hidden() {
        stats.nodes_skipped += 1;
        let placeholder = Node::new(Role::Unknown);
        return bridge.bridge_unknown(
            &placeholder,
            id,
            vec![],
            Established::<UnknownNodeValid>::assert(),
        );
    }

    stats.nodes_visited += 1;

    let is_container = is_container_role(node.role());
    let children: Vec<(T::Widget, Established<RolePreserved>)> = node
        .children()
        .iter()
        .map(|child_id| render_dfs(bridge, nodes, *child_id, stats, wcag))
        .collect();

    if is_container {
        stats.containers_rendered += 1;
    } else {
        stats.widgets_rendered += 1;
    }

    dispatch_role(bridge, node, id, children, wcag)
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
    children: Vec<(T::Widget, Established<RolePreserved>)>,
    _wcag: &Established<WcagVerified>,
) -> (T::Widget, Established<RolePreserved>) {
    match node.role() {
        Role::Unknown => bridge.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::assert(),
        ),
        Role::TextRun => bridge.bridge_text_run(
            node,
            id,
            children,
            Established::<TextRunNodeValid>::assert(),
        ),
        Role::Cell => {
            bridge.bridge_cell(node, id, children, Established::<CellNodeValid>::assert())
        }
        Role::Label => {
            bridge.bridge_label(node, id, children, Established::<LabelNodeValid>::assert())
        }
        Role::Image => {
            bridge.bridge_image(node, id, children, Established::<ImageNodeValid>::assert())
        }
        Role::Link => {
            bridge.bridge_link(node, id, children, Established::<LinkNodeValid>::assert())
        }
        Role::Row => bridge.bridge_row(node, id, children, Established::<RowNodeValid>::assert()),
        Role::ListItem => bridge.bridge_list_item(
            node,
            id,
            children,
            Established::<ListItemNodeValid>::assert(),
        ),
        Role::ListMarker => bridge.bridge_list_marker(
            node,
            id,
            children,
            Established::<ListMarkerNodeValid>::assert(),
        ),
        Role::TreeItem => bridge.bridge_tree_item(
            node,
            id,
            children,
            Established::<TreeItemNodeValid>::assert(),
        ),
        Role::ListBoxOption => bridge.bridge_list_box_option(
            node,
            id,
            children,
            Established::<ListBoxOptionNodeValid>::assert(),
        ),
        Role::MenuItem => bridge.bridge_menu_item(
            node,
            id,
            children,
            Established::<MenuItemNodeValid>::assert(),
        ),
        Role::MenuListOption => bridge.bridge_menu_list_option(
            node,
            id,
            children,
            Established::<MenuListOptionNodeValid>::assert(),
        ),
        Role::Paragraph => bridge.bridge_paragraph(
            node,
            id,
            children,
            Established::<ParagraphNodeValid>::assert(),
        ),
        Role::GenericContainer => bridge.bridge_generic_container(
            node,
            id,
            children,
            Established::<GenericContainerNodeValid>::assert(),
        ),
        Role::CheckBox => bridge.bridge_check_box(
            node,
            id,
            children,
            Established::<CheckBoxNodeValid>::assert(),
        ),
        Role::RadioButton => bridge.bridge_radio_button(
            node,
            id,
            children,
            Established::<RadioButtonNodeValid>::assert(),
        ),
        Role::TextInput => bridge.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::assert(),
        ),
        Role::Button => {
            bridge.bridge_button(node, id, children, Established::<ButtonNodeValid>::assert())
        }
        Role::DefaultButton => bridge.bridge_default_button(
            node,
            id,
            children,
            Established::<DefaultButtonNodeValid>::assert(),
        ),
        Role::Pane => {
            bridge.bridge_pane(node, id, children, Established::<PaneNodeValid>::assert())
        }
        Role::RowHeader => bridge.bridge_row_header(
            node,
            id,
            children,
            Established::<RowHeaderNodeValid>::assert(),
        ),
        Role::ColumnHeader => bridge.bridge_column_header(
            node,
            id,
            children,
            Established::<ColumnHeaderNodeValid>::assert(),
        ),
        Role::RowGroup => bridge.bridge_row_group(
            node,
            id,
            children,
            Established::<RowGroupNodeValid>::assert(),
        ),
        Role::List => {
            bridge.bridge_list(node, id, children, Established::<ListNodeValid>::assert())
        }
        Role::Table => {
            bridge.bridge_table(node, id, children, Established::<TableNodeValid>::assert())
        }
        Role::LayoutTableCell => bridge.bridge_layout_table_cell(
            node,
            id,
            children,
            Established::<LayoutTableCellNodeValid>::assert(),
        ),
        Role::LayoutTableRow => bridge.bridge_layout_table_row(
            node,
            id,
            children,
            Established::<LayoutTableRowNodeValid>::assert(),
        ),
        Role::LayoutTable => bridge.bridge_layout_table(
            node,
            id,
            children,
            Established::<LayoutTableNodeValid>::assert(),
        ),
        Role::Switch => {
            bridge.bridge_switch(node, id, children, Established::<SwitchNodeValid>::assert())
        }
        Role::Menu => {
            bridge.bridge_menu(node, id, children, Established::<MenuNodeValid>::assert())
        }
        Role::MultilineTextInput => bridge.bridge_multiline_text_input(
            node,
            id,
            children,
            Established::<MultilineTextInputNodeValid>::assert(),
        ),
        Role::SearchInput => bridge.bridge_search_input(
            node,
            id,
            children,
            Established::<SearchInputNodeValid>::assert(),
        ),
        Role::DateInput => bridge.bridge_date_input(
            node,
            id,
            children,
            Established::<DateInputNodeValid>::assert(),
        ),
        Role::DateTimeInput => bridge.bridge_date_time_input(
            node,
            id,
            children,
            Established::<DateTimeInputNodeValid>::assert(),
        ),
        Role::WeekInput => bridge.bridge_week_input(
            node,
            id,
            children,
            Established::<WeekInputNodeValid>::assert(),
        ),
        Role::MonthInput => bridge.bridge_month_input(
            node,
            id,
            children,
            Established::<MonthInputNodeValid>::assert(),
        ),
        Role::TimeInput => bridge.bridge_time_input(
            node,
            id,
            children,
            Established::<TimeInputNodeValid>::assert(),
        ),
        Role::EmailInput => bridge.bridge_email_input(
            node,
            id,
            children,
            Established::<EmailInputNodeValid>::assert(),
        ),
        Role::NumberInput => bridge.bridge_number_input(
            node,
            id,
            children,
            Established::<NumberInputNodeValid>::assert(),
        ),
        Role::PasswordInput => bridge.bridge_password_input(
            node,
            id,
            children,
            Established::<PasswordInputNodeValid>::assert(),
        ),
        Role::PhoneNumberInput => bridge.bridge_phone_number_input(
            node,
            id,
            children,
            Established::<PhoneNumberInputNodeValid>::assert(),
        ),
        Role::UrlInput => bridge.bridge_url_input(
            node,
            id,
            children,
            Established::<UrlInputNodeValid>::assert(),
        ),
        Role::Abbr => {
            bridge.bridge_abbr(node, id, children, Established::<AbbrNodeValid>::assert())
        }
        Role::Alert => {
            bridge.bridge_alert(node, id, children, Established::<AlertNodeValid>::assert())
        }
        Role::AlertDialog => bridge.bridge_alert_dialog(
            node,
            id,
            children,
            Established::<AlertDialogNodeValid>::assert(),
        ),
        Role::Application => bridge.bridge_application(
            node,
            id,
            children,
            Established::<ApplicationNodeValid>::assert(),
        ),
        Role::Article => bridge.bridge_article(
            node,
            id,
            children,
            Established::<ArticleNodeValid>::assert(),
        ),
        Role::Audio => {
            bridge.bridge_audio(node, id, children, Established::<AudioNodeValid>::assert())
        }
        Role::Banner => {
            bridge.bridge_banner(node, id, children, Established::<BannerNodeValid>::assert())
        }
        Role::Blockquote => bridge.bridge_blockquote(
            node,
            id,
            children,
            Established::<BlockquoteNodeValid>::assert(),
        ),
        Role::Canvas => {
            bridge.bridge_canvas(node, id, children, Established::<CanvasNodeValid>::assert())
        }
        Role::Caption => bridge.bridge_caption(
            node,
            id,
            children,
            Established::<CaptionNodeValid>::assert(),
        ),
        Role::Caret => {
            bridge.bridge_caret(node, id, children, Established::<CaretNodeValid>::assert())
        }
        Role::Code => {
            bridge.bridge_code(node, id, children, Established::<CodeNodeValid>::assert())
        }
        Role::ColorWell => bridge.bridge_color_well(
            node,
            id,
            children,
            Established::<ColorWellNodeValid>::assert(),
        ),
        Role::ComboBox => bridge.bridge_combo_box(
            node,
            id,
            children,
            Established::<ComboBoxNodeValid>::assert(),
        ),
        Role::EditableComboBox => bridge.bridge_editable_combo_box(
            node,
            id,
            children,
            Established::<EditableComboBoxNodeValid>::assert(),
        ),
        Role::Complementary => bridge.bridge_complementary(
            node,
            id,
            children,
            Established::<ComplementaryNodeValid>::assert(),
        ),
        Role::Comment => bridge.bridge_comment(
            node,
            id,
            children,
            Established::<CommentNodeValid>::assert(),
        ),
        Role::ContentDeletion => bridge.bridge_content_deletion(
            node,
            id,
            children,
            Established::<ContentDeletionNodeValid>::assert(),
        ),
        Role::ContentInsertion => bridge.bridge_content_insertion(
            node,
            id,
            children,
            Established::<ContentInsertionNodeValid>::assert(),
        ),
        Role::ContentInfo => bridge.bridge_content_info(
            node,
            id,
            children,
            Established::<ContentInfoNodeValid>::assert(),
        ),
        Role::Definition => bridge.bridge_definition(
            node,
            id,
            children,
            Established::<DefinitionNodeValid>::assert(),
        ),
        Role::DescriptionList => bridge.bridge_description_list(
            node,
            id,
            children,
            Established::<DescriptionListNodeValid>::assert(),
        ),
        Role::Details => bridge.bridge_details(
            node,
            id,
            children,
            Established::<DetailsNodeValid>::assert(),
        ),
        Role::Dialog => {
            bridge.bridge_dialog(node, id, children, Established::<DialogNodeValid>::assert())
        }
        Role::DisclosureTriangle => bridge.bridge_disclosure_triangle(
            node,
            id,
            children,
            Established::<DisclosureTriangleNodeValid>::assert(),
        ),
        Role::Document => bridge.bridge_document(
            node,
            id,
            children,
            Established::<DocumentNodeValid>::assert(),
        ),
        Role::EmbeddedObject => bridge.bridge_embedded_object(
            node,
            id,
            children,
            Established::<EmbeddedObjectNodeValid>::assert(),
        ),
        Role::Emphasis => bridge.bridge_emphasis(
            node,
            id,
            children,
            Established::<EmphasisNodeValid>::assert(),
        ),
        Role::Feed => {
            bridge.bridge_feed(node, id, children, Established::<FeedNodeValid>::assert())
        }
        Role::FigureCaption => bridge.bridge_figure_caption(
            node,
            id,
            children,
            Established::<FigureCaptionNodeValid>::assert(),
        ),
        Role::Figure => {
            bridge.bridge_figure(node, id, children, Established::<FigureNodeValid>::assert())
        }
        Role::Footer => {
            bridge.bridge_footer(node, id, children, Established::<FooterNodeValid>::assert())
        }
        Role::Form => {
            bridge.bridge_form(node, id, children, Established::<FormNodeValid>::assert())
        }
        Role::Grid => {
            bridge.bridge_grid(node, id, children, Established::<GridNodeValid>::assert())
        }
        Role::GridCell => bridge.bridge_grid_cell(
            node,
            id,
            children,
            Established::<GridCellNodeValid>::assert(),
        ),
        Role::Group => {
            bridge.bridge_group(node, id, children, Established::<GroupNodeValid>::assert())
        }
        Role::Header => {
            bridge.bridge_header(node, id, children, Established::<HeaderNodeValid>::assert())
        }
        Role::Heading => bridge.bridge_heading(
            node,
            id,
            children,
            Established::<HeadingNodeValid>::assert(),
        ),
        Role::Iframe => {
            bridge.bridge_iframe(node, id, children, Established::<IframeNodeValid>::assert())
        }
        Role::IframePresentational => bridge.bridge_iframe_presentational(
            node,
            id,
            children,
            Established::<IframePresentationalNodeValid>::assert(),
        ),
        Role::ImeCandidate => bridge.bridge_ime_candidate(
            node,
            id,
            children,
            Established::<ImeCandidateNodeValid>::assert(),
        ),
        Role::Keyboard => bridge.bridge_keyboard(
            node,
            id,
            children,
            Established::<KeyboardNodeValid>::assert(),
        ),
        Role::Legend => {
            bridge.bridge_legend(node, id, children, Established::<LegendNodeValid>::assert())
        }
        Role::LineBreak => bridge.bridge_line_break(
            node,
            id,
            children,
            Established::<LineBreakNodeValid>::assert(),
        ),
        Role::ListBox => bridge.bridge_list_box(
            node,
            id,
            children,
            Established::<ListBoxNodeValid>::assert(),
        ),
        Role::Log => bridge.bridge_log(node, id, children, Established::<LogNodeValid>::assert()),
        Role::Main => {
            bridge.bridge_main(node, id, children, Established::<MainNodeValid>::assert())
        }
        Role::Mark => {
            bridge.bridge_mark(node, id, children, Established::<MarkNodeValid>::assert())
        }
        Role::Marquee => bridge.bridge_marquee(
            node,
            id,
            children,
            Established::<MarqueeNodeValid>::assert(),
        ),
        Role::Math => {
            bridge.bridge_math(node, id, children, Established::<MathNodeValid>::assert())
        }
        Role::MenuBar => bridge.bridge_menu_bar(
            node,
            id,
            children,
            Established::<MenuBarNodeValid>::assert(),
        ),
        Role::MenuItemCheckBox => bridge.bridge_menu_item_check_box(
            node,
            id,
            children,
            Established::<MenuItemCheckBoxNodeValid>::assert(),
        ),
        Role::MenuItemRadio => bridge.bridge_menu_item_radio(
            node,
            id,
            children,
            Established::<MenuItemRadioNodeValid>::assert(),
        ),
        Role::MenuListPopup => bridge.bridge_menu_list_popup(
            node,
            id,
            children,
            Established::<MenuListPopupNodeValid>::assert(),
        ),
        Role::Meter => {
            bridge.bridge_meter(node, id, children, Established::<MeterNodeValid>::assert())
        }
        Role::Navigation => bridge.bridge_navigation(
            node,
            id,
            children,
            Established::<NavigationNodeValid>::assert(),
        ),
        Role::Note => {
            bridge.bridge_note(node, id, children, Established::<NoteNodeValid>::assert())
        }
        Role::PluginObject => bridge.bridge_plugin_object(
            node,
            id,
            children,
            Established::<PluginObjectNodeValid>::assert(),
        ),
        Role::ProgressIndicator => bridge.bridge_progress_indicator(
            node,
            id,
            children,
            Established::<ProgressIndicatorNodeValid>::assert(),
        ),
        Role::RadioGroup => bridge.bridge_radio_group(
            node,
            id,
            children,
            Established::<RadioGroupNodeValid>::assert(),
        ),
        Role::Region => {
            bridge.bridge_region(node, id, children, Established::<RegionNodeValid>::assert())
        }
        Role::RootWebArea => bridge.bridge_root_web_area(
            node,
            id,
            children,
            Established::<RootWebAreaNodeValid>::assert(),
        ),
        Role::Ruby => {
            bridge.bridge_ruby(node, id, children, Established::<RubyNodeValid>::assert())
        }
        Role::RubyAnnotation => bridge.bridge_ruby_annotation(
            node,
            id,
            children,
            Established::<RubyAnnotationNodeValid>::assert(),
        ),
        Role::ScrollBar => bridge.bridge_scroll_bar(
            node,
            id,
            children,
            Established::<ScrollBarNodeValid>::assert(),
        ),
        Role::ScrollView => bridge.bridge_scroll_view(
            node,
            id,
            children,
            Established::<ScrollViewNodeValid>::assert(),
        ),
        Role::Search => {
            bridge.bridge_search(node, id, children, Established::<SearchNodeValid>::assert())
        }
        Role::Section => bridge.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::assert(),
        ),
        Role::SectionFooter => bridge.bridge_section_footer(
            node,
            id,
            children,
            Established::<SectionFooterNodeValid>::assert(),
        ),
        Role::SectionHeader => bridge.bridge_section_header(
            node,
            id,
            children,
            Established::<SectionHeaderNodeValid>::assert(),
        ),
        Role::Slider => {
            bridge.bridge_slider(node, id, children, Established::<SliderNodeValid>::assert())
        }
        Role::SpinButton => bridge.bridge_spin_button(
            node,
            id,
            children,
            Established::<SpinButtonNodeValid>::assert(),
        ),
        Role::Splitter => bridge.bridge_splitter(
            node,
            id,
            children,
            Established::<SplitterNodeValid>::assert(),
        ),
        Role::Status => {
            bridge.bridge_status(node, id, children, Established::<StatusNodeValid>::assert())
        }
        Role::Strong => {
            bridge.bridge_strong(node, id, children, Established::<StrongNodeValid>::assert())
        }
        Role::Suggestion => bridge.bridge_suggestion(
            node,
            id,
            children,
            Established::<SuggestionNodeValid>::assert(),
        ),
        Role::SvgRoot => bridge.bridge_svg_root(
            node,
            id,
            children,
            Established::<SvgRootNodeValid>::assert(),
        ),
        Role::Tab => bridge.bridge_tab(node, id, children, Established::<TabNodeValid>::assert()),
        Role::TabList => bridge.bridge_tab_list(
            node,
            id,
            children,
            Established::<TabListNodeValid>::assert(),
        ),
        Role::TabPanel => bridge.bridge_tab_panel(
            node,
            id,
            children,
            Established::<TabPanelNodeValid>::assert(),
        ),
        Role::Term => {
            bridge.bridge_term(node, id, children, Established::<TermNodeValid>::assert())
        }
        Role::Time => {
            bridge.bridge_time(node, id, children, Established::<TimeNodeValid>::assert())
        }
        Role::Timer => {
            bridge.bridge_timer(node, id, children, Established::<TimerNodeValid>::assert())
        }
        Role::TitleBar => bridge.bridge_title_bar(
            node,
            id,
            children,
            Established::<TitleBarNodeValid>::assert(),
        ),
        Role::Toolbar => bridge.bridge_toolbar(
            node,
            id,
            children,
            Established::<ToolbarNodeValid>::assert(),
        ),
        Role::Tooltip => bridge.bridge_tooltip(
            node,
            id,
            children,
            Established::<TooltipNodeValid>::assert(),
        ),
        Role::Tree => {
            bridge.bridge_tree(node, id, children, Established::<TreeNodeValid>::assert())
        }
        Role::TreeGrid => bridge.bridge_tree_grid(
            node,
            id,
            children,
            Established::<TreeGridNodeValid>::assert(),
        ),
        Role::Video => {
            bridge.bridge_video(node, id, children, Established::<VideoNodeValid>::assert())
        }
        Role::WebView => bridge.bridge_web_view(
            node,
            id,
            children,
            Established::<WebViewNodeValid>::assert(),
        ),
        Role::Window => {
            bridge.bridge_window(node, id, children, Established::<WindowNodeValid>::assert())
        }
        Role::PdfActionableHighlight => bridge.bridge_pdf_actionable_highlight(
            node,
            id,
            children,
            Established::<PdfActionableHighlightNodeValid>::assert(),
        ),
        Role::PdfRoot => bridge.bridge_pdf_root(
            node,
            id,
            children,
            Established::<PdfRootNodeValid>::assert(),
        ),
        Role::GraphicsDocument => bridge.bridge_graphics_document(
            node,
            id,
            children,
            Established::<GraphicsDocumentNodeValid>::assert(),
        ),
        Role::GraphicsObject => bridge.bridge_graphics_object(
            node,
            id,
            children,
            Established::<GraphicsObjectNodeValid>::assert(),
        ),
        Role::GraphicsSymbol => bridge.bridge_graphics_symbol(
            node,
            id,
            children,
            Established::<GraphicsSymbolNodeValid>::assert(),
        ),
        Role::DocAbstract => bridge.bridge_doc_abstract(
            node,
            id,
            children,
            Established::<DocAbstractNodeValid>::assert(),
        ),
        Role::DocAcknowledgements => bridge.bridge_doc_acknowledgements(
            node,
            id,
            children,
            Established::<DocAcknowledgementsNodeValid>::assert(),
        ),
        Role::DocAfterword => bridge.bridge_doc_afterword(
            node,
            id,
            children,
            Established::<DocAfterwordNodeValid>::assert(),
        ),
        Role::DocAppendix => bridge.bridge_doc_appendix(
            node,
            id,
            children,
            Established::<DocAppendixNodeValid>::assert(),
        ),
        Role::DocBackLink => bridge.bridge_doc_back_link(
            node,
            id,
            children,
            Established::<DocBackLinkNodeValid>::assert(),
        ),
        Role::DocBiblioEntry => bridge.bridge_doc_biblio_entry(
            node,
            id,
            children,
            Established::<DocBiblioEntryNodeValid>::assert(),
        ),
        Role::DocBibliography => bridge.bridge_doc_bibliography(
            node,
            id,
            children,
            Established::<DocBibliographyNodeValid>::assert(),
        ),
        Role::DocBiblioRef => bridge.bridge_doc_biblio_ref(
            node,
            id,
            children,
            Established::<DocBiblioRefNodeValid>::assert(),
        ),
        Role::DocChapter => bridge.bridge_doc_chapter(
            node,
            id,
            children,
            Established::<DocChapterNodeValid>::assert(),
        ),
        Role::DocColophon => bridge.bridge_doc_colophon(
            node,
            id,
            children,
            Established::<DocColophonNodeValid>::assert(),
        ),
        Role::DocConclusion => bridge.bridge_doc_conclusion(
            node,
            id,
            children,
            Established::<DocConclusionNodeValid>::assert(),
        ),
        Role::DocCover => bridge.bridge_doc_cover(
            node,
            id,
            children,
            Established::<DocCoverNodeValid>::assert(),
        ),
        Role::DocCredit => bridge.bridge_doc_credit(
            node,
            id,
            children,
            Established::<DocCreditNodeValid>::assert(),
        ),
        Role::DocCredits => bridge.bridge_doc_credits(
            node,
            id,
            children,
            Established::<DocCreditsNodeValid>::assert(),
        ),
        Role::DocDedication => bridge.bridge_doc_dedication(
            node,
            id,
            children,
            Established::<DocDedicationNodeValid>::assert(),
        ),
        Role::DocEndnote => bridge.bridge_doc_endnote(
            node,
            id,
            children,
            Established::<DocEndnoteNodeValid>::assert(),
        ),
        Role::DocEndnotes => bridge.bridge_doc_endnotes(
            node,
            id,
            children,
            Established::<DocEndnotesNodeValid>::assert(),
        ),
        Role::DocEpigraph => bridge.bridge_doc_epigraph(
            node,
            id,
            children,
            Established::<DocEpigraphNodeValid>::assert(),
        ),
        Role::DocEpilogue => bridge.bridge_doc_epilogue(
            node,
            id,
            children,
            Established::<DocEpilogueNodeValid>::assert(),
        ),
        Role::DocErrata => bridge.bridge_doc_errata(
            node,
            id,
            children,
            Established::<DocErrataNodeValid>::assert(),
        ),
        Role::DocExample => bridge.bridge_doc_example(
            node,
            id,
            children,
            Established::<DocExampleNodeValid>::assert(),
        ),
        Role::DocFootnote => bridge.bridge_doc_footnote(
            node,
            id,
            children,
            Established::<DocFootnoteNodeValid>::assert(),
        ),
        Role::DocForeword => bridge.bridge_doc_foreword(
            node,
            id,
            children,
            Established::<DocForewordNodeValid>::assert(),
        ),
        Role::DocGlossary => bridge.bridge_doc_glossary(
            node,
            id,
            children,
            Established::<DocGlossaryNodeValid>::assert(),
        ),
        Role::DocGlossRef => bridge.bridge_doc_gloss_ref(
            node,
            id,
            children,
            Established::<DocGlossRefNodeValid>::assert(),
        ),
        Role::DocIndex => bridge.bridge_doc_index(
            node,
            id,
            children,
            Established::<DocIndexNodeValid>::assert(),
        ),
        Role::DocIntroduction => bridge.bridge_doc_introduction(
            node,
            id,
            children,
            Established::<DocIntroductionNodeValid>::assert(),
        ),
        Role::DocNoteRef => bridge.bridge_doc_note_ref(
            node,
            id,
            children,
            Established::<DocNoteRefNodeValid>::assert(),
        ),
        Role::DocNotice => bridge.bridge_doc_notice(
            node,
            id,
            children,
            Established::<DocNoticeNodeValid>::assert(),
        ),
        Role::DocPageBreak => bridge.bridge_doc_page_break(
            node,
            id,
            children,
            Established::<DocPageBreakNodeValid>::assert(),
        ),
        Role::DocPageFooter => bridge.bridge_doc_page_footer(
            node,
            id,
            children,
            Established::<DocPageFooterNodeValid>::assert(),
        ),
        Role::DocPageHeader => bridge.bridge_doc_page_header(
            node,
            id,
            children,
            Established::<DocPageHeaderNodeValid>::assert(),
        ),
        Role::DocPageList => bridge.bridge_doc_page_list(
            node,
            id,
            children,
            Established::<DocPageListNodeValid>::assert(),
        ),
        Role::DocPart => bridge.bridge_doc_part(
            node,
            id,
            children,
            Established::<DocPartNodeValid>::assert(),
        ),
        Role::DocPreface => bridge.bridge_doc_preface(
            node,
            id,
            children,
            Established::<DocPrefaceNodeValid>::assert(),
        ),
        Role::DocPrologue => bridge.bridge_doc_prologue(
            node,
            id,
            children,
            Established::<DocPrologueNodeValid>::assert(),
        ),
        Role::DocPullquote => bridge.bridge_doc_pullquote(
            node,
            id,
            children,
            Established::<DocPullquoteNodeValid>::assert(),
        ),
        Role::DocQna => {
            bridge.bridge_doc_qna(node, id, children, Established::<DocQnaNodeValid>::assert())
        }
        Role::DocSubtitle => bridge.bridge_doc_subtitle(
            node,
            id,
            children,
            Established::<DocSubtitleNodeValid>::assert(),
        ),
        Role::DocTip => {
            bridge.bridge_doc_tip(node, id, children, Established::<DocTipNodeValid>::assert())
        }
        Role::DocToc => {
            bridge.bridge_doc_toc(node, id, children, Established::<DocTocNodeValid>::assert())
        }
        Role::ListGrid => bridge.bridge_list_grid(
            node,
            id,
            children,
            Established::<ListGridNodeValid>::assert(),
        ),
        Role::Terminal => bridge.bridge_terminal(
            node,
            id,
            children,
            Established::<TerminalNodeValid>::assert(),
        ),
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
        let wcag = Established::<WcagVerified>::assert();
        let (widget, _) = render_dfs(self, tree.nodes(), tree.root(), &mut stats, &wcag);
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
        let wcag = Established::<WcagVerified>::assert();
        let (widget, _) = render_dfs(
            self,
            tree.nodes(),
            subtree_root.to_node_id(),
            &mut stats,
            &wcag,
        );
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
