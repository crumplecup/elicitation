//! Gallery level C37: tiny state only.
//!
//! **Hypothesis**: if C36 still ICEs without any formal transition, the lowest
//! remaining packaging suspect is the state derive surface itself. This level
//! keeps only the tiny `Elicit` + `KaniVariantState` enum and removes all
//! proposition and invariant items.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c37-state-only
//! ```

use elicitation::{Elicit, KaniVariantState};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Tiny `ElicitComplete`-style enum with no proof items around it.
#[cfg(feature = "gallery-c37-state-only")]
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniVariantState)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub enum C37State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}
