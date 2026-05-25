//! Gallery level C39: tiny `KaniVariantState`-only enum.
//!
//! **Hypothesis**: if C38 ICEs but this level does not, the root cause sits in
//! `#[derive(Elicit)]` rather than the state-side Kani helper derive.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c39-kani-variant-only
//! ```

use elicitation::KaniVariantState;

/// Tiny enum exercising only the Kani variant-construction derive.
#[cfg(feature = "gallery-c39-kani-variant-only")]
#[derive(Debug, Clone, Default, KaniVariantState)]
pub enum C39State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}
