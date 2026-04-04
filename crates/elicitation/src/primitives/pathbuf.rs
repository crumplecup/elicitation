//! PathBuf implementation for filesystem path elicitation.

use crate::{ElicitCommunicator, ElicitResult, Elicitation, Prompt};
use std::path::PathBuf;

// Generate default-only style enum
crate::default_style!(PathBuf => PathBufStyle);

impl Prompt for PathBuf {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a filesystem path:")
    }
}

impl Elicitation for PathBuf {
    type Style = PathBufStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBuf");

        // Elicit as string
        let path_str = String::elicit(communicator).await?;

        // Convert to PathBuf (accepts any valid UTF-8 string)
        let path = PathBuf::from(path_str);

        tracing::debug!(path = ?path, "PathBuf created");
        Ok(path)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_pathbuf()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_pathbuf()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_pathbuf()
    }
}
