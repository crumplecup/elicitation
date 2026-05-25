//! Gallery level C49: generic serde/jsonschema derives with `#[serde(bound = "")]`.
//!
//! **Hypothesis**: the reqwest-style generic bound failure is fundamentally
//! about the generated empty serde bound, not `ElicitComplete` itself.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c49-serde-empty-bound
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Generic parameter struct with explicit wire-format bounds but an empty serde bound.
#[cfg(feature = "gallery-c49-serde-empty-bound")]
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct C49Params<T>
where
    T: elicitation::ElicitComplete + Serialize + JsonSchema + for<'de2> Deserialize<'de2>,
{
    /// Owned parameter payload.
    pub item: T,
}
