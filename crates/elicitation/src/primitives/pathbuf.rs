//! PathBuf implementation for filesystem path elicitation.

use crate::{ElicitResult, Elicitation, Prompt};
use rmcp::service::{Peer, RoleClient};
use std::path::PathBuf;

impl Prompt for PathBuf {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a filesystem path:")
    }
}

impl Elicitation for PathBuf {
    #[tracing::instrument(skip(client))]
    async fn elicit(client: &Peer<RoleClient>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting PathBuf");

        // Elicit as string
        let path_str = String::elicit(client).await?;

        // Convert to PathBuf (accepts any valid UTF-8 string)
        let path = PathBuf::from(path_str);

        tracing::debug!(path = ?path, "PathBuf created");
        Ok(path)
    }
}
