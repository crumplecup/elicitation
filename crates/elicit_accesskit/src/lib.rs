//! `elicit_accesskit` — elicitation-enabled wrappers around `accesskit` types.
//!
//! Provides newtype wrappers for accesskit types with [`schemars::JsonSchema`],
//! MCP-reflectable methods, and lossless `From`/`Into` conversions.
//!
//! # Supported types
//!
//! | Wrapper | Inner type | Pattern |
//! |---------|-----------|---------|
//! | [`Role`] | `accesskit::Role` | Copy enum, all 182 variants |
//! | [`Action`] | `accesskit::Action` | Copy enum |
//! | [`Orientation`] | `accesskit::Orientation` | Copy enum |
//! | [`TextDirection`] | `accesskit::TextDirection` | Copy enum |
//! | [`Invalid`] | `accesskit::Invalid` | Copy enum |
//! | [`Toggled`] | `accesskit::Toggled` | Copy enum |
//! | [`SortDirection`] | `accesskit::SortDirection` | Copy enum |
//! | [`AriaCurrent`] | `accesskit::AriaCurrent` | Copy enum |
//! | [`AutoComplete`] | `accesskit::AutoComplete` | Copy enum |
//! | [`Live`] | `accesskit::Live` | Copy enum |
//! | [`HasPopup`] | `accesskit::HasPopup` | Copy enum |
//! | [`ListStyle`] | `accesskit::ListStyle` | Copy enum |
//! | [`TextAlign`] | `accesskit::TextAlign` | Copy enum |
//! | [`VerticalOffset`] | `accesskit::VerticalOffset` | Copy enum |
//! | [`TextDecorationStyle`] | `accesskit::TextDecorationStyle` | Copy enum |
//! | [`ScrollUnit`] | `accesskit::ScrollUnit` | Copy enum |
//! | [`ScrollHint`] | `accesskit::ScrollHint` | Copy enum |
//! | [`NodeId`] | `accesskit::NodeId` | Id newtype |
//! | [`TreeId`] | `accesskit::TreeId` | Id newtype |
//! | [`CustomAction`] | `accesskit::CustomAction` | Simple struct |
//! | [`TextPosition`] | `accesskit::TextPosition` | Simple struct |
//! | [`TextSelection`] | `accesskit::TextSelection` | Simple struct |
//! | [`Color`] | `accesskit::Color` | Simple struct |
//! | [`TextDecoration`] | `accesskit::TextDecoration` | Simple struct |
//! | [`Tree`] | `accesskit::Tree` | Simple struct |
//! | [`Rect`] | `accesskit::Rect` | Geometry |
//! | [`Point`] | `accesskit::Point` | Geometry |
//! | [`Size`] | `accesskit::Size` | Geometry |
//! | [`Vec2`] | `accesskit::Vec2` | Geometry |
//! | [`Affine`] | `accesskit::Affine` | Geometry |
//! | [`NodeJson`] | `accesskit::Node` | JSON intermediate (Node has no serde) |
//! | [`TreeUpdateJson`] | `accesskit::TreeUpdate` | JSON intermediate |
//!
//! # Usage
//!
//! ```rust
//! use elicit_accesskit::{NodeJson, Role};
//! use accesskit::Role as AkRole;
//!
//! let node = NodeJson::new(Role(AkRole::Button))
//!     .with_label("Submit".to_string())
//!     .with_is_disabled(false);
//!
//! let ak_node = accesskit::Node::from(node);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod enums;
mod geometry;
mod node;
mod node_id;
pub mod sql;
mod status_bar;
mod structs;
mod tree_update;

pub use enums::{
    Action, AriaCurrent, AutoComplete, HasPopup, Invalid, ListStyle, Live, Orientation, Role,
    ScrollHint, ScrollUnit, SortDirection, TextAlign, TextDecorationStyle, TextDirection, Toggled,
    VerticalOffset,
};
pub use geometry::{Affine, Point, Rect, Size, Vec2};
pub use node::{NodeJson, node_label};
pub use node_id::{NodeId, TreeId};
pub use status_bar::{ColorTheme, KeyBinding, StatusBarDescriptor};
pub use structs::{Color, CustomAction, TextDecoration, TextPosition, TextSelection, Tree};
pub use tree_update::{NodeEntry, TreeUpdateJson};

/// Macro to create a thin newtype wrapper around a `Copy` accesskit enum.
///
/// Delegates `JsonSchema` to the inner type (preserving enum variant info),
/// and provides transparent `Serialize`/`Deserialize`, `Deref`, and `From`/`Into`.
#[macro_export]
macro_rules! accesskit_copy_enum {
    ($name:ident, $inner:ty) => {
        #[doc = concat!("Thin wrapper around [`", stringify!($inner), "`] for JSON schema support.")]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub $inner);

        impl schemars::JsonSchema for $name {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                stringify!($name).into()
            }

            fn json_schema(schema_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
                <$inner as schemars::JsonSchema>::json_schema(schema_gen)
            }

            fn inline_schema() -> bool {
                <$inner as schemars::JsonSchema>::inline_schema()
            }
        }

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self(v)
            }
        }

        impl From<$name> for $inner {
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl std::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
