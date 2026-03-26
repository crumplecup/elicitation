//! Runtime-queryable structural registry for elicitation types.
//!
//! Each composite type registered via `#[derive(Elicit)]` submits a
//! [`TypeGraphKey`] that stores a `fn() -> TypeMetadata`. The graph builder
//! uses [`lookup_type_graph`] to traverse field/variant edges without
//! monomorphized generics.
//!
//! # Registration
//!
//! Types register automatically when `#[derive(Elicit)]` is used and the
//! `graph` feature is enabled. Manual registration uses `inventory::submit!`:
//!
//! ```rust,ignore
//! inventory::submit!(elicitation::TypeGraphKey::new(
//!     "MyType",
//!     <MyType as elicitation::ElicitIntrospect>::metadata,
//! ));
//! ```

use crate::TypeMetadata;

/// Registration key for the global type graph inventory.
///
/// Stores a `fn() -> TypeMetadata` so the graph builder can reconstruct
/// full structural information (fields, variant fields) at runtime by name.
pub struct TypeGraphKey {
    type_name: &'static str,
    builder: fn() -> TypeMetadata,
}

impl TypeGraphKey {
    /// Creates a new registry key.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The type name agents and the graph builder use for lookup
    /// * `builder` - Function that constructs the [`TypeMetadata`] on demand
    pub const fn new(type_name: &'static str, builder: fn() -> TypeMetadata) -> Self {
        Self { type_name, builder }
    }

    /// The type name this key is registered under.
    pub fn type_name(&self) -> &str {
        self.type_name
    }

    /// Builds the [`TypeMetadata`] for this type.
    pub fn build(&self) -> TypeMetadata {
        (self.builder)()
    }
}

inventory::collect!(TypeGraphKey);

/// Look up structural metadata for a type by name.
///
/// Returns `None` if no [`TypeGraphKey`] has been submitted for that name.
/// Unknown names are treated as primitive leaf nodes by the graph builder.
pub fn lookup_type_graph(name: &str) -> Option<TypeMetadata> {
    inventory::iter::<TypeGraphKey>()
        .find(|k| k.type_name() == name)
        .map(|k| k.build())
}

/// All registered graphable type names.
///
/// Useful for tooling that needs to enumerate what can be visualized
/// (e.g. `elicitation graph --list` or `type_graph__list_types`).
pub fn all_graphable_types() -> Vec<&'static str> {
    let mut names: Vec<&'static str> = inventory::iter::<TypeGraphKey>()
        .map(|k| k.type_name())
        .collect();
    names.sort_unstable();
    names
}
