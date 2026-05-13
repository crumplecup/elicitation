//! Factory tooling for [`surrealdb_types::SurrealValue`].
//!
//! Exposes the three core `SurrealValue` methods as dynamic MCP tools via
//! `#[reflect_trait]`.  The `type_map` bridges the upstream value types
//! to their serializable proxy counterparts:
//!
//! - `surrealdb_types::Kind`  → [`crate::Kind`]
//! - `surrealdb_types::Value` → [`crate::Value`]
//!
//! `from_value` is intentionally excluded because `surrealdb_types::Error`
//! does not implement `Serialize`, so it cannot be forwarded over the MCP
//! boundary.  Users who need error-path handling should use the SurrealQL
//! expression tools instead.

use elicitation_derive::reflect_trait;

/// Expose [`surrealdb_types::SurrealValue`] as dynamic MCP tools.
///
/// Three methods are reflected:
/// - `kind_of() -> Kind` — the SurrealDB `Kind` that describes this type
/// - `is_value(value: &Value) -> bool` — test whether a `Value` matches this type
/// - `into_value(self) -> Value` — convert an instance into a `Value`
#[reflect_trait(
    surrealdb_types::SurrealValue,
    type_map(
        surrealdb_types::Value => crate::Value,
        surrealdb_types::Kind  => crate::Kind
    )
)]
pub trait SurrealValueTools {
    /// Returns the [`Kind`] that represents this type's schema.
    fn kind_of() -> surrealdb_types::Kind;

    /// Returns `true` if `value` can be converted to this type.
    fn is_value(value: &surrealdb_types::Value) -> bool;

    /// Converts `self` into a [`surrealdb_types::Value`].
    fn into_value(self) -> surrealdb_types::Value;
}
