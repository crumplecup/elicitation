//! Gallery level C52: generic `JsonSchema` derive in isolation.
//!
//! **Hypothesis**: generic `JsonSchema` derive is an independent Creusot issue
//! from the serde-bound failures.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c52-jsonschema-generic
//! ```

use schemars::JsonSchema;

/// Generic parameter struct with `JsonSchema` derive only.
#[cfg(feature = "gallery-c52-jsonschema-generic")]
#[derive(Debug, Clone, JsonSchema)]
pub struct C52Params<T>
where
    T: elicitation::ElicitComplete + JsonSchema,
{
    /// Owned parameter payload.
    pub item: T,
}
