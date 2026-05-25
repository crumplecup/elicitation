//! Gallery level C41: manual `Elicitation` impl with helper-backed Creusot proofs.
//!
//! **Hypothesis**: if C40 compiles cleanly, then adding only the
//! `creusot_single_variant_enum` / `creusot_multi_variant_enum` helper calls to
//! `creusot_proof()` should tell us whether those helper-backed methods are the
//! decisive Creusot packaging issue.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c41-manual-helper-proof
//! ```

use elicitation::{Elicitation, Prompt, style::ElicitationStyle};

/// Tiny manual style enum.
#[cfg(feature = "gallery-c41-manual-helper-proof")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C41StateStyle {
    /// Default style.
    #[default]
    Default,
}

#[cfg(feature = "gallery-c41-manual-helper-proof")]
impl Prompt for C41StateStyle {}

#[cfg(feature = "gallery-c41-manual-helper-proof")]
impl ElicitationStyle for C41StateStyle {}

#[cfg(feature = "gallery-c41-manual-helper-proof")]
impl Elicitation for C41StateStyle {
    type Style = C41StateStyle;

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
        elicitation::verification::proof_helpers::creusot_single_variant_enum("C41StateStyle")
    }
}

/// Tiny manual state enum.
#[cfg(feature = "gallery-c41-manual-helper-proof")]
#[derive(Debug, Clone, Copy, Default)]
pub enum C41State {
    /// Initial state.
    #[default]
    Idle,
    /// Finished state.
    Done,
}

#[cfg(feature = "gallery-c41-manual-helper-proof")]
impl Prompt for C41State {
    fn prompt() -> Option<&'static str> {
        Some("Please select a C41State:")
    }
}

#[cfg(feature = "gallery-c41-manual-helper-proof")]
impl Elicitation for C41State {
    type Style = C41StateStyle;

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
        elicitation::verification::proof_helpers::creusot_multi_variant_enum("C41State")
    }
}
