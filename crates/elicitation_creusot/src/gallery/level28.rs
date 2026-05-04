//! Gallery level C28: test `elicitation::trusted` path resolution.
//!
//! **Hypothesis**: When the `creusot` Cargo feature is active on `elicitation`,
//! `pub use creusot_std::prelude::*` in `elicitation/src/lib.rs` exports `trusted`
//! so that external crates can use it as `elicitation::trusted` and internal
//! code can use it as `crate::trusted`.
//!
//! ## Run
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

/// Test: annotate with `elicitation::trusted` path.
/// If `trusted` is exported from `elicitation`, this compiles.
/// If not, we'll see E0433 "cannot find `trusted` in `elicitation`".
#[elicitation::trusted]
pub fn c28_via_elicitation_path(x: u32) -> u32 {
    x + 1
}

/// Test: annotate with bare `#[trusted]` from local prelude (should always work).
#[trusted]
pub fn c28_via_local_prelude(x: u32) -> u32 {
    x + 2
}
