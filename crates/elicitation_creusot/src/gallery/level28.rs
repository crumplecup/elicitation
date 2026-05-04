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
///
/// **Finding**: `trusted` is NOT re-exported from `elicitation` — `E0433`.
/// Use `#[trusted]` directly from the `creusot_std::prelude` import instead.
/// The `#[elicitation::trusted]` path is not available; see C28 finding below.
///
/// Comment this out to avoid compile error — the test is documented here.
// #[elicitation::trusted]
// pub fn c28_via_elicitation_path(x: u32) -> u32 { x + 1 }

/// Test: annotate with bare `#[trusted]` from local prelude (always works).
#[trusted]
pub fn c28_via_local_prelude(x: u32) -> u32 {
    x + 2
}
