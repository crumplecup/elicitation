//! Gallery level C50: generic serde/jsonschema derives without `#[serde(bound = "")]`.
//!
//! **Hypothesis**: if C49 fails because the empty serde bound discards the
//! required wire-format constraints, letting serde infer bounds should restore
//! the local generic derive.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c50-serde-inferred-bound
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Same generic parameter struct, but without an empty serde bound override.
#[cfg(feature = "gallery-c50-serde-inferred-bound")]
#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct C50Params<T>
where
    T: elicitation::ElicitComplete + Serialize + JsonSchema + for<'de2> Deserialize<'de2>,
{
    /// Owned parameter payload.
    pub item: T,
}
