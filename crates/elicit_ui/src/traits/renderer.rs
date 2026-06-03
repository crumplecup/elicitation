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

use std::collections::BTreeMap;

use accesskit::{ActionRequest, Node, NodeId, Role};
use elicitation::Established;
use tracing::instrument;

use crate::node_roles::*;
use crate::{
    RenderComplete, RenderStats, RolePreserved, UiResult, VerifiedTree, WcagNodeProofs,
    WcagVerified, WidgetId,
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

    // ── Post-render hooks ─────────────────────────────────────────────────

    /// Wrap a widget to add post-render behaviour such as WCAG verification.
    ///
    /// Called by [`render_dfs`] after [`dispatch_role`] returns.  The default
    /// implementation is an identity function; frontends that need to inject
    /// post-render logic (e.g. egui wrapping a closure to run contrast checks
    /// after the widget draws) should override this.
    fn wrap_widget(&self, widget: Self::Widget, _proofs: &WcagNodeProofs) -> Self::Widget {
        widget
    }

    /// Inspect an AccessKit [`Node`] and its proof sidecar for any verification
    /// that can be performed before the widget is drawn.
    ///
    /// Called by [`render_dfs`] after [`dispatch_role`] returns, in addition to
    /// [`wrap_widget`].  The default implementation is a no-op; frontends that
    /// derive colour information from AccessKit node metadata (e.g. leptos)
    /// should override this to run WCAG contrast checks.
    fn verify_node(&self, _node: &Node, _proofs: &WcagNodeProofs) {}

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
        proofs: WcagNodeProofs,
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
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Split-pane or panel — a distinct visible region within a window.
    fn bridge_pane(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<PaneNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Top-level application window frame.
    fn bridge_window(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<WindowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Document root (non-web contexts, e.g. an office document).
    fn bridge_document(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DocumentNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Web-page root (`<html>` element).
    fn bridge_root_web_area(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RootWebAreaNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Application container (ARIA `application` landmark).
    fn bridge_application(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ApplicationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// VT-100-style terminal widget.
    fn bridge_terminal(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TerminalNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Interactive controls ──────────────────────────────────────────────

    /// Standard push button.
    fn bridge_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Default-action button (activated by Enter in a form).
    fn bridge_default_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DefaultButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_button(
            node,
            id,
            children,
            Established::<ButtonNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Hyperlink — navigates or triggers an action.
    fn bridge_link(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LinkNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Two-state checkbox (checked / unchecked / mixed).
    fn bridge_check_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CheckBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// One-of-N radio button.
    fn bridge_radio_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RadioButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Binary toggle switch (on / off).
    fn bridge_switch(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SwitchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Native color-picker control.
    fn bridge_color_well(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ColorWellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Expand/collapse disclosure triangle (summary/details-style).
    fn bridge_disclosure_triangle(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DisclosureTriangleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Dropdown combo-box (read-only input + popup list).
    fn bridge_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ComboBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Editable combo-box (free-text input + popup list).
    fn bridge_editable_combo_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<EditableComboBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_combo_box(
            node,
            id,
            children,
            Established::<ComboBoxNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Standalone listbox (always visible list of selectable options).
    fn bridge_list_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Continuous range slider.
    fn bridge_slider(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SliderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Numeric spin button (increment / decrement).
    fn bridge_spin_button(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SpinButtonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Linear progress indicator (determinate or indeterminate).
    fn bridge_progress_indicator(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ProgressIndicatorNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Scroll bar (horizontal or vertical).
    fn bridge_scroll_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ScrollBarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_progress_indicator(
            node,
            id,
            children,
            Established::<ProgressIndicatorNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Scrollable viewport container.
    fn bridge_scroll_view(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ScrollViewNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Resize handle / pane splitter.
    fn bridge_splitter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SplitterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Text inputs ───────────────────────────────────────────────────────

    /// Single-line plain text input.
    fn bridge_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Multi-line text area.
    fn bridge_multiline_text_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MultilineTextInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Search-specialised text input (`type="search"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_search_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SearchInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Date input (`type="date"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_date_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DateInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Date-and-time input (`type="datetime-local"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_date_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DateTimeInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Week input (`type="week"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_week_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<WeekInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Month input (`type="month"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_month_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MonthInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Time input (`type="time"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_time_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TimeInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Email address input (`type="email"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_email_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<EmailInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Numeric input (`type="number"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<NumberInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Password input (`type="password"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_password_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<PasswordInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Phone-number input (`type="tel"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_phone_number_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<PhoneNumberInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// URL input (`type="url"`).
    ///
    /// Default: delegates to [`Self::bridge_text_input`].
    fn bridge_url_input(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<UrlInputNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Text / inline content ─────────────────────────────────────────────

    /// Inline text run (the leaf-level text node).
    fn bridge_text_run(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TextRunNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Paragraph of text.
    fn bridge_paragraph(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ParagraphNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Accessible label / static caption text.
    fn bridge_label(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LabelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Heading (level is in `node.level()`).
    fn bridge_heading(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<HeadingNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Explicit line break.
    fn bridge_line_break(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<LineBreakNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Block quotation.
    fn bridge_blockquote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<BlockquoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Code block or inline code.
    fn bridge_code(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CodeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Mathematical expression.
    fn bridge_math(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MathNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Short note or advisory text (ARIA `note`).
    fn bridge_note(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<NoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Description-list term (`<dt>`).
    fn bridge_term(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TermNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Description-list definition (`<dd>`).
    fn bridge_definition(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DefinitionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Abbreviated text (`<abbr>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_abbr(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<AbbrNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Emphasised inline text (`<em>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_emphasis(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<EmphasisNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Strong importance inline text (`<strong>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_strong(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<StrongNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Highlighted / marked text (`<mark>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_mark(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MarkNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Machine-readable time or date annotation (`<time>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_time(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TimeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Ruby annotation container (`<ruby>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_ruby(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RubyNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Ruby annotation text (`<rt>` / `<rp>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_ruby_annotation(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RubyAnnotationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Suggested replacement text (e.g. spelling correction).
    ///
    /// Default: delegates to [`Self::bridge_paragraph`].
    fn bridge_suggestion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SuggestionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_paragraph(
            node,
            id,
            children,
            Established::<ParagraphNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Editorial comment (not rendered to end-users in most contexts).
    ///
    /// Default: delegates to [`Self::bridge_paragraph`].
    fn bridge_comment(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CommentNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_paragraph(
            node,
            id,
            children,
            Established::<ParagraphNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Deleted/struck content (`<del>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_content_deletion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ContentDeletionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Inserted content (`<ins>`).
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_content_insertion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ContentInsertionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Legend for a fieldset.
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_legend(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LegendNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Media / embedded ─────────────────────────────────────────────────

    /// Image / raster graphic.
    fn bridge_image(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ImageNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Figure container (`<figure>`).
    fn bridge_figure(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FigureNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Caption for a figure or table.
    fn bridge_figure_caption(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FigureCaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// 2-D drawing canvas.
    fn bridge_canvas(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CanvasNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Video player.
    fn bridge_video(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<VideoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Audio player.
    fn bridge_audio(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<AudioNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// SVG document root.
    ///
    /// Default: delegates to [`Self::bridge_image`].
    fn bridge_svg_root(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<SvgRootNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_image(
            node,
            id,
            children,
            Established::<ImageNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Embedded object (Flash, ActiveX, `<object>`).
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_embedded_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<EmbeddedObjectNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Browser plug-in object.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_plugin_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<PluginObjectNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Embedded web view.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_web_view(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<WebViewNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Inline frame (`<iframe>`).
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_iframe(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<IframeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Presentational iframe (no accessible content).
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_iframe_presentational(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<IframePresentationalNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_generic_container(
            node,
            id,
            children,
            Established::<GenericContainerNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Landmark regions ──────────────────────────────────────────────────

    /// `<main>` landmark — primary content area.
    fn bridge_main(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MainNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<nav>` landmark — navigation links.
    fn bridge_navigation(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<NavigationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<header>` / `<banner>` landmark — page header.
    fn bridge_banner(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<BannerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<footer>` / `contentinfo` landmark — page footer.
    fn bridge_content_info(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ContentInfoNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<aside>` / `complementary` landmark.
    fn bridge_complementary(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ComplementaryNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<form>` landmark / form region.
    fn bridge_form(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<FormNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<search>` landmark.
    fn bridge_search(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SearchNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Generic named region (`<section>` with accessible label).
    fn bridge_region(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RegionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Unnamed section (no accessible label).
    fn bridge_section(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SectionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Section-level heading container (distinct from [`Self::bridge_heading`]).
    fn bridge_section_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SectionHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Section-level footer container.
    fn bridge_section_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<SectionFooterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// `<header>` within a section (not the page banner).
    ///
    /// Default: delegates to [`Self::bridge_section_header`].
    fn bridge_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<HeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_header(
            node,
            id,
            children,
            Established::<SectionHeaderNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// `<footer>` within a section (not the page footer).
    ///
    /// Default: delegates to [`Self::bridge_section_footer`].
    fn bridge_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FooterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_footer(
            node,
            id,
            children,
            Established::<SectionFooterNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// `<article>` — self-contained content unit.
    fn bridge_article(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ArticleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Generic logical grouping (`<div>` / `role="group"`).
    fn bridge_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<GroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Dialogs / overlays ────────────────────────────────────────────────

    /// Modal or non-modal dialog.
    fn bridge_dialog(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DialogNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Alert dialog — requires immediate user response.
    ///
    /// Default: delegates to [`Self::bridge_dialog`].
    fn bridge_alert_dialog(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<AlertDialogNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_dialog(
            node,
            id,
            children,
            Established::<DialogNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Expand/collapse `<details>` container.
    fn bridge_details(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DetailsNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Transient tooltip.
    fn bridge_tooltip(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TooltipNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Status / live regions ─────────────────────────────────────────────

    /// Live alert / notification region.
    fn bridge_alert(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<AlertNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Status bar or status region (non-urgent live region).
    fn bridge_status(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<StatusNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Log — append-only live region.
    ///
    /// Default: delegates to [`Self::bridge_status`].
    fn bridge_log(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LogNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_status(
            node,
            id,
            children,
            Established::<StatusNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Marquee — scrolling live region.
    ///
    /// Default: delegates to [`Self::bridge_status`].
    fn bridge_marquee(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MarqueeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_status(
            node,
            id,
            children,
            Established::<StatusNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Countdown timer.
    fn bridge_timer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TimerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Lists ─────────────────────────────────────────────────────────────

    /// Ordered or unordered list.
    fn bridge_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Item within a list.
    fn bridge_list_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ListItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Bullet / number marker for a list item.
    ///
    /// Default: delegates to [`Self::bridge_label`].
    fn bridge_list_marker(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListMarkerNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Description list (`<dl>`).
    fn bridge_description_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<DescriptionListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Continuous stream of content (`role="feed"`).
    ///
    /// Default: delegates to [`Self::bridge_list`].
    fn bridge_feed(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<FeedNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(
            node,
            id,
            children,
            Established::<ListNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Option within a [`Self::bridge_list_box`].
    ///
    /// Default: delegates to [`Self::bridge_list_item`].
    fn bridge_list_box_option(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListBoxOptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list_item(
            node,
            id,
            children,
            Established::<ListItemNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Tables / grids ────────────────────────────────────────────────────

    /// Data table (`<table>`).
    fn bridge_table(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TableNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Table row (`<tr>`).
    fn bridge_row(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Table data cell (`<td>`).
    fn bridge_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Table caption (`<caption>`).
    fn bridge_caption(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<CaptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Row group — `<thead>`, `<tbody>`, or `<tfoot>`.
    fn bridge_row_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RowGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Row header cell (`<th scope="row">`).
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_row_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<RowHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_cell(
            node,
            id,
            children,
            Established::<CellNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Column header cell (`<th scope="col">`).
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_column_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ColumnHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_cell(
            node,
            id,
            children,
            Established::<CellNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Interactive ARIA grid (keyboard-navigable table).
    ///
    /// Default: delegates to [`Self::bridge_table`].
    fn bridge_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GridNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_table(
            node,
            id,
            children,
            Established::<TableNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Cell within a grid.
    ///
    /// Default: delegates to [`Self::bridge_cell`].
    fn bridge_grid_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GridCellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_cell(
            node,
            id,
            children,
            Established::<CellNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Tree-grid (hierarchical interactive grid).
    ///
    /// Default: delegates to [`Self::bridge_tree`].
    fn bridge_tree_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TreeGridNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_tree(
            node,
            id,
            children,
            Established::<TreeNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Chromium-style list grid.
    ///
    /// Default: delegates to [`Self::bridge_grid`].
    fn bridge_list_grid(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ListGridNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_grid(
            node,
            id,
            children,
            Established::<GridNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Layout table (presentational, not data).
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_layout_table(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LayoutTableNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_generic_container(
            node,
            id,
            children,
            Established::<GenericContainerNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Row within a layout table.
    ///
    /// Default: delegates to [`Self::bridge_row`].
    fn bridge_layout_table_row(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LayoutTableRowNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_row(
            node,
            id,
            children,
            Established::<RowNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Cell within a layout table.
    ///
    /// Default: delegates to [`Self::bridge_generic_container`].
    fn bridge_layout_table_cell(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<LayoutTableCellNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_generic_container(
            node,
            id,
            children,
            Established::<GenericContainerNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Tree ──────────────────────────────────────────────────────────────

    /// Hierarchical tree widget.
    fn bridge_tree(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TreeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Node within a tree widget.
    fn bridge_tree_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TreeItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Tabs ──────────────────────────────────────────────────────────────

    /// Individual tab button.
    fn bridge_tab(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TabNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Tab strip containing tab buttons.
    fn bridge_tab_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TabListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Content panel associated with a tab.
    fn bridge_tab_panel(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<TabPanelNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    // ── Menus ─────────────────────────────────────────────────────────────

    /// Context or popup menu container.
    fn bridge_menu(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Horizontal menu bar.
    ///
    /// Default: delegates to [`Self::bridge_menu`].
    fn bridge_menu_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuBarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_menu(
            node,
            id,
            children,
            Established::<MenuNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Item within a menu.
    fn bridge_menu_item(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<MenuItemNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Option in a menu-style list popup.
    ///
    /// Default: delegates to [`Self::bridge_menu_item`].
    fn bridge_menu_list_option(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuListOptionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_menu_item(
            node,
            id,
            children,
            Established::<MenuItemNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Popup list for a combo-box or select.
    ///
    /// Default: delegates to [`Self::bridge_menu`].
    fn bridge_menu_list_popup(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuListPopupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_menu(
            node,
            id,
            children,
            Established::<MenuNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Checkbox-style menu item.
    ///
    /// Default: delegates to [`Self::bridge_check_box`].
    fn bridge_menu_item_check_box(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuItemCheckBoxNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_check_box(
            node,
            id,
            children,
            Established::<CheckBoxNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Radio-button-style menu item.
    ///
    /// Default: delegates to [`Self::bridge_radio_button`].
    fn bridge_menu_item_radio(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MenuItemRadioNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_radio_button(
            node,
            id,
            children,
            Established::<RadioButtonNodeValid>::prove(&proof),
            proofs,
        )
    }

    // ── Toolbar / navigation aids ─────────────────────────────────────────

    /// Toolbar container.
    fn bridge_toolbar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<ToolbarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Title bar (window chrome header).
    ///
    /// Default: delegates to [`Self::bridge_toolbar`].
    fn bridge_title_bar(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<TitleBarNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_toolbar(
            node,
            id,
            children,
            Established::<ToolbarNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Radio group container.
    fn bridge_radio_group(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        _: Established<RadioGroupNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>);

    /// Gauge / scalar measurement indicator.
    ///
    /// Default: delegates to [`Self::bridge_progress_indicator`].
    fn bridge_meter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<MeterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_progress_indicator(
            node,
            id,
            children,
            Established::<ProgressIndicatorNodeValid>::prove(&proof),
            proofs,
        )
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
        proof: Established<KeyboardNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Text insertion caret.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_caret(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<CaretNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// IME composition candidate.
    ///
    /// Default: delegates to [`Self::bridge_unknown`].
    fn bridge_ime_candidate(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<ImeCandidateNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(&proof),
            proofs,
        )
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
        proof: Established<PdfRootNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_document(
            node,
            id,
            children,
            Established::<DocumentNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// PDF interactive highlight / link.
    ///
    /// Default: delegates to [`Self::bridge_link`].
    fn bridge_pdf_actionable_highlight(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<PdfActionableHighlightNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(
            node,
            id,
            children,
            Established::<LinkNodeValid>::prove(&proof),
            proofs,
        )
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
        proof: Established<GraphicsDocumentNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_document(
            node,
            id,
            children,
            Established::<DocumentNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// ARIA graphics object (sub-graphic with accessible children).
    ///
    /// Default: delegates to [`Self::bridge_group`].
    fn bridge_graphics_object(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GraphicsObjectNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_group(
            node,
            id,
            children,
            Established::<GroupNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// ARIA graphics symbol (standalone meaningful graphic).
    ///
    /// Default: delegates to [`Self::bridge_image`].
    fn bridge_graphics_symbol(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<GraphicsSymbolNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_image(
            node,
            id,
            children,
            Established::<ImageNodeValid>::prove(&proof),
            proofs,
        )
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
        proof: Established<DocAbstractNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Acknowledgements section.  Default → [`Self::bridge_section`].
    fn bridge_doc_acknowledgements(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocAcknowledgementsNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Afterword section.  Default → [`Self::bridge_section`].
    fn bridge_doc_afterword(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocAfterwordNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Appendix section.  Default → [`Self::bridge_section`].
    fn bridge_doc_appendix(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocAppendixNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Back-matter link.  Default → [`Self::bridge_link`].
    fn bridge_doc_back_link(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocBackLinkNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(
            node,
            id,
            children,
            Established::<LinkNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Single bibliographic entry.  Default → [`Self::bridge_list_item`].
    fn bridge_doc_biblio_entry(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocBiblioEntryNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list_item(
            node,
            id,
            children,
            Established::<ListItemNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Bibliography section.  Default → [`Self::bridge_list`].
    fn bridge_doc_bibliography(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocBibliographyNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(
            node,
            id,
            children,
            Established::<ListNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Inline bibliography reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_biblio_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocBiblioRefNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(
            node,
            id,
            children,
            Established::<LinkNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Chapter section.  Default → [`Self::bridge_section`].
    fn bridge_doc_chapter(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocChapterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Colophon (production notes).  Default → [`Self::bridge_section`].
    fn bridge_doc_colophon(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocColophonNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Conclusion section.  Default → [`Self::bridge_section`].
    fn bridge_doc_conclusion(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocConclusionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Cover image or section.  Default → [`Self::bridge_figure`].
    fn bridge_doc_cover(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocCoverNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_figure(
            node,
            id,
            children,
            Established::<FigureNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Individual credit line.  Default → [`Self::bridge_paragraph`].
    fn bridge_doc_credit(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocCreditNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_paragraph(
            node,
            id,
            children,
            Established::<ParagraphNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Credits section.  Default → [`Self::bridge_section`].
    fn bridge_doc_credits(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocCreditsNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Dedication section.  Default → [`Self::bridge_section`].
    fn bridge_doc_dedication(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocDedicationNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Single endnote.  Default → [`Self::bridge_note`].
    fn bridge_doc_endnote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocEndnoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_note(
            node,
            id,
            children,
            Established::<NoteNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Endnotes section.  Default → [`Self::bridge_list`].
    fn bridge_doc_endnotes(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocEndnotesNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(
            node,
            id,
            children,
            Established::<ListNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Epigraph (introductory quotation).  Default → [`Self::bridge_blockquote`].
    fn bridge_doc_epigraph(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocEpigraphNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_blockquote(
            node,
            id,
            children,
            Established::<BlockquoteNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Epilogue section.  Default → [`Self::bridge_section`].
    fn bridge_doc_epilogue(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocEpilogueNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Errata section.  Default → [`Self::bridge_section`].
    fn bridge_doc_errata(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocErrataNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Example / code sample section.  Default → [`Self::bridge_section`].
    fn bridge_doc_example(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocExampleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Single footnote.  Default → [`Self::bridge_note`].
    fn bridge_doc_footnote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocFootnoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_note(
            node,
            id,
            children,
            Established::<NoteNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Foreword section.  Default → [`Self::bridge_section`].
    fn bridge_doc_foreword(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocForewordNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Glossary section.  Default → [`Self::bridge_description_list`].
    fn bridge_doc_glossary(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocGlossaryNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_description_list(
            node,
            id,
            children,
            Established::<DescriptionListNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Inline glossary term reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_gloss_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocGlossRefNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(
            node,
            id,
            children,
            Established::<LinkNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Index section.  Default → [`Self::bridge_section`].
    fn bridge_doc_index(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocIndexNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Introduction section.  Default → [`Self::bridge_section`].
    fn bridge_doc_introduction(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocIntroductionNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Inline note reference.  Default → [`Self::bridge_link`].
    fn bridge_doc_note_ref(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocNoteRefNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_link(
            node,
            id,
            children,
            Established::<LinkNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Notice / warning box.  Default → [`Self::bridge_alert`].
    fn bridge_doc_notice(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocNoticeNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_alert(
            node,
            id,
            children,
            Established::<AlertNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Page break marker.  Default → [`Self::bridge_line_break`].
    fn bridge_doc_page_break(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPageBreakNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_line_break(
            node,
            id,
            children,
            Established::<LineBreakNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Page-level footer.  Default → [`Self::bridge_section_footer`].
    fn bridge_doc_page_footer(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPageFooterNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_footer(
            node,
            id,
            children,
            Established::<SectionFooterNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Page-level header.  Default → [`Self::bridge_section_header`].
    fn bridge_doc_page_header(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPageHeaderNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section_header(
            node,
            id,
            children,
            Established::<SectionHeaderNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Page list (TOC of page numbers).  Default → [`Self::bridge_list`].
    fn bridge_doc_page_list(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPageListNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_list(
            node,
            id,
            children,
            Established::<ListNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Part / volume division.  Default → [`Self::bridge_section`].
    fn bridge_doc_part(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPartNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Preface section.  Default → [`Self::bridge_section`].
    fn bridge_doc_preface(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPrefaceNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Prologue section.  Default → [`Self::bridge_section`].
    fn bridge_doc_prologue(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPrologueNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Pull-quote.  Default → [`Self::bridge_blockquote`].
    fn bridge_doc_pullquote(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocPullquoteNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_blockquote(
            node,
            id,
            children,
            Established::<BlockquoteNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Q&A block.  Default → [`Self::bridge_group`].
    fn bridge_doc_qna(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocQnaNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_group(
            node,
            id,
            children,
            Established::<GroupNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Subtitle / sub-heading.  Default → [`Self::bridge_heading`].
    fn bridge_doc_subtitle(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocSubtitleNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_heading(
            node,
            id,
            children,
            Established::<HeadingNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Tip / hint box.  Default → [`Self::bridge_note`].
    fn bridge_doc_tip(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocTipNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_note(
            node,
            id,
            children,
            Established::<NoteNodeValid>::prove(&proof),
            proofs,
        )
    }

    /// Table of contents.  Default → [`Self::bridge_navigation`].
    fn bridge_doc_toc(
        &self,
        node: &Node,
        id: NodeId,
        children: Vec<(Self::Widget, Established<RolePreserved>)>,
        proof: Established<DocTocNodeValid>,
        proofs: WcagNodeProofs,
    ) -> (Self::Widget, Established<RolePreserved>) {
        self.bridge_navigation(
            node,
            id,
            children,
            Established::<NavigationNodeValid>::prove(&proof),
            proofs,
        )
    }
}

// ── Internal: DFS traversal + role dispatch ───────────────────────────────────

fn render_dfs<T: UiNodeBridge>(
    bridge: &T,
    nodes: &BTreeMap<NodeId, Node>,
    id: NodeId,
    stats: &mut RenderStats,
    wcag: &Established<WcagVerified>,
    node_proofs: &BTreeMap<NodeId, WcagNodeProofs>,
) -> (T::Widget, Established<RolePreserved>) {
    let proofs = node_proofs.get(&id).copied().unwrap_or_default();
    let node = match nodes.get(&id) {
        Some(n) => n,
        None => {
            let placeholder = Node::new(Role::Unknown);
            return bridge.bridge_unknown(
                &placeholder,
                id,
                vec![],
                Established::<UnknownNodeValid>::prove(wcag),
                proofs,
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
            Established::<UnknownNodeValid>::prove(wcag),
            proofs,
        );
    }

    stats.nodes_visited += 1;

    let is_container = is_container_role(node.role());
    let children: Vec<(T::Widget, Established<RolePreserved>)> = node
        .children()
        .iter()
        .map(|child_id| render_dfs(bridge, nodes, *child_id, stats, wcag, node_proofs))
        .collect();

    if is_container {
        stats.containers_rendered += 1;
    } else {
        stats.widgets_rendered += 1;
    }

    let (widget, role) = dispatch_role(bridge, node, id, children, wcag, proofs);
    let widget = bridge.wrap_widget(widget, &proofs);
    bridge.verify_node(node, &proofs);
    (widget, role)
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
    wcag: &Established<WcagVerified>,
    proofs: WcagNodeProofs,
) -> (T::Widget, Established<RolePreserved>) {
    match node.role() {
        Role::Unknown => bridge.bridge_unknown(
            node,
            id,
            children,
            Established::<UnknownNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TextRun => bridge.bridge_text_run(
            node,
            id,
            children,
            Established::<TextRunNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Cell => bridge.bridge_cell(
            node,
            id,
            children,
            Established::<CellNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Label => bridge.bridge_label(
            node,
            id,
            children,
            Established::<LabelNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Image => bridge.bridge_image(
            node,
            id,
            children,
            Established::<ImageNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Link => bridge.bridge_link(
            node,
            id,
            children,
            Established::<LinkNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Row => bridge.bridge_row(
            node,
            id,
            children,
            Established::<RowNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ListItem => bridge.bridge_list_item(
            node,
            id,
            children,
            Established::<ListItemNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ListMarker => bridge.bridge_list_marker(
            node,
            id,
            children,
            Established::<ListMarkerNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TreeItem => bridge.bridge_tree_item(
            node,
            id,
            children,
            Established::<TreeItemNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ListBoxOption => bridge.bridge_list_box_option(
            node,
            id,
            children,
            Established::<ListBoxOptionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MenuItem => bridge.bridge_menu_item(
            node,
            id,
            children,
            Established::<MenuItemNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MenuListOption => bridge.bridge_menu_list_option(
            node,
            id,
            children,
            Established::<MenuListOptionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Paragraph => bridge.bridge_paragraph(
            node,
            id,
            children,
            Established::<ParagraphNodeValid>::prove(wcag),
            proofs,
        ),
        Role::GenericContainer => bridge.bridge_generic_container(
            node,
            id,
            children,
            Established::<GenericContainerNodeValid>::prove(wcag),
            proofs,
        ),
        Role::CheckBox => bridge.bridge_check_box(
            node,
            id,
            children,
            Established::<CheckBoxNodeValid>::prove(wcag),
            proofs,
        ),
        Role::RadioButton => bridge.bridge_radio_button(
            node,
            id,
            children,
            Established::<RadioButtonNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TextInput => bridge.bridge_text_input(
            node,
            id,
            children,
            Established::<TextInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Button => bridge.bridge_button(
            node,
            id,
            children,
            Established::<ButtonNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DefaultButton => bridge.bridge_default_button(
            node,
            id,
            children,
            Established::<DefaultButtonNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Pane => bridge.bridge_pane(
            node,
            id,
            children,
            Established::<PaneNodeValid>::prove(wcag),
            proofs,
        ),
        Role::RowHeader => bridge.bridge_row_header(
            node,
            id,
            children,
            Established::<RowHeaderNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ColumnHeader => bridge.bridge_column_header(
            node,
            id,
            children,
            Established::<ColumnHeaderNodeValid>::prove(wcag),
            proofs,
        ),
        Role::RowGroup => bridge.bridge_row_group(
            node,
            id,
            children,
            Established::<RowGroupNodeValid>::prove(wcag),
            proofs,
        ),
        Role::List => bridge.bridge_list(
            node,
            id,
            children,
            Established::<ListNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Table => bridge.bridge_table(
            node,
            id,
            children,
            Established::<TableNodeValid>::prove(wcag),
            proofs,
        ),
        Role::LayoutTableCell => bridge.bridge_layout_table_cell(
            node,
            id,
            children,
            Established::<LayoutTableCellNodeValid>::prove(wcag),
            proofs,
        ),
        Role::LayoutTableRow => bridge.bridge_layout_table_row(
            node,
            id,
            children,
            Established::<LayoutTableRowNodeValid>::prove(wcag),
            proofs,
        ),
        Role::LayoutTable => bridge.bridge_layout_table(
            node,
            id,
            children,
            Established::<LayoutTableNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Switch => bridge.bridge_switch(
            node,
            id,
            children,
            Established::<SwitchNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Menu => bridge.bridge_menu(
            node,
            id,
            children,
            Established::<MenuNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MultilineTextInput => bridge.bridge_multiline_text_input(
            node,
            id,
            children,
            Established::<MultilineTextInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::SearchInput => bridge.bridge_search_input(
            node,
            id,
            children,
            Established::<SearchInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DateInput => bridge.bridge_date_input(
            node,
            id,
            children,
            Established::<DateInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DateTimeInput => bridge.bridge_date_time_input(
            node,
            id,
            children,
            Established::<DateTimeInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::WeekInput => bridge.bridge_week_input(
            node,
            id,
            children,
            Established::<WeekInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MonthInput => bridge.bridge_month_input(
            node,
            id,
            children,
            Established::<MonthInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TimeInput => bridge.bridge_time_input(
            node,
            id,
            children,
            Established::<TimeInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::EmailInput => bridge.bridge_email_input(
            node,
            id,
            children,
            Established::<EmailInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::NumberInput => bridge.bridge_number_input(
            node,
            id,
            children,
            Established::<NumberInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::PasswordInput => bridge.bridge_password_input(
            node,
            id,
            children,
            Established::<PasswordInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::PhoneNumberInput => bridge.bridge_phone_number_input(
            node,
            id,
            children,
            Established::<PhoneNumberInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::UrlInput => bridge.bridge_url_input(
            node,
            id,
            children,
            Established::<UrlInputNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Abbr => bridge.bridge_abbr(
            node,
            id,
            children,
            Established::<AbbrNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Alert => bridge.bridge_alert(
            node,
            id,
            children,
            Established::<AlertNodeValid>::prove(wcag),
            proofs,
        ),
        Role::AlertDialog => bridge.bridge_alert_dialog(
            node,
            id,
            children,
            Established::<AlertDialogNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Application => bridge.bridge_application(
            node,
            id,
            children,
            Established::<ApplicationNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Article => bridge.bridge_article(
            node,
            id,
            children,
            Established::<ArticleNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Audio => bridge.bridge_audio(
            node,
            id,
            children,
            Established::<AudioNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Banner => bridge.bridge_banner(
            node,
            id,
            children,
            Established::<BannerNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Blockquote => bridge.bridge_blockquote(
            node,
            id,
            children,
            Established::<BlockquoteNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Canvas => bridge.bridge_canvas(
            node,
            id,
            children,
            Established::<CanvasNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Caption => bridge.bridge_caption(
            node,
            id,
            children,
            Established::<CaptionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Caret => bridge.bridge_caret(
            node,
            id,
            children,
            Established::<CaretNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Code => bridge.bridge_code(
            node,
            id,
            children,
            Established::<CodeNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ColorWell => bridge.bridge_color_well(
            node,
            id,
            children,
            Established::<ColorWellNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ComboBox => bridge.bridge_combo_box(
            node,
            id,
            children,
            Established::<ComboBoxNodeValid>::prove(wcag),
            proofs,
        ),
        Role::EditableComboBox => bridge.bridge_editable_combo_box(
            node,
            id,
            children,
            Established::<EditableComboBoxNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Complementary => bridge.bridge_complementary(
            node,
            id,
            children,
            Established::<ComplementaryNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Comment => bridge.bridge_comment(
            node,
            id,
            children,
            Established::<CommentNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ContentDeletion => bridge.bridge_content_deletion(
            node,
            id,
            children,
            Established::<ContentDeletionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ContentInsertion => bridge.bridge_content_insertion(
            node,
            id,
            children,
            Established::<ContentInsertionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ContentInfo => bridge.bridge_content_info(
            node,
            id,
            children,
            Established::<ContentInfoNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Definition => bridge.bridge_definition(
            node,
            id,
            children,
            Established::<DefinitionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DescriptionList => bridge.bridge_description_list(
            node,
            id,
            children,
            Established::<DescriptionListNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Details => bridge.bridge_details(
            node,
            id,
            children,
            Established::<DetailsNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Dialog => bridge.bridge_dialog(
            node,
            id,
            children,
            Established::<DialogNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DisclosureTriangle => bridge.bridge_disclosure_triangle(
            node,
            id,
            children,
            Established::<DisclosureTriangleNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Document => bridge.bridge_document(
            node,
            id,
            children,
            Established::<DocumentNodeValid>::prove(wcag),
            proofs,
        ),
        Role::EmbeddedObject => bridge.bridge_embedded_object(
            node,
            id,
            children,
            Established::<EmbeddedObjectNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Emphasis => bridge.bridge_emphasis(
            node,
            id,
            children,
            Established::<EmphasisNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Feed => bridge.bridge_feed(
            node,
            id,
            children,
            Established::<FeedNodeValid>::prove(wcag),
            proofs,
        ),
        Role::FigureCaption => bridge.bridge_figure_caption(
            node,
            id,
            children,
            Established::<FigureCaptionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Figure => bridge.bridge_figure(
            node,
            id,
            children,
            Established::<FigureNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Footer => bridge.bridge_footer(
            node,
            id,
            children,
            Established::<FooterNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Form => bridge.bridge_form(
            node,
            id,
            children,
            Established::<FormNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Grid => bridge.bridge_grid(
            node,
            id,
            children,
            Established::<GridNodeValid>::prove(wcag),
            proofs,
        ),
        Role::GridCell => bridge.bridge_grid_cell(
            node,
            id,
            children,
            Established::<GridCellNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Group => bridge.bridge_group(
            node,
            id,
            children,
            Established::<GroupNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Header => bridge.bridge_header(
            node,
            id,
            children,
            Established::<HeaderNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Heading => bridge.bridge_heading(
            node,
            id,
            children,
            Established::<HeadingNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Iframe => bridge.bridge_iframe(
            node,
            id,
            children,
            Established::<IframeNodeValid>::prove(wcag),
            proofs,
        ),
        Role::IframePresentational => bridge.bridge_iframe_presentational(
            node,
            id,
            children,
            Established::<IframePresentationalNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ImeCandidate => bridge.bridge_ime_candidate(
            node,
            id,
            children,
            Established::<ImeCandidateNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Keyboard => bridge.bridge_keyboard(
            node,
            id,
            children,
            Established::<KeyboardNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Legend => bridge.bridge_legend(
            node,
            id,
            children,
            Established::<LegendNodeValid>::prove(wcag),
            proofs,
        ),
        Role::LineBreak => bridge.bridge_line_break(
            node,
            id,
            children,
            Established::<LineBreakNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ListBox => bridge.bridge_list_box(
            node,
            id,
            children,
            Established::<ListBoxNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Log => bridge.bridge_log(
            node,
            id,
            children,
            Established::<LogNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Main => bridge.bridge_main(
            node,
            id,
            children,
            Established::<MainNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Mark => bridge.bridge_mark(
            node,
            id,
            children,
            Established::<MarkNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Marquee => bridge.bridge_marquee(
            node,
            id,
            children,
            Established::<MarqueeNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Math => bridge.bridge_math(
            node,
            id,
            children,
            Established::<MathNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MenuBar => bridge.bridge_menu_bar(
            node,
            id,
            children,
            Established::<MenuBarNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MenuItemCheckBox => bridge.bridge_menu_item_check_box(
            node,
            id,
            children,
            Established::<MenuItemCheckBoxNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MenuItemRadio => bridge.bridge_menu_item_radio(
            node,
            id,
            children,
            Established::<MenuItemRadioNodeValid>::prove(wcag),
            proofs,
        ),
        Role::MenuListPopup => bridge.bridge_menu_list_popup(
            node,
            id,
            children,
            Established::<MenuListPopupNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Meter => bridge.bridge_meter(
            node,
            id,
            children,
            Established::<MeterNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Navigation => bridge.bridge_navigation(
            node,
            id,
            children,
            Established::<NavigationNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Note => bridge.bridge_note(
            node,
            id,
            children,
            Established::<NoteNodeValid>::prove(wcag),
            proofs,
        ),
        Role::PluginObject => bridge.bridge_plugin_object(
            node,
            id,
            children,
            Established::<PluginObjectNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ProgressIndicator => bridge.bridge_progress_indicator(
            node,
            id,
            children,
            Established::<ProgressIndicatorNodeValid>::prove(wcag),
            proofs,
        ),
        Role::RadioGroup => bridge.bridge_radio_group(
            node,
            id,
            children,
            Established::<RadioGroupNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Region => bridge.bridge_region(
            node,
            id,
            children,
            Established::<RegionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::RootWebArea => bridge.bridge_root_web_area(
            node,
            id,
            children,
            Established::<RootWebAreaNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Ruby => bridge.bridge_ruby(
            node,
            id,
            children,
            Established::<RubyNodeValid>::prove(wcag),
            proofs,
        ),
        Role::RubyAnnotation => bridge.bridge_ruby_annotation(
            node,
            id,
            children,
            Established::<RubyAnnotationNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ScrollBar => bridge.bridge_scroll_bar(
            node,
            id,
            children,
            Established::<ScrollBarNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ScrollView => bridge.bridge_scroll_view(
            node,
            id,
            children,
            Established::<ScrollViewNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Search => bridge.bridge_search(
            node,
            id,
            children,
            Established::<SearchNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Section => bridge.bridge_section(
            node,
            id,
            children,
            Established::<SectionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::SectionFooter => bridge.bridge_section_footer(
            node,
            id,
            children,
            Established::<SectionFooterNodeValid>::prove(wcag),
            proofs,
        ),
        Role::SectionHeader => bridge.bridge_section_header(
            node,
            id,
            children,
            Established::<SectionHeaderNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Slider => bridge.bridge_slider(
            node,
            id,
            children,
            Established::<SliderNodeValid>::prove(wcag),
            proofs,
        ),
        Role::SpinButton => bridge.bridge_spin_button(
            node,
            id,
            children,
            Established::<SpinButtonNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Splitter => bridge.bridge_splitter(
            node,
            id,
            children,
            Established::<SplitterNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Status => bridge.bridge_status(
            node,
            id,
            children,
            Established::<StatusNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Strong => bridge.bridge_strong(
            node,
            id,
            children,
            Established::<StrongNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Suggestion => bridge.bridge_suggestion(
            node,
            id,
            children,
            Established::<SuggestionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::SvgRoot => bridge.bridge_svg_root(
            node,
            id,
            children,
            Established::<SvgRootNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Tab => bridge.bridge_tab(
            node,
            id,
            children,
            Established::<TabNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TabList => bridge.bridge_tab_list(
            node,
            id,
            children,
            Established::<TabListNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TabPanel => bridge.bridge_tab_panel(
            node,
            id,
            children,
            Established::<TabPanelNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Term => bridge.bridge_term(
            node,
            id,
            children,
            Established::<TermNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Time => bridge.bridge_time(
            node,
            id,
            children,
            Established::<TimeNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Timer => bridge.bridge_timer(
            node,
            id,
            children,
            Established::<TimerNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TitleBar => bridge.bridge_title_bar(
            node,
            id,
            children,
            Established::<TitleBarNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Toolbar => bridge.bridge_toolbar(
            node,
            id,
            children,
            Established::<ToolbarNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Tooltip => bridge.bridge_tooltip(
            node,
            id,
            children,
            Established::<TooltipNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Tree => bridge.bridge_tree(
            node,
            id,
            children,
            Established::<TreeNodeValid>::prove(wcag),
            proofs,
        ),
        Role::TreeGrid => bridge.bridge_tree_grid(
            node,
            id,
            children,
            Established::<TreeGridNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Video => bridge.bridge_video(
            node,
            id,
            children,
            Established::<VideoNodeValid>::prove(wcag),
            proofs,
        ),
        Role::WebView => bridge.bridge_web_view(
            node,
            id,
            children,
            Established::<WebViewNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Window => bridge.bridge_window(
            node,
            id,
            children,
            Established::<WindowNodeValid>::prove(wcag),
            proofs,
        ),
        Role::PdfActionableHighlight => bridge.bridge_pdf_actionable_highlight(
            node,
            id,
            children,
            Established::<PdfActionableHighlightNodeValid>::prove(wcag),
            proofs,
        ),
        Role::PdfRoot => bridge.bridge_pdf_root(
            node,
            id,
            children,
            Established::<PdfRootNodeValid>::prove(wcag),
            proofs,
        ),
        Role::GraphicsDocument => bridge.bridge_graphics_document(
            node,
            id,
            children,
            Established::<GraphicsDocumentNodeValid>::prove(wcag),
            proofs,
        ),
        Role::GraphicsObject => bridge.bridge_graphics_object(
            node,
            id,
            children,
            Established::<GraphicsObjectNodeValid>::prove(wcag),
            proofs,
        ),
        Role::GraphicsSymbol => bridge.bridge_graphics_symbol(
            node,
            id,
            children,
            Established::<GraphicsSymbolNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocAbstract => bridge.bridge_doc_abstract(
            node,
            id,
            children,
            Established::<DocAbstractNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocAcknowledgements => bridge.bridge_doc_acknowledgements(
            node,
            id,
            children,
            Established::<DocAcknowledgementsNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocAfterword => bridge.bridge_doc_afterword(
            node,
            id,
            children,
            Established::<DocAfterwordNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocAppendix => bridge.bridge_doc_appendix(
            node,
            id,
            children,
            Established::<DocAppendixNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocBackLink => bridge.bridge_doc_back_link(
            node,
            id,
            children,
            Established::<DocBackLinkNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocBiblioEntry => bridge.bridge_doc_biblio_entry(
            node,
            id,
            children,
            Established::<DocBiblioEntryNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocBibliography => bridge.bridge_doc_bibliography(
            node,
            id,
            children,
            Established::<DocBibliographyNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocBiblioRef => bridge.bridge_doc_biblio_ref(
            node,
            id,
            children,
            Established::<DocBiblioRefNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocChapter => bridge.bridge_doc_chapter(
            node,
            id,
            children,
            Established::<DocChapterNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocColophon => bridge.bridge_doc_colophon(
            node,
            id,
            children,
            Established::<DocColophonNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocConclusion => bridge.bridge_doc_conclusion(
            node,
            id,
            children,
            Established::<DocConclusionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocCover => bridge.bridge_doc_cover(
            node,
            id,
            children,
            Established::<DocCoverNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocCredit => bridge.bridge_doc_credit(
            node,
            id,
            children,
            Established::<DocCreditNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocCredits => bridge.bridge_doc_credits(
            node,
            id,
            children,
            Established::<DocCreditsNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocDedication => bridge.bridge_doc_dedication(
            node,
            id,
            children,
            Established::<DocDedicationNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocEndnote => bridge.bridge_doc_endnote(
            node,
            id,
            children,
            Established::<DocEndnoteNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocEndnotes => bridge.bridge_doc_endnotes(
            node,
            id,
            children,
            Established::<DocEndnotesNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocEpigraph => bridge.bridge_doc_epigraph(
            node,
            id,
            children,
            Established::<DocEpigraphNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocEpilogue => bridge.bridge_doc_epilogue(
            node,
            id,
            children,
            Established::<DocEpilogueNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocErrata => bridge.bridge_doc_errata(
            node,
            id,
            children,
            Established::<DocErrataNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocExample => bridge.bridge_doc_example(
            node,
            id,
            children,
            Established::<DocExampleNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocFootnote => bridge.bridge_doc_footnote(
            node,
            id,
            children,
            Established::<DocFootnoteNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocForeword => bridge.bridge_doc_foreword(
            node,
            id,
            children,
            Established::<DocForewordNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocGlossary => bridge.bridge_doc_glossary(
            node,
            id,
            children,
            Established::<DocGlossaryNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocGlossRef => bridge.bridge_doc_gloss_ref(
            node,
            id,
            children,
            Established::<DocGlossRefNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocIndex => bridge.bridge_doc_index(
            node,
            id,
            children,
            Established::<DocIndexNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocIntroduction => bridge.bridge_doc_introduction(
            node,
            id,
            children,
            Established::<DocIntroductionNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocNoteRef => bridge.bridge_doc_note_ref(
            node,
            id,
            children,
            Established::<DocNoteRefNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocNotice => bridge.bridge_doc_notice(
            node,
            id,
            children,
            Established::<DocNoticeNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPageBreak => bridge.bridge_doc_page_break(
            node,
            id,
            children,
            Established::<DocPageBreakNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPageFooter => bridge.bridge_doc_page_footer(
            node,
            id,
            children,
            Established::<DocPageFooterNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPageHeader => bridge.bridge_doc_page_header(
            node,
            id,
            children,
            Established::<DocPageHeaderNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPageList => bridge.bridge_doc_page_list(
            node,
            id,
            children,
            Established::<DocPageListNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPart => bridge.bridge_doc_part(
            node,
            id,
            children,
            Established::<DocPartNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPreface => bridge.bridge_doc_preface(
            node,
            id,
            children,
            Established::<DocPrefaceNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPrologue => bridge.bridge_doc_prologue(
            node,
            id,
            children,
            Established::<DocPrologueNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocPullquote => bridge.bridge_doc_pullquote(
            node,
            id,
            children,
            Established::<DocPullquoteNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocQna => bridge.bridge_doc_qna(
            node,
            id,
            children,
            Established::<DocQnaNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocSubtitle => bridge.bridge_doc_subtitle(
            node,
            id,
            children,
            Established::<DocSubtitleNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocTip => bridge.bridge_doc_tip(
            node,
            id,
            children,
            Established::<DocTipNodeValid>::prove(wcag),
            proofs,
        ),
        Role::DocToc => bridge.bridge_doc_toc(
            node,
            id,
            children,
            Established::<DocTocNodeValid>::prove(wcag),
            proofs,
        ),
        Role::ListGrid => bridge.bridge_list_grid(
            node,
            id,
            children,
            Established::<ListGridNodeValid>::prove(wcag),
            proofs,
        ),
        Role::Terminal => bridge.bridge_terminal(
            node,
            id,
            children,
            Established::<TerminalNodeValid>::prove(wcag),
            proofs,
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
        let wcag = Established::<WcagVerified>::prove(tree);
        let (widget, _) = render_dfs(
            self,
            tree.nodes(),
            tree.root(),
            &mut stats,
            &wcag,
            tree.node_proofs(),
        );
        tracing::debug!(
            visited = stats.nodes_visited,
            widgets = stats.widgets_rendered,
            containers = stats.containers_rendered,
            skipped = stats.nodes_skipped,
            "render complete"
        );
        Ok((widget, stats, Established::<RenderComplete>::prove(&wcag)))
    }

    #[instrument(skip(self, tree), fields(backend = self.backend_name(), root = ?subtree_root))]
    fn render_partial(
        &self,
        subtree_root: WidgetId,
        tree: &VerifiedTree,
    ) -> UiResult<(Self::Widget, RenderStats)> {
        let mut stats = RenderStats::default();
        let wcag = Established::<WcagVerified>::prove(tree);
        let (widget, _) = render_dfs(
            self,
            tree.nodes(),
            subtree_root.to_node_id(),
            &mut stats,
            &wcag,
            tree.node_proofs(),
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
