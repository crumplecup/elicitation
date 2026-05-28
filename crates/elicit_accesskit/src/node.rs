//! JSON-serializable intermediate representation of [`accesskit::Node`].
//!
//! `accesskit::Node` does not implement `Serialize`/`Deserialize` even when
//! accesskit's `serde` feature is enabled (only its internal `Properties`
//! struct does). This module provides [`NodeJson`] as a complete, serializable
//! view of a node that converts losslessly to/from `accesskit::Node`.
//!
//! # Design
//!
//! [`NodeJson`] covers all public node properties exposed by accesskit 0.24:
//! - Structural: `role`, `children`, relationship vecs
//! - Text: `label`, `description`, `value`, `placeholder`, `tooltip`, `url`
//! - State flags: `is_disabled`, `is_hidden`, `is_read_only`, etc.
//! - Numeric: `numeric_value`, `min/max_numeric_value`, table indices, etc.
//! - Style/text: `font_family`, `font_size`, `font_weight`, text decorations
//! - Enums: `invalid`, `toggled`, `orientation`, `live`, `sort_direction`, etc.
//! - Actions: `actions` as a sorted `Vec<Action>`
//! - Geometry: `bounds`, `transform`
//!
//! # Usage
//!
//! ```rust
//! use elicit_accesskit::{NodeJson, Role};
//! use accesskit::Role as AkRole;
//!
//! let json = NodeJson::new(Role(AkRole::Button))
//!     .with_label("Save".to_string())
//!     .with_is_disabled(false);
//!
//! let node = accesskit::Node::from(json.clone());
//! let roundtrip = NodeJson::from(&node);
//! assert_eq!(json.role, roundtrip.role);
//! assert_eq!(json.label, roundtrip.label);
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    Action, Affine, AriaCurrent, AutoComplete, Color, CustomAction, HasPopup, Invalid, ListStyle,
    Live, NodeId, Orientation, Rect, SortDirection, TextAlign, TextDecoration, TextDirection,
    TextSelection, Toggled, TreeId, VerticalOffset,
};

/// JSON-serializable representation of an [`accesskit::Node`].
///
/// Covers all public properties from the accesskit 0.24 API. Fields that are
/// absent on a node are `None` or empty `Vec`; the `From` conversions are
/// lossless for all currently-exposed properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct NodeJson {
    // ── Core ─────────────────────────────────────────────────────────────────
    /// The accessibility role of this node.
    pub role: crate::Role,

    /// IDs of this node's children in document order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeId>,

    /// Actions supported by this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<Action>,

    // ── Text properties ──────────────────────────────────────────────────────
    /// The accessible label (primary name). Use [`NodeJson::description`] for
    /// supplementary information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// Supplementary description beyond the primary label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The current value of a control (e.g. text field content, slider value
    /// as a string).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Placeholder text shown when the control is empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// Tooltip text. Use only when the tooltip is the sole source of a name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,

    /// URL associated with this node (e.g. for links).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Access key — a single character that activates this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,

    /// Full keyboard shortcut string (e.g. "Ctrl+S").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyboard_shortcut: Option<String>,

    /// Role description override for custom controls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_description: Option<String>,

    /// State description override (replaces defaults like "checked").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_description: Option<String>,

    /// Author-assigned automation ID (must be unique among siblings).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,

    /// CSS class name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,

    /// HTML tag name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_tag: Option<String>,

    /// Font family name (only when different from parent).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Language tag (BCP 47, only when different from parent).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Inner HTML (used only for top-level MathML nodes).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_html: Option<String>,

    // ── Boolean flags ────────────────────────────────────────────────────────
    /// Whether this node is excluded from the accessibility tree.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_hidden: bool,

    /// Whether this node (or its group) disallows user input.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_disabled: bool,

    /// Whether this text widget allows focus/selection but not editing.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_read_only: bool,

    /// Whether this field must be filled before form submission.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_required: bool,

    /// Whether multiple items can be selected simultaneously.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_multiselectable: bool,

    /// Whether this is a modal dialog.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_modal: bool,

    /// Whether this node is in a busy/loading state.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_busy: bool,

    /// Whether live region updates should be presented atomically.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_live_atomic: bool,

    /// Whether the node clips its children.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clips_children: bool,

    /// Whether the node is marked as bold.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_italic: bool,

    /// Whether the node causes a hard line break.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_line_breaking_object: bool,

    /// Whether the node has been visited (e.g. a visited link).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_visited: bool,

    /// Whether this node allows touch pass-through.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_touch_transparent: bool,

    /// Whether this node is selected (`Some(true)`), not selected (`Some(false)`),
    /// or selection doesn't apply (`None`).
    ///
    /// Maps to `accesskit::Node::is_selected` / `set_selected` / `clear_selected`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,

    // ── Render IR extension ──────────────────────────────────────────────────
    /// Rich-text render payload (serialised [`elicit_ui::ParagraphText`]).
    ///
    /// When set on a `Paragraph` node, the render backend uses this for
    /// per-span styling instead of applying a whole-widget style.  The value
    /// is stored as raw JSON so `elicit_accesskit` stays independent of
    /// `elicit_ui`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<serde_json::Value>,

    // ── Enum state properties ────────────────────────────────────────────────
    /// Whether the input value is invalid, and why.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalid: Option<Invalid>,

    /// Toggle state (true / false / mixed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toggled: Option<Toggled>,

    /// Orientation of a scrollbar, slider, or similar element.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<Orientation>,

    /// Text direction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_direction: Option<TextDirection>,

    /// Sorting direction for a table column or row header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,

    /// Which element the node represents in a multi-step process
    /// (ARIA `aria-current`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aria_current: Option<AriaCurrent>,

    /// Autocomplete behavior for a combobox or textbox.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_complete: Option<AutoComplete>,

    /// Live region politeness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live: Option<Live>,

    /// Whether a popup is attached, and what kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_popup: Option<HasPopup>,

    /// List marker style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_style: Option<ListStyle>,

    /// Text alignment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_align: Option<TextAlign>,

    /// Vertical text offset (subscript / superscript).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertical_offset: Option<VerticalOffset>,

    // ── Numeric properties ───────────────────────────────────────────────────
    /// Current numeric value of a range control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_value: Option<f64>,

    /// Minimum allowed numeric value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_numeric_value: Option<f64>,

    /// Maximum allowed numeric value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_numeric_value: Option<f64>,

    /// Step size for incrementing/decrementing the numeric value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_value_step: Option<f64>,

    /// Jump size for page increment/decrement actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_value_jump: Option<f64>,

    /// Font size in points.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f32>,

    /// Font weight.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<f32>,

    // ── Table properties ─────────────────────────────────────────────────────
    /// Number of rows (for tables/grids).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_count: Option<usize>,

    /// Number of columns (for tables/grids).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_count: Option<usize>,

    /// Zero-based row index of this cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_index: Option<usize>,

    /// Zero-based column index of this cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_index: Option<usize>,

    /// Number of rows this cell spans.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_span: Option<usize>,

    /// Number of columns this cell spans.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_span: Option<usize>,

    // ── Set/list properties ──────────────────────────────────────────────────
    /// Hierarchical level (e.g. heading level 1–6).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<usize>,

    /// One-based position of this item within its set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_in_set: Option<usize>,

    /// Total size of the set this item belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_of_set: Option<usize>,

    // ── Scroll properties ────────────────────────────────────────────────────
    /// Current horizontal scroll offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_x: Option<f64>,

    /// Minimum horizontal scroll offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_x_min: Option<f64>,

    /// Maximum horizontal scroll offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_x_max: Option<f64>,

    /// Current vertical scroll offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_y: Option<f64>,

    /// Minimum vertical scroll offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_y_min: Option<f64>,

    /// Maximum vertical scroll offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_y_max: Option<f64>,

    // ── Color / text decoration ──────────────────────────────────────────────
    /// Foreground (text) color as RGBA packed u32.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<Color>,

    /// Background color as RGBA packed u32.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,

    /// Overline decoration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overline: Option<TextDecoration>,

    /// Strikethrough decoration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<TextDecoration>,

    /// Underline decoration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub underline: Option<TextDecoration>,

    // ── Geometry ─────────────────────────────────────────────────────────────
    /// Bounding rectangle in the node's coordinate space.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounds: Option<Rect>,

    /// 2D affine transform relative to the parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<Affine>,

    // ── Relationship node-id vecs ─────────────────────────────────────────────
    /// IDs of nodes that this node controls (ARIA `aria-controls`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub controls: Vec<NodeId>,

    /// IDs of nodes that provide details for this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<NodeId>,

    /// IDs of nodes that describe this node (ARIA `aria-describedby`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub described_by: Vec<NodeId>,

    /// IDs of nodes that the reading order flows to from this node.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub flow_to: Vec<NodeId>,

    /// IDs of nodes that label this node (ARIA `aria-labelledby`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labelled_by: Vec<NodeId>,

    /// IDs of nodes owned by this node but not descendants in the tree.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owns: Vec<NodeId>,

    /// IDs of all radio buttons in the same group as this one.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub radio_group: Vec<NodeId>,

    // ── Relationship single-node IDs ──────────────────────────────────────────
    /// The active descendant when focus stays on this container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_descendant: Option<NodeId>,

    /// The node that describes an error for this input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<NodeId>,

    /// The in-page link target for this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_page_link_target: Option<NodeId>,

    /// The member-of grouping node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_of: Option<NodeId>,

    /// The next node on the same text line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_on_line: Option<NodeId>,

    /// The previous node on the same text line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_on_line: Option<NodeId>,

    /// The popup node for this node (e.g. a combobox dropdown).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popup_for: Option<NodeId>,

    // ── Subtree graft ─────────────────────────────────────────────────────────
    /// If set, this node grafts a subtree with the specified tree ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<TreeId>,

    // ── Text analytics ────────────────────────────────────────────────────────
    /// Text selection range (anchor + focus positions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_selection: Option<TextSelection>,

    // ── Custom actions ────────────────────────────────────────────────────────
    /// Custom application-defined actions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_actions: Vec<CustomAction>,
}

impl NodeJson {
    /// Creates a new `NodeJson` with the given role and all other fields at
    /// their defaults (empty / `None` / `false`).
    pub fn new(role: crate::Role) -> Self {
        Self {
            role,
            children: Vec::new(),
            actions: Vec::new(),
            label: None,
            description: None,
            value: None,
            placeholder: None,
            tooltip: None,
            url: None,
            access_key: None,
            keyboard_shortcut: None,
            role_description: None,
            state_description: None,
            author_id: None,
            class_name: None,
            html_tag: None,
            font_family: None,
            language: None,
            inner_html: None,
            is_hidden: false,
            is_disabled: false,
            is_read_only: false,
            is_required: false,
            is_multiselectable: false,
            is_modal: false,
            is_busy: false,
            is_live_atomic: false,
            clips_children: false,
            is_italic: false,
            is_line_breaking_object: false,
            is_visited: false,
            is_touch_transparent: false,
            is_selected: None,
            rich_text: None,
            invalid: None,
            toggled: None,
            orientation: None,
            text_direction: None,
            sort_direction: None,
            aria_current: None,
            auto_complete: None,
            live: None,
            has_popup: None,
            list_style: None,
            text_align: None,
            vertical_offset: None,
            numeric_value: None,
            min_numeric_value: None,
            max_numeric_value: None,
            numeric_value_step: None,
            numeric_value_jump: None,
            font_size: None,
            font_weight: None,
            row_count: None,
            column_count: None,
            row_index: None,
            column_index: None,
            row_span: None,
            column_span: None,
            level: None,
            position_in_set: None,
            size_of_set: None,
            scroll_x: None,
            scroll_x_min: None,
            scroll_x_max: None,
            scroll_y: None,
            scroll_y_min: None,
            scroll_y_max: None,
            foreground_color: None,
            background_color: None,
            overline: None,
            strikethrough: None,
            underline: None,
            bounds: None,
            transform: None,
            controls: Vec::new(),
            details: Vec::new(),
            described_by: Vec::new(),
            flow_to: Vec::new(),
            labelled_by: Vec::new(),
            owns: Vec::new(),
            radio_group: Vec::new(),
            active_descendant: None,
            error_message: None,
            in_page_link_target: None,
            member_of: None,
            next_on_line: None,
            previous_on_line: None,
            popup_for: None,
            tree_id: None,
            text_selection: None,
            custom_actions: Vec::new(),
        }
    }

    // ── Builder methods ───────────────────────────────────────────────────────

    /// Sets the accessible label.
    pub fn with_label(mut self, v: String) -> Self {
        self.label = Some(v);
        self
    }

    /// Sets the description.
    pub fn with_description(mut self, v: String) -> Self {
        self.description = Some(v);
        self
    }

    /// Sets the CSS class name.
    pub fn with_class_name(mut self, v: String) -> Self {
        self.class_name = Some(v);
        self
    }

    /// Sets the value.
    pub fn with_value(mut self, v: String) -> Self {
        self.value = Some(v);
        self
    }

    /// Sets the placeholder.
    pub fn with_placeholder(mut self, v: String) -> Self {
        self.placeholder = Some(v);
        self
    }

    /// Sets the children.
    pub fn with_children(mut self, v: Vec<NodeId>) -> Self {
        self.children = v;
        self
    }

    /// Appends a child node ID.
    pub fn push_child(mut self, id: NodeId) -> Self {
        self.children.push(id);
        self
    }

    /// Adds a supported action.
    pub fn push_action(mut self, action: Action) -> Self {
        if !self.actions.contains(&action) {
            self.actions.push(action);
        }
        self
    }

    /// Sets the bounds.
    pub fn with_bounds(mut self, v: Rect) -> Self {
        self.bounds = Some(v);
        self
    }

    /// Sets `is_disabled`.
    pub fn with_is_disabled(mut self, v: bool) -> Self {
        self.is_disabled = v;
        self
    }

    /// Sets `is_hidden`.
    pub fn with_is_hidden(mut self, v: bool) -> Self {
        self.is_hidden = v;
        self
    }

    /// Sets `is_required`.
    pub fn with_is_required(mut self, v: bool) -> Self {
        self.is_required = v;
        self
    }

    /// Sets `is_read_only`.
    pub fn with_is_read_only(mut self, v: bool) -> Self {
        self.is_read_only = v;
        self
    }

    /// Sets the numeric value.
    pub fn with_numeric_value(mut self, v: f64) -> Self {
        self.numeric_value = Some(v);
        self
    }

    /// Sets the toggled state.
    pub fn with_toggled(mut self, v: Toggled) -> Self {
        self.toggled = Some(v);
        self
    }

    /// Sets the invalid state.
    pub fn with_invalid(mut self, v: Invalid) -> Self {
        self.invalid = Some(v);
        self
    }

    /// Sets the selected state.
    ///
    /// `Some(true)` = selected, `Some(false)` = explicitly not selected,
    /// `None` = selection not applicable (the default).
    pub fn with_selected(mut self, v: bool) -> Self {
        self.is_selected = Some(v);
        self
    }

    /// Attaches a rich-text render payload (serialised `elicit_ui::ParagraphText`).
    ///
    /// When set, the ratatui bridge renders this as a styled `Paragraph`
    /// with per-span colours instead of highlighting the whole widget.
    pub fn with_rich_text_value(mut self, value: serde_json::Value) -> Self {
        self.rich_text = Some(value);
        self
    }
}

// ── From<&accesskit::Node> ────────────────────────────────────────────────────

impl From<&accesskit::Node> for NodeJson {
    fn from(n: &accesskit::Node) -> Self {
        use accesskit::Action;

        let all_actions = [
            Action::Click,
            Action::Focus,
            Action::Blur,
            Action::Collapse,
            Action::Expand,
            Action::CustomAction,
            Action::Decrement,
            Action::Increment,
            Action::HideTooltip,
            Action::ShowTooltip,
            Action::ReplaceSelectedText,
            Action::ScrollDown,
            Action::ScrollLeft,
            Action::ScrollRight,
            Action::ScrollUp,
            Action::ScrollIntoView,
            Action::ScrollToPoint,
            Action::SetScrollOffset,
            Action::SetTextSelection,
            Action::SetSequentialFocusNavigationStartingPoint,
            Action::SetValue,
            Action::ShowContextMenu,
        ];

        let actions = all_actions
            .iter()
            .filter(|&&a| n.supports_action(a))
            .map(|&a| crate::Action(a))
            .collect();

        NodeJson {
            role: crate::Role(n.role()),
            children: n.children().iter().copied().map(NodeId).collect(),
            actions,
            label: n.label().map(str::to_owned),
            description: n.description().map(str::to_owned),
            value: n.value().map(str::to_owned),
            placeholder: n.placeholder().map(str::to_owned),
            tooltip: n.tooltip().map(str::to_owned),
            url: n.url().map(str::to_owned),
            access_key: n.access_key().map(str::to_owned),
            keyboard_shortcut: n.keyboard_shortcut().map(str::to_owned),
            role_description: n.role_description().map(str::to_owned),
            state_description: n.state_description().map(str::to_owned),
            author_id: n.author_id().map(str::to_owned),
            class_name: n.class_name().map(str::to_owned),
            html_tag: n.html_tag().map(str::to_owned),
            font_family: n.font_family().map(str::to_owned),
            language: n.language().map(str::to_owned),
            inner_html: n.inner_html().map(str::to_owned),
            is_hidden: n.is_hidden(),
            is_disabled: n.is_disabled(),
            is_read_only: n.is_read_only(),
            is_required: n.is_required(),
            is_multiselectable: n.is_multiselectable(),
            is_modal: n.is_modal(),
            is_busy: n.is_busy(),
            is_live_atomic: n.is_live_atomic(),
            clips_children: n.clips_children(),
            is_italic: n.is_italic(),
            is_line_breaking_object: n.is_line_breaking_object(),
            is_visited: n.is_visited(),
            is_touch_transparent: n.is_touch_transparent(),
            is_selected: n.is_selected(),
            rich_text: None,
            invalid: n.invalid().map(Invalid),
            toggled: n.toggled().map(Toggled),
            orientation: n.orientation().map(Orientation),
            text_direction: n.text_direction().map(TextDirection),
            sort_direction: n.sort_direction().map(SortDirection),
            aria_current: n.aria_current().map(AriaCurrent),
            auto_complete: n.auto_complete().map(AutoComplete),
            live: n.live().map(Live),
            has_popup: n.has_popup().map(HasPopup),
            list_style: n.list_style().map(ListStyle),
            text_align: n.text_align().map(TextAlign),
            vertical_offset: n.vertical_offset().map(VerticalOffset),
            numeric_value: n.numeric_value(),
            min_numeric_value: n.min_numeric_value(),
            max_numeric_value: n.max_numeric_value(),
            numeric_value_step: n.numeric_value_step(),
            numeric_value_jump: n.numeric_value_jump(),
            font_size: n.font_size(),
            font_weight: n.font_weight(),
            row_count: n.row_count(),
            column_count: n.column_count(),
            row_index: n.row_index(),
            column_index: n.column_index(),
            row_span: n.row_span(),
            column_span: n.column_span(),
            level: n.level(),
            position_in_set: n.position_in_set(),
            size_of_set: n.size_of_set(),
            scroll_x: n.scroll_x(),
            scroll_x_min: n.scroll_x_min(),
            scroll_x_max: n.scroll_x_max(),
            scroll_y: n.scroll_y(),
            scroll_y_min: n.scroll_y_min(),
            scroll_y_max: n.scroll_y_max(),
            foreground_color: n.foreground_color().map(Color::from),
            background_color: n.background_color().map(Color::from),
            overline: n.overline().map(TextDecoration::from),
            strikethrough: n.strikethrough().map(TextDecoration::from),
            underline: n.underline().map(TextDecoration::from),
            bounds: n.bounds().map(Rect),
            transform: n.transform().map(|a| Affine(*a)),
            controls: n.controls().iter().copied().map(NodeId).collect(),
            details: n.details().iter().copied().map(NodeId).collect(),
            described_by: n.described_by().iter().copied().map(NodeId).collect(),
            flow_to: n.flow_to().iter().copied().map(NodeId).collect(),
            labelled_by: n.labelled_by().iter().copied().map(NodeId).collect(),
            owns: n.owns().iter().copied().map(NodeId).collect(),
            radio_group: n.radio_group().iter().copied().map(NodeId).collect(),
            active_descendant: n.active_descendant().map(NodeId),
            error_message: n.error_message().map(NodeId),
            in_page_link_target: n.in_page_link_target().map(NodeId),
            member_of: n.member_of().map(NodeId),
            next_on_line: n.next_on_line().map(NodeId),
            previous_on_line: n.previous_on_line().map(NodeId),
            popup_for: n.popup_for().map(NodeId),
            tree_id: n.tree_id().map(TreeId),
            text_selection: n.text_selection().map(|s| TextSelection(*s)),
            custom_actions: n
                .custom_actions()
                .iter()
                .cloned()
                .map(CustomAction)
                .collect(),
        }
    }
}

// ── From<NodeJson> for accesskit::Node ───────────────────────────────────────

impl From<NodeJson> for accesskit::Node {
    fn from(j: NodeJson) -> Self {
        let mut n = accesskit::Node::new(j.role.0);

        // Children
        if !j.children.is_empty() {
            n.set_children(j.children.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }

        // Actions
        for action in j.actions {
            n.add_action(action.0);
        }

        // Text properties
        if let Some(v) = j.label {
            n.set_label(v);
        }
        if let Some(v) = j.description {
            n.set_description(v);
        }
        if let Some(v) = j.value {
            n.set_value(v);
        }
        if let Some(v) = j.placeholder {
            n.set_placeholder(v);
        }
        if let Some(v) = j.tooltip {
            n.set_tooltip(v);
        }
        if let Some(v) = j.url {
            n.set_url(v);
        }
        if let Some(v) = j.access_key {
            n.set_access_key(v);
        }
        if let Some(v) = j.keyboard_shortcut {
            n.set_keyboard_shortcut(v);
        }
        if let Some(v) = j.role_description {
            n.set_role_description(v);
        }
        if let Some(v) = j.state_description {
            n.set_state_description(v);
        }
        if let Some(v) = j.author_id {
            n.set_author_id(v);
        }
        // Encode rich_text as a class_name sentinel so the ratatui bridge can
        // recover it from the accesskit::Node (which has no native rich_text slot).
        if let Some(rt_value) = j.rich_text {
            if let Ok(encoded) = serde_json::to_string(&rt_value) {
                n.set_class_name(format!("__rich_text__:{encoded}"));
            }
        } else if let Some(v) = j.class_name {
            n.set_class_name(v);
        }
        if let Some(v) = j.html_tag {
            n.set_html_tag(v);
        }
        if let Some(v) = j.font_family {
            n.set_font_family(v);
        }
        if let Some(v) = j.language {
            n.set_language(v);
        }
        if let Some(v) = j.inner_html {
            n.set_inner_html(v);
        }

        // Boolean flags
        if j.is_hidden {
            n.set_hidden();
        }
        if j.is_disabled {
            n.set_disabled();
        }
        if j.is_read_only {
            n.set_read_only();
        }
        if j.is_required {
            n.set_required();
        }
        if j.is_multiselectable {
            n.set_multiselectable();
        }
        if j.is_modal {
            n.set_modal();
        }
        if j.is_busy {
            n.set_busy();
        }
        if j.is_live_atomic {
            n.set_live_atomic();
        }
        if j.clips_children {
            n.set_clips_children();
        }
        if j.is_italic {
            n.set_italic();
        }
        if j.is_line_breaking_object {
            n.set_is_line_breaking_object();
        }
        if j.is_visited {
            n.set_visited();
        }
        if j.is_touch_transparent {
            n.set_touch_transparent();
        }
        if let Some(v) = j.is_selected {
            n.set_selected(v);
        }

        // Enum state
        if let Some(v) = j.invalid {
            n.set_invalid(v.0);
        }
        if let Some(v) = j.toggled {
            n.set_toggled(v.0);
        }
        if let Some(v) = j.orientation {
            n.set_orientation(v.0);
        }
        if let Some(v) = j.text_direction {
            n.set_text_direction(v.0);
        }
        if let Some(v) = j.sort_direction {
            n.set_sort_direction(v.0);
        }
        if let Some(v) = j.aria_current {
            n.set_aria_current(v.0);
        }
        if let Some(v) = j.auto_complete {
            n.set_auto_complete(v.0);
        }
        if let Some(v) = j.live {
            n.set_live(v.0);
        }
        if let Some(v) = j.has_popup {
            n.set_has_popup(v.0);
        }
        if let Some(v) = j.list_style {
            n.set_list_style(v.0);
        }
        if let Some(v) = j.text_align {
            n.set_text_align(v.0);
        }
        if let Some(v) = j.vertical_offset {
            n.set_vertical_offset(v.0);
        }

        // Numeric
        if let Some(v) = j.numeric_value {
            n.set_numeric_value(v);
        }
        if let Some(v) = j.min_numeric_value {
            n.set_min_numeric_value(v);
        }
        if let Some(v) = j.max_numeric_value {
            n.set_max_numeric_value(v);
        }
        if let Some(v) = j.numeric_value_step {
            n.set_numeric_value_step(v);
        }
        if let Some(v) = j.numeric_value_jump {
            n.set_numeric_value_jump(v);
        }
        if let Some(v) = j.font_size {
            n.set_font_size(v);
        }
        if let Some(v) = j.font_weight {
            n.set_font_weight(v);
        }

        // Table
        if let Some(v) = j.row_count {
            n.set_row_count(v);
        }
        if let Some(v) = j.column_count {
            n.set_column_count(v);
        }
        if let Some(v) = j.row_index {
            n.set_row_index(v);
        }
        if let Some(v) = j.column_index {
            n.set_column_index(v);
        }
        if let Some(v) = j.row_span {
            n.set_row_span(v);
        }
        if let Some(v) = j.column_span {
            n.set_column_span(v);
        }

        // Set/list
        if let Some(v) = j.level {
            n.set_level(v);
        }
        if let Some(v) = j.position_in_set {
            n.set_position_in_set(v);
        }
        if let Some(v) = j.size_of_set {
            n.set_size_of_set(v);
        }

        // Scroll
        if let Some(v) = j.scroll_x {
            n.set_scroll_x(v);
        }
        if let Some(v) = j.scroll_x_min {
            n.set_scroll_x_min(v);
        }
        if let Some(v) = j.scroll_x_max {
            n.set_scroll_x_max(v);
        }
        if let Some(v) = j.scroll_y {
            n.set_scroll_y(v);
        }
        if let Some(v) = j.scroll_y_min {
            n.set_scroll_y_min(v);
        }
        if let Some(v) = j.scroll_y_max {
            n.set_scroll_y_max(v);
        }

        // Color / text decoration
        if let Some(v) = j.foreground_color {
            n.set_foreground_color(v.into());
        }
        if let Some(v) = j.background_color {
            n.set_background_color(v.into());
        }
        if let Some(v) = j.overline {
            n.set_overline(v.into());
        }
        if let Some(v) = j.strikethrough {
            n.set_strikethrough(v.into());
        }
        if let Some(v) = j.underline {
            n.set_underline(v.into());
        }

        // Geometry
        if let Some(v) = j.bounds {
            n.set_bounds(v.0);
        }
        if let Some(v) = j.transform {
            n.set_transform(v.0);
        }

        // Relationship vecs
        if !j.controls.is_empty() {
            n.set_controls(j.controls.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }
        if !j.details.is_empty() {
            n.set_details(j.details.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }
        if !j.described_by.is_empty() {
            n.set_described_by(
                j.described_by
                    .into_iter()
                    .map(|id| id.0)
                    .collect::<Vec<_>>(),
            );
        }
        if !j.flow_to.is_empty() {
            n.set_flow_to(j.flow_to.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }
        if !j.labelled_by.is_empty() {
            n.set_labelled_by(j.labelled_by.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }
        if !j.owns.is_empty() {
            n.set_owns(j.owns.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }
        if !j.radio_group.is_empty() {
            n.set_radio_group(j.radio_group.into_iter().map(|id| id.0).collect::<Vec<_>>());
        }

        // Single-node relationships
        if let Some(v) = j.active_descendant {
            n.set_active_descendant(v.0);
        }
        if let Some(v) = j.error_message {
            n.set_error_message(v.0);
        }
        if let Some(v) = j.in_page_link_target {
            n.set_in_page_link_target(v.0);
        }
        if let Some(v) = j.member_of {
            n.set_member_of(v.0);
        }
        if let Some(v) = j.next_on_line {
            n.set_next_on_line(v.0);
        }
        if let Some(v) = j.previous_on_line {
            n.set_previous_on_line(v.0);
        }
        if let Some(v) = j.popup_for {
            n.set_popup_for(v.0);
        }
        if let Some(v) = j.tree_id {
            n.set_tree_id(v.0);
        }

        // Text selection
        if let Some(v) = j.text_selection {
            n.set_text_selection(v.0);
        }

        // Custom actions
        if !j.custom_actions.is_empty() {
            n.set_custom_actions(
                j.custom_actions
                    .into_iter()
                    .map(|ca| ca.0)
                    .collect::<Vec<_>>(),
            );
        }

        n
    }
}

/// Return the display text of an AccessKit node — label first, then value.
///
/// Bridges should call this instead of accessing `.label()` / `.value()` directly
/// so that the priority rule (label before value) is consistent across renderers.
pub fn node_label(node: &accesskit::Node) -> &str {
    node.label().or_else(|| node.value()).unwrap_or("")
}
