//! Declarative descriptor for a Bevy UI entity tree.
//!
//! [`BevyUiNode`] is the `Widget` type produced by `BevyUiBackend`.  Each
//! bridge method converts one AccessKit IR node into a `BevyUiNode`, carrying
//! the pre-rendered children so the full entity hierarchy can be spawned into
//! Bevy ECS without any information loss.
//!
//! Every variant includes a `children` field because the IR may attach
//! sub-nodes to any role (e.g. `TextRun` children on a `Button` provide styled
//! label text; `InlineTextBox` children on a `TextInput` carry the current
//! value with cursor position).  Callers must walk the tree to spawn faithful
//! Bevy entity hierarchies.

use elicit_ui::WcagNodeProofs;

// ── BevyUiNode ────────────────────────────────────────────────────────────────

/// A descriptor for a single Bevy UI entity (or entity sub-tree).
///
/// Produced by [`BevyUiBackend`](crate::BevyUiBackend) during DFS traversal of
/// a verified AccessKit tree.  Consumers convert this tree into Bevy ECS spawn
/// commands — spawning the parent entity for each node, then recursively
/// spawning child entities from `children`.
///
/// **Every variant carries `children`** because the IR may attach sub-nodes to
/// any role.  An empty `children` vec simply means no sub-entities are needed.
#[derive(Debug, Clone)]
pub enum BevyUiNode {
    /// A generic flex/grid container entity.
    Container {
        /// ARIA role string for semantic identity.
        role: String,
        /// Accessible label (from the IR node, if set).
        label: Option<String>,
        /// Child entity descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A static text entity (label, paragraph, heading, text run, etc.).
    ///
    /// `children` typically holds inline `TextRun` sub-nodes when the text is
    /// composed of styled spans rather than a single flat string.
    Text {
        /// Display content (from the IR label/value).
        content: String,
        /// Heading level (1–6) when the role is Heading, otherwise `None`.
        level: Option<u8>,
        /// Inline child nodes (styled spans, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A push-button entity.
    ///
    /// `children` carry the button's label content (commonly one or more
    /// `Text` sub-nodes produced from `TextRun` children in the IR).
    Button {
        /// Flat label string (from `node.label()`, if set directly on the node).
        label: String,
        /// Whether the button is disabled.
        disabled: bool,
        /// Child entity descriptors (label text nodes, icon images, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A two-state (or tri-state) checkbox entity.
    ///
    /// `children` carry the checkbox's label text nodes.
    CheckBox {
        /// Flat label string.
        label: String,
        /// `true` = checked, `false` = unchecked, `None` = indeterminate.
        checked: Option<bool>,
        /// Whether the checkbox is disabled.
        disabled: bool,
        /// Child entity descriptors (label text, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A radio-button entity (one-of-N selection).
    RadioButton {
        /// Flat label string.
        label: String,
        /// Whether this radio button is the selected option.
        selected: bool,
        /// Whether the radio button is disabled.
        disabled: bool,
        /// Child entity descriptors (label text, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A binary toggle-switch entity (on / off).
    Switch {
        /// Flat label string.
        label: String,
        /// `true` = on, `false` = off.
        on: bool,
        /// Whether the switch is disabled.
        disabled: bool,
        /// Child entity descriptors (label text, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A continuous range-slider entity.
    Slider {
        /// Flat label string.
        label: String,
        /// Current value.
        value: Option<f64>,
        /// Minimum value.
        min: Option<f64>,
        /// Maximum value.
        max: Option<f64>,
        /// Whether the slider is disabled.
        disabled: bool,
        /// Child entity descriptors (value label text, tick marks, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A numeric spin-button entity (increment/decrement arrows).
    SpinButton {
        /// Flat label string.
        label: String,
        /// Current numeric value.
        value: Option<f64>,
        /// Minimum value.
        min: Option<f64>,
        /// Maximum value.
        max: Option<f64>,
        /// Whether the spin button is disabled.
        disabled: bool,
        /// Child entity descriptors (value text, arrow buttons, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A progress-indicator entity (determinate or indeterminate).
    Progress {
        /// Optional flat label string.
        label: Option<String>,
        /// Current value (raw; normalise against `min`/`max` if needed).
        value: Option<f64>,
        /// Child entity descriptors (label text, fill indicator, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A single-line text-input entity.
    ///
    /// `children` commonly hold `InlineTextBox` nodes carrying the current
    /// value with precise character and cursor position metadata from the IR.
    TextInput {
        /// Accessible label string.
        label: String,
        /// Current text value (from `node.value()`).
        value: String,
        /// Whether the input is disabled.
        disabled: bool,
        /// Child entity descriptors (inline text boxes, placeholder, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A multi-line text-area entity.
    ///
    /// `children` carry `InlineTextBox` nodes for each run of text in the
    /// current value.
    TextArea {
        /// Accessible label string.
        label: String,
        /// Current text value (from `node.value()`).
        value: String,
        /// Whether the text area is disabled.
        disabled: bool,
        /// Child entity descriptors (inline text boxes, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A colour-picker (color-well) entity.
    ColorWell {
        /// Accessible label string.
        label: String,
        /// Child entity descriptors (swatch preview, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A combo-box entity (input + popup list).
    ComboBox {
        /// Accessible label string.
        label: String,
        /// Currently selected / entered value string.
        value: String,
        /// Whether the combo box is disabled.
        disabled: bool,
        /// Child entity descriptors (option list items, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A standalone list-box entity (always-visible scrollable option list).
    ListBox {
        /// Accessible label string.
        label: String,
        /// Child entity descriptors (option items).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// An image entity.
    ///
    /// `children` may carry caption or overlay nodes from the IR.
    Image {
        /// Alternative text (accessible description).
        alt: Option<String>,
        /// Child entity descriptors (captions, overlays, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A scrollable viewport container.
    ScrollView {
        /// Accessible label, if set.
        label: Option<String>,
        /// Child entity descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A data table entity.
    Table {
        /// Accessible caption/label.
        label: Option<String>,
        /// Row/cell child descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A tab entity within a tab list.
    ///
    /// `children` carry the tab's label text nodes.
    Tab {
        /// Flat label string.
        label: String,
        /// Whether this tab is currently selected.
        selected: bool,
        /// Child entity descriptors (label text, icon, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A tab-panel (content area for the selected tab).
    TabPanel {
        /// Accessible label, if set.
        label: Option<String>,
        /// Child entity descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A menu entity (popup or drop-down).
    Menu {
        /// Accessible label, if set.
        label: Option<String>,
        /// Menu-item child descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A menu-item entity.
    ///
    /// `children` carry the item's label text or icon nodes.
    MenuItem {
        /// Flat label string.
        label: String,
        /// Whether the item is disabled.
        disabled: bool,
        /// Child entity descriptors (label text, shortcut, icon, etc.).
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A tree-view entity (hierarchical list).
    Tree {
        /// Accessible label, if set.
        label: Option<String>,
        /// Tree-item child descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// A tree-item entity within a tree view.
    TreeItem {
        /// Flat label string.
        label: String,
        /// Whether the item is expanded.
        expanded: bool,
        /// Child tree-item or content descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },

    /// Fallback for unrecognised or future roles.
    Unknown {
        /// Accessible label, if present.
        label: Option<String>,
        /// Child entity descriptors.
        children: Vec<BevyUiNode>,
        /// WCAG proofs from the bridge.
        proofs: WcagNodeProofs,
    },
}
