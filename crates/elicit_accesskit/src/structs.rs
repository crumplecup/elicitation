//! Wrappers for simple accesskit struct types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{NodeId, TextDecorationStyle};

/// Wrapper around [`accesskit::CustomAction`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CustomAction(pub accesskit::CustomAction);

impl JsonSchema for CustomAction {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "CustomAction".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::CustomAction::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::CustomAction::inline_schema()
    }
}

impl From<accesskit::CustomAction> for CustomAction {
    fn from(v: accesskit::CustomAction) -> Self {
        Self(v)
    }
}

impl From<CustomAction> for accesskit::CustomAction {
    fn from(v: CustomAction) -> Self {
        v.0
    }
}

impl std::ops::Deref for CustomAction {
    type Target = accesskit::CustomAction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around [`accesskit::TextPosition`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TextPosition(pub accesskit::TextPosition);

impl JsonSchema for TextPosition {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TextPosition".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::TextPosition::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::TextPosition::inline_schema()
    }
}

impl From<accesskit::TextPosition> for TextPosition {
    fn from(v: accesskit::TextPosition) -> Self {
        Self(v)
    }
}

impl From<TextPosition> for accesskit::TextPosition {
    fn from(v: TextPosition) -> Self {
        v.0
    }
}

impl std::ops::Deref for TextPosition {
    type Target = accesskit::TextPosition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Wrapper around [`accesskit::TextSelection`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TextSelection(pub accesskit::TextSelection);

impl JsonSchema for TextSelection {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TextSelection".into()
    }

    fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        accesskit::TextSelection::json_schema(schema_gen)
    }

    fn inline_schema() -> bool {
        accesskit::TextSelection::inline_schema()
    }
}

impl From<accesskit::TextSelection> for TextSelection {
    fn from(v: accesskit::TextSelection) -> Self {
        Self(v)
    }
}

impl From<TextSelection> for accesskit::TextSelection {
    fn from(v: TextSelection) -> Self {
        v.0
    }
}

impl std::ops::Deref for TextSelection {
    type Target = accesskit::TextSelection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// RGBA color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    /// Red channel (0–255).
    pub red: u8,
    /// Green channel (0–255).
    pub green: u8,
    /// Blue channel (0–255).
    pub blue: u8,
    /// Alpha channel (0–255).
    pub alpha: u8,
}

impl From<accesskit::Color> for Color {
    fn from(c: accesskit::Color) -> Self {
        Self {
            red: c.red,
            green: c.green,
            blue: c.blue,
            alpha: c.alpha,
        }
    }
}

impl From<Color> for accesskit::Color {
    fn from(c: Color) -> Self {
        accesskit::Color {
            red: c.red,
            green: c.green,
            blue: c.blue,
            alpha: c.alpha,
        }
    }
}

/// The style and color for a type of text decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TextDecoration {
    /// The decoration style (solid, dotted, dashed, etc.).
    pub style: TextDecorationStyle,
    /// The decoration color.
    pub color: Color,
}

impl From<accesskit::TextDecoration> for TextDecoration {
    fn from(d: accesskit::TextDecoration) -> Self {
        Self {
            style: d.style.into(),
            color: d.color.into(),
        }
    }
}

impl From<TextDecoration> for accesskit::TextDecoration {
    fn from(d: TextDecoration) -> Self {
        accesskit::TextDecoration {
            style: d.style.into(),
            color: d.color.into(),
        }
    }
}

/// Wrapper around [`accesskit::Tree`].
///
/// Holds the stable metadata for an accessibility tree: the root node ID
/// and optional toolkit identification strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    /// The identifier of the tree's root node.
    pub root: NodeId,
    /// The name of the UI toolkit in use.
    pub toolkit_name: Option<String>,
    /// The version of the UI toolkit.
    pub toolkit_version: Option<String>,
}

impl From<accesskit::Tree> for Tree {
    fn from(t: accesskit::Tree) -> Self {
        Self {
            root: t.root.into(),
            toolkit_name: t.toolkit_name,
            toolkit_version: t.toolkit_version,
        }
    }
}

impl From<Tree> for accesskit::Tree {
    fn from(t: Tree) -> Self {
        accesskit::Tree {
            root: t.root.into(),
            toolkit_name: t.toolkit_name,
            toolkit_version: t.toolkit_version,
        }
    }
}
