//! Elicitation shadow crate for `toml`, `toml_edit`, and `toml_datetime`.
//!
//! Exposes the full API surface of all three crates as MCP tools via a
//! [`TomlPlugin`] that holds live [`toml_edit::DocumentMut`], [`toml_edit::Table`],
//! [`toml_edit::Array`], and [`toml_edit::InlineTable`] instances keyed by UUID.
//!
//! # Plugin
//!
//! [`TomlPlugin`] — stateful plugin holding live document/table/array objects.
//!
//! # Typical workflow
//!
//! 1. `toml__parse__from_str` → `document_id`
//! 2. `toml__document__get` to inspect keys
//! 3. `toml__document__insert` to modify
//! 4. `toml__document__to_string` to serialise back to TOML text

#![forbid(unsafe_code)]

mod array_tools;
mod datetime_tools;
mod document_tools;
mod item_tools;
mod key_tools;
mod parse_tools;
mod plugin;
mod table_tools;
mod value_tools;

pub use plugin::{TomlCtx, TomlPlugin};
