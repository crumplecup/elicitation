//! Trenchcoat wrapper for [`redb::TypeName`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A globally unique type identifier used by redb to name key and value types.
///
/// Wraps `redb::TypeName` to add [`JsonSchema`] for MCP boundary crossing.
/// Use `TypeName::new("crate_name::MyType")` — prefix with crate name to
/// avoid collisions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TypeName {
    /// The fully-qualified type name string (e.g. `"my_crate::MyKey"`).
    pub name: String,
}

impl TypeName {
    /// Create a new type name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[cfg(feature = "redb-types")]
impl From<redb::TypeName> for TypeName {
    fn from(t: redb::TypeName) -> Self {
        Self {
            name: t.name().to_string(),
        }
    }
}

#[cfg(feature = "redb-types")]
impl From<TypeName> for redb::TypeName {
    fn from(t: TypeName) -> Self {
        redb::TypeName::new(&t.name)
    }
}
