//! Global registry of type specs via inventory.
//!
//! Each type that implements [`ElicitSpec`](crate::ElicitSpec) submits a
//! [`TypeSpecInventoryKey`] at compile time. The [`lookup_type_spec`] function
//! searches the registry at runtime by type name.

use crate::TypeSpec;

/// Registration key for the global type spec inventory.
///
/// Submit one of these for each type that implements [`ElicitSpec`](crate::ElicitSpec).
/// The `type_name` must be a string literal (`&'static str`) because
/// `inventory::submit!` requires const-constructible values.
///
/// ```rust,ignore
/// inventory::submit!(TypeSpecInventoryKey::new(
///     "I32Positive",
///     I32Positive::type_spec,
/// ));
/// ```
pub struct TypeSpecInventoryKey {
    type_name: &'static str,
    builder: fn() -> TypeSpec,
}

impl TypeSpecInventoryKey {
    /// Creates a new registry key.
    ///
    /// # Arguments
    ///
    /// * `type_name` - The name agents use to look up this type (e.g., `"I32Positive"`)
    /// * `builder` - Function that constructs the [`TypeSpec`] on demand
    pub const fn new(type_name: &'static str, builder: fn() -> TypeSpec) -> Self {
        Self { type_name, builder }
    }

    /// The type name this key is registered under.
    pub fn type_name(&self) -> &str {
        self.type_name
    }

    /// Builds the [`TypeSpec`] for this type.
    pub fn build(&self) -> TypeSpec {
        (self.builder)()
    }
}

inventory::collect!(TypeSpecInventoryKey);

/// Look up the spec for a type by name.
///
/// Returns `None` if no [`TypeSpecInventoryKey`] has been submitted for that name.
///
/// # Example
///
/// ```rust,ignore
/// if let Some(spec) = lookup_type_spec("I32Positive") {
///     println!("{}: {}", spec.type_name(), spec.summary());
/// }
/// ```
pub fn lookup_type_spec(name: &str) -> Option<TypeSpec> {
    inventory::iter::<TypeSpecInventoryKey>()
        .find(|k| k.type_name() == name)
        .map(|k| k.build())
}
