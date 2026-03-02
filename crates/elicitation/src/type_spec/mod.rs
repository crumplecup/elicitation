//! Type specification layer for agent-accessible type exploration.
//!
//! Provides the [`TypeSpec`] data structure and [`ElicitSpec`] trait that
//! enable agents to lazily explore type contracts without overwhelming context windows.
//!
//! # The Lazy Dictionary Pattern
//!
//! Instead of dumping full specs into prompts, agents call `describe_type`
//! to get a summary and available categories, then `explore_type` to
//! fetch individual categories on demand.
//!
//! # Example
//!
//! ```rust,ignore
//! // Agent asks: what categories does I32Positive have?
//! // → "requires (1), ensures (1)"
//!
//! // Agent asks: show me the requires
//! // → [SpecEntry { label: "positive", description: "value must be > 0", expression: Some("value > 0") }]
//! ```

mod bool_contracts;
mod char_contracts;
mod collection_contracts;
mod collections;
mod datetime_specs;
mod elicit_spec;
mod float_contracts;
mod http_specs;
mod integer_contracts;
mod integers;
mod network_specs;
mod regex_specs;
mod registry;
mod scalars;
mod std_extras;
mod string_contracts;
mod strings;
pub mod type_spec_plugin;
mod url_specs;
mod uuid_specs;
mod value_specs;

pub use collections::{HashMapSpec, HashSetSpec, OptionSpec, ResultSpec, VecSpec};

pub use elicit_spec::ElicitSpec;
pub use registry::{TypeSpecInventoryKey, lookup_type_spec};

use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;

/// A single spec condition or constraint entry.
///
/// Represents one line of a spec category — e.g., a single `requires` condition,
/// one field description, or one postcondition.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters, Builder)]
#[setters(prefix = "with_")]
pub struct SpecEntry {
    /// Short identifier for this entry (e.g., `"positive"`, `"non_empty"`).
    label: String,

    /// Human-readable description of this condition.
    description: String,

    /// Optional Rust expression mirroring the anodized `#[spec]` condition.
    ///
    /// For example, `Some("value > 0")` for an I32Positive requires entry.
    #[builder(default)]
    expression: Option<String>,
}

/// A named group of spec entries.
///
/// Groups related entries under a single category name such as `"requires"`,
/// `"ensures"`, `"maintains"`, `"bounds"`, or `"fields"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters, Builder)]
#[setters(prefix = "with_")]
pub struct SpecCategory {
    /// Category name: `"requires"`, `"ensures"`, `"maintains"`, `"bounds"`, or `"fields"`.
    name: String,

    /// The entries in this category.
    #[builder(default)]
    entries: Vec<SpecEntry>,
}

/// Complete spec description for a type, browsable by agents on demand.
///
/// Built alongside the anodized `#[spec]` annotations on constructors so the
/// conditions stay in sync. Agents retrieve this through the `describe_type`
/// and `explore_type` MCP tools rather than having it injected into every prompt.
#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, Builder)]
#[setters(prefix = "with_")]
pub struct TypeSpec {
    /// Type name (e.g., `"I32Positive"`, `"StringNonEmpty"`).
    type_name: String,

    /// One-line summary of what this type represents.
    summary: String,

    /// Available spec categories for this type.
    #[builder(default)]
    categories: Vec<SpecCategory>,
}
