//! Gallery level C38: tiny `Elicit`-only state.
//!
//! **Hypothesis**: if the clean C37 isolate still ICEs, the decisive packaging
//! ingredient may be `#[derive(Elicit)]`, specifically its generated
//! `creusot_proof()` surface for fieldless enums.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c38-elicit-only
//! ```

use elicitation::Elicit;

/// Tiny enum exercising only the `Elicit` derive surface.
#[cfg(feature = "gallery-c38-elicit-only")]
#[derive(Debug, Clone, Default, Elicit)]
pub enum C38State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}
