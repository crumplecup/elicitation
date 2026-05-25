//! Gallery level C48: generic parameter struct with explicit wire-format bounds.
//!
//! **Hypothesis**: if C47 fails because `ElicitComplete` no longer carries the
//! serde/schema surface under Creusot, then adding those bounds explicitly
//! should remove the local derive failure.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c48-generic-param-wire-bounds
//! ```

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Same generic param-struct shape, but with explicit wire-format bounds.
#[cfg(feature = "gallery-c48-generic-param-wire-bounds")]
#[derive(Debug, Clone, Elicit, JsonSchema, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct C48Params<T>
where
    T: elicitation::ElicitComplete + Serialize + JsonSchema + for<'de2> Deserialize<'de2>,
{
    /// Owned parameter payload.
    pub item: T,
}
