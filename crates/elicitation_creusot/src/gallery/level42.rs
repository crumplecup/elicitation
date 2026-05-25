//! Gallery level C42: manual traced async `Elicitation`.
//!
//! **Hypothesis**: if manual `creusot_proof()` methods do not ICE, the next
//! suspect from `#[derive(Elicit)]` is the traced `async fn elicit` machinery.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c42-manual-async-traced
//! ```

use elicitation::{Elicitation, Prompt, style::ElicitationStyle};

/// Tiny manual style enum.
#[cfg(feature = "gallery-c42-manual-async-traced")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C42StateStyle {
    /// Default style.
    #[default]
    Default,
}

#[cfg(feature = "gallery-c42-manual-async-traced")]
impl Prompt for C42StateStyle {}

#[cfg(feature = "gallery-c42-manual-async-traced")]
impl ElicitationStyle for C42StateStyle {}

#[cfg(feature = "gallery-c42-manual-async-traced")]
impl Elicitation for C42StateStyle {
    type Style = C42StateStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: elicitation::ElicitCommunicator>(
        _communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        Ok(Self::Default)
    }

    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        elicitation::proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        elicitation::proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        elicitation::proc_macro2::TokenStream::new()
    }
}

/// Tiny manual state enum.
#[cfg(feature = "gallery-c42-manual-async-traced")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C42State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

#[cfg(feature = "gallery-c42-manual-async-traced")]
impl Prompt for C42State {
    fn prompt() -> Option<&'static str> {
        Some("Please select a C42State:")
    }
}

#[cfg(feature = "gallery-c42-manual-async-traced")]
impl Elicitation for C42State {
    type Style = C42StateStyle;

    #[tracing::instrument(skip(_communicator))]
    async fn elicit<C: elicitation::ElicitCommunicator>(
        _communicator: &C,
    ) -> elicitation::ElicitResult<Self> {
        Ok(Self::Idle)
    }

    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        elicitation::proc_macro2::TokenStream::new()
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        elicitation::proc_macro2::TokenStream::new()
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        elicitation::proc_macro2::TokenStream::new()
    }
}
