//! Gallery level C36: same tiny state and props, no `#[formal_method]`.
//!
//! **Hypothesis**: if C35 still ICEs without a machine derive, then
//! `#[formal_method]` may be the decisive packaging ingredient. This level keeps
//! the same tiny `ElicitComplete` state, proposition types, and named invariant
//! function, but removes the formal transition entirely.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c36-state-props-only
//! ```

use elicitation::{Elicit, KaniVariantState, Prop};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Same tiny state shape used by C34/C35.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit, KaniVariantState)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub enum C36State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

/// Shared invariant proposition.
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "c36_consistent",
    creusot_invariant_fn = "c36_consistent",
    creusot_inv_body = "true",
    verus_inv_body = "true"
)]
pub struct C36Consistent;

/// Additional proposition to match the earlier surface more closely.
#[derive(Prop)]
pub struct C36Step;

/// Source invariant function, with no formal transition consuming it.
#[cfg(feature = "gallery-c36-state-props-only")]
pub fn c36_consistent(_state: &C36State) -> bool {
    true
}
