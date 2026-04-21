//! Trenchcoat wrapper for [`redb::Durability`].

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Write durability level for a redb write transaction.
///
/// Wraps `redb::Durability` to add [`JsonSchema`] for MCP boundary crossing.
///
/// Note: `redb::Durability` is `#[non_exhaustive]` with two public variants.
/// Future redb releases may add more; unknown variants map to [`Durability::Immediate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Durability {
    /// Commits will not be persisted to disk unless followed by an
    /// [`Immediate`](Durability::Immediate) commit. Fastest writes.
    None,
    /// Commits are guaranteed persistent as soon as
    /// [`WriteTransaction::commit`](https://docs.rs/redb/latest/redb/struct.WriteTransaction.html#method.commit)
    /// returns.
    Immediate,
}

#[cfg(feature = "redb-types")]
impl From<redb::Durability> for Durability {
    fn from(d: redb::Durability) -> Self {
        match d {
            redb::Durability::None => Durability::None,
            redb::Durability::Immediate => Durability::Immediate,
            _ => Durability::Immediate,
        }
    }
}

#[cfg(feature = "redb-types")]
impl From<Durability> for redb::Durability {
    fn from(d: Durability) -> Self {
        match d {
            Durability::None => redb::Durability::None,
            Durability::Immediate => redb::Durability::Immediate,
        }
    }
}
