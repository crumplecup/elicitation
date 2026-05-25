//! Gallery level C51: generic serde derives with explicit derive-time bounds.
//!
//! **Hypothesis**: the generic reqwest-style serde failure is satisfiable if the
//! derive gets explicit serialize/deserialize bounds, without weakening
//! `ElicitComplete`.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c51-serde-explicit-bounds
//! ```

use serde::{Deserialize, Serialize};

/// Generic parameter struct with explicit serde derive bounds.
#[cfg(feature = "gallery-c51-serde-explicit-bounds")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: serde::Deserialize<'de>"))]
pub struct C51Params<T>
where
    T: elicitation::ElicitComplete + Serialize + for<'de2> Deserialize<'de2>,
{
    /// Owned parameter payload.
    pub item: T,
}
