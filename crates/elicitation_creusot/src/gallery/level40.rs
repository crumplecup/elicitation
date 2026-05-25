//! Gallery level C40: manual `Elicitation` impl with empty proof methods.
//!
//! **Hypothesis**: if `#[derive(Elicit)]` is the culprit, a hand-written
//! `Elicitation` impl with concrete `Ready<_>` futures and no helper-backed
//! `creusot_proof()` should compile cleanly under Creusot.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c40-manual-empty-proof
//! ```

use elicitation::{Elicitation, Prompt, style::ElicitationStyle};

/// Tiny manual style enum.
#[cfg(feature = "gallery-c40-manual-empty-proof")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C40StateStyle {
    /// Default style.
    #[default]
    Default,
}

#[cfg(feature = "gallery-c40-manual-empty-proof")]
impl Prompt for C40StateStyle {}

#[cfg(feature = "gallery-c40-manual-empty-proof")]
impl ElicitationStyle for C40StateStyle {}

#[cfg(feature = "gallery-c40-manual-empty-proof")]
impl Elicitation for C40StateStyle {
    type Style = C40StateStyle;

    fn elicit<C: elicitation::ElicitCommunicator>(
        _communicator: &C,
    ) -> impl std::future::Future<Output = elicitation::ElicitResult<Self>> + Send {
        std::future::ready(Ok(Self::Default))
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
#[cfg(feature = "gallery-c40-manual-empty-proof")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C40State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

#[cfg(feature = "gallery-c40-manual-empty-proof")]
impl Prompt for C40State {
    fn prompt() -> Option<&'static str> {
        Some("Please select a C40State:")
    }
}

#[cfg(feature = "gallery-c40-manual-empty-proof")]
impl Elicitation for C40State {
    type Style = C40StateStyle;

    fn elicit<C: elicitation::ElicitCommunicator>(
        _communicator: &C,
    ) -> impl std::future::Future<Output = elicitation::ElicitResult<Self>> + Send {
        std::future::ready(Ok(Self::Idle))
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
