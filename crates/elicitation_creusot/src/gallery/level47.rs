//! Gallery level C47: generic parameter struct with only `ElicitComplete`.
//!
//! **Hypothesis**: the `reflect_methods` Creusot failure is caused by generated
//! generic param structs using `#[serde(bound = "")]` while relying on
//! `ElicitComplete` to imply the serde/schema surface.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c47-generic-param-elicit-complete
//! ```

use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Minimal generic param-struct shape matching `reflect_methods`.
#[cfg(feature = "gallery-c47-generic-param-elicit-complete")]
#[derive(Debug, Clone, Elicit, JsonSchema, Serialize, Deserialize)]
#[serde(bound = "")]
pub struct C47Params<T>
where
    T: elicitation::ElicitComplete,
{
    /// Owned parameter payload.
    pub item: T,
}
