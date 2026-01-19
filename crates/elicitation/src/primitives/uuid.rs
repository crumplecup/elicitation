//! UUID implementation for universally unique identifier elicitation.

use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
use uuid::Uuid;

// Generate default-only style enum
crate::default_style!(Uuid => UuidStyle);

impl Prompt for Uuid {
    fn prompt() -> Option<&'static str> {
        Some(
            "Please provide a UUID (hyphenated format like '550e8400-e29b-41d4-a716-446655440000'), or type 'generate' to create a new random UUID:",
        )
    }
}

impl Elicitation for Uuid {
    type Style = UuidStyle;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting UUID");

        // Elicit as string
        let input = String::elicit(client).await?;
        let trimmed = input.trim();

        // Check if user wants to generate a new UUID
        if trimmed.eq_ignore_ascii_case("generate")
            || trimmed.eq_ignore_ascii_case("random")
            || trimmed.eq_ignore_ascii_case("new")
        {
            let uuid = Uuid::new_v4();
            tracing::debug!(uuid = %uuid, "Generated random UUID v4");
            return Ok(uuid);
        }

        // Try to parse as UUID
        match Uuid::parse_str(trimmed) {
            Ok(uuid) => {
                tracing::debug!(uuid = %uuid, "UUID parsed successfully");
                Ok(uuid)
            }
            Err(e) => {
                tracing::error!(error = ?e, input = %trimmed, "Failed to parse UUID");
                Err(crate::ElicitErrorKind::ParseError(format!(
                    "Invalid UUID format: {}. Expected hyphenated format or 'generate' keyword.",
                    trimmed
                ))
                .into())
            }
        }
    }
}
