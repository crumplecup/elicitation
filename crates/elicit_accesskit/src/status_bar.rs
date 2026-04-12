//! Zellij-style keybinding status bar descriptor.
//!
//! Defines [`ColorTheme`], [`KeyBinding`], and [`StatusBarDescriptor`].
//! All three are AccessKit IR constructs: `to_ak_nodes` produces a
//! `Role::Status` root whose `Role::Group` children represent individual
//! key/action chip pairs.  Renderers (ratatui, leptos) interpret this IR to
//! produce their native output.

use accesskit::Role;
use elicitation::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{NodeId, NodeJson};

// ── ColorTheme ────────────────────────────────────────────────────────────────

/// Selectable color theme for the status bar and other themed UI regions.
///
/// The default is [`ColorTheme::Dark`].  Renderers use this to select the
/// appropriate palette; for ratatui the palette drives `ratatui::style::Color`
/// values; for leptos/HTML it is emitted as a CSS class name on the container
/// (e.g. `"theme-dark"`).
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    ToCodeLiteral,
    strum::Display,
    strum::EnumIter,
    strum::EnumString,
)]
#[serde(rename_all = "snake_case")]
pub enum ColorTheme {
    /// Dark background with muted-colour accents (default).
    #[default]
    Dark,
    /// Light background with darker accents.
    Light,
    /// Maximum contrast — suitable for users with low vision.
    HighContrast,
    /// Solarized palette.
    Solarized,
}

impl ColorTheme {
    /// CSS class name to apply to a themed HTML container.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Dark => "theme-dark",
            Self::Light => "theme-light",
            Self::HighContrast => "theme-high-contrast",
            Self::Solarized => "theme-solarized",
        }
    }
}

// ── KeyBinding ────────────────────────────────────────────────────────────────

/// A single keybinding chip shown in the status bar.
///
/// `key` is the short key label (e.g. `"q"`, `"↑↓"`, `"Enter"`);
/// `action` is the human-readable description (e.g. `"Quit"`, `"Navigate"`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct KeyBinding {
    /// Key label (short, displayed with keyboard highlight).
    pub key: String,
    /// Action description (plain text).
    pub action: String,
}

impl KeyBinding {
    /// Construct a [`KeyBinding`].
    pub fn new(key: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            action: action.into(),
        }
    }
}

// ── StatusBarDescriptor ───────────────────────────────────────────────────────

/// A Zellij-style status bar: a row of key/action chip pairs at the bottom of
/// the screen, themed by [`ColorTheme`].
///
/// # AccessKit IR
///
/// `to_ak_nodes` produces:
/// - A `Role::Status` root node whose `class_name` encodes the theme.
/// - One `Role::Group` child per [`KeyBinding`] with:
///   - `label` = the key string (e.g. `"q"`)
///   - `description` = the action string (e.g. `"Quit"`)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StatusBarDescriptor {
    /// Key bindings to display.
    pub bindings: Vec<KeyBinding>,
    /// Color theme.
    pub theme: ColorTheme,
}

impl Default for StatusBarDescriptor {
    fn default() -> Self {
        Self {
            bindings: Vec::new(),
            theme: ColorTheme::Dark,
        }
    }
}

impl StatusBarDescriptor {
    /// Construct with an explicit binding list and theme.
    pub fn new(bindings: Vec<KeyBinding>, theme: ColorTheme) -> Self {
        Self { bindings, theme }
    }

    /// Standard bindings for the archive browse view.
    pub fn archive_browse() -> Self {
        Self::new(
            vec![
                KeyBinding::new("q", "Quit"),
                KeyBinding::new("↑↓", "Navigate"),
                KeyBinding::new("Enter", "Select"),
                KeyBinding::new("r", "Refresh"),
                KeyBinding::new("?", "Help"),
            ],
            ColorTheme::Dark,
        )
    }

    /// Convert to AccessKit IR nodes.
    ///
    /// `id_base` is used as the first [`NodeId`] for the status root; children
    /// are allocated sequentially from `id_base + 1`.  Callers must ensure
    /// `id_base` is large enough to avoid collisions with other nodes in the
    /// same tree (the archive pipeline uses `10_000`).
    ///
    /// Returns `(root_id, pairs)` where `pairs` contains both the root node
    /// and all child nodes ready for insertion into a [`HashMap`].
    ///
    /// [`HashMap`]: std::collections::HashMap
    pub fn to_ak_nodes(&self, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId(accesskit::NodeId::from(id_base));
        let mut pairs: Vec<(NodeId, NodeJson)> = Vec::with_capacity(self.bindings.len() + 1);

        // Build chip children first so we can collect their IDs for the root.
        let mut child_ids: Vec<NodeId> = Vec::with_capacity(self.bindings.len());
        for (i, binding) in self.bindings.iter().enumerate() {
            let cid = NodeId(accesskit::NodeId::from(id_base + 1 + i as u64));
            let chip = NodeJson::new(crate::Role(Role::Group))
                .with_label(binding.key.clone())
                .with_description(binding.action.clone());
            child_ids.push(cid);
            pairs.push((cid, chip));
        }

        // Root Role::Status node.
        let mut root = NodeJson::new(crate::Role(Role::Status))
            .with_class_name(self.theme.css_class().to_string());
        root = root.with_children(child_ids);

        // Root goes first in pairs so callers can index `[0]` for the root.
        pairs.insert(0, (root_id, root));

        (root_id, pairs)
    }
}
