//! Shadow of [`redb::Savepoint`].
//!
//! Serializes as a UUID string.  The actual `redb::Savepoint` lives in
//! [`RedbCtx`] keyed by UUID.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Shadow of `redb::Savepoint`.
///
/// A UUID handle identifying a live `redb::Savepoint` in the plugin context.
#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema)]
pub struct Savepoint(pub Uuid);

impl Serialize for Savepoint {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Savepoint {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        s.parse::<Uuid>()
            .map(Savepoint)
            .map_err(serde::de::Error::custom)
    }
}
